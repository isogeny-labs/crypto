//! Blind-spot item 10: program search over the binary Semaev S4 decomposition
//! kernel, fitness = counted oracle cost.
//!
//! Standard library only (the `crypto` library does not compile at this branch
//! base, see PROTOCOL.md). Build (each card does this):
//!   rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native \
//!         -o target/blindspots-10/<dir>/b10 examples/blindspots_10_kernel_search.rs
//!
//! Subcommands: selftest | search | finalists | heldout | conserve | ablate |
//! summarize (see PROTOCOL.md for what each card runs).
//!
//! # The object
//!
//! `F = GF(2^n)`, `n` odd, the Koblitz curve `E: y^2 + xy = x^3 + x^2 + 1`
//! (`a = b = 1`). A factor base is a set `X` of `2^l` abscissae; `x in X` is
//! *rational* if some `P in E(F)` has `x(P) = x`. A *relation* for the target
//! `R` is a triple `(x1, x2, x3)` of rational members of `X` with
//! `S4(x1, x2, x3, x_R) = 0` (so `R = +-P1 +-P2 +-P3` for points over `F`).
//! `S4` is a quartic in each argument; the kernel loops over pairs `(x1, x2)`,
//! builds the quartic in `x3` and finds its roots in `X`.
//!
//! # The incumbent
//!
//! Replica of `SubspaceOracle::decompose` (src/cryptanalysis/semaev_decomp.rs)
//! as the pipeline `ic_boundary.rs` uses it: base = a subspace `V` of
//! dimension `l` (all `2^l` abscissae, rational or not), sorted pairs `x1 <= x2`
//! in row order, general-`b` quartic, one batch inversion per row, the monic
//! Horner `L_V mod f` route with the quartic Frobenius table, the
//! pseudo-remainder gcd, stop at the first RAW witness; the caller then checks
//! that the witness lifts (all three abscissae rational) and a witness that
//! does not lift yields no relation for that target (`lift_failures`).
//!
//! # The kernel DSL
//!
//! A program is a vector of small integer genes (see `Prog`): base family
//! (subspace low-order or random, union of cosets, Hamming ball, geometric
//! progression), lift prefilter, stop rule, root finder, quartic form, square
//! table, pair mode, row-abort fraction, inversion batching, gcd form.
//!
//! # The unit
//!
//! Counted unit: field multiplications + squarings (`core`), every inversion
//! executed as an Itoh-Tsujii chain of counted multiplications and squarings.
//! Unpriced native work, each counted and priced at w in {0, 0.1, 1} core
//! operations (PROTOCOL.md, AGENTS.md section 5a): coefficient word XORs
//! (`lin`), hash/table probes (`probe`), trace tests (`trace`), pair
//! iterations (`pairs`). Wall time is descriptive only.

#![allow(dead_code)]

use std::cell::Cell;
use std::collections::HashMap;
use std::io::Write as IoWrite;
use std::time::Instant;

// ------------------------------------------------------------------ utilities

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        let lim = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next();
            if v < lim {
                return v % n;
            }
        }
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn mix(a: u64, b: u64) -> u64 {
    let mut r = Rng(a ^ b.wrapping_mul(0xD6E8_FEB8_6659_FD93));
    r.next()
}

fn mix3(a: u64, b: u64, c: u64) -> u64 {
    mix(mix(a, b), c)
}

struct Args {
    v: Vec<String>,
}
impl Args {
    fn flag(&self, name: &str) -> Option<String> {
        self.v
            .iter()
            .position(|a| a == name)
            .and_then(|i| self.v.get(i + 1))
            .cloned()
    }
    fn num<T: std::str::FromStr>(&self, name: &str, default: T) -> T {
        self.flag(name)
            .map(|s| s.parse().unwrap_or_else(|_| panic!("bad value for {name}")))
            .unwrap_or(default)
    }
    fn req(&self, name: &str) -> String {
        self.flag(name)
            .unwrap_or_else(|| panic!("missing required flag {name}"))
    }
}

fn rss_peak_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

// ------------------------------------------------------------------- GF(2^n)

fn pdeg(a: u64) -> u32 {
    63 - a.leading_zeros()
}

fn pmod(mut a: u64, b: u64) -> u64 {
    let db = pdeg(b);
    while a != 0 && pdeg(a) >= db {
        a ^= b << (pdeg(a) - db);
    }
    a
}

fn pmulmod(mut a: u64, mut b: u64, f: u64, n: u32) -> u64 {
    let mut r = 0u64;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        b >>= 1;
        a <<= 1;
        if (a >> n) & 1 == 1 {
            a ^= f;
        }
    }
    r
}

fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a = pmod(a, b);
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            out.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

/// Rabin's irreducibility test for `f` of degree `n`.
fn is_irreducible(f: u64) -> bool {
    let n = pdeg(f);
    if n == 1 {
        return true;
    }
    let mut t = 2u64;
    for _ in 0..n {
        t = pmulmod(t, t, f, n);
    }
    if t != 2 {
        return false;
    }
    for p in prime_factors(n) {
        let mut t = 2u64;
        for _ in 0..(n / p) {
            t = pmulmod(t, t, f, n);
        }
        if pgcd(f, t ^ 2) != 1 {
            return false;
        }
    }
    true
}

/// Same candidate order as the repository's `find_irreducible_sparse` (and as
/// the other blind-spot instruments): the least `low` among constant + up to
/// three further terms for which `z^n + low` is irreducible.
fn find_irreducible_sparse(n: u32) -> u64 {
    let mut cands: Vec<u64> = vec![1];
    for i in 1..n {
        cands.push(1 | (1 << i));
        for j in (i + 1)..n {
            cands.push(1 | (1 << i) | (1 << j));
            for k in (j + 1)..n {
                cands.push(1 | (1 << i) | (1 << j) | (1 << k));
            }
        }
    }
    cands.sort_unstable();
    let hi = 1u64 << n;
    for low in cands {
        if is_irreducible(hi | low) {
            return low;
        }
    }
    panic!("no sparse irreducible of degree {n}");
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn clmul_hw(a: u64, b: u64) -> u128 {
    use std::arch::x86_64::*;
    let x = _mm_set_epi64x(0, a as i64);
    let y = _mm_set_epi64x(0, b as i64);
    let r = _mm_clmulepi64_si128(x, y, 0x00);
    let lo = _mm_cvtsi128_si64(r) as u64;
    let hi = _mm_cvtsi128_si64(_mm_unpackhi_epi64(r, r)) as u64;
    ((hi as u128) << 64) | lo as u128
}

fn clmul_soft(a: u64, mut b: u64) -> u128 {
    let (mut r, mut s) = (0u128, a as u128);
    while b != 0 {
        if b & 1 == 1 {
            r ^= s;
        }
        s <<= 1;
        b >>= 1;
    }
    r
}

/// The operation counters. `mul`, `sqr` are the counted unit; the rest are the
/// unpriced native work priced at a weight.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct Ops {
    mul: u64,
    sqr: u64,
    lin: u64,
    probe: u64,
    trace: u64,
    pairs: u64,
}
impl Ops {
    fn sub(self, o: Ops) -> Ops {
        Ops {
            mul: self.mul - o.mul,
            sqr: self.sqr - o.sqr,
            lin: self.lin - o.lin,
            probe: self.probe - o.probe,
            trace: self.trace - o.trace,
            pairs: self.pairs - o.pairs,
        }
    }
    fn core(&self) -> u64 {
        self.mul + self.sqr
    }
    fn nat(&self) -> u64 {
        self.lin + self.probe + self.trace + self.pairs
    }
    fn priced(&self, w: f64) -> f64 {
        self.core() as f64 + w * self.nat() as f64
    }
}

struct Gf {
    n: u32,
    mask: u64,
    tail: u64,
    trmask: u64,
    ht: Vec<u64>,
    hw: bool,
    c_mul: Cell<u64>,
    c_sqr: Cell<u64>,
    c_lin: Cell<u64>,
    c_probe: Cell<u64>,
    c_trace: Cell<u64>,
    c_pairs: Cell<u64>,
}

impl Gf {
    fn new(n: u32) -> Gf {
        assert!(n % 2 == 1 && (5..=61).contains(&n), "odd n in 5..=61");
        let low = find_irreducible_sparse(n);
        #[cfg(target_arch = "x86_64")]
        let hw = is_x86_feature_detected!("pclmulqdq");
        #[cfg(not(target_arch = "x86_64"))]
        let hw = false;
        let mut gf = Gf {
            n,
            mask: (1u64 << n) - 1,
            tail: low,
            trmask: 0,
            ht: vec![],
            hw,
            c_mul: Cell::new(0),
            c_sqr: Cell::new(0),
            c_lin: Cell::new(0),
            c_probe: Cell::new(0),
            c_trace: Cell::new(0),
            c_pairs: Cell::new(0),
        };
        let mut tm = 0u64;
        for k in 0..n {
            let e = 1u64 << k;
            let (mut t, mut s) = (0u64, e);
            for _ in 0..n {
                t ^= s;
                s = gf.sqr_nc(s);
            }
            assert!(t == 0 || t == 1);
            if t == 1 {
                tm |= e;
            }
        }
        gf.trmask = tm;
        let mut ht = vec![0u64; n as usize];
        for k in 0..n {
            let mut s = 1u64 << k;
            let mut t = 0u64;
            for _ in 0..=((n - 1) / 2) {
                t ^= s;
                s = gf.sqr_nc(gf.sqr_nc(s));
            }
            ht[k as usize] = t;
        }
        gf.ht = ht;
        gf
    }
    fn snap(&self) -> Ops {
        Ops {
            mul: self.c_mul.get(),
            sqr: self.c_sqr.get(),
            lin: self.c_lin.get(),
            probe: self.c_probe.get(),
            trace: self.c_trace.get(),
            pairs: self.c_pairs.get(),
        }
    }
    #[inline(always)]
    fn reduce(&self, mut r: u128) -> u64 {
        let n = self.n;
        loop {
            let hi = (r >> n) as u64;
            if hi == 0 {
                return r as u64;
            }
            r = (r & self.mask as u128) ^ self.clmul(hi, self.tail);
        }
    }
    #[inline(always)]
    fn clmul(&self, a: u64, b: u64) -> u128 {
        #[cfg(target_arch = "x86_64")]
        {
            if self.hw {
                return unsafe { clmul_hw(a, b) };
            }
        }
        clmul_soft(a, b)
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.c_mul.set(self.c_mul.get() + 1);
        self.reduce(self.clmul(a, b))
    }
    #[inline(always)]
    fn sqr(&self, a: u64) -> u64 {
        self.c_sqr.set(self.c_sqr.get() + 1);
        self.reduce(self.clmul(a, a))
    }
    #[inline(always)]
    fn mul_nc(&self, a: u64, b: u64) -> u64 {
        self.reduce(self.clmul(a, b))
    }
    #[inline(always)]
    fn sqr_nc(&self, a: u64) -> u64 {
        self.reduce(self.clmul(a, a))
    }
    /// A counted word XOR (the native `lin` counter).
    #[inline(always)]
    fn add(&self, a: u64, b: u64) -> u64 {
        self.c_lin.set(self.c_lin.get() + 1);
        a ^ b
    }
    #[inline(always)]
    fn add3(&self, a: u64, b: u64, c: u64) -> u64 {
        self.c_lin.set(self.c_lin.get() + 2);
        a ^ b ^ c
    }
    /// `a*b + c*d` as two counted multiplications and one XOR.
    #[inline(always)]
    fn dot2(&self, a: u64, b: u64, c: u64, d: u64) -> u64 {
        self.add(self.mul(a, b), self.mul(c, d))
    }
    fn sqr_k(&self, mut a: u64, k: u32) -> u64 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }
    /// a^(2^n - 2) by the Itoh-Tsujii chain on n - 1, every step counted.
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        let e = self.n - 1;
        let bits = 32 - e.leading_zeros();
        let (mut b, mut k) = (a, 1u32);
        for i in (0..bits - 1).rev() {
            let mut t = b;
            for _ in 0..k {
                t = self.sqr(t);
            }
            b = self.mul(t, b);
            k *= 2;
            if (e >> i) & 1 == 1 {
                b = self.mul(self.sqr(b), a);
                k += 1;
            }
        }
        debug_assert_eq!(k, e);
        self.sqr(b)
    }
    fn inv_nc(&self, a: u64) -> u64 {
        let before = self.snap();
        let r = self.inv(a);
        self.restore(before);
        r
    }
    fn restore(&self, o: Ops) {
        self.c_mul.set(o.mul);
        self.c_sqr.set(o.sqr);
        self.c_lin.set(o.lin);
        self.c_probe.set(o.probe);
        self.c_trace.set(o.trace);
        self.c_pairs.set(o.pairs);
    }
    /// Montgomery batch inversion of nonzero `xs` in place: `3(m-1)`
    /// multiplications and one inversion.
    fn batch_inv(&self, xs: &mut [u64], scratch: &mut Vec<u64>) {
        let m = xs.len();
        if m == 0 {
            return;
        }
        scratch.clear();
        let mut acc = 1u64;
        for &x in xs.iter() {
            scratch.push(acc);
            acc = self.mul(acc, x);
        }
        let mut inv = self.inv(acc);
        for i in (0..m).rev() {
            let xi = xs[i];
            xs[i] = self.mul(inv, scratch[i]);
            inv = self.mul(inv, xi);
        }
    }
    #[inline(always)]
    fn tr(&self, a: u64) -> u64 {
        self.c_trace.set(self.c_trace.get() + 1);
        ((a & self.trmask).count_ones() & 1) as u64
    }
    #[inline(always)]
    fn tr_nc(&self, a: u64) -> u64 {
        ((a & self.trmask).count_ones() & 1) as u64
    }
    /// A root z of z^2 + z = c + Tr(c).
    fn half_trace(&self, c: u64) -> u64 {
        let mut r = 0u64;
        let mut x = c;
        while x != 0 {
            let i = x.trailing_zeros();
            r ^= self.ht[i as usize];
            x &= x - 1;
        }
        r
    }
    fn sqrt_nc(&self, a: u64) -> u64 {
        let mut t = a;
        for _ in 0..(self.n - 1) {
            t = self.sqr_nc(t);
        }
        t
    }
    fn pow_nc(&self, a: u64, mut e: u128) -> u64 {
        let (mut r, mut b) = (1u64, a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul_nc(r, b);
            }
            b = self.sqr_nc(b);
            e >>= 1;
        }
        r
    }
}

