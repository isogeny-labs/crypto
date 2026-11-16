//! **Blind-spot item 01: Gaudry's `k = 2` index calculus on `E(F_{p²})`
//! against matched rho** (`research/blindspots_20261018/01-fp2-gaudry-k2`).
//!
//! The file depends on nothing but `std`, so `rustc` builds it on its own:
//! the `crypto_lib` library does not compile at the branch base (a bad merge
//! left `ecbench` and `gaudry_quartic` inconsistent), and nothing here needs
//! it.  The index-calculus pieces are ports of the repository's audited ones,
//! named where they are used: `line_oracle.rs` (the Weil-descended `S₃`
//! oracle), `gaudry_cubic.rs` (`square_core`, Wiedemann, Berlekamp–Massey,
//! the large-prime eliminator), `glv_invariant_base.rs` (the fold),
//! `gls_fp2.rs` (the GLS twist and `ψ`), `ic_boundary.rs` (the walk and the
//! counted rho).
//!
//! ```sh
//! rustc --edition 2021 -C opt-level=3 -C codegen-units=1 \
//!     -o <bin> examples/blindspots_01_fp2_gaudry.rs
//! <bin> smoke [--bits 24]
//! <bin> run --family prime|torsion|gls --bits B --idx I --arms a,b,... --out FILE
//! <bin> summarize --dir DIR --out FILE
//! ```
//!
//! Arms: `ic-a` (plain Gaudry–Diem, `x ∈ F_p`, fold `⟨−1⟩`), `ic-b` (the
//! FGHR 2-torsion line `x ∈ x₀ + c·F_p`, fold `⟨−1, τ_T⟩`), `ic-c1`/`ic-c2`
//! (`ic-b` with single/double large primes), `ic-gls-neg`/`ic-gls-psi` (the
//! GLS line folded by `⟨−1⟩` / `⟨−1, ψ⟩`), `rho-neg`, `rho-psi`, and the
//! negative control `control-shuffled-a`.
//!
//! **The unit.**  Every index-calculus arm counts the `F_p` multiplications
//! and inversions it performs (inversions at `--inv-cost` multiplications),
//! divides by the multiplication equivalents of one affine addition on the
//! same curve (`c_add`, measured on random operand pairs) to get
//! group-addition equivalents (GAE), and lists the native work the unit does
//! not price (linear-algebra row operations, filter passes, hash probes,
//! walk hashes) so it can be priced at 0, 0.1 and 1 GAE per operation
//! (AGENTS.md §5a).  Rho counts group additions and doublings (one GAE each)
//! and lists its canonicalisations, partition hashes and table operations
//! the same way.  No Python.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::hash::{BuildHasherDefault, Hasher};
use std::io::Write as _;
use std::time::Instant;

const SCHEMA: &str = "blindspot01.v1";
const BAND: [f64; 3] = [0.0, 0.1, 1.0];
const BAND_NAMES: [&str; 3] = ["w0", "w0.1", "w1"];

// ── JSON (std only) ────────────────────────────────────────────────

#[derive(Clone, Debug)]
enum J {
    Null,
    B(bool),
    N(f64),
    S(String),
    A(Vec<J>),
    O(Vec<(String, J)>),
}

impl J {
    fn obj(items: Vec<(&str, J)>) -> J {
        J::O(items.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
    fn u(v: u64) -> J {
        J::N(v as f64)
    }
    fn f(v: f64) -> J {
        J::N(v)
    }
    fn s(v: &str) -> J {
        J::S(v.to_string())
    }
    fn get(&self, k: &str) -> &J {
        static NULL: J = J::Null;
        if let J::O(items) = self {
            for (kk, v) in items {
                if kk == k {
                    return v;
                }
            }
        }
        &NULL
    }
    fn f64(&self) -> Option<f64> {
        if let J::N(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    fn u64(&self) -> Option<u64> {
        self.f64().map(|v| v as u64)
    }
    fn str(&self) -> Option<&str> {
        if let J::S(s) = self {
            Some(s)
        } else {
            None
        }
    }
    fn bool(&self) -> Option<bool> {
        if let J::B(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    fn write(&self, out: &mut String, indent: Option<usize>, level: usize) {
        match self {
            J::Null => out.push_str("null"),
            J::B(b) => out.push_str(if *b { "true" } else { "false" }),
            J::N(v) => {
                if !v.is_finite() {
                    out.push_str("null")
                } else if *v == v.trunc() && v.abs() < 9.0e15 {
                    let _ = write!(out, "{}", *v as i64);
                } else {
                    let _ = write!(out, "{:e}", v);
                }
            }
            J::S(s) => {
                out.push('"');
                for c in s.chars() {
                    match c {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        c if (c as u32) < 0x20 => {
                            let _ = write!(out, "\\u{:04x}", c as u32);
                        }
                        c => out.push(c),
                    }
                }
                out.push('"');
            }
            J::A(items) => {
                out.push('[');
                for (i, v) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    if let Some(w) = indent {
                        out.push('\n');
                        out.push_str(&" ".repeat(w * (level + 1)));
                    }
                    v.write(out, indent, level + 1);
                }
                if let (Some(w), false) = (indent, items.is_empty()) {
                    out.push('\n');
                    out.push_str(&" ".repeat(w * level));
                }
                out.push(']');
            }
            J::O(items) => {
                out.push('{');
                for (i, (k, v)) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    if let Some(w) = indent {
                        out.push('\n');
                        out.push_str(&" ".repeat(w * (level + 1)));
                    }
                    J::S(k.clone()).write(out, indent, level + 1);
                    out.push(':');
                    if indent.is_some() {
                        out.push(' ');
                    }
                    v.write(out, indent, level + 1);
                }
                if let (Some(w), false) = (indent, items.is_empty()) {
                    out.push('\n');
                    out.push_str(&" ".repeat(w * level));
                }
                out.push('}');
            }
        }
    }
    fn compact(&self) -> String {
        let mut s = String::new();
        self.write(&mut s, None, 0);
        s
    }
    fn pretty(&self) -> String {
        let mut s = String::new();
        self.write(&mut s, Some(2), 0);
        s
    }
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.i < self.b.len() && (self.b[self.i] as char).is_whitespace() {
            self.i += 1;
        }
    }
    fn value(&mut self) -> Result<J, String> {
        self.ws();
        match self.b.get(self.i).copied() {
            None => Err("unexpected end".into()),
            Some(b'{') => {
                self.i += 1;
                let mut items = Vec::new();
                loop {
                    self.ws();
                    if self.b[self.i] == b'}' {
                        self.i += 1;
                        return Ok(J::O(items));
                    }
                    let k = match self.value()? {
                        J::S(s) => s,
                        _ => return Err("object key".into()),
                    };
                    self.ws();
                    if self.b[self.i] != b':' {
                        return Err("expected :".into());
                    }
                    self.i += 1;
                    let v = self.value()?;
                    items.push((k, v));
                    self.ws();
                    if self.b[self.i] == b',' {
                        self.i += 1;
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut items = Vec::new();
                loop {
                    self.ws();
                    if self.b[self.i] == b']' {
                        self.i += 1;
                        return Ok(J::A(items));
                    }
                    items.push(self.value()?);
                    self.ws();
                    if self.b[self.i] == b',' {
                        self.i += 1;
                    }
                }
            }
            Some(b'"') => {
                self.i += 1;
                let mut s = String::new();
                while self.b[self.i] != b'"' {
                    if self.b[self.i] == b'\\' {
                        self.i += 1;
                        match self.b[self.i] {
                            b'n' => s.push('\n'),
                            b'u' => {
                                let h = std::str::from_utf8(&self.b[self.i + 1..self.i + 5])
                                    .map_err(|e| e.to_string())?;
                                s.push(
                                    char::from_u32(
                                        u32::from_str_radix(h, 16).map_err(|e| e.to_string())?,
                                    )
                                    .unwrap_or('?'),
                                );
                                self.i += 4;
                            }
                            c => s.push(c as char),
                        }
                    } else {
                        s.push(self.b[self.i] as char);
                    }
                    self.i += 1;
                }
                self.i += 1;
                Ok(J::S(s))
            }
            Some(b't') => {
                self.i += 4;
                Ok(J::B(true))
            }
            Some(b'f') => {
                self.i += 5;
                Ok(J::B(false))
            }
            Some(b'n') => {
                self.i += 4;
                Ok(J::Null)
            }
            Some(_) => {
                let st = self.i;
                while self.i < self.b.len()
                    && matches!(
                        self.b[self.i],
                        b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'
                    )
                {
                    self.i += 1;
                }
                std::str::from_utf8(&self.b[st..self.i])
                    .map_err(|e| e.to_string())?
                    .parse::<f64>()
                    .map(J::N)
                    .map_err(|e| format!("number: {e}"))
            }
        }
    }
}

fn parse_json(s: &str) -> Result<J, String> {
    Parser {
        b: s.as_bytes(),
        i: 0,
    }
    .value()
}

// ── PRNG, hashing, number theory ───────────────────────────────────

fn mix64(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(mix64(seed ^ 0xD1B5_4A32_D192_ED03))
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in `[0, n)`.
    fn below(&mut self, n: u64) -> u64 {
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.below(hi - lo)
    }
}

#[derive(Default, Clone, Copy)]
struct Fx(u64);

impl Hasher for Fx {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x517C_C1B7_2722_0A95);
        self.0 ^= self.0 >> 29;
    }
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64)
    }
}

type FxMap<V> = HashMap<u64, V, BuildHasherDefault<Fx>>;

fn fxmap<V>(cap: usize) -> FxMap<V> {
    HashMap::with_capacity_and_hasher(cap, BuildHasherDefault::<Fx>::default())
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod_plain(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut acc = 1u64 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, b, m);
        }
        b = mulmod(b, b, m);
        e >>= 1;
    }
    acc
}

fn invmod(a: u64, m: u64) -> u64 {
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut r, mut nr) = (m as i128, (a % m) as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    assert!(r == 1, "no inverse of {a} mod {m}");
    if t < 0 {
        t += m as i128;
    }
    t as u64
}

fn is_prime(n: u64) -> bool {
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
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod_plain(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let mut s = (n as f64).sqrt() as u64;
    while s * s > n {
        s -= 1;
    }
    while (s + 1) * (s + 1) <= n {
        s += 1;
    }
    s
}

// ── F_p arithmetic with counters (p < 2^31) ────────────────────────

thread_local! {
    /// `F_p` multiplications and inversions performed by the counted field
    /// arithmetic on this thread since the last [`take_counters`].
    static CNT: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
}

fn take_counters() -> (u64, u64) {
    CNT.with(|c| c.replace((0, 0)))
}

#[inline]
fn fm(a: u64, b: u64, p: u64) -> u64 {
    CNT.with(|c| {
        let (m, i) = c.get();
        c.set((m + 1, i));
    });
    a * b % p
}

#[inline]
fn finv(a: u64, p: u64) -> u64 {
    CNT.with(|c| {
        let (m, i) = c.get();
        c.set((m, i + 1));
    });
    invmod(a, p)
}

fn fpow(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut acc = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            acc = fm(acc, b, p);
        }
        b = fm(b, b, p);
        e >>= 1;
    }
    acc
}

fn fadd(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}

fn fsub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}

/// Tonelli–Shanks over `F_p`, every multiplication counted.
fn fsqrt(n: u64, p: u64) -> Option<u64> {
    let n = n % p;
    if n == 0 {
        return Some(0);
    }
    if fpow(n, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(fpow(n, (p + 1) / 4, p));
    }
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while fpow(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = fpow(z, q, p);
    let mut t = fpow(n, q, p);
    let mut r = fpow(n, q.div_ceil(2), p);
    loop {
        if t == 1 {
            return Some(r);
        }
        let mut i = 0u32;
        let mut tt = t;
        while tt != 1 {
            tt = fm(tt, tt, p);
            i += 1;
            if i == m {
                return None;
            }
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = fm(b, b, p);
        }
        m = i;
        c = fm(b, b, p);
        t = fm(t, c, p);
        r = fm(r, b, p);
    }
}

// ── F_{p²} = F_p[s]/(s² − ν) ───────────────────────────────────────

type El = [u64; 2];

#[derive(Clone, Copy, Debug)]
struct Fp2 {
    p: u64,
    nu: u64,
}

impl Fp2 {
    fn new(p: u64) -> Fp2 {
        assert!(p > 2 && p < (1 << 31) && is_prime(p));
        let nu = (2..p)
            .find(|&n| powmod_plain(n, (p - 1) / 2, p) == p - 1)
            .expect("non-residue");
        Fp2 { p, nu }
    }
    fn zero(&self) -> El {
        [0, 0]
    }
    fn one(&self) -> El {
        [1, 0]
    }
    fn is_zero(&self, a: El) -> bool {
        a == [0, 0]
    }
    fn add(&self, a: El, b: El) -> El {
        [fadd(a[0], b[0], self.p), fadd(a[1], b[1], self.p)]
    }
    fn sub(&self, a: El, b: El) -> El {
        [fsub(a[0], b[0], self.p), fsub(a[1], b[1], self.p)]
    }
    fn neg(&self, a: El) -> El {
        [fsub(0, a[0], self.p), fsub(0, a[1], self.p)]
    }
    fn mul(&self, a: El, b: El) -> El {
        let p = self.p;
        let a0b0 = fm(a[0], b[0], p);
        let a1b1 = fm(a[1], b[1], p);
        let cross = fadd(fm(a[0], b[1], p), fm(a[1], b[0], p), p);
        [fadd(a0b0, fm(self.nu, a1b1, p), p), cross]
    }
    fn sqr(&self, a: El) -> El {
        self.mul(a, a)
    }
    fn scale(&self, a: El, k: u64) -> El {
        [fm(a[0], k, self.p), fm(a[1], k, self.p)]
    }
    fn norm(&self, a: El) -> u64 {
        let p = self.p;
        fsub(fm(a[0], a[0], p), fm(self.nu, fm(a[1], a[1], p), p), p)
    }
    fn frob(&self, a: El) -> El {
        [a[0], fsub(0, a[1], self.p)]
    }
    fn inv(&self, a: El) -> Option<El> {
        let n = self.norm(a);
        if n == 0 {
            return None;
        }
        let ni = finv(n, self.p);
        Some(self.scale(self.frob(a), ni))
    }
    fn pack(&self, a: El) -> u64 {
        a[1] * self.p + a[0]
    }
    fn pow(&self, a: El, mut e: u64) -> El {
        let mut acc = self.one();
        let mut b = a;
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(acc, b);
            }
            b = self.sqr(b);
            e >>= 1;
        }
        acc
    }
    /// A square in `F_{p²}` iff its norm is a square in `F_p`.
    fn is_square(&self, a: El) -> bool {
        self.is_zero(a) || powmod_plain(self.norm(a), (self.p - 1) / 2, self.p) == 1
    }
    /// The library's closed form: `(x₀ + x₁ s)² = a`, `x₀² = (a₀ ± √N)/2`.
    fn sqrt(&self, a: El) -> Option<El> {
        let p = self.p;
        if self.is_zero(a) {
            return Some(self.zero());
        }
        if a[1] == 0 {
            return match fsqrt(a[0], p) {
                Some(r) => Some([r, 0]),
                None => fsqrt(fm(a[0], finv(self.nu, p), p), p).map(|r| [0, r]),
            };
        }
        let n = fsqrt(self.norm(a), p)?;
        let half = finv(2, p);
        for sign in [n, fsub(0, n, p)] {
            let x0_sq = fm(fadd(a[0], sign, p), half, p);
            if let Some(x0) = fsqrt(x0_sq, p) {
                if x0 == 0 {
                    continue;
                }
                let x1 = fm(a[1], finv(fm(2, x0, p), p), p);
                let root = [x0, x1];
                if self.sqr(root) == a {
                    return Some(root);
                }
            }
        }
        None
    }
}

// ── The curve y² = x³ + ax + b over F_{p²} ─────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pt {
    x: El,
    y: El,
    inf: bool,
}

