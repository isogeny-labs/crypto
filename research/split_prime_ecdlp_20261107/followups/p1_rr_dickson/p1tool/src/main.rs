//! p1tool: the native instrument of proposal P1 (IDEA-20261107-RR-DICKSON-TORUS).
//!
//! Emits msolve 0.6.5 input files for four polynomial systems on one planted
//! ECDLP decomposition instance and parses msolve's verbose log into counted
//! metrics.
//!
//! Systems (see README.md for the equations):
//! - `rr`: the separator-free Riemann-Roch norm-divisibility system, a port of
//!   `research/ic_tree_gluing_20261103/round7_rr/genrr.sing` (odd m = 2h-1).
//! - `t`:  the cherry-tree chain system, a port of arm T of
//!   `research/ic_tree_gluing_20261103/gen.sing` (m = 2..6).
//! - `f`:  the full-symmetric system, a port of arm F of gen.sing (m = 3 only):
//!   S4 = Res_X(S3(x1,x2,X), S3(X,x3,xR)) in (e1,e2,e3) with the degree-3
//!   remainder chain of the membership tower.
//!
//! Membership towers:
//! - `kummer`:  roots w in mu_{2^t} (needs 2^t | p-1), x = g*w, step y -> y^2,
//!   terminal 1 (i.e. U | X^{2^t} - g^{2^t} in the x coordinate).
//! - `dickson`: roots in V = {w + 1/w : w in u*mu_{2^t}} inside the norm-one
//!   torus of F_{p^2} (needs 2^t | p+1, p = 3 mod 4), x = w (scale 1),
//!   step y -> y^2 - 2 (D_{2k} = D_k^2 - 2), terminal c = D_{2^t}(x).
//!
//! Subcommands: `curve`, `emit`, `parse`, `check-gb`, `selftest`.
//! Every emitted system is evaluated at its planted assignment and the tool
//! aborts if any equation is nonzero.

#![allow(dead_code)]

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::process::exit;

mod collection;
mod extraction;
mod reduction;

// ───────────────────────────── field F_p, p < 2^32 ─────────────────────────

#[derive(Clone, Copy, Debug)]
struct F {
    p: u64,
}

impl F {
    fn new(p: u64) -> F {
        assert!(p > 3 && p < (1u64 << 32), "need 3 < p < 2^32");
        F { p }
    }
    fn red(&self, a: u64) -> u64 {
        a % self.p
    }
    fn from_i(&self, a: i64) -> u64 {
        let p = self.p as i64;
        (((a % p) + p) % p) as u64
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        a %= self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a % self.p != 0, "inverse of 0");
        self.pow(a, self.p - 2)
    }
    fn is_sq(&self, a: u64) -> bool {
        a == 0 || self.pow(a, (self.p - 1) / 2) == 1
    }
    /// Tonelli-Shanks; None for a non-residue.
    fn sqrt(&self, a: u64) -> Option<u64> {
        let p = self.p;
        let a = a % p;
        if a == 0 {
            return Some(0);
        }
        if !self.is_sq(a) {
            return None;
        }
        let (mut q, mut s) = (p - 1, 0u32);
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut z = 2;
        while self.is_sq(z) {
            z += 1;
        }
        let (mut m, mut c, mut t, mut r) = (s, self.pow(z, q), self.pow(a, q), self.pow(a, q.div_ceil(2)));
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = self.mul(tt, tt);
                i += 1;
            }
            let b = self.pow(c, 1u64 << (m - i - 1));
            m = i;
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        debug_assert_eq!(self.mul(r, r), a);
        Some(r)
    }
}

// ───────────────────────────── deterministic PRNG ──────────────────────────

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        // rejection sampling, no modulo bias
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next();
            if v < zone {
                return v % n;
            }
        }
    }
}

// ───────────────────────────── elliptic curve ──────────────────────────────

type Pt = Option<(u64, u64)>; // None = O

#[derive(Clone, Copy)]
struct Curve {
    f: F,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let f = self.f;
        f.add(f.add(f.mul(f.mul(x, x), x), f.mul(self.a, x)), self.b)
    }
    fn on(&self, p: Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => self.f.mul(y, y) == self.rhs(x),
        }
    }
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, self.f.neg(y)))
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        let lam = if x1 == x2 {
            if f.add(y1, y2) == 0 {
                return None;
            }
            let num = f.add(f.mul(3, f.mul(x1, x1)), self.a);
            f.mul(num, f.inv(f.mul(2, y1)))
        } else {
            f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
        };
        let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    fn mul(&self, mut k: u64, p: Pt) -> Pt {
        let mut r: Pt = None;
        let mut b = p;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, b);
            }
            b = self.add(b, b);
            k >>= 1;
        }
        r
    }
    fn random_point(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = rng.below(self.f.p);
            if let Some(y) = self.f.sqrt(self.rhs(x)) {
                if y == 0 {
                    continue;
                }
                let y = if rng.next() & 1 == 1 { self.f.neg(y) } else { y };
                return Some((x, y));
            }
        }
    }
    /// Exact #E(F_p) by the Legendre-symbol sum (small p only).
    fn count_naive(&self) -> u64 {
        let f = self.f;
        let mut n: i64 = 1 + f.p as i64;
        for x in 0..f.p {
            let r = self.rhs(x);
            if r != 0 {
                n += if f.is_sq(r) { 1 } else { -1 };
            }
        }
        n as u64
    }
    /// All k in the Hasse interval with k*P = O (baby-step giant-step).
    fn hasse_annihilators(&self, pt: Pt) -> Vec<u64> {
        let p = self.f.p;
        let s = isqrt(p) + 1; // > sqrt(p)
        let lo = (p + 1).saturating_sub(2 * s);
        let hi = p + 1 + 2 * s;
        let w = hi - lo + 1;
        let m = isqrt(w) + 1;
        let mut baby: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
        let mut cur: Pt = None;
        let mut zero_js = vec![];
        for j in 0..m {
            match cur {
                None => zero_js.push(j),
                Some(v) => baby.entry(v).or_default().push(j),
            }
            cur = self.add(cur, pt);
        }
        // (lo + i*m + j) P = O  <=>  lo P + i (mP) = -(jP)
        let mp = self.mul(m, pt);
        let mut g = self.mul(lo, pt);
        let mut out = vec![];
        let mut i = 0u64;
        while lo + i * m <= hi {
            match g {
                None => {
                    for &j in &zero_js {
                        out.push(lo + i * m + j);
                    }
                }
                Some(v) => {
                    let nv = (v.0, self.f.neg(v.1));
                    if let Some(js) = baby.get(&nv) {
                        for &j in js {
                            out.push(lo + i * m + j);
                        }
                    }
                }
            }
            g = self.add(g, mp);
            i += 1;
        }
        out.retain(|&k| k >= lo && k <= hi);
        out.sort_unstable();
        out.dedup();
        out
    }
}

fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13] {
        if n % q == 0 {
            return n == q;
        }
    }
    let mut d = 17;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

/// Exact prime group order of y^2 = x^3 + ax + b, or Err(reason).
fn prime_order(c: &Curve, rng: &mut Rng) -> Result<u64, String> {
    let f = c.f;
    if c.a == 0 || c.b == 0 {
        return Err("j in {0,1728}".into());
    }
    let disc = f.add(f.mul(4, f.pow(c.a, 3)), f.mul(27, f.mul(c.b, c.b)));
    if disc == 0 {
        return Err("singular".into());
    }
    let p0 = c.random_point(rng);
    let ks = c.hasse_annihilators(p0);
    let primes: Vec<u64> = ks.iter().copied().filter(|&k| is_prime(k)).collect();
    if primes.len() != 1 {
        return Err(format!("no unique prime annihilator ({:?})", ks.len()));
    }
    let n = primes[0];
    // A point of prime order n with n in the Hasse interval forces #E = n when
    // 2n exceeds the interval's top, i.e. for p > 35; small p are counted.
    if f.p < (1 << 22) {
        let nn = c.count_naive();
        if nn != n {
            return Err(format!("naive count {nn} != {n}"));
        }
    } else if 2 * (f.p + 1 - 2 * (isqrt(f.p) + 1)) <= f.p + 1 + 2 * (isqrt(f.p) + 1) {
        return Err("p too small for the Hasse uniqueness argument".into());
    }
    for _ in 0..8 {
        let q = c.random_point(rng);
        if c.mul(n, q).is_some() {
            return Err("n*P != O for a random P".into());
        }
    }
    Ok(n)
}

// ───────────────────────────── F_{p^2} = F_p(i), torus ─────────────────────

type Fp2 = (u64, u64);
fn f2mul(f: F, x: Fp2, y: Fp2) -> Fp2 {
    (f.sub(f.mul(x.0, y.0), f.mul(x.1, y.1)), f.add(f.mul(x.0, y.1), f.mul(x.1, y.0)))
}
fn f2pow(f: F, mut x: Fp2, mut e: u64) -> Fp2 {
    let mut r = (1, 0);
    while e > 0 {
        if e & 1 == 1 {
            r = f2mul(f, r, x);
        }
        x = f2mul(f, x, x);
        e >>= 1;
    }
    r
}
fn f2inv(f: F, x: Fp2) -> Fp2 {
    let n = f.add(f.mul(x.0, x.0), f.mul(x.1, x.1));
    let ni = f.inv(n);
    (f.mul(x.0, ni), f.mul(f.neg(x.1), ni))
}
fn torus_element(f: F, rng: &mut Rng) -> Fp2 {
    loop {
        let z = (rng.below(f.p), rng.below(f.p));
        if z == (0, 0) {
            continue;
        }
        return f2pow(f, z, f.p - 1);
    }
}

// ───────────────────────────── polynomials ─────────────────────────────────

/// Sparse multivariate polynomial over F_p; key = exponent vector.
/// BTreeMap order on Vec<u16> is lex with var 0 largest, so reverse
/// iteration gives Singular's `lp` term order.
#[derive(Clone, Debug, PartialEq)]
struct MP {
    n: usize,
    t: BTreeMap<Vec<u16>, u64>,
}