// --------------------------------------------------------------------- curve

/// The Koblitz curve y^2 + xy = x^3 + a x^2 + b with a = b = 1 in every cell.
#[derive(Clone, Copy)]
struct Curve {
    a: u64,
    b: u64,
}

type Pt = Option<(u64, u64)>;

fn neg(p: Pt) -> Pt {
    p.map(|(x, y)| (x, y ^ x))
}

fn on_curve(gf: &Gf, cv: &Curve, p: (u64, u64)) -> bool {
    let (x, y) = p;
    let lhs = gf.sqr_nc(y) ^ gf.mul_nc(x, y);
    let x2 = gf.sqr_nc(x);
    let rhs = gf.mul_nc(x2, x) ^ gf.mul_nc(cv.a, x2) ^ cv.b;
    lhs == rhs
}

/// Uncounted group law (target and base construction, never part of a cost).
fn add_pt(gf: &Gf, cv: &Curve, p: Pt, q: Pt) -> Pt {
    let (p, q) = match (p, q) {
        (None, q) => return q,
        (p, None) => return p,
        (Some(p), Some(q)) => (p, q),
    };
    let (x1, y1) = p;
    let (x2, y2) = q;
    if x1 == x2 {
        if y1 != y2 {
            return None;
        }
        if x1 == 0 {
            return None;
        }
        let lam = x1 ^ gf.mul_nc(y1, gf.inv_nc(x1));
        let x3 = gf.sqr_nc(lam) ^ lam ^ cv.a;
        let y3 = gf.sqr_nc(x1) ^ gf.mul_nc(lam ^ 1, x3);
        return Some((x3, y3));
    }
    let lam = gf.mul_nc(y1 ^ y2, gf.inv_nc(x1 ^ x2));
    let x3 = gf.sqr_nc(lam) ^ lam ^ cv.a ^ x1 ^ x2;
    let y3 = gf.mul_nc(lam, x1 ^ x3) ^ x3 ^ y1;
    Some((x3, y3))
}

fn scalar_mul(gf: &Gf, cv: &Curve, p: Pt, mut k: u128) -> Pt {
    let (mut r, mut q) = (None, p);
    while k > 0 {
        if k & 1 == 1 {
            r = add_pt(gf, cv, r, q);
        }
        q = add_pt(gf, cv, q, q);
        k >>= 1;
    }
    r
}

/// A point with abscissa x, if one exists (sign fixed by the half-trace).
fn point_at(gf: &Gf, cv: &Curve, x: u64) -> Option<(u64, u64)> {
    if x == 0 {
        return Some((0, gf.sqrt_nc(cv.b)));
    }
    let xi = gf.inv_nc(x);
    let c = x ^ cv.a ^ gf.mul_nc(cv.b, gf.sqr_nc(xi));
    if gf.tr_nc(c) == 1 {
        return None;
    }
    let z = gf.half_trace(c);
    Some((x, gf.mul_nc(x, z)))
}

/// #E(F_{2^n}) = 2^n + 1 - V_n with V_0 = 2, V_1 = mu, V_k = mu V_{k-1} - 2 V_{k-2}
/// (Koblitz a = 1: mu = 1).
fn koblitz_order(n: u32) -> u128 {
    let (mut v0, mut v1): (i128, i128) = (2, 1);
    for _ in 1..n {
        let v2 = v1 - 2 * v0;
        v0 = v1;
        v1 = v2;
    }
    ((1i128 << n) + 1 - v1) as u128
}

fn random_point(gf: &Gf, cv: &Curve, rng: &mut Rng) -> (u64, u64) {
    loop {
        let x = rng.next() & gf.mask;
        if let Some((x, y)) = point_at(gf, cv, x) {
            return if rng.next() & 1 == 1 {
                (x, y ^ x)
            } else {
                (x, y)
            };
        }
    }
}

// ---------------------------------------------------------- polynomials mod f

const MAXD: usize = 6;

#[derive(Clone, Copy, Debug)]
struct Poly {
    c: [u64; MAXD + 1],
}

impl Poly {
    fn zero() -> Self {
        Poly { c: [0; MAXD + 1] }
    }
    fn deg(&self) -> Option<usize> {
        (0..=MAXD).rev().find(|&i| self.c[i] != 0)
    }
    fn from_low(c: &[u64]) -> Self {
        let mut p = Poly::zero();
        p.c[..c.len()].copy_from_slice(c);
        p
    }
    fn scale_in_place(&mut self, k: u64, hi: usize, gf: &Gf) {
        for i in 0..=hi {
            self.c[i] = gf.mul(self.c[i], k);
        }
    }
    /// Remainder modulo a monic `m` of degree `dm`.
    fn rem_monic(&self, m: &Poly, dm: usize, gf: &Gf) -> Poly {
        let mut r = *self;
        let mut dr = MAXD;
        loop {
            while r.c[dr] == 0 {
                if dr == 0 || dr == dm {
                    return r;
                }
                dr -= 1;
            }
            if dr < dm {
                return r;
            }
            let factor = r.c[dr];
            for i in 0..dm {
                r.c[dr - dm + i] = gf.add(r.c[dr - dm + i], gf.mul(m.c[i], factor));
            }
            r.c[dr] = 0;
            if dr == 0 {
                return r;
            }
            dr -= 1;
        }
    }
    fn sqr_mod_monic(&self, m: &Poly, dm: usize, gf: &Gf) -> Poly {
        let mut wide = Poly::zero();
        for i in 0..=MAXD / 2 {
            if self.c[i] != 0 {
                wide.c[2 * i] = gf.sqr(self.c[i]);
            }
        }
        wide.rem_monic(m, dm, gf)
    }
    /// Product modulo a monic `m` of degree `dm` (operands of degree < dm).
    fn mul_mod_monic(&self, o: &Poly, m: &Poly, dm: usize, gf: &Gf) -> Poly {
        let mut wide = Poly::zero();
        for i in 0..dm {
            if self.c[i] == 0 {
                continue;
            }
            for j in 0..dm {
                if o.c[j] == 0 {
                    continue;
                }
                wide.c[i + j] = gf.add(wide.c[i + j], gf.mul(self.c[i], o.c[j]));
            }
        }
        wide.rem_monic(m, dm, gf)
    }
    fn prem(&self, b: &Poly, gf: &Gf) -> Poly {
        let db = match b.deg() {
            Some(d) => d,
            None => return *self,
        };
        let lb = b.c[db];
        let mut r = *self;
        while let Some(dr) = r.deg() {
            if dr < db {
                break;
            }
            let lr = r.c[dr];
            r.scale_in_place(lb, dr, gf);
            for i in 0..=db {
                r.c[dr - db + i] = gf.add(r.c[dr - db + i], gf.mul(b.c[i], lr));
            }
            r.c[dr] = 0;
        }
        r
    }
    /// gcd up to a nonzero scalar via pseudo-remainders.
    fn gcd(&self, other: &Poly, gf: &Gf) -> Poly {
        let mut a = *self;
        let mut b = *other;
        loop {
            match b.deg() {
                None => return a,
                Some(0) => return b,
                Some(_) => {}
            }
            let r = a.prem(&b, gf);
            a = b;
            b = r;
        }
    }
}

// ------------------------------------------------------- subspace polynomials

/// Coefficients a_0..a_l of the linearized polynomial vanishing exactly on the
/// span of `basis` (uncounted setup).
fn subspace_poly_for_basis(basis: &[u64], gf: &Gf) -> Vec<u64> {
    let mut a = vec![1u64];
    for &v in basis {
        let mut lv = 0u64;
        for (j, &aj) in a.iter().enumerate() {
            let mut s = v;
            for _ in 0..j {
                s = gf.sqr_nc(s);
            }
            lv ^= gf.mul_nc(aj, s);
        }
        assert!(lv != 0, "subspace basis is not independent");
        let mut next = vec![0u64; a.len() + 1];
        for (j, &aj) in a.iter().enumerate() {
            next[j + 1] ^= gf.sqr_nc(aj);
            next[j] ^= gf.mul_nc(lv, aj);
        }
        a = next;
    }
    a
}

fn lin_eval_nc(lv: &[u64], x: u64, gf: &Gf) -> u64 {
    let mut acc = 0u64;
    let mut s = x;
    for &a in lv {
        acc ^= gf.mul_nc(a, s);
        s = gf.sqr_nc(s);
    }
    acc
}

/// `L_V` rearranged for Horner evaluation under `phi(u) = u^2 mod f`.
struct LvHorner {
    top: u64,
    lin: [u64; 2],
    steps: Vec<u64>,
}

impl LvHorner {
    /// Counted: the Frobenius inverse roots are `n - j` squarings each.
    fn new(lv: &[u64], gf: &Gf) -> Self {
        let n = gf.n;
        let root = |a: u64, j: usize| gf.sqr_k(a, (n - j as u32 % n) % n);
        let coef = |j: usize| lv.get(j).copied().unwrap_or(0);
        let len = lv.len().max(3);
        let h = len - 3;
        LvHorner {
            top: root(coef(len - 1), h),
            lin: [root(coef(len - 3), h), root(coef(len - 2), h)],
            steps: (0..h).rev().map(|j| root(coef(j), j)).collect(),
        }
    }
}

struct QuarticFrobenius {
    r4: [u64; 4],
    r6: [u64; 4],
}

impl QuarticFrobenius {
    fn new(f: [u64; 4], gf: &Gf) -> Self {
        let [f0, f1, f2, f3] = f;
        let k = gf.add(f2, gf.sqr(f3));
        let r6 = [
            gf.mul(k, f0),
            gf.dot2(k, f1, f3, f0),
            gf.add(gf.dot2(k, f2, f3, f1), f0),
            gf.add(gf.dot2(k, f3, f3, f2), f1),
        ];
        QuarticFrobenius { r4: f, r6 }
    }
    fn sqr(&self, u: [u64; 4], gf: &Gf) -> [u64; 4] {
        let (s2, s3) = (gf.sqr(u[2]), gf.sqr(u[3]));
        let (r4, r6) = (&self.r4, &self.r6);
        [
            gf.add3(gf.sqr(u[0]), gf.mul(s2, r4[0]), gf.mul(s3, r6[0])),
            gf.dot2(s2, r4[1], s3, r6[1]),
            gf.add3(gf.sqr(u[1]), gf.mul(s2, r4[2]), gf.mul(s3, r6[2])),
            gf.dot2(s2, r4[3], s3, r6[3]),
        ]
    }
}

/// `L mod f` for a monic quartic `f` by the Horner route, as `[u0..u3]`.
fn lv_mod_quartic_horner(
    f: [u64; 4],
    frob: &QuarticFrobenius,
    lvh: &LvHorner,
    gf: &Gf,
) -> [u64; 4] {
    let mut u = [
        gf.mul(lvh.top, f[0]),
        gf.add(gf.mul(lvh.top, f[1]), lvh.lin[0]),
        gf.add(gf.mul(lvh.top, f[2]), lvh.lin[1]),
        gf.mul(lvh.top, f[3]),
    ];
    for &b in &lvh.steps {
        u = frob.sqr(u, gf);
        u[1] = gf.add(u[1], b);
    }
    u
}

/// `L mod f` by the generic route (powers of t modulo f, one squaring and
/// accumulation per coefficient), as a `Poly` of degree < 4.
fn lv_mod_quartic_generic(f: [u64; 4], lv: &[u64], gf: &Gf) -> Poly {
    let monic = Poly::from_low(&[f[0], f[1], f[2], f[3], 1]);
    let mut t = Poly::zero();
    t.c[1] = 1;
    let mut acc = Poly::zero();
    let mut pow = t.rem_monic(&monic, 4, gf);
    for (i, &ai) in lv.iter().enumerate() {
        if ai != 0 {
            for j in 0..4 {
                acc.c[j] = gf.add(acc.c[j], gf.mul(pow.c[j], ai));
            }
        }
        if i + 1 < lv.len() {
            pow = pow.sqr_mod_monic(&monic, 4, gf);
        }
    }
    acc
}

/// gcd(f, g) for monic quartic `f` and `deg g <= 3`: the generic shape written
/// out (as in the incumbent), with the generic routine as fallback.
fn gcd_quartic_fast(f: [u64; 4], g: [u64; 4], gf: &Gf) -> Poly {
    let [f0, f1, f2, f3] = f;
    let [g0, g1, g2, g3] = g;
    let g3sq = gf.sqr(g3);
    let a3 = gf.add(gf.mul(f3, g3), g2);
    let c0 = gf.dot2(f0, g3sq, g0, a3);
    let c1 = gf.add3(gf.mul(f1, g3sq), gf.mul(g0, g3), gf.mul(g1, a3));
    let c2 = gf.add3(gf.mul(f2, g3sq), gf.mul(g1, g3), gf.mul(g2, a3));
    let b0 = gf.mul(g0, c2);
    let b1 = gf.dot2(g1, c2, c0, g3);
    let b2 = gf.dot2(g2, c2, c1, g3);
    let d0 = gf.dot2(b0, c2, c0, b2);
    let d1 = gf.dot2(b1, c2, c1, b2);
    let e0 = gf.mul(c0, d1);
    let e1 = gf.dot2(c1, d1, d0, c2);
    let h0 = gf.dot2(e0, d1, d0, e1);
    let mut out = Poly::zero();
    if g3 == 0 || a3 == 0 || c2 == 0 || b2 == 0 || d1 == 0 || e1 == 0 {
        return gcd_generic(f, g, gf);
    }
    if h0 != 0 {
        out.c[0] = h0;
    } else {
        out.c[0] = d0;
        out.c[1] = d1;
    }
    out
}

