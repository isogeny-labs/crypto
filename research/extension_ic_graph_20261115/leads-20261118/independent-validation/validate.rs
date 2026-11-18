//! Independent validator for the extension-IC graph lane (lane key
//! `independent-validation`, 2026-11-18). Standard library only; built with
//! `rustc -O --edition=2021 validate.rs`. It shares no code with any producer:
//! its own JSON reader, its own F_p / F_{p^2} / F_{p^6} arithmetic, its own
//! Jacobian short-Weierstrass group law, its own Miller-Rabin, its own
//! bitop/v1 price derivation and its own statistics. Every result it prints
//! is a validator observation, [pcc:unverified].
//!
//! Subcommands (every key read is required; a missing key aborts):
//!   ec PUBLIC_JSON SCALAR...        elliptic relation Q = [d]G, ord G = L
//!   nagao SUMMARY_JSON              recompute nagao-e2e statistics from rows
//!   run50 RUN50_DIR                 reprice run50 phases, recompute deltas
//!   descent RUN50_DIR DF_DIR        recompute descent-finder breakdown/compare
//!   rows SUMMARY_JSON < ROWS.jsonl  aggregate traced per-residual rows
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;

// ───────────────────────────── JSON reader ─────────────────────────────
#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
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
    fn value(&mut self) -> J {
        self.ws();
        let c = self.b[self.i];
        match c {
            b'{' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b[self.i] == b'}' {
                    self.i += 1;
                    return J::Obj(v);
                }
                loop {
                    self.ws();
                    let k = match self.value() {
                        J::Str(s) => s,
                        _ => panic!("object key not a string at {}", self.i),
                    };
                    self.ws();
                    assert_eq!(self.b[self.i], b':', "expected ':' at {}", self.i);
                    self.i += 1;
                    let x = self.value();
                    v.push((k, x));
                    self.ws();
                    match self.b[self.i] {
                        b',' => self.i += 1,
                        b'}' => {
                            self.i += 1;
                            return J::Obj(v);
                        }
                        _ => panic!("bad object at {}", self.i),
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b[self.i] == b']' {
                    self.i += 1;
                    return J::Arr(v);
                }
                loop {
                    v.push(self.value());
                    self.ws();
                    match self.b[self.i] {
                        b',' => self.i += 1,
                        b']' => {
                            self.i += 1;
                            return J::Arr(v);
                        }
                        _ => panic!("bad array at {}", self.i),
                    }
                }
            }
            b'"' => {
                self.i += 1;
                let mut s = String::new();
                loop {
                    let c = self.b[self.i];
                    self.i += 1;
                    match c {
                        b'"' => return J::Str(s),
                        b'\\' => {
                            let e = self.b[self.i];
                            self.i += 1;
                            match e {
                                b'"' => s.push('"'),
                                b'\\' => s.push('\\'),
                                b'/' => s.push('/'),
                                b'b' => s.push('\u{8}'),
                                b'f' => s.push('\u{c}'),
                                b'n' => s.push('\n'),
                                b'r' => s.push('\r'),
                                b't' => s.push('\t'),
                                b'u' => {
                                    let h = std::str::from_utf8(&self.b[self.i..self.i + 4]).unwrap();
                                    self.i += 4;
                                    let mut cp = u32::from_str_radix(h, 16).unwrap();
                                    if (0xD800..0xDC00).contains(&cp) {
                                        assert_eq!(&self.b[self.i..self.i + 2], b"\\u");
                                        let h2 = std::str::from_utf8(&self.b[self.i + 2..self.i + 6]).unwrap();
                                        self.i += 6;
                                        let lo = u32::from_str_radix(h2, 16).unwrap();
                                        cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                                    }
                                    s.push(char::from_u32(cp).unwrap());
                                }
                                _ => panic!("bad escape"),
                            }
                        }
                        _ => {
                            // copy one UTF-8 sequence verbatim
                            let start = self.i - 1;
                            let len = if c < 0x80 {
                                1
                            } else if c >= 0xF0 {
                                4
                            } else if c >= 0xE0 {
                                3
                            } else {
                                2
                            };
                            s.push_str(std::str::from_utf8(&self.b[start..start + len]).unwrap());
                            self.i = start + len;
                        }
                    }
                }
            }
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
                while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                assert!(self.i > st, "bad token at {}", st);
                J::Num(std::str::from_utf8(&self.b[st..self.i]).unwrap().to_string())
            }
        }
    }
}

fn parse(text: &str) -> J {
    let mut p = Parser { b: text.as_bytes(), i: 0 };
    let v = p.value();
    p.ws();
    assert_eq!(p.i, p.b.len(), "trailing data after JSON value");
    v
}

fn load(path: &str) -> J {
    parse(&fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}")))
}

impl J {
    /// Required key (lesson L-99ee952350b2: a missing key is an error).
    fn k(&self, key: &str) -> &J {
        match self {
            J::Obj(v) => v.iter().find(|(a, _)| a == key).map(|(_, b)| b).unwrap_or_else(|| panic!("missing key {key}")),
            _ => panic!("not an object when reading {key}"),
        }
    }
    fn has(&self, key: &str) -> bool {
        matches!(self, J::Obj(v) if v.iter().any(|(a, _)| a == key))
    }
    fn a(&self) -> &Vec<J> {
        match self {
            J::Arr(v) => v,
            _ => panic!("not an array"),
        }
    }
    fn o(&self) -> &Vec<(String, J)> {
        match self {
            J::Obj(v) => v,
            _ => panic!("not an object"),
        }
    }
    fn s(&self) -> &str {
        match self {
            J::Str(s) => s,
            _ => panic!("not a string: {:?}", self),
        }
    }
    fn is_null(&self) -> bool {
        matches!(self, J::Null)
    }
    fn b(&self) -> bool {
        match self {
            J::Bool(b) => *b,
            _ => panic!("not a bool: {:?}", self),
        }
    }
    /// Integer from a JSON number or a decimal string (records carry big
    /// counts as strings).
    fn u(&self) -> u128 {
        let t = match self {
            J::Num(s) | J::Str(s) => s,
            _ => panic!("not an integer: {:?}", self),
        };
        t.parse::<u128>().unwrap_or_else(|_| panic!("not an unsigned integer: {t}"))
    }
    fn i(&self) -> i128 {
        let t = match self {
            J::Num(s) | J::Str(s) => s,
            _ => panic!("not an integer: {:?}", self),
        };
        t.parse::<i128>().unwrap_or_else(|_| panic!("not an integer: {t}"))
    }
    fn f(&self) -> f64 {
        match self {
            J::Num(s) => s.parse::<f64>().unwrap(),
            _ => panic!("not a number: {:?}", self),
        }
    }
}

// ───────────────────────────── report helper ─────────────────────────────
struct Report {
    out: String,
    fails: u32,
    checks: u32,
}

impl Report {
    fn new() -> Self {
        Report { out: String::new(), fails: 0, checks: 0 }
    }
    fn check(&mut self, name: &str, ok: bool, detail: String) {
        self.checks += 1;
        if !ok {
            self.fails += 1;
        }
        let _ = writeln!(self.out, "{} {name}: {detail}", if ok { "PASS" } else { "FAIL" });
    }
    fn note(&mut self, name: &str, detail: String) {
        let _ = writeln!(self.out, "NOTE {name}: {detail}");
    }
    fn finish(self, what: &str) {
        print!("{}", self.out);
        println!("SUMMARY {what}: {} checks, {} failed", self.checks, self.fails);
        if self.fails > 0 {
            std::process::exit(1);
        }
    }
}

// ───────────────────────────── modular arithmetic ─────────────────────────────
fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    // operands < 2^64 in every use here
    assert!(a < (1u128 << 64) && b < (1u128 << 64));
    (a * b) % m
}