impl Pt {
    fn o() -> Pt {
        Pt {
            x: [0, 0],
            y: [0, 0],
            inf: true,
        }
    }
    fn aff(x: El, y: El) -> Pt {
        Pt { x, y, inf: false }
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Ops {
    adds: u64,
    doubles: u64,
}

impl Ops {
    fn gae(&self) -> f64 {
        (self.adds + self.doubles) as f64
    }
}

#[derive(Clone, Copy, Debug)]
struct Curve {
    f: Fp2,
    a: El,
    b: El,
}

impl Curve {
    fn rhs(&self, x: El) -> El {
        let f = &self.f;
        f.add(f.add(f.mul(f.sqr(x), x), f.mul(self.a, x)), self.b)
    }
    fn on_curve(&self, p: Pt) -> bool {
        p.inf || self.f.sqr(p.y) == self.rhs(p.x)
    }
    fn lift_x(&self, x: El) -> Vec<Pt> {
        match self.f.sqrt(self.rhs(x)) {
            None => Vec::new(),
            Some(y) if self.f.is_zero(y) => vec![Pt::aff(x, y)],
            Some(y) => vec![Pt::aff(x, y), Pt::aff(x, self.f.neg(y))],
        }
    }
    fn neg(&self, p: Pt) -> Pt {
        if p.inf {
            p
        } else {
            Pt::aff(p.x, self.f.neg(p.y))
        }
    }
    fn key(&self, p: &Pt) -> u64 {
        if p.inf {
            return 0;
        }
        let sign = u64::from(self.f.pack(p.y) > self.f.pack(self.f.neg(p.y)));
        ((self.f.pack(p.x) + 1) << 1) | sign
    }
    fn add_raw(&self, a: Pt, b: Pt) -> Pt {
        if a.inf {
            return b;
        }
        if b.inf {
            return a;
        }
        let f = &self.f;
        if a.x == b.x {
            if f.is_zero(f.add(a.y, b.y)) {
                return Pt::o();
            }
            return self.double_raw(a);
        }
        let lambda = f.mul(f.sub(b.y, a.y), f.inv(f.sub(b.x, a.x)).expect("distinct x"));
        let x3 = f.sub(f.sub(f.sqr(lambda), a.x), b.x);
        let y3 = f.sub(f.mul(lambda, f.sub(a.x, x3)), a.y);
        Pt::aff(x3, y3)
    }
    fn double_raw(&self, a: Pt) -> Pt {
        if a.inf || self.f.is_zero(a.y) {
            return Pt::o();
        }
        let f = &self.f;
        let num = f.add(f.scale(f.sqr(a.x), 3), self.a);
        let lambda = f.mul(num, f.inv(f.scale(a.y, 2)).expect("y != 0"));
        let x3 = f.sub(f.sqr(lambda), f.scale(a.x, 2));
        let y3 = f.sub(f.mul(lambda, f.sub(a.x, x3)), a.y);
        Pt::aff(x3, y3)
    }
    fn add(&self, ops: &mut Ops, a: Pt, b: Pt) -> Pt {
        ops.adds += 1;
        self.add_raw(a, b)
    }
    fn dbl(&self, ops: &mut Ops, a: Pt) -> Pt {
        ops.doubles += 1;
        self.double_raw(a)
    }
    fn mul(&self, ops: &mut Ops, p: Pt, k: u64) -> Pt {
        if k == 0 || p.inf {
            return Pt::o();
        }
        let bits = 64 - k.leading_zeros();
        let mut acc = p;
        for i in (0..bits - 1).rev() {
            acc = self.dbl(ops, acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(ops, acc, p);
            }
        }
        acc
    }
    fn random_point(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = [rng.below(self.f.p), rng.below(self.f.p)];
            if let Some(&pt) = self.lift_x(x).first() {
                if !self.f.is_zero(pt.y) {
                    return pt;
                }
            }
        }
    }
}

// ── Group order by baby-step giant-step in the Hasse interval ──────

fn multiples_in(curve: &Curve, pt: Pt, lo: u64, width: u64) -> Vec<u64> {
    let mut ops = Ops::default();
    let s = isqrt(width) + 1;
    let mut baby: FxMap<Vec<u64>> = fxmap(s as usize);
    let mut cur = Pt::o();
    for j in 0..s {
        baby.entry(curve.key(&curve.neg(cur))).or_default().push(j);
        cur = curve.add(&mut ops, cur, pt);
    }
    let step = curve.mul(&mut ops, pt, s);
    let mut giant = curve.mul(&mut ops, pt, lo);
    let mut out = Vec::new();
    for k in 0..=(width / s + 1) {
        if let Some(js) = baby.get(&curve.key(&giant)) {
            for &j in js {
                let m = lo + k * s + j;
                if m < lo + width && curve.mul(&mut ops, pt, m).inf {
                    out.push(m);
                }
            }
        }
        giant = curve.add(&mut ops, giant, step);
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// `#E(F_{p²})` by Mestre's method, certified on further random points.
fn curve_order(curve: &Curve, rng: &mut Rng) -> Option<u64> {
    let p = curve.f.p;
    let lo = p * p + 1 - 2 * p;
    let width = 4 * p + 1;
    let mut cand: Option<Vec<u64>> = None;
    for _ in 0..12 {
        let pt = curve.random_point(rng);
        let ms = multiples_in(curve, pt, lo, width);
        let next: Vec<u64> = match cand {
            None => ms,
            Some(c) => c.into_iter().filter(|m| ms.contains(m)).collect(),
        };
        if next.is_empty() {
            return None;
        }
        if next.len() == 1 {
            let n = next[0];
            let mut ops = Ops::default();
            for _ in 0..4 {
                let z = curve.random_point(rng);
                if !curve.mul(&mut ops, z, n).inf {
                    return None;
                }
            }
            return Some(n);
        }
        cand = Some(next);
    }
    None
}

// ── Instances ──────────────────────────────────────────────────────

struct Psi {
    cx: El,
    cy: El,
    lambda: u64,
}

struct Inst {
    family: String,
    bits: u32,
    idx: u64,
    p: u64,
    curve: Curve,
    n: u64,
    r: u64,
    h: u64,
    g: Pt,
    q: Pt,
    planted: u64,
    /// Which planted logarithm this is (several targets share one curve).
    target: u64,
    /// `(x₀, c)`: the rational 2-torsion point `T = (x₀, 0)` and a square
    /// root `c` of `f'(x₀)`; torsion family only.
    torsion: Option<(El, El)>,
    /// GLS family: the line direction `u·s` and the endomorphism `ψ`.
    gls: Option<(El, Psi)>,
    name: String,
}

fn family_code(f: &str) -> u64 {
    match f {
        "prime" => 1,
        "torsion" => 2,
        "gls" => 3,
        _ => 9,
    }
}

fn instance_rng(family: &str, bits: u32, idx: u64, salt: u64) -> Rng {
    Rng::new(mix64(
        0xB11D_5907_0001_0000
            ^ (family_code(family) << 56)
            ^ ((bits as u64) << 40)
            ^ (idx << 16)
            ^ salt,
    ))
}

fn prime_range(bits: u32) -> (u64, u64) {
    let top = 1u64 << (bits / 2);
    ((top as f64 * 0.72) as u64, top)
}

fn random_prime_in(lo: u64, hi: u64, rng: &mut Rng) -> u64 {
    loop {
        let c = rng.range(lo, hi) | 1;
        if is_prime(c) {
            return c;
        }
    }
}

fn rand_el(f: &Fp2, rng: &mut Rng) -> El {
    [rng.below(f.p), rng.below(f.p)]
}

#[allow(clippy::too_many_arguments)]
fn finish_instance(
    family: &str,
    bits: u32,
    idx: u64,
    curve: Curve,
    n: u64,
    r: u64,
    h: u64,
    torsion: Option<(El, El)>,
    gls: Option<(El, Psi)>,
    rng: &mut Rng,
) -> Result<Inst, String> {
    let mut ops = Ops::default();
    let g = loop {
        let z = curve.mul(&mut ops, curve.random_point(rng), h);
        if !z.inf {
            break z;
        }
    };
    if !curve.mul(&mut ops, g, r).inf {
        return Err("[r]G != O".into());
    }
    let p = curve.f.p;
    let mut inst = Inst {
        family: family.into(),
        bits,
        idx,
        p,
        name: format!("fp2-{family}-b{bits}-i{idx}-p{p}"),
        curve,
        n,
        r,
        h,
        g,
        q: Pt::o(),
        planted: 0,
        target: 0,
        torsion,
        gls,
    };
    inst.set_target(0);
    Ok(inst)
}

impl Inst {
    /// Plant a logarithm `d` (deterministic in the curve and the target
    /// index) and set `Q = [d]G`.  No arm sees `d`.
    fn set_target(&mut self, t: u64) {
        let mut rng = instance_rng(&self.family, self.bits, self.idx, 1000 + t);
        let mut ops = Ops::default();
        self.target = t;
        self.planted = rng.range(2, self.r - 1);
        self.q = self.curve.mul(&mut ops, self.g, self.planted);
    }
}

/// `family = prime`: a random ordinary curve of prime order.
/// `family = torsion`: `#E = 4·r`, `r` prime, with a rational 2-torsion
/// point `T = (x₀, 0)` whose `f'(x₀)` is a square in `F_{p²}`.
fn build_random_instance(family: &str, bits: u32, idx: u64) -> Result<Inst, String> {
    let mut rng = instance_rng(family, bits, idx, 0);
    let (plo, phi) = prime_range(bits);
    for _ in 0..200 {
        let p = random_prime_in(plo, phi, &mut rng);
        let f = Fp2::new(p);
        for _ in 0..4000 {
            let (curve, torsion) = if family == "prime" {
                (
                    Curve {
                        f,
                        a: rand_el(&f, &mut rng),
                        b: rand_el(&f, &mut rng),
                    },
                    None,
                )
            } else {
                let a = rand_el(&f, &mut rng);
                let x0 = rand_el(&f, &mut rng);
                let b = f.neg(f.add(f.mul(f.sqr(x0), x0), f.mul(a, x0)));
                let kappa = f.add(f.scale(f.sqr(x0), 3), a);
                if f.is_zero(kappa) || !f.is_square(kappa) {
                    continue;
                }
                let c = f.sqrt(kappa).ok_or("kappa is a square")?;
                (Curve { f, a, b }, Some((x0, c)))
            };
            let disc = f.add(
                f.scale(f.mul(f.sqr(curve.a), curve.a), 4),
                f.scale(f.sqr(curve.b), 27),
            );
            if f.is_zero(disc) {
                continue;
            }
            let Some(n) = curve_order(&curve, &mut rng) else {
                continue;
            };
            let (r, h) = if family == "prime" {
                if !is_prime(n) {
                    continue;
                }
                (n, 1)
            } else {
                if n % 4 != 0 || !is_prime(n / 4) || n / 4 <= 5 {
                    continue;
                }
                (n / 4, 4)
            };
            return finish_instance(family, bits, idx, curve, n, r, h, torsion, None, &mut rng);
        }
    }
    Err(format!("no {family} instance at {bits} bits"))
}

/// A GLS curve (Galbraith–Lin–Scott): the quadratic twist over `F_{p²}` of
/// a curve over `F_p`, `#E' = (p − 1)² + t²`, with the order-4 endomorphism
/// `ψ(x, y) = (u^{1−p} x^p, u^{3(1−p)/2} y^p)`, `ψ² = −1`.
fn build_gls_instance(bits: u32, idx: u64) -> Result<Inst, String> {
    let mut rng = instance_rng("gls", bits, idx, 0);
    let (plo, phi) = prime_range(bits);
    for _ in 0..400 {
        let p = random_prime_in(plo, phi, &mut rng);
        let f = Fp2::new(p);
        let (a, b) = (rng.below(p), rng.below(p));
        let disc = fadd(fm(4, fm(fm(a, a, p), a, p), p), fm(27, fm(b, b, p), p), p);
        if disc == 0 {
            continue;
        }
        // #E(F_p) by the quadratic character.
        let mut square = vec![false; p as usize];
        for y in 0..p {
            square[(y * y % p) as usize] = true;
        }
        let mut count = 1u64;
        for x in 0..p {
            let v = (x * x % p * x + a * x + b) % p;
            count += if v == 0 {
                1
            } else if square[v as usize] {
                2
            } else {
                0
            };
        }
        let t = p as i64 + 1 - count as i64;
        if t == 0 {
            continue;
        }
        let n = (p - 1) * (p - 1) + (t * t) as u64;
        let Some((r, h)) = (1..=4u64)
            .find_map(|h| (n % h == 0 && is_prime(n / h) && n / h > 5).then_some((n / h, h)))
        else {
            continue;
        };
        let u = loop {
            let c = rand_el(&f, &mut rng);
            if !f.is_zero(c) && !f.is_square(c) {
                break c;
            }
        };
        let u2 = f.sqr(u);
        let u3 = f.mul(u2, u);
        let curve = Curve {
            f,
            a: f.scale(u2, a),
            b: f.scale(u3, b),
        };
        let mut ops = Ops::default();
        let certified = (0..3).all(|_| {
            let z = curve.random_point(&mut rng);
            curve.mul(&mut ops, z, n).inf
        });
        if !certified {
            continue;
        }
        // ψ and its eigenvalue.
        let uinv = f.inv(u).unwrap();
        let cx = f.mul(u, f.inv(f.frob(u)).unwrap());
        let half = f.pow(uinv, (p - 1) / 2);
        let cy = f.mul(f.sqr(half), half);
        let apply = |z: Pt| -> Pt {
            if z.inf {
                z
            } else {
                Pt::aff(f.mul(cx, f.frob(z.x)), f.mul(cy, f.frob(z.y)))
            }
        };
        let probe = curve.random_point(&mut rng);
        if !curve.on_curve(apply(probe)) || curve.neg(probe) != apply(apply(probe)) {
            continue;
        }
        let mut inst = finish_instance("gls", bits, idx, curve, n, r, h, None, None, &mut rng)?;
        let g = inst.g;
        let image = apply(g);
        // λ² ≡ −1 (mod r): the two square roots of −1.
        let i = (2..r).find_map(|z| {
            let w = powmod_plain(z, (r - 1) / 4, r);
            (mulmod(w, w, r) == r - 1).then_some(w)
        });
        let Some(i) = i else { continue };
        let lambda = [i, r - i]
            .into_iter()
            .find(|&l| curve.mul(&mut ops, g, l) == image);
        let Some(lambda) = lambda else { continue };
        inst.gls = Some((f.mul(u, [0, 1]), Psi { cx, cy, lambda }));
        return Ok(inst);
    }
    Err(format!("no gls instance at {bits} bits"))
}

// ── Pricing ────────────────────────────────────────────────────────

#[derive(Default)]
struct Meter {
    acc: BTreeMap<&'static str, (u64, u64)>,
}

impl Meter {
    fn lap(&mut self, phase: &'static str) {
        let (m, i) = take_counters();
        let e = self.acc.entry(phase).or_insert((0, 0));
        e.0 += m;
        e.1 += i;
    }
    fn mult_eq(&self, phase: &str, inv_cost: f64) -> f64 {
        self.acc
            .get(phase)
            .map(|&(m, i)| m as f64 + inv_cost * i as f64)
            .unwrap_or(0.0)
    }
}

/// Mean `F_p` multiplications and inversions of one affine addition of two
/// random distinct points (the conversion to GAE).
fn measure_c_add(curve: &Curve, seed: u64) -> (f64, f64) {
    let mut rng = Rng::new(seed ^ 0xC0DE_ADD1);
    let pts: Vec<Pt> = (0..96).map(|_| curve.random_point(&mut rng)).collect();
    let mut ops = Ops::default();
    let _ = take_counters();
    let mut n = 0u64;
    for i in 0..pts.len() {
        for j in 0..pts.len() {
            if i != j && pts[i].x != pts[j].x {
                let _ = curve.add(&mut ops, pts[i], pts[j]);
                n += 1;
            }
        }
    }
    let (m, i) = take_counters();
    (m as f64 / n as f64, i as f64 / n as f64)
}

// ── Polynomials over F_p (counted) ─────────────────────────────────

fn ptrim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn pmul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = fadd(out[i + j], fm(x, y, p), p);
        }
    }
    ptrim(out)
}