fn gcd_generic(f: [u64; 4], g: [u64; 4], gf: &Gf) -> Poly {
    let monic = Poly::from_low(&[f[0], f[1], f[2], f[3], 1]);
    let gp = Poly::from_low(&g);
    monic.gcd(&gp, gf)
}

// --------------------------------------------------------------- the quartic

/// Per-target constants of the quartic in `x3`.
struct Tp {
    xr: u64,
    xr2: u64,
    xr4: u64,
    b: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum QForm {
    General,
    B1,
    Naive,
}

/// Coefficients c0..c4 of `S4(x1, x2, t, x_R)` as a polynomial in `t`
/// (the incumbent's general-`b` form with shared subexpressions).
fn q_general(gf: &Gf, x1: u64, x2: u64, sq: Option<(u64, u64)>, t: &Tp) -> [u64; 5] {
    let s = gf.add(x1, x2);
    let p = gf.mul(x1, x2);
    let s2 = match sq {
        Some((a, b)) => gf.add(a, b),
        None => gf.sqr(s),
    };
    let p2 = gf.sqr(p);
    let bp2 = gf.add(t.b, p2);
    let s2x2 = gf.mul(s2, t.xr2);
    let px = gf.mul(p, t.xr);
    let p2x2 = gf.mul(p2, t.xr2);
    let inner = gf.add(p2x2, gf.mul(t.b, gf.add(s2, t.xr2)));
    let c4 = gf.add(gf.sqr(gf.add(s2x2, bp2)), p2x2);
    let c3 = gf.mul(px, gf.add(bp2, s2x2));
    let c2 = gf.add(gf.mul(p2, gf.add(t.b, t.xr4)), gf.mul(s2x2, bp2));
    let c1 = gf.mul(px, inner);
    let c0 = gf.add(gf.sqr(inner), gf.mul(p2x2, t.b));
    [c0, c1, c2, c3, c4]
}

/// The same quartic specialised to `b = 1` (two multiplications by `b` dropped).
fn q_b1(gf: &Gf, x1: u64, x2: u64, sq: Option<(u64, u64)>, t: &Tp) -> [u64; 5] {
    debug_assert_eq!(t.b, 1);
    let s = gf.add(x1, x2);
    let p = gf.mul(x1, x2);
    let s2 = match sq {
        Some((a, b)) => gf.add(a, b),
        None => gf.sqr(s),
    };
    let p2 = gf.sqr(p);
    let bp2 = gf.add(1, p2);
    let s2x2 = gf.mul(s2, t.xr2);
    let px = gf.mul(p, t.xr);
    let p2x2 = gf.mul(p2, t.xr2);
    let inner = gf.add3(p2x2, s2, t.xr2);
    let c4 = gf.add(gf.sqr(gf.add(s2x2, bp2)), p2x2);
    let c3 = gf.mul(px, gf.add(bp2, s2x2));
    let c2 = gf.add(gf.mul(p2, gf.add(1, t.xr4)), gf.mul(s2x2, bp2));
    let c1 = gf.mul(px, inner);
    let c0 = gf.add(gf.sqr(inner), p2x2);
    [c0, c1, c2, c3, c4]
}

/// The quartic straight from the resultant form
/// `S4 = (s^2 p'^2 + s'^2 p^2 + b (s^2 + s'^2))^2 + (s^2 p' + s'^2 p)(p + p')(p p' + b)`,
/// `s' = t + x_R`, `p' = t x_R`, expanded as polynomials in `t` (no shared
/// subexpressions beyond the obvious).
fn q_naive(gf: &Gf, x1: u64, x2: u64, _sq: Option<(u64, u64)>, t: &Tp) -> [u64; 5] {
    let s = gf.add(x1, x2);
    let p = gf.mul(x1, x2);
    let s2 = gf.sqr(s);
    let p2 = gf.sqr(p);
    // A = s2*(xr2 t^2) + p2*(xr2 + t^2) + b*(s2 + xr2 + t^2): A0, A2 (A1 = 0)
    let a0 = gf.add(gf.mul(p2, t.xr2), gf.mul(t.b, gf.add(s2, t.xr2)));
    let a2 = gf.add3(gf.mul(s2, t.xr2), p2, t.b);
    // A^2 = a0^2 + a2^2 t^4
    let a_sq0 = gf.sqr(a0);
    let a_sq4 = gf.sqr(a2);
    // B = s2*(xr t) + p*(xr2 + t^2) = [p xr2, s2 xr, p]
    let bb = [gf.mul(p, t.xr2), gf.mul(s2, t.xr), p];
    // C = p + xr t = [p, xr];  D = p xr t + b = [b, p xr]
    let cc = [p, t.xr];
    let dd = [t.b, gf.mul(p, t.xr)];
    // BC (degree 3)
    let bc = [
        gf.mul(bb[0], cc[0]),
        gf.dot2(bb[0], cc[1], bb[1], cc[0]),
        gf.dot2(bb[1], cc[1], bb[2], cc[0]),
        gf.mul(bb[2], cc[1]),
    ];
    // BCD (degree 4)
    let bcd = [
        gf.mul(bc[0], dd[0]),
        gf.dot2(bc[0], dd[1], bc[1], dd[0]),
        gf.dot2(bc[1], dd[1], bc[2], dd[0]),
        gf.dot2(bc[2], dd[1], bc[3], dd[0]),
        gf.mul(bc[3], dd[1]),
    ];
    [
        gf.add(a_sq0, bcd[0]),
        bcd[1],
        bcd[2],
        bcd[3],
        gf.add(a_sq4, bcd[4]),
    ]
}

/// Reference S4 value (uncounted), from the resultant form.
fn s4_ref(gf: &Gf, cv: &Curve, x1: u64, x2: u64, x3: u64, xr: u64) -> u64 {
    let m = |a, b| gf.mul_nc(a, b);
    let q = |a| gf.sqr_nc(a);
    let b = cv.b;
    // S3(x1,x2,X) = s^2 X^2 + p X + p^2 + b ; resultant eliminating the middle point
    let s = x1 ^ x2;
    let p = m(x1, x2);
    let s_ = x3 ^ xr;
    let p_ = m(x3, xr);
    let a = m(q(s), q(p_)) ^ m(q(s_), q(p)) ^ m(b, q(s) ^ q(s_));
    let c = m(q(s), p_) ^ m(q(s_), p);
    q(a) ^ m(m(c, p ^ p_), m(p, p_) ^ b)
}

// --------------------------------------------------------------- base families

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Sub,
    Coset(usize),
    Other,
}

const NFAM: usize = 24;

fn fam_name(f: usize) -> String {
    match f {
        0 => "sub-low".into(),
        1..=6 => format!("sub-rand{f}"),
        7..=10 => format!("coset2-s{}", f - 6),
        11..=12 => format!("coset4-s{}", f - 10),
        13 => "ham".into(),
        14 => "ham-shift".into(),
        15..=18 => format!("geom-s{}", f - 14),
        19..=20 => format!("affine-rand-s{}", f - 18),
        21..=22 => format!("affine-aligned-s{}", f - 20),
        23 => "affine-anti".into(),
        _ => panic!("family {f}"),
    }
}

fn fam_kind(f: usize) -> Kind {
    match f {
        0..=6 => Kind::Sub,
        7..=10 => Kind::Coset(2),
        11..=12 => Kind::Coset(4),
        19..=23 => Kind::Coset(1),
        _ => Kind::Other,
    }
}

struct Echelon {
    ech: Vec<(u32, u64)>,
}
impl Echelon {
    fn new() -> Self {
        Echelon { ech: vec![] }
    }
    fn reduce(&self, mut x: u64) -> u64 {
        for &(p, e) in &self.ech {
            if (x >> p) & 1 == 1 {
                x ^= e;
            }
        }
        x
    }
    fn insert(&mut self, v: u64) -> bool {
        let x = self.reduce(v);
        if x == 0 {
            return false;
        }
        let p = pdeg(x);
        self.ech.push((p, x));
        self.ech.sort_by(|a, b| b.0.cmp(&a.0));
        true
    }
}

fn span_all(basis: &[u64]) -> Vec<u64> {
    let l = basis.len();
    let mut out = Vec::with_capacity(1 << l);
    let mut cur = 0u64;
    out.push(0);
    for i in 1u64..(1u64 << l) {
        cur ^= basis[i.trailing_zeros() as usize];
        out.push(cur);
    }
    out
}

fn random_basis(gf: &Gf, dim: usize, rng: &mut Rng) -> (Vec<u64>, Echelon) {
    let mut ech = Echelon::new();
    let mut basis = vec![];
    while basis.len() < dim {
        let v = rng.next() & gf.mask;
        if ech.insert(v) {
            basis.push(v);
        }
    }
    (basis, ech)
}

/// One factor base: the abscissae in family order and everything the kernel
/// reads. All construction is uncounted setup.
struct BaseCtx {
    fam: usize,
    kind: Kind,
    xs: Vec<u64>,
    sq: Vec<u64>,
    rat: Vec<bool>,
    idx: HashMap<u64, usize>,
    rlist: Vec<u64>,
    rsq: Vec<u64>,
    pts: Vec<(u64, u64)>,
    /// Linearized polynomial of the (sub)space `U` (the whole subspace for `Sub`).
    lv: Vec<u64>,
    /// Coefficients e_0..e_k (low to high) of prod_i (z + L_U(c_i)) for the coset
    /// translates c_i (coset kinds).
    ys: Vec<u64>,
    /// Rational members with delta = Tr(x + a) equal to 0 and to 1.
    n0: usize,
    n1: usize,
    ell: usize,
}

impl BaseCtx {
    /// Multisets of three rational members with delta_1 + delta_2 + delta_3 = 0
    /// (the only ones that can sum into 2E, where every real target lives).
    fn t_eff(&self) -> f64 {
        let (a, b) = (self.n0 as f64, self.n1 as f64);
        a * (a + 1.0) * (a + 2.0) / 6.0 + a * b * (b + 1.0) / 2.0
    }
}

fn build_base(gf: &Gf, cv: &Curve, ell: usize, fam: usize, cell_seed: u64) -> BaseCtx {
    let kind = fam_kind(fam);
    let mut rng = Rng(mix3(cell_seed, 0xBA5E, fam as u64));
    let mut lv: Vec<u64> = vec![];
    let mut ys: Vec<u64> = vec![];
    let xs: Vec<u64> = match fam {
        0 => {
            let basis: Vec<u64> = (0..ell).map(|i| 1u64 << i).collect();
            lv = subspace_poly_for_basis(&basis, gf);
            span_all(&basis)
        }
        1..=6 => {
            let (basis, _) = random_basis(gf, ell, &mut rng);
            lv = subspace_poly_for_basis(&basis, gf);
            span_all(&basis)
        }
        7..=12 | 19..=23 => {
            let k = if fam <= 10 {
                2usize
            } else if fam <= 12 {
                4
            } else {
                1
            };
            let lk = k.trailing_zeros() as usize;
            // 21, 22, 23: U inside the trace-zero hyperplane
            let in_ker = fam >= 21;
            let mut ech = Echelon::new();
            let mut ub: Vec<u64> = vec![];
            while ub.len() < ell - lk {
                let v = rng.next() & gf.mask;
                if in_ker && gf.tr_nc(v) == 1 {
                    continue;
                }
                if ech.insert(v) {
                    ub.push(v);
                }
            }
            lv = subspace_poly_for_basis(&ub, gf);
            let uspan = span_all(&ub);
            let mut cs: Vec<u64> = vec![];
            if k == 1 {
                loop {
                    let c = rng.next() & gf.mask;
                    let want_tr = match fam {
                        21 | 22 => Some(1),
                        23 => Some(0),
                        _ => None,
                    };
                    if let Some(t) = want_tr {
                        if gf.tr_nc(c) != t {
                            continue;
                        }
                    }
                    if ech.reduce(c) != 0 {
                        cs.push(c);
                        break;
                    }
                }
            } else {
                cs.push(0);
                while cs.len() < k {
                    let c = rng.next() & gf.mask;
                    let ok = cs.iter().all(|&d| ech.reduce(c ^ d) != 0);
                    if ok {
                        cs.push(c);
                    }
                }
            }
            let y: Vec<u64> = cs.iter().map(|&c| lin_eval_nc(&lv, c, gf)).collect();
            // coefficients e_0..e_k of prod_i (z + y_i)
            let mut e = vec![1u64];
            for &yi in &y {
                let mut next = vec![0u64; e.len() + 1];
                for (j, &ej) in e.iter().enumerate() {
                    next[j + 1] ^= ej;
                    next[j] ^= gf.mul_nc(ej, yi);
                }
                e = next;
            }
            // low-to-high: e[0] = prod y_i, e[k] = 1
            ys = e;
            let mut out = Vec::with_capacity(1 << ell);
            for &c in &cs {
                for &u in &uspan {
                    out.push(u ^ c);
                }
            }
            out
        }
        13 | 14 => {
            let shift = if fam == 14 { rng.next() & gf.mask } else { 0 };
            let want = 1usize << ell;
            let mut out: Vec<u64> = Vec::with_capacity(want);
            let n = gf.n;
            'outer: for w in 0..=n {
                // all n-bit words of weight w in increasing numeric order (Gosper)
                if w == 0 {
                    out.push(shift);
                    if out.len() == want {
                        break;
                    }
                    continue;
                }
                let mut v: u64 = (1u64 << w) - 1;
                while v < (1u64 << n) {
                    out.push(v ^ shift);
                    if out.len() == want {
                        break 'outer;
                    }
                    let tz = v & v.wrapping_neg();
                    let ripple = v + tz;
                    v = (((ripple ^ v) >> 2) / tz) | ripple;
                }
            }
            out
        }
        15..=18 => {
            let g = loop {
                let g = rng.next() & gf.mask;
                if g > 1 {
                    break g;
                }
            };
            let mut out = Vec::with_capacity(1 << ell);
            let mut s = 1u64;
            for _ in 0..(1usize << ell) {
                out.push(s);
                s = gf.mul_nc(s, g);
            }
            out
        }
        _ => panic!("family {fam}"),
    };
    assert_eq!(xs.len(), 1usize << ell, "family {fam} size");
    let mut idx = HashMap::new();
    for (i, &x) in xs.iter().enumerate() {
        assert!(
            idx.insert(x, i).is_none(),
            "family {fam} has a repeated abscissa"
        );
    }
    let sq: Vec<u64> = xs.iter().map(|&x| gf.sqr_nc(x)).collect();
    let mut rat = Vec::with_capacity(xs.len());
    let (mut rlist, mut rsq, mut pts) = (vec![], vec![], vec![]);
    for (i, &x) in xs.iter().enumerate() {
        match point_at(gf, cv, x) {
            Some(p) => {
                rat.push(true);
                rlist.push(x);
                rsq.push(sq[i]);
                pts.push(p);
            }
            None => rat.push(false),
        }
    }
    let n0 = pts.iter().filter(|p| gf.tr_nc(p.0 ^ cv.a) == 0).count();
    let n1 = pts.len() - n0;
    BaseCtx {
        fam,
        kind,
        n0,
        n1,
        xs,
        sq,
        rat,
        idx,
        rlist,
        rsq,
        pts,
        lv,
        ys,
        ell,
    }
}

