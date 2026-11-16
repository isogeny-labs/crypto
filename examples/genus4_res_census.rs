//! Genus-4 Jacobians in the isogeny class of `Res_{F_{q^4}/F_q} E`, and
//! Theorem 5's finite check.
//!
//! Round 3 of the Teske-variant search, item 4
//! (`research/weil_descent_angles_20261018/PROTOCOL.md`):
//!
//! - Part T: for `n = 2..16`, every Weierstrass cycle type `(c_i)` of a
//!   hyperelliptic genus-`n` curve, against the 2-torsion characteristic
//!   polynomial `(T+1)^2 χ̄(T^n)` of `Res_{F_{q^n}/F_q} E` (q odd).
//! - Part H4: every hyperelliptic genus-4 curve over `F_3` (control).
//! - Part K: every non-hyperelliptic genus-4 curve over `F_3` (canonical
//!   models on the split quadric, the nonsplit quadric and the cone), exact
//!   smoothness, point counts over `F_{3^k}`, `k ≤ 4`, the Res-shaped ones
//!   (`#C(F_{3^k}) = 3^k + 1`, `k = 1, 2, 3`) and their isomorphism classes.
//!
//! std-only; `rustc --edition 2021 -O examples/genus4_res_census.rs`.
//! Usage: `genus4_res_census <out-dir> [T|H4|K ...]`; env `G4_THREADS`.
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

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Helpers reused from examples/weil_restriction_isogeny_census.rs (same
// semantics; table field F_{p^l} in log form, `nn` = zero, 0 = one).
// ---------------------------------------------------------------------------

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

fn poly_derivative(f: &[u32], p: u32) -> Vec<u32> {
    let mut d: Vec<u32> = f
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &c)| (c * (i as u32 % p)) % p)
        .collect();
    poly_trim(&mut d);
    d
}