/// `a mod f` with the inverse of the leading coefficient of `f` supplied.
fn prem(a: Vec<u64>, f: &[u64], inv_lead: u64, p: u64) -> Vec<u64> {
    let mut a = ptrim(a);
    let df = f.len() - 1;
    while a.len() > df {
        let dq = a.len() - 1;
        let c = fm(a[dq], inv_lead, p);
        for k in 0..=df {
            a[dq - df + k] = fsub(a[dq - df + k], fm(c, f[k], p), p);
        }
        a.pop();
        a = ptrim(a);
    }
    a
}

fn ppowmod(base: &[u64], mut e: u64, f: &[u64], p: u64) -> Vec<u64> {
    let inv_lead = finv(*f.last().unwrap(), p);
    let mut acc = vec![1u64];
    let mut b = prem(base.to_vec(), f, inv_lead, p);
    while e > 0 {
        if e & 1 == 1 {
            acc = prem(pmul(&acc, &b, p), f, inv_lead, p);
        }
        e >>= 1;
        if e > 0 {
            b = prem(pmul(&b, &b, p), f, inv_lead, p);
        }
    }
    acc
}

fn pgcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let (mut a, mut b) = (ptrim(a.to_vec()), ptrim(b.to_vec()));
    while !b.is_empty() {
        let inv_lead = finv(*b.last().unwrap(), p);
        let r = prem(a, &b, inv_lead, p);
        a = b;
        b = r;
    }
    a
}