impl MP {
    fn zero(n: usize) -> MP {
        MP { n, t: BTreeMap::new() }
    }
    fn konst(n: usize, c: u64) -> MP {
        let mut m = MP::zero(n);
        if c != 0 {
            m.t.insert(vec![0; n], c);
        }
        m
    }
    fn var(n: usize, i: usize) -> MP {
        let mut e = vec![0u16; n];
        e[i] = 1;
        let mut m = MP::zero(n);
        m.t.insert(e, 1);
        m
    }
    fn is_zero(&self) -> bool {
        self.t.is_empty()
    }
    fn addterm(&mut self, f: F, e: Vec<u16>, c: u64) {
        if c == 0 {
            return;
        }
        let v = self.t.entry(e.clone()).or_insert(0);
        *v = f.add(*v, c);
        if *v == 0 {
            self.t.remove(&e);
        }
    }
    fn add(&self, f: F, o: &MP) -> MP {
        let mut r = self.clone();
        for (e, &c) in &o.t {
            r.addterm(f, e.clone(), c);
        }
        r
    }
    fn sub(&self, f: F, o: &MP) -> MP {
        let mut r = self.clone();
        for (e, &c) in &o.t {
            r.addterm(f, e.clone(), f.neg(c));
        }
        r
    }
    fn scale(&self, f: F, k: u64) -> MP {
        let mut r = MP::zero(self.n);
        if k % f.p == 0 {
            return r;
        }
        for (e, &c) in &self.t {
            r.t.insert(e.clone(), f.mul(c, k));
        }
        r
    }
    fn mul(&self, f: F, o: &MP) -> MP {
        let mut r = MP::zero(self.n);
        for (e1, &c1) in &self.t {
            for (e2, &c2) in &o.t {
                let e: Vec<u16> = e1.iter().zip(e2).map(|(a, b)| a + b).collect();
                r.addterm(f, e, f.mul(c1, c2));
            }
        }
        r
    }
    fn deg(&self) -> u32 {
        self.t.keys().map(|e| e.iter().map(|&x| x as u32).sum()).max().unwrap_or(0)
    }
    fn eval(&self, f: F, v: &[u64]) -> u64 {
        let mut s = 0;
        for (e, &c) in &self.t {
            let mut term = c;
            for (i, &k) in e.iter().enumerate() {
                if k > 0 {
                    term = f.mul(term, f.pow(v[i], k as u64));
                }
            }
            s = f.add(s, term);
        }
        s
    }
    /// Singular `fmtpoly` text (lp order, coefficients in 0..p-1, `c*mono`).
    fn fmt(&self, names: &[String]) -> String {
        if self.t.is_empty() {
            return "0".into();
        }
        let mut parts = vec![];
        for (e, &c) in self.t.iter().rev() {
            let mono: Vec<String> = e
                .iter()
                .enumerate()
                .filter(|(_, &k)| k > 0)
                .map(|(i, &k)| if k > 1 { format!("{}^{}", names[i], k) } else { names[i].clone() })
                .collect();
            if mono.is_empty() {
                parts.push(format!("{c}"));
            } else {
                parts.push(format!("{c}*{}", mono.join("*")));
            }
        }
        parts.join("+")
    }
}

/// Polynomial in one auxiliary variable Z with MP coefficients; index = Z-degree.
type ZP = Vec<MP>;

fn ztrim(mut a: ZP) -> ZP {
    while a.last().is_some_and(|c| c.is_zero()) {
        a.pop();
    }
    a
}
fn zadd(f: F, n: usize, a: &ZP, b: &ZP) -> ZP {
    let l = a.len().max(b.len());
    ztrim(
        (0..l)
            .map(|i| match (a.get(i), b.get(i)) {
                (Some(x), Some(y)) => x.add(f, y),
                (Some(x), None) => x.clone(),
                (None, Some(y)) => y.clone(),
                _ => MP::zero(n),
            })
            .collect(),
    )
}
fn zneg(f: F, a: &ZP) -> ZP {
    a.iter().map(|c| c.scale(f, f.p - 1)).collect()
}
fn zsub(f: F, n: usize, a: &ZP, b: &ZP) -> ZP {
    zadd(f, n, a, &zneg(f, b))
}
fn zmul(f: F, n: usize, a: &ZP, b: &ZP) -> ZP {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![MP::zero(n); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        if x.is_zero() {
            continue;
        }
        for (j, y) in b.iter().enumerate() {
            if y.is_zero() {
                continue;
            }
            r[i + j] = r[i + j].add(f, &x.mul(f, y));
        }
    }
    ztrim(r)
}
fn zconst(n: usize, c: u64) -> ZP {
    ztrim(vec![MP::konst(n, c)])
}
fn zmono(n: usize, k: usize) -> ZP {
    let mut v = vec![MP::zero(n); k + 1];
    v[k] = MP::konst(n, 1);
    v
}
/// Evaluate Z at a field element: an MP.
fn zeval(f: F, n: usize, a: &ZP, z: u64) -> MP {
    let mut r = MP::zero(n);
    let mut zk = 1;
    for c in a {
        r = r.add(f, &c.scale(f, zk));
        zk = f.mul(zk, z);
    }
    r
}

// numeric univariate polynomials, index = degree
type UP = Vec<u64>;
fn utrim(mut a: UP) -> UP {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn umul(f: F, a: &UP, b: &UP) -> UP {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = f.add(r[i + j], f.mul(x, y));
        }
    }
    utrim(r)
}
fn usubc(f: F, a: &UP, c: u64) -> UP {
    let mut r = a.clone();
    if r.is_empty() {
        r.push(0);
    }
    r[0] = f.sub(r[0], c);
    utrim(r)
}
/// (q, r) with a = q*b + r, b monic.
fn udivrem(f: F, a: &UP, b: &UP) -> (UP, UP) {
    let db = b.len() - 1;
    assert_eq!(b[db], 1);
    let mut r = a.clone();
    if r.len() <= db {
        return (vec![], utrim(r));
    }
    let mut q = vec![0; r.len() - db];
    for k in (db..r.len()).rev() {
        let c = r[k];
        if c == 0 {
            continue;
        }
        q[k - db] = c;
        for (i, &bi) in b.iter().enumerate() {
            r[k - db + i] = f.sub(r[k - db + i], f.mul(c, bi));
        }
    }
    r.truncate(db);
    (utrim(q), utrim(r))
}
fn ueval(f: F, a: &UP, x: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &c| f.add(f.mul(acc, x), c))
}

// ───────────────────────────── instances ───────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Kummer,
    Dickson,
}
impl Kind {
    fn name(&self) -> &'static str {
        match self {
            Kind::Kummer => "kummer",
            Kind::Dickson => "dickson",
        }
    }
    /// One tower step in the tower coordinate.
    fn step(&self, f: F, y: u64) -> u64 {
        match self {
            Kind::Kummer => f.mul(y, y),
            Kind::Dickson => f.sub(f.mul(y, y), 2),
        }
    }
    /// The step's constant shift: y^2 - shift.
    fn shift(&self) -> u64 {
        match self {
            Kind::Kummer => 0,
            Kind::Dickson => 2,
        }
    }
}

/// A planted decomposition instance R = P_1 + ... + P_m with every x(P_i) in
/// the base set {x = scale*w : tower_t(w) = c}.
#[derive(Clone, Debug)]
struct Inst {
    p: u64,
    a: u64,
    b: u64,
    kind: Kind,
    m: usize,
    t: usize,
    scale: u64,  // g (Kummer) or 1 (Dickson)
    c: u64,      // terminal value of the tower in the tower coordinate
    w: Vec<u64>, // tower coordinates of the summand abscissae
    x: Vec<u64>, // x_i = scale * w_i
    y: Vec<u64>, // planted signs: P_i = (x_i, y_i)
    xr: u64,
    yr: u64,
    tree: Vec<u64>,  // T arm separators: cherry sums X1.. (m div 2 of them)
    yval: Option<u64>, // T arm: x(P1+..+P4) for m >= 5
    info: Value,
}

fn tower_value(kind: Kind, f: F, w: u64, t: usize) -> u64 {
    let mut y = w;
    for _ in 0..t {
        y = kind.step(f, y);
    }
    y
}

/// Base abscissae in the tower coordinate: (W, c, scale, info).
fn base_set(kind: Kind, f: F, t: usize, gopt: Option<u64>, rng: &mut Rng) -> (Vec<u64>, u64, u64, Value) {
    let p = f.p;
    match kind {
        Kind::Kummer => {
            assert!((p - 1) % (1u64 << t) == 0, "kummer needs 2^t | p-1 (t={t}, p={p})");
            let zeta = loop {
                let h = 2 + rng.below(p - 3);
                if !f.is_sq(h) {
                    break f.pow(h, (p - 1) >> t);
                }
            };
            assert_eq!(f.pow(zeta, 1 << t), 1);
            if t > 0 {
                assert_eq!(f.pow(zeta, 1 << (t - 1)), p - 1, "zeta order");
            }
            let mut v = vec![];
            let mut w = 1;
            for _ in 0..(1u64 << t) {
                v.push(w);
                w = f.mul(w, zeta);
            }
            let g = gopt.unwrap_or_else(|| 1 + rng.below(p - 1));
            (v, 1, g, json!({"zeta": zeta, "g": g}))
        }
        Kind::Dickson => {
            assert!(p % 4 == 3 && (p + 1) % (1u64 << t) == 0, "dickson torus needs p = 3 mod 4 and 2^t | p+1");
            let zeta = loop {
                let h = torus_element(f, rng);
                if f2pow(f, h, p.div_ceil(2)) == (p - 1, 0) {
                    break f2pow(f, h, (p + 1) >> t);
                }
            };
            loop {
                let u = torus_element(f, rng);
                if u.1 == 0 || f2pow(f, u, 1u64 << (t + 1)) == (1, 0) {
                    continue;
                }
                let mut v = vec![];
                let mut w = u;
                for _ in 0..(1u64 << t) {
                    v.push(f.add(w.0, w.0));
                    w = f2mul(f, w, zeta);
                }
                let mut s = v.clone();
                s.sort_unstable();
                s.dedup();
                if s.len() != 1 << t {
                    continue;
                }
                let ut = f2pow(f, u, 1u64 << t);
                let c = f.add(ut.0, ut.0);
                if c == 2 || c == p - 2 {
                    continue;
                }
                check_torus(f, t, &v, u, zeta, c);
                return (v, c, 1, json!({"case": "torus", "u": [u.0, u.1], "zeta": [zeta.0, zeta.1], "c": c}));
            }
        }
    }
}

