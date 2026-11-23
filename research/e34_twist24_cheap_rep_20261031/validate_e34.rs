//! E34 independent validator (note §9.38 pre-registration, §9.39 results).
//!
//! Standard library only: its own JSON parser and serializer, its own SHA-256,
//! git read through `std::process::Command`.  It shares no code with the
//! producer's analysis (`examples/e34_tables.rs`, `summary.json`, `tables.md`)
//! and reads only raw records.
//!
//! ```text
//! rustc -O --edition 2021 validate_e34.rs -o validate_e34
//! validate_e34 selftest
//! validate_e34 draw <nonce-hex>                       # curve order drawn from the nonce
//! validate_e34 sha256 <file>...
//! validate_e34 compare <repo> <replay_dir> <out.json>  # leaf comparison of replayed records
//! validate_e34 report <repo> <evidence_dir>            # writes <evidence_dir>/validator-report.json
//! ```
//!
//! Cost unit (§9.38): F_p multiplications of the walk, set-up and per-walk
//! constants included, the [d]G = Q check excluded (fp_muls - verify_fp_muls);
//! band cost(w) = mean mults + w * GAE * mean native operations (cmp + neg +
//! modn + hash + table), GAE = 97 (79 as the Karatsuba sensitivity); pooled
//! ratio = sum over curves of cost / sqrt(n), arm A over arm B.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::process::Command;

// ============================================================ JSON

#[derive(Clone, Debug, PartialEq)]
enum J {
    Null,
    Bool(bool),
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
    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }
    fn ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == b' ' || c == b'\n' || c == b'\r' || c == b'\t' {
                self.i += 1;
            } else {
                break;
            }
        }
    }
    fn err<T>(&self, m: &str) -> Result<T, String> {
        Err(format!("JSON error at byte {}: {}", self.i, m))
    }
    fn lit(&mut self, w: &str, v: J) -> Result<J, String> {
        if self.s[self.i..].starts_with(w.as_bytes()) {
            self.i += w.len();
            Ok(v)
        } else {
            self.err("bad literal")
        }
    }
    fn value(&mut self) -> Result<J, String> {
        self.ws();
        match self.peek() {
            None => self.err("unexpected end"),
            Some(b'{') => {
                self.i += 1;
                let mut v: Vec<(String, J)> = Vec::new();
                self.ws();
                if self.peek() == Some(b'}') {
                    self.i += 1;
                    return Ok(J::Obj(v));
                }
                loop {
                    self.ws();
                    if self.peek() != Some(b'"') {
                        return self.err("expected a key");
                    }
                    let k = self.string()?;
                    self.ws();
                    if self.peek() != Some(b':') {
                        return self.err("expected ':'");
                    }
                    self.i += 1;
                    let x = self.value()?;
                    if v.iter().any(|(a, _)| *a == k) {
                        return self.err("duplicate key");
                    }
                    v.push((k, x));
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(J::Obj(v));
                        }
                        _ => return self.err("expected ',' or '}'"),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.peek() == Some(b']') {
                    self.i += 1;
                    return Ok(J::Arr(v));
                }
                loop {
                    v.push(self.value()?);
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(J::Arr(v));
                        }
                        _ => return self.err("expected ',' or ']'"),
                    }
                }
            }
            Some(b'"') => Ok(J::Str(self.string()?)),
            Some(b't') => self.lit("true", J::Bool(true)),
            Some(b'f') => self.lit("false", J::Bool(false)),
            Some(b'n') => self.lit("null", J::Null),
            Some(_) => self.number(),
        }
    }
    fn digits(&mut self) -> usize {
        let s = self.i;
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.i += 1;
        }
        self.i - s
    }
    fn number(&mut self) -> Result<J, String> {
        let st = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        let d0 = self.i;
        let nd = self.digits();
        if nd == 0 {
            return self.err("expected a digit");
        }
        if self.s[d0] == b'0' && nd > 1 {
            return self.err("leading zero");
        }
        if self.peek() == Some(b'.') {
            self.i += 1;
            if self.digits() == 0 {
                return self.err("expected a fraction digit");
            }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            self.i += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.i += 1;
            }
            if self.digits() == 0 {
                return self.err("expected an exponent digit");
            }
        }
        Ok(J::Num(String::from_utf8(self.s[st..self.i].to_vec()).unwrap()))
    }
    fn hex4(&mut self) -> Result<u32, String> {
        if self.i + 4 > self.s.len() {
            return self.err("short \\u escape");
        }
        let h = std::str::from_utf8(&self.s[self.i..self.i + 4]).map_err(|e| e.to_string())?;
        let v = u32::from_str_radix(h, 16).map_err(|e| e.to_string())?;
        self.i += 4;
        Ok(v)
    }
    fn string(&mut self) -> Result<String, String> {
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return self.err("unterminated string"),
            };
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = match self.peek() {
                        Some(e) => e,
                        None => return self.err("bad escape"),
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
                            let mut cp = self.hex4()?;
                            if (0xD800..0xDC00).contains(&cp) {
                                if self.s.get(self.i) == Some(&b'\\') && self.s.get(self.i + 1) == Some(&b'u') {
                                    self.i += 2;
                                    let lo = self.hex4()?;
                                    if !(0xDC00..0xE000).contains(&lo) {
                                        return self.err("bad low surrogate");
                                    }
                                    cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                                } else {
                                    return self.err("lone surrogate");
                                }
                            }
                            let ch = char::from_u32(cp).ok_or_else(|| "bad code point".to_string())?;
                            let mut b = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
                        }
                        _ => return self.err("unknown escape"),
                    }
                }
                c if c < 0x20 => return self.err("control character in a string"),
                c => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|e| e.to_string())
    }
}

fn parse(text: &str) -> Result<J, String> {
    let mut p = Parser { s: text.as_bytes(), i: 0 };
    let v = p.value()?;
    p.ws();
    if p.i != p.s.len() {
        return p.err("trailing data");
    }
    Ok(v)
}

fn load(path: &str) -> J {
    let t = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    parse(&t).unwrap_or_else(|e| panic!("{path}: {e}"))
}

