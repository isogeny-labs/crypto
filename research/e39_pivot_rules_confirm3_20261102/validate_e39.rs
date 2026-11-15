//! E39 independent validator (2026-11-02).  Standard library only: its own
//! JSON parser, SHA-256 and binomial bound; shares no code with
//! `examples/e39_tables.rs`, the producer's scripts or crypto_lib.
//!
//! Modes:
//!   validate_e39 recompute <snapshot root> <stage_a.json> <note.md> <out.json>
//!   validate_e39 compare <committed.json> <replay.json>
//!   validate_e39 sha256 <file>...
//!
//! `recompute` reads only committed records under the snapshot root (a git
//! archive of the validated commit), the hash-checked Stage A file, and the
//! note (for the printed claims of §9.49).  It recomputes every quantity from
//! the raw per-call / per-run fields, compares each with the producer's
//! summary.json (relative 1e-9) and with the printed claims (half a unit of
//! the last printed digit), and writes everything to <out.json>.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;

// ------------------------------------------------------------------ JSON --

#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Num(f64, String),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

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
    fn expect(&mut self, c: u8) {
        self.ws();
        if self.s.get(self.i) != Some(&c) {
            panic!("json: expected '{}' at byte {}", c as char, self.i);
        }
        self.i += 1;
    }
    fn hex4(&mut self) -> u32 {
        let t = std::str::from_utf8(&self.s[self.i..self.i + 4]).unwrap();
        self.i += 4;
        u32::from_str_radix(t, 16).expect("json: \\u escape")
    }
    fn string(&mut self) -> String {
        self.expect(b'"');
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = self.s[self.i];
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = self.s[self.i];
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
                            let mut cp = self.hex4();
                            if (0xD800..0xDC00).contains(&cp) && self.s.get(self.i) == Some(&b'\\') {
                                self.i += 2;
                                let lo = self.hex4();
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                            }
                            let ch = char::from_u32(cp).unwrap_or('\u{FFFD}');
                            let mut b = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
                        }
                        _ => panic!("json: bad escape"),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).expect("json: utf8")
    }
    fn value(&mut self) -> J {
        self.ws();
        match self.s[self.i] {
            b'{' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.s[self.i] == b'}' {
                    self.i += 1;
                    return J::Obj(v);
                }
                loop {
                    let k = self.string();
                    self.expect(b':');
                    let x = self.value();
                    v.push((k, x));
                    self.ws();
                    let c = self.s[self.i];
                    self.i += 1;
                    if c == b'}' {
                        break;
                    }
                    assert_eq!(c, b',', "json: object separator");
                }
                J::Obj(v)
            }
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.s[self.i] == b']' {
                    self.i += 1;
                    return J::Arr(v);
                }
                loop {
                    v.push(self.value());
                    self.ws();
                    let c = self.s[self.i];
                    self.i += 1;
                    if c == b']' {
                        break;
                    }
                    assert_eq!(c, b',', "json: array separator");
                }
                J::Arr(v)
            }
            b'"' => J::Str(self.string()),
            b't' => {
                self.i += 4;
                J::Bool(true)
            }
            b'f' => {
                self.i += 5;
                J::Bool(false)
            }
            b'n' => {
                self.i += 4;
                J::Null
            }
            _ => {
                let st = self.i;
                while self.i < self.s.len()
                    && matches!(self.s[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                {
                    self.i += 1;
                }
                let t = std::str::from_utf8(&self.s[st..self.i]).unwrap().to_string();
                let f: f64 = t.parse().unwrap_or_else(|_| panic!("json: number {t}"));
                J::Num(f, t)
            }
        }
    }
}

fn parse(text: &str) -> J {
    let mut p = Parser { s: text.as_bytes(), i: 0 };
    let v = p.value();
    p.ws();
    assert_eq!(p.i, p.s.len(), "json: trailing data");
    v
}

fn load(path: &str) -> J {
    parse(&fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}")))
}

impl J {
    fn get(&self, k: &str) -> &J {
        match self {
            J::Obj(v) => v
                .iter()
                .find(|(a, _)| a == k)
                .map(|(_, x)| x)
                .unwrap_or_else(|| panic!("missing key {k}")),
            _ => panic!("not an object for key {k}"),
        }
    }
    fn opt(&self, k: &str) -> Option<&J> {
        match self {
            J::Obj(v) => v.iter().find(|(a, _)| a == k).map(|(_, x)| x),
            _ => None,
        }
    }
    fn f(&self) -> f64 {
        match self {
            J::Num(f, _) => *f,
            _ => panic!("not a number: {self:?}"),
        }
    }
    fn u(&self) -> u64 {
        match self {
            J::Num(_, t) => t.parse().unwrap_or_else(|_| panic!("not an integer: {t}")),
            _ => panic!("not a number"),
        }
    }
    fn raw(&self) -> String {
        match self {
            J::Num(_, t) => t.clone(),
            J::Str(s) => s.clone(),
            J::Bool(b) => b.to_string(),
            J::Null => "null".into(),
            _ => "<composite>".into(),
        }
    }
    fn s(&self) -> &str {
        match self {
            J::Str(s) => s,
            _ => panic!("not a string"),
        }
    }
    fn b(&self) -> bool {
        match self {
            J::Bool(b) => *b,
            _ => panic!("not a bool"),
        }
    }
    fn a(&self) -> &Vec<J> {
        match self {
            J::Arr(v) => v,
            _ => panic!("not an array"),
        }
    }
}

fn esc(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => {
                let _ = write!(o, "\\u{:04x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn ser(v: &J, ind: usize, out: &mut String) {
    let pad = |n: usize| " ".repeat(n);
    match v {
        J::Null => out.push_str("null"),
        J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        J::Num(_, t) => out.push_str(t),
        J::Str(s) => out.push_str(&esc(s)),
        J::Arr(a) => {
            if a.iter().all(|x| !matches!(x, J::Arr(_) | J::Obj(_))) {
                out.push('[');
                for (i, x) in a.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    ser(x, ind, out);
                }
                out.push(']');
            } else {
                out.push_str("[\n");
                for (i, x) in a.iter().enumerate() {
                    out.push_str(&pad(ind + 1));
                    ser(x, ind + 1, out);
                    if i + 1 < a.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
                out.push_str(&pad(ind));
                out.push(']');
            }
        }
        J::Obj(o) => {
            out.push_str("{\n");
            for (i, (k, x)) in o.iter().enumerate() {
                out.push_str(&pad(ind + 1));
                out.push_str(&esc(k));
                out.push_str(": ");
                ser(x, ind + 1, out);
                if i + 1 < o.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&pad(ind));
            out.push('}');
        }
    }
}

fn num(x: f64) -> J {
    if x.is_finite() && x == x.trunc() && x.abs() < 1e15 {
        J::Num(x, format!("{}", x as i64))
    } else if x.is_finite() {
        J::Num(x, format!("{x:?}"))
    } else {
        J::Null
    }
}
fn st(s: &str) -> J {
    J::Str(s.to_string())
}
fn arr(v: Vec<J>) -> J {
    J::Arr(v)
}
fn nums(v: &[f64]) -> J {
    J::Arr(v.iter().map(|&x| num(x)).collect())
}
macro_rules! obj {
    ($($k:expr => $v:expr),* $(,)?) => { J::Obj(vec![$(($k.to_string(), $v)),*]) };
}

// --------------------------------------------------------------- SHA-256 --

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
    let mut m = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&bits.to_be_bytes());
    for chunk in m.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (x, y) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *x = x.wrapping_add(y);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

fn sha_file(path: &str) -> String {
    sha256(&fs::read(path).unwrap_or_else(|e| panic!("read {path}: {e}")))
}

// --------------------------------------------------------------- compare --

fn timing_key(k: &str) -> bool {
    k.ends_with("_ms") || k == "ms" || k.contains("wall")
}

/// Leaf-by-leaf comparison; timing keys skipped.  Returns (leaves, differing, paths).
fn cmp_leaves(a: &J, b: &J, path: &str, out: &mut (usize, usize, Vec<String>)) {
    match (a, b) {
        (J::Obj(x), J::Obj(y)) => {
            for (k, v) in x {
                if timing_key(k) {
                    continue;
                }
                match y.iter().find(|(kk, _)| kk == k) {
                    Some((_, w)) => cmp_leaves(v, w, &format!("{path}.{k}"), out),
                    None => {
                        out.0 += 1;
                        out.1 += 1;
                        out.2.push(format!("{path}.{k} (missing in replay)"));
                    }
                }
            }
            for (k, _) in y {
                if !timing_key(k) && !x.iter().any(|(kk, _)| kk == k) {
                    out.1 += 1;
                    out.2.push(format!("{path}.{k} (extra in replay)"));
                }
            }
        }
        (J::Arr(x), J::Arr(y)) => {
            if x.len() != y.len() {
                out.1 += 1;
                out.2.push(format!("{path} (length {} vs {})", x.len(), y.len()));
            }
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                cmp_leaves(v, w, &format!("{path}[{i}]"), out);
            }
        }
        (J::Obj(_), _) | (J::Arr(_), _) | (_, J::Obj(_)) | (_, J::Arr(_)) => {
            out.0 += 1;
            out.1 += 1;
            out.2.push(format!("{path} (type)"));
        }
        _ => {
            out.0 += 1;
            if a.raw() != b.raw() || std::mem::discriminant(a) != std::mem::discriminant(b) {
                out.1 += 1;
                out.2.push(format!("{path}: {} vs {}", a.raw(), b.raw()));
            }
        }
    }
}

// ------------------------------------------------------------ statistics --