/// The torus condition (pkm_dickson_torus PROTOCOL.md section 2): panics on failure.
fn check_torus(f: F, t: usize, v: &[u64], u: Fp2, zeta: Fp2, c: u64) {
    let p = f.p;
    assert_eq!(p % 4, 3);
    assert_eq!((p + 1) % (1u64 << t), 0);
    assert_eq!(f2pow(f, zeta, 1u64 << t), (1, 0), "zeta^(2^t) != 1");
    if t > 0 {
        assert_eq!(f2pow(f, zeta, 1u64 << (t - 1)), (p - 1, 0), "zeta order");
    }
    assert_eq!(f2pow(f, u, p + 1), (1, 0), "u not in torus");
    assert_ne!(u.1, 0, "u in F_p");
    let mut w = u;
    for (k, &x) in v.iter().enumerate() {
        let wi = f2inv(f, w);
        let s = (f.add(w.0, wi.0), f.add(w.1, wi.1));
        assert_eq!(s.1, 0, "w+1/w not in F_p at {k}");
        assert_eq!(s.0, x, "V[{k}] != w+1/w");
        let disc = f.sub(f.mul(x, x), 4);
        assert_eq!(f.pow(disc, (p - 1) / 2), p - 1, "y^2-4 not a non-residue at V[{k}]");
        let mut y = x;
        for _ in 0..t {
            assert_ne!(y, 0, "ramified level at V[{k}]");
            y = Kind::Dickson.step(f, y);
        }
        assert_eq!(y, c, "D_(2^t)(V[{k}]) != c");
        w = f2mul(f, w, zeta);
    }
    assert!(c != 2 && c != p - 2);
}

/// Number of sign patterns of (x_i, +-y_i) whose sum is exactly `r`.
fn sign_patterns(cv: &Curve, xs: &[u64], ys: &[u64], r: Pt) -> (usize, Vec<u64>) {
    let m = xs.len();
    let mut n = 0;
    let mut sg = vec![];
    for pat in 0..(1u32 << m) {
        let mut s: Pt = None;
        let mut cur = vec![];
        for i in 0..m {
            let yi = if (pat >> i) & 1 == 1 { cv.f.neg(ys[i]) } else { ys[i] };
            cur.push(yi);
            s = cv.add(s, Some((xs[i], yi)));
        }
        if s == r {
            n += 1;
            sg = cur;
        }
    }
    (n, sg)
}

fn tree_values(cv: &Curve, inst_pts: &[Pt], m: usize) -> Option<(Vec<u64>, Option<u64>)> {
    let mut tree = vec![];
    if m >= 3 {
        for c in 0..m / 2 {
            let s = cv.add(inst_pts[2 * c], inst_pts[2 * c + 1]);
            tree.push(s?.0);
        }
    }
    let mut yv = None;
    if m >= 5 {
        let s = cv.add(cv.add(inst_pts[0], inst_pts[1]), cv.add(inst_pts[2], inst_pts[3]));
        yv = Some(s?.0);
    }
    Some((tree, yv))
}

/// Sample a planted instance on (p, a, b) for kind/m/t from `seed`.
/// The same seed gives the same instance for both systems (RR and T): every
/// condition of both is enforced here.
fn sample_instance(p: u64, a: u64, b: u64, kind: Kind, m: usize, t: usize, gopt: Option<u64>, seed: u64) -> Inst {
    let f = F::new(p);
    let cv = Curve { f, a, b };
    let mut rng = Rng(seed ^ 0x5031_5252_4449_434B);
    let (v, c, g, info) = base_set(kind, f, t, gopt, &mut rng);
    // base points: x = g*w with f(x) a nonzero square
    let base: Vec<(u64, u64)> = v
        .iter()
        .filter_map(|&w| {
            let x = f.mul(g, w);
            let r = cv.rhs(x);
            if r != 0 && f.is_sq(r) {
                Some((w, x))
            } else {
                None
            }
        })
        .collect();
    assert!(base.len() >= m + 1, "base set too small: {} points for m={m}", base.len());
    for _attempt in 0..100_000 {
        let mut idx = vec![];
        while idx.len() < m {
            let k = rng.below(base.len() as u64) as usize;
            if !idx.contains(&k) {
                idx.push(k);
            }
        }
        let ws: Vec<u64> = idx.iter().map(|&k| base[k].0).collect();
        let xs: Vec<u64> = idx.iter().map(|&k| base[k].1).collect();
        let ys: Vec<u64> = xs
            .iter()
            .map(|&x| {
                let y = f.sqrt(cv.rhs(x)).unwrap();
                if rng.next() & 1 == 1 {
                    f.neg(y)
                } else {
                    y
                }
            })
            .collect();
        let pts: Vec<Pt> = xs.iter().zip(&ys).map(|(&x, &y)| Some((x, y))).collect();
        let r = pts.iter().fold(None, |acc, &q| cv.add(acc, q));
        let (xr, yr) = match r {
            None => continue,
            Some(v) => v,
        };
        if xs.contains(&xr) || yr == 0 {
            continue;
        }
        // RR: exactly one signed decomposition of R on these abscissae
        let (np, _) = sign_patterns(&cv, &xs, &ys, r);
        if np != 1 {
            continue;
        }
        // T: separators defined and distinct from the summands they meet
        let (tree, yval) = match tree_values(&cv, &pts, m) {
            None => continue,
            Some(v) => v,
        };
        if m == 3 && (tree[0] == xs[2]) {
            continue;
        }
        if m == 4 && tree[0] == tree[1] {
            continue;
        }
        if m >= 5 && (tree[0] == tree[1] || (m == 5 && yval == Some(xs[4])) || (m == 6 && yval == Some(tree[2]))) {
            continue;
        }
        if m == 2 && cv.add(pts[0], pts[1]).is_none() {
            continue;
        }
        let inst = Inst {
            p,
            a,
            b,
            kind,
            m,
            t,
            scale: g,
            c,
            w: ws,
            x: xs,
            y: ys,
            xr,
            yr,
            tree,
            yval,
            info: json!({"seed": seed, "base_points": base.len(), "tower": info}),
        };
        verify_instance(&inst);
        return inst;
    }
    panic!("no admissible planted instance found");
}

/// Independent re-check of an instance: points on the curve, x = scale*w,
/// tower_t(w) = c, sum = R.
fn verify_instance(i: &Inst) {
    let f = F::new(i.p);
    let cv = Curve { f, a: i.a, b: i.b };
    let mut s: Pt = None;
    for k in 0..i.m {
        assert_eq!(f.mul(i.scale, i.w[k]), i.x[k], "x != g*w");
        assert_eq!(tower_value(i.kind, f, i.w[k], i.t), i.c, "tower_t(w) != c");
        assert!(cv.on(Some((i.x[k], i.y[k]))), "summand not on curve");
        s = cv.add(s, Some((i.x[k], i.y[k])));
    }
    assert!(cv.on(Some((i.xr, i.yr))));
    assert_eq!(s, Some((i.xr, i.yr)), "planted sum != R");
}

/// Round-7 instance from `instances.txt` format (Kummer, x = g*w, mu_{2^t}).
/// y_R is the square root whose symmetric representative is negative, which
/// is the root genrr.sing's `sqr` returns on all six round-7 cells.
fn round7_instance(path: &str, m: usize, t: usize, k: usize) -> Inst {
    let txt = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let recs: Vec<Vec<u64>> = txt
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    let cvl = &recs[0];
    let (p, a, b, g) = (cvl[0], cvl[1], cvl[2], cvl[3]);
    let rec = recs[1..]
        .iter()
        .find(|r| r[0] as usize == m && r[1] as usize == t && r[2] as usize == k)
        .unwrap_or_else(|| die(&format!("no record m={m} t={t} k={k} in {path}")))
        .clone();
    let f = F::new(p);
    let cv = Curve { f, a, b };
    let xr = rec[3];
    let ws: Vec<u64> = rec[4..4 + m].to_vec();
    let xs: Vec<u64> = ws.iter().map(|&w| f.mul(g, w)).collect();
    let nt = if m >= 3 { m / 2 } else { 0 };
    let tree: Vec<u64> = rec[4 + m + (m.saturating_sub(2))..4 + m + m.saturating_sub(2) + nt].to_vec();
    let yval = if m >= 5 { Some(rec[4 + m + (m - 2) + nt]) } else { None };
    let r0 = f.sqrt(cv.rhs(xr)).unwrap_or_else(|| die("x_R not on curve"));
    let yr = if r0 > p / 2 { r0 } else { f.neg(r0) };
    let y0: Vec<u64> = xs.iter().map(|&x| f.sqrt(cv.rhs(x)).unwrap_or_else(|| die("summand not on curve"))).collect();
    let (np, sg) = sign_patterns(&cv, &xs, &y0, Some((xr, yr)));
    let inst = Inst {
        p,
        a,
        b,
        kind: Kind::Kummer,
        m,
        t,
        scale: g,
        c: 1,
        w: ws,
        x: xs,
        y: if np == 1 { sg } else { y0 },
        xr,
        yr,
        tree,
        yval,
        info: json!({"round7": path, "k": k, "sign_patterns": np}),
    };
    if np != 1 {
        // the T arm (x-only) does not need the signs; the RR arm aborts below
        eprintln!("warning: {np} sign patterns match (x_R, y_R)");
    } else {
        verify_instance(&inst);
    }
    inst
}

// ───────────────────────────── systems ─────────────────────────────────────

struct System {
    names: Vec<String>,
    eqs: Vec<MP>,
    plant: Vec<u64>,
}