fn powmod(mut a: u128, mut e: u128, m: u128) -> u128 {
    let mut r = 1 % m;
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

fn is_prime_u64(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for a in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
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

// F_{p^2} = F_p[t]/(t^2 - w), F_{p^6} = F_{p^2}[u]/(u^3 - s)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct F2(u64, u64);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct F6([F2; 3]);

struct Tower {
    p: u64,
    w: u64,
    s: F2,
}

impl Tower {
    fn a2(&self, x: F2, y: F2) -> F2 {
        F2((x.0 + y.0) % self.p, (x.1 + y.1) % self.p)
    }
    fn s2(&self, x: F2, y: F2) -> F2 {
        F2((x.0 + self.p - y.0) % self.p, (x.1 + self.p - y.1) % self.p)
    }
    fn m2(&self, x: F2, y: F2) -> F2 {
        let p = self.p as u128;
        let (a, b, c, d) = (x.0 as u128, x.1 as u128, y.0 as u128, y.1 as u128);
        let re = (a * c + (self.w as u128) * ((b * d) % p)) % p;
        let im = (a * d + b * c) % p;
        F2(re as u64, im as u64)
    }
    fn pow2(&self, x: F2, mut e: u128) -> F2 {
        let mut r = F2(1, 0);
        let mut b = x;
        while e > 0 {
            if e & 1 == 1 {
                r = self.m2(r, b);
            }
            b = self.m2(b, b);
            e >>= 1;
        }
        r
    }
    fn a6(&self, x: F6, y: F6) -> F6 {
        F6([self.a2(x.0[0], y.0[0]), self.a2(x.0[1], y.0[1]), self.a2(x.0[2], y.0[2])])
    }
    fn s6(&self, x: F6, y: F6) -> F6 {
        F6([self.s2(x.0[0], y.0[0]), self.s2(x.0[1], y.0[1]), self.s2(x.0[2], y.0[2])])
    }
    fn m6(&self, x: F6, y: F6) -> F6 {
        let mut c = [F2(0, 0); 5];
        for i in 0..3 {
            for j in 0..3 {
                c[i + j] = self.a2(c[i + j], self.m2(x.0[i], y.0[j]));
            }
        }
        // u^3 = s, u^4 = s u
        F6([self.a2(c[0], self.m2(self.s, c[3])), self.a2(c[1], self.m2(self.s, c[4])), c[2]])
    }
    fn k6(&self, x: F6, k: u64) -> F6 {
        let p = self.p as u128;
        let f = |e: F2| F2(((e.0 as u128 * k as u128) % p) as u64, ((e.1 as u128 * k as u128) % p) as u64);
        F6([f(x.0[0]), f(x.0[1]), f(x.0[2])])
    }
    fn pow6(&self, x: F6, mut e: u128) -> F6 {
        let mut r = self.one();
        let mut b = x;
        while e > 0 {
            if e & 1 == 1 {
                r = self.m6(r, b);
            }
            b = self.m6(b, b);
            e >>= 1;
        }
        r
    }
    fn one(&self) -> F6 {
        F6([F2(1, 0), F2(0, 0), F2(0, 0)])
    }
    fn zero(&self) -> F6 {
        F6([F2(0, 0); 3])
    }
    fn q6(&self) -> u128 {
        (self.p as u128).pow(6)
    }
    /// Inverse by Fermat in F_{p^6}^*.
    fn inv6(&self, x: F6) -> F6 {
        assert!(x != self.zero());
        self.pow6(x, self.q6() - 2)
    }
}

// Jacobian points on Y^2 = X^3 + A X + B over F_{p^6}; None = infinity
type Jac = Option<(F6, F6, F6)>;

struct Curve<'a> {
    t: &'a Tower,
    a: F6,
}

impl<'a> Curve<'a> {
    fn dbl(&self, p: Jac) -> Jac {
        let t = self.t;
        let (x, y, z) = p?;
        if y == t.zero() {
            return None;
        }
        let yy = t.m6(y, y);
        let s = t.k6(t.m6(x, yy), 4);
        let z2 = t.m6(z, z);
        let m = t.a6(t.k6(t.m6(x, x), 3), t.m6(self.a, t.m6(z2, z2)));
        let x3 = t.s6(t.m6(m, m), t.k6(s, 2));
        let y3 = t.s6(t.m6(m, t.s6(s, x3)), t.k6(t.m6(yy, yy), 8));
        let z3 = t.k6(t.m6(y, z), 2);
        Some((x3, y3, z3))
    }
    fn add(&self, p: Jac, q: Jac) -> Jac {
        let t = self.t;
        let Some((x1, y1, z1)) = p else { return q };
        let Some((x2, y2, z2)) = q else { return p };
        let z1s = t.m6(z1, z1);
        let z2s = t.m6(z2, z2);
        let u1 = t.m6(x1, z2s);
        let u2 = t.m6(x2, z1s);
        let s1 = t.m6(y1, t.m6(z2s, z2));
        let s2 = t.m6(y2, t.m6(z1s, z1));
        if u1 == u2 {
            return if s1 == s2 { self.dbl(p) } else { None };
        }
        let h = t.s6(u2, u1);
        let r = t.s6(s2, s1);
        let h2 = t.m6(h, h);
        let h3 = t.m6(h2, h);
        let u1h2 = t.m6(u1, h2);
        let x3 = t.s6(t.s6(t.m6(r, r), h3), t.k6(u1h2, 2));
        let y3 = t.s6(t.m6(r, t.s6(u1h2, x3)), t.m6(s1, h3));
        let z3 = t.m6(h, t.m6(z1, z2));
        Some((x3, y3, z3))
    }
    /// Left-to-right double-and-add (the producers use right-to-left affine).
    fn mul(&self, p: Jac, k: u128) -> Jac {
        let mut acc: Jac = None;
        for i in (0..128).rev() {
            acc = self.dbl(acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(acc, p);
            }
        }
        acc
    }
    fn eq(&self, p: Jac, q: Jac) -> bool {
        let t = self.t;
        match (p, q) {
            (None, None) => true,
            (None, _) | (_, None) => false,
            (Some((x1, y1, z1)), Some((x2, y2, z2))) => {
                let z1s = t.m6(z1, z1);
                let z2s = t.m6(z2, z2);
                t.m6(x1, z2s) == t.m6(x2, z1s) && t.m6(y1, t.m6(z2s, z2)) == t.m6(y2, t.m6(z1s, z1))
            }
        }
    }
}

fn f2_of(j: &J, p: u64) -> F2 {
    let a = j.a();
    assert_eq!(a.len(), 2);
    let (x, y) = (a[0].u() as u64, a[1].u() as u64);
    assert!(x < p && y < p, "coordinate not reduced");
    F2(x, y)
}

fn f6_of(j: &J, p: u64) -> F6 {
    let a = j.a();
    assert_eq!(a.len(), 3);
    F6([f2_of(&a[0], p), f2_of(&a[1], p), f2_of(&a[2], p)])
}

fn cmd_ec(args: &[String]) {
    let mut r = Report::new();
    let pubj = load(&args[0]);
    let p = pubj.k("p").u() as u64;
    let l = pubj.k("l").u();
    r.check("p_prime", is_prime_u64(p as u128), format!("p = {p}"));
    r.check("p_not_3_mod_dividing", p % 3 != 0, format!("p mod 3 = {}", p % 3));
    r.check("L_prime_miller_rabin_12_bases", is_prime_u64(l), format!("L = {l}"));
    // w: least quadratic non-residue (Euler criterion)
    let w = (2..p as u128).find(|&w| powmod(w, (p as u128 - 1) / 2, p as u128) != 1).unwrap() as u64;
    r.check("w_nonresidue", powmod(w as u128, (p as u128 - 1) / 2, p as u128) == p as u128 - 1, format!("w = {w}"));
    let mut t = Tower { p, w, s: F2(0, 0) };
    let q2 = (p as u128) * (p as u128);
    // s: least non-cube, F_p elements first when p = 1 mod 3 (documented convention)
    let mut cands: Vec<F2> = Vec::new();
    if p % 3 == 1 {
        cands.extend((1..p).map(|x| F2(x, 0)));
    }
    for y in 1..p {
        for x in 0..p {
            cands.push(F2(x, y));
        }
    }
    let s = *cands.iter().find(|&&c| t.pow2(c, (q2 - 1) / 3) != F2(1, 0)).unwrap();
    t.s = s;
    r.check("s_noncube_in_Fp2", t.pow2(s, (q2 - 1) / 3) != F2(1, 0), format!("s = ({}, {})", s.0, s.1));
    // irreducibility sanity: u^(p^6) = u (Frobenius of the generator is periodic with order 6)
    let u = F6([F2(0, 0), F2(1, 0), F2(0, 0)]);
    let mut fr = u;
    for _ in 0..6 {
        fr = t.pow6(fr, p as u128);
    }
    r.check("frobenius_order_divides_6", fr == u, String::from("u^(p^6) = u"));
    let mut fr3 = u;
    for _ in 0..2 {
        fr3 = t.pow6(fr3, p as u128);
    }
    r.check("u_not_in_Fp2", fr3 != u, String::from("u^(p^2) != u"));
    // curve y^2 = x (x - alpha)(x - sigma(alpha)), sigma = p^2-power Frobenius by exponentiation
    let alpha = f6_of(pubj.k("alpha"), p);
    let salpha = t.pow6(alpha, q2);
    let a2 = t.s6(t.zero(), t.a6(alpha, salpha));
    let a4 = t.m6(alpha, salpha);
    r.check(
        "curve_nonsingular",
        alpha != t.zero() && salpha != t.zero() && alpha != salpha,
        String::from("0, alpha, sigma(alpha) distinct"),
    );
    let rhs = |x: F6| t.a6(t.a6(t.m6(t.m6(x, x), x), t.m6(a2, t.m6(x, x))), t.m6(a4, x));
    let gj = pubj.k("g");
    let qj = pubj.k("q");
    assert!(!gj.k("inf").b() && !qj.k("inf").b());
    let (gx, gy) = (f6_of(gj.k("x"), p), f6_of(gj.k("y"), p));
    let (qx, qy) = (f6_of(qj.k("x"), p), f6_of(qj.k("y"), p));
    r.check("G_on_curve", t.m6(gy, gy) == rhs(gx), String::new());
    r.check("Q_on_curve", t.m6(qy, qy) == rhs(qx), String::new());
    // short Weierstrass X = x + a2/3: A = a4 - a2^2/3
    let inv3 = powmod(3, p as u128 - 2, p as u128) as u64;
    let c = t.k6(a2, inv3);
    let aa = t.s6(a4, t.k6(t.m6(a2, a2), inv3));
    let bb = t.s6(
        t.k6(t.m6(t.m6(a2, a2), a2), (2 * powmod(27, p as u128 - 2, p as u128) as u64) % p),
        t.k6(t.m6(a2, a4), inv3),
    );
    let one = t.one();
    let gw = (t.a6(gx, c), gy);
    let qw = (t.a6(qx, c), qy);
    let on_w = |x: F6, y: F6| t.m6(y, y) == t.a6(t.a6(t.m6(t.m6(x, x), x), t.m6(aa, x)), bb);
    r.check("G_on_short_weierstrass_model", on_w(gw.0, gw.1), String::new());
    r.check("Q_on_short_weierstrass_model", on_w(qw.0, qw.1), String::new());
    let cv = Curve { t: &t, a: aa };
    let g: Jac = Some((gw.0, gw.1, one));
    let q: Jac = Some((qw.0, qw.1, one));
    r.check("L_times_G_is_infinity", cv.mul(g, l).is_none(), String::from("[L]G = O, L prime so ord(G) = L"));
    let q6 = t.q6();
    let hasse = 2 * (p as u128).pow(3);
    let tr = (q6 + 1) as i128 - (4 * l) as i128;
    r.note("group_order_context", format!("p^6 = {q6}; if #E = 4L then trace = {tr}, Hasse bound {hasse}: {}", tr.unsigned_abs() <= hasse));
    let mut seen = BTreeSet::new();
    for sarg in &args[1..] {
        let d: u128 = sarg.parse().unwrap();
        if !seen.insert(d) {
            continue;
        }
        r.check(&format!("scalar_{d}_in_range"), d > 0 && d < l, format!("0 < d < L"));
        let dg = cv.mul(g, d);
        r.check(&format!("scalar_{d}_relation_Q_eq_dG"), cv.eq(dg, q), String::from("independent Jacobian left-to-right ladder"));
        // negative controls: the check is not vacuous
        r.check(&format!("scalar_{d}_negative_control_d_plus_1"), !cv.eq(cv.mul(g, d + 1), q), String::from("[d+1]G != Q"));
        r.check(&format!("scalar_{d}_negative_control_L_minus_d"), !cv.eq(cv.mul(g, l - d), q), String::from("[L-d]G = -Q != Q"));
    }
    // affine cross-check of [d]G by an independent inversion path
    if let Some(d) = seen.iter().next() {
        let dg = cv.mul(g, *d).unwrap();
        let zi = t.inv6(dg.2);
        let zi2 = t.m6(zi, zi);
        let x = t.s6(t.m6(dg.0, zi2), c);
        let y = t.m6(dg.1, t.m6(zi2, zi));
        r.check("affine_dG_equals_recorded_Q_coordinates", x == qx && y == qy, String::from("model coordinates"));
    }
    r.finish("ec");
}

// ───────────────────────────── statistics ─────────────────────────────
fn median_odd(v: &[f64]) -> f64 {
    let mut a = v.to_vec();
    a.sort_by(f64::total_cmp);
    assert!(a.len() % 2 == 1);
    a[a.len() / 2]
}

fn geo(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}

fn rel_eq(a: f64, b: f64, tol: f64) -> bool {
    ((a - b) / b).abs() <= tol
}

/// SplitMix64: an RNG unrelated to the producer's ChaCha12 StdRng.
struct Sm(u64);
impl Sm {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        // Lemire-free rejection for exact uniformity
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let x = self.next();
            if x < zone {
                return x % n;
            }
        }
    }
}

