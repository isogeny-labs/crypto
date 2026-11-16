//! Independent scalar verifier for `N2_DUMP` replay lines (validation of the
//! Weil-descent round-3/4 benchmark studies).
//!
//! Written by the validator from the dump format alone; it shares no code with
//! `examples/n2_routes.rs`. Std-only; arithmetic in u128.
//!
//! Usage: `verify_dump <dump.jsonl>...`
//!
//! For every line `{"mode", ids..., "p", "kappa", "a", "b", "P", "Q", "ell",
//! "N", "k", "arm", "k_found"}` it checks, over F_{p^2} = F_p[i]/(i^2 - kappa)
//! and E: y^2 = x^3 + a x + b:
//!   field:   p prime (deterministic Miller-Rabin), kappa a non-residue mod p;
//!   curve:   4a^3 + 27b^2 != 0;
//!   points:  P and Q on E, P != O;
//!   ell:     ell prime, ell * P = O, ell | N;
//!   N:       |N - (p^2 + 1)| <= 2p and N * R = O for 4 random R on E;
//!   answer:  1 <= k < ell, k_found * P = Q, k_found == k.
//! It also classifies j(E): in F_p or not, j = 0, j = 1728 (symmetry question).
//! Prints a per-file and overall summary; exit code 1 if any check fails.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

// ---------------------------------------------------------------- JSON (subset)
#[derive(Debug, Clone)]
enum Js {
    Null,
    Num(u128),
    Str(String),
    Arr(Vec<Js>),
    Obj(Vec<(String, Js)>),
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && (self.s[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn val(&mut self) -> Result<Js, String> {
        self.ws();
        let c = *self.s.get(self.i).ok_or("eof")?;
        match c {
            b'{' => {
                self.i += 1;
                let mut v = Vec::new();
                loop {
                    self.ws();
                    if self.s[self.i] == b'}' {
                        self.i += 1;
                        break;
                    }
                    let k = match self.val()? {
                        Js::Str(s) => s,
                        _ => return Err("key".into()),
                    };
                    self.ws();
                    if self.s[self.i] != b':' {
                        return Err("colon".into());
                    }
                    self.i += 1;
                    let x = self.val()?;
                    v.push((k, x));
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    }
                }
                Ok(Js::Obj(v))
            }
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                loop {
                    self.ws();
                    if self.s[self.i] == b']' {
                        self.i += 1;
                        break;
                    }
                    v.push(self.val()?);
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    }
                }
                Ok(Js::Arr(v))
            }
            b'"' => {
                self.i += 1;
                let st = self.i;
                while self.s[self.i] != b'"' {
                    self.i += 1;
                }
                let r = String::from_utf8(self.s[st..self.i].to_vec()).map_err(|e| e.to_string())?;
                self.i += 1;
                Ok(Js::Str(r))
            }
            b'n' => {
                if &self.s[self.i..self.i + 4] == b"null" {
                    self.i += 4;
                    Ok(Js::Null)
                } else {
                    Err("bad literal".into())
                }
            }
            b'0'..=b'9' => {
                let mut n: u128 = 0;
                while self.i < self.s.len() && self.s[self.i].is_ascii_digit() {
                    n = n.checked_mul(10).ok_or("overflow")? + (self.s[self.i] - b'0') as u128;
                    self.i += 1;
                }
                if self.i < self.s.len() && (self.s[self.i] == b'.' || self.s[self.i] == b'-') {
                    return Err("non-integer number".into());
                }
                Ok(Js::Num(n))
            }
            _ => Err(format!("unexpected byte {} at {}", c as char, self.i)),
        }
    }
}

fn get<'a>(o: &'a Js, k: &str) -> Result<&'a Js, String> {
    if let Js::Obj(v) = o {
        for (kk, x) in v {
            if kk == k {
                return Ok(x);
            }
        }
    }
    Err(format!("missing key {k}"))
}
fn num(o: &Js, k: &str) -> Result<u128, String> {
    match get(o, k)? {
        Js::Num(n) => Ok(*n),
        _ => Err(format!("{k} not a number")),
    }
}
fn st(o: &Js, k: &str) -> Result<String, String> {
    match get(o, k)? {
        Js::Str(s) => Ok(s.clone()),
        _ => Err(format!("{k} not a string")),
    }
}

