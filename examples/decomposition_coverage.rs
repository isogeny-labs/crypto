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
//! Usage: `decomposition_coverage <out-dir> [mult|subgroup]`; env `DC_THREADS`, `DC_SCALE`.
//! `mult` runs the Amendment D diagnostic (binary `n = 2` coverage deficit).
#![allow(dead_code)]
#![allow(
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::unusual_byte_groupings,
    clippy::write_with_newline
)]

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

// ---------------------------------------------------------------------------
// Amendment D: follow-up diagnostic for binary n = 2 (`mult` mode).
// ---------------------------------------------------------------------------

/// Per-point multiplicity statistics of 2-fold multiset sums over `R != O`:
/// (index of dispersion var/mean, histogram of multiplicities 0..=8 with the
/// last bucket ">= 8").
fn multiplicity<G: Group>(g: &G, f: &[Pt]) -> (f64, [u64; 9]) {
    let mut cnt = vec![0u32; g.domain()];
    for i in 0..f.len() {
        for j in i..f.len() {
            cnt[g.idx(g.add(f[i], f[j]))] += 1;
        }
    }
    let mut hist = [0u64; 9];
    let (mut s1, mut s2, mut npts) = (0f64, 0f64, 0f64);
    for x in g.field_elems() {
        for p in g.points_at(x) {
            let c = cnt[g.idx(p)] as f64;
            s1 += c;
            s2 += c * c;
            npts += 1.0;
            hist[(c as usize).min(8)] += 1;
        }
    }
    let mean = s1 / npts;
    let var = s2 / npts - mean * mean;
    (var / mean, hist)
}

/// Gaussian elimination over F_2 on field elements (bit vectors) with the
/// combination of unknowns tracked. Returns (particular solution or None,
/// kernel basis) for sum_j c_j v_j = rhs.
fn f2_solve(cols: &[u32], rhs: u32) -> (Option<u64>, Vec<u64>) {
    let mut piv: Vec<(u32, u64)> = Vec::new();
    let mut ker = Vec::new();
    let reduce = |piv: &Vec<(u32, u64)>, mut v: u32, mut c: u64| {
        for &(pv, pc) in piv {
            let hb = 31 - pv.leading_zeros();
            if v >> hb & 1 == 1 {
                v ^= pv;
                c ^= pc;
            }
        }
        (v, c)
    };
    for (j, &v) in cols.iter().enumerate() {
        let (v, c) = reduce(&piv, v, 1u64 << j);
        if v == 0 {
            ker.push(c);
        } else {
            piv.push((v, c));
            // highest leading bit first, so reduction never reintroduces a pivot bit
            piv.sort_by_key(|&(pv, _)| pv.leading_zeros());
        }
    }
    let (r, c) = reduce(&piv, rhs, 0);
    (if r == 0 { Some(c) } else { None }, ker)
}

#[derive(Default, Debug)]
struct BinDiag {
    m1_pairs: u64,
    m1_viol: u64,
    kdim_hist: [u64; 8],
    solvable: u64,
    x3_total: u64,
    nonpow2: u64,
    valid_hist: [u64; 9],
    distinct_from_cosets: u64,
}

