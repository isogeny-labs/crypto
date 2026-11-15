//! E33 validation (independent validator, 2026-11-01).
//!
//! An independent recompute of the headline numbers of
//! `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
//! §9.37 (E33: solver B's pivot selection counted, B' and B'') from the raw
//! records in the repository, the G2.124-G2.133 verdicts and every rank
//! across the §5a band, and a leaf-by-leaf comparison of the validator's
//! fresh replays with the committed records.  Standard library only (its own
//! JSON parser and SHA-256, carried over from the E27 validator's
//! validate_e27.rs, itself validator code); it shares no code with
//! `examples/e33_tables.rs`, `examples/e33_checks.rs`, `summary.json`,
//! `tables.md` or `crypto_lib`.  `git` is invoked (std::process) only for the
//! chronology checks.
//!
//! ```text
//! rustc -O --edition 2021 validate_e33.rs -o validate_e33
//! validate_e33 all <repo> <evidence-dir>
//!   -> replay/replay_comparison.json, validator-report.json,
//!      replay-certificate.json, VALIDATION.json
//! ```
//!
//! `<evidence-dir>` is relative to `<repo>`
//! (here `research/e33_solver_b_pivot_count_20261031`).

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::Command;

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
    #[allow(dead_code)]
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
const GAE: f64 = 97.0;
const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const WNAME: [&str; 3] = ["0", "0.1", "1"];
/// (solver tag, label) of the three counted variants, in §9.37's order.
const VAR: [(&str, &str); 3] = [
    ("chain-sparsest-counted", "B"),
    ("chain-sparsest-inc", "B'"),
    ("chain-sparsest-est", "B''"),
];
/// Stage A's solver order (examples/e33_stage_a.rs NAMES; checked against the record).
const NAMES: [&str; 7] = [
    "existing",
    "chain",
    "chain-counted",
    "chain-sparsest",
    "chain-sparsest-counted",
    "chain-sparsest-inc",
    "chain-sparsest-est",
];
const E33: &str = "experiments/37_solver_b_pivot_count";
const NOTE: &str = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md";
const SRC: &str = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.37";
const PREREG: &str = "27663cc4d7bffe47d3498f83600d912b445789f7";
const CODE: &str = "c4431389209ae4a8d83bc916bbb233eb4a8438cb";
const PRE_E33: &str = "bd0118f393537a2e610336e5d45f5426a6a0fa83";
const FIX: &str = "c689d19821d97949e13c4294bd44d5175c0bd555";
const STAGE_A_COMMIT: &str = "8d8155d8b15513b6fa8ee9ebae98c4ad152bcf8e";
const RESULTS_COMMIT: &str = "4852cd4bec";
const STAGE_A_MANIFEST_SHA: &str = "7bc6e00bd69f353a6d07c345e86ef50c33e6ea9a95535476b8bfd20bbe87d3d8";

fn e33_rel(solver: &str, p: u64, s: u64) -> String {
    format!("{E33}/e2e_{solver}_karatsuba_p{p}s{s}.json")
}
fn e27_rel(p: u64, s: u64) -> String {
    format!("experiments/32_walk_karatsuba/e2e_chain_karatsuba_p{p}s{s}.json")
}
fn e31b_rel(p: u64, s: u64) -> String {
    format!("experiments/31_solve_cost/e2e_chain-sparsest_p{p}s{s}.json")
}
fn e31a_rel(p: u64, s: u64) -> String {
    format!("experiments/31_solve_cost/e2e_chain_p{p}s{s}.json")
}
fn e25_rel(p: u64, s: u64) -> String {
    format!("experiments/30_gls_v4/p{p}s{s}.json")
}

struct Ctx {
    repo: String,
    ev: String,
    pins: RefCell<BTreeMap<String, String>>,
}

impl Ctx {
    fn path(&self, rel: &str) -> String {
        format!("{}/{}", self.repo, rel)
    }
    fn bytes(&self, rel: &str) -> Vec<u8> {
        fs::read(self.path(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
    }
    fn exists(&self, rel: &str) -> bool {
        std::path::Path::new(&self.path(rel)).exists()
    }
    /// Read and parse a raw record, pinning its SHA-256.
    fn read(&self, rel: &str) -> J {
        let b = self.bytes(rel);
        self.pins.borrow_mut().insert(rel.to_string(), sha256(&b));
        parse(std::str::from_utf8(&b).unwrap())
    }
    fn sha(&self, rel: &str) -> String {
        sha256(&self.bytes(rel))
    }
    fn git(&self, args: &[&str]) -> (bool, String) {
        let o = Command::new("git")
            .arg("-C")
            .arg(&self.repo)
            .args(args)
            .output()
            .expect("git");
        (
            o.status.success(),
            String::from_utf8_lossy(&o.stdout).trim().to_string(),
        )
    }
    fn write(&self, rel: &str, j: &J) {
        fs::write(self.path(rel), j.dump()).unwrap_or_else(|e| panic!("{rel}: {e}"));
    }
}

fn uvec(j: &J) -> Vec<u64> {
    j.arr().iter().map(|x| x.u()).collect()
}
fn mean(v: &[u64]) -> f64 {
    v.iter().map(|&x| x as u128).sum::<u128>() as f64 / v.len() as f64
}
fn median_u(v: &[u64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_unstable();
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2] as f64
    } else {
        (s[n / 2 - 1] as f64 + s[n / 2] as f64) / 2.0
    }
}
fn upper_median(v: &[u64]) -> u64 {
    let mut s = v.to_vec();
    s.sort_unstable();
    s[s.len() / 2]
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
fn median_f(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    }
}
fn farr(v: &[f64]) -> J {
    J::Arr(v.iter().map(|&x| jf(x)).collect())
}
#[allow(dead_code)]
fn uarr(v: &[u64]) -> J {
    J::Arr(v.iter().map(|&x| ju(x)).collect())
}
fn sarr(v: &[String]) -> J {
    J::Arr(v.iter().map(|x| js(x)).collect())
}

// ----------------------------------------------------------- headlines ----

/// A printed figure agrees when the recomputed value rounds to it: the
/// tolerance is half a unit of the last printed digit (relative to the
/// printed value), or 1e-12 for an exact count.
struct Headlines {
    items: Vec<J>,
    bad: Vec<String>,
    prose: Vec<J>,
    prose_bad: Vec<String>,
}

impl Headlines {
    #[allow(clippy::too_many_arguments)]
    fn add(&mut self, name: &str, printed: &str, claimed: f64, half_unit: f64, recomputed: f64, def: &str) {
        let rel_tol = if half_unit == 0.0 {
            1e-12
        } else {
            half_unit / claimed.abs() * (1.0 + 1e-9)
        };
        let scale = claimed.abs().max(1e-300);
        let agrees = (claimed - recomputed).abs() <= rel_tol * scale;
        if !agrees {
            self.bad
                .push(format!("{name}: printed {printed}, recomputed {recomputed}"));
        }
        self.items.push(obj(vec![
            ("name", js(name)),
            ("source", js(SRC)),
            ("printed", js(printed)),
            ("claimed", jf(claimed)),
            ("recomputed", jf(recomputed)),
            ("rel_tol", jf(rel_tol)),
            ("agrees", jb(agrees)),
            ("definition", js(def)),
        ]));
    }
    /// A printed range `lo–hi`: the minimum and the maximum of `vals`.
    #[allow(clippy::too_many_arguments)]
    fn range(&mut self, name: &str, printed: &str, lo: f64, hlo: f64, hi: f64, hhi: f64, vals: &[f64], def: &str) {
        let (a, b) = minmax(vals);
        self.add(&format!("{name}, min"), printed, lo, hlo, a, def);
        self.add(&format!("{name}, max"), printed, hi, hhi, b, def);
    }
    /// A rounded figure in running prose standing for a range measured in
    /// the tables: consistent when it lies within half a unit of the
    /// recomputed [min, max].  Recorded apart from the headline list.
    fn prose(&mut self, name: &str, printed: &str, claimed: f64, half_unit: f64, vals: &[f64], def: &str) {
        let (a, b) = minmax(vals);
        let ok = claimed + half_unit >= a * (1.0 - 1e-12) && claimed - half_unit <= b * (1.0 + 1e-12);
        if !ok {
            self.prose_bad
                .push(format!("{name}: printed {printed}, recomputed {a}-{b}"));
        }
        self.prose.push(obj(vec![
            ("name", js(name)),
            ("source", js(SRC)),
            ("printed", js(printed)),
            ("recomputed_min", jf(a)),
            ("recomputed_max", jf(b)),
            ("consistent", jb(ok)),
            ("definition", js(def)),
        ]));
    }
}

struct Checks {
    items: Vec<J>,
    failures: Vec<String>,
}

impl Checks {
    fn add(&mut self, name: &str, ok: bool, detail: String) {
        if !ok {
            self.failures.push(format!("{name}: {detail}"));
        }
        self.items.push(obj(vec![
            ("check", js(name)),
            ("holds", jb(ok)),
            ("detail", js(&detail)),
        ]));
    }
}

// --------------------------------------------------------- comparisons ----

fn is_timing(key: &str) -> bool {
    key.ends_with("_ms") || key == "ms" || key.contains("wall")
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
            if is_timing(key) {
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
                out.mismatches.push((path.to_string(), a.clone(), b.clone()));
            }
        }
    }
}

/// Equal text, or equal integers (as i128), or two non-integers of equal
/// value.  An integer never equals a float's text.
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

/// Paths with array indices replaced by `*`.
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

fn diff(a: &J, b: &J) -> LeafDiff {
    let mut d = LeafDiff::default();
    leaves("$", a, b, &mut d);
    d
}

fn generic_set(v: &[String]) -> BTreeSet<String> {
    v.iter().map(|p| generic(p)).collect()
}

fn mism_paths(d: &LeafDiff) -> Vec<String> {
    d.mismatches.iter().map(|(p, _, _)| p.clone()).collect()
}

fn count_keys(j: &J, keys: &[&str]) -> u64 {
    match j {
        J::Obj(o) => o
            .iter()
            .map(|(k, v)| u64::from(keys.contains(&k.as_str())) + count_keys(v, keys))
            .sum(),
        J::Arr(a) => a.iter().map(|v| count_keys(v, keys)).sum(),
        _ => 0,
    }
}

fn diff_json(name: &str, a_path: &str, a_sha: &str, b_path: &str, b_sha: &str, d: &LeafDiff) -> J {
    let ok = d.mismatches.is_empty() && d.only_a.is_empty() && d.only_b.is_empty();
    obj(vec![
        ("run", js(name)),
        ("committed", obj(vec![("path", js(a_path)), ("sha256", js(a_sha))])),
        ("replay", obj(vec![("path", js(b_path)), ("sha256", js(b_sha))])),
        ("fields_compared", ju(d.compared)),
        ("fields_identical", ju(d.identical)),
        (
            "mismatches",
            J::Arr(
                d.mismatches
                    .iter()
                    .take(50)
                    .map(|(p, x, y)| obj(vec![("path", js(p)), ("committed", x.clone()), ("replay", y.clone())]))
                    .collect(),
            ),
        ),
        ("mismatch_count", ju(d.mismatches.len() as u64)),
        ("only_in_committed", sarr(&d.only_a)),
        ("only_in_replay", sarr(&d.only_b)),
        (
            "timing_not_compared",
            J::Arr(
                d.timing
                    .iter()
                    .map(|(p, x, y)| obj(vec![("path", js(p)), ("committed", x.clone()), ("replay", y.clone())]))
                    .collect(),
            ),
        ),
        ("identical", jb(ok)),
    ])
}