// ------------------------------------------------------------------- programs

/// Gene vector. Index meanings and cardinalities in `GENE_CARD`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Prog {
    g: [u8; 10],
}

const G_FAM: usize = 0;
const G_LIFT: usize = 1; // 0 off, 1 prefilter x1,x2 to rational members
const G_STOP: usize = 2; // 0 stop at first raw witness, 1 continue until lifted
const G_ROOT: usize = 3; // 0 horner, 1 generic, 2 eval
const G_QUART: usize = 4; // 0 general, 1 b1, 2 naive
const G_SQT: usize = 5; // 0 off, 1 square table
const G_PAIRS: usize = 6; // 0 sorted, 1 full
const G_ABORT: usize = 7; // row fraction index
const G_BATCH: usize = 8; // 0 row, 1 block of 8 rows, 2 per pair
const G_GCD: usize = 9; // 0 fast, 1 generic

const GENE_CARD: [u8; 10] = [NFAM as u8, 2, 2, 3, 3, 2, 2, 5, 3, 2];
const GENE_NAME: [&str; 10] = [
    "fam", "lift", "stop", "root", "quartic", "sqtab", "pairs", "abort", "batch", "gcd",
];
const ABORT_FRAC: [f64; 5] = [1.0, 0.75, 0.5, 0.25, 0.125];

impl Prog {
    fn incumbent() -> Prog {
        Prog { g: [0; 10] }
    }
    /// Replace genes that cannot change behaviour by 0.
    fn canon(mut self) -> Prog {
        if self.g[G_ROOT] == 2 {
            self.g[G_BATCH] = 0;
            self.g[G_GCD] = 0;
        }
        if self.g[G_ROOT] == 1 {
            self.g[G_GCD] = 0;
        }
        if self.g[G_QUART] == 2 {
            self.g[G_SQT] = 0;
        }
        self
    }
    fn valid(&self) -> bool {
        for i in 0..10 {
            if self.g[i] >= GENE_CARD[i] {
                return false;
            }
        }
        let kind = fam_kind(self.g[G_FAM] as usize);
        if self.g[G_ROOT] != 2 && kind == Kind::Other {
            return false;
        }
        true
    }
    fn name(&self) -> String {
        format!(
            "{}|lift{}|stop{}|root{}|q{}|sqt{}|pm{}|ab{}|bt{}|gc{}",
            fam_name(self.g[0] as usize),
            self.g[1],
            self.g[2],
            self.g[3],
            self.g[4],
            self.g[5],
            self.g[6],
            self.g[7],
            self.g[8],
            self.g[9]
        )
    }
    fn json_genes(&self) -> String {
        let v: Vec<String> = self.g.iter().map(|x| x.to_string()).collect();
        format!("[{}]", v.join(","))
    }
    fn from_genes(v: &[u8]) -> Prog {
        let mut g = [0u8; 10];
        g.copy_from_slice(v);
        Prog { g }
    }
}

fn random_prog(rng: &mut Rng) -> Prog {
    loop {
        let mut g = [0u8; 10];
        for i in 0..10 {
            g[i] = rng.below(GENE_CARD[i] as u64) as u8;
        }
        let p = Prog { g }.canon();
        if p.valid() {
            return p;
        }
    }
}

fn mutate(parent: &Prog, rng: &mut Rng) -> Prog {
    loop {
        let mut g = parent.g;
        let k = if rng.unit() < 0.7 { 1 } else { 2 };
        for _ in 0..k {
            let i = rng.below(10) as usize;
            if GENE_CARD[i] > 1 {
                loop {
                    let v = rng.below(GENE_CARD[i] as u64) as u8;
                    if v != g[i] {
                        g[i] = v;
                        break;
                    }
                }
            }
        }
        let p = Prog { g }.canon();
        if p.valid() && p != *parent {
            return p;
        }
    }
}

// ------------------------------------------------------------------ the kernel

struct CellCtx {
    gf: Gf,
    cv: Curve,
    n: u32,
    ell: usize,
    seed: u64,
}

struct ScanRes {
    found: bool,
    aborted: bool,
    /// The accepted witness (x1, x2, x3), if any.
    witness: Option<[u64; 3]>,
}

/// Roots (as abscissae of the base) of the quartic `q` (raw, possibly
/// non-monic or degenerate) found by evaluating over `domain`.
fn eval_roots(gf: &Gf, q: &[u64; 5], domain: &[u64], out: &mut Vec<u64>) {
    out.clear();
    for &t in domain {
        let mut v = q[4];
        for k in (0..4).rev() {
            v = gf.add(gf.mul(v, t), q[k]);
        }
        if v == 0 {
            out.push(t);
        }
    }
}

/// Run program `prog` on the target abscissa `xr` over the base `bc`.
/// `budget` is a cap on the priced-at-1 operations of this call (0 = none).
fn scan(cell: &CellCtx, bc: &BaseCtx, prog: &Prog, xr: u64, budget: u64) -> ScanRes {
    let gf = &cell.gf;
    let b = cell.cv.b;
    let start = gf.snap();
    let xr2 = gf.sqr(xr);
    let xr4 = gf.sqr(xr2);
    let tp = Tp { xr, xr2, xr4, b };
    let lift = prog.g[G_LIFT] == 1;
    let until_lift = prog.g[G_STOP] == 1;
    let root = prog.g[G_ROOT];
    let qform = match prog.g[G_QUART] {
        0 => QForm::General,
        1 => QForm::B1,
        _ => QForm::Naive,
    };
    let use_sq = prog.g[G_SQT] == 1;
    let full_pairs = prog.g[G_PAIRS] == 1;
    let batch = prog.g[G_BATCH];
    let fast_gcd = prog.g[G_GCD] == 0;
    let (list, lsq): (&[u64], &[u64]) = if lift {
        (&bc.rlist, &bc.rsq)
    } else {
        (&bc.xs, &bc.sq)
    };
    let m = list.len();
    let rows = (((m as f64) * ABORT_FRAC[prog.g[G_ABORT] as usize]).ceil() as usize).clamp(1, m);
    // Domain over which Eval looks for roots: the lifted list, or the base.
    let eval_domain: &[u64] = if lift { &bc.rlist } else { &bc.xs };
    let lvh = if root != 2 && !bc.lv.is_empty() {
        Some(LvHorner::new(&bc.lv, gf))
    } else {
        None
    };
    let chunk_rows = if batch == 1 { 8 } else { 1 };

    let mut qs: Vec<[u64; 5]> = Vec::new();
    let mut meta: Vec<(usize, usize)> = Vec::new();
    let mut leads: Vec<u64> = Vec::new();
    let mut lead_of: Vec<usize> = Vec::new();
    let mut scratch: Vec<u64> = Vec::new();
    let mut roots: Vec<u64> = Vec::new();
    let mut r0 = 0usize;
    while r0 < rows {
        let r1 = (r0 + chunk_rows).min(rows);
        qs.clear();
        meta.clear();
        leads.clear();
        lead_of.clear();
        for r in r0..r1 {
            let x1 = list[r];
            let lo = if full_pairs { 0 } else { r };
            for c in lo..m {
                let x2 = list[c];
                gf.c_pairs.set(gf.c_pairs.get() + 1);
                let sq = if use_sq { Some((lsq[r], lsq[c])) } else { None };
                let q = match qform {
                    QForm::General => q_general(gf, x1, x2, sq, &tp),
                    QForm::B1 => q_b1(gf, x1, x2, sq, &tp),
                    QForm::Naive => q_naive(gf, x1, x2, sq, &tp),
                };
                if q[4] != 0 && root != 2 {
                    lead_of.push(leads.len());
                    leads.push(q[4]);
                } else {
                    lead_of.push(usize::MAX);
                }
                qs.push(q);
                meta.push((r, c));
            }
        }
        if root != 2 && batch != 2 {
            gf.batch_inv(&mut leads, &mut scratch);
        }
        for j in 0..qs.len() {
            let q = qs[j];
            let (r, c) = meta[j];
            let (x1, x2) = (list[r], list[c]);
            roots.clear();
            if root == 2 {
                eval_roots(gf, &q, eval_domain, &mut roots);
            } else if q[4] == 0 {
                // degenerate (lower-degree) quartic: evaluate over the base
                let dom: &[u64] = if lift { &bc.rlist } else { &bc.xs };
                eval_roots(gf, &q, dom, &mut roots);
            } else {
                let li = if batch == 2 {
                    gf.inv(q[4])
                } else {
                    leads[lead_of[j]]
                };
                let f = [
                    gf.mul(q[0], li),
                    gf.mul(q[1], li),
                    gf.mul(q[2], li),
                    gf.mul(q[3], li),
                ];
                let g = base_roots(gf, bc, lvh.as_ref(), f, root, fast_gcd);
                match g.deg() {
                    None | Some(0) => {}
                    Some(1) => {
                        roots.push(gf.mul(g.c[0], gf.inv(g.c[1])));
                    }
                    Some(_) => {
                        // several base roots: find them by evaluation over the base
                        for &t in &bc.xs {
                            let mut v = 0u64;
                            for k in (0..=MAXD).rev() {
                                v = gf.add(gf.mul(v, t), g.c[k]);
                            }
                            if v == 0 {
                                roots.push(t);
                            }
                        }
                    }
                }
            }
            for &x3 in roots.iter() {
                // a raw witness: the caller's lift check (three table probes)
                gf.c_probe.set(gf.c_probe.get() + 3);
                let ok = match bc.idx.get(&x3) {
                    Some(&i3) => {
                        bc.rat[i3] && {
                            let i1 = bc.idx[&x1];
                            let i2 = bc.idx[&x2];
                            bc.rat[i1] && bc.rat[i2]
                        }
                    }
                    None => false,
                };
                if ok {
                    return ScanRes {
                        found: true,
                        aborted: false,
                        witness: Some([x1, x2, x3]),
                    };
                }
                if !until_lift {
                    return ScanRes {
                        found: false,
                        aborted: false,
                        witness: None,
                    };
                }
            }
        }
        r0 = r1;
        if budget > 0 {
            let used = gf.snap().sub(start);
            if used.priced(1.0) > budget as f64 {
                return ScanRes {
                    found: false,
                    aborted: true,
                    witness: None,
                };
            }
        }
    }
    ScanRes {
        found: false,
        aborted: false,
        witness: None,
    }
}

/// The polynomial whose roots are the roots of the monic quartic `f` that lie in
/// the base (a gcd, up to a scalar).
fn base_roots(
    gf: &Gf,
    bc: &BaseCtx,
    lvh: Option<&LvHorner>,
    f: [u64; 4],
    root: u8,
    fast: bool,
) -> Poly {
    let u: [u64; 4] = if root == 0 {
        let lvh = lvh.expect("horner needs a subspace polynomial");
        let frob = QuarticFrobenius::new(f, gf);
        let u = lv_mod_quartic_horner(f, &frob, lvh, gf);
        match bc.kind {
            Kind::Sub => u,
            Kind::Coset(_) => coset_product(f, u, &bc.ys, Some(&frob), gf),
            Kind::Other => unreachable!(),
        }
    } else {
        let p = lv_mod_quartic_generic(f, &bc.lv, gf);
        let u = [p.c[0], p.c[1], p.c[2], p.c[3]];
        match bc.kind {
            Kind::Sub => u,
            Kind::Coset(_) => coset_product(f, u, &bc.ys, None, gf),
            Kind::Other => unreachable!(),
        }
    };
    if root == 1 || !fast {
        gcd_generic(f, u, gf)
    } else {
        gcd_quartic_fast(f, u, gf)
    }
}