/// Roots in `F_p` of a polynomial, by `gcd(x^p − x, f)` and Cantor–Zassenhaus.
fn proots(f: &[u64], p: u64, seed: u64) -> Vec<u64> {
    let f = ptrim(f.to_vec());
    if f.len() <= 1 {
        return Vec::new();
    }
    if f.len() == 2 {
        return vec![fsub(0, fm(f[0], finv(f[1], p), p), p)];
    }
    let mut xp = ppowmod(&[0, 1], p, &f, p);
    if xp.len() < 2 {
        xp.resize(2, 0);
    }
    xp[1] = fsub(xp[1], 1, p);
    let g = pgcd(&f, &ptrim(xp), p);
    let mut rng = Rng::new(seed ^ 0x524F_4F54);
    let mut out = Vec::new();
    let mut stack = vec![g];
    while let Some(h) = stack.pop() {
        match h.len() {
            0 | 1 => {}
            2 => out.push(fsub(0, fm(h[0], finv(h[1], p), p), p)),
            _ => {
                let delta = rng.below(p);
                let mut s = ppowmod(&[delta, 1], (p - 1) / 2, &h, p);
                if s.is_empty() {
                    s.push(0);
                }
                s[0] = fsub(s[0], 1, p);
                let d = pgcd(&h, &ptrim(s), p);
                if d.len() <= 1 || d.len() == h.len() {
                    stack.push(h);
                    continue;
                }
                let mut q = h.clone();
                let mut quotient = vec![0u64; h.len() - d.len() + 1];
                let inv_lead = finv(*d.last().unwrap(), p);
                while q.len() >= d.len() && !q.is_empty() {
                    let dq = q.len() - 1;
                    let c = fm(q[dq], inv_lead, p);
                    quotient[dq - (d.len() - 1)] = c;
                    for k in 0..d.len() {
                        q[dq - (d.len() - 1) + k] =
                            fsub(q[dq - (d.len() - 1) + k], fm(c, d[k], p), p);
                    }
                    q.pop();
                    q = ptrim(q);
                }
                stack.push(d);
                stack.push(ptrim(quotient));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn eval_poly(c: &[u64], t: u64, p: u64) -> u64 {
    let mut acc = 0u64;
    for &ci in c.iter().rev() {
        acc = fadd(fm(acc, t, p), ci, p);
    }
    acc
}

fn det4(m: &[[u64; 4]; 4], p: u64) -> u64 {
    let mut a = *m;
    let mut det = 1u64;
    for col in 0..4 {
        let Some(piv) = (col..4).find(|&r| a[r][col] != 0) else {
            return 0;
        };
        if piv != col {
            a.swap(piv, col);
            det = fsub(0, det, p);
        }
        det = fm(det, a[col][col], p);
        let inv = finv(a[col][col], p);
        for r in (col + 1)..4 {
            if a[r][col] == 0 {
                continue;
            }
            let factor = fm(a[r][col], inv, p);
            for c in col..4 {
                a[r][c] = fsub(a[r][c], fm(factor, a[col][c], p), p);
            }
        }
    }
    det
}

/// Newton interpolation through `(0, v₀), …, (n − 1, v_{n−1})`.  The inverses
/// of the integers `1..n` are constants and are not counted.
fn interpolate(values: &[u64], p: u64) -> Vec<u64> {
    let n = values.len();
    let mut dd: Vec<u64> = values.to_vec();
    let mut coef = vec![dd[0]];
    for level in 1..n {
        let inv = invmod(level as u64, p);
        for i in 0..(n - level) {
            dd[i] = fm(fsub(dd[i + 1], dd[i], p), inv, p);
        }
        coef.push(dd[0]);
    }
    let mut out = vec![0u64; n];
    let mut basis = vec![1u64];
    for (k, &c) in coef.iter().enumerate() {
        for (i, &bi) in basis.iter().enumerate() {
            out[i] = fadd(out[i], fm(c, bi, p), p);
        }
        let mut next = vec![0u64; basis.len() + 1];
        for (i, &bi) in basis.iter().enumerate() {
            next[i + 1] = fadd(next[i + 1], bi, p);
            next[i] = fsub(next[i], fm(bi, k as u64, p), p);
        }
        basis = next;
    }
    ptrim(out)
}

// ── Folded factor bases ────────────────────────────────────────────

struct Fb {
    points: Vec<Pt>,
    col_of: Vec<usize>,
    coef_of: Vec<u64>,
    columns: usize,
    /// Packed abscissa → indices of the points above it.
    x_index: FxMap<Vec<u32>>,
    point_index: FxMap<u32>,
}

type Gen<'a> = (&'a dyn Fn(Pt) -> Pt, u64);

/// The fold of `glv_invariant_base::fold_by_endomorphisms` under
/// `Closure::Strict`: orbits of `seed` under the generators (each with its
/// eigenvalue on `⟨G⟩`); an image outside the base, or reached with another
/// coefficient, is an error.  Points of order dividing `h` are dropped.
fn fold(curve: &Curve, r: u64, h: u64, seed: Vec<Pt>, gens: &[Gen]) -> Result<Fb, String> {
    let mut ops = Ops::default();
    let mut points: Vec<Pt> = Vec::with_capacity(seed.len());
    let mut index: FxMap<u32> = fxmap(seed.len() * 2);
    for p in seed {
        if p.inf {
            continue;
        }
        if h > 1 && curve.mul(&mut ops, p, h).inf {
            continue;
        }
        let k = curve.key(&p);
        if let std::collections::hash_map::Entry::Vacant(e) = index.entry(k) {
            e.insert(points.len() as u32);
            points.push(p);
        }
    }
    let n = points.len();
    let mut col_of: Vec<Option<usize>> = vec![None; n];
    let mut coef_of: Vec<u64> = vec![0; n];
    let mut orbits = 0usize;
    for i in 0..n {
        if col_of[i].is_some() {
            continue;
        }
        let orbit = orbits;
        orbits += 1;
        col_of[i] = Some(orbit);
        coef_of[i] = 1;
        let mut queue = std::collections::VecDeque::from([i]);
        let mut size = 0usize;
        while let Some(j) = queue.pop_front() {
            size += 1;
            if size > 16 {
                return Err("an orbit exceeds 16 points".into());
            }
            for (gen, eig) in gens {
                let img = gen(points[j]);
                if img.inf {
                    return Err("a generator maps a base point to the identity".into());
                }
                let coef = mulmod(coef_of[j], *eig, r);
                match index.get(&curve.key(&img)).copied() {
                    Some(t) => {
                        let t = t as usize;
                        match col_of[t] {
                            Some(o) => {
                                if o != orbit || coef_of[t] != coef {
                                    return Err(format!(
                                        "generators do not act as their eigenvalues (point {t})"
                                    ));
                                }
                            }
                            None => {
                                col_of[t] = Some(orbit);
                                coef_of[t] = coef;
                                queue.push_back(t);
                            }
                        }
                    }
                    None => {
                        return Err(
                            "a generator maps a base point outside the base (not invariant)".into(),
                        )
                    }
                }
            }
        }
    }
    let col_of: Vec<usize> = col_of.into_iter().map(|c| c.expect("assigned")).collect();
    let mut x_index: FxMap<Vec<u32>> = fxmap(n);
    for (i, p) in points.iter().enumerate() {
        x_index.entry(curve.f.pack(p.x)).or_default().push(i as u32);
    }
    Ok(Fb {
        points,
        col_of,
        coef_of,
        columns: orbits,
        x_index,
        point_index: index,
    })
}

// ── The two-summand line oracle ────────────────────────────────────

/// Semaev's `S₃` for `y² = x³ + ax + b`.
fn semaev_s3(f: &Fp2, a: El, b: El, x1: El, x2: El, x3: El) -> El {
    let d = f.sub(x1, x2);
    let sum = f.add(x1, x2);
    let prod = f.mul(x1, x2);
    let term1 = f.mul(f.sqr(d), f.sqr(x3));
    let inner = f.add(f.mul(sum, f.add(prod, a)), f.scale(b, 2));
    let term2 = f.mul(f.scale(inner, 2), x3);
    let term3 = f.sqr(f.sub(prod, a));
    let term4 = f.mul(f.scale(b, 4), sum);
    f.sub(f.add(f.sub(term1, term2), term3), term4)
}

fn quadratic_from_values(f: &Fp2, v: [El; 3]) -> [El; 3] {
    let half = invmod(2, f.p);
    let c2 = f.scale(f.add(f.sub(v[0], f.scale(v[1], 2)), v[2]), half);
    let c1 = f.scale(
        f.sub(f.add(f.scale(v[1], 4), f.neg(f.scale(v[0], 3))), v[2]),
        half,
    );
    [v[0], c1, c2]
}

/// `S₃(x₀ + c t₁, x₀ + c t₂, x_R) = 0`, Weil-descended to two equations in
/// `(t₁, t₂) ∈ F_p²`, a resultant in `t₂` evaluated at nine points of `t₁`
/// and interpolated, its roots by `gcd(t^p − t, ·)`, the partner `t₂` from
/// the common roots, and the pair lifted to signed base points.  With
/// `x₀ = 0` it is `line_oracle::LineOracle` (the smoke test compares the
/// two on exhaustive enumeration); the shifted line carries the
/// `τ_T`-closed base.
struct Line2 {
    f: Fp2,
    x0: El,
    c: El,
    /// Solve the system through the FGHR symmetry `t ↦ 1/t` of the
    /// `τ_T`-closed line (see [`Line2::solve_sym`]) instead of the degree-8
    /// resultant.
    sym: bool,
    calls: u64,
    degenerate: u64,
    unliftable: u64,
    lift_failures: u64,
    lookups: u64,
    /// Calls the symmetric solver handed to the resultant solver because the
    /// polynomial was not palindromic and swap-symmetric, or the linear
    /// system was degenerate.
    sym_fallbacks: u64,
}

impl Line2 {
    fn new(f: Fp2, x0: El, c: El) -> Line2 {
        Line2 {
            f,
            x0,
            c,
            sym: false,
            calls: 0,
            degenerate: 0,
            unliftable: 0,
            lift_failures: 0,
            lookups: 0,
            sym_fallbacks: 0,
        }
    }
    fn x_of(&self, t: u64) -> El {
        self.f.add(self.x0, self.f.scale(self.c, t))
    }
    fn descend(&self, curve: &Curve, x_r: El) -> [[[u64; 3]; 3]; 2] {
        let f = &self.f;
        let xs = [self.x_of(0), self.x_of(1), self.x_of(2)];
        let mut grid = [[f.zero(); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                grid[i][j] = semaev_s3(f, curve.a, curve.b, xs[i], xs[j], x_r);
            }
        }
        let mut by_j = [[f.zero(); 3]; 3];
        for j in 0..3 {
            by_j[j] = quadratic_from_values(f, [grid[0][j], grid[1][j], grid[2][j]]);
        }
        let mut m = [[f.zero(); 3]; 3];
        for i in 0..3 {
            m[i] = quadratic_from_values(f, [by_j[0][i], by_j[1][i], by_j[2][i]]);
        }
        let mut out = [[[0u64; 3]; 3]; 2];
        for i in 0..3 {
            for j in 0..3 {
                out[0][i][j] = m[i][j][0];
                out[1][i][j] = m[i][j][1];
            }
        }
        out
    }
    fn solve(&mut self, polys: &[[[u64; 3]; 3]; 2], seed: u64) -> (Vec<(u64, u64)>, bool) {
        let p = self.f.p;
        let coef_in_t1 =
            |q: &[[u64; 3]; 3], j: usize| -> Vec<u64> { vec![q[0][j], q[1][j], q[2][j]] };
        let nonzero: Vec<&[[u64; 3]; 3]> = polys
            .iter()
            .filter(|q| q.iter().any(|row| row.iter().any(|&v| v != 0)))
            .collect();
        if nonzero.len() < 2 {
            return (Vec::new(), true);
        }
        let (f0, f1) = (nonzero[0], nonzero[1]);
        let mut values = Vec::with_capacity(9);
        for t1 in 0..9u64 {
            let a: Vec<u64> = (0..3)
                .map(|j| eval_poly(&coef_in_t1(f0, j), t1, p))
                .collect();
            let b: Vec<u64> = (0..3)
                .map(|j| eval_poly(&coef_in_t1(f1, j), t1, p))
                .collect();
            let m = [
                [a[2], a[1], a[0], 0],
                [0, a[2], a[1], a[0]],
                [b[2], b[1], b[0], 0],
                [0, b[2], b[1], b[0]],
            ];
            values.push(det4(&m, p));
        }
        let res = interpolate(&values, p);
        if res.is_empty() {
            return (Vec::new(), true);
        }
        let t1s = proots(&res, p, seed);
        let mut pairs = Vec::new();
        for t1 in t1s {
            let mut cands: Option<Vec<u64>> = None;
            for q in polys {
                let a: Vec<u64> = (0..3)
                    .map(|j| eval_poly(&coef_in_t1(q, j), t1, p))
                    .collect();
                if a.iter().all(|&v| v == 0) {
                    continue;
                }
                cands = Some(proots(&a, p, seed ^ t1));
                break;
            }
            let Some(cands) = cands else { continue };
            for t2 in cands {
                let all = polys.iter().all(|q| {
                    let a: Vec<u64> = (0..3)
                        .map(|j| eval_poly(&coef_in_t1(q, j), t1, p))
                        .collect();
                    eval_poly(&a, t2, p) == 0
                });
                if all {
                    pairs.push((t1, t2));
                }
            }
        }
        (pairs, false)
    }
    /// **The FGHR symmetrisation at `m = 2`.**  On the line `x = x₀ + c t`
    /// translation by `T` is `t ↦ 1/t`, and `R ± T`-invariance of the
    /// decomposition shows `G(t₁, t₂) = G(1/t₁, 1/t₂)·t₁²t₂²` (checked on
    /// every call): the descended polynomial is a polynomial in the
    /// invariants `σᵢ = tᵢ + 1/tᵢ` and `D = (t₁ − 1/t₁)(t₂ − 1/t₂)`.  With
    /// `S = σ₁ + σ₂`, `P = σ₁σ₂`, the two `F_p` equations are linear in
    /// `(S, P, D)`, and `D² = (P + 4)² − 4S²`: a line cut by a quadric, so a
    /// quadratic over `F_p`, then square roots for `σᵢ` and `tᵢ`.  Returns
    /// `None` when the structure fails or the linear system is degenerate
    /// (the caller then uses the resultant), `Some(pairs)` otherwise.
    fn solve_sym(&mut self, polys: &[[[u64; 3]; 3]; 2]) -> Option<Vec<(u64, u64)>> {
        let p = self.f.p;
        for q in polys.iter() {
            for i in 0..3 {
                for j in 0..3 {
                    if q[i][j] != q[2 - i][2 - j] || q[i][j] != q[j][i] {
                        return None;
                    }
                }
            }
        }
        let g = |i: usize, j: usize| -> El { [polys[0][i][j], polys[1][i][j]] };
        let f = self.f;
        // 2·(g00 + g02) P + ... : (g00 + g02) P + (g00 − g02) D + 2 g01 S + 2 g11 = 0.
        let c_p = f.add(g(0, 0), g(0, 2));
        let c_d = f.sub(g(0, 0), g(0, 2));
        let c_s = f.add(g(0, 1), g(0, 1));
        let c_0 = f.add(g(1, 1), g(1, 1));
        // Unknown order (S, P, D); rows are the real and imaginary parts.
        let l = [[c_s[0], c_p[0], c_d[0]], [c_s[1], c_p[1], c_d[1]]];
        let e = [fsub(0, c_0[0], p), fsub(0, c_0[1], p)];
        let cross = |a: usize, b: usize| fsub(fm(l[0][a], l[1][b], p), fm(l[0][b], l[1][a], p), p);
        let n = [cross(1, 2), cross(2, 0), cross(0, 1)];
        let k = (0..3).find(|&i| n[i] != 0)?;
        let (a, b) = ((k + 1) % 3, (k + 2) % 3);
        let det_inv = finv(n[k], p);
        let mut v = [0u64; 3];
        v[a] = fm(
            fsub(fm(e[0], l[1][b], p), fm(l[0][b], e[1], p), p),
            det_inv,
            p,
        );
        v[b] = fm(
            fsub(fm(l[0][a], e[1], p), fm(e[0], l[1][a], p), p),
            det_inv,
            p,
        );
        // Q(u) = D² − (P + 4)² + 4S² on u = v + λ n.
        let sq = |x: u64| fm(x, x, p);
        let q2 = fadd(fsub(sq(n[2]), sq(n[1]), p), fm(4, sq(n[0]), p), p);
        let q1 = fadd(
            fsub(
                fm(2, fsub(fm(v[2], n[2], p), fm(v[1], n[1], p), p), p),
                fm(8, n[1], p),
                p,
            ),
            fm(8, fm(v[0], n[0], p), p),
            p,
        );
        let q0 = fsub(
            fadd(fsub(sq(v[2]), sq(v[1]), p), fm(4, sq(v[0]), p), p),
            fadd(fm(8, v[1], p), 16 % p, p),
            p,
        );
        let mut lambdas: Vec<u64> = Vec::new();
        if q2 == 0 {
            if q1 == 0 {
                if q0 == 0 {
                    return None;
                }
            } else {
                lambdas.push(fm(fsub(0, q0, p), finv(q1, p), p));
            }
        } else {
            let disc = fsub(sq(q1), fm(4, fm(q2, q0, p), p), p);
            if let Some(rt) = fsqrt(disc, p) {
                let inv2a = finv(fm(2, q2, p), p);
                lambdas.push(fm(fsub(rt, q1, p), inv2a, p));
                if rt != 0 {
                    lambdas.push(fm(fsub(0, fadd(q1, rt, p), p), inv2a, p));
                }
            }
        }
        let half = (p + 1) / 2;
        let mut pairs = Vec::new();
        for lam in lambdas {
            let u = [
                fadd(v[0], fm(lam, n[0], p), p),
                fadd(v[1], fm(lam, n[1], p), p),
                fadd(v[2], fm(lam, n[2], p), p),
            ];
            let (s_, p_, d_) = (u[0], u[1], u[2]);
            let disc = fsub(sq(s_), fm(4, p_, p), p);
            let Some(rt) = fsqrt(disc, p) else { continue };
            let sigma_a = fm(fadd(s_, rt, p), half, p);
            let sigma_b = fm(fsub(s_, rt, p), half, p);
            let Some(da) = fsqrt(fsub(sq(sigma_a), 4 % p, p), p) else {
                continue;
            };
            let Some(db) = fsqrt(fsub(sq(sigma_b), 4 % p, p), p) else {
                continue;
            };
            let prod = fm(da, db, p);
            let db = if prod == d_ {
                db
            } else if prod == fsub(0, d_, p) {
                fsub(0, db, p)
            } else {
                continue;
            };
            let (t1, t2) = (
                fm(fadd(sigma_a, da, p), half, p),
                fm(fadd(sigma_b, db, p), half, p),
            );
            let (u1, u2) = (
                fm(fsub(sigma_a, da, p), half, p),
                fm(fsub(sigma_b, db, p), half, p),
            );
            pairs.push((t1, t2));
            pairs.push((u1, u2));
        }
        Some(pairs)
    }

    /// Indices of a two-point decomposition of `point` over the base.
    fn decompose(
        &mut self,
        curve: &Curve,
        fb: &Fb,
        ops: &mut Ops,
        meter: &mut Meter,
        point: Pt,
    ) -> Option<Vec<usize>> {
        self.calls += 1;
        let polys = self.descend(curve, point.x);
        let seed = curve.key(&point);
        let sym = if self.sym {
            self.solve_sym(&polys)
        } else {
            None
        };
        let (pairs, degenerate) = match sym {
            Some(pairs) => (pairs, false),
            None => {
                if self.sym {
                    self.sym_fallbacks += 1;
                }
                self.solve(&polys, seed)
            }
        };
        meter.lap("oracle");
        if degenerate {
            self.degenerate += 1;
            return None;
        }
        if pairs.is_empty() {
            return None;
        }
        for (t1, t2) in pairs {
            let (k1, k2) = (self.f.pack(self.x_of(t1)), self.f.pack(self.x_of(t2)));
            self.lookups += 2;
            if let (Some(c1), Some(c2)) = (fb.x_index.get(&k1), fb.x_index.get(&k2)) {
                for &i1 in c1 {
                    for &i2 in c2 {
                        let s = curve.add(ops, fb.points[i1 as usize], fb.points[i2 as usize]);
                        if s == point {
                            meter.lap("lift");
                            return Some(vec![i1 as usize, i2 as usize]);
                        }
                    }
                }
            }
            self.lift_failures += 1;
        }
        meter.lap("lift");
        self.unliftable += 1;
        None
    }
}

// ── Sparse linear algebra over Z/r (ports of gaudry_cubic.rs) ──────

fn mmr(a: u64, b: u64, n: u64) -> u64 {
    mulmod(a, b, n)
}

fn amr(a: u64, b: u64, n: u64) -> u64 {
    let s = a + b;
    if s >= n {
        s - n
    } else {
        s
    }
}

fn smr(a: u64, b: u64, n: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + n - b
    }
}

#[derive(Clone, Debug)]
struct SparseRel {
    cols: Vec<(usize, u64)>,
    rhs: u64,
}

impl SparseRel {
    fn sub_scaled(&self, f: u64, other: &SparseRel, n: u64, ops: &mut u64) -> SparseRel {
        let mut acc: HashMap<usize, u64> = self.cols.iter().copied().collect();
        for &(c, v) in &other.cols {
            let e = acc.entry(c).or_insert(0);
            *e = (*e + n - mmr(f, v, n)) % n;
            *ops += 1;
        }
        let mut cols: Vec<(usize, u64)> = acc.into_iter().filter(|&(_, v)| v != 0).collect();
        cols.sort_unstable();
        *ops += 1;
        SparseRel {
            cols,
            rhs: (self.rhs + n - mmr(f, other.rhs, n)) % n,
        }
    }
}

/// Structured elimination of large-prime columns (columns `≥ small`).
struct LargePrimeEliminator {
    small: usize,
    n: u64,
    pivots: HashMap<usize, SparseRel>,
}

impl LargePrimeEliminator {
    fn new(small: usize, n: u64) -> Self {
        LargePrimeEliminator {
            small,
            n,
            pivots: HashMap::new(),
        }
    }
    fn feed(&mut self, mut rel: SparseRel, ops: &mut u64) -> Option<SparseRel> {
        loop {
            let lp: Vec<(usize, u64)> = rel
                .cols
                .iter()
                .copied()
                .filter(|&(c, _)| c >= self.small)
                .collect();
            if lp.is_empty() {
                return Some(rel);
            }
            let Some(&(c, v)) = lp.iter().find(|(c, _)| self.pivots.contains_key(c)) else {
                let (c, v) = lp[0];
                let inv = invmod(v, self.n);
                *ops += rel.cols.len() as u64 + 1;
                let cols = rel
                    .cols
                    .iter()
                    .map(|&(cc, vv)| (cc, mmr(vv, inv, self.n)))
                    .collect();
                let rhs = mmr(rel.rhs, inv, self.n);
                self.pivots.insert(c, SparseRel { cols, rhs });
                return None;
            };
            let piv = self.pivots[&c].clone();
            rel = rel.sub_scaled(v, &piv, self.n, ops);
        }
    }
}

struct SquareCore {
    rows: Vec<usize>,
    columns: Vec<bool>,
}

/// Filtering to a square, singleton-free selection (`keep` is `d`'s column).
fn square_core(rels: &[SparseRel], keep: usize) -> Option<SquareCore> {
    let ncols = rels
        .iter()
        .flat_map(|r| r.cols.iter().map(|&(c, _)| c + 1))
        .max()
        .unwrap_or(0)
        .max(keep + 1);
    let mut alive: Vec<bool> = vec![true; rels.len()];
    let weights = |alive: &[bool]| -> Vec<usize> {
        let mut w = vec![0usize; ncols];
        for (i, r) in rels.iter().enumerate() {
            if alive[i] {
                for &(c, _) in &r.cols {
                    w[c] += 1;
                }
            }
        }
        w
    };
    loop {
        loop {
            let w = weights(&alive);
            let mut dropped = false;
            for (i, r) in rels.iter().enumerate() {
                if alive[i] && r.cols.iter().any(|&(c, _)| c != keep && w[c] == 1) {
                    alive[i] = false;
                    dropped = true;
                }
            }
            if !dropped {
                break;
            }
        }
        let w = weights(&alive);
        let n_rows = alive.iter().filter(|&&a| a).count();
        let n_cols = w.iter().filter(|&&x| x > 0).count();
        if n_rows == 0 || w[keep] == 0 {
            return None;
        }
        if n_rows == n_cols {
            return Some(SquareCore {
                rows: (0..rels.len()).filter(|&i| alive[i]).collect(),
                columns: (0..ncols).map(|c| w[c] > 0).collect(),
            });
        }
        if n_rows < n_cols {
            return None;
        }
        let victim = (0..rels.len()).filter(|&i| alive[i]).min_by_key(|&i| {
            rels[i]
                .cols
                .iter()
                .filter(|&&(c, _)| c != keep && w[c] == 2)
                .count()
        })?;
        alive[victim] = false;
    }
}

fn berlekamp_massey(seq: &[u64], n: u64, ops: &mut u64) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut bb = 1u64;
    for i in 0..seq.len() {
        let mut d = seq[i];
        for j in 1..=l {
            if j < c.len() {
                d = amr(d, mmr(c[j], seq[i - j], n), n);
                *ops += 1;
            }
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = mmr(d, invmod(bb, n), n);
        let t = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for j in 0..b.len() {
            c[j + m] = smr(c[j + m], mmr(coef, b[j], n), n);
            *ops += 1;
        }
        if 2 * l <= i {
            l = i + 1 - l;
            b = t;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    c.truncate(l + 1);
    c
}

/// Sequential Wiedemann for `A x = b` (rows of `A` carry `b` as `rhs`).
fn wiedemann(rows: &[SparseRel], n: u64, rng: &mut Rng, ops: &mut u64) -> Option<Vec<u64>> {
    let ncols = rows.len();
    let mat_vec = |v: &[u64], ops: &mut u64| -> Vec<u64> {
        rows.iter()
            .map(|r| {
                let mut acc = 0u64;
                for &(c, a) in &r.cols {
                    acc = amr(acc, mmr(a, v[c], n), n);
                }
                *ops += r.cols.len() as u64;
                acc
            })
            .collect()
    };
    let b: Vec<u64> = rows.iter().map(|r| r.rhs).collect();
    for _attempt in 0..3 {
        let u: Vec<u64> = (0..ncols).map(|_| rng.below(n)).collect();
        let mut seq = Vec::with_capacity(2 * ncols);
        let mut v = b.clone();
        for _ in 0..2 * ncols {
            let mut d = 0u64;
            for i in 0..ncols {
                d = amr(d, mmr(u[i], v[i], n), n);
            }
            *ops += ncols as u64;
            seq.push(d);
            v = mat_vec(&v, ops);
        }
        let c = berlekamp_massey(&seq, n, ops);
        let l = c.len() - 1;
        if l == 0 || c[l] == 0 {
            continue;
        }
        let mut acc = vec![0u64; ncols];
        for &cj in c.iter().take(l) {
            acc = mat_vec(&acc, ops);
            for i in 0..ncols {
                acc[i] = amr(acc[i], mmr(cj, b[i], n), n);
            }
            *ops += ncols as u64;
        }
        let neg_inv = (n - invmod(c[l], n)) % n;
        let x: Vec<u64> = acc.iter().map(|&a| mmr(a, neg_inv, n)).collect();
        *ops += ncols as u64;
        if mat_vec(&x, ops) == b {
            return Some(x);
        }
    }
    None
}

// ── The relation phase ─────────────────────────────────────────────

struct Walk {
    table: Vec<(Pt, u64, u64)>,
    cur: Pt,
    a: u64,
    b: u64,
    steps: u64,
}

const WALK_JUMPS: usize = 16;
const WALK_RESTART: u64 = 64;

impl Walk {
    fn new(inst: &Inst, ops: &mut Ops, rng: &mut Rng) -> Walk {
        let c = &inst.curve;
        let mut table = Vec::new();
        for _ in 0..WALK_JUMPS {
            let (a, b) = (rng.below(inst.r), rng.range(1, inst.r));
            let ga = c.mul(ops, inst.g, a);
            let qb = c.mul(ops, inst.q, b);
            table.push((c.add(ops, ga, qb), a, b));
        }
        Walk {
            table,
            cur: Pt::o(),
            a: 0,
            b: 0,
            steps: 0,
        }
    }
    /// The next target `R = [a]G + [b]Q`: one addition, plus two scalar
    /// multiplications at each restart (every `WALK_RESTART` targets, so that
    /// two walks that merge repeat at most `WALK_RESTART` targets).
    fn next(&mut self, inst: &Inst, ops: &mut Ops, rng: &mut Rng) -> (Pt, u64, u64) {
        let c = &inst.curve;
        if self.steps % WALK_RESTART == 0 || self.cur.inf {
            self.a = rng.below(inst.r);
            self.b = rng.range(1, inst.r);
            let ga = c.mul(ops, inst.g, self.a);
            let qb = c.mul(ops, inst.q, self.b);
            self.cur = c.add(ops, ga, qb);
        } else {
            let j = (mix64(c.f.pack(self.cur.x)) % WALK_JUMPS as u64) as usize;
            let (m, a, b) = self.table[j];
            self.cur = c.add(ops, self.cur, m);
            self.a = (self.a + a) % inst.r;
            self.b = (self.b + b) % inst.r;
        }
        self.steps += 1;
        (self.cur, self.a, self.b)
    }
}

/// Is there a decomposition of `rr` over the base, by exact enumeration?
fn exact_decomposes(curve: &Curve, fb: &Fb, rr: Pt) -> bool {
    let mut ops = Ops::default();
    for p in &fb.points {
        let d = curve.add(&mut ops, rr, curve.neg(*p));
        if !d.inf && fb.point_index.contains_key(&curve.key(&d)) {
            return true;
        }
    }
    false
}

struct IcCfg {
    max_lp: usize,
    lp_exp: f64,
    crosscheck: u64,
    cap_trials: u64,
}

struct IcOut {
    recovered: Option<u64>,
    verified: bool,
    detail: Vec<(&'static str, J)>,
    meter: Meter,
    natives: BTreeMap<String, u64>,
}

fn relation_row(
    fb: &Fb,
    ix: &[usize],
    colmap: &[usize],
    d_col: usize,
    h: u64,
    a: u64,
    b: u64,
    r: u64,
) -> (Vec<(usize, u64)>, u64) {
    let mut row: BTreeMap<usize, u64> = BTreeMap::new();
    for &i in ix {
        let e = row.entry(colmap[fb.col_of[i]]).or_insert(0);
        *e = (*e + fb.coef_of[i] % r) % r;
    }
    let dc = (r - mulmod(h % r, b, r)) % r;
    if dc != 0 {
        row.insert(d_col, dc);
    }
    (
        row.into_iter().filter(|&(_, v)| v != 0).collect(),
        mulmod(h % r, a, r),
    )
}

/// Try the filter and Wiedemann on the relations so far.
#[allow(clippy::too_many_arguments)]
fn try_solve(
    inst: &Inst,
    full: &[SparseRel],
    d_col: usize,
    unknowns: usize,
    la_rng: &mut Rng,
    la_ops: &mut u64,
    attempts: &mut u64,
    meter: &mut Meter,
    log: &mut Vec<(usize, usize, bool)>,
    last_failed: &mut Option<Vec<usize>>,
    skipped: &mut u64,
    wall: &mut (f64, f64),
) -> Result<(u64, usize), bool> {
    let t_filter = Instant::now();
    let core = square_core(full, d_col);
    wall.0 += t_filter.elapsed().as_secs_f64();
    let Some(core) = core else { return Err(false) };
    // A selection identical to the last one that failed carries no new
    // information: the extra relations did not reach it.
    if last_failed.as_ref() == Some(&core.rows) {
        *skipped += 1;
        return Err(false);
    }
    *attempts += 1;
    let mut map = vec![usize::MAX; unknowns];
    let mut k = 0;
    for (c, slot) in map.iter_mut().enumerate() {
        if core.columns.get(c).copied().unwrap_or(false) {
            *slot = k;
            k += 1;
        }
    }
    let sel: Vec<SparseRel> = core
        .rows
        .iter()
        .map(|&i| SparseRel {
            cols: full[i].cols.iter().map(|&(c, v)| (map[c], v)).collect(),
            rhs: full[i].rhs,
        })
        .collect();
    let t_w = Instant::now();
    let x = wiedemann(&sel, inst.r, la_rng, la_ops);
    wall.1 += t_w.elapsed().as_secs_f64();
    meter.lap("la_field");
    let Some(x) = x else {
        log.push((full.len(), sel.len(), false));
        *last_failed = Some(core.rows.clone());
        return Err(true);
    };
    let dd = x[map[d_col]];
    let mut vops = Ops::default();
    let ok = inst.curve.mul(&mut vops, inst.g, dd) == inst.q;
    meter.lap("verify");
    log.push((full.len(), sel.len(), ok));
    if ok {
        Ok((dd, sel.len()))
    } else {
        *last_failed = Some(core.rows.clone());
        Err(true)
    }
}

fn ic_solve(inst: &Inst, fb: &Fb, oracle: &mut Line2, cfg: &IcCfg, seed: u64) -> IcOut {
    let curve = &inst.curve;
    let (r, h) = (inst.r, inst.h);
    let mut rng = Rng::new(seed ^ 0x1C_A1C0);
    let mut la_rng = Rng::new(seed ^ 0x1A1A);
    let mut meter = Meter::default();
    let mut ops = Ops::default();
    let ncols = fb.columns;
    let small = if cfg.max_lp > 0 {
        ((ncols as f64).powf(cfg.lp_exp).ceil() as usize).clamp(2, ncols)
    } else {
        ncols
    };
    let mut order: Vec<usize> = (0..ncols).collect();
    order.sort_by_key(|&c| mix64(seed ^ 0x5EED ^ c as u64));
    let mut colmap = vec![0usize; ncols];
    for (rank, &c) in order.iter().enumerate() {
        colmap[c] = if rank < small { rank } else { rank + 1 };
    }
    let d_col = small;
    let unknowns = small + 1;
    let mut elim = (cfg.max_lp > 0).then(|| LargePrimeEliminator::new(small + 1, r));
    let mut walk = Walk::new(inst, &mut ops, &mut rng);
    meter.lap("targets");
    let mut full: Vec<SparseRel> = Vec::new();
    let step = (unknowns / 200).max(1);
    let mut trigger = (unknowns / 4).max(2);
    let (mut la_ops, mut merge_ops, mut la_attempts, mut filter_touches) = (0u64, 0u64, 0u64, 0u64);
    let (mut trials, mut found, mut used, mut discarded_lp) = (0u64, 0u64, 0u64, 0u64);
    let (mut cc_checked, mut cc_disagree) = (0u64, 0u64);
    let mut lp_hist = [0u64; 3];
    let mut recovered = None;
    let mut core_unknowns = 0usize;
    let mut wiedemann_failures = 0u64;
    let mut la_log: Vec<(usize, usize, bool)> = Vec::new();
    let mut last_failed: Option<Vec<usize>> = None;
    let mut la_skipped = 0u64;
    let mut la_wall = (0.0f64, 0.0f64);
    let mut seen_x: FxMap<()> = fxmap(1 << 12);
    let mut dup_targets = 0u64;
    while trials < cfg.cap_trials && recovered.is_none() {
        let (rr, a, b) = walk.next(inst, &mut ops, &mut rng);
        meter.lap("targets");
        trials += 1;
        if rr.inf {
            continue;
        }
        // A target whose abscissa was seen before is a rho collision (R = ±R'),
        // not a relation: skip it, so that no trivial collision is counted.
        if seen_x.insert(curve.f.pack(rr.x), ()).is_some() {
            dup_targets += 1;
            continue;
        }
        let dec = oracle.decompose(curve, fb, &mut ops, &mut meter, rr);
        if trials <= cfg.crosscheck {
            cc_checked += 1;
            if exact_decomposes(curve, fb, rr) != dec.is_some() {
                cc_disagree += 1;
            }
            let _ = take_counters();
        }
        let Some(ix) = dec else { continue };
        found += 1;
        let (cols, rhs) = relation_row(fb, &ix, &colmap, d_col, h, a, b, r);
        if cols.is_empty() {
            continue;
        }
        let lp = cols.iter().filter(|&&(c, _)| c > d_col).count();
        if lp > cfg.max_lp {
            discarded_lp += 1;
            continue;
        }
        used += 1;
        lp_hist[lp.min(2)] += 1;
        let rel = SparseRel { cols, rhs };
        let fullrel = if lp == 0 {
            Some(rel)
        } else {
            elim.as_mut().expect("eliminator").feed(rel, &mut merge_ops)
        };
        let Some(fr) = fullrel else { continue };
        full.push(fr);
        if full.len() < trigger {
            continue;
        }
        filter_touches += full.iter().map(|x| x.cols.len() as u64).sum::<u64>();
        match try_solve(
            inst,
            &full,
            d_col,
            unknowns,
            &mut la_rng,
            &mut la_ops,
            &mut la_attempts,
            &mut meter,
            &mut la_log,
            &mut last_failed,
            &mut la_skipped,
            &mut la_wall,
        ) {
            Ok((dd, sz)) => {
                recovered = Some(dd);
                core_unknowns = sz;
            }
            Err(failed_wiedemann) => {
                if failed_wiedemann {
                    wiedemann_failures += 1;
                }
                trigger = full.len() + step;
            }
        }
    }
    meter.lap("targets");
    let mut natives = BTreeMap::new();
    natives.insert("la_modr_ops".to_string(), la_ops);
    natives.insert("elimination_merge_modr_ops".to_string(), merge_ops);
    natives.insert("filter_nnz_touches_lower_bound".to_string(), filter_touches);
    natives.insert("abscissa_lookups".to_string(), oracle.lookups);
    natives.insert("walk_step_hashes".to_string(), trials);
    natives.insert("target_dedup_probes".to_string(), trials);
    let verified = recovered == Some(inst.planted);
    let detail = vec![
        ("duplicate_targets_skipped", J::u(dup_targets)),
        ("columns", J::u(ncols as u64)),
        ("small_columns", J::u(small as u64)),
        ("unknowns", J::u(unknowns as u64)),
        ("max_large_primes", J::u(cfg.max_lp as u64)),
        ("lp_exponent", J::f(cfg.lp_exp)),
        ("trials", J::u(trials)),
        ("decompositions_found", J::u(found)),
        ("hit_rate", J::f(found as f64 / trials.max(1) as f64)),
        ("relations_used", J::u(used)),
        ("discarded_by_large_prime_limit", J::u(discarded_lp)),
        ("full_relations", J::u(full.len() as u64)),
        (
            "relations_with_0_1_2_large_primes",
            J::A(lp_hist.iter().map(|&v| J::u(v)).collect()),
        ),
        ("la_attempts", J::u(la_attempts)),
        ("la_attempts_skipped_same_core", J::u(la_skipped)),
        ("wall_seconds_filter", J::f(la_wall.0)),
        ("wall_seconds_wiedemann", J::f(la_wall.1)),
        (
            "la_attempt_log_relations_core_ok",
            J::A(
                la_log
                    .iter()
                    .map(|&(a, b, c)| J::A(vec![J::u(a as u64), J::u(b as u64), J::B(c)]))
                    .collect(),
            ),
        ),
        ("wiedemann_failures", J::u(wiedemann_failures)),
        ("core_unknowns", J::u(core_unknowns as u64)),
        ("oracle_calls", J::u(oracle.calls)),
        ("oracle_degenerate", J::u(oracle.degenerate)),
        ("oracle_unliftable", J::u(oracle.unliftable)),
        ("oracle_lift_failures", J::u(oracle.lift_failures)),
        ("oracle_symmetric", J::B(oracle.sym)),
        ("oracle_symmetric_fallbacks", J::u(oracle.sym_fallbacks)),
        ("crosscheck_trials", J::u(cc_checked)),
        ("crosscheck_disagreements", J::u(cc_disagree)),
        ("exhausted", J::B(recovered.is_none())),
        ("group_adds", J::u(ops.adds)),
        ("group_doubles", J::u(ops.doubles)),
    ];
    IcOut {
        recovered,
        verified,
        detail,
        meter,
        natives,
    }
}

// ── Records ────────────────────────────────────────────────────────

fn priced_record(
    inst: &Inst,
    arm: &str,
    gae_priced: f64,
    natives: &BTreeMap<String, u64>,
    verified: bool,
    detail: J,
) -> J {
    let native_total: u64 = natives.values().sum();
    let sqrt_r = (inst.r as f64).sqrt();
    let mut s = Vec::new();
    let mut total = Vec::new();
    for (w, name) in BAND.iter().zip(BAND_NAMES) {
        let t = gae_priced + w * native_total as f64;
        total.push((name.to_string(), J::f(t)));
        s.push((name.to_string(), J::f(t / sqrt_r)));
    }
    J::obj(vec![
        ("schema", J::s(SCHEMA)),
        ("item", J::s("BLINDSPOT-01")),
        ("family", J::s(&inst.family)),
        ("bits", J::u(inst.bits as u64)),
        ("idx", J::u(inst.idx)),
        ("target", J::u(inst.target)),
        ("instance", J::s(&inst.name)),
        ("p", J::u(inst.p)),
        ("n", J::u(inst.n)),
        ("log2_n", J::f((inst.n as f64).log2())),
        ("r", J::u(inst.r)),
        ("log2_r", J::f((inst.r as f64).log2())),
        ("cofactor", J::u(inst.h)),
        ("arm", J::s(arm)),
        ("verified", J::B(verified)),
        ("gae_priced", J::f(gae_priced)),
        (
            "natives",
            J::O(natives.iter().map(|(k, v)| (k.clone(), J::u(*v))).collect()),
        ),
        ("native_total", J::u(native_total)),
        ("gae_total_by_weight", J::O(total)),
        ("s_by_weight", J::O(s)),
        ("detail", detail),
    ])
}

fn emit(out: &Option<String>, rec: &J, quiet: bool) {
    let line = rec.compact();
    if quiet {
        println!(
            "{} b{} i{} t{} {}: verified={} gae={:.0} S(w0)={:.2} S(w1)={:.2} wall={:.1}s",
            rec.get("family").str().unwrap_or(""),
            rec.get("bits").u64().unwrap_or(0),
            rec.get("idx").u64().unwrap_or(0),
            rec.get("target").u64().unwrap_or(0),
            rec.get("arm").str().unwrap_or(""),
            rec.get("verified").bool().unwrap_or(false),
            rec.get("gae_priced").f64().unwrap_or(0.0),
            rec.get("s_by_weight").get("w0").f64().unwrap_or(0.0),
            rec.get("s_by_weight").get("w1").f64().unwrap_or(0.0),
            rec.get("detail").get("wall_seconds").f64().unwrap_or(0.0)
        );
    } else {
        println!("{line}");
    }
    if let Some(path) = out {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| panic!("open {path}: {e}"));
        writeln!(f, "{line}").expect("write");
    }
}

/// The line base `x ∈ x₀ + c·F_p^*`, folded by `⟨−1⟩` or `⟨−1, τ_T⟩`.
fn build_line_base(inst: &Inst, tors: bool) -> Result<(Fb, Line2), String> {
    let curve = &inst.curve;
    let f = &curve.f;
    let (x0, c) = if tors {
        inst.torsion.ok_or("no 2-torsion on this instance")?
    } else {
        (f.zero(), f.one())
    };
    let mut line = Line2::new(*f, x0, c);
    line.sym = tors;
    let mut seed = Vec::with_capacity(2 * inst.p as usize);
    for t in 1..inst.p {
        seed.extend(curve.lift_x(line.x_of(t)));
    }
    let neg = |z: Pt| curve.neg(z);
    let tt = Pt::aff(x0, f.zero());
    let translate = |z: Pt| curve.add_raw(z, tt);
    let mut gens: Vec<Gen> = vec![(&neg, inst.r - 1)];
    if tors {
        gens.push((&translate, 1));
    }
    let fb = fold(curve, inst.r, inst.h, seed, &gens)?;
    Ok((fb, line))
}

/// The GLS line `x ∈ u·s·F_p^*`, folded by `⟨−1⟩` or `⟨−1, ψ⟩`.
fn build_gls_base(inst: &Inst, psi_fold: bool) -> Result<(Fb, Line2), String> {
    let curve = &inst.curve;
    let f = &curve.f;
    let (dir, psi) = inst.gls.as_ref().ok_or("not a gls instance")?;
    let line = Line2::new(*f, f.zero(), *dir);
    let mut seed = Vec::with_capacity(2 * inst.p as usize);
    for t in 1..inst.p {
        seed.extend(curve.lift_x(line.x_of(t)));
    }
    let neg = |z: Pt| curve.neg(z);
    let apply = |z: Pt| -> Pt {
        if z.inf {
            z
        } else {
            Pt::aff(f.mul(psi.cx, f.frob(z.x)), f.mul(psi.cy, f.frob(z.y)))
        }
    };
    let mut gens: Vec<Gen> = vec![(&neg, inst.r - 1)];
    if psi_fold {
        gens.push((&apply, psi.lambda));
    }
    let fb = fold(curve, inst.r, inst.h, seed, &gens)?;
    Ok((fb, line))
}

fn run_ic_arm(inst: &Inst, arm: &str, inv_cost: f64, crosscheck: u64) -> Result<J, String> {
    let started = Instant::now();
    let (c_mul, c_inv) = measure_c_add(&inst.curve, inst.p);
    let c_add = c_mul + inv_cost * c_inv;
    let _ = take_counters();
    let mut meter = Meter::default();
    let (fb, mut oracle) = match arm {
        "ic-a" => build_line_base(inst, false)?,
        "ic-b" | "ic-c1" | "ic-c2" => build_line_base(inst, true)?,
        "ic-gls-neg" => build_gls_base(inst, false)?,
        "ic-gls-psi" => build_gls_base(inst, true)?,
        other => return Err(format!("unknown ic arm {other}")),
    };
    meter.lap("base");
    let (max_lp, lp_exp) = match arm {
        "ic-c1" => (1, 0.75),
        "ic-c2" => (2, 0.5),
        _ => (0, 1.0),
    };
    let cfg = IcCfg {
        max_lp,
        lp_exp,
        crosscheck,
        cap_trials: 20 * fb.columns as u64 + 100_000,
    };
    let out = ic_solve(
        inst,
        &fb,
        &mut oracle,
        &cfg,
        0x51_0000 + inst.idx * 1000 + inst.target,
    );
    let mut phases = Vec::new();
    let mut total_mult = 0.0;
    for ph in ["base", "targets", "oracle", "lift", "la_field", "verify"] {
        let me = meter.mult_eq(ph, inv_cost) + out.meter.mult_eq(ph, inv_cost);
        total_mult += me;
        phases.push((
            ph.to_string(),
            J::obj(vec![("mult_eq", J::f(me)), ("gae", J::f(me / c_add))]),
        ));
    }
    let gae_priced = total_mult / c_add;
    let mut natives = out.natives.clone();
    natives.insert("base_columns_hashed".to_string(), fb.columns as u64);
    let mut detail = out.detail.clone();
    detail.push(("phases", J::O(phases)));
    detail.push(("c_add_mult_eq", J::f(c_add)));
    detail.push(("c_add_muls", J::f(c_mul)));
    detail.push(("c_add_invs", J::f(c_inv)));
    detail.push(("inv_cost", J::f(inv_cost)));
    detail.push((
        "c_add_by_inv_cost",
        J::obj(vec![
            ("2", J::f(c_mul + 2.0 * c_inv)),
            ("4", J::f(c_mul + 4.0 * c_inv)),
            ("8", J::f(c_mul + 8.0 * c_inv)),
        ]),
    ));
    detail.push((
        "base",
        J::obj(vec![
            ("points", J::u(fb.points.len() as u64)),
            ("columns", J::u(fb.columns as u64)),
            (
                "points_per_column",
                J::f(fb.points.len() as f64 / fb.columns.max(1) as f64),
            ),
        ]),
    ));
    detail.push(("wall_seconds", J::f(started.elapsed().as_secs_f64())));
    detail.push(("recovered", out.recovered.map(J::u).unwrap_or(J::Null)));
    detail.push(("planted_matches", J::B(out.verified)));
    Ok(priced_record(
        inst,
        arm,
        gae_priced,
        &natives,
        out.verified,
        J::obj(detail),
    ))
}

// ── Rho (Van Oorschot–Wiener, counted) ─────────────────────────────

struct RhoRes {
    recovered: Option<u64>,
    ops: Ops,
    steps: u64,
    walks: u64,
    dps: u64,
    cycles: u64,
    natives: BTreeMap<String, u64>,
    canon_mults: (u64, u64),
    expected_steps: f64,
}

/// Matched rho with the walk on classes: `fold = 2` (`{±P}`) or `4`
/// (`{±P, ±ψP}`).  An r-adding walk with the class of the point choosing the
/// jump; distinguished points stored with the coefficients of the class
/// representative; a collision between two coefficient pairs of one
/// representative gives `d`.  Short cycles (the negation map's fruitless
/// 2-cycles) are escaped by one doubling.  Every group addition and doubling
/// is one GAE, the starts and the verification included; canonicalisations,
/// partition hashes and table operations are listed as natives.
fn rho(inst: &Inst, fold: u32, seed: u64) -> RhoRes {
    let curve = &inst.curve;
    let f = &curve.f;
    let r = inst.r;
    let mut rng = Rng::new(seed ^ 0x4B07);
    let mut ops = Ops::default();
    let mut natives: BTreeMap<String, u64> = BTreeMap::new();
    let mut canon_mults = (0u64, 0u64);
    let psi = inst.gls.as_ref().map(|(_, p)| p);
    let log2r = (r as f64).log2();
    let jumps = if log2r < 30.0 { 16 } else { 32 };
    let small = 1u64 << ((log2r / 2.0).ceil() as u32).min(40);
    let mut table: Vec<(Pt, u64, u64)> = Vec::new();
    for _ in 0..jumps {
        let (a, b) = (rng.range(1, small.min(r)), rng.range(1, small.min(r)));
        let ga = curve.mul(&mut ops, inst.g, a);
        let qb = curve.mul(&mut ops, inst.q, b);
        table.push((curve.add(&mut ops, ga, qb), a, b));
    }
    // Starts Z_k = Z_0 + k·S: one addition each, all distinct.
    let (a0, b0) = (rng.below(r), rng.below(r));
    let ga = curve.mul(&mut ops, inst.g, a0);
    let qb = curve.mul(&mut ops, inst.q, b0);
    let mut z = curve.add(&mut ops, ga, qb);
    let (mut za, mut zb) = (a0, b0);
    let stride = curve.add(&mut ops, table[0].0, table[1].0);
    let (sa, sb) = ((table[0].1 + table[1].1) % r, (table[0].2 + table[1].2) % r);
    // Canonical class representative: the least key among the orbit, with
    // the multiplier m such that rep = [m]P.
    let canon = |p: Pt, mults: &mut (u64, u64), natives: &mut BTreeMap<String, u64>| -> (Pt, u64) {
        let before = take_counters();
        let mut best = (p, 1u64);
        let mut best_key = curve.key(&p);
        let n = curve.neg(p);
        let kn = curve.key(&n);
        if kn < best_key {
            best = (n, r - 1);
            best_key = kn;
        }
        if fold == 4 {
            let ps = psi.expect("psi");
            let q = if p.inf {
                p
            } else {
                Pt::aff(f.mul(ps.cx, f.frob(p.x)), f.mul(ps.cy, f.frob(p.y)))
            };
            let candidates = [(q, ps.lambda), (curve.neg(q), (r - ps.lambda) % r)];
            for (c, m) in candidates {
                let k = curve.key(&c);
                if k < best_key {
                    best = (c, m);
                    best_key = k;
                }
            }
            *natives.entry("psi_maps_uncharged".into()).or_insert(0) += 1;
        }
        let after = take_counters();
        mults.0 += after.0;
        mults.1 += after.1;
        // Restore anything the caller had counted before this call.
        CNT.with(|c| c.set(before));
        *natives
            .entry("canonicalisations_uncharged".into())
            .or_insert(0) += 1;
        *natives
            .entry("negations_examined_uncharged".into())
            .or_insert(0) += 1;
        best
    };
    let dp_bits = ((log2r / 4.0).floor() as u32).max(1);
    let dp_mask = (1u64 << dp_bits) - 1;
    let max_walk = 20u64 << dp_bits;
    let expected = (std::f64::consts::PI * r as f64 / (2.0 * fold as f64)).sqrt();
    let cap = (60.0 * (r as f64).sqrt()) as u64 + 100_000;
    let mut table_dp: FxMap<(u64, u64)> = fxmap(1 << 12);
    let (mut steps, mut walks, mut dps, mut cycles) = (0u64, 0u64, 0u64, 0u64);
    let mut recovered = None;
    'outer: while steps < cap {
        walks += 1;
        // Start the walk at the representative of Z.
        let (mut cur, m0) = canon(z, &mut canon_mults, &mut natives);
        let (mut ca, mut cb) = (mulmod(za, m0, r), mulmod(zb, m0, r));
        z = curve.add(&mut ops, z, stride);
        za = (za + sa) % r;
        zb = (zb + sb) % r;
        let mut hist = [u64::MAX; 8];
        let mut hpos = 0usize;
        let mut len = 0u64;
        loop {
            if cur.inf {
                break;
            }
            let key = curve.key(&cur);
            *natives
                .entry("partition_hashes_uncharged".into())
                .or_insert(0) += 1;
            if mix64(key) & dp_mask == 0 {
                dps += 1;
                *natives.entry("dp_table_ops_uncharged".into()).or_insert(0) += 1;
                if let Some(&(pa, pb)) = table_dp.get(&key) {
                    if (pa, pb) != (ca, cb) && pb != cb {
                        let num = (pa + r - ca) % r;
                        let den = (cb + r - pb) % r;
                        let d = mulmod(num, invmod(den, r), r);
                        let mut vops = Ops::default();
                        let ok = curve.mul(&mut vops, inst.g, d) == inst.q;
                        ops.adds += vops.adds;
                        ops.doubles += vops.doubles;
                        if ok {
                            recovered = Some(d);
                            break 'outer;
                        }
                    }
                } else {
                    table_dp.insert(key, (ca, cb));
                }
                break;
            }
            if len >= max_walk {
                break;
            }
            // Short-cycle detection and escape by doubling.
            if hist.contains(&key) {
                cycles += 1;
                let dbl = curve.dbl(&mut ops, cur);
                let (rep, m) = canon(dbl, &mut canon_mults, &mut natives);
                let two = 2 % r;
                ca = mulmod(mulmod(ca, two, r), m, r);
                cb = mulmod(mulmod(cb, two, r), m, r);
                cur = rep;
                hist = [u64::MAX; 8];
                steps += 1;
                len += 1;
                continue;
            }
            hist[hpos % 8] = key;
            hpos += 1;
            let j = (mix64(key ^ 0x7A11) % table.len() as u64) as usize;
            let (m, ja, jb) = table[j];
            let next = curve.add(&mut ops, cur, m);
            let (na, nb) = ((ca + ja) % r, (cb + jb) % r);
            steps += 1;
            len += 1;
            if next.inf {
                break;
            }
            let (rep, mult) = canon(next, &mut canon_mults, &mut natives);
            cur = rep;
            ca = mulmod(na, mult, r);
            cb = mulmod(nb, mult, r);
            if steps >= cap {
                break 'outer;
            }
        }
    }
    RhoRes {
        recovered,
        ops,
        steps,
        walks,
        dps,
        cycles,
        natives,
        canon_mults,
        expected_steps: expected,
    }
}

fn run_rho_arm(inst: &Inst, arm: &str, inv_cost: f64) -> Result<J, String> {
    let started = Instant::now();
    let fold = match arm {
        "rho-neg" => 2,
        "rho-psi" => 4,
        other => return Err(format!("unknown rho arm {other}")),
    };
    if fold == 4 && inst.gls.is_none() {
        return Err("rho-psi needs a gls instance".into());
    }
    let (c_mul, c_inv) = measure_c_add(&inst.curve, inst.p);
    let c_add = c_mul + inv_cost * c_inv;
    let _ = take_counters();
    let res = rho(inst, fold, 0x120 + inst.idx * 1000 + inst.target);
    let verified = res.recovered == Some(inst.planted);
    let gae = res.ops.gae();
    let canon_gae = (res.canon_mults.0 as f64 + inv_cost * res.canon_mults.1 as f64) / c_add;
    let natives = res.natives.clone();
    let detail = J::obj(vec![
        ("steps", J::u(res.steps)),
        ("walks", J::u(res.walks)),
        ("distinguished_points", J::u(res.dps)),
        ("short_cycles_escaped", J::u(res.cycles)),
        ("fold", J::u(fold as u64)),
        ("expected_steps", J::f(res.expected_steps)),
        (
            "steps_over_expected",
            J::f(res.steps as f64 / res.expected_steps),
        ),
        ("group_adds", J::u(res.ops.adds)),
        ("group_doubles", J::u(res.ops.doubles)),
        (
            "canonicalisation_fp_mult_eq",
            J::f(res.canon_mults.0 as f64 + inv_cost * res.canon_mults.1 as f64),
        ),
        ("canonicalisation_gae_if_priced_by_count", J::f(canon_gae)),
        ("c_add_mult_eq", J::f(c_add)),
        ("inv_cost", J::f(inv_cost)),
        ("recovered", res.recovered.map(J::u).unwrap_or(J::Null)),
        ("planted_matches", J::B(verified)),
        ("wall_seconds", J::f(started.elapsed().as_secs_f64())),
    ]);
    Ok(priced_record(inst, arm, gae, &natives, verified, detail))
}

// ── Negative control: the same pipeline on a shuffled base ─────────

/// Arm (a)'s pipeline on a base of the same number of columns whose points
/// are drawn from the whole curve (no line structure), decomposed by exact
/// enumeration (one group addition per base point per target): the null for
/// "the line is what makes a decomposition cheap".
fn run_shuffled_control(inst: &Inst, inv_cost: f64, rate_trials: Option<u64>) -> Result<J, String> {
    let started = Instant::now();
    let curve = &inst.curve;
    let f = &curve.f;
    let (line_fb, _) = build_line_base(inst, false)?;
    let target_cols = line_fb.columns;
    drop(line_fb);
    let _ = take_counters();
    let mut rng = instance_rng(&inst.family, inst.bits, inst.idx, 77);
    let mut seen = HashSet::new();
    let mut seed_pts = Vec::new();
    let mut cols = 0usize;
    while cols < target_cols {
        let x = rand_el(f, &mut rng);
        if !seen.insert(f.pack(x)) {
            continue;
        }
        let lifted = curve.lift_x(x);
        if lifted.len() == 2 {
            seed_pts.extend(lifted);
            cols += 1;
        }
    }
    let neg = |z: Pt| curve.neg(z);
    let fb = fold(curve, inst.r, inst.h, seed_pts, &[(&neg, inst.r - 1)])?;
    let _ = take_counters();
    let (r, h) = (inst.r, inst.h);
    let ncols = fb.columns;
    let d_col = ncols;
    let unknowns = ncols + 1;
    let mut rng = Rng::new(0x5110 + inst.idx);
    let mut la_rng = Rng::new(0x1A1A + inst.idx);
    let mut ops = Ops::default();
    let mut meter = Meter::default();
    let mut walk = Walk::new(inst, &mut ops, &mut rng);
    let colmap: Vec<usize> = (0..ncols).collect();
    let mut full: Vec<SparseRel> = Vec::new();
    let step = (unknowns / 200).max(1);
    let mut trigger = (unknowns / 4).max(2);
    let (mut trials, mut found, mut la_ops, mut la_attempts) = (0u64, 0u64, 0u64, 0u64);
    let mut recovered = None;
    let mut last_failed: Option<Vec<usize>> = None;
    let mut seen_x: FxMap<()> = fxmap(1 << 12);
    let mut enumeration_adds = 0u64;
    let cap = rate_trials.unwrap_or(80 * ncols as u64 + 100_000);
    while trials < cap && recovered.is_none() {
        let (rr, a, b) = walk.next(inst, &mut ops, &mut rng);
        trials += 1;
        if rr.inf || seen_x.insert(f.pack(rr.x), ()).is_some() {
            continue;
        }
        let mut hit: Option<Vec<usize>> = None;
        for (i, p) in fb.points.iter().enumerate() {
            let dpt = curve.add(&mut ops, rr, curve.neg(*p));
            enumeration_adds += 1;
            if !dpt.inf {
                if let Some(&j) = fb.point_index.get(&curve.key(&dpt)) {
                    hit = Some(vec![i, j as usize]);
                    break;
                }
            }
        }
        let Some(ix) = hit else { continue };
        found += 1;
        if rate_trials.is_some() {
            continue;
        }
        let (cols_v, rhs) = relation_row(&fb, &ix, &colmap, d_col, h, a, b, r);
        if cols_v.is_empty() {
            continue;
        }
        full.push(SparseRel { cols: cols_v, rhs });
        if full.len() < trigger {
            continue;
        }
        match try_solve(
            inst,
            &full,
            d_col,
            unknowns,
            &mut la_rng,
            &mut la_ops,
            &mut la_attempts,
            &mut meter,
            &mut Vec::new(),
            &mut last_failed,
            &mut 0u64,
            &mut (0.0, 0.0),
        ) {
            Ok((dd, _)) => recovered = Some(dd),
            Err(_) => trigger = full.len() + step,
        }
    }
    let verified = recovered == Some(inst.planted);
    let _ = take_counters();
    let gae_priced = ops.gae();
    let mut natives = BTreeMap::new();
    natives.insert("la_modr_ops".to_string(), la_ops);
    natives.insert("base_columns_hashed".to_string(), ncols as u64);
    natives.insert("enumeration_key_probes".to_string(), enumeration_adds);
    let _ = inv_cost;
    let detail = J::obj(vec![
        ("columns", J::u(ncols as u64)),
        ("trials", J::u(trials)),
        ("decompositions_found", J::u(found)),
        ("hit_rate", J::f(found as f64 / trials.max(1) as f64)),
        ("full_relations", J::u(full.len() as u64)),
        ("la_attempts", J::u(la_attempts)),
        ("enumeration_adds", J::u(enumeration_adds)),
        ("group_adds", J::u(ops.adds)),
        ("group_doubles", J::u(ops.doubles)),
        ("recovered", recovered.map(J::u).unwrap_or(J::Null)),
        ("planted_matches", J::B(verified)),
        ("exhausted", J::B(recovered.is_none())),
        ("wall_seconds", J::f(started.elapsed().as_secs_f64())),
    ]);
    let arm = if rate_trials.is_some() {
        "control-shuffled-rate"
    } else {
        "control-shuffled-a"
    };
    Ok(priced_record(
        inst, arm, gae_priced, &natives, verified, detail,
    ))
}

// ── Summary ────────────────────────────────────────────────────────

fn lsq(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let slope = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    (my - slope * mx, slope)
}

fn percentile(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

#[derive(Clone)]
struct Obs {
    bits: u32,
    curve: u64,
    lr: f64,
    lc: f64,
}

/// Least-squares fit of `log2 cost` against `log2 r` with a cluster
/// bootstrap over the curves of each size cell.
fn boot_fit(obs: &[Obs], reps: usize, seed: u64) -> (f64, f64, f64, f64, Vec<(f64, f64)>) {
    let xs: Vec<f64> = obs.iter().map(|o| o.lr).collect();
    let ys: Vec<f64> = obs.iter().map(|o| o.lc).collect();
    let (a0, b0) = lsq(&xs, &ys);
    // Clusters are curves: every target of a sampled curve goes with it.
    let mut cells: BTreeMap<u32, BTreeMap<u64, Vec<&Obs>>> = BTreeMap::new();
    for o in obs {
        cells
            .entry(o.bits)
            .or_default()
            .entry(o.curve)
            .or_default()
            .push(o);
    }
    let mut rng = Rng::new(seed);
    let mut slopes = Vec::new();
    let mut fits = Vec::new();
    for _ in 0..reps {
        let (mut bx, mut by) = (Vec::new(), Vec::new());
        for curves in cells.values() {
            let list: Vec<&Vec<&Obs>> = curves.values().collect();
            for _ in 0..list.len() {
                for o in list[rng.below(list.len() as u64) as usize] {
                    bx.push(o.lr);
                    by.push(o.lc);
                }
            }
        }
        let (a, b) = lsq(&bx, &by);
        slopes.push(b);
        fits.push((a, b));
    }
    let lo = percentile(&mut slopes.clone(), 0.025);
    let hi = percentile(&mut slopes, 0.975);
    (a0, b0, lo, hi, fits)
}

fn load_rows(dir: &str) -> Result<(Vec<J>, usize), String> {
    let mut rows: Vec<J> = Vec::new();
    let mut files: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("read_dir {dir}: {e}"))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "jsonl").unwrap_or(false))
        .collect();
    files.sort();
    for f in &files {
        for line in fs::read_to_string(f).map_err(|e| e.to_string())?.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let v = parse_json(line).map_err(|e| format!("{f:?}: {e}"))?;
            if v.get("schema").str() == Some(SCHEMA) {
                rows.push(v);
            }
        }
    }
    Ok((rows, files.len()))
}

