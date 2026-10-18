//! Exact decomposition coverage over F_{q^n} (odd and binary characteristic).
//!
//! Round 3, item 1 (`research/weil_descent_angles_20261018/PROTOCOL.md`,
//! Addendum A, Part D; Amendment B). For `E/F_{q^n}` with generic
//! coefficients and the Weil-descent factor base `F = {P : x(P) ∈ F_q}`,
//! compute exactly, over the whole group, the share `π` of points that are a
//! sum of `n` factor-base points, the mean number of multiset decompositions,
//! the Gaudry heuristic `1/n!` and the Poisson value `1 − e^{−λ}`; the same
//! for a random `x`-set of equal size (null control) and an interval `{±jG}`
//! (positive control).
//!
//! std-only; `rustc --edition 2021 -O examples/decomposition_coverage.rs`.
//! Usage: `decomposition_coverage <out-dir>`; env `DC_THREADS`, `DC_SCALE`.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

// Helpers reused from examples/genus4_res_census.rs (odd table field).
fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = (r as u128 * b as u128 % m as u128) as u64;
        }
        b = (b as u128 * b as u128 % m as u128) as u64;
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            out.push(d);
            while n.is_multiple_of(d) {
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

fn inv_mod_p(a: u32, p: u32) -> u32 {
    powmod(a as u64, p as u64 - 2, p as u64) as u32
}

fn poly_trim(a: &mut Vec<u32>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn poly_mulmod(a: &[u32], b: &[u32], f: &[u32], p: u32) -> Vec<u32> {
    let l = f.len() - 1;
    let mut prod = vec![0u64; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = (prod[i + j] + x as u64 * y as u64) % p as u64;
        }
    }
    // reduce modulo monic f of degree l
    for k in (l..prod.len()).rev() {
        let c = prod[k];
        if c == 0 {
            continue;
        }
        prod[k] = 0;
        for i in 0..l {
            prod[k - l + i] = (prod[k - l + i] + (p as u64 - f[i] as u64) * c) % p as u64;
        }
    }
    let mut out: Vec<u32> = prod.iter().take(l).map(|&v| v as u32).collect();
    poly_trim(&mut out);
    out
}

fn poly_powmod(a: &[u32], mut e: u64, f: &[u32], p: u32) -> Vec<u32> {
    let mut r = vec![1u32];
    let mut b = a.to_vec();
    while e > 0 {
        if e & 1 == 1 {
            r = poly_mulmod(&r, &b, f, p);
        }
        b = poly_mulmod(&b, &b, f, p);
        e >>= 1;
    }
    r
}

fn poly_rem(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = a.to_vec();
    poly_trim(&mut r);
    let db = b.len() - 1;
    let inv = inv_mod_p(b[db], p);
    while r.len() > db {
        let k = r.len() - 1;
        let c = r[k] * inv % p;
        if c != 0 {
            for i in 0..=db {
                r[k - db + i] = (r[k - db + i] + p - (c * b[i]) % p) % p;
            }
        }
        r.pop();
        poly_trim(&mut r);
    }
    r
}

fn poly_gcd(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    poly_trim(&mut a);
    poly_trim(&mut b);
    while !b.is_empty() {
        let r = poly_rem(&a, &b, p);
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let inv = inv_mod_p(lc, p);
        for c in a.iter_mut() {
            *c = *c * inv % p;
        }
    }
    a
}

fn poly_sub(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut out = vec![0u32; n];
    for (i, o) in out.iter_mut().enumerate() {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        *o = (x + p - y) % p;
    }
    poly_trim(&mut out);
    out
}

fn poly_irreducible(f: &[u32], p: u32) -> bool {
    let l = f.len() - 1;
    let x = vec![0u32, 1];
    let mut h = x.clone();
    for _ in 1..=l / 2 {
        h = poly_powmod(&h, p as u64, f, p);
        let d = poly_sub(&h, &x, p);
        let g = poly_gcd(f, &d, p);
        if g.len() != 1 {
            return false;
        }
    }
    true
}

const INF: u32 = u32::MAX;

struct Gf {
    p: u32,
    #[cfg_attr(not(test), allow(dead_code))]
    l: u32,
    nn: u32,
    #[cfg_attr(not(test), allow(dead_code))]
    exp: Vec<u32>,
    log: Vec<u32>,
    zech: Vec<u32>,
    half: u32,
}

impl Gf {
    fn new(p: u32, l: u32) -> Gf {
        assert!(p >= 3 && is_prime(p as u64));
        let size = (p as u64).pow(l) as u32;
        let nn = size - 1;
        let factors = prime_factors(nn as u64);
        let mut exp = vec![0u32; nn as usize];
        if l == 1 {
            let mut g = 2u32;
            loop {
                if factors
                    .iter()
                    .all(|&r| powmod(g as u64, nn as u64 / r, p as u64) != 1)
                {
                    break;
                }
                g += 1;
            }
            let mut cur = 1u64;
            for e in 0..nn {
                exp[e as usize] = cur as u32;
                cur = cur * g as u64 % p as u64;
            }
        } else {
            // irreducible monic polynomial of degree l
            let mut f = Vec::new();
            'search: for c in 0..size {
                let mut coeffs = vec![0u32; l as usize + 1];
                let mut v = c;
                for coeff in coeffs.iter_mut().take(l as usize) {
                    *coeff = v % p;
                    v /= p;
                }
                coeffs[l as usize] = 1;
                if coeffs[0] == 0 {
                    continue;
                }
                if poly_irreducible(&coeffs, p) {
                    f = coeffs;
                    break 'search;
                }
            }
            assert!(!f.is_empty());
            // primitive element: smallest non-constant digit index of full order
            let to_poly = |idx: u32| -> Vec<u32> {
                let mut v = idx;
                let mut c = Vec::new();
                for _ in 0..l {
                    c.push(v % p);
                    v /= p;
                }
                poly_trim(&mut c);
                c
            };
            let mut g = Vec::new();
            for idx in p..size {
                let cand = to_poly(idx);
                let ok = factors.iter().all(|&r| {
                    let pw = poly_powmod(&cand, nn as u64 / r, &f, p);
                    pw != vec![1u32]
                });
                if ok {
                    g = cand;
                    break;
                }
            }
            assert!(!g.is_empty());
            let mut cur = vec![1u32];
            for e in 0..nn {
                let mut idx = 0u32;
                let mut mulp = 1u32;
                for i in 0..l as usize {
                    idx += cur.get(i).copied().unwrap_or(0) * mulp;
                    mulp *= p;
                }
                exp[e as usize] = idx;
                cur = poly_mulmod(&cur, &g, &f, p);
            }
        }
        let mut log = vec![nn; size as usize];
        for e in 0..nn {
            log[exp[e as usize] as usize] = e;
        }
        // Zech: log(1 + g^d)
        let mut zech = vec![nn; nn as usize];
        for d in 0..nn {
            let idx = exp[d as usize];
            // add 1 to digit 0
            let d0 = idx % p;
            let idx1 = idx - d0 + (d0 + 1) % p;
            zech[d as usize] = log[idx1 as usize];
        }
        Gf {
            p,
            l,
            nn,
            exp,
            log,
            zech,
            half: nn / 2,
        }
    }

    #[inline]
    fn zero(&self) -> u32 {
        self.nn
    }
    #[inline]
    fn is_zero(&self, a: u32) -> bool {
        a == self.nn
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == self.nn || b == self.nn {
            return self.nn;
        }
        let s = a + b;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        debug_assert!(a != self.nn);
        if a == 0 {
            0
        } else {
            self.nn - a
        }
    }
    #[inline]
    fn neg(&self, a: u32) -> u32 {
        if a == self.nn {
            return a;
        }
        let s = a + self.half;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn add(&self, a: u32, b: u32) -> u32 {
        if a == self.nn {
            return b;
        }
        if b == self.nn {
            return a;
        }
        let d = if b >= a { b - a } else { b + self.nn - a };
        let z = self.zech[d as usize];
        if z == self.nn {
            return self.nn;
        }
        let s = a + z;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn sub(&self, a: u32, b: u32) -> u32 {
        self.add(a, self.neg(b))
    }
    /// Constant `c mod p` in log form.
    fn const_int(&self, c: i64) -> u32 {
        let c = c.rem_euclid(self.p as i64) as u32;
        self.log[c as usize]
    }
    /// `nn / (p^d - 1)`: the exponent step of the subfield `F_{p^d}`.
    fn step(&self, d: u32) -> u32 {
        self.nn / ((self.p as u64).pow(d) as u32 - 1)
    }
    fn in_sub(&self, a: u32, d: u32) -> bool {
        a == self.nn || a.is_multiple_of(self.step(d))
    }
    /// Quadratic character of `a` in `F_{p^d}` (`a` must lie there).
    fn chi_sub(&self, a: u32, step: u32) -> i32 {
        if a == self.nn {
            0
        } else if (a / step).is_multiple_of(2) {
            1
        } else {
            -1
        }
    }
    /// Frobenius `a -> a^{p^k}` on log-form elements; `INF` is fixed.
    fn frob(&self, a: u32, pk_mod: u64) -> u32 {
        if a == INF || a == self.nn {
            a
        } else {
            ((a as u64 * pk_mod) % self.nn as u64) as u32
        }
    }
    fn pk_mod(&self, k: u32) -> u64 {
        powmod(self.p as u64, k as u64, self.nn as u64)
    }
    /// Digit vector (ascending) of a log-form element (used by the tests).
    #[cfg_attr(not(test), allow(dead_code))]
    fn digits(&self, a: u32) -> Vec<u32> {
        let mut idx = if a == self.nn {
            0
        } else {
            self.exp[a as usize]
        };
        let mut out = Vec::new();
        for _ in 0..self.l {
            out.push(idx % self.p);
            idx /= self.p;
        }
        out
    }
}

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

// ---------------------------------------------------------------------------
// Binary table field F_{2^k} (k ≤ 21): integer (polynomial-basis) elements,
// log/exp tables, absolute trace, and a table solving z^2 + z = w.
// ---------------------------------------------------------------------------

struct Gf2 {
    k: u32,
    size: u32,
    modp: u32,
    exp: Vec<u32>,
    log: Vec<u32>,
    tr: Vec<u8>,
    /// half[w] = some z with z^2 + z = w, or u32::MAX
    half: Vec<u32>,
}

impl Gf2 {
    fn new(k: u32) -> Gf2 {
        let size = 1u32 << k;
        let nn = size - 1;
        // search a primitive polynomial of degree k
        let mut modp = 0u32;
        'outer: for low in (1u32..size).step_by(2) {
            let poly = (1u32 << k) | low;
            // multiplicative order of x must be nn
            let mut e = 1u32;
            let mut seen1 = false;
            for i in 1..=nn {
                e <<= 1;
                if e & size != 0 {
                    e ^= poly;
                }
                if e == 1 {
                    if i == nn {
                        seen1 = true;
                    }
                    break;
                }
            }
            if seen1 {
                modp = poly;
                break 'outer;
            }
        }
        assert!(modp != 0);
        let mut exp = vec![0u32; nn as usize];
        let mut log = vec![u32::MAX; size as usize];
        let mut e = 1u32;
        for i in 0..nn {
            exp[i as usize] = e;
            log[e as usize] = i;
            e <<= 1;
            if e & size != 0 {
                e ^= modp;
            }
        }
        let mut g = Gf2 {
            k,
            size,
            modp,
            exp,
            log,
            tr: vec![0; size as usize],
            half: vec![u32::MAX; size as usize],
        };
        // trace of basis elements, then linear extension
        let mut tb = vec![0u8; k as usize];
        for (i, t) in tb.iter_mut().enumerate() {
            let mut a = 1u32 << i;
            let mut s = 0u32;
            for _ in 0..k {
                s ^= a;
                a = g.mul(a, a);
            }
            assert!(s <= 1);
            *t = s as u8;
        }
        for a in 0..size {
            let mut s = 0u8;
            for (i, &t) in tb.iter().enumerate() {
                if a >> i & 1 == 1 {
                    s ^= t;
                }
            }
            g.tr[a as usize] = s;
        }
        for z in 0..size {
            let w = g.mul(z, z) ^ z;
            if g.half[w as usize] == u32::MAX {
                g.half[w as usize] = z;
            }
        }
        g
    }
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            return 0;
        }
        let nn = self.size - 1;
        let s = self.log[a as usize] + self.log[b as usize];
        self.exp[(if s >= nn { s - nn } else { s }) as usize]
    }
    fn inv(&self, a: u32) -> u32 {
        let nn = self.size - 1;
        let l = self.log[a as usize];
        self.exp[((nn - l) % nn) as usize]
    }
    fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    /// Elements of the subfield F_{2^m}.
    fn subfield(&self, m: u32) -> Vec<u32> {
        let nn = self.size - 1;
        let st = nn / ((1u32 << m) - 1);
        let mut v = vec![0u32];
        for j in 0..(1u32 << m) - 1 {
            v.push(self.exp[(j * st) as usize]);
        }
        v
    }
}

