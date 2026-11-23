//! The CPython behaviour the legacy validator's outputs depend on, reimplemented
//! so the native outputs are byte-equal to the recorded ones:
//!
//! * `json.dump(obj, f, indent=1, sort_keys=True)` (ensure_ascii, float repr);
//! * `repr()` of JSON-loaded values (used inside diff strings);
//! * `==` between JSON-loaded values (int 1 == float 1.0, True == 1);
//! * int/float arithmetic where the type of the result is visible in output;
//! * builtin `sum()` (Neumaier-compensated over floats since CPython 3.12);
//! * `random.Random(seed)`: MT19937 `init_by_array`, `random()`, `randrange(n)`;
//! * `math.lgamma` (CPython's own Lanczos implementation, not libm's);
//! * `str.split(None, maxsplit)` and `str.splitlines()`.

use serde_json::{Map, Value};
use std::cmp::Ordering;
use std::fmt::Write as _;

// ---------------------------------------------------------------- numbers

/// A Python number as `json.loads` produces it (bool counts as int).
#[derive(Clone, Copy, Debug)]
pub enum Num {
    Int(i128),
    Float(f64),
}

impl Num {
    pub fn of(v: &Value) -> Num {
        match v {
            Value::Bool(b) => Num::Int(*b as i128),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Num::Int(i as i128)
                } else if let Some(u) = n.as_u64() {
                    Num::Int(u as i128)
                } else {
                    Num::Float(n.as_f64().unwrap())
                }
            }
            _ => panic!("STOP: expected a number, got {v}"),
        }
    }
    pub fn f(self) -> f64 {
        match self {
            Num::Int(i) => int_to_f64(i),
            Num::Float(x) => x,
        }
    }
    pub fn add(self, o: Num) -> Num {
        match (self, o) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a + b),
            _ => Num::Float(self.f() + o.f()),
        }
    }
    pub fn sub(self, o: Num) -> Num {
        match (self, o) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a - b),
            _ => Num::Float(self.f() - o.f()),
        }
    }
    pub fn mul(self, o: Num) -> Num {
        match (self, o) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a * b),
            _ => Num::Float(self.f() * o.f()),
        }
    }
    /// `/` (true division): always a float.
    pub fn div(self, o: Num) -> f64 {
        self.f() / o.f()
    }
    pub fn abs(self) -> Num {
        match self {
            Num::Int(a) => Num::Int(a.abs()),
            Num::Float(x) => Num::Float(x.abs()),
        }
    }
    /// Python's exact comparison between ints and floats.
    pub fn cmp(self, o: Num) -> Ordering {
        match (self, o) {
            (Num::Int(a), Num::Int(b)) => a.cmp(&b),
            (Num::Float(a), Num::Float(b)) => a.partial_cmp(&b).expect("NaN"),
            (Num::Int(a), Num::Float(b)) => cmp_int_float(a, b),
            (Num::Float(a), Num::Int(b)) => cmp_int_float(b, a).reverse(),
        }
    }
    pub fn eq(self, o: Num) -> bool {
        match (self, o) {
            (Num::Float(a), Num::Float(b)) => a == b,
            _ => self.cmp(o) == Ordering::Equal,
        }
    }
    pub fn val(self) -> Value {
        match self {
            Num::Int(i) => int_val(i),
            Num::Float(x) => fval(x),
        }
    }
}

fn cmp_int_float(a: i128, b: f64) -> Ordering {
    if b.is_nan() {
        panic!("NaN");
    }
    if a.unsigned_abs() < (1u128 << 53) {
        return (a as f64).partial_cmp(&b).unwrap(); // exact conversion
    }
    if b.abs() >= 1.6e38 {
        return if b > 0.0 { Ordering::Less } else { Ordering::Greater };
    }
    let fl = b.floor() as i128; // b at this size has no fraction below 2**53 ulp
    match a.cmp(&fl) {
        Ordering::Equal if b.fract() != 0.0 => Ordering::Less,
        o => o,
    }
}

/// int -> float as CPython converts (exact below 2**53; these data never exceed it).
pub fn int_to_f64(i: i128) -> f64 {
    assert!(i.unsigned_abs() < (1u128 << 53), "STOP: int {i} beyond exact float range");
    i as f64
}