fn poly_div_exact(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = a.to_vec();
    let db = b.len() - 1;
    let mut qout = vec![0u32; r.len() - db];
    let inv = inv_mod_p(b[db], p);
    for k in (db..r.len()).rev() {
        let c = r[k] * inv % p;
        qout[k - db] = c;
        if c != 0 {
            for i in 0..=db {
                r[k - db + i] = (r[k - db + i] + p - c * b[i] % p) % p;
            }
        }
    }
    poly_trim(&mut qout);
    qout
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

fn gfp_trim(g: &Gf, a: &mut Vec<u32>) {
    while a.last().is_some_and(|&c| g.is_zero(c)) {
        a.pop();
    }
}

fn gfp_rem(g: &Gf, a: &[u32], m: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    gfp_trim(g, &mut r);
    let dm = m.len() - 1;
    let inv = g.inv(m[dm]);
    while r.len() > dm {
        let k = r.len() - 1;
        let c = g.mul(r[k], inv);
        if !g.is_zero(c) {
            for i in 0..=dm {
                r[k - dm + i] = g.sub(r[k - dm + i], g.mul(c, m[i]));
            }
        }
        r.pop();
        gfp_trim(g, &mut r);
    }
    r
}

fn gfp_mulmod(g: &Gf, a: &[u32], b: &[u32], m: &[u32]) -> Vec<u32> {
    let mut prod = vec![g.zero(); a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        if g.is_zero(x) {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = g.add(prod[i + j], g.mul(x, y));
        }
    }
    gfp_rem(g, &prod, m)
}

fn gfp_powmod(g: &Gf, a: &[u32], mut e: u64, m: &[u32]) -> Vec<u32> {
    let mut r = vec![0u32]; // one
    let mut b = gfp_rem(g, a, m);
    while e > 0 {
        if e & 1 == 1 {
            r = gfp_mulmod(g, &r, &b, m);
        }
        b = gfp_mulmod(g, &b, &b, m);
        e >>= 1;
    }
    r
}

fn gfp_gcd(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    gfp_trim(g, &mut a);
    gfp_trim(g, &mut b);
    while !b.is_empty() {
        let r = gfp_rem(g, &a, &b);
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let inv = g.inv(lc);
        for c in a.iter_mut() {
            *c = g.mul(*c, inv);
        }
    }
    a
}

fn gfp_div_exact(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    gfp_trim(g, &mut r);
    let db = b.len() - 1;
    let mut q = vec![g.zero(); r.len() - db];
    let inv = g.inv(b[db]);
    for k in (db..r.len()).rev() {
        let c = g.mul(r[k], inv);
        q[k - db] = c;
        if !g.is_zero(c) {
            for i in 0..=db {
                r[k - db + i] = g.sub(r[k - db + i], g.mul(c, b[i]));
            }
        }
    }
    gfp_trim(g, &mut q);
    q
}

fn gfp_add(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut out = vec![g.zero(); n];
    for (i, o) in out.iter_mut().enumerate() {
        let x = a.get(i).copied().unwrap_or(g.zero());
        let y = b.get(i).copied().unwrap_or(g.zero());
        *o = g.add(x, y);
    }
    gfp_trim(g, &mut out);
    out
}

fn gfp_sub_poly(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let nb: Vec<u32> = b.iter().map(|&c| g.neg(c)).collect();
    gfp_add(g, a, &nb)
}

fn gfp_mul(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut prod = vec![g.zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if g.is_zero(x) {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = g.add(prod[i + j], g.mul(x, y));
        }
    }
    gfp_trim(g, &mut prod);
    prod
}

fn gfp_scale(g: &Gf, a: &[u32], c: u32) -> Vec<u32> {
    let mut out: Vec<u32> = a.iter().map(|&x| g.mul(x, c)).collect();
    gfp_trim(g, &mut out);
    out
}

fn gfp_monic(g: &Gf, a: &[u32]) -> Vec<u32> {
    let mut f = a.to_vec();
    gfp_trim(g, &mut f);
    if let Some(&lc) = f.last() {
        let inv = g.inv(lc);
        for c in f.iter_mut() {
            *c = g.mul(*c, inv);
        }
    }
    f
}

fn gfp_derivative(g: &Gf, f: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(f.len().saturating_sub(1));
    for (i, &c) in f.iter().enumerate().skip(1) {
        out.push(g.mul(c, g.const_int(i as i64)));
    }
    gfp_trim(g, &mut out);
    out
}

// ---------------------------------------------------------------------------
// Part T: cycle types against the 2-torsion polynomial of Res E.
// ---------------------------------------------------------------------------

/// Carry-less product in F_2[T] (bit i = coefficient of T^i).
fn f2_mul(a: u64, b: u64) -> u64 {
    let (mut r, mut a, mut b) = (0u64, a, b);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    r
}

fn partitions(rest: usize, max: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if rest == 0 {
        out.push(cur.clone());
        return;
    }
    for c in (1..=max.min(rest)).rev() {
        cur.push(c);
        partitions(rest - c, c, cur, out);
        cur.pop();
    }
}

#[derive(Clone, Debug)]
struct TRow {
    n: usize,
    t_odd: bool,
    solutions: Vec<Vec<usize>>,
}

/// For `n` and the parity of `t`, every cycle type `(c_i)` of `2n + 2` points
/// with `∏ (T^{c_i} + 1) = (T + 1)^2 χ̄(T^n)` over `F_2`.
fn part_t_rows(nmax: usize) -> Vec<TRow> {
    let mut rows = Vec::new();
    for n in 2..=nmax {
        let total = 2 * n + 2;
        let mut parts = Vec::new();
        partitions(total, total, &mut Vec::new(), &mut parts);
        for t_odd in [true, false] {
            let chi = if t_odd {
                (1u64 << (2 * n)) | (1u64 << n) | 1
            } else {
                (1u64 << (2 * n)) | 1
            };
            let target = f2_mul(0b101, chi);
            let solutions: Vec<Vec<usize>> = parts
                .iter()
                .filter(|p| p.iter().fold(1u64, |acc, &c| f2_mul(acc, (1u64 << c) | 1)) == target)
                .cloned()
                .collect();
            rows.push(TRow {
                n,
                t_odd,
                solutions,
            });
        }
    }
    rows
}

fn part_t(out: &Path, log: &mut File) -> String {
    let rows = part_t_rows(16);
    let mut f = File::create(out.join("part_t.csv")).unwrap();
    writeln!(f, "n,t_parity,solutions,cycle_types").unwrap();
    let mut odd_bad = Vec::new();
    for r in &rows {
        let types: Vec<String> = r
            .solutions
            .iter()
            .map(|p| {
                p.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("+")
            })
            .collect();
        writeln!(
            f,
            "{},{},{},{}",
            r.n,
            if r.t_odd { "odd" } else { "even" },
            r.solutions.len(),
            types.join(" ")
        )
        .unwrap();
        if r.t_odd && r.n >= 3 && !r.solutions.is_empty() {
            odd_bad.push(r.n);
        }
    }
    let n2_odd: Vec<String> = rows
        .iter()
        .filter(|r| r.n == 2 && r.t_odd)
        .flat_map(|r| r.solutions.iter().map(|p| format!("{p:?}")))
        .collect();
    let n4_even_pow2 = rows.iter().filter(|r| r.n == 4 && !r.t_odd).all(|r| {
        r.solutions
            .iter()
            .all(|p| p.iter().all(|c| c.is_power_of_two()))
    });
    let s = format!(
        "partT: n with an odd-t solution and n >= 3: {:?} (P5 predicts none)\npartT: n = 2 odd-t cycle types: {}\npartT: n = 4 even-t solutions all in powers of 2: {}\n",
        odd_bad,
        n2_odd.join(" "),
        n4_even_pow2
    );
    let _ = write!(log, "{s}");
    s
}

// ---------------------------------------------------------------------------
// Shared: the big table field F_{3^12} and its subfields.
// ---------------------------------------------------------------------------

struct Ctx {
    b: Gf,
    /// elements of F_{3^k} (zero included), k | 12
    sub: BTreeMap<u32, Vec<u32>>,
    /// P^1(F_{3^k}): the elements of F_{3^k} and INF
    p1: BTreeMap<u32, Vec<u32>>,
    /// F_9 by index: 0 -> zero, k >= 1 -> log (k-1)·step(2)
    f9: Vec<u32>,
    minus_one: u32,
}

impl Ctx {
    fn new() -> Ctx {
        let b = Gf::new(3, 12);
        let mut sub = BTreeMap::new();
        let mut p1 = BTreeMap::new();
        for k in [1u32, 2, 3, 4, 6, 12] {
            let st = b.step(k);
            let mut v = vec![b.zero()];
            let cnt = 3u32.pow(k) - 1;
            for e in 0..cnt {
                v.push(e * st);
            }
            let mut w = v.clone();
            w.push(INF);
            sub.insert(k, v);
            p1.insert(k, w);
        }
        let st2 = b.step(2);
        let mut f9 = vec![b.zero()];
        for e in 0..8 {
            f9.push(e * st2);
        }
        let minus_one = b.neg(0);
        Ctx {
            b,
            sub,
            p1,
            f9,
            minus_one,
        }
    }
    fn f3(&self, d: u32) -> u32 {
        self.b.const_int(d as i64)
    }
    fn f3_digit(&self, a: u32) -> u32 {
        if a == self.b.zero() {
            0
        } else if a == 0 {
            1
        } else {
            assert_eq!(a, self.minus_one, "not an F_3 element");
            2
        }
    }
    fn f9_index(&self, a: u32) -> u32 {
        if a == self.b.zero() {
            0
        } else {
            let st2 = self.b.step(2);
            assert!(a.is_multiple_of(st2), "not an F_9 element");
            a / st2 + 1
        }
    }
    fn pow(&self, a: u32, e: u32) -> u32 {
        let mut r = 0u32;
        for _ in 0..e {
            r = self.b.mul(r, a);
        }
        r
    }
}

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

/// Distinct-degree pattern of a polynomial over a prime field (small ints);
/// `f` squarefree.
fn ddf_pattern_p(f: &[u32], p: u32) -> Vec<usize> {
    let mut f = f.to_vec();
    poly_trim(&mut f);
    let lc_inv = inv_mod_p(*f.last().unwrap(), p);
    for c in f.iter_mut() {
        *c = *c * lc_inv % p;
    }
    let x = vec![0u32, 1];
    let mut xp = x.clone();
    let mut pattern = Vec::new();
    let mut d = 1usize;
    while f.len() > 2 * d {
        xp = poly_powmod(&xp, p as u64, &f, p);
        let h = poly_sub(&xp, &x, p);
        let g = if h.is_empty() {
            f.clone()
        } else {
            poly_gcd(&f, &h, p)
        };
        if g.len() > 1 {
            for _ in 0..(g.len() - 1) / d {
                pattern.push(d);
            }
            f = poly_div_exact(&f, &g, p);
            xp = poly_rem(&xp, &f, p);
        }
        d += 1;
    }
    if f.len() > 1 {
        pattern.push(f.len() - 1);
    }
    pattern.sort_unstable();
    pattern
}

// ---------------------------------------------------------------------------
// Part H4: hyperelliptic genus 4 over F_3 (control for Theorem 5).
// ---------------------------------------------------------------------------

fn part_h4(ctx: &Ctx, out: &Path, log: &mut File) -> String {
    let b = &ctx.b;
    let mut sqf = 0u64;
    let mut res_rows = Vec::new();
    let steps: Vec<(u32, u32)> = (1..=4u32).map(|k| (k, b.step(k))).collect();
    for code in 1u32..3u32.pow(11) {
        let mut a = [0u32; 11];
        let mut v = code;
        for ai in a.iter_mut() {
            *ai = v % 3;
            v /= 3;
        }
        let (fpoly, inf_root): (Vec<u32>, bool) = if a[10] != 0 {
            (a.to_vec(), false)
        } else if a[9] != 0 {
            (a[..10].to_vec(), true)
        } else {
            continue;
        };
        let d = poly_derivative(&fpoly, 3);
        if d.is_empty() || poly_gcd(&fpoly, &d, 3).len() != 1 {
            continue;
        }
        sqf += 1;
        let coeffs: Vec<u32> = a.iter().map(|&c| ctx.f3(c)).collect();
        let count = |k: u32, st: u32| -> i64 {
            let mut n = 1 + b.chi_sub(coeffs[10], st) as i64; // the point(s) over (1:0)
            for &x in &ctx.sub[&k] {
                let mut val = b.zero();
                for &c in coeffs.iter().rev() {
                    val = b.add(b.mul(val, x), c);
                }
                n += 1 + b.chi_sub(val, st) as i64;
            }
            n
        };
        let n1 = count(1, steps[0].1);
        if n1 != 4 {
            continue;
        }
        let n2 = count(2, steps[1].1);
        if n2 != 10 {
            continue;
        }
        let n3 = count(3, steps[2].1);
        if n3 != 28 {
            continue;
        }
        let n4 = count(4, steps[3].1);
        let mut cyc = ddf_pattern_p(&fpoly, 3);
        if inf_root {
            cyc.push(1);
        }
        cyc.sort_unstable();
        res_rows.push((code, n4, cyc));
    }
    let mass_num = sqf; // mass = sqf / |GL_2(F_3)| = sqf / 48
    let mut f = File::create(out.join("h4_res_shaped.csv")).unwrap();
    writeln!(f, "code,a0..a10,N4,t,t_even,cycle_type,cycles_powers_of_2").unwrap();
    let mut viol = 0usize;
    let mut t_hist: BTreeMap<i64, usize> = BTreeMap::new();
    for (code, n4, cyc) in &res_rows {
        let tnum = 82 - n4;
        let t = if tnum % 4 == 0 { tnum / 4 } else { i64::MIN };
        let pow2 = cyc.iter().all(|c| c.is_power_of_two());
        let even = t != i64::MIN && t % 2 == 0;
        if !(even && pow2) {
            viol += 1;
        }
        *t_hist.entry(t).or_insert(0) += 1;
        let mut digits = Vec::new();
        let mut v = *code;
        for _ in 0..11 {
            digits.push((v % 3).to_string());
            v /= 3;
        }
        writeln!(
            f,
            "{code},{},{n4},{t},{even},{},{pow2}",
            digits.join(" "),
            cyc.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("+")
        )
        .unwrap();
    }
    let s = format!(
        "H4: squarefree degree-10 binary forms = {sqf}; mass = {sqf}/48 = {} (check K1 expects 2187: {})\nH4: Res-shaped models = {}; t histogram {:?}\nH4: check K2 (t even and cycle types in powers of 2) violations = {viol}\n",
        mass_num as f64 / 48.0,
        if mass_num == 2187 * 48 { "ok" } else { "MISMATCH" },
        res_rows.len(),
        t_hist
    );
    let _ = write!(log, "{s}");
    s
}

// ---------------------------------------------------------------------------
// Part K: exact smoothness (resultants over the coefficient field K).
// ---------------------------------------------------------------------------

fn peval(b: &Gf, f: &[u32], z: u32) -> u32 {
    let mut v = b.zero();
    for &c in f.iter().rev() {
        v = b.add(b.mul(v, z), c);
    }
    v
}

fn pdeg(f: &[u32]) -> usize {
    f.len().saturating_sub(1)
}

/// Determinant of a square matrix over the table field (Gaussian elimination).
fn det(b: &Gf, mut m: Vec<Vec<u32>>) -> u32 {
    let n = m.len();
    let mut d = 0u32; // one
    for c in 0..n {
        let Some(piv) = (c..n).find(|&r| !b.is_zero(m[r][c])) else {
            return b.zero();
        };
        if piv != c {
            m.swap(piv, c);
            d = b.neg(d);
        }
        d = b.mul(d, m[c][c]);
        let inv = b.inv(m[c][c]);
        for r in c + 1..n {
            if b.is_zero(m[r][c]) {
                continue;
            }
            let f = b.mul(m[r][c], inv);
            for k in c..n {
                let s = b.mul(f, m[c][k]);
                m[r][k] = b.sub(m[r][k], s);
            }
        }
    }
    d
}

/// Formal Sylvester resultant in `y` of `f = Σ f[j] y^j` (formal degree
/// `f.len()-1`) and `g`, each `f[j]` a polynomial in `x`; computed by
/// evaluation at `bound + 1` points and Newton interpolation. Returns the
/// trimmed coefficient vector (empty = identically zero).
fn resultant_y(b: &Gf, f: &[Vec<u32>], g: &[Vec<u32>]) -> Vec<u32> {
    let df = f.len() - 1;
    let dg = g.len() - 1;
    let mf = f.iter().map(|c| pdeg(c)).max().unwrap_or(0);
    let mg = g.iter().map(|c| pdeg(c)).max().unwrap_or(0);
    let bound = dg * mf + df * mg;
    let npts = bound + 1;
    let n = df + dg;
    let zs: Vec<u32> = (0..npts as u32).collect(); // g^0, g^1, ... distinct nonzero
    let mut vals = Vec::with_capacity(npts);
    for &z in &zs {
        let fv: Vec<u32> = f.iter().map(|c| peval(b, c, z)).collect();
        let gv: Vec<u32> = g.iter().map(|c| peval(b, c, z)).collect();
        let mut m = vec![vec![b.zero(); n]; n];
        for r in 0..dg {
            for (j, &c) in fv.iter().enumerate() {
                // descending powers: column r + (df - j)
                m[r][r + df - j] = c;
            }
        }
        for r in 0..df {
            for (j, &c) in gv.iter().enumerate() {
                m[dg + r][r + dg - j] = c;
            }
        }
        vals.push(det(b, m));
    }
    // Newton divided differences.
    let mut coef = vals.clone();
    for j in 1..npts {
        for i in (j..npts).rev() {
            let num = b.sub(coef[i], coef[i - 1]);
            let den = b.sub(zs[i], zs[i - j]);
            coef[i] = b.mul(num, b.inv(den));
        }
    }
    // Expand Π (x - z_l) basis to monomials.
    let mut poly = vec![b.zero(); npts];
    let mut basis = vec![0u32]; // the polynomial 1
    for (i, &c) in coef.iter().enumerate() {
        for (k, &bc) in basis.iter().enumerate() {
            poly[k] = b.add(poly[k], b.mul(c, bc));
        }
        if i + 1 < npts {
            // basis *= (x - z_i)
            let mut nb = vec![b.zero(); basis.len() + 1];
            for (k, &bc) in basis.iter().enumerate() {
                nb[k + 1] = b.add(nb[k + 1], bc);
                nb[k] = b.sub(nb[k], b.mul(bc, zs[i]));
            }
            basis = nb;
        }
    }
    gfp_trim(b, &mut poly);
    poly
}

fn rand_k_elem(ctx: &Ctx, kdeg: u32, rng: &mut u64) -> u32 {
    let v = &ctx.sub[&kdeg];
    v[(xorshift(rng) % v.len() as u64) as usize]
}

fn edf(ctx: &Ctx, h: &[u32], d: usize, kdeg: u32, rng: &mut u64, out: &mut Vec<Vec<u32>>) {
    let b = &ctx.b;
    let dh = pdeg(h);
    if dh == d {
        out.push(h.to_vec());
        return;
    }
    let q = 3u64.pow(kdeg);
    let e = (q.pow(d as u32) - 1) / 2;
    loop {
        let mut a: Vec<u32> = (0..dh).map(|_| rand_k_elem(ctx, kdeg, rng)).collect();
        gfp_trim(b, &mut a);
        if a.len() < 2 {
            continue;
        }
        let w = gfp_powmod(b, &a, e, h);
        let w1 = gfp_sub_poly(b, &w, &[0u32]);
        if w1.is_empty() {
            continue;
        }
        let g = gfp_gcd(b, h, &w1);
        if g.len() > 1 && g.len() < h.len() {
            let other = gfp_div_exact(b, h, &g);
            edf(ctx, &g, d, kdeg, rng, out);
            edf(ctx, &other, d, kdeg, rng, out);
            return;
        }
    }
}

/// Distinct monic irreducible factors over `K = F_{3^kdeg}` of `r` (whose
/// coefficients lie in K).
fn irreducible_factors(ctx: &Ctx, r: &[u32], kdeg: u32, rng: &mut u64) -> Vec<Vec<u32>> {
    let b = &ctx.b;
    let rm = gfp_monic(b, r);
    let dmax = pdeg(&rm);
    let q = 3u64.pow(kdeg);
    let x = vec![b.zero(), 0u32];
    let mut xp = x.clone();
    let mut found: Vec<Vec<u32>> = Vec::new();
    let mut covered = 0usize;
    for d in 1..=dmax {
        if covered + d > dmax {
            break;
        }
        xp = gfp_powmod(b, &xp, q, &rm);
        let diff = gfp_sub_poly(b, &xp, &x);
        let mut h = if diff.is_empty() {
            rm.clone()
        } else {
            gfp_gcd(b, &rm, &diff)
        };
        for phi in &found {
            if d % pdeg(phi) == 0 && h.len() > phi.len() - 1 && gfp_rem(b, &h, phi).is_empty() {
                h = gfp_div_exact(b, &h, phi);
            }
        }
        if h.len() > 1 {
            let before = found.len();
            edf(ctx, &h, d, kdeg, rng, &mut found);
            covered += (found.len() - before) * d;
        }
    }
    found
}

/// Polynomials over L = K[x]/(phi): coefficient j is an element of L (a
/// polynomial of degree < deg phi, trimmed; empty = zero).
fn lpoly_trim(a: &mut Vec<Vec<u32>>) {
    while a.last().is_some_and(|c| c.is_empty()) {
        a.pop();
    }
}

fn lpoly_rem(
    ctx: &Ctx,
    a: &[Vec<u32>],
    bpoly: &[Vec<u32>],
    phi: &[u32],
    linv_exp: u64,
) -> Vec<Vec<u32>> {
    let b = &ctx.b;
    let mut r = a.to_vec();
    lpoly_trim(&mut r);
    let db = bpoly.len() - 1;
    let inv = gfp_powmod(b, &bpoly[db], linv_exp, phi);
    while r.len() > db {
        let k = r.len() - 1;
        let c = gfp_mulmod(b, &r[k], &inv, phi);
        for i in 0..=db {
            let s = gfp_mulmod(b, &c, &bpoly[i], phi);
            r[k - db + i] = gfp_sub_poly(b, &r[k - db + i], &s);
        }
        r.pop();
        lpoly_trim(&mut r);
    }
    r
}

/// Degree of gcd in L[y] of the given polynomials; `None` if all are zero.
fn lgcd_deg(ctx: &Ctx, polys: &[Vec<Vec<u32>>], phi: &[u32], linv_exp: u64) -> Option<usize> {
    let mut g: Vec<Vec<u32>> = Vec::new();
    for p in polys {
        let mut a = g.clone();
        let mut bp = p.clone();
        lpoly_trim(&mut a);
        lpoly_trim(&mut bp);
        while !bp.is_empty() {
            let r = lpoly_rem(ctx, &a, &bp, phi, linv_exp);
            a = bp;
            bp = r;
        }
        g = a;
    }
    if g.is_empty() {
        None
    } else {
        Some(g.len() - 1)
    }
}

/// Does the affine curve `Σ f[j](x) y^j = 0` (coefficients in K) have a
/// singular point over the algebraic closure?
fn affine_singular(ctx: &Ctx, f: &[Vec<u32>], kdeg: u32, rng: &mut u64) -> bool {
    let b = &ctx.b;
    let dy = f.len() - 1;
    let fy: Vec<Vec<u32>> = (0..dy)
        .map(|j| gfp_scale(b, &f[j + 1], b.const_int((j + 1) as i64)))
        .collect();
    let fx: Vec<Vec<u32>> = f.iter().map(|c| gfp_derivative(b, c)).collect();
    let r1 = resultant_y(b, f, &fy);
    if r1.is_empty() {
        return true;
    }
    let r2 = resultant_y(b, f, &fx);
    let g = if r2.is_empty() {
        r1
    } else {
        gfp_gcd(b, &r1, &r2)
    };
    if g.len() <= 1 {
        return false;
    }
    let q = 3u64.pow(kdeg);
    for phi in irreducible_factors(ctx, &g, kdeg, rng) {
        let d = pdeg(&phi);
        let linv_exp = q.pow(d as u32) - 2;
        // powers of alpha = x mod phi
        let maxdeg = f.iter().map(|c| c.len()).max().unwrap_or(1);
        let mut pw: Vec<Vec<u32>> = vec![vec![0u32]];
        let alpha = gfp_rem(b, &[b.zero(), 0u32], &phi);
        for i in 1..maxdeg {
            let nxt = gfp_mulmod(b, &pw[i - 1], &alpha, &phi);
            pw.push(nxt);
        }
        let ev = |c: &[u32]| -> Vec<u32> {
            let mut acc: Vec<u32> = Vec::new();
            for (i, &ci) in c.iter().enumerate() {
                if !b.is_zero(ci) {
                    acc = gfp_add(b, &acc, &gfp_scale(b, &pw[i], ci));
                }
            }
            acc
        };
        let p0: Vec<Vec<u32>> = f.iter().map(|c| ev(c)).collect();
        let p1: Vec<Vec<u32>> = fy.iter().map(|c| ev(c)).collect();
        let p2: Vec<Vec<u32>> = fx.iter().map(|c| ev(c)).collect();
        match lgcd_deg(ctx, &[p0, p1, p2], &phi, linv_exp) {
            None => return true,
            Some(dg) if dg >= 1 => return true,
            _ => {}
        }
    }
    false
}

/// gcd of three univariate polynomials over the table field nonconstant (or
/// all zero)?
fn gcd3_singular(b: &Gf, a: &[u32], c: &[u32], d: &[u32]) -> bool {
    let g = gfp_gcd(b, &gfp_gcd(b, a, c), d);
    g.is_empty() || g.len() >= 2
}

/// Bicubic form `c[i][j]` (coefficient of x^i y^j, affine coordinates of
/// P^1 × P^1): smooth over the algebraic closure?
fn bicubic_smooth(ctx: &Ctx, c: &[[u32; 4]; 4], kdeg: u32, rng: &mut u64) -> bool {
    let b = &ctx.b;
    let z = b.zero();
    // (inf, inf)
    if c[3][3] == z && c[2][3] == z && c[3][2] == z {
        return false;
    }
    let mut g0: Vec<u32> = (0..4).map(|j| c[3][j]).collect();
    let mut g1: Vec<u32> = (0..4).map(|j| c[2][j]).collect();
    gfp_trim(b, &mut g0);
    gfp_trim(b, &mut g1);
    if gcd3_singular(b, &g0, &g1, &gfp_derivative(b, &g0)) {
        return false;
    }
    let mut h0: Vec<u32> = (0..4).map(|i| c[i][3]).collect();
    let mut h1: Vec<u32> = (0..4).map(|i| c[i][2]).collect();
    gfp_trim(b, &mut h0);
    gfp_trim(b, &mut h1);
    if gcd3_singular(b, &h0, &h1, &gfp_derivative(b, &h0)) {
        return false;
    }
    let f: Vec<Vec<u32>> = (0..4)
        .map(|j| {
            let mut p: Vec<u32> = (0..4).map(|i| c[i][j]).collect();
            gfp_trim(b, &mut p);
            p
        })
        .collect();
    !affine_singular(ctx, &f, kdeg, rng)
}

/// Cone form `y^3 + a2 y^2 + a4 y + a6` (a_d binary forms of degree d,
/// a[i] = coefficient of s^{d-i} t^i): smooth?
fn cone_smooth(ctx: &Ctx, a2: &[u32; 3], a4: &[u32; 5], a6: &[u32; 7], rng: &mut u64) -> bool {
    let b = &ctx.b;
    let z = b.zero();
    // fiber over (s:t) = (0:1), chart w = s/t at w = 0
    let mut p0 = vec![a6[6], a4[4], a2[2], 0u32];
    let mut p1 = vec![a4[4], b.mul(b.const_int(2), a2[2])];
    let mut p2 = vec![a6[5], a4[3], a2[1]];
    gfp_trim(b, &mut p0);
    gfp_trim(b, &mut p1);
    gfp_trim(b, &mut p2);
    if gcd3_singular(b, &p0, &p1, &p2) {
        return false;
    }
    let mut f2 = a2.to_vec();
    let mut f1 = a4.to_vec();
    let mut f0 = a6.to_vec();
    gfp_trim(b, &mut f2);
    gfp_trim(b, &mut f1);
    gfp_trim(b, &mut f0);
    let _ = z;
    let f = vec![f0, f1, f2, vec![0u32]];
    !affine_singular(ctx, &f, 1, rng)
}

// ---------------------------------------------------------------------------
// Part K: point counting.
// ---------------------------------------------------------------------------

fn mono3(b: &Gf, x: u32) -> [u32; 4] {
    let z = b.zero();
    if x == INF {
        [z, z, z, 0]
    } else if x == z {
        [0, z, z, z]
    } else {
        let x2 = b.mul(x, x);
        [0, x, x2, b.mul(x2, x)]
    }
}

fn bicubic_eval(b: &Gf, c: &[[u32; 4]; 4], mx: &[u32; 4], my: &[u32; 4]) -> u32 {
    let mut v = b.zero();
    for j in 0..4 {
        if b.is_zero(my[j]) {
            continue;
        }
        let mut a = b.zero();
        for i in 0..4 {
            if !b.is_zero(mx[i]) && !b.is_zero(c[i][j]) {
                a = b.add(a, b.mul(c[i][j], mx[i]));
            }
        }
        v = b.add(v, b.mul(a, my[j]));
    }
    v
}

struct Monos {
    /// k -> monomial vectors of P^1(F_{3^k})
    m: BTreeMap<u32, Vec<[u32; 4]>>,
}

impl Monos {
    fn new(ctx: &Ctx) -> Monos {
        let mut m = BTreeMap::new();
        for (&k, pts) in &ctx.p1 {
            if k <= 6 {
                m.insert(k, pts.iter().map(|&x| mono3(&ctx.b, x)).collect());
            }
        }
        Monos { m }
    }
}

fn count_split(ctx: &Ctx, mo: &Monos, c: &[[u32; 4]; 4], k: u32) -> u64 {
    let b = &ctx.b;
    let pts = &mo.m[&k];
    let mut n = 0u64;
    for mx in pts {
        let mut a = [b.zero(); 4];
        for (j, aj) in a.iter_mut().enumerate() {
            for i in 0..4 {
                if !b.is_zero(mx[i]) && !b.is_zero(c[i][j]) {
                    *aj = b.add(*aj, b.mul(c[i][j], mx[i]));
                }
            }
        }
        for my in pts {
            let mut v = b.zero();
            for j in 0..4 {
                if !b.is_zero(my[j]) && !b.is_zero(a[j]) {
                    v = b.add(v, b.mul(a[j], my[j]));
                }
            }
            if b.is_zero(v) {
                n += 1;
            }
        }
    }
    n
}

fn count_nonsplit(ctx: &Ctx, mo: &Monos, c: &[[u32; 4]; 4], k: u32) -> u64 {
    let b = &ctx.b;
    if k % 2 == 0 {
        return count_split(ctx, mo, c, k);
    }
    let pk = b.pk_mod(k);
    let mut n = 0u64;
    for &x in &ctx.p1[&(2 * k)] {
        let y = b.frob(x, pk);
        if b.is_zero(bicubic_eval(b, c, &mono3(b, x), &mono3(b, y))) {
            n += 1;
        }
    }
    n
}

fn count_cone(ctx: &Ctx, a2: &[u32; 3], a4: &[u32; 5], a6: &[u32; 7], k: u32) -> u64 {
    let b = &ctx.b;
    let ys = &ctx.sub[&k];
    let mut n = 0u64;
    let mut fiber = |c2: u32, c1: u32, c0: u32| {
        for &y in ys {
            let y2 = b.mul(y, y);
            let v = b.add(b.add(b.mul(y2, y), b.mul(c2, y2)), b.add(b.mul(c1, y), c0));
            if b.is_zero(v) {
                n += 1;
            }
        }
    };
    for &u in ys {
        fiber(peval(b, a2, u), peval(b, a4, u), peval(b, a6, u));
    }
    fiber(a2[2], a4[4], a6[6]);
    n
}

// ---------------------------------------------------------------------------
// Part K: forms, codes and group actions.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum QType {
    Split,
    Nonsplit,
    Cone,
}

