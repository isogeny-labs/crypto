//! E27 validation (independent validator, 2026-10-30).
//!
//! An independent recompute of the headline numbers of
//! `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
//! §9.29 from the raw records in the repository, and a leaf-by-leaf
//! comparison of a fresh replay with the committed records.  Standard library
//! only (its own JSON parser and SHA-256); it shares no code with
//! `examples/e27_walk.rs`, `examples/e27_tables.rs` or `crypto_lib`.
//!
//! ```text
//! rustc -O --edition 2021 validate_e27.rs -o validate_e27
//! validate_e27 compare     <repo> <evidence-dir>   # -> replay/replay_comparison.json
//! validate_e27 report      <repo> <evidence-dir>   # -> validator-report.json
//! validate_e27 certificate <repo> <evidence-dir>   # -> replay-certificate.json
//! ```
//!
//! `<evidence-dir>` is relative to `<repo>` (here `research/e27_walk_20261030`).

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

// ------------------------------------------------------------ the data ----

const PS: [u64; 5] = [101, 149, 197, 269, 397];
const SEEDS: [u64; 2] = [1, 2];
const FOLDS: [&str; 2] = ["Gls", "Negation"];
const ARMS: [&str; 2] = ["A", "B"];
const ARITH: [&str; 3] = ["schoolbook", "karatsuba", "batched"];
const GAE: f64 = 97.0;
const WEIGHTS: [(&str, f64); 3] = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)];

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
}

fn stage_w_rel() -> String {
    "experiments/32_walk_karatsuba/stage_w.json".into()
}
fn e27_rel(p: u64, s: u64) -> String {
    format!("experiments/32_walk_karatsuba/e2e_chain_karatsuba_p{p}s{s}.json")
}
fn e31_rel(p: u64, s: u64) -> String {
    format!("experiments/31_solve_cost/e2e_chain_p{p}s{s}.json")
}
fn e25_rel(p: u64, s: u64) -> String {
    format!("experiments/30_gls_v4/p{p}s{s}.json")
}

#[derive(Clone, Debug, PartialEq)]
struct Walk {
    fold: String,
    seed: u64,
    steps: u64,
    gops: u64,
    muls: u64,
    esc: u64,
    correct: bool,
    rec: Option<u64>,
    digest: Option<u64>,
    vmuls: u64,
    vgops: u64,
}

impl Walk {
    fn of(j: &J) -> Walk {
        Walk {
            fold: j.get("fold").s().to_string(),
            seed: j.get("seed").u(),
            steps: j.get("steps").u(),
            gops: j.get("group_ops").u(),
            muls: j.get("fp_muls").u(),
            esc: j.get("cycle_escapes").u(),
            correct: j.get("correct").b(),
            rec: j.get("recovered").opt_u(),
            digest: j.get("visit_digest").opt_u(),
            vmuls: j.get("verify_fp_muls").opt_u().unwrap_or(0),
            vgops: j.get("verify_group_ops").opt_u().unwrap_or(0),
        }
    }
    /// E25's convention: the `[d]G = Q` check is not charged.
    fn walk_muls(&self) -> u64 {
        self.muls - self.vmuls
    }
    fn walk_gops(&self) -> u64 {
        self.gops - self.vgops
    }
    /// The fields the bit-identity claim (G2.95) is about.
    fn identity(&self) -> (u64, u64, u64, Option<u64>, bool, Option<u64>, u64, u64) {
        (
            self.seed,
            self.steps,
            self.gops,
            self.rec,
            self.correct,
            self.digest,
            self.esc,
            self.vgops,
        )
    }
}