fn names_index(names: &[String], s: &str) -> usize {
    names.iter().position(|x| x == s).unwrap_or_else(|| die(&format!("unknown variable {s}")))
}

/// Coefficients of a Z-polynomial from the top degree down, nonzero only
/// (Singular `coeffs(df, Z)` read from the last row up).
fn push_coeffs(eqs: &mut Vec<MP>, df: &ZP) {
    for c in df.iter().rev() {
        if !c.is_zero() {
            eqs.push(c.clone());
        }
    }
}

/// RR system (genrr.sing, sat = 0) for odd m.
fn build_rr(i: &Inst) -> System {
    let f = F::new(i.p);
    let m = i.m;
    let t = i.t;
    if m % 2 == 0 {
        die("rr needs odd m = 2h-1");
    }
    let h = m.div_ceil(2);
    let mut nm: Vec<String> = vec![];
    for k in 0..h {
        nm.push(format!("a{k}"));
    }
    for k in 0..h.saturating_sub(1) {
        nm.push(format!("c{k}"));
    }
    for k in 0..m {
        nm.push(format!("u{k}"));
    }
    // tower level layout
    let mut qd = vec![0usize; t + 1];
    let mut hasr = vec![false; t + 1];
    let mut known = true;
    let mut degprev = 1usize;
    for j in 1..=t {
        if known && 2 * degprev < m && j < t {
            qd[j] = 0;
            degprev *= 2;
        } else {
            let dq = if known { 2 * degprev - m } else { m - 2 };
            qd[j] = dq + 1;
            for k in 0..=dq {
                nm.push(format!("q{j}_{k}"));
            }
            if j < t {
                hasr[j] = true;
                for k in 0..m {
                    nm.push(format!("r{j}_{k}"));
                }
            }
            known = false;
        }
    }
    let n = nm.len();
    let v = |s: &str| MP::var(n, names_index(&nm, s));
    let mut ap: ZP = zmono(n, h);
    for k in 0..h {
        ap[k] = ap[k].add(f, &v(&format!("a{k}")));
    }
    let mut cp: ZP = vec![MP::zero(n); h.saturating_sub(1).max(1)];
    for k in 0..h.saturating_sub(1) {
        cp[k] = v(&format!("c{k}"));
    }
    let cp = ztrim(cp);
    let mut uw: ZP = zmono(n, m);
    let mut ux: ZP = zmono(n, m);
    for k in 0..m {
        uw[k] = v(&format!("u{k}"));
        ux[k] = v(&format!("u{k}")).scale(f, f.pow(i.scale, (m - k) as u64));
    }
    let fp: ZP = vec![MP::konst(n, i.b), MP::konst(n, i.a), MP::zero(n), MP::konst(n, 1)];
    let mut eqs = vec![];
    let zmxr: ZP = vec![MP::konst(n, f.neg(i.xr)), MP::konst(n, 1)];
    let df = zsub(f, n, &zsub(f, n, &zmul(f, n, &ap, &ap), &zmul(f, n, &fp, &zmul(f, n, &cp, &cp))), &zmul(f, n, &zmxr, &ux));
    push_coeffs(&mut eqs, &df);
    let tgt = zeval(f, n, &ap, i.xr).sub(f, &zeval(f, n, &cp, i.xr).scale(f, i.yr));
    eqs.push(tgt);
    let shift = zconst(n, i.kind.shift());
    let mut vp: ZP = zmono(n, 1);
    for j in 1..=t {
        let sq = zsub(f, n, &zmul(f, n, &vp, &vp), &shift);
        if qd[j] == 0 {
            vp = sq;
            continue;
        }
        let mut qp: ZP = vec![MP::zero(n); qd[j]];
        for k in 0..qd[j] {
            qp[k] = v(&format!("q{j}_{k}"));
        }
        let vj: ZP = if hasr[j] {
            (0..m).map(|k| v(&format!("r{j}_{k}"))).collect()
        } else {
            zconst(n, i.c)
        };
        let df = zsub(f, n, &zsub(f, n, &sq, &zmul(f, n, &qp, &uw)), &vj);
        push_coeffs(&mut eqs, &df);
        vp = ztrim(vj);
    }
    // ── planted assignment
    let mut val = vec![u64::MAX; n];
    // A, C by interpolation: A(x_i) + y_i C(x_i) = 0, A(x_R) - y_R C(x_R) = 0
    let nunk = h + h - 1;
    let mut rows: Vec<Vec<u64>> = vec![];
    let mut pts: Vec<(u64, u64)> = i.x.iter().zip(&i.y).map(|(&x, &y)| (x, y)).collect();
    pts.push((i.xr, f.neg(i.yr)));
    for &(x, y) in &pts {
        let mut row = vec![0u64; nunk + 1];
        for k in 0..h {
            row[k] = f.pow(x, k as u64);
        }
        for k in 0..h - 1 {
            row[h + k] = f.mul(y, f.pow(x, k as u64));
        }
        row[nunk] = f.neg(f.pow(x, h as u64)); // rhs
        rows.push(row);
    }
    let sol = solve_linear(f, rows, nunk).unwrap_or_else(|| die("A,C interpolation not unique or inconsistent"));
    for k in 0..h {
        val[names_index(&nm, &format!("a{k}"))] = sol[k];
    }
    for k in 0..h - 1 {
        val[names_index(&nm, &format!("c{k}"))] = sol[h + k];
    }
    let mut upl: UP = vec![1];
    for &w in &i.w {
        upl = umul(f, &upl, &vec![f.neg(w), 1]);
    }
    for k in 0..m {
        val[names_index(&nm, &format!("u{k}"))] = upl[k];
    }
    let mut vpn: UP = vec![0, 1];
    for j in 1..=t {
        let sq = usubc(f, &umul(f, &vpn, &vpn), i.kind.shift());
        if qd[j] == 0 {
            vpn = sq;
            continue;
        }
        let (q, r) = udivrem(f, &sq, &upl);
        for k in 0..qd[j] {
            val[names_index(&nm, &format!("q{j}_{k}"))] = *q.get(k).unwrap_or(&0);
        }
        if q.len() > qd[j] {
            die("tower quotient too long");
        }
        if hasr[j] {
            for k in 0..m {
                val[names_index(&nm, &format!("r{j}_{k}"))] = *r.get(k).unwrap_or(&0);
            }
        } else if utrim(r.clone()) != utrim(vec![i.c]) {
            die("planted U does not divide the terminal tower polynomial");
        }
        vpn = r;
    }
    assert!(val.iter().all(|&x| x != u64::MAX), "planted assignment incomplete");
    // norm identity as a polynomial identity at the planted A, C, U
    let pa: UP = (0..=h).map(|k| if k == h { 1 } else { val[names_index(&nm, &format!("a{k}"))] }).collect();
    let pc: UP = utrim((0..h - 1).map(|k| val[names_index(&nm, &format!("c{k}"))]).collect());
    let pux: UP = (0..=m).map(|k| f.mul(upl[k], f.pow(i.scale, (m - k) as u64))).collect();
    let fpn: UP = vec![i.b, i.a, 0, 1];
    let lhs = umul(f, &pa, &pa);
    let r1 = umul(f, &fpn, &umul(f, &pc, &pc));
    let r2 = umul(f, &vec![f.neg(i.xr), 1], &pux);
    let l = lhs.len().max(r1.len()).max(r2.len());
    for k in 0..l {
        let s = f.sub(f.sub(*lhs.get(k).unwrap_or(&0), *r1.get(k).unwrap_or(&0)), *r2.get(k).unwrap_or(&0));
        if s != 0 {
            die("norm identity fails at planted A, C, U");
        }
    }
    System { names: nm, eqs, plant: val }
}

fn solve_linear(f: F, mut rows: Vec<Vec<u64>>, n: usize) -> Option<Vec<u64>> {
    let mut piv_row = 0;
    let mut pivcols = vec![];
    for col in 0..n {
        let Some(r) = (piv_row..rows.len()).find(|&r| rows[r][col] != 0) else {
            return None;
        };
        rows.swap(piv_row, r);
        let inv = f.inv(rows[piv_row][col]);
        for k in 0..=n {
            rows[piv_row][k] = f.mul(rows[piv_row][k], inv);
        }
        for r2 in 0..rows.len() {
            if r2 != piv_row && rows[r2][col] != 0 {
                let c = rows[r2][col];
                for k in 0..=n {
                    let s = f.mul(c, rows[piv_row][k]);
                    rows[r2][k] = f.sub(rows[r2][k], s);
                }
            }
        }
        pivcols.push(col);
        piv_row += 1;
    }
    for r in piv_row..rows.len() {
        if rows[r][n] != 0 {
            return None; // inconsistent
        }
    }
    Some((0..n).map(|k| rows[k][n]).collect())
}

fn s3(f: F, n: usize, x1: &MP, x2: &MP, x3: &MP, a: u64, b: u64) -> MP {
    let ka = MP::konst(n, a);
    let d = x1.sub(f, x2);
    let t1 = d.mul(f, &d).mul(f, x3).mul(f, x3);
    let sum = x1.add(f, x2);
    let prod = x1.mul(f, x2);
    let t2 = sum.mul(f, &prod.add(f, &ka)).add(f, &MP::konst(n, f.mul(2, b))).mul(f, x3).scale(f, 2);
    let pa = prod.sub(f, &ka);
    let t3 = pa.mul(f, &pa).sub(f, &sum.scale(f, f.mul(4, b)));
    t1.sub(f, &t2).add(f, &t3)
}

fn s3sym(f: F, n: usize, s: &MP, q: &MP, x3: &MP, a: u64, b: u64) -> MP {
    let ka = MP::konst(n, a);
    let t1 = s.mul(f, s).sub(f, &q.scale(f, 4)).mul(f, x3).mul(f, x3);
    let t2 = s.mul(f, &q.add(f, &ka)).add(f, &MP::konst(n, f.mul(2, b))).mul(f, x3).scale(f, 2);
    let qa = q.sub(f, &ka);
    let t3 = qa.mul(f, &qa).sub(f, &s.scale(f, f.mul(4, b)));
    t1.sub(f, &t2).add(f, &t3)
}