impl QType {
    fn name(self) -> &'static str {
        match self {
            QType::Split => "split",
            QType::Nonsplit => "nonsplit",
            QType::Cone => "cone",
        }
    }
    fn ncodes(self) -> u64 {
        match self {
            QType::Split | QType::Nonsplit => 3u64.pow(16),
            QType::Cone => 3u64.pow(15),
        }
    }
    /// |G| times the number of scalar multiples counted per curve.
    fn mass_den(self) -> u64 {
        match self {
            QType::Split => 2 * 1152,
            QType::Nonsplit => 2 * 1440,
            QType::Cone => 1296,
        }
    }
}

const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

fn split_from_code(ctx: &Ctx, code: u64) -> [[u32; 4]; 4] {
    let mut c = [[ctx.b.zero(); 4]; 4];
    let mut v = code;
    for idx in 0..16 {
        c[idx / 4][idx % 4] = ctx.f3((v % 3) as u32);
        v /= 3;
    }
    c
}

fn split_code(ctx: &Ctx, c: &[[u32; 4]; 4]) -> u64 {
    let mut code = 0u64;
    for idx in (0..16).rev() {
        code = code * 3 + ctx.f3_digit(c[idx / 4][idx % 4]) as u64;
    }
    code
}

fn nonsplit_from_code(ctx: &Ctx, code: u64) -> [[u32; 4]; 4] {
    let b = &ctx.b;
    let mut c = [[b.zero(); 4]; 4];
    let mut v = code;
    for (i, row) in c.iter_mut().enumerate() {
        row[i] = ctx.f3((v % 3) as u32);
        v /= 3;
    }
    let p3 = b.pk_mod(1);
    for &(i, j) in &PAIRS {
        let e = ctx.f9[(v % 9) as usize];
        v /= 9;
        c[i][j] = e;
        c[j][i] = b.frob(e, p3);
    }
    c
}