/// `P(u) = sum_j e_j u^j mod f` where `e` are the coefficients of
/// `prod_i (z + y_i)`: the polynomial in `t` vanishing on the union of cosets
/// when `u = L_U(t) mod f`. Constants known at setup (a zero `e_j`) cost nothing.
fn coset_product(
    f: [u64; 4],
    u: [u64; 4],
    es: &[u64],
    frob: Option<&QuarticFrobenius>,
    gf: &Gf,
) -> [u64; 4] {
    let k = es.len() - 1;
    let monic = Poly::from_low(&[f[0], f[1], f[2], f[3], 1]);
    let sqf = |v: [u64; 4]| -> [u64; 4] {
        match frob {
            Some(fr) => fr.sqr(v, gf),
            None => {
                let s = Poly::from_low(&v).sqr_mod_monic(&monic, 4, gf);
                [s.c[0], s.c[1], s.c[2], s.c[3]]
            }
        }
    };
    let scal = |v: [u64; 4], c: u64| -> [u64; 4] {
        if c == 0 {
            [0; 4]
        } else {
            [
                gf.mul(v[0], c),
                gf.mul(v[1], c),
                gf.mul(v[2], c),
                gf.mul(v[3], c),
            ]
        }
    };
    let addv = |a: [u64; 4], b: [u64; 4]| -> [u64; 4] {
        [
            gf.add(a[0], b[0]),
            gf.add(a[1], b[1]),
            gf.add(a[2], b[2]),
            gf.add(a[3], b[3]),
        ]
    };
    let mut r = match k {
        1 => u,
        2 => addv(sqf(u), scal(u, es[1])),
        4 => {
            let u2 = sqf(u);
            let u4 = sqf(u2);
            let u3p = Poly::from_low(&u2).mul_mod_monic(&Poly::from_low(&u), &monic, 4, gf);
            let u3 = [u3p.c[0], u3p.c[1], u3p.c[2], u3p.c[3]];
            addv(
                addv(u4, scal(u3, es[3])),
                addv(scal(u2, es[2]), scal(u, es[1])),
            )
        }
        _ => unreachable!(),
    };
    if es[0] != 0 {
        r[0] = gf.add(r[0], es[0]);
    }
    r
}

// ------------------------------------------------------------ target sets

struct Targets {
    neg: Vec<u64>,
}

/// Natural S-population targets: x(2Q) for random rational Q, x != 0.
fn gen_neg(cell: &CellCtx, count: usize, seed: u64) -> Targets {
    let gf = &cell.gf;
    let mut rng = Rng(seed);
    let mut neg = vec![];
    while neg.len() < count {
        let q = random_point(gf, &cell.cv, &mut rng);
        if let Some((x, _)) = add_pt(gf, &cell.cv, Some(q), Some(q)) {
            if x != 0 {
                neg.push(x);
            }
        }
    }
    Targets { neg }
}

/// Planted targets over a base: x(P1 +- P2 +- P3) for rational members, with
/// the sum in 2E (delta = 0) and x_R not itself a base member.
fn gen_pos(cell: &CellCtx, bc: &BaseCtx, count: usize, seed: u64) -> Vec<u64> {
    let gf = &cell.gf;
    let mut rng = Rng(seed);
    let mut out = vec![];
    let m = bc.pts.len() as u64;
    let mut guard = 0u64;
    while out.len() < count {
        guard += 1;
        assert!(guard < 1_000_000, "cannot plant targets");
        let pick = |rng: &mut Rng| {
            let (x, y) = bc.pts[rng.below(m) as usize];
            if rng.next() & 1 == 1 {
                (x, y ^ x)
            } else {
                (x, y)
            }
        };
        let (p1, p2, p3) = (pick(&mut rng), pick(&mut rng), pick(&mut rng));
        let s = add_pt(gf, &cell.cv, Some(p1), Some(p2));
        let r = add_pt(gf, &cell.cv, s, Some(p3));
        if let Some((x, _)) = r {
            if x != 0 && gf.tr_nc(x ^ cell.cv.a) == 0 && !bc.idx.contains_key(&x) {
                out.push(x);
            }
        }
    }
    out
}

// ------------------------------------------------------------------ evaluation

struct EvalRes {
    prog: Prog,
    b_r: usize,
    t_eff: f64,
    neg: Vec<(bool, Ops)>,
    pos: Vec<(bool, Ops)>,
    dominated: bool,
    wall_ms: u64,
}

impl EvalRes {
    fn phi(&self) -> f64 {
        if self.pos.is_empty() {
            return 0.0;
        }
        self.pos.iter().filter(|p| p.0).count() as f64 / self.pos.len() as f64
    }
    /// Collection cost index at pricing weight w.
    fn q(&self, w: f64) -> f64 {
        if self.dominated || self.neg.is_empty() {
            return f64::INFINITY;
        }
        let phi = self.phi();
        if phi <= 0.0 {
            return f64::INFINITY;
        }
        let c0: f64 = self.neg.iter().map(|o| o.1.priced(w)).sum::<f64>() / self.neg.len() as f64;
        if self.t_eff <= 0.0 {
            return f64::INFINITY;
        }
        c0 * self.b_r as f64 / (self.t_eff * phi)
    }
    fn mean_priced_pos(&self, w: f64) -> f64 {
        if self.pos.is_empty() {
            return 0.0;
        }
        self.pos.iter().map(|p| p.1.priced(w)).sum::<f64>() / self.pos.len() as f64
    }
}

struct BaseCache {
    map: HashMap<usize, BaseCtx>,
    pos_cache: HashMap<(usize, u64, usize), Vec<u64>>,
}

impl BaseCache {
    fn new() -> Self {
        BaseCache {
            map: HashMap::new(),
            pos_cache: HashMap::new(),
        }
    }
    fn base<'a>(&'a mut self, cell: &CellCtx, fam: usize) -> &'a BaseCtx {
        self.map
            .entry(fam)
            .or_insert_with(|| build_base(&cell.gf, &cell.cv, cell.ell, fam, cell.seed))
    }
    fn pos<'a>(&'a mut self, cell: &CellCtx, fam: usize, set_seed: u64, count: usize) -> Vec<u64> {
        let key = (fam, set_seed, count);
        if let Some(v) = self.pos_cache.get(&key) {
            return v.clone();
        }
        self.base(cell, fam);
        let bc = &self.map[&fam];
        let v = gen_pos(cell, bc, count, mix3(set_seed, 0x9051, fam as u64));
        self.pos_cache.insert(key, v.clone());
        v
    }
}

/// Evaluate a program on the given negative targets and the planted positives
/// of its base. `cap` is a per-target ops cap (0 = none): exceeding it on any
/// target marks the evaluation dominated.
fn eval_prog(
    cell: &CellCtx,
    cache: &mut BaseCache,
    prog: &Prog,
    negs: &[u64],
    pos_seed: u64,
    npos: usize,
    cap: u64,
) -> EvalRes {
    let t0 = Instant::now();
    let fam = prog.g[G_FAM] as usize;
    cache.base(cell, fam);
    let feasible = cache.map[&fam].t_eff() > 0.0;
    let poss = if feasible {
        cache.pos(cell, fam, pos_seed, npos)
    } else {
        vec![]
    };
    let bc = &cache.map[&fam];
    let gf = &cell.gf;
    let mut res = EvalRes {
        prog: *prog,
        b_r: bc.rlist.len(),
        t_eff: bc.t_eff(),
        neg: vec![],
        pos: vec![],
        dominated: false,
        wall_ms: 0,
    };
    'run: {
        if !feasible {
            // no multiset of base members can sum into 2E: zero yield, cost per relation unbounded
            break 'run;
        }
        for &x in negs {
            let s0 = gf.snap();
            let r = scan(cell, bc, prog, x, cap);
            res.neg.push((r.found, gf.snap().sub(s0)));
            if r.aborted {
                res.dominated = true;
                break 'run;
            }
        }
        for &x in &poss {
            let s0 = gf.snap();
            let r = scan(cell, bc, prog, x, cap);
            res.pos.push((r.found, gf.snap().sub(s0)));
            if r.aborted {
                res.dominated = true;
                break 'run;
            }
        }
    }
    res.wall_ms = t0.elapsed().as_millis() as u64;
    res
}

// ---------------------------------------------------------------------- JSON

fn ops_json(o: &Ops) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        o.mul, o.sqr, o.lin, o.probe, o.trace, o.pairs
    )
}

fn f64j(x: f64) -> String {
    if x.is_finite() {
        format!("{:.6e}", x)
    } else {
        "null".into()
    }
}

fn eval_json(e: &EvalRes, extra: &str) -> String {
    let negs: Vec<String> = e
        .neg
        .iter()
        .map(|p| format!("[{},{}]", if p.0 { 1 } else { 0 }, ops_json(&p.1)))
        .collect();
    let poss: Vec<String> = e
        .pos
        .iter()
        .map(|p| format!("[{},{}]", if p.0 { 1 } else { 0 }, ops_json(&p.1)))
        .collect();
    format!(
        "{{{}\"genes\":{},\"name\":\"{}\",\"b_r\":{},\"t_eff\":{},\"dominated\":{},\"phi\":{},\"q0\":{},\"q01\":{},\"q1\":{},\"wall_ms\":{},\"neg\":[{}],\"pos\":[{}]}}",
        extra,
        e.prog.json_genes(),
        e.prog.name(),
        e.b_r,
        f64j(e.t_eff),
        e.dominated,
        f64j(e.phi()),
        f64j(e.q(0.0)),
        f64j(e.q(0.1)),
        f64j(e.q(1.0)),
        e.wall_ms,
        negs.join(","),
        poss.join(",")
    )
}

// ------------------------------------------------------ minimal JSON scanning

/// Extract the raw text of top-level `"key":value` where value is a number,
/// string, or array (balanced), from one flat-ish JSON line.
fn jfield<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let start = line.find(&pat)? + pat.len();
    let rest = &line[start..];
    let bytes = rest.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let end = match bytes[0] {
        b'[' => {
            let mut depth = 0;
            let mut e = 0;
            for (i, &c) in bytes.iter().enumerate() {
                if c == b'[' {
                    depth += 1;
                } else if c == b']' {
                    depth -= 1;
                    if depth == 0 {
                        e = i + 1;
                        break;
                    }
                }
            }
            e
        }
        b'"' => rest[1..].find('"').map(|i| i + 2)?,
        _ => rest.find(|c| c == ',' || c == '}').unwrap_or(rest.len()),
    };
    Some(&rest[..end])
}

fn parse_nums(s: &str) -> Vec<f64> {
    s.split(|c: char| {
        !(c.is_ascii_digit() || c == '.' || c == '-' || c == 'e' || c == 'E' || c == '+')
    })
    .filter(|t| !t.is_empty())
    .filter_map(|t| t.parse::<f64>().ok())
    .collect()
}

// ------------------------------------------------------------------ cell setup

fn make_cell(n: u32, ell: usize, seed: u64) -> CellCtx {
    CellCtx {
        gf: Gf::new(n),
        cv: Curve { a: 1, b: 1 },
        n,
        ell,
        seed,
    }
}

// -------------------------------------------------------------------- selftest

/// All unordered rational triples (repeats allowed) with S4 = 0 for `xr`.
fn brute_exists(cell: &CellCtx, bc: &BaseCtx, xr: u64) -> bool {
    let r = &bc.rlist;
    let m = r.len();
    for i in 0..m {
        for j in i..m {
            for k in j..m {
                if s4_ref(&cell.gf, &cell.cv, r[i], r[j], r[k], xr) == 0 {
                    return true;
                }
            }
        }
    }
    false
}

fn check(name: &str, ok: bool, detail: &str) -> bool {
    println!("{} {} {}", if ok { "PASS" } else { "FAIL" }, name, detail);
    ok
}