/// `int / int` true division (both exact as doubles here, so one IEEE division).
pub fn idiv(a: i128, b: i128) -> f64 {
    int_to_f64(a) / int_to_f64(b)
}

pub fn fval(x: f64) -> Value {
    Value::Number(serde_json::Number::from_f64(x).expect("STOP: non-finite float in output"))
}

pub fn int_val(i: i128) -> Value {
    if let Ok(v) = i64::try_from(i) {
        Value::from(v)
    } else if let Ok(v) = u64::try_from(i) {
        Value::from(v)
    } else {
        panic!("STOP: int {i} outside 64 bits")
    }
}

pub fn int(v: &Value) -> i128 {
    match Num::of(v) {
        Num::Int(i) => i,
        Num::Float(x) => panic!("STOP: expected an int, got float {x}"),
    }
}

/// Python `int(s)` for a decimal string.
pub fn int_str(v: &Value) -> i128 {
    match v {
        Value::String(s) => s.trim().parse::<i128>().expect("STOP: int(str)"),
        _ => int(v),
    }
}

/// Python `int(s, 16)` (accepts a 0x prefix).
pub fn int_hex(v: &Value) -> i128 {
    let s = v.as_str().expect("STOP: hex string").trim();
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    i128::from_str_radix(s, 16).expect("STOP: int(x, 16)")
}

/// Python truthiness of a JSON value.
pub fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(_) => match Num::of(v) {
            Num::Int(i) => i != 0,
            Num::Float(x) => x != 0.0,
        },
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

// ---------------------------------------------------------------- sum()

/// Builtin `sum()` of floats from start 0 (CPython >= 3.12: Neumaier
/// compensation, compensation added at the end when non-zero and finite).
pub fn fsum_py(xs: impl IntoIterator<Item = f64>) -> f64 {
    let mut it = xs.into_iter();
    let first = match it.next() {
        None => return 0.0, // Python returns int 0; callers never hit it
        Some(x) => 0.0 + x,
    };
    let (mut f, mut c) = (first, 0.0f64);
    for x in it {
        let t = f + x;
        if f.abs() >= x.abs() {
            c += (f - t) + x;
        } else {
            c += (x - t) + f;
        }
        f = t;
    }
    if c != 0.0 && c.is_finite() {
        f += c;
    }
    f
}

/// Builtin `sum()` over a homogeneous or mixed sequence of numbers.
pub fn sum_py(xs: &[Num]) -> Num {
    let mut i = 0usize;
    let mut acc: i128 = 0;
    while i < xs.len() {
        match xs[i] {
            Num::Int(v) => acc += v,
            Num::Float(_) => break,
        }
        i += 1;
    }
    if i == xs.len() {
        return Num::Int(acc);
    }
    let mut f = int_to_f64(acc) + xs[i].f();
    let mut c = 0.0f64;
    for x in &xs[i + 1..] {
        match *x {
            Num::Float(x) => {
                let t = f + x;
                if f.abs() >= x.abs() {
                    c += (f - t) + x;
                } else {
                    c += (x - t) + f;
                }
                f = t;
            }
            Num::Int(v) => f += int_to_f64(v), // ints are added uncompensated
        }
    }
    if c != 0.0 && c.is_finite() {
        f += c;
    }
    Num::Float(f)
}

// ---------------------------------------------------------------- MT19937

pub struct PyRandom {
    mt: [u32; 624],
    idx: usize,
}

impl PyRandom {
    /// `random.Random(seed)` for a non-negative int seed.
    pub fn new(seed: u128) -> PyRandom {
        let mut key = Vec::new();
        let mut n = seed;
        while n > 0 {
            key.push((n & 0xffff_ffff) as u32);
            n >>= 32;
        }
        if key.is_empty() {
            key.push(0);
        }
        let mut r = PyRandom { mt: [0; 624], idx: 624 };
        r.init_genrand(19650218);
        let (mut i, mut j) = (1usize, 0usize);
        let kl = key.len();
        let mut k = if 624 > kl { 624 } else { kl };
        while k > 0 {
            let prev = r.mt[i - 1];
            r.mt[i] = (r.mt[i] ^ ((prev ^ (prev >> 30)).wrapping_mul(1664525)))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= 624 {
                r.mt[0] = r.mt[623];
                i = 1;
            }
            if j >= kl {
                j = 0;
            }
            k -= 1;
        }
        k = 623;
        while k > 0 {
            let prev = r.mt[i - 1];
            r.mt[i] = (r.mt[i] ^ ((prev ^ (prev >> 30)).wrapping_mul(1566083941))).wrapping_sub(i as u32);
            i += 1;
            if i >= 624 {
                r.mt[0] = r.mt[623];
                i = 1;
            }
            k -= 1;
        }
        r.mt[0] = 0x8000_0000;
        r
    }