/// One fitted line: `(intercept, slope, slope 2.5 %, slope 97.5 %, bootstrap fits)`.
type Fit = (f64, f64, f64, f64, Vec<(f64, f64)>);

fn summarize(dir: &str, out: &str) -> Result<(), String> {
    let (rows, nfiles) = load_rows(dir)?;
    let mut groups: BTreeMap<(String, String), Vec<&J>> = BTreeMap::new();
    for v in &rows {
        groups
            .entry((
                v.get("family").str().unwrap_or("").to_string(),
                v.get("arm").str().unwrap_or("").to_string(),
            ))
            .or_default()
            .push(v);
    }
    // The largest sizes of the whole data set: the "top three" robustness fit.
    let mut all_bits: Vec<u32> = rows
        .iter()
        .filter_map(|v| v.get("bits").u64())
        .map(|b| b as u32)
        .collect();
    all_bits.sort_unstable();
    all_bits.dedup();
    let top3: Vec<u32> = all_bits.iter().rev().take(3).copied().collect();
    let fourq_log2_r = 246.0_f64;
    let modes: [(&str, Option<&Vec<u32>>); 2] = [("all_sizes", None), ("top3_sizes", Some(&top3))];
    let mut per_arm = Vec::new();
    let mut fits_store: BTreeMap<(String, String, usize, usize), Fit> = BTreeMap::new();
    for ((family, arm), vs) in &groups {
        let mut by_w = Vec::new();
        for (wi, wname) in BAND_NAMES.iter().enumerate() {
            let mut by_mode = Vec::new();
            for (mi, (mname, filter)) in modes.iter().enumerate() {
                let obs: Vec<Obs> = vs
                    .iter()
                    .filter(|v| v.get("verified").bool() == Some(true))
                    .filter(|v| {
                        filter
                            .map(|f| f.contains(&(v.get("bits").u64().unwrap_or(0) as u32)))
                            .unwrap_or(true)
                    })
                    .map(|v| Obs {
                        bits: v.get("bits").u64().unwrap_or(0) as u32,
                        curve: v.get("idx").u64().unwrap_or(0),
                        lr: v.get("log2_r").f64().unwrap(),
                        lc: v
                            .get("gae_total_by_weight")
                            .get(wname)
                            .f64()
                            .unwrap()
                            .log2(),
                    })
                    .collect();
                let mut sizes: Vec<u32> = obs.iter().map(|o| o.bits).collect();
                sizes.sort_unstable();
                sizes.dedup();
                if obs.len() < 3 || sizes.len() < 2 {
                    by_mode.push((
                        mname.to_string(),
                        J::obj(vec![("n", J::u(obs.len() as u64)), ("fit", J::Null)]),
                    ));
                    continue;
                }
                let (a, b, lo, hi, fits) = boot_fit(&obs, 2000, 7 + wi as u64);
                let mut cells = Vec::new();
                for s in &sizes {
                    let cell: Vec<&&J> = vs
                        .iter()
                        .filter(|v| {
                            v.get("bits").u64() == Some(*s as u64)
                                && v.get("verified").bool() == Some(true)
                        })
                        .collect();
                    let mean_s = cell
                        .iter()
                        .map(|v| v.get("s_by_weight").get(wname).f64().unwrap())
                        .sum::<f64>()
                        / cell.len() as f64;
                    let mean_lr = cell
                        .iter()
                        .map(|v| v.get("log2_r").f64().unwrap())
                        .sum::<f64>()
                        / cell.len() as f64;
                    let mut curves: Vec<u64> =
                        cell.iter().filter_map(|v| v.get("idx").u64()).collect();
                    curves.sort_unstable();
                    curves.dedup();
                    cells.push((
                        s.to_string(),
                        J::obj(vec![
                            ("curves", J::u(curves.len() as u64)),
                            ("rows", J::u(cell.len() as u64)),
                            ("mean_log2_r", J::f(mean_lr)),
                            ("mean_S", J::f(mean_s)),
                        ]),
                    ));
                }
                by_mode.push((
                    mname.to_string(),
                    J::obj(vec![
                        ("n", J::u(obs.len() as u64)),
                        (
                            "sizes",
                            J::A(sizes.iter().map(|&s| J::u(s as u64)).collect()),
                        ),
                        ("alpha", J::f(b)),
                        ("alpha_ci95", J::A(vec![J::f(lo), J::f(hi)])),
                        ("intercept_log2", J::f(a)),
                        ("cells", J::O(cells)),
                    ]),
                ));
                fits_store.insert((family.clone(), arm.clone(), wi, mi), (a, b, lo, hi, fits));
            }
            by_w.push((wname.to_string(), J::O(by_mode)));
        }
        let unverified = if arm.starts_with("control-") {
            0
        } else {
            vs.iter()
                .filter(|v| v.get("verified").bool() != Some(true))
                .count()
        };
        per_arm.push((
            format!("{family}/{arm}"),
            J::obj(vec![
                ("rows", J::u(vs.len() as u64)),
                ("unverified_rows", J::u(unverified as u64)),
                ("by_weight", J::O(by_w)),
            ]),
        ));
    }
    // Each index-calculus arm against the matched rho of its family, per weight.
    let mut gaps = Vec::new();
    let mut order = Vec::new();
    for ((family, arm), vs) in &groups {
        if arm.starts_with("rho") || arm.starts_with("control") {
            continue;
        }
        let rho_arms: Vec<&str> = if family == "gls" {
            vec!["rho-neg", "rho-psi"]
        } else {
            vec!["rho-neg"]
        };
        for rho_arm in rho_arms {
            let Some(rvs) = groups.get(&(family.clone(), rho_arm.to_string())) else {
                continue;
            };
            let mut bits: Vec<u64> = vs.iter().filter_map(|v| v.get("bits").u64()).collect();
            bits.sort_unstable();
            bits.dedup();
            let rho_fold = rvs
                .iter()
                .filter_map(|v| v.get("detail").get("fold").f64())
                .next()
                .unwrap_or(2.0);
            let mut per_w = Vec::new();
            for (wi, wname) in BAND_NAMES.iter().enumerate() {
                let mut per_mode = Vec::new();
                for (mi, (mname, _)) in modes.iter().enumerate() {
                    let (Some((a_ic, b_ic, _, _, fits_ic)), Some((a_rho, b_rho, _, _, fits_rho))) = (
                        fits_store.get(&(family.clone(), arm.clone(), wi, mi)),
                        fits_store.get(&(family.clone(), rho_arm.to_string(), wi, mi)),
                    ) else {
                        continue;
                    };
                    let n = fits_ic.len().min(fits_rho.len());
                    let mut ex: Vec<f64> = (0..n)
                        .map(|i| {
                            (fits_ic[i].0 + fits_ic[i].1 * fourq_log2_r)
                                - (fits_rho[i].0 + fits_rho[i].1 * fourq_log2_r)
                        })
                        .collect();
                    let point = (a_ic + b_ic * fourq_log2_r) - (a_rho + b_rho * fourq_log2_r);
                    let mut sd: Vec<f64> = (0..n).map(|i| fits_ic[i].1 - fits_rho[i].1).collect();
                    let mut entry = vec![
                        ("alpha_ic", J::f(*b_ic)),
                        ("alpha_rho", J::f(*b_rho)),
                        ("alpha_difference", J::f(b_ic - b_rho)),
                        (
                            "alpha_difference_ci95",
                            J::A(vec![
                                J::f(percentile(&mut sd.clone(), 0.025)),
                                J::f(percentile(&mut sd, 0.975)),
                            ]),
                        ),
                        (
                            "extrapolated_at_log2_r_246",
                            J::obj(vec![
                                ("log2_gap", J::f(point)),
                                (
                                    "log2_gap_ci95",
                                    J::A(vec![
                                        J::f(percentile(&mut ex.clone(), 0.025)),
                                        J::f(percentile(&mut ex, 0.975)),
                                    ]),
                                ),
                                ("gap", J::f(2f64.powf(point))),
                            ]),
                        ),
                    ];
                    if mname == &"all_sizes" {
                        // Sensitivity: rho at its asymptote, GAE per expected step at the largest size.
                        let top = *bits.last().unwrap();
                        let ratios: Vec<f64> = rvs
                            .iter()
                            .filter(|v| {
                                v.get("bits").u64() == Some(top)
                                    && v.get("verified").bool() == Some(true)
                            })
                            .map(|v| {
                                v.get("gae_total_by_weight").get(wname).f64().unwrap()
                                    / v.get("detail").get("expected_steps").f64().unwrap()
                            })
                            .collect();
                        if !ratios.is_empty() {
                            let ratio = ratios.iter().sum::<f64>() / ratios.len() as f64;
                            let log2_rho = ratio.log2()
                                + 0.5
                                    * ((std::f64::consts::PI / (2.0 * rho_fold)).log2()
                                        + fourq_log2_r);
                            let log2_ic = a_ic + b_ic * fourq_log2_r;
                            entry.push((
                                "sensitivity_rho_at_its_asymptote",
                                J::obj(vec![("gae_per_expected_step_at_largest_size", J::f(ratio)), ("log2_gap", J::f(log2_ic - log2_rho)), ("gap", J::f(2f64.powf(log2_ic - log2_rho))), ("note", J::s("IC from the all-size fit; rho from its analytic sqrt(pi r / 2A) steps at the largest size's cost per step"))]),
                            ));
                        }
                    }
                    per_mode.push((mname.to_string(), J::obj(entry)));
                }
                // Per-size gap: ratio of the mean over curves of the totals.
                let mut size_gap = Vec::new();
                for &s in &bits {
                    let sel = |g: &Vec<&J>| -> Vec<f64> {
                        let mut per: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
                        for v in g.iter().filter(|v| {
                            v.get("bits").u64() == Some(s) && v.get("verified").bool() == Some(true)
                        }) {
                            per.entry(v.get("idx").u64().unwrap_or(0))
                                .or_default()
                                .push(v.get("gae_total_by_weight").get(wname).f64().unwrap());
                        }
                        per.values()
                            .map(|x| x.iter().sum::<f64>() / x.len() as f64)
                            .collect()
                    };
                    let (ic, rh) = (sel(vs), sel(rvs));
                    if ic.is_empty() || rh.is_empty() {
                        continue;
                    }
                    let mi = ic.iter().sum::<f64>() / ic.len() as f64;
                    let mr = rh.iter().sum::<f64>() / rh.len() as f64;
                    let mut rng = Rng::new(900 + s);
                    let mut boots = Vec::new();
                    for _ in 0..2000 {
                        let bi: f64 = (0..ic.len())
                            .map(|_| ic[rng.below(ic.len() as u64) as usize])
                            .sum::<f64>()
                            / ic.len() as f64;
                        let br: f64 = (0..rh.len())
                            .map(|_| rh[rng.below(rh.len() as u64) as usize])
                            .sum::<f64>()
                            / rh.len() as f64;
                        boots.push(bi / br);
                    }
                    let lo = percentile(&mut boots.clone(), 0.025);
                    let hi = percentile(&mut boots, 0.975);
                    size_gap.push((
                        s.to_string(),
                        J::obj(vec![
                            ("gap", J::f(mi / mr)),
                            ("gap_ci95", J::A(vec![J::f(lo), J::f(hi)])),
                            ("ic_curves", J::u(ic.len() as u64)),
                            ("rho_curves", J::u(rh.len() as u64)),
                        ]),
                    ));
                }
                per_mode.push(("per_size".to_string(), J::O(size_gap)));
                per_w.push((wname.to_string(), J::O(per_mode)));
            }
            gaps.push((format!("{family}/{arm}-vs-{rho_arm}"), J::O(per_w)));
            // Does the ordering of the two arms hold at every weight of the band?
            let mut per_size_order = Vec::new();
            for &s in &bits {
                let signs: Vec<J> = BAND_NAMES
                    .iter()
                    .map(|w| {
                        let m = |g: &Vec<&J>| {
                            let xs: Vec<f64> = g
                                .iter()
                                .filter(|v| {
                                    v.get("bits").u64() == Some(s)
                                        && v.get("verified").bool() == Some(true)
                                })
                                .map(|v| v.get("gae_total_by_weight").get(w).f64().unwrap())
                                .collect();
                            (!xs.is_empty()).then(|| xs.iter().sum::<f64>() / xs.len() as f64)
                        };
                        match (m(vs), m(rvs)) {
                            (Some(i), Some(r)) => J::B(i < r),
                            _ => J::Null,
                        }
                    })
                    .collect();
                let holds = signs.iter().all(|x| x.bool().is_some())
                    && signs.iter().all(|x| x.bool() == signs[0].bool());
                per_size_order.push((
                    s.to_string(),
                    J::obj(vec![
                        ("ic_cheaper_at_w0_w01_w1", J::A(signs)),
                        ("order_holds_across_band", J::B(holds)),
                    ]),
                ));
            }
            order.push((format!("{family}/{arm}-vs-{rho_arm}"), J::O(per_size_order)));
        }
    }
    let mut ctl = Vec::new();
    for ((family, arm), vs) in &groups {
        if !arm.starts_with("control") {
            continue;
        }
        let items: Vec<J> = vs
            .iter()
            .map(|v| {
                J::obj(vec![
                    ("bits", v.get("bits").clone()),
                    ("idx", v.get("idx").clone()),
                    ("target", v.get("target").clone()),
                    ("verified", v.get("verified").clone()),
                    ("hit_rate", v.get("detail").get("hit_rate").clone()),
                    ("trials", v.get("detail").get("trials").clone()),
                    ("gae", v.get("gae_priced").clone()),
                    ("S_w0", v.get("s_by_weight").get("w0").clone()),
                ])
            })
            .collect();
        ctl.push((format!("{family}/{arm}"), J::A(items)));
    }
    // Same-curve, same-size comparison of the control with arm (a): hit rate and S.
    let mut ctl_vs_a = Vec::new();
    if let Some(cs) = groups.get(&("torsion".to_string(), "control-shuffled-a".to_string())) {
        if let Some(avs) = groups.get(&("torsion".to_string(), "ic-a".to_string())) {
            for c in cs {
                let (b, i) = (c.get("bits").u64(), c.get("idx").u64());
                let a_rows: Vec<&&J> = avs
                    .iter()
                    .filter(|a| {
                        a.get("bits").u64() == b
                            && a.get("idx").u64() == i
                            && a.get("target").u64() == c.get("target").u64()
                    })
                    .collect();
                if let Some(a) = a_rows.first() {
                    ctl_vs_a.push(J::obj(vec![
                        ("bits", c.get("bits").clone()),
                        ("idx", c.get("idx").clone()),
                        ("target", c.get("target").clone()),
                        ("hit_rate_control", c.get("detail").get("hit_rate").clone()),
                        ("hit_rate_arm_a", a.get("detail").get("hit_rate").clone()),
                        ("S_w0_control", c.get("s_by_weight").get("w0").clone()),
                        ("S_w0_arm_a", a.get("s_by_weight").get("w0").clone()),
                    ]));
                }
            }
        }
    }
    ctl.push((
        "torsion/control-vs-ic-a_same_curve".to_string(),
        J::A(ctl_vs_a),
    ));
    let ic_rows: Vec<&J> = rows
        .iter()
        .filter(|v| {
            v.get("arm")
                .str()
                .map(|a| a.starts_with("ic-"))
                .unwrap_or(false)
        })
        .collect();
    let cc_checked: u64 = ic_rows
        .iter()
        .filter_map(|v| v.get("detail").get("crosscheck_trials").u64())
        .sum();
    let cc_dis: u64 = ic_rows
        .iter()
        .filter_map(|v| v.get("detail").get("crosscheck_disagreements").u64())
        .sum();
    let dup: u64 = ic_rows
        .iter()
        .filter_map(|v| v.get("detail").get("duplicate_targets_skipped").u64())
        .sum();
    let result = J::obj(vec![
        ("schema", J::s(SCHEMA)),
        ("fourq_log2_r", J::f(fourq_log2_r)),
        (
            "top3_sizes",
            J::A(top3.iter().map(|&b| J::u(b as u64)).collect()),
        ),
        ("arms", J::O(per_arm)),
        ("gaps", J::O(gaps)),
        ("band_order", J::O(order)),
        ("controls", J::O(ctl)),
        (
            "positive_control",
            J::obj(vec![
                ("ic_rows", J::u(ic_rows.len() as u64)),
                (
                    "ic_verified",
                    J::u(
                        ic_rows
                            .iter()
                            .filter(|v| v.get("verified").bool() == Some(true))
                            .count() as u64,
                    ),
                ),
                (
                    "solve_rows",
                    J::u(
                        rows.iter()
                            .filter(|v| v.get("arm").str() != Some("control-shuffled-rate"))
                            .count() as u64,
                    ),
                ),
                (
                    "solve_rows_verified",
                    J::u(
                        rows.iter()
                            .filter(|v| {
                                v.get("arm").str() != Some("control-shuffled-rate")
                                    && v.get("verified").bool() == Some(true)
                            })
                            .count() as u64,
                    ),
                ),
                (
                    "rate_only_rows",
                    J::u(
                        rows.iter()
                            .filter(|v| v.get("arm").str() == Some("control-shuffled-rate"))
                            .count() as u64,
                    ),
                ),
                ("crosscheck_trials", J::u(cc_checked)),
                ("crosscheck_disagreements", J::u(cc_dis)),
                ("duplicate_targets_skipped", J::u(dup)),
            ]),
        ),
    ]);
    fs::write(out, result.pretty() + "\n").map_err(|e| e.to_string())?;
    println!("summary written: {} rows from {} files", rows.len(), nfiles);
    Ok(())
}