/// Arm T of gen.sing (cherry tree), with the Kummer or Dickson tower.
fn build_t(i: &Inst) -> System {
    let f = F::new(i.p);
    let (m, t) = (i.m, i.t);
    if !(2..=6).contains(&m) {
        die("t system supports m = 2..6 (gen.sing shapes)");
    }
    let mut nm: Vec<String> = vec![];
    let mn = |nme: &str, nm: &mut Vec<String>| {
        for j in 1..t {
            // 2^j >= k = 2 for every j >= 1
            for k in 0..2 {
                nm.push(format!("{nme}r{j}_{k}"));
            }
        }
    };
    let kn = |nme: &str, nm: &mut Vec<String>| {
        for j in 1..t {
            nm.push(format!("{nme}u{j}"));
        }
    };
    for c in 0..m / 2 {
        nm.push(format!("s{c}"));
        nm.push(format!("q{c}"));
        mn(&format!("h{c}"), &mut nm);
    }
    match m {
        3 => {
            nm.push("w3".into());
            kn("c2", &mut nm);
            nm.push("X1".into());
        }
        4 => {
            nm.push("X1".into());
            nm.push("X2".into());
        }
        5 => {
            nm.push("w5".into());
            kn("c4", &mut nm);
            for s in ["X1", "X2", "Y"] {
                nm.push(s.into());
            }
        }
        6 => {
            for s in ["X1", "X2", "X3", "Y"] {
                nm.push(s.into());
            }
        }
        _ => {}
    }
    let n = nm.len();
    let v = |s: &str| MP::var(n, names_index(&nm, s));
    let shift = i.kind.shift();
    let mut eqs: Vec<MP> = vec![];
    // remainder chain T^(2^j) (or the Dickson iterate) mod T^2 - s T + q
    let mchain = |c: usize, eqs: &mut Vec<MP>| {
        let s = v(&format!("s{c}"));
        let q = v(&format!("q{c}"));
        let mut r: ZP = zmono(n, 1);
        for j in 1..=t {
            let mut sq = zmul(f, n, &r, &r);
            sq = zsub(f, n, &sq, &zconst(n, shift));
            // reduce T^2 -> s T - q (degree of sq is <= 2)
            let mut red: ZP = vec![MP::zero(n), MP::zero(n)];
            for (d, cf) in sq.iter().enumerate() {
                match d {
                    0 => red[0] = red[0].add(f, cf),
                    1 => red[1] = red[1].add(f, cf),
                    2 => {
                        red[1] = red[1].add(f, &cf.mul(f, &s));
                        red[0] = red[0].sub(f, &cf.mul(f, &q));
                    }
                    _ => die("remainder degree"),
                }
            }
            let red = ztrim(red);
            let rsym: ZP = if j == t {
                zconst(n, i.c)
            } else {
                vec![v(&format!("h{c}r{j}_0")), v(&format!("h{c}r{j}_1"))]
            };
            push_coeffs(eqs, &zsub(f, n, &red, &rsym));
            r = rsym;
        }
    };
    let kchain = |nme: &str, w: MP, eqs: &mut Vec<MP>| {
        let mut prev = w;
        for j in 1..t {
            let u = v(&format!("{nme}u{j}"));
            eqs.push(u.sub(f, &prev.mul(f, &prev).sub(f, &MP::konst(n, shift))));
            prev = u;
        }
        eqs.push(prev.mul(f, &prev).sub(f, &MP::konst(n, shift)).sub(f, &MP::konst(n, i.c)));
    };
    for c in 0..m / 2 {
        mchain(c, &mut eqs);
    }
    let g = i.scale;
    let g2 = f.mul(g, g);
    let (a, b) = (i.a, i.b);
    let sg = |c: usize| v(&format!("s{c}")).scale(f, g);
    let qg = |c: usize| v(&format!("q{c}")).scale(f, g2);
    let xr = MP::konst(n, i.xr);
    match m {
        2 => eqs.push(s3sym(f, n, &sg(0), &qg(0), &xr, a, b)),
        3 => {
            kchain("c2", v("w3"), &mut eqs);
            eqs.push(s3sym(f, n, &sg(0), &qg(0), &v("X1"), a, b));
            eqs.push(s3(f, n, &v("X1"), &v("w3").scale(f, g), &xr, a, b));
        }
        4 => {
            eqs.push(s3sym(f, n, &sg(0), &qg(0), &v("X1"), a, b));
            eqs.push(s3sym(f, n, &sg(1), &qg(1), &v("X2"), a, b));
            eqs.push(s3(f, n, &v("X1"), &v("X2"), &xr, a, b));
        }
        5 => {
            kchain("c4", v("w5"), &mut eqs);
            eqs.push(s3sym(f, n, &sg(0), &qg(0), &v("X1"), a, b));
            eqs.push(s3sym(f, n, &sg(1), &qg(1), &v("X2"), a, b));
            eqs.push(s3(f, n, &v("X1"), &v("X2"), &v("Y"), a, b));
            eqs.push(s3(f, n, &v("Y"), &v("w5").scale(f, g), &xr, a, b));
        }
        6 => {
            for c in 0..3 {
                eqs.push(s3sym(f, n, &sg(c), &qg(c), &v(&format!("X{}", c + 1)), a, b));
            }
            eqs.push(s3(f, n, &v("X1"), &v("X2"), &v("Y"), a, b));
            eqs.push(s3(f, n, &v("Y"), &v("X3"), &xr, a, b));
        }
        _ => unreachable!(),
    }
    // ── planted assignment
    let mut val = vec![u64::MAX; n];
    for c in 0..m / 2 {
        let (w1, w2) = (i.w[2 * c], i.w[2 * c + 1]);
        val[names_index(&nm, &format!("s{c}"))] = f.add(w1, w2);
        val[names_index(&nm, &format!("q{c}"))] = f.mul(w1, w2);
        let qn: UP = vec![f.mul(w1, w2), f.neg(f.add(w1, w2)), 1];
        let mut r: UP = vec![0, 1];
        for j in 1..t {
            let sq = usubc(f, &umul(f, &r, &r), shift);
            r = udivrem(f, &sq, &qn).1;
            val[names_index(&nm, &format!("h{c}r{j}_0"))] = *r.first().unwrap_or(&0);
            val[names_index(&nm, &format!("h{c}r{j}_1"))] = *r.get(1).unwrap_or(&0);
        }
    }
    let plant_k = |nme: &str, w: u64, val: &mut Vec<u64>| {
        let mut y = w;
        for j in 1..t {
            y = i.kind.step(f, y);
            val[names_index(&nm, &format!("{nme}u{j}"))] = y;
        }
    };
    match m {
        3 => {
            val[names_index(&nm, "w3")] = i.w[2];
            plant_k("c2", i.w[2], &mut val);
            val[names_index(&nm, "X1")] = i.tree[0];
        }
        4 => {
            val[names_index(&nm, "X1")] = i.tree[0];
            val[names_index(&nm, "X2")] = i.tree[1];
        }
        5 | 6 => {
            for c in 0..m / 2 {
                val[names_index(&nm, &format!("X{}", c + 1))] = i.tree[c];
            }
            val[names_index(&nm, "Y")] = i.yval.unwrap();
            if m == 5 {
                val[names_index(&nm, "w5")] = i.w[4];
                plant_k("c4", i.w[4], &mut val);
            }
        }
        _ => {}
    }
    assert!(val.iter().all(|&x| x != u64::MAX), "planted assignment incomplete");
    System { names: nm, eqs, plant: val }
}

/// S4(x1,x2,x3; x_R) = Res_X(S3(x1,x2,X), S3(X,x3,x_R)) rewritten in the
/// elementary symmetric functions E1, E2, E3 of (x1,x2,x3), as gen.sing's
/// "S4 (arm F)" section computes it (x_R numeric here; gen.sing keeps it
/// symbolic and substitutes x_R afterwards, which gives the same polynomial).
/// Returns {(i,j,k) -> coefficient of E1^i E2^j E3^k}.
fn s4_sym(f: F, a: u64, b: u64, xr: u64) -> BTreeMap<(u16, u16, u16), u64> {
    // variables x1, x2, x3, X
    let n = 4;
    let (x1, x2, x3, xx) = (MP::var(n, 0), MP::var(n, 1), MP::var(n, 2), MP::var(n, 3));
    let xrp = MP::konst(n, xr);
    let p1 = s3(f, n, &x1, &x2, &xx, a, b);
    let p2 = s3(f, n, &xx, &x3, &xrp, a, b);
    // coefficients in X (degree <= 2), as 3-variable polynomials in x1, x2, x3
    let cx = |p: &MP| -> Vec<MP> {
        let mut v = vec![MP::zero(3); 3];
        for (e, &c) in &p.t {
            let d = e[3] as usize;
            assert!(d <= 2, "S3 not quadratic in X");
            v[d].addterm(f, e[..3].to_vec(), c);
        }
        v
    };
    let (fa, fb) = (cx(&p1), cx(&p2));
    // Sylvester determinant of two quadratics f2 X^2 + f1 X + f0, g2 X^2 + g1 X + g0:
    // (f2 g0 - f0 g2)^2 - (f2 g1 - f1 g2)(f1 g0 - f0 g1)
    let u = fa[2].mul(f, &fb[0]).sub(f, &fa[0].mul(f, &fb[2]));
    let v = fa[2].mul(f, &fb[1]).sub(f, &fa[1].mul(f, &fb[2]));
    let w = fa[1].mul(f, &fb[0]).sub(f, &fa[0].mul(f, &fb[1]));
    let mut rem = u.mul(f, &u).sub(f, &v.mul(f, &w));
    if rem.is_zero() {
        die("S4 vanished");
    }
    // symmetric reduction: lex-leading x1^a x2^b x3^c -> E1^(a-b) E2^(b-c) E3^c
    let e1 = MP::var(3, 0).add(f, &MP::var(3, 1)).add(f, &MP::var(3, 2));
    let e2 = MP::var(3, 0).mul(f, &MP::var(3, 1)).add(f, &MP::var(3, 0).mul(f, &MP::var(3, 2))).add(f, &MP::var(3, 1).mul(f, &MP::var(3, 2)));
    let e3 = MP::var(3, 0).mul(f, &MP::var(3, 1)).mul(f, &MP::var(3, 2));
    let pw = |base: &MP, k: u16| -> MP {
        let mut r = MP::konst(3, 1);
        for _ in 0..k {
            r = r.mul(f, base);
        }
        r
    };
    let mut out: BTreeMap<(u16, u16, u16), u64> = BTreeMap::new();
    while let Some((e, &c)) = rem.t.iter().next_back() {
        let e = e.clone();
        if !(e[0] >= e[1] && e[1] >= e[2]) {
            die("S4 not symmetric?");
        }
        let key = (e[0] - e[1], e[1] - e[2], e[2]);
        let term = pw(&e1, key.0).mul(f, &pw(&e2, key.1)).mul(f, &pw(&e3, key.2)).scale(f, c);
        rem = rem.sub(f, &term);
        out.insert(key, c);
    }
    out
}