    fn init_genrand(&mut self, s: u32) {
        self.mt[0] = s;
        for i in 1..624 {
            let prev = self.mt[i - 1];
            self.mt[i] = 1812433253u32.wrapping_mul(prev ^ (prev >> 30)).wrapping_add(i as u32);
        }
        self.idx = 624;
    }

    pub fn genrand_u32(&mut self) -> u32 {
        const UPPER: u32 = 0x8000_0000;
        const LOWER: u32 = 0x7fff_ffff;
        const MATRIX_A: u32 = 0x9908_b0df;
        if self.idx >= 624 {
            for kk in 0..624 {
                let y = (self.mt[kk] & UPPER) | (self.mt[(kk + 1) % 624] & LOWER);
                let mag = if y & 1 == 1 { MATRIX_A } else { 0 };
                self.mt[kk] = self.mt[(kk + 397) % 624] ^ (y >> 1) ^ mag;
            }
            self.idx = 0;
        }
        let mut y = self.mt[self.idx];
        self.idx += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^= y >> 18;
        y
    }

    /// `random.random()` (genrand_res53).
    pub fn random(&mut self) -> f64 {
        let a = (self.genrand_u32() >> 5) as f64;
        let b = (self.genrand_u32() >> 6) as f64;
        (a * 67108864.0 + b) * (1.0 / 9007199254740992.0)
    }

    /// `getrandbits(k)` for 0 <= k <= 32.
    fn getrandbits(&mut self, k: u32) -> u64 {
        assert!(k <= 32);
        if k == 0 {
            return 0;
        }
        (self.genrand_u32() >> (32 - k)) as u64
    }

    /// `randrange(n)` = `_randbelow_with_getrandbits(n)`.
    pub fn randrange(&mut self, n: u64) -> u64 {
        assert!(n > 0 && n < (1u64 << 32));
        let k = 64 - n.leading_zeros();
        let mut r = self.getrandbits(k);
        while r >= n {
            r = self.getrandbits(k);
        }
        r
    }
}

// ---------------------------------------------------------------- lgamma

const LANCZOS_N: usize = 13;
const LANCZOS_G: f64 = 6.024680040776729583740234375;
const LANCZOS_G_MINUS_HALF: f64 = 5.524680040776729583740234375;
const LANCZOS_NUM: [f64; LANCZOS_N] = [
    23531376880.410759688572007674451636754734846804940,
    42919803642.649098768957899047001988850926355848959,
    35711959237.355668049440185451547166705960488635843,
    17921034426.037209699919755754458931112671403265390,
    6039542586.3520280050642916443072979210699388420708,
    1439720407.3117216736632230727949123939715485786772,
    248874557.86205415651146038641322942321632125127801,
    31426415.585400194380614231628318205362874684987640,
    2876370.6289353724412254090516208496135991145378768,
    186056.26539522349504029498971604569928220784236328,
    8071.6720023658162106380029022722506138218516325024,
    210.82427775157934587250973392071336271166969580291,
    2.5066282746310002701649081771338373386264310793408,
];
const LANCZOS_DEN: [f64; LANCZOS_N] = [
    0.0, 39916800.0, 120543840.0, 150917976.0, 105258076.0, 45995730.0, 13339535.0, 2637558.0, 357423.0,
    32670.0, 1925.0, 66.0, 1.0,
];

fn lanczos_sum(x: f64) -> f64 {
    let (mut num, mut den) = (0.0f64, 0.0f64);
    if x < 5.0 {
        for i in (0..LANCZOS_N).rev() {
            num = num * x + LANCZOS_NUM[i];
            den = den * x + LANCZOS_DEN[i];
        }
    } else {
        for i in 0..LANCZOS_N {
            num = num / x + LANCZOS_NUM[i];
            den = den / x + LANCZOS_DEN[i];
        }
    }
    num / den
}