// ---------------------------------------------------------------------------
// Groups.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pt {
    O,
    A(u32, u32),
}

trait Group {
    fn add(&self, a: Pt, b: Pt) -> Pt;
    fn neg(&self, a: Pt) -> Pt;
    /// injective index into 0..domain()
    fn idx(&self, a: Pt) -> usize;
    fn domain(&self) -> usize;
    /// all points with the given x (0, 1 or 2 of them)
    fn points_at(&self, x: u32) -> Vec<Pt>;
    fn order(&self) -> u64;
    fn field_elems(&self) -> Vec<u32>;
    fn subfield_elems(&self) -> Vec<u32>;
}

/// y^2 = x^3 + a x + b over an odd table field (log-form elements).
struct OddCurve {
    g: Gf,
    m: u32, // subfield degree (factor base x ∈ F_{p^m}); here m = 1
    a: u32,
    b: u32,
    order: u64,
}

impl OddCurve {
    fn rhs(&self, x: u32) -> u32 {
        let g = &self.g;
        let x2 = g.mul(x, x);
        g.add(g.add(g.mul(x2, x), g.mul(self.a, x)), self.b)
    }
    fn new(p: u32, n: u32, rng: &mut u64) -> OddCurve {
        let g = Gf::new(p, n);
        let nn = g.nn;
        loop {
            let a = (xorshift(rng) % nn as u64) as u32;
            let b = (xorshift(rng) % nn as u64) as u32;
            // generic: a, b outside every proper subfield
            if (1..n).any(|d| n % d == 0 && (g.in_sub(a, d) || g.in_sub(b, d))) {
                continue;
            }
            let four_a3 = g.mul(g.const_int(4), g.mul(a, g.mul(a, a)));
            let tw_b2 = g.mul(g.const_int(27), g.mul(b, b));
            if g.is_zero(g.add(four_a3, tw_b2)) {
                continue;
            }
            let mut c = OddCurve {
                g,
                m: 1,
                a,
                b,
                order: 0,
            };
            let mut cnt = 1u64;
            for x in 0..=c.g.nn {
                let v = c.rhs(x);
                cnt += if c.g.is_zero(v) {
                    1
                } else if v % 2 == 0 {
                    2
                } else {
                    0
                };
            }
            c.order = cnt;
            return c;
        }
    }
    fn xi(&self, x: u32) -> usize {
        if x == self.g.nn {
            0
        } else {
            self.g.exp[x as usize] as usize
        }
    }
}

