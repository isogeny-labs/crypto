//! E41 validator (independent; standard library only).  Shares no code with
//! `examples/e41_run.rs`, `examples/e41_tables.rs`, `examples/e41_stage0.rs`,
//! `experiments/48_twist24_shortfall/summary.json` or `tables.md`: its own JSON
//! parser (number and string text kept verbatim), its own SHA-256, its own
//! statistics.  Python is not used for any number.
//!
//!   rustc -O --edition 2021 validate_e41.rs -o validate_e41
//!   validate_e41 sha256 <file>...
//!   validate_e41 draw <nonce-hex> <outdir>          selection files for the replay driver
//!   validate_e41 compare <repo> <replay-dir>        replay outputs against the committed records
//!   validate_e41 report <repo>                      the recompute (JSON on stdout)

use std::collections::BTreeMap;
use std::env;
use std::fs;

// ---------------------------------------------------------------- JSON

#[derive(Clone, Debug, PartialEq)]
enum J {
    Null,
    Bool(bool),
    Num(String),
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
    fn raw_string(&mut self) -> String {
        assert_eq!(self.s[self.i], b'"', "string expected at {}", self.i);
        self.i += 1;
        let st = self.i;
        while self.s[self.i] != b'"' {
            self.i += if self.s[self.i] == b'\\' { 2 } else { 1 };
        }
        let r = String::from_utf8(self.s[st..self.i].to_vec()).unwrap();
        self.i += 1;
        r
    }
    fn lit(&mut self, w: &str) {
        assert_eq!(&self.s[self.i..self.i + w.len()], w.as_bytes());
        self.i += w.len();
    }
    fn val(&mut self) -> J {
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
                    self.ws();
                    let k = self.raw_string();
                    self.ws();
                    assert_eq!(self.s[self.i], b':');
                    self.i += 1;
                    let x = self.val();
                    v.push((k, x));
                    self.ws();
                    match self.s[self.i] {
                        b',' => self.i += 1,
                        b'}' => {
                            self.i += 1;
                            return J::Obj(v);
                        }
                        c => panic!("bad object char {} at {}", c as char, self.i),
                    }
                }
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
                    v.push(self.val());
                    self.ws();
                    match self.s[self.i] {
                        b',' => self.i += 1,
                        b']' => {
                            self.i += 1;
                            return J::Arr(v);
                        }
                        c => panic!("bad array char {} at {}", c as char, self.i),
                    }
                }
            }
            b'"' => J::Str(self.raw_string()),
            b't' => {
                self.lit("true");
                J::Bool(true)
            }
            b'f' => {
                self.lit("false");
                J::Bool(false)
            }
            b'n' => {
                self.lit("null");
                J::Null
            }
            _ => {
                let st = self.i;
                while self.i < self.s.len() && matches!(self.s[self.i], b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') {
                    self.i += 1;
                }
                assert!(self.i > st, "number expected at {}", st);
                J::Num(String::from_utf8(self.s[st..self.i].to_vec()).unwrap())
            }
        }
    }
}

fn parse(text: &str) -> J {
    let mut p = Parser { s: text.as_bytes(), i: 0 };
    let v = p.val();
    p.ws();
    assert_eq!(p.i, p.s.len(), "trailing data");
    v
}

fn load(path: &str) -> (String, J) {
    let t = fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let j = parse(&t);
    (t, j)
}

/// Compact serialization with verbatim number and string text.
fn ser(j: &J, o: &mut String) {
    match j {
        J::Null => o.push_str("null"),
        J::Bool(b) => o.push_str(if *b { "true" } else { "false" }),
        J::Num(t) => o.push_str(t),
        J::Str(s) => {
            o.push('"');
            o.push_str(s);
            o.push('"');
        }
        J::Arr(v) => {
            o.push('[');
            for (i, x) in v.iter().enumerate() {
                if i > 0 {
                    o.push(',');
                }
                ser(x, o);
            }
            o.push(']');
        }
        J::Obj(v) => {
            o.push('{');
            for (i, (k, x)) in v.iter().enumerate() {
                if i > 0 {
                    o.push(',');
                }
                o.push('"');
                o.push_str(k);
                o.push_str("\":");
                ser(x, o);
            }
            o.push('}');
        }
    }
}

fn compact(j: &J) -> String {
    let mut s = String::new();
    ser(j, &mut s);
    s
}

fn pretty(j: &J, ind: usize, o: &mut String) {
    let pad = |n: usize| " ".repeat(n);
    match j {
        J::Arr(v) if !v.is_empty() => {
            if v.iter().all(|x| !matches!(x, J::Arr(_) | J::Obj(_))) {
                ser(j, o);
                return;
            }
            o.push_str("[\n");
            for (i, x) in v.iter().enumerate() {
                o.push_str(&pad(ind + 1));
                pretty(x, ind + 1, o);
                if i + 1 < v.len() {
                    o.push(',');
                }
                o.push('\n');
            }
            o.push_str(&pad(ind));
            o.push(']');
        }
        J::Obj(v) if !v.is_empty() => {
            o.push_str("{\n");
            for (i, (k, x)) in v.iter().enumerate() {
                o.push_str(&pad(ind + 1));
                o.push('"');
                o.push_str(k);
                o.push_str("\": ");
                pretty(x, ind + 1, o);
                if i + 1 < v.len() {
                    o.push(',');
                }
                o.push('\n');
            }
            o.push_str(&pad(ind));
            o.push('}');
        }
        _ => ser(j, o),
    }
}

fn get<'a>(j: &'a J, k: &str) -> &'a J {
    if let J::Obj(v) = j {
        for (kk, x) in v {
            if kk == k {
                return x;
            }
        }
    }
    panic!("no key {k}")
}
fn has(j: &J, k: &str) -> bool {
    matches!(j, J::Obj(v) if v.iter().any(|(kk, _)| kk == k))
}
fn fnum(j: &J) -> f64 {
    match j {
        J::Num(t) => t.parse().unwrap(),
        _ => panic!("number expected, got {j:?}"),
    }
}
fn unum(j: &J) -> u64 {
    match j {
        J::Num(t) => t.parse().unwrap(),
        _ => panic!("integer expected, got {j:?}"),
    }
}
fn st(j: &J) -> &str {
    match j {
        J::Str(s) => s,
        _ => panic!("string expected"),
    }
}
fn bo(j: &J) -> bool {
    match j {
        J::Bool(b) => *b,
        _ => panic!("bool expected"),
    }
}
fn arr(j: &J) -> &Vec<J> {
    match j {
        J::Arr(v) => v,
        _ => panic!("array expected"),
    }
}
fn ntext(j: &J) -> String {
    match j {
        J::Num(t) => t.clone(),
        J::Null => "null".into(),
        _ => panic!("number or null expected"),
    }
}

// output builders
fn n(x: f64) -> J {
    if x.is_finite() {
        J::Num(format!("{x}"))
    } else {
        J::Null
    }
}
fn nr(x: f64, d: usize) -> J {
    J::Num(format!("{:.*}", d, x))
}
fn iu(x: u64) -> J {
    J::Num(x.to_string())
}
fn s(x: &str) -> J {
    J::Str(x.replace('\\', "\\\\").replace('"', "\\\""))
}
fn b(x: bool) -> J {
    J::Bool(x)
}
fn o(v: Vec<(&str, J)>) -> J {
    J::Obj(v.into_iter().map(|(k, x)| (k.to_string(), x)).collect())
}
fn a(v: Vec<J>) -> J {
    J::Arr(v)
}

/// Leaf-by-leaf comparison: key order, array lengths, verbatim scalars.
fn cmp_tree(x: &J, y: &J, path: &str, leaves: &mut u64, same: &mut u64, mism: &mut Vec<String>) {
    match (x, y) {
        (J::Obj(u), J::Obj(v)) => {
            let ku: Vec<&String> = u.iter().map(|p| &p.0).collect();
            let kv: Vec<&String> = v.iter().map(|p| &p.0).collect();
            if ku != kv {
                *leaves += 1;
                mism.push(format!("{path}: keys {ku:?} vs {kv:?}"));
                return;
            }
            for ((k, a1), (_, b1)) in u.iter().zip(v.iter()) {
                cmp_tree(a1, b1, &format!("{path}.{k}"), leaves, same, mism);
            }
        }
        (J::Arr(u), J::Arr(v)) => {
            if u.len() != v.len() {
                *leaves += 1;
                mism.push(format!("{path}: lengths {} vs {}", u.len(), v.len()));
                return;
            }
            for (i, (a1, b1)) in u.iter().zip(v.iter()).enumerate() {
                cmp_tree(a1, b1, &format!("{path}[{i}]"), leaves, same, mism);
            }
        }
        _ => {
            *leaves += 1;
            if x == y {
                *same += 1;
            } else {
                mism.push(format!("{path}: {} vs {}", compact(x), compact(y)));
            }
        }
    }
}

// ---------------------------------------------------------------- SHA-256

fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let mut m = data.to_vec();
    let bitlen = (data.len() as u64).wrapping_mul(8);
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&bitlen.to_be_bytes());
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
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