fn bin_diag(c: &BinCurve) -> BinDiag {
    let g = &c.g;
    let m = c.m;
    let mut d = BinDiag::default();
    let sub = g.subfield(m);
    let insub = {
        let mut v = vec![false; g.size as usize];
        for &s in &sub {
            v[s as usize] = true;
        }
        v
    };
    // M1
    for (i, &x1) in sub.iter().enumerate() {
        for &x2 in &sub[i + 1..] {
            let (p1, p2) = (c.points_at(x1), c.points_at(x2));
            if p1.is_empty() || p2.is_empty() {
                continue;
            }
            let (a, b) = (c.add(p1[0], p2[0]), c.add(p1[0], c.neg(p2[0])));
            if let (Pt::A(xa, _), Pt::A(xb, _)) = (a, b) {
                d.m1_pairs += 1;
                let s = x1 ^ x2;
                if xa ^ xb != g.mul(g.mul(x1, x2), g.inv(g.sq(s))) {
                    d.m1_viol += 1;
                }
            }
        }
    }
    // F_2 basis of F_q
    let mut basis: Vec<u32> = Vec::new();
    let mut ech: Vec<u32> = Vec::new();
    for &s in &sub {
        let mut v = s;
        for &e in &ech {
            v = v.min(v ^ e);
        }
        if v != 0 {
            ech.push(v);
            ech.sort_unstable_by(|a, b| b.cmp(a));
            basis.push(s);
        }
    }
    assert_eq!(basis.len() as u32, m);
    let has_pt = |x: u32| !c.points_at(x).is_empty();
    let mut distinct = 1u64; // O
    for x3 in 0..g.size {
        d.x3_total += 1;
        let x3s = g.sq(x3);
        let mut cols = Vec::with_capacity(2 * m as usize);
        for &bi in &basis {
            cols.push(g.mul(x3s, g.sq(bi)));
        }
        for &bi in &basis {
            cols.push(g.sq(bi) ^ g.mul(x3, bi));
        }
        let (part, ker) = f2_solve(&cols, c.b);
        d.kdim_hist[ker.len().min(7)] += 1;
        let Some(p0) = part else { continue };
        d.solvable += 1;
        let mut valid = 0u64;
        let mut cnt = 0u64;
        for mask in 0u64..(1u64 << ker.len()) {
            let mut comb = p0;
            for (t, &k) in ker.iter().enumerate() {
                if mask >> t & 1 == 1 {
                    comb ^= k;
                }
            }
            cnt += 1;
            let (mut e1, mut e2) = (0u32, 0u32);
            for (t, &bi) in basis.iter().enumerate() {
                if comb >> t & 1 == 1 {
                    e1 ^= bi;
                }
                if comb >> (t + m as usize) & 1 == 1 {
                    e2 ^= bi;
                }
            }
            if e1 == 0 {
                // double root x1 = sqrt(e2): multiset {P, P}
                let mut r = e2;
                for _ in 0..g.k - 1 {
                    r = g.sq(r);
                }
                if r != 0 && has_pt(r) {
                    valid += 1;
                }
                continue;
            }
            let w = g.mul(e2, g.inv(g.sq(e1)));
            let z = g.half[w as usize];
            if z == u32::MAX || !insub[z as usize] {
                continue; // roots not in F_q
            }
            let (x1, x2) = (g.mul(e1, z), g.mul(e1, z ^ 1));
            if has_pt(x1) && has_pt(x2) {
                valid += 1;
            }
        }
        if cnt & (cnt - 1) != 0 {
            d.nonpow2 += 1;
        }
        d.valid_hist[(valid as usize).min(8)] += 1;
        if valid > 0 {
            distinct += c.points_at(x3).len() as u64;
        }
    }
    d.distinct_from_cosets = distinct;
    d
}

// ---------------------------------------------------------------------------
// Addendum H7: coverage of the target subgroup and of the layers 2^k E.
// ---------------------------------------------------------------------------

