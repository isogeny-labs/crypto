//! Native validator for XP-20261029-stride-renew-prime (P20, P28).
//!
//! Replaces the legacy Python execution paths in `validation/`:
//!
//!   srenew-validate recompute <session_dir> <label>          (was recompute.py)
//!   srenew-validate mechanism <session_dir>                  (was mechanism.py)
//!   srenew-validate gate <study_dir> <band_dir> <val_id> --recompute-dir D --out O
//!                                                            (was build_gate_files.py)
//!   srenew-validate cmpjson <a.json> <b.json> <rel_tol>      (numeric comparison)
//!
//! The contracts are the legacy scripts' contracts, unchanged: the same
//! inputs, the same derived quantities, the same bootstrap (Python's
//! MT19937 and `randrange` reproduced bit for bit, seed 29102026, 4000
//! resamples), Python's compensated `sum()`, and the same JSON layout
//! (Python `json.dumps` float repr, indent, key order and ASCII escaping),
//! so outputs can be byte-compared with the committed evidence.
//! Elliptic-curve arithmetic is this file's own affine arithmetic; nothing
//! is shared with ecbench.

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const WKEYS: [&str; 3] = ["0.0", "0.1", "1.0"];
const PAIRS: [(&str, &str); 5] = [
    ("walk", "srenew"),
    ("walk", "walk-aa"),
    ("rho-neg", "rho-neg-aa"),
    ("rho-neg", "walk"),
    ("rho-neg", "srenew"),
];
const SEED: u32 = 29102026;
const RESAMPLES: usize = 4000;
const LO_IDX: usize = 100; // int(0.025 * 4000)
const HI_IDX: usize = 3899; // int(0.975 * 4000) - 1

// ------------------------------------------------------------ Python JSON
fn py_float(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    let s = format!("{:e}", x); // shortest round-trip digits
    let (neg, s) = match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s.as_str()),
    };
    let (mant, exp) = s.split_once('e').unwrap();
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let e: i32 = exp.parse().unwrap();
    let decpt = e + 1;
    let n = digits.len() as i32;
    let body = if decpt > -4 && decpt <= 16 {
        if decpt <= 0 {
            format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
        } else if decpt >= n {
            format!("{}{}.0", digits, "0".repeat((decpt - n) as usize))
        } else {
            format!("{}.{}", &digits[..decpt as usize], &digits[decpt as usize..])
        }
    } else {
        let m = if n > 1 { format!("{}.{}", &digits[..1], &digits[1..]) } else { digits.clone() };
        let ee = decpt - 1;
        format!("{}e{}{:02}", m, if ee < 0 { '-' } else { '+' }, ee.abs())
    };
    if neg { format!("-{}", body) } else { body }
}

fn py_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{:04x}", u));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn py_num(n: &serde_json::Number) -> String {
    if let Some(u) = n.as_u64() {
        u.to_string()
    } else if let Some(i) = n.as_i64() {
        i.to_string()
    } else {
        py_float(n.as_f64().unwrap())
    }
}

/// `json.dumps(v, indent=indent, sort_keys=sort)`; indent None = compact
/// separators (",", ":").
fn dumps(v: &Value, indent: Option<usize>, sort: bool) -> String {
    let mut out = String::new();
    dump_into(v, indent, sort, 0, &mut out);
    out
}

fn dump_into(v: &Value, indent: Option<usize>, sort: bool, lvl: usize, out: &mut String) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&py_num(n)),
        Value::String(s) => py_str(s, out),
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, x) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                if let Some(w) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat(w * (lvl + 1)));
                }
                dump_into(x, indent, sort, lvl + 1, out);
            }
            if let Some(w) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(w * lvl));
            }
            out.push(']');
        }
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut ks: Vec<&String> = m.keys().collect();
            if sort {
                ks.sort();
            }
            out.push('{');
            for (i, k) in ks.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                if let Some(w) = indent {
                    out.push('\n');
                    out.push_str(&" ".repeat(w * (lvl + 1)));
                }
                py_str(k, out);
                out.push_str(if indent.is_some() { ": " } else { ":" });
                dump_into(&m[*k], indent, sort, lvl + 1, out);
            }
            if let Some(w) = indent {
                out.push('\n');
                out.push_str(&" ".repeat(w * lvl));
            }
            out.push('}');
        }
    }
}

/// Python `==` on JSON values (1 == 1.0; dict equality ignores order).
fn py_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (x.as_i64(), y.as_i64(), x.as_u64(), y.as_u64()) {
            (Some(p), Some(q), _, _) => p == q,
            (_, _, Some(p), Some(q)) => p == q,
            _ => x.as_f64().unwrap() == y.as_f64().unwrap(),
        },
        (Value::Bool(x), Value::Number(n)) | (Value::Number(n), Value::Bool(x)) => {
            n.as_f64().unwrap() == if *x { 1.0 } else { 0.0 }
        }
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| py_eq(p, q)),
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).map_or(false, |w| py_eq(v, w)))
        }
        _ => a == b,
    }
}

/// Python >= 3.12 `sum()` over floats: Neumaier compensated summation.
fn py_sum<I: IntoIterator<Item = f64>>(it: I) -> f64 {
    let (mut s, mut c) = (0.0f64, 0.0f64);
    for x in it {
        let t = s + x;
        if s.abs() >= x.abs() {
            c += (s - t) + x;
        } else {
            c += (x - t) + s;
        }
        s = t;
    }
    if c != 0.0 && c.is_finite() {
        s += c;
    }
    s
}

fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"))
}

