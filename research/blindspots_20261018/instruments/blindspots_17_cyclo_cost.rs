//! Blind-spot item 17: cyclotomic base, counted cost arm up to n = 83.
//!
//! Instrument.  Std only: no `crypto_lib` library (it does not compile at this
//! branch base) and no external crate.  Build with
//!
//! ```bash
//! rustc --edition 2021 -C opt-level=3 -C codegen-units=1 \
//!   -o /tmp/b17 examples/blindspots_17_cyclo_cost.rs
//! /tmp/b17 --mode validate
//! /tmp/b17 --mode cell --n 23 --a 1 --k 30 --cyc-w 2 --pair 1
//! ```
//!
//! What it does (see `research/blindspots_20261018/17-cyclotomic-cost-n83/
//! PROTOCOL.md`):
//!
//! * a wide GF(2^n) (n <= 127, `u128` words), the Koblitz curve `K_a`, point
//!   arithmetic and double-and-add exactly as `ic_boundary::CountedGroup`
//!   counts it (`adds`, `doubles`, `scalar_mults`);
//! * the three factor-base builders as the production plug-ins
//!   (`compact-orbit-scan`, `cyclotomic`, `onb-weight` in
//!   `src/cryptanalysis/ic_framework/plugins.rs`) re-implemented for any n,
//!   with their group-operation ledger and a native (unpriced) ledger;
//! * a "tidy" accounting of the same three builders (random subset order,
//!   single root, one visit per Frobenius orbit of raw abscissae, no sanity
//!   multiplications);
//! * the Frobenius-folded pair table of `ic_boundary::FrobeniusPairTable`
//!   and its probe, so the oracle-ready cost and a per-probe cost are counted;
//! * a negative-control population of generic bases (random scan order);
//! * a structural leg: variables, degree and feasibility of the algebraic
//!   membership systems (`--mode degree`).
//!
//! `--mode validate` compares the group-operation ledger of the instrument at
//! n = 23 with the counters stored in the committed ecbench records of
//! `research/cyclotomic_falsifier_20261015/sessions/` (a hard gate).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::collections::{HashMap, HashSet};
use std::time::Instant;

// ───────────────────────────── small helpers ─────────────────────────────

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn rand_u128(state: &mut u64) -> u128 {
    ((splitmix(state) as u128) << 64) | splitmix(state) as u128
}

fn mulmod(mut a: u128, mut b: u128, m: u128) -> u128 {
    // m < 2^127, so a + a never overflows.
    a %= m;
    let mut acc = 0u128;
    while b > 0 {
        if b & 1 == 1 {
            acc += a;
            if acc >= m {
                acc -= m;
            }
        }
        a += a;
        if a >= m {
            a -= m;
        }
        b >>= 1;
    }
    acc
}

fn powmod(mut a: u128, mut e: u128, m: u128) -> u128 {
    let mut acc = 1u128 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    acc
}

fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41] {
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
    'bases: for a in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

fn popcount(x: u128) -> u32 {
    x.count_ones()
}

fn bits_of(k: u128) -> u32 {
    128 - k.leading_zeros()
}

fn binomial(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut acc: u128 = 1;
    for i in 0..k {
        acc = acc * (n - i) as u128 / (i + 1) as u128;
    }
    acc as u64
}

fn sqrt_mod(a: u128, p: u128) -> Option<u128> {
    // Tonelli-Shanks; p an odd prime.
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u128;
    while powmod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(a, q, p);
    let mut r = powmod(a, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulmod(tt, tt, p);
            i += 1;
            if i == m {
                return None;
            }
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = mulmod(b, b, p);
        }
        m = i;
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        r = mulmod(r, b, p);
    }
    Some(r)
}

// ───────────────────────────── the field ─────────────────────────────────

/// GF(2^n) (or the ring F_2[x]/(f) while testing irreducibility), n <= 127.
/// Elements are `u128` polynomial-basis words, bit i = coefficient of z^i.
struct Gf {
    n: u32,
    low: u128, // f = z^n + low
    mask: u128,
    sq: Vec<[u128; 256]>,
    trm: u128,
}

impl Gf {
    fn new(n: u32, low: u128) -> Gf {
        assert!((2..=127).contains(&n));
        let mask = (1u128 << n) - 1;
        // z^j mod f for j in 0..2n
        let mut xp = Vec::with_capacity(2 * n as usize);
        let mut cur = 1u128;
        for _ in 0..2 * n {
            xp.push(cur);
            let carry = (cur >> (n - 1)) & 1;
            cur = (cur << 1) & mask;
            if carry == 1 {
                cur ^= low;
            }
        }
        let chunks = (n as usize).div_ceil(8);
        let mut sq = vec![[0u128; 256]; chunks];
        for (c, tab) in sq.iter_mut().enumerate() {
            for byte in 0..256usize {
                let mut v = 0u128;
                for b in 0..8 {
                    if (byte >> b) & 1 == 1 {
                        let i = 8 * c + b;
                        if i < n as usize {
                            v ^= xp[2 * i];
                        }
                    }
                }
                tab[byte] = v;
            }
        }
        let mut g = Gf {
            n,
            low,
            mask,
            sq,
            trm: 0,
        };
        // Tr is F_2-linear: Tr(a) = parity(a & trm).
        let mut trm = 0u128;
        for i in 0..n {
            let mut t = 1u128 << i;
            let mut s = t;
            for _ in 1..n {
                t = g.sqr(t);
                s ^= t;
            }
            if s & 1 == 1 {
                trm |= 1u128 << i;
            }
        }
        g.trm = trm;
        g
    }

    fn mul(&self, a: u128, b: u128) -> u128 {
        let n = self.n;
        let mut acc = 0u128;
        for i in (0..n).rev() {
            let carry = (acc >> (n - 1)) & 1;
            acc = (acc << 1) & self.mask;
            if carry == 1 {
                acc ^= self.low;
            }
            if (b >> i) & 1 == 1 {
                acc ^= a;
            }
        }
        acc
    }

    fn sqr(&self, a: u128) -> u128 {
        let mut v = 0u128;
        for (c, tab) in self.sq.iter().enumerate() {
            v ^= tab[((a >> (8 * c)) & 255) as usize];
        }
        v
    }

    fn sqr_k(&self, mut a: u128, k: u32) -> u128 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }

    /// a^(2^k - 1)
    fn itoh(&self, a: u128, k: u32) -> u128 {
        if k == 1 {
            a
        } else if k % 2 == 0 {
            let t = self.itoh(a, k / 2);
            let u = self.sqr_k(t, k / 2);
            self.mul(u, t)
        } else {
            let t = self.itoh(a, k - 1);
            self.mul(self.sqr(t), a)
        }
    }

    fn inv(&self, a: u128) -> u128 {
        assert!(a != 0, "inverse of zero");
        self.sqr(self.itoh(a, self.n - 1))
    }

    fn pow(&self, a: u128, mut e: u128) -> u128 {
        let mut acc = 1u128;
        let mut base = a;
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(acc, base);
            }
            base = self.sqr(base);
            e >>= 1;
        }
        acc
    }

    fn tr(&self, a: u128) -> u32 {
        (a & self.trm).count_ones() & 1
    }

    /// Half-trace, n odd: y^2 + y = c for Tr(c) = 0.
    fn half_trace(&self, c: u128) -> u128 {
        let mut t = c;
        let mut h = c;
        for _ in 0..(self.n - 1) / 2 {
            t = self.sqr(self.sqr(t));
            h ^= t;
        }
        h
    }

    /// Frobenius orbit key: the least of x, x^2, ..., x^(2^(n-1)) and the
    /// first index at which it is attained.
    fn canon(&self, x: u128) -> (u128, u32) {
        let mut best = x;
        let mut idx = 0;
        let mut cur = x;
        for t in 1..self.n {
            cur = self.sqr(cur);
            if cur < best {
                best = cur;
                idx = t;
            }
        }
        (best, idx)
    }
}

fn poly_deg(a: u128) -> i32 {
    127 - a.leading_zeros() as i32
}

fn poly_mod(mut a: u128, b: u128) -> u128 {
    let db = poly_deg(b);
    while a != 0 && poly_deg(a) >= db {
        a ^= b << (poly_deg(a) - db);
    }
    a
}

fn poly_gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = poly_mod(a, b);
        a = b;
        b = r;
    }
    a
}

fn prime_divisors(mut n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            out.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

/// Rabin's test for f = z^n + low over F_2.
fn is_irreducible(n: u32, low: u128) -> bool {
    if low & 1 == 0 {
        return false;
    }
    let g = Gf::new(n, low);
    let x = 2u128;
    if g.sqr_k(x, n) != x {
        return false;
    }
    let full = (1u128 << n) | low;
    for q in prime_divisors(n) {
        let t = g.sqr_k(x, n / q) ^ x;
        if poly_gcd(full, t) != 1 {
            return false;
        }
    }
    true
}

/// The smallest-mask irreducible with at most three terms below z^n (the
/// rule of `find_irreducible_sparse[_wide]`).
fn find_irreducible_sparse(n: u32) -> u128 {
    let mut cands: Vec<u128> = vec![1];
    for i in 1..n {
        cands.push(1 | (1u128 << i));
        for j in (i + 1)..n {
            cands.push(1 | (1u128 << i) | (1u128 << j));
            for k in (j + 1)..n {
                cands.push(1 | (1u128 << i) | (1u128 << j) | (1u128 << k));
            }
        }
    }
    cands.sort_unstable();
    for low in cands {
        if is_irreducible(n, low) {
            return low;
        }
    }
    panic!("no sparse irreducible of degree {n}");
}

fn terms_string(n: u32, low: u128) -> String {
    let mut s = format!("z^{n}");
    for i in (0..n).rev() {
        if (low >> i) & 1 == 1 {
            if i == 0 {
                s.push_str("+1");
            } else {
                s.push_str(&format!("+z^{i}"));
            }
        }
    }
    s
}

// ───────────────────────────── the curve ─────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    inf: bool,
    x: u128,
    y: u128,
}