fn ln_choose(n: u64, k: u64) -> f64 {
    let lf = |m: u64| (1..=m).map(|i| (i as f64).ln()).sum::<f64>();
    lf(n) - lf(k) - lf(n - k)
}
/// P(X >= k), X ~ Bin(n, q).
fn upper_tail(n: u64, k: u64, q: f64) -> f64 {
    (k..=n)
        .map(|j| (ln_choose(n, j) + j as f64 * q.ln() + (n - j) as f64 * (1.0 - q).ln()).exp())
        .sum()
}
/// One-sided 95 % Clopper–Pearson lower bound for k successes in n.
fn cp_lower(k: u64, n: u64) -> f64 {
    if k == 0 {
        return 0.0;
    }
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if upper_tail(n, k, mid) < 0.05 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

// ------------------------------------------------------------- the model --

const GAE: f64 = 97.0;
const GS: [f64; 3] = [0.0, 0.1, 1.0];
const GLS: [(u64, u64); 10] = [
    (101, 1), (101, 2), (149, 1), (149, 2), (197, 1), (197, 2), (269, 1), (269, 2), (397, 1), (397, 2),
];
const E24: [(u64, u64); 12] = [
    (101, 1), (101, 2), (149, 1), (149, 2), (197, 1), (197, 2), (269, 1), (269, 2), (397, 1), (397, 2),
    (521, 1), (521, 2),
];
/// End-to-end arms: (label, solver tag).
const ARMS: [(&str, &str); 7] = [
    ("A", "chain-counted"),
    ("E25", "macaulay"),
    ("F", "chain-fifo"),
    ("S", "chain-static"),
    ("H", "chain-fifo-static"),
    ("B2", "chain-sparsest-est"),
    ("L", "chain-lifo"),
];
const CHAIN: [&str; 6] = ["A", "F", "S", "H", "B2", "L"];

#[derive(Clone, Default)]
struct StageCurve {
    p: u64,
    seed: u64,
    // per solver name: means
    c: BTreeMap<String, f64>,
    ech: BTreeMap<String, f64>,
    ent: BTreeMap<String, f64>,
    sel: BTreeMap<String, f64>,
    lead: BTreeMap<String, f64>,
    cont: BTreeMap<String, f64>,
    ord: BTreeMap<String, f64>,
    upd: BTreeMap<String, f64>,
    fb: BTreeMap<String, u64>,
    median: BTreeMap<String, f64>,
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn median_of(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    s[n / 2]
}

struct Checks {
    list: Vec<(String, bool, String)>,
}
impl Checks {
    fn add(&mut self, name: &str, ok: bool, detail: String) {
        self.list.push((name.to_string(), ok, detail));
    }
    fn all(&self, prefix: &str) -> bool {
        self.list.iter().filter(|x| x.0.starts_with(prefix)).all(|x| x.1)
    }
}

/// Comparisons with the producer's summary.json.
struct Cmp {
    n: usize,
    bad: Vec<String>,
}
impl Cmp {
    fn eq(&mut self, what: &str, mine: f64, theirs: f64) {
        self.n += 1;
        let d = (mine - theirs).abs();
        let tol = 1e-9 * mine.abs().max(theirs.abs()).max(1e-300);
        if d > tol && !(mine == 0.0 && theirs == 0.0) {
            self.bad.push(format!("{what}: mine {mine:?} producer {theirs:?}"));
        }
    }
}

/// A printed claim against a recomputed value.
struct Head {
    v: Vec<J>,
    bad: Vec<String>,
    /// Printed values that disagree with the recomputation: errata, kept out
    /// of the gate's headline list and reported separately.
    errata: Vec<J>,
}
impl Head {
    /// `printed` is the text as printed; tolerance half a unit of its last digit.
    fn add(&mut self, name: &str, source: &str, printed: &str, recomputed: f64, definition: &str) {
        let (claimed, half) = parse_printed(printed);
        let tol = if claimed == 0.0 { 0.0 } else { (half / claimed.abs()) * (1.0 + 1e-12) };
        let ok = if claimed == 0.0 { recomputed == 0.0 } else { ((recomputed - claimed) / claimed).abs() <= tol };
        let e = obj! {
            "name" => st(name), "source" => st(source), "printed" => st(printed),
            "claimed" => num(claimed), "recomputed" => num(recomputed), "rel_tol" => num(tol),
            "agrees" => J::Bool(ok), "definition" => st(definition),
        };
        if ok {
            self.v.push(e);
        } else {
            self.bad.push(format!("{name}: printed {printed} recomputed {recomputed}"));
            self.errata.push(e);
        }
    }
    /// Exact count claims.
    fn count(&mut self, name: &str, source: &str, claimed: f64, recomputed: f64, definition: &str) {
        let ok = claimed == recomputed;
        if !ok {
            self.bad.push(format!("{name}: claimed {claimed} recomputed {recomputed}"));
        }
        self.v.push(obj! {
            "name" => st(name), "source" => st(source), "printed" => st(&format!("{claimed}")),
            "claimed" => num(claimed), "recomputed" => num(recomputed), "rel_tol" => num(0.0),
            "agrees" => J::Bool(ok), "definition" => st(definition),
        });
    }
}

/// Parse a printed number such as `1.4572`, `5.18·10³`, `8·10⁻⁵`, `0.990`:
/// (value, half a unit of the last printed digit).
fn parse_printed(t: &str) -> (f64, f64) {
    let t = t.trim().trim_matches('`');
    let (mant, exp) = match t.find("·10") {
        Some(i) => {
            let e: String = t[i + "·10".len()..]
                .chars()
                .map(|c| match c {
                    '⁰' => '0', '¹' => '1', '²' => '2', '³' => '3', '⁴' => '4', '⁵' => '5',
                    '⁶' => '6', '⁷' => '7', '⁸' => '8', '⁹' => '9', '⁻' => '-', c => c,
                })
                .collect();
            (&t[..i], e.parse::<i32>().unwrap_or_else(|_| panic!("exponent in {t}")))
        }
        None => (t, 0),
    };
    let m: f64 = mant.replace('%', "").parse().unwrap_or_else(|_| panic!("printed {t}"));
    let dec = mant.split('.').nth(1).map_or(0, |d| d.trim_end_matches('%').len()) as i32;
    let scale = 10f64.powi(exp);
    (m * scale, 0.5 * 10f64.powi(-dec) * scale)
}

fn backticks(s: &str) -> Vec<String> {
    s.split('`').enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, x)| x.to_string()).collect()
}

fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)))
}

struct Row {
    set: &'static str,
    p: u64,
    seed: u64,
    arm: String, // "A"/"B" for GLS, "-" for E24
    rec: BTreeMap<String, J>,
    s: BTreeMap<String, [f64; 3]>,
    s1: BTreeMap<String, [f64; 3]>,
    s2: BTreeMap<String, [f64; 3]>,
    s3: BTreeMap<String, [f64; 3]>,
    rho: f64,
    steps: f64,
    rho_g: [f64; 3],
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("sha256") => {
            for f in &args[2..] {
                println!("{}  {}", sha_file(f), f);
            }
        }
        Some("compare") => {
            let (a, b) = (load(&args[2]), load(&args[3]));
            let mut o = (0usize, 0usize, Vec::new());
            cmp_leaves(&a, &b, "$", &mut o);
            println!("{} {} {} {}", o.0, o.1, sha_file(&args[2]), sha_file(&args[3]));
            for p in o.2.iter().take(20) {
                println!("  {p}");
            }
        }
        Some("recompute") => recompute(&args[2], &args[3], &args[4], &args[5]),
        Some("cert") => cert(&args[2], &args[3], &args[4]),
        _ => eprintln!("usage: validate_e39 recompute|compare|sha256 ..."),
    }
}