// ---------------------------------------------------------------- number theory

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}
fn is_prime(nn: u64) -> bool {
    if nn < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if nn % q == 0 {
            return nn == q;
        }
    }
    let (mut d, mut r) = (nn - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    'w: for a0 in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a0, d, nn);
        if x == 1 || x == nn - 1 {
            continue;
        }
        for _ in 1..r {
            x = mulmod(x, x, nn);
            if x == nn - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

// ---------------------------------------------------------------- records

const PS: [u64; 8] = [241, 337, 541, 601, 733, 769, 997, 1069];
const GAE: f64 = 97.0;
const WS: [f64; 3] = [0.0, 0.1, 1.0];
const MAIN_LABELS: [&str; 5] = ["O6B", "O6C", "O24", "O24L", "O24F"];

#[derive(Clone, Debug)]
struct Walk {
    label: String,
    jumps: String,
    arm: String,
    kar: bool,
    fold: String,
    seed: u64,
    steps: u64,
    escapes: u64,
    fp: u64,
    vfp: u64,
    vgo: u64,
    ops: [u64; 5], // cmp hash modn neg table
    rep_calls: u64,
    rep_fp: u64,
    rep_ops: [u64; 5],
    table_fp: u64,
    correct: bool,
    recovered: String,
    digest: String,
    group_ops: u64,
    // instrument
    class_size: u64,
    classes: u64,
    ideal_rec: f64,
    stored: u64,
    fruitless: u64,
    fruitless_len: Vec<u64>,
    fruitless_cycle_steps: u64,
    redirects: u64,
    jumps_n: u64,
    added: [u64; 3], // group ops, fp muls, modn
    escape_rehits: u64,
    tie_inputs: u64,
}

impl Walk {
    fn native(&self) -> u64 {
        self.ops.iter().sum()
    }
    fn cost(&self, w: f64) -> f64 {
        (self.fp - self.vfp) as f64 + w * GAE * self.native() as f64
    }
}

fn ops5(j: &J) -> [u64; 5] {
    [unum(get(j, "cmp")), unum(get(j, "hash")), unum(get(j, "modn")), unum(get(j, "neg")), unum(get(j, "table"))]
}

fn walk_of(rec: &J) -> Walk {
    let w = get(rec, "walk");
    let run = get(w, "run");
    let sf = get(rec, "shortfall");
    Walk {
        label: st(get(rec, "label")).to_string(),
        jumps: st(get(rec, "jumps")).to_string(),
        arm: st(get(w, "arm")).to_string(),
        kar: bo(get(w, "karatsuba")),
        fold: st(get(run, "fold")).to_string(),
        seed: unum(get(run, "seed")),
        steps: unum(get(run, "steps")),
        escapes: unum(get(run, "cycle_escapes")),
        fp: unum(get(run, "fp_muls")),
        vfp: unum(get(run, "verify_fp_muls")),
        vgo: unum(get(run, "verify_group_ops")),
        ops: ops5(get(w, "ops")),
        rep_calls: unum(get(w, "rep_calls")),
        rep_fp: unum(get(w, "rep_fp_muls")),
        rep_ops: ops5(get(w, "rep_ops")),
        table_fp: unum(get(w, "table_fp_muls")),
        correct: bo(get(run, "correct")),
        recovered: ntext(get(run, "recovered")),
        digest: ntext(get(run, "visit_digest")),
        group_ops: unum(get(run, "group_ops")),
        class_size: unum(get(sf, "class_size")),
        classes: unum(get(sf, "classes")),
        ideal_rec: fnum(get(sf, "ideal_steps")),
        stored: unum(get(sf, "stored")),
        fruitless: unum(get(sf, "fruitless")),
        fruitless_len: arr(get(sf, "fruitless_len")).iter().map(unum).collect(),
        fruitless_cycle_steps: unum(get(sf, "fruitless_cycle_steps")),
        redirects: unum(get(sf, "redirects")),
        jumps_n: unum(get(sf, "jumps")),
        added: [unum(get(sf, "added_group_ops")), unum(get(sf, "added_fp_muls")), unum(get(sf, "added_modn"))],
        escape_rehits: unum(get(sf, "escape_rehits")),
        tie_inputs: unum(get(sf, "tie_inputs")),
    }
}

struct Curve {
    p: u64,
    n: u64,
    b0: Option<u64>,
    walks: Vec<Walk>,
}

fn load_run(path: &str) -> Vec<Curve> {
    let (_, j) = load(path);
    arr(&j)
        .iter()
        .map(|c| Curve {
            p: unum(get(c, "p")),
            n: unum(get(c, "n")),
            b0: if has(c, "b0") { Some(unum(get(c, "b0"))) } else { None },
            walks: arr(get(c, "walks")).iter().map(walk_of).collect(),
        })
        .collect()
}

fn of<'a>(run: &'a [Curve], p: u64, label: &str) -> Vec<&'a Walk> {
    let c = run.iter().find(|c| c.p == p).unwrap();
    let mut v: Vec<&Walk> = c.walks.iter().filter(|w| w.label == label).collect();
    v.sort_by_key(|w| w.seed);
    v
}

fn ideal(nn: u64, m: u64) -> f64 {
    (std::f64::consts::PI * ((nn - 1) / m) as f64 / 2.0).sqrt()
}

// ---------------------------------------------------------------- statistics

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn svar(v: &[f64]) -> f64 {
    let m = mean(v);
    v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() as f64 - 1.0)
}
fn scov(x: &[f64], y: &[f64]) -> f64 {
    let (mx, my) = (mean(x), mean(y));
    x.iter().zip(y).map(|(a1, b1)| (a1 - mx) * (b1 - my)).sum::<f64>() / (x.len() as f64 - 1.0)
}

/// Pooled Σ_p mean(q)/√n and its variance Σ_p var(q)/N/n.
fn pool(run: &[Curve], label: &str, q: &dyn Fn(&Walk) -> f64) -> (f64, f64) {
    let (mut x, mut v) = (0.0, 0.0);
    for c in run {
        let xs: Vec<f64> = of(run, c.p, label).iter().map(|w| q(w)).collect();
        assert!(!xs.is_empty(), "no walks for {label} at {}", c.p);
        x += mean(&xs) / (c.n as f64).sqrt();
        v += svar(&xs) / xs.len() as f64 / c.n as f64;
    }
    (x, v)
}

/// Paired covariance Σ_p cov(qa, qb)/N/n over seed-matched walks.
fn pool_cov(run: &[Curve], la: &str, lb: &str, q: &dyn Fn(&Walk) -> f64) -> f64 {
    let mut cv = 0.0;
    for c in run {
        let wa = of(run, c.p, la);
        let wb = of(run, c.p, lb);
        assert_eq!(wa.len(), wb.len());
        for (x, y) in wa.iter().zip(&wb) {
            assert_eq!(x.seed, y.seed);
        }
        let xa: Vec<f64> = wa.iter().map(|w| q(w)).collect();
        let xb: Vec<f64> = wb.iter().map(|w| q(w)).collect();
        cv += scov(&xa, &xb) / xa.len() as f64 / c.n as f64;
    }
    cv
}

struct Ratio {
    r: f64,
    se: f64,
    se_paired: Option<f64>,
}

fn ratio(ra: &[Curve], la: &str, rb: &[Curve], lb: &str, paired: bool, q: &dyn Fn(&Walk) -> f64) -> Ratio {
    let (xa, va) = pool(ra, la, q);
    let (xb, vb) = pool(rb, lb, q);
    let r = xa / xb;
    let rel = va / (xa * xa) + vb / (xb * xb);
    let se_paired = if paired {
        let cv = pool_cov(ra, la, lb, q);
        Some(r * (rel - 2.0 * cv / (xa * xb)).sqrt())
    } else {
        None
    };
    Ratio { r, se: r * rel.sqrt(), se_paired }
}

fn ci(r: &Ratio) -> (f64, f64) {
    (r.r - 1.96 * r.se, r.r + 1.96 * r.se)
}

/// Agreement of a printed figure with a recomputed value: within half a unit
/// of the printed figure's last digit (exact for integers).
fn agrees(printed: &str, x: f64) -> bool {
    let t = printed.trim();
    let (mant, exp) = match t.find(['e', 'E']) {
        Some(i) => (&t[..i], t[i + 1..].parse::<i32>().unwrap()),
        None => (t, 0),
    };
    let dec = mant.find('.').map(|i| mant.len() - i - 1).unwrap_or(0) as i32;
    let p: f64 = t.parse().unwrap();
    let half = 0.5 * 10f64.powi(exp - dec);
    (x - p).abs() <= half * (1.0 + 1e-9)
}

// ---------------------------------------------------------------- draw

fn draw(nonce: &str, outdir: &str) {
    let key = |parts: &str| sha256(format!("{nonce}:{parts}").as_bytes());
    let pick = |run: &str, p: u64, extra: &str, total: u64, take: usize| -> Vec<u64> {
        let mut ks: Vec<(String, u64)> = (0..total).map(|k| (key(&format!("{run}:{p}:{extra}{k}")), k)).collect();
        ks.sort();
        ks.into_iter().take(take).map(|x| x.1).collect()
    };
    let mut log = String::new();
    // main stream, schoolbook: 16 of 128 seeds per curve, all five arms.
    let mut f = String::new();
    for p in PS {
        let ks = pick("school", p, "", 128, 16);
        log.push_str(&format!("school p={p} k={ks:?}\n"));
        for k in ks {
            for l in MAIN_LABELS {
                f.push_str(&format!("school {p} {l} {}\n", 48000 + k));
            }
        }
    }
    fs::write(format!("{outdir}/sel_school.txt"), f).unwrap();
    // A/A stream: 16 of 128 per curve.
    let mut f = String::new();
    for p in PS {
        let ks = pick("aa", p, "", 128, 16);
        log.push_str(&format!("aa p={p} k={ks:?}\n"));
        for k in ks {
            f.push_str(&format!("aa {p} O24AA {}\n", 49000 + k));
        }
    }
    fs::write(format!("{outdir}/sel_aa.txt"), f).unwrap();
    // main stream, Karatsuba: 8 of 32 per curve, all five arms.
    let mut f = String::new();
    for p in PS {
        let ks = pick("kar", p, "", 32, 8);
        log.push_str(&format!("kar p={p} k={ks:?}\n"));
        for k in ks {
            for l in MAIN_LABELS {
                f.push_str(&format!("kar {p} {l} {}\n", 48000 + k));
            }
        }
    }
    fs::write(format!("{outdir}/sel_kar.txt"), f).unwrap();
    // Stage 0 (pre-registration instrument run): 2 of 64 per curve and arm.
    let mut f = String::new();
    for p in PS {
        for arm in ["Order6Cheap", "Order24Cheap"] {
            let ks = pick("s0", p, &format!("{arm}:"), 64, 2);
            log.push_str(&format!("s0 p={p} {arm} k={ks:?}\n"));
            for k in ks {
                f.push_str(&format!("s0 {p} {arm} {}\n", 41000 + k));
            }
        }
    }
    fs::write(format!("{outdir}/sel_s0.txt"), f).unwrap();
    fs::write(format!("{outdir}/selection.txt"), log).unwrap();
}

/// The E34 identity population (all 1,280 Stage R walks), in file order.
fn e34_selection(repo: &str, outdir: &str) {
    let mut f = String::new();
    for p in PS {
        let (_, j) = load(&format!("{repo}/experiments/38_twist24_cheap_rep/stage_r_p{p}.json"));
        for w in arr(get(&arr(&j)[0], "walks")) {
            f.push_str(&format!(
                "e34 {p} {} {} {}\n",
                st(get(w, "arm")),
                bo(get(w, "karatsuba")),
                unum(get(get(w, "run"), "seed"))
            ));
        }
    }
    fs::write(format!("{outdir}/sel_e34.txt"), f).unwrap();
}

// ---------------------------------------------------------------- compare