const INF: Pt = Pt {
    inf: true,
    x: 0,
    y: 0,
};

#[derive(Default, Clone, Copy)]
struct Ops {
    adds: u64,
    doubles: u64,
    smults: u64,
}

impl Ops {
    fn g(&self) -> u64 {
        self.adds + self.doubles
    }
    fn merge(&mut self, o: &Ops) {
        self.adds += o.adds;
        self.doubles += o.doubles;
        self.smults += o.smults;
    }
}

/// The native (unpriced) ledger, in field-level operations.  Every item is one
/// operation of the named kind; `total()` is the number priced at c per
/// operation in the section 5a band.
#[derive(Default, Clone, Copy)]
struct Nat {
    fadd: u64,    // field additions while enumerating subset sums
    sort: u64,    // comparisons of the sort of an enumerated set (m log2 m)
    inv: u64,     // field inversions of the lift
    sqr: u64,     // field squarings of the lift (x^-2)
    mul: u64,     // field multiplications of the lift (y = x t)
    tr: u64,      // trace evaluations
    ht_sqr: u64,  // squarings inside half-traces
    key_sqr: u64, // squarings inside orbit keys / canonicalisations
    hash: u64,    // hash-set / hash-map operations
    frob: u64,    // point Frobenius maps (each two squarings)
    neg: u64,     // point negations
}

impl Nat {
    fn total(&self) -> u64 {
        self.fadd
            + self.sort
            + self.inv
            + self.sqr
            + self.mul
            + self.tr
            + self.ht_sqr
            + self.key_sqr
            + self.hash
            + 2 * self.frob
            + self.neg
    }
    fn merge(&mut self, o: &Nat) {
        self.fadd += o.fadd;
        self.sort += o.sort;
        self.inv += o.inv;
        self.sqr += o.sqr;
        self.mul += o.mul;
        self.tr += o.tr;
        self.ht_sqr += o.ht_sqr;
        self.key_sqr += o.key_sqr;
        self.hash += o.hash;
        self.frob += o.frob;
        self.neg += o.neg;
    }
    fn json(&self) -> String {
        format!(
            "{{\"fadd\":{},\"sort\":{},\"inv\":{},\"sqr\":{},\"mul\":{},\"tr\":{},\"ht_sqr\":{},\"key_sqr\":{},\"hash\":{},\"frob\":{},\"neg\":{},\"total\":{}}}",
            self.fadd, self.sort, self.inv, self.sqr, self.mul, self.tr, self.ht_sqr,
            self.key_sqr, self.hash, self.frob, self.neg, self.total()
        )
    }
}

struct Curve {
    gf: Gf,
    a: u128, // 0 or 1 (b = 1)
    t: i32,  // trace of Frobenius over F_2: +1 for a = 1, -1 for a = 0
}

impl Curve {
    fn neg(&self, p: Pt) -> Pt {
        if p.inf {
            p
        } else {
            Pt {
                inf: false,
                x: p.x,
                y: p.y ^ p.x,
            }
        }
    }

    fn frob(&self, p: Pt) -> Pt {
        if p.inf {
            p
        } else {
            Pt {
                inf: false,
                x: self.gf.sqr(p.x),
                y: self.gf.sqr(p.y),
            }
        }
    }

    fn on_curve(&self, p: Pt) -> bool {
        if p.inf {
            return true;
        }
        let g = &self.gf;
        let lhs = g.sqr(p.y) ^ g.mul(p.x, p.y);
        let x2 = g.sqr(p.x);
        let rhs = g.mul(x2, p.x) ^ if self.a == 1 { x2 } else { 0 } ^ 1;
        lhs == rhs
    }

    fn dbl_raw(&self, p: Pt) -> Pt {
        if p.inf || p.x == 0 {
            return INF;
        }
        let g = &self.gf;
        let l = p.x ^ g.mul(p.y, g.inv(p.x));
        let x3 = g.sqr(l) ^ l ^ self.a;
        let y3 = g.sqr(p.x) ^ g.mul(l ^ 1, x3);
        Pt {
            inf: false,
            x: x3,
            y: y3,
        }
    }

    fn add_raw(&self, p: Pt, q: Pt) -> Pt {
        if p.inf {
            return q;
        }
        if q.inf {
            return p;
        }
        if p.x == q.x {
            if p.y == q.y && p.x != 0 {
                return self.dbl_raw(p);
            }
            return INF;
        }
        let g = &self.gf;
        let l = g.mul(p.y ^ q.y, g.inv(p.x ^ q.x));
        let x3 = g.sqr(l) ^ l ^ p.x ^ q.x ^ self.a;
        let y3 = g.mul(l, p.x ^ x3) ^ x3 ^ p.y;
        Pt {
            inf: false,
            x: x3,
            y: y3,
        }
    }

    fn add(&self, ops: &mut Ops, p: Pt, q: Pt) -> Pt {
        ops.adds += 1;
        self.add_raw(p, q)
    }

    fn dbl(&self, ops: &mut Ops, p: Pt) -> Pt {
        ops.doubles += 1;
        self.dbl_raw(p)
    }

    /// `[k]P` by left-to-right double-and-add, counted as
    /// `ic_boundary::CountedGroup::mul`.
    fn mul(&self, ops: &mut Ops, p: Pt, k: u128) -> Pt {
        ops.smults += 1;
        if k == 0 || p.inf {
            return INF;
        }
        let bits = bits_of(k);
        let mut acc = p;
        for i in (0..bits - 1).rev() {
            acc = self.dbl(ops, acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(ops, acc, p);
            }
        }
        acc
    }

    /// The points above the abscissa x (both roots, or the single 2-torsion
    /// point at x = 0), with the native work of finding them.
    fn lift(&self, nat: &mut Nat, x: u128) -> Vec<Pt> {
        let g = &self.gf;
        if x == 0 {
            return vec![Pt {
                inf: false,
                x: 0,
                y: 1,
            }];
        }
        nat.inv += 1;
        nat.sqr += 1;
        nat.tr += 1;
        let c = x ^ self.a ^ g.sqr(g.inv(x));
        if g.tr(c) == 1 {
            return Vec::new();
        }
        nat.ht_sqr += g.n as u64 - 1;
        nat.mul += 1;
        let t = g.half_trace(c);
        let y = g.mul(x, t);
        vec![
            Pt { inf: false, x, y },
            Pt {
                inf: false,
                x,
                y: y ^ x,
            },
        ]
    }
}

/// The group data of `K_a / F_{2^n}`.
struct Info {
    n: u32,
    a: u8,
    low: u128,
    order: u128, // #E
    r: u128,
    h: u128,
    lambda: Option<u128>,
}

fn lucas_order(n: u32, t: i128) -> i128 {
    // #E(F_{2^n}) = 2^n + 1 - V_n, V_0 = 2, V_1 = t, V_{k+1} = t V_k - 2 V_{k-1}
    let (mut v0, mut v1) = (2i128, t);
    if n == 0 {
        return (1i128 << 0) + 1 - v0;
    }
    for _ in 1..n {
        let v2 = t * v1 - 2 * v0;
        v0 = v1;
        v1 = v2;
    }
    (1i128 << n) + 1 - v1
}

fn make_info(n: u32, a: u8) -> Result<Info, String> {
    let low = find_irreducible_sparse(n);
    let t: i128 = if a == 0 { -1 } else { 1 };
    let order = lucas_order(n, t) as u128;
    let (r, h) = if n == 83 {
        if a == 1 {
            (8_569_786_107_849_059u128, 1_128_547_018u128)
        } else {
            (2_417_851_639_230_796_216_685_689u128, 4u128)
        }
    } else {
        // trial division, largest prime factor must be simple
        let mut rem = order;
        let mut largest = 1u128;
        let mut mult = 0u32;
        let mut d = 2u128;
        while d * d <= rem {
            if rem % d == 0 {
                let mut e = 0;
                while rem % d == 0 {
                    rem /= d;
                    e += 1;
                }
                largest = d;
                mult = e;
            }
            d += if d == 2 { 1 } else { 2 };
        }
        if rem > 1 {
            largest = rem;
            mult = 1;
        }
        if mult != 1 {
            return Err(format!(
                "largest prime factor of #E = {order} is not simple"
            ));
        }
        (largest, order / largest)
    };
    assert_eq!(r * h, order, "r h != #E");
    assert!(is_prime(r), "r is not prime");
    if r <= h {
        return Err(format!(
            "no usable prime-order subgroup: largest prime factor r = {r} <= cofactor h = {h} (#E = {order})"
        ));
    }
    Ok(Info {
        n,
        a,
        low,
        order,
        r,
        h,
        lambda: None,
    })
}

/// Frobenius eigenvalue on the order-r subgroup, chosen with a seed point.
fn eigenvalue(curve: &Curve, info: &Info, seed: Pt) -> u128 {
    let r = info.r;
    let t = if info.a == 0 { r - 1 } else { 1 };
    let disc = (mulmod(t, t, r) + r - 8 % r) % r;
    let s = sqrt_mod(disc, r).expect("t^2 - 8 is not a square mod r");
    let inv2 = (r + 1) / 2;
    let l1 = mulmod((t + s) % r, inv2, r);
    let l2 = mulmod((t + r - s) % r, inv2, r);
    let mut scratch = Ops::default();
    let target = curve.frob(seed);
    for l in [l1, l2] {
        if curve.mul(&mut scratch, seed, l) == target {
            return l;
        }
    }
    panic!("neither root of x^2 - t x + 2 is the Frobenius eigenvalue");
}

// ───────────────────────────── the arms ──────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Arm {
    Cos, // compact-orbit-scan (incumbent)
    Cyc, // cyclotomic gamma-weight-w
    Onb, // Hamming-weight base in an (optimal) normal basis
}

impl Arm {
    fn name(&self) -> &'static str {
        match self {
            Arm::Cos => "cos",
            Arm::Cyc => "cyc",
            Arm::Onb => "onb",
        }
    }
}