/// Exact paired bootstrap distribution: all n^n ordered resamples, equal weight.
fn exact_boot(r: &[f64]) -> Vec<f64> {
    let n = r.len();
    let total = n.pow(n as u32);
    let mut out = Vec::with_capacity(total);
    for code in 0..total {
        let mut c = code;
        let mut s = 0.0;
        for _ in 0..n {
            s += r[c % n].ln();
            c /= n;
        }
        out.push((s / n as f64).exp());
    }
    out.sort_by(f64::total_cmp);
    out
}

/// Quantile rank of x in a sorted distribution: fraction of values < x and <= x.
fn rank_of(sorted: &[f64], x: f64) -> (f64, f64) {
    let lt = sorted.iter().filter(|&&v| v < x * (1.0 - 1e-13)).count();
    let le = sorted.iter().filter(|&&v| v <= x * (1.0 + 1e-13)).count();
    (lt as f64 / sorted.len() as f64, le as f64 / sorted.len() as f64)
}

/// Is x the geometric mean of some n-multiset of r?
fn attainable(r: &[f64], x: f64) -> Option<Vec<usize>> {
    let n = r.len();
    fn rec(r: &[f64], n: usize, start: usize, left: usize, acc: &mut Vec<usize>, x: f64, out: &mut Option<Vec<usize>>) {
        if out.is_some() {
            return;
        }
        if left == 0 {
            let g = (acc.iter().map(|&i| r[i].ln()).sum::<f64>() / n as f64).exp();
            if ((g - x) / x).abs() < 1e-13 {
                *out = Some(acc.clone());
            }
            return;
        }
        for i in start..r.len() {
            acc.push(i);
            rec(r, n, i, left - 1, acc, x, out);
            acc.pop();
        }
    }
    let mut out = None;
    rec(r, n, 0, n, &mut Vec::new(), x, &mut out);
    out
}