fn nonsplit_code(ctx: &Ctx, c: &[[u32; 4]; 4]) -> u64 {
    let mut code = 0u64;
    for &(i, j) in PAIRS.iter().rev() {
        code = code * 9 + ctx.f9_index(c[i][j]) as u64;
    }
    for i in (0..4).rev() {
        code = code * 3 + ctx.f3_digit(c[i][i]) as u64;
    }
    code
}

type Cone = ([u32; 3], [u32; 5], [u32; 7]);

fn cone_from_code(ctx: &Ctx, code: u64) -> Cone {
    let z = ctx.b.zero();
    let (mut a2, mut a4, mut a6) = ([z; 3], [z; 5], [z; 7]);
    let mut v = code;
    for x in a2.iter_mut().chain(a4.iter_mut()).chain(a6.iter_mut()) {
        *x = ctx.f3((v % 3) as u32);
        v /= 3;
    }
    (a2, a4, a6)
}

fn cone_code(ctx: &Ctx, f: &Cone) -> u64 {
    let all: Vec<u32> =
        f.0.iter()
            .chain(f.1.iter())
            .chain(f.2.iter())
            .copied()
            .collect();
    let mut code = 0u64;
    for &x in all.iter().rev() {
        code = code * 3 + ctx.f3_digit(x) as u64;
    }
    code
}