fn recompute(root: &str, stage_path: &str, note_path: &str, out_path: &str) {
    let e39 = format!("{root}/experiments/46_pivot_rules_confirm3");
    let e37 = format!("{root}/experiments/44_pivot_rules_confirm");
    let mut ck = Checks { list: Vec::new() };
    let mut cmp = Cmp { n: 0, bad: Vec::new() };
    let mut head = Head { v: Vec::new(), bad: Vec::new(), errata: Vec::new() };
    let summary = load(&format!("{e39}/summary.json"));
    let mut out: Vec<(String, J)> = Vec::new();

    // ---------------------------------------------------------- Stage A --
    let stage_sha = sha_file(stage_path);
    let recorded_sha = fs::read_to_string(format!("{e39}/stage_a.json.sha256")).unwrap();
    ck.add(
        "stage_a.sha256",
        recorded_sha.starts_with(&stage_sha),
        format!("{stage_sha} vs recorded {}", recorded_sha.trim()),
    );
    let stage = load(stage_path);
    let mut curves: Vec<StageCurve> = Vec::new();
    let mut lowest_eq_total = 0usize;
    let mut fb_same_total = 0usize;
    let mut sel_zero_fl = true;
    let mut calls_total = 0usize;
    let mut agree_all = true;
    let mut stage_salt_ok = true;
    let mut pooled_cont: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let mut per_call_below: BTreeMap<String, [usize; 3]> = BTreeMap::new();
    let mut pooled_ord: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for (ci, c) in stage.a().iter().enumerate() {
        let (p, seed) = (c.get("p").u(), c.get("seed").u());
        assert_eq!((p, seed), GLS[ci], "Stage A curve order");
        stage_salt_ok &= c.get("stream_salt").u() == 0xE39A_0001;
        let calls = c.get("calls").u() as usize;
        calls_total += calls;
        agree_all &= c.get("agree").a().iter().all(|x| x.u() as usize == calls);
        let mut sc = StageCurve { p, seed, ..Default::default() };
        let arms = c.get("arms").a();
        let by = |name: &str| arms.iter().find(|a| a.get("solver").s() == name).unwrap();
        for a in arms {
            let name = a.get("solver").s().to_string();
            let call: Vec<f64> = a.get("call").a().iter().map(J::f).collect();
            assert_eq!(call.len(), calls);
            let ech: Vec<f64> = a.get("echelon").a().iter().map(J::f).collect();
            let ent: Vec<f64> = a.get("entries").a().iter().map(J::f).collect();
            let sel: Vec<f64> =
                a.get("sel").a().iter().map(|x| x.a().iter().map(J::f).sum()).collect();
            let lead: Vec<f64> = a.get("lead_scan").a().iter().map(J::f).collect();
            let diag = a.get("diag").a();
            let cont: Vec<f64> = diag.iter().map(|x| x.a()[0].f()).collect();
            let ord: Vec<f64> = diag.iter().map(|x| x.a()[1].f()).collect();
            let upd: Vec<f64> = diag.iter().map(|x| x.a()[2].f()).collect();
            if name == "chain-fifo" || name == "chain-lifo" {
                sel_zero_fl &= a.get("sel").a().iter().all(|x| x.a().iter().all(|y| y.u() == 0));
            }
            assert_eq!(a.get("fallback_calls").a().len() as u64, a.get("fallbacks").u());
            pooled_cont.entry(name.clone()).or_default().extend(&cont);
            pooled_ord.entry(name.clone()).or_default().extend(&ord);
            sc.c.insert(name.clone(), mean(&call));
            sc.median.insert(name.clone(), median_of(&call));
            sc.ech.insert(name.clone(), mean(&ech));
            sc.ent.insert(name.clone(), mean(&ent));
            sc.sel.insert(name.clone(), mean(&sel));
            sc.lead.insert(name.clone(), mean(&lead));
            sc.cont.insert(name.clone(), mean(&cont));
            sc.ord.insert(name.clone(), mean(&ord));
            sc.upd.insert(name.clone(), mean(&upd));
            sc.fb.insert(name.clone(), a.get("fallbacks").u());
        }
        // Paired per-call comparison against A counted: calls where X's per-call
        // S(g) = call + 97 g (entries + selection) is below A's.
        {
            let per_s = |a: &J, i: usize, g: f64| -> f64 {
                a.get("call").a()[i].f() + GAE * g * (a.get("entries").a()[i].f() + a.get("sel").a()[i].a().iter().map(J::f).sum::<f64>())
            };
            let ac = by("chain-counted");
            for (x, n) in [("F", "chain-fifo"), ("S", "chain-static"), ("H", "chain-fifo-static"), ("L", "chain-lifo")] {
                let xa = by(n);
                let e = per_call_below.entry(x.to_string()).or_insert([0usize; 3]);
                for (gi, g) in GS.iter().enumerate() {
                    e[gi] += (0..calls).filter(|&i| per_s(xa, i, *g) < per_s(ac, i, *g)).count();
                }
            }
        }
        // Positive control: A's rule through E36's echelon = A counted, per call.
        let (ac, lo) = (by("chain-counted"), by("chain-lowest-e36"));
        let mut eq = 0usize;
        for i in 0..calls {
            let same = ["call", "echelon", "lead_scan", "digest"]
                .iter()
                .all(|k| ac.get(k).a()[i].raw() == lo.get(k).a()[i].raw());
            eq += usize::from(same);
        }
        lowest_eq_total += eq;
        ck.add(
            &format!("G2.191.stageA.lowest_equals_counted.p{p}s{seed}"),
            eq == calls && c.get("lowest_equals_a").u() as usize == calls,
            format!("{eq}/{calls} per-call equal (call, echelon, lead_scan, digest); recorded {}", c.get("lowest_equals_a").raw()),
        );
        // Same fallback calls for every chain solver.
        let fa: Vec<String> = ac.get("fallback_calls").a().iter().map(J::raw).collect();
        let chain_names = ["chain", "chain-counted", "chain-sparsest-est", "chain-lowest-e36", "chain-fifo", "chain-static", "chain-fifo-static", "chain-lifo"];
        let same_fb = chain_names.iter().all(|n| {
            by(n).get("fallback_calls").a().iter().map(J::raw).collect::<Vec<_>>() == fa
        });
        fb_same_total += usize::from(same_fb);
        ck.add(
            &format!("G2.191.stageA.same_fallback_calls.p{p}s{seed}"),
            same_fb,
            format!("{} fallback calls, identical index lists for the 8 chain solvers", fa.len()),
        );
        // Producer comparison.
        let sp = summary.get("stage_a").a().iter().find(|x| x.get("p").u() == p && x.get("seed").u() == seed).unwrap();
        for (name, v) in sp.get("per").opt_obj() {
            let name = name.as_str();
            cmp.eq(&format!("stage_a p{p}s{seed} {name} c_mean"), sc.c[name], v.get("c_mean").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} c_median"), sc.median[name], v.get("c_median").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} echelon_mean"), sc.ech[name], v.get("echelon_mean").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} entries_mean"), sc.ent[name], v.get("entries_mean").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} sel_mean"), sc.sel[name], v.get("sel_mean").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} lead_scan_mean"), sc.lead[name], v.get("lead_scan_mean").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} contested_mean"), sc.cont[name], v.get("contested_mean").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} ordered_mean"), sc.ord[name], v.get("ordered_candidates_mean").f());
            cmp.eq(&format!("stage_a p{p}s{seed} {name} fallbacks"), sc.fb[name] as f64, v.get("fallbacks").f());
            let s = stage_s(&sc, name);
            for k in 0..4 {
                let key = ["s", "s_sigma1", "s_sigma2", "s_sigma3"][k];
                for g in 0..3 {
                    cmp.eq(&format!("stage_a p{p}s{seed} {name} {key}[{g}]"), s[k][g], v.get(key).a()[g].f());
                }
            }
        }
        curves.push(sc);
    }
    ck.add("G2.191.stageA.calls", calls_total == 10_000, format!("{calls_total} calls"));
    ck.add("G2.191.stageA.agree_recorded", agree_all, "every solver's recorded agreement count equals the curve's calls (recorded by the binary; reproduced by the replay)".into());
    ck.add("stageA.salt", stage_salt_ok, "stream_salt 0xE39A0001 on every curve".into());
    ck.add("G2.194.F_L_zero_every_call", sel_zero_fl, "F and L selection parts 0 on every one of the 10,000 calls".into());

    // Stage A ratios.
    let rule = |n: &str| match n {
        "F" => "chain-fifo", "S" => "chain-static", "H" => "chain-fifo-static", "L" => "chain-lifo",
        "B2" => "chain-sparsest-est", "A" => "chain-counted", "E25" => "existing", _ => panic!(),
    };
    let mut sa_rows: Vec<J> = Vec::new();
    let mut r_ca: BTreeMap<&str, Vec<f64>> = BTreeMap::new(); // C_A / C_X
    let mut r_s: BTreeMap<(&str, usize, usize), Vec<f64>> = BTreeMap::new(); // (X, sigma, g) -> S_A/S_X
    let mut r_e25: BTreeMap<(&str, usize, usize), Vec<f64>> = BTreeMap::new();
    let mut sel_s = Vec::new();
    let mut sel_h_minus_s = Vec::new();
    let mut c_b2_over_f = Vec::new();
    for sc in &curves {
        let sa = stage_s(sc, "chain-counted");
        let se = stage_s(sc, "existing");
        let mut row = vec![("p".to_string(), num(sc.p as f64)), ("seed".to_string(), num(sc.seed as f64))];
        for x in ["F", "S", "H", "L", "B2"] {
            let sx = stage_s(sc, rule(x));
            let ca = sc.c["chain-counted"] / sc.c[rule(x)];
            r_ca.entry(x).or_default().push(ca);
            let mut per = Vec::new();
            for k in 0..4 {
                for g in 0..3 {
                    r_s.entry((x, k, g)).or_default().push(sa[k][g] / sx[k][g]);
                    r_e25.entry((x, k, g)).or_default().push(se[k][g] / sx[k][g]);
                }
                per.push(nums(&[sa[k][0] / sx[k][0], sa[k][1] / sx[k][1], sa[k][2] / sx[k][2]]));
            }
            row.push((x.to_string(), obj! {"c_a_over_c_x" => num(ca), "s_a_over_s_x_primary_s1_s2_s3" => arr(per), "selection_per_call" => num(sc.sel[rule(x)])}));
        }
        sel_s.push(sc.sel["chain-static"]);
        sel_h_minus_s.push(sc.sel["chain-fifo-static"] - sc.sel["chain-static"]);
        c_b2_over_f.push(sc.c["chain-sparsest-est"] / sc.c["chain-fifo"]);
        sa_rows.push(J::Obj(row));
    }
    let c_l_over_a: Vec<f64> = r_ca["L"].iter().map(|x| 1.0 / x).collect();
    let c_f_over_b2: Vec<f64> = c_b2_over_f.iter().map(|x| 1.0 / x).collect();
    let inb = |v: &[f64], lo: f64, hi: f64| v.iter().all(|&x| x >= lo && x <= hi);
    ck.add("G2.194", sel_zero_fl && inb(&sel_s, 500.0, 525.0) && inb(&sel_h_minus_s, 325.0, 345.0),
        format!("S {:?}; H-S {:?}", minmax(&sel_s), minmax(&sel_h_minus_s)));
    ck.add("G2.195", inb(&r_ca["F"], 1.44, 1.58) && inb(&r_ca["S"], 1.38, 1.49) && inb(&r_ca["H"], 1.48, 1.59)
        && inb(&c_l_over_a, 1.36, 1.44) && inb(&c_f_over_b2, 1.00, 1.08),
        format!("C_A/C_F {:?} C_A/C_S {:?} C_A/C_H {:?} C_L/C_A {:?} C_F/C_B2 {:?}", minmax(&r_ca["F"]), minmax(&r_ca["S"]), minmax(&r_ca["H"]), minmax(&c_l_over_a), minmax(&c_f_over_b2)));
    let ahead_all = |x: &str, k: usize| (0..3).all(|g| r_s[&(x, k, g)].iter().all(|&r| r > 1.0));
    let g196 = ["F", "S", "H"].iter().all(|x| (0..4).all(|k| ahead_all(x, k)))
        && inb(&r_s[&("F", 0, 2)], 1.009, 1.016)
        && inb(&r_s[&("S", 0, 2)], 1.006, 1.011)
        && inb(&r_s[&("H", 0, 2)], 1.006, 1.011)
        && (0..3).all(|g| r_s[&("L", 0, g)].iter().all(|&r| r < 1.0));
    ck.add("G2.196", g196, "F,S,H ahead of A at every weight on every curve under primary and sigma1-3; 1 GAE bands; L behind A at every weight (primary)".into());

    // ------------------------------------------------------------ rho --
    let stage_w = load(&format!("{root}/experiments/32_walk_karatsuba/stage_w.json"));
    let walk_e24 = load(&format!("{root}/experiments/44_pivot_rules_confirm/walk_e24.json"));
    let walk_mean = |walks: &J| -> (f64, f64, bool) {
        let w = walks.a();
        let m: Vec<f64> = w.iter().map(|x| x.get("fp_muls").f() - x.get("verify_fp_muls").f()).collect();
        let s: Vec<f64> = w.iter().map(|x| x.get("steps").f()).collect();
        let ok = w.iter().all(|x| x.get("correct").b());
        assert_eq!(w.len(), 16);
        (mean(&m), mean(&s), ok)
    };
    let rho_of = |set: &str, p: u64, seed: u64| -> (f64, f64, bool, f64, f64) {
        if set == "gls" {
            let c = stage_w.get("curves").a().iter().find(|c| c.get("p").u() == p && c.get("seed").u() == seed).unwrap();
            let g = c.get("folds").get("Gls");
            let (m, s, ok) = walk_mean(g.get("walks").get("batched"));
            (m, s, ok, g.get("mean_fp_muls").get("batched").f(), g.get("mean_steps").f())
        } else {
            let c = walk_e24.get("curves").a().iter().find(|c| c.get("p").u() == p && c.get("seed").u() == seed).unwrap();
            let (m, s, ok) = walk_mean(c.get("walks").get("batched"));
            (m, s, ok, c.get("mean_fp_muls").get("batched").f(), c.get("mean_steps").f())
        }
    };

    // ------------------------------------------------------ end to end --
    let pooled = |m: &BTreeMap<String, Vec<f64>>, n: &str| mean(&m[n]);
    let mut rows: Vec<Row> = Vec::new();
    let mut salt_ok = true;
    let mut runs_seen = 0usize;
    for (set, list) in [("gls", &GLS[..]), ("e24", &E24[..])] {
        for &(p, seed) in list {
            let recs: BTreeMap<&str, J> = ARMS
                .iter()
                .map(|(lab, solver)| {
                    runs_seen += 1;
                    (*lab, load(&format!("{e39}/e2e/{set}_{solver}_p{p}s{seed}.json")))
                })
                .collect();
            let arms: Vec<String> = if set == "gls" { vec!["A".into(), "B".into()] } else { vec!["-".into()] };
            for arm in arms {
                let pick = |j: &J| -> J {
                    let a = j.a();
                    if set == "gls" {
                        a.iter().find(|e| e.get("arm").s() == arm).unwrap().clone()
                    } else {
                        assert_eq!(a.len(), 1);
                        a[0].clone()
                    }
                };
                let rec: BTreeMap<String, J> = recs.iter().map(|(k, v)| (k.to_string(), pick(v))).collect();
                let (rho, steps, walks_ok, rho_rec, steps_rec) = rho_of(set, p, seed);
                ck.add(&format!("rho.walks_correct.{set}.p{p}s{seed}"), walks_ok, "all 16 batched walks correct".into());
                cmp.eq(&format!("rho {set} p{p}s{seed} vs record mean_fp_muls.batched"), rho, rho_rec);
                cmp.eq(&format!("rho steps {set} p{p}s{seed} vs record mean_steps"), steps, steps_rec);
                let calls = rec["A"].get("call").get("calls").f();
                ck.add(&format!("calls_equal_residuals.{set}.p{p}s{seed}.{arm}"), ARMS.iter().all(|(l, _)| rec[*l].get("call").get("calls").f() == rec[*l].get("residuals").f() && rec[*l].get("call").get("calls").f() == calls), "call.calls = residuals, equal for all seven arms".into());
                let sc = if set == "gls" { curves.iter().find(|c| c.p == p && c.seed == seed).cloned() } else { None };
                let ordm = |n: &str| match &sc { Some(c) => c.ord[n], None => pooled(&pooled_ord, n) };
                let contm = |n: &str| match &sc { Some(c) => c.cont[n], None => pooled(&pooled_cont, n) };
                let mut s = BTreeMap::new();
                let (mut s1, mut s2, mut s3) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
                for (lab, _) in ARMS {
                    let r = &rec[lab];
                    salt_ok &= r.get("stream_salt").s() == "0xe39e0001";
                    // E25 (macaulay) records carry no selection or lead-scan fields: its
                    // selection is 0 in the primary band; sigma1/sigma2 take A's.
                    let selops = if lab == "E25" {
                        assert!(r.opt("pivot_selection_ops").is_none() && r.opt("lead_scan_reads").is_none());
                        0.0
                    } else {
                        let sel_parts: u64 = r.get("pivot_selection").a().iter().map(J::u).sum();
                        ck.add(&format!("selection_parts_sum.{set}.p{p}s{seed}.{arm}.{lab}"), sel_parts == r.get("pivot_selection_ops").u(), format!("{sel_parts}"));
                        r.get("pivot_selection_ops").f()
                    };
                    let tm = r.get("total_muls").f();
                    let native = r.get("macaulay_entries").f() + selops;
                    let lead = if lab == "E25" { rec["A"].get("lead_scan_reads").f() } else { r.get("lead_scan_reads").f() };
                    let o2 = match lab { "A" | "E25" => ordm("chain-lowest-e36"), "S" => ordm("chain-static"), "H" => ordm("chain-fifo-static"), _ => 0.0 } * calls;
                    let o3 = match lab { "F" => contm("chain-fifo"), "S" => contm("chain-static"), "H" => contm("chain-fifo-static"), "L" => contm("chain-lifo"), _ => 0.0 } * calls;
                    let f = |extra: f64| [0, 1, 2].map(|g| tm + GAE * GS[g] * (native + extra));
                    s.insert(lab.to_string(), f(0.0));
                    s1.insert(lab.to_string(), f(lead));
                    s2.insert(lab.to_string(), f(o2));
                    s3.insert(lab.to_string(), f(o3));
                }
                let rho_g = [0, 1, 2].map(|g| rho + GAE * GS[g] * steps);
                rows.push(Row { set, p, seed, arm: arm.clone(), rec, s, s1, s2, s3, rho, steps, rho_g });
            }
        }
    }
    ck.add("e2e.runs", runs_seen == 154, format!("{runs_seen} pipeline records read"));
    ck.add("e2e.stream_salt", salt_ok, "every record's stream_salt is 0xe39e0001".into());

    // Producer comparison, per row.
    let sumrow = |r: &Row| -> &J {
        summary.get("e2e").a().iter().find(|x| x.get("set").s() == r.set && x.get("p").u() == r.p && x.get("seed").u() == r.seed
            && (r.set == "e24" || x.get("arm").s() == r.arm)).unwrap()
    };
    for r in &rows {
        let sr = sumrow(r);
        let tag = format!("{} p{}s{} {}", r.set, r.p, r.seed, r.arm);
        cmp.eq(&format!("e2e {tag} rho"), r.rho, sr.get("rho").f());
        cmp.eq(&format!("e2e {tag} walk_steps"), r.steps, sr.get("walk_steps").f());
        for g in 0..3 {
            cmp.eq(&format!("e2e {tag} rho_g[{g}]"), r.rho_g[g], sr.get("rho_g").a()[g].f());
        }
        for (lab, solver) in ARMS {
            let v = sr.get(solver);
            let calls = r.rec[lab].get("call").get("calls").f();
            cmp.eq(&format!("e2e {tag} {solver} calls"), calls, v.get("calls").f());
            cmp.eq(&format!("e2e {tag} {solver} c_mean"), r.rec[lab].get("call").get("mean").f(), v.get("c_mean").f());
            let selops = r.rec[lab].opt("pivot_selection_ops").map_or(0.0, J::f);
            cmp.eq(&format!("e2e {tag} {solver} selection_per_call"), selops / calls, v.get("selection_per_call").f());
            for (key, m) in [("S", &r.s), ("S_sigma1", &r.s1), ("S_sigma2", &r.s2), ("S_sigma3", &r.s3)] {
                for g in 0..3 {
                    cmp.eq(&format!("e2e {tag} {solver} {key}[{g}]"), m[lab][g], v.get(key).a()[g].f());
                }
            }
            for g in 0..3 {
                cmp.eq(&format!("e2e {tag} {solver} over_rho[{g}]"), r.s[lab][g] / r.rho_g[g], v.get("over_rho").a()[g].f());
            }
        }
    }

    // Bands, orders, sigma orders.
    let refs = ["A", "E25", "B2"];
    let xs = ["F", "S", "H", "B2", "L"];
    let mut bands: Vec<J> = Vec::new();
    let mut band_map: BTreeMap<(String, String, String), [(f64, f64); 3]> = BTreeMap::new();
    let mut ahead_map: BTreeMap<(String, String, String, usize), [usize; 3]> = BTreeMap::new();
    for set in ["gls", "e24", "all"] {
        let rs: Vec<&Row> = rows.iter().filter(|r| set == "all" || r.set == set).collect();
        for x in xs {
            for rf in refs {
                if x == rf {
                    continue;
                }
                let mut ranges = [(0.0, 0.0); 3];
                let mut ahead = [[0usize; 3]; 4];
                for g in 0..3 {
                    let v: Vec<f64> = rs.iter().map(|r| r.s[rf][g] / r.s[x][g]).collect();
                    ranges[g] = minmax(&v);
                    for (k, m) in [|r: &Row| r.s.clone(), |r: &Row| r.s1.clone(), |r: &Row| r.s2.clone(), |r: &Row| r.s3.clone()].iter().enumerate() {
                        ahead[k][g] = rs.iter().filter(|r| { let mm = m(r); mm[x][g] < mm[rf][g] }).count();
                    }
                }
                let n = rs.len();
                let order = |a: &[usize; 3]| {
                    if a.iter().all(|&c| c == n) { "X ahead across the band" } else if a.iter().all(|&c| c == 0) { "X behind across the band" } else { "unresolved" }
                };
                band_map.insert((set.into(), x.into(), rf.into()), ranges);
                for k in 0..4 {
                    ahead_map.insert((set.into(), x.into(), rf.into(), k), ahead[k]);
                }
                bands.push(obj! {
                    "set" => st(set), "x" => st(x), "reference" => st(rf), "rows" => num(n as f64),
                    "s_ref_over_s_x_ranges" => arr(ranges.iter().map(|(a, b)| nums(&[*a, *b])).collect()),
                    "rows_x_cheaper_primary" => nums(&ahead[0].map(|c| c as f64)),
                    "rows_x_cheaper_sigma1" => nums(&ahead[1].map(|c| c as f64)),
                    "rows_x_cheaper_sigma2" => nums(&ahead[2].map(|c| c as f64)),
                    "rows_x_cheaper_sigma3" => nums(&ahead[3].map(|c| c as f64)),
                    "order_primary" => st(order(&ahead[0])),
                    "order_sigma" => arr((1..4).map(|k| st(order(&ahead[k]))).collect()),
                });
            }
        }
    }
    // Producer's ranks comparison (ranges, counts).
    let xsolver = |x: &str| ARMS.iter().find(|a| a.0 == x).unwrap().1;
    for rk in summary.get("ranks").a() {
        let set = rk.get("set").s();
        let x = ARMS.iter().find(|a| a.1 == rk.get("x").s()).unwrap().0;
        let rf = ARMS.iter().find(|a| a.1 == rk.get("reference").s()).unwrap().0;
        if let Some(rg) = band_map.get(&(set.to_string(), x.to_string(), rf.to_string())) {
            for g in 0..3 {
                cmp.eq(&format!("ranks {set} {x}/{rf} min[{g}]"), rg[g].0, rk.get("ranges").a()[g].a()[0].f());
                cmp.eq(&format!("ranks {set} {x}/{rf} max[{g}]"), rg[g].1, rk.get("ranges").a()[g].a()[1].f());
                cmp.eq(&format!("ranks {set} {x}/{rf} rows ahead[{g}]"), ahead_map[&(set.to_string(), x.to_string(), rf.to_string(), 0)][g] as f64, rk.get("rows_x_ahead").a()[g].f());
            }
        }
    }
    let _ = xsolver;
    let g = |x: &str, rf: &str, k: usize| ahead_map[&("all".to_string(), x.to_string(), rf.to_string(), k)];
    let rank_x = |x: &str| g(x, "A", 0) == [32; 3] && g(x, "E25", 0) == [32; 3];
    let bm = |x: &str, rf: &str| band_map[&("all".to_string(), x.to_string(), rf.to_string())];
    let within = |r: (f64, f64), lo: f64, hi: f64| r.0 >= lo && r.1 <= hi;
    let g197 = within(bm("F", "A")[0], 1.43, 1.57) && within(bm("F", "A")[1], 1.085, 1.115) && within(bm("F", "A")[2], 1.010, 1.015)
        && within(bm("S", "A")[0], 1.38, 1.48) && within(bm("S", "A")[1], 1.075, 1.100) && within(bm("S", "A")[2], 1.006, 1.011)
        && within(bm("H", "A")[0], 1.47, 1.57) && within(bm("H", "A")[1], 1.087, 1.110) && within(bm("H", "A")[2], 1.006, 1.011)
        && ["F", "S", "H"].iter().all(|x| g(x, "E25", 0) == [32; 3])
        && within(bm("F", "E25")[0], 3.3, 4.2);
    ck.add("G2.197", g197, "F,S,H inside every registered band on all 32 rows; S_E25/S_F at 0 GAE in [3.3,4.2]".into());
    // G2.199: B2 ahead of F,S,H at 0 (S_B2/S_X in [0.92,1.0)), behind at 0.1 and 1.
    let g199 = ["F", "S", "H"].iter().all(|x| {
        let b = bm(x, "B2");
        g(x, "B2", 0)[0] == 0 && g(x, "B2", 0)[1] == 32 && g(x, "B2", 0)[2] == 32 && (*x != "F" || (b[0].0 >= 0.92 && b[0].1 < 1.0))
    });
    ck.add("G2.199", g199, "B2 cheaper than F,S,H at 0 GAE and dearer at 0.1 and 1 on all 32 rows; S_B2/S_F at 0 in [0.92,1)".into());
    // G2.200: L behind A at every weight; bands; under sigma2 L ahead of A at 1 GAE on every row.
    let g200 = g("L", "A", 0) == [0; 3] && within(bm("L", "A")[0], 0.68, 0.75) && within(bm("L", "A")[1], 0.88, 0.92)
        && within(bm("L", "A")[2], 0.980, 0.990) && g("L", "A", 2)[2] == 32;
    ck.add("G2.200", g200, format!("L vs A primary rows cheaper {:?}; sigma2 at 1 GAE {}", g("L", "A", 0), g("L", "A", 2)[2]));

    // Rho distances.
    let mut rho_ranges: Vec<J> = Vec::new();
    let mut rho_map: BTreeMap<(String, String), [(f64, f64); 3]> = BTreeMap::new();
    let mut below = 0usize;
    for set in ["gls", "e24", "all"] {
        for (lab, _) in ARMS {
            let rs: Vec<&Row> = rows.iter().filter(|r| set == "all" || r.set == set).collect();
            let mut rg = [(0.0, 0.0); 3];
            for g in 0..3 {
                let v: Vec<f64> = rs.iter().map(|r| r.s[lab][g] / r.rho_g[g]).collect();
                rg[g] = minmax(&v);
                if set == "all" {
                    below += rs.iter().filter(|r| r.s[lab][g] <= r.rho_g[g]).count();
                }
            }
            rho_map.insert((set.into(), lab.into()), rg);
            rho_ranges.push(obj! {"set" => st(set), "x" => st(lab), "s_x_over_rho_ranges" => arr(rg.iter().map(|(a, b)| nums(&[*a, *b])).collect())});
        }
    }
    for rr in summary.get("rho_ranges").a() {
        let set = rr.get("set").s();
        let lab = ARMS.iter().find(|a| a.1 == rr.get("x").s()).unwrap().0;
        let rg = rho_map[&(set.to_string(), lab.to_string())];
        for g in 0..3 {
            cmp.eq(&format!("rho_ranges {set} {lab} min[{g}]"), rg[g].0, rr.get("ranges").a()[g].a()[0].f());
            cmp.eq(&format!("rho_ranges {set} {lab} max[{g}]"), rg[g].1, rr.get("ranges").a()[g].a()[1].f());
        }
    }
    let rf = rho_map[&("all".to_string(), "F".to_string())];
    ck.add("G2.203", below == 0 && rf[0].0 >= 2e3 && rf[0].1 <= 4e5 && rf[2].0 >= 5e4 && rf[2].1 <= 1e7,
        format!("rows at or below rho (any arm, any weight): {below}; S_F/rho at 0 {:?}, at 1 {:?}", rf[0], rf[2]));

    // Health.
    let mut health: Vec<J> = Vec::new();
    let mut max_uns = (0.0f64, String::new());
    let mut max_fb = (0.0f64, String::new());
    let mut health_ok = true;
    let mut fb_equal_rows = 0usize;
    for r in &rows {
        let e37rec = load(&format!("{e37}/e2e/{}_chain-fifo_p{}s{}.json", r.set, r.p, r.seed));
        let e37e = if r.set == "gls" { e37rec.a().iter().find(|e| e.get("arm").s() == r.arm).unwrap().clone() } else { e37rec.a()[0].clone() };
        let e37_rate = e37e.get("solver_fallbacks").f() / e37e.get("call").get("calls").f();
        let ceil = e37_rate + 0.005;
        if r.set == "e24" && r.p == 101 && r.seed == 2 {
            ck.add("G2.198.e24_p101s2_registered_ceiling", ceil <= 0.0163 + 1e-12, format!("E37 rate {e37_rate} + 0.005 = {ceil} (registered 1.63 %)"));
        }
        let fa = r.rec["A"].get("solver_fallbacks").u();
        let same = CHAIN.iter().all(|x| r.rec[*x].get("solver_fallbacks").u() == fa);
        fb_equal_rows += usize::from(same);
        for (lab, solver) in ARMS {
            let e = &r.rec[lab];
            let calls = e.get("call").get("calls").f();
            let uns = e.get("unsolved").f() / calls;
            let fbs = e.opt("solver_fallbacks").map_or(0.0, J::f) / calls;
            let logs = e.get("folded").get("correct").b() && e.get("folded").get("solved").b();
            let walks = e.get("rho").a().iter().all(|w| w.get("correct").b());
            let tag = format!("{} p{}s{} {} {}", r.set, r.p, r.seed, r.arm, lab);
            if uns > max_uns.0 { max_uns = (uns, tag.clone()); }
            if lab != "E25" && fbs > max_fb.0 { max_fb = (fbs, tag.clone()); }
            let ok = logs && walks && uns <= 0.001 && (lab == "E25" || fbs <= ceil);
            health_ok &= ok;
            if let Some(h) = summary.get("health").a().iter().find(|h| h.get("set").s() == r.set && h.get("p").u() == r.p && h.get("seed").u() == r.seed
                && (r.set == "e24" || h.get("arm").s() == r.arm) && h.get("solver").s() == solver) {
                cmp.eq(&format!("health {tag} unsolved_share"), uns, h.get("unsolved_share").f());
                cmp.eq(&format!("health {tag} fallback_share"), fbs, h.get("fallback_share").f());
                if lab != "E25" {
                    cmp.eq(&format!("health {tag} fallback_ceiling"), ceil, h.get("fallback_ceiling").f());
                }
            }
            health.push(obj! {"row" => st(&tag), "calls" => num(calls), "unsolved_share" => num(uns), "fallback_share" => num(fbs),
                "fallback_ceiling" => if lab == "E25" { J::Null } else { num(ceil) }, "log_recovered" => J::Bool(logs), "walks_correct" => J::Bool(walks), "ok" => J::Bool(ok)});
        }
    }
    ck.add("G2.198", health_ok && fb_equal_rows == 32, format!("max unsolved {:?}; max chain fallback share {:?}; rows with equal chain fallbacks {fb_equal_rows}/32", max_uns, max_fb));

    // E2E identity: F, S, H, B2, L against A, leaf by leaf.
    let allowed = ["total_muls", "solve_muls", "ratio", "pivot_selection", "pivot_selection_ops", "lead_scan_reads", "solver", "call.mean", "call.median", "call.min", "call.max", "call.p10", "call.p90"];
    let mut id_rows = 0usize;
    let mut id_fields: BTreeMap<String, usize> = BTreeMap::new();
    let mut id_bad: Vec<String> = Vec::new();
    let mut leaves_total = 0usize;
    let mut e25_diff: BTreeMap<String, usize> = BTreeMap::new();
    for r in &rows {
        for x in ["F", "S", "H", "B2", "L", "E25"] {
            let mut o = (0usize, 0usize, Vec::new());
            cmp_leaves(&r.rec["A"], &r.rec[x], "", &mut o);
            if x != "E25" {
                leaves_total += o.0;
            }
            let mut ok = true;
            for pth in &o.2 {
                let key = pth.trim_start_matches('.').split(':').next().unwrap().split('[').next().unwrap().trim().to_string();
                if x == "E25" {
                    *e25_diff.entry(key).or_default() += 1;
                    continue;
                }
                *id_fields.entry(key.clone()).or_default() += 1;
                if !allowed.contains(&key.as_str()) {
                    ok = false;
                    id_bad.push(format!("{} p{}s{} {} {x}: {pth}", r.set, r.p, r.seed, r.arm));
                }
            }
            if x != "E25" {
                id_rows += usize::from(ok);
            }
        }
    }
    ck.add("G2.191.e2e_identity", id_rows == 160 && id_bad.is_empty(), format!("{id_rows}/160 (32 rows x 5 arms) equal to A outside the registered fields; differing keys {id_fields:?}"));

    // A/A against E37.
    let mut aa: Vec<J> = Vec::new();
    let mut aa_max = [0.0f64; 3];
    for r in &rows {
        let ld = |solver: &str| {
            let j = load(&format!("{e37}/e2e/{}_{solver}_p{}s{}.json", r.set, r.p, r.seed));
            if r.set == "gls" { j.a().iter().find(|e| e.get("arm").s() == r.arm).unwrap().clone() } else { j.a()[0].clone() }
        };
        let (a7, f7) = (ld("chain-counted"), ld("chain-fifo"));
        let s7 = |e: &J| [0, 1, 2].map(|g| e.get("total_muls").f() + GAE * GS[g] * (e.get("macaulay_entries").f() + e.opt("pivot_selection_ops").map_or(0.0, J::f)));
        let (sa7, sf7) = (s7(&a7), s7(&f7));
        let e37r = [0, 1, 2].map(|g| sa7[g] / sf7[g]);
        let e39r = [0, 1, 2].map(|g| r.s["A"][g] / r.s["F"][g]);
        let d = [0, 1, 2].map(|g| e39r[g] - e37r[g]);
        for g in 0..3 { aa_max[g] = aa_max[g].max(d[g].abs()); }
        if let Some(s) = summary.get("aa").a().iter().find(|h| h.get("set").s() == r.set && h.get("p").u() == r.p && h.get("seed").u() == r.seed && (r.set == "e24" || h.get("arm").s() == r.arm)) {
            for g in 0..3 {
                cmp.eq(&format!("aa {} p{}s{} {} e37[{g}]", r.set, r.p, r.seed, r.arm), e37r[g], s.get("e37").a()[g].f());
                cmp.eq(&format!("aa {} p{}s{} {} delta[{g}]", r.set, r.p, r.seed, r.arm), d[g], s.get("delta").a()[g].f());
            }
        }
        aa.push(obj! {"row" => st(&format!("{} p{}s{} {}", r.set, r.p, r.seed, r.arm)), "e37_stream_salt" => st(a7.get("stream_salt").s()), "s_a_over_s_f_e37" => nums(&e37r), "s_a_over_s_f_e39" => nums(&e39r), "delta" => nums(&d)});
    }
    ck.add("G2.202", aa_max[0] <= 0.05 && aa_max[1] <= 0.01 && aa_max[2] <= 0.002, format!("max |delta| {aa_max:?}"));

    // ---------------------------------------------------------- controls --
    let pilot = load(&format!("{e39}/pilot.json"));
    let checks = load(&format!("{e39}/checks.json"));
    let ctl = |d: &J, grp: &str, rule: &str| d.get(grp).get(rule).clone();
    let gating = [("c_f", "F"), ("c_f_post", "F"), ("c_sh", "S"), ("c_sh", "H")];
    let mut ctl_out: Vec<J> = Vec::new();
    let mut pilot_ok = pilot.get("fallbacks_used").a().is_empty() && pilot.get("identity_failures").u() == 0;
    let mut main_ok = checks.get("fallbacks_used").a().is_empty() && checks.get("identity_failures").u() == 0;
    for (tag, d, n) in [("pilot", &pilot, 80.0), ("main", &checks, 320.0)] {
        for (grp, rule) in [("c_f", "F"), ("c_f_post", "F"), ("c_sh", "S"), ("c_sh", "H"), ("c_0", "F"), ("c_0", "S"), ("c_0", "H"), ("c_0", "L")] {
            let t = ctl(d, grp, rule);
            let calls = t.get("calls").f();
            let pos = t.get("positive").f();
            let ne = t.get("nonempty").f();
            let ned = t.get("nonempty_detected_positive").f();
            let det = t.get("detected_positive").f();
            let pr = pos / calls;
            let nr = ned / ne;
            let dr = det / calls;
            let is_gating = gating.contains(&(grp, rule));
            let met = if tag == "pilot" { pos >= 76.0 && nr >= 0.90 } else { pr >= 0.95 && nr >= 0.96 };
            if is_gating {
                if tag == "pilot" { pilot_ok &= met; } else { main_ok &= met; }
            }
            assert_eq!(calls, n);
            if let Some(sc) = (if tag == "pilot" { summary.get("pilots").a()[0].get("controls").a() } else { summary.get("controls").a() })
                .iter().find(|c| c.get("control").s() == grp && c.get("rule").s() == rule) {
                cmp.eq(&format!("controls {tag} {grp} {rule} positive_rate"), pr, sc.get("positive_rate").f());
                cmp.eq(&format!("controls {tag} {grp} {rule} detection_rate"), dr, sc.get("detection_rate").f());
                cmp.eq(&format!("controls {tag} {grp} {rule} nonempty_detection_rate"), nr, sc.get("nonempty_detection_rate").f());
            }
            ctl_out.push(obj! {"run" => st(tag), "control" => st(grp), "rule" => st(rule), "check" => st(t.get("check").s()), "gating" => J::Bool(is_gating),
                "calls" => num(calls), "aimed" => num(t.get("aimed").f()), "positive" => num(pos), "positive_rate" => num(pr),
                "strict" => num(t.get("strict").f()), "pivots_changed" => num(t.get("pivots_changed").f()), "targets_changed" => num(t.get("targets_changed").f()),
                "structure_intact" => num(t.get("structure_intact").f()), "first_at_target" => num(t.get("first_at_target").f()), "nf_read_altered" => num(t.get("nf_read_altered").f()),
                "detected" => num(det), "detection_rate" => num(dr), "nonempty" => num(ne), "nonempty_detected" => num(ned), "nonempty_detection_rate" => num(nr),
                "via_fallback" => num(t.get("via_fallback").f()), "thresholds_met" => J::Bool(met)});
        }
    }
    for (tag, d) in [("pilot", &pilot), ("main", &checks)] {
        let ne = d.get("nonempty").u();
        let pert = ["F", "S", "H", "L"].iter().all(|x| d.get("perturbed_detected").get(x).u() == ne);
        let dig = ["F", "L"].iter().all(|x| d.get("digest_differs_from_a").get(x).u() == d.get("contested_calls_f").u());
        ck.add(&format!("G2.192.kept.{tag}"), pert && dig, format!("perturbed {}/{}; digests differ on {} of {} contested", d.get("perturbed_detected").get("F").raw(), ne, d.get("digest_differs_from_a").get("F").raw(), d.get("contested_calls_f").raw()));
        if tag == "pilot" { pilot_ok &= pert && dig; } else { main_ok &= pert && dig; }
    }
    let cfp = ctl(&pilot, "c_f_post", "F");
    pilot_ok &= cfp.get("aimed").u() == 80;
    ck.add("G2.190", pilot_ok, "every gating primary: positive >= 76/80 and non-empty detection >= 0.90 on P1; no fallback; C_F^post aimed 80/80".into());
    ck.add("G2.192", main_ok, "every gating control: positive >= 95% of 320 and non-empty detection >= 0.96; kept checks; no clean identity failure".into());
    let cf = ctl(&checks, "c_f", "F");
    let cp = ctl(&checks, "c_f_post", "F");
    let g193 = cf.get("positive").f() / 320.0 >= 0.99 && cf.get("strict").f() / 320.0 <= 0.05
        && cp.get("aimed").f() / 320.0 >= 0.99 && cp.get("positive").u() == cp.get("aimed").u()
        && cp.get("structure_intact").u() == cp.get("aimed").u() && cp.get("first_at_target").u() == cp.get("aimed").u();
    ck.add("G2.193", g193, format!("C_F P_ext {}/320 strict {}; C_F^post aimed {} P_post {} intact {} first-at-r* {}", cf.get("positive").raw(), cf.get("strict").raw(), cp.get("aimed").raw(), cp.get("positive").raw(), cp.get("structure_intact").raw(), cp.get("first_at_target").raw()));
    // Aims of the main run.
    let aims = checks.get("aims").a();
    let vv: Vec<f64> = aims.iter().map(|a| a.get("v").f()).collect();
    let ll: Vec<f64> = aims.iter().map(|a| a.get("l").f()).collect();
    let nn: Vec<f64> = aims.iter().map(|a| a.get("nf_reads_at_l").f()).collect();
    let cs: Vec<f64> = aims.iter().map(|a| a.get("c_star").f()).collect();
    let aimed_all = aims.iter().all(|a| !matches!(a.get("r"), J::Null) && !matches!(a.get("l"), J::Null));
    ck.add("G2.193.aims_complete", aimed_all && aims.len() == 320, format!("{} aims, l* and r* non-null on all", aims.len()));
    let ca = summary.get("control_aims");
    for (k, v) in [("v", &vv), ("l_star", &ll), ("nf_reads_at_l", &nn), ("c_star", &cs)] {
        let (a, b) = minmax(v);
        cmp.eq(&format!("control_aims {k} min"), a, ca.get(k).a()[0].f());
        cmp.eq(&format!("control_aims {k} max"), b, ca.get(k).a()[1].f());
    }
    // Threshold derivation (G2.192 gates): one-sided 95 % Clopper-Pearson.
    let cp480 = cp_lower(480, 480);
    let cp309 = cp_lower(309, 314);
    let cp195 = cp_lower(195, 199);

    // ----------------------------------------------- printed claims (9.49) --
    let note = fs::read_to_string(note_path).unwrap();
    let start = note.find("### 9.49 E39 measured").expect("9.49");
    let sec = &note[start..];
    let sec = &sec[..sec[10..].find("\n### ").map_or(sec.len(), |i| i + 10)];
    // Band table rows.
    let lab_of = |t: &str| match t { "F" => "F", "S" => "S", "H" => "H", "B″" => "B2", "L" => "L", "A" => "A", "E25" => "E25", _ => "" };
    let mut band_lines = 0usize;
    for line in sec.lines() {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        if cells.len() < 7 { continue; }
        let (x, rf) = (lab_of(cells[1]), lab_of(cells[2]));
        if x.is_empty() || rf.is_empty() || !["A", "E25", "B2"].contains(&rf) || x == "A" || x == "E25" { continue; }
        let Some(rg) = band_map.get(&("all".to_string(), x.to_string(), rf.to_string())) else { continue };
        band_lines += 1;
        for g in 0..3 {
            let t = backticks(cells[3 + g]);
            if t.len() < 2 { continue; }
            let w = ["0", "0.1", "1"][g];
            head.add(&format!("band S_{rf}/S_{x} at {w} GAE, min over 32 rows"), "note §9.49 band table", &t[0], rg[g].0, "S(g) = total_muls + 97 g (macaulay_entries + pivot_selection_ops) per record");
            head.add(&format!("band S_{rf}/S_{x} at {w} GAE, max over 32 rows"), "note §9.49 band table", &t[1], rg[g].1, "as above");
        }
    }
    ck.add("parse.band_table", band_lines == 13, format!("{band_lines} band lines parsed"));
    // Per-row distance table.
    let mut dist_lines = 0usize;
    for line in sec.lines() {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        if cells.len() < 14 || !(cells[1] == "GLS" || cells[1] == "E24") { continue; }
        let set = if cells[1] == "GLS" { "gls" } else { "e24" };
        let (p, seed): (u64, u64) = (cells[2].parse().unwrap(), cells[3].parse().unwrap());
        let arm = if set == "gls" { cells[4].to_string() } else { "-".to_string() };
        let r = rows.iter().find(|r| r.set == set && r.p == p && r.seed == seed && r.arm == arm).unwrap();
        dist_lines += 1;
        for (j, (lab, g)) in [("F", 0), ("S", 0), ("H", 0), ("A", 0), ("F", 2), ("S", 2), ("H", 2), ("A", 2)].iter().enumerate() {
            let t = backticks(cells[5 + j]);
            let w = ["0", "0.1", "1"][*g];
            head.add(&format!("S_{lab}/rho {set} p{p} s{seed} {arm} at {w} GAE"), "note §9.49 per-row distance table", &t[0], r.s[*lab][*g] / r.rho_g[*g], "rho(g) = mean walk (iii) F_p mults (no [d]G check) + 97 g mean steps");
        }
    }
    ck.add("parse.distance_table", dist_lines == 32, format!("{dist_lines} distance rows parsed"));
    // Rho range paragraph.
    let rp = sec.find("**Against the matched rho**").unwrap();
    let rpara = sec[rp..rp + sec[rp..].find("\n\n").unwrap()].replace('\n', " ");
    let rpara = rpara.as_str();
    let mut rho_claims = 0usize;
    for (lab, key) in [("F", " F `"), ("S", "; S `"), ("H", "; H `"), ("A", "; A `"), ("E25", "; E25 `"), ("B2", "; B″ `"), ("L", "; L `")] {
        let i = rpara.find(key).unwrap_or_else(|| panic!("rho paragraph {lab}"));
        let toks: Vec<String> = backticks(&rpara[i + key.len() - 1..]).into_iter().filter(|t| t.contains("·10")).take(6).collect();
        let rg = rho_map[&("all".to_string(), lab.to_string())];
        for g in 0..3 {
            let w = ["0", "0.1", "1"][g];
            head.add(&format!("S_{lab}/rho min over 32 rows at {w} GAE"), "note §9.49 'Against the matched rho'", &toks[2 * g], rg[g].0, "as the distance table");
            head.add(&format!("S_{lab}/rho max over 32 rows at {w} GAE"), "note §9.49 'Against the matched rho'", &toks[2 * g + 1], rg[g].1, "as the distance table");
            rho_claims += 2;
        }
    }
    ck.add("parse.rho_paragraph", rho_claims == 42, format!("{rho_claims}"));
    // Stage A paragraph (typed from the note).
    let src = "note §9.49 Stage A paragraph / G2.194-G2.196 rows";
    let mm = |v: &[f64]| minmax(v);
    let (a, b) = mm(&sel_s);
    head.add("Stage A S selection per call, min curve mean", src, "511.9", a, "mean over calls of the three selection parts");
    head.add("Stage A S selection per call, max curve mean", src, "512.7", b, "");
    let (a, b) = mm(&sel_h_minus_s);
    head.add("Stage A H - S selection, min", src, "333.95", a, "");
    head.add("Stage A H - S selection, max", src, "334.00", b, "");
    for (x, lo, hi) in [("F", "1.471", "1.548"), ("S", "1.411", "1.463"), ("H", "1.503", "1.563")] {
        let (a, b) = mm(&r_ca[x]);
        head.add(&format!("Stage A C_A/C_{x} min"), src, lo, a, "ratio of per-curve mean counted solve multiplications");
        head.add(&format!("Stage A C_A/C_{x} max"), src, hi, b, "");
    }
    let (a, b) = mm(&r_ca["L"]);
    head.add("Stage A C_A/C_L min", src, "0.710", a, "");
    head.add("Stage A C_A/C_L max", src, "0.721", b, "");
    let (a, b) = mm(&c_l_over_a);
    head.add("Stage A C_L/C_A min", "note §9.49 G2.195 row", "1.387", a, "");
    head.add("Stage A C_L/C_A max", "note §9.49 G2.195 row", "1.408", b, "");
    let (a, b) = mm(&c_b2_over_f);
    head.add("Stage A C_B2/C_F min", src, "0.952", a, "");
    head.add("Stage A C_B2/C_F max", src, "0.985", b, "");
    let (a, b) = mm(&c_f_over_b2);
    head.add("Stage A C_F/C_B2 min", "note §9.49 G2.195 row", "1.016", a, "");
    head.add("Stage A C_F/C_B2 max", "note §9.49 G2.195 row", "1.050", b, "");
    for (x, lo, hi) in [("F", "1.0116", "1.0128"), ("S", "1.0077", "1.0086"), ("H", "1.0076", "1.0083"), ("L", "0.9856", "0.9862")] {
        let (a, b) = mm(&r_s[&(x, 0, 2)]);
        head.add(&format!("Stage A S_A/S_{x} at 1 GAE min"), src, lo, a, "per-curve means, S = C + 97 g (entries + selection)");
        head.add(&format!("Stage A S_A/S_{x} at 1 GAE max"), src, hi, b, "");
    }
    for (k, lo, hi) in [(1, "1.0081", "1.0089"), (2, "1.0341", "1.0360"), (3, "1.0098", "1.0110")] {
        let (a, b) = mm(&r_s[&("F", k, 2)]);
        head.add(&format!("Stage A S_A/S_F sigma{k} at 1 GAE min"), src, lo, a, "sigma1 lead scans every arm; sigma2 |b|-1 per contested column for ordering arms; sigma3 one per contested column for F,S,H,L");
        head.add(&format!("Stage A S_A/S_F sigma{k} at 1 GAE max"), src, hi, b, "");
    }
    // Controls (typed from the note's table and text).
    let csrc = "note §9.49 controls table";
    let t = |d: &J, grp: &str, rule: &str, k: &str| ctl(d, grp, rule).get(k).f();
    head.count("pilot calls", csrc, 80.0, pilot.get("calls").f(), "");
    head.count("pilot non-empty calls", csrc, 56.0, pilot.get("nonempty").f(), "");
    for (grp, rule, nd) in [("c_f", "F", 55.0), ("c_f_post", "F", 55.0), ("c_sh", "S", 55.0), ("c_sh", "H", 56.0)] {
        head.count(&format!("pilot {grp} {rule} positive"), csrc, 80.0, t(&pilot, grp, rule, "positive"), "");
        head.count(&format!("pilot {grp} {rule} non-empty detected"), csrc, nd, t(&pilot, grp, rule, "nonempty_detected_positive"), "");
    }
    head.count("main calls", csrc, 320.0, checks.get("calls").f(), "");
    head.count("main non-empty calls", csrc, 194.0, checks.get("nonempty").f(), "");
    for (grp, rule, det, nd, nr) in [("c_f", "F", 316.0, 192.0, "0.990"), ("c_f_post", "F", 271.0, 193.0, "0.995"), ("c_sh", "S", 276.0, 194.0, "1.000"), ("c_sh", "H", 280.0, 193.0, "0.995")] {
        head.count(&format!("main {grp} {rule} aimed"), csrc, 320.0, t(&checks, grp, rule, "aimed"), "");
        head.count(&format!("main {grp} {rule} positive"), csrc, 320.0, t(&checks, grp, rule, "positive"), "");
        head.count(&format!("main {grp} {rule} detected (all)"), csrc, det, t(&checks, grp, rule, "detected_positive"), "");
        head.count(&format!("main {grp} {rule} non-empty detected"), csrc, nd, t(&checks, grp, rule, "nonempty_detected_positive"), "");
        head.add(&format!("main {grp} {rule} non-empty detection rate"), csrc, nr, t(&checks, grp, rule, "nonempty_detected_positive") / 194.0, "non-empty detected / non-empty calls");
        head.add(&format!("main {grp} {rule} detection rate (all)"), csrc, &format!("{:.3}", det / 320.0), t(&checks, grp, rule, "detected_positive") / 320.0, "");
    }
    head.count("main c_f strict", csrc, 0.0, t(&checks, "c_f", "F", "strict"), "");
    head.count("main c_f P changed", csrc, 320.0, t(&checks, "c_f", "F", "pivots_changed"), "");
    head.count("main c_f T changed", csrc, 320.0, t(&checks, "c_f", "F", "targets_changed"), "");
    head.count("main c_f_post P,T intact", csrc, 320.0, t(&checks, "c_f_post", "F", "structure_intact"), "");
    head.count("main c_f_post first diff at r*", csrc, 320.0, t(&checks, "c_f_post", "F", "first_at_target"), "");
    head.count("main c_f_post NF read altered", csrc, 320.0, t(&checks, "c_f_post", "F", "nf_read_altered"), "");
    let c0d: Vec<f64> = ["F", "S", "H", "L"].iter().map(|r| t(&checks, "c_0", r, "detected_positive")).collect();
    let c0n: Vec<f64> = ["F", "S", "H", "L"].iter().map(|r| t(&checks, "c_0", r, "nonempty_detected_positive")).collect();
    head.count("main C_0 detected min", csrc, 265.0, minmax(&c0d).0, "");
    head.count("main C_0 detected max", csrc, 273.0, minmax(&c0d).1, "");
    head.count("main C_0 non-empty detected min", csrc, 191.0, minmax(&c0n).0, "");
    head.count("main C_0 non-empty detected max", csrc, 193.0, minmax(&c0n).1, "");
    let asrc = "note §9.49 C_F^post aims";
    head.count("C_F^post |V| min", asrc, 161.0, minmax(&vv).0, "");
    head.count("C_F^post |V| max", asrc, 240.0, minmax(&vv).1, "");
    head.count("C_F^post l* min", asrc, 255.0, minmax(&ll).0, "");
    head.count("C_F^post l* max", asrc, 302.0, minmax(&ll).1, "");
    head.count("C_F^post phase-1 reads at l* min", asrc, 1.0, minmax(&nn).0, "");
    head.count("C_F^post phase-1 reads at l* max", asrc, 5.0, minmax(&nn).1, "");
    head.count("perturbed coefficient detected (F) of non-empty", asrc, 194.0, checks.get("perturbed_detected").get("F").f(), "");
    head.count("F digest differs from A", asrc, 320.0, checks.get("digest_differs_from_a").get("F").f(), "");
    // Thresholds.
    let tsrc = "note §9.48 kill thresholds";
    head.add("CP lower bound 480/480", tsrc, "0.9938", cp480, "one-sided 95 % Clopper-Pearson");
    head.add("CP lower bound 309/314", tsrc, "0.9668", cp309, "");
    head.add("CP lower bound 195/199 (E38's figure)", tsrc, "0.9546", cp195, "");
    // Health and A/A.
    let hsrc = "note §9.49 Health and A/A";
    head.add("max unsolved share (any arm, any row)", hsrc, "0.05%", max_uns.0 * 100.0, "unsolved / calls, in %");
    head.add("max chain-arm fallback share", hsrc, "1.30%", max_fb.0 * 100.0, "solver_fallbacks / calls, in %");
    let r242 = rows.iter().find(|r| r.set == "e24" && r.p == 101 && r.seed == 2).unwrap();
    head.add("E24 p101 s2 fallback share", hsrc, "1.13%", 100.0 * r242.rec["A"].get("solver_fallbacks").f() / r242.rec["A"].get("call").get("calls").f(), "");
    let rgls = rows.iter().find(|r| r.set == "gls" && r.p == 101 && r.seed == 2 && r.arm == "A").unwrap();
    head.add("GLS p101 s2 A fallback ceiling", hsrc, "1.60%", 100.0 * health_ceiling(&e37, rgls), "E37 chain-fifo rate + 0.5 points");
    head.add("A/A max |delta S_A/S_F| at 0 GAE", hsrc, "0.0111", aa_max[0], "E39 minus E37 (stream 0xE37E0001), per row");
    head.add("A/A max |delta S_A/S_F| at 0.1 GAE", hsrc, "0.0010", aa_max[1], "");
    head.add("A/A max |delta S_A/S_F| at 1 GAE", hsrc, "8·10⁻⁵", aa_max[2], "");
    // Identity counts.
    let isrc = "note §9.49 identity";
    head.count("Stage A calls", isrc, 10000.0, calls_total as f64, "");
    head.count("Stage A lowest-e36 equal to A counted (calls)", isrc, 10000.0, lowest_eq_total as f64, "");
    head.count("Stage A curves with identical fallback calls across chain solvers", isrc, 10.0, fb_same_total as f64, "");
    head.count("end-to-end row records (F, S, H, B2, L x 32 rows) equal to A outside the registered fields", "summary.json e2e_identity (160 non-informational entries, all empty); the note prints 128 (erratum)", 160.0, id_rows as f64, "leaf-by-leaf against A's record of the same row; allowed: total_muls, solve_muls, ratio, pivot_selection(_ops), lead_scan_reads, solver, call.{mean,median,min,max,p10,p90}, timings; call.calls must be equal");

    // ---------------------------------------------------------- output --
    let checks_j = arr(ck.list.iter().map(|(n, ok, d)| obj! {"check" => st(n), "holds" => J::Bool(*ok), "detail" => st(d)}).collect());
    out.push(("schema".into(), st("e39-validator-recompute/v1")));
    out.push(("stage_a_sha256".into(), st(&stage_sha)));
    out.push(("checks".into(), checks_j));
    out.push(("all_checks_hold".into(), J::Bool(ck.list.iter().all(|x| x.1))));
    out.push(("g2".into(), obj! {
        "G2.190" => J::Bool(ck.all("G2.190")), "G2.191" => J::Bool(ck.all("G2.191")), "G2.192" => J::Bool(ck.all("G2.192")),
        "G2.193" => J::Bool(ck.all("G2.193")), "G2.194" => J::Bool(ck.all("G2.194")), "G2.195" => J::Bool(ck.all("G2.195")),
        "G2.196" => J::Bool(ck.all("G2.196")), "G2.197" => J::Bool(ck.all("G2.197")), "G2.198" => J::Bool(ck.all("G2.198")),
        "G2.199" => J::Bool(ck.all("G2.199")), "G2.200" => J::Bool(ck.all("G2.200")),
        "G2.201_F" => J::Bool(ck.all("G2.191") && ck.all("G2.192") && rank_x("F")),
        "G2.201_S" => J::Bool(ck.all("G2.191") && ck.all("G2.192") && rank_x("S")),
        "G2.201_H" => J::Bool(ck.all("G2.191") && ck.all("G2.192") && rank_x("H")),
        "G2.202" => J::Bool(ck.all("G2.202")), "G2.203" => J::Bool(ck.all("G2.203")),
    }));
    out.push(("producer_summary_comparisons".into(), obj! {"compared" => num(cmp.n as f64), "disagreements" => arr(cmp.bad.iter().map(|s| st(s)).collect())}));
    out.push(("printed_errata".into(), arr(head.errata.clone())));
    out.push(("e2e_identity".into(), obj! {"rows_x_arms_equal" => num(id_rows as f64), "of" => num(160.0), "leaves_compared" => num(leaves_total as f64),
        "differing_keys_count" => J::Obj(id_fields.iter().map(|(k, v)| (k.clone(), num(*v as f64))).collect()),
        "outside_registered" => arr(id_bad.iter().map(|s| st(s)).collect()),
        "e25_differing_keys_count_informational" => J::Obj(e25_diff.iter().map(|(k, v)| (k.clone(), num(*v as f64))).collect())}));
    out.push(("stage_a_curves".into(), arr(sa_rows)));
    out.push(("stage_a_paired_calls_x_below_a".into(), J::Obj(per_call_below.iter().map(|(k, v)| (k.clone(), nums(&v.map(|c| c as f64)))).collect())));
    out.push(("stage_a_pooled".into(), obj! {"contested_mean" => J::Obj(pooled_cont.iter().map(|(k, v)| (k.clone(), num(mean(v)))).collect()),
        "ordered_candidates_mean" => J::Obj(pooled_ord.iter().map(|(k, v)| (k.clone(), num(mean(v)))).collect())}));
    out.push(("bands".into(), arr(bands)));
    out.push(("rho_ranges".into(), arr(rho_ranges)));
    out.push(("aa".into(), arr(aa)));
    out.push(("aa_max_abs_delta".into(), nums(&aa_max)));
    out.push(("health".into(), arr(health)));
    out.push(("controls".into(), arr(ctl_out)));
    out.push(("clopper_pearson".into(), obj! {"480_480" => num(cp480), "309_314" => num(cp309), "195_199" => num(cp195)}));
    let mut gb = Vec::new();
    for r in &rows {
        let mut o = vec![("row".to_string(), st(&format!("{} p{} s{} {}", r.set, r.p, r.seed, r.arm))),
            ("calls".to_string(), num(r.rec["A"].get("call").get("calls").f())),
            ("fallbacks".to_string(), num(r.rec["A"].get("solver_fallbacks").f())),
            ("rho_walk_iii".to_string(), num(r.rho)), ("steps".to_string(), num(r.steps)), ("rho".to_string(), nums(&r.rho_g))];
        for (lab, _) in ARMS {
            o.push((format!("S_{lab}"), nums(&r.s[lab])));
        }
        gb.push(J::Obj(o));
    }
    out.push(("gae_band_rows".into(), arr(gb)));
    let n_head = head.v.len();
    out.push(("headline".into(), arr(head.v)));
    let mut s = String::new();
    ser(&J::Obj(out), 0, &mut s);
    s.push('\n');
    fs::write(out_path, s).unwrap();
    println!("checks {} (failing: {:?})", ck.list.len(), ck.list.iter().filter(|x| !x.1).map(|x| format!("{} {}", x.0, x.2)).collect::<Vec<_>>());
    println!("producer comparisons {} disagreements {}", cmp.n, cmp.bad.len());
    for b in cmp.bad.iter().take(30) { println!("  {b}"); }
    println!("headline entries {} (agreeing); printed errata {}", n_head, head.bad.len());
    for b in head.bad.iter().take(60) { println!("  {b}"); }
}