/// CPython `math.lgamma` (Modules/mathmodule.c `m_lgamma`) for x > 0, which
/// is all the validator calls it with.
pub fn lgamma(x: f64) -> f64 {
    assert!(x.is_finite() && x > 0.0, "lgamma domain used by the validator is x > 0");
    if x == x.floor() && x <= 2.0 {
        return 0.0;
    }
    let absx = x.abs();
    if absx < 1e-20 {
        return -absx.ln();
    }
    let mut r = lanczos_sum(absx).ln() - LANCZOS_G;
    r += (absx - 0.5) * ((absx + LANCZOS_G_MINUS_HALF).ln() - 1.0);
    r
}

// ---------------------------------------------------------------- strings

/// `s.split(None, maxsplit)` (maxsplit < 0: unlimited).
pub fn split_ws(s: &str, maxsplit: i64) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = s.trim_start();
    while !rest.is_empty() {
        if maxsplit >= 0 && out.len() as i64 == maxsplit {
            out.push(rest.to_string());
            break;
        }
        match rest.find(char::is_whitespace) {
            Some(i) => {
                out.push(rest[..i].to_string());
                rest = rest[i..].trim_start();
            }
            None => {
                out.push(rest.to_string());
                break;
            }
        }
    }
    out
}

/// `s.splitlines()`.
pub fn splitlines(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push(std::mem::take(&mut cur));
            }
            '\n' | '\x0b' | '\x0c' | '\x1c' | '\x1d' | '\x1e' | '\u{85}' | '\u{2028}' | '\u{2029}' => {
                out.push(std::mem::take(&mut cur))
            }
            _ => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Python `str[:n]` on code points.
pub fn prefix_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

// ---------------------------------------------------------------- equality

/// Python `==` between two JSON-loaded values.
pub fn py_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(_) | Value::Number(_), Value::Bool(_) | Value::Number(_)) => Num::of(a).eq(Num::of(b)),
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| py_eq(p, q)),
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| py_eq(v, w)))
        }
        _ => false,
    }
}

// ---------------------------------------------------------------- repr

/// `repr(float)`.
pub fn float_repr(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf".into() } else { "-inf".into() };
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    let neg = x < 0.0;
    let e = format!("{:e}", x.abs()); // shortest round-trip digits
    let (mant, exp) = e.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let n = digits.len() as i32;
    let decpt = exp + 1;
    let mut s = String::new();
    if neg {
        s.push('-');
    }
    if decpt > -4 && decpt <= 16 {
        if decpt <= 0 {
            s.push_str("0.");
            for _ in 0..(-decpt) {
                s.push('0');
            }
            s.push_str(&digits);
        } else if decpt >= n {
            s.push_str(&digits);
            for _ in 0..(decpt - n) {
                s.push('0');
            }
            s.push_str(".0");
        } else {
            s.push_str(&digits[..decpt as usize]);
            s.push('.');
            s.push_str(&digits[decpt as usize..]);
        }
    } else {
        s.push_str(&digits[..1]);
        if n > 1 {
            s.push('.');
            s.push_str(&digits[1..]);
        }
        let ee = decpt - 1;
        let _ = write!(s, "e{}{:02}", if ee < 0 { '-' } else { '+' }, ee.abs());
    }
    s
}

pub fn num_repr(n: &serde_json::Number) -> String {
    if let Some(i) = n.as_i64() {
        i.to_string()
    } else if let Some(u) = n.as_u64() {
        u.to_string()
    } else {
        float_repr(n.as_f64().unwrap())
    }
}