/// Arm F of gen.sing (m = 3): the full-symmetric system in (e1, e2, e3) with
/// the degree-3 remainder chain, for the Kummer or Dickson tower.
fn build_f(i: &Inst) -> System {
    let f = F::new(i.p);
    let (m, t) = (i.m, i.t);
    if m != 3 {
        die("f system is defined for m = 3 only (gen.sing arm F, S4 in e1,e2,e3)");
    }
    if t < 2 {
        die("f system needs t >= 2");
    }
    let k = 3usize;
    let mut nm: Vec<String> = vec!["e1".into(), "e2".into(), "e3".into()];
    for j in 1..t {
        if (1usize << j) >= k {
            for r in 0..k {
                nm.push(format!("fr{j}_{r}"));
            }
        }
    }
    let n = nm.len();
    let v = |s: &str| MP::var(n, names_index(&nm, s));
    let shift = i.kind.shift();
    let es = [v("e1"), v("e2"), v("e3")];
    let mut eqs: Vec<MP> = vec![];
    // remainder chain: R_0 = T, R_j = R_{j-1}^2 - shift mod Q, Q = T^3 - e1 T^2 + e2 T - e3
    let mut r: ZP = zmono(n, 1);
    for j in 1..=t {
        let sq = zsub(f, n, &zmul(f, n, &r, &r), &zconst(n, shift));
        if j < t && (1usize << j) < k {
            r = sq; // already reduced: no variables (gen.sing mchain)
            continue;
        }
        // reduce modulo Q: T^d -> T^(d-3) (e1 T^2 - e2 T + e3)
        let mut red = sq.clone();
        red.resize(red.len().max(k), MP::zero(n));
        for d in (k..red.len()).rev() {
            let c = std::mem::replace(&mut red[d], MP::zero(n));
            if c.is_zero() {
                continue;
            }
            for (ii, e) in es.iter().enumerate() {
                // T^k = sum_ii (-1)^ii e_{ii+1} T^{k-1-ii}
                let term = c.mul(f, e);
                let tgt = d - 1 - ii;
                red[tgt] = if ii % 2 == 0 { red[tgt].add(f, &term) } else { red[tgt].sub(f, &term) };
            }
        }
        let red = ztrim(red);
        let rsym: ZP = if j == t {
            zconst(n, i.c)
        } else {
            (0..k).map(|q| v(&format!("fr{j}_{q}"))).collect()
        };
        push_coeffs(&mut eqs, &zsub(f, n, &red, &rsym));
        r = rsym;
    }
    // S4 with E_i -> g^i e_i
    let g = i.scale;
    let mut s4 = MP::zero(n);
    for (&(p1, p2, p3), &c) in &s4_sym(f, i.a, i.b, i.xr) {
        let mut e = vec![0u16; n];
        e[0] = p1;
        e[1] = p2;
        e[2] = p3;
        let gk = f.pow(g, (p1 as u64) + 2 * (p2 as u64) + 3 * (p3 as u64));
        s4.addterm(f, e, f.mul(c, gk));
    }
    eqs.push(s4);
    // ── planted assignment
    let mut val = vec![u64::MAX; n];
    let (w1, w2, w3) = (i.w[0], i.w[1], i.w[2]);
    val[0] = f.add(f.add(w1, w2), w3);
    val[1] = f.add(f.add(f.mul(w1, w2), f.mul(w1, w3)), f.mul(w2, w3));
    val[2] = f.mul(f.mul(w1, w2), w3);
    let mut qn: UP = vec![1];
    for &w in &i.w {
        qn = umul(f, &qn, &vec![f.neg(w), 1]);
    }
    let mut rr: UP = vec![0, 1];
    for j in 1..t {
        rr = udivrem(f, &usubc(f, &umul(f, &rr, &rr), shift), &qn).1;
        if (1usize << j) >= k {
            for q in 0..k {
                val[names_index(&nm, &format!("fr{j}_{q}"))] = *rr.get(q).unwrap_or(&0);
            }
        }
    }
    assert!(val.iter().all(|&x| x != u64::MAX), "planted assignment incomplete");
    System { names: nm, eqs, plant: val }
}

// ───────────────────────────── output ──────────────────────────────────────

fn ms_text(sys: &System, p: u64) -> String {
    let mut s = sys.names.join(",");
    s.push('\n');
    s.push_str(&p.to_string());
    s.push('\n');
    let polys: Vec<String> = sys.eqs.iter().map(|e| e.fmt(&sys.names)).collect();
    s.push_str(&polys.join(",\n"));
    s.push('\n');
    s
}

fn sha256_hex(b: &[u8]) -> String {
    let d = Sha256::digest(b);
    d.iter().map(|x| format!("{x:02x}")).collect()
}

fn planted_check(sys: &System, f: F) {
    for (k, e) in sys.eqs.iter().enumerate() {
        if e.is_zero() {
            die(&format!("zero equation {}", k + 1));
        }
        if e.eval(f, &sys.plant) != 0 {
            die(&format!("planted check failed at equation {}", k + 1));
        }
    }
}

fn inst_json(i: &Inst) -> Value {
    json!({
        "p": i.p, "a": i.a, "b": i.b, "kind": i.kind.name(), "m": i.m, "t": i.t,
        "scale_g": i.scale, "terminal_c": i.c, "w": i.w, "x": i.x, "y": i.y,
        "R": [i.xr, i.yr], "tree": i.tree, "Y": i.yval, "info": i.info,
    })
}

// ───────────────────────────── msolve log parser ───────────────────────────

fn parse_log(txt: &str) -> Value {
    // Same rules as research/ic_tree_gluing_20261103/parse.sh, plus rows*cols.
    let mut d = 0u64;
    let mut lp = 0u64;
    let mut w = 0u64;
    let mut wrows = 0u64;
    let mut cells: f64 = 0.0;
    let mut maxcells = 0u64;
    let mut maxcells_dims = (0u64, 0u64);
    let mut dsel = 0u64;
    let mut rounds = 0u64;
    let mut perdeg: BTreeMap<u64, u64> = BTreeMap::new();
    let mut q: Option<u64> = None;
    let mut ex: Option<i64> = None;
    let mut wall: Option<f64> = None;
    let isnum = |s: &str| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit());
    for line in txt.lines() {
        let fl: Vec<&str> = line.split_whitespace().collect();
        if fl.len() >= 11
            && isnum(fl[0])
            && isnum(fl[1])
            && isnum(fl[2])
            && isnum(fl[3])
            && fl[4] == "x"
            && isnum(fl[5])
            && fl[6].ends_with('%')
            && fl[6][..fl[6].len() - 1].bytes().all(|c| c.is_ascii_digit() || c == b'.')
            && isnum(fl[7])
            && fl[8] == "new"
            && isnum(fl[9])
            && fl[10] == "zero"
        {
            let deg: u64 = fl[0].parse().unwrap();
            let r: u64 = fl[3].parse().unwrap();
            let c: u64 = fl[5].parse().unwrap();
            let nw: u64 = fl[7].parse().unwrap();
            rounds += 1;
            d = d.max(deg);
            if c > w {
                w = c;
                wrows = r;
            }
            cells += (r as f64) * (c as f64);
            if r * c > maxcells {
                maxcells = r * c;
                maxcells_dims = (r, c);
            }
            if nw > 0 && deg > lp {
                lp = deg;
            }
            let e = perdeg.entry(deg).or_insert(0);
            *e = (*e).max(c);
        }
        if fl.len() >= 5 && isnum(fl[0]) && fl[4] == "x" && isnum(fl[3]) {
            dsel = dsel.max(fl[0].parse().unwrap());
        }
        if q.is_none() {
            if let Some(pos) = line.find("Dimension of quotient:") {
                let rest = line[pos + 22..].trim_start();
                let num: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !num.is_empty() {
                    q = Some(num.parse().unwrap());
                }
            }
        }
        if ex.is_none() {
            if let Some(pos) = line.find("exit=") {
                let rest = &line[pos + 5..];
                let mut it = rest.split_whitespace();
                if let (Some(e), Some(wl)) = (it.next(), it.next()) {
                    if let (Ok(e), Some(wl)) = (e.parse::<i64>(), wl.strip_prefix("wall=")) {
                        ex = Some(e);
                        wall = wl.parse().ok();
                    }
                }
            }
        }
    }
    let alloc = txt.contains("allocat") || txt.contains("Could not") || txt.contains("out of memory");
    let status = match ex {
        Some(0) if q.is_some() => "ok",
        Some(0) => "ok-no-quotient",
        Some(124) => "timeout",
        Some(139) | Some(134) | Some(137) => "cap",
        Some(_) if alloc => "cap",
        Some(_) => "error",
        None => "unterminated",
    };
    let perdeg: Map<String, Value> = perdeg.into_iter().map(|(k, v)| (k.to_string(), json!(v))).collect();
    json!({
        "D": d, "lastprod": lp, "Dsel": dsel,
        "widest_cols": w, "widest_rows": wrows, "widest_cells": wrows * w,
        "max_cells": maxcells, "max_cells_dims": [maxcells_dims.0, maxcells_dims.1],
        "sumcells": cells, "rounds": rounds, "widest_cols_by_degree": perdeg,
        "quotient_dim": q, "exit": ex, "wall_s": wall, "status": status,
        "D_is_lower_bound": status != "ok",
    })
}

