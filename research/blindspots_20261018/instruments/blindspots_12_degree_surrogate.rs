//! Blind-spot item 12: a surrogate for the solving-degree surface.
//!
//! std only. Subcommands (see research/blindspots_20261018/12-degree-surrogate/PROTOCOL.md):
//!   selftest                       known answers and controls on synthetic data
//!   harvest   --manifest M --raw D verify sha256 of every source, write D/data.tsv
//!   evaluate  --manifest M --raw D leave-one-family-out arms (a) (b) (c1) (c2) (e)
//!   planted   --manifest M --raw D positive control on the real covariates
//!   nullperm  --manifest M --raw D label-permutation negative control
//!   t3        --manifest M --raw D measured F4 matrix size (descriptive)
//!   rank      --manifest M --raw D next five cells to buy
//!   verdict   --raw D              frozen rule applied to the written key=value files
//! The host runs one thread; everything is deterministic (fixed seeds).

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;

// ----------------------------------------------------------------------------
// SHA-256
// ----------------------------------------------------------------------------

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
            let ch = (v[4] & v[5]) ^ ((!v[4]) & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
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
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    let mut s = String::new();
    for x in h.iter() {
        let _ = write!(s, "{:08x}", x);
    }
    s
}

// ----------------------------------------------------------------------------
// Minimal JSON
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum J {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

impl J {
    fn get(&self, k: &str) -> Option<&J> {
        if let J::Obj(v) = self {
            for (kk, vv) in v {
                if kk == k {
                    return Some(vv);
                }
            }
        }
        None
    }
    fn f(&self) -> Option<f64> {
        if let J::Num(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    fn s(&self) -> Option<&str> {
        if let J::Str(x) = self {
            Some(x)
        } else {
            None
        }
    }
    fn b(&self) -> Option<bool> {
        if let J::Bool(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    fn arr(&self) -> Option<&Vec<J>> {
        if let J::Arr(x) = self {
            Some(x)
        } else {
            None
        }
    }
    fn is_null(&self) -> bool {
        matches!(self, J::Null)
    }
    /// A value for dedup keys.
    fn show(&self) -> String {
        match self {
            J::Null => "null".into(),
            J::Bool(b) => b.to_string(),
            J::Num(x) => format!("{}", x),
            J::Str(s) => s.clone(),
            J::Arr(v) => v.iter().map(|x| x.show()).collect::<Vec<_>>().join(","),
            J::Obj(_) => "{}".into(),
        }
    }
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && (self.b[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn value(&mut self) -> Result<J, String> {
        self.ws();
        if self.i >= self.b.len() {
            return Err("eof".into());
        }
        match self.b[self.i] {
            b'{' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.i < self.b.len() && self.b[self.i] == b'}' {
                    self.i += 1;
                    return Ok(J::Obj(v));
                }
                loop {
                    self.ws();
                    let k = match self.value()? {
                        J::Str(s) => s,
                        _ => return Err("key".into()),
                    };
                    self.ws();
                    if self.i >= self.b.len() || self.b[self.i] != b':' {
                        return Err("colon".into());
                    }
                    self.i += 1;
                    let val = self.value()?;
                    v.push((k, val));
                    self.ws();
                    if self.i >= self.b.len() {
                        return Err("eof in obj".into());
                    }
                    match self.b[self.i] {
                        b',' => self.i += 1,
                        b'}' => {
                            self.i += 1;
                            return Ok(J::Obj(v));
                        }
                        _ => return Err("obj sep".into()),
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.i < self.b.len() && self.b[self.i] == b']' {
                    self.i += 1;
                    return Ok(J::Arr(v));
                }
                loop {
                    v.push(self.value()?);
                    self.ws();
                    if self.i >= self.b.len() {
                        return Err("eof in arr".into());
                    }
                    match self.b[self.i] {
                        b',' => self.i += 1,
                        b']' => {
                            self.i += 1;
                            return Ok(J::Arr(v));
                        }
                        _ => return Err("arr sep".into()),
                    }
                }
            }
            b'"' => {
                self.i += 1;
                let mut out: Vec<u8> = Vec::new();
                loop {
                    if self.i >= self.b.len() {
                        return Err("eof in str".into());
                    }
                    let c = self.b[self.i];
                    self.i += 1;
                    match c {
                        b'"' => break,
                        b'\\' => {
                            if self.i >= self.b.len() {
                                return Err("esc".into());
                            }
                            let e = self.b[self.i];
                            self.i += 1;
                            match e {
                                b'n' => out.push(b'\n'),
                                b't' => out.push(b'\t'),
                                b'r' => out.push(b'\r'),
                                b'b' => out.push(8),
                                b'f' => out.push(12),
                                b'u' => {
                                    if self.i + 4 > self.b.len() {
                                        return Err("u esc".into());
                                    }
                                    let hex = std::str::from_utf8(&self.b[self.i..self.i + 4]).map_err(|e| e.to_string())?;
                                    let cp = u32::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
                                    self.i += 4;
                                    let ch = char::from_u32(cp).unwrap_or('?');
                                    let mut buf = [0u8; 4];
                                    out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                                }
                                other => out.push(other),
                            }
                        }
                        other => out.push(other),
                    }
                }
                Ok(J::Str(String::from_utf8_lossy(&out).into_owned()))
            }
            b't' if self.b[self.i..].starts_with(b"true") => {
                self.i += 4;
                Ok(J::Bool(true))
            }
            b'f' if self.b[self.i..].starts_with(b"false") => {
                self.i += 5;
                Ok(J::Bool(false))
            }
            b'n' if self.b[self.i..].starts_with(b"null") => {
                self.i += 4;
                Ok(J::Null)
            }
            _ => {
                let st = self.i;
                while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                if st == self.i {
                    return Err(format!("unexpected byte {} at {}", self.b[self.i], self.i));
                }
                let t = std::str::from_utf8(&self.b[st..self.i]).map_err(|e| e.to_string())?;
                t.parse::<f64>().map(J::Num).map_err(|e| format!("{}: {}", t, e))
            }
        }
    }
}

fn parse_json(s: &str) -> Result<J, String> {
    let mut p = Parser { b: s.as_bytes(), i: 0 };
    let v = p.value()?;
    p.ws();
    if p.i != p.b.len() {
        return Err("trailing".into());
    }
    Ok(v)
}

// ----------------------------------------------------------------------------
// Deterministic RNG
// ----------------------------------------------------------------------------

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Rng {
        let mut r = Rng(seed.wrapping_add(0x9E3779B97F4A7C15));
        r.next();
        r
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn unif(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % (n as u64)) as usize
    }
    fn normal(&mut self) -> f64 {
        let u1 = self.unif().max(1e-300);
        let u2 = self.unif();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

// ----------------------------------------------------------------------------
// Combinatorics: columns, semi-regular degree
// ----------------------------------------------------------------------------

fn binom(n: i64, k: i64) -> u128 {
    if k < 0 || k > n {
        return 0;
    }
    let k = k.min(n - k) as u128;
    let n = n as u128;
    let mut r: u128 = 1;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

/// log2 of the number of monomials of degree <= d in N Boolean unknowns.
fn logcols(nn: i64, d: i64) -> f64 {
    let d = d.max(0).min(nn);
    let mut s: u128 = 0;
    for i in 0..=d {
        s += binom(nn, i);
    }
    (s as f64).log2()
}

fn nvars(m: i64, n: i64, l: i64) -> i64 {
    m * l + (m - 2) * n
}

/// Semi-regular degree of regularity: index of the first non-positive
/// coefficient of (1+z)^N / prod(1+z^d), degrees [2]*n + [3]*((m-2)*n).
fn dreg(m: i64, n: i64, l: i64) -> i64 {
    let nn = nvars(m, n, l);
    let cap = 400usize;
    let mut c: Vec<i128> = (0..=cap).map(|i| binom(nn, i as i64) as i128).collect();
    let mut degs: Vec<usize> = vec![2; n as usize];
    degs.extend(vec![3usize; ((m - 2) * n) as usize]);
    for d in degs {
        let mut o = vec![0i128; cap + 1];
        for i in 0..=cap {
            o[i] = c[i] - if i >= d { o[i - d] } else { 0 };
        }
        c = o;
    }
    for (i, x) in c.iter().enumerate() {
        if *x <= 0 {
            return i as i64;
        }
    }
    cap as i64
}

// ----------------------------------------------------------------------------
// Records and harvesting
// ----------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Rec {
    fam: String,
    eng: u8, // 0 xl, 1 closure, 2 f4
    m: i64,
    n: i64,
    l: i64,
    sat: bool,
    d: f64,
    cens: bool,
    cols: Option<f64>,
    key: String,
}

impl Rec {
    fn nn(&self) -> i64 {
        nvars(self.m, self.n, self.l)
    }
}

fn eng_name(e: u8) -> &'static str {
    match e {
        0 => "xl",
        1 => "cl",
        _ => "f4",
    }
}

#[derive(Default, Clone)]
struct Audit {
    files: usize,
    lines: usize,
    parse_err: usize,
    exact: usize,
    cens: usize,
    drop_nolabel: usize,
    drop_nvars: usize,
    drop_dup: usize,
    drop_incons: usize,
}

fn read_manifest(path: &str) -> Result<Vec<(String, String, String)>, String> {
    let t = fs::read_to_string(path).map_err(|e| format!("{}: {}", path, e))?;
    let mut v = Vec::new();
    for line in t.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let p: Vec<&str> = line.split('\t').collect();
        if p.len() != 3 {
            return Err(format!("bad manifest line: {}", line));
        }
        v.push((p[0].to_string(), p[1].to_string(), p[2].to_string()));
    }
    Ok(v)
}

fn mk(fam_eng: u8, m: i64, n: i64, l: i64, sat: bool, d: f64, cens: bool, key: String) -> Rec {
    Rec { fam: format!("{}-m{}", eng_name(fam_eng), m), eng: fam_eng, m, n, l, sat, d, cens, cols: None, key }
}

fn harvest_obj(tag: &str, o: &J, a: &mut Audit) -> Option<Rec> {
    match tag {
        "xl" => {
            let n = match o.get("n").and_then(|x| x.f()) {
                Some(x) => x as i64,
                None => {
                    a.drop_nolabel += 1;
                    return None;
                }
            };
            let m = o.get("m").and_then(|x| x.f()).unwrap_or(3.0) as i64;
            let l = o.get("ell")?.f()? as i64;
            if let Some(nv) = o.get("n_vars").and_then(|x| x.f()) {
                if nv as i64 != nvars(m, n, l) {
                    a.drop_nvars += 1;
                    return None;
                }
            }
            let oc = o.get("outcome")?;
            let kind = oc.get("kind")?.s()?;
            let deg = oc.get("degree").and_then(|x| x.f());
            let key = format!(
                "xl|{}|{}|{}|{}|{}",
                m,
                n,
                l,
                o.get("v_basis").map(|x| x.show()).unwrap_or_default(),
                o.get("x_r").map(|x| x.show()).unwrap_or_default()
            );
            match (kind, deg) {
                ("resolved", Some(d)) if oc.get("refuted").and_then(|x| x.b()).unwrap_or(true) => {
                    Some(mk(0, m, n, l, false, d, false, key))
                }
                ("at_least", Some(d)) => Some(mk(0, m, n, l, false, d, true, key)),
                _ => {
                    a.drop_nolabel += 1;
                    None
                }
            }
        }
        "fdb" => {
            let n = o.get("n")?.f()? as i64;
            let l = o.get("nprime")?.f()? as i64;
            let unsat = o.get("unsat")?.b()?;
            let key = format!(
                "fdb|{}|{}|{}|{}|{}|{}",
                n,
                o.get("seed").map(|x| x.show()).unwrap_or_default(),
                o.get("family").map(|x| x.show()).unwrap_or_default(),
                unsat,
                o.get("x3").map(|x| x.show()).unwrap_or_default(),
                o.get("b").map(|x| x.show()).unwrap_or_default()
            );
            let x = o.get("xl_solving")?;
            if let Some(d) = x.f() {
                Some(mk(0, 2, n, l, !unsat, d, false, key))
            } else {
                let pd = o.get("plain_dims")?;
                let mut mx = 0f64;
                if let J::Obj(v) = pd {
                    for (k, _) in v {
                        if let Ok(x) = k.parse::<f64>() {
                            mx = mx.max(x);
                        }
                    }
                }
                if mx == 0.0 {
                    a.drop_nolabel += 1;
                    return None;
                }
                Some(mk(0, 2, n, l, !unsat, mx + 1.0, true, key))
            }
        }
        "closure" => {
            let m = o.get("m")?.f()? as i64;
            let n = o.get("n")?.f()? as i64;
            let l = o.get("ell")?.f()? as i64;
            if let Some(oc) = o.get("committed_outcome") {
                if oc.get("kind").and_then(|x| x.s()) == Some("satisfiable") {
                    a.drop_nolabel += 1;
                    return None;
                }
            }
            if let Some(s) = o.get("solutions").and_then(|x| x.f()) {
                if s != 0.0 {
                    a.drop_nolabel += 1;
                    return None;
                }
            }
            if let Some(nv) = o.get("n_vars").and_then(|x| x.f()) {
                if nv as i64 != nvars(m, n, l) {
                    a.drop_nvars += 1;
                    return None;
                }
            }
            let scan = o.get("scan")?.arr()?;
            if scan.is_empty() {
                a.drop_nolabel += 1;
                return None;
            }
            let mut first: Option<f64> = None;
            for e in scan {
                if e.get("closure_refuted").and_then(|x| x.b()) == Some(true) {
                    let d = e.get("degree")?.f()?;
                    first = Some(first.map_or(d, |f: f64| f.min(d)));
                }
            }
            let key = format!(
                "cl|{}|{}|{}|{}|{}",
                o.get("source").map(|x| x.show()).unwrap_or_default(),
                o.get("cell").map(|x| x.show()).unwrap_or_default(),
                o.get("draw").map(|x| x.show()).unwrap_or_default(),
                m,
                n
            );
            match first {
                Some(d) => Some(mk(1, m, n, l, false, d, false, key)),
                None => {
                    let mt = o.get("measured_to")?.f()?;
                    Some(mk(1, m, n, l, false, mt + 1.0, true, key))
                }
            }
        }
        "f4" => {
            let ex = o.get("export")?;
            let m = ex.get("m")?.f()? as i64;
            let n = ex.get("n")?.f()? as i64;
            let l = ex.get("ell")?.f()? as i64;
            let sols = ex.get("solutions")?.f()?;
            let sat = sols != 0.0;
            if let Some(nv) = ex.get("n_vars").and_then(|x| x.f()) {
                if nv as i64 != nvars(m, n, l) {
                    a.drop_nvars += 1;
                    return None;
                }
            }
            let finished = o.get("exit").and_then(|x| x.f()) == Some(0.0);
            let df4 = match o.get("d_f4") {
                Some(x) if !x.is_null() => x.f()?,
                _ => {
                    a.drop_nolabel += 1;
                    return None;
                }
            };
            let gb1 = o.get("gb_is_one").and_then(|x| x.b()).unwrap_or(false);
            let key = format!(
                "f4|{}|{}|{}|{}|{}|{}",
                m,
                n,
                l,
                ex.get("v_basis").map(|x| x.show()).unwrap_or_default(),
                ex.get("x_r").map(|x| x.show()).unwrap_or_default(),
                sat
            );
            let cols = if finished {
                o.get("max_matrix").and_then(|x| x.arr()).and_then(|v| v.get(1)).and_then(|x| x.f()).map(|c| c.log2())
            } else {
                None
            };
            let mut r = if !sat {
                if finished && gb1 {
                    mk(2, m, n, l, false, df4, false, key)
                } else if finished {
                    a.drop_incons += 1;
                    return None;
                } else {
                    mk(2, m, n, l, false, df4, true, key)
                }
            } else if finished {
                mk(2, m, n, l, true, df4, false, key)
            } else {
                mk(2, m, n, l, true, df4, true, key)
            };
            r.cols = cols;
            Some(r)
        }
        "f4rand" => {
            let label = o.get("label")?.s()?;
            let (n, l) = match label.split('-').next()? {
                "r62" => (6, 2),
                "r93" => (9, 3),
                _ => {
                    a.drop_nolabel += 1;
                    return None;
                }
            };
            let df4 = o.get("d_f4")?.f()?;
            let finished = o.get("exit").and_then(|x| x.f()) == Some(0.0);
            let mut r = mk(2, 4, n, l, false, df4, !finished, format!("f4r|{}", label));
            r.fam = "f4r-m4".into();
            Some(r)
        }
        _ => None,
    }
}

fn harvest(manifest: &str) -> Result<(Vec<Rec>, BTreeMap<String, Audit>, String), String> {
    let man = read_manifest(manifest)?;
    let mut recs: Vec<Rec> = Vec::new();
    let mut audits: BTreeMap<String, Audit> = BTreeMap::new();
    let mut seen: HashSet<String> = HashSet::new();
    for (path, sha, tag) in &man {
        let bytes = fs::read(path).map_err(|e| format!("{}: {}", path, e))?;
        let got = sha256(&bytes);
        if &got != sha {
            return Err(format!("SHA MISMATCH {} expected {} got {}", path, sha, got));
        }
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let a = audits.entry(tag.clone()).or_default();
        a.files += 1;
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            a.lines += 1;
            let o = match parse_json(line) {
                Ok(o) => o,
                Err(_) => {
                    a.parse_err += 1;
                    continue;
                }
            };
            if let Some(r) = harvest_obj(tag, &o, a) {
                if !seen.insert(r.key.clone()) {
                    a.drop_dup += 1;
                    continue;
                }
                if r.cens {
                    a.cens += 1;
                } else {
                    a.exact += 1;
                }
                recs.push(r);
            }
        }
    }
    let mut tsv = String::from("fam\teng\tm\tn\tl\tN\tE\tS\tsat\td\tcens\tlogcols_meas\tkey\n");
    let mut rows: Vec<String> = recs
        .iter()
        .map(|r| {
            format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                r.fam,
                r.eng,
                r.m,
                r.n,
                r.l,
                r.nn(),
                (r.m - 1) * r.n,
                r.n - r.m * r.l,
                r.sat as u8,
                r.d,
                r.cens as u8,
                r.cols.map(|c| format!("{:.6}", c)).unwrap_or_else(|| "NA".into()),
                r.key
            )
        })
        .collect();
    rows.sort();
    for r in rows {
        tsv.push_str(&r);
        tsv.push('\n');
    }
    Ok((recs, audits, tsv))
}

// ----------------------------------------------------------------------------
// Items, models
// ----------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Item {
    fam: String,
    cell: String,
    x1: Vec<f64>, // m, n, l, N, E, S, m*l, eng, sat
    y: f64,
    cens: bool,
    eng: u8,
    nn: i64,
    m: i64,
    n: i64,
    l: i64,
    sat: bool,
    dreg: f64,
    a_pred: f64,
}

fn item_from(r: &Rec, y: f64, t3: bool) -> Item {
    let nn = r.nn();
    let dr = dreg(r.m, r.n, r.l) as f64;
    let x1 = vec![
        r.m as f64,
        r.n as f64,
        r.l as f64,
        nn as f64,
        ((r.m - 1) * r.n) as f64,
        (r.n - r.m * r.l) as f64,
        (r.m * r.l) as f64,
        r.eng as f64,
        r.sat as u8 as f64,
    ];
    Item {
        fam: r.fam.clone(),
        cell: format!("{}|{}|{}|{}", r.fam, r.n, r.l, r.sat as u8),
        x1,
        y,
        cens: r.cens,
        eng: r.eng,
        nn,
        m: r.m,
        n: r.n,
        l: r.l,
        sat: r.sat,
        dreg: dr,
        a_pred: if t3 { logcols(nn, dr as i64) } else { dr },
    }
}

fn items_t1(recs: &[Rec]) -> Vec<Item> {
    recs.iter().map(|r| item_from(r, r.d, false)).collect()
}

fn items_t3(recs: &[Rec]) -> Vec<Item> {
    recs.iter().filter(|r| r.eng == 2 && !r.cens && r.cols.is_some()).map(|r| item_from(r, r.cols.unwrap(), true)).collect()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Arm {
    A,
    B,
    C1,
    C2,
    E,
}

fn arm_name(a: Arm) -> &'static str {
    match a {
        Arm::A => "a",
        Arm::B => "b",
        Arm::C1 => "c1",
        Arm::C2 => "c2",
        Arm::E => "e",
    }
}

// ---- gradient-boosted trees

#[derive(Clone, Debug)]
enum Node {
    Leaf(f64),
    Split { f: usize, t: f64, l: usize, r: usize },
}

#[derive(Clone, Debug)]
struct Gbt {
    init: f64,
    lr: f64,
    trees: Vec<Vec<Node>>,
    with_dreg: bool,
}

const GBT_ROUNDS: usize = 200;
const GBT_DEPTH: usize = 3;
const GBT_LR: f64 = 0.1;
const GBT_MIN_LEAF: f64 = 5.0;

fn build_tree(nodes: &mut Vec<Node>, xs: &[Vec<f64>], w: &[f64], r: &[f64], idx: &[usize], depth: usize) -> usize {
    let wt: f64 = idx.iter().map(|&i| w[i]).sum();
    let st: f64 = idx.iter().map(|&i| w[i] * r[i]).sum();
    let me = nodes.len();
    nodes.push(Node::Leaf(if wt > 0.0 { st / wt } else { 0.0 }));
    if depth >= GBT_DEPTH || idx.len() < 2 {
        return me;
    }
    let p = xs[0].len();
    let mut best_gain = 1e-12;
    let mut best: Option<(usize, f64)> = None;
    for f in 0..p {
        let mut order: Vec<usize> = idx.to_vec();
        order.sort_by(|&a, &b| xs[a][f].partial_cmp(&xs[b][f]).unwrap().then(a.cmp(&b)));
        let mut wl = 0.0;
        let mut sl = 0.0;
        for k in 0..order.len() - 1 {
            let i = order[k];
            wl += w[i];
            sl += w[i] * r[i];
            let nx = order[k + 1];
            if xs[i][f] == xs[nx][f] {
                continue;
            }
            if wl < GBT_MIN_LEAF || wt - wl < GBT_MIN_LEAF {
                continue;
            }
            let gain = sl * sl / wl + (st - sl) * (st - sl) / (wt - wl) - st * st / wt;
            if gain > best_gain {
                best_gain = gain;
                best = Some((f, 0.5 * (xs[i][f] + xs[nx][f])));
            }
        }
    }
    match best {
        None => me,
        Some((f, t)) => {
            let (li, ri): (Vec<usize>, Vec<usize>) = idx.iter().partition(|&&i| xs[i][f] <= t);
            let l = build_tree(nodes, xs, w, r, &li, depth + 1);
            let rr = build_tree(nodes, xs, w, r, &ri, depth + 1);
            nodes[me] = Node::Split { f, t, l, r: rr };
            me
        }
    }
}

fn tree_eval(nodes: &[Node], x: &[f64]) -> f64 {
    let mut i = 0;
    loop {
        match &nodes[i] {
            Node::Leaf(v) => return *v,
            Node::Split { f, t, l, r } => {
                i = if x[*f] <= *t { *l } else { *r };
            }
        }
    }
}

fn gx(it: &Item, with_dreg: bool) -> Vec<f64> {
    let mut x = it.x1.clone();
    if with_dreg {
        x.push(it.dreg);
    }
    x
}

fn fit_gbt(train: &[&Item], with_dreg: bool) -> Gbt {
    // aggregate identical feature vectors: weighted trees on group means are
    // the same as trees on the records
    let mut groups: BTreeMap<Vec<u64>, (Vec<f64>, f64, f64)> = BTreeMap::new();
    for it in train {
        let x = gx(it, with_dreg);
        let k: Vec<u64> = x.iter().map(|v| v.to_bits()).collect();
        let e = groups.entry(k).or_insert((x, 0.0, 0.0));
        e.1 += 1.0;
        e.2 += it.y;
    }
    let xs: Vec<Vec<f64>> = groups.values().map(|g| g.0.clone()).collect();
    let w: Vec<f64> = groups.values().map(|g| g.1).collect();
    let ym: Vec<f64> = groups.values().map(|g| g.2 / g.1).collect();
    let wsum: f64 = w.iter().sum();
    let init = if wsum > 0.0 { w.iter().zip(ym.iter()).map(|(a, b)| a * b).sum::<f64>() / wsum } else { 0.0 };
    let mut f = vec![init; xs.len()];
    let mut trees = Vec::new();
    let idx: Vec<usize> = (0..xs.len()).collect();
    if xs.is_empty() {
        return Gbt { init, lr: GBT_LR, trees, with_dreg };
    }
    for _ in 0..GBT_ROUNDS {
        let r: Vec<f64> = (0..xs.len()).map(|i| ym[i] - f[i]).collect();
        let mut nodes = Vec::new();
        build_tree(&mut nodes, &xs, &w, &r, &idx, 0);
        for i in 0..xs.len() {
            f[i] += GBT_LR * tree_eval(&nodes, &xs[i]);
        }
        trees.push(nodes);
    }
    Gbt { init, lr: GBT_LR, trees, with_dreg }
}

fn gbt_predict(g: &Gbt, it: &Item) -> f64 {
    let x = gx(it, g.with_dreg);
    let mut v = g.init;
    for t in &g.trees {
        v += g.lr * tree_eval(t, &x);
    }
    v
}

// ---- ridge linear

fn lin_feats(it: &Item) -> Vec<f64> {
    vec![1.0, it.m as f64, it.l as f64, it.n as f64, (it.m * it.l) as f64, (it.eng == 1) as u8 as f64, (it.eng == 2) as u8 as f64, it.sat as u8 as f64]
}

fn fit_lin(train: &[&Item]) -> Vec<f64> {
    let p = 8;
    let mut a = vec![vec![0.0; p + 1]; p];
    for it in train {
        let x = lin_feats(it);
        for i in 0..p {
            for j in 0..p {
                a[i][j] += x[i] * x[j];
            }
            a[i][p] += x[i] * it.y;
        }
    }
    for i in 1..p {
        a[i][i] += 1e-3;
    }
    for c in 0..p {
        let mut piv = c;
        for r in c + 1..p {
            if a[r][c].abs() > a[piv][c].abs() {
                piv = r;
            }
        }
        a.swap(c, piv);
        let d = a[c][c];
        if d.abs() < 1e-12 {
            continue;
        }
        for j in c..=p {
            a[c][j] /= d;
        }
        for r in 0..p {
            if r != c {
                let f = a[r][c];
                if f != 0.0 {
                    for j in c..=p {
                        let v = a[c][j];
                        a[r][j] -= f * v;
                    }
                }
            }
        }
    }
    (0..p).map(|i| a[i][p]).collect()
}

fn median(v: &mut Vec<f64>) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

enum Model {
    A,
    Median(HashMap<u8, f64>, f64),
    G(Gbt),
    L(Vec<f64>),
}

fn fit_arm(arm: Arm, train: &[&Item]) -> Model {
    match arm {
        Arm::A => Model::A,
        Arm::B => {
            let mut by: HashMap<u8, Vec<f64>> = HashMap::new();
            let mut all = Vec::new();
            for it in train {
                by.entry(it.eng).or_default().push(it.y);
                all.push(it.y);
            }
            let g = median(&mut all);
            let mut mm = HashMap::new();
            for (k, mut v) in by {
                mm.insert(k, median(&mut v));
            }
            Model::Median(mm, g)
        }
        Arm::C1 => Model::G(fit_gbt(train, false)),
        Arm::C2 => Model::G(fit_gbt(train, true)),
        Arm::E => Model::L(fit_lin(train)),
    }
}

fn predict(m: &Model, it: &Item) -> f64 {
    match m {
        Model::A => it.a_pred,
        Model::Median(mm, g) => *mm.get(&it.eng).unwrap_or(g),
        Model::G(g) => gbt_predict(g, it),
        Model::L(b) => lin_feats(it).iter().zip(b.iter()).map(|(x, c)| x * c).sum(),
    }
}

// ----------------------------------------------------------------------------
// Leave-one-family-out with repeated split conformal
// ----------------------------------------------------------------------------

#[derive(Clone, Default)]
struct ArmRes {
    yhat: f64,
    err: f64,
    serr: f64,
    cover: f64,
    q: f64,
    cons: f64,
}

struct LofoCfg {
    arms: Vec<Arm>,
    nsplits: usize,
    seed0: u64,
    t3: bool,
    /// drop from training every record whose (m, n, l) shape occurs in the
    /// held-out family (so the held-out cells are unseen in every engine)
    disjoint: bool,
}

fn quantile_k(res: &mut Vec<f64>, level: f64) -> f64 {
    res.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = res.len();
    if n == 0 {
        return f64::INFINITY;
    }
    let k = ((level * (n as f64 + 1.0)).ceil() as usize).max(1).min(n);
    res[k - 1]
}

fn size_of(it: &Item, yhat: f64) -> f64 {
    let d = yhat.round().max(1.0).min(it.nn as f64) as i64;
    logcols(it.nn, d)
}

/// Returns, per item, the per-arm split-averaged results (None when the
/// item's family has no training data).
fn lofo(items: &[Item], cfg: &LofoCfg) -> Vec<Option<Vec<ArmRes>>> {
    let mut out: Vec<Option<Vec<ArmRes>>> = vec![None; items.len()];
    let mut fams: Vec<String> = items.iter().map(|i| i.fam.clone()).collect::<HashSet<_>>().into_iter().collect();
    fams.sort();
    for f in &fams {
        let held_shapes: HashSet<(i64, i64, i64)> = items.iter().filter(|i| &i.fam == f).map(|i| (i.m, i.n, i.l)).collect();
        let train: Vec<&Item> = items
            .iter()
            .filter(|i| &i.fam != f && !i.cens && !(cfg.disjoint && held_shapes.contains(&(i.m, i.n, i.l))))
            .collect();
        if train.is_empty() {
            continue;
        }
        let test_idx: Vec<usize> = (0..items.len()).filter(|&i| &items[i].fam == f).collect();
        let mut cells: Vec<String> = train.iter().map(|i| i.cell.clone()).collect::<HashSet<_>>().into_iter().collect();
        cells.sort();
        let ncal = (((cells.len() as f64) * 0.3).round() as usize).max(1).min(cells.len().saturating_sub(1).max(1));
        let mut acc: Vec<Vec<ArmRes>> = vec![vec![ArmRes::default(); cfg.arms.len()]; test_idx.len()];
        for r in 0..cfg.nsplits {
            let mut rng = Rng::new(cfg.seed0 + r as u64);
            let mut order = cells.clone();
            rng.shuffle(&mut order);
            let cal_cells: HashSet<&String> = order.iter().take(ncal).collect();
            let proper: Vec<&Item> = train.iter().copied().filter(|i| !cal_cells.contains(&i.cell)).collect();
            let cal: Vec<&Item> = train.iter().copied().filter(|i| cal_cells.contains(&i.cell)).collect();
            for (ai, arm) in cfg.arms.iter().enumerate() {
                let model = fit_arm(*arm, &proper);
                let mut res: Vec<f64> = cal.iter().map(|it| (it.y - predict(&model, it)).abs()).collect();
                let q = quantile_k(&mut res, 0.9);
                for (ti, &idx) in test_idx.iter().enumerate() {
                    let it = &items[idx];
                    let yh = predict(&model, it);
                    let a = &mut acc[ti][ai];
                    a.yhat += yh;
                    a.q += q;
                    if it.cens {
                        a.cons += ((yh + q) >= it.y) as u8 as f64;
                    } else {
                        a.err += (it.y - yh).abs();
                        a.cover += ((it.y - yh).abs() <= q) as u8 as f64;
                        if cfg.t3 {
                            a.serr += (it.y - yh).abs();
                        } else {
                            a.serr += (size_of(it, it.y) - size_of(it, yh)).abs();
                        }
                    }
                }
            }
        }
        let rr = cfg.nsplits as f64;
        for (ti, &idx) in test_idx.iter().enumerate() {
            let v: Vec<ArmRes> = acc[ti]
                .iter()
                .map(|a| ArmRes { yhat: a.yhat / rr, err: a.err / rr, serr: a.serr / rr, cover: a.cover / rr, q: a.q / rr, cons: a.cons / rr })
                .collect();
            out[idx] = Some(v);
        }
    }
    out
}

// ----------------------------------------------------------------------------
// Aggregation, bootstrap, rank correlation
// ----------------------------------------------------------------------------

fn spearman(x: &[f64], y: &[f64]) -> f64 {
    fn ranks(v: &[f64]) -> Vec<f64> {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
        let mut r = vec![0.0; v.len()];
        let mut i = 0;
        while i < idx.len() {
            let mut j = i;
            while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
                j += 1;
            }
            let avg = (i + j) as f64 / 2.0 + 1.0;
            for k in i..=j {
                r[idx[k]] = avg;
            }
            i = j + 1;
        }
        r
    }
    let n = x.len();
    if n < 2 {
        return 0.0;
    }
    let (rx, ry) = (ranks(x), ranks(y));
    let mx = rx.iter().sum::<f64>() / n as f64;
    let my = ry.iter().sum::<f64>() / n as f64;
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..n {
        sxy += (rx[i] - mx) * (ry[i] - my);
        sxx += (rx[i] - mx).powi(2);
        syy += (ry[i] - my).powi(2);
    }
    if sxx < 1e-12 || syy < 1e-12 {
        0.0
    } else {
        sxy / (sxx * syy).sqrt()
    }
}

struct FoldInfo {
    fam: String,
    cells: Vec<Vec<usize>>, // exact items per cell
    evaluable: bool,
}

fn build_folds(items: &[Item], res: &[Option<Vec<ArmRes>>], min_cells: usize) -> Vec<FoldInfo> {
    let mut by: BTreeMap<String, BTreeMap<String, Vec<usize>>> = BTreeMap::new();
    for (i, it) in items.iter().enumerate() {
        if it.cens || res[i].is_none() {
            continue;
        }
        by.entry(it.fam.clone()).or_default().entry(it.cell.clone()).or_default().push(i);
    }
    by.into_iter()
        .map(|(fam, cells)| {
            let cells: Vec<Vec<usize>> = cells.into_values().collect();
            let evaluable = cells.len() >= min_cells;
            FoldInfo { fam, cells, evaluable }
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Met {
    Err,
    Serr,
    Cover,
}

fn metv(r: &ArmRes, m: Met) -> f64 {
    match m {
        Met::Err => r.err,
        Met::Serr => r.serr,
        Met::Cover => r.cover,
    }
}

fn cell_mean(res: &[Option<Vec<ArmRes>>], cell: &[usize], arm: usize, m: Met) -> f64 {
    cell.iter().map(|&i| metv(&res[i].as_ref().unwrap()[arm], m)).sum::<f64>() / cell.len() as f64
}

fn macro_metric(res: &[Option<Vec<ArmRes>>], folds: &[FoldInfo], arm: usize, m: Met) -> f64 {
    let mut s = 0.0;
    let mut k = 0.0;
    for f in folds.iter().filter(|f| f.evaluable) {
        let fm: f64 = f.cells.iter().map(|c| cell_mean(res, c, arm, m)).sum::<f64>() / f.cells.len() as f64;
        s += fm;
        k += 1.0;
    }
    if k == 0.0 {
        f64::NAN
    } else {
        s / k
    }
}

fn macro_rho(items: &[Item], res: &[Option<Vec<ArmRes>>], folds: &[FoldInfo], arm: usize) -> f64 {
    let mut s = 0.0;
    let mut k = 0.0;
    for f in folds.iter().filter(|f| f.evaluable) {
        let yh: Vec<f64> = f.cells.iter().map(|c| c.iter().map(|&i| res[i].as_ref().unwrap()[arm].yhat).sum::<f64>() / c.len() as f64).collect();
        let yy: Vec<f64> = f.cells.iter().map(|c| c.iter().map(|&i| items[i].y).sum::<f64>() / c.len() as f64).collect();
        s += spearman(&yh, &yy);
        k += 1.0;
    }
    if k == 0.0 {
        f64::NAN
    } else {
        s / k
    }
}

/// Paired cluster bootstrap of macro(err_other) - macro(err_ref), per metric.
/// Returns (lo, hi) of the 95 % percentile interval for each `other` arm.
fn boot_diff(res: &[Option<Vec<ArmRes>>], folds: &[FoldInfo], ref_arm: usize, others: &[usize], m: Met, b: usize, seed: u64) -> Vec<(f64, f64)> {
    let ev: Vec<&FoldInfo> = folds.iter().filter(|f| f.evaluable).collect();
    // per fold, per cell, per arm cell means
    let mut arms: Vec<usize> = others.to_vec();
    arms.push(ref_arm);
    let cm: Vec<Vec<Vec<f64>>> = ev.iter().map(|f| f.cells.iter().map(|c| arms.iter().map(|&a| cell_mean(res, c, a, m)).collect()).collect()).collect();
    let mut rng = Rng::new(seed);
    let mut draws: Vec<Vec<f64>> = vec![Vec::new(); others.len()];
    for _ in 0..b {
        let mut macro_sum = vec![0.0; arms.len()];
        for f in &cm {
            let c = f.len();
            let mut fs = vec![0.0; arms.len()];
            for _ in 0..c {
                let k = rng.below(c);
                for a in 0..arms.len() {
                    fs[a] += f[k][a];
                }
            }
            for a in 0..arms.len() {
                macro_sum[a] += fs[a] / c as f64;
            }
        }
        let refv = macro_sum[arms.len() - 1];
        for (o, d) in draws.iter_mut().enumerate() {
            d.push((macro_sum[o] - refv) / cm.len() as f64);
        }
    }
    draws
        .into_iter()
        .map(|mut d| {
            d.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let lo = d[((0.025 * b as f64) as usize).min(b - 1)];
            let hi = d[((0.975 * b as f64) as usize).min(b - 1)];
            (lo, hi)
        })
        .collect()
}

// ----------------------------------------------------------------------------
// Output helpers
// ----------------------------------------------------------------------------

struct Kv {
    s: String,
}
impl Kv {
    fn new() -> Kv {
        Kv { s: String::new() }
    }
    fn put(&mut self, k: &str, v: &str) {
        let _ = writeln!(self.s, "{}={}", k, v);
        println!("KV {}={}", k, v);
    }
    fn putf(&mut self, k: &str, v: f64) {
        self.put(k, &format!("{:.6}", v));
    }
    fn write(&self, dir: &str, name: &str) -> Result<(), String> {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        fs::write(format!("{}/{}", dir, name), &self.s).map_err(|e| e.to_string())
    }
}

fn read_kv(dir: &str, name: &str) -> Result<HashMap<String, String>, String> {
    let t = fs::read_to_string(format!("{}/{}", dir, name)).map_err(|e| format!("{}/{}: {}", dir, name, e))?;
    let mut m = HashMap::new();
    for l in t.lines() {
        if let Some((k, v)) = l.split_once('=') {
            m.insert(k.to_string(), v.to_string());
        }
    }
    Ok(m)
}

fn kvf(m: &HashMap<String, String>, k: &str) -> Result<f64, String> {
    m.get(k).ok_or(format!("missing key {}", k))?.parse::<f64>().map_err(|e| format!("{}: {}", k, e))
}

fn arg(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned()
}

const ALL_ARMS: [Arm; 5] = [Arm::A, Arm::B, Arm::C1, Arm::C2, Arm::E];
const SEED0: u64 = 20261018;
const NSPLITS: usize = 20;
const BOOT: usize = 2000;

fn load(args: &[String]) -> Result<(Vec<Rec>, BTreeMap<String, Audit>, String, String), String> {
    let man = arg(args, "--manifest").ok_or("--manifest required")?;
    let raw = arg(args, "--raw").ok_or("--raw required")?;
    let (mut recs, audits, tsv) = harvest(&man)?;
    if args.iter().any(|a| a == "--plant") {
        // smoke-test switch: every label becomes the planted law, so the code
        // paths can be exercised without reading any real result
        for r in recs.iter_mut() {
            r.d = plant(r);
            r.cens = false;
            if r.cols.is_some() {
                r.cols = Some(logcols(r.nn(), r.d as i64));
            }
        }
    }
    Ok((recs, audits, tsv, raw))
}

// ----------------------------------------------------------------------------
// Subcommands
// ----------------------------------------------------------------------------

fn cmd_harvest(args: &[String]) -> Result<(), String> {
    let (recs, audits, tsv, raw) = load(args)?;
    fs::create_dir_all(&raw).map_err(|e| e.to_string())?;
    fs::write(format!("{}/data.tsv", raw), &tsv).map_err(|e| e.to_string())?;
    println!("DATA_SHA256 {}", sha256(tsv.as_bytes()));
    println!("SOURCES_VERIFIED {}", audits.values().map(|a| a.files).sum::<usize>());
    for (t, a) in &audits {
        println!(
            "AUDIT tag={} files={} lines={} parse_err={} exact={} censored={} drop_nolabel={} drop_nvars={} drop_dup={} drop_incons={}",
            t, a.files, a.lines, a.parse_err, a.exact, a.cens, a.drop_nolabel, a.drop_nvars, a.drop_dup, a.drop_incons
        );
    }
    let mut fam: BTreeMap<String, (usize, usize, HashSet<String>, HashSet<String>)> = BTreeMap::new();
    for r in &recs {
        let e = fam.entry(r.fam.clone()).or_insert((0, 0, HashSet::new(), HashSet::new()));
        if r.cens {
            e.1 += 1;
        } else {
            e.0 += 1;
            e.2.insert(format!("{}|{}|{}", r.n, r.l, r.sat as u8));
        }
        e.3.insert(format!("{}|{}|{}", r.n, r.l, r.sat as u8));
    }
    for (f, (ex, ce, ec, ac)) in &fam {
        println!("FAMILY {} exact_records={} censored_records={} exact_cells={} all_cells={}", f, ex, ce, ec.len(), ac.len());
    }
    // value histogram per family (labels, not errors)
    let mut hist: BTreeMap<(String, i64, bool), usize> = BTreeMap::new();
    for r in &recs {
        *hist.entry((r.fam.clone(), r.d as i64, r.cens)).or_insert(0) += 1;
    }
    for ((f, d, c), k) in hist {
        println!("LABELS {} D{}{} {}", f, if c { ">=" } else { "=" }, d, k);
    }
    println!("TOTAL records={} exact={} censored={}", recs.len(), recs.iter().filter(|r| !r.cens).count(), recs.iter().filter(|r| r.cens).count());
    Ok(())
}

fn print_arm_table(items: &[Item], res: &[Option<Vec<ArmRes>>], folds: &[FoldInfo], arms: &[Arm]) {
    for f in folds {
        let mut line = format!("FOLD {} cells={} evaluable={}", f.fam, f.cells.len(), f.evaluable);
        for (ai, a) in arms.iter().enumerate() {
            let fm: f64 = f.cells.iter().map(|c| cell_mean(res, c, ai, Met::Err)).sum::<f64>() / f.cells.len() as f64;
            let cv: f64 = f.cells.iter().map(|c| cell_mean(res, c, ai, Met::Cover)).sum::<f64>() / f.cells.len() as f64;
            let yh: Vec<f64> = f.cells.iter().map(|c| c.iter().map(|&i| res[i].as_ref().unwrap()[ai].yhat).sum::<f64>() / c.len() as f64).collect();
            let yy: Vec<f64> = f.cells.iter().map(|c| c.iter().map(|&i| items[i].y).sum::<f64>() / c.len() as f64).collect();
            let _ = write!(line, " | {} mae={:.4} cover={:.4} rho={:.4}", arm_name(*a), fm, cv, spearman(&yh, &yy));
        }
        println!("{}", line);
    }
}

fn cmd_evaluate(args: &[String]) -> Result<(), String> {
    let (recs, _a, _t, raw) = load(args)?;
    let items = items_t1(&recs);
    let cfg = LofoCfg { arms: ALL_ARMS.to_vec(), nsplits: NSPLITS, seed0: SEED0, t3: false, disjoint: true };
    let res = lofo(&items, &cfg);
    let folds = build_folds(&items, &res, 3);
    print_arm_table(&items, &res, &folds, &cfg.arms);
    let mut kv = Kv::new();
    let nf = folds.iter().filter(|f| f.evaluable).count();
    kv.put("evaluable_folds", &nf.to_string());
    kv.put("folds", &folds.iter().filter(|f| f.evaluable).map(|f| f.fam.clone()).collect::<Vec<_>>().join(","));
    for (ai, a) in cfg.arms.iter().enumerate() {
        let n = arm_name(*a);
        kv.putf(&format!("mae_d_{}", n), macro_metric(&res, &folds, ai, Met::Err));
        kv.putf(&format!("mae_size_{}", n), macro_metric(&res, &folds, ai, Met::Serr));
        kv.putf(&format!("cover_{}", n), macro_metric(&res, &folds, ai, Met::Cover));
        kv.putf(&format!("rho_{}", n), macro_rho(&items, &res, &folds, ai));
        // micro (pooled over exact held-out records), descriptive
        let (mut s, mut k) = (0.0, 0.0);
        for f in folds.iter().filter(|f| f.evaluable) {
            for c in &f.cells {
                for &i in c {
                    s += res[i].as_ref().unwrap()[ai].err;
                    k += 1.0;
                }
            }
        }
        kv.putf(&format!("micro_mae_d_{}", n), s / k);
    }
    // paired bootstrap: differences other - c1 (positive = c1 better)
    let c1 = 2;
    let others = vec![0usize, 1, 3, 4];
    for (m, name) in [(Met::Err, "d"), (Met::Serr, "size")] {
        let ci = boot_diff(&res, &folds, c1, &others, m, BOOT, SEED0 + 777);
        for (k, &o) in others.iter().enumerate() {
            kv.putf(&format!("diff_{}_{}_minus_c1_lo", name, arm_name(cfg.arms[o])), ci[k].0);
            kv.putf(&format!("diff_{}_{}_minus_c1_hi", name, arm_name(cfg.arms[o])), ci[k].1);
        }
    }
    // censored consistency: share with yhat+q >= bound, over censored held-out records
    for (ai, a) in cfg.arms.iter().enumerate() {
        let mut s = 0.0;
        let mut k = 0.0;
        for (i, it) in items.iter().enumerate() {
            if it.cens {
                if let Some(r) = &res[i] {
                    s += r[ai].cons;
                    k += 1.0;
                }
            }
        }
        kv.putf(&format!("cens_consistent_{}", arm_name(*a)), if k > 0.0 { s / k } else { f64::NAN });
        if ai == 0 {
            kv.put("cens_records", &format!("{}", k));
        }
    }
    // sensitivity (descriptive): the same folds without the shape-disjoint
    // training rule, i.e. the held-out cells may be present in other engines
    let cfg_leaky = LofoCfg { arms: ALL_ARMS.to_vec(), nsplits: NSPLITS, seed0: SEED0, t3: false, disjoint: false };
    let res_l = lofo(&items, &cfg_leaky);
    let folds_l = build_folds(&items, &res_l, 3);
    for (ai, a) in cfg.arms.iter().enumerate() {
        kv.putf(&format!("leaky_mae_d_{}", arm_name(*a)), macro_metric(&res_l, &folds_l, ai, Met::Err));
        kv.putf(&format!("leaky_cover_{}", arm_name(*a)), macro_metric(&res_l, &folds_l, ai, Met::Cover));
    }
    kv.write(&raw, "eval.kv")?;
    println!("EVALUATE DONE");
    Ok(())
}

fn plant(r: &Rec) -> f64 {
    3.0 + r.l as f64 + if r.n - r.m * r.l < 0 { 1.0 } else { 0.0 }
}

fn cmd_planted(args: &[String]) -> Result<(), String> {
    let (recs, _a, _t, raw) = load(args)?;
    let planted: Vec<Rec> = recs.iter().filter(|r| !r.cens).map(|r| {
        let mut x = r.clone();
        x.d = plant(r);
        x
    }).collect();
    let items = items_t1(&planted);
    let cfg = LofoCfg { arms: ALL_ARMS.to_vec(), nsplits: 5, seed0: SEED0, t3: false, disjoint: true };
    let res = lofo(&items, &cfg);
    let folds = build_folds(&items, &res, 3);
    print_arm_table(&items, &res, &folds, &cfg.arms);
    let mut kv = Kv::new();
    for (ai, a) in cfg.arms.iter().enumerate() {
        kv.putf(&format!("planted_mae_d_{}", arm_name(*a)), macro_metric(&res, &folds, ai, Met::Err));
    }
    let b = macro_metric(&res, &folds, 1, Met::Err);
    let c = macro_metric(&res, &folds, 2, Met::Err);
    let pass = c <= 0.5 * b;
    kv.put("planted_pass", if pass { "1" } else { "0" });
    kv.put("planted_rule", "mae_c1 <= 0.5 * mae_b");
    kv.write(&raw, "planted.kv")?;
    println!("PLANTED {}", if pass { "PASS" } else { "FAIL" });
    Ok(())
}

fn permute_exact(recs: &[Rec], k: u64) -> Vec<Rec> {
    let mut out: Vec<Rec> = recs.to_vec();
    for e in 0..3u8 {
        let idx: Vec<usize> = (0..out.len()).filter(|&i| out[i].eng == e && !out[i].cens).collect();
        let mut vals: Vec<f64> = idx.iter().map(|&i| out[i].d).collect();
        let mut rng = Rng::new(SEED0 + 100000 + 17 * k + e as u64);
        rng.shuffle(&mut vals);
        for (j, &i) in idx.iter().enumerate() {
            out[i].d = vals[j];
        }
    }
    out
}

fn cmd_null(args: &[String]) -> Result<(), String> {
    let (recs, _a, _t, raw) = load(args)?;
    let nsp = 3;
    let cfg = LofoCfg { arms: vec![Arm::C1], nsplits: nsp, seed0: SEED0, t3: false, disjoint: true };
    // real, same 3 splits
    let items = items_t1(&recs);
    let res = lofo(&items, &cfg);
    let folds = build_folds(&items, &res, 3);
    let real = macro_metric(&res, &folds, 0, Met::Err);
    let mut nulls = Vec::new();
    for k in 0..100u64 {
        let pr = permute_exact(&recs, k);
        let it = items_t1(&pr);
        let rs = lofo(&it, &cfg);
        let fo = build_folds(&it, &rs, 3);
        nulls.push(macro_metric(&rs, &fo, 0, Met::Err));
    }
    nulls.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| nulls[((p * nulls.len() as f64) as usize).min(nulls.len() - 1)];
    let mut kv = Kv::new();
    kv.putf("real3_mae_d_c1", real);
    kv.putf("null_q025", q(0.025));
    kv.putf("null_q500", q(0.5));
    kv.putf("null_q975", q(0.975));
    kv.putf("null_min", nulls[0]);
    kv.putf("null_max", nulls[nulls.len() - 1]);
    let below = nulls.iter().filter(|&&v| v <= real).count();
    kv.put("null_perms_at_or_below_real", &below.to_string());
    kv.put("null_excluded", if real < q(0.025) { "1" } else { "0" });
    kv.write(&raw, "null.kv")?;
    println!("NULL DONE");
    Ok(())
}

fn cmd_t3(args: &[String]) -> Result<(), String> {
    let (recs, _a, _t, raw) = load(args)?;
    let items = items_t3(&recs);
    let arms = vec![Arm::A, Arm::B, Arm::C1, Arm::E];
    let cfg = LofoCfg { arms: arms.clone(), nsplits: NSPLITS, seed0: SEED0, t3: true, disjoint: true };
    let res = lofo(&items, &cfg);
    let folds = build_folds(&items, &res, 3);
    print_arm_table(&items, &res, &folds, &arms);
    let mut kv = Kv::new();
    kv.put("t3_items", &items.len().to_string());
    kv.put("t3_folds", &folds.iter().filter(|f| f.evaluable).map(|f| f.fam.clone()).collect::<Vec<_>>().join(","));
    for (ai, a) in arms.iter().enumerate() {
        kv.putf(&format!("t3_mae_log2cols_{}", arm_name(*a)), macro_metric(&res, &folds, ai, Met::Err));
        kv.putf(&format!("t3_rho_{}", arm_name(*a)), macro_rho(&items, &res, &folds, ai));
    }
    kv.write(&raw, "t3.kv")?;
    println!("T3 DONE");
    Ok(())
}

fn cmd_rank(args: &[String]) -> Result<(), String> {
    let (recs, _a, _t, raw) = load(args)?;
    let items = items_t1(&recs);
    let exact: Vec<&Item> = items.iter().filter(|i| !i.cens).collect();
    let mut cells: Vec<String> = exact.iter().map(|i| i.cell.clone()).collect::<HashSet<_>>().into_iter().collect();
    cells.sort();
    let ncal = ((cells.len() as f64 * 0.3).round() as usize).max(1);
    let mut models = Vec::new();
    for r in 0..NSPLITS {
        let mut rng = Rng::new(SEED0 + r as u64);
        let mut order = cells.clone();
        rng.shuffle(&mut order);
        let cal: HashSet<&String> = order.iter().take(ncal).collect();
        let proper: Vec<&Item> = exact.iter().copied().filter(|i| !cal.contains(&i.cell)).collect();
        models.push(fit_arm(Arm::C1, &proper));
    }
    let measured: HashSet<(u8, i64, i64, i64)> = recs.iter().map(|r| (r.eng, r.m, r.n, r.l)).collect();
    #[derive(Clone)]
    struct Cand {
        eng: u8,
        m: i64,
        n: i64,
        l: i64,
        nn: i64,
        yhat: f64,
        dreg: f64,
        size: f64,
        score: f64,
    }
    let mut cands: Vec<Cand> = Vec::new();
    for eng in [1u8, 2u8] {
        for m in [3i64, 4] {
            for l in 2..=7i64 {
                for n in 3..=24i64 {
                    let nn = nvars(m, n, l);
                    if nn > 60 || measured.contains(&(eng, m, n, l)) {
                        continue;
                    }
                    let r = Rec { fam: format!("{}-m{}", eng_name(eng), m), eng, m, n, l, sat: false, d: 0.0, cens: false, cols: None, key: String::new() };
                    let it = item_from(&r, 0.0, false);
                    let yh: f64 = models.iter().map(|mo| predict(mo, &it)).sum::<f64>() / models.len() as f64;
                    let size = size_of(&it, yh);
                    if size > 22.0 {
                        continue;
                    }
                    cands.push(Cand { eng, m, n, l, nn, yhat: yh, dreg: it.dreg, size, score: (yh - it.dreg).abs() });
                }
            }
        }
    }
    cands.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap().then(a.size.partial_cmp(&b.size).unwrap()).then((a.eng, a.m, a.n, a.l).cmp(&(b.eng, b.m, b.n, b.l))));
    let mut picked: Vec<Cand> = Vec::new();
    let mut used: HashSet<(i64, i64)> = HashSet::new();
    for c in &cands {
        if used.insert((c.m, c.l)) {
            picked.push(c.clone());
        }
        if picked.len() == 5 {
            break;
        }
    }
    let mut txt = String::new();
    let _ = writeln!(txt, "candidates_considered={}", cands.len());
    let _ = writeln!(txt, "rank engine m n l N yhat_c1 dreg_semi |gap| predicted_log2_cols");
    for (i, c) in picked.iter().enumerate() {
        let _ = writeln!(txt, "{} {} {} {} {} {} {:.3} {} {:.3} {:.2}", i + 1, eng_name(c.eng), c.m, c.n, c.l, c.nn, c.yhat, c.dreg, c.score, c.size);
    }
    let _ = writeln!(txt, "-- next 15 by score (not one per (m, l)) --");
    for c in cands.iter().take(15) {
        let _ = writeln!(txt, "- {} {} {} {} {} {:.3} {} {:.3} {:.2}", eng_name(c.eng), c.m, c.n, c.l, c.nn, c.yhat, c.dreg, c.score, c.size);
    }
    print!("{}", txt);
    fs::create_dir_all(&raw).map_err(|e| e.to_string())?;
    fs::write(format!("{}/rank.txt", raw), &txt).map_err(|e| e.to_string())?;
    println!("RANK DONE");
    Ok(())
}

fn cmd_verdict(args: &[String]) -> Result<(), String> {
    let raw = arg(args, "--raw").ok_or("--raw required")?;
    let st = read_kv(&raw, "selftest.kv")?;
    let ev = read_kv(&raw, "eval.kv")?;
    let nu = read_kv(&raw, "null.kv")?;
    let pl = read_kv(&raw, "planted.kv")?;
    let selftest_ok = st.get("selftest_ok").map(|s| s == "1").unwrap_or(false);
    let planted_ok = pl.get("planted_pass").map(|s| s == "1").unwrap_or(false);
    let nf = kvf(&ev, "evaluable_folds")?;
    let (a, b, c) = (kvf(&ev, "mae_d_a")?, kvf(&ev, "mae_d_b")?, kvf(&ev, "mae_d_c1")?);
    let (alo, ahi) = (kvf(&ev, "diff_d_a_minus_c1_lo")?, kvf(&ev, "diff_d_a_minus_c1_hi")?);
    let (blo, bhi) = (kvf(&ev, "diff_d_b_minus_c1_lo")?, kvf(&ev, "diff_d_b_minus_c1_hi")?);
    let cover = kvf(&ev, "cover_c1")?;
    let real3 = kvf(&nu, "real3_mae_d_c1")?;
    let q025 = kvf(&nu, "null_q025")?;
    let valid = selftest_ok && planted_ok && nf >= 5.0;
    let beat_a = c < a && alo > 0.0;
    let beat_b = c < b && blo > 0.0;
    let null_ok = real3 < q025;
    let cov_ok = (0.85..=0.95).contains(&cover);
    let regress = (c > a && ahi < 0.0) || (c > b && bhi < 0.0);
    println!("CONDITION selftest_ok={} planted_ok={} evaluable_folds={}", selftest_ok, planted_ok, nf);
    println!("CONDITION macro_mae_d a={:.4} b={:.4} c1={:.4}", a, b, c);
    println!("CONDITION beat_a={} (diff a-c1 interval [{:.4}, {:.4}])", beat_a, alo, ahi);
    println!("CONDITION beat_b={} (diff b-c1 interval [{:.4}, {:.4}])", beat_b, blo, bhi);
    println!("CONDITION null_excluded={} (real3 {:.4}, null q025 {:.4})", null_ok, real3, q025);
    println!("CONDITION cover_c1={:.4} in_band={}", cover, cov_ok);
    let word = if !valid {
        "INCONCLUSIVE"
    } else if beat_a && beat_b && null_ok {
        if cov_ok {
            "ADVANCES"
        } else {
            "TRADES"
        }
    } else if regress {
        "REGRESSES"
    } else {
        "MATCHES"
    };
    println!("VERDICT {} (primitive)", word);
    Ok(())
}

// ----------------------------------------------------------------------------
// Selftest
// ----------------------------------------------------------------------------

fn check(kv: &mut Kv, ok: &mut bool, name: &str, pass: bool, detail: String) {
    println!("CHECK {} {} {}", name, if pass { "PASS" } else { "FAIL" }, detail);
    kv.put(&format!("check_{}", name), if pass { "1" } else { "0" });
    if !pass {
        *ok = false;
    }
}

fn synth_f(x: &[f64]) -> f64 {
    3.0 * (x[0] > 0.5) as u8 as f64 + 2.0 * x[1] + (x[2] * 6.0).floor()
}

fn cmd_selftest(args: &[String]) -> Result<(), String> {
    let raw = arg(args, "--raw").unwrap_or_else(|| "/dev/null".to_string());
    let mut kv = Kv::new();
    let mut ok = true;
    // sha256
    check(&mut kv, &mut ok, "sha256_empty", sha256(b"") == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", String::new());
    check(&mut kv, &mut ok, "sha256_abc", sha256(b"abc") == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad", String::new());
    let long = vec![b'a'; 1000];
    check(&mut kv, &mut ok, "sha256_1000a", sha256(&long) == "41edece42d63e8d9bf515a9ba6932e1c20cbc9f5a5d134645adb5db1b9737ea3", String::new());
    // json
    let j = parse_json("{\"a\": [1, -2.5e1, null, true], \"b\": {\"c\": \"x\\\"y\\u0041\"}, \"d\": 12}").map_err(|e| e.to_string())?;
    let jok = j.get("a").and_then(|x| x.arr()).map(|v| v.len() == 4 && v[1].f() == Some(-25.0) && v[2].is_null() && v[3].b() == Some(true)).unwrap_or(false)
        && j.get("b").and_then(|b| b.get("c")).and_then(|c| c.s()) == Some("x\"yA")
        && j.get("d").and_then(|x| x.f()) == Some(12.0);
    check(&mut kv, &mut ok, "json", jok, String::new());
    // semi-regular known answers (committed reference values)
    let known: [(i64, i64, i64, i64); 13] = [
        (3, 10, 5, 7),
        (3, 11, 5, 7),
        (3, 13, 5, 7),
        (3, 15, 5, 8),
        (4, 4, 1, 5),
        (4, 5, 1, 5),
        (4, 6, 1, 5),
        (4, 5, 2, 6),
        (4, 6, 2, 7),
        (4, 7, 2, 7),
        (4, 8, 2, 7),
        (4, 9, 3, 8),
        (4, 10, 3, 9),
    ];
    let mut bad = Vec::new();
    for (m, n, l, e) in known.iter() {
        let g = dreg(*m, *n, *l);
        if g != *e {
            bad.push(format!("({},{},{}) got {} want {}", m, n, l, g, e));
        }
    }
    check(&mut kv, &mut ok, "dreg_known_answers", bad.is_empty(), format!("{} values, mismatches: {:?}", known.len(), bad));
    let c42 = (0..=4).map(|i| binom(42, i)).sum::<u128>();
    check(&mut kv, &mut ok, "columns_124314", c42 == 124314, format!("{}", c42));
    check(&mut kv, &mut ok, "logcols", (logcols(42, 4) - 124314f64.log2()).abs() < 1e-9, String::new());
    // quantile and spearman
    let mut v: Vec<f64> = (1..=9).map(|x| x as f64).collect();
    check(&mut kv, &mut ok, "conformal_quantile", quantile_k(&mut v, 0.9) == 9.0, "n=9: ceil(0.9*10)=9".into());
    let mut v2: Vec<f64> = (1..=19).map(|x| x as f64).collect();
    check(&mut kv, &mut ok, "conformal_quantile19", quantile_k(&mut v2, 0.9) == 18.0, "n=19: ceil(0.9*20)=18".into());
    let sp1 = spearman(&[1.0, 2.0, 3.0, 4.0], &[10.0, 20.0, 30.0, 40.0]);
    let sp2 = spearman(&[1.0, 2.0, 3.0, 4.0], &[4.0, 3.0, 2.0, 1.0]);
    let sp3 = spearman(&[1.0, 1.0, 1.0], &[1.0, 2.0, 3.0]);
    check(&mut kv, &mut ok, "spearman", (sp1 - 1.0).abs() < 1e-12 && (sp2 + 1.0).abs() < 1e-12 && sp3 == 0.0, format!("{} {} {}", sp1, sp2, sp3));
    // gbt fits a planted function; linear cannot
    let mut rng = Rng::new(5);
    let mk_items = |n: usize, noise: f64, rng: &mut Rng| -> Vec<Item> {
        (0..n)
            .map(|k| {
                let x: Vec<f64> = vec![rng.unif(), rng.unif(), rng.unif()];
                let y = synth_f(&x) + noise * rng.normal();
                let mut x1 = vec![0.0; 9];
                x1[0] = x[0];
                x1[1] = x[1];
                x1[2] = x[2];
                x1[3] = k as f64 * 1e-9; // unique key per item
                Item { fam: "s".into(), cell: format!("c{}", k), x1, y, cens: false, eng: 0, nn: 10, m: 3, n: 5, l: 2, sat: false, dreg: 5.0, a_pred: 5.0 }
            })
            .collect()
    };
    let tr = mk_items(600, 0.0, &mut rng);
    let te = mk_items(400, 0.0, &mut rng);
    let trr: Vec<&Item> = tr.iter().collect();
    let g = fit_gbt(&trr, false);
    let mae_g: f64 = te.iter().map(|i| (gbt_predict(&g, i) - i.y).abs()).sum::<f64>() / te.len() as f64;
    let mut yv: Vec<f64> = tr.iter().map(|i| i.y).collect();
    let med = median(&mut yv);
    let mae_c: f64 = te.iter().map(|i| (med - i.y).abs()).sum::<f64>() / te.len() as f64;
    check(&mut kv, &mut ok, "gbt_planted_function", mae_g < 0.35 * mae_c, format!("gbt mae {:.3} vs constant mae {:.3}", mae_g, mae_c));
    // conformal coverage on iid synthetic data
    let mut covs = Vec::new();
    for rep in 0..200u64 {
        let mut r = Rng::new(1000 + rep);
        let all = mk_items(500, 0.5, &mut r);
        let proper: Vec<&Item> = all.iter().take(200).collect();
        let g = fit_gbt(&proper, false);
        let mut res: Vec<f64> = all[200..300].iter().map(|i| (i.y - gbt_predict(&g, i)).abs()).collect();
        let q = quantile_k(&mut res, 0.9);
        let cov = all[300..].iter().filter(|i| (i.y - gbt_predict(&g, i)).abs() <= q).count() as f64 / 200.0;
        covs.push(cov);
    }
    let mc = covs.iter().sum::<f64>() / covs.len() as f64;
    check(&mut kv, &mut ok, "conformal_coverage_iid", (0.87..=0.93).contains(&mc), format!("mean coverage {:.4} over 200 repetitions", mc));
    // linear model recovers an exact linear law
    let lin_items: Vec<Item> = (0..40)
        .map(|k| {
            let (m, n, l) = (3 + (k % 2) as i64, 4 + (k % 7) as i64, 1 + (k % 5) as i64);
            let mut it = Item { fam: "s".into(), cell: format!("c{}", k), x1: vec![0.0; 9], y: 0.0, cens: false, eng: (k % 3) as u8, nn: 10, m, n, l, sat: false, dreg: 5.0, a_pred: 5.0 };
            it.y = 1.0 + 0.5 * m as f64 + 0.25 * l as f64 + 0.125 * n as f64 + 0.01 * (it.eng == 1) as u8 as f64;
            it
        })
        .collect();
    let lr: Vec<&Item> = lin_items.iter().collect();
    let b = fit_lin(&lr);
    let err = lin_items.iter().map(|i| (lin_feats(i).iter().zip(b.iter()).map(|(x, c)| x * c).sum::<f64>() - i.y).abs()).fold(0.0, f64::max);
    check(&mut kv, &mut ok, "ridge_recovers_linear_law", err < 0.02, format!("max abs err {:.5}", err));
    // harvest parsers on fixed fake lines (one of each source kind)
    let mut a = Audit::default();
    let l1 = parse_json("{\"cell\":\"n10l5\",\"n\":10,\"ell\":5,\"n_vars\":25,\"v_basis\":[1,2],\"x_r\":3,\"outcome\":{\"kind\":\"resolved\",\"degree\":7,\"refuted\":true}}").unwrap();
    let r1 = harvest_obj("xl", &l1, &mut a);
    let l2 = parse_json("{\"m\":3,\"n\":10,\"ell\":2,\"draw\":0,\"n_vars\":16,\"source\":\"s\",\"cell\":\"c\",\"committed_outcome\":{\"kind\":\"resolved\",\"degree\":5},\"measured_to\":4,\"scan\":[{\"degree\":3,\"closure_refuted\":false},{\"degree\":4,\"closure_refuted\":true}]}").unwrap();
    let r2 = harvest_obj("closure", &l2, &mut a);
    let l3 = parse_json("{\"export\":{\"m\":4,\"n\":11,\"ell\":4,\"n_vars\":38,\"solutions\":0,\"v_basis\":[1],\"x_r\":2},\"exit\":0,\"d_f4\":4,\"gb_is_one\":true,\"max_matrix\":[100,1024]}").unwrap();
    let r3 = harvest_obj("f4", &l3, &mut a);
    let l4 = parse_json("{\"export\":{\"m\":4,\"n\":13,\"ell\":5,\"n_vars\":46,\"solutions\":0,\"v_basis\":[1],\"x_r\":2},\"exit\":-11,\"d_f4\":4,\"gb_is_one\":false,\"max_matrix\":[100,1024]}").unwrap();
    let r4 = harvest_obj("f4", &l4, &mut a);
    let ok_h = r1.map(|r| r.d == 7.0 && !r.cens && r.m == 3 && r.eng == 0).unwrap_or(false)
        && r2.map(|r| r.d == 4.0 && !r.cens && r.eng == 1).unwrap_or(false)
        && r3.map(|r| r.d == 4.0 && !r.cens && r.cols == Some(10.0) && r.eng == 2).unwrap_or(false)
        && r4.map(|r| r.d == 4.0 && r.cens && r.cols.is_none()).unwrap_or(false);
    check(&mut kv, &mut ok, "harvest_parsers", ok_h, String::new());
    kv.put("selftest_ok", if ok { "1" } else { "0" });
    if raw != "/dev/null" {
        kv.write(&raw, "selftest.kv")?;
    }
    println!("SELFTEST {}", if ok { "OK" } else { "FAILED" });
    if ok {
        Ok(())
    } else {
        Err("selftest failed".into())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).cloned().unwrap_or_default();
    let r = match cmd.as_str() {
        "selftest" => cmd_selftest(&args),
        "harvest" => cmd_harvest(&args),
        "evaluate" => cmd_evaluate(&args),
        "planted" => cmd_planted(&args),
        "nullperm" => cmd_null(&args),
        "t3" => cmd_t3(&args),
        "rank" => cmd_rank(&args),
        "verdict" => cmd_verdict(&args),
        _ => Err("usage: selftest|harvest|evaluate|planted|nullperm|t3|rank|verdict".to_string()),
    };
    if let Err(e) = r {
        eprintln!("ERROR {}", e);
        std::process::exit(1);
    }
}