fn compare(repo: &str, rdir: &str) {
    let ex = format!("{repo}/experiments/48_twist24_shortfall");
    let mut runs_out = Vec::new();
    let mut tot = (0u64, 0u64, 0u64, 0u64); // records, byte-identical, leaves, leaves identical
    for (run, file) in [
        ("school", "main_school.json"),
        ("aa", "aa_school.json"),
        ("kar", "main_karatsuba.json"),
        ("s0", "stage0_run.json"),
        ("e34", ""),
    ] {
        let outp = format!("{rdir}/out_{run}.tsv");
        let Ok(text) = fs::read_to_string(&outp) else {
            continue;
        };
        let sel = fs::read_to_string(format!("{rdir}/sel_{run}.txt")).unwrap();
        let committed = if file.is_empty() { None } else { Some(load(&format!("{ex}/{file}")).1) };
        let mut e34: BTreeMap<u64, J> = BTreeMap::new();
        let (mut recs, mut bytes_same, mut leaves, mut same) = (0u64, 0u64, 0u64, 0u64);
        let mut mism = Vec::new();
        let mut walks_correct = 0u64;
        for line in text.lines() {
            let (selline, js) = line.split_once('\t').unwrap();
            let t: Vec<&str> = selline.split_whitespace().collect();
            let p: u64 = t[1].parse().unwrap();
            let mine = parse(js);
            recs += 1;
            let (theirs, theirs_text): (J, String) = match run {
                "e34" => {
                    let c = e34.entry(p).or_insert_with(|| {
                        load(&format!("{repo}/experiments/38_twist24_cheap_rep/stage_r_p{p}.json")).1
                    });
                    let w = arr(get(&arr(c)[0], "walks"))
                        .iter()
                        .find(|w| {
                            st(get(w, "arm")) == t[2]
                                && bo(get(w, "karatsuba")) == (t[3] == "true")
                                && unum(get(get(w, "run"), "seed")) == t[4].parse::<u64>().unwrap()
                        })
                        .expect("E34 walk")
                        .clone();
                    let tt = compact(&w);
                    (w, tt)
                }
                "s0" => {
                    // Stage 0 record (written by e41_stage0 at 6115e0ea0a): compare
                    // every field it has against the replay's walk and instrument.
                    let c = arr(get(committed.as_ref().unwrap(), "curves")).iter().find(|c| unum(get(c, "p")) == p).unwrap();
                    let w = arr(get(c, "walks"))
                        .iter()
                        .find(|w| st(get(w, "arm")) == t[2] && unum(get(w, "seed")) == t[3].parse::<u64>().unwrap())
                        .expect("stage 0 walk")
                        .clone();
                    let tt = compact(&w);
                    (w, tt)
                }
                _ => {
                    let c = arr(committed.as_ref().unwrap()).iter().find(|c| unum(get(c, "p")) == p).unwrap();
                    let w = arr(get(c, "walks"))
                        .iter()
                        .find(|w| {
                            st(get(w, "label")) == t[2]
                                && unum(get(get(get(w, "walk"), "run"), "seed")) == t[3].parse::<u64>().unwrap()
                        })
                        .expect("committed record")
                        .clone();
                    let tt = compact(&w);
                    (w, tt)
                }
            };
            if run == "s0" {
                // Project the replay onto the Stage 0 record's fields.
                let w = get(&mine, "walk");
                let r = get(w, "run");
                let sf = get(&mine, "shortfall");
                let native: u64 = ops5(get(w, "ops")).iter().sum();
                let mut fields = vec![
                    ("arm".to_string(), get(w, "arm").clone()),
                    ("correct".to_string(), get(r, "correct").clone()),
                    ("fp_muls".to_string(), get(r, "fp_muls").clone()),
                    ("native".to_string(), J::Num(native.to_string())),
                    ("seed".to_string(), get(r, "seed").clone()),
                ];
                let mut sfo = Vec::new();
                if let J::Obj(kv) = get(&theirs, "shortfall") {
                    for (k, _) in kv {
                        sfo.push((k.clone(), get(sf, k).clone()));
                    }
                }
                fields.push(("shortfall".to_string(), J::Obj(sfo)));
                fields.push(("steps".to_string(), get(r, "steps").clone()));
                let proj = J::Obj(fields);
                let mut lv = 0;
                let mut sm = 0;
                cmp_tree(&theirs, &proj, &format!("{selline}"), &mut lv, &mut sm, &mut mism);
                leaves += lv;
                same += sm;
                if lv == sm {
                    bytes_same += u64::from(compact(&proj) == theirs_text);
                }
                if bo(get(r, "correct")) {
                    walks_correct += 1;
                }
                continue;
            }
            let mut lv = 0;
            let mut sm = 0;
            cmp_tree(&theirs, &mine, selline, &mut lv, &mut sm, &mut mism);
            leaves += lv;
            same += sm;
            if js == theirs_text {
                bytes_same += 1;
            }
            let runobj = if run == "e34" { get(get(&mine, "run"), "correct") } else { get(get(get(&mine, "walk"), "run"), "correct") };
            if bo(runobj) {
                walks_correct += 1;
            }
        }
        let selected = sel.lines().filter(|l| !l.trim().is_empty()).count() as u64;
        tot.0 += recs;
        tot.1 += bytes_same;
        tot.2 += leaves;
        tot.3 += same;
        runs_out.push(o(vec![
            ("run", s(run)),
            ("committed", s(if file.is_empty() { "experiments/38_twist24_cheap_rep/stage_r_p*.json" } else { file })),
            ("selected", iu(selected)),
            ("replayed", iu(recs)),
            ("byte_identical", iu(bytes_same)),
            ("leaves_compared", iu(leaves)),
            ("leaves_identical", iu(same)),
            ("walks_correct", iu(walks_correct)),
            ("mismatches", a(mism.iter().take(20).map(|m| s(m)).collect())),
            ("replay_output_sha256", s(&sha256(text.as_bytes()))),
            ("selection_sha256", s(&sha256(sel.as_bytes()))),
        ]));
    }
    // Whole-file replays by the rebuilt e41_run binary.
    let mut files_out = Vec::new();
    for (name, committed) in [
        ("identity.json", format!("{ex}/identity.json")),
        ("control.json", format!("{ex}/control.json")),
        ("main_karatsuba.json", format!("{ex}/main_karatsuba.json")),
        ("aa_school.json", format!("{ex}/aa_school.json")),
        ("main_school.json", format!("{ex}/main_school.json")),
    ] {
        let rp = format!("{rdir}/bin/{name}");
        let Ok(rt) = fs::read(&rp) else {
            continue;
        };
        let ct = fs::read(&committed).unwrap();
        let (mut lv, mut sm) = (0, 0);
        let mut mism = Vec::new();
        cmp_tree(&parse(std::str::from_utf8(&ct).unwrap()), &parse(std::str::from_utf8(&rt).unwrap()), name, &mut lv, &mut sm, &mut mism);
        files_out.push(o(vec![
            ("file", s(name)),
            ("committed_sha256", s(&sha256(&ct))),
            ("replay_sha256", s(&sha256(&rt))),
            ("byte_identical", b(ct == rt)),
            ("leaves_compared", iu(lv)),
            ("leaves_identical", iu(sm)),
            ("mismatches", a(mism.iter().take(20).map(|m| s(m)).collect())),
        ]));
    }
    let out = o(vec![
        ("schema", s("e41-validator-replay-comparison/v1")),
        (
            "rule",
            s("each replayed record against the committed one: the replay driver's serialized record must equal the committed record's text byte for byte (the committed files are compact; this program re-serializes them verbatim), and every leaf is compared (key order, array lengths, number and string text). Stage 0 records were written by a different example (e41_stage0 at 6115e0ea0a) in another layout: every field they hold is compared against the replay's walk and instrument (native = cmp + hash + modn + neg + table). Whole files written by the rebuilt e41_run binary are compared byte for byte and leaf by leaf."),
        ),
        ("sampled_runs", a(runs_out)),
        ("whole_file_runs", a(files_out)),
        ("records_replayed", iu(tot.0)),
        ("records_byte_identical", iu(tot.1)),
        ("leaves_compared", iu(tot.2)),
        ("leaves_identical", iu(tot.3)),
    ]);
    let mut t = String::new();
    pretty(&out, 0, &mut t);
    println!("{t}");
}

// ---------------------------------------------------------------- report

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args[0].as_str() {
        "sha256" => {
            for f in &args[1..] {
                println!("{}  {}", sha256(&fs::read(f).unwrap()), f);
            }
        }
        "draw" => {
            draw(&args[1], &args[2]);
            e34_selection(&args[3], &args[2]);
        }
        "compare" => compare(&args[1], &args[2]),
        "report" => report(&args[1]),
        _ => panic!("unknown command"),
    }
}

struct Claims {
    v: Vec<J>,
    cat: Vec<J>,
    all: bool,
}

/// Half a unit of the printed figure's last digit, relative to the printed value
/// (the gate's rel_tol), exact (1e-12) for integers.
fn rel_tol(printed: &str) -> f64 {
    let t = printed.trim();
    let (mant, exp) = match t.find(['e', 'E']) {
        Some(i) => (&t[..i], t[i + 1..].parse::<i32>().unwrap()),
        None => (t, 0),
    };
    let dec = mant.find('.').map(|i| mant.len() - i - 1).unwrap_or(0) as i32;
    let p: f64 = t.parse().unwrap();
    if dec == 0 && exp == 0 {
        return 1e-12;
    }
    0.5 * 10f64.powi(exp - dec) / p.abs() * (1.0 + 1e-6)
}
impl Claims {
    fn add(&mut self, name: &str, printed: &str, x: f64, src: &str) {
        let ok = agrees(printed, x);
        self.all &= ok;
        self.v.push(o(vec![("name", s(name)), ("source", s(src)), ("printed", s(printed)), ("claimed", J::Num(printed.trim().to_string())), ("recomputed", n(x)), ("rel_tol", n(rel_tol(printed))), ("agrees", b(ok))]));
    }
    fn add_count(&mut self, name: &str, printed: &str, x: &str, src: &str) {
        let ok = printed == x;
        self.all &= ok;
        if let (Ok(_), Ok(xv)) = (printed.parse::<f64>(), x.parse::<f64>()) {
            self.v.push(o(vec![("name", s(name)), ("source", s(src)), ("printed", s(printed)), ("claimed", J::Num(printed.to_string())), ("recomputed", n(xv)), ("rel_tol", n(1e-12)), ("agrees", b(ok))]));
        } else {
            self.cat.push(o(vec![("name", s(name)), ("source", s(src)), ("printed", s(printed)), ("recomputed", s(x)), ("agrees", b(ok))]));
        }
    }
}