impl Group for OddCurve {
    fn add(&self, u: Pt, v: Pt) -> Pt {
        let g = &self.g;
        match (u, v) {
            (Pt::O, w) | (w, Pt::O) => w,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                let lam = if x1 == x2 {
                    if g.is_zero(g.add(y1, y2)) {
                        return Pt::O;
                    }
                    let num = g.add(g.mul(g.const_int(3), g.mul(x1, x1)), self.a);
                    g.mul(num, g.inv(g.mul(g.const_int(2), y1)))
                } else {
                    g.mul(g.sub(y2, y1), g.inv(g.sub(x2, x1)))
                };
                let x3 = g.sub(g.sub(g.mul(lam, lam), x1), x2);
                let y3 = g.sub(g.mul(lam, g.sub(x1, x3)), y1);
                Pt::A(x3, y3)
            }
        }
    }
    fn neg(&self, a: Pt) -> Pt {
        match a {
            Pt::O => Pt::O,
            Pt::A(x, y) => Pt::A(x, self.g.neg(y)),
        }
    }
    fn idx(&self, a: Pt) -> usize {
        match a {
            Pt::O => 0,
            Pt::A(x, y) => {
                let ny = self.g.neg(y);
                1 + 2 * self.xi(x) + usize::from(y > ny)
            }
        }
    }
    fn domain(&self) -> usize {
        2 * (self.g.nn as usize + 1) + 1
    }
    fn points_at(&self, x: u32) -> Vec<Pt> {
        let v = self.rhs(x);
        if self.g.is_zero(v) {
            vec![Pt::A(x, v)]
        } else if v % 2 == 0 {
            let y = v / 2;
            vec![Pt::A(x, y), Pt::A(x, self.g.neg(y))]
        } else {
            vec![]
        }
    }
    fn order(&self) -> u64 {
        self.order
    }
    fn field_elems(&self) -> Vec<u32> {
        (0..=self.g.nn).collect()
    }
    fn subfield_elems(&self) -> Vec<u32> {
        let st = self.g.step(self.m);
        let mut v = vec![self.g.nn];
        let cnt = self.g.p.pow(self.m) - 1;
        for j in 0..cnt {
            v.push(j * st);
        }
        v
    }
}