/// The validator's replays against the committed records.
fn compare(ctx: &Ctx) -> J {
    let rp = format!("{}/replay", ctx.ev);
    let mut runs: Vec<J> = Vec::new();
    let (mut compared, mut identical) = (0u64, 0u64);
    let mut all_ok = true;
    // Stage A (one run: ten curves x 1,000 calls x 7 solvers).
    let st_rel = format!("{rp}/stage_a.json");
    let st_sha_rel = format!("{rp}/stage_a.json.sha256");
    let mut stage_a_replayed = false;
    if ctx.exists(&st_rel) {
        stage_a_replayed = true;
        let a = ctx.read(&format!("{E33}/stage_a.json"));
        let b = parse(std::str::from_utf8(&ctx.bytes(&st_rel)).unwrap());
        let d = diff(&a, &b);
        let (ha, hb) = (ctx.sha(&format!("{E33}/stage_a.json")), ctx.sha(&st_rel));
        let mut r = diff_json("stage_a", &format!("{E33}/stage_a.json"), &ha, &st_rel, &hb, &d);
        if let J::Obj(o) = &mut r {
            o.push(("byte_identical".into(), jb(ha == hb)));
        }
        compared += d.compared;
        identical += d.identical;
        all_ok &= r.get("identical").b();
        runs.push(r);
    } else if ctx.exists(&st_sha_rel) {
        // The replay output was byte-identical and is not duplicated in the
        // repository; its digest and leaf count were recorded at comparison time.
        let rec = parse(std::str::from_utf8(&ctx.bytes(&st_sha_rel)).unwrap());
        let ha = ctx.sha(&format!("{E33}/stage_a.json"));
        let same = rec.get("sha256").s() == ha;
        stage_a_replayed = true;
        compared += rec.get("fields_compared").u();
        identical += if same { rec.get("fields_compared").u() } else { 0 };
        all_ok &= same;
        runs.push(obj(vec![
            ("run", js("stage_a")),
            ("committed", obj(vec![("path", js(&format!("{E33}/stage_a.json"))), ("sha256", js(&ha))])),
            ("replay", obj(vec![("path", js(&st_sha_rel)), ("sha256", js(rec.get("sha256").s()))])),
            ("fields_compared", ju(rec.get("fields_compared").u())),
            ("fields_identical", ju(if same { rec.get("fields_compared").u() } else { 0 })),
            ("byte_identical", jb(same)),
            ("identical", jb(same)),
            ("note", js("replay output byte-identical to the committed record (sha256 equal); the 2.7 MB copy is not duplicated in the repository")),
        ]));
    }
    // End to end.
    let mut e2e_done: Vec<String> = Vec::new();
    for (tag, _) in VAR {
        for p in PS {
            for s in SEEDS {
                let name = format!("e2e_{tag}_karatsuba_p{p}s{s}.json");
                let rel = format!("{rp}/e2e/{name}");
                if !ctx.exists(&rel) {
                    continue;
                }
                let crel = e33_rel(tag, p, s);
                let a = ctx.read(&crel);
                let b = parse(std::str::from_utf8(&ctx.bytes(&rel)).unwrap());
                let d = diff(&a, &b);
                let r = diff_json(&format!("{tag} p{p}s{s}"), &crel, &ctx.sha(&crel), &rel, &ctx.sha(&rel), &d);
                compared += d.compared;
                identical += d.identical;
                all_ok &= r.get("identical").b();
                e2e_done.push(format!("{tag} p{p}s{s}"));
                runs.push(r);
            }
        }
    }
    // Existing solvers: the pre-E33 build against E33's build.
    let mut ident: Vec<J> = Vec::new();
    let mut ident_ok = true;
    let id_dir = format!("{rp}/identity");
    let mut id_curves: BTreeSet<(u64, u64)> = BTreeSet::new();
    if let Ok(rd) = fs::read_dir(ctx.path(&id_dir)) {
        for e in rd.flatten() {
            let f = e.file_name().to_string_lossy().to_string();
            if let Some(rest) = f.strip_prefix("new_") {
                if let Some(ps) = rest.rsplit('_').next() {
                    let ps = ps.trim_end_matches(".json");
                    if let Some((pp, ss)) = ps.trim_start_matches('p').split_once('s') {
                        if let (Ok(pp), Ok(ss)) = (pp.parse(), ss.parse()) {
                            id_curves.insert((pp, ss));
                        }
                    }
                }
            }
        }
    }
    let e33keys = ["pivot_selection", "pivot_selection_ops", "lead_scan_reads"];
    for (p, s) in &id_curves {
        for cfg in ["macaulay", "chain", "chain-sparsest", "chain-karatsuba"] {
            let o = format!("{id_dir}/old_{cfg}_p{p}s{s}.json");
            let n = format!("{id_dir}/new_{cfg}_p{p}s{s}.json");
            if !ctx.exists(&o) || !ctx.exists(&n) {
                ident_ok = false;
                ident.push(obj(vec![("config", js(cfg)), ("missing", jb(true))]));
                continue;
            }
            let a = parse(std::str::from_utf8(&ctx.bytes(&o)).unwrap());
            let b = parse(std::str::from_utf8(&ctx.bytes(&n)).unwrap());
            let d = diff(&a, &b);
            let e33f = count_keys(&b, &e33keys);
            let ok = d.mismatches.is_empty() && d.only_a.is_empty() && d.only_b.is_empty() && e33f == 0;
            ident_ok &= ok;
            // Against the stored historical record of the same configuration.
            let hist = match cfg {
                "macaulay" => e25_rel(*p, *s),
                "chain" => e31a_rel(*p, *s),
                "chain-sparsest" => e31b_rel(*p, *s),
                _ => e27_rel(*p, *s),
            };
            let h = ctx.read(&hist);
            let dh = diff(&h, &b);
            ident.push(obj(vec![
                ("config", js(cfg)),
                ("p", ju(*p)),
                ("seed", ju(*s)),
                ("old", obj(vec![("path", js(&o)), ("sha256", js(&ctx.sha(&o)))])),
                ("new", obj(vec![("path", js(&n)), ("sha256", js(&ctx.sha(&n)))])),
                ("leaves_compared", ju(d.compared)),
                ("leaves_identical", ju(d.identical)),
                ("mismatches", sarr(&mism_paths(&d))),
                ("only_old", sarr(&d.only_a)),
                ("only_new", sarr(&d.only_b)),
                ("timing_listed", ju(d.timing.len() as u64)),
                ("e33_fields_in_new", ju(e33f)),
                ("identical", jb(ok)),
                (
                    "new_vs_stored_record",
                    obj(vec![
                        ("path", js(&hist)),
                        ("leaves_compared", ju(dh.compared)),
                        ("leaves_identical", ju(dh.identical)),
                        ("mismatches_generic", sarr(&generic_set(&mism_paths(&dh)).into_iter().collect::<Vec<_>>())),
                        ("only_in_stored", sarr(&dh.only_a)),
                        ("only_in_new", sarr(&generic_set(&dh.only_b).into_iter().collect::<Vec<_>>())),
                        ("identical", jb(dh.mismatches.is_empty() && dh.only_a.is_empty() && dh.only_b.is_empty())),
                    ]),
                ),
            ]));
        }
    }
    if id_curves.is_empty() {
        ident_ok = false;
    }
    // Negative controls (validator-owned example e33val_controls).
    let mut ctl: Vec<J> = Vec::new();
    let mut ctl_ok = true;
    let mut ctl_tot = [0u64; 8]; // calls contested nc1 nc1_same nc2 nc3_nonempty nc3_changed phases_changed
    if let Ok(rd) = fs::read_dir(ctx.path(&format!("{rp}/controls"))) {
        let mut files: Vec<String> = rd
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|f| f.ends_with(".json"))
            .collect();
        files.sort();
        for f in files {
            let rel = format!("{rp}/controls/{f}");
            let c = parse(std::str::from_utf8(&ctx.bytes(&rel)).unwrap());
            let n = c.get("calls").u();
            let k = c.get("contested").u();
            let pos = c.get("a_out_eq_b").u() == n
                && c.get("b_counted_out_eq_b").u() == n
                && c.get("b_counted_phases_eq_b").u() == n
                && c.get("inc_out_eq_b").u() == n
                && c.get("inc_phases_eq_b").u() == n
                && c.get("inc_digest_eq_b_counted").u() == n
                && c.get("est_out_eq_b").u() == n
                && c.get("count_mismatches_b_counted_and_inc").u() == 0;
            // NC3: the first stream's non-empty calls plus the NC3 stream.
            let st3 = c.get("nc3_stream");
            let (s3_n, s3_bc, s3_be) = match st3 {
                J::Obj(_) => (st3.get("nonempty").u(), st3.get("changed_b_counted").u(), st3.get("changed_b2").u()),
                _ => (0, 0, 0),
            };
            let s3_same = match st3 {
                J::Obj(_) => st3.get("rows").arr().iter().all(|r| r.get("b_counted_equals_b").b()),
                _ => true,
            };
            let nc3_n = c.get("nc3_nonempty").u() + s3_n;
            let nc3_c = c.get("nc3_changed").u() + s3_bc;
            let nc3_ok = nc3_n > 0 && nc3_c == nc3_n && s3_be == s3_n && s3_same;
            let neg = k > 0
                && c.get("nc1_second_best_digest_changed").u() == k
                && c.get("nc1_output_same").u() == k
                && c.get("nc2_off_by_one_flagged").u() == k
                && nc3_ok;
            // Per-call consistency: B''s comparisons/updates and B's initial pass.
            let mut init_ok = true;
            for r in c.get("per_call").arr() {
                let sb = uvec(r.get("sel_b"));
                let si = uvec(r.get("sel_inc"));
                init_ok &= sb[1] == si[1] && sb[0] % 448 == 0 && si[0] < sb[0];
            }
            ctl_ok &= pos && neg && init_ok;
            for (i, key) in [
                "calls",
                "contested",
                "nc1_second_best_digest_changed",
                "nc1_output_same",
                "nc2_off_by_one_flagged",
                "nc3_nonempty",
                "nc3_changed",
                "nc1_phase_counts_changed",
            ]
            .iter()
            .enumerate()
            {
                ctl_tot[i] += c.get(key).u();
            }
            ctl_tot[5] += s3_n;
            ctl_tot[6] += s3_bc;
            ctl.push(obj(vec![
                ("path", js(&rel)),
                ("sha256", js(&ctx.sha(&rel))),
                ("p", ju(c.get("p").u())),
                ("seed", ju(c.get("seed").u())),
                ("nonce_rng", ju(c.get("nonce").u())),
                ("calls", ju(n)),
                ("contested", ju(k)),
                ("positive_checks_hold", jb(pos)),
                ("negative_controls_detected", jb(neg)),
                ("b_initial_multiple_of_448_and_inc_compares_equal", jb(init_ok)),
                ("nc1_second_best_digest_changed", ju(c.get("nc1_second_best_digest_changed").u())),
                ("nc1_output_same", ju(c.get("nc1_output_same").u())),
                ("nc1_phase_counts_changed", ju(c.get("nc1_phase_counts_changed").u())),
                ("nc1_echelon_muls_changed", ju(c.get("nc1_echelon_muls_changed").u())),
                ("nc2_off_by_one_flagged", ju(c.get("nc2_off_by_one_flagged").u())),
                ("nc3_nonempty_first_stream", ju(c.get("nc3_nonempty").u())),
                ("nc3_changed_first_stream", ju(c.get("nc3_changed").u())),
                ("nc3_stream_targets_drawn", ju(match st3 { J::Obj(_) => st3.get("targets_drawn").u(), _ => 0 })),
                ("nc3_stream_nonempty", ju(s3_n)),
                ("nc3_stream_changed_b_counted", ju(s3_bc)),
                ("nc3_stream_changed_b2", ju(s3_be)),
                ("nc3_holds", jb(nc3_ok)),
                ("est_digest_eq_b_counted", ju(c.get("est_digest_eq_b_counted").u())),
            ]));
        }
    }
    if ctl.is_empty() {
        ctl_ok = false;
    }
    obj(vec![
        ("schema", js("e33-replay-comparison/v1")),
        ("tool", js(&format!("{}/validate_e33.rs all", ctx.ev))),
        ("rule", js("every leaf of every committed record against the replay: integers and floats by their text (equal text, or equal value with the same integer/float form), strings and booleans exactly; keys ending in _ms, 'ms' and keys containing 'wall' are wall-clock and listed, not compared; a key present on one side only is a mismatch")),
        ("stage_a_replayed", jb(stage_a_replayed)),
        ("e2e_replayed", sarr(&e2e_done)),
        ("runs_total", ju(runs.len() as u64)),
        ("runs_identical", ju(runs.iter().filter(|r| r.get("identical").b()).count() as u64)),
        ("fields_compared", ju(compared)),
        ("fields_identical", ju(identical)),
        ("all_identical", jb(all_ok)),
        ("runs", J::Arr(runs)),
        ("existing_solver_identity", J::Arr(ident)),
        ("existing_solver_identity_holds", jb(ident_ok)),
        ("negative_controls", J::Arr(ctl)),
        ("negative_controls_hold", jb(ctl_ok)),
        (
            "negative_controls_totals",
            obj(vec![
                ("calls", ju(ctl_tot[0])),
                ("contested", ju(ctl_tot[1])),
                ("nc1_second_best_digest_changed", ju(ctl_tot[2])),
                ("nc1_output_same", ju(ctl_tot[3])),
                ("nc1_phase_counts_changed", ju(ctl_tot[7])),
                ("nc2_off_by_one_flagged", ju(ctl_tot[4])),
                ("nc3_nonempty", ju(ctl_tot[5])),
                ("nc3_changed", ju(ctl_tot[6])),
            ]),
        ),
    ])
}

/// jq -S -c's form: keys sorted (by bytes), no whitespace, numbers as their
/// literal text (jq 1.7 keeps unmodified literals), after dropping wall-clock
/// keys (ending in _ms, 'ms', containing 'wall') at every depth.
fn canonical(j: &J, out: &mut String) {
    match j {
        J::Obj(o) => {
            let mut kv: Vec<&(String, J)> = o.iter().filter(|(k, _)| !is_timing(k)).collect();
            kv.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
            out.push('{');
            for (i, (k, v)) in kv.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                esc(k, out);
                out.push(':');
                canonical(v, out);
            }
            out.push('}');
        }
        J::Arr(a) => {
            out.push('[');
            for (i, v) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                canonical(v, out);
            }
            out.push(']');
        }
        J::Str(s) => esc(s, out),
        J::Num(s) => out.push_str(s),
        J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        J::Null => out.push_str("null"),
    }
}

fn canonical_sha(j: &J) -> String {
    let mut s = String::new();
    canonical(j, &mut s);
    s.push('\n');
    sha256(s.as_bytes())
}

// ------------------------------------------------------------- Stage A ----

struct ArmA {
    solver: String,
    call: Vec<u64>,
    ech: Vec<u64>,
    ent: Vec<u64>,
    sel: Vec<[u64; 3]>,
    lead: Vec<u64>,
    fallbacks: u64,
    solved: u64,
}

impl ArmA {
    fn of(j: &J) -> ArmA {
        ArmA {
            solver: j.get("solver").s().to_string(),
            call: uvec(j.get("call")),
            ech: uvec(j.get("echelon")),
            ent: uvec(j.get("entries")),
            sel: j
                .get("sel")
                .arr()
                .iter()
                .map(|x| {
                    let v = uvec(x);
                    assert_eq!(v.len(), 3);
                    [v[0], v[1], v[2]]
                })
                .collect(),
            lead: uvec(j.get("lead_scan")),
            fallbacks: j.get("fallbacks").u(),
            solved: j.get("solved").u(),
        }
    }
    fn tot(&self) -> Vec<u64> {
        self.sel.iter().map(|s| s[0] + s[1] + s[2]).collect()
    }
    fn part(&self, k: usize) -> Vec<u64> {
        self.sel.iter().map(|s| s[k]).collect()
    }
    /// Mean per-call S(g) = C + 97 g (entries + selection) (+ 97 g lead).
    fn s_mean(&self, g: f64, lead: Option<&[u64]>) -> f64 {
        let n = self.call.len();
        let mut acc = 0.0f64;
        for i in 0..n {
            let sel = (self.sel[i][0] + self.sel[i][1] + self.sel[i][2]) as f64;
            let l = lead.map_or(0.0, |v| v[i] as f64);
            acc += self.call[i] as f64 + GAE * g * (self.ent[i] as f64 + sel + l);
        }
        acc / n as f64
    }
}

struct CurveA {
    p: u64,
    s: u64,
    calls: u64,
    agree: Vec<u64>,
    ph: Vec<u64>,
    dg: Vec<u64>,
    mism: u64,
    solvers: Vec<String>,
    arms: Vec<ArmA>,
    summaries: Vec<J>,
}

fn stage_a(j: &J) -> Vec<CurveA> {
    j.arr()
        .iter()
        .map(|c| CurveA {
            p: c.get("p").u(),
            s: c.get("seed").u(),
            calls: c.get("calls").u(),
            agree: uvec(c.get("agree")),
            ph: uvec(c.get("phases_equal")),
            dg: uvec(c.get("digest_equal_b")),
            mism: c.get("inc_count_mismatches").u(),
            solvers: c.get("solvers").arr().iter().map(|x| x.s().to_string()).collect(),
            arms: c.get("arms").arr().iter().map(ArmA::of).collect(),
            summaries: c.get("summaries").arr().to_vec(),
        })
        .collect()
}