/// Fixed-length product of binary forms (index = power of the second variable).
fn bmul(b: &Gf, a: &[u32], c: &[u32]) -> Vec<u32> {
    let mut out = vec![b.zero(); a.len() + c.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if b.is_zero(x) {
            continue;
        }
        for (j, &y) in c.iter().enumerate() {
            if !b.is_zero(y) {
                out[i + j] = b.add(out[i + j], b.mul(x, y));
            }
        }
    }
    out
}

fn badd(b: &Gf, a: &[u32], c: &[u32]) -> Vec<u32> {
    let n = a.len().max(c.len());
    (0..n)
        .map(|i| {
            b.add(
                a.get(i).copied().unwrap_or(b.zero()),
                c.get(i).copied().unwrap_or(b.zero()),
            )
        })
        .collect()
}

fn bscale(b: &Gf, a: &[u32], s: u32) -> Vec<u32> {
    a.iter().map(|&x| b.mul(x, s)).collect()
}

/// `F(A·(x0, x1))` for a binary form of degree `coeffs.len() - 1`.
fn subst_form(b: &Gf, coeffs: &[u32], a: [[u32; 2]; 2]) -> Vec<u32> {
    let d = coeffs.len() - 1;
    let l0 = vec![a[0][0], a[0][1]];
    let l1 = vec![a[1][0], a[1][1]];
    let mut p0 = vec![vec![0u32]];
    let mut p1 = vec![vec![0u32]];
    for k in 1..=d {
        p0.push(bmul(b, &p0[k - 1], &l0));
        p1.push(bmul(b, &p1[k - 1], &l1));
    }
    let mut out = vec![b.zero(); d + 1];
    for (i, &c) in coeffs.iter().enumerate() {
        if b.is_zero(c) {
            continue;
        }
        let term = bmul(b, &p0[d - i], &p1[i]);
        out = badd(b, &out, &bscale(b, &term, c));
    }
    out
}

fn tmat(b: &Gf, a: [[u32; 2]; 2]) -> [[u32; 4]; 4] {
    let mut t = [[b.zero(); 4]; 4];
    for (i, row) in t.iter_mut().enumerate() {
        let mut e = vec![b.zero(); 4];
        e[i] = 0;
        let s = subst_form(b, &e, a);
        row.copy_from_slice(&s);
    }
    t
}

fn apply_t(b: &Gf, c: &[[u32; 4]; 4], ta: &[[u32; 4]; 4], tb: &[[u32; 4]; 4]) -> [[u32; 4]; 4] {
    let mut mid = [[b.zero(); 4]; 4]; // mid[k][j] = Σ_i c[i][j] ta[i][k]
    for i in 0..4 {
        for j in 0..4 {
            if b.is_zero(c[i][j]) {
                continue;
            }
            for k in 0..4 {
                if !b.is_zero(ta[i][k]) {
                    mid[k][j] = b.add(mid[k][j], b.mul(c[i][j], ta[i][k]));
                }
            }
        }
    }
    let mut out = [[b.zero(); 4]; 4];
    for k in 0..4 {
        for j in 0..4 {
            if b.is_zero(mid[k][j]) {
                continue;
            }
            for l in 0..4 {
                if !b.is_zero(tb[j][l]) {
                    out[k][l] = b.add(out[k][l], b.mul(mid[k][j], tb[j][l]));
                }
            }
        }
    }
    out
}

fn transpose4(c: &[[u32; 4]; 4]) -> [[u32; 4]; 4] {
    let mut t = *c;
    for i in 0..4 {
        for j in 0..4 {
            t[i][j] = c[j][i];
        }
    }
    t
}