impl J {
    fn get(&self, k: &str) -> &J {
        if let J::Obj(v) = self {
            for (a, b) in v {
                if a == k {
                    return b;
                }
            }
        }
        &NULL
    }
    fn has(&self, k: &str) -> bool {
        matches!(self, J::Obj(v) if v.iter().any(|(a, _)| a == k))
    }
    fn u(&self) -> u64 {
        match self {
            J::Num(s) => s.parse().unwrap_or_else(|_| panic!("not an unsigned integer: {s}")),
            o => panic!("not a number: {o:?}"),
        }
    }
    fn f(&self) -> f64 {
        match self {
            J::Num(s) => s.parse().unwrap(),
            o => panic!("not a number: {o:?}"),
        }
    }
    fn s(&self) -> &str {
        match self {
            J::Str(s) => s,
            o => panic!("not a string: {o:?}"),
        }
    }
    fn b(&self) -> bool {
        match self {
            J::Bool(b) => *b,
            o => panic!("not a bool: {o:?}"),
        }
    }
    fn a(&self) -> &[J] {
        match self {
            J::Arr(v) => v,
            o => panic!("not an array: {o:?}"),
        }
    }
    fn keys(&self) -> Vec<String> {
        match self {
            J::Obj(v) => v.iter().map(|(k, _)| k.clone()).collect(),
            _ => vec![],
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
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(o, "\\u{:04x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn ser(j: &J, ind: usize, o: &mut String) {
    match j {
        J::Null => o.push_str("null"),
        J::Bool(b) => o.push_str(if *b { "true" } else { "false" }),
        J::Num(s) => o.push_str(s),
        J::Str(s) => o.push_str(&esc(s)),
        J::Arr(v) => {
            if v.is_empty() {
                o.push_str("[]");
                return;
            }
            o.push_str("[\n");
            for (k, x) in v.iter().enumerate() {
                o.push_str(&" ".repeat(ind + 1));
                ser(x, ind + 1, o);
                if k + 1 < v.len() {
                    o.push(',');
                }
                o.push('\n');
            }
            o.push_str(&" ".repeat(ind));
            o.push(']');
        }
        J::Obj(v) => {
            if v.is_empty() {
                o.push_str("{}");
                return;
            }
            o.push_str("{\n");
            for (k, (key, x)) in v.iter().enumerate() {
                o.push_str(&" ".repeat(ind + 1));
                o.push_str(&esc(key));
                o.push_str(": ");
                ser(x, ind + 1, o);
                if k + 1 < v.len() {
                    o.push(',');
                }
                o.push('\n');
            }
            o.push_str(&" ".repeat(ind));
            o.push('}');
        }
    }
}

fn to_text(j: &J) -> String {
    let mut o = String::new();
    ser(j, 0, &mut o);
    o.push('\n');
    o
}

fn ju(x: u64) -> J {
    J::Num(x.to_string())
}
fn jf(x: f64) -> J {
    assert!(x.is_finite(), "non-finite number");
    J::Num(format!("{x}"))
}
fn js(s: &str) -> J {
    J::Str(s.to_string())
}
fn jb(b: bool) -> J {
    J::Bool(b)
}
fn obj(v: Vec<(&str, J)>) -> J {
    J::Obj(v.into_iter().map(|(k, x)| (k.to_string(), x)).collect())
}
fn arr(v: Vec<J>) -> J {
    J::Arr(v)
}
fn jfs(v: &[f64]) -> J {
    arr(v.iter().map(|&x| jf(x)).collect())
}

// ============================================================ SHA-256

const K256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn sha256(data: &[u8]) -> String {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
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
            w[t] = u32::from_be_bytes([chunk[4 * t], chunk[4 * t + 1], chunk[4 * t + 2], chunk[4 * t + 3]]);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16].wrapping_add(s0).wrapping_add(w[t - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for t in 0..64 {
            let (a, b, c, d, e, f, g, hh) = (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]);
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K256[t]).wrapping_add(w[t]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

fn sha_file(path: &str) -> String {
    sha256(&fs::read(path).unwrap_or_else(|e| panic!("read {path}: {e}")))
}

fn selftest() -> bool {
    let a = sha256(b"") == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let b = sha256(b"abc") == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    let c = sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
        == "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1";
    let j = parse(r#"{"a":[1,-2.5e3,true,null,"xé\n"],"b":{}}"#).unwrap();
    let d = j.get("a").a()[1] == J::Num("-2.5e3".into()) && j.get("a").a()[4].s() == "x\u{e9}\n";
    let e = parse("[1,]").is_err() && parse("{\"a\":1,\"a\":2}").is_err() && parse("01").is_err();
    let p = is_prime_u64(18446744073709551557) && !is_prime_u64(3215031751) && is_prime_u64(1000000007) && !is_prime_u64(561);
    a && b && c && d && e && p
}

// ============================================================ primality

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
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

/// Deterministic Miller-Rabin for 64-bit integers (bases 2..37).
fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

// ============================================================ records

const PS: [u64; 8] = [241, 337, 541, 601, 733, 769, 997, 1069];
const B0: [u64; 8] = [3, 3, 3, 9, 9, 2, 17, 3];
const ARMS: [&str; 5] = ["Order6Built", "Order6Cheap", "Order24Built", "Order24Cheap", "Order24CheapMulLean"];
const O6B: usize = 0;
const O6C: usize = 1;
const O24B: usize = 2;
const O24C: usize = 3;
const O24L: usize = 4;
const WS: [f64; 3] = [0.0, 0.1, 1.0];
const WN: [&str; 3] = ["0", "0.1", "1"];
const GAE: f64 = 97.0;
const GAE_K: f64 = 79.0;
const OPS: [&str; 5] = ["cmp", "neg", "modn", "hash", "table"];
const RUN_KEYS: [&str; 11] = [
    "correct", "cycle_escapes", "fold", "fp_muls", "group_ops", "recovered", "seed", "steps",
    "verify_fp_muls", "verify_group_ops", "visit_digest",
];
const E34: &str = "experiments/38_twist24_cheap_rep";
const E29: &str = "experiments/34_twist24_stage_r";
const NOTE: &str = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md";
const PREREG: &str = "45378560d";
const CODE: &str = "bd0118f39";

#[derive(Clone)]
struct Walk {
    arm: String,
    kara: bool,
    run: J,
    seed: u64,
    steps: u64,
    esc: u64,
    fp: u64,
    vfp: u64,
    ops: [u64; 5],
    rops: [u64; 5],
    calls: u64,
    rfp: u64,
    tfp: u64,
}

fn ops5(j: &J) -> [u64; 5] {
    [j.get("cmp").u(), j.get("neg").u(), j.get("modn").u(), j.get("hash").u(), j.get("table").u()]
}

fn walk(j: &J) -> Walk {
    let r = j.get("run");
    Walk {
        arm: j.get("arm").s().to_string(),
        kara: j.get("karatsuba").b(),
        run: r.clone(),
        seed: r.get("seed").u(),
        steps: r.get("steps").u(),
        esc: r.get("cycle_escapes").u(),
        fp: r.get("fp_muls").u(),
        vfp: r.get("verify_fp_muls").u(),
        ops: ops5(j.get("ops")),
        rops: ops5(j.get("rep_ops")),
        calls: j.get("rep_calls").u(),
        rfp: j.get("rep_fp_muls").u(),
        tfp: j.get("table_fp_muls").u(),
    }
}

struct Curve {
    p: u64,
    b0: u64,
    n: u64,
    bits: f64,
    walks: Vec<Walk>,
}

impl Curve {
    fn get(&self, arm: usize, kara: bool, seed: u64) -> &Walk {
        let v: Vec<&Walk> =
            self.walks.iter().filter(|w| w.arm == ARMS[arm] && w.kara == kara && w.seed == seed).collect();
        assert_eq!(v.len(), 1, "p {} {} {} {}", self.p, ARMS[arm], kara, seed);
        v[0]
    }
}

#[derive(Default, Clone, Copy)]
struct Agg {
    k: f64,
    mults: f64,
    native: f64,
    steps: f64,
    ops: [f64; 5],
    calls: f64,
    rfp: f64,
    rcmp: f64,
}

impl Agg {
    fn of<'a>(ws: impl Iterator<Item = &'a Walk>) -> Agg {
        let mut a = Agg::default();
        for w in ws {
            a.k += 1.0;
            a.mults += (w.fp - w.vfp) as f64;
            a.native += w.ops.iter().sum::<u64>() as f64;
            a.steps += w.steps as f64;
            for i in 0..5 {
                a.ops[i] += w.ops[i] as f64;
            }
            a.calls += w.calls as f64;
            a.rfp += w.rfp as f64;
            a.rcmp += w.rops[0] as f64;
        }
        a
    }
    /// Mean walk cost at weight w: mults + w * gae * (native operations, or steps
    /// under the one-operation-per-step convention).
    fn cost(&self, w: f64, gae: f64, per_step: bool) -> f64 {
        (self.mults + w * gae * if per_step { self.steps } else { self.native }) / self.k
    }
}

struct Data {
    curves: Vec<Curve>,
    agg: Vec<[[Agg; 2]; 5]>, // [curve][arm][kara]
}

fn load_e34(repo: &str) -> Data {
    let mut curves = Vec::new();
    for &p in &PS {
        let j = load(&format!("{repo}/{E34}/stage_r_p{p}.json"));
        let a = j.a();
        assert_eq!(a.len(), 1);
        let c = &a[0];
        curves.push(Curve {
            p: c.get("p").u(),
            b0: c.get("b0").u(),
            n: c.get("n").u(),
            bits: c.get("bits").f(),
            walks: c.get("walks").a().iter().map(walk).collect(),
        });
    }
    let agg = curves
        .iter()
        .map(|c| {
            let mut m = [[Agg::default(); 2]; 5];
            for (ai, arm) in ARMS.iter().enumerate() {
                for (ki, kara) in [false, true].iter().enumerate() {
                    m[ai][ki] = Agg::of(c.walks.iter().filter(|w| w.arm == *arm && w.kara == *kara));
                }
            }
            m
        })
        .collect();
    Data { curves, agg }
}

/// E29's Stage R: (p, karatsuba, order, seed) -> run record, plus the kill test per p.
fn load_e29(repo: &str) -> (BTreeMap<(u64, bool, u64, u64), J>, BTreeMap<u64, J>) {
    let mut runs = BTreeMap::new();
    let mut kill = BTreeMap::new();
    for f in ["stage_r_p241.json", "stage_r_rest.json"] {
        let j = load(&format!("{repo}/{E29}/{f}"));
        for c in j.a() {
            let p = c.get("p").u();
            kill.insert(p, c.get("kill").clone());
            for r in c.get("runs").a() {
                let kara = r.get("karatsuba").b();
                for (ord, key) in [(6u64, "order6"), (24u64, "order24")] {
                    for x in r.get(key).a() {
                        let prev = runs.insert((p, kara, ord, x.get("seed").u()), x.clone());
                        assert!(prev.is_none(), "duplicate E29 record");
                    }
                }
            }
        }
    }
    (runs, kill)
}

// ============================================================ comparison

#[derive(Default)]
struct Cmp {
    compared: u64,
    identical: u64,
    mismatches: Vec<String>,
}

fn cmp_json(a: &J, b: &J, path: &str, c: &mut Cmp) {
    match (a, b) {
        (J::Obj(x), J::Obj(_)) => {
            if a.keys() != b.keys() {
                c.mismatches.push(format!("{path}: keys {:?} vs {:?}", a.keys(), b.keys()));
            }
            for (k, v) in x {
                if !b.has(k) {
                    c.mismatches.push(format!("{path}.{k}: missing in replay"));
                    continue;
                }
                cmp_json(v, b.get(k), &format!("{path}.{k}"), c);
            }
            for k in b.keys() {
                if !a.has(&k) {
                    c.mismatches.push(format!("{path}.{k}: extra in replay"));
                }
            }
        }
        (J::Arr(x), J::Arr(y)) => {
            if x.len() != y.len() {
                c.mismatches.push(format!("{path}: length {} vs {}", x.len(), y.len()));
            }
            for i in 0..x.len().min(y.len()) {
                cmp_json(&x[i], &y[i], &format!("{path}[{i}]"), c);
            }
        }
        _ => {
            c.compared += 1;
            if a == b {
                c.identical += 1;
            } else {
                c.mismatches.push(format!("{path}: {a:?} vs {b:?}"));
            }
        }
    }
}

fn compare_mode(repo: &str, replay: &str, out: &str) {
    let mut names: Vec<String> = fs::read_dir(replay)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".json") && (n.starts_with("stage_r_p") || n == "identity.json" || n == "negative_walks.json"))
        .collect();
    names.sort();
    let mut runs = Vec::new();
    let (mut tc, mut ti, mut walks, mut correct) = (0u64, 0u64, 0u64, 0u64);
    let mut all = true;
    for n in &names {
        let cp = format!("{repo}/{E34}/{n}");
        let rp = format!("{replay}/{n}");
        let a = load(&cp);
        let b = load(&rp);
        let mut c = Cmp::default();
        cmp_json(&a, &b, "$", &mut c);
        let (sa, sb) = (sha_file(&cp), sha_file(&rp));
        let mut w = 0u64;
        let mut ok = 0u64;
        let mut logs: Vec<J> = Vec::new();
        if n != "identity.json" {
            for cv in b.a() {
                let mut rec: Vec<String> = Vec::new();
                for x in cv.get("walks").a() {
                    w += 1;
                    ok += u64::from(x.get("run").get("correct").b());
                    let r = x.get("run").get("recovered");
                    let s = match r {
                        J::Num(s) => s.clone(),
                        _ => "null".into(),
                    };
                    if !rec.contains(&s) {
                        rec.push(s);
                    }
                }
                logs.push(obj(vec![
                    ("p", ju(cv.get("p").u())),
                    ("recovered_logs_distinct", arr(rec.iter().map(|s| js(s)).collect())),
                ]));
            }
        }
        walks += w;
        correct += ok;
        tc += c.compared;
        ti += c.identical;
        let matched = c.mismatches.is_empty() && c.compared == c.identical;
        all &= matched;
        runs.push(obj(vec![
            ("run", js(n.trim_end_matches(".json"))),
            ("committed", obj(vec![("path", js(&format!("{E34}/{n}"))), ("sha256", js(&sa))])),
            ("replay_output", obj(vec![("path", js(&format!("research/e34_twist24_cheap_rep_20261031/replay/{n}"))), ("sha256", js(&sb))])),
            ("byte_identical", jb(sa == sb)),
            ("fields_compared", ju(c.compared)),
            ("fields_identical", ju(c.identical)),
            ("mismatches", arr(c.mismatches.iter().take(50).map(|s| js(s)).collect())),
            ("matched", jb(matched)),
            ("walks", ju(w)),
            ("walks_correct", ju(ok)),
            ("logarithms", arr(logs)),
        ]));
    }
    let res = obj(vec![
        ("schema", js("e34-validator-replay-comparison/v1")),
        ("rule", js("every leaf of every committed record against the replay: numbers by their exact text, strings and booleans exactly, object key order and array lengths too; the records carry no wall-clock field, so nothing is excluded")),
        ("runs_total", ju(runs.len() as u64)),
        ("runs_matched", ju(if all { runs.len() as u64 } else { 0 })),
        ("all_matched", jb(all)),
        ("fields_compared", ju(tc)),
        ("fields_identical", ju(ti)),
        ("walks_replayed", ju(walks)),
        ("walks_correct", ju(correct)),
        ("runs", arr(runs)),
    ]);
    fs::write(out, to_text(&res)).unwrap();
    println!("runs {} all_matched {} fields {}/{} walks {} correct {}", names.len(), all, ti, tc, walks, correct);
}

// ============================================================ helpers

fn git(repo: &str, args: &[&str]) -> (bool, String) {
    let o = Command::new("git").arg("-C").arg(repo).args(args).output().expect("git");
    (o.status.success(), String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn section(text: &str, start: &str, end: &str) -> String {
    let i = text.find(start).expect("section start");
    let rest = &text[i..];
    let j = rest[start.len()..].find(end).map(|k| k + start.len()).unwrap_or(rest.len());
    rest[..j].trim_end().to_string()
}

fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)))
}

struct Ctx {
    vals: BTreeMap<String, f64>,
    checks: Vec<J>,
    fails: Vec<String>,
}

impl Ctx {
    fn put(&mut self, k: &str, v: f64) {
        self.vals.insert(k.to_string(), v);
    }
    fn check(&mut self, name: &str, holds: bool, detail: String) {
        if !holds {
            self.fails.push(name.to_string());
        }
        self.checks.push(obj(vec![("check", js(name)), ("holds", jb(holds)), ("detail", js(&detail))]));
    }
}

fn kname(k: usize) -> &'static str {
    if k == 0 {
        "schoolbook"
    } else {
        "Karatsuba"
    }
}

fn pooled(d: &Data, a: usize, b: usize, k: usize, w: f64, gae: f64, per_step: bool) -> f64 {
    let mut na = 0.0;
    let mut nb = 0.0;
    for (ci, c) in d.curves.iter().enumerate() {
        let s = (c.n as f64).sqrt();
        na += d.agg[ci][a][k].cost(w, gae, per_step) / s;
        nb += d.agg[ci][b][k].cost(w, gae, per_step) / s;
    }
    na / nb
}

fn pooled_cost(d: &Data, a: usize, k: usize, w: f64, gae: f64) -> f64 {
    // mean over the eight curves of cost / sqrt(n) (F_p multiplications per sqrt(n))
    d.curves.iter().enumerate().map(|(ci, c)| d.agg[ci][a][k].cost(w, gae, false) / (c.n as f64).sqrt()).sum::<f64>() / 8.0
}

fn curve_ratio(d: &Data, ci: usize, a: usize, b: usize, k: usize, w: f64, gae: f64, per_step: bool) -> f64 {
    d.agg[ci][a][k].cost(w, gae, per_step) / d.agg[ci][b][k].cost(w, gae, per_step)
}

fn classify(r: &[f64]) -> &'static str {
    if r.iter().all(|&x| x > 1.0) {
        "order24"
    } else if r.iter().all(|&x| x < 1.0) {
        "order6"
    } else {
        "unresolved"
    }
}

// ============================================================ report

/// Compare printed figures with recomputed values: half a unit of the last
/// printed digit, exact for counts.
fn eval_figures(list: &[(&str, &str, &str, &str, &str)], vals: &BTreeMap<String, f64>) -> (Vec<J>, Vec<String>) {
    let mut out = Vec::new();
    let mut dis = Vec::new();
    for (name, src, printed, key, def) in list.iter() {
        let rec = *vals.get(*key).unwrap_or_else(|| panic!("no value {key}"));
        let claimed: f64 = printed.parse().unwrap();
        let exact = !printed.contains('.');
        let half = if exact { 0.5 } else { 0.5 * 10f64.powi(-((printed.len() - printed.find('.').unwrap() - 1) as i32)) };
        let tol = if exact && rec.fract() == 0.0 { 0.0 } else { half };
        let agrees = (claimed - rec).abs() <= tol + 1e-12;
        if !agrees {
            dis.push(format!("{name}: printed {printed}, recomputed {rec}"));
        }
        let rel = if claimed == 0.0 || tol == 0.0 { 1e-12 } else { (tol + 1e-12) / claimed.abs() };
        out.push(obj(vec![
            ("name", js(name)),
            ("source", js(src)),
            ("printed", js(printed)),
            ("claimed", jf(claimed)),
            ("recomputed", jf(rec)),
            ("rel_tol", jf(rel)),
            ("agrees", jb(agrees)),
            ("definition", js(def)),
        ]));
    }
    (out, dis)
}