/// y^2 + x y = x^3 + a x^2 + b over F_{2^k} (ordinary).
struct BinCurve {
    g: Gf2,
    m: u32,
    a: u32,
    b: u32,
    sqrt_b: u32,
    order: u64,
}

impl BinCurve {
    fn new(m: u32, n: u32, rng: &mut u64) -> BinCurve {
        let g = Gf2::new(m * n);
        let size = g.size as u64;
        loop {
            let a = (xorshift(rng) % size) as u32;
            let b = 1 + (xorshift(rng) % (size - 1)) as u32;
            let generic = |v: u32| {
                (1..n).filter(|d| n % d == 0).all(|d| {
                    let sub = g.subfield(m * d);
                    !sub.contains(&v)
                })
            };
            if !generic(b) {
                continue;
            }
            let mut sb = b;
            for _ in 0..g.k - 1 {
                sb = g.sq(sb);
            }
            let mut c = BinCurve {
                g,
                m,
                a,
                b,
                sqrt_b: sb,
                order: 0,
            };
            let mut cnt = 2u64; // O and (0, sqrt b)
            for x in 1..c.g.size {
                let w = c.cval(x);
                if c.g.tr[w as usize] == 0 {
                    cnt += 2;
                }
            }
            c.order = cnt;
            return c;
        }
    }
    /// w = x + a + b/x^2 (for y = x z: z^2 + z = w)
    fn cval(&self, x: u32) -> u32 {
        let g = &self.g;
        x ^ self.a ^ g.mul(self.b, g.inv(g.sq(x)))
    }
}