/// gamma, the powers of gamma and the normal-basis atoms of the field.
struct Cyc {
    n: u32,
    p: u32,
    gamma_pow: Vec<u128>, // gamma^k, k in 0..p (empty if 2n+1 is not a suitable prime)
    onb: Vec<u128>,       // normal basis atoms beta_i
    onb_type2: bool,
}

fn is_small_prime(v: u32) -> bool {
    if v < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= v {
        if v % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn order_of_2_mod(p: u32) -> Option<u32> {
    let mut acc = 1u64;
    for k in 1..=p {
        acc = (acc * 2) % p as u64;
        if acc == 1 {
            return Some(k);
        }
    }
    None
}

fn rank_f2(mut rows: Vec<u128>) -> usize {
    let mut rank = 0;
    for bit in (0..128).rev() {
        let mut piv = None;
        for (i, r) in rows.iter().enumerate().skip(rank) {
            if (r >> bit) & 1 == 1 {
                piv = Some(i);
                break;
            }
        }
        if let Some(i) = piv {
            rows.swap(rank, i);
            let pr = rows[rank];
            for (j, r) in rows.iter_mut().enumerate() {
                if j != rank && (*r >> bit) & 1 == 1 {
                    *r ^= pr;
                }
            }
            rank += 1;
        }
    }
    rank
}

impl Cyc {
    fn new(gf: &Gf) -> Cyc {
        let n = gf.n;
        let p = 2 * n + 1;
        if is_small_prime(p) && order_of_2_mod(p) == Some(n) {
            // gamma: the least word among the powers of the first non-trivial
            // (2^n-1)/p-th power of 2, 3, 4, ... (as CyclotomicField::new).
            let group = (1u128 << n) - 1;
            assert_eq!(group % p as u128, 0);
            let e = group / p as u128;
            let mut gamma = 0u128;
            let mut g0 = 2u128;
            loop {
                let cand = gf.pow(g0, e);
                if cand != 1 && cand != 0 {
                    let mut least = cand;
                    let mut cur = cand;
                    for _ in 1..p {
                        cur = gf.mul(cur, cand);
                        if cur < least && cur != 1 {
                            least = cur;
                        }
                    }
                    gamma = least;
                    break;
                }
                g0 += 1;
            }
            let mut gamma_pow = Vec::with_capacity(p as usize);
            let mut cur = 1u128;
            for _ in 0..p {
                gamma_pow.push(cur);
                cur = gf.mul(cur, gamma);
            }
            assert_eq!(cur, 1, "gamma^p != 1");
            let mut onb = Vec::with_capacity(n as usize);
            let mut e2 = 1u64;
            for _ in 0..n {
                let e_us = e2 as usize;
                onb.push(gamma_pow[e_us] ^ gamma_pow[(p as usize - e_us) % p as usize]);
                e2 = (e2 * 2) % p as u64;
            }
            assert_eq!(
                rank_f2(onb.clone()),
                n as usize,
                "type-II ONB is not a basis"
            );
            Cyc {
                n,
                p,
                gamma_pow,
                onb,
                onb_type2: true,
            }
        } else {
            // A generic normal basis: the least word theta whose conjugates are
            // linearly independent.
            let mut theta = 1u128;
            let basis = loop {
                theta += 1;
                let mut v = Vec::with_capacity(n as usize);
                let mut cur = theta;
                for _ in 0..n {
                    v.push(cur);
                    cur = gf.sqr(cur);
                }
                if rank_f2(v.clone()) == n as usize {
                    break v;
                }
            };
            Cyc {
                n,
                p: 0,
                gamma_pow: Vec::new(),
                onb: basis,
                onb_type2: false,
            }
        }
    }
}

/// An abscissa source.
enum Source<'a> {
    Scan(u128),                               // raw x = next, next+1, ...
    Rng(u64),                                 // uniform random raw x
    Sorted(&'a [u128], usize),                // a pre-enumerated sorted set
    Lex(&'a [u128], usize, Vec<usize>, bool), // lazily enumerated w-subsets of atoms
}

struct Counters {
    ops: Ops,
    nat: Nat,
    visited: u64,   // abscissae drawn from the source
    lifts: u64,     // points lifted
    projected: u64, // points surviving the cofactor projection
    columns: usize,
}

impl Counters {
    fn new() -> Counters {
        Counters {
            ops: Ops::default(),
            nat: Nat::default(),
            visited: 0,
            lifts: 0,
            projected: 0,
            columns: 0,
        }
    }
}

/// The weight-w sums of the atoms, sorted by word and deduplicated (the set
/// the production plug-ins enumerate before scanning it).
fn weight_set(gf: &Gf, atoms: &[u128], w: usize, nat: &mut Nat) -> Vec<u128> {
    let m = atoms.len();
    let mut out: Vec<u128> = Vec::with_capacity(binomial(m as u64, w as u64) as usize);
    let mut idx: Vec<usize> = (0..w).collect();
    let _ = gf;
    loop {
        let mut acc = 0u128;
        for &i in &idx {
            acc ^= atoms[i];
        }
        out.push(acc);
        // next combination
        let mut i = w;
        loop {
            if i == 0 {
                nat.fadd += (out.len() * w) as u64;
                let cnt = out.len() as u64;
                nat.sort += cnt * (64 - cnt.leading_zeros() as u64);
                out.sort_unstable();
                out.dedup();
                return out;
            }
            i -= 1;
            if idx[i] != i + m - w {
                break;
            }
            if i == 0 {
                nat.fadd += (out.len() * w) as u64;
                let cnt = out.len() as u64;
                nat.sort += cnt * (64 - cnt.leading_zeros() as u64);
                out.sort_unstable();
                out.dedup();
                return out;
            }
        }
        idx[i] += 1;
        for j in i + 1..w {
            idx[j] = idx[j - 1] + 1;
        }
    }
}

struct Built {
    seeds: Vec<Pt>,
    c: Counters,
    exhausted: bool,
}

/// Faithful accounting: the production plug-in loops.  `cos` adds the
/// `seed.x <= 1` skip and the Frobenius-eigenvalue check; `cap == 0` takes
/// every column of the source (cyclotomic / ONB without `columns`).
fn build_faithful(
    curve: &Curve,
    info: &Info,
    src: &mut Source,
    cap: usize,
    cos_rules: bool,
    lambda: &mut Option<u128>,
) -> Built {
    let n = info.n as u64;
    let mut c = Counters::new();
    let mut seen: HashSet<u128> = HashSet::new();
    let mut seeds: Vec<Pt> = Vec::new();
    let mut raw_limit = (1u128 << info.n) - 1;
    if raw_limit == 0 {
        raw_limit = 1;
    }
    'outer: loop {
        // next abscissa
        let x = match src {
            Source::Scan(next) => {
                if *next > raw_limit {
                    break;
                }
                let v = *next;
                *next += 1;
                v
            }
            Source::Rng(state) => 1 + rand_u128(state) % raw_limit,
            Source::Sorted(v, pos) => {
                if *pos >= v.len() {
                    break;
                }
                let x = v[*pos];
                *pos += 1;
                x
            }
            Source::Lex(..) => panic!("faithful accounting uses sorted sources"),
        };
        c.visited += 1;
        let pts = curve.lift(&mut c.nat, x);
        for p in pts {
            c.lifts += 1;
            let seed = curve.mul(&mut c.ops, p, info.h);
            if seed.inf || (cos_rules && seed.x <= 1) {
                continue;
            }
            c.projected += 1;
            let (key, _) = curve.gf.canon(seed.x);
            c.nat.key_sqr += n - 1;
            c.nat.hash += 1;
            if !seen.insert(key) {
                continue;
            }
            // order check
            let chk = curve.mul(&mut c.ops, seed, info.r);
            assert!(chk.inf, "projected point has wrong order");
            if cos_rules {
                let lam = *lambda.get_or_insert_with(|| eigenvalue(curve, info, seed));
                let m = curve.mul(&mut c.ops, seed, lam);
                assert!(m == curve.frob(seed), "eigenvalue check failed");
                c.nat.frob += 1;
            }
            // orbit closure bookkeeping (n Frobenius maps, n negations)
            c.nat.frob += n;
            c.nat.neg += n;
            c.nat.hash += 1;
            seeds.push(seed);
            if cap > 0 && seeds.len() == cap {
                break 'outer;
            }
        }
    }
    c.columns = seeds.len();
    let exhausted = cap > 0 && seeds.len() < cap;
    Built {
        seeds,
        c,
        exhausted,
    }
}

/// Tidy accounting: one visit per Frobenius orbit of raw abscissae, one root,
/// the cofactor projection, the orbit key of the projected abscissa.  No
/// order check, no eigenvalue check.  The source is lazy (no sort).
fn build_tidy(
    curve: &Curve,
    info: &Info,
    gen: &mut dyn FnMut(&mut Nat) -> Option<u128>,
    cap: usize,
    cos_rules: bool,
) -> Built {
    let n = info.n as u64;
    let mut c = Counters::new();
    let mut seen_raw: HashSet<u128> = HashSet::new();
    let mut seen: HashSet<u128> = HashSet::new();
    let mut seeds: Vec<Pt> = Vec::new();
    while let Some(x) = gen(&mut c.nat) {
        c.visited += 1;
        let (rk, _) = curve.gf.canon(x);
        c.nat.key_sqr += n - 1;
        c.nat.hash += 1;
        if !seen_raw.insert(rk) {
            continue;
        }
        let pts = curve.lift(&mut c.nat, x);
        let Some(&p) = pts.first() else { continue };
        c.lifts += 1;
        let seed = curve.mul(&mut c.ops, p, info.h);
        if seed.inf || (cos_rules && seed.x <= 1) {
            continue;
        }
        c.projected += 1;
        let (key, _) = curve.gf.canon(seed.x);
        c.nat.key_sqr += n - 1;
        c.nat.hash += 1;
        if !seen.insert(key) {
            continue;
        }
        c.nat.frob += n;
        c.nat.neg += n;
        c.nat.hash += 1;
        seeds.push(seed);
        if cap > 0 && seeds.len() == cap {
            break;
        }
    }
    c.columns = seeds.len();
    let exhausted = cap > 0 && seeds.len() < cap;
    Built {
        seeds,
        c,
        exhausted,
    }
}

/// Lazy random w-subset sums of the atoms (tidy accounting): a seeded stream of
/// w-subsets drawn uniformly, so consecutive draws are in different Frobenius
/// orbits with high probability (lexicographic order is not: {0,b} and {0,2b}
/// are one orbit).  Each draw costs w field additions.
struct SubsetGen<'a> {
    atoms: &'a [u128],
    w: usize,
    state: u64,
    drawn: u64,
    limit: u64,
}

impl<'a> SubsetGen<'a> {
    fn new(atoms: &'a [u128], w: usize, seed: u64) -> SubsetGen<'a> {
        // Stop after 64 times the number of subsets (never reached in practice).
        let total = binomial(atoms.len() as u64, w as u64);
        SubsetGen {
            atoms,
            w,
            state: seed,
            drawn: 0,
            limit: total.saturating_mul(64),
        }
    }
    fn next(&mut self, nat: &mut Nat) -> Option<u128> {
        if self.w == 0 || self.w > self.atoms.len() || self.drawn >= self.limit {
            return None;
        }
        self.drawn += 1;
        let mut chosen: Vec<usize> = Vec::with_capacity(self.w);
        while chosen.len() < self.w {
            let i = (splitmix(&mut self.state) % self.atoms.len() as u64) as usize;
            if !chosen.contains(&i) {
                chosen.push(i);
            }
        }
        let mut acc = 0u128;
        for &i in &chosen {
            acc ^= self.atoms[i];
        }
        nat.fadd += self.w as u64;
        Some(acc)
    }
}

// ───────────────────────────── pair table ────────────────────────────────

struct Pair {
    adds: u64,
    entries: u64,
    reps: u64,
    nat: Nat,
    map: HashMap<u128, (u32, u32, u32)>,
    points: Vec<Pt>,
}

fn point_index(o: usize, s: usize, sg: usize, n: usize) -> usize {
    o * 2 * n + 2 * s + sg
}

fn pair_table(curve: &Curve, info: &Info, seeds: &[Pt]) -> Pair {
    let n = info.n as usize;
    let k = seeds.len();
    let mut points: Vec<Pt> = Vec::with_capacity(2 * n * k);
    for &seed in seeds {
        let mut m = seed;
        for _ in 0..n {
            points.push(m);
            points.push(curve.neg(m));
            m = curve.frob(m);
        }
    }
    let f = points.len();
    let mut nat = Nat::default();
    // member_of table of the production code: f (n - 1) Frobenius maps and
    // lookups.
    nat.frob += (f * (n - 1)) as u64;
    nat.hash += (f * (n - 1)) as u64;
    let mut ops = Ops::default();
    let mut map: HashMap<u128, (u32, u32, u32)> = HashMap::new();
    for o in 0..k {
        let rep = point_index(o, 0, 0, n);
        for j in 0..f {
            if j / (2 * n) < o {
                continue;
            }
            let u = curve.add(&mut ops, points[rep], points[j]);
            if u.inf {
                continue;
            }
            let (key, t_u) = curve.gf.canon(u.x);
            nat.key_sqr += n as u64 - 1;
            nat.hash += 1;
            map.entry(key).or_insert((rep as u32, j as u32, t_u));
        }
    }
    Pair {
        adds: ops.adds,
        entries: map.len() as u64,
        reps: k as u64,
        nat,
        map,
        points,
    }
}

/// The member of the Frobenius class: sigma^t of base point i.
fn member(i: usize, t: usize, n: usize) -> usize {
    let o = i / (2 * n);
    let rem = i % (2 * n);
    let s = rem / 2;
    let sg = rem % 2;
    point_index(o, (s + t) % n, sg, n)
}

/// One probe of the pair table at target R; returns the number of group
/// additions it used (0 on a miss, 1 on a hit) and whether the decomposition
/// verified.
fn probe(
    curve: &Curve,
    info: &Info,
    pair: &Pair,
    nat: &mut Nat,
    rr: Pt,
) -> (u64, Option<(usize, usize)>) {
    let n = info.n as usize;
    let (key, t_r) = curve.gf.canon(rr.x);
    nat.key_sqr += n as u64 - 1;
    nat.hash += 1;
    let Some(&(i, j, t_u)) = pair.map.get(&key) else {
        return (0, None);
    };
    let t = (t_u as usize + n - t_r as usize) % n;
    let i2 = member(i as usize, t, n);
    let j2 = member(j as usize, t, n);
    let mut scratch = Ops::default();
    let s = curve.add(&mut scratch, pair.points[i2], pair.points[j2]);
    if s == rr {
        (1, Some((i2, j2)))
    } else if s == curve.neg(rr) {
        let ni = i2 ^ 1;
        let nj = j2 ^ 1;
        (1, Some((ni, nj)))
    } else {
        (1, None)
    }
}

// ───────────────────────────── cell driver ───────────────────────────────

fn json_ops(o: &Ops) -> String {
    format!(
        "{{\"adds\":{},\"doubles\":{},\"scalar_mults\":{},\"g\":{}}}",
        o.adds,
        o.doubles,
        o.smults,
        o.g()
    )
}

struct Args {
    mode: String,
    n: u32,
    a: u8,
    k: usize,
    cyc_w: usize,
    pair: bool,
    nulls: usize,
    seed: u64,
    targets: usize,
    probes: usize,
    arms: String,
    accounting: String,
    kfull: bool,
    files: String,
}

fn parse_args() -> Args {
    let mut a = Args {
        mode: "cell".into(),
        n: 23,
        a: 1,
        k: 0,
        cyc_w: 2,
        pair: false,
        nulls: 0,
        seed: 20261018,
        targets: 16,
        probes: 200,
        arms: "cos,cyc,onb".into(),
        accounting: "faithful,tidy".into(),
        kfull: false,
        files: String::new(),
    };
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut val = || it.next().expect("flag value");
        match flag.as_str() {
            "--mode" => a.mode = val(),
            "--n" => a.n = val().parse().unwrap(),
            "--a" => a.a = val().parse().unwrap(),
            "--k" => a.k = val().parse().unwrap(),
            "--cyc-w" => a.cyc_w = val().parse().unwrap(),
            "--pair" => a.pair = val() == "1",
            "--nulls" => a.nulls = val().parse().unwrap(),
            "--seed" => a.seed = val().parse().unwrap(),
            "--targets" => a.targets = val().parse().unwrap(),
            "--probes" => a.probes = val().parse().unwrap(),
            "--arms" => a.arms = val(),
            "--accounting" => a.accounting = val(),
            "--kfull" => a.kfull = val() == "1",
            "--files" => a.files = val(),
            other => panic!("unknown flag {other}"),
        }
    }
    a
}

struct World {
    info: Info,
    curve: Curve,
    cyc: Cyc,
}

fn make_world(n: u32, a: u8) -> World {
    match try_world(n, a) {
        Ok(w) => w,
        Err(e) => panic!("{e}"),
    }
}

fn try_world(n: u32, a: u8) -> Result<World, String> {
    let info = make_info(n, a)?;
    let gf = Gf::new(n, info.low);
    let cyc = Cyc::new(&gf);
    let t = if a == 0 { -1 } else { 1 };
    let curve = Curve {
        gf,
        a: a as u128,
        t,
    };
    Ok(World { info, curve, cyc })
}

/// Build one arm with one accounting.  `cap == 0` takes every column
/// (only meaningful for the cyclotomic and ONB sets).
fn build_arm(
    w: &mut World,
    arm: Arm,
    weight: usize,
    cap: usize,
    tidy: bool,
) -> Result<Built, String> {
    let cos_rules = arm == Arm::Cos;
    let info = &w.info;
    let curve = &w.curve;
    let mut lambda = w.info.lambda;
    let built = match (arm, tidy) {
        (Arm::Cos, false) => {
            if cap == 0 {
                return Err("compact-orbit-scan needs a column count".into());
            }
            let mut src = Source::Scan(1);
            build_faithful(curve, info, &mut src, cap, true, &mut lambda)
        }
        (Arm::Cos, true) => {
            if cap == 0 {
                return Err("compact-orbit-scan needs a column count".into());
            }
            let top = (1u128 << info.n) - 1;
            let mut next = 1u128;
            let mut gen = |_: &mut Nat| {
                if next > top {
                    None
                } else {
                    next += 1;
                    Some(next - 1)
                }
            };
            build_tidy(curve, info, &mut gen, cap, true)
        }
        (Arm::Cyc, _) | (Arm::Onb, _) => {
            let atoms: &[u128] = if arm == Arm::Cyc {
                if w.cyc.gamma_pow.is_empty() {
                    return Err("2n+1 is not a prime with ord(2) = n: no cyclotomic base".into());
                }
                &w.cyc.gamma_pow
            } else {
                &w.cyc.onb
            };
            if tidy {
                let mut lex = SubsetGen::new(
                    atoms,
                    weight,
                    0x5eed_17 ^ (info.n as u64) << 8 ^ (info.a as u64) << 16 ^ weight as u64,
                );
                let mut gen = |nat: &mut Nat| lex.next(nat);
                build_tidy(curve, info, &mut gen, cap, false)
            } else {
                let mut nat0 = Nat::default();
                let set = weight_set(&curve.gf, atoms, weight, &mut nat0);
                let mut src = Source::Sorted(&set, 0);
                let mut b = build_faithful(curve, info, &mut src, cap, false, &mut lambda);
                b.c.nat.merge(&nat0);
                b
            }
        }
    };
    let _ = cos_rules;
    w.info.lambda = lambda;
    if built.exhausted {
        return Err(format!(
            "source exhausted with {} of {} columns",
            built.c.columns, cap
        ));
    }
    if built.c.columns == 0 {
        return Err("no column".into());
    }
    Ok(built)
}

fn s_at(g: u64, nat: u64, c: f64) -> f64 {
    g as f64 + c * nat as f64
}

fn emit_build(w: &World, arm: Arm, weight: usize, cap: usize, tidy: bool, b: &Built, secs: f64) {
    let c = &b.c;
    println!(
        "{{\"kind\":\"build\",\"n\":{},\"a\":{},\"arm\":\"{}\",\"w\":{},\"k_req\":{},\"k\":{},\"accounting\":\"{}\",\"visited\":{},\"lifts\":{},\"projected\":{},\"ops\":{},\"nat\":{},\"s0\":{:.1},\"s01\":{:.1},\"s1\":{:.1},\"secs\":{:.3}}}",
        w.info.n,
        w.info.a,
        arm.name(),
        weight,
        cap,
        c.columns,
        if tidy { "tidy" } else { "faithful" },
        c.visited,
        c.lifts,
        c.projected,
        json_ops(&c.ops),
        c.nat.json(),
        s_at(c.ops.g(), c.nat.total(), 0.0),
        s_at(c.ops.g(), c.nat.total(), 0.1),
        s_at(c.ops.g(), c.nat.total(), 1.0),
        secs
    );
}

fn run_cell(args: &Args) {
    let mut w = match try_world(args.n, args.a) {
        Ok(w) => w,
        Err(e) => {
            println!(
                "{{\"kind\":\"no_curve\",\"n\":{},\"a\":{},\"why\":\"{}\"}}",
                args.n, args.a, e
            );
            return;
        }
    };
    let n = w.info.n;
    eprintln!(
        "cell n={} a={} f={} #E={} r={} h={} type2_onb={}",
        n,
        args.a,
        terms_string(n, w.info.low),
        w.info.order,
        w.info.r,
        w.info.h,
        w.cyc.onb_type2
    );
    println!(
        "{{\"kind\":\"curve\",\"n\":{},\"a\":{},\"poly\":\"{}\",\"order\":\"{}\",\"r\":\"{}\",\"h\":\"{}\",\"log2_r\":{:.3},\"type2_onb\":{}}}",
        n,
        args.a,
        terms_string(n, w.info.low),
        w.info.order,
        w.info.r,
        w.info.h,
        (w.info.r as f64).log2(),
        w.cyc.onb_type2
    );
    let arms: Vec<&str> = args.arms.split(',').collect();
    let acct: Vec<&str> = args.accounting.split(',').collect();

    // Column count K: the cyclotomic full base when --kfull 1, else --k.
    let mut kcols = args.k;
    let mut kfull_built: Option<(Built, f64)> = None;
    if args.kfull {
        let t0 = Instant::now();
        match build_arm(&mut w, Arm::Cyc, args.cyc_w, 0, false) {
            Ok(b) => {
                kcols = b.c.columns;
                kfull_built = Some((b, t0.elapsed().as_secs_f64()));
            }
            Err(e) => {
                println!(
                    "{{\"kind\":\"no_base\",\"n\":{},\"a\":{},\"arm\":\"cyc\",\"w\":{},\"why\":\"{}\"}}",
                    n, args.a, args.cyc_w, e
                );
                return;
            }
        }
    }
    assert!(kcols > 0, "need --k or --kfull 1");

    let mut base_for_pair: HashMap<&'static str, Vec<Pt>> = HashMap::new();
    let mut onb_w_used = 0usize;
    for arm_name in &arms {
        let arm = match *arm_name {
            "cos" => Arm::Cos,
            "cyc" => Arm::Cyc,
            "onb" => Arm::Onb,
            other => panic!("arm {other}"),
        };
        // weight: cyc as given; onb the least weight reaching kcols
        let weights: Vec<usize> = match arm {
            Arm::Cos => vec![0],
            Arm::Cyc => vec![args.cyc_w],
            Arm::Onb => (1..=(n as usize / 2)).collect(),
        };
        let mut done = false;
        for wt in weights {
            for acc in &acct {
                let tidy = *acc == "tidy";
                let t0 = Instant::now();
                // Production semantics: `cyclotomic:w=W` without `columns` takes the
                // whole set; the tidy accounting always stops at K columns.
                let cap = if arm == Arm::Cyc && !tidy && args.kfull {
                    0
                } else {
                    kcols
                };
                let res = if cap == 0 && arm == Arm::Cyc {
                    match kfull_built.take() {
                        Some((b, _)) => Ok(b),
                        None => build_arm(&mut w, arm, wt, cap, tidy),
                    }
                } else {
                    build_arm(&mut w, arm, wt, cap, tidy)
                };
                match res {
                    Ok(b) => {
                        emit_build(&w, arm, wt, cap, tidy, &b, t0.elapsed().as_secs_f64());
                        if !tidy {
                            base_for_pair.insert(arm.name(), b.seeds.clone());
                            if arm == Arm::Onb {
                                onb_w_used = wt;
                            }
                        }
                        done = true;
                    }
                    Err(e) => {
                        if arm != Arm::Onb {
                            println!(
                                "{{\"kind\":\"no_base\",\"n\":{},\"a\":{},\"arm\":\"{}\",\"w\":{},\"accounting\":\"{}\",\"why\":\"{}\"}}",
                                n, args.a, arm.name(), wt, acc, e
                            );
                            done = true;
                        } else {
                            done = false;
                            break;
                        }
                    }
                }
            }
            if done {
                break;
            }
        }
        if !done {
            println!(
                "{{\"kind\":\"no_base\",\"n\":{},\"a\":{},\"arm\":\"{}\",\"why\":\"no weight up to n/2 reaches {} columns\"}}",
                n, args.a, arm.name(), kcols
            );
        }
    }
    let _ = onb_w_used;

    // negative-control population of generic bases (random scan order)
    if args.nulls > 0 {
        let mut gs_f = Vec::new();
        let mut ns_f = Vec::new();
        let mut gs_t = Vec::new();
        let mut ns_t = Vec::new();
        for d in 0..args.nulls {
            let st = args.seed ^ ((d as u64 + 1).wrapping_mul(0x2545_f491_4f6c_dd1d)) ^ (n as u64);
            let mut st2 = st;
            let mut lam = w.info.lambda;
            let mut src = Source::Rng(st);
            let bf = build_faithful(&w.curve, &w.info, &mut src, kcols, true, &mut lam);
            w.info.lambda = lam;
            let mut gen = |_: &mut Nat| {
                let top = (1u128 << n) - 1;
                Some(1 + rand_u128(&mut st2) % top)
            };
            let bt = build_tidy(&w.curve, &w.info, &mut gen, kcols, true);
            gs_f.push(bf.c.ops.g());
            ns_f.push(bf.c.nat.total());
            gs_t.push(bt.c.ops.g());
            ns_t.push(bt.c.nat.total());
        }
        let arr = |v: &Vec<u64>| {
            format!(
                "[{}]",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };
        println!(
            "{{\"kind\":\"null\",\"n\":{},\"a\":{},\"k\":{},\"draws\":{},\"faithful_g\":{},\"faithful_nat\":{},\"tidy_g\":{},\"tidy_nat\":{}}}",
            n, args.a, kcols, args.nulls, arr(&gs_f), arr(&ns_f), arr(&gs_t), arr(&ns_t)
        );
    }

    // pair table, probes and controls for each arm that has a base
    if args.pair {
        for arm_name in &arms {
            let Some(seeds) = base_for_pair.get(*arm_name) else {
                continue;
            };
            let t0 = Instant::now();
            let pair = pair_table(&w.curve, &w.info, seeds);
            let secs_pair = t0.elapsed().as_secs_f64();
            let nn = n as u64;
            let k = seeds.len() as u64;
            let y_pred = 2.0 * nn as f64 * pair.entries as f64 / (w.info.r as f64 - 1.0);
            let mut ops_setup = Ops::default();
            ops_setup.adds = pair.adds;
            println!(
                "{{\"kind\":\"pair\",\"n\":{},\"a\":{},\"arm\":\"{}\",\"k\":{},\"adds\":{},\"expected_adds\":{},\"entries\":{},\"reps\":{},\"nat\":{},\"s0\":{},\"s01\":{:.1},\"s1\":{:.1},\"y_pred\":{:.6e},\"secs\":{:.3}}}",
                n, args.a, arm_name, k, pair.adds, nn * k * (k + 1), pair.entries, pair.reps,
                pair.nat.json(), pair.adds, pair.adds as f64 + 0.1 * pair.nat.total() as f64,
                pair.adds as f64 + pair.nat.total() as f64, y_pred, secs_pair
            );
            // controls
            let mut st = args.seed ^ 0x17_17_17 ^ (n as u64) << 8 ^ (args.a as u64);
            let np = pair.points.len();
            let mut planted_found = 0usize;
            let mut planted_nat = Nat::default();
            for _ in 0..args.targets {
                let i = (splitmix(&mut st) % np as u64) as usize;
                let mut j = (splitmix(&mut st) % np as u64) as usize;
                while j / 2 == i / 2 {
                    j = (splitmix(&mut st) % np as u64) as usize;
                }
                let mut scratch = Ops::default();
                let rr = w.curve.add(&mut scratch, pair.points[i], pair.points[j]);
                if rr.inf {
                    continue;
                }
                let (_, found) = probe(&w.curve, &w.info, &pair, &mut planted_nat, rr);
                if let Some((a, b)) = found {
                    let mut sc = Ops::default();
                    let s = w.curve.add(&mut sc, pair.points[a], pair.points[b]);
                    if s == rr {
                        planted_found += 1;
                    }
                }
            }
            // random targets, stride probes
            let mut hits = 0u64;
            let mut probes_done = 0u64;
            let mut hit_adds = 0u64;
            let mut rnat = Nat::default();
            let g0 = seeds[0];
            let mut verified_hits = 0u64;
            for _ in 0..args.targets {
                let kk = 1 + (rand_u128(&mut st) % (w.info.r - 1));
                let mut scratch = Ops::default();
                let mut t = w.curve.mul(&mut scratch, g0, kk);
                for _ in 0..args.probes {
                    probes_done += 1;
                    if !t.inf {
                        let (ad, found) = probe(&w.curve, &w.info, &pair, &mut rnat, t);
                        hit_adds += ad;
                        if let Some((a, b)) = found {
                            hits += 1;
                            let mut sc = Ops::default();
                            let s = w.curve.add(&mut sc, pair.points[a], pair.points[b]);
                            if s == t {
                                verified_hits += 1;
                            }
                        }
                    }
                    t = w.curve.add(&mut scratch, t, g0);
                }
            }
            println!(
                "{{\"kind\":\"controls\",\"n\":{},\"a\":{},\"arm\":\"{}\",\"k\":{},\"planted_targets\":{},\"planted_found\":{},\"random_targets\":{},\"probes\":{},\"hits\":{},\"verified_hits\":{},\"expected_hits\":{:.4},\"probe_nat_each\":{:.2},\"probe_adds_per_probe\":{:.6},\"stride_adds_per_probe\":1}}",
                n, args.a, arm_name, k, args.targets, planted_found, args.targets, probes_done, hits,
                verified_hits, y_pred * probes_done as f64,
                rnat.total() as f64 / probes_done as f64,
                hit_adds as f64 / probes_done as f64
            );
        }
    }
}

// ───────────────────────────── validation ────────────────────────────────

struct Expect {
    n: u32,
    a: u8,
    arm: Arm,
    weight: usize,
    cap: usize,
    adds: u64,
    doubles: u64,
    smults: u64,
    projected: u64,
    lifts: u64,
    k: usize,
    entries: u64,
}

fn run_validate() -> bool {
    // Counters of the committed ecbench records of
    // research/cyclotomic_falsifier_20261015/sessions/{a1,a0}-n23/records.jsonl
    // (factor_base phase adds/doubles/scalar_mults, cofactor_projected_points,
    // point_lifts_uncharged, oracle_setup pair_table_entries).
    let cases = [
        Expect {
            n: 23,
            a: 1,
            arm: Arm::Cyc,
            weight: 1,
            cap: 0,
            adds: 6,
            doubles: 114,
            smults: 93,
            projected: 92,
            lifts: 92,
            k: 1,
            entries: 20,
        },
        Expect {
            n: 23,
            a: 1,
            arm: Arm::Cos,
            weight: 0,
            cap: 1,
            adds: 12,
            doubles: 41,
            smults: 3,
            projected: 1,
            lifts: 1,
            k: 1,
            entries: 20,
        },
        Expect {
            n: 23,
            a: 1,
            arm: Arm::Cyc,
            weight: 2,
            cap: 0,
            adds: 180,
            doubles: 2040,
            smults: 1410,
            projected: 1380,
            lifts: 1380,
            k: 30,
            entries: 18452,
        },
        Expect {
            n: 23,
            a: 1,
            arm: Arm::Cos,
            weight: 0,
            cap: 30,
            adds: 360,
            doubles: 1273,
            smults: 133,
            projected: 73,
            lifts: 73,
            k: 30,
            entries: 18463,
        },
        Expect {
            n: 23,
            a: 0,
            arm: Arm::Cyc,
            weight: 2,
            cap: 0,
            adds: 272,
            doubles: 1904,
            smults: 799,
            projected: 782,
            lifts: 782,
            k: 17,
            entries: 6115,
        },
        Expect {
            n: 23,
            a: 0,
            arm: Arm::Cos,
            weight: 0,
            cap: 17,
            adds: 374,
            doubles: 694,
            smults: 75,
            projected: 39,
            lifts: 41,
            k: 17,
            entries: 6125,
        },
        Expect {
            n: 23,
            a: 0,
            arm: Arm::Cyc,
            weight: 3,
            cap: 3,
            adds: 48,
            doubles: 70,
            smults: 8,
            projected: 5,
            lifts: 5,
            k: 3,
            entries: 198,
        },
        Expect {
            n: 23,
            a: 0,
            arm: Arm::Cos,
            weight: 0,
            cap: 3,
            adds: 66,
            doubles: 126,
            smults: 15,
            projected: 7,
            lifts: 9,
            k: 3,
            entries: 198,
        },
    ];
    let mut ok = true;
    for e in &cases {
        let mut w = make_world(e.n, e.a);
        let b = build_arm(&mut w, e.arm, e.weight, e.cap, false).expect("build");
        let pair = pair_table(&w.curve, &w.info, &b.seeds);
        let got = (
            b.c.ops.adds,
            b.c.ops.doubles,
            b.c.ops.smults,
            b.c.projected,
            b.c.lifts,
            b.c.columns,
            pair.entries,
        );
        let want = (
            e.adds,
            e.doubles,
            e.smults,
            e.projected,
            e.lifts,
            e.k,
            e.entries,
        );
        let pass = got == want && pair.adds == (e.n as u64) * (e.k as u64) * (e.k as u64 + 1);
        println!(
            "{{\"kind\":\"validate\",\"n\":{},\"a\":{},\"arm\":\"{}\",\"w\":{},\"cap\":{},\"got\":[{},{},{},{},{},{},{}],\"want\":[{},{},{},{},{},{},{}],\"pair_adds\":{},\"pass\":{}}}",
            e.n, e.a, e.arm.name(), e.weight, e.cap,
            got.0, got.1, got.2, got.3, got.4, got.5, got.6,
            want.0, want.1, want.2, want.3, want.4, want.5, want.6,
            pair.adds, pass
        );
        ok &= pass;
    }
    println!("{{\"kind\":\"validate_summary\",\"pass\":{ok}}}");
    ok
}

// ───────────────────────────── structural leg ────────────────────────────

/// ANF over d <= 6 variables: bit M of a u64 is the coefficient of the
/// monomial whose variable set is M.
fn anf_mul(a: u64, b: u64, d: usize) -> u64 {
    let mut out = 0u64;
    for ma in 0..(1usize << d) {
        if (a >> ma) & 1 == 0 {
            continue;
        }
        for mb in 0..(1usize << d) {
            if (b >> mb) & 1 == 1 {
                out ^= 1u64 << (ma | mb);
            }
        }
    }
    out
}

/// Symbolic field element: n coordinates, each an ANF over d variables.
fn sym_mul(prod_words: &[Vec<u128>], a: &[u64], b: &[u64], n: usize, d: usize) -> Vec<u64> {
    let mut out = vec![0u64; n];
    for i in 0..n {
        if a[i] == 0 {
            continue;
        }
        for j in 0..n {
            if b[j] == 0 {
                continue;
            }
            let pr = anf_mul(a[i], b[j], d);
            if pr == 0 {
                continue;
            }
            let word = prod_words[i][j];
            for (k, o) in out.iter_mut().enumerate() {
                if (word >> k) & 1 == 1 {
                    *o ^= pr;
                }
            }
        }
    }
    out
}

fn sym_sqr(gf: &Gf, a: &[u64], n: usize) -> Vec<u64> {
    // (sum a_i z^i)^2 = sum a_i z^(2i)  (a_i^2 = a_i as Boolean functions)
    let mut out = vec![0u64; n];
    for i in 0..n {
        if a[i] == 0 {
            continue;
        }
        let img = gf.sqr(1u128 << i);
        for (k, o) in out.iter_mut().enumerate() {
            if (img >> k) & 1 == 1 {
                *o ^= a[i];
            }
        }
    }
    out
}

/// Certify that the coordinates of z^p (p = 2n+1) contain a monomial of degree
/// popcount(p): restrict to d = popcount(p) free coordinates of z, the other
/// coordinates random constants, and test the all-variables monomial.
fn unity_degree_certificate(n: u32, trials: usize, seed: u64) -> (u32, usize, usize) {
    let low = find_irreducible_sparse(n);
    let gf = Gf::new(n, low);
    let p = 2 * n + 1;
    let d = p.count_ones() as usize;
    let nn = n as usize;
    // z^i * z^j words
    let mut xp = Vec::new();
    let mut cur = 1u128;
    for _ in 0..2 * nn {
        xp.push(cur);
        let carry = (cur >> (nn - 1)) & 1;
        cur = (cur << 1) & gf.mask;
        if carry == 1 {
            cur ^= gf.low;
        }
    }
    let prod_words: Vec<Vec<u128>> = (0..nn)
        .map(|i| (0..nn).map(|j| xp[i + j]).collect())
        .collect();
    let mut st = seed;
    let mut hit = 0usize;
    let full = (1u64 << (1u64 << d)) - 1; // not used for d = 6
    let _ = full;
    let top_monomial: u64 = 1u64 << ((1usize << d) - 1);
    for _ in 0..trials {
        // choose d distinct free coordinates
        let mut free: Vec<usize> = Vec::new();
        while free.len() < d {
            let c = (splitmix(&mut st) % nn as u64) as usize;
            if !free.contains(&c) {
                free.push(c);
            }
        }
        let mut z = vec![0u64; nn];
        for c in 0..nn {
            if let Some(pos) = free.iter().position(|&f| f == c) {
                z[c] = 1u64 << (1usize << pos); // the monomial x_pos
            } else if splitmix(&mut st) & 1 == 1 {
                z[c] = 1; // the constant 1
            }
        }
        // product of z^(2^j) over the set bits j of p
        let mut images: Vec<Vec<u64>> = Vec::new();
        let mut curz = z.clone();
        for j in 0..32 {
            if (p >> j) == 0 {
                break;
            }
            if (p >> j) & 1 == 1 {
                images.push(curz.clone());
            }
            curz = sym_sqr(&gf, &curz, nn);
        }
        let mut acc = images[0].clone();
        for im in &images[1..] {
            acc = sym_mul(&prod_words, &acc, im, nn, d);
        }
        if acc.iter().any(|&c| c & top_monomial != 0) {
            hit += 1;
        }
    }
    (d as u32, hit, trials)
}

fn run_degree(args: &Args) {
    // Variables and degree of the m = 2 descended membership systems.
    let cells: Vec<(u32, u8)> = vec![
        (11, 1),
        (23, 1),
        (23, 0),
        (47, 1),
        (47, 0),
        (83, 1),
        (83, 0),
    ];
    for (n, a) in cells {
        if args.n != 0 && n != args.n {
            continue;
        }
        let mut w = match try_world(n, a) {
            Ok(w) => w,
            Err(e) => {
                println!("{{\"kind\":\"degree\",\"n\":{n},\"a\":{a},\"no_curve\":\"{e}\"}}");
                continue;
            }
        };
        let nn = n as u64;
        let p = 2 * n + 1;
        let cyc_ok = !w.cyc.gamma_pow.is_empty();
        let mut line = format!("{{\"kind\":\"degree\",\"n\":{n},\"a\":{a},\"cyc\":{cyc_ok}");
        if cyc_ok {
            let (d, hit, trials) = unity_degree_certificate(n, 300, args.seed ^ n as u64);
            line.push_str(&format!(
                ",\"p\":{p},\"popcount_p\":{},\"unity_degree_cert\":{{\"d\":{d},\"top_monomial_hits\":{hit},\"trials\":{trials}}}",
                p.count_ones()
            ));
        }
        // matched-weight table: columns of cyc w=1,2 and the least ONB weight
        let mut cols = String::new();
        let mut kmax = 0usize;
        for wt in [1usize, 2] {
            if cyc_ok {
                if let Ok(b) = build_arm(&mut w, Arm::Cyc, wt, 0, false) {
                    cols.push_str(&format!("\"cyc_w{wt}_columns\":{},", b.c.columns));
                    if wt == 2 {
                        kmax = b.c.columns;
                    }
                }
            }
        }
        if kmax == 0 {
            kmax = args.k;
        }
        if kmax > 0 {
            for wt in 1..=(n as usize / 2) {
                if build_arm(&mut w, Arm::Onb, wt, kmax, false).is_ok() {
                    // degree of the ONB membership system: w + 1
                    cols.push_str(&format!(
                        "\"onb_matched_w\":{wt},\"onb_degree\":{},\"onb_vars_m2\":{},",
                        wt + 1,
                        2 * nn
                    ));
                    break;
                }
            }
        }
        line.push_str(&format!(
            ",{}\"cyc_vars_m2_w1\":{},\"cyc_vars_m2_w2\":{},\"max_vars_production\":64,\"cyc_degree\":{}}}",
            cols,
            2 * nn,
            4 * nn,
            if cyc_ok { p.count_ones() } else { 0 }
        ));
        println!("{line}");
    }
}

// ───────────────────────────── the frozen rule ───────────────────────────
//
// `--mode rule --files a.jsonl,b.jsonl,...` applies PROTOCOL.md "Decision
// rule" to the instrument's own output.  Mechanical: ratios, an envelope,
// classes.  Interpretation is the reader's.

#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

struct JP<'a> {
    b: &'a [u8],
    i: usize,
}

impl JP<'_> {
    fn ws(&mut self) {
        while self.i < self.b.len() && (self.b[self.i] as char).is_whitespace() {
            self.i += 1;
        }
    }
    fn val(&mut self) -> Option<J> {
        self.ws();
        let c = *self.b.get(self.i)?;
        match c {
            b'{' => {
                self.i += 1;
                let mut kv = Vec::new();
                loop {
                    self.ws();
                    if self.b.get(self.i) == Some(&b'}') {
                        self.i += 1;
                        return Some(J::Obj(kv));
                    }
                    let k = match self.val()? {
                        J::Str(s) => s,
                        _ => return None,
                    };
                    self.ws();
                    if self.b.get(self.i) != Some(&b':') {
                        return None;
                    }
                    self.i += 1;
                    let v = self.val()?;
                    kv.push((k, v));
                    self.ws();
                    if self.b.get(self.i) == Some(&b',') {
                        self.i += 1;
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                loop {
                    self.ws();
                    if self.b.get(self.i) == Some(&b']') {
                        self.i += 1;
                        return Some(J::Arr(v));
                    }
                    v.push(self.val()?);
                    self.ws();
                    if self.b.get(self.i) == Some(&b',') {
                        self.i += 1;
                    }
                }
            }
            b'"' => {
                self.i += 1;
                let st = self.i;
                while *self.b.get(self.i)? != b'"' {
                    self.i += 1;
                }
                let s = String::from_utf8_lossy(&self.b[st..self.i]).to_string();
                self.i += 1;
                Some(J::Str(s))
            }
            b't' => {
                self.i += 4;
                Some(J::Bool(true))
            }
            b'f' => {
                self.i += 5;
                Some(J::Bool(false))
            }
            b'n' => {
                self.i += 4;
                Some(J::Null)
            }
            _ => {
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
                    .ok()?
                    .parse::<f64>()
                    .ok()
                    .map(J::Num)
            }
        }
    }
}

impl J {
    fn get(&self, k: &str) -> Option<&J> {
        if let J::Obj(kv) = self {
            kv.iter().find(|(a, _)| a == k).map(|(_, v)| v)
        } else {
            None
        }
    }
    fn num(&self) -> f64 {
        if let J::Num(x) = self {
            *x
        } else {
            f64::NAN
        }
    }
    fn text(&self) -> String {
        if let J::Str(s) = self {
            s.clone()
        } else {
            String::new()
        }
    }
    fn nums(&self) -> Vec<f64> {
        if let J::Arr(v) = self {
            v.iter().map(|x| x.num()).collect()
        } else {
            Vec::new()
        }
    }
    fn n(&self, k: &str) -> f64 {
        self.get(k).map(|x| x.num()).unwrap_or(f64::NAN)
    }
    fn s(&self, k: &str) -> String {
        self.get(k).map(|x| x.text()).unwrap_or_default()
    }
}

#[derive(Default, Clone)]
struct CellData {
    // (arm, accounting) -> (g, nat)
    builds: Vec<(String, String, f64, f64)>,
    // arm -> (adds, nat, entries)
    pairs: Vec<(String, f64, f64, f64)>,
    null: Option<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)>,
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    let i = ((sorted.len() - 1) as f64 * q).round() as usize;
    sorted[i]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    Lower,
    Higher,
    Within,
}

fn class_name(c: Class) -> &'static str {
    match c {
        Class::Lower => "LOWER",
        Class::Higher => "HIGHER",
        Class::Within => "WITHIN",
    }
}