fn health_ceiling(e37: &str, r: &Row) -> f64 {
    let j = load(&format!("{e37}/e2e/{}_chain-fifo_p{}s{}.json", r.set, r.p, r.seed));
    let e = if r.set == "gls" { j.a().iter().find(|e| e.get("arm").s() == r.arm).unwrap().clone() } else { j.a()[0].clone() };
    e.get("solver_fallbacks").f() / e.get("call").get("calls").f() + 0.005
}

/// Stage A per-curve S at the three weights, primary and sigma1-3.
fn stage_s(sc: &StageCurve, name: &str) -> [[f64; 3]; 4] {
    let c = sc.c[name];
    let native = sc.ent[name] + sc.sel[name];
    // "existing" (E25) and "chain" (A uncounted) take A counted's lead scans.
    let lead = if name == "existing" || name == "chain" { sc.lead["chain-counted"] } else { sc.lead[name] };
    let ord = match name {
        "chain-counted" | "existing" | "chain" => sc.ord["chain-lowest-e36"],
        "chain-static" | "chain-fifo-static" | "chain-lowest-e36" => sc.ord[name],
        _ => 0.0,
    };
    let cont = match name {
        "chain-fifo" | "chain-static" | "chain-fifo-static" | "chain-lifo" => sc.cont[name],
        _ => 0.0,
    };
    let f = |extra: f64| [0, 1, 2].map(|g| c + GAE * GS[g] * (native + extra));
    [f(0.0), f(lead), f(ord), f(cont)]
}