fn walks(j: &J) -> Vec<Walk> {
    j.arr().iter().map(Walk::of).collect()
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

fn compare(ctx: &Ctx) -> J {
    let mut runs = Vec::new();
    let mut files: Vec<(String, String, String)> = vec![(
        "stage_w".into(),
        stage_w_rel(),
        format!("{}/replay/stage_w.json", ctx.ev),
    )];
    for p in PS {
        for s in SEEDS {
            files.push((
                format!("e2e_p{p}s{s}"),
                e27_rel(p, s),
                format!("{}/replay/e2e/e2e_chain_karatsuba_p{p}s{s}.json", ctx.ev),
            ));
        }
    }
    let (mut all_compared, mut all_identical, mut all_ok) = (0u64, 0u64, true);
    for (name, committed, replay) in files {
        let (a, ha) = ctx.read(&committed);
        let (b, hb) = ctx.read(&replay);
        let mut d = LeafDiff::default();
        leaves("$", &a, &b, &mut d);
        let ok = d.mismatches.is_empty() && d.only_a.is_empty() && d.only_b.is_empty();
        all_ok &= ok;
        all_compared += d.compared;
        all_identical += d.identical;
        runs.push(obj(vec![
            ("run", js(&name)),
            (
                "committed",
                obj(vec![("path", js(&committed)), ("sha256", js(&ha))]),
            ),
            (
                "replay",
                obj(vec![("path", js(&replay)), ("sha256", js(&hb))]),
            ),
            ("fields_compared", ju(d.compared)),
            ("fields_identical", ju(d.identical)),
            (
                "mismatches",
                J::Arr(
                    d.mismatches
                        .iter()
                        .map(|(p, x, y)| {
                            obj(vec![
                                ("path", js(p)),
                                ("committed", x.clone()),
                                ("replay", y.clone()),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "only_in_committed",
                J::Arr(d.only_a.iter().map(|p| js(p)).collect()),
            ),
            (
                "only_in_replay",
                J::Arr(d.only_b.iter().map(|p| js(p)).collect()),
            ),
            (
                "timing_not_compared",
                J::Arr(
                    d.timing
                        .iter()
                        .map(|(p, x, y)| {
                            obj(vec![
                                ("path", js(p)),
                                ("committed", x.clone()),
                                ("replay", y.clone()),
                            ])
                        })
                        .collect(),
                ),
            ),
            ("identical", jb(ok)),
        ]));
    }
    obj(vec![
        ("schema", js("e27-replay-comparison/v1")),
        ("tool", js(&format!("{}/validate_e27.rs compare", ctx.ev))),
        ("rule", js("every leaf of every committed record against the replay: integers and floats by their text (equal text, or equal value with the same integer/float form), strings and booleans exactly; wall_ms and solve_ms are wall-clock and listed, not compared")),
        ("runs_total", ju(runs.len() as u64)),
        ("runs_identical", ju(runs.iter().filter(|r| r.get("identical").b()).count() as u64)),
        ("fields_compared", ju(all_compared)),
        ("fields_identical", ju(all_identical)),
        ("all_identical", jb(all_ok)),
        ("runs", J::Arr(runs)),
    ])
}

// ----------------------------------------------------------- recompute ----

struct Row {
    p: u64,
    s: u64,
    arm: &'static str,
    s_a: f64,
    s_k: f64,
    e: f64,
    rho_e31: f64,
    rho_iii: f64,
    steps: f64,
    share_gls: f64,
    inv: f64,
}

impl Row {
    fn d_e31(&self, g: f64) -> f64 {
        (self.s_a + g * GAE * self.e) / (self.rho_e31 + g * GAE * self.steps)
    }
    fn d_e27(&self, g: f64) -> f64 {
        (self.s_k + g * GAE * self.e) / (self.rho_iii + g * GAE * self.steps)
    }
    fn ic_k(&self, g: f64) -> f64 {
        self.s_k + g * GAE * self.e
    }
    fn rho_k(&self, g: f64) -> f64 {
        self.rho_iii + g * GAE * self.steps
    }
}

fn report(ctx: &Ctx) -> J {
    let note = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.29";
    let mut raw: Vec<J> = Vec::new();
    let mut pin = |rel: &str, h: &str| {
        let depth = ctx.ev.split('/').count();
        raw.push(obj(vec![
            ("path", js(&format!("{}{}", "../".repeat(depth), rel))),
            ("sha256", js(h)),
        ]));
    };
    let mut checks: Vec<J> = Vec::new();
    let mut failures: Vec<String> = Vec::new();
    let mut check = |name: &str, ok: bool, detail: String| {
        if !ok {
            failures.push(format!("{name}: {detail}"));
        }
        checks.push(obj(vec![
            ("check", js(name)),
            ("holds", jb(ok)),
            ("detail", js(&detail)),
        ]));
    };

    // ---------------------------------------------------------- Stage W
    let (sw, hsw) = ctx.read(&stage_w_rel());
    pin(&stage_w_rel(), &hsw);
    let curves = sw.get("curves").arr();
    check(
        "stage W covers E25's ten curves, 16 walks per fold, pool 16",
        curves.len() == 10 && sw.get("walks_per_fold").u() == 16 && sw.get("pool").u() == 16,
        format!(
            "{} curves, walks_per_fold {}, pool {}",
            curves.len(),
            sw.get("walks_per_fold").u(),
            sw.get("pool").u()
        ),
    );
    let (mut identical3, mut total_walks, mut e25_match_a, mut e25_match_b, mut correct_all) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ratio_i_iii: Vec<(u64, u64, &str, f64)> = Vec::new();
    let mut ratio_i_ii: Vec<f64> = Vec::new();
    let mut shares: Vec<(u64, u64, &str, f64)> = Vec::new();
    let mut means_ok = true;
    let mut batch_identity_ok = true;
    let mut batch_cost_identity = 0u64;
    let mut flags_ok = true;
    let mut seeds_ok = true;
    let mut logs: Vec<(u64, u64, u64)> = Vec::new(); // (p, s, d)
    let mut fold_rows: Vec<J> = Vec::new();
    let mut gls_means: Vec<(u64, u64, f64, f64, f64, f64)> = Vec::new(); // p s mean_i mean_iii steps share
    let mut per_curve_n: Vec<(u64, u64, u64, u64)> = Vec::new();
    for c in curves {
        let (p, s) = (c.get("p").u(), c.get("seed").u());
        per_curve_n.push((p, s, c.get("n").u(), c.get("cofactor").u()));
        let (e25, h25) = ctx.read(&e25_rel(p, s));
        pin(&e25_rel(p, s), &h25);
        let e25a = walks(e25.arr()[0].get("rho"));
        let e25b = walks(e25.arr()[1].get("rho"));
        let mut d_curve: BTreeSet<Option<u64>> = BTreeSet::new();
        for fold in FOLDS {
            let fj = c.get("folds").get(fold);
            let ws: Vec<Vec<Walk>> = ARITH
                .iter()
                .map(|a| walks(fj.get("walks").get(a)))
                .collect();
            let expect_seeds: Vec<u64> = (0..16).map(|k| 1000 * s + k).collect();
            for w in &ws {
                seeds_ok &=
                    w.len() == 16 && w.iter().map(|x| x.seed).collect::<Vec<_>>() == expect_seeds;
                seeds_ok &= w.iter().all(|x| x.fold == fold);
            }
            let mut n_ident = 0u64;
            for k in 0..16 {
                total_walks += 1;
                let (a, b, cc) = (&ws[0][k], &ws[1][k], &ws[2][k]);
                if a.identity() == b.identity() && a.identity() == cc.identity() {
                    identical3 += 1;
                    n_ident += 1;
                }
                if a.correct && b.correct && cc.correct {
                    correct_all += 1;
                }
                for w in [a, b, cc] {
                    d_curve.insert(w.rec);
                }
                let matches = |rec: &[Walk]| {
                    rec.iter()
                        .filter(|r| r.fold == fold && r.seed == a.seed)
                        .count()
                        == 1
                        && rec.iter().any(|r| {
                            r.fold == fold
                                && r.seed == a.seed
                                && r.steps == a.steps
                                && r.gops == a.walk_gops()
                                && r.muls == a.walk_muls()
                                && r.esc == a.esc
                                && r.correct == a.correct
                        })
                };
                if matches(&e25a) {
                    e25_match_a += 1;
                }
                if matches(&e25b) {
                    e25_match_b += 1;
                }
            }
            let sums: Vec<u64> = ws
                .iter()
                .map(|w| w.iter().map(|x| x.walk_muls()).sum())
                .collect();
            let means: Vec<f64> = sums.iter().map(|&x| x as f64 / 16.0).collect();
            let vmeans: Vec<f64> = ws
                .iter()
                .map(|w| w.iter().map(|x| x.vmuls).sum::<u64>() as f64 / 16.0)
                .collect();
            let steps = ws[0].iter().map(|x| x.steps).sum::<u64>() as f64 / 16.0;
            let gops = ws[0].iter().map(|x| x.walk_gops()).sum::<u64>() as f64 / 16.0;
            for (k, a) in ARITH.iter().enumerate() {
                means_ok &= fj.get("mean_fp_muls").get(a).f() == means[k];
                means_ok &= fj.get("mean_verify_fp_muls").get(a).f() == vmeans[k];
            }
            means_ok &= fj.get("mean_steps").f() == steps && fj.get("mean_group_ops").f() == gops;
            flags_ok &= fj.get("replay_matches_e25").b()
                && fj.get("karatsuba_identical").b()
                && fj.get("batched_identical").b();
            flags_ok &= fj.get("e25_records").u() == 16;
            let r13 = sums[0] as f64 / sums[2] as f64;
            ratio_i_iii.push((p, s, fold, r13));
            ratio_i_ii.push(sums[0] as f64 / sums[1] as f64);
            let bt = fj.get("batch");
            let (inv, bm, rinv, rbm, rounds) = (
                bt.get("inversions").u(),
                bt.get("batch_muls").u(),
                bt.get("recorded_inversions").u(),
                bt.get("recorded_batch_muls").u(),
                bt.get("rounds").u(),
            );
            // Batched (iii) differs from (ii) only by the price of the recorded
            // walks' F_p inversions: 16 each unbatched, their batch share batched.
            let kara_total: u64 = ws[1].iter().map(|x| x.muls).sum();
            let batch_total: u64 = ws[2].iter().map(|x| x.muls).sum();
            let ident = kara_total - batch_total == 16 * rinv - rbm;
            batch_identity_ok &= ident;
            if bm == 3 * inv + 13 * rounds {
                batch_cost_identity += 1;
            }
            let share = rbm as f64 / rinv as f64;
            means_ok &= fj.get("inversion_share").f() == share;
            shares.push((p, s, fold, share));
            if fold == "Gls" {
                gls_means.push((p, s, means[0], means[2], steps, share));
            }
            fold_rows.push(obj(vec![
                ("p", ju(p)),
                ("seed", ju(s)),
                ("fold", js(fold)),
                ("walks_identical_3_ways", ju(n_ident)),
                ("mean_walk_fp_muls_i", jf(means[0])),
                ("mean_walk_fp_muls_ii", jf(means[1])),
                ("mean_walk_fp_muls_iii", jf(means[2])),
                (
                    "mean_verify_fp_muls_i_ii_iii",
                    J::Arr(vmeans.iter().map(|&v| jf(v)).collect()),
                ),
                ("mean_steps", jf(steps)),
                ("mean_walk_group_ops", jf(gops)),
                ("ratio_i_over_iii", jf(r13)),
                ("ratio_i_over_ii", jf(sums[0] as f64 / sums[1] as f64)),
                ("recorded_inversions", ju(rinv)),
                ("recorded_batch_muls", ju(rbm)),
                ("inversion_share", jf(share)),
                ("batched_addition_fp_muls", jf(79.0 - 16.0 + share)),
                ("kara_minus_batched_total_eq_16inv_minus_share", jb(ident)),
                (
                    "pool_batch_muls_eq_3inv_plus_13rounds",
                    jb(bm == 3 * inv + 13 * rounds),
                ),
                ("filler_walks", ju(bt.get("filler_walks").u())),
            ]));
        }
        let ds: Vec<Option<u64>> = d_curve.into_iter().collect();
        if ds.len() == 1 && ds[0].is_some() {
            logs.push((p, s, ds[0].unwrap()));
        } else {
            logs.push((p, s, u64::MAX));
        }
    }
    check(
        "every walk uses E25's seeds 1000*seed + k, k < 16, in order, for its fold",
        seeds_ok,
        format!("{seeds_ok}"),
    );
    check("320 walks identical under (i), (ii), (iii) (seed, steps, group ops, recovered, correct, visit digest, cycle escapes, verify group ops)", identical3 == 320 && total_walks == 320, format!("{identical3} of {total_walks}"));
    check("every walk under every arithmetic is correct (recovered = planted d and [d]G = Q in the producing binary)", correct_all == 320, format!("{correct_all} of 320 walk triples"));
    check("(i) equals E25's arm-A walk record field for field (steps, group ops and F_p mults without the [d]G check, cycle escapes, correct)", e25_match_a == 320, format!("{e25_match_a} of 320"));
    check(
        "(i) equals E25's arm-B walk record field for field (E25 records each walk in both arms)",
        e25_match_b == 320,
        format!("{e25_match_b} of 320"),
    );
    check(
        "one logarithm per curve across all 96 walks",
        logs.iter().all(|x| x.2 != u64::MAX),
        format!("{:?}", logs),
    );
    // The planted secrets were checked independently by E25's validator
    // (its own arithmetic: G, Q on the curve, n prime, nG = O, dG = Q, ...).
    let e25cert_rel = "benchmarks/20261018-k4-gls-v4/replay-certificate.json";
    let (e25cert, hcert) = ctx.read(e25cert_rel);
    pin(e25cert_rel, &hcert);
    let mut planted_ok = 0u64;
    for ic in e25cert.get("instance_checks").arr() {
        let (p, s) = (ic.get("p").u(), ic.get("seed").u());
        let n_ok = per_curve_n.iter().any(|x| {
            x.0 == p && x.1 == s && x.2 == ic.get("n").u() && x.3 == ic.get("cofactor").u()
        });
        let d_ok = logs
            .iter()
            .any(|x| x.0 == p && x.1 == s && x.2 == ic.get("planted_d").u());
        if n_ok && d_ok && ic.get("all_checks_pass").b() && ic.get("experiment").s() == "e25" {
            planted_ok += 1;
        }
    }
    check("the recovered logarithm equals the planted d that E25's independent validator checked (dG = Q in its own arithmetic), with equal n and cofactor, on all ten curves", planted_ok == 10, format!("{planted_ok} of 10 ({e25cert_rel})"));
    check("recorded means, verify means, mean steps/group ops and inversion shares equal the recompute from the per-walk records", means_ok, format!("{means_ok}"));
    check("recorded flags replay_matches_e25, karatsuba_identical, batched_identical true and e25_records = 16", flags_ok, format!("{flags_ok}"));
    check("per pool: sum (ii) - sum (iii) = 16 * recorded_inversions - recorded_batch_muls (only the inversion price differs)", batch_identity_ok, format!("{batch_identity_ok} on 20 pools"));
    check("per pool: batch_muls = 3 * inversions + 13 * rounds (3(m-1)+16 per round, every round non-empty)", batch_cost_identity == 20, format!("{batch_cost_identity} of 20 pools"));

    // Per-operation costs, recorded on the counters by the binary, against the
    // costs derived from the code: Fp4::inv = 24 + 16, add = inv + 3 mul,
    // doubling = inv + 4 mul, psi = 2 mul.
    let po = sw.get("per_op_costs");
    let mut per_op_ok = true;
    for (arith, mul) in [("schoolbook", 19u64), ("karatsuba", 13u64)] {
        let o = po.get(arith);
        per_op_ok &= o.get("fp4_mul").u() == mul
            && o.get("fp4_inv_unbatched").u() == 40
            && o.get("add_distinct_unbatched").u() == 40 + 3 * mul
            && o.get("double_unbatched").u() == 40 + 4 * mul
            && o.get("psi_gls").u() == 2 * mul;
    }
    check("per-op costs: mul 19/13, inv 40, add 40+3mul, doubling 40+4mul, psi 2mul (derived from gaudry_quartic.rs and equal to the counters)", per_op_ok, format!("{po:?}").chars().take(300).collect());

    // ------------------------------------------------------- IC side
    let mut rows: Vec<Row> = Vec::new();
    let mut s_ratio: Vec<f64> = Vec::new();
    let mut nonsolve: Vec<f64> = Vec::new();
    let mut ic_inv: Vec<f64> = Vec::new();
    let mut bound13: Vec<f64> = Vec::new();
    let mut bound16: Vec<f64> = Vec::new();
    let mut diff_paths: BTreeSet<String> = BTreeSet::new();
    let mut added_paths: BTreeSet<String> = BTreeSet::new();
    let mut removed_paths: BTreeSet<String> = BTreeSet::new();
    let (mut ic_fields_compared, mut ic_fields_identical) = (0u64, 0u64);
    let mut ic_ok = true;
    let mut ic_log_ok = true;
    let mut e2e_walks_eq_stage_w = 0u64;
    let mut e2e_walks_total = 0u64;
    let mut e2e_rho_field_incl_verify = true;
    let mut e2e_field_factor: Vec<f64> = Vec::new();
    let mut instance_ok = true;
    let mut ic_rows_json: Vec<J> = Vec::new();
    for p in PS {
        for s in SEEDS {
            let (e27, h27) = ctx.read(&e27_rel(p, s));
            let (e31, h31) = ctx.read(&e31_rel(p, s));
            let (e25, _) = ctx.read(&e25_rel(p, s));
            pin(&e27_rel(p, s), &h27);
            pin(&e31_rel(p, s), &h31);
            let c = curves
                .iter()
                .find(|c| c.get("p").u() == p && c.get("seed").u() == s)
                .unwrap();
            let d = logs.iter().find(|x| x.0 == p && x.1 == s).unwrap().2;
            let gm = gls_means.iter().find(|x| x.0 == p && x.1 == s).unwrap();
            let kara_gls = walks(c.get("folds").get("Gls").get("walks").get("karatsuba"));
            let kara_neg = walks(c.get("folds").get("Negation").get("walks").get("karatsuba"));
            for (ai, arm) in ARMS.iter().enumerate() {
                let r27 = &e27.arr()[ai];
                let r31 = &e31.arr()[ai];
                let r25 = &e25.arr()[ai];
                ic_ok &= r27.get("arm").s() == *arm
                    && r31.get("arm").s() == *arm
                    && r25.get("arm").s() == *arm;
                ic_ok &= r27.get("solver").s() == "chain" && r31.get("solver").s() == "chain";
                ic_ok &= r27.get("fp4_product").s() == "karatsuba";
                ic_ok &= r27.get("p").u() == p && r27.get("seed").u() == s;
                for k in ["n", "group_order", "cofactor", "base_points"] {
                    instance_ok &= r27.get(k) == r31.get(k) && r27.get(k) == r25.get(k);
                }
                instance_ok &= r27.get("n").u() == c.get("n").u()
                    && r27.get("cofactor").u() == c.get("cofactor").u();
                // Scalars: the index calculus's own logarithm equals the walks'.
                let fo = r27.get("folded");
                ic_log_ok &=
                    fo.get("solved").b() && fo.get("correct").b() && fo.get("recovered").u() == d;
                ic_log_ok &= r31.get("folded").get("correct").b();
                // Every leaf of E31's record against E27's.
                let mut ld = LeafDiff::default();
                leaves("$", r31, r27, &mut ld);
                ic_fields_compared += ld.compared;
                ic_fields_identical += ld.identical;
                for (pth, _, _) in &ld.mismatches {
                    diff_paths.insert(generic(pth));
                }
                for pth in &ld.only_b {
                    added_paths.insert(generic(pth));
                }
                for pth in &ld.only_a {
                    removed_paths.insert(generic(pth));
                }
                // The totals.
                let comp = |r: &J| {
                    [
                        "generation_muls",
                        "base_muls",
                        "precompute_muls",
                        "weil_muls",
                        "lift_muls",
                        "check_muls",
                        "solve_muls",
                        "la_fp_muls",
                    ]
                    .iter()
                    .map(|k| r.get(k).u())
                    .sum::<u64>()
                };
                ic_ok &= comp(r27) == r27.get("total_muls").u()
                    && comp(r31) == r31.get("total_muls").u();
                ic_ok &= r27.get("solve_muls") == r31.get("solve_muls")
                    && r27.get("la_fp_muls") == r31.get("la_fp_muls");
                ic_ok &= r27.get("macaulay_entries") == r31.get("macaulay_entries");
                ic_ok &= r31.get("fp_muls_per_add").f() == 97.0
                    && r27.get("fp_muls_per_add").f() == 79.0;
                let s_a = r31.get("total_muls").u() as f64;
                let s_k = r27.get("total_muls").u() as f64;
                let e = r31.get("macaulay_entries").u() as f64;
                let solve_la = (r31.get("solve_muls").u() + r31.get("la_fp_muls").u()) as f64;
                s_ratio.push(s_a / s_k);
                nonsolve.push((s_a - solve_la) / s_a);
                let inv = r27.get("ic_fp_inversions").u() as f64;
                ic_inv.push(inv);
                bound13.push(13.0 * inv / s_k);
                bound16.push(16.0 * inv / s_k);
                // E31's matched rho is E25's walk (i) without the check; its steps are walk (i)'s.
                ic_ok &= r31.get("rho_matched_muls").f() == gm.2;
                ic_ok &= r31.get("rho_matched_steps").f() == gm.4;
                ic_ok &= r27.get("rho_matched_steps").f() == gm.4;
                // The pipeline process's own walks are Stage W's walk (ii), check included.
                let pw = walks(r27.get("rho"));
                for (fold, sw_w) in [("Gls", &kara_gls), ("Negation", &kara_neg)] {
                    for w in sw_w.iter() {
                        e2e_walks_total += 1;
                        if pw
                            .iter()
                            .filter(|x| x.fold == fold && x.seed == w.seed)
                            .count()
                            == 1
                            && pw.iter().any(|x| x == w)
                        {
                            e2e_walks_eq_stage_w += 1;
                        }
                    }
                }
                let incl: f64 = kara_gls.iter().map(|w| w.muls).sum::<u64>() as f64 / 16.0;
                e2e_rho_field_incl_verify &= r27.get("rho_matched_muls").f() == incl;
                e2e_field_factor.push(r27.get("rho_matched_muls").f() / gm.3);
                let row = Row {
                    p,
                    s,
                    arm,
                    s_a,
                    s_k,
                    e,
                    rho_e31: gm.2,
                    rho_iii: gm.3,
                    steps: gm.4,
                    share_gls: gm.5,
                    inv,
                };
                ic_rows_json.push(obj(vec![
                    ("p", ju(p)),
                    ("seed", ju(s)),
                    ("arm", js(arm)),
                    ("S_A_E31", jf(s_a)),
                    ("S_AK_E27", jf(s_k)),
                    ("S_A_over_S_AK", jf(s_a / s_k)),
                    ("non_solve_share_of_S_A", jf((s_a - solve_la) / s_a)),
                    ("macaulay_entries", jf(e)),
                    ("ic_fp_inversions", jf(inv)),
                    ("rho_E31_walk_i", jf(gm.2)),
                    ("rho_E27_walk_iii", jf(gm.3)),
                    ("rho_steps", jf(gm.4)),
                    (
                        "band",
                        J::Obj(
                            WEIGHTS
                                .iter()
                                .map(|(k, g)| {
                                    (
                                        k.to_string(),
                                        obj(vec![
                                            ("ic_E27", jf(row.ic_k(*g))),
                                            ("rho_E27", jf(row.rho_k(*g))),
                                            ("S_over_rho_E31", jf(row.d_e31(*g))),
                                            ("S_over_rho_E27", jf(row.d_e27(*g))),
                                            ("growth", jf(row.d_e27(*g) / row.d_e31(*g))),
                                        ]),
                                    )
                                })
                                .collect(),
                        ),
                    ),
                ]));
                rows.push(row);
            }
        }
    }
    check("E27 and E31 records: arms, solver chain, fp4_product karatsuba, p, seed", ic_ok, format!("{ic_ok} (also: total = sum of the eight phase counts on both sides; solve_muls, la_fp_muls, macaulay_entries equal; E31 rho_matched = walk (i) mean; fp_muls_per_add 97 / 79)"));
    check("instance identity: n, group_order, cofactor, base_points equal in E25, E31, E27 and Stage W", instance_ok, format!("{instance_ok}"));
    check("index-calculus logarithm (Wiedemann) equals the walks' logarithm on every row; solved and correct", ic_log_ok, format!("{ic_log_ok} on 20 rows"));
    check("the pipeline process's own <±psi> and negation walks equal Stage W's walk (ii) record for record (cost included)", e2e_walks_eq_stage_w == e2e_walks_total, format!("{e2e_walks_eq_stage_w} of {e2e_walks_total}"));
    check("disclosure: the E27 e2e records' rho_matched_muls is walk (ii) WITH the [d]G check, so their `ratio` field is not the E27 distance", e2e_rho_field_incl_verify, format!("{e2e_rho_field_incl_verify}"));
    let allowed_diff: BTreeSet<&str> = [
        "$.total_muls",
        "$.ratio",
        "$.generation_muls",
        "$.base_muls",
        "$.precompute_muls",
        "$.weil_muls",
        "$.lift_muls",
        "$.check_muls",
        "$.fp_muls_per_add",
        "$.call.max",
        "$.call.mean",
        "$.call.median",
        "$.call.min",
        "$.call.p10",
        "$.call.p90",
        "$.rho_matched_muls",
        "$.rho_negation_muls",
        "$.rho_unfolded_muls",
        "$.rho[*].fp_muls",
        "$.rho[*].group_ops",
    ]
    .into_iter()
    .collect();
    let unexpected: Vec<&String> = diff_paths
        .iter()
        .filter(|p| !allowed_diff.contains(p.as_str()))
        .collect();
    check(
        "every other field of E31 A's records reproduced: differing leaves are only multiplication counts and what derives from them, walk group ops (the [d]G check, now charged), and wall times",
        unexpected.is_empty() && removed_paths.is_empty(),
        format!("{ic_fields_identical} of {ic_fields_compared} compared leaves identical; differing: {diff_paths:?}; added in E27: {added_paths:?}; unexpected: {unexpected:?}; missing in E27: {removed_paths:?}"),
    );

    // ---------------------------------------------------------- headlines
    let mut h = Headlines {
        items: Vec::new(),
        bad: Vec::new(),
    };
    let po_s = po.get("schoolbook");
    let po_k = po.get("karatsuba");
    h.add(
        "Fp4::mul schoolbook",
        note,
        "19",
        19.0,
        0.0,
        po_s.get("fp4_mul").u() as f64,
        "per_op_costs.schoolbook.fp4_mul (counter)",
    );
    h.add(
        "Fp4::mul Karatsuba",
        note,
        "13",
        13.0,
        0.0,
        po_k.get("fp4_mul").u() as f64,
        "per_op_costs.karatsuba.fp4_mul (counter)",
    );
    h.add(
        "affine addition, schoolbook (E25 reference)",
        note,
        "97",
        97.0,
        0.0,
        po_s.get("add_distinct_unbatched").u() as f64,
        "per_op_costs.schoolbook.add_distinct_unbatched",
    );
    h.add(
        "affine addition, Karatsuba unbatched",
        note,
        "79",
        79.0,
        0.0,
        po_k.get("add_distinct_unbatched").u() as f64,
        "per_op_costs.karatsuba.add_distinct_unbatched",
    );
    h.add(
        "doubling, schoolbook",
        note,
        "116",
        116.0,
        0.0,
        po_s.get("double_unbatched").u() as f64,
        "per_op_costs.schoolbook.double_unbatched",
    );
    h.add(
        "doubling, Karatsuba unbatched",
        note,
        "92",
        92.0,
        0.0,
        po_k.get("double_unbatched").u() as f64,
        "per_op_costs.karatsuba.double_unbatched",
    );
    h.add(
        "GLS map psi, schoolbook",
        note,
        "38",
        38.0,
        0.0,
        po_s.get("psi_gls").u() as f64,
        "per_op_costs.schoolbook.psi_gls",
    );
    h.add(
        "GLS map psi, Karatsuba",
        note,
        "26",
        26.0,
        0.0,
        po_k.get("psi_gls").u() as f64,
        "per_op_costs.karatsuba.psi_gls",
    );
    let sh: Vec<f64> = shares.iter().map(|x| x.3).collect();
    let (sh_lo, sh_hi) = minmax(&sh);
    h.add(
        "norm-inversion share per inversion, min over 20 pools",
        note,
        "3.27",
        3.27,
        0.005,
        sh_lo,
        "recorded_batch_muls / recorded_inversions per (curve, fold) pool",
    );
    h.add(
        "norm-inversion share per inversion, max over 20 pools",
        note,
        "3.75",
        3.75,
        0.005,
        sh_hi,
        "recorded_batch_muls / recorded_inversions per (curve, fold) pool",
    );
    h.add("affine addition, batched (iii), min", note, "66.3", 66.3, 0.05, 79.0 - 16.0 + sh_lo, "Karatsuba add 79 with the 16 of its F_p inversion replaced by the pool's share (identity checked per pool)");
    h.add(
        "affine addition, batched (iii), max",
        note,
        "66.7",
        66.7,
        0.05,
        79.0 - 16.0 + sh_hi,
        "as above",
    );
    h.add(
        "walks identical under (i), (ii), (iii)",
        note,
        "320",
        320.0,
        0.0,
        identical3 as f64,
        "count of walks whose identity fields agree three ways",
    );
    h.add(
        "(i) reproduces E25's walk records field for field",
        note,
        "320",
        320.0,
        0.0,
        e25_match_a as f64,
        "count of (i) walks equal to E25's arm-A record (E25 convention)",
    );
    let all13: Vec<f64> = ratio_i_iii.iter().map(|x| x.3).collect();
    let gls13: Vec<f64> = ratio_i_iii
        .iter()
        .filter(|x| x.2 == "Gls")
        .map(|x| x.3)
        .collect();
    let neg13: Vec<f64> = ratio_i_iii
        .iter()
        .filter(|x| x.2 == "Negation")
        .map(|x| x.3)
        .collect();
    let (a_lo, a_hi) = minmax(&all13);
    let (g_lo, g_hi) = minmax(&gls13);
    let (n_lo, n_hi) = minmax(&neg13);
    h.add(
        "walk cost (i)/(iii), min over both walks and ten curves",
        note,
        "1.4533",
        1.4533,
        0.00005,
        a_lo,
        "sum over 16 walks of (fp_muls - verify_fp_muls), (i) over (iii)",
    );
    h.add(
        "walk cost (i)/(iii), max over both walks and ten curves",
        note,
        "1.4634",
        1.4634,
        0.00005,
        a_hi,
        "as above",
    );
    h.add(
        "<±psi> walk cost (i)/(iii), min",
        note,
        "1.4559",
        1.4559,
        0.00005,
        g_lo,
        "Gls fold only",
    );
    h.add(
        "<±psi> walk cost (i)/(iii), max",
        note,
        "1.4634",
        1.4634,
        0.00005,
        g_hi,
        "Gls fold only",
    );
    h.add(
        "negation walk cost (i)/(iii), min",
        note,
        "1.4533",
        1.4533,
        0.00005,
        n_lo,
        "Negation fold only",
    );
    h.add(
        "negation walk cost (i)/(iii), max",
        note,
        "1.4621",
        1.4621,
        0.00005,
        n_hi,
        "Negation fold only",
    );
    // (i)/(ii) is printed as "≈ 1.27" for both walks; it is an erratum, not a headline.
    let gls12: Vec<f64> = ratio_i_ii
        .iter()
        .zip(&ratio_i_iii)
        .filter(|(_, x)| x.2 == "Gls")
        .map(|(v, _)| *v)
        .collect();
    let neg12: Vec<f64> = ratio_i_ii
        .iter()
        .zip(&ratio_i_iii)
        .filter(|(_, x)| x.2 == "Negation")
        .map(|(v, _)| *v)
        .collect();
    let (g12_lo, g12_hi) = minmax(&gls12);
    let (n12_lo, n12_hi) = minmax(&neg12);
    // The [d]G = Q check main's 652238c03 began charging inside every walk.
    let c101 = curves
        .iter()
        .find(|c| c.get("p").u() == 101 && c.get("seed").u() == 1)
        .unwrap();
    let w101 = walks(c101.get("folds").get("Gls").get("walks").get("schoolbook"));
    let vg: BTreeSet<u64> = w101.iter().map(|w| w.vgops).collect();
    let vm: BTreeSet<u64> = w101.iter().map(|w| w.vmuls).collect();
    h.add(
        "[d]G check at p=101 seed 1: group operations per <±psi> walk",
        note,
        "+31",
        31.0,
        0.0,
        if vg.len() == 1 {
            *vg.iter().next().unwrap() as f64
        } else {
            -1.0
        },
        "verify_group_ops of every (i) walk (one value)",
    );
    h.add(
        "[d]G check at p=101 seed 1: F_p multiplications per <±psi> walk",
        note,
        "+3,309",
        3309.0,
        0.0,
        if vm.len() == 1 {
            *vm.iter().next().unwrap() as f64
        } else {
            -1.0
        },
        "verify_fp_muls of every (i) walk (one value)",
    );
    let vshare: Vec<f64> = curves
        .iter()
        .map(|c| {
            let w = walks(c.get("folds").get("Gls").get("walks").get("schoolbook"));
            w.iter().map(|x| x.vmuls).sum::<u64>() as f64
                / w.iter().map(|x| x.walk_muls()).sum::<u64>() as f64
        })
        .collect();
    let (vs_lo, vs_hi) = minmax(&vshare);
    h.add(
        "[d]G check, % of a <±psi> walk, min over the ten curves",
        note,
        "0.13",
        0.13,
        0.005,
        100.0 * vs_lo,
        "sum verify_fp_muls / sum (fp_muls - verify_fp_muls), walk (i), per curve",
    );
    h.add(
        "[d]G check, % of a <±psi> walk, max over the ten curves",
        note,
        "0.78",
        0.78,
        0.005,
        100.0 * vs_hi,
        "as above",
    );
    let (sr_lo, sr_hi) = minmax(&s_ratio);
    h.add(
        "S_A / S_A,K, min over 20 rows",
        note,
        "1.00475",
        1.00475,
        0.000005,
        sr_lo,
        "E31 total_muls over E27 total_muls, same (p, seed, arm)",
    );
    h.add(
        "S_A / S_A,K, max over 20 rows",
        note,
        "1.00566",
        1.00566,
        0.000005,
        sr_hi,
        "as above",
    );
    let (ns_lo, ns_hi) = minmax(&nonsolve);
    h.add(
        "non-solve F_p4 work, % of S_A, min",
        note,
        "2.00",
        2.00,
        0.005,
        100.0 * ns_lo,
        "(generation + base + precompute + weil + lift + check) / total_muls on E31's records",
    );
    h.add(
        "non-solve F_p4 work, % of S_A, max",
        note,
        "2.48",
        2.48,
        0.005,
        100.0 * ns_hi,
        "as above",
    );
    let (iv_lo, iv_hi) = minmax(&ic_inv);
    h.add(
        "IC-side F_p inversions per run, min",
        note,
        "4.7e5",
        4.7e5,
        0.05e5,
        iv_lo,
        "ic_fp_inversions",
    );
    h.add(
        "IC-side F_p inversions per run, max",
        note,
        "2.6e7",
        2.6e7,
        0.05e7,
        iv_hi,
        "ic_fp_inversions",
    );
    let (b13_lo, b13_hi) = minmax(&bound13);
    let (b16_lo, b16_hi) = minmax(&bound16);
    // The printed figure reproduces with the saving (16 - that curve's <±psi>
    // pool share) per inversion; see errata for why it is not an upper bound.
    let bshare: Vec<f64> = rows
        .iter()
        .map(|r| (16.0 - r.share_gls) * r.inv / r.s_k)
        .collect();
    let (bs_lo, bs_hi) = minmax(&bshare);
    h.add("saving from batching the IC side's inversions, % of S, min", note, "0.127", 0.127, 0.0005, 100.0 * bs_lo, "(16 - the curve's <±psi> pool share) * ic_fp_inversions / S_A,K (the definition the printed figure uses; errata E2)");
    h.add(
        "saving from batching the IC side's inversions, % of S, max",
        note,
        "0.170",
        0.170,
        0.0005,
        100.0 * bs_hi,
        "as above",
    );
    for (k, g) in WEIGHTS {
        let d31: Vec<f64> = rows.iter().map(|r| r.d_e31(g)).collect();
        let d27: Vec<f64> = rows.iter().map(|r| r.d_e27(g)).collect();
        let gr: Vec<f64> = rows.iter().map(|r| r.d_e27(g) / r.d_e31(g)).collect();
        let (lo31, hi31) = minmax(&d31);
        let (lo27, hi27) = minmax(&d27);
        let (glo, ghi) = minmax(&gr);
        let printed: [(&str, f64, f64, &str, f64, f64, &str, f64, &str, f64); 3] = [
            (
                "4.42e3", 4.42e3, 0.005e3, "2.79e5", 2.79e5, 0.005e5, "6.39e3", 6.39e3, "4.06e5",
                4.06e5,
            ),
            (
                "1.50e4", 1.50e4, 0.005e4, "9.77e5", 9.77e5, 0.005e5, "2.13e4", 2.13e4, "1.39e6",
                1.39e6,
            ),
            (
                "7.33e4", 7.33e4, 0.005e4, "5.22e6", 5.22e6, 0.005e6, "9.07e4", 9.07e4, "6.64e6",
                6.64e6,
            ),
        ];
        let growth_printed = [
            ("1.4477", 1.4477, "1.4564", 1.4564),
            ("1.4150", 1.4150, "1.4427", 1.4427),
            ("1.2364", 1.2364, "1.3252", 1.3252),
        ];
        let i = WEIGHTS.iter().position(|w| w.0 == k).unwrap();
        let pr = printed[i];
        let gp = growth_printed[i];
        let unit = |v: f64| 0.5 * 10f64.powi(v.log10().floor() as i32 - 2);
        let def31 = format!("(S_A + {k} * 97 * macaulay_entries) / (rho_E31 + {k} * 97 * steps), E31 solver-A records against E25's walk (i)");
        let def27 = format!("(S_A,K + {k} * 97 * macaulay_entries) / (rho_iii + {k} * 97 * steps), E27 records against Stage W walk (iii), E25 convention");
        h.add(
            &format!("S/rho E31 A at {k} GAE, min"),
            note,
            pr.0,
            pr.1,
            pr.2,
            lo31,
            &def31,
        );
        h.add(
            &format!("S/rho E31 A at {k} GAE, max"),
            note,
            pr.3,
            pr.4,
            pr.5,
            hi31,
            &def31,
        );
        h.add(
            &format!("S/rho E27 at {k} GAE, min"),
            note,
            pr.6,
            pr.7,
            unit(pr.7),
            lo27,
            &def27,
        );
        h.add(
            &format!("S/rho E27 at {k} GAE, max"),
            note,
            pr.8,
            pr.9,
            unit(pr.9),
            hi27,
            &def27,
        );
        h.add(
            &format!("growth of S/rho per row at {k} GAE, min"),
            note,
            gp.0,
            gp.1,
            0.00005,
            glo,
            "E27 S/rho over E31 S/rho, same row",
        );
        h.add(
            &format!("growth of S/rho per row at {k} GAE, max"),
            note,
            gp.2,
            gp.3,
            0.00005,
            ghi,
            "as above",
        );
    }
    // The rounded forms quoted in G2.99's verdict.
    for (k, g, lo_p, lo, hi_p, hi) in [
        ("0", 0.0, "1.448", 1.448, "1.456", 1.456),
        ("0.1", 0.1, "1.415", 1.415, "1.443", 1.443),
        ("1", 1.0, "1.236", 1.236, "1.325", 1.325),
    ] {
        let gr: Vec<f64> = rows.iter().map(|r| r.d_e27(g) / r.d_e31(g)).collect();
        let (glo, ghi) = minmax(&gr);
        h.add(
            &format!("G2.99 verdict growth at {k} GAE, min (rounded)"),
            note,
            lo_p,
            lo,
            0.0005,
            glo,
            "as above, three decimals",
        );
        h.add(
            &format!("G2.99 verdict growth at {k} GAE, max (rounded)"),
            note,
            hi_p,
            hi,
            0.0005,
            ghi,
            "as above, three decimals",
        );
    }
    let best = rows
        .iter()
        .find(|r| r.p == 397 && r.s == 1 && r.arm == "B")
        .unwrap();
    h.add(
        "best row p=397 s1 arm B, S/rho at 0 GAE, E31",
        note,
        "4.42e3",
        4.42e3,
        0.005e3,
        best.d_e31(0.0),
        "that row",
    );
    h.add(
        "best row p=397 s1 arm B, S/rho at 0 GAE, E27",
        note,
        "6.39e3",
        6.39e3,
        0.005e3,
        best.d_e27(0.0),
        "that row",
    );
    let p101s2: Vec<f64> = rows
        .iter()
        .filter(|r| r.p == 101 && r.s == 2)
        .map(|r| r.d_e27(1.0) / r.d_e31(1.0))
        .collect();
    let (q_lo, _q_hi) = minmax(&p101s2);
    h.add(
        "growth at 1 GAE on the p=101 seed 2 rows (both above G2.99's 1.30)",
        note,
        "1.3252",
        1.3252,
        0.00005,
        q_lo,
        "min over the two rows",
    );

    // G2.99 band per row and "rho ahead of every row at every weight".
    let bands = [(0.0, 1.37, 1.50), (0.1, 1.30, 1.48), (1.0, 1.12, 1.30)];
    let mut outside: Vec<String> = Vec::new();
    let mut min_dist = f64::INFINITY;
    for r in &rows {
        for (g, lo, hi) in bands {
            let gr = r.d_e27(g) / r.d_e31(g);
            if gr < lo || gr > hi {
                outside.push(format!(
                    "p={} s{} arm {} at {} GAE: {:.4} (band [{lo}, {hi}])",
                    r.p, r.s, r.arm, g, gr
                ));
            }
            min_dist = min_dist.min(r.d_e27(g)).min(r.d_e31(g));
        }
    }
    let falsified_as_reported = outside.len() == 2
        && outside
            .iter()
            .all(|x| x.starts_with("p=101 s2") && x.contains("at 1 GAE"));
    check("G2.99: rows outside the registered growth bands are exactly the two p=101 seed 2 rows at 1 GAE (falsified high), as reported", falsified_as_reported, format!("{outside:?}"));
    check("rho ahead of every row at every weight (min S/rho over 20 rows x 3 weights, both references)", min_dist > 1.0, format!("min S/rho {min_dist}"));
    // G2.96-G2.98 registered bands.
    let g96 = sh_lo + 63.0 >= 64.0 && sh_hi + 63.0 <= 70.0 && a_lo >= 1.38 && a_hi <= 1.50;
    check("G2.96: Fp4::mul = 13; batched distinct addition in [64, 70]; per group op (i)/(iii) in [1.38, 1.50]", g96 && po_k.get("fp4_mul").u() == 13, format!("addition {:.3}-{:.3}, ratio {a_lo:.4}-{a_hi:.4}", 63.0 + sh_lo, 63.0 + sh_hi));
    check(
        "G2.97: (i)/(iii) in [1.38, 1.50] on every curve, both walks",
        a_lo >= 1.38 && a_hi <= 1.50,
        format!("{a_lo:.4}-{a_hi:.4}"),
    );
    check(
        "G2.98: S_A/S_A,K in [1.001, 1.008] on every row",
        sr_lo >= 1.001 && sr_hi <= 1.008,
        format!("{sr_lo:.6}-{sr_hi:.6}"),
    );

    // Sensitivity: GAE re-measured under (iii) on the E27 side (secondary).
    let sens: Vec<f64> = rows
        .iter()
        .map(|r| {
            let g2 = 63.0 + r.share_gls;
            let d = (r.s_k + g2 * r.e) / (r.rho_iii + g2 * r.steps);
            d / r.d_e31(1.0)
        })
        .collect();
    let (se_lo, se_hi) = minmax(&sens);
    let sens_alt: Vec<f64> = rows
        .iter()
        .map(|r| {
            let g2 = 63.0 + r.share_gls;
            let d = (r.s_k + g2 * r.e) / (r.rho_iii + g2 * r.steps);
            d / r.d_e27(1.0)
        })
        .collect();
    let (sa_lo, sa_hi) = minmax(&sens_alt);

    // ------------------------------------------------- §5a band (arm B)
    let arm_b: Vec<&Row> = rows.iter().filter(|r| r.arm == "B").collect();
    let mut band = Vec::new();
    for (k, g) in WEIGHTS {
        let ic: Vec<f64> = arm_b.iter().map(|r| r.ic_k(g)).collect();
        let rho: Vec<f64> = arm_b.iter().map(|r| r.rho_k(g)).collect();
        band.push((
            k.to_string(),
            obj(vec![("ic", jf(median(&ic))), ("rho", jf(median(&rho)))]),
        ));
    }
    let order_all = rows
        .iter()
        .all(|r| WEIGHTS.iter().all(|(_, g)| r.ic_k(*g) > r.rho_k(*g)));
    check("§5a: the matched walk (iii) is below the index calculus on all 20 rows at 0, 0.1 and 1 GAE", order_all, format!("{order_all}"));

    // Checks run outside this program (git ancestry, build, unit tests), each
    // with its command and output, recorded by the validator in one file.
    let (manual, hmanual) = ctx.read(&format!("{}/replay/manual_checks.json", ctx.ev));
    for m in manual.get("checks").arr() {
        check(
            &format!("manual: {}", m.get("check").s()),
            m.get("holds").b(),
            m.get("result").s().to_string(),
        );
    }
    let unit_tests_pass = manual
        .get("checks")
        .arr()
        .iter()
        .any(|m| m.get("id").s() == "unit_tests_e27" && m.get("holds").b());
    check(
        "G2.95's unit tests (e27_) were run by the validator and passed",
        unit_tests_pass,
        format!("{unit_tests_pass}"),
    );

    let (ef_lo, ef_hi) = minmax(&e2e_field_factor);
    let errata = J::Arr(vec![
        obj(vec![
            ("id", js("E1")),
            ("claim", js("§9.29 Stage W: \"(i)/(ii) (Karatsuba alone) ≈ 1.27\" (for both walks)")),
            ("recomputed", js(&format!("<±psi> {g12_lo:.4}-{g12_hi:.4}; negation {n12_lo:.4}-{n12_hi:.4}"))),
            ("effect", js("none on any verdict or headline: (i)/(iii), the registered quantity, is reproduced exactly; the ≈ 1.27 holds for the <±psi> walk only")),
        ]),
        obj(vec![
            ("id", js("E2")),
            ("claim", js("§9.29 IC side: \"Batching the IC side's F_p inversions too could save at most a further 0.127-0.170 % of S (bound, not built)\"")),
            ("recomputed", js(&format!("0.127-0.170 % reproduces only as (16 - the curve's <±psi> pool share) per inversion ({:.4}-{:.4} %), i.e. batching at the walk's measured share; under the batch cost 3(m-1)+16 the saving per inversion tends to 13 as m grows, so the upper bound is 13 * ic_fp_inversions / S = {:.4}-{:.4} % (and 16 per inversion, all of it, {:.4}-{:.4} %)", 100.0 * bs_lo, 100.0 * bs_hi, 100.0 * b13_lo, 100.0 * b13_hi, 100.0 * b16_lo, 100.0 * b16_hi))),
            ("effect", js("none on any verdict: the quantity is a side bound; \"at most\" should read \"at the walk's batch share\" or the bound should be 0.130-0.180 %")),
        ]),
        obj(vec![
            ("id", js("E3")),
            ("claim", js("experiments/32_walk_karatsuba/e2e_chain_karatsuba_p*s*.json fields rho_matched_muls and ratio")),
            ("recomputed", js("in these records rho_matched_muls is the mean of walk (ii) (Karatsuba, unbatched) WITH the [d]G check (e.g. p=101 s1: 407,712.125 = 405,069.125 + 2,643), and ratio = total_muls / that; neither is the E27 distance, which uses Stage W walk (iii) without the check (E25's convention)")),
            ("effect", js(&format!("none on §9.29 (its distances use walk (iii), reproduced here); a reader taking `ratio` from the E27 records would understate the distance at 0 GAE by the factor rho_matched_muls / rho_iii = {ef_lo:.4}-{ef_hi:.4}; the convention is disclosed in §9.29 but the records do not say it"))),
        ]),
    ]);

    let verdicts = J::Arr(vec![
        obj(vec![("id", js("G2.95")), ("note", js("holds for the runs; unit tests run with the session's final test build")), ("recomputed", js(&format!("320/320 walks identical three ways; (i) = E25 on 320/320 (both arms); unit tests e27_ by the validator: {}", if unit_tests_pass { "pass" } else { "NOT passed" })))]),
        obj(vec![("id", js("G2.96")), ("note", js("holds")), ("recomputed", js(&format!("mul 13; batched addition {:.2}-{:.2}; (i)/(iii) {a_lo:.4}-{a_hi:.4}: holds", 63.0 + sh_lo, 63.0 + sh_hi)))]),
        obj(vec![("id", js("G2.97")), ("note", js("holds")), ("recomputed", js(&format!("{a_lo:.4}-{a_hi:.4} on every curve, both walks: holds")))]),
        obj(vec![("id", js("G2.98")), ("note", js("holds")), ("recomputed", js(&format!("{sr_lo:.5}-{sr_hi:.5}; non-solve share {:.2}-{:.2} %; every other field reproduced: holds", 100.0 * ns_lo, 100.0 * ns_hi)))]),
        obj(vec![("id", js("G2.99")), ("note", js("holds at 0 and 0.1; falsified high at 1 GAE on the two p = 101 seed 2 rows (1.3252, band <= 1.30)")), ("recomputed", js(&format!("rows outside the bands: {outside:?}; rho ahead of every row at every weight: {}", min_dist > 1.0)))]),
    ]);

    let limitations = J::Arr(vec![
        js("Operation counts only. Wall times were neither claimed nor compared; the replay ran locally in the validator's 4-vCPU container (not isolab-eks), which is acceptable because no wall time is admitted."),
        js("The matched rho reference prices every walk without the [d]G = Q check (E25's and E31's convention, 0.13-0.78 % of a <±psi> walk) while the index calculus keeps its own check_muls; this favours rho, so the distances are conservative for the index calculus."),
        js("The batched walk (iii) runs in e27_walk, the index calculus in quartic_pipeline_gls: two binaries of one build (same commit, same crypto_lib counters). The pipeline process's own walks equal Stage W's walk (ii) record for record, and (iii) differs from (ii) only by the price of the recorded F_p inversions (identity checked on all 20 pools); same_binary_or_process is asserted on that basis."),
        js("Batched inversions are charged 3(m-1)+16 per round with m the round's inversions (all independent: begin/end additions; the inverses returned are the ones used, so bit-identity covers them). m averages above 16 because set-up rounds carry many independent scalar-multiplication steps; fillers keep the pool at 16 walks. The 16 charged per F_p inversion is the repository's convention, unchanged."),
        js("The index-calculus side is not batched (bound in errata E2); §5a's unpriced native work is priced only as one GAE per Macaulay entry and per walk step, as registered; F_p additions and the walks' hash traffic stay unpriced on both sides."),
        js("Scale: p = 101-397 (n of 22.6-33.6 bits), two seeds per p, 16 walks per fold; the 1.45x reference correction is measured on these curves only."),
        js("Unit tests ran on the recorded tree plus the producer's later test-only repair of an unrelated test helper (src/ml_dataset/storage.rs, commit ed49f1378), without which the lib test target does not compile at af2504ef1."),
    ]);

    let status = if failures.is_empty() && h.bad.is_empty() {
        "pass"
    } else {
        "fail"
    };
    obj(vec![
        ("schema", js("validator-report/v1")),
        ("status", js(status)),
        ("independent", jb(true)),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E27; this recompute is its own standard-library Rust program (validate_e27.rs: own JSON parser, own SHA-256) and shares no code with examples/e27_walk.rs, examples/e27_tables.rs, experiments/32_walk_karatsuba/summary.json or tables.md")),
        ("date", js("2026-10-30")),
        ("row", js("E27 (GLV note §9.25 pre-registration, §9.29 results): the matched <±psi> walk with Karatsuba F_p4 products and batched norm inversions over 16 lockstep walks (Stage W), and E31's solver-A pipeline with Karatsuba products (IC side), on E25's ten GLS curves, both arms")),
        ("script", js(&format!("{}/validate_e27.rs (rustc -O --edition 2021; `validate_e27 report . {}`)", ctx.ev, ctx.ev))),
        ("status_rule", js("pass iff every check holds and every headline agrees with its printed figure within half a unit of the last printed digit (exact for counts)")),
        ("raw_data", J::Arr(raw)),
        ("headline", J::Arr(h.items)),
        ("headline_disagreements", J::Arr(h.bad.iter().map(|x| js(x)).collect())),
        ("checks", J::Arr(checks)),
        ("check_failures", J::Arr(failures.iter().map(|x| js(x)).collect())),
        ("errata", errata),
        ("verdicts_recomputed", verdicts),
        ("limitations", limitations),
        ("manual_checks", obj(vec![("path", js("replay/manual_checks.json")), ("sha256", js(&hmanual))])),
        ("logarithms", J::Arr(logs.iter().map(|(p, s, d)| obj(vec![("p", ju(*p)), ("seed", ju(*s)), ("d", ju(*d))])).collect())),
        ("stage_w_pools", J::Arr(fold_rows)),
        ("ic_rows", J::Arr(ic_rows_json)),
        (
            "secondary",
            obj(vec![
                ("gae_remeasured_sensitivity", obj(vec![
                    ("claim", js("§9.29: with the GAE re-measured under (iii) (66.3-66.7 instead of 97) on the E27 side, the 1-GAE growth is 1.016-1.017")),
                    ("definition_a", js("(S_A,K + g' E)/(rho_iii + g' steps) over E31's 1-GAE S/rho (g' = 63 + that curve's <±psi> pool share)")),
                    ("definition_a_range", J::Arr(vec![jf(se_lo), jf(se_hi)])),
                    ("definition_b", js("(S_A,K + g' E)/(rho_iii + g' steps) over E27's own 1-GAE S/rho at GAE 97")),
                    ("definition_b_range", J::Arr(vec![jf(sa_lo), jf(sa_hi)])),
                ])),
                ("bound_16_per_inversion_pct", J::Arr(vec![jf(100.0 * b16_lo), jf(100.0 * b16_hi)])),
                ("ic_leaf_comparison_e31_vs_e27", obj(vec![
                    ("compared", ju(ic_fields_compared)),
                    ("identical", ju(ic_fields_identical)),
                    ("differing_paths", J::Arr(diff_paths.iter().map(|x| js(x)).collect())),
                    ("added_in_e27", J::Arr(added_paths.iter().map(|x| js(x)).collect())),
                ])),
            ]),
        ),
        (
            "gae_band",
            obj(vec![
                ("definition", js("medians over the ten curves of arm B (E25's GLS-fold benchmark arm), E27 IC side S_A,K + g*97*macaulay_entries against the matched <±psi> walk (iii) rho_iii + g*97*steps (E25 convention, [d]G check excluded), g = 0, 0.1, 1")),
                ("values", J::Obj(band)),
                ("order_rho_below_ic_on_all_20_rows_all_weights", jb(order_all)),
            ]),
        ),
    ])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: validate_e27 compare|report|certificate <repo> <evidence-dir>");
        std::process::exit(2);
    }
    let ctx = Ctx {
        repo: args[2].clone(),
        ev: args[3].clone(),
    };
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

// --------------------------------------------------------- certificate ----

fn certificate(ctx: &Ctx) -> J {
    let (cmp, hcmp) = ctx.read(&format!("{}/replay/replay_comparison.json", ctx.ev));
    let (build, hbuild) = ctx.read(&format!("{}/replay/build.json", ctx.ev));
    let (rsw, _) = ctx.read(&format!("{}/replay/stage_w.json", ctx.ev));
    // Scalars in the replay itself.
    let mut walks_correct = 0u64;
    let mut walks_total = 0u64;
    let mut logs: Vec<(u64, u64, u64)> = Vec::new();
    for c in rsw.get("curves").arr() {
        let mut ds = BTreeSet::new();
        for fold in FOLDS {
            for a in ARITH {
                for w in walks(c.get("folds").get(fold).get("walks").get(a)) {
                    walks_total += 1;
                    if w.correct {
                        walks_correct += 1;
                    }
                    ds.insert(w.rec);
                }
            }
        }
        let d = if ds.len() == 1 {
            ds.into_iter().next().unwrap().unwrap_or(u64::MAX)
        } else {
            u64::MAX
        };
        logs.push((c.get("p").u(), c.get("seed").u(), d));
    }
    let mut ic_ok = 0u64;
    let mut ic_total = 0u64;
    let mut ic_walks_correct = true;
    let mut runs = Vec::new();
    for r in cmp.get("runs").arr() {
        let name = r.get("run").s().to_string();
        let mut entry = vec![
            ("run", js(&name)),
            ("committed", r.get("committed").clone()),
            ("replay_output", r.get("replay").clone()),
            ("fields_compared", r.get("fields_compared").clone()),
            ("fields_identical", r.get("fields_identical").clone()),
            ("mismatches", r.get("mismatches").clone()),
            ("matched", r.get("identical").clone()),
        ];
        if name.starts_with("e2e_") {
            let rel = r.get("replay").get("path").s().to_string();
            let (rep, _) = ctx.read(&rel);
            let mut arms = Vec::new();
            for rec in rep.arr() {
                ic_total += 1;
                let (p, s) = (rec.get("p").u(), rec.get("seed").u());
                let d = logs
                    .iter()
                    .find(|x| x.0 == p && x.1 == s)
                    .map(|x| x.2)
                    .unwrap_or(u64::MAX);
                let fo = rec.get("folded");
                let ok =
                    fo.get("solved").b() && fo.get("correct").b() && fo.get("recovered").u() == d;
                if ok {
                    ic_ok += 1;
                }
                let wc = walks(rec.get("rho")).iter().all(|w| w.correct);
                ic_walks_correct &= wc;
                arms.push(obj(vec![
                    ("arm", rec.get("arm").clone()),
                    ("ic_solved", fo.get("solved").clone()),
                    ("ic_correct", fo.get("correct").clone()),
                    ("ic_recovered", fo.get("recovered").clone()),
                    ("walks_logarithm", ju(d)),
                    ("ic_equals_walks", jb(fo.get("recovered").u() == d)),
                    ("all_48_walks_correct", jb(wc)),
                    ("total_muls", rec.get("total_muls").clone()),
                    ("macaulay_entries", rec.get("macaulay_entries").clone()),
                ]));
            }
            entry.push(("scalars", J::Arr(arms)));
            entry.push(("timing_not_compared", r.get("timing_not_compared").clone()));
        } else {
            entry.push(("walks", ju(walks_total)));
            entry.push(("walks_correct", ju(walks_correct)));
            entry.push((
                "logarithms",
                J::Arr(
                    logs.iter()
                        .map(|(p, s, d)| obj(vec![("p", ju(*p)), ("seed", ju(*s)), ("d", ju(*d))]))
                        .collect(),
                ),
            ));
        }
        runs.push(obj(entry.into_iter().collect()));
    }
    let all_identical = cmp.get("all_identical").b();
    let scalars = walks_correct == walks_total
        && walks_total == 960
        && ic_ok == ic_total
        && ic_total == 20
        && ic_walks_correct
        && logs.iter().all(|x| x.2 != u64::MAX);
    obj(vec![
        ("schema", js("replay-certificate/v1")),
        ("status", js(if all_identical && scalars { "verified" } else { "failed" })),
        ("independent", jb(true)),
        ("scalars_verified", jb(scalars)),
        ("date", js("2026-10-30")),
        ("row", js("E27 (GLV note §9.25/§9.29): Stage W (10 curves x {<±psi>, negation} x 16 walks x 3 arithmetics) and the E31 solver-A pipeline with Karatsuba products (10 curves x 2 arms); evidence directory research/e27_walk_20261030")),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E27: fresh build from an export of the recorded binary commit in a fresh target directory, fresh runs, own comparison (validate_e27.rs compare)")),
        ("recorded_binary", js("af2504ef161cbe804d855f3fa26c435bc02ba282 (experiments/32_walk_karatsuba/binary_commit.txt); E27 code 6490d0fac plus compile repairs af2504ef1")),
        ("build", build),
        ("build_record", obj(vec![("path", js("replay/build.json")), ("sha256", js(&hbuild))])),
        ("comparison", obj(vec![("path", js("replay/replay_comparison.json")), ("sha256", js(&hcmp)), ("rule", cmp.get("rule").clone())])),
        ("runs_total", cmp.get("runs_total").clone()),
        ("runs_matched", cmp.get("runs_identical").clone()),
        ("fields_compared", cmp.get("fields_compared").clone()),
        ("fields_identical", cmp.get("fields_identical").clone()),
        ("coverage", js("full: every recorded run replayed (the Stage W run: 960 walks, the per-op cost probe; all ten end-to-end runs, both arms each, 48 walks per arm); no sampling")),
        ("scalar_verification", js("Every replayed walk (960) reports correct = true, which the binary sets only when the collision-recovered logarithm equals the instance's planted d and [d]G = Q is checked on the curve (quartic_folds.rs rho4_folded_with / WalkMachine::finish); all 96 walks of a curve recover the same d. Every replayed index-calculus arm (20) reports solved and correct (Wiedemann logarithm checked against the planted d and [x]G = Q in the binary) and its recovered logarithm equals the walks' d: two independent algorithms agree on every curve. The instances are E25's (n, group_order, cofactor, base_points equal, checked leaf by leaf in validator-report.json); benchmarks/20261018-k4-gls-v4/replay-certificate.json verified them independently.")),
        ("runs", J::Arr(runs)),
    ])
}
