#![allow(dead_code)]
use std::collections::BTreeMap;
use std::process::exit;
// ------------------------------------------------------------------ JSON

#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, k: &str) -> Option<&Json> {
        match self {
            Json::Obj(v) => v.iter().find(|(kk, _)| kk == k).map(|(_, x)| x),
            _ => None,
        }
    }
    fn at(&self, i: usize) -> Option<&Json> {
        match self {
            Json::Arr(v) => v.get(i),
            _ => None,
        }
    }
    fn path(&self, p: &[&str]) -> Option<&Json> {
        let mut cur = self;
        for k in p {
            cur = match k.parse::<usize>() {
                Ok(i) if matches!(cur, Json::Arr(_)) => cur.at(i)?,
                _ => cur.get(k)?,
            };
        }
        Some(cur)
    }
    fn str(&self) -> Option<&str> {
        if let Json::Str(s) = self { Some(s) } else { None }
    }
    fn arr(&self) -> Option<&Vec<Json>> {
        if let Json::Arr(v) = self { Some(v) } else { None }
    }
    fn u128(&self) -> Option<u128> {
        match self {
            Json::Num(s) => s.parse().ok(),
            Json::Str(s) => parse_int(s),
            _ => None,
        }
    }
    fn i128(&self) -> Option<i128> {
        match self {
            Json::Num(s) | Json::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
    fn bool(&self) -> Option<bool> {
        if let Json::Bool(b) = self { Some(*b) } else { None }
    }
}

fn parse_int(s: &str) -> Option<u128> {
    if let Some(h) = s.strip_prefix("0x") {
        u128::from_str_radix(h, 16).ok()
    } else {
        s.parse().ok()
    }
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\n' | b'\r' | b'\t') {
            self.i += 1;
        }
    }
    fn err<T>(&self, m: &str) -> Result<T, String> {
        Err(format!("JSON: {m} at byte {}", self.i))
    }
    fn expect(&mut self, c: u8) -> Result<(), String> {
        self.ws();
        if self.b.get(self.i) == Some(&c) {
            self.i += 1;
            Ok(())
        } else {
            self.err(&format!("expected '{}'", c as char))
        }
    }
    fn lit(&mut self, s: &str, v: Json) -> Result<Json, String> {
        if self.b[self.i..].starts_with(s.as_bytes()) {
            self.i += s.len();
            Ok(v)
        } else {
            self.err("bad literal")
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let h = self.b.get(self.i..self.i + 4).ok_or("JSON: short \\u escape")?;
        self.i += 4;
        u32::from_str_radix(std::str::from_utf8(h).map_err(|e| e.to_string())?, 16).map_err(|e| e.to_string())
    }
    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = *self.b.get(self.i).ok_or("JSON: unterminated string")?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or("JSON: bad escape")?;
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
                            if (0xD800..0xDC00).contains(&cp) && self.b[self.i..].starts_with(b"\\u") {
                                self.i += 2;
                                let lo = self.hex4()?;
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo.wrapping_sub(0xDC00) & 0x3FF);
                            }
                            let ch = char::from_u32(cp).unwrap_or('\u{FFFD}');
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return self.err("bad escape"),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|e| e.to_string())
    }
    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self.b.get(self.i) {
            Some(b'{') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(Json::Obj(v));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.expect(b':')?;
                    let x = self.value()?;
                    v.push((k, x));
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(Json::Obj(v));
                        }
                        _ => return self.err("expected ',' or '}'"),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(Json::Arr(v));
                }
                loop {
                    v.push(self.value()?);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(Json::Arr(v));
                        }
                        _ => return self.err("expected ',' or ']'"),
                    }
                }
            }
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => self.lit("true", Json::Bool(true)),
            Some(b'f') => self.lit("false", Json::Bool(false)),
            Some(b'n') => self.lit("null", Json::Null),
            Some(_) => {
                let s = self.i;
                while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                if s == self.i {
                    return self.err("unexpected byte");
                }
                Ok(Json::Num(String::from_utf8_lossy(&self.b[s..self.i]).into_owned()))
            }
            None => self.err("unexpected end"),
        }
    }
}