impl Group for BinCurve {
    fn add(&self, u: Pt, v: Pt) -> Pt {
        let g = &self.g;
        match (u, v) {
            (Pt::O, w) | (w, Pt::O) => w,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                if x1 == x2 {
                    if y2 == (y1 ^ x1) {
                        return Pt::O; // P + (-P)
                    }
                    // doubling (y1 == y2)
                    if x1 == 0 {
                        return Pt::O;
                    }
                    let lam = x1 ^ g.mul(y1, g.inv(x1));
                    let x3 = g.sq(lam) ^ lam ^ self.a;
                    let y3 = g.sq(x1) ^ g.mul(lam ^ 1, x3);
                    return Pt::A(x3, y3);
                }
                let lam = g.mul(y1 ^ y2, g.inv(x1 ^ x2));
                let x3 = g.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a;
                let y3 = g.mul(lam, x1 ^ x3) ^ x3 ^ y1;
                Pt::A(x3, y3)
            }
        }
    }
    fn neg(&self, a: Pt) -> Pt {
        match a {
            Pt::O => Pt::O,
            Pt::A(x, y) => Pt::A(x, x ^ y),
        }
    }
    fn idx(&self, a: Pt) -> usize {
        match a {
            Pt::O => 0,
            Pt::A(x, y) => 1 + 2 * x as usize + usize::from(y > (x ^ y)),
        }
    }
    fn domain(&self) -> usize {
        2 * self.g.size as usize + 1
    }
    fn points_at(&self, x: u32) -> Vec<Pt> {
        if x == 0 {
            return vec![Pt::A(0, self.sqrt_b)];
        }
        let w = self.cval(x);
        let z = self.g.half[w as usize];
        if z == u32::MAX {
            return vec![];
        }
        let y = self.g.mul(x, z);
        vec![Pt::A(x, y), Pt::A(x, y ^ x)]
    }
    fn order(&self) -> u64 {
        self.order
    }
    fn field_elems(&self) -> Vec<u32> {
        (0..self.g.size).collect()
    }
    fn subfield_elems(&self) -> Vec<u32> {
        self.g.subfield(self.m)
    }
}