fn selftest(args: &Args) -> bool {
    let mut all = true;
    let t0 = Instant::now();
    // 1. field and curve at every cell size
    for &n in &[13u32, 17, 31, 37, 41] {
        let cell = make_cell(n, 6, 1);
        let gf = &cell.gf;
        let mut rng = Rng(77 + n as u64);
        let mut ok = is_irreducible((1u64 << n) | gf.tail);
        for _ in 0..200 {
            let a = (rng.next() & gf.mask).max(1);
            let b = rng.next() & gf.mask;
            ok &= gf.mul_nc(a, gf.inv_nc(a)) == 1;
            ok &= gf.mul_nc(a, b) == gf.mul_nc(b, a);
            ok &= gf.sqr_nc(a) == gf.mul_nc(a, a);
            ok &= gf.mul_nc(a, b ^ 5) == gf.mul_nc(a, b) ^ gf.mul_nc(a, 5);
        }
        all &= check(
            &format!("field n={n}"),
            ok,
            &format!("tail=0x{:x}", gf.tail),
        );
        // counted inversion is a pure mul/sqr chain
        let s0 = gf.snap();
        let a = (rng.next() & gf.mask).max(1);
        let ai = gf.inv(a);
        let d = gf.snap().sub(s0);
        all &= check(
            &format!("inv n={n}"),
            gf.mul_nc(a, ai) == 1 && d.lin == 0,
            &format!("mul={} sqr={}", d.mul, d.sqr),
        );
        let ord = koblitz_order(n);
        let mut ok = true;
        for _ in 0..12 {
            let q = random_point(gf, &cell.cv, &mut rng);
            ok &= on_curve(gf, &cell.cv, q);
            ok &= scalar_mul(gf, &cell.cv, Some(q), ord).is_none();
        }
        all &= check(
            &format!("curve n={n}"),
            ok,
            &format!("#E=2^{n}+1-V_n={ord} (mod 4 = {})", ord % 4),
        );
        all &= check(
            &format!("2E=ker(delta) n={n}"),
            ord % 4 == 2,
            "cyclic 2-part Z/2",
        );
    }
    // 2. quartic forms agree with each other and with the resultant S4
    {
        let cell = make_cell(31, 6, 1);
        let gf = &cell.gf;
        let mut rng = Rng(5);
        let mut ok = true;
        for _ in 0..300 {
            let x1 = rng.next() & gf.mask;
            let x2 = rng.next() & gf.mask;
            let xr = (rng.next() & gf.mask).max(1);
            let xr2 = gf.sqr_nc(xr);
            let tp = Tp {
                xr,
                xr2,
                xr4: gf.sqr_nc(xr2),
                b: 1,
            };
            let a = q_general(gf, x1, x2, None, &tp);
            let b = q_b1(gf, x1, x2, None, &tp);
            let c = q_naive(gf, x1, x2, None, &tp);
            let d = q_b1(gf, x1, x2, Some((gf.sqr_nc(x1), gf.sqr_nc(x2))), &tp);
            ok &= a == b && a == c && a == d;
            let t = rng.next() & gf.mask;
            let mut v = 0u64;
            for k in (0..5).rev() {
                v = gf.mul_nc(v, t) ^ a[k];
            }
            ok &= v == s4_ref(gf, &cell.cv, x1, x2, t, xr);
            ok &= s4_ref(gf, &cell.cv, x1, x2, t, xr) == s4_ref(gf, &cell.cv, t, x2, x1, xr);
        }
        all &= check(
            "quartic forms == resultant S4 (n=31)",
            ok,
            "300 random (x1,x2,xr,t)",
        );
    }
    // 3. S4 vanishes on true sums
    for &n in &[17u32, 31, 41] {
        let cell = make_cell(n, 6, 1);
        let gf = &cell.gf;
        let mut rng = Rng(9 + n as u64);
        let mut ok = true;
        for _ in 0..100 {
            let p1 = random_point(gf, &cell.cv, &mut rng);
            let p2 = random_point(gf, &cell.cv, &mut rng);
            let p3 = random_point(gf, &cell.cv, &mut rng);
            for s2 in 0..2 {
                for s3 in 0..2 {
                    let q2 = if s2 == 1 { neg(Some(p2)) } else { Some(p2) };
                    let q3 = if s3 == 1 { neg(Some(p3)) } else { Some(p3) };
                    let r = add_pt(gf, &cell.cv, add_pt(gf, &cell.cv, Some(p1), q2), q3);
                    if let Some((xr, _)) = r {
                        ok &= s4_ref(gf, &cell.cv, p1.0, p2.0, p3.0, xr) == 0;
                    }
                }
            }
        }
        all &= check(
            &format!("S4(x1,x2,x3,x(P1+-P2+-P3))=0 n={n}"),
            ok,
            "100 triples x 4 signs",
        );
    }
    // 4. family construction, at the three cell sizes
    for &nn in &[31u32, 37, 41] {
        let cell = make_cell(nn, 9, cell_seed(nn));
        let mut ok = true;
        let mut det = String::new();
        for fam in 0..NFAM {
            let bc = build_base(&cell.gf, &cell.cv, 9, fam, cell.seed);
            ok &= bc.xs.len() == 512;
            if matches!(bc.kind, Kind::Sub) {
                ok &= bc.xs.iter().all(|&x| lin_eval_nc(&bc.lv, x, &cell.gf) == 0);
            }
            if let Kind::Coset(_) = bc.kind {
                // every base member is a root of sum_j e_j L_U(t)^j = prod (L_U(t) + y_i)
                let g = &cell.gf;
                ok &= bc.xs.iter().all(|&x| {
                    let u = lin_eval_nc(&bc.lv, x, g);
                    let mut v = 0u64;
                    for &ej in bc.ys.iter().rev() {
                        v = g.mul_nc(v, u) ^ ej;
                    }
                    v == 0
                });
            }
            det += &format!(" {}:{}", fam_name(fam), bc.rlist.len());
        }
        all &= check(
            &format!("families n={nn} l=9 size/membership"),
            ok,
            &format!("rational counts{det}"),
        );
    }
    // 4b. trace alignment and the feasible-triple count
    {
        let cell = make_cell(17, 6, cell_seed(17));
        let mut ok = true;
        let mut det = String::new();
        for fam in 0..NFAM {
            let bc = build_base(&cell.gf, &cell.cv, 6, fam, cell.seed);
            let deltas: Vec<u64> = bc
                .rlist
                .iter()
                .map(|&x| cell.gf.tr_nc(x ^ cell.cv.a))
                .collect();
            let m = deltas.len();
            let mut cnt = 0u64;
            for i in 0..m {
                for j in i..m {
                    for k in j..m {
                        if (deltas[i] + deltas[j] + deltas[k]) % 2 == 0 {
                            cnt += 1;
                        }
                    }
                }
            }
            ok &= cnt as f64 == bc.t_eff();
            if (21..=22).contains(&fam) {
                ok &= bc.n1 == 0 && bc.n0 == m;
            }
            if fam == 23 {
                ok &= bc.n0 == 0 && bc.t_eff() == 0.0;
            }
            det += &format!(" {}:{}/{}", fam_name(fam), bc.n0, bc.n1);
        }
        all &= check(
            "delta classes and T_eff (n=17 l=6)",
            ok,
            &format!("n0/n1{det}"),
        );
    }
    // 5. kernel vs brute force over random programs, small cell
    {
        let cell = make_cell(17, 6, 11);
        let mut cache = BaseCache::new();
        let mut rng = Rng(123);
        let negs = gen_neg(&cell, 6, 999).neg;
        let mut bad = 0;
        let mut exact_checked = 0;
        let mut found_total = 0;
        let nprog = args.num("--progs", 400usize);
        for _ in 0..nprog {
            let prog = random_prog(&mut rng);
            let fam = prog.g[G_FAM] as usize;
            cache.base(&cell, fam);
            let poss = if cache.map[&fam].t_eff() > 0.0 {
                cache.pos(&cell, fam, 4242, 4)
            } else {
                vec![]
            };
            let bc = &cache.map[&fam];
            for &x in negs.iter().chain(poss.iter()) {
                let exists = brute_exists(&cell, bc, x);
                let r = scan(&cell, bc, &prog, x, 0);
                if r.found {
                    found_total += 1;
                    let w = r.witness.unwrap();
                    let valid = s4_ref(&cell.gf, &cell.cv, w[0], w[1], w[2], x) == 0
                        && w.iter()
                            .all(|v| bc.idx.get(v).map(|&i| bc.rat[i]).unwrap_or(false));
                    if !valid || !exists {
                        bad += 1;
                        println!("  bad witness {} target {x}", prog.name());
                    }
                }
                // exact programs (full scan, continue until lifted) must agree with brute force
                if prog.g[G_STOP] == 1 && prog.g[G_ABORT] == 0 {
                    exact_checked += 1;
                    if r.found != exists {
                        bad += 1;
                        println!(
                            "  decision mismatch {} target {x}: kernel {} brute {}",
                            prog.name(),
                            r.found,
                            exists
                        );
                    }
                }
            }
        }
        all &= check(
            "kernel == brute force (n=17 l=6, random programs)",
            bad == 0,
            &format!("{nprog} programs, exact decisions checked {exact_checked}, witnesses {found_total}, bad {bad}"),
        );
    }
    // 6. cost positive control: handicaps cost what they must
    {
        let cell = make_cell(31, 9, 21);
        let mut cache = BaseCache::new();
        let negs = gen_neg(&cell, 2, 31).neg;
        let run = |cache: &mut BaseCache, p: &Prog| -> Ops {
            let r = eval_prog(&cell, cache, p, &negs, 1, 0, 0);
            let mut t = Ops::default();
            for (_, o) in &r.neg {
                t.mul += o.mul;
                t.sqr += o.sqr;
                t.lin += o.lin;
                t.probe += o.probe;
                t.trace += o.trace;
                t.pairs += o.pairs;
            }
            t
        };
        let base = Prog::incumbent();
        let mut full = base;
        full.g[G_PAIRS] = 1;
        let mut lifted = base;
        lifted.g[G_LIFT] = 1;
        lifted.g[G_STOP] = 1;
        let mut cont = base;
        cont.g[G_STOP] = 1;
        let mut ab = cont;
        ab.g[G_ABORT] = 4;
        let mut gen = base;
        gen.g[G_ROOT] = 1;
        let mut b1 = base;
        b1.g[G_QUART] = 1;
        let o_inc = run(&mut cache, &cont);
        let o_full = run(&mut cache, &{
            let mut p = cont;
            p.g[G_PAIRS] = 1;
            p
        });
        let o_lift = run(&mut cache, &{
            let mut p = cont;
            p.g[G_LIFT] = 1;
            p
        });
        let o_ab = run(&mut cache, &ab);
        let o_gen = run(&mut cache, &{
            let mut p = cont;
            p.g[G_ROOT] = 1;
            p
        });
        let o_b1 = run(&mut cache, &{
            let mut p = cont;
            p.g[G_QUART] = 1;
            p
        });
        let r_full = o_full.pairs as f64 / o_inc.pairs as f64;
        let r_lift = o_lift.pairs as f64 / o_inc.pairs as f64;
        let r_ab = o_ab.pairs as f64 / o_inc.pairs as f64;
        let c_inc = o_inc.core() as f64;
        println!(
            "  incumbent-continue core/pair {:.1}, lin/pair {:.1}",
            c_inc / o_inc.pairs as f64,
            o_inc.lin as f64 / o_inc.pairs as f64
        );
        all &= check(
            "handicap pairs=full costs ~2x pairs",
            (1.95..2.05).contains(&r_full),
            &format!("ratio {r_full:.3}"),
        );
        all &= check(
            "lift prefilter costs ~1/4 pairs",
            (0.20..0.30).contains(&r_lift),
            &format!("ratio {r_lift:.3}"),
        );
        all &= check(
            "abort 1/8 costs 1-(7/8)^2 = 0.234 of pairs",
            (0.22..0.25).contains(&r_ab),
            &format!("ratio {r_ab:.3}"),
        );
        all &= check(
            "generic root finder costs more core ops",
            o_gen.core() > o_inc.core() && (o_gen.core() as f64 / c_inc) > 1.3,
            &format!("ratio {:.3}", o_gen.core() as f64 / c_inc),
        );
        all &= check(
            "b1 quartic costs fewer core ops",
            o_b1.core() < o_inc.core(),
            &format!("ratio {:.4}", o_b1.core() as f64 / c_inc),
        );
        let _ = (base, full, lifted, gen, b1);
    }
    println!(
        "selftest {} in {:.1}s, peak RSS {} MB",
        if all { "PASSED" } else { "FAILED" },
        t0.elapsed().as_secs_f64(),
        rss_peak_kb() / 1024
    );
    all
}

// ---------------------------------------------------------------- the search

const TRAIN_NEG: usize = 6;
const TRAIN_POS: usize = 32;
const CAP_FACTOR: u64 = 3;

fn cell_seed(n: u32) -> u64 {
    0xB10_5EED_0000 + n as u64
}

fn handicapped() -> Prog {
    // Full pairs, naive quartic, generic root finder, one inversion per pair.
    let mut g = [0u8; 10];
    g[G_ROOT] = 1;
    g[G_QUART] = 2;
    g[G_PAIRS] = 1;
    g[G_BATCH] = 2;
    Prog { g }.canon()
}

struct Archive {
    progs: Vec<Prog>,
    q01: Vec<f64>,
}

fn load_archive(path: &str) -> Archive {
    let mut a = Archive {
        progs: vec![],
        q01: vec![],
    };
    if let Ok(text) = std::fs::read_to_string(path) {
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let genes: Vec<u8> = parse_nums(jfield(line, "genes").unwrap())
                .iter()
                .map(|&v| v as u8)
                .collect();
            a.progs.push(Prog::from_genes(&genes));
            let q = jfield(line, "q01").unwrap();
            a.q01.push(if q == "null" {
                f64::INFINITY
            } else {
                q.parse().unwrap()
            });
        }
    }
    a
}

fn best_of(a: &Archive) -> Option<usize> {
    let mut best: Option<usize> = None;
    for (i, &q) in a.q01.iter().enumerate() {
        if q.is_finite() && best.map(|b| q < a.q01[b]).unwrap_or(true) {
            best = Some(i);
        }
    }
    best
}

fn cmd_search(args: &Args) {
    let n: u32 = args.num("--n", 31);
    let ell: usize = args.num("--ell", 9);
    let arm = args.req("--arm");
    let state = args.req("--state");
    let budget: usize = args.num("--budget", 256);
    let max_new: usize = args.num("--max-new", budget);
    let cell = make_cell(n, ell, cell_seed(n));
    let mut cache = BaseCache::new();
    let negs = gen_neg(&cell, TRAIN_NEG, mix(cell.seed, 0x7A1)).neg;
    let pos_seed = mix(cell.seed, 0x7A2);
    // cap from the arm's start program (the incumbent; the handicapped incumbent for
    // the positive control) on the training negatives
    let start_prog = if arm == "p" {
        handicapped()
    } else {
        Prog::incumbent()
    };
    let inc = eval_prog(&cell, &mut cache, &start_prog, &negs, pos_seed, 0, 0);
    let mean1 = inc.neg.iter().map(|o| o.1.priced(1.0)).sum::<f64>() / inc.neg.len() as f64;
    let cap = (mean1 * CAP_FACTOR as f64) as u64;
    std::fs::create_dir_all(&state).expect("state dir");
    let path = format!("{state}/n{n}-{arm}.jsonl");
    let mut archive = load_archive(&path);
    println!(
        "search n={n} ell={ell} arm={arm} budget={budget} resume_at={} cap={cap}",
        archive.progs.len()
    );
    let seed = mix3(cell.seed, 0x5EA2C, arm.bytes().next().unwrap() as u64);
    let mut done_now = 0usize;
    while archive.progs.len() < budget && done_now < max_new {
        let i = archive.progs.len();
        let mut rng = Rng(mix(seed, i as u64));
        let seen = |p: &Prog, a: &Archive| a.progs.iter().any(|q| q == p);
        let prog = match arm.as_str() {
            "c" => loop {
                let p = random_prog(&mut rng);
                if !seen(&p, &archive) {
                    break p;
                }
            },
            "b" | "p" => {
                if i == 0 {
                    if arm == "b" {
                        Prog::incumbent()
                    } else {
                        handicapped()
                    }
                } else {
                    // elite parent: the best so far with probability 0.7, else one of the top 8
                    let mut order: Vec<usize> = (0..archive.progs.len())
                        .filter(|&k| archive.q01[k].is_finite())
                        .collect();
                    order.sort_by(|&x, &y| {
                        archive.q01[x]
                            .partial_cmp(&archive.q01[y])
                            .unwrap()
                            .then(x.cmp(&y))
                    });
                    let parent = if order.is_empty() {
                        archive.progs[0]
                    } else if rng.unit() < 0.7 {
                        archive.progs[order[0]]
                    } else {
                        let top = order.len().min(8);
                        archive.progs[order[rng.below(top as u64) as usize]]
                    };
                    let mut tries = 0;
                    loop {
                        let p = mutate(&parent, &mut rng);
                        if !seen(&p, &archive) {
                            break p;
                        }
                        tries += 1;
                        if tries > 200 {
                            let q = random_prog(&mut rng);
                            if !seen(&q, &archive) {
                                break q;
                            }
                        }
                    }
                }
            }
            _ => panic!("arm must be b, c or p"),
        };
        let e = eval_prog(&cell, &mut cache, &prog, &negs, pos_seed, TRAIN_POS, cap);
        let line = format!(
            "{{\"idx\":{i},\"arm\":\"{arm}\",\"n\":{n},\"genes\":{},\"name\":\"{}\",\"b_r\":{},\"t_eff\":{},\"dominated\":{},\"phi\":{},\"q0\":{},\"q01\":{},\"q1\":{},\"wall_ms\":{}}}",
            prog.json_genes(),
            prog.name(),
            e.b_r,
            f64j(e.t_eff),
            e.dominated,
            f64j(e.phi()),
            f64j(e.q(0.0)),
            f64j(e.q(0.1)),
            f64j(e.q(1.0)),
            e.wall_ms
        );
        {
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .expect("archive file");
            writeln!(f, "{line}").unwrap();
        }
        println!(
            "eval {i:3} {:<58} q01={:>10.4e} phi={:.3} dom={} {}ms",
            prog.name(),
            e.q(0.1),
            e.phi(),
            e.dominated,
            e.wall_ms
        );
        archive.progs.push(prog);
        archive.q01.push(e.q(0.1));
        done_now += 1;
    }
    if let Some(b) = best_of(&archive) {
        println!(
            "best so far idx {b} {} q01={:.4e} ({} of {budget} evaluations done)",
            archive.progs[b].name(),
            archive.q01[b],
            archive.progs.len()
        );
    }
}