fn cmd_nagao(args: &[String]) {
    let mut r = Report::new();
    let s = load(&args[0]);
    let runs = s.k("runs").a();
    r.check("row_count_24", runs.len() == 24, format!("{} rows", runs.len()));
    let mut keys = BTreeSet::new();
    let mut walls: BTreeMap<(String, String), BTreeMap<u64, f64>> = BTreeMap::new();
    let mut order: BTreeMap<u64, Vec<String>> = BTreeMap::new();
    let mut scalars = BTreeSet::new();
    let mut total_wall = 0.0;
    let mut all_valid = true;
    let mut residuals_ok = true;
    for row in runs {
        let phase = row.k("phase").s().to_string();
        let arm = row.k("arm").s().to_string();
        let round = row.k("round").u() as u64;
        assert!(keys.insert((phase.clone(), round, arm.clone())), "duplicate key");
        all_valid &= row.k("valid").b() && row.k("isolation_admitted").b();
        scalars.insert(row.k("scalar").u());
        let w = row.k("wall_seconds").f();
        total_wall += w;
        let res = row.k("residuals");
        if arm == "rho" {
            residuals_ok &= res.is_null();
        } else {
            residuals_ok &= res.u() == 1506;
        }
        walls.entry((phase.clone(), arm.clone())).or_default().insert(round, w);
        if phase == "measurement" {
            order.entry(round).or_default().push(arm.clone());
        }
    }
    let mut expected = BTreeSet::new();
    for arm in ["baseline", "unit", "rho"] {
        expected.insert(("warmup".to_string(), 0u64, arm.to_string()));
        for i in 0..2 {
            expected.insert(("control".to_string(), i, arm.to_string()));
        }
        for i in 0..5 {
            expected.insert(("measurement".to_string(), i, arm.to_string()));
        }
    }
    r.check("registered_key_set", keys == expected, String::from("3 warmup + 6 control + 15 measurement"));
    r.check("all_valid_and_isolation_admitted", all_valid, String::new());
    r.check("ic_residuals_1506_rho_null", residuals_ok, String::new());
    r.check("single_recorded_scalar", scalars.len() == 1, format!("{:?}", scalars));
    let outer = 1311.20168185f64;
    r.note("sum_of_24_walls_vs_runner_outer", format!("{total_wall:.3} s of 24 walls; results/flow.json outer_seconds {outer}; slack {:.3} s", outer - total_wall));
    for (rd, arms) in &order {
        r.note(&format!("measurement_round_{rd}_order"), arms.join(","));
    }
    // preregistered: IC order reversed on odd rounds, rho position rotated
    let ic_first: Vec<String> = order
        .values()
        .map(|a| a.iter().filter(|x| *x != "rho").next().unwrap().clone())
        .collect();
    let rho_pos: Vec<usize> = order.values().map(|a| a.iter().position(|x| x == "rho").unwrap()).collect();
    r.note("ic_first_by_round", format!("{:?}", ic_first));
    r.note("rho_position_by_round", format!("{:?}", rho_pos));
    let alternates = ic_first.windows(2).all(|w| w[0] != w[1]);
    r.check("ic_order_alternates_by_round", alternates, String::new());
    let rho_rotates = rho_pos.windows(2).all(|w| w[0] != w[1]);
    r.check("rho_position_changes_each_round", rho_rotates, String::new());
    // per-arm statistics from rows, compared with the producer's arms[] and controls[]
    let arms = s.k("arms").a();
    let controls = s.k("controls").a();
    let mut med = BTreeMap::new();
    for arm in ["baseline", "unit", "rho"] {
        let m: Vec<f64> = walls[&("measurement".to_string(), arm.to_string())].values().copied().collect();
        let c: Vec<f64> = walls[&("control".to_string(), arm.to_string())].values().copied().collect();
        let mymed = median_odd(&m);
        let mymin = m.iter().copied().fold(f64::INFINITY, f64::min);
        med.insert(arm, mymed);
        let pa = arms.iter().find(|a| a.k("arm").s() == arm).unwrap();
        let pw: Vec<f64> = pa.k("walls").a().iter().map(J::f).collect();
        r.check(&format!("{arm}_walls_array_equals_rows"), pw == m, String::new());
        r.check(
            &format!("{arm}_median"),
            mymed == pa.k("median_seconds").f(),
            format!("recomputed {mymed:.9}, recorded {:.9}", pa.k("median_seconds").f()),
        );
        r.check(
            &format!("{arm}_minimum"),
            mymin == pa.k("minimum_seconds").f(),
            format!("recomputed {mymin:.9}, recorded {:.9}", pa.k("minimum_seconds").f()),
        );
        let aa = c[0].max(c[1]) / c[0].min(c[1]);
        let pc = controls.iter().find(|a| a.k("arm").s() == arm).unwrap();
        r.check(
            &format!("{arm}_AA_max_over_min_le_1.10"),
            aa <= 1.10 && rel_eq(aa, pc.k("max_over_min").f(), 1e-15),
            format!("recomputed {aa:.9}, recorded {:.9}", pc.k("max_over_min").f()),
        );
    }
    let comps = s.k("comparisons").a();
    for (n, d, readme) in [("unit", "baseline", (0.99832, 0.96725, 1.02717)), ("unit", "rho", (0.18701, 0.18372, 0.18928)), ("baseline", "rho", (0.18732, 0.18325, 0.19370))] {
        let x = &walls[&("measurement".to_string(), n.to_string())];
        let y = &walls[&("measurement".to_string(), d.to_string())];
        let ratios: Vec<f64> = (0..5).map(|i| x[&i] / y[&i]).collect();
        let g = geo(&ratios);
        let mr = med[n] / med[d];
        let pc = comps.iter().find(|c| c.k("numerator").s() == n && c.k("denominator").s() == d).unwrap();
        let pr: Vec<f64> = pc.k("paired_ratios").a().iter().map(J::f).collect();
        let tag = format!("{n}_over_{d}");
        r.check(&format!("{tag}_paired_ratios"), pr.iter().zip(&ratios).all(|(a, b)| rel_eq(*a, *b, 1e-15)), format!("{:?}", ratios));
        r.check(&format!("{tag}_geometric_mean"), rel_eq(g, pc.k("geometric_mean").f(), 1e-14), format!("recomputed {g:.12}, recorded {:.12}", pc.k("geometric_mean").f()));
        r.check(&format!("{tag}_median_ratio"), rel_eq(mr, pc.k("median_ratio").f(), 1e-15), format!("recomputed {mr:.12}, recorded {:.12}", pc.k("median_ratio").f()));
        let b = pc.k("bootstrap95").a();
        let (blo, bhi) = (b[0].f(), b[1].f());
        r.check(&format!("{tag}_bootstrap_draws_10000"), pc.k("bootstrap_draws").u() == 10000, String::new());
        let exact = exact_boot(&ratios);
        let nn = exact.len();
        // percentile rule of the producer's analyzer applied to the exact distribution
        let elo = exact[(0.025 * nn as f64).floor() as usize - 1 + 1 - 1];
        let ehi = exact[(0.975 * nn as f64).floor() as usize - 1];
        let (lo_lt, lo_le) = rank_of(&exact, blo);
        let (hi_lt, hi_le) = rank_of(&exact, bhi);
        r.check(
            &format!("{tag}_bootstrap_lo_attainable"),
            attainable(&ratios, blo).is_some(),
            format!("recorded {blo:.12} = geometric mean of multiset {:?}; exact-distribution rank [{lo_lt:.4},{lo_le:.4}]", attainable(&ratios, blo)),
        );
        r.check(
            &format!("{tag}_bootstrap_hi_attainable"),
            attainable(&ratios, bhi).is_some(),
            format!("recorded {bhi:.12} = geometric mean of multiset {:?}; exact-distribution rank [{hi_lt:.4},{hi_le:.4}]", attainable(&ratios, bhi)),
        );
        // the recorded MC endpoints must sit near the exact 2.5% / 97.5% points
        r.check(
            &format!("{tag}_bootstrap_lo_near_exact_2.5pct"),
            lo_le >= 0.015 && lo_lt <= 0.035,
            format!("exact 2.5% point {elo:.12}"),
        );
        r.check(
            &format!("{tag}_bootstrap_hi_near_exact_97.5pct"),
            hi_le >= 0.965 && hi_lt <= 0.985,
            format!("exact 97.5% point {ehi:.12}"),
        );
        // own Monte Carlo (SplitMix64), same draw count and order-statistic rule, 8 seeds
        let mut mc = Vec::new();
        for seed in 0..8u64 {
            let mut rng = Sm(0x5EED_0000 + seed);
            let mut v: Vec<f64> = (0..10000)
                .map(|_| {
                    let smp: Vec<f64> = (0..5).map(|_| ratios[rng.below(5) as usize]).collect();
                    geo(&smp)
                })
                .collect();
            v.sort_by(f64::total_cmp);
            mc.push((v[249], v[9749]));
        }
        let mclo = mc.iter().map(|x| x.0).fold((f64::INFINITY, f64::NEG_INFINITY), |a, b| (a.0.min(b), a.1.max(b)));
        let mchi = mc.iter().map(|x| x.1).fold((f64::INFINITY, f64::NEG_INFINITY), |a, b| (a.0.min(b), a.1.max(b)));
        r.note(&format!("{tag}_own_mc_8_seeds"), format!("lo in [{:.6},{:.6}], hi in [{:.6},{:.6}]", mclo.0, mclo.1, mchi.0, mchi.1));
        // README rounding
        r.check(
            &format!("{tag}_readme_rounding"),
            (g - readme.0).abs() < 5e-6 && (blo - readme.1).abs() < 5e-6 && (bhi - readme.2).abs() < 5e-6,
            format!("README {:?}", readme),
        );
        if n == "unit" && d == "baseline" {
            r.check("unit_over_baseline_interval_contains_1", blo < 1.0 && bhi > 1.0, String::from("no demonstrated E2E gain"));
            r.check("unit_over_baseline_exact_interval_contains_1", elo < 1.0 && ehi > 1.0, format!("exact [{elo:.6},{ehi:.6}]"));
            r.check("readme_ratio_of_medians_1.01870", (mr - 1.01870).abs() < 5e-6, format!("{mr:.6}"));
        }
    }
    for (arm, rm) in [("baseline", 20.71449), ("unit", 21.10177), ("rho", 111.40204)] {
        r.check(&format!("readme_table_median_{arm}"), (med[arm] - rm).abs() < 5e-6, format!("{:.6}", med[arm]));
    }
    r.check("summary_admitted_true", s.k("admitted").b(), String::new());
    r.check("rejection_reasons_empty", s.k("rejection_reasons").a().is_empty(), String::new());
    r.check("complete_operation_count_null", s.k("complete_operation_count").is_null(), String::new());
    r.check("general_rho_parity_null", s.k("general_rho_parity").is_null(), String::new());
    r.finish("nagao");
}