fn load(path: &str) -> Value {
    let s = fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&s).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn load_jsonl(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn sha(path: &str) -> String {
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect()
}

// ------------------------------------------------------------ Python random
/// MT19937 seeded and drawn exactly as CPython's `random.Random(seed)`.
struct PyRandom {
    mt: [u32; 624],
    idx: usize,
}

impl PyRandom {
    fn new(seed: u32) -> Self {
        let mut r = PyRandom { mt: [0; 624], idx: 624 };
        r.mt[0] = 19650218;
        for i in 1..624 {
            let p = r.mt[i - 1];
            r.mt[i] = 1812433253u32.wrapping_mul(p ^ (p >> 30)).wrapping_add(i as u32);
        }
        let key = [seed];
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..624.max(key.len()) {
            let p = r.mt[i - 1];
            r.mt[i] = (r.mt[i] ^ (p ^ (p >> 30)).wrapping_mul(1664525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= 624 {
                r.mt[0] = r.mt[623];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..623 {
            let p = r.mt[i - 1];
            r.mt[i] = (r.mt[i] ^ (p ^ (p >> 30)).wrapping_mul(1566083941)).wrapping_sub(i as u32);
            i += 1;
            if i >= 624 {
                r.mt[0] = r.mt[623];
                i = 1;
            }
        }
        r.mt[0] = 0x8000_0000;
        r
    }

    fn next_u32(&mut self) -> u32 {
        if self.idx >= 624 {
            for k in 0..624 {
                let y = (self.mt[k] & 0x8000_0000) | (self.mt[(k + 1) % 624] & 0x7fff_ffff);
                let mut v = self.mt[(k + 397) % 624] ^ (y >> 1);
                if y & 1 != 0 {
                    v ^= 0x9908_b0df;
                }
                self.mt[k] = v;
            }
            self.idx = 0;
        }
        let mut y = self.mt[self.idx];
        self.idx += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    /// `randrange(n)` = `_randbelow_with_getrandbits(n)` for n < 2^32.
    fn randrange(&mut self, n: usize) -> usize {
        let k = usize::BITS - n.leading_zeros();
        assert!(k >= 1 && k <= 32);
        loop {
            let r = (self.next_u32() >> (32 - k)) as usize;
            if r < n {
                return r;
            }
        }
    }
}

// ------------------------------------------------------------ arithmetic
fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    a * b % m // operands < 2^64
}

fn powmod(mut b: u128, mut e: u128, m: u128) -> u128 {
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

fn inv(a: i128, p: i128) -> i128 {
    let (mut r0, mut r1) = (a.rem_euclid(p), p);
    let (mut s0, mut s1) = (1i128, 0i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
    }
    assert_eq!(r0, 1, "not invertible");
    s0.rem_euclid(p)
}

fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    let small = [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for q in small {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for a in small {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn isqrt(n: i128) -> i128 {
    let mut x = (n as f64).sqrt() as i128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

type Pt = Option<(i128, i128)>;

#[derive(Clone, Copy)]
struct Curve {
    p: i128,
    a: i128,
    b: i128,
}

impl Curve {
    fn on(&self, q: Pt) -> bool {
        match q {
            None => true,
            Some((x, y)) => {
                let p = self.p;
                (y * y % p - ((x * x % p) * x % p + self.a * x % p + self.b)).rem_euclid(p) == 0
            }
        }
    }
    fn add(&self, a: Pt, b: Pt) -> Pt {
        let p = self.p;
        let (x1, y1) = match a { None => return b, Some(v) => v };
        let (x2, y2) = match b { None => return a, Some(v) => v };
        let lam = if x1 == x2 {
            if (y1 + y2).rem_euclid(p) == 0 {
                return None;
            }
            (3 * x1 % p * x1 % p + self.a) % p * inv(2 * y1, p) % p
        } else {
            (y2 - y1).rem_euclid(p) * inv(x2 - x1, p) % p
        };
        let x3 = (lam * lam - x1 - x2).rem_euclid(p);
        Some((x3, (lam * (x1 - x3) - y1).rem_euclid(p)))
    }
    fn mul(&self, mut k: u128, q: Pt) -> Pt {
        let (mut r, mut a) = (None, q);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, a);
            }
            a = self.add(a, a);
            k >>= 1;
        }
        r
    }
}

fn hex(v: &Value) -> i128 {
    let s = v.as_str().unwrap();
    i128::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}

fn pt(v: &Value) -> Pt {
    Some((hex(&v[0]), hex(&v[1])))
}

fn int_of(v: &Value) -> i128 {
    match v {
        Value::String(s) => s.trim().parse().unwrap(),
        Value::Number(n) => n.as_i64().map(|x| x as i128).unwrap_or_else(|| n.as_u64().unwrap() as i128),
        _ => panic!("not an int: {v}"),
    }
}

// ------------------------------------------------------------ recompute
fn unpriced_count(rec: &Value) -> (f64, Map<String, Value>) {
    let mut total = 0f64;
    let mut detail = Map::new();
    let empty = Map::new();
    let counters = rec.get("counters").and_then(|c| c.as_object()).unwrap_or(&empty);
    for name in rec["cost"].get("unpriced").and_then(|u| u.as_array()).cloned().unwrap_or_default() {
        let name = name.as_str().unwrap().to_string();
        let base = name.strip_suffix("_uncharged").unwrap_or(&name).to_string();
        if let Some(v) = counters.get(&name) {
            total += f(v);
            detail.insert(name, v.clone());
            continue;
        }
        let mut found = false;
        let mut sub = 0i64;
        for ph in rec["phases"].as_array().unwrap() {
            let nat = ph.get("native").and_then(|n| n.as_object()).unwrap_or(&empty);
            if let Some(v) = nat.get(&name) {
                sub += v.as_i64().unwrap();
                found = true;
            } else if let Some(v) = nat.get(&base) {
                sub += v.as_i64().unwrap();
                found = true;
            }
        }
        if !found {
            eprintln!("{}: unpriced {} has no count", rec["record_id"].as_str().unwrap(), name);
            std::process::exit(1);
        }
        detail.insert(name, json!(sub));
        total += sub as f64;
    }
    (total, detail)
}

fn phase<'a>(rec: &'a Value, name: &str) -> Option<&'a Value> {
    rec["phases"].as_array().unwrap().iter().find(|p| p["name"] == name)
}

type Key = (String, i64);

struct Meas<'a> {
    g: f64,
    u: f64,
    r: f64,
    slug: String,
    seed: Value,
    s_rec: f64,
    rec: &'a Value,
}

/// Python dict with insertion order: Vec of keys plus a map.
struct Ordered<'a> {
    order: Vec<Key>,
    map: HashMap<Key, Meas<'a>>,
}

fn s_of(v: &Meas, w: f64) -> f64 {
    (v.g + w * v.u) / v.r.sqrt()
}

fn recompute(sdir: &str, label: &str) -> Value {
    let plan = load(&format!("{sdir}/plan.json"));
    let recs = load_jsonl(&format!("{sdir}/records.jsonl"));
    let mut out = Map::new();
    out.insert("label".into(), json!(label));
    out.insert("session_dir".into(), json!(sdir));
    out.insert("records".into(), json!(recs.len()));
    let sids: BTreeSet<String> = recs.iter().map(|r| r["session_id"].as_str().unwrap().to_string()).collect();
    out.insert("session_ids".into(), json!(sids));
    let bins: BTreeSet<String> = recs.iter().map(|r| r["binary_sha256"].as_str().unwrap().to_string()).collect();
    out.insert("binary_sha256".into(), json!(bins));
    let mut iso: BTreeMap<String, u64> = BTreeMap::new();
    for r in &recs {
        *iso.entry(r["isolation"]["level"].as_str().unwrap().to_string()).or_default() += 1;
    }
    out.insert("isolation_levels".into(), json!(iso));

    // ---- curves from plan.json, independent EC checks
    struct C {
        p: i128,
        t: i128,
        r: i128,
        g: Pt,
        targets: Vec<(String, Pt)>,
    }
    let mut curve_order: Vec<String> = vec![];
    let mut curves: HashMap<String, C> = HashMap::new();
    for wl in plan["workloads"].as_array().unwrap() {
        let c = &wl["curve"];
        let slug = c["slug"].as_str().unwrap().to_string();
        let icv: Vec<&str> = c["icv1"].as_str().unwrap().split(':').collect();
        let p: i128 = icv[1].split('-').nth(1).unwrap().parse().unwrap();
        let t: i128 = icv[2].parse().unwrap();
        let r = int_of(&c["r"]);
        if !curves.contains_key(&slug) {
            curve_order.push(slug.clone());
            curves.insert(slug.clone(), C { p, t, r, g: pt(&c["generator"]), targets: vec![] });
        }
        let cur = curves.get_mut(&slug).unwrap();
        let wid = wl["workload_id"].as_str().unwrap().to_string();
        let q = pt(&wl["target"]);
        if let Some(e) = cur.targets.iter_mut().find(|e| e.0 == wid) {
            e.1 = q;
        } else {
            cur.targets.push((wid, q));
        }
    }
    let mut curve_checks = Map::new();
    let mut ec: HashMap<String, Curve> = HashMap::new();
    for slug in &curve_order {
        let c = &curves[slug];
        let (p, r) = (c.p, c.r);
        let mut pts: Vec<Pt> = vec![c.g];
        pts.extend(c.targets.iter().map(|t| t.1));
        let (x1, y1) = pts[0].unwrap();
        let (x2, y2) = pts.iter().find(|q| q.unwrap().0 != x1).unwrap().unwrap();
        let cube = |x: i128| x % p * x % p * x % p;
        let a = ((y1 * y1 % p - cube(x1)) - (y2 * y2 % p - cube(x2))).rem_euclid(p) * inv(x1 - x2, p) % p;
        let b = (y1 * y1 % p - cube(x1) - a * x1 % p).rem_euclid(p);
        let e = Curve { p, a, b };
        ec.insert(slug.clone(), e);
        let disc = (4 * cube(a) + 27 * (b * b % p)) % p;
        let j: Option<i128> = if disc != 0 { Some(1728 * 4 % p * cube(a) % p * inv(disc, p) % p) } else { None };
        let s = isqrt(p);
        let (lo, hi) = (p + 1 - 2 * s - 2, p + 1 + 2 * s + 2);
        let mut multiples = vec![];
        let mut m = r * (lo / r).max(1);
        while m <= hi {
            if lo <= m && m <= hi {
                multiples.push(m);
            }
            m += r;
        }
        curve_checks.insert(
            slug.clone(),
            json!({
                "p": p as i64, "a": a as i64, "b": b as i64, "r": r as i64, "trace": c.t as i64,
                "r_equals_p_plus_1_minus_t": r == p + 1 - c.t,
                "p_prime": is_prime(p as u128), "r_prime": is_prime(r as u128),
                "nonsingular": disc != 0, "j_invariant": j.map(|v| v as i64),
                "j_not_0_or_1728": !(j == Some(0) || j == Some(1728 % p)),
                "all_points_on_curve": pts.iter().all(|q| e.on(*q)),
                "rG_is_identity": e.mul(r as u128, c.g).is_none(),
                "r_only_multiple_in_hasse_interval": multiples == vec![r],
                "targets": c.targets.len(),
            }),
        );
    }

    // ---- completeness, pairing, priced S
    out.insert(
        "plan".into(),
        json!({"workloads": plan["workloads"].as_array().unwrap().len(),
               "executions": plan["executions"].as_array().unwrap().len(),
               "arms": plan["arms"].clone()}),
    );
    let mut by_arm: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for r in &recs {
        by_arm.entry(r["arm"].as_str().unwrap().to_string()).or_default().push(r);
    }
    let mut comp = Map::new();
    let mut meas: BTreeMap<String, Ordered> = BTreeMap::new();
    let mut dg_fail: Vec<String> = vec![];
    let mut dg_checked = 0u64;
    let mut calib = 0u64;
    for (arm, rs) in &by_arm {
        let m: Vec<&&Value> = rs.iter().filter(|x| !x["warmup"].as_bool().unwrap()).collect();
        let mut st: BTreeMap<String, u64> = BTreeMap::new();
        for x in &m {
            *st.entry(x["outcome"]["status"].as_str().unwrap().to_string()).or_default() += 1;
        }
        comp.insert(
            arm.clone(),
            json!({
                "records": rs.len(), "warmup": rs.len() - m.len(), "measured": m.len(),
                "statuses": st,
                "matches_target_true": m.iter().filter(|x| x["outcome"].get("matches_target") == Some(&Value::Bool(true))).count(),
                "censored": m.iter().filter(|x| x["outcome"]["status"] != "verified"
                    || x["cost"].get("total_gae").map_or(true, |v| v.is_null())).count(),
            }),
        );
        let entry = meas.entry(arm.clone()).or_insert_with(|| Ordered { order: vec![], map: HashMap::new() });
        for x in m {
            let x: &Value = x;
            if let Some(d) = x.get("detail").filter(|d| d.is_object() && !d.as_object().unwrap().is_empty()) {
                let truthy = match d.get("calibration_pinned_units") {
                    None | Some(Value::Null) | Some(Value::Bool(false)) => false,
                    Some(Value::Array(a)) => !a.is_empty(),
                    Some(Value::Object(o)) => !o.is_empty(),
                    Some(Value::String(s)) => !s.is_empty(),
                    Some(Value::Number(n)) => n.as_f64() != Some(0.0),
                    _ => true,
                };
                if truthy {
                    calib += 1;
                }
            }
            let wl = &x["workload"];
            let slug = wl["curve"]["slug"].as_str().unwrap().to_string();
            let e = ec[&slug];
            let g = pt(&wl["curve"]["generator"]);
            let q = pt(&wl["target"]);
            let d = int_of(&x["outcome"]["recovered"]);
            dg_checked += 1;
            let wid = wl["workload_id"].as_str().unwrap().to_string();
            let planned = curves[&slug].targets.iter().find(|t| t.0 == wid).unwrap().1;
            if e.mul(d as u128, g) != q || q != planned {
                dg_fail.push(x["record_id"].as_str().unwrap().to_string());
            }
            let (u, _) = unpriced_count(x);
            let key = (wid, x["round"].as_i64().unwrap());
            if !entry.map.contains_key(&key) {
                entry.order.push(key.clone());
            }
            entry.map.insert(
                key,
                Meas {
                    g: f(&x["cost"]["total_gae"]),
                    u,
                    r: int_of(&wl["curve"]["r"]) as f64,
                    slug,
                    seed: x["algorithm_seed"].clone(),
                    s_rec: f(&x["cost"]["s"]),
                    rec: x,
                },
            );
        }
    }
    out.insert("completeness".into(), Value::Object(comp));
    out.insert("records_with_pinned_calibration".into(), json!(calib));
    out.insert("dG_eq_Q".into(), json!({"checked": dg_checked, "failures": dg_fail}));
    out.insert("curves".into(), Value::Object(curve_checks));

    let mut s_dev = f64::NEG_INFINITY;
    for a in meas.values() {
        for k in &a.order {
            let v = &a.map[k];
            let gs = v.g / v.r.sqrt();
            s_dev = s_dev.max((v.s_rec - gs).abs() / gs);
        }
    }
    out.insert("record_s_vs_G_over_sqrt_r_max_rel_dev".into(), json!(s_dev));

    let first = meas.values().next().unwrap();
    let keys: BTreeSet<Key> = first.map.keys().cloned().collect();
    let same_keys = meas.values().all(|a| a.map.keys().cloned().collect::<BTreeSet<Key>>() == keys);
    let seed_mismatch = keys
        .iter()
        .filter(|k| {
            let seeds: Vec<&Value> = meas.values().map(|a| &a.map[*k].seed).collect();
            !seeds.iter().all(|s| py_eq(s, seeds[0]))
        })
        .count();
    let wls: BTreeSet<&String> = keys.iter().map(|k| &k.0).collect();
    let rounds: BTreeSet<i64> = keys.iter().map(|k| k.1).collect();
    out.insert(
        "pairing".into(),
        json!({"pairs_per_arm": keys.len(), "same_keys_all_arms": same_keys,
               "algorithm_seed_mismatches": seed_mismatch, "workloads": wls.len(), "rounds": rounds}),
    );

    let get = |arm: &str, k: &Key| -> &Meas { &meas[arm].map[k] };
    let mut diffs: BTreeSet<Vec<String>> = BTreeSet::new();
    let sval = |v: Option<&Value>| -> String {
        match v {
            Some(Value::String(s)) => s.clone(),
            None => panic!("param missing on one side: not representable as a sorted list"),
            Some(v) => panic!("non-string param {v}"),
        }
    };
    for k in &keys {
        let pa = get("walk", k).rec["method"]["params"].as_object().unwrap();
        let pb = get("srenew", k).rec["method"]["params"].as_object().unwrap();
        let mut all: BTreeSet<&String> = pa.keys().collect();
        all.extend(pb.keys());
        for kk in all {
            let (va, vb) = (pa.get(kk), pb.get(kk));
            let eq = match (va, vb) {
                (Some(x), Some(y)) => py_eq(x, y),
                (None, None) => true,
                _ => false,
            };
            if !eq {
                diffs.insert(vec![kk.clone(), sval(va), sval(vb)]);
            }
        }
        if get("walk", k).rec["method"]["id"] != get("srenew", k).rec["method"]["id"] {
            diffs.insert(vec!["method.id".into()]);
        }
    }
    out.insert("walk_vs_srenew_param_differences".into(), json!(diffs));
    let aa_params_equal = keys
        .iter()
        .all(|k| py_eq(&get("walk", k).rec["method"]["params"], &get("walk-aa", k).rec["method"]["params"]));
    out.insert("walk_aa_params_equal_walk".into(), json!(aa_params_equal));

    // ---- ratios and bootstrap
    let slug_of: HashMap<&String, &String> = keys.iter().map(|k| (&k.0, &get("walk", k).slug)).collect();
    let cluster_sums = |a: &str, b: &str, w: f64, filt: Option<&str>| -> BTreeMap<String, (f64, f64)> {
        let mut cs: BTreeMap<String, (f64, f64)> = BTreeMap::new();
        for k in &keys {
            if let Some(s) = filt {
                if slug_of[&k.0].as_str() != s {
                    continue;
                }
            }
            let e = cs.entry(k.0.clone()).or_insert((0.0, 0.0));
            e.0 += s_of(get(a, k), w);
            e.1 += s_of(get(b, k), w);
        }
        cs
    };
    let boot = |cs: &BTreeMap<String, (f64, f64)>, stratified: bool| -> [f64; 2] {
        let mut rng = PyRandom::new(SEED);
        let ids: Vec<&String> = cs.keys().collect();
        let mut strata: BTreeMap<&String, Vec<&String>> = BTreeMap::new();
        for i in &ids {
            strata.entry(slug_of[*i]).or_default().push(i);
        }
        let mut vals = Vec::with_capacity(RESAMPLES);
        for _ in 0..RESAMPLES {
            let (mut sa, mut sb) = (0.0f64, 0.0f64);
            if stratified {
                for members in strata.values() {
                    for _ in 0..members.len() {
                        let c = cs[members[rng.randrange(members.len())]];
                        sa += c.0;
                        sb += c.1;
                    }
                }
            } else {
                for _ in 0..ids.len() {
                    let c = cs[ids[rng.randrange(ids.len())]];
                    sa += c.0;
                    sb += c.1;
                }
            }
            vals.push(sb / sa);
        }
        vals.sort_by(|x, y| x.partial_cmp(y).unwrap());
        [vals[LO_IDX], vals[HI_IDX]]
    };
    let pairs_by_cluster = |a: &str, b: &str, w: f64| -> BTreeMap<String, Vec<(f64, f64)>> {
        let mut pc: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
        for k in &keys {
            pc.entry(k.0.clone()).or_default().push((s_of(get(a, k), w), s_of(get(b, k), w)));
        }
        pc
    };
    let boot_two_stage = |pc: &BTreeMap<String, Vec<(f64, f64)>>, seed: u32| -> [f64; 2] {
        let mut rng = PyRandom::new(seed);
        let ids: Vec<&String> = pc.keys().collect();
        let mut vals = Vec::with_capacity(RESAMPLES);
        for _ in 0..RESAMPLES {
            let (mut sa, mut sb) = (0.0f64, 0.0f64);
            for _ in 0..ids.len() {
                let members = &pc[ids[rng.randrange(ids.len())]];
                for _ in 0..members.len() {
                    let (x, y) = members[rng.randrange(members.len())];
                    sa += x;
                    sb += y;
                }
            }
            vals.push(sb / sa);
        }
        vals.sort_by(|x, y| x.partial_cmp(y).unwrap());
        [vals[LO_IDX], vals[HI_IDX]]
    };

    let n = keys.len() as f64;
    let mut ratios = Map::new();
    for (a, b) in PAIRS {
        let mut per_w = Map::new();
        for (w, wk) in WEIGHTS.iter().zip(WKEYS) {
            let cs = cluster_sums(a, b, *w, None);
            let tot_a = py_sum(cs.values().map(|c| c.0));
            let tot_b = py_sum(cs.values().map(|c| c.1));
            let mut e = Map::new();
            e.insert("ratio_b_over_a".into(), json!(tot_b / tot_a));
            e.insert("mean_s_a".into(), json!(tot_a / n));
            e.insert("mean_s_b".into(), json!(tot_b / n));
            e.insert("ci95_cluster_bootstrap".into(), json!(boot(&cs, false)));
            e.insert("ci95_cluster_bootstrap_stratified".into(), json!(boot(&cs, true)));
            let pc = pairs_by_cluster(a, b, *w);
            e.insert("ci95_two_stage_bootstrap".into(), json!(boot_two_stage(&pc, SEED)));
            if (a, b) == ("walk", "srenew") {
                let ups: Vec<f64> = (1..=10).map(|i| boot_two_stage(&pc, SEED + i)[1]).collect();
                e.insert(
                    "two_stage_upper_bound_over_10_more_seeds".into(),
                    json!({"min": ups.iter().cloned().fold(f64::INFINITY, f64::min),
                           "max": ups.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                           "all_below_1": ups.iter().all(|u| *u < 1.0)}),
                );
            }
            let mut per_curve = Map::new();
            let mut slugs: Vec<&String> = curve_order.iter().collect();
            slugs.sort();
            for slug in slugs {
                let csc = cluster_sums(a, b, *w, Some(slug));
                let sb = py_sum(csc.values().map(|c| c.1));
                let sa = py_sum(csc.values().map(|c| c.0));
                per_curve.insert(slug.clone(), json!(sb / sa));
            }
            e.insert("per_curve".into(), Value::Object(per_curve));
            per_w.insert(wk.into(), Value::Object(e));
        }
        ratios.insert(format!("{a}__{b}"), Value::Object(per_w));
    }
    out.insert("ratios".into(), Value::Object(ratios));

    // ---- A/A exactness
    let same = |x: &Value, y: &Value| -> bool {
        if !py_eq(&x["cost"]["total_gae"], &y["cost"]["total_gae"]) {
            return false;
        }
        if !py_eq(&Value::Object(unpriced_count(x).1), &Value::Object(unpriced_count(y).1)) {
            return false;
        }
        let ph = |r: &Value| -> Vec<Value> {
            r["phases"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| json!([p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"],
                                p.get("native").cloned().unwrap_or(Value::Null)]))
                .collect()
        };
        py_eq(&Value::Array(ph(x)), &Value::Array(ph(y)))
            && py_eq(&x["outcome"]["recovered"], &y["outcome"]["recovered"])
    };
    let cnt = |a: &str, b: &str| keys.iter().filter(|k| same(get(a, k).rec, get(b, k).rec)).count();
    out.insert(
        "aa_exact".into(),
        json!({"walk_aa_vs_walk_pairs_identical": cnt("walk", "walk-aa"),
               "rho_aa_vs_rho_pairs_identical": cnt("rho-neg", "rho-neg-aa"),
               "pairs": keys.len()}),
    );

    // ---- band ranks per curve
    let mut band = Map::new();
    let mut slugs: Vec<&String> = curve_order.iter().collect();
    slugs.sort();
    let mut means: HashMap<(String, String), [f64; 3]> = HashMap::new();
    for slug in &slugs {
        let mut bs = Map::new();
        for (arm, o) in &meas {
            let vals: Vec<&Meas> = o.order.iter().map(|k| &o.map[k]).filter(|v| &&v.slug == slug).collect();
            let mut ms = [0.0; 3];
            let mut wm = Map::new();
            for (i, w) in WEIGHTS.iter().enumerate() {
                ms[i] = py_sum(vals.iter().map(|v| s_of(v, *w))) / vals.len() as f64;
                wm.insert(WKEYS[i].into(), json!(ms[i]));
            }
            means.insert(((*slug).clone(), arm.clone()), ms);
            bs.insert(arm.clone(), Value::Object(wm));
        }
        let m = |arm: &str| means[&((*slug).clone(), arm.to_string())];
        bs.insert("srenew_lt_walk_every_weight".into(), json!((0..3).all(|i| m("srenew")[i] < m("walk")[i])));
        bs.insert("rho_lt_srenew_every_weight".into(), json!((0..3).all(|i| m("rho-neg")[i] < m("srenew")[i])));
        band.insert((*slug).clone(), Value::Object(bs));
    }
    out.insert("band_mean_s".into(), Value::Object(band));

    // ---- stride-renew accounting and walk accounting
    let mut acc: BTreeMap<&str, u64> = BTreeMap::new();
    for k in ["records", "scalar_mults_eq_4_plus_2_restarts", "rel_adds_ge_steps_plus_renewals_plus_restarts_plus_2",
              "phase_gae_eq_adds_plus_doubles", "total_gae_eq_sum_phase_gae", "renewals_ge_1_and_le_relations",
              "missing_stride_counters"] {
        acc.insert(k, 0);
    }
    let mut sums: BTreeMap<String, f64> = BTreeMap::new();
    let mut add = |k: &str, v: f64| *sums.entry(k.to_string()).or_insert(0.0) += v;
    for k in &keys {
        let x = get("srenew", k).rec;
        let y = get("walk", k).rec;
        *acc.get_mut("records").unwrap() += 1;
        let rel = phase(x, "relations").unwrap();
        let nat = &rel["native"];
        if !["stride_steps", "stride_restarts", "stride_renewals"].iter().all(|c| nat.get(*c).is_some()) {
            *acc.get_mut("missing_stride_counters").unwrap() += 1;
            continue;
        }
        let (steps, restarts, renewals) = (f(&nat["stride_steps"]), f(&nat["stride_restarts"]), f(&nat["stride_renewals"]));
        if f(&rel["scalar_mults"]) == 4.0 + 2.0 * restarts {
            *acc.get_mut("scalar_mults_eq_4_plus_2_restarts").unwrap() += 1;
        }
        if f(&rel["adds"]) >= steps + renewals + restarts + 2.0 {
            *acc.get_mut("rel_adds_ge_steps_plus_renewals_plus_restarts_plus_2").unwrap() += 1;
        }
        let gae_ok = |r: &Value| r["phases"].as_array().unwrap().iter()
            .all(|p| (f(&p["gae"]) - (f(&p["adds"]) + f(&p["doubles"]))).abs() < 1e-9);
        if gae_ok(x) && gae_ok(y) {
            *acc.get_mut("phase_gae_eq_adds_plus_doubles").unwrap() += 1;
        }
        let tot_ok = |r: &Value| (py_sum(r["phases"].as_array().unwrap().iter().map(|p| f(&p["gae"])))
            - f(&r["cost"]["total_gae"])).abs() < 1e-9;
        if tot_ok(x) && tot_ok(y) {
            *acc.get_mut("total_gae_eq_sum_phase_gae").unwrap() += 1;
        }
        if 1.0 <= renewals && renewals <= f(&nat["relations"]) {
            *acc.get_mut("renewals_ge_1_and_le_relations").unwrap() += 1;
        }
        let ry = phase(y, "relations").unwrap();
        for (tag, rec_, ph) in [("srenew", x, rel), ("walk", y, ry)] {
            add(&format!("{tag}.relations.adds"), f(&ph["adds"]));
            add(&format!("{tag}.relations.doubles"), f(&ph["doubles"]));
            add(&format!("{tag}.relations.scalar_mults"), f(&ph["scalar_mults"]));
            add(&format!("{tag}.relations.trials"), f(&ph["native"]["trials"]));
            add(&format!("{tag}.relations.relations"), f(&ph["native"]["relations"]));
            add(&format!("{tag}.total_gae"), f(&rec_["cost"]["total_gae"]));
            for pn in ["factor_base", "oracle_setup", "linear_algebra", "verify"] {
                add(&format!("{tag}.{pn}.gae"), phase(rec_, pn).map_or(0.0, |p| f(&p["gae"])));
            }
        }
        add("srenew.stride_steps", steps);
        add("srenew.stride_renewals", renewals);
        add("srenew.stride_restarts", restarts);
        let g0 = |n: &str| ry["native"].get(n).map_or(0.0, f);
        add("walk.walk_steps", g0("walk_steps"));
        add("walk.walk_pool_ops", g0("walk_pool_ops"));
        add("walk.walk_jumps", g0("walk_jumps"));
    }
    let mut accv = Map::new();
    for (k, v) in &acc {
        accv.insert(k.to_string(), json!(v));
    }
    accv.insert(
        "pairs_oracle_setup_gae_equal".into(),
        json!(keys.iter().filter(|k| py_eq(&phase(get("srenew", k).rec, "oracle_setup").unwrap()["gae"],
                                           &phase(get("walk", k).rec, "oracle_setup").unwrap()["gae"])).count()),
    );
    accv.insert(
        "pairs_factor_base_native_equal".into(),
        json!(keys.iter().filter(|k| py_eq(&phase(get("srenew", k).rec, "factor_base").unwrap()["native"],
                                           &phase(get("walk", k).rec, "factor_base").unwrap()["native"])).count()),
    );
    out.insert("stride_renew_accounting".into(), Value::Object(accv));
    let mut ppm = Map::new();
    for (k, v) in &sums {
        ppm.insert(k.clone(), json!(v / n));
    }
    out.insert("per_pair_means".into(), Value::Object(ppm));
    Value::Object(out)
}

// ------------------------------------------------------------ mechanism
fn mechanism(sdir: &str) -> Value {
    let recs = load_jsonl(&format!("{sdir}/records.jsonl"));
    const FIELDS: [&str; 7] = ["trials", "relations", "dep", "rank", "rows", "rep", "G"];
    let mut m: HashMap<String, BTreeMap<Key, (HashMap<&str, f64>, Value)>> = HashMap::new();
    for x in &recs {
        let arm = x["arm"].as_str().unwrap();
        if x["warmup"].as_bool().unwrap() || !(arm == "walk" || arm == "srenew") {
            continue;
        }
        let rel = phase(x, "relations").unwrap();
        let la = phase(x, "linear_algebra").unwrap();
        let mut h = HashMap::new();
        h.insert("trials", f(&rel["native"]["trials"]));
        h.insert("relations", f(&rel["native"]["relations"]));
        h.insert("dep", x["counters"].get("matrix_dependent").map_or(0.0, f));
        h.insert("rank", f(&x["counters"]["matrix_rank"]));
        h.insert("rows", f(&x["counters"]["matrix_rows"]));
        h.insert("rep", f(&rel["native"]["repeated_targets_skipped"]));
        h.insert("G", f(&x["cost"]["total_gae"]));
        let cols = la["native"].get("columns").cloned().unwrap_or(Value::Null);
        let key = (x["workload"]["workload_id"].as_str().unwrap().to_string(), x["round"].as_i64().unwrap());
        m.entry(arm.to_string()).or_default().insert(key, (h, cols));
    }
    let keys: Vec<Key> = m["walk"].keys().cloned().collect();
    let tot = |arm: &str, fld: &str| py_sum(keys.iter().map(|k| m[arm][k].0[fld]));
    let n = keys.len() as f64;
    let mut out = Map::new();
    for fld in FIELDS {
        let mut e = Map::new();
        e.insert("walk_mean".into(), json!(tot("walk", fld) / n));
        e.insert("srenew_mean".into(), json!(tot("srenew", fld) / n));
        if tot("walk", fld) > 0.0 {
            e.insert("ratio".into(), json!(tot("srenew", fld) / tot("walk", fld)));
            let mut cl: BTreeMap<&String, Vec<&Key>> = BTreeMap::new();
            for k in &keys {
                cl.entry(&k.0).or_default().push(k);
            }
            let ids: Vec<&&String> = cl.keys().collect();
            let mut rng = PyRandom::new(SEED);
            let mut vals = Vec::with_capacity(RESAMPLES);
            for _ in 0..RESAMPLES {
                let (mut a, mut b) = (0.0f64, 0.0f64);
                for _ in 0..ids.len() {
                    let mem = &cl[ids[rng.randrange(ids.len())]];
                    for _ in 0..mem.len() {
                        let k = mem[rng.randrange(mem.len())];
                        a += m["walk"][k].0[fld];
                        b += m["srenew"][k].0[fld];
                    }
                }
                vals.push(if a != 0.0 { b / a } else { f64::NAN });
            }
            vals.sort_by(|x, y| x.partial_cmp(y).unwrap());
            e.insert("ci95_two_stage".into(), json!([vals[LO_IDX], vals[HI_IDX]]));
        }
        out.insert(fld.into(), Value::Object(e));
    }
    let cols: BTreeSet<i64> = keys.iter().map(|k| m["walk"][k].1.as_i64().expect("columns")).collect();
    out.insert("cols".into(), json!(cols));
    Value::Object(out)
}

// ------------------------------------------------------------ gate files
const SIZES: [u32; 2] = [20, 28];
const GWEIGHTS: [(&str, &str); 3] = [("0", "0.0"), ("0.1", "0.1"), ("1", "1.0")];
const GPAIRS: [&str; 5] = ["walk__srenew", "walk__walk-aa", "rho-neg__rho-neg-aa", "rho-neg__walk", "rho-neg__srenew"];
const REPLAY_BIN: &str = "bf5eec6d61fbe83a7e6911bf73d05d847cc0ab23fd6db3342290a26b9f3807d9";
const MEAS_BIN: &str = "583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9";

fn s3(b: u32) -> &'static str {
    match b {
        20 => "s3://isolab-results-104186811093-us-east-2/runs/ic-srenew-p20-20261029/8d5ba87a-31b6-46fb-9d07-14bee7f6c08b/",
        28 => "s3://isolab-results-104186811093-us-east-2/runs/ic-srenew-p28-20261029/e8fc61b5-4fb6-4257-b3ae-0692df8ff7c4/",
        _ => unreachable!(),
    }
}

fn round3(v: f64) -> f64 {
    format!("{v:.3}").parse().unwrap()
}

fn gate(study: &str, banddir: &str, val_id: &str, rdir: &str, odir: &str) {
    let vdir = format!("{study}/validation");
    let mut rec = HashMap::new();
    let mut mech = HashMap::new();
    let mut rep = HashMap::new();
    let mut prod_audit = HashMap::new();
    let mut band = HashMap::new();
    let mut cmpf = HashMap::new();
    for b in SIZES {
        rec.insert(b, load(&format!("{rdir}/recompute-P{b}.json")));
        mech.insert(b, load(&format!("{rdir}/mechanism-P{b}.json")));
        rep.insert(b, load(&format!("{vdir}/replay-P{b}.audit.json")));
        prod_audit.insert(b, load(&format!("{study}/P{b}/eks-results/audit.json")));
        band.insert(b, load(&format!("{banddir}/band-P{b}.ecbench.json")));
        for p in GPAIRS {
            for (ws, _) in GWEIGHTS {
                cmpf.insert((b, p, ws), load(&format!("{study}/P{b}/analysis/compare-{p}-w{ws}.json")));
            }
        }
    }

    // ---------------------------------------------------- runs (per record)
    let mut runs: Vec<Value> = vec![];
    for b in SIZES {
        let sdir = format!("{study}/P{b}/eks-results/session");
        let ec: HashMap<String, Curve> = rec[&b]["curves"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(s, v)| (s.clone(), Curve { p: int_of(&v["p"]), a: int_of(&v["a"]), b: int_of(&v["b"]) }))
            .collect();
        let replays: HashMap<&str, &Value> = rep[&b]["replays"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| (r["record_id"].as_str().unwrap(), r))
            .collect();
        for x in load_jsonl(&format!("{sdir}/records.jsonl")) {
            if x["warmup"].as_bool().unwrap() {
                continue;
            }
            let wl = &x["workload"];
            let e = ec[wl["curve"]["slug"].as_str().unwrap()];
            let ok = e.mul(int_of(&x["outcome"]["recovered"]) as u128, pt(&wl["curve"]["generator"])) == pt(&wl["target"]);
            let rp = replays.get(x["record_id"].as_str().unwrap());
            let g = |k: &str| rp.and_then(|r| r.get(k)).cloned().unwrap_or(Value::Null);
            runs.push(json!({
                "session": format!("P{b}"), "seq": x["seq"], "record_id": x["record_id"],
                "arm": x["arm"], "target_index": wl["target_index"],
                "round": x["round"],
                "recovered": x["outcome"]["recovered"],
                "validator_dG_equals_Q": ok,
                "reproduced": g("reproduced"), "detail": g("detail"),
            }));
        }
    }
    assert!(runs.iter().all(|r| r["validator_dG_equals_Q"] == true && r["reproduced"] == true && r["detail"] == "identical"));

    let mut sessions = vec![];
    for b in SIZES {
        let sdir = format!("P{b}/eks-results/session");
        let mut files = Map::new();
        for n in ["host.json", "plan.json", "records.jsonl", "session.json", "spec.json"] {
            files.insert(n.into(), json!(sha(&format!("{study}/{sdir}/{n}"))));
        }
        let files = Value::Object(files);
        let a = &rep[&b];
        let reps = a["replays"].as_array().unwrap();
        let log = fs::read_to_string(format!("{vdir}/replay-P{b}.verify.log")).unwrap();
        sessions.push(json!({
            "session_dir": sdir, "session_id": a["session_id"], "s3_uri": format!("{}session/", s3(b)),
            "receipt": format!("validation/replay-P{b}.audit.json"),
            "receipt_sha256": sha(&format!("{vdir}/replay-P{b}.audit.json")),
            "verify_log": format!("validation/replay-P{b}.verify.log"),
            "records": a["records"], "verified_records": a["verified_records"],
            "replays": reps.len(),
            "replays_reproduced": reps.iter().filter(|r| r["reproduced"] == true && r["detail"] == "identical").count(),
            "problems": a["problems"], "checks": a["checks"], "ok": a["ok"],
            "session_files_sha256": files,
            "session_files_equal_replay_receipt": py_eq(&files, &a["files"]),
            "session_files_equal_producer_receipt": py_eq(&files, &prod_audit[&b]["files"]),
            "session_files_equal_s3_originals": true,
            "producer_receipt": format!("P{b}/eks-results/audit.json"),
            "producer_receipt_sha256": sha(&format!("{study}/P{b}/eks-results/audit.json")),
            "producer_auditor_binary_sha256": prod_audit[&b]["auditor_binary_sha256"],
            "auditor_binary_sha256": a["auditor_binary_sha256"],
            "auditor_env_class_id": a["auditor_env_class_id"],
            "measured_env_class_id": prod_audit[&b]["auditor_env_class_id"],
            "audited_unix_ms": a["audited_unix_ms"],
            "end_line": log.trim().lines().last().unwrap(),
        }));
    }

    let mut rows: Vec<String> = SIZES.iter().flat_map(|b| rec[b]["curves"].as_object().unwrap().keys().cloned()).collect();
    rows.sort();
    let snapshot = json!({
        "branch": "claude/ic-stride-renew-prime-20261029",
        "head_commit": "0e7243aa6cbeae8f0f5a85e618929928d2aa1fd3",
        "protocol_freeze_commit": "4d4593aabda667a681fab4a67e351dc5bcb7c1f4",
        "evidence_committed": false,
        "note": "The study directory was not in any commit when validated (staged by another session during validation). Every file this certificate names is bound by sha256; the eks-results files are byte-identical to the S3 originals written by the worker (validation/provenance.txt). A commit of the study directory must carry these hashes.",
    });
    let mut curves_all = Map::new();
    for b in SIZES {
        for (s, c) in rec[&b]["curves"].as_object().unwrap() {
            curves_all.insert(s.clone(), c.clone());
        }
    }
    let mut measured = Map::new();
    for b in SIZES {
        measured.insert(format!("P{b}"), rec[&b]["dG_eq_Q"]["checked"].clone());
    }
    let failures: Vec<Value> = SIZES.iter().flat_map(|b| rec[b]["dG_eq_Q"]["failures"].as_array().unwrap().clone()).collect();
    let cert = json!({
        "schema": "replay-certificate/v1",
        "evidence_dir": "research/ic_stride_renew_prime_20261029",
        "rows": rows,
        "snapshot": snapshot,
        "status": "verified",
        "independent": true,
        "scalars_verified": true,
        "independence": {
            "replayer": "Validator subagent (claude-opus-5-5), separate from the producer session; re-executed every measured run of both sessions into new receipts under validation/; producer receipts untouched (replayed on byte-identical copies)",
            "same_binary_as_measurement": false,
            "same_host_class_as_measurement": false,
            "independent_implementation": false,
            "independent_scalar_check": true,
            "statement": format!("Cross-binary, cross-host replay: the worker measured with ecbench {} (image, source cb2ec6873f30) on c8a.2xlarge (env ECBENV2h3a51beea0bc2); the validator replayed with ecbench {} on env ECBENV3h13e37662a118, and all 1440 measured runs of each session reproduced identical counts. Every recovered scalar of every measured run re-verifies [d]G = Q under the validator's own affine arithmetic. Not an independent implementation of the IC pipeline.", &MEAS_BIN[..8], &REPLAY_BIN[..8]),
        },
        "binary": {
            "replay_binary_path": "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/bin/ecbench-bf5eec6d",
            "replay_binary_sha256": REPLAY_BIN,
            "measuring_binary_sha256": MEAS_BIN,
            "measuring_binary_source_revision": "cb2ec6873f30b683f7885b53c410d69965f02c20",
            "equals_session_binary_sha256": false,
        },
        "command": "RAYON_NUM_THREADS=1 taskset -c 2 <replay_binary> verify --dir <copy of P<b>/eks-results/session> --replay-all --exit-code --out validation/replay-P<b>.audit.json",
        "independent_scalar_check": {
            "method": "validation/recompute.py: curve coefficients derived from plan.json points; p, r prime; all points on the curve; [r]G = O; r the only multiple of r in the Hasse interval (so #E = r); j-invariant equal to the ICV1 string; [d]G = Q in affine coordinates for every measured record of every arm",
            "curves": curves_all,
            "measured_records": measured,
            "failures": failures,
        },
        "sessions": sessions,
    });
    fs::create_dir_all(odir).unwrap();
    let cert_path = format!("{odir}/replay-certificate.json");
    let body = dumps(&cert, Some(1), false);
    let mut text = String::new();
    text.push_str(&body[..body.len() - 2]);
    text.push_str(",\n \"runs\": [\n");
    text.push_str(&runs.iter().map(|r| format!("  {}", dumps(r, None, false))).collect::<Vec<_>>().join(",\n"));
    text.push_str("\n ]\n}\n");
    fs::write(&cert_path, text).unwrap();

    // ---------------------------------------------------- validator report
    let mut headline: Vec<Value> = vec![];
    for b in SIZES {
        for p in GPAIRS {
            let (a_arm, b_arm) = p.split_once("__").unwrap();
            for (ws, wk) in GWEIGHTS {
                let c = &cmpf[&(b, p, ws)];
                let mine = &rec[&b]["ratios"][p][wk];
                headline.push(json!({
                    "name": format!("P{b} sum S_{b_arm} / sum S_{a_arm} at w={ws} (288 paired runs, 48 targets, 2 curves)"),
                    "claimed": c["ops"]["ratio_b_over_a"], "recomputed": mine["ratio_b_over_a"], "rel_tol": 1e-12,
                    "claimed_source": format!("P{b}/analysis/compare-{p}-w{ws}.json ops.ratio_b_over_a"),
                    "recomputed_by": "validation/recompute.py (validator, stdlib, records.jsonl read directly)",
                }));
                for cv in c["curves"].as_array().unwrap() {
                    let slug = cv["slug"].as_str().unwrap();
                    headline.push(json!({
                        "name": format!("P{b} {slug} sum S_{b_arm} / sum S_{a_arm} at w={ws}"),
                        "claimed": cv["ratio_b_over_a"], "recomputed": mine["per_curve"][slug],
                        "rel_tol": 1e-12,
                        "claimed_source": format!("P{b}/analysis/compare-{p}-w{ws}.json curves[].ratio_b_over_a"),
                        "recomputed_by": "validation/recompute.py",
                    }));
                }
                if p == "walk__srenew" {
                    for (i, side) in [(0usize, "lower"), (1, "upper")] {
                        headline.push(json!({
                            "name": format!("P{b} srenew/walk 95% cluster-bootstrap {side} bound at w={ws}"),
                            "claimed": c["ops"]["ci95_bootstrap"][i],
                            "recomputed": mine["ci95_two_stage_bootstrap"][i], "rel_tol": 0.005,
                            "claimed_source": format!("P{b}/analysis/compare-{p}-w{ws}.json ops.ci95_bootstrap (ecbench, seed 20261002)"),
                            "recomputed_by": "validation/recompute.py two-stage cluster bootstrap, own RNG seed 29102026; agreement is Monte Carlo, not bitwise",
                        }));
                    }
                }
            }
        }
        for row in band[&b]["rows"].as_array().unwrap() {
            for (i, (ws, wk)) in GWEIGHTS.iter().enumerate() {
                let (arm, curve) = (row["arm"].as_str().unwrap(), row["curve"].as_str().unwrap());
                headline.push(json!({
                    "name": format!("P{b} band mean S {arm} on {curve} at w={ws}"),
                    "claimed": row["mean_s"][i], "recomputed": rec[&b]["band_mean_s"][curve][arm][*wk],
                    "rel_tol": 1e-12,
                    "claimed_source": format!("ecbench band --dir P{b}/eks-results/session --json (re-run by the validator; band.md is its 3-decimal rendering, byte-identical)"),
                    "recomputed_by": "validation/recompute.py",
                }));
            }
        }
    }
    let bad: Vec<&Value> = headline
        .iter()
        .filter(|h| (f(&h["claimed"]) - f(&h["recomputed"])).abs() > f(&h["rel_tol"]) * f(&h["claimed"]).abs())
        .collect();
    assert!(bad.is_empty(), "{bad:?}");

    let main_claims = |b: u32, ws: &str| -> [f64; 3] {
        match (b, ws) {
            (20, "0") => [0.842, 0.807, 0.878],
            (20, "0.1") => [0.858, 0.821, 0.897],
            (20, "1") => [0.926, 0.879, 0.975],
            (28, "0") => [0.977, 0.961, 0.992],
            (28, "0.1") => [0.975, 0.958, 0.993],
            (28, "1") => [0.968, 0.941, 0.996],
            _ => unreachable!(),
        }
    };
    let mut mcheck = vec![];
    for b in SIZES {
        for (ws, _) in GWEIGHTS {
            let o = &cmpf[&(b, "walk__srenew", ws)]["ops"];
            let exact = [o["ratio_b_over_a"].clone(), o["ci95_bootstrap"][0].clone(), o["ci95_bootstrap"][1].clone()];
            let got: Vec<f64> = exact.iter().map(|v| round3(f(v))).collect();
            let ms = main_claims(b, ws);
            mcheck.push(json!({"size": b, "w": ws, "main_session": ms, "exact": exact,
                               "rounded": got, "agree": got == ms.to_vec()}));
        }
    }

    let mut gae_band = Map::new();
    for b in SIZES {
        let mut m = Map::new();
        for (ws, wk) in GWEIGHTS {
            let r = &rec[&b]["ratios"];
            m.insert(ws.into(), json!({
                "mean_s_walk": r["walk__srenew"][wk]["mean_s_a"],
                "mean_s_srenew": r["walk__srenew"][wk]["mean_s_b"],
                "mean_s_rho_neg": r["rho-neg__walk"][wk]["mean_s_a"],
                "srenew_over_walk": r["walk__srenew"][wk]["ratio_b_over_a"],
                "srenew_over_walk_ci95_ecbench": cmpf[&(b, "walk__srenew", ws)]["ops"]["ci95_bootstrap"],
                "srenew_over_walk_ci95_validator_two_stage": r["walk__srenew"][wk]["ci95_two_stage_bootstrap"],
                "walk_over_rho_neg": r["rho-neg__walk"][wk]["ratio_b_over_a"],
                "walk_over_rho_neg_ci95_ecbench": cmpf[&(b, "rho-neg__walk", ws)]["ops"]["ci95_bootstrap"],
                "srenew_over_rho_neg": r["rho-neg__srenew"][wk]["ratio_b_over_a"],
                "srenew_over_rho_neg_ci95_ecbench": cmpf[&(b, "rho-neg__srenew", ws)]["ops"]["ci95_bootstrap"],
            }));
        }
        gae_band.insert(format!("P{b}"), Value::Object(m));
    }

    let mut raw = vec![];
    for b in SIZES {
        for n in ["records.jsonl", "plan.json", "session.json", "spec.json", "host.json"] {
            let p = format!("P{b}/eks-results/session/{n}");
            raw.push(json!({"path": p, "sha256": sha(&format!("{study}/{p}")), "s3": format!("{}session/{n}", s3(b))}));
        }
        for n in ["SHA256SUMS", "audit.json", "l2-gate.json", "input-spec.json", "build-info.json", "job-exit.json"] {
            let p = format!("P{b}/eks-results/{n}");
            raw.push(json!({"path": p, "sha256": sha(&format!("{study}/{p}")), "s3": format!("{}{n}", s3(b))}));
        }
    }
    let per = |src: &HashMap<u32, Value>, sub: Option<&str>| -> Value {
        let mut m = Map::new();
        for b in SIZES {
            m.insert(format!("P{b}"), match sub { Some(s) => src[&b][s].clone(), None => src[&b].clone() });
        }
        Value::Object(m)
    };

    let report = json!({
        "schema": "validator-report/v1",
        "id": val_id,
        "status": "pass",
        "verdict": "passed",
        "independent": true,
        "validator": "Validator subagent (claude-opus-5-5); did not produce the protocol, runs, analysis or claim",
        "claim": "XP-20261029-stride-renew-prime: at 20 and 28 bits, ic.pipeline with targets = stride-renew (srenew) has lower whole-pipeline S than targets = walk (walk) at w = 0, 0.1 and 1, with the 95% cluster-bootstrap upper bound below 1 at every weight, and band --ranks srenew < walk on every curve (PROTOCOL.md section 6)",
        "claim_verdict": {
            "20": "improvement: section 6 conditions 1-4 hold; validator: pass",
            "28": "improvement: section 6 conditions 1-4 hold; validator: pass with caveats (see caveats)",
        },
        "snapshot": snapshot,
        "method": "records.jsonl and plan.json read directly by validation/recompute.py (stdlib): priced S(w) = (G + w U)/sqrt(r), C = n_C = 0 (no record names a pinned calibration unit), U per the band definition; paired ratios of totals; own one-stage, stratified and two-stage cluster bootstraps; own affine EC arithmetic for [d]G = Q. ecbench compare/band re-run with a different binary and byte-compared with analysis/. Local verify --replay-all of both sessions. Mechanism split by validation/mechanism.py.",
        "raw_data": raw,
        "headline": headline,
        "main_session_claims_check": mcheck,
        "gae_band": gae_band,
        "completeness": per(&rec, Some("completeness")),
        "null_controls": per(&rec, Some("aa_exact")),
        "stride_renew_accounting": per(&rec, Some("stride_renew_accounting")),
        "per_pair_means": per(&rec, Some("per_pair_means")),
        "mechanism": per(&mech, None),
        "replay_certificate": {"path": "validation/replay-certificate.json", "sha256": sha(&cert_path)},
        "caveats": [
            "P28: about 2/3 of the mean saving (about 4,000 of 6,226 GAE per pair at w = 0) comes from srenew needing 4.4% fewer relations (203.3 vs 212.7; two-stage 95% CI of the ratio [0.916, 0.996]) and 4.3% fewer trials ([0.915, 0.999]); about 1/3 (about 2,200 GAE: 1,482 measured doublings plus an estimated 730 additions inside the 58.75 fewer scalar multiplications) is the deterministic scalar-multiplication saving, about 0.8% of the walk's S on its own. PROTOCOL.md section 7 attributes the gain to the scalar-multiplication share only; the relation-count effect was not predicted, has no stated mechanism, and is about two standard errors from zero.",
            "P28: the w = 1 upper bound is 0.99549 (ecbench) and 0.9945-0.9965 over 11 validator bootstrap seeds: below 1 but within 0.6% of it.",
            "Four sizes are decided separately at 95% each, with no multiplicity correction (as frozen). P24 (null, point 0.982) and P32 run 1 (rejected by the L2 gate, point 1.001) are producer numbers that this validation did not check.",
            "Main-session rounding: the P28 w = 1 upper bound 0.996 should read 0.995 (0.99549). All other quoted numbers round correctly.",
        ],
        "limitations": [
            "Same IC implementation (ecbench) in measurement and replay; replay is cross-binary and cross-host, not an independent implementation.",
            "Measuring binary 583e62f0 was built from cb2ec6873f30 (build-info.json); not rebuilt by the validator. Session git_commit/git_dirty are null (image build).",
            "Evidence directory not committed at validation time; bound by sha256 to S3 originals.",
            "Two curves per size, 24 targets per curve, 6 rounds; intervals cover target-to-target variation on these curves. No exponent, cross-size or asymptotic claim.",
            "Counted unit only (ecbench GAE / sqrt(r), section 5a band). No wall-clock claim.",
        ],
        "review_attestation": {
            "joints_owned": ["whole receipt for P20 and P28 (no review_plan on the task card)"],
            "sources_read": [
                "crypto-autoresearcher: docs/agent-runtime-core.md, agents/validator.md",
                "wt-stride: backlog/XP-20261029-stride-renew-prime/{PROTOCOL.md, spec-P20.json, spec-P28.json}",
                "wt-stride: research/ic_stride_renew_prime_20261029/launch/card-0{1,2,3,4}.md (headers, pins)",
                "wt-stride: research/ic_stride_renew_prime_20261029/P{20,28}/{launch/*, eks-results/* (receipt files, records.jsonl and plan.json by script), analysis/compare-*.json, analysis/band.md}",
                "wt-stride: research/ic_stride_renew_prime_20261029/P24/analysis/compare-walk__srenew-w*.json, P32-run1/analysis/compare-walk__srenew-w*.json (context only)",
                "S3: both job prefixes (listing, hashes of every file, launcher download-hashes.txt for P20)",
                "crypto origin/main: docs/VALIDATION_GATE.md, scripts/validation_gate.py (key requirements), src/cryptanalysis/ecbench/band.rs (priced_gae definition), src/cryptanalysis/ecbench/stats.rs (cluster_bootstrap_ci scheme), src/cryptanalysis/ic_boundary.rs (TargetSource::StrideRenew)",
                "crypto cb2ec6873f30: src/cryptanalysis/ic_boundary.rs (TargetSource::StrideRenew, draw_jumps, GroupOps::gae, CountedGroup::mul)",
                "crypto origin/main: research/prime_base_oracle_tuning_20261029/{validation/validation_report.yaml, validation/provenance.txt, replay-certificate.json, validator-report.json, VALIDATION.json} (format only)",
            ],
            "not_read": ["research/ic_stride_renew_prime_20261029/figures/", "docs/session-logs/ (producer narrative)"],
            "read_sibling_reports": false,
            "verdict": "passed",
        },
    });
    let rep_path = format!("{odir}/validator-report.json");
    fs::write(&rep_path, dumps(&report, Some(1), false) + "\n").unwrap();
    let agree: Vec<Value> = mcheck.iter().map(|m| m["agree"].clone()).collect();
    println!(
        "{}",
        dumps(&json!({"runs": runs.len(), "headline": headline.len(), "main_claims_agree": agree,
                      "cert_sha256": sha(&cert_path), "report_sha256": sha(&rep_path)}), None, false)
    );
}

// ------------------------------------------------------------ cmpjson
#[derive(Default)]
struct Cmp {
    numbers: u64,
    exact: u64,
    max_rel: f64,
    worst: String,
    mismatches: Vec<String>,
    differing: Vec<String>,
}

fn cmp_walk(a: &Value, b: &Value, path: &str, c: &mut Cmp) {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            c.numbers += 1;
            let (p, q) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            if py_eq(a, b) && (x.is_f64() == y.is_f64()) {
                c.exact += 1;
            } else {
                if x.is_f64() != y.is_f64() {
                    c.mismatches.push(format!("{path}: int/float type differs"));
                }
                let rel = (p - q).abs() / p.abs().max(q.abs());
                c.differing.push(path.to_string());
                if rel > c.max_rel {
                    c.max_rel = rel;
                    c.worst = path.to_string();
                }
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (p, q)) in x.iter().zip(y).enumerate() {
                cmp_walk(p, q, &format!("{path}[{i}]"), c);
            }
        }
        (Value::Object(x), Value::Object(y)) => {
            let kx: BTreeSet<&String> = x.keys().collect();
            let ky: BTreeSet<&String> = y.keys().collect();
            if kx != ky {
                c.mismatches.push(format!("{path}: key sets differ"));
            }
            for k in kx.intersection(&ky) {
                cmp_walk(&x[*k], &y[*k], &format!("{path}.{k}"), c);
            }
        }
        _ => {
            if a != b {
                c.mismatches.push(format!("{path}: {a} != {b}"));
            }
        }
    }
}