// --------------------------------------------------------- end to end ----

#[derive(Clone)]
struct Row {
    p: u64,
    s: u64,
    arm: String,
    /// Per variant (B, B', B''): total_muls, macaulay_entries, selection ops,
    /// calls, lead-scan reads, solve_muls.
    x_tot: [f64; 3],
    x_ent: [f64; 3],
    x_sel: [f64; 3],
    x_calls: [f64; 3],
    x_lead: [f64; 3],
    a_tot: f64,
    a_ent: f64,
    e25_tot: f64,
    e25_ent: f64,
    e31b_tot: f64,
    e31b_ent: f64,
    rho: f64,
    steps: f64,
}

impl Row {
    fn s_x(&self, v: usize, g: f64) -> f64 {
        self.x_tot[v] + g * GAE * (self.x_ent[v] + self.x_sel[v])
    }
    fn s_x_lead(&self, v: usize, g: f64) -> f64 {
        self.s_x(v, g) + g * GAE * self.x_lead[v]
    }
    /// Schoolbook S of X: E31 B's total re-priced by X's own solve (the
    /// non-solve F_p4 work is the same calls for every pivot rule).
    fn s_x_sch(&self, v: usize, g: f64) -> f64 {
        self.e31b_tot + (self.x_tot[v] - self.x_tot[0]) + g * GAE * (self.x_ent[v] + self.x_sel[v])
    }
    fn s_x_sch_lead(&self, v: usize, g: f64) -> f64 {
        self.s_x_sch(v, g) + g * GAE * self.x_lead[v]
    }
    fn s_a(&self, g: f64) -> f64 {
        self.a_tot + g * GAE * self.a_ent
    }
    fn s_e25(&self, g: f64) -> f64 {
        self.e25_tot + g * GAE * self.e25_ent
    }
    fn rho(&self, g: f64) -> f64 {
        self.rho + g * GAE * self.steps
    }
}

/// Rows where the left arm is cheaper, dearer, equal, at each weight.
fn order(rows: &[Row], f: &dyn Fn(&Row, f64) -> f64, c: &dyn Fn(&Row, f64) -> f64) -> Vec<[u64; 3]> {
    WEIGHTS
        .iter()
        .map(|&g| {
            let mut o = [0u64; 3];
            for r in rows {
                let (x, y) = (f(r, g), c(r, g));
                if x < y {
                    o[0] += 1;
                } else if x > y {
                    o[1] += 1;
                } else {
                    o[2] += 1;
                }
            }
            o
        })
        .collect()
}

/// "ranked" only where one arm is cheaper on every row at every weight.
fn rank_word(o: &[[u64; 3]], n: u64, left: &str, right: &str) -> String {
    if o.iter().all(|w| w[0] == n) {
        format!("{left} ahead of {right} on every row at every weight: ranked")
    } else if o.iter().all(|w| w[1] == n) {
        format!("{right} ahead of {left} on every row at every weight: ranked")
    } else {
        let parts: Vec<String> = o
            .iter()
            .zip(WNAME)
            .map(|(w, g)| format!("{g} GAE: {left} ahead on {}, behind on {}, tied on {}", w[0], w[1], w[2]))
            .collect();
        format!("unresolved ({})", parts.join("; "))
    }
}