// ───────────────────────────── bitop/v1 prices ─────────────────────────────
fn bits(x: u128) -> u32 {
    128 - x.leading_zeros()
}

/// bitop/v1 prices derived from the unit's written rules (docs and
/// src/cryptanalysis/bitops.rs doc comments): ripple add 5w, schoolbook
/// product w^2 + 5w(w-1), Montgomery product 2*mul + add, etc.
fn prices(p: u128, l: u128) -> BTreeMap<&'static str, u128> {
    let b = bits(p) as u128;
    let w = bits(l) as u128;
    let iadd = |w: u128| 5 * w;
    let imul = |w: u128| w * w + 5 * w * (w - 1);
    let fadd = |w: u128| 2 * iadd(w) + 3 * w;
    let fmul = |w: u128| 2 * imul(w) + fadd(w);
    let finv = |w: u128| 2 * w * fadd(w);
    let mut m = BTreeMap::new();
    m.insert("field_add", fadd(b));
    m.insert("field_mul", fmul(b));
    m.insert("field_sqr", fmul(b));
    m.insert("field_inv", finv(b));
    m.insert("field_sqrt", 2 * b * fmul(b));
    m.insert("legendre", finv(b));
    m.insert("hash_probe", 4 * b);
    m.insert("modr_add", fadd(w));
    m.insert("modr_mul", fmul(w));
    m.insert("modr_inv", finv(w));
    m.insert("word_op", 64);
    m.insert("rng_words", 0);
    m
}

fn price_prims(prims: &J, pr: &BTreeMap<&'static str, u128>) -> u128 {
    let mut t = 0;
    for (k, v) in prims.o() {
        let price = *pr.get(k.as_str()).unwrap_or_else(|| panic!("unpriced primitive {k}"));
        t += v.u() * price;
    }
    t
}

fn read_time(path: &str) -> f64 {
    let s = fs::read_to_string(path).unwrap();
    let line = s.lines().find(|l| l.starts_with("real_s=")).unwrap();
    line[7..].parse().unwrap()
}

