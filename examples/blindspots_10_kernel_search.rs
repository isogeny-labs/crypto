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
            return if rng.next() & 1 == 1 { (x, y ^ x) } else { (x, y) };
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
fn lv_mod_quartic_horner(f: [u64; 4], frob: &QuarticFrobenius, lvh: &LvHorner, gf: &Gf) -> [u64; 4] {
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

const NFAM: usize = 19;

fn fam_name(f: usize) -> String {
    match f {
        0 => "sub-low".into(),
        1..=6 => format!("sub-rand{f}"),
        7..=10 => format!("coset2-s{}", f - 6),
        11..=12 => format!("coset4-s{}", f - 10),
        13 => "ham".into(),
        14 => "ham-shift".into(),
        15..=18 => format!("geom-s{}", f - 14),
        _ => panic!("family {f}"),
    }
}

fn fam_kind(f: usize) -> Kind {
    match f {
        0..=6 => Kind::Sub,
        7..=10 => Kind::Coset(2),
        11..=12 => Kind::Coset(4),
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
    /// `L_U(c_i)` for the coset translates `c_1..c_{k-1}` (coset kinds).
    ys: Vec<u64>,
    /// Bits of `delta = Tr(x + a)` over the rational points: used by planting.
    ell: usize,
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
        7..=12 => {
            let k = if fam <= 10 { 2usize } else { 4 };
            let lk = k.trailing_zeros() as usize;
            let (ub, ech) = random_basis(gf, ell - lk, &mut rng);
            lv = subspace_poly_for_basis(&ub, gf);
            let uspan = span_all(&ub);
            let mut cs: Vec<u64> = vec![0];
            while cs.len() < k {
                let c = rng.next() & gf.mask;
                let ok = cs.iter().all(|&d| ech.reduce(c ^ d) != 0);
                if ok {
                    cs.push(c);
                }
            }
            ys = cs[1..].iter().map(|&c| lin_eval_nc(&lv, c, gf)).collect();
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
        assert!(idx.insert(x, i).is_none(), "family {fam} has a repeated abscissa");
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
    BaseCtx {
        fam,
        kind,
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
                let li = if batch == 2 { gf.inv(q[4]) } else { leads[lead_of[j]] };
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
                            let i1 = if lift { bc.idx[&x1] } else { bc.idx[&x1] };
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
fn base_roots(gf: &Gf, bc: &BaseCtx, lvh: Option<&LvHorner>, f: [u64; 4], root: u8, fast: bool) -> Poly {
    let u: [u64; 4] = if root == 0 {
        let lvh = lvh.expect("horner needs a subspace polynomial");
        let frob = QuarticFrobenius::new(f, gf);
        let u = lv_mod_quartic_horner(f, &frob, lvh, gf);
        match bc.kind {
            Kind::Sub => u,
            Kind::Coset(k) => coset_product(f, u, &bc.ys, k, Some(&frob), gf),
            Kind::Other => unreachable!(),
        }
    } else {
        let p = lv_mod_quartic_generic(f, &bc.lv, gf);
        let u = [p.c[0], p.c[1], p.c[2], p.c[3]];
        match bc.kind {
            Kind::Sub => u,
            Kind::Coset(k) => coset_product(f, u, &bc.ys, k, None, gf),
            Kind::Other => unreachable!(),
        }
    };
    if root == 1 || !fast {
        gcd_generic(f, u, gf)
    } else {
        gcd_quartic_fast(f, u, gf)
    }
}

/// `prod_i (u + y_i) mod f` with `y_0 = 0`: the polynomial in `t` vanishing on
/// the union of cosets when `u = L_U(t) mod f`.
fn coset_product(f: [u64; 4], u: [u64; 4], ys: &[u64], k: usize, frob: Option<&QuarticFrobenius>, gf: &Gf) -> [u64; 4] {
    let monic = Poly::from_low(&[f[0], f[1], f[2], f[3], 1]);
    let up = Poly::from_low(&u);
    let shift = |y: u64| -> Poly {
        let mut v = up;
        v.c[0] = gf.add(v.c[0], y);
        v
    };
    let res = if k == 2 {
        // u * (u + y1) = u^2 + y1 u
        let sq: [u64; 4] = match frob {
            Some(fr) => fr.sqr(u, gf),
            None => {
                let s = up.sqr_mod_monic(&monic, 4, gf);
                [s.c[0], s.c[1], s.c[2], s.c[3]]
            }
        };
        let mut r = [0u64; 4];
        for i in 0..4 {
            r[i] = gf.add(sq[i], gf.mul(ys[0], u[i]));
        }
        r
    } else {
        let a = up.mul_mod_monic(&shift(ys[0]), &monic, 4, gf);
        let bq = shift(ys[1]).mul_mod_monic(&shift(ys[2]), &monic, 4, gf);
        let p = a.mul_mod_monic(&bq, &monic, 4, gf);
        [p.c[0], p.c[1], p.c[2], p.c[3]]
    };
    res
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
    neg: Vec<Ops>,
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
        let c0: f64 = self.neg.iter().map(|o| o.priced(w)).sum::<f64>() / self.neg.len() as f64;
        c0 / ((self.b_r as f64) * (self.b_r as f64) * phi)
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
    let poss = cache.pos(cell, fam, pos_seed, npos);
    let bc = &cache.map[&fam];
    let gf = &cell.gf;
    let mut res = EvalRes {
        prog: *prog,
        b_r: bc.rlist.len(),
        neg: vec![],
        pos: vec![],
        dominated: false,
        wall_ms: 0,
    };
    'run: {
        for &x in negs {
            let s0 = gf.snap();
            let r = scan(cell, bc, prog, x, cap);
            res.neg.push(gf.snap().sub(s0));
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
    let negs: Vec<String> = e.neg.iter().map(ops_json).collect();
    let poss: Vec<String> = e
        .pos
        .iter()
        .map(|p| format!("[{},{}]", if p.0 { 1 } else { 0 }, ops_json(&p.1)))
        .collect();
    format!(
        "{{{}\"genes\":{},\"name\":\"{}\",\"b_r\":{},\"dominated\":{},\"phi\":{},\"q0\":{},\"q01\":{},\"q1\":{},\"wall_ms\":{},\"neg\":[{}],\"pos\":[{}]}}",
        extra,
        e.prog.json_genes(),
        e.prog.name(),
        e.b_r,
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
    s.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == 'e' || c == 'E' || c == '+'))
        .filter(|t| !t.is_empty())
        .filter_map(|t| t.parse::<f64>().ok())
        .collect()
}

include!("blindspots_10_kernel_search_cmds.rs");