// ---------------------------------------------------------------- F_p, F_{p^2}
#[derive(Clone, Copy)]
struct Fld {
    p: u128,
    kappa: u128,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct F2 {
    re: u128,
    im: u128,
}

impl Fld {
    fn md(&self, a: u128) -> u128 {
        a % self.p
    }
    fn fadd(&self, a: u128, b: u128) -> u128 {
        (a + b) % self.p
    }
    fn fsub(&self, a: u128, b: u128) -> u128 {
        (a + self.p - b) % self.p
    }
    fn fmul(&self, a: u128, b: u128) -> u128 {
        // p < 2^63 is asserted at load, so a*b < 2^126
        (a * b) % self.p
    }
    fn fpow(&self, mut b: u128, mut e: u128) -> u128 {
        let mut r = 1 % self.p;
        b %= self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.fmul(r, b);
            }
            b = self.fmul(b, b);
            e >>= 1;
        }
        r
    }
    fn finv(&self, a: u128) -> u128 {
        assert!(a % self.p != 0, "inverse of 0 in F_p");
        self.fpow(a, self.p - 2)
    }
    fn c(&self, re: u128, im: u128) -> F2 {
        F2 { re: self.md(re), im: self.md(im) }
    }
    fn zero(&self) -> F2 {
        F2 { re: 0, im: 0 }
    }
    fn one(&self) -> F2 {
        F2 { re: 1 % self.p, im: 0 }
    }
    fn add(&self, a: F2, b: F2) -> F2 {
        F2 { re: self.fadd(a.re, b.re), im: self.fadd(a.im, b.im) }
    }
    fn sub(&self, a: F2, b: F2) -> F2 {
        F2 { re: self.fsub(a.re, b.re), im: self.fsub(a.im, b.im) }
    }
    fn neg(&self, a: F2) -> F2 {
        self.sub(self.zero(), a)
    }
    fn mul(&self, a: F2, b: F2) -> F2 {
        // (a0 + a1 i)(b0 + b1 i) = a0 b0 + kappa a1 b1 + (a0 b1 + a1 b0) i
        let re = self.fadd(self.fmul(a.re, b.re), self.fmul(self.kappa, self.fmul(a.im, b.im)));
        let im = self.fadd(self.fmul(a.re, b.im), self.fmul(a.im, b.re));
        F2 { re, im }
    }
    fn scal(&self, k: u128, a: F2) -> F2 {
        self.mul(self.c(k, 0), a)
    }
    fn inv(&self, a: F2) -> F2 {
        // 1/(a0 + a1 i) = (a0 - a1 i) / (a0^2 - kappa a1^2)
        let nrm = self.fsub(self.fmul(a.re, a.re), self.fmul(self.kappa, self.fmul(a.im, a.im)));
        let ni = self.finv(nrm);
        F2 { re: self.fmul(a.re, ni), im: self.fmul(self.fsub(0, a.im), ni) }
    }
    fn pow(&self, mut b: F2, mut e: u128) -> F2 {
        let mut r = self.one();
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    fn is_zero(&self, a: F2) -> bool {
        a.re == 0 && a.im == 0
    }
    /// Tonelli-Shanks in F_{q}, q = p^2; `z` a fixed non-square of F_q.
    fn sqrt(&self, a: F2, z: F2) -> Option<F2> {
        if self.is_zero(a) {
            return Some(self.zero());
        }
        let q = self.p * self.p;
        if self.pow(a, (q - 1) / 2) != self.one() {
            return None;
        }
        let mut s = 0u32;
        let mut t = q - 1;
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        let mut m = s;
        let mut c = self.pow(z, t);
        let mut tt = self.pow(a, t);
        let mut r = self.pow(a, (t + 1) / 2);
        while tt != self.one() {
            let mut i = 0u32;
            let mut t2 = tt;
            while t2 != self.one() {
                t2 = self.mul(t2, t2);
                i += 1;
                if i == m {
                    return None;
                }
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(b, b);
            }
            m = i;
            c = self.mul(b, b);
            tt = self.mul(tt, c);
            r = self.mul(r, b);
        }
        assert!(self.mul(r, r) == a, "sqrt self-check");
        Some(r)
    }
}