fn cmd_run50(args: &[String]) {
    let dir = &args[0];
    let mut r = Report::new();
    let q = load(&format!("{dir}/qualified-original.json"));
    let st = load(&format!("{dir}/run-study.json"));
    let pubj = load(&format!("{dir}/public.json"));
    let p = pubj.k("p").u();
    let l = pubj.k("l").u();
    let pr = prices(p, l);
    r.note("bitop_v1_prices", format!("field_bits {} r_bits {} {:?}", bits(p), bits(l), pr));
    let readme: BTreeMap<&str, (u128, f64, f64)> = [
        ("original-ic", (4_525_556_132_414u128, 18.553, 18.558)),
        ("quadratic-table", (5_230_830_441_710, 14.953, 15.116)),
        ("pair-discriminant", (6_235_988_188_572, 27.726, 31.531)),
        ("pair-deferred", (5_913_875_255_822, 24.727, 27.829)),
        ("rho-plain-b1", (196_545_134_504_212, 119.764, 119.768)),
        ("rho-neg-b1", (181_105_019_144_798, 117.560, 117.565)),
        ("rho-plain-b16", (109_992_238_398_614, 36.902, 36.908)),
        ("rho-neg-b16", (274_525_819_370_690, 102.918, 102.924)),
    ]
    .into_iter()
    .collect();
    let mut totals: BTreeMap<String, u128> = BTreeMap::new();
    let mut phase_tot: BTreeMap<(String, String), u128> = BTreeMap::new();
    let mut inner: BTreeMap<String, f64> = BTreeMap::new();
    let mut process: BTreeMap<String, f64> = BTreeMap::new();
    let mut scal = BTreeSet::new();
    for o in q.k("outcomes").a() {
        let mode = o.k("mode").s().to_string();
        let acc = o.k("proof").k("accounting");
        let mut sum = 0u128;
        let mut obs: BTreeMap<String, u128> = BTreeMap::new();
        let mut all_exact = true;
        for ph in acc.k("phases").a() {
            let name = ph.k("phase").s().to_string();
            let priced = price_prims(ph.k("primitives"), &pr);
            let rec = ph.k("diagnostic_priced_bitops").u();
            all_exact &= priced == rec;
            sum += rec;
            phase_tot.insert((mode.clone(), name), rec);
            for (k, v) in ph.k("primitives").o() {
                *obs.entry(k.clone()).or_default() += v.u();
            }
        }
        r.check(&format!("{mode}_every_phase_repriced_exactly"), all_exact, String::new());
        let sub = acc.k("observed_primitive_subtotal_bitops").u();
        r.check(&format!("{mode}_phase_sum_equals_subtotal"), sum == sub, format!("{sum} vs {sub}"));
        let op = o.k("observed_primitives");
        let obs_ok = op.o().iter().all(|(k, v)| obs.get(k).copied().unwrap_or(0) == v.u());
        r.check(&format!("{mode}_observed_primitives_equal_phase_sums"), obs_ok, String::new());
        totals.insert(mode.clone(), sub);
        let iw = o.k("inner_wall_ns").u() as f64 / 1e9;
        inner.insert(mode.clone(), iw);
        let pt = read_time(&format!("{dir}/{mode}.time.txt"));
        process.insert(mode.clone(), pt);
        let (rb, ri, rp) = readme[mode.as_str()];
        r.check(&format!("{mode}_readme_bitops"), rb == sub, format!("{sub}"));
        r.check(&format!("{mode}_readme_inner_seconds"), (iw - ri).abs() < 5e-4, format!("{iw:.4}"));
        r.check(&format!("{mode}_readme_process_seconds"), (pt - rp).abs() < 5e-4, format!("{pt:.3}"));
        r.check(&format!("{mode}_inner_le_process"), iw <= pt, String::new());
        // run-study agreement
        let so = st.k("outcomes").a().iter().find(|x| x.k("mode").s() == mode).unwrap();
        r.check(&format!("{mode}_run_study_bitops_equal"), so.k("observed_incomplete_bitops").u() == sub, String::new());
        r.check(&format!("{mode}_run_study_process_equal"), so.k("process_real_seconds_practicality_only").f() == pt, String::new());
        scal.insert(so.k("actual_scalar").u());
        let sp: BTreeMap<String, u128> = so.k("phases").a().iter().map(|x| (x.k("phase").s().to_string(), x.k("observed_incomplete_bitops").u())).collect();
        let phases_ok = sp.iter().all(|(k, v)| phase_tot.get(&(mode.clone(), k.clone())).copied().unwrap_or(0) == *v);
        r.check(&format!("{mode}_run_study_phases_equal"), phases_ok, String::new());
    }
    // raw records retained for original-ic and the four rho arms
    for mode in ["original-ic", "rho-plain-b1", "rho-neg-b1", "rho-plain-b16", "rho-neg-b16"] {
        let raw = load(&format!("{dir}/{mode}.raw.json"));
        scal.insert(raw.k("actual_scalar").u());
        let pubr = raw.k("public");
        r.check(&format!("{mode}_raw_public_matches"), pubr.k("p").u() == p && pubr.k("l").u() == l && format!("{:?}", pubr.k("alpha")) == format!("{:?}", pubj.k("alpha")) && format!("{:?}", pubr.k("q")) == format!("{:?}", pubj.k("q")) && format!("{:?}", pubr.k("g")) == format!("{:?}", pubj.k("g")), String::new());
        let iw = raw.k("inner_wall_ns").u() as f64 / 1e9;
        r.check(&format!("{mode}_raw_inner_wall_equals_qualified"), (iw - inner[mode]).abs() < 1e-12, String::new());
        if mode.starts_with("rho") {
            let sig = raw.k("signature");
            scal.insert(sig.k("scalar").u());
            scal.insert(raw.k("recovered_scalar").u());
            let wn = sig.k("work_native");
            let mut sum = 0u128;
            let mut exact = true;
            for ph in wn.k("phases").a() {
                let priced = price_prims(ph.k("primitives"), &pr);
                exact &= priced == ph.k("diagnostic_priced_bitops").u();
                sum += ph.k("diagnostic_priced_bitops").u();
            }
            r.check(&format!("{mode}_raw_phases_repriced_exactly"), exact && sum == wn.k("diagnostic_priced_bitops").u() && sum == totals[mode], format!("{sum}"));
            let steps = sig.k("work").k("run").k("steps").u() as f64;
            let neg = raw.k("rho_options").k("negation").b();
            let expect = if neg { (std::f64::consts::PI / 4.0).sqrt() } else { (std::f64::consts::PI / 2.0).sqrt() };
            let ratio = steps / (l as f64).sqrt();
            r.note(&format!("{mode}_walk_length"), format!("steps {steps}, steps/sqrt(L) {ratio:.4}, generic expectation {expect:.4} (ratio to expectation {:.3})", ratio / expect));
            let unc: Vec<String> = wn.k("known_uncounted").a().iter().map(|u| format!("{}={}", u.k("name").s(), if u.k("count").is_null() { "null".to_string() } else { u.k("count").u().to_string() })).collect();
            r.note(&format!("{mode}_known_uncounted"), unc.join(", "));
        }
    }
    r.check("all_scalars_identical", scal.len() == 1, format!("{:?}", scal));
    // deltas
    let g = totals["pair-discriminant"];
    let d = totals["pair-deferred"];
    let saving = g - d;
    let pct = saving as f64 / g as f64 * 100.0;
    r.check("deferred_saving_322112932750", saving == 322_112_932_750, format!("{saving}"));
    r.check("deferred_saving_percent_5.165387153", (pct - 5.165387153).abs() < 5e-10, format!("{pct:.12}"));
    r.check("run_study_saving_string", st.k("deferred_observed_saving_vs_grounded").u() == saving, String::new());
    r.check("run_study_saving_percent", rel_eq(st.k("deferred_observed_saving_percent_vs_grounded").f(), pct, 1e-15), String::new());
    let ph = |m: &str, n: &str| phase_tot[&(m.to_string(), n.to_string())] as i128;
    let rel_save = ph("pair-discriminant", "relations") - ph("pair-deferred", "relations");
    let la_inc = ph("pair-deferred", "linear_algebra") - ph("pair-discriminant", "linear_algebra");
    r.check("relations_saving_385877619300", rel_save == 385_877_619_300, format!("{rel_save}"));
    r.check("la_increase_63764686550", la_inc == 63_764_686_550, format!("{la_inc}"));
    r.check("saving_decomposes", rel_save - la_inc == saving as i128, String::new());
    let other_equal = ["model", "factor_base", "descent", "verification", "walk", "transport", "unattributed"].iter().all(|n| ph("pair-discriminant", n) == ph("pair-deferred", n));
    r.check("other_phases_equal_between_graph_arms", other_equal, String::new());
    let desc = ph("pair-deferred", "descent");
    r.check("descent_3426280619436_in_all_three_graph_arms", ["quadratic-table", "pair-discriminant", "pair-deferred"].iter().all(|m| ph(m, "descent") == 3_426_280_619_436) && desc == 3_426_280_619_436, String::new());
    // orderings stated in README
    r.check("subtotal_order_original_lt_table_lt_deferred", totals["original-ic"] < totals["quadratic-table"] && totals["quadratic-table"] < d, String::new());
    let fastest_ic = ["original-ic", "quadratic-table", "pair-discriminant", "pair-deferred"].iter().min_by(|a, b| inner[**a].total_cmp(&inner[**b])).unwrap();
    r.check("table_lowest_elapsed", *fastest_ic == "quadratic-table", format!("{fastest_ic}"));
    let rho_modes = ["rho-plain-b1", "rho-neg-b1", "rho-plain-b16", "rho-neg-b16"];
    let fastest_rho = rho_modes.iter().min_by(|a, b| inner[**a].total_cmp(&inner[**b])).unwrap();
    let cheapest_rho = rho_modes.iter().min_by_key(|m| totals[**m]).unwrap();
    r.check("fastest_rho_plain_b16", *fastest_rho == st.k("fastest_observed_rho_mode").s(), format!("{fastest_rho}"));
    r.check("cheapest_rho_plain_b16", *cheapest_rho == st.k("cheapest_observed_rho_mode").s(), format!("{cheapest_rho}"));
    r.check("deferred_process_below_all_four_rho_processes", rho_modes.iter().all(|m| process["pair-deferred"] < process[*m]), String::new());
    let sum_proc: f64 = process.values().sum();
    r.note("sum_of_8_process_seconds_vs_time_txt", format!("{sum_proc:.3} vs {}", read_time(&format!("{dir}/time.txt"))));
    // graph statistics: internal consistency only (raw graph witnesses are LOCAL)
    let so = st.k("outcomes").a().iter().find(|x| x.k("mode").s() == "pair-deferred").unwrap();
    let gs = so.k("graph_stats");
    r.check("deferred_minus_activated_equals_live", gs.k("deferred").u() - gs.k("activated").u() == gs.k("live_deferred").u(), format!("{} - {} = {}", gs.k("deferred").u(), gs.k("activated").u(), gs.k("live_deferred").u()));
    r.check("anchor_classes_sum", gs.k("one_class_anchors").u() + gs.k("two_class_anchors").u() == gs.k("anchors").u(), String::new());
    r.check("readme_graph_numbers", gs.k("vertices").u() == 125491 && gs.k("anchors").u() == 20510 && gs.k("deferred").u() == 77994 && gs.k("activated").u() == 8145 && gs.k("max_stored_witness").u() == 16 && gs.k("max_emitted_witness").u() == 13 && gs.k("edge_refusals").u() + gs.k("vertex_refusals").u() + gs.k("witness_refusals").u() == 0, String::new());
    let rc = &so.k("rank_checks").a()[0];
    let gso = st.k("outcomes").a().iter().find(|x| x.k("mode").s() == "pair-discriminant").unwrap();
    let grc = &gso.k("rank_checks").a()[0];
    r.check("readme_rank_rows_sources", rc.k("rank_certificate").u() == 508 && rc.k("b_count").u() == 155648 && rc.k("rows").u() == 518 && so.k("sources").u() == 90877 && grc.k("b_count").u() == 185344 && grc.k("rows").u() == 522 && gso.k("sources").u() == 108058, String::new());
    r.check("run_study_complete_cost_false_ratios_null_parity_false", !st.k("complete_cost").b() && st.k("ratios").is_null() && !st.k("rho_parity").b() && st.k("complete_total").is_null(), String::new());
    r.finish("run50");
}

