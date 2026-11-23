//! E30 validation (independent validator, 2026-10-31).
//!
//! An independent recompute of the headline numbers of
//! `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
//! §9.32 (E30; pre-registration §9.28) from the raw records in the repository,
//! and a leaf-by-leaf comparison of a fresh replay with the committed records.
//! Standard library only.  The JSON parser, SHA-256 and the leaf comparator are
//! copied from the E27 validator's `research/e27_walk_20261030/validate_e27.rs`
//! (validator code); everything else is new.  It shares no code with
//! `examples/e30_audit.rs`, `examples/e30_tables.rs` or `crypto_lib`.
//!
//! ```text
//! rustc -O --edition 2021 validate_e30.rs -o validate_e30
//! validate_e30 compare     <repo> <evidence-dir>   # -> replay/replay_comparison.json
//! validate_e30 report      <repo> <evidence-dir>   # -> validator-report.json
//! validate_e30 certificate <repo> <evidence-dir>   # -> replay-certificate.json
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
        // A printed 0 with a half unit h agrees when |recomputed| <= h; the
        // gate's relative test is then given tolerance 1 (it passes exactly
        // when the recomputed value is nonzero-but-small or zero), or 0 when it
        // does not agree, so the gate fails with this report.
        let (rel_tol, agrees) = if claimed == 0.0 {
            let ok = recomputed.abs() <= half_unit;
            (if ok { 1.0 } else { 0.0 }, ok)
        } else {
            let rel_tol = if half_unit == 0.0 {
                1e-12
            } else {
                half_unit / claimed.abs() * (1.0 + 1e-9)
            };
            let scale = claimed.abs().max(recomputed.abs()).max(1e-300);
            (rel_tol, (claimed - recomputed).abs() <= rel_tol * scale)
        };
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

const TIMING: [&str; 2] = ["wall_ms", "solve_ms"];

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

const PS: [u64; 5] = [101, 149, 197, 269, 397];
const SEEDS: [u64; 2] = [1, 2];
const ARMS: [&str; 2] = ["A", "B"];
const GAE: f64 = 97.0;
const WEIGHTS: [(&str, f64); 3] = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)];
const E30: &str = "experiments/35_invariants_lite";
const E27: &str = "experiments/32_walk_karatsuba";

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