impl OddCurve {
    fn with_ab(g: Gf, a: u32, b: u32) -> OddCurve {
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
        c
    }
    /// `E ≅ Y² = Π(X − d e_i)` with `e_i ∈ F_q` distinct and `d` generic,
    /// in short Weierstrass form, with the factor base `X ∈ d·F_q` as x-values.
    fn split_twist(p: u32, n: u32, rng: &mut u64, d_square: Option<bool>) -> (OddCurve, Vec<u32>) {
        let g = Gf::new(p, n);
        let sub = {
            let st = g.step(1);
            let mut v = vec![g.nn];
            for j in 0..(p - 1) {
                v.push(j * st);
            }
            v
        };
        loop {
            let d = (xorshift(rng) % g.nn as u64) as u32;
            if (1..n).any(|k| n % k == 0 && g.in_sub(d, k)) {
                continue;
            }
            // log-form element: even log = square
            if d_square.is_some_and(|sq| (d % 2 == 0) != sq) {
                continue;
            }
            let pick = |rng: &mut u64| sub[(xorshift(rng) % sub.len() as u64) as usize];
            let (e1, e2, e3) = (pick(rng), pick(rng), pick(rng));
            if e1 == e2 || e1 == e3 || e2 == e3 {
                continue;
            }
            let r: Vec<u32> = [e1, e2, e3].iter().map(|&e| g.mul(d, e)).collect();
            let s1 = g.add(g.add(r[0], r[1]), r[2]);
            let s2 = g.add(
                g.add(g.mul(r[0], r[1]), g.mul(r[0], r[2])),
                g.mul(r[1], r[2]),
            );
            let s3 = g.mul(g.mul(r[0], r[1]), r[2]);
            // X^3 + c2 X^2 + c1 X + c0 with c2 = -s1, c1 = s2, c0 = -s3; X = x - c2/3
            let (c2, c1, c0) = (g.neg(s1), s2, g.neg(s3));
            let i3 = g.inv(g.const_int(3));
            let a = g.sub(c1, g.mul(g.mul(c2, c2), i3));
            let b = g.add(
                g.sub(
                    g.mul(
                        g.mul(g.const_int(2), g.mul(c2, g.mul(c2, c2))),
                        g.inv(g.const_int(27)),
                    ),
                    g.mul(g.mul(c2, c1), i3),
                ),
                c0,
            );
            let shift = g.mul(c2, i3); // x = X + c2/3
            let xs: Vec<u32> = sub.iter().map(|&e| g.add(g.mul(d, e), shift)).collect();
            let c = OddCurve::with_ab(g, a, b);
            return (c, xs);
        }
    }
}