// ───────────────────────────── descent-finder ─────────────────────────────
fn cmd_descent(args: &[String]) {
    let (dir, df) = (&args[0], &args[1]);
    let mut r = Report::new();
    let q = load(&format!("{dir}/qualified-original.json"));
    let pubj = load(&format!("{dir}/public.json"));
    let (p, l) = (pubj.k("p").u(), pubj.k("l").u());
    let pr = prices(p, l);
    let bk = load(&format!("{df}/breakdown.json"));
    let readme_share: BTreeMap<&str, f64> = [("original-ic", 38.3), ("quadratic-table", 65.5), ("pair-discriminant", 54.9), ("pair-deferred", 57.9)].into_iter().collect();
    for o in q.k("outcomes").a() {
        let mode = o.k("mode").s();
        if mode.starts_with("rho") {
            continue;
        }
        let acc = o.k("proof").k("accounting");
        let ph = acc.k("phases").a().iter().find(|x| x.k("phase").s() == "descent").unwrap();
        let rec = ph.k("diagnostic_priced_bitops").u();
        let priced = price_prims(ph.k("primitives"), &pr);
        let res = o.k("descent_residuals").u();
        let sub = acc.k("observed_primitive_subtotal_bitops").u();
        let share = rec as f64 / sub as f64 * 100.0;
        let per = rec as f64 / res as f64;
        let ba = bk.k("arms").a().iter().find(|x| x.k("mode").s() == mode).unwrap();
        r.check(&format!("{mode}_descent_reprice"), priced == rec && ba.k("repriced_descent_bitops").u() == rec && ba.k("recorded_descent_bitops").u() == rec, format!("{rec}"));
        r.check(&format!("{mode}_descent_share"), (share - ba.k("descent_share_of_observed_subtotal_percent").f()).abs() < 5e-5 && (share - readme_share[mode]).abs() < 0.05, format!("{share:.4}%"));
        r.check(&format!("{mode}_per_residual"), (per - ba.k("descent_bitops_per_residual").f()).abs() < 0.05, format!("{per:.1}"));
        let fm = ph.k("primitives").k("field_mul").u() * pr["field_mul"];
        r.note(&format!("{mode}_field_mul_share"), format!("{:.4}%", fm as f64 / rec as f64 * 100.0));
    }
    // legacy stage split from the retained original-ic raw record
    let raw = load(&format!("{dir}/original-ic.raw.json"));
    let w = raw.k("work");
    let f4 = w.k("descent_f4_muls").u();
    let lin = w.k("descent_lin_muls").u();
    let weil = w.k("descent_weil_muls").u();
    let post = w.k("descent_post_muls").u();
    let stream = w.k("descent_stream_muls").u();
    let ver = w.k("descent_verify_muls").u();
    let tot = w.k("descent_muls").u();
    r.check("legacy_stage_sum_equals_total", f4 + lin + weil + post + stream + ver == tot, format!("{} vs {tot}", f4 + lin + weil + post + stream + ver));
    let sh = |x: u128| x as f64 / tot as f64 * 100.0;
    r.check("legacy_f4_share_77.43", (sh(f4) - 77.43).abs() < 0.005, format!("{:.4}%", sh(f4)));
    r.check("legacy_linear_share_21.87", (sh(lin) - 21.87).abs() < 0.005, format!("{:.4}%", sh(lin)));
    r.check("legacy_weil_0.30_stream_0.23_lift_0.17", (sh(weil) - 0.30).abs() < 0.005 && (sh(stream) - 0.23).abs() < 0.005 && (sh(post) - 0.17).abs() < 0.005, format!("{:.4} {:.4} {:.4}", sh(weil), sh(stream), sh(post)));
    r.check("legacy_successes_2_of_1728_stopped_1728_fallbacks_3_replayed_0", w.k("descent_successes").u() == 2 && w.k("descent_residuals").u() == 1728 && w.k("descent_stopped").u() == 1728 && w.k("descent_lin_fallback").u() == 3 && w.k("descent_replayed").u() == 0, String::new());
    let hooked_mul = q.k("outcomes").a().iter().find(|x| x.k("mode").s() == "original-ic").unwrap().k("proof").k("accounting").k("phases").a().iter().find(|x| x.k("phase").s() == "descent").unwrap().k("primitives").k("field_mul").u();
    let without = tot - lin;
    r.check("reconciliation_1245470049_vs_1243056697", hooked_mul == 1_245_470_049 && without == 1_243_056_697, format!("diff {:.4}%", (hooked_mul as f64 - without as f64) / without as f64 * 100.0));
    r.check("reconciliation_gap_345496566", tot - hooked_mul == 345_496_566, String::new());
    // traced compare arithmetic
    for (name, readme) in [("seed389-n3415", (2.29, 0.451, 0.559, 0.586, 1.71)), ("seed401-n1506", (2.30, 0.448, 0.556, 0.617, 1.62))] {
        let n = name.split("-n").nth(1).unwrap();
        let sm = load(&format!("{df}/runs/traced-{name}.summary.json"));
        let cp = load(&format!("{df}/runs/compare-{name}.json"));
        let arm = |a: &str| sm.k("arms").a().iter().find(|x| x.k("arm").s() == a).unwrap().clone();
        let (pa, ta, oa) = (arm("production"), arm("traced"), arm("oracle"));
        let pf4 = pa.k("f4_ops").u();
        let tf4 = ta.k("f4_ops").u();
        let ph = pa.k("hooked_descent_bitops").u();
        let th = ta.k("hooked_descent_bitops").u();
        let lm = pa.k("lin_muls").u();
        let tr = sm.k("trace");
        let seen = tr.k("linear_stage_hooked_bitops").u();
        let lin_price = lm * pr["field_mul"] - seen;
        let learn = tr.k("learning_rref_upper_bound_uncounted_by_f4fp").u() * (pr["field_mul"] + pr["field_add"]);
        let r1 = pf4 as f64 / tf4 as f64;
        let r2 = th as f64 / ph as f64;
        let r3 = (th + lin_price) as f64 / (ph + lin_price) as f64;
        let r4 = (th + lin_price + learn) as f64 / (ph + lin_price) as f64;
        r.check(&format!("{name}_lin_muls_equal_all_arms"), lm == ta.k("lin_muls").u() && lm == oa.k("lin_muls").u() && lm == tr.k("linear_stage_lin_muls").u(), String::new());
        r.check(&format!("{name}_unhooked_linear_price"), lin_price == cp.k("linear_stage").k("unhooked_price_lower_bound").u(), format!("{lm} x {} - {seen} = {lin_price}", pr["field_mul"]));
        r.check(&format!("{name}_with_linear_priced"), ph + lin_price == cp.k("with_linear_stage_priced").k("production").u() && th + lin_price == cp.k("with_linear_stage_priced").k("traced").u(), String::new());
        r.check(&format!("{name}_with_learning_bound"), th + lin_price + learn == cp.k("with_linear_stage_and_learning_bound").k("traced").u(), String::new());
        r.check(&format!("{name}_readme_ratios"), (r1 - readme.0).abs() < 0.005 && (r2 - readme.1).abs() < 0.0005 && (r3 - readme.2).abs() < 0.0005 && (r4 - readme.3).abs() < 0.0005 && (1.0 / r4 - readme.4).abs() < 0.005, format!("{r1:.4} {r2:.4} {r3:.4} {r4:.4} ({:.3}x)", 1.0 / r4));
        r.check(&format!("{name}_residuals"), sm.k("residuals").u().to_string() == n, String::new());
        r.check(&format!("{name}_no_disagreement_or_bad"), sm.k("disagreements").u() + sm.k("bad_decompositions").u() + sm.k("bad_residuals").u() + sm.k("bad_solutions").u() == 0 && sm.k("foreign_hook_calls").u() == 0, String::new());
        let of4 = oa.k("f4_ops").u();
        r.note(&format!("{name}_oracle_vs_production_f4"), format!("production is {:.2}% below oracle", (1.0 - pf4 as f64 / of4 as f64) * 100.0));
        r.note(&format!("{name}_learning_bound_share_of_traced"), format!("{:.2}%", learn as f64 / (th + lin_price + learn) as f64 * 100.0));
        if name == "seed389-n3415" {
            let rec = 3_426_280_619_436u128;
            let below = (rec - ph) as f64 / rec as f64 * 100.0;
            r.check("production_hooked_0.30pct_below_run50", (below - 0.30).abs() < 0.005, format!("{below:.4}%"));
            let lower = lin_price as f64;
            let full = (lm * pr["field_mul"]) as f64;
            r.note("omitted_linear_stage_fraction", format!(
                "unhooked lower bound {lin_price} = {:.2}% of the production hooked subtotal, {:.2}% of the recorded run50 subtotal, but {:.2}% of (recorded + omitted) descent work; full lin_muls x fp_mul {} = {:.2}% of recorded",
                lower / ph as f64 * 100.0, lower / rec as f64 * 100.0, lower / (rec as f64 + lower) * 100.0, full as u128, full / rec as f64 * 100.0));
        }
    }
    r.finish("descent");
}