fn report(repo: &str) {
    let ex = format!("{repo}/experiments/48_twist24_shortfall");
    let school = load_run(&format!("{ex}/main_school.json"));
    let aa = load_run(&format!("{ex}/aa_school.json"));
    let kar = load_run(&format!("{ex}/main_karatsuba.json"));
    let ctl = load_run(&format!("{ex}/control.json"));
    let mut checks: Vec<J> = Vec::new();
    let mut all_checks = true;
    let mut check = |name: &str, holds: bool, detail: String| {
        all_checks &= holds;
        checks.push(o(vec![("check", s(name)), ("holds", b(holds)), ("detail", s(&detail))]));
    };

    // ---- structure, curves, n
    let mut struct_ok = true;
    let mut ndetail = Vec::new();
    for (name, run, labels, base, k) in [
        ("main_school", &school, &MAIN_LABELS[..], 48000u64, 128u64),
        ("aa_school", &aa, &["O24AA"][..], 49000, 128),
        ("main_karatsuba", &kar, &MAIN_LABELS[..], 48000, 32),
    ] {
        struct_ok &= run.iter().map(|c| c.p).collect::<Vec<_>>() == PS.to_vec();
        for c in run.iter() {
            let mut i = 0;
            for kk in 0..k {
                for l in labels {
                    let w = &c.walks[i];
                    struct_ok &= w.label == *l && w.seed == base + kk && w.kar == (name == "main_karatsuba");
                    i += 1;
                }
            }
            struct_ok &= i == c.walks.len();
        }
    }
    check("record layout: eight curves in E18's order; main stream seeds 48000+k (k < 128 schoolbook, k < 32 Karatsuba) x O6B, O6C, O24, O24L, O24F seed-major; A/A stream 49000+k (k < 128) x O24AA; karatsuba flag as named", struct_ok, "every record in place".into());
    let mut nok = true;
    for c in &school {
        let pr = is_prime(c.n);
        nok &= pr && c.n % 24 == 1;
        ndetail.push(format!("p={} n={} prime={} n mod 24={}", c.p, c.n, pr, c.n % 24));
        for r in [&aa, &kar] {
            let c2 = r.iter().find(|x| x.p == c.p).unwrap();
            nok &= c2.n == c.n && c2.b0 == c.b0;
        }
    }
    let b0s: Vec<u64> = school.iter().map(|c| c.b0.unwrap()).collect();
    nok &= b0s == vec![3, 3, 3, 9, 9, 2, 17, 3];
    check("curves: canonical b0 = 3, 3, 3, 9, 9, 2, 17, 3 as registered; n prime (deterministic Miller-Rabin, own code) and n = 1 mod 24 on every curve; n and b0 equal across the three runs", nok, ndetail.join("; "));

    // ---- label to arm and jumps
    let mut lab_ok = true;
    let want = |l: &str| -> (&'static str, &'static str, &'static str, u64) {
        match l {
            "O6B" => ("Order6Built", "E34", "Psi", 32),
            "O6C" => ("Order6Cheap", "E34", "Psi", 32),
            "O24" | "O24AA" => ("Order24Cheap", "E34", "Twist24", 32),
            "O24L" => ("Order24CheapMulLean", "E34", "Twist24", 32),
            "O24F" => ("Order24Cheap", "PairSums", "Twist24", 64),
            "O24F-broken" => ("Order24Cheap", "PairSumsBroken", "Twist24", 64),
            _ => panic!(),
        }
    };
    for run in [&school, &aa, &kar, &ctl] {
        for c in run.iter() {
            for w in &c.walks {
                let (arm, jumps, fold, jn) = want(&w.label);
                lab_ok &= w.arm == arm && w.jumps == jumps && w.fold == fold && w.jumps_n == jn;
                lab_ok &= w.class_size == if fold == "Psi" { 6 } else { 24 };
            }
        }
    }
    check("arms as registered: O6B = Order6Built, O6C = Order6Cheap, O24/O24AA = Order24Cheap, O24L = Order24CheapMulLean (jumps E34, r = 32); O24F = Order24Cheap with PairSums (r = 64); control = PairSumsBroken (r = 64); fold Psi (class size 6) on order 6, Twist24 (24) on order 24", lab_ok, String::new());

    // ---- correctness
    let mut total = 0u64;
    let mut wrong = 0u64;
    let mut rec_ok = true;
    let mut recs = Vec::new();
    for c in &school {
        let d0 = c.walks[0].recovered.clone();
        for run in [&school, &aa, &kar] {
            for w in &run.iter().find(|x| x.p == c.p).unwrap().walks {
                total += 1;
                if !w.correct {
                    wrong += 1;
                }
                rec_ok &= w.recovered == d0 && w.vfp > 0 && w.vgo > 0;
            }
        }
        // E34's logarithm on the same curve
        let (_, e) = load(&format!("{repo}/experiments/38_twist24_cheap_rep/stage_r_p{}.json", c.p));
        let e34d = ntext(get(get(&arr(get(&arr(&e)[0], "walks"))[0], "run"), "recovered"));
        rec_ok &= e34d == d0;
        recs.push(format!("p={}: {}", c.p, d0));
    }
    check("G2.217 correctness: every walk of the three runs reports correct = true and recovers the same logarithm on its curve, equal to E34's recorded logarithm there; each ran the [d]G = Q check (verify counters > 0)", wrong == 0 && rec_ok && total == 7424, format!("{total} walks, {wrong} wrong; logarithms {}", recs.join(", ")));

    // ---- positive control
    let mut cflag = 0u64;
    let mut ctot = 0u64;
    let mut cdetail = Vec::new();
    for c in &ctl {
        let mut seeds_ok = true;
        for (k, w) in c.walks.iter().enumerate() {
            ctot += 1;
            seeds_ok &= w.seed == 48000 + k as u64;
            if !w.correct {
                cflag += 1;
            }
        }
        let main_d = &school.iter().find(|x| x.p == c.p).unwrap().walks[0].recovered;
        let differ = c.walks.iter().filter(|w| &w.recovered != main_d).count();
        cdetail.push(format!("p={}: {} walks seeds 48000-48015 {}, flagged {} , recovered != planted on {}", c.p, c.walks.len(), seeds_ok, c.walks.iter().filter(|w| !w.correct).count(), differ));
    }
    check("G2.218 positive control F-slash (b of each pair jump = b_j): wrong logarithm flagged on >= 31/32 walks at p = 241 and 541, seeds 48000-48015", ctot == 32 && cflag >= 31, format!("{cflag}/{ctot}; {}", cdetail.join("; ")));

    // ---- instrument identities and ideal
    let mut id_ok = true;
    let mut ideal_maxrel: f64 = 0.0;
    let mut add_vals: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    let mut added_ok = true;
    let mut tf_ok = true;
    for run in [&school, &aa, &kar, &ctl] {
        for c in run.iter() {
            for w in &c.walks {
                let m = w.class_size;
                id_ok &= w.classes == (c.n - 1) / m;
                let mi = ideal(c.n, m);
                ideal_maxrel = ideal_maxrel.max((mi - w.ideal_rec).abs() / mi);
                if w.correct {
                    id_ok &= w.stored + w.fruitless + 1 == w.steps;
                    id_ok &= w.ops[4] == 2 * w.steps - w.escapes - 1 && w.ops[1] == 2 * (w.steps - w.escapes - 1);
                }
                id_ok &= w.fruitless == w.escapes && w.fruitless_len.iter().sum::<u64>() == w.fruitless;
                id_ok &= w.rep_ops[1] == 0 && w.rep_ops[4] == 0 && w.rep_ops[0] == w.ops[0] && w.rep_ops[3] == w.ops[3];
                if w.jumps != "E34" {
                    added_ok &= w.added[0] == 32 && w.added[2] == 64;
                    add_vals.entry(format!("{} karatsuba={}", w.jumps, w.kar)).or_default().push(w.added[1]);
                } else {
                    added_ok &= w.added == [0, 0, 0];
                }
                let cheap = w.arm != "Order6Built" && w.arm != "Order24Built";
                tf_ok &= cheap == (w.table_fp > 0);
            }
        }
    }
    let mut add_detail = Vec::new();
    for (k, v) in &add_vals {
        let mn = *v.iter().min().unwrap();
        let mx = *v.iter().max().unwrap();
        add_detail.push(format!("{k}: {} walks, added F_p multiplications {}..{} ({:.3} per addition)", v.len(), mn, mx, mn as f64 / 32.0));
    }
    check("instrument identities on every walk: classes = (n - 1)/m; stored + fruitless + 1 = steps (correct walks); fruitless = cycle escapes = sum of the length histogram; table = 2 steps - escapes - 1 and hash = 2 (steps - escapes - 1) (E34's counter identity); every cmp and neg is the representative's, which emits no hash or table operation", id_ok, String::new());
    check("ideal sqrt(pi (n - 1)/(2m)) recomputed here from n and m equals the recorded ideal_steps on every walk", ideal_maxrel < 1e-12, format!("max relative difference {ideal_maxrel:e}"));
    check("G2.221 F's added work: every O24F (and control) walk adds exactly 32 group additions and 64 mod-n operations, charged (added F_p multiplications are inside fp_muls); every E34-jump walk adds nothing", added_ok, add_detail.join("; "));
    check("set-up multiplications (cheap representative tables) charged on the cheap arms only", tf_ok, String::new());

    // ---- paired identities
    let mut pair_ok = true;
    for run in [&school, &kar] {
        for c in run.iter() {
            for (x, y) in [("O6B", "O6C"), ("O24", "O24L")] {
                for (u, v) in of(run, c.p, x).iter().zip(of(run, c.p, y)) {
                    pair_ok &= u.seed == v.seed && u.steps == v.steps && u.digest == v.digest && u.escapes == v.escapes && u.recovered == v.recovered;
                }
            }
        }
    }
    check("same walk on the paired arms: O6C equals O6B and O24L equals O24 per seed (steps, escapes, visit digest, logarithm), both arithmetics", pair_ok, String::new());
    let mut ka_ok = true;
    let mut fp_differ = 0u64;
    for c in &kar {
        for l in MAIN_LABELS {
            for (u, v) in of(&kar, c.p, l).iter().zip(of(&school, c.p, l)) {
                ka_ok &= u.seed == v.seed && u.steps == v.steps && u.digest == v.digest && u.ops == v.ops && u.rep_calls == v.rep_calls && u.rep_fp == v.rep_fp && u.group_ops == v.group_ops;
                if u.fp != v.fp {
                    fp_differ += 1;
                }
            }
        }
    }
    check("Karatsuba walks equal the schoolbook walks of the same seed (steps, visit digest, group operations, every native counter, representative calls and multiplications); only the field arithmetic's F_p multiplications differ", ka_ok, format!("1,280 pairs; fp_muls differ on {fp_differ}"));

    // ---- steps / ideal
    let mut claims = Claims { v: Vec::new(), cat: Vec::new(), all: true };
    let mut notes = Claims { v: Vec::new(), cat: Vec::new(), all: true };
    let soi = |run: &[Curve], l: &str| -> (f64, f64, f64, usize) {
        let mut v = Vec::new();
        let (mut ss, mut si) = (0.0, 0.0);
        for c in run {
            for w in of(run, c.p, l) {
                let i = ideal(c.n, w.class_size);
                v.push(w.steps as f64 / i);
                ss += w.steps as f64;
                si += i;
            }
        }
        (mean(&v), (svar(&v) / v.len() as f64).sqrt(), ss / si, v.len())
    };
    let mut steps_tab = Vec::new();
    let mut soi_map: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    for (l, run) in [("O6B", &school), ("O6C", &school), ("O24", &school), ("O24L", &school), ("O24F", &school), ("O24AA", &aa)] {
        let (m, se, ros, nw) = soi(run, l);
        soi_map.insert(l.into(), (m, se));
        let (mut fr, mut rd, mut stp) = (0u64, 0u64, 0u64);
        for c in run.iter() {
            for w in of(run, c.p, l) {
                fr += w.fruitless;
                rd += w.redirects;
                stp += w.steps;
            }
        }
        let mut per_curve = Vec::new();
        for c in run.iter() {
            let v: Vec<f64> = of(run, c.p, l).iter().map(|w| w.steps as f64 / ideal(c.n, w.class_size)).collect();
            per_curve.push(o(vec![("p", iu(c.p)), ("mean", n(mean(&v))), ("se", n((svar(&v) / v.len() as f64).sqrt()))]));
        }
        steps_tab.push(o(vec![
            ("arm", s(l)),
            ("walks", iu(nw as u64)),
            ("steps_over_ideal_mean", n(m)),
            ("standard_error", n(se)),
            ("ratio_of_sums", n(ros)),
            ("fruitless_per_step", n(fr as f64 / stp as f64)),
            ("redirects_per_step", n(rd as f64 / stp as f64)),
            ("per_curve", a(per_curve)),
        ]));
        let printed = match l {
            "O6B" => Some(("0.969", "0.016", "0.9694", "0.0159")),
            "O24" => Some(("1.006", "0.016", "1.0061", "0.0159")),
            "O24F" => Some(("0.999", "0.016", "0.9986", "0.0163")),
            "O24AA" => Some(("0.987", "0.016", "0.9868", "0.0158")),
            _ => None,
        };
        if let Some((pm, pse, tm, tse)) = printed {
            claims.add(&format!("steps / ideal {l} (mean over 1,024 walks)"), pm, m, "§9.53");
            claims.add(&format!("steps / ideal {l} standard error"), pse, se, "§9.53");
            notes.add(&format!("steps / ideal {l} (tables.md)"), tm, m, "tables.md");
            notes.add(&format!("steps / ideal {l} SE (tables.md)"), tse, se, "tables.md");
        }
        let pr = match l {
            "O6B" => Some(("3.61e-4", "0.03123")),
            "O24" => Some(("8.29e-5", "0.03124")),
            "O24F" => Some(("2.30e-5", "0.01563")),
            "O24AA" => Some(("8.47e-5", "0.03128")),
            _ => None,
        };
        if let Some((pf, pr)) = pr {
            notes.add(&format!("fruitless revisits / step {l} (tables.md)"), pf, fr as f64 / stp as f64, "tables.md");
            notes.add(&format!("redirects / step {l} (tables.md)"), pr, rd as f64 / stp as f64, "tables.md");
        }
    }
    // steps ratio pooled per sqrt(n)
    let steps_q = |w: &Walk| w.steps as f64;
    let sr_main = ratio(&school, "O6B", &school, "O24", true, &steps_q);
    let sr_aa = ratio(&school, "O6B", &aa, "O24AA", false, &steps_q);
    claims.add("steps ratio order 6 / order 24, pooled per sqrt(n), main stream", "1.927", sr_main.r, "§9.53");
    claims.add("steps ratio order 6 / order 24 against the A/A stream", "1.965", sr_aa.r, "§9.53");
    notes.add("steps ratio (tables.md)", "1.9270", sr_main.r, "tables.md");
    notes.add("steps ratio A/A (tables.md)", "1.9647", sr_aa.r, "tables.md");
    let f_saving_rel = (soi_map["O24"].0 - soi_map["O24F"].0) / soi_map["O24"].0;
    let f_saving_abs = soi_map["O24"].0 - soi_map["O24F"].0;
    let f_saving_se = (soi_map["O24"].1.powi(2) + soi_map["O24F"].1.powi(2)).sqrt();
    {
        let fr = |run: &[Curve], l: &str| { let (mut f, mut s2) = (0u64, 0u64); for c in run { for w in of(run, c.p, l) { f += w.fruitless; s2 += w.steps; } } f as f64 / s2 as f64 };
        notes.add("fruitless revisits per step, O24 over O24F (REPORT: 3.6x)", "3.6", fr(&school, "O24") / fr(&school, "O24F"), "REPORT.typ");
    }
    claims.add("F's step saving, % (O24 - O24F)/O24 steps/ideal", "0.75", 100.0 * f_saving_rel, "§9.53 / tables.md");
    let g219 = (0.98..=1.08).contains(&soi_map["O6B"].0) && (0.98..=1.08).contains(&soi_map["O24"].0) && (1.85..=2.10).contains(&sr_main.r);
    let g221 = (-0.02..=0.03).contains(&f_saving_rel) && added_ok;

    // ---- E34's records (16 seeds) and Stage 0
    let mut e34: Vec<Curve> = Vec::new();
    for p in PS {
        let (_, j) = load(&format!("{repo}/experiments/38_twist24_cheap_rep/stage_r_p{p}.json"));
        let c0 = &arr(&j)[0];
        let walks = arr(get(c0, "walks"))
            .iter()
            .map(|w| {
                let run = get(w, "run");
                let arm = st(get(w, "arm")).to_string();
                let label = match arm.as_str() {
                    "Order6Built" => "O6B",
                    "Order6Cheap" => "O6C",
                    "Order24Built" => "O24B",
                    "Order24Cheap" => "O24",
                    "Order24CheapMulLean" => "O24L",
                    _ => panic!(),
                };
                let kar = bo(get(w, "karatsuba"));
                Walk {
                    label: format!("{label}{}", if kar { "k" } else { "" }),
                    jumps: "E34".into(),
                    arm: arm.clone(),
                    kar,
                    fold: st(get(run, "fold")).to_string(),
                    seed: unum(get(run, "seed")),
                    steps: unum(get(run, "steps")),
                    escapes: unum(get(run, "cycle_escapes")),
                    fp: unum(get(run, "fp_muls")),
                    vfp: unum(get(run, "verify_fp_muls")),
                    vgo: unum(get(run, "verify_group_ops")),
                    ops: ops5(get(w, "ops")),
                    rep_calls: unum(get(w, "rep_calls")),
                    rep_fp: unum(get(w, "rep_fp_muls")),
                    rep_ops: ops5(get(w, "rep_ops")),
                    table_fp: unum(get(w, "table_fp_muls")),
                    correct: bo(get(run, "correct")),
                    recovered: ntext(get(run, "recovered")),
                    digest: ntext(get(run, "visit_digest")),
                    group_ops: unum(get(run, "group_ops")),
                    class_size: if arm.contains("24") { 24 } else { 6 },
                    classes: 0,
                    ideal_rec: 0.0,
                    stored: 0,
                    fruitless: 0,
                    fruitless_len: vec![],
                    fruitless_cycle_steps: 0,
                    redirects: 0,
                    jumps_n: 32,
                    added: [0; 3],
                    escape_rehits: 0,
                    tie_inputs: 0,
                }
            })
            .collect();
        e34.push(Curve { p, n: unum(get(c0, "n")), b0: Some(unum(get(c0, "b0"))), walks });
    }
    let (e6, e6se, e6ros, e6n) = soi(&e34, "O6C");
    let (e24, e24se, e24ros, e24n) = soi(&e34, "O24");
    let e34_sr = ratio(&e34, "O6C", &e34, "O24", true, &steps_q);
    claims.add("E34 records, order 6 cheap steps/ideal (128 walks)", "0.924", e6, "§9.52 / §9.53");
    claims.add("E34 records, order 6 SE", "0.046", e6se, "§9.52");
    claims.add("E34 records, order 24 cheap steps/ideal (128 walks)", "1.086", e24, "§9.52 / §9.53");
    claims.add("E34 records, order 24 SE", "0.045", e24se, "§9.52");
    claims.add("E34 steps ratio order 6 / order 24 pooled per sqrt(n)", "1.701", e34_sr.r, "§9.52 / tables.md (E34)");
    notes.add("E34 steps ratio (stage0_records.log)", "1.7009", e34_sr.r, "stage0_records.log");
    notes.add("E34 order-6 seeds under ideal, %", "7.6", 100.0 * (1.0 - e6), "§9.52");
    notes.add("E34 order-24 seeds over ideal, %", "8.6", 100.0 * (e24 - 1.0), "§9.52");
    notes.add("E34 order-6 deviation in SE ('each ~1.7 standard errors')", "1.7", (1.0 - e6) / e6se, "§9.52");
    notes.add("E34 order-24 deviation in SE ('each ~1.7 standard errors')", "1.7", (e24 - 1.0) / e24se, "§9.52");
    // Stage 0 run (fresh seeds 41000+k, pre-registration)
    let (_, s0j) = load(&format!("{ex}/stage0_run.json"));
    let mut s0tab = Vec::new();
    let mut s0soi: BTreeMap<String, f64> = BTreeMap::new();
    let mut s0_ratios: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for arm in ["Order6Cheap", "Order24Cheap"] {
        let m = if arm == "Order6Cheap" { 6 } else { 24 };
        let (mut v, mut fr, mut rd, mut stp, mut rh, mut lost, mut lost1) = (Vec::new(), 0u64, 0u64, 0u64, 0u64, Vec::new(), Vec::new());
        let mut sum_ideal = 0.0f64;
        let mut sum_sms = 0u64;
        let mut lens = vec![0u64; 9];
        let mut nw = 0;
        let mut seeds_ok = true;
        for c in arr(get(&s0j, "curves")) {
            let nn = unum(get(c, "n"));
            let mut k = 0;
            for w in arr(get(c, "walks")).iter().filter(|w| st(get(w, "arm")) == arm) {
                seeds_ok &= unum(get(w, "seed")) == 41000 + k;
                seeds_ok &= bo(get(w, "correct"));
                k += 1;
                nw += 1;
                let sf = get(w, "shortfall");
                let i = ideal(nn, m);
                let steps = unum(get(w, "steps"));
                v.push(steps as f64 / i);
                s0_ratios.entry(arm.to_string()).or_default().push(steps as f64 / i);
                fr += unum(get(sf, "fruitless"));
                rd += unum(get(sf, "redirects"));
                rh += unum(get(sf, "escape_rehits"));
                stp += steps;
                lost.push(unum(get(sf, "fruitless")) as f64 / i);
                lost1.push((steps - unum(get(sf, "stored"))) as f64 / i);
                sum_ideal += i;
                sum_sms += steps - unum(get(sf, "stored"));
                for (q, x) in arr(get(sf, "fruitless_len")).iter().enumerate() {
                    lens[q] += unum(x);
                }
            }
        }
        let m0 = mean(&v);
        let se0 = (svar(&v) / v.len() as f64).sqrt();
        s0soi.insert(arm.into(), m0);
        let other: u64 = lens.iter().enumerate().filter(|(q, _)| *q != 1 && *q != 3).map(|x| x.1).sum();
        s0tab.push(o(vec![
            ("arm", s(arm)),
            ("walks", iu(nw)),
            ("seeds_41000_plus_k_all_correct", b(seeds_ok)),
            ("steps_over_ideal_mean", n(m0)),
            ("standard_error", n(se0)),
            ("fruitless_per_step", n(fr as f64 / stp as f64)),
            ("two_cycles", iu(lens[1])),
            ("four_cycles", iu(lens[3])),
            ("other_lengths", iu(other)),
            ("redirects_per_step", n(rd as f64 / stp as f64)),
            ("escape_rehits", iu(rh)),
            ("fruitless_revisits_over_ideal_mean", n(mean(&lost))),
            ("steps_minus_stored_over_ideal_mean", n(mean(&lost1))),
            ("fruitless_revisits_over_ideal_ratio_of_sums", n(fr as f64 / sum_ideal)),
            ("steps_minus_stored_over_ideal_ratio_of_sums", n(sum_sms as f64 / sum_ideal)),
        ]));
        let (pm, pf, pr, pl, p2, p4, po) = if m == 6 {
            ("1.027", "3.57e-4", "0.03124", "3.7e-4", "44784", "3993", "109")
        } else {
            ("1.038", "0.84e-4", "0.03123", "0.95e-4", "5733", "116", "3")
        };
        claims.add(&format!("Stage 0 {arm} steps/ideal (512 fresh walks)"), pm, m0, "§9.52");
        claims.add(&format!("Stage 0 {arm} SE"), "0.023", se0, "§9.52");
        notes.add(&format!("Stage 0 {arm} fruitless revisits / step"), pf, fr as f64 / stp as f64, "§9.52");
        notes.add(&format!("Stage 0 {arm} redirects / step"), pr, rd as f64 / stp as f64, "§9.52");
        notes.add(&format!("Stage 0 {arm} steps lost to fruitless cycles, of the ideal, as printed: sum of (steps - stored) / sum of ideals, i.e. fruitless revisits plus the final collision step"), pl, sum_sms as f64 / sum_ideal, "§9.52 table (stage0_run.json pooled excess_from_fruitless)");
        notes.add_count(&format!("Stage 0 {arm} 2-cycles"), p2, &lens[1].to_string(), "§9.52");
        notes.add_count(&format!("Stage 0 {arm} 4-cycles"), p4, &lens[3].to_string(), "§9.52");
        notes.add_count(&format!("Stage 0 {arm} other lengths"), po, &other.to_string(), "§9.52");
        notes.add_count(&format!("Stage 0 {arm} escape re-hits ('1 in 512 walks per order')"), "1", &rh.to_string(), "§9.52");
        notes.add(&format!("Stage 0 {arm} step excess over ideal, %"), if m == 6 { "2.7" } else { "3.8" }, 100.0 * (m0 - 1.0), "§9.52");
    }
    {
        let o24 = &s0tab[1];
        let two = unum(get(o24, "two_cycles")) as f64;
        let tot = two + unum(get(o24, "four_cycles")) as f64 + unum(get(o24, "other_lengths")) as f64;
        notes.add("Stage 0 order-24 fruitless revisits that are 2-cycles, % (REPORT: 98 %)", "98", 100.0 * two / tot, "REPORT.typ");
    }
    notes.add("Stage 0 implied steps ratio 2 x O6/O24 ('~1.98')", "1.98", 2.0 * s0soi["Order6Cheap"] / s0soi["Order24Cheap"], "§9.52");
    notes.add("r-adding factor 1/sqrt(1 - 1/r), r = 32", "1.016", 1.0 / (1.0f64 - 1.0 / 32.0).sqrt(), "§9.52");
    notes.add("r-adding excess at r = 64, %", "0.8", 100.0 * (1.0 / (1.0f64 - 1.0 / 64.0).sqrt() - 1.0), "§9.52");
    notes.add("O6B under its ideal on the main stream, % ('3 %')", "3", 100.0 * (1.0 - soi_map["O6B"].0), "§9.53");

    // ---- the band
    let aa_r: Vec<Ratio> = WS.iter().map(|&w| ratio(&school, "O24", &aa, "O24AA", false, &move |x: &Walk| x.cost(w))).collect();
    let mut band = Vec::new();
    let mut resolved_map: BTreeMap<String, (bool, [f64; 3])> = BTreeMap::new();
    // (pair, arithmetic, printed ratios, printed CIs, printed resolved, printed per-curve)
    let printed: Vec<(&str, &str, &str, [&str; 3], [[&str; 2]; 3], bool, &str)> = vec![
        ("O6B", "O24", "school", ["1.761", "1.418", "1.275"], [["1.686", "1.836"], ["1.356", "1.481"], ["1.218", "1.332"]], true, "8/0/0"),
        ("O6C", "O24", "school", ["1.696", "1.320", "1.162"], [["1.623", "1.768"], ["1.262", "1.378"], ["1.110", "1.214"]], true, "7/0/1"),
        ("O6B", "O24L", "school", ["1.838", "1.266", "1.073"], [["1.760", "1.917"], ["1.211", "1.322"], ["1.025", "1.121"]], true, "6/0/2"),
        ("O6C", "O24L", "school", ["1.770", "1.178", "0.978"], [["1.695", "1.846"], ["1.126", "1.230"], ["0.935", "1.022"]], false, "5/0/3"),
        ("O24", "O24F", "school", ["1.021", "1.019", "1.018"], [["0.978", "1.064"], ["0.975", "1.064"], ["0.973", "1.064"]], false, "4/4/0"),
        ("O6B", "O24F", "school", ["1.798", "1.446", "1.298"], [["1.720", "1.877"], ["1.381", "1.510"], ["1.240", "1.357"]], true, "8/0/0"),
        ("O6C", "O24F", "school", ["1.732", "1.345", "1.183"], [["1.657", "1.807"], ["1.285", "1.405"], ["1.130", "1.237"]], true, "8/0/0"),
        ("O24L", "O24", "school", ["0.958", "1.120", "1.188"], [["0.918", "0.998"], ["1.072", "1.168"], ["1.136", "1.240"]], false, "0/0/8"),
        ("O24", "O24AA", "school", ["1.019", "1.019", "1.020"], [["0.976", "1.061"], ["0.975", "1.063"], ["0.975", "1.064"]], false, "0/0/8"),
        ("O6B", "O24", "kar", ["1.711", "1.369", "1.249"], [["1.564", "1.858"], ["1.248", "1.490"], ["1.137", "1.360"]], true, "8/0/0"),
        ("O6C", "O24", "kar", ["1.635", "1.267", "1.137"], [["1.495", "1.775"], ["1.155", "1.379"], ["1.035", "1.238"]], true, "8/0/0"),
        ("O6B", "O24L", "kar", ["1.802", "1.215", "1.050"], [["1.647", "1.956"], ["1.108", "1.323"], ["0.956", "1.144"]], false, "5/0/3"),
        ("O6C", "O24L", "kar", ["1.721", "1.124", "0.956"], [["1.574", "1.869"], ["1.025", "1.224"], ["0.870", "1.041"]], false, "3/0/5"),
        ("O24", "O24F", "kar", ["0.984", "0.981", "0.979"], [["0.898", "1.070"], ["0.892", "1.070"], ["0.890", "1.069"]], false, "3/5/0"),
        ("O6B", "O24F", "kar", ["1.684", "1.343", "1.223"], [["1.538", "1.831"], ["1.223", "1.463"], ["1.113", "1.333"]], true, "8/0/0"),
        ("O6C", "O24F", "kar", ["1.609", "1.242", "1.113"], [["1.470", "1.749"], ["1.131", "1.353"], ["1.013", "1.214"]], true, "7/0/1"),
        ("O24L", "O24", "kar", ["0.950", "1.127", "1.189"], [["0.868", "1.032"], ["1.026", "1.228"], ["1.081", "1.298"]], false, "0/0/8"),
    ];
    for (la, lb, ar, pr, pci, pres, ppc) in &printed {
        let (ra, rb): (&[Curve], &[Curve]) = match (*ar, *lb) {
            ("school", "O24AA") => (&school, &aa),
            ("school", _) => (&school, &school),
            _ => (&kar, &kar),
        };
        let paired = *lb != "O24AA";
        let rs: Vec<Ratio> = WS.iter().map(|&w| ratio(ra, la, rb, lb, paired, &move |x: &Walk| x.cost(w))).collect();
        let mut resolved = true;
        let mut same_side = true;
        let mut resolved_paired = true;
        for (i, r) in rs.iter().enumerate() {
            let (lo, hi) = ci(r);
            let floor = (aa_r[i].r - 1.0).abs();
            resolved &= (lo > 1.0 || hi < 1.0) && (r.r - 1.0).abs() > floor;
            same_side &= (r.r > 1.0) == (rs[0].r > 1.0);
            if let Some(sp) = r.se_paired {
                resolved_paired &= (r.r - 1.96 * sp > 1.0 || r.r + 1.96 * sp < 1.0) && (r.r - 1.0).abs() > floor;
            }
        }
        // per curve: ratio of means at each weight
        let mut pcs = Vec::new();
        let (mut second, mut first, mut mixed) = (0, 0, 0);
        let mut per_curve_ci_resolved = 0;
        let mut ci_unresolved_ps: Vec<J> = Vec::new();
        for c in ra {
            let mut rr = [0.0; 3];
            let mut cires = true;
            for (i, &w) in WS.iter().enumerate() {
                let xa: Vec<f64> = of(ra, c.p, la).iter().map(|x| x.cost(w)).collect();
                let xb: Vec<f64> = of(rb, c.p, lb).iter().map(|x| x.cost(w)).collect();
                rr[i] = mean(&xa) / mean(&xb);
                let rel = (svar(&xa) / xa.len() as f64) / mean(&xa).powi(2) + (svar(&xb) / xb.len() as f64) / mean(&xb).powi(2);
                let se = rr[i] * rel.sqrt();
                cires &= rr[i] - 1.96 * se > 1.0 || rr[i] + 1.96 * se < 1.0;
            }
            if rr.iter().all(|x| *x > 1.0) {
                second += 1;
            } else if rr.iter().all(|x| *x < 1.0) {
                first += 1;
            } else {
                mixed += 1;
            }
            if cires {
                per_curve_ci_resolved += 1;
            } else {
                ci_unresolved_ps.push(iu(c.p));
            }
            pcs.push(o(vec![("p", iu(c.p)), ("ratio_0_0p1_1", a(rr.iter().map(|x| n(*x)).collect()))]));
        }
        let pc = format!("{second}/{first}/{mixed}");
        let name = format!("{la} / {lb} ({ar})");
        resolved_map.insert(name.clone(), (resolved, [rs[0].r, rs[1].r, rs[2].r]));
        let headline = (*la == "O6B" || *la == "O6C" || *la == "O24") && *lb == "O24" || (*la == "O24" && *lb == "O24F" && *ar == "school");
        for i in 0..3 {
            let tgt = if headline { &mut claims } else { &mut notes };
            tgt.add(&format!("{name} pooled ratio at {} GAE", WS[i]), pr[i], rs[i].r, "§9.53 / tables.md");
            if *ar == "school" && headline {
                let (lo, hi) = ci(&rs[i]);
                claims.add(&format!("{name} 95% interval low at {} GAE", WS[i]), pci[i][0], lo, "§9.53 / tables.md");
                claims.add(&format!("{name} 95% interval high at {} GAE", WS[i]), pci[i][1], hi, "§9.53 / tables.md");
            } else {
                let (lo, hi) = ci(&rs[i]);
                notes.add(&format!("{name} 95% interval low at {} GAE", WS[i]), pci[i][0], lo, "tables.md");
                notes.add(&format!("{name} 95% interval high at {} GAE", WS[i]), pci[i][1], hi, "tables.md");
            }
        }
        let tgt = if headline && *ar == "school" { &mut claims } else { &mut notes };
        let ranked = resolved && same_side;
        tgt.add_count(&format!("{name} resolved (tables.md column; reproduced as: registered rule and the same side of 1 at all three weights)"), if *pres { "true" } else { "false" }, if ranked { "true" } else { "false" }, "tables.md");
        tgt.add_count(&format!("{name} per curve: second arm cheaper at every weight / first / order changes"), ppc, &pc, "tables.md");
        band.push(o(vec![
            ("pair", s(&name)),
            ("ratio_0_0p1_1", a(rs.iter().map(|r| n(r.r)).collect())),
            ("se_independent_0_0p1_1", a(rs.iter().map(|r| n(r.se)).collect())),
            ("ci95_0_0p1_1", a(rs.iter().map(|r| { let (lo, hi) = ci(r); a(vec![n(lo), n(hi)]) }).collect())),
            ("se_paired_by_seed_0_0p1_1", a(rs.iter().map(|r| r.se_paired.map(n).unwrap_or(J::Null)).collect())),
            ("resolved_registered_rule", b(resolved)),
            ("same_side_of_1_at_all_weights", b(same_side)),
            ("ranked_across_band", b(resolved && same_side)),
            ("resolved_with_paired_se", if paired { b(resolved_paired) } else { J::Null }),
            ("per_curve_second_first_mixed", s(&pc)),
            ("per_curve_resolved_by_own_95pct_interval_at_every_weight", iu(per_curve_ci_resolved)),
            ("per_curve_not_resolved_by_own_interval", a(ci_unresolved_ps)),
            ("per_curve", a(pcs)),
        ]));
    }
    // Karatsuba priced at its own GAE (an addition costs 79 F_p multiplications
    // under Karatsuba: F's 32 added additions cost 2,528), a sensitivity only.
    let mut kar79 = Vec::new();
    for (la, lb) in [("O6B", "O24"), ("O6C", "O24"), ("O24", "O24F")] {
        let rs: Vec<f64> = WS
            .iter()
            .map(|&w| ratio(&kar, la, &kar, lb, true, &move |x: &Walk| (x.fp - x.vfp) as f64 + w * 79.0 * x.native() as f64).r)
            .collect();
        kar79.push(o(vec![("pair", s(&format!("{la} / {lb} (Karatsuba, GAE = 79)"))), ("ratio_0_0p1_1", a(rs.iter().map(|x| n(*x)).collect()))]));
    }
    // A/A
    for (i, pa) in ["1.0185", "1.0192", "1.0195"].iter().enumerate() {
        notes.add(&format!("A/A O24/O24AA at {} GAE (tables.log)", WS[i]), pa, aa_r[i].r, "tables.log");
    }
    let g220 = aa_r.iter().all(|r| (0.94..=1.06).contains(&r.r));
    let (o6b24, o6c24) = (resolved_map["O6B / O24 (school)"].clone(), resolved_map["O6C / O24 (school)"].clone());
    let (o24f, k6b, k6c) = (resolved_map["O24 / O24F (school)"].clone(), resolved_map["O6B / O24 (kar)"].clone(), resolved_map["O6C / O24 (kar)"].clone());
    let inb = |x: f64, lo: f64, hi: f64| x >= lo && x <= hi;
    let g223 = o6b24.0 && inb(o6b24.1[0], 1.65, 1.95) && inb(o6b24.1[1], 1.32, 1.56) && inb(o6b24.1[2], 1.18, 1.40);
    let g224 = o6c24.0 && inb(o6c24.1[0], 1.58, 1.88) && inb(o6c24.1[1], 1.23, 1.45) && inb(o6c24.1[2], 1.08, 1.28);
    let g222 = !o24f.0;
    let g226 = k6b.1.iter().all(|x| *x > 1.0) && k6c.1.iter().all(|x| *x > 1.0);
    // per-curve O6C/O24 for G2.225
    let mut g225n = 0;
    for c in &school {
        let ok = WS.iter().all(|&w| {
            let xa: Vec<f64> = of(&school, c.p, "O6C").iter().map(|x| x.cost(w)).collect();
            let xb: Vec<f64> = of(&school, c.p, "O24").iter().map(|x| x.cost(w)).collect();
            mean(&xa) > mean(&xb)
        });
        if ok {
            g225n += 1;
        }
    }
    let g225 = g225n >= 7;

    // ---- strongest walks, E18 factors, handover
    let pooled_cost = |run: &[Curve], l: &str, w: f64| pool(run, l, &move |x: &Walk| x.cost(w)).0;
    let mut strongest = Vec::new();
    let mut e18 = Vec::new();
    for (i, &w) in WS.iter().enumerate() {
        let mut o6: Vec<(f64, &str)> = ["O6B", "O6C"].iter().map(|l| (pooled_cost(&school, l, w), *l)).collect();
        let mut o24: Vec<(f64, &str)> = ["O24", "O24L", "O24F"].iter().map(|l| (pooled_cost(&school, l, w), *l)).collect();
        o6.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        o24.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        let ob = pooled_cost(&school, "O6B", w);
        let f = o6b24.1[i];
        strongest.push(o(vec![
            ("weight_gae", n(w)),
            ("strongest_order6", s(o6[0].1)),
            ("strongest_order24", s(o24[0].1)),
            ("o6b_over_strongest", n(ob / o24[0].0)),
            ("handover_shift_bits_4log2_strongest", n(4.0 * (ob / o24[0].0).log2())),
            ("pooled_cost_per_sqrt_n", o(vec![
                ("O6B", n(ob)),
                ("O6C", n(pooled_cost(&school, "O6C", w))),
                ("O24", n(pooled_cost(&school, "O24", w))),
                ("O24L", n(pooled_cost(&school, "O24L", w))),
                ("O24F", n(pooled_cost(&school, "O24F", w))),
            ])),
        ]));
        let ps = ["1.838", "1.446", "1.298"];
        let psn = [("O6C", "O24L"), ("O6C", "O24F"), ("O6C", "O24F")];
        notes.add(&format!("O6B / strongest order-24 arm at {w} GAE"), ps[i], ob / o24[0].0, "§9.53 / tables.md");
        notes.add_count(&format!("strongest order 6 / order 24 at {w} GAE"), &format!("{} / {}", psn[i].0, psn[i].1), &format!("{} / {}", o6[0].1, o24[0].1), "tables.md");
        let shift = 4.0 * f.log2();
        let pshift = ["3.27", "2.02", "1.40"][i];
        claims.add(&format!("E18 psi-row distance factor (O6B / O24, schoolbook) at {w} GAE"), ["1.761", "1.418", "1.275"][i], f, "task / §9.53");
        claims.add(&format!("E18 handover shift 4 log2(O6B/O24), bits, at {w} GAE"), pshift, shift, "§9.53 / tables.md");
        e18.push(o(vec![
            ("weight_gae", n(w)),
            ("factor_o6b_over_o24_schoolbook", n(f)),
            ("factor_o6b_over_o24_karatsuba", n(k6b.1[i])),
            ("handover_shift_bits_4log2", n(shift)),
        ]));
    }
    // E34's band (O6B / cheap 24, O6C / cheap 24) from E34's records, same cost rule
    let mut e34band = Vec::new();
    for (la, pr) in [("O6B", ["1.563", "1.254", "1.126"]), ("O6C", ["1.505", "1.167", "1.026"])] {
        let rs: Vec<f64> = WS.iter().map(|&w| ratio(&e34, la, &e34, "O24", true, &move |x: &Walk| x.cost(w)).r).collect();
        for i in 0..3 {
            notes.add(&format!("E34 (16 walks) {la} / O24 at {} GAE", WS[i]), pr[i], rs[i], "§9.53 (E34)");
        }
        let sh: Vec<f64> = rs.iter().map(|x| 4.0 * x.log2()).collect();
        if la == "O6B" {
            for (i, p) in ["2.58", "1.31", "0.68"].iter().enumerate() {
                notes.add(&format!("E34 handover shift at {} GAE", WS[i]), p, sh[i], "§9.53 (E34)");
            }
        }
        e34band.push(o(vec![("pair", s(&format!("{la} / O24 (E34 records, schoolbook)"))), ("ratio_0_0p1_1", a(rs.iter().map(|x| n(*x)).collect()))]));
    }

    // ---- every walk of each order pooled (E34 records, Stage 0, main, A/A), and per-curve heterogeneity
    let per_walk = |run: &[Curve], l: &str| -> Vec<f64> { let mut v = Vec::new(); for c in run { for w in of(run, c.p, l) { v.push(w.steps as f64 / ideal(c.n, w.class_size)); } } v };
    let mut all6 = per_walk(&e34, "O6C");
    all6.extend(s0_ratios["Order6Cheap"].iter().copied());
    all6.extend(per_walk(&school, "O6B"));
    let mut all24 = per_walk(&e34, "O24");
    all24.extend(s0_ratios["Order24Cheap"].iter().copied());
    all24.extend(per_walk(&school, "O24"));
    all24.extend(per_walk(&aa, "O24AA"));
    let (m6, s6) = (mean(&all6), (svar(&all6) / all6.len() as f64).sqrt());
    let (m24, s24) = (mean(&all24), (svar(&all24) / all24.len() as f64).sqrt());
    let comb_ratio = 2.0 * m6 / m24;
    let comb_se = comb_ratio * ((s6 / m6).powi(2) + (s24 / m24).powi(2)).sqrt();
    let chi = |run: &[Curve], l: &str| -> f64 { let mut x2 = 0.0; for c in run { let v: Vec<f64> = of(run, c.p, l).iter().map(|w| w.steps as f64 / ideal(c.n, w.class_size)).collect(); let z = (mean(&v) - 1.0) / (svar(&v) / v.len() as f64).sqrt(); x2 += z * z; } x2 };
    let pooled_all = o(vec![
        ("rule", s("every walk of each order across E34's records (16 per curve), Stage 0 (64), the main stream (128) and, for order 24, the A/A stream (128): mean steps/ideal over walks; ratio = 2 x order 6 / order 24; heterogeneity = sum over curves of ((curve mean - 1)/SE)^2 against chi-square with 8 degrees of freedom (95 % point 15.51)")),
        ("order6_walks", iu(all6.len() as u64)), ("order6_steps_over_ideal", n(m6)), ("order6_se", n(s6)),
        ("order24_walks", iu(all24.len() as u64)), ("order24_steps_over_ideal", n(m24)), ("order24_se", n(s24)),
        ("steps_ratio_2x", n(comb_ratio)), ("steps_ratio_se", n(comb_se)), ("z_from_2", n((2.0 - comb_ratio) / comb_se)),
        ("chi2_8df_main_O6B", n(chi(&school, "O6B"))), ("chi2_8df_main_O24", n(chi(&school, "O24"))), ("chi2_8df_aa_O24AA", n(chi(&aa, "O24AA"))), ("chi2_8df_main_O24F", n(chi(&school, "O24F"))),
    ]);

    // ---- null control: does the shortfall decay with sample size?
    let dev = |r: &Ratio| (2.0 - r.r, r.se);
    let (d34, s34) = dev(&e34_sr);
    let (dm, sm) = dev(&sr_main);
    let (da, sa) = dev(&sr_aa);
    let z_e34_vs_main = (sr_main.r - e34_sr.r) / (e34_sr.se.powi(2) + sr_main.se.powi(2)).sqrt();
    let decay = o(vec![
        ("rule", s("the shortfall 2 - (steps ratio order 6 / order 24, pooled per sqrt(n)) is the measured quantity; if it is sampling noise it must shrink like 1/sqrt(walks) as the seed count grows from 16 to 128 per curve and order, and fresh streams must not reproduce it")),
        ("e34_16_walks", o(vec![("ratio", n(e34_sr.r)), ("se_independent", n(e34_sr.se)), ("se_paired", n(e34_sr.se_paired.unwrap())), ("shortfall", n(d34)), ("z_from_2", n(d34 / s34))])),
        ("e41_main_128_walks", o(vec![("ratio", n(sr_main.r)), ("se_independent", n(sr_main.se)), ("se_paired", n(sr_main.se_paired.unwrap())), ("shortfall", n(dm)), ("z_from_2", n(dm / sm))])),
        ("e41_aa_stream_128_walks", o(vec![("ratio", n(sr_aa.r)), ("se_independent", n(sa)), ("shortfall", n(da)), ("z_from_2", n(da / sa))])),
        ("expected_se_scaling_16_to_128", n(1.0 / 8f64.sqrt())),
        ("steps_ratio_by_ratio_of_sums_e34_main_aa", a(vec![n(4.0 * e6ros / e24ros / 2.0), n(2.0 * soi(&school, "O6B").2 / soi(&school, "O24").2), n(2.0 * soi(&school, "O6B").2 / soi(&aa, "O24AA").2)])),
        ("f_steps_ratio_of_sums_o24_o24f", a(vec![n(soi(&school, "O24").2), n(soi(&school, "O24F").2)])),
        ("z_e34_vs_e41_main", n(z_e34_vs_main)),
    ]);

    // ---- IC rows: tables.md vs E27's summary
    let (_, e27) = load(&format!("{repo}/experiments/32_walk_karatsuba/summary.json"));
    let tm = fs::read_to_string(format!("{ex}/tables.md")).unwrap();
    let mut ic_rows = 0;
    let mut ic_ok = true;
    let mut ic_detail = Vec::new();
    for line in tm.lines() {
        let cells: Vec<&str> = line.split('|').map(|x| x.trim()).collect();
        if cells.len() < 7 || !cells[1].chars().all(|c| c.is_ascii_digit()) || cells[1].is_empty() || cells[3].len() != 1 {
            continue;
        }
        let (p, seed, arm) = (cells[1].parse::<u64>().unwrap(), cells[2].parse::<u64>().unwrap(), cells[3]);
        let before: Vec<&str> = cells[4].split(',').map(|x| x.trim()).collect();
        let after: Vec<&str> = cells[5].split(',').map(|x| x.trim()).collect();
        let row = arr(get(&e27, "rows")).iter().find(|r| unum(get(r, "p")) == p && unum(get(r, "seed")) == seed && st(get(r, "arm")) == arm).expect("E27 row");
        let dn: Vec<f64> = arr(get(row, "dist_new")).iter().map(fnum).collect();
        for i in 0..3 {
            ic_ok &= agrees(before[i], dn[i]) && agrees(after[i], dn[i]) && before[i] == after[i];
        }
        ic_ok &= cells[6] == "1";
        ic_rows += 1;
        if ic_rows <= 2 || (p == 397 && seed == 1) {
            ic_detail.push(format!("p={p} s{seed} {arm}: E27 dist_new {:.4e}, {:.4e}, {:.4e}; printed {}", dn[0], dn[1], dn[2], cells[4]));
        }
    }
    check("G2.228 IC rows: tables.md lists 20 GLS rows (E31 solver A/B with E27 products); 'before' and 'after' columns are equal and each equals E27's dist_new (experiments/32_walk_karatsuba/summary.json, E27's evidence, unchanged since b2a710d504) to the printed digits; factor 1", ic_ok && ic_rows == 20, format!("{ic_rows} rows; {}", ic_detail.join("; ")));

    // ---- identity record (G2.216) as committed
    let (_, idj) = load(&format!("{ex}/identity.json"));
    let id_ok = unum(get(&idj, "compared")) == 1280 && unum(get(&idj, "byte_identical")) == 1280 && unum(get(&idj, "value_identical")) == 1280 && arr(get(&idj, "differences")).is_empty() && st(get(&idj, "source")) == "experiments/38_twist24_cheap_rep";
    check("G2.216 identity record as committed: compared 1280, byte-identical 1280, value-identical 1280, no differences, source E34 Stage R (the replay re-establishes it independently)", id_ok, compact(&idj));

    // ---- registered verdicts recomputed
    let verdicts = vec![
        ("G2.217", wrong == 0 && total == 7424, "holds"),
        ("G2.218", cflag >= 31, "holds"),
        ("G2.219", g219, "falsified"),
        ("G2.220", g220, "holds"),
        ("G2.221", g221, "holds"),
        ("G2.222", g222, "holds"),
        ("G2.223", g223, "holds"),
        ("G2.224", g224, "holds"),
        ("G2.225", g225, "holds"),
        ("G2.226", g226, "holds"),
    ];
    let mut vout = Vec::new();
    let mut vok = true;
    for (id, holds, printed) in verdicts {
        let rec = if holds { "holds" } else { "falsified" };
        vok &= rec == printed;
        vout.push(o(vec![("id", s(id)), ("printed", s(printed)), ("recomputed", s(rec)), ("agrees", b(rec == printed))]));
    }
    claims.add_count("G2.225 curves where O6C/O24 favours order 24 at every weight", "7", &g225n.to_string(), "§9.53");
    check("registered verdicts G2.217-G2.226 recomputed from the raw records agree with §9.53", vok, String::new());
    notes.add("F step saving in percentage points of the ideal", "0.75", 100.0 * f_saving_abs, "tables.md (\"F lower by 0.75 %\")");

    let out = o(vec![
        ("schema", s("e41-validator-recompute/v1")),
        ("cost_definition", s("walk cost(w) = (fp_muls - verify_fp_muls) + w * 97 * (cmp + neg + modn + hash + table) from walk.ops, w = 0, 0.1, 1; pooled = sum over the eight curves of mean cost / sqrt(n); ratio = first arm / second arm; standard errors by the delta method over the per-curve means (independent arms; the seed-paired covariance version is given alongside); resolved = 95% interval excludes 1 at all three weights and |ratio - 1| exceeds |O24/O24AA - 1| at each weight (registered rule, §9.52)")),
        ("checks_all_hold", b(all_checks)),
        ("headline_all_agree", b(claims.all)),
        ("note_figures_all_agree", b(notes.all)),
        ("checks", a(checks)),
        ("headline", a(claims.v)),
        ("headline_categorical", a(claims.cat)),
        ("note_figures", a(notes.v.into_iter().chain(notes.cat).collect())),
        ("verdicts_recomputed", a(vout)),
        ("steps_over_ideal", a(steps_tab)),
        ("f_step_saving", o(vec![("relative_percent", n(100.0 * f_saving_rel)), ("percentage_points", n(100.0 * f_saving_abs)), ("se_points", n(100.0 * f_saving_se)), ("z", n(f_saving_abs / f_saving_se))])),
        ("stage0_run_recomputed", a(s0tab)),
        ("e34_records", o(vec![("o6_cheap_steps_over_ideal", n(e6)), ("o6_se", n(e6se)), ("o6_walks", iu(e6n as u64)), ("o24_cheap_steps_over_ideal", n(e24)), ("o24_se", n(e24se)), ("o24_walks", iu(e24n as u64)), ("steps_ratio", n(e34_sr.r))])),
        ("shortfall_decay_null_control", decay),
        ("all_walks_pooled", pooled_all),
        ("aa_floor_0_0p1_1", a(aa_r.iter().map(|r| n(r.r)).collect())),
        ("band", a(band)),
        ("karatsuba_priced_at_gae79", a(kar79)),
        ("e34_band_recomputed", a(e34band)),
        ("strongest_walks", a(strongest)),
        ("e18_psi_rows", a(e18)),
        ("added_fp_muls", s(&add_detail.join("; "))),
    ]);
    let mut t = String::new();
    pretty(&out, 0, &mut t);
    println!("{t}");
}