fn gmul<G: Group>(g: &G, p: Pt, mut k: u64) -> Pt {
    let (mut r, mut b) = (Pt::O, p);
    while k > 0 {
        if k & 1 == 1 {
            r = g.add(r, b);
        }
        b = g.add(b, b);
        k >>= 1;
    }
    r
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut best = 1;
    let mut d = 2u64;
    while d * d <= n {
        while n % d == 0 {
            best = d;
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        best = n;
    }
    best
}

fn sums_set<G: Group>(g: &G, f: &[Pt], n: usize) -> (Vec<u64>, u64) {
    let mut seen = vec![0u64; g.domain().div_ceil(64)];
    let (mut d, mut t) = (0u64, 0u64);
    sums_rec(g, f, 0, n, Pt::O, &mut seen, &mut d, &mut t);
    (seen, t)
}

/// Rows: (layer, size, covered by subfield, covered by null, multisets sub, multisets null)
fn subgroup_rows<G: Group>(
    g: &G,
    fsub: &[Pt],
    fnull: &[Pt],
    n: usize,
) -> Vec<(String, u64, u64, u64)> {
    let mut pts = vec![Pt::O];
    for x in g.field_elems() {
        pts.extend(g.points_at(x));
    }
    let ord = g.order();
    assert_eq!(pts.len() as u64, ord);
    let ell = largest_prime_factor(ord);
    let (ss, _) = sums_set(g, fsub, n);
    let (sn, _) = sums_set(g, fnull, n);
    let bit = |s: &[u64], i: usize| s[i / 64] >> (i % 64) & 1 == 1;
    let mut layers: Vec<(String, Vec<Pt>)> = Vec::new();
    layers.push(("E".into(), pts.clone()));
    let mut cur = pts.clone();
    let v2 = ord.trailing_zeros();
    for k in 1..=v2.min(4) {
        let mut set = std::collections::HashSet::new();
        for &p in &cur {
            set.insert(g.idx(g.add(p, p)));
        }
        cur = pts
            .iter()
            .copied()
            .filter(|p| set.contains(&g.idx(*p)))
            .collect();
        layers.push((format!("{}E", 1u64 << k), cur.clone()));
    }
    let lsub: Vec<Pt> = pts
        .iter()
        .copied()
        .filter(|&p| gmul(g, p, ell) == Pt::O)
        .collect();
    layers.push(("ell".into(), lsub));
    layers
        .into_iter()
        .map(|(name, set)| {
            let cs = set.iter().filter(|p| bit(&ss, g.idx(**p))).count() as u64;
            let cn = set.iter().filter(|p| bit(&sn, g.idx(**p))).count() as u64;
            (name, set.len() as u64, cs, cn)
        })
        .collect()
}

fn null_arm<G: Group>(g: &G, size: usize, rng: &mut u64) -> Vec<Pt> {
    let all = g.field_elems();
    let mut used = std::collections::HashSet::new();
    let mut xs = Vec::new();
    while xs.len() < size {
        let x = all[(xorshift(rng) % all.len() as u64) as usize];
        if used.insert(x) {
            xs.push(x);
        }
    }
    xs.iter().flat_map(|&x| g.points_at(x)).collect()
}

fn subgroup_main(out: &Path) {
    let mut rng = 0x4837_5ab9_u64;
    let mut csv = String::from(
        "kind,n,q,curve,order,ell,cofactor,layer,layer_size,F_sub,F_null,cov_sub,cov_null,ratio\n",
    );
    // per (kind, n): ratios on the ell-subgroup
    let mut acc: BTreeMap<(String, usize), Vec<f64>> = BTreeMap::new();
    let mut emit = |csv: &mut String,
                    kind: &str,
                    n: usize,
                    q: u64,
                    ci: usize,
                    ord: u64,
                    rows: Vec<(String, u64, u64, u64)>,
                    fs: usize,
                    fnl: usize| {
        let ell = largest_prime_factor(ord);
        for (layer, size, cs, cn) in rows {
            let (a, b) = (cs as f64 / size as f64, cn as f64 / size as f64);
            let ratio = if b > 0.0 { a / b } else { f64::NAN };
            csv.push_str(&format!(
                "{kind},{n},{q},{ci},{ord},{ell},{},{layer},{size},{fs},{fnl},{a:.6},{b:.6},{ratio:.5}\n",
                ord / ell
            ));
            if layer == "ell" && ratio.is_finite() {
                acc.entry((kind.to_string(), n)).or_default().push(ratio);
            }
        }
    };
    // binary: n = 2 (m = 4..10), n = 4 (m = 3..5)
    for (n, ms) in [(2u32, 4u32..=10), (4, 3..=5)] {
        for m in ms {
            for ci in 0..3 {
                let c = BinCurve::new(m, n, &mut rng);
                let fsub: Vec<Pt> = c
                    .subfield_elems()
                    .iter()
                    .flat_map(|&x| c.points_at(x))
                    .collect();
                let fnl = null_arm(&c, c.subfield_elems().len(), &mut rng);
                let rows = subgroup_rows(&c, &fsub, &fnl, n as usize);
                emit(
                    &mut csv,
                    "binary",
                    n as usize,
                    1u64 << m,
                    ci,
                    c.order(),
                    rows,
                    fsub.len(),
                    fnl.len(),
                );
            }
        }
    }
    // odd: generic control and split twist, n = 2 and 4
    for (n, ps) in [(2u32, vec![53u32, 101, 211, 401]), (4, vec![5u32, 7, 11])] {
        for &p in &ps {
            for ci in 0..3 {
                let c = OddCurve::new(p, n, &mut rng);
                let fsub: Vec<Pt> = c
                    .subfield_elems()
                    .iter()
                    .flat_map(|&x| c.points_at(x))
                    .collect();
                let fnl = null_arm(&c, c.subfield_elems().len(), &mut rng);
                let rows = subgroup_rows(&c, &fsub, &fnl, n as usize);
                emit(
                    &mut csv,
                    "odd_generic",
                    n as usize,
                    p as u64,
                    ci,
                    c.order(),
                    rows,
                    fsub.len(),
                    fnl.len(),
                );
                let (c2, xs) = OddCurve::split_twist(p, n, &mut rng, None);
                let fsub: Vec<Pt> = xs.iter().flat_map(|&x| c2.points_at(x)).collect();
                let fnl = null_arm(&c2, xs.len(), &mut rng);
                let rows = subgroup_rows(&c2, &fsub, &fnl, n as usize);
                emit(
                    &mut csv,
                    "odd_split_twist",
                    n as usize,
                    p as u64,
                    ci,
                    c2.order(),
                    rows,
                    fsub.len(),
                    fnl.len(),
                );
                // Amendment H-a: d a square, factor base the first |F_q|/2 of d·F_q
                // in a fixed random order (as many points as the null arm)
                let (c3, mut xs3) = OddCurve::split_twist(p, n, &mut rng, Some(true));
                for i in (1..xs3.len()).rev() {
                    let j = (xorshift(&mut rng) % (i as u64 + 1)) as usize;
                    xs3.swap(i, j);
                }
                xs3.truncate(xs3.len() / 2);
                let fsub: Vec<Pt> = xs3.iter().flat_map(|&x| c3.points_at(x)).collect();
                let fnl = null_arm(&c3, 2 * xs3.len(), &mut rng);
                let rows = subgroup_rows(&c3, &fsub, &fnl, n as usize);
                emit(
                    &mut csv,
                    "odd_split_twist_matched",
                    n as usize,
                    p as u64,
                    ci,
                    c3.order(),
                    rows,
                    fsub.len(),
                    fnl.len(),
                );
            }
        }
        eprintln!("subgroup: odd n = {n} done");
    }
    let mut checks = String::new();
    for ((kind, n), v) in &acc {
        let k = v.len() as f64;
        let m = v.iter().sum::<f64>() / k;
        let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (k - 1.0).max(1.0)).sqrt();
        checks.push_str(&format!(
            "H7 {kind} n={n}: ell-subgroup coverage ratio subfield/null mean {m:.3} (s.e. {:.3}, {} curves)\n",
            sd / k.sqrt(),
            v.len()
        ));
    }
    std::fs::write(out.join("subgroup.csv"), csv).unwrap();
    std::fs::write(out.join("checks.txt"), &checks).unwrap();
    eprint!("{checks}");
}