/// PGL_2 representatives over the field with element list `elems`
/// (first nonzero entry in row-major order equal to one).
fn pgl2(b: &Gf, elems: &[u32]) -> Vec<[[u32; 2]; 2]> {
    let mut out = Vec::new();
    for &a in elems {
        for &bb in elems {
            for &c in elems {
                for &d in elems {
                    let det = b.sub(b.mul(a, d), b.mul(bb, c));
                    if b.is_zero(det) {
                        continue;
                    }
                    let first = [a, bb, c, d].into_iter().find(|&x| !b.is_zero(x)).unwrap();
                    if first == 0 {
                        out.push([[a, bb], [c, d]]);
                    }
                }
            }
        }
    }
    out
}

fn gl2(b: &Gf, elems: &[u32]) -> Vec<[[u32; 2]; 2]> {
    let mut out = Vec::new();
    for &a in elems {
        for &bb in elems {
            for &c in elems {
                for &d in elems {
                    if !b.is_zero(b.sub(b.mul(a, d), b.mul(bb, c))) {
                        out.push([[a, bb], [c, d]]);
                    }
                }
            }
        }
    }
    out
}

fn scale4(b: &Gf, c: &[[u32; 4]; 4], s: u32) -> [[u32; 4]; 4] {
    let mut o = *c;
    for row in o.iter_mut() {
        for x in row.iter_mut() {
            *x = b.mul(*x, s);
        }
    }
    o
}

/// Orbit (as codes) of a Res-shaped form under the quadric's automorphisms,
/// scalar multiples included.
fn orbit_codes(ctx: &Ctx, ty: QType, code: u64, groups: &Groups) -> HashSet<u64> {
    let b = &ctx.b;
    let mut out = HashSet::new();
    match ty {
        QType::Split => {
            let c = split_from_code(ctx, code);
            for base in [c, transpose4(&c)] {
                for ta in &groups.t3 {
                    for tb in &groups.t3 {
                        let img = apply_t(b, &base, ta, tb);
                        out.insert(split_code(ctx, &img));
                        out.insert(split_code(ctx, &scale4(b, &img, ctx.minus_one)));
                    }
                }
            }
        }
        QType::Nonsplit => {
            let c = nonsplit_from_code(ctx, code);
            let p3 = b.pk_mod(1);
            for base in [c, transpose4(&c)] {
                for (ta, tb) in &groups.t9pairs {
                    let img = apply_t(b, &base, ta, tb);
                    // λ with img^σ = λ img, where img^σ_{ij} = img_{ji}^3
                    let (i0, j0) = (0..16)
                        .map(|k| (k / 4, k % 4))
                        .find(|&(i, j)| !b.is_zero(img[i][j]))
                        .unwrap();
                    let lam = b.mul(b.frob(img[j0][i0], p3), b.inv(img[i0][j0]));
                    let mu = ctx.f9[1..]
                        .iter()
                        .copied()
                        .find(|&m| b.mul(b.mul(m, m), lam) == 0)
                        .expect("lambda is a square in F_9");
                    for s in [mu, b.neg(mu)] {
                        let f = scale4(b, &img, s);
                        for i in 0..4 {
                            for j in 0..4 {
                                assert_eq!(f[i][j], b.frob(f[j][i], p3), "sigma structure");
                            }
                        }
                        out.insert(nonsplit_code(ctx, &f));
                    }
                }
            }
        }
        QType::Cone => {
            let (a2, a4, a6) = cone_from_code(ctx, code);
            for a in &groups.gl2f3 {
                let s2 = subst_form(b, &a2, *a);
                let s4 = subst_form(b, &a4, *a);
                let s6 = subst_form(b, &a6, *a);
                for &lam in &[0u32, ctx.minus_one] {
                    for bq in &groups.quads {
                        // y -> lam*y + bq
                        let b2 = bmul(b, bq, bq);
                        let b3 = bmul(b, &b2, bq);
                        let two = b.const_int(2);
                        let c2 = s2.clone(); // lam^2 = 1
                        let c1 = badd(
                            b,
                            &bscale(b, &bmul(b, bq, &s2), b.mul(two, lam)),
                            &bscale(b, &s4, lam),
                        );
                        let c0 = badd(
                            b,
                            &badd(b, &bmul(b, &b2, &s2), &bmul(b, bq, &s4)),
                            &badd(b, &s6, &b3),
                        );
                        // divide by lam^3 = lam (lam^{-1} = lam)
                        let n2: Vec<u32> = bscale(b, &c2, lam);
                        let n1: Vec<u32> = bscale(b, &c1, lam);
                        let n0: Vec<u32> = bscale(b, &c0, lam);
                        let mut f2 = [b.zero(); 3];
                        let mut f1 = [b.zero(); 5];
                        let mut f0 = [b.zero(); 7];
                        f2.copy_from_slice(&n2[..3]);
                        f1.copy_from_slice(&n1[..5]);
                        f0.copy_from_slice(&n0[..7]);
                        out.insert(cone_code(ctx, &(f2, f1, f0)));
                    }
                }
            }
        }
    }
    out
}

struct Groups {
    t3: Vec<[[u32; 4]; 4]>,
    t9pairs: Vec<([[u32; 4]; 4], [[u32; 4]; 4])>,
    gl2f3: Vec<[[u32; 2]; 2]>,
    quads: Vec<Vec<u32>>,
}

impl Groups {
    fn new(ctx: &Ctx) -> Groups {
        let b = &ctx.b;
        let f3: Vec<u32> = ctx.sub[&1].clone();
        let t3: Vec<[[u32; 4]; 4]> = pgl2(b, &f3).into_iter().map(|a| tmat(b, a)).collect();
        assert_eq!(t3.len(), 24);
        let p3 = b.pk_mod(1);
        let t9pairs: Vec<_> = pgl2(b, &ctx.f9)
            .into_iter()
            .map(|g| {
                let g3 = [
                    [b.frob(g[0][0], p3), b.frob(g[0][1], p3)],
                    [b.frob(g[1][0], p3), b.frob(g[1][1], p3)],
                ];
                (tmat(b, g), tmat(b, g3))
            })
            .collect();
        assert_eq!(t9pairs.len(), 720);
        let gl2f3 = gl2(b, &f3);
        assert_eq!(gl2f3.len(), 48);
        let mut quads = Vec::new();
        for &x in &f3 {
            for &y in &f3 {
                for &z in &f3 {
                    quads.push(vec![x, y, z]);
                }
            }
        }
        Groups {
            t3,
            t9pairs,
            gl2f3,
            quads,
        }
    }
}

// ---------------------------------------------------------------------------
// Part K: drivers.
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
struct KStats {
    forms: u64,
    smooth: u64,
    n1_4: u64,
    res: Vec<(u64, u64)>, // (code, N4)
}

fn scan_range(ctx: &Ctx, mo: &Monos, ty: QType, lo: u64, hi: u64, seed: u64) -> KStats {
    let mut st = KStats::default();
    let mut rng = seed | 1;
    for code in lo..hi {
        if code == 0 {
            continue;
        }
        st.forms += 1;
        let (smooth, n1, counts): (bool, u64, Box<dyn Fn(u32) -> u64>) = match ty {
            QType::Split => {
                let c = split_from_code(ctx, code);
                let n1 = count_split(ctx, mo, &c, 1);
                let sm = bicubic_smooth(ctx, &c, 1, &mut rng);
                (sm, n1, Box::new(move |k| count_split(ctx, mo, &c, k)))
            }
            QType::Nonsplit => {
                let c = nonsplit_from_code(ctx, code);
                let n1 = count_nonsplit(ctx, mo, &c, 1);
                let sm = bicubic_smooth(ctx, &c, 2, &mut rng);
                (sm, n1, Box::new(move |k| count_nonsplit(ctx, mo, &c, k)))
            }
            QType::Cone => {
                let (a2, a4, a6) = cone_from_code(ctx, code);
                let n1 = count_cone(ctx, &a2, &a4, &a6, 1);
                let sm = cone_smooth(ctx, &a2, &a4, &a6, &mut rng);
                (sm, n1, Box::new(move |k| count_cone(ctx, &a2, &a4, &a6, k)))
            }
        };
        if !smooth {
            continue;
        }
        st.smooth += 1;
        if n1 != 4 {
            continue;
        }
        st.n1_4 += 1;
        if counts(2) != 10 || counts(3) != 28 {
            continue;
        }
        st.res.push((code, counts(4)));
    }
    st
}