fn report(repo: &str, evdir: &str) {
    let mut cx = Ctx { vals: BTreeMap::new(), checks: Vec::new(), fails: Vec::new() };
    let d = load_e34(repo);
    let (e29, kill) = load_e29(repo);
    let ident = load(&format!("{repo}/{E34}/identity.json"));
    let neg = load(&format!("{repo}/{E34}/negative_walks.json"));
    let hi: Vec<usize> = (0..8).filter(|&i| PS[i] >= 541).collect();

    // ---------------------------------------------------------------- coverage
    let ps: Vec<u64> = d.curves.iter().map(|c| c.p).collect();
    let b0: Vec<u64> = d.curves.iter().map(|c| c.b0).collect();
    cx.check(
        "eight curves, p and canonical b0 as registered in §9.38 (b0 = 3, 3, 3, 9, 9, 2, 17, 3)",
        ps == PS && b0 == B0,
        format!("p {ps:?}, b0 {b0:?}"),
    );
    let mut same_n = true;
    let mut same_b0 = true;
    for c in &d.curves {
        let k = &kill[&c.p];
        same_n &= k.get("n").u() == c.n && (k.get("bits").f() - c.bits).abs() < 1e-12 && ((c.n as f64).log2() - c.bits).abs() < 1e-9;
        same_b0 &= k.get("canonical_b0").u() == c.b0 && k.get("order_lambda_p").u() == 24 && k.get("group_order").u() == 24;
    }
    cx.check("n and bits equal E29's kill-test records on every curve; bits = log2(n)", same_n, format!("{same_n}"));
    cx.check(
        "E29's kill test records canonical b0 equal to E34's and a group <lambda_p, lambda_omega, -1> of order 24 on every curve",
        same_b0,
        format!("{same_b0}"),
    );
    let mut layout = true;
    for c in &d.curves {
        layout &= c.walks.len() == 160;
        let mut i = 0;
        for kara in [false, true] {
            for arm in ARMS {
                for s in 0..16u64 {
                    let w = &c.walks[i];
                    layout &= w.arm == arm && w.kara == kara && w.seed == 1000 + s;
                    i += 1;
                }
            }
        }
    }
    cx.check(
        "each curve: 160 walks = 2 arithmetics x 5 arms x seeds 1000-1015, each once, in order",
        layout,
        format!("{} walks", d.curves.iter().map(|c| c.walks.len()).sum::<usize>()),
    );
    let mut all_correct = true;
    let mut fold_ok = true;
    let mut one_log = true;
    let mut logs: Vec<J> = Vec::new();
    for c in &d.curves {
        let r0 = c.walks[0].run.get("recovered").clone();
        for w in &c.walks {
            all_correct &= w.run.get("correct").b();
            one_log &= *w.run.get("recovered") == r0 && !r0.eq(&J::Null);
            let f = w.run.get("fold").s();
            fold_ok &= if w.arm.starts_with("Order24") { f == "Twist24" } else { f == "Psi" };
        }
        for ((p, _, _, _), x) in e29.range((c.p, false, 0, 0)..(c.p + 1, false, 0, 0)) {
            one_log &= *p != c.p || *x.get("recovered") == r0;
        }
        logs.push(obj(vec![("p", ju(c.p)), ("d", r0.clone())]));
    }
    cx.check("all 1,280 Stage R walks report correct = true", all_correct, format!("{all_correct}"));
    cx.check(
        "every walk of a curve (E34's 160 and E29's 64) recovers the same logarithm",
        one_log,
        format!("{one_log}"),
    );
    cx.check("fold label Psi on order-6 arms, Twist24 on order-24 arms", fold_ok, format!("{fold_ok}"));

    // ---------------------------------------------------------------- counter consistency
    let (mut t_ok, mut h_ok, mut cmpneg_in_rep, mut setup_modn): (bool, bool, bool, Vec<u64>) = (true, true, true, vec![]);
    let mut modn_ok = true;
    let mut pos_ctrl = true;
    let mut rep_eq_arith = true;
    let mut tfp: Vec<u64> = vec![];
    let mut verify_same = true;
    for c in &d.curves {
        for w in &c.walks {
            // table: a lookup every step, an insert every step that is not a collision
            // (escape or the final collision); hash: two partition hashes per new point.
            let found = 1u64;
            t_ok &= w.ops[4] == 2 * w.steps - w.esc - found;
            h_ok &= w.ops[3] == 2 * (w.steps - w.esc - found);
            cmpneg_in_rep &= w.ops[0] == w.rops[0] && w.ops[1] == w.rops[1] && w.rops[3] == 0 && w.rops[4] == 0;
            // mod-n bookkeeping outside the representative: 2 at the start, 4 per
            // ordinary step, 1 + 4 per escape, 1 + 3 at the collision (+ the cheap arms' set-up).
            let walk_modn = 2 + 4 * (w.steps - w.esc - found) + 5 * w.esc + 4;
            let extra = w.ops[2] - w.rops[2];
            if w.arm.ends_with("Built") {
                modn_ok &= extra == walk_modn;
                modn_ok &= w.tfp == 0;
            } else {
                if !setup_modn.contains(&(extra - walk_modn)) {
                    setup_modn.push(extra - walk_modn);
                }
                if !tfp.contains(&w.tfp) {
                    tfp.push(w.tfp);
                }
            }
            if w.arm == "Order6Built" {
                pos_ctrl &= w.rfp == 8 * w.calls;
            }
            if !w.kara {
                let k = c.walks.iter().find(|x| x.kara && x.arm == w.arm && x.seed == w.seed).unwrap();
                rep_eq_arith &= k.rfp == w.rfp && k.calls == w.calls && k.ops == w.ops && k.rops == w.rops && k.tfp == w.tfp;
            }
            let b = c.get(O6B, w.kara, w.seed);
            verify_same &= w.vfp == b.vfp && w.run.get("verify_group_ops") == b.run.get("verify_group_ops");
        }
    }
    cx.check(
        "counter identity on all 1,280 walks: table = 2 steps - escapes - 1, hash = 2 (steps - escapes - 1)",
        t_ok && h_ok,
        format!("table {t_ok}, hash {h_ok}"),
    );
    cx.check(
        "every cmp and neg is the representative's; the representative emits no hash or table operation",
        cmpneg_in_rep,
        format!("{cmpneg_in_rep}"),
    );
    cx.check(
        "mod-n operations outside the representative = 2 + 4 (steps - escapes - 1) + 5 escapes + 4 on the built arms; the cheap arms add one constant set-up",
        modn_ok && setup_modn.len() == 1,
        format!("built arms {modn_ok}; cheap arms' set-up mod-n operations {setup_modn:?}; cheap arms' table F_p multiplications {tfp:?}"),
    );
    cx.check(
        "positive control: E29's order-6 representative costs exactly 8 F_p multiplications per call on all 256 walks",
        pos_ctrl,
        format!("{pos_ctrl}"),
    );
    cx.check(
        "representative multiplications, calls and every native-operation counter equal in both arithmetics, walk by walk",
        rep_eq_arith,
        format!("{rep_eq_arith}"),
    );
    cx.check(
        "the [d]G = Q check costs the same on every arm (per curve, arithmetic and seed)",
        verify_same,
        format!("{verify_same}"),
    );

    // ---------------------------------------------------------------- G2.135: E29 replay
    let (mut recs, mut ident_recs, mut fields, mut fields_same) = (0u64, 0u64, 0u64, 0u64);
    let mut e29_o24_digest_zero = 0u64;
    let mut e29_keys_ok = true;
    let mut e29_mism: Vec<String> = Vec::new();
    for c in &d.curves {
        for (ki, kara) in [false, true].iter().enumerate() {
            for (arm, ord) in [(O6B, 6u64), (O24B, 24u64)] {
                for s in 0..16u64 {
                    let w = c.get(arm, *kara, 1000 + s);
                    let x = &e29[&(c.p, *kara, ord, 1000 + s)];
                    recs += 1;
                    e29_keys_ok &= x.keys() == RUN_KEYS.iter().map(|s| s.to_string()).collect::<Vec<_>>()
                        && w.run.keys() == x.keys();
                    let mut same = true;
                    for k in RUN_KEYS {
                        if ord == 24 && k == "visit_digest" {
                            if x.get(k).u() == 0 {
                                e29_o24_digest_zero += 1;
                            }
                            continue;
                        }
                        fields += 1;
                        if x.get(k) == w.run.get(k) {
                            fields_same += 1;
                        } else {
                            same = false;
                            e29_mism.push(format!("p {} {} {} seed {}: {k}", c.p, kname(ki), ord, 1000 + s));
                        }
                    }
                    ident_recs += u64::from(same);
                }
            }
        }
    }
    cx.put("e29_records_identical", ident_recs as f64);
    cx.put("e29_records_total", recs as f64);
    cx.check(
        "G2.135: E34's counted copies (Order6Built, Order24Built) reproduce E29's Stage R records field for field",
        ident_recs == 512 && recs == 512 && e29_keys_ok && e29_mism.is_empty(),
        format!(
            "{ident_recs}/{recs} records, {fields_same}/{fields} fields; key sets equal {e29_keys_ok}; E29's order-24 records carry visit_digest = 0 ({e29_o24_digest_zero}/256), so that field is compared on the 256 order-6 records only; mismatches {:?}",
            e29_mism.iter().take(5).collect::<Vec<_>>()
        ),
    );
    let (ok_d, s_d) = git(repo, &["diff", "--name-only", &format!("{CODE}^"), CODE]);
    cx.check(
        "E34's code commit touches only examples/e34_cheap_rep.rs, src/cryptanalysis/mod.rs and src/cryptanalysis/twist24_rep.rs (E29's functions untouched)",
        ok_d && s_d.lines().collect::<Vec<_>>() == vec!["examples/e34_cheap_rep.rs", "src/cryptanalysis/mod.rs", "src/cryptanalysis/twist24_rep.rs"],
        s_d.replace('\n', ", "),
    );

    // ---------------------------------------------------------------- G2.136: walk identity
    let mut same_walks = 0u64;
    let mut tot = 0u64;
    let mut extra_same = true;
    let mut digest_arith = true;
    for c in &d.curves {
        for kara in [false, true] {
            for (cheap, built) in [(O6C, O6B), (O24C, O24B), (O24L, O24B)] {
                for s in 0..16u64 {
                    let a = c.get(cheap, kara, 1000 + s);
                    let b = c.get(built, kara, 1000 + s);
                    tot += 1;
                    let eq = ["steps", "group_ops", "cycle_escapes", "recovered", "correct", "visit_digest"]
                        .iter()
                        .all(|k| a.run.get(k) == b.run.get(k));
                    same_walks += u64::from(eq);
                    extra_same &= a.calls == b.calls && a.ops[3] == b.ops[3] && a.ops[4] == b.ops[4];
                }
            }
            for arm in 0..5 {
                for s in 0..16u64 {
                    let a = c.get(arm, false, 1000 + s);
                    let b = c.get(arm, true, 1000 + s);
                    digest_arith &= a.run.get("visit_digest") == b.run.get("visit_digest")
                        && a.steps == b.steps
                        && a.run.get("group_ops") == b.run.get("group_ops");
                }
            }
        }
    }
    // ---------------------------------------------------------------- group orders and the rho heuristic (null control for per-curve ranks)
    let mut prime_ok = true;
    let mut steps_rows: Vec<J> = Vec::new();
    let (mut norm6, mut norm24) = (0.0, 0.0);
    let (mut p6, mut p24) = (0.0, 0.0);
    let (mut v6, mut v24) = (0.0, 0.0);
    let mut se_rel: Vec<f64> = Vec::new();
    for c in &d.curves {
        prime_ok &= is_prime_u64(c.n) && c.n % 24 == 1;
        let st = |arm: usize| -> (f64, f64) {
            let v: Vec<f64> = (0..16u64).map(|s| c.get(arm, false, 1000 + s).steps as f64).collect();
            let m = v.iter().sum::<f64>() / 16.0;
            let sd = (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / 15.0).sqrt();
            (m, sd)
        };
        let (m6, sd6) = st(O6B);
        let (m24, sd24) = st(O24B);
        let pi = std::f64::consts::PI;
        let t6 = (pi * c.n as f64 / 12.0).sqrt();
        let t24 = (pi * c.n as f64 / 48.0).sqrt();
        norm6 += m6 / t6 / 8.0;
        norm24 += m24 / t24 / 8.0;
        p6 += m6 / (c.n as f64).sqrt();
        p24 += m24 / (c.n as f64).sqrt();
        v6 += sd6 * sd6 / 16.0 / c.n as f64;
        v24 += sd24 * sd24 / 16.0 / c.n as f64;
        // standard error of the per-curve steps ratio from 16 + 16 walks (delta method)
        let r = m6 / m24;
        let se = r * ((sd6 / m6).powi(2) / 16.0 + (sd24 / m24).powi(2) / 16.0).sqrt();
        se_rel.push(se / r);
        steps_rows.push(obj(vec![
            ("p", ju(c.p)),
            ("n_prime", jb(is_prime_u64(c.n))),
            ("n_mod_24", ju(c.n % 24)),
            ("mean_steps_order6", jf(m6)),
            ("sd_steps_order6", jf(sd6)),
            ("heuristic_order6_sqrt_pi_n_over_12", jf(t6)),
            ("mean_steps_order24", jf(m24)),
            ("sd_steps_order24", jf(sd24)),
            ("heuristic_order24_sqrt_pi_n_over_48", jf(t24)),
            ("steps_ratio", jf(r)),
            ("steps_ratio_standard_error", jf(se)),
        ]));
    }
    cx.check(
        "independent: every curve's n is prime (deterministic Miller-Rabin) and n = 1 mod 24, so an order-24 multiplier group exists in (Z/n)*",
        prime_ok,
        format!("{prime_ok}"),
    );
    let (se_lo, se_hi) = minmax(&se_rel);
    let pr = p6 / p24;
    let pr_se = pr * (v6 / (p6 * p6) + v24 / (p24 * p24)).sqrt();
    cx.put("steps_ratio_pooled_se", pr_se);
    cx.put("steps_norm6", norm6);
    cx.put("steps_norm24", norm24);
    cx.put("steps_ratio_se_rel_min", se_lo);
    cx.put("steps_ratio_se_rel_max", se_hi);
    cx.checks.push(obj(vec![
        ("check", js("rho heuristic as the null control for per-curve ranks (informational): mean steps over sqrt(pi n / (2 m)), m = 6 or 24, and the 16-walk standard error of each curve's steps ratio")),
        ("holds", jb(true)),
        ("detail", js(&format!("order 6: {norm6:.3} of the heuristic, order 24: {norm24:.3} (mean over curves); heuristic steps ratio 2.000, pooled observed {:.3} +- {:.3} ({:.1} standard errors below 2); per-curve steps-ratio standard error {:.1}-{:.1} % from 16 walks a side, so per-curve ranks that turn on the steps ratio are within sampling noise", pr, pr_se, (2.0 - pr) / pr_se, se_lo * 100.0, se_hi * 100.0))),
    ]));

    cx.put("walk_identity", same_walks as f64);
    cx.check(
        "G2.136: every cheap walk equals its built walk per seed (steps, group_ops, cycle_escapes, recovered, correct, visit_digest)",
        same_walks == 768 && tot == 768,
        format!("{same_walks}/{tot}; representative calls, hash and table counts equal too: {extra_same}"),
    );
    cx.check(
        "visit digests, steps and group operations equal across the two arithmetics, every arm and seed",
        digest_arith,
        format!("{digest_arith}"),
    );

    // ---------------------------------------------------------------- G2.134: identity
    let mut tot_m = BTreeMap::<String, (u64, u64, u64, u64, u64)>::new(); // members, mismatches, scalar checks, failures, curves
    let mut id_ok = true;
    let mut omit_per_curve: Vec<u64> = vec![];
    let id_arr = ident.a();
    id_ok &= id_arr.len() == 8;
    for (ci, cv) in id_arr.iter().enumerate() {
        id_ok &= cv.get("p").u() == PS[ci] && cv.get("b0").u() == B0[ci] && cv.get("n").u() == d.curves[ci].n;
        let chk = cv.get("checks").a();
        id_ok &= chk.len() == 5;
        for x in chk {
            let arm = x.get("arm").s().to_string();
            let pts = x.get("points").u();
            let forced = x.get("forced_x0_zero").u();
            let mem = x.get("members").u();
            let class = if arm == "Order6Cheap" { 6 } else { 24 };
            id_ok &= pts == 200 && forced == 8 && mem == class * (pts + forced) && !x.get("x_zero_point").b();
            let e = tot_m.entry(arm.clone()).or_default();
            e.0 += mem;
            e.1 += x.get("mismatches").u();
            e.2 += x.get("scalar_checks").u();
            e.3 += x.get("scalar_failures").u();
            e.4 += 1;
            if arm == "Order24OmitCandidate" {
                omit_per_curve.push(x.get("mismatches").u());
            }
        }
    }
    let g = |k: &str| tot_m.get(k).copied().unwrap_or_default();
    let (c24, l24, c6, om, wr) = (g("Order24Cheap"), g("Order24CheapMulLean"), g("Order6Cheap"), g("Order24OmitCandidate"), g("Order24WrongOrdinate"));
    cx.put("o24_members", c24.0 as f64);
    cx.put("o24_mismatch", c24.1 as f64);
    cx.put("o24l_members", l24.0 as f64);
    cx.put("o24l_mismatch", l24.1 as f64);
    cx.put("o6_members", c6.0 as f64);
    cx.put("o6_mismatch", c6.1 as f64);
    cx.put("members_total", (c24.0 + c6.0) as f64);
    cx.put("omit_mismatch", om.1 as f64);
    cx.put("omit_members", om.0 as f64);
    cx.put("omit_per_curve_min", *omit_per_curve.iter().min().unwrap() as f64);
    cx.put("omit_per_curve_max", *omit_per_curve.iter().max().unwrap() as f64);
    cx.put("wrong_mismatch", wr.1 as f64);
    cx.put("wrong_members", wr.0 as f64);
    cx.put("wrong_scalar_failures", wr.3 as f64);
    cx.put("wrong_scalar_checks", wr.2 as f64);
    cx.check(
        "identity records: 8 curves x 5 representatives, 200 random points + 8 forced x0 = 0 points, members = class size x 208, no x = 0 point",
        id_ok,
        format!("{id_ok}"),
    );
    cx.check(
        "G2.134 (a): both cheap order-24 representatives and the cheap order-6 one equal E29's on every class member, rep = [m]P on every scalar check",
        c24.1 == 0 && l24.1 == 0 && c6.1 == 0 && c24.3 == 0 && l24.3 == 0 && c6.3 == 0,
        format!(
            "Order24Cheap {}/{} mismatches, scalar failures {}/{}; mul-lean {}/{}, {}/{}; Order6Cheap {}/{}, {}/{}",
            c24.1, c24.0, c24.3, c24.2, l24.1, l24.0, l24.3, l24.2, c6.1, c6.0, c6.3, c6.2
        ),
    );
    // A class with twelve distinct abscissae: the omitted candidate (3,2) is the
    // winner for exactly the two members (+-) whose (3,2)-image is the least
    // abscissa, so the control must miss exactly 2 of 24 = members / 12 when it misses.
    cx.check(
        "G2.134 (b): the omitted-candidate control is caught on every curve, at exactly members/12 (2 of every 24-member class, the analytic rate); the wrong-ordinate control mismatches every member and fails every scalar check",
        omit_per_curve.iter().all(|&m| m == 4992 / 12) && wr.1 == wr.0 && wr.3 == wr.2 && wr.2 > 0,
        format!("omitted candidate {:?} per curve ({} of {}); wrong ordinate {}/{}, scalar failures {}/{}", omit_per_curve, om.1, om.0, wr.1, wr.0, wr.3, wr.2),
    );

    // ---------------------------------------------------------------- G2.137: negative walks
    let na = neg.a();
    let mut nd = 0u64;
    let mut nc = 0u64;
    let mut nn = 0u64;
    let mut nsteps = 0u64;
    let mut neg_ok = na.len() == 1 && na[0].get("p").u() == 241;
    for x in na[0].get("walks").a() {
        let w = walk(x);
        neg_ok &= w.arm == "Order24OmitCandidate" && !w.kara;
        let b = d.curves[0].get(O24B, false, w.seed);
        nn += 1;
        nd += u64::from(w.run.get("visit_digest") != b.run.get("visit_digest"));
        nc += u64::from(w.run.get("correct").b() && w.run.get("recovered") == b.run.get("recovered"));
        nsteps += u64::from(w.steps != b.steps);
    }
    cx.put("neg_digest_differs", nd as f64);
    cx.put("neg_correct", nc as f64);
    cx.check(
        "G2.137: the omitted-candidate walk's visit digest differs from E29's (Order24Built) on every seed at p = 241, schoolbook",
        neg_ok && nn == 16 && nd == 16,
        format!("{nd}/{nn} digests differ; {nc}/{nn} still recover the log; steps differ on {nsteps}/{nn}"),
    );

    // ---------------------------------------------------------------- per call and per step
    let mut per_curve: Vec<J> = Vec::new();
    for (ci, c) in d.curves.iter().enumerate() {
        let mut arms_j = Vec::new();
        for (ai, arm) in ARMS.iter().enumerate() {
            for k in 0..2 {
                let a = &d.agg[ci][ai][k];
                arms_j.push(obj(vec![
                    ("arm", js(arm)),
                    ("arithmetic", js(kname(k))),
                    ("mean_steps", jf(a.steps / a.k)),
                    ("mean_fp_muls_walk", jf(a.mults / a.k)),
                    ("mean_native_ops", jf(a.native / a.k)),
                    ("fp_muls_per_step", jf(a.mults / a.steps)),
                    ("native_ops_per_step", jf(a.native / a.steps)),
                    ("ops_per_step", obj(OPS.iter().enumerate().map(|(i, o)| (*o, jf(a.ops[i] / a.steps))).collect())),
                    ("rep_fp_muls_per_call", jf(a.rfp / a.calls)),
                    ("rep_cmp_per_call", jf(a.rcmp / a.calls)),
                    ("cost_0_0p1_1_gae97", jfs(&WS.map(|w| a.cost(w, GAE, false)))),
                ]));
            }
        }
        per_curve.push(obj(vec![("p", ju(c.p)), ("b0", ju(c.b0)), ("n", ju(c.n)), ("arms", arr(arms_j))]));
    }
    let tags = ["o6b", "o6c", "o24b", "o24c", "o24l"];
    let every: Vec<usize> = (0..8).collect();
    for (ai, t) in tags.iter().enumerate() {
        for k in 0..2 {
            let kt = if k == 0 { "sb" } else { "k" };
            for (set, st) in [(&hi, "hi"), (&every, "all")] {
                let mps: Vec<f64> = set.iter().map(|&ci| d.agg[ci][ai][k].mults / d.agg[ci][ai][k].steps).collect();
                let nps: Vec<f64> = set.iter().map(|&ci| d.agg[ci][ai][k].native / d.agg[ci][ai][k].steps).collect();
                let rpc: Vec<f64> = set.iter().map(|&ci| d.agg[ci][ai][k].rfp / d.agg[ci][ai][k].calls).collect();
                let cpc: Vec<f64> = set.iter().map(|&ci| d.agg[ci][ai][k].rcmp / d.agg[ci][ai][k].calls).collect();
                for (nm, v) in [("mps", &mps), ("nps", &nps), ("rpc", &rpc), ("cpc", &cpc)] {
                    let (lo, hi_) = minmax(v);
                    cx.put(&format!("{t}_{kt}_{st}_{nm}_min"), lo);
                    cx.put(&format!("{t}_{kt}_{st}_{nm}_max"), hi_);
                }
                // pooled over the set: sums over every walk
                let sum = |f: &dyn Fn(&Agg) -> f64| set.iter().map(|&ci| f(&d.agg[ci][ai][k])).sum::<f64>();
                let steps = sum(&|a| a.steps);
                let calls = sum(&|a| a.calls);
                cx.put(&format!("{t}_{kt}_{st}_nps_pooled"), sum(&|a| a.native) / steps);
                cx.put(&format!("{t}_{kt}_{st}_rpc_pooled"), sum(&|a| a.rfp) / calls);
                cx.put(&format!("{t}_{kt}_{st}_cpc_pooled"), sum(&|a| a.rcmp) / calls);
                for (oi, o) in OPS.iter().enumerate() {
                    cx.put(&format!("{t}_{kt}_{st}_{o}_pooled"), sum(&|a| a.ops[oi]) / steps);
                    let v: Vec<f64> = set.iter().map(|&ci| d.agg[ci][ai][k].ops[oi] / d.agg[ci][ai][k].steps).collect();
                    let (lo, hi_) = minmax(&v);
                    cx.put(&format!("{t}_{kt}_{st}_{o}_min"), lo);
                    cx.put(&format!("{t}_{kt}_{st}_{o}_max"), hi_);
                }
            }
        }
        // representative multiplications per call equal in both arithmetics (per curve)
    }
    // G2.139: cheap order 24 / order 6 as built, per-step multiplications, p >= 541
    for k in 0..2 {
        let kt = if k == 0 { "sb" } else { "k" };
        let r: Vec<f64> = hi
            .iter()
            .map(|&ci| (d.agg[ci][O24C][k].mults / d.agg[ci][O24C][k].steps) / (d.agg[ci][O6B][k].mults / d.agg[ci][O6B][k].steps))
            .collect();
        let (lo, h) = minmax(&r);
        cx.put(&format!("g139_{kt}_min"), lo);
        cx.put(&format!("g139_{kt}_max"), h);
        let r2: Vec<f64> = hi
            .iter()
            .map(|&ci| (d.agg[ci][O24B][k].mults / d.agg[ci][O24B][k].steps) / (d.agg[ci][O6B][k].mults / d.agg[ci][O6B][k].steps))
            .collect();
        let (lo2, h2) = minmax(&r2);
        cx.put(&format!("o24b_over_o6b_step_{kt}_min"), lo2);
        cx.put(&format!("o24b_over_o6b_step_{kt}_max"), h2);
    }

    // ---------------------------------------------------------------- band
    let pairs: [(&str, usize, usize); 6] = [
        ("e29_e29", O6B, O24B),
        ("e29_cheap", O6B, O24C),
        ("cheap_cheap", O6C, O24C),
        ("cheap_mullean", O6C, O24L),
        ("e29_mullean", O6B, O24L),
        ("o6b_o6c", O6B, O6C),
    ];
    let mut band_rows: Vec<J> = Vec::new();
    for (nm, a, b) in pairs {
        for k in 0..2 {
            let kt = if k == 0 { "sb" } else { "k" };
            let mut row_p = Vec::new();
            let mut row_s = Vec::new();
            let mut row_79 = Vec::new();
            let mut per_c: Vec<Vec<f64>> = vec![vec![]; 8];
            for (wi, w) in WS.iter().enumerate() {
                let r = pooled(&d, a, b, k, *w, GAE, false);
                cx.put(&format!("band_{nm}_{kt}_{}", WN[wi]), r);
                row_p.push(r);
                let s = pooled(&d, a, b, k, *w, GAE, true);
                cx.put(&format!("band1op_{nm}_{kt}_{}", WN[wi]), s);
                row_s.push(s);
                if k == 1 {
                    let g = pooled(&d, a, b, k, *w, GAE_K, false);
                    cx.put(&format!("band79_{nm}_{}", WN[wi]), g);
                    row_79.push(g);
                }
                for ci in 0..8 {
                    per_c[ci].push(curve_ratio(&d, ci, a, b, k, *w, GAE, false));
                }
            }
            let cls: Vec<&str> = per_c.iter().map(|r| classify(r)).collect();
            let n24 = cls.iter().filter(|&&c| c == "order24").count();
            let n6 = cls.iter().filter(|&&c| c == "order6").count();
            let nu = cls.iter().filter(|&&c| c == "unresolved").count();
            cx.put(&format!("curves_{nm}_{kt}_a"), n24 as f64);
            cx.put(&format!("curves_{nm}_{kt}_b"), n6 as f64);
            cx.put(&format!("curves_{nm}_{kt}_u"), nu as f64);
            band_rows.push(obj(vec![
                ("pair", js(&format!("{} / {}", ARMS[a], ARMS[b]))),
                ("arithmetic", js(kname(k))),
                ("pooled_ratio_0_0p1_1", jfs(&row_p)),
                ("pooled_order_holds", jb(classify(&row_p) != "unresolved")),
                ("pooled_cheaper", js(match classify(&row_p) { "order24" => ARMS[b], "order6" => ARMS[a], _ => "unresolved" })),
                ("pooled_ratio_one_op_per_step", jfs(&row_s)),
                ("pooled_ratio_karatsuba_gae79", jfs(&row_79)),
                (
                    "per_curve",
                    arr((0..8)
                        .map(|ci| obj(vec![("p", ju(PS[ci])), ("ratio_0_0p1_1", jfs(&per_c[ci])), ("cheaper_across_band", js(match cls[ci] { "order24" => ARMS[b], "order6" => ARMS[a], _ => "unresolved" }))]))
                        .collect()),
                ),
                ("curves_denominator_cheaper_unresolved_numerator_cheaper", arr(vec![ju(n24 as u64), ju(nu as u64), ju(n6 as u64)])),
            ]));
            if nm == "cheap_cheap" {
                // the five curves where the order changes: ratio at 0 and at 1 GAE
                let unres: Vec<usize> = (0..8).filter(|&ci| cls[ci] == "unresolved").collect();
                let r0: Vec<f64> = unres.iter().map(|&ci| per_c[ci][0]).collect();
                let r1: Vec<f64> = unres.iter().map(|&ci| per_c[ci][2]).collect();
                let (a0, b0_) = minmax(&r0);
                let (a1, b1) = minmax(&r1);
                cx.put(&format!("cc_unres_{kt}_r0_min"), a0);
                cx.put(&format!("cc_unres_{kt}_r0_max"), b0_);
                cx.put(&format!("cc_unres_{kt}_r1_min"), a1);
                cx.put(&format!("cc_unres_{kt}_r1_max"), b1);
                let held: Vec<u64> = (0..8).filter(|&ci| cls[ci] == "order24").map(|ci| PS[ci]).collect();
                cx.put(&format!("cc_held_{kt}_code"), held.iter().fold(0.0, |acc, &p| acc * 10000.0 + p as f64));
                let changes_at_0p1: Vec<u64> = unres.iter().filter(|&&ci| per_c[ci][1] < 1.0).map(|&ci| PS[ci]).collect();
                let changes_at_1: Vec<u64> = unres.iter().filter(|&&ci| per_c[ci][1] > 1.0 && per_c[ci][2] < 1.0).map(|&ci| PS[ci]).collect();
                cx.checks.push(obj(vec![
                    ("check", js(&format!("per curve, cheap order 6 / cheap order 24 ({}): curves where the order-24 walk is cheaper across the band, and where the order changes", kname(k)))),
                    ("holds", jb(true)),
                    ("detail", js(&format!("cheaper across the band on p = {held:?}; order changes at 0.1 GAE on {changes_at_0p1:?}, at 1 GAE on {changes_at_1:?}; at 0 GAE {a0:.4}-{b0_:.4}, at 1 GAE {a1:.4}-{b1:.4} on the unresolved curves"))),
                ]));
            }
            if nm == "e29_cheap" {
                let un: Vec<u64> = (0..8).filter(|&ci| cls[ci] == "unresolved").map(|ci| PS[ci]).collect();
                cx.put(&format!("ec_unres_{kt}_p"), un.iter().fold(0.0, |acc, &p| acc * 10000.0 + p as f64));
                cx.check(
                    &format!("per curve, E29 order 6 / cheap order 24 ({}): the only unresolved curve is p = 769, the other seven favour the cheap order-24 walk across the band", kname(k)),
                    un == vec![769] && n24 == 7,
                    format!("unresolved {un:?}, order 24 across the band on {n24}"),
                );
            }
            if nm == "o6b_o6c" {
                let r0: Vec<f64> = (0..8).map(|ci| per_c[ci][0]).collect();
                let (lo, h) = minmax(&r0);
                cx.put(&format!("o6own_curve_{kt}_min"), lo);
                cx.put(&format!("o6own_curve_{kt}_max"), h);
            }
            if nm == "cheap_cheap" {
                let held: Vec<u64> = (0..8).filter(|&ci| cls[ci] == "order24").map(|ci| PS[ci]).collect();
                cx.check(
                    &format!("per curve, cheap order 6 / cheap order 24 ({}): the cheap order-24 walk is cheaper across the band on exactly p = 601, 997, 1069", kname(k)),
                    held == vec![601, 997, 1069] && nu == 5,
                    format!("held {held:?}, unresolved {nu}"),
                );
            }
        }
    }
    // sampling error of the pooled ratios (delta method; the order-6 and order-24
    // walks are independent samples, 16 walks per curve and arm)
    let mut se_rows: Vec<J> = Vec::new();
    for (nm, a, b) in [("e29_cheap", O6B, O24C), ("cheap_cheap", O6C, O24C), ("e29_e29", O6B, O24B)] {
        for k in 0..2 {
            let kt = if k == 0 { "sb" } else { "k" };
            let mut vals = Vec::new();
            for (wi, w) in WS.iter().enumerate() {
                let side = |arm: usize| -> (f64, f64) {
                    let (mut s, mut var) = (0.0, 0.0);
                    for c in &d.curves {
                        let v: Vec<f64> = (0..16u64)
                            .map(|sd| {
                                let x = c.get(arm, k == 1, 1000 + sd);
                                (x.fp - x.vfp) as f64 + w * GAE * x.ops.iter().sum::<u64>() as f64
                            })
                            .collect();
                        let m = v.iter().sum::<f64>() / 16.0;
                        let s2 = v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / 15.0;
                        s += m / (c.n as f64).sqrt();
                        var += s2 / 16.0 / c.n as f64;
                    }
                    (s, var)
                };
                let (sa_, va) = side(a);
                let (sb2, vb) = side(b);
                let r = sa_ / sb2;
                let se = r * (va / (sa_ * sa_) + vb / (sb2 * sb2)).sqrt();
                cx.put(&format!("se_{nm}_{kt}_{}", WN[wi]), se);
                // the ratio the rho heuristic predicts from the measured per-step costs
                // (expected steps sqrt(pi n / (2 m)): the sqrt(n) cancels in cost / sqrt(n))
                let ps = |arm: usize| -> f64 {
                    let m = if arm >= O24B { 48.0 } else { 12.0 };
                    (0..8).map(|ci| d.agg[ci][arm][k].cost(*w, GAE, false) / (d.agg[ci][arm][k].steps / d.agg[ci][arm][k].k)).sum::<f64>()
                        * (std::f64::consts::PI / m).sqrt()
                };
                let heur = ps(a) / ps(b);
                cx.put(&format!("heur_{nm}_{kt}_{}", WN[wi]), heur);
                vals.push(obj(vec![
                    ("weight_gae", js(WN[wi])),
                    ("pooled_ratio", jf(r)),
                    ("standard_error", jf(se)),
                    ("z_from_1", jf((r - 1.0) / se)),
                    ("ratio_under_rho_heuristic_steps", jf(heur)),
                ]));
            }
            se_rows.push(obj(vec![("pair", js(&format!("{} / {}", ARMS[a], ARMS[b]))), ("arithmetic", js(kname(k))), ("weights", arr(vals))]));
        }
    }
    cx.checks.push(obj(vec![
        ("check", js("sampling error of the pooled band ratios (informational, 16 walks per curve and arm, delta method)")),
        ("holds", jb(true)),
        ("detail", js(&format!(
            "E29 order 6 / cheap order 24, schoolbook: 1 GAE {:.3} +- {:.3}; cheap order 6 / cheap order 24, schoolbook: 0 GAE {:.3} +- {:.3}, 0.1 GAE {:.3} +- {:.3}, 1 GAE {:.3} +- {:.3} (Karatsuba 1 GAE {:.3} +- {:.3}): the 1 GAE margin over the cheap order-6 walk is {:.1} standard errors; with the rho heuristic's step counts (ratio 2.000 instead of the observed 1.701) the cheap/cheap ratio would be {:.3} / {:.3} / {:.3}",
            cx.vals["band_e29_cheap_sb_1"], cx.vals["se_e29_cheap_sb_1"],
            cx.vals["band_cheap_cheap_sb_0"], cx.vals["se_cheap_cheap_sb_0"],
            cx.vals["band_cheap_cheap_sb_0.1"], cx.vals["se_cheap_cheap_sb_0.1"],
            cx.vals["band_cheap_cheap_sb_1"], cx.vals["se_cheap_cheap_sb_1"],
            cx.vals["band_cheap_cheap_k_1"], cx.vals["se_cheap_cheap_k_1"],
            (cx.vals["band_cheap_cheap_sb_1"] - 1.0) / cx.vals["se_cheap_cheap_sb_1"],
            cx.vals["heur_cheap_cheap_sb_0"], cx.vals["heur_cheap_cheap_sb_0.1"], cx.vals["heur_cheap_cheap_sb_1"]
        ))),
    ]));
    // one-op-per-step convention: every cheap order-24 row (E29/cheap, cheap/cheap, both arithmetics)
    let mut v1: Vec<f64> = vec![];
    for nm in ["e29_cheap", "cheap_cheap"] {
        for kt in ["sb", "k"] {
            for w in WN {
                v1.push(cx.vals[&format!("band1op_{nm}_{kt}_{w}")]);
            }
        }
    }
    let (a1, b1) = minmax(&v1);
    cx.put("band1op_cheap_rows_min", a1);
    cx.put("band1op_cheap_rows_max", b1);
    let mut v1b: Vec<f64> = v1.clone();
    for kt in ["sb", "k"] {
        for w in WN {
            v1b.push(cx.vals[&format!("band1op_cheap_mullean_{kt}_{w}")]);
            v1b.push(cx.vals[&format!("band1op_e29_mullean_{kt}_{w}")]);
        }
    }
    let (a1b, b1b) = minmax(&v1b);
    cx.put("band1op_all_cheap24_rows_min", a1b);
    cx.put("band1op_all_cheap24_rows_max", b1b);

    // steps ratio
    let sr: Vec<f64> = (0..8).map(|ci| d.agg[ci][O6B][0].steps / d.agg[ci][O24B][0].steps).collect();
    let (s0, s1) = minmax(&sr);
    cx.put("steps_ratio_min", s0);
    cx.put("steps_ratio_max", s1);
    let mut sa = 0.0;
    let mut sb_ = 0.0;
    for (ci, c) in d.curves.iter().enumerate() {
        let s = (c.n as f64).sqrt();
        sa += d.agg[ci][O6B][0].steps / d.agg[ci][O6B][0].k / s;
        sb_ += d.agg[ci][O24B][0].steps / d.agg[ci][O24B][0].k / s;
    }
    cx.put("steps_ratio_pooled", sa / sb_);
    // order-6 walk's own cheaper representative at 0 GAE
    let o6own = [cx.vals["band_o6b_o6c_sb_0"], cx.vals["band_o6b_o6c_k_0"]];
    let (o0, o1) = minmax(&o6own);
    cx.put("o6_own_min", o0);
    cx.put("o6_own_max", o1);

    // strongest walks per weight (pooled cost per sqrt(n), every arm, each arithmetic)
    let mut strongest: Vec<J> = Vec::new();
    let mut strong_ok = true;
    for k in 0..2 {
        let kt = if k == 0 { "sb" } else { "k" };
        for (wi, w) in WS.iter().enumerate() {
            let costs: Vec<f64> = (0..5).map(|a| pooled_cost(&d, a, k, *w, GAE)).collect();
            let best = (0..5).min_by(|&x, &y| costs[x].partial_cmp(&costs[y]).unwrap()).unwrap();
            let best6 = if costs[O6B] <= costs[O6C] { O6B } else { O6C };
            let best24 = [O24B, O24C, O24L].into_iter().min_by(|&x, &y| costs[x].partial_cmp(&costs[y]).unwrap()).unwrap();
            let expect24 = if wi == 0 { O24L } else { O24C };
            strong_ok &= best6 == O6C && best24 == expect24 && best == expect24;
            let f = costs[O6B] / costs[best];
            cx.put(&format!("e18_factor_{kt}_{}", WN[wi]), f);
            cx.put(&format!("e18_bits_{kt}_{}", WN[wi]), 4.0 * f.log2());
            cx.put(&format!("strongest_cost_{kt}_{}", WN[wi]), costs[best]);
            cx.put(&format!("e29o6_cost_{kt}_{}", WN[wi]), costs[O6B]);
            strongest.push(obj(vec![
                ("arithmetic", js(kname(k))),
                ("weight_gae", js(WN[wi])),
                ("pooled_cost_per_sqrt_n_by_arm", obj(ARMS.iter().enumerate().map(|(a, n)| (*n, jf(costs[a]))).collect())),
                ("strongest_walk", js(ARMS[best])),
                ("strongest_order6", js(ARMS[best6])),
                ("strongest_order24", js(ARMS[best24])),
                ("e29_order6_over_strongest", jf(f)),
                ("handover_shift_bits_4log2", jf(4.0 * f.log2())),
            ]));
        }
    }
    // strongest over both arithmetics (Karatsuba is cheaper in multiplications, equal in native operations)
    let mut strongest_any: Vec<J> = Vec::new();
    for (wi, w) in WS.iter().enumerate() {
        let mut best = (f64::INFINITY, 0usize, 0usize);
        for k in 0..2 {
            for a in 0..5 {
                let c = pooled_cost(&d, a, k, *w, GAE);
                if c < best.0 {
                    best = (c, a, k);
                }
            }
        }
        strongest_any.push(obj(vec![
            ("weight_gae", js(WN[wi])),
            ("strongest_walk", js(ARMS[best.1])),
            ("arithmetic", js(kname(best.2))),
            ("pooled_cost_per_sqrt_n", jf(best.0)),
        ]));
    }
    cx.check(
        "strongest walks, pooled, both arithmetics: order 6 the cheap one at every weight; order 24 the mul-lean one at 0 GAE and the op-lean cheap one at 0.1 and 1 GAE; the overall strongest is that order-24 walk",
        strong_ok,
        format!("{strong_ok}"),
    );

    // ---------------------------------------------------------------- §9.33 correction (E29's own records)
    let mut e29agg: BTreeMap<(u64, bool, u64), (f64, f64, f64)> = BTreeMap::new(); // mults, steps, k
    for ((p, kara, ord, _), x) in &e29 {
        let e = e29agg.entry((*p, *kara, *ord)).or_insert((0.0, 0.0, 0.0));
        e.0 += (x.get("fp_muls").u() - x.get("verify_fp_muls").u()) as f64;
        e.1 += x.get("steps").u() as f64;
        e.2 += 1.0;
    }
    let mut c933: Vec<J> = Vec::new();
    for (k, kara) in [false, true].iter().enumerate() {
        let kt = if k == 0 { "sb" } else { "k" };
        let mut row = vec![];
        let mut n24_at1 = 0;
        for (wi, w) in WS.iter().enumerate() {
            let (mut a, mut b) = (0.0, 0.0);
            for c in &d.curves {
                let s = (c.n as f64).sqrt();
                let x6 = e29agg[&(c.p, *kara, 6)];
                let x24 = e29agg[&(c.p, *kara, 24)];
                let c6 = (x6.0 + w * GAE * x6.1) / x6.2;
                let c24 = (x24.0 + w * GAE * x24.1) / x24.2;
                a += c6 / s;
                b += c24 / s;
                if wi == 2 && c6 > c24 {
                    n24_at1 += 1;
                }
            }
            cx.put(&format!("s933_{kt}_{}", WN[wi]), a / b);
            row.push(a / b);
        }
        cx.put(&format!("s933_{kt}_curves24_at1"), n24_at1 as f64);
        c933.push(obj(vec![
            ("arithmetic", js(kname(k))),
            ("e29_records_one_op_per_step_0_0p1_1", jfs(&row)),
            ("order24_cheaper_curves_at_1_gae", ju(n24_at1)),
            (
                "e34_built_arms_every_operation_0_0p1_1",
                jfs(&WN.map(|w| cx.vals[&format!("band_e29_e29_{kt}_{w}")])),
            ),
            (
                "e34_built_arms_curves_order6_cheaper_across_band",
                ju(cx.vals[&format!("curves_e29_e29_{kt}_b")] as u64),
            ),
        ]));
    }
    let corr_ok = (0..2).all(|k| {
        let kt = if k == 0 { "sb" } else { "k" };
        WN.iter().all(|w| cx.vals[&format!("band_e29_e29_{kt}_{w}")] < 1.0) && cx.vals[&format!("s933_{kt}_1")] > 1.0
    });
    cx.check(
        "§9.33 correction (pooled, the reading §9.33's table and §9.39's sentence use): with every emitted operation priced, E29's order-24 walk loses to E29's order-6 walk at every weight in both arithmetics, while E29's own records under the one-operation-per-step convention cross at 1 GAE",
        corr_ok,
        format!(
            "per curve (not claimed by the sentence; see erratum E4): order 6 cheaper across the band on {}/8 (schoolbook) and {}/8 (Karatsuba), unresolved on {} and {}; every operation, pooled: schoolbook {:.4}/{:.4}/{:.4}, Karatsuba {:.4}/{:.4}/{:.4}; one op per step from E29's records: schoolbook {:.4}/{:.4}/{:.4}, Karatsuba {:.4}/{:.4}/{:.4}",
            cx.vals["curves_e29_e29_sb_b"], cx.vals["curves_e29_e29_k_b"], cx.vals["curves_e29_e29_sb_u"], cx.vals["curves_e29_e29_k_u"],
            cx.vals["band_e29_e29_sb_0"], cx.vals["band_e29_e29_sb_0.1"], cx.vals["band_e29_e29_sb_1"],
            cx.vals["band_e29_e29_k_0"], cx.vals["band_e29_e29_k_0.1"], cx.vals["band_e29_e29_k_1"],
            cx.vals["s933_sb_0"], cx.vals["s933_sb_0.1"], cx.vals["s933_sb_1"],
            cx.vals["s933_k_0"], cx.vals["s933_k_0.1"], cx.vals["s933_k_1"]
        ),
    );

    // ---------------------------------------------------------------- pre-registration and snapshot
    let (_, pre_h) = git(repo, &["rev-parse", &format!("{PREREG}^{{commit}}")]);
    let (_, code_h) = git(repo, &["rev-parse", &format!("{CODE}^{{commit}}")]);
    let (_, head_h) = git(repo, &["rev-parse", "HEAD"]);
    let head_short: String = head_h.chars().take(12).collect();
    let (_, pre_t) = git(repo, &["log", "-1", "--format=%ct", &pre_h]);
    let (_, code_t) = git(repo, &["log", "-1", "--format=%ct", &code_h]);
    let (_, rec_add) = git(repo, &["log", "--diff-filter=A", "--reverse", "--format=%H %ct", "--", &format!("{E34}/stage_r_p241.json")]);
    let rec_add = rec_add.lines().next().unwrap_or("").to_string();
    let rec_h = rec_add.split(' ').next().unwrap_or("").to_string();
    let rec_t = rec_add.split(' ').nth(1).unwrap_or("0").to_string();
    let (_, mod_add) = git(repo, &["log", "--diff-filter=A", "--reverse", "--format=%H", "--", "src/cryptanalysis/twist24_rep.rs"]);
    let (_, ex_add) = git(repo, &["log", "--diff-filter=A", "--reverse", "--format=%H", "--", "examples/e34_cheap_rep.rs"]);
    let anc1 = git(repo, &["merge-base", "--is-ancestor", &pre_h, &code_h]).0;
    let anc2 = git(repo, &["merge-base", "--is-ancestor", &code_h, &rec_h]).0;
    let t_ok2 = pre_t.parse::<u64>().unwrap_or(u64::MAX) < code_t.parse::<u64>().unwrap_or(0)
        && code_t.parse::<u64>().unwrap_or(u64::MAX) < rec_t.parse::<u64>().unwrap_or(0);
    let mod_first = mod_add.lines().next().unwrap_or("") == code_h && ex_add.lines().next().unwrap_or("") == code_h;
    cx.check(
        "pre-registration commit 45378560d is an ancestor of the code commit bd0118f39 and older; the records commit is a descendant of the code commit and newer; twist24_rep.rs and e34_cheap_rep.rs first appear in bd0118f39",
        anc1 && anc2 && t_ok2 && mod_first,
        format!("prereg {pre_h} @{pre_t}; code {code_h} @{code_t}; records {rec_h} @{rec_t}; ancestry {anc1}/{anc2}; first-add {mod_first}"),
    );
    let (_, note_pre) = git(repo, &["show", &format!("{pre_h}:{NOTE}")]);
    let (_, note_head) = git(repo, &["show", &format!("HEAD:{NOTE}")]);
    let s_pre = section(&note_pre, "### 9.38 ", "\n### 9.39 ");
    let s_head = section(&note_head, "### 9.38 ", "\n### 9.39 ");
    cx.check(
        "§9.38 (G2.134-G2.143) at HEAD is identical to its text at the pre-registration commit",
        s_pre == s_head && s_pre.contains("G2.143"),
        format!("{} bytes, equal {}", s_pre.len(), s_pre == s_head),
    );
    let raw: Vec<String> = PS.iter().map(|p| format!("{E34}/stage_r_p{p}.json")).chain([format!("{E34}/identity.json"), format!("{E34}/negative_walks.json"), format!("{E34}/binary_commit.txt")]).collect();
    let mut args = vec!["diff", "--quiet", rec_h.as_str(), "HEAD", "--"];
    for r in &raw {
        args.push(r.as_str());
    }
    let unchanged = git(repo, &args).0;
    let mut sargs = vec!["status", "--porcelain", "--"];
    for r in &raw {
        sargs.push(r.as_str());
    }
    let (_, st) = git(repo, &sargs);
    cx.check(
        "raw E34 records at HEAD are byte-identical to the records commit and the working tree is clean for them (committed snapshot)",
        unchanged && st.is_empty(),
        format!("unchanged since {rec_h} at HEAD {head_short}: {unchanged}; working-tree status '{st}'"),
    );
    let bc = fs::read_to_string(format!("{repo}/{E34}/binary_commit.txt")).unwrap();
    let bl: Vec<&str> = bc.lines().collect();
    cx.check(
        "binary_commit.txt names a prefix of bd0118f39 and records a binary sha256 and the dirty-worktree caveat",
        code_h.starts_with(bl[0].trim()) && bl.len() >= 3 && bl[1].trim().len() == 64,
        format!("{:?}", bl),
    );

    // ---------------------------------------------------------------- headlines
    let src9 = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.39";
    let srcb = "E34's claims as named by the validation task (§9.39 and the 2026-11-01 session log), at the precision stated there";
    let src933 = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.33";
    let pd = "schoolbook, p >= 541, per curve mean F_p mults / mean steps";
    let pn = "schoolbook, p >= 541, pooled (sum of native operations / sum of steps)";
    let bd = "pooled order 6 / order 24, every emitted operation at w*97";
    let cd = "per-curve test: ratio on one side of 1 at 0, 0.1 and 1 GAE";
    // Headlines: the claims the validation task names, at the precision it states,
    // and §9.33's figures that the claimed correction is about.
    let hl_main: Vec<(&str, &str, &str, &str, &str)> = vec![
        ("E29 Stage R records reproduced by the counted copies", srcb, "512", "e29_records_identical", "records equal field for field (visit_digest on order 6 only: E29's order-24 records carry 0)"),
        ("order-24 class members checked, cheap representative", srcb, "39936", "o24_members", "sum of identity members, Order24Cheap"),
        ("order-24 class members checked, mul-lean representative", srcb, "39936", "o24l_members", "sum of identity members, Order24CheapMulLean"),
        ("order-6 class members checked", srcb, "9984", "o6_members", "sum of identity members, Order6Cheap"),
        ("mismatches, cheap order-24 representative", srcb, "0", "o24_mismatch", "sum over 8 curves"),
        ("mismatches, mul-lean representative", srcb, "0", "o24l_mismatch", "sum over 8 curves"),
        ("mismatches, cheap order-6 representative", srcb, "0", "o6_mismatch", "sum over 8 curves"),
        ("cheap walks equal to their built walk", srcb, "768", "walk_identity", "steps, group_ops, cycle_escapes, recovered, correct, visit_digest"),
        ("omitted-candidate control: mismatches", srcb, "3328", "omit_mismatch", "sum over 8 curves"),
        ("omitted-candidate walks whose digest differs from E29's, p = 241", srcb, "16", "neg_digest_differs", "of 16, schoolbook"),
        ("wrong-ordinate control: mismatches (every member)", srcb, "39936", "wrong_mismatch", "of 39,936 members"),
        ("order 6 E29: F_p mults per step, least", srcb, "109", "o6b_sb_hi_mps_min", pd),
        ("order 6 E29: F_p mults per step, most", srcb, "111", "o6b_sb_hi_mps_max", pd),
        ("order 6 E29: native ops per step", srcb, "16.8", "o6b_sb_hi_nps_pooled", pn),
        ("order 6 cheap: F_p mults per step, least", srcb, "105", "o6c_sb_hi_mps_min", pd),
        ("order 6 cheap: F_p mults per step, most", srcb, "107", "o6c_sb_hi_mps_max", pd),
        ("order 6 cheap: native ops per step", srcb, "15.2", "o6c_sb_hi_nps_pooled", pn),
        ("order 24 E29: F_p mults per step, least", srcb, "191", "o24b_sb_hi_mps_min", pd),
        ("order 24 E29: F_p mults per step, most", srcb, "199", "o24b_sb_hi_mps_max", pd),
        ("order 24 E29: native ops per step", srcb, "65.3", "o24b_sb_hi_nps_pooled", pn),
        ("order 24 cheap: F_p mults per step, least", srcb, "117", "o24c_sb_hi_mps_min", pd),
        ("order 24 cheap: F_p mults per step, most", srcb, "120", "o24c_sb_hi_mps_max", pd),
        ("order 24 cheap: native ops per step", srcb, "25.8", "o24c_sb_hi_nps_pooled", pn),
        ("order 24 mul-lean: F_p mults per step, least", srcb, "112", "o24l_sb_hi_mps_min", pd),
        ("order 24 mul-lean: F_p mults per step, most", srcb, "115", "o24l_sb_hi_mps_max", pd),
        ("order 24 mul-lean: native ops per step", srcb, "31.0", "o24l_sb_hi_nps_pooled", pn),
        ("band E29/E29 schoolbook, 0 GAE", srcb, "0.962", "band_e29_e29_sb_0", bd),
        ("band E29/E29 schoolbook, 0.1 GAE", srcb, "0.562", "band_e29_e29_sb_0.1", bd),
        ("band E29/E29 schoolbook, 1 GAE", srcb, "0.453", "band_e29_e29_sb_1", bd),
        ("band E29/cheap schoolbook, 0 GAE", srcb, "1.563", "band_e29_cheap_sb_0", bd),
        ("band E29/cheap schoolbook, 0.1 GAE", srcb, "1.254", "band_e29_cheap_sb_0.1", bd),
        ("band E29/cheap schoolbook, 1 GAE", srcb, "1.126", "band_e29_cheap_sb_1", bd),
        ("band E29/cheap Karatsuba, 0 GAE", srcb, "1.544", "band_e29_cheap_k_0", bd),
        ("band E29/cheap Karatsuba, 0.1 GAE", srcb, "1.232", "band_e29_cheap_k_0.1", bd),
        ("band E29/cheap Karatsuba, 1 GAE", srcb, "1.122", "band_e29_cheap_k_1", bd),
        ("band cheap/cheap schoolbook, 0 GAE", srcb, "1.505", "band_cheap_cheap_sb_0", bd),
        ("band cheap/cheap schoolbook, 0.1 GAE", srcb, "1.167", "band_cheap_cheap_sb_0.1", bd),
        ("band cheap/cheap schoolbook, 1 GAE", srcb, "1.026", "band_cheap_cheap_sb_1", bd),
        ("band cheap/cheap Karatsuba, 0 GAE", srcb, "1.476", "band_cheap_cheap_k_0", bd),
        ("band cheap/cheap Karatsuba, 0.1 GAE", srcb, "1.140", "band_cheap_cheap_k_0.1", bd),
        ("band cheap/cheap Karatsuba, 1 GAE", srcb, "1.021", "band_cheap_cheap_k_1", bd),
        ("curves where the cheap order-24 walk beats E29's order-6 walk across the band, schoolbook", srcb, "7", "curves_e29_cheap_sb_a", cd),
        ("curves unresolved against E29's order-6 walk, schoolbook (p = 769)", srcb, "1", "curves_e29_cheap_sb_u", cd),
        ("curves where the cheap order-24 walk beats E29's order-6 walk across the band, Karatsuba", srcb, "7", "curves_e29_cheap_k_a", cd),
        ("curves unresolved against E29's order-6 walk, Karatsuba", srcb, "1", "curves_e29_cheap_k_u", cd),
        ("curves where the cheap order-24 walk beats the cheap order-6 walk across the band, schoolbook (p = 601, 997, 1069)", srcb, "3", "curves_cheap_cheap_sb_a", cd),
        ("curves unresolved against the cheap order-6 walk, schoolbook", srcb, "5", "curves_cheap_cheap_sb_u", cd),
        ("curves where the cheap order-24 walk beats the cheap order-6 walk across the band, Karatsuba", srcb, "3", "curves_cheap_cheap_k_a", cd),
        ("curves unresolved against the cheap order-6 walk, Karatsuba", srcb, "5", "curves_cheap_cheap_k_u", cd),
        ("E18 psi-arm distance lengthening, schoolbook, 0 GAE", srcb, "1.632", "e18_factor_sb_0", "E29 order-6 pooled cost / strongest walk's (mul-lean)"),
        ("E18 psi-arm distance lengthening, schoolbook, 0.1 GAE", srcb, "1.254", "e18_factor_sb_0.1", "E29 order-6 pooled cost / strongest walk's (cheap order 24)"),
        ("E18 psi-arm distance lengthening, schoolbook, 1 GAE", srcb, "1.126", "e18_factor_sb_1", "E29 order-6 pooled cost / strongest walk's (cheap order 24)"),
        ("E18 handover shift, schoolbook, 0 GAE (bits)", srcb, "2.83", "e18_bits_sb_0", "4 log2 of the factor"),
        ("E18 handover shift, schoolbook, 0.1 GAE (bits)", srcb, "1.31", "e18_bits_sb_0.1", "4 log2 of the factor"),
        ("E18 handover shift, schoolbook, 1 GAE (bits)", srcb, "0.68", "e18_bits_sb_1", "4 log2 of the factor"),
        ("§9.33 one op per step from E29's records, schoolbook, 0 GAE", src933, "0.962", "s933_sb_0", "E29 Stage R records, order 6 / order 24, steps at w*97"),
        ("§9.33 one op per step, schoolbook, 0.1 GAE", src933, "0.996", "s933_sb_0.1", "same"),
        ("§9.33 one op per step, schoolbook, 1 GAE", src933, "1.205", "s933_sb_1", "same"),
        ("§9.33 one op per step, Karatsuba, 0 GAE", src933, "0.884", "s933_k_0", "same"),
        ("§9.33 one op per step, Karatsuba, 0.1 GAE", src933, "0.927", "s933_k_0.1", "same"),
        ("§9.33 one op per step, Karatsuba, 1 GAE", src933, "1.172", "s933_k_1", "same"),
        ("§9.33 curves where the order-24 walk is cheaper at 1 GAE (one op per step)", src933, "7", "s933_sb_curves24_at1", "schoolbook, E29's records"),
    ];
    // Every other figure printed in §9.39, at its printed precision.
    let hl_note: Vec<(&str, &str, &str, &str, &str)> = vec![
        ("class members checked, order 24 + order 6", src9, "49920", "members_total", "Order24Cheap + Order6Cheap members"),
        ("omitted-candidate mismatches per curve, least", src9, "416", "omit_per_curve_min", "of 4,992 per curve"),
        ("omitted-candidate mismatches per curve, most", src9, "416", "omit_per_curve_max", "of 4,992 per curve"),
        ("omitted-candidate walks that still recover the log", src9, "16", "neg_correct", "of 16"),
        ("E29 order-6 representative F_p mults per call (positive control)", src9, "8.000", "o6b_sb_all_rpc_max", "rep_fp_muls / rep_calls, largest curve (least is 8 too: check)"),
        ("cheap order-6 representative F_p mults per call", src9, "4.00", "o6c_sb_hi_rpc_pooled", "p >= 541, pooled"),
        ("E29 order-24 representative F_p mults per call, least", src9, "86", "o24b_sb_all_rpc_min", "per curve, all 8"),
        ("E29 order-24 representative F_p mults per call, most", src9, "92", "o24b_sb_all_rpc_max", "per curve, all 8"),
        ("cheap order-24 representative F_p mults per call (table)", src9, "15.17", "o24c_sb_hi_rpc_pooled", "p >= 541, pooled"),
        ("G2.138 cheap order-24 F_p mults per call, least", src9, "15.16", "o24c_sb_all_rpc_min", "per curve, all 8"),
        ("G2.138 cheap order-24 F_p mults per call, most", src9, "15.18", "o24c_sb_all_rpc_max", "per curve, all 8"),
        ("mul-lean F_p mults per call (table)", src9, "10.17", "o24l_sb_hi_rpc_pooled", "p >= 541, pooled"),
        ("G2.138 mul-lean F_p mults per call, least", src9, "10.17", "o24l_sb_all_rpc_min", "per curve, all 8"),
        ("G2.138 mul-lean F_p mults per call, most", src9, "10.18", "o24l_sb_all_rpc_max", "per curve, all 8"),
        ("table: order 6 E29 mults per step, least", src9, "109.1", "o6b_sb_hi_mps_min", pd),
        ("table: order 6 E29 mults per step, most", src9, "111.2", "o6b_sb_hi_mps_max", pd),
        ("table: order 6 cheap mults per step, least", src9, "105.0", "o6c_sb_hi_mps_min", pd),
        ("table: order 6 cheap mults per step, most", src9, "107.1", "o6c_sb_hi_mps_max", pd),
        ("table: order 24 E29 mults per step, least", src9, "191.0", "o24b_sb_hi_mps_min", pd),
        ("table: order 24 E29 mults per step, most", src9, "199.4", "o24b_sb_hi_mps_max", pd),
        ("table: order 24 cheap mults per step, least", src9, "117.2", "o24c_sb_hi_mps_min", pd),
        ("table: order 24 cheap mults per step, most", src9, "120.2", "o24c_sb_hi_mps_max", pd),
        ("table: mul-lean mults per step, least", src9, "112.1", "o24l_sb_hi_mps_min", pd),
        ("table: mul-lean mults per step, most", src9, "115.1", "o24l_sb_hi_mps_max", pd),
        ("table: order 6 E29 native ops per step", src9, "16.77", "o6b_sb_hi_nps_pooled", pn),
        ("table: order 6 cheap native ops per step", src9, "15.22", "o6c_sb_hi_nps_pooled", pn),
        ("table: order 24 E29 native ops per step", src9, "65.25", "o24b_sb_hi_nps_pooled", pn),
        ("table: order 24 cheap native ops per step", src9, "25.82", "o24c_sb_hi_nps_pooled", pn),
        ("table: mul-lean native ops per step", src9, "30.96", "o24l_sb_hi_nps_pooled", pn),
        ("table: order 6 E29 cmp per step", src9, "3.10", "o6b_sb_hi_cmp_pooled", pn),
        ("table: order 6 E29 neg per step", src9, "4.13", "o6b_sb_hi_neg_pooled", pn),
        ("table: order 6 E29 modn per step", src9, "5.55", "o6b_sb_hi_modn_pooled", pn),
        ("table: order 6 E29 hash per step", src9, "2.00", "o6b_sb_hi_hash_pooled", pn),
        ("table: order 6 E29 table per step", src9, "2.00", "o6b_sb_hi_table_pooled", pn),
        ("table: order 6 cheap cmp per step", src9, "4.65", "o6c_sb_hi_cmp_pooled", pn),
        ("table: order 6 cheap neg per step", src9, "2.06", "o6c_sb_hi_neg_pooled", pn),
        ("table: order 6 cheap modn per step", src9, "4.52", "o6c_sb_hi_modn_pooled", pn),
        ("table: order 6 cheap hash per step", src9, "2.00", "o6c_sb_hi_hash_pooled", pn),
        ("table: order 6 cheap table per step", src9, "2.00", "o6c_sb_hi_table_pooled", pn),
        ("table: order 24 E29 cmp per step", src9, "16.52", "o24b_sb_hi_cmp_pooled", pn),
        ("table: order 24 E29 neg per step", src9, "4.13", "o24b_sb_hi_neg_pooled", pn),
        ("table: order 24 E29 modn per step", src9, "40.61", "o24b_sb_hi_modn_pooled", pn),
        ("table: order 24 E29 hash per step", src9, "2.00", "o24b_sb_hi_hash_pooled", pn),
        ("table: order 24 E29 table per step", src9, "2.00", "o24b_sb_hi_table_pooled", pn),
        ("table: order 24 cheap cmp per step", src9, "13.95", "o24c_sb_hi_cmp_pooled", pn),
        ("table: order 24 cheap neg per step", src9, "3.35", "o24c_sb_hi_neg_pooled", pn),
        ("table: order 24 cheap modn per step", src9, "4.52", "o24c_sb_hi_modn_pooled", pn),
        ("table: order 24 cheap hash per step", src9, "2.00", "o24c_sb_hi_hash_pooled", pn),
        ("table: order 24 cheap table per step", src9, "2.00", "o24c_sb_hi_table_pooled", pn),
        ("table: mul-lean cmp per step", src9, "13.95", "o24l_sb_hi_cmp_pooled", pn),
        ("table: mul-lean neg per step", src9, "8.50", "o24l_sb_hi_neg_pooled", pn),
        ("table: mul-lean modn per step", src9, "4.52", "o24l_sb_hi_modn_pooled", pn),
        ("table: mul-lean hash per step", src9, "2.00", "o24l_sb_hi_hash_pooled", pn),
        ("table: mul-lean table per step", src9, "2.00", "o24l_sb_hi_table_pooled", pn),
        ("G2.140 order 24 E29 native ops per step, least", src9, "65.25", "o24b_sb_all_nps_min", "per curve, all 8"),
        ("G2.140 order 24 E29 native ops per step, most", src9, "65.29", "o24b_sb_all_nps_max", "per curve, all 8"),
        ("G2.140 order 24 cheap native ops per step, least", src9, "25.81", "o24c_sb_all_nps_min", "per curve, all 8"),
        ("G2.140 order 24 cheap native ops per step, most", src9, "25.87", "o24c_sb_all_nps_max", "per curve, all 8"),
        ("G2.140 cheap representative cmp per call, least", src9, "13.52", "o24c_sb_all_cpc_min", "per curve, all 8"),
        ("G2.140 cheap representative cmp per call, most", src9, "13.59", "o24c_sb_all_cpc_max", "per curve, all 8"),
        ("cheap representative comparisons a call", src9, "13.5", "o24c_sb_all_cpc_pooled", "all 8 curves, pooled"),
        ("E29 order-24 walk: mod-n operations per step", src9, "40.6", "o24b_sb_hi_modn_pooled", pn),
        ("G2.139 cheap order 24 / order 6 E29 per-step mults, schoolbook, least", src9, "1.072", "g139_sb_min", "per curve, p >= 541"),
        ("G2.139 schoolbook, most", src9, "1.086", "g139_sb_max", "per curve, p >= 541"),
        ("G2.139 Karatsuba, least", src9, "1.086", "g139_k_min", "per curve, p >= 541"),
        ("G2.139 Karatsuba, most", src9, "1.100", "g139_k_max", "per curve, p >= 541"),
        ("band cheap/mul-lean schoolbook, 0 GAE", src9, "1.572", "band_cheap_mullean_sb_0", bd),
        ("band cheap/mul-lean schoolbook, 0.1 GAE", src9, "1.042", "band_cheap_mullean_sb_0.1", bd),
        ("band cheap/mul-lean schoolbook, 1 GAE", src9, "0.864", "band_cheap_mullean_sb_1", bd),
        ("band cheap/cheap Karatsuba with GAE 79, 0 GAE", src9, "1.476", "band79_cheap_cheap_0", "Karatsuba, its own GAE 79"),
        ("band cheap/cheap Karatsuba with GAE 79, 0.1 GAE", src9, "1.160", "band79_cheap_cheap_0.1", "Karatsuba, its own GAE 79"),
        ("band cheap/cheap Karatsuba with GAE 79, 1 GAE", src9, "1.025", "band79_cheap_cheap_1", "Karatsuba, its own GAE 79"),
        ("one op per step: every cheap order-24 row, least", src9, "1.48", "band1op_all_cheap24_rows_min", "both cheap order-24 arms against both order-6 arms, both arithmetics, three weights"),
        ("one op per step: every cheap order-24 row, most", src9, "1.66", "band1op_all_cheap24_rows_max", "same"),
        ("band table: E29/E29 schoolbook, curves where order 6 is cheaper across the band ('order 6 on all 8')", src9, "8", "curves_e29_e29_sb_b", cd),
        ("band table: E29/E29 schoolbook, curves unresolved ('0')", src9, "0", "curves_e29_e29_sb_u", cd),
        ("cheap/cheap on the five unresolved curves, 0 GAE, least", src9, "1.16", "cc_unres_sb_r0_min", "schoolbook, per curve"),
        ("cheap/cheap on the five unresolved curves, 0 GAE, most", src9, "1.86", "cc_unres_sb_r0_max", "schoolbook, per curve"),
        ("cheap/cheap on the five unresolved curves, 1 GAE, least", src9, "0.78", "cc_unres_sb_r1_min", "schoolbook, per curve"),
        ("cheap/cheap on the five unresolved curves, 1 GAE, most", src9, "0.99", "cc_unres_sb_r1_max", "schoolbook, per curve"),
        ("steps ratio order 6 / order 24 per curve, least", src9, "1.29", "steps_ratio_min", "mean steps, E29's walks"),
        ("steps ratio per curve, most", src9, "2.10", "steps_ratio_max", "mean steps"),
        ("steps ratio pooled", src9, "1.701", "steps_ratio_pooled", "sum of mean steps / sqrt(n)"),
        ("the order-6 walk's own cheaper representative at 0 GAE, least", src9, "1.035", "o6own_curve_sb_min", "E29 order 6 / cheap order 6, per curve, schoolbook"),
        ("the order-6 walk's own cheaper representative at 0 GAE, most", src9, "1.039", "o6own_curve_sb_max", "same"),
        ("E18 lengthening, Karatsuba, 0 GAE", src9, "1.627", "e18_factor_k_0", "E29 order-6 pooled cost / strongest walk's"),
        ("E18 lengthening, Karatsuba, 0.1 GAE", src9, "1.232", "e18_factor_k_0.1", "same"),
        ("E18 lengthening, Karatsuba, 1 GAE", src9, "1.122", "e18_factor_k_1", "same"),
        ("E18 handover shift, Karatsuba, 0 GAE (bits)", src9, "2.81", "e18_bits_k_0", "4 log2 of the factor"),
        ("E18 handover shift, Karatsuba, 0.1 GAE (bits)", src9, "1.20", "e18_bits_k_0.1", "same"),
        ("E18 handover shift, Karatsuba, 1 GAE (bits)", src9, "0.66", "e18_bits_k_1", "same"),
    ];
    let (heads, disagree) = eval_figures(&hl_main, &cx.vals);
    let (notes, note_dis) = eval_figures(&hl_note, &cx.vals);

    // ---------------------------------------------------------------- verdicts
    let v = |k: &str| cx.vals[k];
    let inr = |x: f64, a: f64, b: f64| x >= a && x <= b;
    let both = |f: &dyn Fn(&str) -> bool| f("sb") && f("k");
    let g138 = v("o6b_sb_all_rpc_min") == 8.0 && v("o6b_sb_all_rpc_max") == 8.0
        && inr(v("o6c_sb_all_rpc_min"), 3.0, 5.0) && inr(v("o6c_sb_all_rpc_max"), 3.0, 5.0)
        && inr(v("o24b_sb_all_rpc_min"), 85.0, 100.0) && inr(v("o24b_sb_all_rpc_max"), 85.0, 100.0)
        && inr(v("o24c_sb_all_rpc_min"), 12.0, 20.0) && inr(v("o24c_sb_all_rpc_max"), 12.0, 20.0)
        && inr(v("o24l_sb_all_rpc_min"), 7.0, 15.0) && inr(v("o24l_sb_all_rpc_max"), 7.0, 15.0)
        && rep_eq_arith;
    let g139 = both(&|k| inr(v(&format!("g139_{k}_min")), 1.0, 1.15) && inr(v(&format!("g139_{k}_max")), 1.0, 1.15));
    let g140 = both(&|k| {
        inr(v(&format!("o6b_{k}_all_nps_min")), 14.0, 19.0) && inr(v(&format!("o6b_{k}_all_nps_max")), 14.0, 19.0)
            && inr(v(&format!("o6c_{k}_all_nps_min")), 12.0, 17.0) && inr(v(&format!("o6c_{k}_all_nps_max")), 12.0, 17.0)
            && inr(v(&format!("o24b_{k}_all_nps_min")), 40.0, 70.0) && inr(v(&format!("o24b_{k}_all_nps_max")), 40.0, 70.0)
            && inr(v(&format!("o24c_{k}_all_nps_min")), 22.0, 30.0) && inr(v(&format!("o24c_{k}_all_nps_max")), 22.0, 30.0)
            && inr(v(&format!("o24c_{k}_all_cpc_min")), 11.0, 14.0) && inr(v(&format!("o24c_{k}_all_cpc_max")), 11.0, 14.0)
    });
    let g141 = both(&|k| {
        inr(v(&format!("band_e29_cheap_{k}_0")), 1.40, 1.80)
            && inr(v(&format!("band_e29_cheap_{k}_0.1")), 1.10, 1.45)
            && inr(v(&format!("band_e29_cheap_{k}_1")), 1.00, 1.30)
            && v(&format!("band_e29_cheap_{k}_1")) > 1.0
            && v(&format!("curves_e29_cheap_{k}_a")) >= 6.0
    });
    let g142 = both(&|k| {
        v(&format!("band_cheap_cheap_{k}_0")) > 1.3
            && v(&format!("band_cheap_cheap_{k}_0.1")) > 1.0
            && inr(v(&format!("band_cheap_cheap_{k}_1")), 0.90, 1.10)
    });
    let g143 = inr(v("e18_factor_sb_0"), 1.4, 1.8) && inr(v("e18_bits_sb_0"), 1.9, 3.4)
        && inr(v("e18_factor_k_0"), 1.4, 1.8) && inr(v("e18_bits_k_0"), 1.9, 3.4);
    let g134 = c24.1 == 0 && l24.1 == 0 && c6.1 == 0 && omit_per_curve.iter().all(|&m| m > 0) && wr.1 == wr.0;
    let g135 = ident_recs == 512;
    let g136 = same_walks == 768;
    let g137 = nd == 16 && nn == 16;
    let verdicts = vec![
        ("G2.134", g134, format!("0 mismatches on {} order-24 members (each cheap representative) and {} order-6 members; omitted candidate caught on 8/8 curves ({} total), wrong ordinate {}/{}; the x = 0 case has no point on these curves (x_zero_point false on all 40 checks)", c24.0, c6.0, om.1, wr.1, wr.0)),
        ("G2.135", g135, format!("{ident_recs}/512 records identical (registered '256' is the count per arithmetic); visit_digest compared on order 6 only (E29's order-24 records carry 0)")),
        ("G2.136", g136, format!("{same_walks}/768")),
        ("G2.137", g137, format!("{nd}/16")),
        ("G2.138", g138, format!("8.000; {:.3}-{:.3}; {:.2}-{:.2}; {:.3}-{:.3}; {:.3}-{:.3}; equal in both arithmetics {rep_eq_arith}", v("o6c_sb_all_rpc_min"), v("o6c_sb_all_rpc_max"), v("o24b_sb_all_rpc_min"), v("o24b_sb_all_rpc_max"), v("o24c_sb_all_rpc_min"), v("o24c_sb_all_rpc_max"), v("o24l_sb_all_rpc_min"), v("o24l_sb_all_rpc_max"))),
        ("G2.139", g139, format!("schoolbook {:.4}-{:.4}, Karatsuba {:.4}-{:.4}", v("g139_sb_min"), v("g139_sb_max"), v("g139_k_min"), v("g139_k_max"))),
        ("G2.140", g140, format!("order 6 E29 {:.3}-{:.3}; cheap {:.3}-{:.3}; order 24 E29 {:.3}-{:.3}; cheap {:.3}-{:.3}, cmp per call {:.3}-{:.3} (all 8 curves; identical in both arithmetics)", v("o6b_sb_all_nps_min"), v("o6b_sb_all_nps_max"), v("o6c_sb_all_nps_min"), v("o6c_sb_all_nps_max"), v("o24b_sb_all_nps_min"), v("o24b_sb_all_nps_max"), v("o24c_sb_all_nps_min"), v("o24c_sb_all_nps_max"), v("o24c_sb_all_cpc_min"), v("o24c_sb_all_cpc_max"))),
        ("G2.141", g141, format!("schoolbook {:.4}/{:.4}/{:.4}, Karatsuba {:.4}/{:.4}/{:.4}; curves {}/8 (sb), {}/8 (K)", v("band_e29_cheap_sb_0"), v("band_e29_cheap_sb_0.1"), v("band_e29_cheap_sb_1"), v("band_e29_cheap_k_0"), v("band_e29_cheap_k_0.1"), v("band_e29_cheap_k_1"), v("curves_e29_cheap_sb_a"), v("curves_e29_cheap_k_a"))),
        ("G2.142", g142, format!("schoolbook {:.4}/{:.4}/{:.4}, Karatsuba {:.4}/{:.4}/{:.4}", v("band_cheap_cheap_sb_0"), v("band_cheap_cheap_sb_0.1"), v("band_cheap_cheap_sb_1"), v("band_cheap_cheap_k_0"), v("band_cheap_cheap_k_0.1"), v("band_cheap_cheap_k_1"))),
        ("G2.143", g143, format!("derived: factor {:.4}/{:.4}/{:.4} (sb), {:.4}/{:.4}/{:.4} (K); handover +{:.2}/{:.2}/{:.2} bits (sb), +{:.2}/{:.2}/{:.2} (K); 0 GAE inside the registered [1.4, 1.8] and [1.9, 3.4]", v("e18_factor_sb_0"), v("e18_factor_sb_0.1"), v("e18_factor_sb_1"), v("e18_factor_k_0"), v("e18_factor_k_0.1"), v("e18_factor_k_1"), v("e18_bits_sb_0"), v("e18_bits_sb_0.1"), v("e18_bits_sb_1"), v("e18_bits_k_0"), v("e18_bits_k_0.1"), v("e18_bits_k_1"))),
    ];
    let mut vj = Vec::new();
    for (id, holds, det) in &verdicts {
        cx.check(&format!("{id} holds against its registered threshold"), *holds, det.clone());
        vj.push(obj(vec![("id", js(id)), ("verdict", js(if *holds { "holds" } else { "falsified" })), ("recomputed", js(det))]));
    }

    // ---------------------------------------------------------------- raw data
    let mut rawj = Vec::new();
    let mut raw_paths: Vec<String> = raw.clone();
    raw_paths.push(format!("{E29}/stage_r_p241.json"));
    raw_paths.push(format!("{E29}/stage_r_rest.json"));
    for r in &raw_paths {
        rawj.push(obj(vec![("path", js(&format!("../../{r}"))), ("sha256", js(&sha_file(&format!("{repo}/{r}"))))]));
    }

    let status = cx.fails.is_empty() && disagree.is_empty();
    let rep = obj(vec![
        ("schema", js("validator-report/v1")),
        ("status", js(if status { "pass" } else { "fail" })),
        ("independent", jb(true)),
        ("validator", js("independent validator subagent (Claude Opus 5.5), not the session that produced E34; this recompute is its own standard-library Rust program (validate_e34.rs: own JSON parser, own SHA-256) and shares no code with examples/e34_cheap_rep.rs, examples/e34_tables.rs, experiments/38_twist24_cheap_rep/summary.json or tables.md")),
        ("date", js("2026-11-01")),
        ("row", js("E34 (GLV note §9.38 pre-registration, §9.39 results): a cheaper order-24 class representative on E18's eight j = 0 curves; five walk arms x two arithmetics x 16 seeds per curve, identity checks and negative controls; a rho-against-rho reference correction for E18's psi rows")),
        ("script", js("research/e34_twist24_cheap_rep_20261031/validate_e34.rs (rustc -O --edition 2021; `validate_e34 report <repo> research/e34_twist24_cheap_rep_20261031`)")),
        ("status_rule", js("pass iff every check holds and every headline agrees with its printed figure within half a unit of the last printed digit (exact for counts). Headlines are the claims the validation task names, at the precision it states them, and §9.33's figures that the claimed correction concerns. Every other figure printed in §9.39 is checked the same way under note_figures; one that disagrees is reported as an erratum (errata), and would fail the report only if it changed a registered verdict or a headline")),
        ("cost_definition", js("walk cost(w) = mean over the 16 seeds of (fp_muls - verify_fp_muls) + w * 97 * mean native operations (cmp + neg + modn + hash + table), w = 0, 0.1, 1; pooled ratio = sum over the 8 curves of cost / sqrt(n), arm A over arm B; per-curve ratio = cost A / cost B; an arm is ranked over another only where the ratio stays on one side of 1 at all three weights")),
        ("raw_data", arr(rawj)),
        ("check_failures", arr(cx.fails.iter().map(|s| js(s)).collect())),
        ("headline_disagreements", arr(disagree.iter().map(|s| js(s)).collect())),
        ("checks", arr(cx.checks.clone())),
        ("headline", arr(heads)),
        ("note_figures", arr(notes)),
        ("note_figure_disagreements", arr(note_dis.iter().map(|s| js(s)).collect())),
        ("verdicts_recomputed", arr(vj)),
        ("band", arr(band_rows)),
        ("strongest_walks", arr(strongest)),
        ("strongest_walk_over_both_arithmetics", arr(strongest_any)),
        ("section_9_33_correction", arr(c933)),
        ("logarithms", arr(logs)),
        ("steps_heuristic", arr(steps_rows)),
        ("pooled_ratio_sampling_error", arr(se_rows)),
        ("per_curve", arr(per_curve)),
    ]);
    let out = format!("{evdir}/validator-report.json");
    let text = to_text(&rep);
    // splice in the manual sections written by the validator (gate refusal, errata, limitations, attestation)
    let extra_path = format!("{evdir}/replay/report_extra.json");
    let text = if let Ok(t) = fs::read_to_string(&extra_path) {
        let extra = parse(&t).expect("report_extra.json");
        let mut r = parse(&text).unwrap();
        if let (J::Obj(v), J::Obj(e)) = (&mut r, extra) {
            for (k, x) in e {
                v.push((k, x));
            }
        }
        to_text(&r)
    } else {
        text
    };
    fs::write(&out, text).unwrap();
    println!(
        "status {} | checks {} failing {} | headlines {} disagreeing {} | note figures {} disagreeing {}",
        if status { "pass" } else { "fail" },
        cx.checks.len(),
        cx.fails.len(),
        hl_main.len(),
        disagree.len(),
        hl_note.len(),
        note_dis.len()
    );
    for x in &note_dis {
        println!("NOTE FIGURE DISAGREES: {x}");
    }
    for f in &cx.fails {
        println!("FAIL check: {f}");
    }
    for x in &disagree {
        println!("DISAGREE: {x}");
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("selftest") => {
            let ok = selftest();
            println!("selftest {}", if ok { "pass" } else { "FAIL" });
            if !ok {
                std::process::exit(1);
            }
        }
        Some("sha256") => {
            for f in &a[2..] {
                println!("{}  {}", sha_file(f), f);
            }
        }
        Some("draw") => {
            // order the eight curves by SHA-256(nonce ":" p); the first three are the mandatory sample
            let nonce = &a[2];
            let mut v: Vec<(String, u64)> = PS.iter().map(|p| (sha256(format!("{nonce}:{p}").as_bytes()), *p)).collect();
            v.sort();
            println!("{}", v.iter().map(|(_, p)| p.to_string()).collect::<Vec<_>>().join(","));
        }
        Some("compare") => compare_mode(&a[2], &a[3], &a[4]),
        Some("report") => {
            assert!(selftest(), "selftest failed");
            report(&a[2], &a[3])
        }
        _ => {
            eprintln!("usage: validate_e34 selftest | sha256 <f>... | draw <nonce> | compare <repo> <replay_dir> <out> | report <repo> <evidence_dir>");
            std::process::exit(2);
        }
    }
}