fn mult_main(out: &Path) {
    let mut rng = 0xd1a9_0000_0d0d_u64;
    let mut csv = String::from(
        "char,q,curve,order,arm,F,lambda,poisson,pi,dispersion,h0,h1,h2,h3,h4,h5,h6,h7,h8plus\n",
    );
    let mut bcsv = String::from("m,curve,order,m1_pairs,m1_violations,x3_total,solvable,nonpow2_cosets,kdim0,kdim1,kdim2,kdim3,kdim4plus,valid0,valid1,valid2,valid3,valid4,valid5,valid6,valid7,valid8plus,distinct_subfield,distinct_from_cosets\n");
    let mut checks = String::new();
    let mut disp: BTreeMap<(u8, &'static str), Vec<f64>> = BTreeMap::new();
    let mut emit = |csv: &mut String,
                    ch: u8,
                    q: u64,
                    ci: usize,
                    ord: u64,
                    arm: &'static str,
                    f: usize,
                    distinct: u64,
                    total: u64,
                    dsp: f64,
                    h: [u64; 9]| {
        let lam = total as f64 / ord as f64;
        csv.push_str(&format!(
            "{ch},{q},{ci},{ord},{arm},{f},{lam:.6},{:.6},{:.6},{dsp:.5},{}\n",
            1.0 - (-lam).exp(),
            distinct as f64 / ord as f64,
            h.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
        disp.entry((ch, arm)).or_default().push(dsp);
    };
    let (mut m1v, mut npw, mut xchk) = (0u64, 0u64, 0u64);
    for mm in 4u32..=10 {
        for ci in 0..3 {
            let c = BinCurve::new(mm, 2, &mut rng);
            let arms = arms(&c, 2, &mut rng);
            let mut dsub = 0;
            for (arm, _xs, f) in &arms[..2] {
                let (dd, tt) = coverage(&c, f, 2);
                let (dsp, h) = multiplicity(&c, f);
                if *arm == "subfield" {
                    dsub = dd;
                }
                emit(
                    &mut csv,
                    2,
                    1 << mm,
                    ci,
                    c.order,
                    arm,
                    f.len(),
                    dd,
                    tt,
                    dsp,
                    h,
                );
            }
            let d = bin_diag(&c);
            m1v += d.m1_viol;
            npw += d.nonpow2;
            xchk += u64::from(d.distinct_from_cosets != dsub);
            let k = &d.kdim_hist;
            bcsv.push_str(&format!(
                "{mm},{ci},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                c.order,
                d.m1_pairs,
                d.m1_viol,
                d.x3_total,
                d.solvable,
                d.nonpow2,
                k[0],
                k[1],
                k[2],
                k[3],
                k[4..].iter().sum::<u64>(),
                d.valid_hist
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                dsub,
                d.distinct_from_cosets
            ));
            eprintln!(
                "mult binary m={mm} curve {ci}: kdim {:?} solvable {}/{} valid {:?}",
                d.kdim_hist, d.solvable, d.x3_total, d.valid_hist
            );
        }
    }
    for p in [53u32, 101, 211, 401, 809] {
        for ci in 0..3 {
            let c = OddCurve::new(p, 2, &mut rng);
            let arms = arms(&c, 2, &mut rng);
            for (arm, _xs, f) in &arms[..2] {
                let (dd, tt) = coverage(&c, f, 2);
                let (dsp, h) = multiplicity(&c, f);
                emit(
                    &mut csv,
                    0,
                    p as u64,
                    ci,
                    c.order,
                    arm,
                    f.len(),
                    dd,
                    tt,
                    dsp,
                    h,
                );
            }
        }
    }
    checks.push_str(&format!(
        "M1 identity x(P+Q)+x(P-Q) = x1x2/(x1+x2)^2 violations: {m1v}\n"
    ));
    checks.push_str(&format!("M2 solution cosets not a power of 2: {npw}\n"));
    checks.push_str(&format!(
        "M3 cross-check distinct(subfield) != distinct from cosets: {xchk} curves\n"
    ));
    for ((ch, arm), v) in &disp {
        let k = v.len() as f64;
        let mean = v.iter().sum::<f64>() / k;
        let sd = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (k - 1.0)).sqrt();
        checks.push_str(&format!(
            "M3 dispersion char {ch} {arm}: mean {mean:.4} sd {sd:.4} over {} curves\n",
            v.len()
        ));
    }
    std::fs::write(out.join("mult.csv"), csv).unwrap();
    std::fs::write(out.join("binary_cosets.csv"), bcsv).unwrap();
    std::fs::write(out.join("checks.txt"), &checks).unwrap();
    eprint!("{checks}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = Path::new(
        args.get(1)
            .expect("usage: decomposition_coverage <out-dir>"),
    );
    std::fs::create_dir_all(out).unwrap();
    if args.get(2).map(String::as_str) == Some("subgroup") {
        subgroup_main(out);
        return;
    }
    if args.get(2).map(String::as_str) == Some("mult") {
        mult_main(out);
        return;
    }
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

    #[test]
    fn binary_cosets_reproduce_coverage() {
        let mut rng = 31u64;
        for m in [4u32, 5] {
            let c = BinCurve::new(m, 2, &mut rng);
            let a = arms(&c, 2, &mut rng);
            let (dsub, _) = coverage(&c, &a[0].2, 2);
            let d = bin_diag(&c);
            assert_eq!(d.m1_viol, 0);
            assert_eq!(d.nonpow2, 0);
            assert_eq!(d.distinct_from_cosets, dsub);
        }
    }

    #[test]
    fn f2_solve_small() {
        // columns 0b01, 0b10, 0b11: kernel {c0+c1+c2}, rhs 0b11 solvable
        let (p, k) = f2_solve(&[1, 2, 3], 3);
        assert_eq!(k.len(), 1);
        let pc = p.unwrap();
        let img = (0..3)
            .filter(|j| pc >> j & 1 == 1)
            .fold(0u32, |a, j| a ^ [1u32, 2, 3][j]);
        assert_eq!(img, 3);
        assert!(f2_solve(&[1, 1], 2).0.is_none());
    }
}