fn str_repr(s: &str) -> String {
    let q = if s.contains('\'') && !s.contains('"') { '"' } else { '\'' };
    let mut o = String::new();
    o.push(q);
    for c in s.chars() {
        match c {
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if c == q => {
                o.push('\\');
                o.push(c)
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                let _ = write!(o, "\\x{:02x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o.push(q);
    o
}

/// `repr()` of a JSON-loaded value (dicts in insertion order).
pub fn repr(v: &Value) -> String {
    match v {
        Value::Null => "None".into(),
        Value::Bool(true) => "True".into(),
        Value::Bool(false) => "False".into(),
        Value::Number(n) => num_repr(n),
        Value::String(s) => str_repr(s),
        Value::Array(a) => format!("[{}]", a.iter().map(repr).collect::<Vec<_>>().join(", ")),
        Value::Object(o) => format!(
            "{{{}}}",
            o.iter().map(|(k, v)| format!("{}: {}", str_repr(k), repr(v))).collect::<Vec<_>>().join(", ")
        ),
    }
}

// ---------------------------------------------------------------- json.dump

/// Marker for a dict key that was a Python int (sorted numerically by
/// `sort_keys`, written as its decimal string).
const INT_KEY: char = '\u{1}';

pub fn int_key(n: i64) -> String {
    assert!(n >= 0);
    format!("{INT_KEY}{n:020}")
}

fn write_key(o: &mut String, k: &str) {
    if let Some(rest) = k.strip_prefix(INT_KEY) {
        let t = rest.trim_start_matches('0');
        write_str(o, if t.is_empty() { "0" } else { t });
    } else {
        write_str(o, k);
    }
}

fn write_str(o: &mut String, s: &str) {
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            '\x08' => o.push_str("\\b"),
            '\x0c' => o.push_str("\\f"),
            ' '..='~' => o.push(c),
            c => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    let _ = write!(o, "\\u{:04x}", u);
                }
            }
        }
    }
    o.push('"');
}

fn write_num(o: &mut String, n: &serde_json::Number) {
    if n.is_f64() {
        let x = n.as_f64().unwrap();
        if x.is_nan() {
            o.push_str("NaN")
        } else if x.is_infinite() {
            o.push_str(if x > 0.0 { "Infinity" } else { "-Infinity" })
        } else {
            o.push_str(&float_repr(x))
        }
    } else {
        o.push_str(&num_repr(n))
    }
}

fn write_val(o: &mut String, v: &Value, level: usize) {
    match v {
        Value::Null => o.push_str("null"),
        Value::Bool(b) => o.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => write_num(o, n),
        Value::String(s) => write_str(o, s),
        Value::Array(a) => {
            if a.is_empty() {
                o.push_str("[]");
                return;
            }
            o.push('[');
            for (i, x) in a.iter().enumerate() {
                if i > 0 {
                    o.push(',');
                }
                o.push('\n');
                indent(o, level + 1);
                write_val(o, x, level + 1);
            }
            o.push('\n');
            indent(o, level);
            o.push(']');
        }
        Value::Object(m) => {
            if m.is_empty() {
                o.push_str("{}");
                return;
            }
            let mut items: Vec<(&String, &Value)> = m.iter().collect();
            items.sort_by(|a, b| a.0.cmp(b.0));
            o.push('{');
            for (i, (k, x)) in items.into_iter().enumerate() {
                if i > 0 {
                    o.push(',');
                }
                o.push('\n');
                indent(o, level + 1);
                write_key(o, k);
                o.push_str(": ");
                write_val(o, x, level + 1);
            }
            o.push('\n');
            indent(o, level);
            o.push('}');
        }
    }
}

fn indent(o: &mut String, level: usize) {
    for _ in 0..level {
        o.push(' ');
    }
}

/// The bytes `json.dump(obj, f, indent=1, sort_keys=True); f.write("\n")` writes.
pub fn dumps(v: &Value) -> String {
    let mut o = String::new();
    write_val(&mut o, v, 0);
    o.push('\n');
    o
}

pub fn dump(v: &Value, path: &std::path::Path) {
    std::fs::write(path, dumps(v)).unwrap_or_else(|e| panic!("STOP: write {}: {e}", path.display()));
}

/// An ordered JSON object builder.
pub struct Obj(pub Map<String, Value>);

impl Obj {
    pub fn new() -> Obj {
        Obj(Map::new())
    }
    pub fn set(&mut self, k: impl Into<String>, v: impl Into<Value>) -> &mut Obj {
        self.0.insert(k.into(), v.into());
        self
    }
    pub fn val(self) -> Value {
        Value::Object(self.0)
    }
}

impl Default for Obj {
    fn default() -> Self {
        Obj::new()
    }
}