trait ObjIter {
    fn opt_obj(&self) -> Vec<(String, J)>;
}
impl ObjIter for J {
    fn opt_obj(&self) -> Vec<(String, J)> {
        match self {
            J::Obj(v) => v.clone(),
            _ => Vec::new(),
        }
    }
}

/// `cert <list> <rel-prefix> <out.json>`: one line per replayed run,
/// `name<TAB>kind<TAB>committed path<TAB>replay path<TAB>committed path as
/// written in the certificate`; writes the certificate's `runs` array.
fn cert(list: &str, _prefix: &str, out_path: &str) {
    let mut runs = Vec::new();
    let (mut leaves, mut diff, mut matched) = (0usize, 0usize, 0usize);
    for line in fs::read_to_string(list).unwrap().lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 5 {
            continue;
        }
        let (name, kind, committed, replay, shown) = (f[0], f[1], f[2], f[3], f[4]);
        let (a, b) = (load(committed), load(replay));
        let mut o = (0usize, 0usize, Vec::new());
        cmp_leaves(&a, &b, "$", &mut o);
        let (ca, cb) = (sha_file(committed), sha_file(replay));
        leaves += o.0;
        diff += o.1;
        matched += usize::from(o.1 == 0);
        // Scalars of pipeline records (each entry of the record's list).
        let mut sc: Vec<J> = Vec::new();
        if let J::Arr(entries) = &b {
            for e in entries {
                if e.opt("total_muls").is_some() {
                    let g = |k: &str| e.opt(k).map_or(J::Null, |x| x.clone());
                    sc.push(obj! {"arm" => g("arm"), "solver" => g("solver"), "stream_salt" => g("stream_salt"),
                        "total_muls" => g("total_muls"), "macaulay_entries" => g("macaulay_entries"),
                        "pivot_selection_ops" => g("pivot_selection_ops"), "lead_scan_reads" => g("lead_scan_reads"),
                        "solver_fallbacks" => g("solver_fallbacks"), "residuals" => g("residuals"), "unsolved" => g("unsolved"),
                        "recovered" => e.get("folded").get("recovered").clone(), "correct" => e.get("folded").get("correct").clone()});
                }
            }
        } else if let Some(x) = b.opt("identity_failures") {
            sc.push(obj! {"identity_failures" => x.clone(), "calls" => b.opt("calls").cloned().unwrap_or(J::Null),
                "nonempty" => b.opt("nonempty").cloned().unwrap_or(J::Null)});
        }
        runs.push(obj! {
            "run" => st(name), "kind" => st(kind),
            "committed" => obj! {"path" => st(shown), "sha256" => st(&ca)},
            "replay_output" => obj! {"sha256" => st(&cb), "leaves_compared" => num(o.0 as f64), "leaves_differing" => num(o.1 as f64),
                "raw_bytes_identical" => J::Bool(ca == cb)},
            "matches" => J::Bool(o.1 == 0),
            "scalars" => arr(sc),
        });
    }
    let n = runs.len();
    let out = obj! {"runs" => arr(runs), "runs_total" => num(n as f64), "runs_matched" => num(matched as f64),
        "fields_compared" => num(leaves as f64), "fields_differing" => num(diff as f64)};
    let mut s = String::new();
    ser(&out, 0, &mut s);
    s.push('\n');
    fs::write(out_path, s).unwrap();
    println!("runs {n} matched {matched} leaves {leaves} differing {diff}");
}