// ---------------------------------------------------------------------------
// Coverage of n-fold multiset sums.
// ---------------------------------------------------------------------------

fn sums_rec<G: Group>(
    g: &G,
    f: &[Pt],
    start: usize,
    depth: usize,
    acc: Pt,
    seen: &mut [u64],
    distinct: &mut u64,
    total: &mut u64,
) {
    if depth == 0 {
        let i = g.idx(acc);
        if seen[i / 64] >> (i % 64) & 1 == 0 {
            seen[i / 64] |= 1 << (i % 64);
            *distinct += 1;
        }
        *total += 1;
        return;
    }
    for k in start..f.len() {
        sums_rec(g, f, k, depth - 1, g.add(acc, f[k]), seen, distinct, total);
    }
}

fn coverage<G: Group>(g: &G, f: &[Pt], n: usize) -> (u64, u64) {
    let mut seen = vec![0u64; g.domain().div_ceil(64)];
    let (mut d, mut t) = (0u64, 0u64);
    sums_rec(g, f, 0, n, Pt::O, &mut seen, &mut d, &mut t);
    (d, t)
}

fn random_point<G: Group>(g: &G, rng: &mut u64) -> Pt {
    let elems = g.field_elems();
    loop {
        let x = elems[(xorshift(rng) % elems.len() as u64) as usize];
        if let Some(&p) = g.points_at(x).first() {
            return p;
        }
    }
}

#[derive(Clone, Debug)]
struct Row {
    char2: bool,
    n: usize,
    q: u64,
    curve: usize,
    order: u64,
    arm: &'static str,
    xs: usize,
    fsize: usize,
    lambda: f64,
    poisson: f64,
    distinct: u64,
    multisets: u64,
    pi: f64,
}

fn arms<G: Group>(g: &G, n: usize, rng: &mut u64) -> Vec<(&'static str, usize, Vec<Pt>)> {
    let sub = g.subfield_elems();
    let fs: Vec<Pt> = sub.iter().flat_map(|&x| g.points_at(x)).collect();
    let all = g.field_elems();
    let mut xs: Vec<u32> = Vec::new();
    let mut used = std::collections::HashSet::new();
    while xs.len() < sub.len() {
        let x = all[(xorshift(rng) % all.len() as u64) as usize];
        if used.insert(x) {
            xs.push(x);
        }
    }
    let fnull: Vec<Pt> = xs.iter().flat_map(|&x| g.points_at(x)).collect();
    let gen = random_point(g, rng);
    let mut fpos = Vec::new();
    let mut q = gen;
    for _ in 0..fs.len() / 2 {
        fpos.push(q);
        fpos.push(g.neg(q));
        q = g.add(q, gen);
    }
    let _ = n;
    vec![
        ("subfield", sub.len(), fs),
        ("null", xs.len(), fnull),
        ("positive", sub.len(), fpos),
    ]
}

fn n_multisets(k: usize, n: usize) -> f64 {
    // C(k + n - 1, n)
    let mut v = 1f64;
    for i in 0..n {
        v *= (k + i) as f64 / (i + 1) as f64;
    }
    v
}

fn run_curve<G: Group>(g: &G, char2: bool, n: usize, q: u64, ci: usize, rng: &mut u64) -> Vec<Row> {
    let mut rows = Vec::new();
    for (arm, xs, f) in arms(g, n, rng) {
        let (distinct, total) = coverage(g, &f, n);
        let order = g.order();
        let lambda = total as f64 / order as f64;
        debug_assert!((total as f64 - n_multisets(f.len(), n)).abs() < 0.5);
        rows.push(Row {
            char2,
            n,
            q,
            curve: ci,
            order,
            arm,
            xs,
            fsize: f.len(),
            lambda,
            poisson: 1.0 - (-lambda).exp(),
            distinct,
            multisets: total,
            pi: distinct as f64 / order as f64,
        });
    }
    rows
}

fn primes_in(lo: u32, hi: u32) -> Vec<u32> {
    (lo..=hi).filter(|&p| is_prime(p as u64)).collect()
}