fn cmd_finalists(args: &Args) {
    let n: u32 = args.num("--n", 31);
    let state = args.req("--state");
    let mut out = String::from("{");
    let mut first = true;
    for arm in ["b", "c", "p"] {
        let path = format!("{state}/n{n}-{arm}.jsonl");
        let a = load_archive(&path);
        if let Some(b) = best_of(&a) {
            if !first {
                out.push(',');
            }
            first = false;
            // running best (training Q at w = 0.1) after 1, 2, 4, .. evaluations
            let mut curve: Vec<String> = vec![];
            let mut best = f64::INFINITY;
            let mut next = 1usize;
            for (i, &q) in a.q01.iter().enumerate() {
                if q < best {
                    best = q;
                }
                if i + 1 == next || i + 1 == a.q01.len() {
                    curve.push(format!("[{},{}]", i + 1, f64j(best)));
                    next *= 2;
                }
            }
            let dominated = a.q01.iter().filter(|q| !q.is_finite()).count();
            let mut fams: Vec<usize> = a.progs.iter().map(|p| p.g[G_FAM] as usize).collect();
            fams.sort();
            fams.dedup();
            out += &format!(
                "\"{arm}\":{{\"idx\":{b},\"genes\":{},\"name\":\"{}\",\"q01_train\":{},\"start_q01\":{},\"evaluations\":{},\"infinite_fitness\":{dominated},\"families_visited\":{},\"curve\":[{}]}}",
                a.progs[b].json_genes(),
                a.progs[b].name(),
                f64j(a.q01[b]),
                f64j(a.q01[0]),
                a.progs.len(),
                fams.len(),
                curve.join(",")
            );
        }
    }
    out.push('}');
    println!("{out}");
    std::fs::write(format!("{state}/finalists-n{n}.json"), format!("{out}\n"))
        .expect("write finalists");
}

fn read_finalist(state: &str, n: u32, arm: &str) -> Option<Prog> {
    let text = std::fs::read_to_string(format!("{state}/finalists-n{n}.json")).ok()?;
    let pat = format!("\"{arm}\":{{");
    let at = text.find(&pat)?;
    let rest = &text[at + pat.len()..];
    let genes = jfield(rest, "genes")?;
    let v: Vec<u8> = parse_nums(genes).iter().map(|&x| x as u8).collect();
    Some(Prog::from_genes(&v))
}

/// The labelled programs a held-out stage evaluates.
fn labelled_programs(state: &str, n: u32) -> Vec<(String, Prog)> {
    let mut v: Vec<(String, Prog)> = vec![("A".into(), Prog::incumbent())];
    let mut a1 = Prog::incumbent();
    a1.g[G_LIFT] = 1;
    a1.g[G_STOP] = 1;
    v.push(("A1".into(), a1));
    for (lab, arm) in [("B", "b"), ("C", "c"), ("P", "p")] {
        if let Some(p) = read_finalist(state, n, arm) {
            v.push((lab.into(), p));
        }
    }
    v
}

fn cmd_heldout(args: &Args) {
    let n: u32 = args.num("--n", 31);
    let ell: usize = args.num("--ell", 9);
    let state = args.req("--state");
    let out = args.req("--out");
    let nneg: usize = args.num("--neg", 128);
    let npos: usize = args.num("--pos", 128);
    let only = args.flag("--only");
    let cell = make_cell(n, ell, cell_seed(n));
    let mut cache = BaseCache::new();
    let negs = gen_neg(&cell, nneg, mix(cell.seed, 0x4E1D)).neg;
    let pos_seed = mix(cell.seed, 0x4E1E);
    let mut file = std::fs::File::create(&out).expect("output file");
    for (label, prog) in labelled_programs(&state, n) {
        if let Some(o) = &only {
            if !o.split(',').any(|x| x == label) {
                continue;
            }
        }
        let e = eval_prog(&cell, &mut cache, &prog, &negs, pos_seed, npos, 0);
        let extra = format!("\"label\":\"{label}\",\"n\":{n},\"ell\":{ell},");
        writeln!(file, "{}", eval_json(&e, &extra)).unwrap();
        println!(
            "heldout n={n} {label:<3} {:<58} q0={:.4e} q01={:.4e} q1={:.4e} phi={:.3} b_r={} {}ms",
            prog.name(),
            e.q(0.0),
            e.q(0.1),
            e.q(1.0),
            e.phi(),
            e.b_r,
            e.wall_ms
        );
    }
}

fn cmd_ablate(args: &Args) {
    let n: u32 = args.num("--n", 31);
    let ell: usize = args.num("--ell", 9);
    let state = args.req("--state");
    let out = args.req("--out");
    let nneg: usize = args.num("--neg", 48);
    let npos: usize = args.num("--pos", 48);
    let cell = make_cell(n, ell, cell_seed(n));
    let mut cache = BaseCache::new();
    let negs = gen_neg(&cell, nneg, mix(cell.seed, 0x4E1D)).neg;
    let pos_seed = mix(cell.seed, 0x4E1E);
    let mut file = std::fs::File::create(&out).expect("output file");
    let inc = Prog::incumbent();
    let mut todo: Vec<(String, Prog)> = vec![("A".into(), inc)];
    for (lab, arm) in [("B", "b"), ("C", "c")] {
        let Some(best) = read_finalist(&state, n, arm) else {
            continue;
        };
        todo.push((format!("{lab}"), best));
        for i in 0..10 {
            if best.g[i] == inc.g[i] {
                continue;
            }
            // revert gene i of the finalist to the incumbent value
            let mut r = best;
            r.g[i] = inc.g[i];
            let r = r.canon();
            if r.valid() {
                todo.push((format!("{lab}-revert-{}", GENE_NAME[i]), r));
            }
            // add only gene i of the finalist to the incumbent
            let mut a = inc;
            a.g[i] = best.g[i];
            let a = a.canon();
            if a.valid() {
                todo.push((format!("{lab}-only-{}", GENE_NAME[i]), a));
            }
        }
    }
    for (label, prog) in todo {
        let e = eval_prog(&cell, &mut cache, &prog, &negs, pos_seed, npos, 0);
        let extra = format!("\"label\":\"{label}\",\"n\":{n},\"ell\":{ell},");
        writeln!(file, "{}", eval_json(&e, &extra)).unwrap();
        println!(
            "ablate n={n} {label:<22} {:<58} q01={:.4e} phi={:.3}",
            prog.name(),
            e.q(0.1),
            e.phi()
        );
    }
}

/// Yield conservation at a cell where natural relations are common: the
/// found-rate of exact programs on natural S targets, per family, against
/// the triple count C(B_r + 2, 3).
fn cmd_conserve(args: &Args) {
    let n: u32 = args.num("--n", 17);
    let ell: usize = args.num("--ell", 6);
    let ntargets: usize = args.num("--targets", 3000);
    let out = args.req("--out");
    let states: Vec<String> = args
        .flag("--states")
        .map(|s| s.split(',').map(|x| x.to_string()).collect())
        .unwrap_or_default();
    let cell = make_cell(n, ell, cell_seed(n));
    let mut cache = BaseCache::new();
    let negs = gen_neg(&cell, ntargets, mix(cell.seed, 0xC0));
    let mut file = std::fs::File::create(&out).expect("output file");
    let mut progs: Vec<(String, Prog)> = vec![];
    let with_families: usize = args.num("--families", 1);
    for fam in 0..NFAM {
        if with_families == 0 && fam != 0 {
            continue;
        }
        if args.num("--skip-other", 0usize) == 1 && fam_kind(fam) == Kind::Other {
            continue;
        }
        let mut p = Prog::incumbent();
        p.g[G_FAM] = fam as u8;
        p.g[G_LIFT] = 1;
        p.g[G_STOP] = 1;
        p.g[G_QUART] = 1;
        if fam_kind(fam) == Kind::Other {
            p.g[G_ROOT] = 2;
        }
        progs.push((format!("exact-{}", fam_name(fam)), p.canon()));
    }
    progs.push(("A".into(), Prog::incumbent()));
    for st in &states {
        for nn in [31u32, 37, 41] {
            for (lab, p) in labelled_programs(st, nn) {
                if lab == "A" || lab == "A1" {
                    continue;
                }
                progs.push((format!("{lab}-n{nn}"), p));
            }
        }
    }
    for (label, prog) in progs {
        if !prog.valid() {
            continue;
        }
        let fam = prog.g[G_FAM] as usize;
        cache.base(&cell, fam);
        let bc = &cache.map[&fam];
        let b_r = bc.rlist.len();
        let mut found = 0usize;
        let mut core = 0u64;
        let mut nat = 0u64;
        for &x in &negs.neg {
            let s0 = cell.gf.snap();
            let r = scan(&cell, bc, &prog, x, 0);
            let d = cell.gf.snap().sub(s0);
            core += d.core();
            nat += d.nat();
            if r.found {
                found += 1;
            }
        }
        let triples = bc.t_eff();
        let rate = found as f64 / ntargets as f64;
        // Poisson-corrected yield per feasible triple: rate = 1 - exp(-kappa T_eff)
        let kappa = if found > 0 && found < ntargets {
            -(1.0 - rate).ln() / triples
        } else {
            f64::NAN
        };
        // relation-collection cost: counted cost per relation found, times the
        // number of columns (rational base abscissae)
        let wd = |w: f64| (core as f64 + w * nat as f64) / (found.max(1)) as f64 * b_r as f64;
        writeln!(
            file,
            "{{\"label\":\"{label}\",\"genes\":{},\"n\":{n},\"ell\":{ell},\"targets\":{ntargets},\"found\":{found},\"rate\":{},\"b_r\":{b_r},\"triples\":{triples},\"rate_per_triple\":{},\"kappa\":{},\"core_per_target\":{},\"nat_per_target\":{},\"wd0\":{},\"wd01\":{},\"wd1\":{}}}",
            prog.json_genes(),
            f64j(rate),
            f64j(rate / triples),
            f64j(kappa),
            f64j(core as f64 / ntargets as f64),
            f64j(nat as f64 / ntargets as f64),
            f64j(wd(0.0)),
            f64j(wd(0.1)),
            f64j(wd(1.0))
        )
        .unwrap();
        println!(
            "conserve {label:<24} b_r={b_r:3} found={found:4}/{ntargets} rate={rate:.4} kappa={:.4e} core/target={:.0} W(w=0.1)={:.4e}",
            kappa,
            core as f64 / ntargets as f64,
            wd(0.1)
        );
    }
}

// --------------------------------------------------------------- summarising

struct Loaded {
    label: String,
    b_r: usize,
    t_eff: f64,
    neg_core: Vec<f64>,
    neg_nat: Vec<f64>,
    neg_found: Vec<bool>,
    pos_found: Vec<bool>,
    pos_core: Vec<f64>,
    pos_nat: Vec<f64>,
}

fn load_heldout(path: &str) -> Vec<Loaded> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("cannot read {path}"));
    let mut out = vec![];
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let label = jfield(line, "label").unwrap().trim_matches('"').to_string();
        let b_r = jfield(line, "b_r").unwrap().parse().unwrap();
        let t_eff: f64 = jfield(line, "t_eff").unwrap().parse().unwrap();
        let neg = parse_nums(jfield(line, "neg").unwrap());
        let pos = parse_nums(jfield(line, "pos").unwrap());
        // each entry is [found, mul, sqr, lin, probe, trace, pairs]
        let mut l = Loaded {
            label,
            b_r,
            t_eff,
            neg_core: vec![],
            neg_nat: vec![],
            neg_found: vec![],
            pos_found: vec![],
            pos_core: vec![],
            pos_nat: vec![],
        };
        for c in neg.chunks(7) {
            l.neg_found.push(c[0] > 0.5);
            l.neg_core.push(c[1] + c[2]);
            l.neg_nat.push(c[3] + c[4] + c[5] + c[6]);
        }
        for c in pos.chunks(7) {
            l.pos_found.push(c[0] > 0.5);
            l.pos_core.push(c[1] + c[2]);
            l.pos_nat.push(c[3] + c[4] + c[5] + c[6]);
        }
        out.push(l);
    }
    out
}