fn cmpjson(a: &str, b: &str, tol: f64) -> bool {
    let (va, vb) = (load(a), load(b));
    let mut c = Cmp::default();
    cmp_walk(&va, &vb, "$", &mut c);
    let byte_equal = fs::read(a).unwrap() == fs::read(b).unwrap();
    let ok = c.mismatches.is_empty() && c.max_rel <= tol;
    println!(
        "{}",
        dumps(&json!({"a": a, "b": b, "a_sha256": sha(a), "b_sha256": sha(b), "byte_equal": byte_equal,
                      "numbers": c.numbers, "numbers_bitwise_equal": c.exact, "max_rel_dev": c.max_rel,
                      "worst_path": c.worst, "rel_tol": tol, "structural_mismatches": c.mismatches,
                      "differing_number_paths": c.differing, "equal_within_tol": ok}), None, false)
    );
    ok
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let usage = "usage: srenew-validate recompute <session_dir> <label> | mechanism <session_dir> | \
                 gate <study_dir> <band_dir> <val_id> --recompute-dir <dir> --out <dir> | cmpjson <a> <b> <rel_tol>";
    match args.get(1).map(String::as_str) {
        Some("recompute") if args.len() == 4 => println!("{}", dumps(&recompute(&args[2], &args[3]), Some(1), true)),
        Some("mechanism") if args.len() == 3 => println!("{}", dumps(&mechanism(&args[2]), Some(1), false)),
        Some("gate") if args.len() == 9 && args[5] == "--recompute-dir" && args[7] == "--out" => {
            assert!(Path::new(&args[2]).is_dir());
            gate(&args[2], &args[3], &args[4], &args[6], &args[8]);
        }
        Some("cmpjson") if args.len() == 5 => {
            if !cmpjson(&args[2], &args[3], args[4].parse().unwrap()) {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    }
}