/// Parse msolve's `-o` Groebner-basis file into polynomials over the names.
fn parse_gb(txt: &str, names: &[String], f: F) -> Vec<MP> {
    let n = names.len();
    let body: String = txt.lines().filter(|l| !l.starts_with('#')).collect::<Vec<_>>().join("\n");
    let start = body.find('[').unwrap_or_else(|| die("no '[' in GB file"));
    let end = body[start..].find(']').unwrap_or_else(|| die("no ']' in GB file")) + start;
    let inner = &body[start + 1..end];
    let idx: HashMap<&str, usize> = names.iter().enumerate().map(|(i, s)| (s.as_str(), i)).collect();
    let mut out = vec![];
    for ptxt in inner.split(',') {
        let ptxt: String = ptxt.chars().filter(|c| !c.is_whitespace()).collect();
        if ptxt.is_empty() {
            continue;
        }
        let mut poly = MP::zero(n);
        // split into signed terms
        let mut terms: Vec<(bool, String)> = vec![];
        let mut cur = String::new();
        let mut neg = false;
        for (k, ch) in ptxt.chars().enumerate() {
            if (ch == '+' || ch == '-') && k > 0 && !cur.ends_with('^') {
                terms.push((neg, cur.clone()));
                cur.clear();
                neg = ch == '-';
            } else if ch == '-' && k == 0 {
                neg = true;
            } else if ch != '+' {
                cur.push(ch);
            }
        }
        terms.push((neg, cur));
        for (neg, tm) in terms {
            let mut coef = 1u64;
            let mut e = vec![0u16; n];
            for fac in tm.split('*') {
                if fac.bytes().all(|c| c.is_ascii_digit()) {
                    coef = f.mul(coef, f.red(fac.parse::<u128>().map(|x| (x % f.p as u128) as u64).unwrap()));
                } else {
                    let (nmv, ex) = match fac.split_once('^') {
                        Some((a, b)) => (a, b.parse::<u16>().unwrap()),
                        None => (fac, 1),
                    };
                    let vi = *idx.get(nmv).unwrap_or_else(|| die(&format!("unknown GB variable {nmv}")));
                    e[vi] += ex;
                }
            }
            poly.addterm(f, e, if neg { f.neg(coef) } else { coef });
        }
        out.push(poly);
    }
    out
}

// ───────────────────────────── CLI ─────────────────────────────────────────

fn die(msg: &str) -> ! {
    eprintln!("ERROR: {msg}");
    exit(1);
}

fn args_map(args: &[String]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let mut k = 0;
    while k < args.len() {
        let a = &args[k];
        if let Some(key) = a.strip_prefix("--") {
            if k + 1 < args.len() && !args[k + 1].starts_with("--") {
                m.insert(key.to_string(), args[k + 1].clone());
                k += 2;
            } else {
                m.insert(key.to_string(), "1".into());
                k += 1;
            }
        } else {
            m.insert(format!("_{k}"), a.clone());
            k += 1;
        }
    }
    m
}

fn get_u(m: &HashMap<String, String>, k: &str) -> Option<u64> {
    m.get(k).map(|v| {
        let v = v.replace('_', "");
        if let Some(h) = v.strip_prefix("0x") {
            u64::from_str_radix(h, 16).unwrap_or_else(|_| die(&format!("bad --{k}")))
        } else {
            v.parse().unwrap_or_else(|_| die(&format!("bad --{k}")))
        }
    })
}
fn req_u(m: &HashMap<String, String>, k: &str) -> u64 {
    get_u(m, k).unwrap_or_else(|| die(&format!("missing --{k}")))
}

fn cmd_curve(m: &HashMap<String, String>) {
    let p = req_u(m, "p");
    let seed = get_u(m, "seed").unwrap_or(0);
    if !is_prime(p) {
        die("p is not prime");
    }
    let f = F::new(p);
    let mut rng = Rng(seed ^ 0x4355_5256_4531_3130);
    for tries in 1..=1_000_000u64 {
        let a = 1 + rng.below(p - 1);
        let b = 1 + rng.below(p - 1);
        let cv = Curve { f, a, b };
        match prime_order(&cv, &mut rng) {
            Ok(n) => {
                let g = cv.random_point(&mut rng);
                let (gx, gy) = g.unwrap();
                assert!(cv.mul(n, g).is_none());
                let j_num = f.mul(1728, f.mul(4, f.pow(a, 3)));
                let j_den = f.add(f.mul(4, f.pow(a, 3)), f.mul(27, f.mul(b, b)));
                let j = f.mul(j_num, f.inv(j_den));
                let out = json!({
                    "schema": "p1tool.curve/v1", "p": p, "a": a, "b": b, "n": n, "j": j,
                    "generator": [gx, gy], "seed": seed, "tries": tries,
                    "v2_p_minus_1": (p - 1).trailing_zeros(), "v2_p_plus_1": (p + 1).trailing_zeros(),
                    "order_method": if p < (1 << 22) { "bsgs+naive_count" } else { "bsgs+hasse_uniqueness" },
                    "random_point_checks": 8,
                });
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
                return;
            }
            Err(_) => continue,
        }
    }
    die("no prime-order curve found");
}

/// Exact #E(F_p) by the Legendre-symbol sum (independent of BSGS), p < 2^25.
fn cmd_count(m: &HashMap<String, String>) {
    let p = req_u(m, "p");
    if p >= (1 << 25) {
        die("count: naive sum only for p < 2^25");
    }
    let cv = Curve { f: F::new(p), a: req_u(m, "a"), b: req_u(m, "b") };
    let n = cv.count_naive();
    println!("{}", json!({"p": p, "a": cv.a, "b": cv.b, "order_naive": n, "prime": is_prime(n)}));
}

fn cmd_emit(m: &HashMap<String, String>) {
    let system = m.get("system").map(|s| s.as_str()).unwrap_or_else(|| die("missing --system rr|t|f"));
    if system == "f" && req_u(m, "m") != 3 {
        die("--system f is defined for m = 3 only");
    }
    let kind = match m.get("kind").map(|s| s.as_str()) {
        Some("kummer") => Kind::Kummer,
        Some("dickson") => Kind::Dickson,
        _ => die("--kind kummer|dickson"),
    };
    let mm = req_u(m, "m") as usize;
    let t = req_u(m, "t") as usize;
    let out = m.get("out").unwrap_or_else(|| die("missing --out FILE.ms"));
    let inst = if let Some(path) = m.get("round7") {
        if kind != Kind::Kummer {
            die("--round7 instances are Kummer");
        }
        round7_instance(path, mm, t, get_u(m, "k").unwrap_or(0) as usize)
    } else {
        sample_instance(req_u(m, "p"), req_u(m, "a"), req_u(m, "b"), kind, mm, t, get_u(m, "g"), req_u(m, "seed"))
    };
    if system == "rr" && inst.info.get("sign_patterns").is_some_and(|v| v != 1) {
        die("RR needs exactly one sign pattern matching (x_R, y_R)");
    }
    let sys = match system {
        "rr" => build_rr(&inst),
        "t" => build_t(&inst),
        "f" => build_f(&inst),
        _ => die("--system rr|t|f"),
    };
    let f = F::new(inst.p);
    planted_check(&sys, f);
    let (sys, reduction) = if let Some(mode) = m.get("reduce") {
        if system != "rr" {
            die("--reduce is currently defined only for RR");
        }
        reduction::reduce(sys, f, mode)
    } else {
        (sys, Value::Null)
    };
    planted_check(&sys, f);
    let txt = ms_text(&sys, inst.p);
    std::fs::write(out, &txt).unwrap_or_else(|e| die(&format!("{out}: {e}")));
    let degs: Vec<u32> = sys.eqs.iter().map(|e| e.deg()).collect();
    let mut hist: BTreeMap<u32, usize> = BTreeMap::new();
    for &d in &degs {
        *hist.entry(d).or_default() += 1;
    }
    let plant: Map<String, Value> = sys.names.iter().zip(&sys.plant).map(|(k, &v)| (k.clone(), json!(v))).collect();
    let summary = json!({
        "schema": "p1tool.emit/v1",
        "system": system, "kind": kind.name(), "m": mm, "t": t, "p": inst.p,
        "N": sys.names.len(), "equations": sys.eqs.len(),
        "max_input_degree": degs.iter().max(), "min_input_degree": degs.iter().min(),
        "input_degree_histogram": hist.iter().map(|(k, v)| (k.to_string(), json!(v))).collect::<Map<String, Value>>(),
        "file": out, "sha256": sha256_hex(txt.as_bytes()), "planted_check": "all equations vanish",
        "instance": inst_json(&inst),
    });
    let plant_path = m.get("plant").cloned().unwrap_or_else(|| format!("{out}.plant.json"));
    let mut full = json!({"summary": summary, "planted": plant});
    if !reduction.is_null() {
        full["reduction"] = reduction;
    }
    std::fs::write(&plant_path, serde_json::to_string_pretty(&full).unwrap()).unwrap();
    println!("{}", serde_json::to_string(&summary).unwrap());
}

fn cmd_parse(m: &HashMap<String, String>) {
    let path = m.get("_1").unwrap_or_else(|| die("usage: parse LOG"));
    let txt = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut v = parse_log(&txt);
    v["file"] = json!(path);
    println!("{}", serde_json::to_string(&v).unwrap());
}