// ---------------------------------------------------------------- curve
type Pt = Option<(F2, F2)>; // None = point at infinity

struct Curve {
    f: Fld,
    a: F2,
    b: F2,
}

impl Curve {
    fn rhs(&self, x: F2) -> F2 {
        let f = &self.f;
        f.add(f.add(f.mul(f.mul(x, x), x), f.mul(self.a, x)), self.b)
    }
    fn on(&self, p: Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => self.f.mul(y, y) == self.rhs(x),
        }
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = &self.f;
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        let lam = if x1 == x2 {
            if f.is_zero(f.add(y1, y2)) {
                return None; // P + (-P), includes 2-torsion doubling
            }
            // tangent: (3 x^2 + a) / (2 y)
            let num = f.add(f.scal(3, f.mul(x1, x1)), self.a);
            f.mul(num, f.inv(f.scal(2, y1)))
        } else {
            f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
        };
        let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    fn mul(&self, p: Pt, k: u128) -> Pt {
        let mut r: Pt = None;
        let mut base = p;
        let mut e = k;
        while e > 0 {
            if e & 1 == 1 {
                r = self.add(r, base);
            }
            base = self.add(base, base);
            e >>= 1;
        }
        r
    }
}

// ---------------------------------------------------------------- integers
fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    // operands < 2^64 here
    (a % m) * (b % m) % m
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
/// Deterministic Miller-Rabin for n < 2^64 (bases: first 12 primes).
fn is_prime(n: u128) -> bool {
    assert!(n < (1u128 << 64));
    if n < 2 {
        return false;
    }
    for sp in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % sp == 0 {
            return n == sp;
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

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64 (public-domain constants)
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn parse_f2(f: &Fld, v: &Js) -> Result<F2, String> {
    match v {
        Js::Arr(x) if x.len() == 2 => match (&x[0], &x[1]) {
            (Js::Num(a), Js::Num(b)) => {
                if *a >= f.p || *b >= f.p {
                    return Err("field element not reduced".into());
                }
                Ok(f.c(*a, *b))
            }
            _ => Err("bad F_p^2 element".into()),
        },
        _ => Err("bad F_p^2 element".into()),
    }
}
fn parse_pt(f: &Fld, v: &Js) -> Result<Pt, String> {
    match v {
        Js::Null => Ok(None),
        Js::Arr(x) if x.len() == 2 => Ok(Some((parse_f2(f, &x[0])?, parse_f2(f, &x[1])?))),
        _ => Err("bad point".into()),
    }
}

#[derive(Default)]
struct Tally {
    lines: u64,
    checks: BTreeMap<&'static str, (u64, u64)>, // name -> (ok, total)
    arms: BTreeMap<String, (u64, u64)>,         // arm -> (k_found == k and k_found P = Q, total)
    j_in_fp: u64,
    j_zero_or_1728: u64,
    curves: std::collections::BTreeSet<(u128, u128, F2Key, F2Key)>,
    failures: Vec<String>,
}
type F2Key = (u128, u128);

impl Tally {
    fn rec(&mut self, name: &'static str, ok: bool, ctx: &str) {
        let e = self.checks.entry(name).or_insert((0, 0));
        e.1 += 1;
        if ok {
            e.0 += 1;
        } else if self.failures.len() < 50 {
            self.failures.push(format!("{name} FAILED: {ctx}"));
        }
    }
}

/// Per-curve checks cached by (p, kappa, a, b, P, ell, N).
fn curve_checks(t: &mut Tally, e: &Curve, pt: Pt, ell: u128, n: u128, ctx: &str, seed: u64) {
    let f = &e.f;
    let p = f.p;
    t.rec("p_prime", is_prime(p), ctx);
    t.rec("kappa_nonresidue", f.fpow(f.kappa, (p - 1) / 2) == p - 1, ctx);
    let disc = f.add(f.scal(4, f.mul(f.mul(e.a, e.a), e.a)), f.scal(27, f.mul(e.b, e.b)));
    t.rec("curve_nonsingular", !f.is_zero(disc), ctx);
    // j = 1728 * 4a^3 / (4a^3 + 27 b^2)
    if !f.is_zero(disc) {
        let j = f.mul(f.scal(1728, f.scal(4, f.mul(f.mul(e.a, e.a), e.a))), f.inv(disc));
        if j.im == 0 {
            t.j_in_fp += 1;
        }
        if f.is_zero(j) || j == f.c(1728, 0) {
            t.j_zero_or_1728 += 1;
        }
        t.rec("j_not_in_Fp", j.im != 0, ctx);
    }
    t.rec("P_on_E", e.on(pt), ctx);
    t.rec("P_not_O", pt.is_some(), ctx);
    t.rec("ell_prime", is_prime(ell), ctx);
    t.rec("ell_P_is_O", e.mul(pt, ell).is_none(), ctx);
    t.rec("ell_divides_N", n % ell == 0, ctx);
    let q = p * p + 1;
    let dev = if n > q { n - q } else { q - n };
    t.rec("N_in_Hasse_interval", dev <= 2 * p, ctx);
    // a fixed non-square of F_{p^2}
    let mut rng = Rng(seed ^ 0x7661_6c69_6461_7465);
    let qq = p * p;
    let z = loop {
        let c = f.c(rng.next() as u128 % p, rng.next() as u128 % p);
        if !f.is_zero(c) && f.pow(c, (qq - 1) / 2) != f.one() {
            break c;
        }
    };
    let mut got = 0;
    while got < 4 {
        let x = f.c(rng.next() as u128 % p, rng.next() as u128 % p);
        let r = e.rhs(x);
        if let Some(y) = f.sqrt(r, z) {
            let rp: Pt = Some((x, y));
            t.rec("random_R_on_E", e.on(rp), ctx);
            t.rec("N_R_is_O", e.mul(rp, n).is_none(), ctx);
            got += 1;
        }
    }
}

fn main() {
    let files: Vec<String> = std::env::args().skip(1).collect();
    assert!(!files.is_empty(), "usage: verify_dump <dump.jsonl>...");
    let mut all_ok = true;
    let mut grand = Tally::default();
    for path in &files {
        let mut t = Tally::default();
        let fh = std::fs::File::open(path).expect("open dump");
        let mut cache: std::collections::HashSet<(u128, u128, F2Key, F2Key, String, u128, u128)> =
            std::collections::HashSet::new();
        for (ln, line) in BufReader::new(fh).lines().enumerate() {
            let line = line.expect("read");
            if line.trim().is_empty() {
                continue;
            }
            t.lines += 1;
            let ctx = format!("{path}:{}", ln + 1);
            let res: Result<(), String> = (|| {
                let o = Parser { s: line.as_bytes(), i: 0 }.val()?;
                let p = num(&o, "p")?;
                if p >= (1u128 << 62) || p < 5 {
                    return Err("p out of supported range".into());
                }
                let kappa = num(&o, "kappa")?;
                let f = Fld { p, kappa: kappa % p };
                let a = parse_f2(&f, get(&o, "a")?)?;
                let b = parse_f2(&f, get(&o, "b")?)?;
                let pt = parse_pt(&f, get(&o, "P")?)?;
                let qt = parse_pt(&f, get(&o, "Q")?)?;
                let ell = num(&o, "ell")?;
                let n = num(&o, "N")?;
                let k = num(&o, "k")?;
                let kf = num(&o, "k_found")?;
                let arm = st(&o, "arm")?;
                let e = Curve { f, a, b };
                let key = (p, kappa, (a.re, a.im), (b.re, b.im), format!("{pt:?}"), ell, n);
                t.curves.insert((p, kappa, (a.re, a.im), (b.re, b.im)));
                if !cache.contains(&key) {
                    curve_checks(&mut t, &e, pt, ell, n, &ctx, (p as u64) ^ ((a.re as u64) << 7) ^ (ln as u64));
                    cache.insert(key);
                }
                t.rec("Q_on_E", e.on(qt), &ctx);
                t.rec("k_in_range", k >= 1 && k < ell, &ctx);
                let kp = e.mul(pt, kf % ell.max(1));
                let ok_q = kf < ell && kp == qt;
                t.rec("k_found_P_eq_Q", ok_q, &ctx);
                t.rec("k_found_eq_k", kf == k, &ctx);
                let ae = t.arms.entry(arm).or_insert((0, 0));
                ae.1 += 1;
                if ok_q && kf == k {
                    ae.0 += 1;
                }
                Ok(())
            })();
            if let Err(msg) = res {
                t.rec("line_parses", false, &format!("{ctx}: {msg}"));
            } else {
                t.rec("line_parses", true, &ctx);
            }
        }
        let file_ok = t.checks.values().all(|(ok, tot)| ok == tot) && t.lines > 0;
        all_ok &= file_ok;
        println!("FILE {path}");
        println!("  lines {} distinct_curves {} j_in_Fp {} j_0_or_1728 {}", t.lines, t.curves.len(), t.j_in_fp, t.j_zero_or_1728);
        for (name, (ok, tot)) in &t.checks {
            println!("  check {name}: {ok}/{tot}");
            let g = grand.checks.entry(name).or_insert((0, 0));
            g.0 += ok;
            g.1 += tot;
        }
        for (arm, (ok, tot)) in &t.arms {
            println!("  arm {arm}: {ok}/{tot} answers verified");
            let g = grand.arms.entry(arm.clone()).or_insert((0, 0));
            g.0 += ok;
            g.1 += tot;
        }
        for fl in &t.failures {
            println!("  {fl}");
        }
        println!("  FILE_STATUS {}", if file_ok { "ok" } else { "FAIL" });
        grand.lines += t.lines;
        grand.j_in_fp += t.j_in_fp;
        grand.j_zero_or_1728 += t.j_zero_or_1728;
        grand.curves.extend(t.curves);
    }
    println!("TOTAL lines {} distinct_curves {} j_in_Fp {} j_0_or_1728 {}", grand.lines, grand.curves.len(), grand.j_in_fp, grand.j_zero_or_1728);
    for (name, (ok, tot)) in &grand.checks {
        println!("TOTAL check {name}: {ok}/{tot}");
    }
    for (arm, (ok, tot)) in &grand.arms {
        println!("TOTAL arm {arm}: {ok}/{tot}");
    }
    println!("STATUS {}", if all_ok { "ALL_SCALARS_VERIFIED" } else { "FAILURES" });
    std::process::exit(if all_ok { 0 } else { 1 });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_controls() {
        // tiny curve over F_{11^2}, i^2 = -1 (11 = 3 mod 4 so -1 is a non-residue)
        let f = Fld { p: 11, kappa: 10 };
        assert_eq!(f.fpow(10, 5), 10);
        let e = Curve { f, a: f.c(1, 1), b: f.c(3, 0) };
        let mut rng = Rng(1);
        let z = loop {
            let c = f.c(rng.next() as u128 % 11, rng.next() as u128 % 11);
            if !f.is_zero(c) && f.pow(c, 60) != f.one() {
                break c;
            }
        };
        // brute-force the group order and check N R = O
        let mut n: u128 = 1;
        for x0 in 0..11 {
            for x1 in 0..11 {
                let x = f.c(x0, x1);
                let r = e.rhs(x);
                if f.is_zero(r) {
                    n += 1;
                } else if f.sqrt(r, z).is_some() {
                    n += 2;
                }
            }
        }
        let mut cnt = 0;
        for x0 in 0..11 {
            for x1 in 0..11 {
                let x = f.c(x0, x1);
                if let Some(y) = f.sqrt(e.rhs(x), z) {
                    let p: Pt = Some((x, y));
                    assert!(e.on(p));
                    assert!(e.mul(p, n).is_none());
                    // a wrong order must fail for some point
                    if e.mul(p, n + 1).is_some() {
                        cnt += 1;
                    }
                    // associativity spot check
                    let q = e.add(p, p);
                    assert_eq!(e.add(e.add(p, q), p), e.add(p, e.add(q, p)));
                    assert_eq!(e.mul(p, 5), e.add(q, e.add(q, p)));
                }
            }
        }
        assert!(cnt > 0);
        assert!(is_prime(1_000_000_007));
        assert!(!is_prime(1_000_000_007u128 * 998_244_353));
        assert!(!is_prime(3215031751)); // strong pseudoprime to bases 2,3,5,7
    }
}