#[derive(Clone, Debug)]
enum Job {
    Odd { p: u32, n: u32 },
    Bin { m: u32, n: u32 },
}

fn jobs(scale: f64) -> Vec<Job> {
    let s = |lo: u32, hi: u32| lo + ((hi - lo) as f64 * scale) as u32;
    let mut v = Vec::new();
    for p in primes_in(53, s(53, 1500)) {
        v.push(Job::Odd { p, n: 2 });
    }
    for p in primes_in(13, s(13, 101)) {
        v.push(Job::Odd { p, n: 3 });
    }
    for p in primes_in(5, s(5, 23)) {
        v.push(Job::Odd { p, n: 4 });
    }
    for p in primes_in(5, s(5, 11)) {
        v.push(Job::Odd { p, n: 5 });
    }
    for m in 4..=s(4, 10) {
        v.push(Job::Bin { m, n: 2 });
    }
    for m in 3..=s(3, 7) {
        v.push(Job::Bin { m, n: 3 });
    }
    for m in 3..=s(3, 5) {
        v.push(Job::Bin { m, n: 4 });
    }
    for m in 2..=s(2, 4) {
        v.push(Job::Bin { m, n: 5 });
    }
    v
}

fn run_job(job: &Job, seed: u64) -> Vec<Row> {
    let mut rng = seed | 1;
    let mut rows = Vec::new();
    for ci in 0..3 {
        match *job {
            Job::Odd { p, n } => {
                let c = OddCurve::new(p, n, &mut rng);
                rows.extend(run_curve(&c, false, n as usize, p as u64, ci, &mut rng));
            }
            Job::Bin { m, n } => {
                let c = BinCurve::new(m, n, &mut rng);
                rows.extend(run_curve(&c, true, n as usize, 1u64 << m, ci, &mut rng));
            }
        }
    }
    rows
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = Path::new(
        args.get(1)
            .expect("usage: decomposition_coverage <out-dir>"),
    );
    std::fs::create_dir_all(out).unwrap();
    let threads: usize = std::env::var("DC_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    let scale: f64 = std::env::var("DC_SCALE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    let mut log = File::create(out.join("tool_log.txt")).unwrap();
    let t0 = Instant::now();
    let mut rng = 0xdec0_7e5_c0ffee_u64;
    let js: Vec<(usize, Job, u64)> = jobs(scale)
        .into_iter()
        .enumerate()
        .map(|(i, j)| (i, j, xorshift(&mut rng)))
        .collect();
    let _ = writeln!(log, "jobs: {}", js.len());
    let queue = std::sync::Mutex::new(js);
    let results = std::sync::Mutex::new(Vec::<(usize, Vec<Row>)>::new());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let j = queue.lock().unwrap().pop();
                let Some((i, job, seed)) = j else { break };
                let r = run_job(&job, seed);
                results.lock().unwrap().push((i, r));
            });
        }
    });
    let mut res = results.into_inner().unwrap();
    res.sort_by_key(|(i, _)| *i);
    let rows: Vec<Row> = res.into_iter().flat_map(|(_, r)| r).collect();
    let mut f = File::create(out.join("coverage.csv")).unwrap();
    writeln!(
        f,
        "char,n,q,curve,order,arm,x_values,F,lambda,poisson,inv_n_factorial,distinct,multisets,pi"
    )
    .unwrap();
    for r in &rows {
        let nf: f64 = (1..=r.n).map(|i| i as f64).product();
        writeln!(
            f,
            "{},{},{},{},{},{},{},{},{:.6},{:.6},{:.6},{},{},{:.6}",
            if r.char2 { 2 } else { 0 },
            r.n,
            r.q,
            r.curve,
            r.order,
            r.arm,
            r.xs,
            r.fsize,
            r.lambda,
            r.poisson,
            1.0 / nf,
            r.distinct,
            r.multisets,
            r.pi
        )
        .unwrap();
    }
    // Summary per (char, n): residuals pi - poisson paired subfield vs null.
    let mut cells: BTreeMap<(bool, usize), Vec<[f64; 6]>> = BTreeMap::new();
    let mut by_curve: BTreeMap<(bool, usize, u64, usize), [Option<(f64, f64, f64)>; 3]> =
        BTreeMap::new();
    for r in &rows {
        let e = by_curve
            .entry((r.char2, r.n, r.q, r.curve))
            .or_insert([None, None, None]);
        let k = match r.arm {
            "subfield" => 0,
            "null" => 1,
            _ => 2,
        };
        e[k] = Some((r.pi, r.pi - r.poisson, r.lambda));
    }
    for ((c2, n, _q, _ci), v) in by_curve {
        if let [Some(a), Some(b), Some(c)] = v {
            cells
                .entry((c2, n))
                .or_default()
                .push([a.0, b.0, c.0, a.1, b.1, a.2]);
        }
    }
    let mut sf = File::create(out.join("coverage_summary.csv")).unwrap();
    writeln!(
        sf,
        "char,n,curves,mean_lambda_subfield,mean_pi_subfield,mean_pi_null,mean_pi_positive,inv_n_factorial,resid_subfield,resid_null,resid_diff,sd_resid_diff,z_resid_diff"
    )
    .unwrap();
    let mut checks = String::new();
    for ((c2, n), v) in &cells {
        let k = v.len() as f64;
        let mean = |i: usize| v.iter().map(|x| x[i]).sum::<f64>() / k;
        let md = v.iter().map(|x| x[3] - x[4]).sum::<f64>() / k;
        let sd =
            (v.iter().map(|x| (x[3] - x[4] - md).powi(2)).sum::<f64>() / (k - 1.0).max(1.0)).sqrt();
        let z = if sd > 0.0 { md / (sd / k.sqrt()) } else { 0.0 };
        let nf: f64 = (1..=*n).map(|i| i as f64).product();
        writeln!(
            sf,
            "{},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.2}",
            if *c2 { 2 } else { 0 },
            n,
            v.len(),
            mean(5),
            mean(0),
            mean(1),
            mean(2),
            1.0 / nf,
            mean(3),
            mean(4),
            md,
            sd,
            z
        )
        .unwrap();
        checks.push_str(&format!(
            "D char {} n={}: curves {} lambda {:.4} | pi subfield {:.4} null {:.4} positive {:.4} 1/n! {:.4} | resid subfield {:+.5} null {:+.5} diff {:+.5} (z {:.2})\n",
            if *c2 { 2 } else { 0 },
            n,
            v.len(),
            mean(5),
            mean(0),
            mean(1),
            mean(2),
            1.0 / nf,
            mean(3),
            mean(4),
            md,
            z
        ));
    }
    std::fs::write(out.join("checks.txt"), &checks).unwrap();
    let _ = write!(log, "{checks}total {:.1}s\n", t0.elapsed().as_secs_f64());
    eprint!("{checks}");
    eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order_check<G: Group>(g: &G, rng: &mut u64) {
        // #E * P = O for random points, and P + (-P) = O
        for _ in 0..5 {
            let p = random_point(g, rng);
            assert_eq!(g.add(p, g.neg(p)), Pt::O);
            let mut acc = Pt::O;
            let mut base = p;
            let mut e = g.order();
            while e > 0 {
                if e & 1 == 1 {
                    acc = g.add(acc, base);
                }
                base = g.add(base, base);
                e >>= 1;
            }
            assert_eq!(acc, Pt::O);
        }
    }

    #[test]
    fn odd_group_law() {
        let mut rng = 17u64;
        for (p, n) in [(101u32, 2u32), (13, 3), (5, 4)] {
            let c = OddCurve::new(p, n, &mut rng);
            order_check(&c, &mut rng);
        }
    }

    #[test]
    fn binary_group_law() {
        let mut rng = 23u64;
        for (m, n) in [(5u32, 2u32), (4, 3), (3, 4)] {
            let c = BinCurve::new(m, n, &mut rng);
            order_check(&c, &mut rng);
            // order counted equals the number of indexed points
            let mut cnt = 1u64;
            for x in 0..c.g.size {
                cnt += c.points_at(x).len() as u64;
            }
            assert_eq!(cnt, c.order());
        }
    }

    #[test]
    fn coverage_counts_multisets() {
        let mut rng = 29u64;
        let c = OddCurve::new(53, 2, &mut rng);
        let a = arms(&c, 2, &mut rng);
        let (_d, t) = coverage(&c, &a[0].2, 2);
        assert_eq!(t as f64, n_multisets(a[0].2.len(), 2));
    }
}