fn wmuls(w: &J) -> f64 {
    (w.get("fp_muls").u() - w.get("verify_fp_muls").opt_u().unwrap_or(0)) as f64
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// `2.6965e10`, `0.3087 %`, `-0.3086 %`, `0.6`.
fn printed(s: &str) -> (f64, f64) {
    let t = s.trim().trim_matches('`').trim_end_matches('%').trim().to_string();
    let (mant, exp) = match t.split_once('e') {
        Some((m, e)) => (m.to_string(), e.parse::<i32>().unwrap()),
        None => (t.clone(), 0),
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

// --------------------------------------------------------- comparisons ----

fn diff_runs(pairs: Vec<(String, String, String)>, ctx: &Ctx, schema: &str, rule: &str) -> J {
    let mut runs = Vec::new();
    let (mut all_c, mut all_i, mut all_ok) = (0u64, 0u64, true);
    for (name, a_rel, b_rel) in pairs {
        let (a, ha) = ctx.read(&a_rel);
        let (b, hb) = ctx.read(&b_rel);
        let mut d = LeafDiff::default();
        leaves("$", &a, &b, &mut d);
        let ok = d.mismatches.is_empty() && d.only_a.is_empty() && d.only_b.is_empty();
        let trip = |v: &Vec<(String, J, J)>| {
            J::Arr(v.iter().map(|(p, x, y)| obj(vec![("path", js(p)), ("a", x.clone()), ("b", y.clone())])).collect())
        };
        all_ok &= ok;
        all_c += d.compared;
        all_i += d.identical;
        runs.push(obj(vec![
            ("run", js(&name)),
            ("a", obj(vec![("path", js(&a_rel)), ("sha256", js(&ha))])),
            ("b", obj(vec![("path", js(&b_rel)), ("sha256", js(&hb))])),
            ("fields_compared", ju(d.compared)),
            ("fields_identical", ju(d.identical)),
            ("mismatches", trip(&d.mismatches)),
            ("only_in_a", J::Arr(d.only_a.iter().map(|p| js(p)).collect())),
            ("only_in_b", J::Arr(d.only_b.iter().map(|p| js(p)).collect())),
            ("timing_not_compared", trip(&d.timing)),
            ("identical", jb(ok)),
        ]));
    }
    obj(vec![
        ("schema", js(schema)),
        ("rule", js(rule)),
        ("runs_total", ju(runs.len() as u64)),
        ("runs_identical", ju(runs.iter().filter(|r| r.get("identical").b()).count() as u64)),
        ("fields_compared", ju(all_c)),
        ("fields_identical", ju(all_i)),
        ("all_identical", jb(all_ok)),
        ("runs", J::Arr(runs)),
    ])
}

const RULE: &str = "every leaf of a against b: integers and floats by their text (equal text, or equal value with the same integer/float form; integers compared exactly as i128), strings and booleans exactly; wall_ms and solve_ms are wall-clock and listed, not compared";

fn compare(ctx: &Ctx) -> J {
    let mut pairs = vec![("walk_audit".to_string(), format!("{E30}/walk_audit.json"), format!("{}/replay/walk_audit.json", ctx.ev))];
    for p in PS {
        for s in SEEDS {
            pairs.push((format!("e2e_audit_p{p}s{s}"), format!("{E30}/e2e_audit_p{p}s{s}.json"), format!("{}/replay/e2e/e2e_audit_p{p}s{s}.json", ctx.ev)));
        }
    }
    diff_runs(pairs, ctx, "e30-replay-comparison/v1 (a = committed, b = replay)", RULE)
}

fn comparator_controls(ctx: &Ctx) -> J {
    let rel = format!("{E30}/e2e_audit_p101s1.json");
    let (a, _) = ctx.read(&rel);
    let (t, _) = ctx.text(&rel);
    let mut d = LeafDiff::default();
    leaves("$", &a, &a, &mut d);
    let mut all = d.mismatches.is_empty() && d.compared > 0;
    let mut out = vec![obj(vec![("control", js("record against itself")), ("fields_compared", ju(d.compared)), ("mismatches", ju(d.mismatches.len() as u64)), ("holds", jb(all))])];
    for (name, from, to) in [
        ("ic_trivial_product_savings + 1", "\"ic_trivial_product_savings\": 83252899", "\"ic_trivial_product_savings\": 83252900"),
        ("total_muls + 1", "\"total_muls\": 26964723596", "\"total_muls\": 26964723597"),
        ("visit_digest + 1 (beyond f64 precision)", "15841963702626341602", "15841963702626341603"),
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

struct Row {
    p: u64,
    s: u64,
    arm: &'static str,
    total: f64,
    sav: f64,
    m: f64,
    rho3: f64,
    st: f64,
    ws: f64,
    unaudited: f64,
    fp4: f64,
}

impl Row {
    fn ic(&self, w: f64, audited: bool) -> f64 {
        self.total - if audited { self.sav } else { 0.0 } + w * GAE * self.m
    }
    fn rho(&self, w: f64, audited: bool) -> f64 {
        self.rho3 - if audited { self.ws } else { 0.0 } + w * GAE * self.st
    }
    fn change(&self, w: f64) -> f64 {
        (self.ic(w, true) / self.rho(w, true)) / (self.ic(w, false) / self.rho(w, false)) - 1.0
    }
}

fn report(ctx: &Ctx) -> J {
    let mut raw: Vec<J> = Vec::new();
    let pin = |rel: &str, h: &str, raw: &mut Vec<J>| raw.push(obj(vec![("path", js(&format!("../../{rel}"))), ("sha256", js(h))]));
    let mut ck = Checks { items: Vec::new(), bad: Vec::new() };
    let mut hl = Headlines { items: Vec::new(), bad: Vec::new() };
    let (tables, ht) = ctx.text(&format!("{E30}/tables.md"));
    pin(&format!("{E30}/tables.md"), &ht, &mut raw);
    let (sw, hsw) = ctx.read(&format!("{E27}/stage_w.json"));
    pin(&format!("{E27}/stage_w.json"), &hsw, &mut raw);
    let (wa, hwa) = ctx.read(&format!("{E30}/walk_audit.json"));
    pin(&format!("{E30}/walk_audit.json"), &hwa, &mut raw);

    // G2.110: E30's records against E27's, leaf by leaf.
    let mut pairs = Vec::new();
    for p in PS {
        for s in SEEDS {
            pairs.push((format!("p{p}s{s}"), format!("{E27}/e2e_chain_karatsuba_p{p}s{s}.json"), format!("{E30}/e2e_audit_p{p}s{s}.json")));
        }
    }
    let g110 = diff_runs(pairs, ctx, "e30-vs-e27/v1 (a = E27's Karatsuba record, b = E30's audited record)", RULE);
    let mut g110_ok = true;
    for r in g110.get("runs").arr() {
        let extra: Vec<String> = r.get("only_in_b").arr().iter().map(|x| x.s().to_string()).collect();
        g110_ok &= r.get("mismatches").arr().is_empty()
            && r.get("only_in_a").arr().is_empty()
            && extra.len() == 2
            && extra.iter().all(|x| x == "$[0].ic_trivial_product_savings" || x == "$[1].ic_trivial_product_savings");
    }
    ck.add("G.1", "G2.110: every field of E27's 20 Karatsuba records reproduced by E30's audited records (leaf by leaf, 10 files x 2 arms); the only new leaf is ic_trivial_product_savings; wall_ms and solve_ms not compared", g110_ok, format!("{} of {} fields identical", g110.get("fields_identical").u(), g110.get("fields_compared").u()));

    // Rows.
    let mut rows: Vec<Row> = Vec::new();
    let (mut walk_ii_eq, mut gls_not_fp, mut logs_ok) = (true, true, true);
    for p in PS {
        for s in SEEDS {
            let rel = format!("{E30}/e2e_audit_p{p}s{s}.json");
            let (j, h) = ctx.read(&rel);
            pin(&rel, &h, &mut raw);
            let rel27 = format!("{E27}/e2e_chain_karatsuba_p{p}s{s}.json");
            let (_, h27) = ctx.read(&rel27);
            pin(&rel27, &h27, &mut raw);
            let c = sw.get("curves").arr().iter().find(|c| c.get("p").u() == p && c.get("seed").u() == s).expect("stage W curve");
            let g = c.get("folds").get("Gls").get("walks");
            let iii: Vec<&J> = g.get("batched").arr().iter().collect();
            let ii: Vec<&J> = g.get("karatsuba").arr().iter().collect();
            let rho3 = mean(&iii.iter().map(|w| wmuls(w)).collect::<Vec<_>>());
            let rho2 = mean(&ii.iter().map(|w| wmuls(w)).collect::<Vec<_>>());
            let st = mean(&iii.iter().map(|w| w.get("steps").f()).collect::<Vec<_>>());
            let d = iii[0].get("recovered").u();
            let wrow = wa.get("rows").arr().iter().find(|r| r.get("p").u() == p && r.get("seed").u() == s && r.get("fold").s() == "Gls").expect("walk audit row");
            walk_ii_eq &= wrow.get("mean_walk_muls_karatsuba").f() == rho2;
            for x in wa.get("rows").arr().iter().filter(|r| r.get("p").u() == p && r.get("seed").u() == s) {
                let nr = x.get("fold").s() == "Negation";
                let w2: Vec<f64> = c.get("folds").get(x.get("fold").s()).get("walks").get("karatsuba").arr().iter().map(wmuls).collect();
                walk_ii_eq &= x.get("mean_walk_muls_karatsuba").f() == mean(&w2);
                if !nr {
                    gls_not_fp &= x.get("gls_multipliers_in_fp").arr().iter().all(|b| !b.b());
                }
            }
            for (k, arm) in ARMS.iter().enumerate() {
                let r = &j.arr()[k];
                assert_eq!(r.get("arm").s(), *arm);
                let fo = r.get("folded");
                logs_ok &= fo.get("solved").b() && fo.get("correct").b() && fo.get("recovered").u() == d
                    && r.get("rho").arr().iter().all(|w| w.get("correct").b() && w.get("recovered").opt_u() == Some(d));
                rows.push(Row {
                    p,
                    s,
                    arm,
                    total: r.get("total_muls").f(),
                    sav: r.get("ic_trivial_product_savings").f(),
                    m: r.get("macaulay_entries").f(),
                    rho3,
                    st,
                    ws: wrow.get("mean_saved").f(),
                    unaudited: r.get("precompute_muls").f() + r.get("base_muls").f() + r.get("check_muls").f(),
                    fp4: r.get("weil_muls").f() + r.get("generation_muls").f() + r.get("lift_muls").f(),
                });
            }
        }
    }
    ck.add("G.2", "Every audited arm (20) solved and correct, its logarithm equal to the walks' (E25's planted d, as E27's validation), and all 48 in-process walks per arm correct with the same d", logs_ok, format!("{logs_ok}"));
    ck.add("G.3", "E30's walk audit (binary 0e4fccce1, serial Karatsuba walks) reproduces E27's walk (ii) means exactly on all 20 (curve, fold) pools: the 0e4fccce1 build walks the same walks at the same price as af2504ef1", walk_ii_eq, format!("{walk_ii_eq}"));
    ck.add("G.4", "The <±psi> walks' psi multipliers (c_x, c_y) are not in F_p on any of the ten curves (so the audit has nothing to re-price in psi)", gls_not_fp, format!("{gls_not_fp}"));

    // Headlines from tables.md.
    let mut out_rows = Vec::new();
    let md_rows: Vec<Vec<String>> = tables.lines().filter(|l| l.starts_with("| ") && !l.starts_with("| p ")).map(cells).collect();
    assert_eq!(md_rows.len(), 20);
    for (row, md) in rows.iter().zip(&md_rows) {
        assert_eq!(md[0].parse::<u64>().unwrap(), row.p);
        assert_eq!(md[2], row.arm);
        let tag = format!("p{}s{}{}", row.p, row.s, row.arm);
        let src = "experiments/35_invariants_lite/tables.md";
        let mut add = |name: &str, cell: &str, val: f64, def: &str| {
            let (v, h) = printed(cell);
            hl.add(&format!("{name}_{tag}"), src, cell, v, h, val, def);
        };
        add("s_k", &md[3], row.total, "total_muls (E27's Karatsuba S, reproduced)");
        add("saving_ic", &md[4], row.sav, "ic_trivial_product_savings");
        add("saving_share_pct", &md[5], 100.0 * row.sav / row.total, "100 * saving / S");
        add("walk_iii", &md[7], row.rho3, "mean of E27's 16 <±psi> walks (iii), fp_muls - verify_fp_muls");
        add("walk_saving", &md[8], row.ws, "walk audit mean_saved, <±psi>");
        let ch: Vec<&str> = md[9].split(" / ").collect();
        for (k, w) in WEIGHTS.iter().enumerate() {
            add(&format!("distance_change_pct_{}gae", w.0), ch[k], 100.0 * row.change(w.1), "100 * ((S - saving + w 97 M)/(rho - ws + w 97 steps)) / ((S + w 97 M)/(rho + w 97 steps)) - 100");
        }
        out_rows.push(obj(vec![
            ("p", ju(row.p)),
            ("seed", ju(row.s)),
            ("arm", js(row.arm)),
            ("s", jf(row.total)),
            ("saving", jf(row.sav)),
            ("saving_share", jf(row.sav / row.total)),
            ("walk_iii", jf(row.rho3)),
            ("walk_saving", jf(row.ws)),
            ("walk_saving_share_of_iii", jf(row.ws / row.rho3)),
            ("change", J::Arr(WEIGHTS.iter().map(|w| jf(row.change(w.1))).collect())),
            ("unaudited_setup_share_bound", jf(row.unaudited / row.total)),
            ("fp4_work_share", jf(row.fp4 / row.total)),
            ("saving_over_fp4_work", jf(row.sav / row.fp4)),
        ]));
    }
    let share: Vec<f64> = rows.iter().map(|r| r.sav / r.total).collect();
    let (shlo, shhi) = minmax(&share);
    let (svlo, svhi) = minmax(&rows.iter().map(|r| r.sav).collect::<Vec<_>>());
    let wsh: Vec<f64> = rows.iter().map(|r| r.ws / r.rho3).collect();
    let (_, wshi) = minmax(&wsh);
    let (wslo, wsmax) = minmax(&rows.iter().map(|r| r.ws).collect::<Vec<_>>());
    let ch: Vec<Vec<f64>> = WEIGHTS.iter().map(|w| rows.iter().map(|r| r.change(w.1)).collect()).collect();
    let n = "note §9.32";
    hl.add("saving_share_min_pct", n, "0.3087", 0.3087, 0.00005, 100.0 * shlo, "least 100 * saving / S over 20 rows");
    hl.add("saving_share_max_pct", n, "0.3131", 0.3131, 0.00005, 100.0 * shhi, "largest");
    hl.add("saving_min", n, "1.4·10⁷", 1.4e7, 0.05e7, svlo, "least saving per run");
    hl.add("saving_max", n, "7.1·10⁸", 7.1e8, 0.05e8, svhi, "largest");
    hl.add("walk_saving_min", n, "0", 0.0, 0.5, wslo, "least <±psi> walk saving per walk (mean of 16)");
    hl.add("walk_saving_max", n, "1.1", 1.1, 0.05, wsmax, "largest");
    hl.add("walk_saving_share_max_pct", n, "0.0002", 0.0002, 0.00005, 100.0 * wshi, "largest walk saving / walk (iii), in %; claimed as an upper bound (<= 0.0002 %)");
    let (a, b) = minmax(&ch[0]);
    hl.add("change_0gae_min_pct", n, "-0.313", -0.313, 0.0005, 100.0 * a, "most negative distance change at 0 GAE");
    hl.add("change_0gae_max_pct", n, "-0.309", -0.309, 0.0005, 100.0 * b, "least negative");
    let (a, b) = minmax(&ch[1]);
    hl.add("change_0.1gae_min_pct", n, "-0.086", -0.086, 0.0005, 100.0 * a, "most negative at 0.1 GAE");
    hl.add("change_0.1gae_max_pct", n, "-0.084", -0.084, 0.0005, 100.0 * b, "least negative");
    let (a, b) = minmax(&ch[2]);
    hl.add("change_1gae_min_pct", n, "-0.011", -0.011, 0.0005, 100.0 * a, "most negative at 1 GAE");
    hl.add("change_1gae_max_pct", n, "-0.011", -0.011, 0.0005, 100.0 * b, "least negative");
    let maxabs = ch.iter().flatten().fold(0f64, |m, &x| m.max(x.abs()));
    hl.add("max_abs_change_pct", "note §9.32 G2.113 and tables.md (\"Largest |distance change| ... 0.3131 %\")", "0.3131", 0.3131, 0.00005, 100.0 * maxabs, "largest |change| over rows and weights");
    let (ulo, uhi) = minmax(&rows.iter().map(|r| r.unaudited / r.total).collect::<Vec<_>>());
    let (flo, fhi) = minmax(&rows.iter().map(|r| r.fp4 / r.total).collect::<Vec<_>>());
    let (sflo, sfhi) = minmax(&rows.iter().map(|r| r.sav / r.fp4).collect::<Vec<_>>());
    ck.add("G.5", "Audit scope (validator's own bound): the counter is reset after set-up, so trivial products in precompute, base construction and the [x]G check are not audited; those phases are at most this share of S, which bounds what the audit can have missed", uhi < 1e-3, format!("{:.5}-{:.5} % of S", 100.0 * ulo, 100.0 * uhi));
    ck.add("G.6", "Note's \"F_p4 work ≈ 2 % of S, about one in seven of its counted products trivial\": the F_p4-priced phases (Weil restriction + target generation + lift) are this share of S, and the saving is this share of their multiplications; product counts are not emitted, so \"one in seven products\" is not checkable from the records (a trivial product saves 9 or 13 of 13)", fhi < 0.03, format!("F_p4 phases {:.3}-{:.3} % of S; saving {:.3}-{:.3} of their multiplications", 100.0 * flo, 100.0 * fhi, sflo, sfhi));

    // §5a band for the gate: audited S against audited walk (iii).
    let band_for = |sel: &dyn Fn(&Row) -> bool| -> Vec<J> {
        WEIGHTS
            .iter()
            .map(|w| {
                let rs: Vec<&Row> = rows.iter().filter(|r| sel(r)).collect();
                let ic: Vec<f64> = rs.iter().map(|r| r.ic(w.1, true)).collect();
                let rho: Vec<f64> = rs.iter().map(|r| r.rho(w.1, true)).collect();
                let ratio: Vec<f64> = rs.iter().map(|r| r.ic(w.1, true) / r.rho(w.1, true)).collect();
                let (lo, hi) = minmax(&ratio);
                obj(vec![("weight", js(w.0)), ("ic_median", jf(median(&ic))), ("rho_median", jf(median(&rho))), ("ic_over_rho_min", jf(lo)), ("ic_over_rho_max", jf(hi)), ("rho_below_ic_on_every_row", jb(lo > 1.0))])
            })
            .collect()
    };
    let band_b = band_for(&|r: &Row| r.arm == "B");
    let band_all = band_for(&|_r: &Row| true);
    let order_holds = band_all.iter().all(|b| b.get("rho_below_ic_on_every_row").b());

    let verdicts = J::Arr(vec![
        obj(vec![("id", js("G2.110")), ("note", js("holds")), ("recomputed", js(&format!("E30 = E27 on {} of {} compared leaves; the only extra leaf is the audit counter", g110.get("fields_identical").u(), g110.get("fields_compared").u())))]),
        obj(vec![("id", js("G2.111")), ("note", js("holds")), ("recomputed", js(&format!("S falls {:.4}-{:.4} % in [0.01 %, 1 %]", 100.0 * shlo, 100.0 * shhi)))]),
        obj(vec![("id", js("G2.112")), ("note", js("holds")), ("recomputed", js(&format!("<±psi> walk falls at most {:.6} % in [0 %, 1 %]", 100.0 * wshi)))]),
        obj(vec![("id", js("G2.113")), ("note", js("holds")), ("recomputed", js(&format!("largest |S/rho change| {:.4} % < 1 %", 100.0 * maxabs)))]),
    ]);
    let limitations = J::Arr(vec![
        js("Accounting only: the audit re-prices products by 0, 1 or an F_p element; nothing is computed differently, so it is not an improvement of the index calculus (class fixed in §9.28)."),
        js("The audit's window excludes set-up (precompute, base) and the [x]G check (G.5: those phases are 0.0008-0.04 % of S, so the audit can have missed at most that much; G2.111's band is unaffected); the walk audit's counter includes the walk's [d]G check while the walk cost excludes it, so the walk saving is an upper bound (it is <= 1.1 per walk either way)."),
        js("The matched rho is E27's walk (iii) from E27's binary af2504ef1 (validated, research/e27_walk_20261030); E30's binary 0e4fccce1 reproduces E27's records leaf for leaf (G.1) and its walks' (ii) means exactly (G.3), and (iii) differs from (ii) only by the price of the recorded F_p inversions (E27 validation)."),
        js("Operation counts only. The replay ran locally in the validator's 4-vCPU container, not on isolab-eks; no wall time is compared or claimed."),
    ]);
    let ok = ck.bad.is_empty() && hl.bad.is_empty();
    obj(vec![
        ("schema", js("validator-report/v1")),
        ("status", js(if ok { "pass" } else { "fail" })),
        ("independent", jb(true)),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E30; this recompute is its own standard-library Rust program (validate_e30.rs: JSON parser, SHA-256 and leaf comparator taken from the E27 validator's validate_e27.rs, the rest new) and shares no code with examples/e30_audit.rs, examples/e30_tables.rs, crypto_lib or experiments/35_invariants_lite/summary.json")),
        ("date", js("2026-10-31")),
        ("row", js("E30 (GLV note §9.28 pre-registration, §9.32 results): invariants-lite, an accounting audit of trivial F_p4 products on E27's 20 Karatsuba solver-A rows (E25's ten GLS curves, both arms) and the matched <±psi> walks. Class: accounting. Evidence directory research/e30_invariants_lite_20261030")),
        ("script", js("research/e30_invariants_lite_20261030/validate_e30.rs (rustc -O --edition 2021; `validate_e30 report <repo> research/e30_invariants_lite_20261030`)")),
        ("status_rule", js("pass iff every check holds and every headline agrees with its printed figure within half a unit of the last printed digit (exact for counts)")),
        ("raw_data", J::Arr(raw)),
        ("check_failures", J::Arr(ck.bad.iter().map(|s| js(s)).collect())),
        ("headline_disagreements", J::Arr(hl.bad.iter().map(|s| js(s)).collect())),
        ("checks", J::Arr(ck.items)),
        ("headline", J::Arr(hl.items)),
        ("rows", J::Arr(out_rows)),
        ("e30_against_e27", g110),
        ("gae_band", obj(vec![
            ("definition", js("IC: audited S = total_muls - ic_trivial_product_savings, + w * 97 * macaulay_entries; rho: E27's <±psi> walk (iii) mean - the walk audit's mean saving, + w * 97 * mean steps; GAE = 97 F_p multiplications (E25's affine addition), as E27's validation")),
            ("arm_b_medians", J::Arr(band_b)),
            ("all_20_rows", J::Arr(band_all)),
            ("order_holds_across_band", jb(order_holds)),
        ])),
        ("verdicts_recomputed", verdicts),
        ("errata", J::Arr(vec![obj(vec![("id", js("E1")), ("claim", js("§9.32: \"which here is ≈ 2 % of S, so about one in seven of its counted products is trivial\"")), ("recomputed", js(&format!("the F_p4-priced phases are {:.2}-{:.2} % of S and the saving is {:.3}-{:.3} of their multiplications; the product count is not emitted, so \"one in seven products\" cannot be checked from the records", 100.0 * flo, 100.0 * fhi, sflo, sfhi))), ("effect", js("none on a verdict or headline: explanatory prose"))])])),
        ("pre_registration", js("§9.28 committed b3ddccb13 (2026-10-30T19:01:45Z) before E30's code 0e4fccce1 (21:47:35Z) and records 22f560ff3/f8be1a0fe; its text at HEAD equals the registered text (one trailing blank line)")),
        ("limitations", limitations),
        ("comparator_controls", comparator_controls(ctx)),
    ])
}

fn certificate(ctx: &Ctx) -> J {
    let (cmp, hcmp) = ctx.read(&format!("{}/replay/replay_comparison.json", ctx.ev));
    let (build, hbuild) = ctx.read(&format!("{}/replay/build.json", ctx.ev));
    let (sw, _) = ctx.read(&format!("{E27}/stage_w.json"));
    let mut runs = Vec::new();
    let (mut arms_ok, mut arms, mut walks_ok, mut walks) = (0u64, 0u64, 0u64, 0u64);
    for r in cmp.get("runs").arr() {
        let name = r.get("run").s().to_string();
        let (rep, _) = ctx.read(r.get("b").get("path").s());
        let mut entry = vec![
            ("run", js(&name)),
            ("committed", r.get("a").clone()),
            ("replay_output", r.get("b").clone()),
            ("fields_compared", r.get("fields_compared").clone()),
            ("fields_identical", r.get("fields_identical").clone()),
            ("mismatches", r.get("mismatches").clone()),
            ("matched", r.get("identical").clone()),
        ];
        if name.starts_with("e2e_") {
            let mut sc = Vec::new();
            for x in rep.arr() {
                let (p, s) = (x.get("p").u(), x.get("seed").u());
                let c = sw.get("curves").arr().iter().find(|c| c.get("p").u() == p && c.get("seed").u() == s).unwrap();
                let d = c.get("folds").get("Gls").get("walks").get("batched").arr()[0].get("recovered").u();
                let fo = x.get("folded");
                arms += 1;
                let ok = fo.get("solved").b() && fo.get("correct").b() && fo.get("recovered").u() == d;
                if ok {
                    arms_ok += 1;
                }
                let mut wc = 0;
                for w in x.get("rho").arr() {
                    walks += 1;
                    if w.get("correct").b() && w.get("recovered").opt_u() == Some(d) {
                        walks_ok += 1;
                        wc += 1;
                    }
                }
                sc.push(obj(vec![("arm", x.get("arm").clone()), ("ic_solved_correct_equal_to_d", jb(ok)), ("d", ju(d)), ("walks_correct", ju(wc)), ("total_muls", x.get("total_muls").clone()), ("ic_trivial_product_savings", x.get("ic_trivial_product_savings").clone())]));
            }
            entry.push(("scalars", J::Arr(sc)));
            entry.push(("timing_not_compared", r.get("timing_not_compared").clone()));
        }
        runs.push(obj(entry));
    }
    let scalars = arms_ok == arms && arms == 20 && walks_ok == walks && walks == 960;
    let all_identical = cmp.get("all_identical").b();
    obj(vec![
        ("schema", js("replay-certificate/v1")),
        ("status", js(if all_identical && scalars { "verified" } else { "failed" })),
        ("independent", jb(true)),
        ("scalars_verified", jb(scalars)),
        ("date", js("2026-10-31")),
        ("row", js("E30 (GLV note §9.28/§9.32): the walk audit (10 curves x {<±psi>, negation} x 16 serial Karatsuba walks with the audit counter) and the ten audited end-to-end runs (E31 solver A with Karatsuba products and --audit, both arms); evidence directory research/e30_invariants_lite_20261030")),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E30: fresh build from an export of the recorded binary commit in a fresh target directory, fresh runs, own comparison (validate_e30.rs compare)")),
        ("recorded_binary", js("0e4fccce1ad1baf7ace18a550483771f02315838 (experiments/35_invariants_lite/binary_commit.txt)")),
        ("local_replay_note", js("Replayed locally in the validator's 4-vCPU container, not on isolab-eks: the replay admits operation counts only; no wall time is compared or claimed (wall_ms and solve_ms listed, not compared).")),
        ("build", build),
        ("build_record", obj(vec![("path", js("replay/build.json")), ("sha256", js(&hbuild))])),
        ("comparison", obj(vec![("path", js("replay/replay_comparison.json")), ("sha256", js(&hcmp)), ("rule", cmp.get("rule").clone())])),
        ("runs_total", cmp.get("runs_total").clone()),
        ("runs_matched", cmp.get("runs_identical").clone()),
        ("fields_compared", cmp.get("fields_compared").clone()),
        ("fields_identical", cmp.get("fields_identical").clone()),
        ("coverage", js("full: every recorded run replayed (the walk audit and all ten audited end-to-end runs, both arms each, 48 walks per arm); no sampling")),
        ("scalar_verification", js("Every replayed index-calculus arm (20) is solved and correct (Wiedemann logarithm checked against the planted d and [x]G = Q in the binary) and recovers E25's planted d as recorded by E27's walks; every in-process walk (960) is correct with the same d.")),
        ("runs", J::Arr(runs)),
    ])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: validate_e30 compare|report|certificate <repo> <evidence-dir>");
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