fn cmd_check_gb(m: &HashMap<String, String>) {
    let gb = m.get("gb").unwrap_or_else(|| die("missing --gb FILE"));
    let pl = m.get("plant").unwrap_or_else(|| die("missing --plant FILE.plant.json"));
    let pj: Value = serde_json::from_str(&std::fs::read_to_string(pl).unwrap()).unwrap();
    let p = pj["summary"]["p"].as_u64().unwrap();
    let f = F::new(p);
    let msfile = pj["summary"]["file"].as_str().unwrap();
    let mstxt = std::fs::read_to_string(m.get("ms").map(|s| s.as_str()).unwrap_or(msfile)).unwrap();
    let names: Vec<String> = mstxt.lines().next().unwrap().split(',').map(|s| s.to_string()).collect();
    let vals: Vec<u64> = names.iter().map(|k| pj["planted"][k].as_u64().unwrap()).collect();
    let gbtxt = std::fs::read_to_string(gb).unwrap();
    let polys = parse_gb(&gbtxt, &names, f);
    let nonzero = polys.iter().filter(|q| q.eval(f, &vals) != 0).count();
    let linear = polys.iter().filter(|q| q.deg() == 1 && q.t.len() <= 2).count();
    let unit = polys.iter().any(|q| q.deg() == 0 && !q.is_zero());
    let out = json!({
        "gb": gb, "elements": polys.len(), "nonzero_at_planted": nonzero,
        "linear_univariate_elements": linear, "unit_ideal": unit,
        "planted_on_variety": nonzero == 0 && !unit,
        "planted_is_unique_solution": nonzero == 0 && !unit && linear == names.len() && polys.len() == names.len(),
    });
    println!("{}", serde_json::to_string(&out).unwrap());
    if nonzero != 0 || unit {
        exit(2);
    }
}

// ───────────────────────────── selftest ────────────────────────────────────

/// The round-7 m3 t4 k0 instance line and the sha256 recorded in
/// round7_rr/SYSTEMS.sha256 for genrr.sing's output on it.
const R7_FILE: &str = "# curve\n786433 779083 163026 780070\n3 4 0 206735 241748 388249 570203 191580 191580\n";
const R7_M3T4_RR_SHA: &str = "2ceb4c5970c27253ec12844303164715d8f4e3e53b7f7bff249885757b82385a";
/// sha256 of gen.sing's (unmodified, Singular 4.3.2) arm-F output m3_t4_k0_F.ms on the
/// same instance; calibration/round2/controlA/controlA_F.txt.
const R7_M3T4_F_SHA: &str = "16589edffb43504a4861596e9bb8c30adab0654a5d9ee2fc51a6b37cd3c3536f";

fn cmd_selftest() {
    let mut n = 0;
    // field
    for &p in &[786433u64, 14_680_063, 2_046_820_351, 1019, 1999] {
        let f = F::new(p);
        let mut rng = Rng(p);
        for _ in 0..200 {
            let a = rng.below(p);
            if let Some(r) = f.sqrt(a) {
                assert_eq!(f.mul(r, r), a);
            } else {
                assert!(!f.is_sq(a));
            }
            if a != 0 {
                assert_eq!(f.mul(a, f.inv(a)), 1);
            }
        }
        n += 1;
    }
    // Dickson identity D_{2k} = D_k^2 - 2 on x = w + 1/w: D_{2^t}(x) = w^{2^t} + w^{-2^t}
    {
        let f = F::new(786433);
        let mut rng = Rng(7);
        for _ in 0..100 {
            let w = 1 + rng.below(f.p - 1);
            let x = f.add(w, f.inv(w));
            for t in 0..8 {
                let e = 1u64 << t;
                let want = f.add(f.pow(w, e), f.inv(f.pow(w, e)));
                assert_eq!(tower_value(Kind::Dickson, f, x, t), want);
            }
        }
        n += 1;
    }
    // curve order: BSGS vs naive count at small primes
    for &p in &[1019u64, 1999, 2039, 4093] {
        let f = F::new(p);
        let mut rng = Rng(p * 3);
        for _ in 0..20 {
            let cv = Curve { f, a: 1 + rng.below(p - 1), b: 1 + rng.below(p - 1) };
            let nn = cv.count_naive();
            let pt = cv.random_point(&mut rng);
            assert!(cv.mul(nn, pt).is_none(), "naive count does not annihilate");
            assert!(cv.hasse_annihilators(pt).contains(&nn));
        }
        n += 1;
    }
    // torus base sets at the protocol primes
    for &p in &[14_680_063u64, 2_046_820_351, 1279, 1999] {
        let f = F::new(p);
        let tmax = (p + 1).trailing_zeros().min(10) as usize;
        for t in 1..=tmax {
            let (v, c, _, _) = base_set(Kind::Dickson, f, t, None, &mut Rng(t as u64));
            assert_eq!(v.len(), 1 << t);
            assert!(v.iter().all(|&x| tower_value(Kind::Dickson, f, x, t) == c));
        }
        n += 1;
    }
    // Kummer refused at a torus prime, Dickson refused at a split prime
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    assert!(std::panic::catch_unwind(|| base_set(Kind::Kummer, F::new(14_680_063), 3, None, &mut Rng(1))).is_err());
    assert!(std::panic::catch_unwind(|| base_set(Kind::Dickson, F::new(786433), 3, None, &mut Rng(1))).is_err());
    std::panic::set_hook(hook);
    n += 1;
    // round-7 RR m3 t4 byte identity with genrr.sing's recorded hash
    {
        let tmp = std::env::temp_dir().join(format!("p1tool_selftest_{}.txt", std::process::id()));
        std::fs::write(&tmp, R7_FILE).unwrap();
        let inst = round7_instance(tmp.to_str().unwrap(), 3, 4, 0);
        let _ = std::fs::remove_file(&tmp);
        let sys = build_rr(&inst);
        planted_check(&sys, F::new(786433));
        assert_eq!(sha256_hex(ms_text(&sys, 786433).as_bytes()), R7_M3T4_RR_SHA, "RR m3t4 differs from genrr.sing");
        let st = build_t(&inst);
        planted_check(&st, F::new(786433));
        let sf = build_f(&inst);
        planted_check(&sf, F::new(786433));
        assert_eq!(sha256_hex(ms_text(&sf, 786433).as_bytes()), R7_M3T4_F_SHA, "F m3t4 differs from gen.sing");
        assert_eq!((sf.names.len(), sf.eqs.len()), (9, 10), "F m3t4 size");
        n += 1;
    }
    // planted systems on sampled instances, both kinds, both systems
    for &(p, kind, t) in &[(1279u64, Kind::Dickson, 4usize), (1999, Kind::Dickson, 4), (786433, Kind::Kummer, 4), (14_680_063, Kind::Dickson, 4)] {
        let f = F::new(p);
        let mut rng = Rng(p);
        let (a, b) = loop {
            let cv = Curve { f, a: 1 + rng.below(p - 1), b: 1 + rng.below(p - 1) };
            if prime_order(&cv, &mut rng).is_ok() {
                break (cv.a, cv.b);
            }
        };
        for mm in [3usize] {
            let inst = sample_instance(p, a, b, kind, mm, t, None, 11);
            planted_check(&build_rr(&inst), f);
            planted_check(&build_t(&inst), f);
            // arm F on the same instance, t = 3 and the cell's t
            planted_check(&build_f(&inst), f);
            let inst3 = sample_instance(p, a, b, kind, mm, 3, None, 11);
            let f3 = build_f(&inst3);
            planted_check(&f3, f);
            assert_eq!((f3.names.len(), f3.eqs.len()), (6, 7), "F m3t3 size");
        }
        n += 1;
    }
    // S4 in (e1,e2,e3) vanishes at random 3-term decompositions (independent of gen.sing)
    {
        let p = 786433u64;
        let f = F::new(p);
        let cv = Curve { f, a: 779083, b: 163026 };
        let mut rng = Rng(0xF4);
        for _ in 0..20 {
            let pts: Vec<Pt> = (0..3).map(|_| cv.random_point(&mut rng)).collect();
            let r = pts.iter().fold(None, |acc, &q| cv.add(acc, q));
            let Some((xr, _)) = r else { continue };
            let s4 = s4_sym(f, cv.a, cv.b, xr);
            let xs: Vec<u64> = pts.iter().map(|q| q.unwrap().0).collect();
            let e = [
                f.add(f.add(xs[0], xs[1]), xs[2]),
                f.add(f.add(f.mul(xs[0], xs[1]), f.mul(xs[0], xs[2])), f.mul(xs[1], xs[2])),
                f.mul(f.mul(xs[0], xs[1]), xs[2]),
            ];
            let val = s4.iter().fold(0, |acc, (&(i, j, k), &c)| {
                f.add(acc, f.mul(c, f.mul(f.pow(e[0], i as u64), f.mul(f.pow(e[1], j as u64), f.pow(e[2], k as u64)))))
            });
            assert_eq!(val, 0, "S4 does not vanish at a decomposition");
        }
        n += 1;
    }
    // log parser on a synthetic log
    {
        let log = "  2       3      16      10 x 15         23.33%        2 new       1 zero         0.00 | 0.00\n  4     697    1769    2989 x 2801        3.98%      319 new     383 zero         0.14 | 0.13\nreduce final basis       18 x 19         11.11%       18 new       0 zeroDimension of quotient: 1\nexit=0 wall=.46\n";
        let v = parse_log(log);
        assert_eq!(v["D"], 4);
        assert_eq!(v["widest_cols"], 2801);
        assert_eq!(v["widest_rows"], 2989);
        assert_eq!(v["quotient_dim"], 1);
        assert_eq!(v["status"], "ok");
        n += 1;
    }
    println!("{}", json!({"selftest": "pass", "groups": n}));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: p1tool curve|count|emit|parse|check-gb|lift|extract|collect|selftest [--flags]");
        exit(2);
    }
    let m = args_map(&args[1..]);
    let mut m2 = m.clone();
    if let Some(p) = args.get(1) {
        if !p.starts_with("--") {
            m2.insert("_1".into(), p.clone());
        }
    }
    match args[0].as_str() {
        "curve" => cmd_curve(&m),
        "count" => cmd_count(&m),
        "emit" => cmd_emit(&m),
        "parse" => cmd_parse(&m2),
        "check-gb" => cmd_check_gb(&m),
        "lift" => reduction::cmd_lift(&m),
        "extract" => extraction::cmd_extract(&m),
        "collect" => collection::cmd_collect(&m),
        "selftest" => cmd_selftest(),
        _ => die("unknown subcommand"),
    }
}