fn pool(classes: &[Class]) -> &'static str {
    if classes.iter().all(|&c| c == Class::Lower) {
        "ADVANCES"
    } else if classes.iter().all(|&c| c == Class::Higher) {
        "REGRESSES"
    } else if classes.iter().all(|&c| c == Class::Within) {
        "MATCHES"
    } else {
        "TRADES"
    }
}

fn run_rule(files: &str) -> bool {
    let mut cells: Vec<((u32, u32, usize), CellData)> = Vec::new();
    let mut validate_pass = None;
    let mut planted_all = true;
    let mut planted_records = 0usize;
    let mut find =
        |cells: &mut Vec<((u32, u32, usize), CellData)>, key: (u32, u32, usize)| -> usize {
            if let Some(i) = cells.iter().position(|(k, _)| *k == key) {
                i
            } else {
                cells.push((key, CellData::default()));
                cells.len() - 1
            }
        };
    for f in files.split(',').filter(|f| !f.is_empty()) {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        for line in text.lines() {
            if !line.starts_with('{') {
                continue;
            }
            let Some(j) = (JP {
                b: line.as_bytes(),
                i: 0,
            })
            .val() else {
                continue;
            };
            let kind = j.s("kind");
            let key = (j.n("n") as u32, j.n("a") as u32, j.n("k") as usize);
            match kind.as_str() {
                "validate_summary" => {
                    validate_pass = Some(matches!(j.get("pass"), Some(J::Bool(true))));
                }
                "controls" => {
                    planted_records += 1;
                    planted_all &= j.n("planted_found") == j.n("planted_targets")
                        && j.n("hits") == j.n("verified_hits");
                }
                "build" => {
                    let i = find(&mut cells, key);
                    let g = j.get("ops").map(|o| o.n("g")).unwrap_or(f64::NAN);
                    let nat = j.get("nat").map(|o| o.n("total")).unwrap_or(f64::NAN);
                    cells[i]
                        .1
                        .builds
                        .push((j.s("arm"), j.s("accounting"), g, nat));
                }
                "pair" => {
                    let i = find(&mut cells, key);
                    let nat = j.get("nat").map(|o| o.n("total")).unwrap_or(f64::NAN);
                    cells[i]
                        .1
                        .pairs
                        .push((j.s("arm"), j.n("adds"), nat, j.n("entries")));
                }
                "null" => {
                    let i = find(&mut cells, key);
                    let g = |k: &str| j.get(k).map(|x| x.nums()).unwrap_or_default();
                    cells[i].1.null = Some((
                        g("faithful_g"),
                        g("faithful_nat"),
                        g("tidy_g"),
                        g("tidy_nat"),
                    ));
                }
                _ => {}
            }
        }
    }
    cells.sort_by(|a, b| a.0.cmp(&b.0));
    let prices = [0.0f64, 0.1, 1.0];
    let mut headline: Vec<(String, Vec<Class>, Vec<Class>)> = Vec::new();
    let mut ok = true;
    println!(
        "{{\"kind\":\"rule_gate\",\"validate_pass\":{},\"planted_all_found\":{},\"control_records\":{}}}",
        match validate_pass {
            Some(true) => "true",
            Some(false) => "false",
            None => "null",
        },
        planted_all,
        planted_records
    );
    for ((n, a, k), cd) in &cells {
        let get_b = |arm: &str, acct: &str| {
            cd.builds
                .iter()
                .find(|b| b.0 == arm && b.1 == acct)
                .map(|b| (b.2, b.3))
        };
        let get_p = |arm: &str| {
            cd.pairs
                .iter()
                .find(|p| p.0 == arm)
                .map(|p| (p.1, p.2, p.3))
        };
        let Some((setup_g_cos, setup_n_cos, entries_cos)) = get_p("cos") else {
            continue;
        };
        for arm in ["cyc", "onb"] {
            let Some(_) = get_p(arm) else { continue };
            let mut per_acct: Vec<(&str, Vec<Class>)> = Vec::new();
            for acct in ["faithful", "tidy"] {
                let (Some(xb), Some(bb)) = (get_b(arm, acct), get_b("cos", acct)) else {
                    continue;
                };
                let (xs_g, xs_n, _) = get_p(arm).unwrap();
                let mut classes = Vec::new();
                for &c in &prices {
                    let s_ready = |g: f64, nat: f64, sg: f64, sn: f64| g + c * nat + sg + c * sn;
                    let sx = s_ready(xb.0, xb.1, xs_g, xs_n);
                    let sb = s_ready(bb.0, bb.1, setup_g_cos, setup_n_cos);
                    // envelope: incumbent and its generic null population
                    let (mut lo, mut hi) = (sb, sb);
                    if let Some((fg, fn_, tg, tn)) = &cd.null {
                        let (gv, nv) = if acct == "faithful" {
                            (fg, fn_)
                        } else {
                            (tg, tn)
                        };
                        let mut v: Vec<f64> = gv
                            .iter()
                            .zip(nv.iter())
                            .map(|(g, nn)| s_ready(*g, *nn, setup_g_cos, setup_n_cos))
                            .collect();
                        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        if !v.is_empty() {
                            lo = lo.min(quantile(&v, 0.025));
                            hi = hi.max(quantile(&v, 0.975));
                        }
                    }
                    let rho = sx / sb;
                    let class = if sx < lo && rho <= 0.99 {
                        Class::Lower
                    } else if sx > hi && rho >= 1.01 {
                        Class::Higher
                    } else {
                        Class::Within
                    };
                    // FB-only ratio, descriptive
                    let fb_rho = (xb.0 + c * xb.1) / (bb.0 + c * bb.1);
                    println!(
                        "{{\"kind\":\"rule_row\",\"n\":{n},\"a\":{a},\"k\":{k},\"arm\":\"{arm}\",\"accounting\":\"{acct}\",\"price\":{c},\"s_ready\":{sx:.1},\"s_ready_cos\":{sb:.1},\"ratio\":{rho:.6},\"envelope\":[{lo:.1},{hi:.1}],\"class\":\"{}\",\"fb_only_ratio\":{fb_rho:.4}}}",
                        class_name(class)
                    );
                    classes.push(class);
                }
                per_acct.push((acct, classes));
            }
            let faithful = per_acct
                .iter()
                .find(|p| p.0 == "faithful")
                .map(|p| p.1.clone());
            let tidy = per_acct.iter().find(|p| p.0 == "tidy").map(|p| p.1.clone());
            if let (Some(f), Some(t)) = (faithful, tidy) {
                println!(
                    "{{\"kind\":\"rule_cell\",\"n\":{n},\"a\":{a},\"k\":{k},\"arm\":\"{arm}\",\"faithful\":\"{}\",\"tidy\":\"{}\",\"cell_verdict\":\"{}\"}}",
                    pool(&f),
                    pool(&t),
                    cell_verdict(pool(&f), pool(&t))
                );
                if *n == 83 {
                    headline.push((format!("{arm}@{n}/{a}/{k}"), f, t));
                }
            }
            // modelled whole-solve S (not run) and the distance to rho
            if let (Some(xb), Some((sg, sn, ent))) = (get_b(arm, "faithful"), get_p(arm)) {
                let r_log2 = 0.0f64;
                let _ = r_log2;
                for &c in &prices {
                    let y = 2.0 * (*n as f64) * ent / (rho_r(*n, *a) - 1.0);
                    let probes = (*k as f64 + 1.0) / y;
                    let probe_cost = 1.0 + y + c * (*n as f64);
                    let s_model = xb.0 + c * xb.1 + sg + c * sn + probes * probe_cost;
                    let s_ref = (std::f64::consts::PI / 2.0).sqrt() / (2.0 * *n as f64).sqrt();
                    let s_units = s_model / rho_r(*n, *a).sqrt();
                    println!(
                        "{{\"kind\":\"model_row\",\"n\":{n},\"a\":{a},\"k\":{k},\"arm\":\"{arm}\",\"price\":{c},\"y\":{y:.4e},\"s_model\":{s_units:.4},\"rho_frob_theory_s\":{s_ref:.4},\"distance_remaining\":{:.2},\"note\":\"model, not run; rho side is the ecbench theory S, a floor (measured rho S was 2.37x theory at n = 23), with its native work unpriced, so this distance is an upper bound\"}}",
                        s_units / s_ref
                    );
                }
            }
            let _ = entries_cos;
        }
    }
    // positive control of the classifier at the n = 23 cells (c = 0): the
    // prior ecbench cost leg found cyclotomic costlier at both.
    let mut pc_ok = true;
    for a in [0u32, 1u32] {
        let mut found = false;
        for ((n, aa, _k), cd) in &cells {
            if *n == 23 && *aa == a {
                if let (Some(xb), Some(bb), Some(px), Some(pb)) = (
                    cd.builds.iter().find(|b| b.0 == "cyc" && b.1 == "faithful"),
                    cd.builds.iter().find(|b| b.0 == "cos" && b.1 == "faithful"),
                    cd.pairs.iter().find(|p| p.0 == "cyc"),
                    cd.pairs.iter().find(|p| p.0 == "cos"),
                ) {
                    let rho = (xb.2 + px.1) / (bb.2 + pb.1);
                    println!("{{\"kind\":\"classifier_positive_control\",\"a\":{a},\"s_ready_ratio_c0\":{rho:.4},\"prior_ecbench_ratio\":{},\"higher\":{}}}", if a == 1 { "1.022" } else { "1.122" }, rho >= 1.01);
                    found = true;
                    pc_ok &= rho >= 1.01;
                }
            }
        }
        if !found {
            pc_ok = false;
        }
    }
    let faithful_pool: Vec<Class> = Vec::new();
    let _ = faithful_pool;
    let mut cyc_f: Vec<&'static str> = Vec::new();
    for (name, f, t) in &headline {
        if name.starts_with("cyc") {
            cyc_f.push(cell_verdict(pool(f), pool(t)));
        }
    }
    let pooled = if cyc_f.is_empty() {
        "NO_HEADLINE_CELL"
    } else if cyc_f.iter().all(|v| *v == "ADVANCES") {
        "ADVANCES"
    } else if cyc_f.iter().all(|v| v.starts_with("REGRESSES")) {
        "REGRESSES"
    } else if cyc_f.iter().all(|v| *v == "MATCHES") {
        "MATCHES"
    } else {
        "TRADES"
    };
    let final_v = if validate_pass != Some(true) || !pc_ok || !planted_all || planted_records == 0 {
        "INCONCLUSIVE"
    } else {
        pooled
    };
    println!(
        "{{\"kind\":\"rule_verdict\",\"gate_validate\":{},\"gate_classifier_positive_control\":{},\"gate_planted\":{},\"headline_cells\":{},\"cell_verdicts\":[{}],\"verdict\":\"{}\",\"level\":\"constant (oracle-ready S = factor base + pair table, section 5a band)\"}}",
        validate_pass == Some(true),
        pc_ok,
        planted_all && planted_records > 0,
        cyc_f.len(),
        cyc_f.iter().map(|v| format!("\"{v}\"")).collect::<Vec<_>>().join(","),
        final_v
    );
    ok &= final_v != "INCONCLUSIVE";
    ok
}