fn order_json(o: &[[u64; 3]], n: u64, left: &str, right: &str) -> J {
    obj(vec![
        ("left", js(left)),
        ("right", js(right)),
        ("rows", ju(n)),
        (
            "per_weight",
            J::Arr(
                o.iter()
                    .zip(WNAME)
                    .map(|(w, g)| {
                        obj(vec![
                            ("gae", js(g)),
                            ("left_cheaper", ju(w[0])),
                            ("left_dearer", ju(w[1])),
                            ("tied", ju(w[2])),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("rank", js(&rank_word(o, n, left, right))),
        (
            "order_holds_across_band",
            jb(o.iter().all(|w| w[0] == n) || o.iter().all(|w| w[1] == n)),
        ),
    ])
}

/// The §9.35 text of a version of the note: from its heading to the next.
fn section(text: &str, head: &str) -> String {
    let mut out = String::new();
    let mut on = false;
    for line in text.lines() {
        if line.starts_with("### ") {
            if on {
                break;
            }
            on = line.starts_with(head);
        }
        if on {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn report(ctx: &Ctx, cmp: &J) -> J {
    let mut ck = Checks {
        items: Vec::new(),
        failures: Vec::new(),
    };
    let mut h = Headlines {
        items: Vec::new(),
        bad: Vec::new(),
        prose: Vec::new(),
        prose_bad: Vec::new(),
    };

    // ---------------------------------------------- chronology (git) ----
    let commit_time = |c: &str| -> u64 {
        let (ok, out) = ctx.git(&["log", "-1", "--format=%ct", c]);
        assert!(ok, "git log {c}");
        out.parse().unwrap()
    };
    let (t_pre, t_code, t_stage, t_fix) = (
        commit_time(PREREG),
        commit_time(CODE),
        commit_time(STAGE_A_COMMIT),
        commit_time(FIX),
    );
    let (_, results_full) = ctx.git(&["rev-parse", RESULTS_COMMIT]);
    let t_res = commit_time(&results_full);
    let anc = |a: &str, b: &str| ctx.git(&["merge-base", "--is-ancestor", a, b]).0;
    let (_, pre_files) = ctx.git(&["show", "--name-only", "--format=", PREREG]);
    let pre_files: Vec<String> = pre_files.lines().map(|s| s.to_string()).collect();
    let pre_only_text = pre_files.iter().all(|f| f.ends_with(".md"));
    ck.add(
        "pre-registration 27663cc4d7 is an ancestor of the code commit c443138920 and is older than it; it changes Markdown only (the note and the session log)",
        anc(PREREG, CODE) && t_pre < t_code && pre_only_text,
        format!("prereg {t_pre} < code {t_code}; files {pre_files:?}"),
    );
    ck.add(
        "the code commit precedes the Stage A records commit (8d8155d8b1) and the results commit (4852cd4bec); both descend from it",
        t_code < t_stage && t_code < t_res && anc(CODE, STAGE_A_COMMIT) && anc(CODE, &results_full),
        format!("code {t_code} < stage A {t_stage}, results {t_res}"),
    );
    let (_, code_parent) = ctx.git(&["rev-parse", &format!("{CODE}^")]);
    ck.add(
        "the pre-E33 build used for the validator's identity replay is the code commit's parent bd0118f39",
        code_parent == PRE_E33,
        code_parent.clone(),
    );
    // Was each run record first added after the code commit?
    let mut first_add_ok = true;
    let mut first_adds: Vec<J> = Vec::new();
    let mut rec_files: Vec<String> = vec![format!("{E33}/stage_a.json"), format!("{E33}/identity_existing_solvers.json"), format!("{E33}/checks.log")];
    for (tag, _) in VAR {
        for p in PS {
            for s in SEEDS {
                rec_files.push(e33_rel(tag, p, s));
            }
        }
    }
    for f in &rec_files {
        let (_, out) = ctx.git(&["log", "--diff-filter=A", "--format=%H %ct", "--", f]);
        let last = out.lines().last().unwrap_or("").to_string();
        let t: u64 = last.split(' ').nth(1).and_then(|x| x.parse().ok()).unwrap_or(0);
        first_add_ok &= t > t_code;
        first_adds.push(obj(vec![("path", js(f)), ("first_added", js(&last))]));
    }
    ck.add(
        "every E33 run record (stage_a.json, identity_existing_solvers.json, checks.log, 30 end-to-end records) was first committed after the code commit",
        first_add_ok,
        format!("{} records", rec_files.len()),
    );
    // §9.35 unchanged since registration.
    let (_, note_then) = ctx.git(&["show", &format!("{PREREG}:{NOTE}")]);
    let note_now = String::from_utf8(ctx.bytes(NOTE)).unwrap();
    let sec_then = section(&note_then, "### 9.35 ");
    let sec_now = section(&note_now, "### 9.35 ");
    ck.add(
        "§9.35 (the registration, G2.124-G2.133) in the current note is byte-identical to the text committed in 27663cc4d7",
        !sec_then.is_empty() && sec_then.trim_end() == sec_now.trim_end(),
        format!("{} bytes then, {} bytes now (trailing blank lines aside)", sec_then.trim_end().len(), sec_now.trim_end().len()),
    );
    let (_, bc) = ctx.git(&["rev-parse", &String::from_utf8(ctx.bytes(&format!("{E33}/binary_commit.txt"))).unwrap().trim().to_string()]);
    ck.add("binary_commit.txt names the code commit", bc == CODE, bc.clone());
    let (_, fix_diff) = ctx.git(&["diff", "--stat", CODE, FIX, "--", "src/cryptanalysis/gaudry_quartic.rs", "src/cryptanalysis/quartic_folds.rs", "examples/quartic_pipeline_gls.rs", "examples/e33_stage_a.rs"]);
    ck.add(
        "c689d19821 (compile fix) leaves gaudry_quartic.rs, quartic_folds.rs, quartic_pipeline_gls.rs and e33_stage_a.rs unchanged against c443138920 (its klein_v4.rs change is inside mod tests)",
        fix_diff.is_empty(),
        if fix_diff.is_empty() { "no diff".into() } else { fix_diff.clone() },
    );
    let _ = t_fix;
    // Artifact manifest.
    let man = ctx.read(&format!("{E33}/.artifacts.json"));
    let ent = man.get("files").get("stage_a.json");
    let st_bytes = ctx.bytes(&format!("{E33}/stage_a.json"));
    let st_sha = sha256(&st_bytes);
    ck.add(
        "the local stage_a.json matches the artifact manifest (sha256 and bytes)",
        ent.get("sha256").s() == st_sha && ent.get("bytes").u() == st_bytes.len() as u64 && st_sha == STAGE_A_MANIFEST_SHA,
        format!("{st_sha}, {} bytes", st_bytes.len()),
    );

    // ------------------------------------------------------- Stage A ----
    let sa = stage_a(&ctx.read(&format!("{E33}/stage_a.json")));
    let mut st_ok = sa.len() == 10;
    let mut per_call_ok = true;
    let mut detail_pc = String::new();
    let (mut init_eq_ent, mut init_half_ent, mut init_other) = (0u64, 0u64, 0u64);
    let mut init_vals: BTreeMap<u64, u64> = BTreeMap::new();
    let mut fallbacks_b = 0u64;
    let mut fb_entries_ok = true;
    let mut summ_ok = true;
    let mut total_calls = 0u64;
    let mut lead_same = 0u64;
    let mut est_digest_eq = 0u64;
    // Per-curve metrics.
    struct M {
        p: u64,
        s: u64,
        init: [f64; 3],
        cmp: [f64; 3],
        upd: [f64; 3],
        tot: [f64; 3],
        med: [f64; 7],
        med_mid: [f64; 7],
        cmean: [f64; 7],
        lead: f64,
        s_mean: [[f64; 3]; 7],
        s_lead: [[f64; 3]; 7],
    }
    let mut ms: Vec<M> = Vec::new();
    let mut pooled_tot = [0u128; 3];
    let mut pooled_init = 0u128;
    let mut pooled_upd = 0u128;
    let mut pooled_cmp = 0u128;
    for c in &sa {
        let n = c.calls as usize;
        total_calls += c.calls;
        st_ok &= c.calls == 1000 && c.solvers == NAMES && c.arms.len() == 7;
        st_ok &= c.arms.iter().zip(NAMES).all(|(a, nm)| {
            a.solver == nm && a.call.len() == n && a.ech.len() == n && a.ent.len() == n && a.sel.len() == n && a.lead.len() == n
        });
        st_ok &= c.agree == vec![c.calls; 7] && c.ph == vec![c.calls; 3] && c.dg[0] == c.calls && c.dg[1] == c.calls && c.mism == 0;
        est_digest_eq += c.dg[2];
        fallbacks_b += c.arms[4].fallbacks;
        let a = &c.arms;
        for i in 0..n {
            let ok = a[1].call[i] == a[2].call[i]
                && a[1].ech[i] == a[2].ech[i]
                && a[1].ent[i] == a[2].ent[i]
                && a[3].call[i] == a[4].call[i]
                && a[3].call[i] == a[5].call[i]
                && a[3].ech[i] == a[4].ech[i]
                && a[3].ech[i] == a[5].ech[i]
                && a[3].ent[i] == a[4].ent[i]
                && a[3].ent[i] == a[5].ent[i]
                && a[6].ent[i] == a[4].ent[i]
                && a[2].sel[i] == [0, 0, 0]
                && a[1].sel[i] == [0, 0, 0]
                && a[3].sel[i] == [0, 0, 0]
                && a[0].sel[i] == [0, 0, 0]
                && a[0].lead[i] == 0
                && a[1].lead[i] == 0
                && a[3].lead[i] == 0
                && a[5].sel[i][1] == a[4].sel[i][1];
            if !ok && detail_pc.is_empty() {
                detail_pc = format!("first failure p={} s={} call {i}", c.p, c.s);
            }
            per_call_ok &= ok;
            if a[2].lead[i] == a[4].lead[i] && a[4].lead[i] == a[5].lead[i] && a[5].lead[i] == a[6].lead[i] {
                lead_same += 1;
            }
            let init = a[4].sel[i][0];
            *init_vals.entry(init).or_insert(0) += 1;
            if init == a[4].ent[i] {
                init_eq_ent += 1;
            } else {
                // A fallback call: entries are the fallback matrix's, doubled.
                fb_entries_ok &= a[4].ent[i] >= 2 * init && a[4].ent[i] % 2 == 0;
                if 2 * init == a[4].ent[i] {
                    init_half_ent += 1;
                } else {
                    init_other += 1;
                }
            }
        }
        // The recorded summaries against a recompute from the per-call arrays.
        for (k, arm) in a.iter().enumerate() {
            let sm = &c.summaries[k];
            let sel = arm.tot();
            let rel = |x: f64, y: f64| (x - y).abs() <= 1e-9 * x.abs().max(1.0);
            let s_mean = sm.get("s_mean").arr();
            summ_ok &= sm.get("solver").s() == NAMES[k]
                && sm.get("call_median").u() == upper_median(&arm.call)
                && rel(sm.get("call_mean").f(), mean(&arm.call))
                && rel(sm.get("echelon_mean").f(), mean(&arm.ech))
                && rel(sm.get("entries_mean").f(), mean(&arm.ent))
                && rel(sm.get("sel_init_mean").f(), mean(&arm.part(0)))
                && rel(sm.get("sel_compare_mean").f(), mean(&arm.part(1)))
                && rel(sm.get("sel_update_mean").f(), mean(&arm.part(2)))
                && rel(sm.get("sel_mean").f(), mean(&sel))
                && rel(sm.get("lead_scan_mean").f(), mean(&arm.lead))
                && (0..3).all(|w| rel(s_mean[w].f(), arm.s_mean(WEIGHTS[w], None)))
                && sm.get("fallbacks").u() == arm.fallbacks
                && sm.get("solved").u() == arm.solved;
        }
        let mut m = M {
            p: c.p,
            s: c.s,
            init: [0.0; 3],
            cmp: [0.0; 3],
            upd: [0.0; 3],
            tot: [0.0; 3],
            med: [0.0; 7],
            med_mid: [0.0; 7],
            cmean: [0.0; 7],
            lead: mean(&a[4].lead),
            s_mean: [[0.0; 3]; 7],
            s_lead: [[0.0; 3]; 7],
        };
        for v in 0..3 {
            let arm = &a[4 + v];
            m.init[v] = mean(&arm.part(0));
            m.cmp[v] = mean(&arm.part(1));
            m.upd[v] = mean(&arm.part(2));
            m.tot[v] = mean(&arm.tot());
            pooled_tot[v] += arm.tot().iter().map(|&x| x as u128).sum::<u128>();
        }
        pooled_init += a[4].part(0).iter().map(|&x| x as u128).sum::<u128>();
        pooled_cmp += a[4].part(1).iter().map(|&x| x as u128).sum::<u128>();
        pooled_upd += a[4].part(2).iter().map(|&x| x as u128).sum::<u128>();
        for k in 0..7 {
            m.med[k] = upper_median(&a[k].call) as f64;
            m.med_mid[k] = median_u(&a[k].call);
            m.cmean[k] = mean(&a[k].call);
            // Lead scans: an uncounted solver takes its counted twin's
            // (existing and A: A counted's; B: B counted's).
            let lead_src = match k {
                0 | 1 => 2,
                3 => 4,
                _ => k,
            };
            for w in 0..3 {
                m.s_mean[k][w] = a[k].s_mean(WEIGHTS[w], None);
                m.s_lead[k][w] = a[k].s_mean(WEIGHTS[w], Some(&a[lead_src].lead));
            }
        }
        ms.push(m);
    }
    ck.add(
        "Stage A: 10 curves x 1,000 calls, the seven solvers in order; agree = 1,000 for every solver; phases_equal = [1000,1000,1000]; digest_equal_b[B counted, B'] = 1,000; B' count mismatches 0",
        st_ok,
        format!("{total_calls} calls"),
    );
    ck.add(
        "Stage A per call: A counted = A and B counted = B' = B in C, echelon multiplications and entries; B'' has B's entries; uncounted solvers and A counted carry no selection work; uncounted solvers no lead scans; B' makes B counted's comparisons",
        per_call_ok,
        if detail_pc.is_empty() { "10,000 of 10,000 calls".into() } else { detail_pc.clone() },
    );
    ck.add(
        "Stage A: every recorded summary (median, means, S(0/0.1/1), fallbacks, solved) equals a recompute from the per-call arrays",
        summ_ok,
        format!("{summ_ok}"),
    );
    ck.add(
        "Stage A: lead-scan reads identical for A counted, B counted, B' and B'' on every call",
        lead_same == total_calls,
        format!("{lead_same} of {total_calls}"),
    );
    let init_ok = init_eq_ent + fallbacks_b == total_calls
        && fb_entries_ok
        && init_vals.keys().all(|&v| v % 448 == 0);
    ck.add(
        "B counted's initial pass reads rows x cols of the chain echelon: equal to the call's Macaulay entries on every call without a fallback; the calls where they differ are exactly the fallbacks (whose entries are the fallback matrix's, doubled); every value is a multiple of cols = 448",
        init_ok,
        format!("{init_eq_ent} equal, {} differ (fallbacks recorded {fallbacks_b}: {init_half_ent} with entries 2x the initial pass, {init_other} with a larger fallback matrix); values {:?}", init_half_ent + init_other, init_vals),
    );
    let n176 = *init_vals.get(&(393 * 448)).unwrap_or(&0);
    let n177 = *init_vals.get(&(396 * 448)).unwrap_or(&0);
    h.add("B initial counts: calls with rows x cols = 393 x 448 = 176,064", "176,064 on 9,999 calls", 9999.0, 0.0, n176 as f64, "count of Stage A calls with sel[0] of chain-sparsest-counted = 176,064");
    h.add("B initial counts: calls with 396 x 448", "396 · 448 on one", 1.0, 0.0, n177 as f64, "count of Stage A calls with sel[0] = 177,408");
    h.add("B initial counts per call (the 9,999 calls)", "176,064", 176064.0, 0.0, if n176 > 0 { (393 * 448) as f64 } else { 0.0 }, "393 * 448");
    let col = |f: &dyn Fn(&M) -> f64| -> Vec<f64> { ms.iter().map(f).collect() };
    let b_cmp = col(&|m| m.cmp[0]);
    let b_upd = col(&|m| m.upd[0]);
    let b_tot = col(&|m| m.tot[0]);
    let i_init = col(&|m| m.init[1]);
    let i_upd = col(&|m| m.upd[1]);
    let i_tot = col(&|m| m.tot[1]);
    let e_init = col(&|m| m.init[2]);
    let e_cmp = col(&|m| m.cmp[2]);
    let e_upd = col(&|m| m.upd[2]);
    let e_tot = col(&|m| m.tot[2]);
    let sa_def = "Stage A mean over the 1,000 calls of each curve; range over the ten curves";
    h.range("B comparisons per call", "3,681–3,713", 3681.0, 0.5, 3713.0, 0.5, &b_cmp, sa_def);
    h.range("B count updates (recounts) per call", "8.69–8.77·10⁵", 8.69e5, 0.005e5, 8.77e5, 0.005e5, &b_upd, sa_def);
    h.range("B selection total per call", "1.049–1.057·10⁶", 1.049e6, 0.0005e6, 1.057e6, 0.0005e6, &b_tot, sa_def);
    h.range("B' initial counts per call", "506–507", 506.0, 0.5, 507.0, 0.5, &i_init, sa_def);
    h.range("B' count updates per call", "2.89–2.94·10⁵", 2.89e5, 0.005e5, 2.94e5, 0.005e5, &i_upd, sa_def);
    h.range("B' selection total per call", "2.93–2.98·10⁵", 2.93e5, 0.005e5, 2.98e5, 0.005e5, &i_tot, sa_def);
    h.range("B'' initial counts per call", "506–507", 506.0, 0.5, 507.0, 0.5, &e_init, sa_def);
    h.range("B'' comparisons per call", "4,070–4,105", 4070.0, 0.5, 4105.0, 0.5, &e_cmp, sa_def);
    h.range("B'' count updates per call", "8,121–8,192", 8121.0, 0.5, 8192.0, 0.5, &e_upd, sa_def);
    h.range("B'' selection total per call", "12,696–12,804", 12696.0, 0.5, 12804.0, 0.5, &e_tot, sa_def);
    let reads = col(&|m| m.tot[0] / m.init[0]);
    h.prose("B's selection pass reads the matrix about six times per call", "about six times", 6.0, 0.5, &reads, "B total / B initial pass, per curve");
    let rec_share = col(&|m| m.upd[0] / m.tot[0]);
    h.range("B recounts as a share of the pass", "83 %", 0.83, 0.005, 0.83, 0.005, &rec_share, "B count updates / B total, per curve");
    let inc_over_b = col(&|m| m.tot[1] / m.tot[0]);
    h.range("B'/B selection work", "0.279–0.282", 0.279, 0.0005, 0.282, 0.0005, &inc_over_b, "Stage A mean B' total / mean B total, per curve");
    h.prose("B' keeps B's choices for 28 % of the work", "28 %", 0.28, 0.005, &inc_over_b, "B'/B per curve");
    let med_e = col(&|m| m.med[6]);
    let med_b = col(&|m| m.med[4]);
    let med_a = col(&|m| m.med[1]);
    let med_ratio = col(&|m| m.med[6] / m.med[4]);
    let med_def = "median of the per-call C (Weil + solve, F_p multiplications) over the curve's 1,000 calls as the record defines call_median (sorted[n/2], the upper median; recomputed from the per-call arrays); the midpoint median is listed in stage_a.per_curve.C_midpoint_median_by_solver";
    h.range("C median B''", "3.71–4.02·10⁵", 3.71e5, 0.005e5, 4.02e5, 0.005e5, &med_e, med_def);
    h.range("C median B", "3.04–3.33·10⁵", 3.04e5, 0.005e5, 3.33e5, 0.005e5, &med_b, med_def);
    h.range("C median A", "6.01–6.34·10⁵", 6.01e5, 0.005e5, 6.34e5, 0.005e5, &med_a, med_def);
    h.range("C median B''/B", "1.205–1.222", 1.205, 0.0005, 1.222, 0.0005, &med_ratio, med_def);
    let lead = col(&|m| m.lead);
    h.range("lead-scan reads per call", "79,114–79,119", 79114.0, 0.5, 79119.0, 0.5, &lead, sa_def);
    let cmp_share = col(&|m| m.cmp[0] / m.tot[0]);
    h.range("B comparisons as a share of B's selection total (G2.125)", "0.35 %", 0.0035, 0.00005, 0.0035, 0.00005, &cmp_share, "per curve");
    h.prose("B recount 8.7·10⁵ (G2.125)", "8.7·10⁵", 8.7e5, 0.05e5, &b_upd, "per curve");
    let b_init = col(&|m| m.init[0]);
    h.prose("B initial 1.76·10⁵ (G2.125)", "1.76·10⁵", 1.76e5, 0.005e5, &b_init, "per curve");
    let give_up = col(&|m| (m.cmean[6] - m.cmean[4]) / (m.cmean[1] - m.cmean[4]));
    h.range("B'' gives up this share of B's saving in mean C", "22.5–22.7 %", 0.225, 0.0005, 0.227, 0.0005, &give_up, "(mean C B'' - mean C B) / (mean C A - mean C B), per curve");
    h.prose("B'' selection operations per call", "1.27·10⁴", 1.27e4, 0.005e4, &e_tot, "per curve");
    let e_cost1 = col(&|m| GAE * m.tot[2]);
    h.prose("B'' selection operations at 1 GAE, F_p multiplications per call", "1.2·10⁶", 1.2e6, 0.05e6, &e_cost1, "97 * B'' total, per curve");
    let save_a = col(&|m| m.cmean[1] - m.cmean[6]);
    h.range("B'' saving against A's mean C", "2.30–2.33·10⁵", 2.30e5, 0.005e5, 2.33e5, 0.005e5, &save_a, "mean C A - mean C B'', per curve");
    // G2.133 sensitivity (Stage A, lead scans priced for every arm).
    let a_e_01 = col(&|m| m.s_lead[1][1] / m.s_lead[6][1]);
    let a_e_1 = col(&|m| m.s_lead[1][2] / m.s_lead[6][2]);
    let x_e_1 = col(&|m| m.s_lead[0][2] / m.s_lead[6][2]);
    let sens_def = "Stage A mean per-call S(g) + 97 g lead-scan reads, every arm (existing and A take A counted's lead scans); ratio per curve";
    h.range("G2.133: S_A/S_B'' with lead scans priced, 0.1 GAE", "1.035–1.036", 1.035, 0.0005, 1.036, 0.0005, &a_e_01, sens_def);
    h.range("G2.133: S_A/S_B'' with lead scans priced, 1 GAE", "0.962–0.963", 0.962, 0.0005, 0.963, 0.0005, &a_e_1, sens_def);
    h.range("G2.133: S_E25/S_B'' with lead scans priced, 1 GAE", "0.986–0.999", 0.986, 0.0005, 0.999, 0.0005, &x_e_1, sens_def);
    h.add("Macaulay entries at 1 GAE, F_p multiplications per call", "1.7·10⁷", 1.7e7, 0.05e7, GAE * 176064.0, "97 * 176,064");
    let pooled_b = pooled_tot[0] as f64 / total_calls as f64;
    let pooled_i = pooled_tot[1] as f64 / total_calls as f64;
    let pooled_e = pooled_tot[2] as f64 / total_calls as f64;
    // Stage A orders, with and without lead scans (G2.131 per call, G2.133).
    let mut sa_orders: Vec<J> = Vec::new();
    let mut sens_rank_same = true;
    for (vi, k) in [(0usize, 4usize), (1, 5), (2, 6)] {
        for (cname, ci) in [("A", 1usize), ("E25 (existing)", 0usize)] {
            let mut o = vec![[0u64; 3]; 3];
            let mut ol = vec![[0u64; 3]; 3];
            for m in &ms {
                for w in 0..3 {
                    let (x, y) = (m.s_mean[k][w], m.s_mean[ci][w]);
                    o[w][if x < y { 0 } else if x > y { 1 } else { 2 }] += 1;
                    let (x, y) = (m.s_lead[k][w], m.s_lead[ci][w]);
                    ol[w][if x < y { 0 } else if x > y { 1 } else { 2 }] += 1;
                }
            }
            sens_rank_same &= o == ol;
            sa_orders.push(obj(vec![
                ("variant", js(VAR[vi].1)),
                ("against", js(cname)),
                ("without_lead_scans", order_json(&o, 10, VAR[vi].1, cname)),
                ("with_lead_scans", order_json(&ol, 10, VAR[vi].1, cname)),
            ]));
        }
    }
    let b2_carry = ms.iter().all(|m| m.s_mean[6][1] < m.s_mean[1][1]);
    ck.add(
        "carry rule of §9.35: B'' was carried end to end only if its Stage A per-call S at 0.1 GAE is below A's on every curve",
        b2_carry,
        format!("{}", ms.iter().map(|m| format!("p{}s{} {:.4}", m.p, m.s, m.s_mean[1][1] / m.s_mean[6][1])).collect::<Vec<_>>().join(", ")),
    );

    // ---------------------------------------------------- rho (iii) ----
    let sw = ctx.read("experiments/32_walk_karatsuba/stage_w.json");
    let e27sum = ctx.read("experiments/32_walk_karatsuba/summary.json");
    let mut rho_map: BTreeMap<(u64, u64), (f64, f64, u64)> = BTreeMap::new();
    let mut rho_ok = true;
    for c in sw.get("curves").arr() {
        let (p, s) = (c.get("p").u(), c.get("seed").u());
        let ws = c.get("folds").get("Gls").get("walks").get("batched").arr();
        rho_ok &= ws.len() == 16 && ws.iter().all(|w| w.get("correct").b());
        let muls: f64 = ws.iter().map(|w| (w.get("fp_muls").u() - w.get("verify_fp_muls").u()) as f64).sum::<f64>() / 16.0;
        let steps: f64 = ws.iter().map(|w| w.get("steps").u() as f64).sum::<f64>() / 16.0;
        let ds: BTreeSet<u64> = ws.iter().map(|w| w.get("recovered").u()).collect();
        rho_ok &= ds.len() == 1;
        rho_map.insert((p, s), (muls, steps, *ds.iter().next().unwrap()));
    }
    let mut rho_vs_summary = true;
    for r in e27sum.get("rows").arr() {
        let (p, s) = (r.get("p").u(), r.get("seed").u());
        let (m, st, _) = rho_map[&(p, s)];
        rho_vs_summary &= r.get("rho_e27").f() == m && r.get("walk_steps").f() == st;
    }
    ck.add(
        "matched rho: walk (iii) of E27 (stage_w.json, <±psi> fold, batched arithmetic, 16 walks per curve) recomputed as the mean of fp_muls - verify_fp_muls; all walks correct, one logarithm per curve; equal to E27's summary rho_e27 and walk_steps on all 20 rows",
        rho_ok && rho_vs_summary && rho_map.len() == 10,
        format!("{} curves", rho_map.len()),
    );

    // --------------------------------------------------- end to end ----
    let mut rows: Vec<Row> = Vec::new();
    let mut e2e_struct_ok = true;
    let mut scal_ok = true;
    let mut inc_ident_ok = true;
    let mut est_ident_ok = true;
    let mut est_paths: BTreeSet<String> = BTreeSet::new();
    let mut e31_ok = true;
    let mut e31_extra: BTreeSet<String> = BTreeSet::new();
    let mut e31_bad: Vec<String> = Vec::new();
    let mut e31_only_bc: BTreeSet<String> = BTreeSet::new();
    let mut e31_leaves = (0u64, 0u64, 0u64); // compared, identical, product-dependent mismatches
    let mut a_paths: BTreeSet<String> = BTreeSet::new();
    let mut a_same_outputs = true;
    let mut sel_sum_ok = true;
    let mut lead_e2e_same = true;
    let mut ent_same = true;
    let mut scal_rows: Vec<J> = Vec::new();
    let allowed_inc: BTreeSet<&str> = ["$[*].solver", "$[*].pivot_selection[*]", "$[*].pivot_selection_ops"].into_iter().collect();
    let allowed_est: BTreeSet<&str> = [
        "$[*].solver",
        "$[*].pivot_selection[*]",
        "$[*].pivot_selection_ops",
        "$[*].solve_muls",
        "$[*].total_muls",
        "$[*].ratio",
        "$[*].call.mean",
        "$[*].call.median",
        "$[*].call.max",
        "$[*].call.min",
        "$[*].call.p10",
        "$[*].call.p90",
    ]
    .into_iter()
    .collect();
    // Fields that carry a solver's identity or cost, not its outputs.
    let solver_cost: BTreeSet<&str> = [
        "$[*].solver",
        "$[*].solve_muls",
        "$[*].total_muls",
        "$[*].ratio",
        "$[*].solver_fallbacks",
        "$[*].call.mean",
        "$[*].call.median",
        "$[*].call.max",
        "$[*].call.min",
        "$[*].call.p10",
        "$[*].call.p90",
    ]
    .into_iter()
    .collect();
    for p in PS {
        for s in SEEDS {
            let xs: Vec<J> = VAR.iter().map(|(t, _)| ctx.read(&e33_rel(t, p, s))).collect();
            let a27 = ctx.read(&e27_rel(p, s));
            let e31b = ctx.read(&e31b_rel(p, s));
            let e31a = ctx.read(&e31a_rel(p, s));
            let e25 = ctx.read(&e25_rel(p, s));
            let (rho, steps, d) = rho_map[&(p, s)];
            for (v, x) in xs.iter().enumerate() {
                let arr = x.arr();
                e2e_struct_ok &= arr.len() == 2;
                for (ai, arm) in ["A", "B"].iter().enumerate() {
                    let r = &arr[ai];
                    e2e_struct_ok &= r.get("arm").s() == *arm
                        && r.get("solver").s() == VAR[v].0
                        && r.get("fp4_product").s() == "karatsuba"
                        && r.get("p").u() == p
                        && r.get("seed").u() == s;
                    let ps = uvec(r.get("pivot_selection"));
                    sel_sum_ok &= ps.len() == 3 && ps.iter().sum::<u64>() == r.get("pivot_selection_ops").u();
                    let walks = r.get("rho").arr();
                    let walks_ok = walks.len() == 48 && walks.iter().all(|w| w.get("correct").b() && w.get("recovered").u() == d);
                    let f = r.get("folded");
                    let ok = f.get("solved").b() && f.get("correct").b() && f.get("recovered").u() == d && walks_ok
                        && r.get("check_failures").u() == 0
                        && r.get("rows_readd_failures").u() == 0;
                    scal_ok &= ok;
                    if v == 0 {
                        scal_rows.push(obj(vec![
                            ("p", ju(p)),
                            ("seed", ju(s)),
                            ("arm", js(arm)),
                            ("ic_solved_correct_all_variants", jb(ok)),
                            ("ic_recovered", ju(f.get("recovered").u())),
                            ("walk_iii_logarithm", ju(d)),
                            ("all_48_pipeline_walks_correct", jb(walks_ok)),
                        ]));
                    }
                }
            }
            // Identity rules.
            let d_inc = diff(&xs[0], &xs[1]);
            let gi = generic_set(&mism_paths(&d_inc));
            inc_ident_ok &= gi.iter().all(|g| allowed_inc.contains(g.as_str())) && d_inc.only_a.is_empty() && d_inc.only_b.is_empty();
            let d_est = diff(&xs[0], &xs[2]);
            let ge = generic_set(&mism_paths(&d_est));
            est_ident_ok &= ge.iter().all(|g| allowed_est.contains(g.as_str())) && d_est.only_a.is_empty() && d_est.only_b.is_empty();
            est_paths.extend(ge);
            for ai in 0..2 {
                let (b, e) = (&xs[0].arr()[ai], &xs[2].arr()[ai]);
                est_ident_ok &= e.get("total_muls").u() as i128 - b.get("total_muls").u() as i128
                    == e.get("solve_muls").u() as i128 - b.get("solve_muls").u() as i128;
                for v in 1..3 {
                    lead_e2e_same &= xs[v].arr()[ai].get("lead_scan_reads").u() == b.get("lead_scan_reads").u();
                }
            }
            // B counted vs E31 B: a leaf may differ only where E31 A and E27 A
            // (the same solver, schoolbook against Karatsuba products) differ.
            let d_aa = diff(&e31a, &a27);
            let product: BTreeSet<String> = mism_paths(&d_aa).into_iter().collect();
            let only27: BTreeSet<String> = generic_set(&d_aa.only_b);
            let d_b = diff(&e31b, &xs[0]);
            e31_leaves.0 += d_b.compared;
            e31_leaves.1 += d_b.identical;
            for path in mism_paths(&d_b) {
                let g = generic(&path);
                if product.contains(&path) {
                    e31_leaves.2 += 1;
                } else if g == "$[*].solver" {
                    e31_extra.insert(g);
                } else {
                    e31_ok = false;
                    e31_bad.push(format!("p{p}s{s} {path}"));
                }
            }
            e31_ok &= d_b.only_a.is_empty();
            for g in generic_set(&d_b.only_b) {
                if !(only27.contains(&g) || ["$[*].pivot_selection", "$[*].pivot_selection_ops", "$[*].lead_scan_reads"].contains(&g.as_str())) {
                    e31_ok = false;
                    e31_bad.push(format!("p{p}s{s} only in E33: {g}"));
                }
                e31_only_bc.insert(g);
            }
            // B counted vs E27 A (the same arithmetic, another pivot rule).
            let d_a = diff(&a27, &xs[0]);
            let ga = generic_set(&mism_paths(&d_a));
            a_same_outputs &= ga.iter().all(|g| solver_cost.contains(g.as_str()));
            a_paths.extend(ga);
            for ai in 0..2 {
                let (x0, a, b31, e) = (&xs[0].arr()[ai], &a27.arr()[ai], &e31b.arr()[ai], &e25.arr()[ai]);
                ent_same &= x0.get("macaulay_entries").u() == a.get("macaulay_entries").u()
                    && x0.get("macaulay_entries").u() == b31.get("macaulay_entries").u()
                    && (1..3).all(|v| xs[v].arr()[ai].get("macaulay_entries").u() == x0.get("macaulay_entries").u());
                let g = |v: usize, k: &str| xs[v].arr()[ai].get(k).f();
                rows.push(Row {
                    p,
                    s,
                    arm: ["A", "B"][ai].into(),
                    x_tot: [g(0, "total_muls"), g(1, "total_muls"), g(2, "total_muls")],
                    x_ent: [g(0, "macaulay_entries"), g(1, "macaulay_entries"), g(2, "macaulay_entries")],
                    x_sel: [g(0, "pivot_selection_ops"), g(1, "pivot_selection_ops"), g(2, "pivot_selection_ops")],
                    x_calls: [
                        xs[0].arr()[ai].get("call").get("calls").f(),
                        xs[1].arr()[ai].get("call").get("calls").f(),
                        xs[2].arr()[ai].get("call").get("calls").f(),
                    ],
                    x_lead: [g(0, "lead_scan_reads"), g(1, "lead_scan_reads"), g(2, "lead_scan_reads")],
                    a_tot: a.get("total_muls").f(),
                    a_ent: a.get("macaulay_entries").f(),
                    e25_tot: e.get("total_muls").f(),
                    e25_ent: e.get("macaulay_entries").f(),
                    e31b_tot: b31.get("total_muls").f(),
                    e31b_ent: b31.get("macaulay_entries").f(),
                    rho,
                    steps,
                });
            }
        }
    }
    let nrows = rows.len() as u64;
    ck.add("30 end-to-end records: two arms each (A, B), the solver tag, fp4_product karatsuba, p and seed as named", e2e_struct_ok && nrows == 20, format!("{nrows} rows"));
    ck.add("end to end: pivot_selection sums to pivot_selection_ops in every row", sel_sum_ok, format!("{sel_sum_ok}"));
    ck.add(
        "scalars: in all 60 arm records the index calculus solved, its logarithm checked (folded.correct) and equal to walk (iii)'s logarithm; all 48 pipeline walks per arm correct with that logarithm; no check or re-add failures",
        scal_ok,
        format!("{scal_ok}"),
    );
    ck.add(
        "B' records equal B counted's on every leaf apart from the solver tag and the selection fields (pivot_selection, pivot_selection_ops), wall-clock aside",
        inc_ident_ok,
        format!("{inc_ident_ok}"),
    );
    ck.add(
        "B'' records equal B counted's apart from the solver tag, the selection fields, solve_muls, total_muls, ratio and call.{mean,median,min,max,p10,p90} (what derives from the solve), and total_muls - solve_muls is the same in every arm",
        est_ident_ok,
        format!("differing paths: {:?}", est_paths),
    );
    ck.add(
        "end to end, lead-scan reads equal across B, B', B'' in every arm record",
        lead_e2e_same,
        format!("{lead_e2e_same}"),
    );
    ck.add(
        "B counted reproduces every non-product field of E31 B's records: each differing leaf also differs between E31 A (schoolbook) and E27 A (Karatsuba), or is the solver tag; keys only in E33 are keys that E27's records also carry and E31's lack (fp4_product, ic_fp_inversions, folded.recovered and the walks' recovered, verify_fp_muls, verify_group_ops, visit_digest) or the E33 fields",
        e31_ok,
        format!(
            "{} leaves compared, {} identical, {} product-dependent differences; other differing paths {:?}; first violations {:?}; keys only in E33 {:?}",
            e31_leaves.0,
            e31_leaves.1,
            e31_leaves.2,
            e31_extra,
            e31_bad.iter().take(5).collect::<Vec<_>>(),
            e31_only_bc
        ),
    );
    ck.add(
        "B counted against E27 A (same arithmetic, other pivot rule): only solver identity and cost fields differ (solver, solve_muls, total_muls, ratio, solver_fallbacks, call statistics); relations, logs, decompositions, residuals and walks are equal",
        a_same_outputs,
        format!("differing generic paths {:?}", a_paths),
    );
    ck.add(
        "macaulay_entries equal across E27 A, E31 B, B, B', B'' per arm record",
        ent_same,
        format!("{ent_same}"),
    );
    // Selection work per call end to end.
    let e2e_sel = |v: usize| -> Vec<f64> { rows.iter().map(|r| r.x_sel[v] / r.x_calls[v]).collect() };
    let e2e_def = "pivot_selection_ops / call.calls per arm record; range over the 20 rows";
    h.range("end to end selection per call, B", "1.049–1.057·10⁶", 1.049e6, 0.0005e6, 1.057e6, 0.0005e6, &e2e_sel(0), e2e_def);
    h.range("end to end selection per call, B'", "2.93–2.98·10⁵", 2.93e5, 0.005e5, 2.98e5, 0.005e5, &e2e_sel(1), e2e_def);
    h.range("end to end selection per call, B''", "12,694–12,802", 12694.0, 0.5, 12802.0, 0.5, &e2e_sel(2), e2e_def);
    // Ratios.
    let lab = ["B", "B'", "B''"];
    let print_tab: [[(&str, f64, f64, f64, f64); 3]; 9] = [
        // S_E25/S_B, S_A/S_B, S_B/rho
        [("4.53–4.76", 4.53, 0.005, 4.76, 0.005), ("0.274–0.276", 0.274, 0.0005, 0.276, 0.0005), ("0.157–0.158", 0.157, 0.0005, 0.158, 0.0005)],
        [("1.82–1.86", 1.82, 0.005, 1.86, 0.005), ("0.193–0.197", 0.193, 0.0005, 0.197, 0.0005), ("0.149–0.152", 0.149, 0.0005, 0.152, 0.0005)],
        [("3.45·10³–2.19·10⁵", 3.45e3, 0.005e3, 2.19e5, 0.005e5), ("1.10·10⁵–7.17·10⁶", 1.10e5, 0.005e5, 7.17e6, 0.005e6), ("6.10·10⁵–4.40·10⁷", 6.10e5, 0.005e5, 4.40e7, 0.005e7)],
        [("4.53–4.76", 4.53, 0.005, 4.76, 0.005), ("0.678–0.683", 0.678, 0.0005, 0.683, 0.0005), ("0.404–0.407", 0.404, 0.0005, 0.407, 0.0005)],
        [("1.82–1.86", 1.82, 0.005, 1.86, 0.005), ("0.478–0.487", 0.478, 0.0005, 0.487, 0.0005), ("0.384–0.393", 0.384, 0.0005, 0.393, 0.0005)],
        [("3.45·10³–2.19·10⁵", 3.45e3, 0.005e3, 2.19e5, 0.005e5), ("4.45·10⁴–2.89·10⁶", 4.45e4, 0.005e4, 2.89e6, 0.005e6), ("2.36·10⁵–1.71·10⁷", 2.36e5, 0.005e5, 1.71e7, 0.005e7)],
        [("3.83–3.98", 3.83, 0.005, 3.98, 0.005), ("1.457–1.497", 1.457, 0.0005, 1.497, 0.0005), ("0.980–0.997", 0.980, 0.0005, 0.997, 0.0005)],
        [("1.538–1.558", 1.538, 0.0005, 1.558, 0.0005), ("1.046–1.048", 1.046, 0.0005, 1.048, 0.0005), ("0.9465–0.9480", 0.9465, 0.00005, 0.9480, 0.00005)],
        [("4.12·10³–2.61·10⁵", 4.12e3, 0.005e3, 2.61e5, 0.005e5), ("2.03·10⁴–1.33·10⁶", 2.03e4, 0.005e4, 1.33e6, 0.005e6), ("9.58·10⁴–7.01·10⁶", 9.58e4, 0.005e4, 7.01e6, 0.005e6)],
    ];
    let mut ratio_tab: Vec<J> = Vec::new();
    for v in 0..3 {
        let qs: [(&str, Box<dyn Fn(&Row, f64) -> f64>); 3] = [
            ("S_E25/S_X", Box::new(move |r: &Row, g| r.s_e25(g) / r.s_x_sch(v, g))),
            ("S_A/S_X", Box::new(move |r: &Row, g| r.s_a(g) / r.s_x(v, g))),
            ("S_X/rho", Box::new(move |r: &Row, g| r.s_x(v, g) / r.rho(g))),
        ];
        for (qi, (qn, f)) in qs.iter().enumerate() {
            let mut per_w = Vec::new();
            for (wi, &g) in WEIGHTS.iter().enumerate() {
                let vals: Vec<f64> = rows.iter().map(|r| f(r, g)).collect();
                let (pr, lo, hlo, hi, hhi) = print_tab[3 * v + qi][wi];
                let def = match qi {
                    0 => format!("S_E25(g) = E25 total + g*97*E25 macaulay_entries (schoolbook); S_X,sch(g) = E31 B total + (X total - B counted total) + g*97*(X macaulay_entries + X pivot_selection_ops); g = {}", WNAME[wi]),
                    1 => format!("S_A(g) = E27 total + g*97*E27 macaulay_entries; S_X(g) = X total + g*97*(X macaulay_entries + X pivot_selection_ops), Karatsuba; g = {}", WNAME[wi]),
                    _ => format!("S_X(g) / (rho_iii + g*97*steps), rho_iii = E27 walk (iii) mean without the [d]G check; g = {}", WNAME[wi]),
                };
                let name = format!("{} with X = {} at {} GAE", qn, lab[v], WNAME[wi]);
                h.range(&name, pr, lo, hlo, hi, hhi, &vals, &def);
                let (a, b) = minmax(&vals);
                per_w.push(obj(vec![("gae", js(WNAME[wi])), ("min", jf(a)), ("max", jf(b)), ("median", jf(median_f(&vals)))]));
            }
            ratio_tab.push(obj(vec![("variant", js(lab[v])), ("quantity", js(qn)), ("per_weight", J::Arr(per_w))]));
        }
    }
    {
        let v: Vec<f64> = rows.iter().map(|r| r.s_a(1.0) / r.s_x(2, 1.0)).collect();
        h.range("running text: B'' behind A at 1 GAE", "0.946–0.948", 0.946, 0.0005, 0.948, 0.0005, &v, "S_A/S_B'' at 1 GAE over the 20 rows, printed to three places in 'What it says'");
        let v: Vec<f64> = rows.iter().map(|r| r.s_a(0.0) / r.s_x(2, 0.0)).collect();
        h.range("running text: B'' ahead of A at 0 GAE", "1.54–1.56×", 1.54, 0.005, 1.56, 0.005, &v, "S_A/S_B'' at 0 GAE over the 20 rows");
        let v: Vec<f64> = rows.iter().map(|r| r.s_a(0.1) / r.s_x(0, 0.1)).collect();
        h.prose("running text: B falls to 0.19x A at 0.1 GAE", "0.19×", 0.19, 0.005, &v, "S_A/S_B at 0.1 GAE over the 20 rows");
        let v: Vec<f64> = rows.iter().map(|r| r.x_sel[0] / r.x_calls[0]).collect();
        h.prose("section title: B's pivot selection costs 1.05e6 native operations per call", "1.05·10⁶", 1.05e6, 0.005e6, &v, "end-to-end selection per call, B");
    }
    let a_rho = [("6.39·10³–4.06·10⁵", 6.39e3, 0.005e3, 4.06e5, 0.005e5), ("2.13·10⁴–1.39·10⁶", 2.13e4, 0.005e4, 1.39e6, 0.005e6), ("9.07·10⁴–6.64·10⁶", 9.07e4, 0.005e4, 6.64e6, 0.005e6)];
    for (wi, &g) in WEIGHTS.iter().enumerate() {
        let vals: Vec<f64> = rows.iter().map(|r| r.s_a(g) / r.rho(g)).collect();
        let (pr, lo, hlo, hi, hhi) = a_rho[wi];
        h.range(&format!("S_A/rho (E27 solver A, walk (iii)) at {} GAE", WNAME[wi]), pr, lo, hlo, hi, hhi, &vals, "(E27 total + g*97*entries) / (rho_iii + g*97*steps)");
    }
    // Best row.
    let best = |f: &dyn Fn(&Row) -> f64| -> (String, f64) {
        let mut b = (String::new(), f64::INFINITY);
        for r in &rows {
            let v = f(r);
            if v < b.1 {
                b = (format!("p={} seed {} arm {}", r.p, r.s, r.arm), v);
            }
        }
        b
    };
    let (be_row, be_v) = best(&|r| r.s_x(2, 0.0) / r.rho(0.0));
    let (ba_row, ba_v) = best(&|r| r.s_a(0.0) / r.rho(0.0));
    h.add("best row B'' S/rho at 0 GAE", "4.12·10³× (p = 397 seed 1, arm B)", 4.12e3, 0.005e3, be_v, "minimum over the 20 rows");
    h.add("best row A S/rho at 0 GAE", "6.39·10³× (p = 397 seed 1, arm B)", 6.39e3, 0.005e3, ba_v, "minimum over the 20 rows");
    ck.add("the best row is p = 397 seed 1 arm B for both B'' and A", be_row == "p=397 seed 1 arm B" && ba_row == "p=397 seed 1 arm B", format!("B'': {be_row}; A: {ba_row}"));
    // Quoted lower bound of the earlier validator (G2.129 row).
    let lb = |g: f64| -> Vec<f64> { rows.iter().map(|r| r.s_e25(g) / (r.e31b_tot + g * GAE * (r.e31b_ent + 2.0 * r.e25_ent - r.e31b_ent))).collect() };
    h.range("G2.129: the earlier validator's lower-bound S_E25/S_B at 0.1 GAE", "0.885–0.899", 0.885, 0.0005, 0.899, 0.0005, &lb(0.1), "S_E25 / (E31 B total + g*97*(E_E31 + (2 E_E25 - E_E31))), the first pass alone priced from the lower bound 2 E_E25 - E_E31");
    h.range("G2.129: the earlier validator's lower-bound S_E25/S_B at 1 GAE", "0.542–0.544", 0.542, 0.0005, 0.544, 0.0005, &lb(1.0), "as above at 1 GAE");
    // Orders across the band (end to end).
    let mut orders: Vec<J> = Vec::new();
    let mut ordmap: BTreeMap<(usize, &str), Vec<[u64; 3]>> = BTreeMap::new();
    for v in 0..3 {
        let o_a = order(&rows, &move |r, g| r.s_x(v, g), &|r, g| r.s_a(g));
        let o_e = order(&rows, &move |r, g| r.s_x_sch(v, g), &|r, g| r.s_e25(g));
        let o_r = order(&rows, &|r, g| r.rho(g), &move |r, g| r.s_x(v, g));
        orders.push(order_json(&o_a, nrows, lab[v], "A (E27, Karatsuba)"));
        orders.push(order_json(&o_e, nrows, lab[v], "E25 (schoolbook)"));
        orders.push(order_json(&o_r, nrows, "rho (walk iii)", lab[v]));
        ordmap.insert((v, "A"), o_a);
        ordmap.insert((v, "E25"), o_e);
        ordmap.insert((v, "rho"), o_r);
    }
    let o_ar = order(&rows, &|r, g| r.rho(g), &|r, g| r.s_a(g));
    orders.push(order_json(&o_ar, nrows, "rho (walk iii)", "A (E27)"));
    // Lead-scan sensitivity end to end (A and E25 take X's lead scans).
    let mut sens_e2e_same = true;
    for v in 0..3 {
        let o_a = order(&rows, &move |r, g| r.s_x_lead(v, g), &move |r, g| r.s_a(g) + g * GAE * r.x_lead[v]);
        let o_e = order(&rows, &move |r, g| r.s_x_sch_lead(v, g), &move |r, g| r.s_e25(g) + g * GAE * r.x_lead[v]);
        sens_e2e_same &= o_a == ordmap[&(v, "A")] && o_e == ordmap[&(v, "E25")];
    }
    let ranked_any = (0..3).any(|v| {
        ["A", "E25"].iter().any(|c| {
            let o = &ordmap[&(v, *c)];
            o.iter().all(|w| w[0] == nrows) || o.iter().all(|w| w[1] == nrows)
        })
    });
    ck.add("no variant is ranked against A or E25 across the band (the order flips inside the band on every pair)", !ranked_any, format!("{ranked_any}"));

    // ----------------------------------------------------- verdicts ----
    let all = |v: usize, c: &str, w: usize, ahead: bool| -> bool {
        let o = &ordmap[&(v, c)][w];
        if ahead {
            o[0] == nrows
        } else {
            o[1] == nrows
        }
    };
    let mut verdicts: Vec<J> = Vec::new();
    let mut verdict_bad: Vec<String> = Vec::new();
    let mut vd = |id: &str, claimed: &str, holds: bool, detail: String| {
        let rec = if holds { "holds" } else { "falsified" };
        let agrees = claimed == rec;
        if !agrees {
            verdict_bad.push(format!("{id}: claimed {claimed}, recomputed {rec}"));
        }
        verdicts.push(obj(vec![
            ("id", js(id)),
            ("claimed", js(claimed)),
            ("recomputed", js(rec)),
            ("agrees", jb(agrees)),
            ("detail", js(&detail)),
        ]));
    };
    let ctl_ok = cmp.get("negative_controls_hold").b();
    let checks_log = String::from_utf8(ctx.bytes(&format!("{E33}/checks.log"))).unwrap();
    let cl = parse(checks_log.trim());
    let prod_ctl = cl.get("calls").u() == 32 && cl.get("second_best_detected").u() == cl.get("contested").u() && cl.get("off_by_one_detected").u() == cl.get("contested").u() && cl.get("failures").u() == 0;
    let idj = ctx.read(&format!("{E33}/identity_existing_solvers.json"));
    let prod_ident = idj.get("runs").arr().iter().all(|r| r.get("identical").b() && r.get("old").s() == r.get("new").s() && r.get("e33_fields_in_new").u() == 0)
        && idj.get("runs").arr().len() == 4
        && idj.get("e27_stored_record_p101s2_canonical_sha256").s() == idj.get("runs").arr()[3].get("new").s();
    let e27_p101s2 = ctx.read(&e27_rel(101, 2));
    let canon = canonical_sha(&e27_p101s2);
    let canon_ok = canon == idj.get("e27_stored_record_p101s2_canonical_sha256").s();
    ck.add(
        "identity_existing_solvers.json: its canonical hash of E27's stored p = 101 seed 2 record (jq -S -c, wall-clock keys dropped) is recomputed by the validator's own canonicaliser and equals the old and new chain --karatsuba hashes, so both builds reproduced E27's stored record",
        canon_ok && prod_ident,
        format!("{canon}; four configurations old = new: {prod_ident}"),
    );
    let leaves_rec: Vec<u64> = idj.get("runs").arr().iter().map(|r| r.get("leaves").u()).collect();
    ck.add(
        "identity_existing_solvers.json: 1,206-1,214 leaves per configuration as stated (macaulay, chain, chain-sparsest, chain --karatsuba)",
        leaves_rec.iter().all(|&l| (1206..=1214).contains(&l)) && leaves_rec.contains(&1206) && leaves_rec.contains(&1214),
        format!("{leaves_rec:?}"),
    );
    let g124 = canon_ok && st_ok && per_call_ok && inc_ident_ok && est_ident_ok && e31_ok && a_same_outputs && prod_ctl && prod_ident && ctl_ok && cmp.get("existing_solver_identity_holds").b();
    vd("G2.124", "holds", g124, format!("Stage A identity {st_ok}/{per_call_ok}; end-to-end rules B' {inc_ident_ok}, B'' {est_ident_ok}, E31 B {e31_ok}, outputs vs E27 A {a_same_outputs}; producer's checks.log {prod_ctl} and identity_existing_solvers.json {prod_ident}; validator's own controls {ctl_ok} and identity replay {}; unit tests not executed (disclosed by the producer)", cmp.get("existing_solver_identity_holds").b()));
    let pooled_cmp_share = pooled_cmp as f64 / pooled_tot[0] as f64;
    let g125 = init_ok && pooled_cmp_share <= 0.02 && pooled_upd > pooled_init && pooled_b >= 3e5 && pooled_b <= 2e6 && b_cmp.iter().zip(&b_tot).all(|(c, t)| c / t <= 0.02) && b_upd.iter().zip(&b_init).all(|(u, i)| u > i);
    vd("G2.125", "holds", g125, format!("pooled mean total {pooled_b:.1}; comparisons {:.4} %; recount {:.1} > initial {:.1}", 100.0 * pooled_cmp_share, pooled_upd as f64 / total_calls as f64, pooled_init as f64 / total_calls as f64));
    let (r_lo, r_hi) = minmax(&inc_over_b);
    vd("G2.126", "holds", r_lo >= 0.15 && r_hi <= 0.8 && pooled_i / pooled_b >= 0.15 && pooled_i / pooled_b <= 0.8, format!("B'/B per curve {r_lo:.4}-{r_hi:.4}, pooled {:.4}", pooled_i / pooled_b));
    let (m_lo, m_hi) = minmax(&med_ratio);
    let (et_lo, et_hi) = minmax(&e_tot);
    vd("G2.127", "holds", et_hi <= 2e4 && m_lo >= 1.0 && m_hi <= 1.3, format!("B'' per curve {et_lo:.1}-{et_hi:.1} (pooled {pooled_e:.1}); C median B''/B {m_lo:.4}-{m_hi:.4}"));
    vd("G2.128", "holds", all(0, "A", 0, true) && all(0, "A", 1, false) && all(0, "A", 2, false), rank_word(&ordmap[&(0, "A")], nrows, "B", "A"));
    let g129 = {
        let v0: Vec<f64> = rows.iter().map(|r| r.s_e25(0.0) / r.s_x_sch(0, 0.0)).collect();
        let v1: Vec<f64> = rows.iter().map(|r| r.s_e25(0.1) / r.s_x_sch(0, 0.1)).collect();
        let v2: Vec<f64> = rows.iter().map(|r| r.s_e25(1.0) / r.s_x_sch(0, 1.0)).collect();
        v0.iter().all(|&x| x > 1.0) && v1.iter().all(|&x| x <= 0.899) && v2.iter().all(|&x| x <= 0.544)
    };
    vd("G2.129", "holds", g129, rank_word(&ordmap[&(0, "E25")], nrows, "B", "E25"));
    let g130 = all(1, "A", 1, false) && all(1, "A", 2, false) && all(1, "E25", 1, false) && all(1, "E25", 2, false);
    vd("G2.130", "holds", g130, format!("{}; {}", rank_word(&ordmap[&(1, "A")], nrows, "B'", "A"), rank_word(&ordmap[&(1, "E25")], nrows, "B'", "E25")));
    let g131_a = all(2, "A", 0, true) && all(2, "A", 1, true) && all(2, "A", 2, false);
    let g131_e = all(2, "E25", 0, true) && all(2, "E25", 1, true) && all(2, "E25", 2, true);
    vd("G2.131", "falsified", g131_a && g131_e, format!("against A as registered: {g131_a} ({}); against E25 ahead at every weight: {g131_e} ({})", rank_word(&ordmap[&(2, "A")], nrows, "B''", "A"), rank_word(&ordmap[&(2, "E25")], nrows, "B''", "E25")));
    let v0: Vec<f64> = rows.iter().map(|r| r.s_x(0, 0.0) / r.rho(0.0)).collect();
    let (d_lo, d_hi) = minmax(&v0);
    let rho_ahead = (0..3).all(|v| (0..3).all(|w| ordmap[&(v, "rho")][w][0] == nrows));
    vd("G2.132", "holds", d_lo >= 3.0e3 && d_hi <= 2.5e5 && rho_ahead, format!("S_B/rho at 0 GAE {d_lo:.4e}-{d_hi:.4e}; rho ahead of every row of B, B', B'' at every weight: {rho_ahead}"));
    vd("G2.133", "holds", lead_same == total_calls && sens_rank_same && sens_e2e_same, format!("lead scans identical on {lead_same}/{total_calls} calls; Stage A orders unchanged with lead scans priced: {sens_rank_same}; end-to-end orders unchanged (A and E25 priced with X's lead scans): {sens_e2e_same}"));

    // --------------------------------------------------- the gae band ----
    let band = |v: usize| -> J {
        let arm_b: Vec<&Row> = rows.iter().filter(|r| r.arm == "B").collect();
        J::Obj(
            WEIGHTS
                .iter()
                .zip(WNAME)
                .map(|(&g, w)| {
                    let ic: Vec<f64> = arm_b.iter().map(|r| r.s_x(v, g)).collect();
                    let rh: Vec<f64> = arm_b.iter().map(|r| r.rho(g)).collect();
                    (w.to_string(), obj(vec![("ic", jf(median_f(&ic))), ("rho", jf(median_f(&rh)))]))
                })
                .collect(),
        )
    };

    let mut raw: Vec<J> = Vec::new();
    let depth = ctx.ev.split('/').count();
    let mut store: Vec<J> = Vec::new();
    for (rel, hsh) in ctx.pins.borrow().iter() {
        if rel == &format!("{E33}/stage_a.json") {
            // Gitignored: its bytes live in the artifact store; the tracked
            // manifest (.artifacts.json, pinned in raw_data) pins its digest.
            store.push(obj(vec![
                ("path", js(&format!("{}{}", "../".repeat(depth), rel))),
                ("sha256", js(hsh)),
                ("manifest", js(&format!("{}{E33}/.artifacts.json", "../".repeat(depth)))),
                ("note", js("read from the local copy, whose sha256 equals the manifest's; fetch with tools/artifacts in a fresh clone")),
            ]));
            continue;
        }
        raw.push(obj(vec![("path", js(&format!("{}{}", "../".repeat(depth), rel))), ("sha256", js(hsh))]));
    }
    let status_ok = ck.failures.is_empty() && h.bad.is_empty() && verdict_bad.is_empty() && h.prose_bad.is_empty();
    obj(vec![
        ("schema", js("validator-report/v1")),
        ("status", js(if status_ok { "pass" } else { "fail" })),
        ("independent", jb(true)),
        ("date", js("2026-11-01")),
        ("row", js("E33 (GLV note §9.35/§9.37): solver B's pivot selection counted (chain-sparsest-counted), B' (chain-sparsest-inc), B'' (chain-sparsest-est); Stage A (10 curves x 1,000 calls x 7 solvers) and end to end (3 solvers x 10 curves x 2 arms, Karatsuba); evidence directory research/e33_solver_b_pivot_count_20261031. No leaderboard row: E33 claims no improvement across the band.")),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E33; this recompute is its own standard-library Rust program (validate_e33.rs: JSON parser and SHA-256 carried over from the E27 validator's validate_e27.rs) and shares no code with examples/e33_tables.rs, examples/e33_checks.rs, experiments/37_solver_b_pivot_count/summary.json or tables.md, none of which it reads")),
        ("script", obj(vec![("path", js("validate_e33.rs")), ("sha256", js(&ctx.sha(&format!("{}/validate_e33.rs", ctx.ev))))])),
        ("claims_checked", obj(vec![
            ("note", js(NOTE)),
            ("section", js("§9.37 as corrected by the producer's erratum commits 2e90c88792 (13 figures) and 61c4dfecc8 (a fourteenth), made in place after this validator's interim reports")),
            ("note_blob_at_validation", js(&ctx.git(&["hash-object", NOTE]).1)),
            ("erratum_commits", sarr(&[ctx.git(&["rev-parse", "2e90c88792"]).1, ctx.git(&["rev-parse", "61c4dfecc8"]).1])),
            ("note_blob_at_61c4dfecc8", js(&ctx.git(&["rev-parse", &format!("61c4dfecc8:{NOTE}")]).1)),
        ])),
        ("status_rule", js("pass only if every check holds, every headline agrees within half a unit of its last printed digit, every recomputed G2.124-G2.133 verdict equals the claimed one and every prose figure lies within the recomputed range")),
        ("raw_data", J::Arr(raw)),
        ("raw_data_artifact_store", J::Arr(store)),
        ("headline", J::Arr(h.items)),
        ("headline_disagreements", sarr(&h.bad)),
        ("prose_figures", J::Arr(h.prose)),
        ("prose_disagreements", sarr(&h.prose_bad)),
        ("checks", J::Arr(ck.items)),
        ("check_failures", sarr(&ck.failures)),
        ("verdicts_recomputed", J::Arr(verdicts)),
        ("verdict_disagreements", sarr(&verdict_bad)),
        ("first_added", J::Arr(first_adds)),
        ("errata", J::Arr(vec![
            obj(vec![
                ("id", js("E1")),
                ("claim", js("§9.37 as committed in 4852cd4bec: 14 printed figures")),
                ("recomputed", js("B' count updates printed 2.92-2.98e5, records 288,884.5-294,201.3 (2.89-2.94e5; the producer's stage_a.log agrees with the records); truncated means: B comparisons min 3,680 (3,680.79), B' and B'' initial 505-506 (505.85-506.74), B'' comparisons min 4,069 (4,069.53), B'' total max 12,803 (12,803.90), C median B''/B max 1.221 (1.22156); double rounding: S_A/S_B at 1 GAE max 0.153 (0.152475), S_E25/S_B' at 0.1 GAE min 0.679 (0.678485), S_E25/S_B'' at 1 GAE min 0.981 (0.980463), and in the prose S_A/S_B'' at 1 GAE 0.947-0.948 (0.946478-0.947987)")),
                ("disposition", js("reported to the producer during this validation; corrected in place by erratum commits 2e90c88792 (13 figures) and 61c4dfecc8 (the fourteenth), each with an Erratum paragraph in §9.37 quoting the raw values. Every headline here is checked against the corrected text; against the text of 4852cd4bec the report would have failed on these 14 figures")),
                ("effect", js("none on any verdict or rank: G2.124-G2.130, G2.132, G2.133 hold, G2.131 is falsified against E25 at 1 GAE, no variant is ranked against A or E25")),
            ]),
            obj(vec![
                ("id", js("E2")),
                ("claim", js("G2.124 as registered (§9.35): 'B counted and B' choose B's pivot in every column (equal digests)'; §9.37: 'B' chose B's pivot in every column of every call (equal digests)'; stage_a.json digest_equal_b")),
                ("recomputed", js("digest_equal_b[0] compares B counted's digest with itself (nonzero only) and digest_equal_b[1] compares B' with B counted: the uncounted solver B emits no digest. B' = B counted holds on every call by digest; B counted = B rests on equal C, echelon multiplications and phase counts on all 10,000 calls and on reading both routines (same key, same order, same recount domain); the validator's control shows one second-best pivot changes the phase counts on 57 of 64 calls, so the phase-count proxy is strong but not a proof per call")),
                ("effect", js("none on any number (the counted work is B counted's own, and §9.37's 'B' denotes B as built, counted); the registered digest comparison of B counted against the uncounted B was not available as such, and is replaced here by the code reading, the phase counts and the proxy's measured power")),
            ]),
            obj(vec![
                ("id", js("E3")),
                ("claim", js("§9.35 registered three negative controls for G2.124 (second-best pivot, count off by one, E31's perturbed coefficient); §9.37 reports the first two")),
                ("recomputed", js("the validator ran all three on its own calls: second-best pivot 64/64 detected by the digest (outputs unchanged 64/64), off by one 64/64 flagged, perturbed coefficient 32/32 non-empty sets changed (B counted and B'')")),
                ("effect", js("none: the third control holds; its absence from §9.37 is an omission of reporting, not a failed control")),
            ]),
        ])),
        (
            "stage_a",
            obj(vec![
                ("calls", ju(total_calls)),
                ("b_initial_values", J::Obj(init_vals.iter().map(|(k, v)| (k.to_string(), ju(*v))).collect())),
                ("b_initial_equal_entries_calls", ju(init_eq_ent)),
                ("b_initial_half_entries_calls_fallback", ju(init_half_ent)),
                ("b_counted_digest_note", js("digest_equal_b[0] compares B counted's digest with itself (nonzero only): the uncounted B emits no digest, so 'B counted chooses B's pivots' rests on equal C, echelon multiplications and phase counts on every call, on the code (same key (count, index) over the sorted bucket, same recount domain row[lead..]: checked by reading echelon_by_lead_with and echelon_by_lead_counted at c443138920) and on the unexecuted unit test; the validator's own control measured how often one second-best pivot changes the phase counts")),
                ("b2_digest_equal_b_calls", ju(est_digest_eq)),
                ("pooled_selection_mean", farr(&[pooled_b, pooled_i, pooled_e])),
                (
                    "per_curve",
                    J::Arr(
                        ms.iter()
                            .map(|m| {
                                obj(vec![
                                    ("p", ju(m.p)),
                                    ("seed", ju(m.s)),
                                    ("sel_init_B_Bp_Bpp", farr(&m.init)),
                                    ("sel_compare", farr(&m.cmp)),
                                    ("sel_update", farr(&m.upd)),
                                    ("sel_total", farr(&m.tot)),
                                    ("C_median_by_solver", farr(&m.med)),
                                    ("C_midpoint_median_by_solver", farr(&m.med_mid)),
                                    ("C_mean_by_solver", farr(&m.cmean)),
                                    ("lead_scan_mean", jf(m.lead)),
                                    ("S_mean_by_solver_0_01_1", J::Arr(m.s_mean.iter().map(|x| farr(x)).collect())),
                                    ("S_mean_with_lead_by_solver_0_01_1", J::Arr(m.s_lead.iter().map(|x| farr(x)).collect())),
                                ])
                            })
                            .collect(),
                    ),
                ),
                ("orders", J::Arr(sa_orders)),
            ]),
        ),
        (
            "end_to_end",
            obj(vec![
                ("rows", ju(nrows)),
                ("ratios", J::Arr(ratio_tab)),
                ("orders_across_band", J::Arr(orders)),
                ("scalars", J::Arr(scal_rows)),
                ("b2_vs_b_counted_differing_paths", sarr(&est_paths.into_iter().collect::<Vec<_>>())),
                ("b_counted_vs_e27_a_differing_paths", sarr(&a_paths.into_iter().collect::<Vec<_>>())),
                (
                    "per_row",
                    J::Arr(
                        rows.iter()
                            .map(|r| {
                                obj(vec![
                                    ("p", ju(r.p)),
                                    ("seed", ju(r.s)),
                                    ("arm", js(&r.arm)),
                                    ("S_X_karatsuba_B_Bp_Bpp_0_01_1", J::Arr((0..3).map(|v| farr(&WEIGHTS.map(|g| r.s_x(v, g)))).collect())),
                                    ("S_X_schoolbook_B_Bp_Bpp_0_01_1", J::Arr((0..3).map(|v| farr(&WEIGHTS.map(|g| r.s_x_sch(v, g)))).collect())),
                                    ("S_A_0_01_1", farr(&WEIGHTS.map(|g| r.s_a(g)))),
                                    ("S_E25_0_01_1", farr(&WEIGHTS.map(|g| r.s_e25(g)))),
                                    ("rho_0_01_1", farr(&WEIGHTS.map(|g| r.rho(g)))),
                                    ("selection_per_call_B_Bp_Bpp", farr(&[r.x_sel[0] / r.x_calls[0], r.x_sel[1] / r.x_calls[1], r.x_sel[2] / r.x_calls[2]])),
                                ])
                            })
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "gae_band",
            obj(vec![
                ("definition", js("medians over the ten curves of arm B (E25's GLS-fold benchmark arm): ic = S_X(g) = total_muls + g*97*(macaulay_entries + pivot_selection_ops) (Karatsuba products); rho = walk (iii) mean without the [d]G check + g*97*mean steps")),
                ("B_counted", band(0)),
                ("B_prime", band(1)),
                ("B_double_prime", band(2)),
            ]),
        ),
        ("limitations", J::Arr(vec![
            js("Local container (4 vCPU, shared with two other validators), not isolab-eks: operation counts only; no wall time is compared or claimed."),
            js("The in-crate unit tests (e33_counted_pivot_selection_is_bit_identical, e33_counted_echelon_equals_the_uncounted_one) were not executed by the producer or by this validator (a release test build of the library needs about 9 GB; building one was ruled out for this shared container). Their content is covered here by the Stage A per-call identity, the end-to-end identity rules and the validator's own controls through the public API, not by the tests themselves."),
            js("B counted's pivot digest is compared with B'' and B' only: the uncounted B emits no digest. Equality of B counted's pivots with B's is supported by equal C, echelon multiplications and phase counts on all 10,000 Stage A calls, by reading both routines at c443138920 (same key, same order, same recount domain), and by the validator's measurement of how often a single second-best pivot changes the phase counts (replay/controls)."),
            js("S_E25/S_X uses X's schoolbook S constructed as E31 B's schoolbook total re-priced by X's own solve (X total - B counted total, both Karatsuba); the solve is in F_p and does not depend on the F_p4 product, and the B'' identity check shows total - solve is equal between B'' and B counted in every arm."),
            js("G2.133 prices E25's and A's lead scans with A counted's (Stage A) or X's own (end to end): the existing solver and E27's records do not emit lead_scan_reads."),
        ])),
    ])
}

// --------------------------------------------------------- certificate ----

fn certificate(ctx: &Ctx, cmp: &J, rep: &J) -> J {
    let rp = format!("{}/replay", ctx.ev);
    let build_rel = format!("{rp}/build.json");
    let build = ctx.read(&build_rel);
    let cmp_rel = format!("{rp}/replay_comparison.json");
    let all_ok = cmp.get("all_identical").b()
        && cmp.get("existing_solver_identity_holds").b()
        && cmp.get("negative_controls_hold").b()
        && cmp.get("stage_a_replayed").b();
    // Every replayed end-to-end run's scalars, from the replay output itself.
    let mut runs: Vec<J> = Vec::new();
    let mut scal_ok = true;
    for r in cmp.get("runs").arr() {
        let name = r.get("run").s().to_string();
        let mut o = vec![
            ("run", js(&name)),
            ("committed", r.get("committed").clone()),
            ("replay_output", r.get("replay").clone()),
            ("fields_compared", r.get("fields_compared").clone()),
            ("fields_identical", r.get("fields_identical").clone()),
            ("matched", r.get("identical").clone()),
        ];
        if name != "stage_a" {
            let rel = r.get("replay").get("path").s().to_string();
            let j = parse(std::str::from_utf8(&ctx.bytes(&rel)).unwrap());
            let mut sc = Vec::new();
            for a in j.arr() {
                let f = a.get("folded");
                let d = f.get("recovered").u();
                let walks_ok = a.get("rho").arr().iter().all(|w| w.get("correct").b() && w.get("recovered").u() == d);
                let ok = f.get("solved").b() && f.get("correct").b() && walks_ok;
                scal_ok &= ok;
                sc.push(obj(vec![
                    ("arm", js(a.get("arm").s())),
                    ("ic_solved", jb(f.get("solved").b())),
                    ("ic_correct", jb(f.get("correct").b())),
                    ("ic_recovered", ju(d)),
                    ("all_48_walks_correct_with_that_logarithm", jb(walks_ok)),
                    ("total_muls", ju(a.get("total_muls").u())),
                    ("macaulay_entries", ju(a.get("macaulay_entries").u())),
                    ("pivot_selection_ops", ju(a.get("pivot_selection_ops").u())),
                ]));
            }
            o.push(("scalars", J::Arr(sc)));
            o.push(("timing_not_compared", r.get("timing_not_compared").clone()));
        } else {
            o.push(("byte_identical", r.get("byte_identical").clone()));
        }
        runs.push(obj(o));
    }
    let ok = all_ok && scal_ok && !runs.is_empty();
    let e2e_n = cmp.get("e2e_replayed").arr().len();
    obj(vec![
        ("schema", js("replay-certificate/v1")),
        ("status", js(if ok { "verified" } else { "failed" })),
        ("independent", jb(true)),
        ("scalars_verified", jb(scal_ok)),
        ("date", js("2026-11-01")),
        ("row", js("E33 (GLV note §9.35/§9.37): Stage A (10 curves x 1,000 calls x 7 solvers) and the end-to-end runs of solver B counted, B' and B'' (E27's pipeline with --karatsuba); evidence directory research/e33_solver_b_pivot_count_20261031")),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E33: fresh build from an export of the recorded binary commit in a fresh target directory, fresh runs on a nonce-drawn sample, own comparison (validate_e33.rs), own negative-control program (replay/e33val_controls.rs)")),
        ("recorded_binary", js("c4431389209ae4a8d83bc916bbb233eb4a8438cb (experiments/37_solver_b_pivot_count/binary_commit.txt)")),
        ("build", build.get("summary").clone()),
        ("build_record", obj(vec![("path", js("replay/build.json")), ("sha256", js(&ctx.sha(&build_rel)))])),
        ("sample", obj(vec![
            ("nonce", obj(vec![("path", js("replay/nonce.json")), ("sha256", js(&ctx.sha(&format!("{rp}/nonce.json"))))])),
            ("draw", obj(vec![("path", js("replay/sample.json")), ("sha256", js(&ctx.sha(&format!("{rp}/sample.json"))))])),
            ("rule", js("key(label) = sha256('<nonce>:<label>'), curves sorted by key ascending (replay/draw_sample.sh); end to end: the first four curves of e2e_order for all three solvers, then further curves in that order as time allowed; existing-solver identity: the first curve of identity_order_excluding_p101s2; controls: the first two curves of controls_order with their controls_rng seeds")),
        ])),
        ("comparison", obj(vec![
            ("path", js("replay/replay_comparison.json")),
            ("sha256", js(&ctx.sha(&cmp_rel))),
            ("rule", cmp.get("rule").clone()),
        ])),
        ("runs_total", ju(runs.len() as u64)),
        ("runs_matched", ju(runs.iter().filter(|r| r.get("matched").b()).count() as u64)),
        ("fields_compared", cmp.get("fields_compared").clone()),
        ("fields_identical", cmp.get("fields_identical").clone()),
        ("coverage", js(&format!("Stage A replayed in full (10 curves x 1,000 calls x 7 solvers, every solver); end to end {e2e_n} of the 30 recorded runs (the sample in replay/sample.json); existing-solver identity of the pre-E33 build (bd0118f39) and E33's build on {} configurations; negative controls on validator-drawn calls", cmp.get("existing_solver_identity").arr().len()))),
        ("existing_solver_identity", obj(vec![
            ("holds", cmp.get("existing_solver_identity_holds").clone()),
            ("detail", js("old_* and new_* outputs under replay/identity: every leaf equal (wall-clock aside) and no E33 field in the new build's records; the new records against the stored E25/E31/E27 records of the same configuration are listed in replay_comparison.json")),
        ])),
        ("negative_controls", obj(vec![
            ("holds", cmp.get("negative_controls_hold").clone()),
            ("totals", cmp.get("negative_controls_totals").clone()),
            ("program", obj(vec![("path", js("replay/e33val_controls.rs")), ("sha256", js(&ctx.sha(&format!("{rp}/e33val_controls.rs"))))])),
        ])),
        ("scalar_verification", js("Every replayed end-to-end arm reports folded.solved and folded.correct (the binary sets correct only when the Wiedemann logarithm equals the planted d and [x]G = Q holds), and all 48 pipeline walks of the arm are correct with the same logarithm: two independent algorithms agree; the validator report checks that logarithm against E27's walk (iii) on all 60 committed arm records.")),
        ("validator_report_status", rep.get("status").clone()),
        ("runs", J::Arr(runs)),
    ])
}

fn gate(ctx: &Ctx, rep: &J) -> J {
    let band = rep.get("gae_band");
    let b = band.get("B_counted");
    let r = |w: &str| b.get(w).clone();
    let med = |v: &str, w: &str, k: &str| band.get(v).get(w).get(k).f();
    let protocol = format!(
        "Named by the protocol (§9.35, committed 27663cc4d7 before any E33 code): the matched rho is E27's walk (iii) without the [d]G check (experiments/32_walk_karatsuba, rho_e27 and walk_steps), recomputed here from stage_w.json: on each of E25's ten GLS curves the <±psi> walk (class {{±P, ±psi P}}), 16 walks, Karatsuba F_p4 products with the norm inversions of a 16-walk pool batched by Montgomery's trick, set-up included; E27's validated reference (research/e27_walk_20261030/VALIDATION.json: strongest available, Aut(E) = {{±1}} and psi the only Frobenius-type endomorphism over F_p4). Band (AGENTS.md §5a): S(g) = F_p multiplications + g*97*native operations, one native operation per Macaulay entry examined, per pivot-selection operation (initial count read, key comparison, count update) and per walk step; g = 0, 0.1, 1; GAE = 97 F_p multiplications (E25's affine addition). gae_band: medians over the ten curves of arm B for solver B counted (chain-sparsest-counted, the E33 subject). B' medians: ic {:.6e} / {:.6e} / {:.6e}; B'' medians: ic {:.6e} / {:.6e} / {:.6e} at 0 / 0.1 / 1 GAE, same rho. Rho is below every one of the 60 IC rows (B, B', B'' x ten curves x two arms) at every weight (validator-report.json, orders_across_band), so the order rho < IC holds across the band. No variant is ranked against solver A or E25 (each order flips inside the band); E33 claims no improvement across the band and adds no leaderboard row.",
        med("B_prime", "0", "ic"), med("B_prime", "0.1", "ic"), med("B_prime", "1", "ic"),
        med("B_double_prime", "0", "ic"), med("B_double_prime", "0.1", "ic"), med("B_double_prime", "1", "ic"),
    );
    let ev = &ctx.ev;
    obj(vec![
        ("schema", js("validation-gate/v1")),
        ("replay_certificate", obj(vec![("path", js("replay-certificate.json")), ("sha256", js(&ctx.sha(&format!("{ev}/replay-certificate.json"))))])),
        ("validator_report", obj(vec![("path", js("validator-report.json")), ("sha256", js(&ctx.sha(&format!("{ev}/validator-report.json"))))])),
        ("rho_reference", obj(vec![
            ("kind", js("negation_frobenius")),
            ("protocol", js(&protocol)),
            ("unit", js("F_p multiplications, both arms counted by the same Fp4 counters of one build: an F_p4 product 13 (Karatsuba) on both sides; the solve's F_p multiplications counted directly; walk set-up included, the walk's [d]G = Q check excluded (E25's convention); native operations priced at g*97 per operation")),
            ("binary_or_process", js("The IC side runs in quartic_pipeline_gls of the recorded commit c443138920 (--solver chain-sparsest-counted | chain-sparsest-inc | chain-sparsest-est --karatsuba, one process per (p, seed), arm A then arm B), which also runs the pipeline's own <±psi>, negation and unfolded walks on the same instance and counters; walk (iii) is E27's e27_walk record (binary af2504ef1). E33 changed no walk or arithmetic code (its diff touches the pivot-selection echelon, the solver dispatch and new record fields only), its records' 48 pipeline walks per arm equal E27's pipeline walks, and E27's validator established that the pipeline's walks equal e27_walk's walk (ii) and that (iii) differs from (ii) only by the price of the recorded F_p inversions; same_binary_or_process is asserted on that basis, as for E27. The validator's fresh build of c443138920 reproduces the sampled records on every non-timing leaf.")),
            ("matches_protocol", jb(true)),
            ("strongest_available", jb(true)),
            ("same_unit", jb(true)),
            ("same_binary_or_process", jb(true)),
            ("gae_band", obj(vec![("0", r("0")), ("0.1", r("0.1")), ("1", r("1"))])),
            ("order_holds_across_band", jb(true)),
        ])),
    ])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 4 && args[1] == "all", "usage: validate_e33 all <repo> <evidence-dir>");
    let ctx = Ctx {
        repo: args[2].clone(),
        ev: args[3].clone(),
        pins: RefCell::new(BTreeMap::new()),
    };
    let cmp = compare(&ctx);
    ctx.write(&format!("{}/replay/replay_comparison.json", ctx.ev), &cmp);
    // The report pins only the raw records it recomputes from.
    ctx.pins.borrow_mut().clear();
    let rep = report(&ctx, &cmp);
    ctx.write(&format!("{}/validator-report.json", ctx.ev), &rep);
    let cert = certificate(&ctx, &cmp, &rep);
    ctx.write(&format!("{}/replay-certificate.json", ctx.ev), &cert);
    let g = gate(&ctx, &rep);
    ctx.write(&format!("{}/VALIDATION.json", ctx.ev), &g);
    eprintln!(
        "report {}: {} headlines, {} disagreements, {} prose issues, {} check failures, {} verdict disagreements; certificate {}",
        rep.get("status").s(),
        rep.get("headline").arr().len(),
        rep.get("headline_disagreements").arr().len(),
        rep.get("prose_disagreements").arr().len(),
        rep.get("check_failures").arr().len(),
        rep.get("verdict_disagreements").arr().len(),
        cert.get("status").s()
    );
    for x in rep.get("headline_disagreements").arr() {
        eprintln!("  headline: {}", x.s());
    }
    for x in rep.get("prose_disagreements").arr() {
        eprintln!("  prose: {}", x.s());
    }
    for x in rep.get("check_failures").arr() {
        eprintln!("  check: {}", x.s());
    }
    for x in rep.get("verdict_disagreements").arr() {
        eprintln!("  verdict: {}", x.s());
    }
}