fn run_type(ctx: &Ctx, mo: &Monos, ty: QType, threads: usize, limit: Option<u64>) -> KStats {
    let total = limit.unwrap_or(ty.ncodes());
    let chunk = total.div_ceil(threads as u64);
    let parts: Vec<KStats> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..threads as u64)
            .map(|t| {
                let lo = t * chunk;
                let hi = ((t + 1) * chunk).min(total);
                s.spawn(move || scan_range(ctx, mo, ty, lo, hi, 0x9e3779b97f4a7c15 ^ (t + 1)))
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut st = KStats::default();
    for p in parts {
        st.forms += p.forms;
        st.smooth += p.smooth;
        st.n1_4 += p.n1_4;
        st.res.extend(p.res);
    }
    st.res.sort_unstable();
    st
}

#[derive(Clone, Debug)]
struct KClass {
    ty: QType,
    rep: u64,
    orbit: usize,
    aut: f64,
    n4: u64,
    t: i64,
}

fn elliptic_trace_exists(t: i64) -> bool {
    t.abs() <= 18 && (t % 3 != 0 || [0, 9, -9, 18, -18].contains(&t))
}

fn classify(
    ctx: &Ctx,
    groups: &Groups,
    ty: QType,
    st: &KStats,
    log: &mut File,
) -> (Vec<KClass>, usize) {
    let res: HashMap<u64, u64> = st.res.iter().copied().collect();
    let mut visited: HashSet<u64> = HashSet::new();
    let mut classes = Vec::new();
    let mut k5_viol = 0usize;
    for &(code, n4) in &st.res {
        if visited.contains(&code) {
            continue;
        }
        let orb = orbit_codes(ctx, ty, code, groups);
        for c in &orb {
            match res.get(c) {
                Some(&m) if m == n4 => {}
                _ => {
                    k5_viol += 1;
                }
            }
            visited.insert(*c);
        }
        let den = ty.mass_den();
        if den % orb.len() as u64 != 0 {
            k5_viol += 1;
        }
        let tnum = 82 - n4 as i64;
        let t = if tnum % 4 == 0 { tnum / 4 } else { i64::MIN };
        classes.push(KClass {
            ty,
            rep: *orb.iter().min().unwrap(),
            orbit: orb.len(),
            aut: den as f64 / orb.len() as f64,
            n4,
            t,
        });
    }
    let _ = writeln!(
        log,
        "classify {}: {} classes, K5 violations {}",
        ty.name(),
        classes.len(),
        k5_viol
    );
    (classes, k5_viol)
}

fn describe(ctx: &Ctx, ty: QType, code: u64) -> String {
    let b = &ctx.b;
    let show = |x: u32| -> String {
        if b.is_zero(x) {
            "0".into()
        } else if x == 0 {
            "1".into()
        } else if x == ctx.minus_one {
            "2".into()
        } else {
            format!("w^{}", x / b.step(2))
        }
    };
    match ty {
        QType::Split | QType::Nonsplit => {
            let c = if ty == QType::Split {
                split_from_code(ctx, code)
            } else {
                nonsplit_from_code(ctx, code)
            };
            let mut terms = Vec::new();
            for (i, row) in c.iter().enumerate() {
                for (j, &x) in row.iter().enumerate() {
                    if !b.is_zero(x) {
                        terms.push(format!("{}*x^{i}y^{j}", show(x)));
                    }
                }
            }
            terms.join(" + ")
        }
        QType::Cone => {
            let (a2, a4, a6) = cone_from_code(ctx, code);
            let f = |a: &[u32]| a.iter().map(|&x| show(x)).collect::<Vec<_>>().join(" ");
            format!("y^3 + [{}] y^2 + [{}] y + [{}]", f(&a2), f(&a4), f(&a6))
        }
    }
}

fn part_k(ctx: &Ctx, out: &Path, log: &mut File, threads: usize, limit: Option<u64>) -> String {
    let mo = Monos::new(ctx);
    let groups = Groups::new(ctx);
    let mut summary = File::create(out.join("k_summary.csv")).unwrap();
    writeln!(
        summary,
        "type,forms,smooth,mass_den,mass,smooth_with_N1_4,res_shaped_forms,res_shaped_classes,k5_violations,seconds"
    )
    .unwrap();
    let mut cls_f = File::create(out.join("k_classes.csv")).unwrap();
    writeln!(
        cls_f,
        "type,rep_code,orbit_forms,aut,N4,t,numE,numE_prime,elliptic_trace_exists,form"
    )
    .unwrap();
    let mut mass_total = 0f64;
    let mut all_classes = Vec::new();
    let mut k5_total = 0usize;
    for ty in [QType::Split, QType::Nonsplit, QType::Cone] {
        let t0 = Instant::now();
        let st = run_type(ctx, &mo, ty, threads, limit);
        let (classes, k5) = classify(ctx, &groups, ty, &st, log);
        k5_total += k5;
        let mass = st.smooth as f64 / ty.mass_den() as f64;
        mass_total += mass;
        let secs = t0.elapsed().as_secs_f64();
        writeln!(
            summary,
            "{},{},{},{},{:.6},{},{},{},{},{:.1}",
            ty.name(),
            st.forms,
            st.smooth,
            ty.mass_den(),
            mass,
            st.n1_4,
            st.res.len(),
            classes.len(),
            k5,
            secs
        )
        .unwrap();
        let _ = writeln!(
            log,
            "part K {}: forms {} smooth {} mass {:.4} res_forms {} classes {} ({:.1}s)",
            ty.name(),
            st.forms,
            st.smooth,
            mass,
            st.res.len(),
            classes.len(),
            secs
        );
        for c in &classes {
            let num_e = 82 - c.t;
            writeln!(
                cls_f,
                "{},{},{},{},{},{},{},{},{},{}",
                ty.name(),
                c.rep,
                c.orbit,
                c.aut,
                c.n4,
                c.t,
                num_e,
                c.t != i64::MIN && is_prime(num_e as u64),
                c.t != i64::MIN && elliptic_trace_exists(c.t),
                describe(ctx, ty, c.rep)
            )
            .unwrap();
        }
        all_classes.extend(classes);
    }
    let k4_viol = all_classes
        .iter()
        .filter(|c| c.t == i64::MIN || c.t.abs() > 18)
        .count();
    let prime_classes: Vec<&KClass> = all_classes
        .iter()
        .filter(|c| c.t != i64::MIN && elliptic_trace_exists(c.t) && is_prime((82 - c.t) as u64))
        .collect();
    let mut t_hist: BTreeMap<i64, usize> = BTreeMap::new();
    for c in &all_classes {
        *t_hist.entry(c.t).or_insert(0) += 1;
    }
    let lit = 3f64.powi(9) + 3f64.powi(8) - 3f64.powi(6);
    let s = format!(
        "K: non-hyperelliptic mass total = {:.6} (check K3 literature value q^9+q^8-q^6 = {lit}: {})\nK: Res-shaped classes = {}; t histogram (class counts) {:?}\nK: check K4 violations = {k4_viol}; check K5 violations = {k5_total}\nK: P6 prime-order classes (82 - t prime, elliptic trace exists) = {} : {:?}\n",
        mass_total,
        if (mass_total - lit).abs() < 1e-6 { "match" } else { "MISMATCH" },
        all_classes.len(),
        t_hist,
        prime_classes.len(),
        prime_classes
            .iter()
            .map(|c| format!("{}:t={}:#E={}:aut={}", c.ty.name(), c.t, 82 - c.t, c.aut))
            .collect::<Vec<_>>()
    );
    let _ = write!(log, "{s}");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <out-dir> [T|H4|K ...]", args[0]);
        std::process::exit(2);
    }
    let out = Path::new(&args[1]);
    std::fs::create_dir_all(out).unwrap();
    let parts: Vec<String> = if args.len() > 2 {
        args[2..].to_vec()
    } else {
        vec!["T".into(), "H4".into(), "K".into()]
    };
    let threads: usize = std::env::var("G4_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    let limit: Option<u64> = std::env::var("G4_LIMIT").ok().and_then(|s| s.parse().ok());
    let mut log = File::create(out.join("tool_log.txt")).unwrap();
    let t0 = Instant::now();
    let mut checks = String::new();
    if parts.iter().any(|p| p == "T") {
        checks.push_str(&part_t(out, &mut log));
    }
    if parts.iter().any(|p| p == "H4" || p == "K") {
        let ctx = Ctx::new();
        let _ = writeln!(log, "field F_3^12 ready {:.1}s", t0.elapsed().as_secs_f64());
        if parts.iter().any(|p| p == "H4") {
            checks.push_str(&part_h4(&ctx, out, &mut log));
        }
        if parts.iter().any(|p| p == "K") {
            checks.push_str(&part_k(&ctx, out, &mut log, threads, limit));
        }
    }
    std::fs::write(out.join("checks.txt"), &checks).unwrap();
    let _ = writeln!(log, "total {:.1}s", t0.elapsed().as_secs_f64());
    eprint!("{checks}");
    eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Brute-force: a singular point over P^1(F_{3^k})^2, k | 12, k ≤ kmax.
    fn bicubic_brute_singular(ctx: &Ctx, c: &[[u32; 4]; 4], ks: &[u32]) -> bool {
        let b = &ctx.b;
        for rx in [false, true] {
            for ry in [false, true] {
                // chart coefficients: x' = 1/x reverses the i index, y' = 1/y the j index
                let mut cc = [[b.zero(); 4]; 4];
                for i in 0..4 {
                    for j in 0..4 {
                        cc[if rx { 3 - i } else { i }][if ry { 3 - j } else { j }] = c[i][j];
                    }
                }
                let f: Vec<Vec<u32>> = (0..4).map(|j| (0..4).map(|i| cc[i][j]).collect()).collect();
                for &k in ks {
                    for &x in &ctx.sub[&k] {
                        for &y in &ctx.sub[&k] {
                            let mut v = b.zero();
                            let mut vx = b.zero();
                            let mut vy = b.zero();
                            let mut yp = 0u32;
                            for (j, fj) in f.iter().enumerate() {
                                let a = peval(b, fj, x);
                                let ax = peval(b, &gfp_derivative(b, fj), x);
                                v = b.add(v, b.mul(a, yp));
                                vx = b.add(vx, b.mul(ax, yp));
                                if j >= 1 {
                                    let ypm = ctx.pow(y, j as u32 - 1);
                                    let ypm = if b.is_zero(y) && j == 1 { 0 } else { ypm };
                                    vy = b.add(vy, b.mul(b.mul(a, b.const_int(j as i64)), ypm));
                                }
                                yp = b.mul(yp, y);
                            }
                            if b.is_zero(v) && b.is_zero(vx) && b.is_zero(vy) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    #[test]
    fn part_t_matches_round_one() {
        let rows = part_t_rows(5);
        let r3 = rows.iter().find(|r| r.n == 3 && !r.t_odd).unwrap();
        let mut got: Vec<Vec<usize>> = r3.solutions.clone();
        got.sort();
        let mut want = vec![vec![6, 2], vec![6, 1, 1], vec![3, 3, 2], vec![3, 3, 1, 1]];
        want.sort();
        assert_eq!(got, want);
        assert!(rows
            .iter()
            .filter(|r| r.n >= 3 && r.t_odd)
            .all(|r| r.solutions.is_empty()));
    }

    /// Exact smoothness agrees with brute force: "smooth" never has a
    /// singular point over F_{3^k}, k ∈ {1,2,3,4}; "singular" usually has one.
    #[test]
    fn smoothness_vs_brute_force() {
        let ctx = Ctx::new();
        let mut rng = 0x1234_5678_9abc_def1u64;
        let (mut smooth_n, mut sing_n, mut sing_found) = (0, 0, 0);
        for _ in 0..60 {
            let code = xorshift(&mut rng) % 3u64.pow(16);
            for (c, kdeg) in [
                (split_from_code(&ctx, code), 1u32),
                (nonsplit_from_code(&ctx, code), 2),
            ] {
                let sm = bicubic_smooth(&ctx, &c, kdeg, &mut rng);
                let ks: &[u32] = if kdeg == 1 { &[1, 2, 3, 4] } else { &[2, 4] };
                let br = bicubic_brute_singular(&ctx, &c, ks);
                if sm {
                    smooth_n += 1;
                    assert!(
                        !br,
                        "exact says smooth but brute force found a singular point"
                    );
                } else {
                    sing_n += 1;
                    if br {
                        sing_found += 1;
                    }
                }
            }
        }
        assert!(
            smooth_n > 20 && sing_n > 5,
            "smooth {smooth_n} singular {sing_n}"
        );
        assert!(
            sing_found * 10 >= sing_n * 7,
            "singular {sing_n}, brute found {sing_found}"
        );
    }

    /// A reducible form is singular; x^3 y^3 + ... examples.
    #[test]
    fn reducible_is_singular() {
        let ctx = Ctx::new();
        let b = &ctx.b;
        let mut rng = 7u64;
        // (x - y) * (random (2,2)) is reducible
        let mut c = [[b.zero(); 4]; 4];
        let g = [[1u32, 2, 0], [0, 1, 1], [2, 0, 1]];
        for i in 0..3 {
            for j in 0..3 {
                let v = ctx.f3(g[i][j]);
                c[i + 1][j] = b.add(c[i + 1][j], v); // x * g
                c[i][j + 1] = b.sub(c[i][j + 1], v); // - y * g
            }
        }
        assert!(!bicubic_smooth(&ctx, &c, 1, &mut rng));
    }

    /// Point counts are invariant on group orbits (split and cone).
    #[test]
    fn orbits_preserve_counts() {
        let ctx = Ctx::new();
        let mo = Monos::new(&ctx);
        let groups = Groups::new(&ctx);
        let mut rng = 99u64;
        let mut tested = 0;
        while tested < 3 {
            let code = xorshift(&mut rng) % 3u64.pow(16);
            let c = split_from_code(&ctx, code);
            if !bicubic_smooth(&ctx, &c, 1, &mut rng) {
                continue;
            }
            let n: Vec<u64> = (1..=3).map(|k| count_split(&ctx, &mo, &c, k)).collect();
            let orb = orbit_codes(&ctx, QType::Split, code, &groups);
            assert_eq!((2 * 1152) % orb.len(), 0);
            for &o in orb.iter().take(200) {
                let co = split_from_code(&ctx, o);
                let m: Vec<u64> = (1..=3).map(|k| count_split(&ctx, &mo, &co, k)).collect();
                assert_eq!(n, m);
            }
            tested += 1;
        }
        let mut tested = 0;
        while tested < 3 {
            let code = xorshift(&mut rng) % 3u64.pow(15);
            let (a2, a4, a6) = cone_from_code(&ctx, code);
            if !cone_smooth(&ctx, &a2, &a4, &a6, &mut rng) {
                continue;
            }
            let n: Vec<u64> = (1..=3)
                .map(|k| count_cone(&ctx, &a2, &a4, &a6, k))
                .collect();
            let orb = orbit_codes(&ctx, QType::Cone, code, &groups);
            assert_eq!(1296 % orb.len(), 0, "orbit {}", orb.len());
            for &o in orb.iter().take(200) {
                let (b2, b4, b6) = cone_from_code(&ctx, o);
                let m: Vec<u64> = (1..=3)
                    .map(|k| count_cone(&ctx, &b2, &b4, &b6, k))
                    .collect();
                assert_eq!(n, m);
            }
            tested += 1;
        }
        let mut tested = 0;
        while tested < 2 {
            let code = xorshift(&mut rng) % 3u64.pow(16);
            let c = nonsplit_from_code(&ctx, code);
            if !bicubic_smooth(&ctx, &c, 2, &mut rng) {
                continue;
            }
            let n: Vec<u64> = (1..=3).map(|k| count_nonsplit(&ctx, &mo, &c, k)).collect();
            let orb = orbit_codes(&ctx, QType::Nonsplit, code, &groups);
            assert_eq!((2 * 1440) % orb.len(), 0, "orbit {}", orb.len());
            for &o in orb.iter().take(100) {
                let co = nonsplit_from_code(&ctx, o);
                let m: Vec<u64> = (1..=3).map(|k| count_nonsplit(&ctx, &mo, &co, k)).collect();
                assert_eq!(n, m);
            }
            tested += 1;
        }
    }
}