fn q_of(l: &Loaded, w: f64, neg_idx: &[usize], pos_idx: &[usize]) -> f64 {
    let c0: f64 = neg_idx
        .iter()
        .map(|&i| l.neg_core[i] + w * l.neg_nat[i])
        .sum::<f64>()
        / neg_idx.len() as f64;
    let phi = pos_idx.iter().filter(|&&i| l.pos_found[i]).count() as f64 / pos_idx.len() as f64;
    if phi <= 0.0 {
        return f64::INFINITY;
    }
    if l.t_eff <= 0.0 {
        return f64::INFINITY;
    }
    c0 * l.b_r as f64 / (l.t_eff * phi)
}

fn resample(rng: &mut Rng, n: usize) -> Vec<usize> {
    (0..n).map(|_| rng.below(n as u64) as usize).collect()
}

/// Paired (negatives shared) bootstrap of Q_x / Q_y; positives resampled
/// independently per program. Returns (point, lo, hi) at 95%.
fn ratio_ci(
    x: &Loaded,
    y: &Loaded,
    w: f64,
    resamples: usize,
    seed: u64,
    neg_range: Option<(usize, usize)>,
) -> (f64, f64, f64) {
    let (a, b) = neg_range.unwrap_or((0, x.neg_core.len()));
    let all_neg: Vec<usize> = (a..b).collect();
    let all_pos_x: Vec<usize> = (0..x.pos_found.len()).collect();
    let all_pos_y: Vec<usize> = (0..y.pos_found.len()).collect();
    let point = q_of(x, w, &all_neg, &all_pos_x) / q_of(y, w, &all_neg, &all_pos_y);
    let mut rng = Rng(seed);
    let mut rs: Vec<f64> = vec![];
    for _ in 0..resamples {
        let ni: Vec<usize> = resample(&mut rng, all_neg.len())
            .iter()
            .map(|&i| all_neg[i])
            .collect();
        let px = resample(&mut rng, all_pos_x.len());
        let py = resample(&mut rng, all_pos_y.len());
        let r = q_of(x, w, &ni, &px) / q_of(y, w, &ni, &py);
        if r.is_finite() {
            rs.push(r);
        }
    }
    rs.sort_by(|p, q| p.partial_cmp(q).unwrap());
    if rs.is_empty() {
        return (point, f64::NAN, f64::NAN);
    }
    let lo = rs[((rs.len() as f64) * 0.025) as usize];
    let hi = rs[(((rs.len() as f64) * 0.975) as usize).min(rs.len() - 1)];
    (point, lo, hi)
}

fn classify(ci: &[(f64, f64, f64)]) -> &'static str {
    let better = ci.iter().filter(|c| c.2 < 1.0).count();
    let worse = ci.iter().filter(|c| c.1 > 1.0).count();
    if better == ci.len() {
        "better"
    } else if worse == ci.len() {
        "worse"
    } else if better > 0 && worse > 0 {
        "trades"
    } else {
        "straddles"
    }
}

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];

fn cmd_summarize(args: &Args) {
    let dir = args.req("--dir");
    let ns: Vec<u32> = args
        .flag("--ns")
        .unwrap_or_else(|| "31,37,41".into())
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let resamples: usize = args.num("--resamples", 2000);
    let mut json = String::from("{\"cells\":[");
    let mut verdicts: Vec<(u32, String)> = vec![];
    for (ci_, &n) in ns.iter().enumerate() {
        let path = format!("{dir}/heldout-n{n}.jsonl");
        if !std::path::Path::new(&path).exists() {
            println!("n={n}: no held-out file");
            continue;
        }
        let ls = load_heldout(&path);
        let get = |lab: &str| ls.iter().find(|l| l.label == lab);
        println!(
            "== n={n} (ell from file; negatives {}, positives {}) ==",
            ls[0].neg_core.len(),
            ls[0].pos_found.len()
        );
        println!(
            "{:<4} {:>5} {:>7} {:>8} | {:>12} {:>12} {:>12} | natural found",
            "prog", "B_r", "phi", "core/neg", "Q(w=0)", "Q(w=0.1)", "Q(w=1)"
        );
        let all_n: Vec<usize> = (0..ls[0].neg_core.len()).collect();
        for l in &ls {
            let all_p: Vec<usize> = (0..l.pos_found.len()).collect();
            let phi = l.pos_found.iter().filter(|&&f| f).count() as f64 / l.pos_found.len() as f64;
            let core = l.neg_core.iter().sum::<f64>() / l.neg_core.len() as f64;
            println!(
                "{:<4} {:>5} {:>7.3} {:>8.0} | {:>12.4e} {:>12.4e} {:>12.4e} | {}/{}",
                l.label,
                l.b_r,
                phi,
                core,
                q_of(l, 0.0, &all_n, &all_p),
                q_of(l, 0.1, &all_n, &all_p),
                q_of(l, 1.0, &all_n, &all_p),
                l.neg_found.iter().filter(|&&f| f).count(),
                l.neg_found.len()
            );
        }
        let mut cell_json = format!("{{\"n\":{n},\"comparisons\":[");
        let mut verdict = String::from("INCONCLUSIVE");
        let mut first = true;
        let pairs = [
            ("B", "A"),
            ("B", "A1"),
            ("B", "C"),
            ("C", "A"),
            ("C", "A1"),
            ("A1", "A"),
            ("P", "A"),
        ];
        let mut cls: HashMap<(String, String), &'static str> = HashMap::new();
        for (xl, yl) in pairs {
            let (Some(x), Some(y)) = (get(xl), get(yl)) else {
                continue;
            };
            let mut cis = vec![];
            for (wi, &w) in WEIGHTS.iter().enumerate() {
                cis.push(ratio_ci(
                    x,
                    y,
                    w,
                    resamples,
                    mix3(0xB007, n as u64 * 7 + ci_ as u64, wi as u64),
                    None,
                ));
            }
            let c = classify(&cis);
            cls.insert((xl.into(), yl.into()), c);
            println!(
                "  Q({xl})/Q({yl}): w=0 {:.3} [{:.3},{:.3}]  w=0.1 {:.3} [{:.3},{:.3}]  w=1 {:.3} [{:.3},{:.3}]  -> {c}",
                cis[0].0, cis[0].1, cis[0].2, cis[1].0, cis[1].1, cis[1].2, cis[2].0, cis[2].1, cis[2].2
            );
            if !first {
                cell_json.push(',');
            }
            first = false;
            cell_json += &format!(
                "{{\"x\":\"{xl}\",\"y\":\"{yl}\",\"class\":\"{c}\",\"w0\":[{},{},{}],\"w01\":[{},{},{}],\"w1\":[{},{},{}]}}",
                f64j(cis[0].0), f64j(cis[0].1), f64j(cis[0].2),
                f64j(cis[1].0), f64j(cis[1].1), f64j(cis[1].2),
                f64j(cis[2].0), f64j(cis[2].1), f64j(cis[2].2)
            );
        }
        // null control: the incumbent against itself on disjoint halves of the negatives
        if let Some(a) = get("A") {
            let h = a.neg_core.len() / 2;
            let mut cis = vec![];
            for (wi, &w) in WEIGHTS.iter().enumerate() {
                // first half vs second half, positives shared => ratio of neg-cost halves
                let first_half: Vec<usize> = (0..h).collect();
                let second_half: Vec<usize> = (h..a.neg_core.len()).collect();
                let all_p: Vec<usize> = (0..a.pos_found.len()).collect();
                let pt = q_of(a, w, &first_half, &all_p) / q_of(a, w, &second_half, &all_p);
                let mut rng = Rng(mix3(0x2111, n as u64, wi as u64));
                let mut rs = vec![];
                for _ in 0..resamples {
                    let fi: Vec<usize> = resample(&mut rng, h)
                        .iter()
                        .map(|&i| first_half[i])
                        .collect();
                    let si: Vec<usize> = resample(&mut rng, a.neg_core.len() - h)
                        .iter()
                        .map(|&i| second_half[i])
                        .collect();
                    let pp = resample(&mut rng, all_p.len());
                    let r = q_of(a, w, &fi, &pp) / q_of(a, w, &si, &pp);
                    if r.is_finite() {
                        rs.push(r);
                    }
                }
                rs.sort_by(|p, q| p.partial_cmp(q).unwrap());
                cis.push((
                    pt,
                    rs[(rs.len() as f64 * 0.025) as usize],
                    rs[((rs.len() as f64 * 0.975) as usize).min(rs.len() - 1)],
                ));
            }
            let covers = cis.iter().all(|c| c.1 <= 1.0 && c.2 >= 1.0);
            println!(
                "  null (A first half / second half): {:.3} [{:.3},{:.3}] {:.3} [{:.3},{:.3}] {:.3} [{:.3},{:.3}] interval covers 1 at all weights: {covers}",
                cis[0].0, cis[0].1, cis[0].2, cis[1].0, cis[1].1, cis[1].2, cis[2].0, cis[2].1, cis[2].2
            );
            cell_json += &format!(
                ",{{\"x\":\"A-half1\",\"y\":\"A-half2\",\"covers_one_all_weights\":{covers}}}"
            );
        }
        cell_json.push(']');
        // the frozen rule
        let ba = cls.get(&("B".into(), "A".into())).copied();
        let bc = cls.get(&("B".into(), "C".into())).copied();
        if let Some(c) = ba {
            verdict = match c {
                "better" => {
                    if bc == Some("better") {
                        "ADVANCES".into()
                    } else {
                        "MATCHES".into()
                    }
                }
                "worse" => "REGRESSES".into(),
                "trades" => "TRADES".into(),
                _ => "INCONCLUSIVE".into(),
            };
        }
        println!(
            "  cell verdict by the frozen rule: {verdict} (B vs A: {:?}, B vs C: {:?})",
            ba, bc
        );
        cell_json += &format!(",\"verdict\":\"{verdict}\"}}");
        if ci_ > 0 {
            json.push(',');
        }
        json += &cell_json;
        verdicts.push((n, verdict));
    }
    json.push_str("],");
    // overall: the common cell verdict, else INCONCLUSIVE
    let overall = if !verdicts.is_empty() && verdicts.iter().all(|v| v.1 == verdicts[0].1) {
        verdicts[0].1.clone()
    } else {
        "INCONCLUSIVE".to_string()
    };
    println!("overall verdict: {overall}; per cell {:?}", verdicts);
    json += &format!("\"overall\":\"{overall}\"");
    // search positive control (arm p, n = 31)
    if let Some(state) = args.flag("--state") {
        if let Ok(text) = std::fs::read_to_string(format!("{state}/finalists-n31.json")) {
            let get = |arm: &str, key: &str| -> Option<f64> {
                let pat = format!("\"{arm}\":{{");
                let at = text.find(&pat)?;
                jfield(&text[at + pat.len()..], key)?.parse().ok()
            };
            if let (Some(pb), Some(ps), Some(inc)) = (
                get("p", "q01_train"),
                get("p", "start_q01"),
                get("b", "start_q01"),
            ) {
                let pass = pb <= inc && pb <= 0.5 * ps;
                println!(
                    "search positive control (arm p, n=31): best {pb:.4e}, handicapped start {ps:.4e}, incumbent {inc:.4e} -> {}",
                    if pass { "PASS" } else { "FAIL" }
                );
                json += &format!(",\"positive_control_p\":{{\"best\":{},\"start\":{},\"incumbent\":{},\"pass\":{pass}}}", f64j(pb), f64j(ps), f64j(inc));
            }
        }
    }
    // yield-model check of the finalists' families against the conservation cell
    {
        // the conservation cell's output sits next to the held-out files
        let cons = format!("{dir}/conserve-17-6.jsonl");
        if let Ok(text) = std::fs::read_to_string(&cons) {
            let mut sub: Vec<f64> = vec![];
            let mut by_label: HashMap<String, f64> = HashMap::new();
            for line in text.lines().filter(|l| !l.trim().is_empty()) {
                let label = jfield(line, "label").unwrap().trim_matches('"').to_string();
                if let Some(Ok(v)) = jfield(line, "kappa").map(|v| v.parse::<f64>()) {
                    if label.starts_with("exact-sub-") {
                        sub.push(v);
                    }
                    by_label.insert(label, v);
                }
            }
            sub.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if !sub.is_empty() {
                let med = sub[sub.len() / 2];
                println!("yield model (kappa = -ln(1-rate)/T_eff at the conservation cell), median over sub families {med:.4e}");
                json += &format!(",\"yield_model_median\":{}", f64j(med));
                for &n in &ns {
                    for arm in ["b", "c"] {
                        if let Some(pr) =
                            read_finalist(args.flag("--state").as_deref().unwrap_or("."), n, arm)
                        {
                            let key = format!("exact-{}", fam_name(pr.g[G_FAM] as usize));
                            if let Some(&v) = by_label.get(&key) {
                                let ok = (v / med - 1.0).abs() <= 0.25;
                                println!("  n={n} finalist {arm}: family {} kappa {v:.4e} ({:+.1}% of median) -> {}", fam_name(pr.g[G_FAM] as usize), (v / med - 1.0) * 100.0, if ok { "within 25%" } else { "VIOLATES" });
                            }
                        }
                    }
                }
            }
        }
    }
    json.push('}');
    if let Some(out) = args.flag("--out") {
        std::fs::write(out, format!("{json}\n")).expect("write summary");
    }
}

fn main() {
    let args = Args {
        v: std::env::args().skip(1).collect(),
    };
    let cmd = args.v.first().cloned().unwrap_or_default();
    match cmd.as_str() {
        "selftest" => {
            if !selftest(&args) {
                std::process::exit(1);
            }
        }
        "search" => cmd_search(&args),
        "finalists" => cmd_finalists(&args),
        "heldout" => cmd_heldout(&args),
        "ablate" => cmd_ablate(&args),
        "conserve" => cmd_conserve(&args),
        "summarize" => cmd_summarize(&args),
        _ => {
            eprintln!("usage: b10 selftest | search | finalists | heldout | conserve | ablate | summarize");
            std::process::exit(2);
        }
    }
    let _ = std::io::stdout().flush();
}