// ── Smoke test ─────────────────────────────────────────────────────

fn smoke(args: &HashMap<String, String>) -> Result<(), String> {
    let inv_cost: f64 = args
        .get("inv-cost")
        .map(|s| s.parse().unwrap())
        .unwrap_or(4.0);
    let bits: u32 = args.get("bits").map(|s| s.parse().unwrap()).unwrap_or(24);
    let mut failures = 0;
    let report = |label: &str, rec: &J, failures: &mut u32| {
        let ok = rec.get("verified").bool() == Some(true);
        let cc = rec
            .get("detail")
            .get("crosscheck_disagreements")
            .u64()
            .unwrap_or(0);
        println!(
            "  {label}: verified={ok} gae={:.0} S(w=0)={:.1} trials={} steps={} crosscheck_disagreements={cc} wall={:.1}s",
            rec.get("gae_priced").f64().unwrap(),
            rec.get("s_by_weight").get("w0").f64().unwrap(),
            rec.get("detail").get("trials").u64().unwrap_or(0),
            rec.get("detail").get("steps").u64().unwrap_or(0),
            rec.get("detail").get("wall_seconds").f64().unwrap_or(0.0)
        );
        if !ok || cc != 0 {
            *failures += 1;
        }
    };
    for (family, arms) in [
        ("torsion", vec!["ic-a", "ic-b", "ic-c1", "ic-c2", "rho-neg"]),
        ("prime", vec!["ic-a", "rho-neg"]),
    ] {
        let inst = build_random_instance(family, bits, 0)?;
        println!(
            "[{}] n = {} = {}*{}, p = {}",
            inst.name, inst.n, inst.h, inst.r, inst.p
        );
        for arm in arms {
            let rec = if arm.starts_with("rho") {
                run_rho_arm(&inst, arm, inv_cost)?
            } else {
                run_ic_arm(&inst, arm, inv_cost, u64::MAX)?
            };
            report(arm, &rec, &mut failures);
        }
    }
    let inst = build_gls_instance(bits, 0)?;
    println!(
        "[{}] n = {}, r = {}, cofactor {}, lambda = {}",
        inst.name,
        inst.n,
        inst.r,
        inst.h,
        inst.gls.as_ref().unwrap().1.lambda
    );
    for arm in ["ic-gls-neg", "ic-gls-psi", "rho-neg", "rho-psi"] {
        let rec = if arm.starts_with("rho") {
            run_rho_arm(&inst, arm, inv_cost)?
        } else {
            run_ic_arm(&inst, arm, inv_cost, u64::MAX)?
        };
        report(arm, &rec, &mut failures);
    }
    // The symmetrised oracle against the resultant oracle and exact enumeration.
    {
        let inst = build_random_instance("torsion", bits, 0)?;
        let (fb, mut sym) = build_line_base(&inst, true)?;
        let mut generic = Line2::new(inst.curve.f, sym.x0, sym.c);
        let mut ops = Ops::default();
        let mut meter = Meter::default();
        let mut rng = Rng::new(31);
        let (mut found, mut bad, mut sym_mults, mut gen_mults) = (0u64, 0u64, 0f64, 0f64);
        let n = 300;
        let _ = take_counters();
        for _ in 0..n {
            let rr = inst.curve.mul(&mut ops, inst.g, rng.below(inst.r));
            let _ = take_counters();
            let spent = |m: &Meter| m.mult_eq("oracle", 4.0) + m.mult_eq("lift", 4.0);
            let before = spent(&meter);
            let a = sym.decompose(&inst.curve, &fb, &mut ops, &mut meter, rr);
            sym_mults += spent(&meter) - before;
            let before = spent(&meter);
            let b = generic.decompose(&inst.curve, &fb, &mut ops, &mut meter, rr);
            gen_mults += spent(&meter) - before;
            let ex = exact_decomposes(&inst.curve, &fb, rr);
            let _ = take_counters();
            if a.is_some() != ex || b.is_some() != ex {
                bad += 1;
            }
            if let Some(ix) = &a {
                let sum = inst.curve.add_raw(fb.points[ix[0]], fb.points[ix[1]]);
                if sum != rr {
                    bad += 1;
                }
            }
            found += u64::from(ex);
        }
        println!(
            "  symmetric oracle vs resultant oracle vs exact enumeration: {n} targets, {found} decomposable, {bad} disagreements; F_p multiplication equivalents per call (oracle and lift): symmetric {:.0}, resultant {:.0}; symmetric fallbacks {}",
            sym_mults / n as f64,
            gen_mults / n as f64,
            sym.sym_fallbacks
        );
        if bad > 0 {
            failures += 1;
        }
    }
    let inst = build_random_instance("torsion", bits, 0)?;
    let rec = run_shuffled_control(&inst, inv_cost, None)?;
    report("control-shuffled-a", &rec, &mut failures);
    println!(
        "  hit rate of the shuffled base: {:.3}",
        rec.get("detail").get("hit_rate").f64().unwrap()
    );
    // JSON round trip.
    let parsed = parse_json(&rec.compact())?;
    if parsed.get("verified").bool() != rec.get("verified").bool() {
        failures += 1;
    }
    if failures > 0 {
        return Err(format!("{failures} smoke failures"));
    }
    println!("SMOKE OK");
    Ok(())
}