/// Subgroup order r as f64 for the curves of the cells (for the model rows).
fn rho_r(n: u32, a: u32) -> f64 {
    match (n, a) {
        (23, 1) => 4196903.0,
        (23, 0) => 2095853.0,
        (47, 1) => 106781081677.0,
        (83, 1) => 8569786107849059.0,
        (83, 0) => 2417851639230796216685689.0,
        _ => f64::NAN,
    }
}

fn cell_verdict(faithful: &'static str, tidy: &'static str) -> &'static str {
    match (faithful, tidy) {
        ("ADVANCES", "ADVANCES") => "ADVANCES",
        ("REGRESSES", "REGRESSES") => "REGRESSES",
        ("REGRESSES", _) => "REGRESSES (production accounting only)",
        ("MATCHES", "ADVANCES") => "TRADES",
        ("MATCHES", _) => "MATCHES",
        _ => "TRADES",
    }
}

// ───────────────────────────── main ──────────────────────────────────────

fn main() {
    let args = parse_args();
    match args.mode.as_str() {
        "validate" => {
            let ok = run_validate();
            if !ok {
                std::process::exit(1);
            }
        }
        "cell" => run_cell(&args),
        "rule" => {
            if !run_rule(&args.files) {
                std::process::exit(1);
            }
        }
        "degree" => run_degree(&args),
        "curve" => {
            let w = match try_world(args.n, args.a) {
                Ok(w) => w,
                Err(e) => {
                    println!(
                        "{{\"kind\":\"no_curve\",\"n\":{},\"a\":{},\"why\":\"{}\"}}",
                        args.n, args.a, e
                    );
                    return;
                }
            };
            println!(
                "{{\"kind\":\"curve\",\"n\":{},\"a\":{},\"poly\":\"{}\",\"order\":\"{}\",\"r\":\"{}\",\"h\":\"{}\",\"type2_onb\":{}}}",
                args.n,
                args.a,
                terms_string(args.n, w.info.low),
                w.info.order,
                w.info.r,
                w.info.h,
                w.cyc.onb_type2
            );
        }
        other => panic!("unknown mode {other}"),
    }
}