fn load(path: &str) -> Json {
    let b = std::fs::read(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut p = Parser { b: &b, i: 0 };
    let v = p.value().unwrap_or_else(|e| die(&format!("{path}: {e}")));
    p.ws();
    if p.i != b.len() {
        die(&format!("{path}: trailing data"));
    }
    v
}

fn die(m: &str) -> ! {
    eprintln!("verify_scalars: {m}");
    exit(2)
}

fn jstr(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}


// ------------------------------------------------------------------ iid seed sensitivity (native)
// 500 independent SplitMix64 seeds x 2000 iid resamples of the 2026-10-29 per-round
// ratios, plus one 400,000-resample reference; percentile bounds [50] and [1949]
// per 2000 (and the 2.5% / 97.5% order statistics for the reference).
// Usage: seeds <study dir>

fn f64v(j: &Json) -> f64 {
    match j { Json::Num(s) => s.parse().unwrap(), _ => die("not a number") }
}
fn med(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 }
}
fn per(p: &str) -> (f64, f64) {
    let d = load(p);
    let reps = d.get("repetitions").and_then(|v| v.arr()).unwrap();
    let mut c: Vec<f64> = reps.iter().map(|r| f64v(r.path(&["phases_ns", "collect"]).unwrap())).collect();
    let mut t: Vec<f64> = reps.iter().map(|r| f64v(r.get("total_ns").unwrap())).collect();
    (med(&mut c), med(&mut t))
}
struct SM(u64);
impl SM {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        ((self.next() >> 11) as f64 / (1u64 << 53) as f64 * n as f64) as usize
    }
}
fn interval(xs: &[f64], rng: &mut SM, b: usize) -> (f64, f64) {
    let n = xs.len();
    let mut meds = Vec::with_capacity(b);
    let mut buf = vec![0f64; n];
    for _ in 0..b {
        for j in 0..n { buf[j] = xs[rng.below(n)]; }
        meds.push(med(&mut buf));
    }
    meds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if b == 2000 { (meds[50], meds[1949]) } else { (meds[b * 25 / 1000], meds[b * 975 / 1000 - 1]) }
}
fn main() {
    let study = std::env::args().nth(1).unwrap();
    let aa2 = format!("{study}/aa2_20261029/runs");
    let rows: Vec<((f64, f64), (f64, f64), (f64, f64))> = (1..=42)
        .map(|r| (per(&format!("{aa2}/a-r{r}.json")), per(&format!("{aa2}/q-r{r}.json")), per(&format!("{aa2}/c-r{r}.json"))))
        .collect();
    let abc: Vec<f64> = rows.iter().map(|(a, _, c)| a.0 / c.0).collect();
    let aac: Vec<f64> = rows.iter().map(|(a, q, _)| a.0 / q.0).collect();
    let abt: Vec<f64> = rows.iter().map(|(a, _, c)| a.1 / c.1).collect();
    let aat: Vec<f64> = rows.iter().map(|(a, q, _)| a.1 / q.1).collect();
    let mut est = [0usize; 2];
    let mut rng_lo = [[f64::MAX, f64::MIN]; 4];
    let mut rng_hi = [[f64::MAX, f64::MIN]; 4];
    for seed in 0..500u64 {
        let mut rng = SM(0xA1A2_2026_1029_0000 ^ seed);
        let iv: Vec<(f64, f64)> = [&abc, &aac, &abt, &aat].iter().map(|x| interval(x, &mut rng, 2000)).collect();
        for k in 0..4 {
            rng_lo[k] = [rng_lo[k][0].min(iv[k].0), rng_lo[k][1].max(iv[k].0)];
            rng_hi[k] = [rng_hi[k][0].min(iv[k].1), rng_hi[k][1].max(iv[k].1)];
        }
        if iv[0].0 > 1.0 && iv[0].0 > iv[1].1 { est[0] += 1; }
        if iv[2].0 > 1.0 && iv[2].0 > iv[3].1 { est[1] += 1; }
    }
    let mut rng = SM(0x5EED_0400_0000);
    let refs: Vec<(f64, f64)> = [&abc, &aac, &abt, &aat].iter().map(|x| interval(x, &mut rng, 400_000)).collect();
    let names = ["ab_collect", "aa_collect", "ab_total", "aa_total"];
    let mut o = Vec::new();
    for k in 0..4 {
        o.push(format!("\"{}\": {{\"lo_range_500_seeds\": [{:?}, {:?}], \"hi_range_500_seeds\": [{:?}, {:?}], \"reference_400k\": [{:?}, {:?}]}}",
            names[k], rng_lo[k][0], rng_lo[k][1], rng_hi[k][0], rng_hi[k][1], refs[k].0, refs[k].1));
    }
    println!("{{\"generator\": \"SplitMix64, iid draws, index floor(u * n)\", \"seeds\": 500, \"resamples\": 2000, \"established_collect_seeds\": {}, \"established_total_seeds\": {}, {}}}",
             est[0], est[1], o.join(", "));
}