// ── Main ───────────────────────────────────────────────────────────

/// `0`, `0,1,2` or `0-7`.
fn parse_targets(s: &str) -> Result<Vec<u64>, String> {
    let mut out = Vec::new();
    for part in s.split(',') {
        if let Some((a, b)) = part.split_once('-') {
            let (a, b): (u64, u64) = (
                a.parse().map_err(|_| "--targets")?,
                b.parse().map_err(|_| "--targets")?,
            );
            out.extend(a..=b);
        } else {
            out.push(part.parse().map_err(|_| "--targets")?);
        }
    }
    Ok(out)
}

fn arg_map(args: &[String]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        if let Some(k) = args[i].strip_prefix("--") {
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                m.insert(k.to_string(), args[i + 1].clone());
                i += 2;
                continue;
            }
            m.insert(k.to_string(), "true".to_string());
        }
        i += 1;
    }
    m
}

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    let cmd = argv.first().cloned().unwrap_or_default();
    let args = arg_map(&argv[argv.len().min(1)..]);
    let inv_cost: f64 = args
        .get("inv-cost")
        .map(|s| s.parse().expect("--inv-cost"))
        .unwrap_or(4.0);
    let result: Result<(), String> = (|| match cmd.as_str() {
        "smoke" => smoke(&args),
        "run" => {
            let family = args.get("family").ok_or("--family")?.clone();
            let bits: u32 = args
                .get("bits")
                .ok_or("--bits")?
                .parse()
                .map_err(|_| "--bits")?;
            let curves = parse_targets(args.get("idx").ok_or("--idx")?)?;
            let default_targets = args
                .get("targets")
                .cloned()
                .unwrap_or_else(|| "0".to_string());
            // `--arms name[:targets],name[:targets],...`
            let mut arms: Vec<(String, Vec<u64>)> = Vec::new();
            for spec in args.get("arms").ok_or("--arms")?.split(',') {
                let (name, tg) = spec
                    .split_once(':')
                    .unwrap_or((spec, default_targets.as_str()));
                arms.push((name.to_string(), parse_targets(tg)?));
            }
            let crosscheck: u64 = args
                .get("crosscheck")
                .map(|s| s.parse().unwrap())
                .unwrap_or(0);
            let quiet = args.contains_key("quiet");
            let out = args.get("out").cloned();
            for idx in curves {
                let mut inst = if family == "gls" {
                    build_gls_instance(bits, idx)?
                } else {
                    build_random_instance(&family, bits, idx)?
                };
                eprintln!(
                    "[{}] n = {}, r = {}, cofactor {}",
                    inst.name, inst.n, inst.r, inst.h
                );
                for (arm, targets) in &arms {
                    for &t in targets {
                        inst.set_target(t);
                        let rec = if arm.starts_with("rho") {
                            run_rho_arm(&inst, arm, inv_cost)?
                        } else if arm == "control-shuffled-a" {
                            run_shuffled_control(&inst, inv_cost, None)?
                        } else if arm == "control-shuffled-rate" {
                            let n: u64 = args
                                .get("control-trials")
                                .ok_or("--control-trials")?
                                .parse()
                                .map_err(|_| "--control-trials")?;
                            run_shuffled_control(&inst, inv_cost, Some(n))?
                        } else {
                            let cc = if matches!(
                                arm.as_str(),
                                "ic-a" | "ic-b" | "ic-gls-neg" | "ic-gls-psi"
                            ) {
                                crosscheck
                            } else {
                                0
                            };
                            run_ic_arm(&inst, arm, inv_cost, cc)?
                        };
                        emit(&out, &rec, quiet);
                    }
                }
            }
            Ok(())
        }
        "list" => {
            // The instances of the cells, one line each: provenance for the raw data.
            // `--all` lists every cell of the protocol.
            let cells: Vec<(String, u32, Vec<u64>)> = if args.contains_key("all") {
                let mut v = Vec::new();
                for bits in [24u32, 28, 32, 36, 40] {
                    v.push(("torsion".to_string(), bits, (0..6).collect()));
                    v.push(("prime".to_string(), bits, (0..3).collect()));
                    v.push(("gls".to_string(), bits, (0..3).collect()));
                }
                v
            } else {
                vec![(
                    args.get("family").ok_or("--family")?.clone(),
                    args.get("bits")
                        .ok_or("--bits")?
                        .parse()
                        .map_err(|_| "--bits")?,
                    parse_targets(args.get("idx").ok_or("--idx")?)?,
                )]
            };
            for (family, bits, idxs) in cells {
                for idx in idxs {
                    let inst = if family == "gls" {
                        build_gls_instance(bits, idx)?
                    } else {
                        build_random_instance(&family, bits, idx)?
                    };
                    println!(
                        "{} p={} n={} cofactor={} r={} log2_r={:.3} g=({},{}),({},{})",
                        inst.name,
                        inst.p,
                        inst.n,
                        inst.h,
                        inst.r,
                        (inst.r as f64).log2(),
                        inst.g.x[0],
                        inst.g.x[1],
                        inst.g.y[0],
                        inst.g.y[1]
                    );
                }
            }
            Ok(())
        }
        "summarize" => summarize(
            args.get("dir").ok_or("--dir")?,
            args.get("out").ok_or("--out")?,
        ),
        other => Err(format!(
            "unknown subcommand `{other}`; try smoke, run, summarize"
        )),
    })();
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(2);
    }
}