fn cmd_rows(args: &[String]) {
    let mut r = Report::new();
    let sm = load(&args[0]);
    let mut text = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut text).unwrap();
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    let n = sm.k("residuals").u() as usize;
    r.check("row_count_plus_trailer", lines.len() == n + 1, format!("{} lines for {n} residuals", lines.len()));
    let trailer = parse(lines[n]);
    r.check("trailer_equals_summary", format!("{:?}", trailer) == format!("{:?}", sm), String::new());
    let mut f4 = [0u128; 3];
    let mut lin = [0u128; 3];
    let mut sol = [0u128; 3];
    let mut decs = [0u128; 3];
    let (mut replayed, mut mismatch, mut disagree) = (0u128, 0u128, 0u128);
    let mut idx_ok = true;
    let mut dec_idx = Vec::new();
    let mut identical_decs = true;
    let mut per_res_ratio: Vec<f64> = Vec::new();
    for (i, line) in lines[..n].iter().enumerate() {
        let row = parse(line);
        idx_ok &= row.k("residual_index").u() as usize == i;
        for a in 0..3 {
            f4[a] += row.k("f4_ops").a()[a].u();
            lin[a] += row.k("lin_muls").a()[a].u();
            sol[a] += row.k("solutions").a()[a].u();
            decs[a] += row.k("decompositions").a()[a].a().len() as u128;
        }
        let d = row.k("decompositions").a();
        identical_decs &= format!("{:?}", d[0]) == format!("{:?}", d[1]) && format!("{:?}", d[1]) == format!("{:?}", d[2]);
        if !d[0].a().is_empty() {
            dec_idx.push(i);
            for dd in d[0].a() {
                let terms = dd.k("terms").a();
                r.note(&format!("decomposition_at_{i}"), format!("{} terms {:?}", terms.len(), terms.iter().map(|t| (t.a()[0].u(), t.a()[1].i())).collect::<Vec<_>>()));
            }
        }
        replayed += row.k("traced_replayed").b() as u128;
        mismatch += row.k("traced_mismatch").b() as u128;
        disagree += (!row.k("agree").b()) as u128;
        let fo = row.k("f4_ops").a();
        if row.k("traced_replayed").b() {
            per_res_ratio.push(fo[0].u() as f64 / fo[1].u() as f64);
        }
    }
    r.check("residual_indices_sequential", idx_ok, String::new());
    let arms = sm.k("arms").a();
    for (a, name) in ["production", "traced", "oracle"].iter().enumerate() {
        let sa = arms.iter().find(|x| x.k("arm").s() == *name).unwrap();
        r.check(&format!("{name}_f4_ops_sum"), f4[a] == sa.k("f4_ops").u(), format!("{}", f4[a]));
        r.check(&format!("{name}_lin_muls_sum"), lin[a] == sa.k("lin_muls").u(), format!("{}", lin[a]));
        r.check(&format!("{name}_solutions_sum"), sol[a] == sa.k("solutions").u(), format!("{}", sol[a]));
        r.check(&format!("{name}_decompositions_sum"), decs[a] == sa.k("decompositions").u(), format!("{}", decs[a]));
    }
    r.check("decomposition_lists_identical_across_arms", identical_decs, String::new());
    let rec_idx: Vec<usize> = sm.k("decomposing_residual_indices").a().iter().map(|x| x.u() as usize).collect();
    r.check("decomposing_indices", rec_idx == dec_idx, format!("{:?}", dec_idx));
    r.check("last_residual_decomposes_stop_rule", dec_idx.last() == Some(&(n - 1)), String::new());
    let tr = sm.k("trace");
    r.check("replays_accepted_count", replayed == tr.k("accepted").u(), format!("{replayed}"));
    r.check("mismatch_count_equals_accepted_after_mismatch", mismatch == tr.k("accepted_after_mismatch").u(), format!("{mismatch}"));
    r.check("no_disagreements", disagree == 0 && sm.k("disagreements").u() == 0, String::new());
    per_res_ratio.sort_by(f64::total_cmp);
    if !per_res_ratio.is_empty() {
        r.note("per_residual_f4_ratio_production_over_traced", format!("min {:.4} median {:.4} max {:.4}", per_res_ratio[0], per_res_ratio[per_res_ratio.len() / 2], per_res_ratio[per_res_ratio.len() - 1]));
    }
    r.finish("rows");
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("ec") => cmd_ec(&a[1..]),
        Some("nagao") => cmd_nagao(&a[1..]),
        Some("run50") => cmd_run50(&a[1..]),
        Some("descent") => cmd_descent(&a[1..]),
        Some("rows") => cmd_rows(&a[1..]),
        _ => {
            eprintln!("usage: validate ec|nagao|run50|descent|rows ...");
            std::process::exit(2);
        }
    }
}
