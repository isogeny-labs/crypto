//! Round 7, lead R7.1: complete-cost ordering of kernel-finding routes.
//!
//! R5.3 and R6.4 priced every row as `M + W + 91 I` only.  This example
//! re-runs R6.4's three routes on the same instances (`j = −3375`,
//! `D = −7`, 61-bit `p`, seeds `0x1630 ^ ℓ`) with a counter that also tallies
//! additions (`PrimeField::counted_with_adds`; `M`, `W`, `I` are unchanged by
//! that, so the `a = 0` columns reproduce R6.4's frozen counts, checked in
//! `analyze`), and adds what R6.4 left out of route (c):
//!
//! * `ladder`, `ladder_list`: (a) the `F_{p^k}` ladder (`PolyMul::Auto`),
//!   descending and ascending, with the cost of evaluating the image of one
//!   point from the kernel polynomial;
//! * `project`: (a′) the ladder's per-step cost measured in `F_{p^k}` up to
//!   `ℓ = 2^16`, with additions;
//! * `psi`: (b) `ψ_ℓ` and equal-degree splitting / Frobenius gcd;
//! * `modpoly`: (c) a counted port of the library's `Φ_ℓ mod p` builder
//!   (the q-series algorithm, same loops), evaluation of `Φ_ℓ(j, Y)`, root
//!   finding, and the counted Elkies kernel polynomial (port of
//!   `kernel::elkies_isogeny` without its certificate), each compared with the
//!   library's output.  Sutherland's CRT evaluation is not implemented;
//! * `selftest`: the fast builder against a per-operation port and the
//!   library;
//! * `analyze`: fits with OLS and bootstrap intervals per addition weight
//!   `a ∈ {0, 0.1, 0.3, 1}` (an addition priced at `a·M`), the ordering table
//!   per `(ℓ, a)`, the two falsifiers and the crossover with rho.
//!
//! Instances are those of `examples/r6_4_vertical_margin_2.rs`, ported
//! unchanged (same RNG calls), so `ℓ` gives the same `p`.
//!
//! Wall times are contended (several agents share four cores).
//!
//! Usage:
//!   r7_1_complete_cost ladder <min_ell> <max_ell> <step_log2> <cap_s> <karatsuba|auto|ntt> <reps> <out.json>
//!   r7_1_complete_cost ladder_list <ell,...> <cap_s> <karatsuba|auto|ntt> <reps> <out.json>
//!   r7_1_complete_cost project <min_log2> <max_log2> <step_log2> <arms> <out.json>
//!   r7_1_complete_cost psi <ell,...> <cap_s> <out.json>
//!   r7_1_complete_cost modpoly <min_ell> <max_ell> <cap_s> <lib_check_max_ell> <out.json>
//!   r7_1_complete_cost selftest <ell,...> <out.json>
//!   r7_1_complete_cost analyze <dir> <r6_4_dir> <out.json>

use crypto_lib::cryptanalysis::isogeny_eval::montgomery::{xadd, xdbl, FieldArith};
use crypto_lib::cryptanalysis::isogeny_eval::poly::{self, MonicDivisor};
use crypto_lib::cryptanalysis::isogeny_eval::{CompleteTally, PolyMul, PrimeField, XZ};
use crypto_lib::cryptanalysis::isogeny_walk::curve::Model;
use crypto_lib::cryptanalysis::isogeny_walk::field::Field as LField;
use crypto_lib::cryptanalysis::isogeny_walk::{kernel, modpoly};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, ToPrimitive, Zero};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;

type Fp = PrimeField<crypto_lib::cryptanalysis::isogeny_eval::OpCounts>;

const INV_COST: u64 = 91;

/// `Meq = M + W + 91 I`: R6.4's number, additions excluded.
fn meq_of(t: &CompleteTally) -> u64 {
    t.ops.mul_equivalents(INV_COST)
}

/// The addition weights `a` (in units of `M` per addition or subtraction).
const WEIGHTS: [f64; 4] = [0.0, 0.1, 0.3, 1.0];

/// `Meq + a·(A + WA)`.
fn cost_a(t: &CompleteTally, a: f64) -> f64 {
    t.weighted(INV_COST, a)
}

fn tjs(t: &CompleteTally) -> Value {
    json!({"M": t.ops.mul, "W": t.ops.word, "I": t.ops.inv, "Meq": meq_of(t),
           "A": t.adds.add, "WA": t.adds.word_add})
}

fn field(p: u64, m: PolyMul) -> Fp {
    PrimeField::counted_with_adds(p)
        .expect("prime in [5, 2^63)")
        .with_poly_mul(m)
}

fn red(f: &Fp, v: i128) -> u64 {
    v.rem_euclid(f.p() as i128) as u64
}

fn inv(f: &Fp, a: u64) -> u64 {
    f.inv(a).expect("inverse of zero")
}

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    poly::trim(&mut a);
    a
}

fn j_of_a(f: &Fp, a: u64) -> u64 {
    let a2 = f.mul(a, a);
    let t = f.sub(a2, 3);
    let num = f.mul(256, f.mul(t, f.mul(t, t)));
    f.mul(num, inv(f, f.sub(a2, 4)))
}

// ---------------------------------------------------------------------------
// Number theory helpers (as in round 4).

fn is_prime_u64(n: u64) -> bool {
    crypto_lib::cryptanalysis::isogeny_eval::counted_fp::is_prime_u64(n)
}

fn next_prime(mut n: u64) -> u64 {
    while !is_prime_u64(n) {
        n += 1;
    }
    n
}

fn primitive_root(l: u64) -> u64 {
    let mut fac = Vec::new();
    let mut m = l - 1;
    let mut d = 2;
    while d * d <= m {
        if m.is_multiple_of(d) {
            fac.push(d);
            while m.is_multiple_of(d) {
                m /= d;
            }
        }
        d += 1;
    }
    if m > 1 {
        fac.push(m);
    }
    let pw = |a: u64, mut e: u64| {
        let mut r = 1u128;
        let mut b = a as u128;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % l as u128;
            }
            b = b * b % l as u128;
            e >>= 1;
        }
        r as u64
    };
    (2..l)
        .find(|&g| fac.iter().all(|&q| pw(g, (l - 1) / q) != 1))
        .unwrap()
}

fn val(n: &BigUint, l: u64) -> (u32, BigUint) {
    let lb = BigUint::from(l);
    let mut n = n.clone();
    let mut e = 0;
    while !n.is_zero() && (&n % &lb).is_zero() {
        n /= &lb;
        e += 1;
    }
    (e, n)
}

// ---------------------------------------------------------------------------
// x-only ladder with a big scalar, generic over the library's FieldArith.

fn ladder_big<F: FieldArith>(f: &F, p: &XZ<F::Elem>, n: &BigUint, a24: &F::Elem) -> XZ<F::Elem> {
    if n.is_zero() {
        return XZ {
            x: p.x.clone(),
            z: f.zero(),
        };
    }
    let mut r0 = p.clone();
    let mut r1 = xdbl(f, p, a24);
    for i in (0..n.bits() - 1).rev() {
        if n.bit(i) {
            r0 = xadd(f, &r1, &r0, p);
            r1 = xdbl(f, &r1, a24);
        } else {
            r1 = xadd(f, &r1, &r0, p);
            r0 = xdbl(f, &r0, a24);
        }
    }
    r0
}

// ---------------------------------------------------------------------------
// Instances (ported from round 4 unchanged in behaviour).

#[derive(Clone, Debug)]
#[allow(dead_code)] // ell and lambda kept for Debug output, as in round 4
struct Inst {
    ell: u64,
    lambda: u64,
    p: u64,
    t: i64,
    v: u64,
    n: u128,
    a: u64,
    a24: u64,
    chi_n: i32,
    alpha: u64,
    s: u64,
    jcrater: u64,
}

fn make_instance(ell: u64, lambda: u64, rng: &mut StdRng) -> Inst {
    let lo: u128 = 1 << 62;
    let hi: u128 = 1 << 63;
    loop {
        let wmax = (((hi - 1) / 7) as f64).sqrt() as u128 / ell as u128;
        let w = rng.gen_range(1..=wmax.max(1)) as u64;
        let v = ell * w;
        let sv = 7u128 * (v as u128) * (v as u128);
        if sv >= hi {
            continue;
        }
        let tmax = ((hi - 1 - sv) as f64).sqrt() as i128;
        let tmin = if sv >= lo {
            0
        } else {
            ((lo - sv) as f64).sqrt().ceil() as i128
        };
        if tmin > tmax {
            continue;
        }
        let l = ell as i128;
        let r_l = (2 * lambda as i128) % l;
        let r2 = (v % 2) as i128;
        let r = if r_l % 2 == r2 { r_l } else { r_l + l };
        let span = (tmax - tmin) / (2 * l);
        if span < 1 {
            continue;
        }
        let j = rng.gen_range(0..=span);
        let mut t = tmin - tmin.rem_euclid(2 * l) + r + 2 * l * j;
        if t < tmin {
            t += 2 * l;
        }
        if t > tmax {
            continue;
        }
        let four_p = (t * t) as u128 + sv;
        if !four_p.is_multiple_of(4) || four_p < lo || four_p >= hi {
            continue;
        }
        let p = (four_p / 4) as u64;
        if !is_prime_u64(p) {
            continue;
        }
        let n = (p as i128 + 1 - t) as u128;
        if let Some(inst) = finish_instance(ell, lambda, p, t as i64, v, n, rng) {
            return inst;
        }
    }
}

fn finish_instance(
    ell: u64,
    lambda: u64,
    p: u64,
    t: i64,
    v: u64,
    n: u128,
    rng: &mut StdRng,
) -> Option<Inst> {
    let f = field(p, PolyMul::Karatsuba);
    let aw = red(&f, -35);
    let mut roots = vec![7u64];
    let disc = red(&f, -7);
    if let Some(sq) = f.sqrt(disc) {
        let inv2 = inv(&f, 2);
        roots.push(f.mul(f.sub(sq, 7), inv2));
        roots.push(f.mul(f.sub(f.neg(sq), 7), inv2));
    }
    for alpha in roots {
        let w = f.add(f.mul(3, f.mul(alpha, alpha)), aw);
        let Some(s) = f.sqrt(w) else { continue };
        if s == 0 {
            continue;
        }
        let a = f.mul(f.mul(3, alpha), inv(&f, s));
        let a24 = f.mul(f.add(a, 2), inv(&f, 4));
        let jcrater = j_of_a(&f, a);
        assert_eq!(jcrater, red(&f, -3375), "CM curve must have j = −3375");
        let nb = BigUint::from(n);
        let mut chi_n = 0;
        for _ in 0..8 {
            let x = rng.gen_range(2..p);
            let rhs = f.mul(x, f.add(f.mul(x, x), f.add(f.mul(a, x), 1)));
            let ch = f.legendre(rhs);
            if ch == 0 {
                continue;
            }
            let q = ladder_big(&f, &XZ { x, z: 1 }, &nb, &a24);
            if q.z == 0 {
                if chi_n != 0 && chi_n != ch {
                    return None;
                }
                chi_n = ch;
            } else if chi_n == ch {
                return None;
            }
        }
        if chi_n == 0 {
            return None;
        }
        return Some(Inst {
            ell,
            lambda,
            p,
            t,
            v,
            n,
            a,
            a24,
            chi_n,
            alpha,
            s,
            jcrater,
        });
    }
    None
}

fn order_check(f: &Fp, a_new: u64, x: u64, n: u128) -> bool {
    let a24 = f.mul(f.add(a_new, 2), inv(f, 4));
    ladder_big(f, &XZ { x, z: 1 }, &BigUint::from(n), &a24).z == 0
}

fn lib_field(p: u64) -> LField {
    LField::new(&BigUint::from(p)).expect("prime field")
}

/// Short-Weierstrass kernel polynomial from a Montgomery-x one:
/// `h_w(X) = Σ c_j s^{d−j} (X − α)^j`.
fn to_weierstrass_kernel(f: &Fp, hm: &[u64], alpha: u64, s: u64) -> Vec<u64> {
    let d = hm.len() - 1;
    let mut spow = vec![1u64; d + 1];
    for i in 1..=d {
        spow[i] = f.mul(spow[i - 1], s);
    }
    let mut h: Vec<u64> = Vec::new();
    for j in (0..=d).rev() {
        let mut nh = vec![0u64; h.len() + 1];
        for (i, &c) in h.iter().enumerate() {
            nh[i + 1] = f.add(nh[i + 1], c);
            nh[i] = f.sub(nh[i], f.mul(c, alpha));
        }
        nh[0] = f.add(nh[0], f.mul(hm[j], spow[d - j]));
        h = nh;
    }
    trim(h)
}

/// Certify `h_w` with the library's division-polynomial test and return
/// the library's codomain `j`.
fn lib_certify(p: u64, hw: &[u64], ell: u64) -> Result<u64, String> {
    let lf = lib_field(p);
    let e = Model {
        a: lf.from_i64(-35),
        b: lf.from_i64(-98),
    };
    let h: Vec<_> = hw.iter().map(|&c| lf.from_u64(c)).collect();
    let cod = kernel::verify_kernel(&lf, &e, ell, &h).map_err(|e| format!("{e:?}"))?;
    let j = cod.j(&lf).ok_or("singular codomain")?;
    Ok(lf.to_big(&j).to_u64().unwrap())
}

/// The round-4 Edwards codomain from `Π(x_s + 1)` and `Π(x_s − 1)` (up to
/// common factors removed by eighth powers).
fn edwards_codomain(f: &Fp, a: u64, ell: u64, prod_plus: u64, prod_minus: u64) -> u64 {
    let pow8 = |v: u64| {
        let v2 = f.mul(v, v);
        let v4 = f.mul(v2, v2);
        f.mul(v4, v4)
    };
    let ae = f.mul(f.pow(f.add(a, 2), ell as u128), pow8(prod_plus));
    let de = f.mul(f.pow(f.sub(a, 2), ell as u128), pow8(prod_minus));
    f.mul(f.mul(2, f.add(ae, de)), inv(f, f.sub(ae, de)))
}

// ---------------------------------------------------------------------------
// Mode: the NTT threshold (counted cost and contended wall time).

struct Fpk<'a> {
    f: &'a Fp,
    k: usize,
    m: usize,
    c: u64,
}

impl Fpk<'_> {
    fn reduce(&self, mut r: Vec<u64>) -> Vec<u64> {
        let k = self.k;
        let f = self.f;
        r.resize((2 * k).max(k), 0);
        for i in (k..r.len()).rev() {
            let q = r[i];
            if q == 0 {
                continue;
            }
            r[i] = 0;
            let j = i - k + self.m;
            r[j] = f.sub(r[j], q);
            let t = f.mul(q, self.c);
            r[i - k] = f.sub(r[i - k], t);
        }
        r.truncate(k);
        r
    }
    fn modulus(&self) -> Vec<u64> {
        let mut v = vec![0u64; self.k + 1];
        v[0] = self.c;
        v[self.m] = self.f.add(v[self.m], 1);
        v[self.k] = 1;
        v
    }
    fn x(&self) -> Vec<u64> {
        let mut v = vec![0u64; self.k];
        if self.k > 1 {
            v[1] = 1;
        } else {
            v[0] = self.f.neg(self.c);
        }
        v
    }
    fn pow_big(&self, a: &[u64], e: &BigUint) -> Vec<u64> {
        let mut r = FieldArith::one(self);
        let a = a.to_vec();
        for i in (0..e.bits()).rev() {
            r = FieldArith::mul(self, &r, &r);
            if e.bit(i) {
                r = FieldArith::mul(self, &r, &a);
            }
        }
        r
    }
    fn inv(&self, a: &[u64]) -> Vec<u64> {
        let mut g = poly_xgcd_inv(self.f, &trim(a.to_vec()), &self.modulus())
            .expect("non-invertible extension element");
        g.resize(self.k, 0);
        g
    }
}

impl FieldArith for Fpk<'_> {
    type Elem = Vec<u64>;
    fn mul(&self, a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
        // Same reference ⇒ the NTT squares (two transforms per prime).
        let prod = if std::ptr::eq(a, b) {
            poly::mul(self.f, a, a)
        } else {
            poly::mul(self.f, a, b)
        };
        self.reduce(prod)
    }
    fn add(&self, a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
        a.iter().zip(b).map(|(x, y)| self.f.add(*x, *y)).collect()
    }
    fn sub(&self, a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
        a.iter().zip(b).map(|(x, y)| self.f.sub(*x, *y)).collect()
    }
    fn zero(&self) -> Vec<u64> {
        vec![0u64; self.k]
    }
    fn one(&self) -> Vec<u64> {
        let mut v = vec![0u64; self.k];
        v[0] = 1;
        v
    }
    fn is_zero(&self, a: &Vec<u64>) -> bool {
        a.iter().all(|v| *v == 0)
    }
}

fn psub(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    poly::sub(f, a, b)
}

fn poly_rem(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let b = trim(b.to_vec());
    let db = b.len() - 1;
    let lc_inv = inv(f, b[db]);
    let mut r = trim(a.to_vec());
    while r.len() > db {
        let i = r.len() - 1;
        let q = f.mul(r[i], lc_inv);
        for j in 0..=db {
            let t = f.mul(q, b[j]);
            r[i - db + j] = f.sub(r[i - db + j], t);
        }
        r = trim(r);
    }
    r
}

fn poly_gcd(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut a = trim(a.to_vec());
    let mut b = trim(b.to_vec());
    while !b.is_empty() {
        let r = poly_rem(f, &a, &b);
        a = b;
        b = r;
    }
    a
}

fn poly_xgcd_inv(f: &Fp, a: &[u64], m: &[u64]) -> Option<Vec<u64>> {
    let (mut r0, mut r1) = (trim(m.to_vec()), trim(a.to_vec()));
    let (mut s0, mut s1): (Vec<u64>, Vec<u64>) = (Vec::new(), vec![1]);
    while !r1.is_empty() {
        let d1 = r1.len() - 1;
        let lc_inv = inv(f, r1[d1]);
        let mut r = r0.clone();
        let mut q = vec![0u64; r0.len().saturating_sub(d1).max(1)];
        while r.len() > d1 {
            let i = r.len() - 1;
            let c = f.mul(r[i], lc_inv);
            q[i - d1] = c;
            for j in 0..=d1 {
                let t = f.mul(c, r1[j]);
                r[i - d1 + j] = f.sub(r[i - d1 + j], t);
            }
            r = trim(r);
        }
        let ns = psub(f, &s0, &trim(poly::mul(f, &trim(q), &s1)));
        (r0, r1) = (r1, r);
        (s0, s1) = (s1, trim(ns));
    }
    if r0.len() != 1 {
        return None;
    }
    let c = inv(f, r0[0]);
    Some(s0.iter().map(|v| f.mul(*v, c)).collect())
}

fn find_irreducible<'a>(f: &'a Fp, k: usize, rng: &mut StdRng) -> (Fpk<'a>, u64) {
    let mut cands = 0;
    loop {
        cands += 1;
        let m = if k > 1 { rng.gen_range(1..k) } else { 0 };
        let c = rng.gen_range(1..f.p());
        let fk = Fpk { f, k, m, c };
        if k == 1 {
            return (fk, cands);
        }
        let modp = fk.modulus();
        let x = fk.x();
        let pb = BigUint::from(f.p());
        let mut xq = x.clone();
        let mut ok = true;
        for _ in 1..=k / 2 {
            xq = fk.pow_big(&xq, &pb);
            let g = poly_gcd(f, &modp, &trim(psub(f, &xq, &x)));
            if g.len() > 1 {
                ok = false;
                break;
            }
        }
        if ok {
            return (fk, cands);
        }
    }
}

fn berlekamp_massey(f: &Fp, s: &[u64]) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut bb = 1u64;
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l {
            d = f.add(d, f.mul(*c.get(i).unwrap_or(&0), s[n - i]));
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = f.mul(d, inv(f, bb));
        let mut nc = c.clone();
        nc.resize(nc.len().max(b.len() + m), 0);
        for (i, &bi) in b.iter().enumerate() {
            nc[i + m] = f.sub(nc[i + m], f.mul(coef, bi));
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = c;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
        c = nc;
    }
    c.resize(l + 1, 0);
    c
}

fn orders_over_ext(inst: &Inst, k: usize) -> (BigUint, BigUint) {
    let pb = BigInt::from(inst.p);
    let (mut t0, mut t1) = (BigInt::from(2), BigInt::from(inst.t));
    for _ in 1..k {
        let t2 = BigInt::from(inst.t) * &t1 - &pb * &t0;
        t0 = t1;
        t1 = t2;
    }
    let pk = pb.pow(k as u32);
    let one = BigInt::from(1);
    let n_e = (&pk + &one - &t1).abs().to_biguint().unwrap();
    let n_t = (&pk + &one + &t1).abs().to_biguint().unwrap();
    (n_e, n_t)
}

fn generic_ells(min_ell: u64, max_ell: u64, step_log2: f64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    let mut e = 5.0f64;
    while 2f64.powf(e) <= max_ell as f64 * 1.0001 {
        let l = next_prime(2f64.powf(e).round() as u64);
        if l >= min_ell && l <= max_ell && l != 7 && !v.contains(&l) {
            v.push(l);
        }
        e += step_log2;
    }
    v
}
fn t975(df: f64) -> f64 {
    let z: f64 = 1.959964;
    z + (z.powi(3) + z) / (4.0 * df)
        + (5.0 * z.powi(5) + 16.0 * z.powi(3) + 3.0 * z) / (96.0 * df * df)
        + (3.0 * z.powi(7) + 19.0 * z.powi(5) + 17.0 * z.powi(3) - 15.0 * z) / (384.0 * df.powi(3))
}

#[derive(Clone, Copy, Debug)]
struct Fit {
    slope: f64,
    slope_lo: f64,
    slope_hi: f64,
    icpt: f64,
    n: usize,
    r2: f64,
}

impl Fit {
    fn eval_log2(&self, log2_l: f64) -> f64 {
        self.icpt + self.slope * log2_l
    }
}

/// OLS of `y` on `x` (already transformed).
fn ols(xs: &[f64], ys: &[f64]) -> Fit {
    let n = xs.len();
    assert!(n >= 3, "need at least three points to fit");
    let mx = xs.iter().sum::<f64>() / n as f64;
    let my = ys.iter().sum::<f64>() / n as f64;
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let slope = sxy / sxx;
    let icpt = my - slope * mx;
    let sse: f64 = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (y - icpt - slope * x).powi(2))
        .sum();
    let syy: f64 = ys.iter().map(|y| (y - my).powi(2)).sum();
    let se = (sse / (n as f64 - 2.0) / sxx).sqrt();
    let t = t975(n as f64 - 2.0);
    Fit {
        slope,
        slope_lo: slope - t * se,
        slope_hi: slope + t * se,
        icpt,
        n,
        r2: if syy > 0.0 { 1.0 - sse / syy } else { 1.0 },
    }
}

fn fit_loglog(pts: &[(f64, f64)]) -> Fit {
    let xs: Vec<f64> = pts.iter().map(|p| p.0.log2()).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1.log2()).collect();
    ols(&xs, &ys)
}

fn crossover(cost: impl Fn(f64) -> f64, target: f64, lo: f64, hi: f64) -> Option<f64> {
    if cost(hi) < target {
        return None;
    }
    if cost(lo) >= target {
        return Some(lo);
    }
    let (mut a, mut b) = (lo, hi);
    for _ in 0..200 {
        let m = 0.5 * (a + b);
        if cost(m) >= target {
            b = m;
        } else {
            a = m;
        }
    }
    Some(b)
}

fn sel(v: &[(f64, f64)], lo: f64) -> Vec<(f64, f64)> {
    v.iter().filter(|p| p.0 >= lo).copied().collect()
}

fn load(path: &str) -> Value {
    let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    serde_json::from_str(&s).expect("parse json")
}

fn rows_of(dir: &str, names: &[&str]) -> Vec<Value> {
    let mut out = Vec::new();
    for n in names {
        let path = format!("{dir}/{n}");
        if std::path::Path::new(&path).exists() {
            out.extend(load(&path)["rows"].as_array().unwrap().iter().cloned());
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Shared helpers for this lead.

/// The library certificate for a kernel polynomial on `y² = x³ + a4 x + a6`;
/// returns the library's codomain `j`.
fn lib_certify_model(p: u64, a4: u64, a6: u64, hw: &[u64], ell: u64) -> Result<u64, String> {
    let lf = lib_field(p);
    let e = Model {
        a: lf.from_u64(a4),
        b: lf.from_u64(a6),
    };
    let h: Vec<_> = hw.iter().map(|&c| lf.from_u64(c)).collect();
    let cod = kernel::verify_kernel(&lf, &e, ell, &h).map_err(|e| format!("{e:?}"))?;
    let j = cod.j(&lf).ok_or("singular codomain")?;
    Ok(lf.to_big(&j).to_u64().unwrap())
}

/// Short Weierstrass `(a4, a6)` of the Montgomery curve `y² = x³ + A x² + x`
/// under `X = x + A/3`.
fn weierstrass_of_montgomery(f: &Fp, a: u64) -> (u64, u64) {
    let i3 = inv(f, 3);
    let i27 = inv(f, 27);
    let a2 = f.mul(a, a);
    let a4 = f.sub(1, f.mul(a2, i3));
    let a6 = f.sub(f.mul(f.mul(2, f.mul(a2, a)), i27), f.mul(a, i3));
    (a4, a6)
}

/// `h(x + c)` (Taylor shift, schoolbook, counted).
fn shift_poly(f: &Fp, h: &[u64], c: u64) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    for &coef in h.iter().rev() {
        let mut nh = vec![0u64; out.len() + 1];
        for (i, &o) in out.iter().enumerate() {
            nh[i + 1] = f.add(nh[i + 1], o);
            nh[i] = f.add(nh[i], f.mul(o, c));
        }
        nh[0] = f.add(nh[0], coef);
        out = nh;
    }
    trim(out)
}

fn horner(f: &Fp, h: &[u64], x: u64) -> u64 {
    h.iter().rev().fold(0u64, |acc, c| f.add(f.mul(acc, x), *c))
}

/// Codomain Montgomery `A` and the `x`-images of `pts` for the kernel
/// polynomial `h` (Montgomery `x`, monic) on the curve with coefficient `a`.
fn apply_kernel(f: &Fp, a: u64, ell: u64, h: &[u64], pts: &[u64]) -> (u64, Vec<u64>) {
    let h1 = horner(f, h, 1);
    let hm1 = horner(f, h, f.neg(1));
    let a_new = edwards_codomain(f, a, ell, hm1, h1);
    let rev: Vec<u64> = h.iter().rev().copied().collect();
    let images = pts
        .iter()
        .map(|&x| {
            let den = horner(f, h, x);
            let num = horner(f, &rev, x);
            f.mul(f.mul(x, f.mul(num, num)), inv(f, f.mul(den, den)))
        })
        .collect();
    (a_new, images)
}

fn monic(f: &Fp, a: &[u64]) -> Vec<u64> {
    let a = trim(a.to_vec());
    let c = inv(f, *a.last().unwrap());
    a.iter().map(|v| f.mul(*v, c)).collect()
}

/// Exact quotient `a / b` (schoolbook long division, counted).
fn poly_divexact(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let b = monic(f, b);
    let db = b.len() - 1;
    let mut r = trim(a.to_vec());
    let lc = *r.last().unwrap();
    if lc != 1 {
        let c = inv(f, lc);
        r = r.iter().map(|v| f.mul(*v, c)).collect();
    }
    let mut q = vec![0u64; r.len() - db];
    for i in (db..r.len()).rev() {
        let c = r[i];
        q[i - db] = c;
        if c == 0 {
            continue;
        }
        for j in 0..db {
            let t = f.mul(c, b[j]);
            r[i - db + j] = f.sub(r[i - db + j], t);
        }
        r[i] = 0;
    }
    assert!(trim(r[..db].to_vec()).is_empty(), "division not exact");
    trim(q)
}

fn mulmod(f: &Fp, md: &mut MonicDivisor, a: &[u64], b: &[u64]) -> Vec<u64> {
    let prod = if std::ptr::eq(a, b) {
        poly::mul(f, a, a)
    } else {
        poly::mul(f, a, b)
    };
    md.rem(f, &prod)
}

/// `x·a mod m` for monic `m` (shift and one reduction step).
fn mulx_mod(f: &Fp, m: &[u64], a: &[u64]) -> Vec<u64> {
    let d = m.len() - 1;
    let mut r = vec![0u64; d + 1];
    r[1..=a.len()].copy_from_slice(a);
    let top = r[d];
    if top != 0 {
        for j in 0..d {
            let t = f.mul(top, m[j]);
            r[j] = f.sub(r[j], t);
        }
    }
    r.truncate(d);
    trim(r)
}

/// `base^e mod m`, 4-bit fixed window.
fn powmod(f: &Fp, md: &mut MonicDivisor, base: &[u64], e: &BigUint) -> Vec<u64> {
    let mut tbl = vec![vec![1u64], base.to_vec()];
    for i in 2..16 {
        let n = mulmod(f, md, &tbl[i - 1], base);
        tbl.push(n);
    }
    let bits = e.bits();
    let nwin = bits.div_ceil(4);
    let mut r = vec![1u64];
    for w in (0..nwin).rev() {
        if w + 1 != nwin {
            for _ in 0..4 {
                let rr = r.clone();
                r = mulmod(f, md, &rr, &rr);
            }
        }
        let mut d = 0usize;
        for b in 0..4 {
            if e.bit(w * 4 + b) {
                d |= 1 << b;
            }
        }
        if d != 0 {
            r = if r == [1] {
                tbl[d].clone()
            } else {
                mulmod(f, md, &r, &tbl[d])
            };
        }
    }
    r
}

/// `x^e mod m` by square-and-multiply with multiplications by `x` as shifts.
fn powx_mod(f: &Fp, m: &[u64], e: &BigUint) -> Vec<u64> {
    let mut md = MonicDivisor::new(m.to_vec()).expect("monic");
    let mut r = vec![1u64];
    for i in (0..e.bits()).rev() {
        let rr = r.clone();
        r = mulmod(f, &mut md, &rr, &rr);
        if e.bit(i) {
            r = mulx_mod(f, m, &r);
        }
    }
    r
}

fn rand_poly(rng: &mut StdRng, p: u64, d: usize) -> Vec<u64> {
    trim((0..d).map(|_| rng.gen_range(0..p)).collect())
}

type GenericInstance = (StdRng, Inst, BigUint, BigUint, u32, u32, u64);

/// One generic instance as R5.3 (seed `0x1630 ^ ℓ`, `v_ℓ = 0` on `E` and
/// `2` on the twist over `F_{p^k}`).
fn generic_instance(ell: u64) -> GenericInstance {
    let mut rng = StdRng::seed_from_u64(0x1630 ^ ell);
    let lambda = primitive_root(ell);
    let k = ((ell - 1) / 2) as usize;
    let mut rejected = 0u64;
    loop {
        let inst = make_instance(ell, lambda, &mut rng);
        let (n_e, n_t) = orders_over_ext(&inst, k);
        let (ve, _) = val(&n_e, ell);
        let (vt, _) = val(&n_t, ell);
        if ve == 0 && vt == 2 {
            return (rng, inst, n_e, n_t, ve, vt, rejected);
        }
        rejected += 1;
    }
}

/// `(−7/ℓ)` by Euler's criterion.
fn chi_minus7(ell: u64) -> i64 {
    let m = (ell - 7 % ell) % ell;
    let mut r = 1u128;
    let mut b = m as u128;
    let mut e = (ell - 1) / 2;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % ell as u128;
        }
        b = b * b % ell as u128;
        e >>= 1;
    }
    if r == 1 {
        1
    } else {
        -1
    }
}

// ---------------------------------------------------------------------------
// Arm (a): the F_{p^k} ladder, descending and ascending.

struct LadderFind {
    ladder: CompleteTally,
    minpoly: CompleteTally,
    find: CompleteTally,
    attempts_total: u64,
    attempts_final: u64,
    retries: u64,
    minpoly_h: Vec<u64>,
    a_new: u64,
    wall: f64,
}

/// The R5.3 ladder find on the Montgomery curve `a` (kernel generated by
/// an `ℓ`-torsion point of the twist over `F_{p^k}`).  `descending`: a
/// codomain with `j = j_crater` is a horizontal draw and is retried (as
/// R5.3).  Ascending: the codomain is returned as found.
#[allow(clippy::too_many_arguments)]
fn ladder_find(
    f: &Fp,
    fk: &Fpk,
    a: u64,
    a24: u64,
    ell: u64,
    m: &BigUint,
    e: u32,
    jcrater: u64,
    descending: bool,
    rng: &mut StdRng,
) -> LadderFind {
    let k = fk.k;
    let p = f.p();
    let mut a24k = fk.zero();
    a24k[0] = a24;
    let lb = BigUint::from(ell);
    let wall = Instant::now();
    let mut retries = 0u64;
    let mut attempts = 0u64;
    loop {
        let c_find = f.complete_tally();
        let mut att_final = 0u64;
        let q = 'outer: loop {
            attempts += 1;
            att_final += 1;
            let x: Vec<u64> = (0..k).map(|_| rng.gen_range(0..p)).collect();
            let mut q = ladder_big(fk, &XZ { x, z: fk.one() }, m, &a24k);
            if fk.is_zero(&q.z) {
                continue;
            }
            for _ in 0..e {
                let r = ladder_big(fk, &q, &lb, &a24k);
                if fk.is_zero(&r.z) {
                    break 'outer q;
                }
                q = r;
            }
        };
        let ladder_cnt = f.complete_tally().since(c_find);
        let c_mp = f.complete_tally();
        let x1 = FieldArith::mul(fk, &q.x, &fk.inv(&q.z));
        let proj: Vec<u64> = (0..k).map(|_| rng.gen_range(0..p)).collect();
        let mut pw = fk.one();
        let mut seq = Vec::with_capacity(2 * k);
        for _ in 0..2 * k {
            let s = pw
                .iter()
                .zip(&proj)
                .fold(0u64, |acc, (a, b)| f.add(acc, f.mul(*a, *b)));
            seq.push(s);
            pw = FieldArith::mul(fk, &pw, &x1);
        }
        let conn = berlekamp_massey(f, &seq);
        let minpoly: Vec<u64> = conn.iter().rev().copied().collect();
        let minpoly_cnt = f.complete_tally().since(c_mp);
        let find_cnt = f.complete_tally().since(c_find);
        assert_eq!(minpoly.len() - 1, k, "minimal polynomial degree ≠ (ℓ−1)/2");
        let mut acc = fk.zero();
        for &c in minpoly.iter().rev() {
            acc = FieldArith::mul(fk, &acc, &x1);
            acc[0] = f.add(acc[0], c);
        }
        assert!(fk.is_zero(&acc), "minpoly does not vanish at x1");
        let h1 = horner(f, &minpoly, 1);
        let hm1 = horner(f, &minpoly, f.neg(1));
        let a_new = edwards_codomain(f, a, ell, hm1, h1);
        if descending && j_of_a(f, a_new) == jcrater {
            retries += 1;
            continue;
        }
        return LadderFind {
            ladder: ladder_cnt,
            minpoly: minpoly_cnt,
            find: find_cnt,
            attempts_total: attempts,
            attempts_final: att_final,
            retries,
            minpoly_h: minpoly,
            a_new,
            wall: wall.elapsed().as_secs_f64(),
        };
    }
}

/// `{"0": x, "0.1": y, ...}` for a cost function of the weight.
fn by_a(cost: impl Fn(f64) -> f64) -> Value {
    let mut m = serde_json::Map::new();
    for a in WEIGHTS {
        m.insert(format!("{a}"), json!(cost(a)));
    }
    Value::Object(m)
}

fn find_json(lf: &LadderFind) -> Value {
    let per_attempt = meq_of(&lf.ladder) as f64 / lf.attempts_final as f64;
    let att = lf.attempts_final as f64;
    json!({
        "ladder": tjs(&lf.ladder), "minpoly": tjs(&lf.minpoly), "find": tjs(&lf.find),
        "attempts_total": lf.attempts_total, "attempts_final_find": lf.attempts_final,
        "horizontal_retries": lf.retries, "ladder_per_attempt_Meq": per_attempt,
        "find_expected_Meq": 2.0 * per_attempt + meq_of(&lf.minpoly) as f64,
        "find_expected_by_a": by_a(|a| 2.0 * cost_a(&lf.ladder, a) / att + cost_a(&lf.minpoly, a)),
        "find_wall_s_contended": lf.wall,
    })
}

#[allow(clippy::too_many_arguments)]
fn mode_ladder(ells: Vec<u64>, cap_s: f64, arm: PolyMul, reps: usize, reps_max_ell: u64) -> Value {
    let mut rows = Vec::new();
    let t_start = Instant::now();
    for ell in ells {
        if t_start.elapsed().as_secs_f64() > cap_s {
            eprintln!("ladder: time cap reached before ℓ={ell}");
            break;
        }
        let (mut rng, inst, n_e, n_t, ve, vt, rejected) = generic_instance(ell);
        let k = ((ell - 1) / 2) as usize;
        let f = field(inst.p, arm);
        let c_setup = f.complete_tally();
        let (fk, cands) = find_irreducible(&f, k, &mut rng);
        let setup = f.complete_tally().since(c_setup);
        let (ntarget, side) = if vt >= 2 { (n_t, "twist") } else { (n_e, "E") };
        let (e, m) = val(&ntarget, ell);
        assert!(e >= 2, "ℓ² must divide the order over F_(p^k)");
        let mut pts = Vec::new();
        while pts.len() < 2 {
            let x = rng.gen_range(2..inst.p);
            let rhs = f.mul(x, f.add(f.mul(x, x), f.add(f.mul(inst.a, x), 1)));
            if f.legendre(rhs) == inst.chi_n {
                pts.push(x);
            }
        }
        // Descending (crater → floor), R5.3's RNG sequence.
        let desc = ladder_find(
            &f,
            &fk,
            inst.a,
            inst.a24,
            ell,
            &m,
            e,
            inst.jcrater,
            true,
            &mut rng,
        );
        let (a_floor, img1) = apply_kernel(&f, inst.a, ell, &desc.minpoly_h, &pts);
        assert_eq!(a_floor, desc.a_new);
        // Cost of evaluating the isogeny at one more point from h (the
        // marginal cost of a point: apply_kernel with one point minus none).
        let eval_pt = {
            let c0 = f.complete_tally();
            let _ = apply_kernel(&f, inst.a, ell, &desc.minpoly_h, &[]);
            let none = f.complete_tally().since(c0);
            let c1 = f.complete_tally();
            let _ = apply_kernel(&f, inst.a, ell, &desc.minpoly_h, &pts[..1]);
            let one = f.complete_tally().since(c1);
            tjs(&CompleteTally {
                ops: crypto_lib::cryptanalysis::isogeny_eval::OpTally {
                    mul: one.ops.mul - none.ops.mul,
                    inv: one.ops.inv - none.ops.inv,
                    word: one.ops.word - none.ops.word,
                },
                adds: crypto_lib::cryptanalysis::isogeny_eval::AddTally {
                    add: one.adds.add - none.adds.add,
                    word_add: one.adds.word_add - none.adds.word_add,
                },
            })
        };
        let j_floor = j_of_a(&f, a_floor);
        for &x in &img1 {
            assert!(order_check(&f, a_floor, x, inst.n), "image has wrong order");
        }
        let mut checks = vec!["minpoly_vanishes", "desc_images_order_N"];
        // Ascending (floor → crater).
        let a24f = f.mul(f.add(a_floor, 2), inv(&f, 4));
        let asc = ladder_find(
            &f,
            &fk,
            a_floor,
            a24f,
            ell,
            &m,
            e,
            inst.jcrater,
            false,
            &mut rng,
        );
        let (a_up, img2) = apply_kernel(&f, a_floor, ell, &asc.minpoly_h, &img1);
        let j_up = j_of_a(&f, a_up);
        let asc_ok = j_up == inst.jcrater;
        if asc_ok {
            for &x in &img2 {
                assert!(
                    order_check(&f, a_up, x, inst.n),
                    "asc image has wrong order"
                );
            }
            checks.push("asc_reaches_crater_images_order_N");
        }
        if ell <= 211 {
            let hw = to_weierstrass_kernel(&f, &desc.minpoly_h, inst.alpha, inst.s);
            let lj = lib_certify(inst.p, &hw, ell).expect("library rejects desc kernel");
            assert_eq!(lj, j_floor, "library j ≠ Montgomery j (desc)");
            let (a4, a6) = weierstrass_of_montgomery(&f, a_floor);
            let c3 = f.mul(a_floor, inv(&f, 3));
            let hw2 = shift_poly(&f, &asc.minpoly_h, f.neg(c3));
            let lj2 =
                lib_certify_model(inst.p, a4, a6, &hw2, ell).expect("library rejects asc kernel");
            assert_eq!(lj2, j_up, "library j ≠ Montgomery j (asc)");
            checks.push("lib_verify_kernel_and_j_both");
        }
        // Repeated realised finds for the ascending/descending comparison.
        let mut rep_desc = Vec::new();
        let mut rep_asc = Vec::new();
        if ell <= reps_max_ell {
            for _ in 0..reps {
                let d = ladder_find(
                    &f,
                    &fk,
                    inst.a,
                    inst.a24,
                    ell,
                    &m,
                    e,
                    inst.jcrater,
                    true,
                    &mut rng,
                );
                rep_desc.push(meq_of(&d.find));
                let u = ladder_find(
                    &f,
                    &fk,
                    a_floor,
                    a24f,
                    ell,
                    &m,
                    e,
                    inst.jcrater,
                    false,
                    &mut rng,
                );
                if j_of_a(&f, u.a_new) == inst.jcrater {
                    rep_asc.push(meq_of(&u.find));
                }
            }
        }
        let dj = find_json(&desc);
        let aj = find_json(&asc);
        eprintln!(
            "ladder[{arm:?}] ℓ={ell} k={k} desc exp={:.3e} asc exp={:.3e} asc_ok={asc_ok} wall {:.1}+{:.1}s",
            dj["find_expected_Meq"].as_f64().unwrap(),
            aj["find_expected_Meq"].as_f64().unwrap(),
            desc.wall,
            asc.wall
        );
        rows.push(json!({
            "ell": ell, "k": k, "p": inst.p, "t": inst.t, "v": inst.v, "chi_m7": chi_minus7(ell),
            "side": side, "v_ell_E": ve, "v_ell_twist": vt, "instances_rejected": rejected,
            "irreducible_candidates": cands, "ext_modulus": [k, fk.m, fk.c],
            "setup": tjs(&setup), "cofactor_bits": m.bits(), "eval_pt": eval_pt,
            "desc": dj, "asc": aj, "crater_j": inst.jcrater, "floor_j": j_floor,
            "floor_A": a_floor, "asc_codomain_j": j_up, "asc_reaches_crater": asc_ok,
            "rep_desc_find_Meq": rep_desc, "rep_asc_find_Meq": rep_asc, "checks": checks,
        }));
    }
    json!({"mode": "ladder", "arm": format!("{arm:?}"), "rows": rows})
}

// ---------------------------------------------------------------------------
// Arm (a′): measured per-step ladder cost, projected to the exact step count.

fn mode_project(min_log2: f64, max_log2: f64, step_log2: f64, arm_names: &[String]) -> Value {
    let mut rows = Vec::new();
    let mut e = min_log2;
    let mut seen = Vec::new();
    while e <= max_log2 + 1e-9 {
        let ell = next_prime(2f64.powf(e).round() as u64);
        e += step_log2;
        if ell == 7 || seen.contains(&ell) {
            continue;
        }
        seen.push(ell);
        let k = ((ell - 1) / 2) as usize;
        // Up to ℓ = 4096 the R5.3 instance (same p as the measured rows);
        // above, the first λ-instance of the same seed without the v_ℓ
        // filter (exact cofactor bit length still from the twist order).
        let (inst, cof_bits, filtered) = if ell <= 4096 {
            let (_, inst, _, n_t, _, _, _) = generic_instance(ell);
            let (_, m) = val(&n_t, ell);
            (inst, m.bits(), true)
        } else {
            let mut rng = StdRng::seed_from_u64(0x1630 ^ ell);
            let inst = make_instance(ell, primitive_root(ell), &mut rng);
            let (_, n_t) = orders_over_ext(&inst, k);
            let bits = n_t.bits() - 2 * (ell as f64).log2().ceil() as u64;
            (inst, bits, false)
        };
        let ell_bits = 64 - ell.leading_zeros() as u64;
        let mut arms = serde_json::Map::new();
        for (name, arm) in [
            ("karatsuba", PolyMul::Karatsuba),
            ("auto", PolyMul::Auto),
            ("ntt", PolyMul::Ntt),
        ] {
            if !arm_names.iter().any(|n| n == name) {
                continue;
            }
            let f = field(inst.p, arm);
            let mut rng = StdRng::seed_from_u64(0x6400 ^ ell);
            let fk = Fpk {
                f: &f,
                k,
                m: if k > 1 { rng.gen_range(1..k) } else { 0 },
                c: rng.gen_range(1..inst.p),
            };
            let mut a24k = fk.zero();
            a24k[0] = inst.a24;
            let rp = |rng: &mut StdRng| -> Vec<u64> {
                (0..k).map(|_| rng.gen_range(0..inst.p)).collect()
            };
            let pp = XZ {
                x: rp(&mut rng),
                z: rp(&mut rng),
            };
            let qq = XZ {
                x: rp(&mut rng),
                z: rp(&mut rng),
            };
            let dd = XZ {
                x: rp(&mut rng),
                z: rp(&mut rng),
            };
            let c0 = f.complete_tally();
            let _ = xdbl(&fk, &pp, &a24k);
            let dbl = f.complete_tally().since(c0);
            let c0 = f.complete_tally();
            let _ = xadd(&fk, &pp, &qq, &dd);
            let add = f.complete_tally().since(c0);
            let step = meq_of(&dbl) + meq_of(&add);
            let per_attempt = (cof_bits + ell_bits) as f64 * step as f64;
            let c0 = f.complete_tally();
            let _ = FieldArith::mul(&fk, &pp.x, &qq.x);
            let ext_mul = f.complete_tally().since(c0);
            let seq: Vec<u64> = (0..2 * k).map(|_| rng.gen_range(0..inst.p)).collect();
            let c0 = f.complete_tally();
            let _ = berlekamp_massey(&f, &seq);
            let bm = f.complete_tally().since(c0);
            let c0 = f.complete_tally();
            let _ = fk.inv(&pp.z);
            let invk = f.complete_tally().since(c0);
            // R6.4's decomposition: minpoly = ext. inversion + 2k extension
            // products + 2k² inner-product multiplications + BM.  With
            // additions: the inner products are fused multiply-adds
            // (`fold(add(acc, mul(a, b)))`), 2k² additions as well.
            let bits = (cof_bits + ell_bits) as f64;
            let kf = k as f64;
            let cost = |a: f64| -> (f64, f64, f64) {
                let st = cost_a(&dbl, a) + cost_a(&add, a);
                let pa = bits * st;
                let mp = cost_a(&invk, a)
                    + 2.0 * kf * cost_a(&ext_mul, a)
                    + 2.0 * kf * kf * (1.0 + a)
                    + cost_a(&bm, a);
                (st, pa, mp)
            };
            let (_, _, minpoly_meq) = cost(0.0);
            arms.insert(
                name.into(),
                json!({"xdbl": tjs(&dbl), "xadd": tjs(&add), "step_Meq": step,
                       "ext_mul": tjs(&ext_mul), "ext_inv": tjs(&invk), "bm": tjs(&bm),
                       "ext_mul_Meq": meq_of(&ext_mul), "per_attempt_Meq": per_attempt,
                       "minpoly_Meq": minpoly_meq, "bm_Meq": meq_of(&bm) as f64,
                       "ext_inv_Meq": meq_of(&invk) as f64,
                       "find_expected_Meq": 2.0 * per_attempt + minpoly_meq,
                       "step_by_a": by_a(|a| cost(a).0),
                       "minpoly_by_a": by_a(|a| cost(a).2),
                       "find_expected_by_a": by_a(|a| 2.0 * cost(a).1 + cost(a).2)}),
            );
        }
        let g = |n: &str| -> f64 {
            arms.get(n)
                .map(|v| v["find_expected_Meq"].as_f64().unwrap())
                .unwrap_or(f64::NAN)
        };
        eprintln!(
            "project ℓ={ell} k={k} cof_bits={cof_bits} auto exp={:.3e} ntt exp={:.3e} kara exp={:.3e}",
            g("auto"),
            g("ntt"),
            g("karatsuba")
        );
        rows.push(
            json!({"ell": ell, "k": k, "p": inst.p, "cofactor_bits": cof_bits,
                         "instance_filtered": filtered, "arms": arms}),
        );
    }
    json!({"mode": "project", "rows": rows,
           "derivation": "per attempt = (cofactor bits + bits(ell)) x (counted xDBL + xADD in F_(p^k)); minpoly = counted F_(p^k) inversion + 2k counted products + 2k^2 inner-product mults (and 2k^2 additions) + counted BM; expected find = 2 x per attempt + minpoly (R5.3's definition); *_by_a prices every addition at a M"})
}

// ---------------------------------------------------------------------------
// Arm (b): ψ_ℓ and Cantor–Zassenhaus.

/// `f_n` (`ψ_n` for odd `n`, `ψ_n / 2y` for even `n`) on
/// `y² = x³ + a4 x + a6`, by the doubling recurrences, memoised.
fn divpoly(f: &Fp, a4: u64, a6: u64, n: usize, memo: &mut HashMap<usize, Vec<u64>>) -> Vec<u64> {
    if let Some(v) = memo.get(&n) {
        return v.clone();
    }
    let r = match n {
        0 => vec![],
        1 | 2 => vec![1],
        3 => trim(vec![
            f.neg(f.mul(a4, a4)),
            f.mul(12, a6),
            f.mul(6, a4),
            0,
            3,
        ]),
        4 => {
            let a2 = f.mul(a4, a4);
            trim(vec![
                f.neg(f.add(f.mul(16, f.mul(a6, a6)), f.mul(2, f.mul(a2, a4)))),
                f.neg(f.mul(8, f.mul(a4, a6))),
                f.neg(f.mul(10, a2)),
                f.mul(40, a6),
                f.mul(10, a4),
                0,
                2,
            ])
        }
        _ => {
            let m = n / 2;
            let mut g = |i: usize| divpoly(f, a4, a6, i, memo);
            if n % 2 == 1 {
                let (fm2, fm, fm1, fp1) = (g(m + 2), g(m), g(m - 1), g(m + 1));
                let fm3 = poly::mul(f, &poly::mul(f, &fm, &fm), &fm);
                let fp13 = poly::mul(f, &poly::mul(f, &fp1, &fp1), &fp1);
                let t1 = poly::mul(f, &fm2, &fm3);
                let t2 = poly::mul(f, &fm1, &fp13);
                let big_f = vec![f.mul(4, a6), f.mul(4, a4), 0, 4];
                let f2 = poly::mul(f, &big_f, &big_f);
                if m.is_multiple_of(2) {
                    trim(poly::sub(f, &poly::mul(f, &f2, &t1), &t2))
                } else {
                    trim(poly::sub(f, &t1, &poly::mul(f, &f2, &t2)))
                }
            } else {
                let (fm, fp2, fm1, fm2, fp1) = (g(m), g(m + 2), g(m - 1), g(m - 2), g(m + 1));
                let a = poly::mul(f, &fp2, &poly::mul(f, &fm1, &fm1));
                let b = poly::mul(f, &fm2, &poly::mul(f, &fp1, &fp1));
                trim(poly::mul(f, &fm, &poly::sub(f, &a, &b)))
            }
        }
    };
    memo.insert(n, r.clone());
    r
}

fn psi_monic(f: &Fp, a4: u64, a6: u64, ell: u64) -> Vec<u64> {
    let mut memo = HashMap::new();
    let fl = divpoly(f, a4, a6, ell as usize, &mut memo);
    assert_eq!(fl.len() as u64 - 1, (ell * ell - 1) / 2, "deg ψ_ℓ");
    assert_eq!(*fl.last().unwrap(), ell % f.p(), "lc ψ_ℓ = ℓ");
    monic(f, &fl)
}

/// One irreducible factor of degree `k` of `g` (all of whose irreducible
/// factors have degree `k`), by Cantor–Zassenhaus splitting, keeping the
/// smaller part each time.  Returns the factor and the number of splits
/// attempted.
fn cz_one_factor(f: &Fp, g: &[u64], k: usize, rng: &mut StdRng) -> (Vec<u64>, u64) {
    let p = f.p();
    let e: BigUint = (BigUint::from(p).pow(k as u32) - BigUint::one()) >> 1usize;
    let mut g = g.to_vec();
    let mut tries = 0u64;
    while g.len() - 1 > k {
        tries += 1;
        let mut md = MonicDivisor::new(g.clone()).expect("monic");
        let a = rand_poly(rng, p, g.len() - 1);
        if a.len() < 2 {
            continue;
        }
        let b = powmod(f, &mut md, &a, &e);
        let bm1 = trim(poly::sub(f, &b, &[1]));
        if bm1.is_empty() {
            continue;
        }
        let d = poly_gcd(f, &g, &bm1);
        let dd = d.len() - 1;
        if dd == 0 || dd == g.len() - 1 {
            continue;
        }
        let d = monic(f, &d);
        if 2 * dd < g.len() {
            g = d;
        } else {
            g = poly_divexact(f, &g, &d);
        }
    }
    (g, tries)
}

fn mode_psi(ells: &[u64], cap_s: f64) -> Value {
    let mut rows = Vec::new();
    let t_start = Instant::now();
    for &ell in ells {
        if t_start.elapsed().as_secs_f64() > cap_s {
            eprintln!("psi: time cap reached before ℓ={ell}");
            break;
        }
        let (_, inst, _, _, _, _, _) = generic_instance(ell);
        let mut rng = StdRng::seed_from_u64(0x6404 ^ ell);
        let k = ((ell - 1) / 2) as usize;
        let f = field(inst.p, PolyMul::Auto);
        let p = inst.p;
        let mut pts = Vec::new();
        while pts.len() < 2 {
            let x = rng.gen_range(2..p);
            let rhs = f.mul(x, f.add(f.mul(x, x), f.add(f.mul(inst.a, x), 1)));
            if f.legendre(rhs) == inst.chi_n {
                pts.push(x);
            }
        }
        // Descending: ψ_ℓ of the crater (Montgomery A → Weierstrass X).
        let w0 = Instant::now();
        let c0 = f.complete_tally();
        let (a4, a6) = weierstrass_of_montgomery(&f, inst.a);
        let psi = psi_monic(&f, a4, a6, ell);
        let c_psi = f.complete_tally().since(c0);
        let c3 = f.mul(inst.a, inv(&f, 3));
        let mut horizontal = 0u64;
        let mut splits = 0u64;
        let (h_desc, a_floor, img1) = loop {
            let (hw, tries) = cz_one_factor(&f, &psi, k, &mut rng);
            splits += tries;
            let hm = shift_poly(&f, &hw, c3);
            let (a_new, imgs) = apply_kernel(&f, inst.a, ell, &hm, &pts);
            if j_of_a(&f, a_new) == inst.jcrater {
                horizontal += 1;
                continue;
            }
            break (hm, a_new, imgs);
        };
        let c_desc = f.complete_tally().since(c0);
        let wall_desc = w0.elapsed().as_secs_f64();
        let j_floor = j_of_a(&f, a_floor);
        for &x in &img1 {
            assert!(order_check(&f, a_floor, x, inst.n), "image has wrong order");
        }
        let mut checks = vec!["desc_images_order_N"];
        if ell <= 211 {
            let hw = to_weierstrass_kernel(&f, &h_desc, inst.alpha, inst.s);
            let lj = lib_certify(inst.p, &hw, ell).expect("library rejects desc kernel");
            assert_eq!(lj, j_floor);
            checks.push("lib_verify_kernel_desc");
        }
        // Ascending: ψ_ℓ of the floor, gcd with x^{p^k} − x.
        let w1 = Instant::now();
        let c1 = f.complete_tally();
        let (b4, b6) = weierstrass_of_montgomery(&f, a_floor);
        let psi_f = psi_monic(&f, b4, b6, ell);
        let c_psi_f = f.complete_tally().since(c1);
        let xq = powx_mod(&f, &psi_f, &BigUint::from(p).pow(k as u32));
        let g = poly_gcd(&f, &psi_f, &trim(poly::sub(&f, &xq, &[0, 1])));
        let g = monic(&f, &g);
        let deg_g = g.len() - 1;
        let cf3 = f.mul(a_floor, inv(&f, 3));
        let (hw_up, asc_splits) = if deg_g == k {
            (g, 0)
        } else {
            cz_one_factor(&f, &g, k, &mut rng)
        };
        let hm_up = shift_poly(&f, &hw_up, cf3);
        let (a_up, img2) = apply_kernel(&f, a_floor, ell, &hm_up, &img1);
        let c_asc = f.complete_tally().since(c1);
        let wall_asc = w1.elapsed().as_secs_f64();
        let asc_ok = j_of_a(&f, a_up) == inst.jcrater;
        if asc_ok {
            for &x in &img2 {
                assert!(order_check(&f, a_up, x, inst.n), "asc image order");
            }
            checks.push("asc_reaches_crater_images_order_N");
        }
        if ell <= 211 {
            let lj = lib_certify_model(p, b4, b6, &hw_up, ell).expect("library rejects asc");
            assert_eq!(lj, j_of_a(&f, a_up));
            checks.push("lib_verify_kernel_asc");
        }
        eprintln!(
            "psi ℓ={ell} k={k} desc={:.3e} (ψ {:.3e}, splits {splits}) asc={:.3e} (ψ {:.3e}, gcd deg {deg_g}) asc_ok={asc_ok} wall {wall_desc:.1}+{wall_asc:.1}s",
            meq_of(&c_desc) as f64,
            meq_of(&c_psi) as f64,
            meq_of(&c_asc) as f64,
            meq_of(&c_psi_f) as f64
        );
        rows.push(json!({
            "ell": ell, "k": k, "p": p, "chi_m7": chi_minus7(ell),
            "desc": {"psi": tjs(&c_psi), "find": tjs(&c_desc), "cz_splits": splits,
                     "horizontal_retries": horizontal, "wall_s_contended": wall_desc},
            "asc": {"psi": tjs(&c_psi_f), "find": tjs(&c_asc), "frobenius_gcd_degree": deg_g,
                    "cz_splits": asc_splits, "wall_s_contended": wall_asc},
            "crater_j": inst.jcrater, "floor_j": j_floor, "asc_reaches_crater": asc_ok,
            "checks": checks,
        }));
    }
    json!({"mode": "psi", "rows": rows, "arithmetic": "PolyMul::Auto; remainders by MonicDivisor (Newton); gcd and exact division schoolbook; CZ exponentiation 4-bit fixed window; x^(p^k) by square-and-multiply with x-multiplications as shifts"})
}

// ---------------------------------------------------------------------------
// Arm (c): roots of Φ_ℓ(j, Y), with Φ_ℓ construction and the Elkies kernel
// polynomial counted.

/// One root of `g` (squarefree, split over `F_p`) by Cantor–Zassenhaus.
fn cz_root(f: &Fp, g: &[u64], rng: &mut StdRng) -> (u64, u64) {
    let (h, t) = cz_one_factor(f, g, 1, rng);
    (f.neg(h[0]), t)
}

fn eval_phi(f: &Fp, c: &[Vec<u64>], j: u64) -> Vec<u64> {
    let n = c.len();
    let mut jp = vec![1u64; n];
    for i in 1..n {
        jp[i] = f.mul(jp[i - 1], j);
    }
    trim(
        (0..n)
            .map(|b| (0..n).fold(0u64, |acc, a| f.add(acc, f.mul(c[a][b], jp[a]))))
            .collect(),
    )
}

/// `gcd(Y^p − Y, φ)` for monic `φ`.
fn rational_part(f: &Fp, phi: &[u64]) -> Vec<u64> {
    let yp = powx_mod(f, phi, &BigUint::from(f.p()));
    monic(f, &poly_gcd(f, phi, &trim(poly::sub(f, &yp, &[0, 1]))))
}

/// How many products below `(p−1)²` can be added to a reduced value in a
/// `u128` (one less than the library's bound, for safety).
fn fold_of(p: u64) -> usize {
    let pm1 = (p - 1) as u128;
    (((u128::MAX - p as u128) / (pm1 * pm1)).min(1 << 20) as usize)
        .saturating_sub(1)
        .max(1)
}

/// `σ_k(m)` for `m < n` by a sieve.
fn sigma_table(n: usize, k: u32) -> Vec<u128> {
    let mut s = vec![0u128; n];
    for d in 1..n {
        let dk = (d as u128).pow(k);
        let mut m = d;
        while m < n {
            s[m] += dk;
            m += d;
        }
    }
    s
}

/// `out[i + j] += a[i]·b[j]` for the nonzero `a[i]`, `i < min(|a|, len)`,
/// `j < min(|b|, len − i)`, `out` of length `len`, from zero.  This is the
/// loop of the library's `isogeny_walk::poly::mul` (with `len = |a| + |b| −
/// 1`) and `modpoly::mul_trunc`: each executed `(i, j)` is one product and
/// one addition (`out[i+j] = add(out[i+j], mul(a[i], b[j]))`; zero `a[i]` are
/// skipped, zero `b[j]` are not).  `fast`: lazy `u128` accumulation with
/// periodic reduction and the `M` and `A` counts added in bulk; otherwise
/// every operation goes through the counted field.  Both give the same
/// vector and the same tally (tested).
fn conv_counted(f: &Fp, a: &[u64], b: &[u64], len: usize, fast: bool) -> Vec<u64> {
    if !fast {
        let mut out = vec![0u64; len];
        for (i, &ai) in a.iter().enumerate().take(len) {
            if ai == 0 {
                continue;
            }
            for (j, &bj) in b.iter().enumerate().take(len - i) {
                out[i + j] = f.add(out[i + j], f.mul(ai, bj));
            }
        }
        return out;
    }
    let p = f.p() as u128;
    let fold = fold_of(f.p());
    let mut acc = vec![0u128; len];
    let (mut pend, mut nm) = (0usize, 0u64);
    for (i, &ai) in a.iter().enumerate().take(len) {
        if ai == 0 {
            continue;
        }
        let lim = b.len().min(len - i);
        for (slot, &bj) in acc[i..i + lim].iter_mut().zip(b) {
            *slot += ai as u128 * bj as u128;
        }
        nm += lim as u64;
        pend += 1;
        if pend == fold {
            for v in acc.iter_mut() {
                *v %= p;
            }
            pend = 0;
        }
    }
    f.count_muls(nm);
    f.count_adds(nm);
    acc.into_iter().map(|v| (v % p) as u64).collect()
}

/// `J_k` with `j(q) = q^{-1} Σ J_k q^k`, `k < len` (library `j_series`).
fn phi_j_series(f: &Fp, len: usize, fast: bool) -> Vec<u64> {
    let n = len + 1;
    let s3 = sigma_table(n, 3);
    let s5 = sigma_table(n, 5);
    let mut e4 = vec![0u64; n];
    let mut e6 = vec![0u64; n];
    e4[0] = 1;
    e6[0] = 1;
    for m in 1..n {
        e4[m] = f.mul(f.elem(240), f.reduce_u128(s3[m]));
        e6[m] = f.neg(f.mul(f.elem(504), f.reduce_u128(s5[m])));
    }
    let tr = |mut a: Vec<u64>| -> Vec<u64> {
        a.resize(n, 0);
        a
    };
    let sq = tr(conv_counted(f, &e4, &e4, 2 * n - 1, fast));
    let e4_3 = tr(conv_counted(f, &sq, &e4, 2 * n - 1, fast));
    let e6_2 = tr(conv_counted(f, &e6, &e6, 2 * n - 1, fast));
    let diff: Vec<u64> = (0..n).map(|i| f.sub(e4_3[i], e6_2[i])).collect();
    assert_eq!(diff[0], 0, "E4^3 - E6^2 has no constant term");
    let den = &diff[1..];
    let inv0 = inv(f, den[0]);
    let mut invs = vec![0u64; len];
    invs[0] = inv0;
    let p = f.p() as u128;
    let fold = fold_of(f.p());
    for k in 1..len {
        let terms = k.min(den.len() - 1);
        let acc = if fast {
            let mut acc = 0u128;
            let mut pend = 0usize;
            for i in 1..=terms {
                acc += den[i] as u128 * invs[k - i] as u128;
                pend += 1;
                if pend == fold {
                    acc %= p;
                    pend = 0;
                }
            }
            f.count_muls(terms as u64);
            f.count_adds(terms as u64);
            (acc % p) as u64
        } else {
            let mut acc = 0u64;
            for i in 1..=terms {
                acc = f.add(acc, f.mul(den[i], invs[k - i]));
            }
            acc
        };
        invs[k] = f.neg(f.mul(acc, inv0));
    }
    let num = conv_counted(f, &e4_3[..len], &invs, 2 * len - 1, fast);
    let k1728 = f.elem(1728);
    (0..len).map(|k| f.mul(k1728, num[k])).collect()
}

/// Truncated series product (library `modpoly::mul_trunc`).
fn mul_trunc_counted(f: &Fp, a: &[u64], b: &[u64], len: usize, fast: bool) -> Vec<u64> {
    conv_counted(f, a, b, len, fast)
}

const CHECK_BAND: usize = 8;

/// A counted port of the library's `modpoly::modular_polynomial`: the same
/// algorithm and loop structure (q-series of `j`, powers `A[a]`, `B[b]`,
/// triangular solve of the symmetric unknowns with every other exponent
/// checked) on the counted `F_p`.  Returns `Φ_ℓ mod p` as `c[a][b]` and the
/// number of checked exponents.
fn phi_build(f: &Fp, ell: u64, fast: bool) -> Result<(Vec<Vec<u64>>, usize), String> {
    let l = ell as usize;
    let top = l * (l + 1);
    let len = top + CHECK_BAND + 1;
    let p = f.p() as u128;
    let fold = fold_of(f.p());
    let jser = phi_j_series(f, len, fast);
    let mut unit = vec![0u64; len];
    unit[0] = 1;
    let mut apow = vec![unit.clone()];
    for a in 1..=l + 1 {
        let next = mul_trunc_counted(f, &apow[a - 1], &jser, len, fast);
        apow.push(next);
    }
    let mut jl = vec![0u64; len];
    for k in 0..len {
        if k * l < len {
            jl[k * l] = jser[k];
        }
    }
    let mut bpow = vec![unit];
    for b in 1..=l + 1 {
        let next = mul_trunc_counted(f, &bpow[b - 1], &jl, len, fast);
        bpow.push(next);
    }
    let mut r = vec![0u64; len];
    let add_monomial = |r: &mut Vec<u64>, a: usize, b: usize, coef: u64| {
        let pole = a + l * b;
        let need = (pole + CHECK_BAND + 1).min(len);
        let base = top - pole;
        if !fast {
            for i in (0..need).step_by(l) {
                let bi = bpow[b][i];
                if bi == 0 {
                    continue;
                }
                let cb = f.mul(coef, bi);
                for (k, &ak) in apow[a].iter().enumerate().take(need - i) {
                    let idx = base + i + k;
                    r[idx] = f.add(r[idx], f.mul(cb, ak));
                }
            }
            return;
        }
        let mut acc: Vec<u128> = r[base..base + need].iter().map(|&v| v as u128).collect();
        let (mut pend, mut nm, mut na, mut nc) = (0usize, 0u64, 0u64, 0u64);
        for i in (0..need).step_by(l) {
            let bi = bpow[b][i];
            if bi == 0 {
                continue;
            }
            let cb = ((coef as u128 * bi as u128) % p) as u64;
            nc += 1;
            let lim = need - i;
            for (slot, &ak) in acc[i..i + lim].iter_mut().zip(&apow[a]) {
                *slot += cb as u128 * ak as u128;
            }
            nm += lim as u64;
            na += lim as u64;
            pend += 1;
            if pend == fold {
                for v in acc.iter_mut() {
                    *v %= p;
                }
                pend = 0;
            }
        }
        f.count_muls(nc + nm);
        f.count_adds(na);
        for (slot, v) in r[base..base + need].iter_mut().zip(acc) {
            *slot = (v % p) as u64;
        }
    };
    let one = 1u64;
    let minus_one = f.neg(one);
    add_monomial(&mut r, l + 1, 0, one);
    add_monomial(&mut r, 0, l + 1, one);
    add_monomial(&mut r, l, l, minus_one);
    let mut by_pole: Vec<Option<(usize, usize)>> = vec![None; top + 1];
    for b in 0..=l {
        for a in 0..=b {
            if a == l && b == l {
                continue;
            }
            let pole = a + l * b;
            assert!(by_pole[pole].is_none(), "pole orders are distinct");
            by_pole[pole] = Some((a, b));
        }
    }
    let mut c = vec![vec![0u64; l + 2]; l + 2];
    c[l + 1][0] = one;
    c[0][l + 1] = one;
    c[l][l] = minus_one;
    let mut checks = 0usize;
    for idx in 0..len {
        let pole = top as isize - idx as isize;
        let slot = if pole >= 0 {
            by_pole[pole as usize]
        } else {
            None
        };
        match slot {
            Some((a, b)) => {
                let coef = f.neg(r[idx]);
                c[a][b] = coef;
                c[b][a] = coef;
                add_monomial(&mut r, a, b, coef);
                if a != b {
                    add_monomial(&mut r, b, a, coef);
                }
                if r[idx] != 0 {
                    return Err(format!(
                        "Φ_{ell}: unknown ({a},{b}) left q^{} nonzero",
                        -pole
                    ));
                }
            }
            None => {
                if r[idx] != 0 {
                    return Err(format!("Φ_{ell}: q^{} does not vanish", -pole));
                }
                checks += 1;
            }
        }
    }
    Ok((c, checks))
}

// ---- Elkies kernel polynomial (counted port of the library's step).

/// `℘ = z^{-2} + Σ_{k≥1} c_k z^{2k}` coefficients for `y² = x³ + a x + b`.
fn wp_coeffs(f: &Fp, a: u64, b: u64, n: usize) -> Vec<u64> {
    let dv = |x: u64, y: u64| f.div(x, y).expect("nonzero denominator");
    let mut c = vec![0u64; n + 1];
    if n >= 1 {
        c[1] = dv(f.neg(a), f.elem(5));
    }
    if n >= 2 {
        c[2] = dv(f.neg(b), f.elem(7));
    }
    for k in 3..=n {
        let mut acc = 0u64;
        for j in 1..=k - 2 {
            acc = f.add(acc, f.mul(c[j], c[k - 1 - j]));
        }
        let den = f.elem(((k - 2) * (2 * k + 3)) as u64);
        c[k] = dv(f.mul(f.elem(3), acc), den);
    }
    c
}

fn derivative_counted(f: &Fp, a: &[u64]) -> Vec<u64> {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| f.mul(c, f.elem(i as u64)))
            .collect(),
    )
}

/// The monic polynomial of degree `d` with root power sums `s[0..d]`
/// (library `poly::from_power_sums`).
fn from_power_sums_counted(f: &Fp, s: &[u64], d: usize) -> Vec<u64> {
    let mut e = vec![1u64];
    for k in 1..=d {
        let mut acc = 0u64;
        for i in 1..=k {
            let t = f.mul(e[k - i], s[i - 1]);
            acc = if i % 2 == 1 {
                f.add(acc, t)
            } else {
                f.sub(acc, t)
            };
        }
        e.push(f.div(acc, f.elem(k as u64)).expect("p > d"));
    }
    let mut h = vec![0u64; d + 1];
    for (k, &ek) in e.iter().enumerate() {
        h[d - k] = if k % 2 == 1 { f.neg(ek) } else { ek };
    }
    h
}

/// The kernel polynomial from the codomain and `S₁` (library
/// `kernel::kernel_from_sums`).
fn kernel_from_sums_counted(f: &Fp, e: (u64, u64), e2: (u64, u64), s1: u64, d: usize) -> Vec<u64> {
    let c = wp_coeffs(f, e.0, e.1, d);
    let ct = wp_coeffs(f, e2.0, e2.1, d);
    let cubic = trim(vec![f.mul(f.elem(4), e.1), f.mul(f.elem(4), e.0), 0, 4]);
    let quad = trim(vec![f.mul(f.elem(2), e.0), 0, 6]);
    let mut s = vec![f.elem((2 * d) as u64), s1];
    let mut fk: Vec<u64> = vec![0, 1];
    let mut fact = 1u64;
    for k in 1..d {
        let d1 = derivative_counted(f, &fk);
        let d2 = derivative_counted(f, &d1);
        fk = trim(poly::add(
            f,
            &poly::mul(f, &d2, &cubic),
            &poly::mul(f, &d1, &quad),
        ));
        fact = f.mul(fact, f.elem(((2 * k - 1) * (2 * k)) as u64));
        let mut rhs = f.mul(fact, f.sub(ct[k], c[k]));
        for i in 0..=k {
            if let Some(&coef) = fk.get(i) {
                rhs = f.sub(rhs, f.mul(coef, s[i]));
            }
        }
        let lead = fk[k + 1];
        s.push(f.div(rhs, lead).expect("nonzero leading coefficient"));
    }
    let half = inv(f, 2);
    let halves: Vec<u64> = s[1..=d].iter().map(|&v| f.mul(v, half)).collect();
    from_power_sums_counted(f, &halves, d)
}

/// Elkies' kernel polynomial of the isogeny from `y² = x³ + a x + b`
/// (`j(E) = j`) to the curve with invariant `j2`, a simple root of
/// `Φ_ℓ(j, Y)`: the counted port of `kernel::elkies_isogeny` without its
/// final certificate (`verify_kernel`).  `phi_y` is `Φ_ℓ(j, Y)` as already
/// computed by the root finding (the `X`-row sums `S0_b`); the other partial
/// derivatives are evaluated from shared row sums (two further `n²` passes
/// for `∂_X`, `∂_X²`), a cheaper schedule than the library's per-term
/// `partial()`.  Returns the kernel polynomial and the codomain
/// `(a', b')`.
#[allow(clippy::too_many_arguments)]
fn elkies_counted(
    f: &Fp,
    e: (u64, u64),
    c: &[Vec<u64>],
    phi_y: &[u64],
    j: u64,
    j2: u64,
    ell: u64,
) -> Option<(Vec<u64>, (u64, u64))> {
    let n = c.len();
    let k1728 = f.elem(1728);
    if j == 0 || j == k1728 || j2 == 0 || j2 == k1728 {
        return None;
    }
    let dv = |x: u64, y: u64| f.div(x, y);
    // Row sums over a: S1_b = Σ a c_ab j^{a−1}, S2_b = Σ a(a−1) c_ab j^{a−2}.
    let mut jp = vec![1u64; n];
    for i in 1..n {
        jp[i] = f.mul(jp[i - 1], j);
    }
    let w1: Vec<u64> = (0..n)
        .map(|a| {
            if a >= 1 {
                f.mul(f.elem(a as u64), jp[a - 1])
            } else {
                0
            }
        })
        .collect();
    let w2: Vec<u64> = (0..n)
        .map(|a| {
            if a >= 2 {
                f.mul(f.elem((a * (a - 1)) as u64), jp[a - 2])
            } else {
                0
            }
        })
        .collect();
    let dot = |w: &[u64], b: usize| -> u64 {
        (0..n).fold(0u64, |acc, a| f.add(acc, f.mul(c[a][b], w[a])))
    };
    let s1: Vec<u64> = (0..n).map(|b| dot(&w1, b)).collect();
    let s2: Vec<u64> = (0..n).map(|b| dot(&w2, b)).collect();
    let mut s0 = phi_y.to_vec();
    s0.resize(n, 0);
    // Powers of y = j2 and the Y-weights.
    let mut yp = vec![1u64; n];
    for i in 1..n {
        yp[i] = f.mul(yp[i - 1], j2);
    }
    let eval_row = |s: &[u64], dy: usize| -> u64 {
        let mut acc = 0u64;
        for b in dy..n {
            let wgt = if dy == 0 {
                yp[b]
            } else if dy == 1 {
                f.mul(f.elem(b as u64), yp[b - 1])
            } else {
                f.mul(f.elem((b * (b - 1)) as u64), yp[b - 2])
            };
            acc = f.add(acc, f.mul(s[b], wgt));
        }
        acc
    };
    let px = eval_row(&s1, 0);
    let py = eval_row(&s0, 1);
    let pxx = eval_row(&s2, 0);
    let pxy = eval_row(&s1, 1);
    let pyy = eval_row(&s0, 2);
    if py == 0 || px == 0 {
        return None;
    }
    let l = f.elem(ell);
    let e4 = dv(f.neg(e.0), f.elem(3))?;
    let e6 = dv(f.neg(e.1), f.elem(2))?;
    let jj = f.neg(dv(f.mul(j, e6), e4)?);
    let jt = f.neg(dv(f.mul(px, jj), f.mul(l, py))?);
    let e4t = dv(f.sqr(jt), f.mul(j2, f.sub(j2, k1728)))?;
    let e6t = f.neg(dv(f.mul(jt, e4t), j2)?);
    let l2 = f.sqr(l);
    let l4 = f.sqr(l2);
    let l6 = f.mul(l4, l2);
    let codomain = (
        f.mul(f.from_i64(-3), f.mul(l4, e4t)),
        f.mul(f.from_i64(-2), f.mul(l6, e6t)),
    );
    let two_thirds = dv(f.elem(2), f.elem(3))?;
    let half = dv(1, f.elem(2))?;
    let term = |jv: u64, e4v: u64, e6v: u64| -> Option<u64> {
        let r = dv(f.sqr(e6v), f.sqr(e4v))?;
        Some(f.add(f.mul(two_thirds, f.mul(jv, r)), f.mul(half, f.mul(jv, e4v))))
    };
    let mut bracket = f.mul(pxx, f.sqr(jj));
    bracket = f.add(
        bracket,
        f.mul(f.elem(2), f.mul(l, f.mul(pxy, f.mul(jj, jt)))),
    );
    bracket = f.add(bracket, f.mul(l2, f.mul(pyy, f.sqr(jt))));
    bracket = f.add(bracket, f.mul(px, term(j, e4, e6)?));
    bracket = f.add(bracket, f.mul(l2, f.mul(py, term(j2, e4t, e6t)?)));
    let g = dv(f.mul(f.elem(6), bracket), f.mul(l, f.mul(py, jt)))?;
    let s1sum = f.neg(f.mul(l, g));
    let d = ((ell - 1) / 2) as usize;
    let h = kernel_from_sums_counted(f, e, codomain, s1sum, d);
    Some((h, codomain))
}

/// Self-test of the counted ports: the fast Φ_ℓ builder against the
/// per-operation port (same table, same tally) and against the library.
fn mode_selftest(ells: &[u64]) -> Value {
    let mut rows = Vec::new();
    for &ell in ells {
        let (_, inst, _, _, _, _, _) = generic_instance(ell);
        let p = inst.p;
        let fa = field(p, PolyMul::Auto);
        let fr = field(p, PolyMul::Auto);
        let (ca, _) = phi_build(&fa, ell, true).expect("fast Φ");
        let (cr, _) = phi_build(&fr, ell, false).expect("reference Φ");
        let lf = lib_field(p);
        let lib = modpoly::modular_polynomial(&lf, ell).expect("library Φ");
        let lc: Vec<Vec<u64>> = lib
            .c
            .iter()
            .map(|r| r.iter().map(|v| lf.to_big(v).to_u64().unwrap()).collect())
            .collect();
        let (ta, tr) = (fa.complete_tally(), fr.complete_tally());
        let ok = ca == cr && ca == lc && ta == tr;
        eprintln!(
            "selftest ℓ={ell}: fast==reference table {} tally {} library {} (M {} A {})",
            ca == cr,
            ta == tr,
            ca == lc,
            ta.ops.mul,
            ta.adds.add
        );
        assert!(ok, "counted Φ_ℓ port self-test failed at ℓ={ell}");
        rows.push(
            json!({"ell": ell, "p": p, "tables_equal_fast_reference": ca == cr,
            "tables_equal_library": ca == lc, "tallies_equal": ta == tr,
            "fast": tjs(&ta), "reference": tjs(&tr)}),
        );
    }
    json!({"mode": "selftest", "rows": rows})
}

fn mode_modpoly(ells: &[u64], cap_s: f64, lib_check_max: u64) -> Value {
    let mut rows = Vec::new();
    let t_start = Instant::now();
    for &ell in ells {
        if t_start.elapsed().as_secs_f64() > cap_s {
            eprintln!("modpoly: time cap reached before ℓ={ell}");
            break;
        }
        let (_, inst, _, _, _, _, _) = generic_instance(ell);
        let mut rng = StdRng::seed_from_u64(0x6405 ^ ell);
        let p = inst.p;
        let f = field(p, PolyMul::Auto);
        let lf = lib_field(p);
        // Φ_ℓ mod p: the counted port of the library builder.
        let w0 = Instant::now();
        let c0 = f.complete_tally();
        let (c, checks) = phi_build(&f, ell, true).expect("Φ_ℓ construction");
        let build = f.complete_tally().since(c0);
        let t_build = w0.elapsed().as_secs_f64();
        // The library's own table, for equality (wall time, uncounted).
        let (lib_equal, t_lib) = if ell <= lib_check_max {
            let w = Instant::now();
            let phi_lib = modpoly::modular_polynomial(&lf, ell).expect("library Φ_ℓ");
            let t = w.elapsed().as_secs_f64();
            let lc: Vec<Vec<u64>> = phi_lib
                .c
                .iter()
                .map(|r| r.iter().map(|v| lf.to_big(v).to_u64().unwrap()).collect())
                .collect();
            assert_eq!(lc, c, "counted Φ_ℓ port differs from the library's");
            (json!(true), json!(t))
        } else {
            (json!(null), json!(null))
        };
        let phi = modpoly::ModPoly {
            ell,
            c: c.iter()
                .map(|r| r.iter().map(|&v| lf.from_u64(v)).collect())
                .collect(),
            checks_passed: checks,
        };
        let jc = inst.jcrater;
        // Descending.
        let c0 = f.complete_tally();
        let py = eval_phi(&f, &c, jc);
        assert_eq!(py.last(), Some(&1), "Φ_ℓ monic in Y");
        let c_eval = f.complete_tally().since(c0);
        let c1 = f.complete_tally();
        let mut g = rational_part(&f, &py);
        let n_rat = g.len() - 1;
        let mut crater_mult = 0;
        while horner(&f, &g, jc) == 0 {
            g = poly_divexact(&f, &g, &[f.neg(jc), 1]);
            crater_mult += 1;
        }
        let (j_floor, splits) = cz_root(&f, &g, &mut rng);
        let c_roots = f.complete_tally().since(c1);
        assert_eq!(horner(&f, &py, j_floor), 0);
        // Elkies kernel polynomial of the descending edge.
        let e_crater = (red(&f, -35), red(&f, -98));
        let c2 = f.complete_tally();
        let el = elkies_counted(&f, e_crater, &c, &py, jc, j_floor, ell);
        let c_elk = f.complete_tally().since(c2);
        // Ascending.
        let c3 = f.complete_tally();
        let pf = eval_phi(&f, &c, j_floor);
        let a_eval = f.complete_tally().since(c3);
        let c4 = f.complete_tally();
        let gf = rational_part(&f, &pf);
        let n_rat_f = gf.len() - 1;
        let (j_up, asc_splits) = if n_rat_f == 1 {
            (f.neg(gf[0]), 0)
        } else {
            cz_root(&f, &gf, &mut rng)
        };
        let a_roots = f.complete_tally().since(c4);
        let asc_ok = j_up == jc;
        let kap = f.mul(j_floor, inv(&f, f.sub(1728, j_floor)));
        let e_floor = (f.mul(3, kap), f.mul(2, kap));
        let c5 = f.complete_tally();
        let el_up = elkies_counted(&f, e_floor, &c, &pf, j_floor, jc, ell);
        let a_elk = f.complete_tally().since(c5);
        // Library Elkies kernels for both edges (wall, uncounted) and
        // equality with the counted port's kernel polynomials.
        let e_cm = Model {
            a: lf.from_i64(-35),
            b: lf.from_i64(-98),
        };
        let jf = lf.from_u64(j_floor);
        let w1 = Instant::now();
        let el_desc = kernel::elkies_isogeny(&lf, &e_cm, &phi, &jf);
        let t_el_desc = w1.elapsed().as_secs_f64();
        let e_fl = Model {
            a: lf.from_u64(e_floor.0),
            b: lf.from_u64(e_floor.1),
        };
        let w2 = Instant::now();
        let el_asc = kernel::elkies_isogeny(&lf, &e_fl, &phi, &lf.from_u64(jc));
        let t_el_asc = w2.elapsed().as_secs_f64();
        let same_kernel = |ours: &Option<(Vec<u64>, (u64, u64))>,
                           lib: &Result<kernel::Isogeny, kernel::EdgeError>|
         -> bool {
            match (ours, lib) {
                (Some((h, cod)), Ok(iso)) => {
                    let hl: Vec<u64> = iso
                        .kernel
                        .iter()
                        .map(|v| lf.to_big(v).to_u64().unwrap())
                        .collect();
                    let cl = (
                        lf.to_big(&iso.codomain.a).to_u64().unwrap(),
                        lf.to_big(&iso.codomain.b).to_u64().unwrap(),
                    );
                    trim(h.clone()) == trim(hl) && *cod == cl
                }
                (None, Err(_)) => true,
                _ => false,
            }
        };
        let k_desc = same_kernel(&el, &el_desc);
        let k_asc = same_kernel(&el_up, &el_asc);
        let jc_of = |r: &Result<kernel::Isogeny, kernel::EdgeError>| -> Value {
            match r {
                Ok(iso) => json!(lf.to_big(&iso.codomain.j(&lf).unwrap()).to_u64()),
                Err(e) => json!(format!("{e:?}")),
            }
        };
        eprintln!(
            "modpoly ℓ={ell} build {:.3e} Meq {:.1}s | desc eval+roots {:.3e} elkies {:.3e} | asc {:.3e} {:.3e} | asc_ok={asc_ok} kernels_equal={k_desc}/{k_asc} lib_equal={lib_equal}",
            meq_of(&build) as f64,
            t_build,
            (meq_of(&c_eval) + meq_of(&c_roots)) as f64,
            meq_of(&c_elk) as f64,
            (meq_of(&a_eval) + meq_of(&a_roots)) as f64,
            meq_of(&a_elk) as f64,
        );
        rows.push(json!({
            "ell": ell, "p": p, "chi_m7": chi_minus7(ell),
            "build": tjs(&build), "build_wall_s_contended": t_build,
            "library_phi_wall_s_contended": t_lib, "port_equals_library_phi": lib_equal,
            "phi_checks_passed": checks,
            "desc": {"eval": tjs(&c_eval), "roots": tjs(&c_roots), "elkies": tjs(&c_elk),
                     "rational_roots": n_rat, "crater_root_multiplicity": crater_mult,
                     "cz_splits": splits,
                     "elkies_ok": el.is_some() && el_desc.is_ok(),
                     "elkies_kernel_equals_library": k_desc,
                     "elkies_codomain_j": jc_of(&el_desc),
                     "elkies_wall_s_contended": t_el_desc},
            "asc": {"eval": tjs(&a_eval), "roots": tjs(&a_roots), "elkies": tjs(&a_elk),
                    "rational_roots": n_rat_f, "cz_splits": asc_splits,
                    "elkies_ok": el_up.is_some() && el_asc.is_ok(),
                    "elkies_kernel_equals_library": k_asc,
                    "elkies_codomain_j": jc_of(&el_asc),
                    "elkies_wall_s_contended": t_el_asc},
            "crater_j": jc, "floor_j": j_floor, "asc_reaches_crater": asc_ok,
        }));
    }
    json!({"mode": "modpoly", "rows": rows,
           "counted": "Phi_l mod p: counted port of the library's q-series builder (same loops; lazy u128 accumulation with bulk M/A counts, tested equal to a per-operation port and to the library table); evaluation of Phi_l(j, Y); root finding (gcd with Y^p - Y, Cantor-Zassenhaus); Elkies kernel polynomial (counted port of kernel::elkies_isogeny without its verify_kernel certificate, partial derivatives from shared row sums); the library's Elkies kernel is compared for equality (uncounted wall time)",
           "sutherland_crt": "not implemented"})
}

// ---------------------------------------------------------------------------
// Mode: fits per addition weight, the ordering table, the crossover table.

/// `Meq + a·(A + WA)` of a [`tjs`] object.
fn tc(v: &Value, a: f64) -> f64 {
    v["Meq"].as_f64().unwrap() + a * (v["A"].as_f64().unwrap() + v["WA"].as_f64().unwrap())
}

fn tsum(vs: &[&Value], a: f64) -> f64 {
    vs.iter().map(|v| tc(v, a)).sum()
}

fn akey(a: f64) -> String {
    format!("{a}")
}

/// All rows of every `<prefix>*.json` file in `dir` (sorted by name).
fn rows_prefix(dir: &str, prefix: &str) -> Vec<Value> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| n.starts_with(prefix) && n.ends_with(".json"))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let mut out = Vec::new();
    for n in names {
        out.extend(
            load(&format!("{dir}/{n}"))["rows"]
                .as_array()
                .unwrap()
                .iter()
                .cloned(),
        );
    }
    out
}

/// Rows with the same `ell` keep their first occurrence.
fn dedup_ell(rows: Vec<Value>) -> Vec<Value> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out: Vec<Value> = rows
        .into_iter()
        .filter(|r| seen.insert(r["ell"].as_u64().unwrap()))
        .collect();
    out.sort_by_key(|r| r["ell"].as_u64().unwrap());
    out
}

/// A fit with both the OLS t-interval and a pairs-bootstrap percentile
/// interval for the exponent (2000 resamples, seeded from the series name).
#[derive(Clone, Copy, Debug)]
struct Fit2 {
    ols: Fit,
    boot_lo: f64,
    boot_hi: f64,
}

fn fit_boot(pts: &[(f64, f64)], tag: &str) -> Fit2 {
    let ols = fit_loglog(pts);
    let xs: Vec<f64> = pts.iter().map(|p| p.0.log2()).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1.log2()).collect();
    let n = xs.len();
    let seed = tag.bytes().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ b as u64).wrapping_mul(0x100000001b3)
    });
    let mut rng = StdRng::seed_from_u64(seed);
    let mut slopes = Vec::with_capacity(2000);
    for _ in 0..2000 {
        let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
        let mx = idx.iter().map(|&i| xs[i]).sum::<f64>() / n as f64;
        let my = idx.iter().map(|&i| ys[i]).sum::<f64>() / n as f64;
        let sxx: f64 = idx.iter().map(|&i| (xs[i] - mx).powi(2)).sum();
        if sxx < 1e-12 {
            continue;
        }
        let sxy: f64 = idx.iter().map(|&i| (xs[i] - mx) * (ys[i] - my)).sum();
        slopes.push(sxy / sxx);
    }
    slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |t: f64| slopes[((slopes.len() as f64 - 1.0) * t).round() as usize];
    Fit2 {
        ols,
        boot_lo: q(0.025),
        boot_hi: q(0.975),
    }
}

impl Fit2 {
    fn js(&self, name: &str, range: &str, a: f64) -> Value {
        json!({"series": name, "range": range, "a": a, "exponent": self.ols.slope,
               "ols_ci95": [self.ols.slope_lo, self.ols.slope_hi],
               "bootstrap_ci95": [self.boot_lo, self.boot_hi],
               "log2_constant": self.ols.icpt, "n": self.ols.n, "r2": self.ols.r2})
    }
}

fn ci_str(f: &Fit2) -> String {
    format!(
        "{:.3} OLS [{:.3}, {:.3}] boot [{:.3}, {:.3}]",
        f.ols.slope, f.ols.slope_lo, f.ols.slope_hi, f.boot_lo, f.boot_hi
    )
}

/// The routes compared in the ordering table, with the cost at weight `a`.
const ROUTES: [&str; 4] = [
    "ladder (a)",
    "psi + factoring (b)",
    "Phi(j,Y) amortised (c)",
    "Phi(j,Y) cold (c)",
];

fn mode_analyze(dir: &str, r64: &str, out: &str) -> Value {
    let mut text = String::new();
    // ---- data
    let lad_grid = dedup_ell(rows_prefix(dir, "ladder_grid"));
    let lad_all = dedup_ell(
        rows_prefix(dir, "ladder_grid")
            .into_iter()
            .chain(rows_prefix(dir, "ladder_extra"))
            .collect(),
    );
    let mut proj_by_ell: std::collections::BTreeMap<u64, Value> = Default::default();
    for r in rows_prefix(dir, "project") {
        let l = r["ell"].as_u64().unwrap();
        match proj_by_ell.get_mut(&l) {
            None => {
                proj_by_ell.insert(l, r);
            }
            Some(old) => {
                for (k, v) in r["arms"].as_object().unwrap() {
                    old["arms"][k] = v.clone();
                }
            }
        }
    }
    let proj: Vec<Value> = proj_by_ell.into_values().collect();
    let psi_grid = dedup_ell(rows_prefix(dir, "psi_grid"));
    let psi_all = dedup_ell(
        rows_prefix(dir, "psi_grid")
            .into_iter()
            .chain(rows_prefix(dir, "psi_extra"))
            .collect(),
    );
    let mp = dedup_ell(rows_prefix(dir, "modpoly_grid"));
    let mp_all = dedup_ell(
        rows_prefix(dir, "modpoly_grid")
            .into_iter()
            .chain(rows_prefix(dir, "modpoly_extra"))
            .collect(),
    );
    text.push_str(&format!(
        "rows: ladder grid {} (+extra {}), projection {}, psi grid {} (+extra {}), modpoly grid {} (+extra {})\n",
        lad_grid.len(),
        lad_all.len() - lad_grid.len(),
        proj.len(),
        psi_grid.len(),
        psi_all.len() - psi_grid.len(),
        mp.len(),
        mp_all.len() - mp.len()
    ));
    // ---- reproduction of the frozen R6.4 counts (a = 0 columns).
    let mut repro = serde_json::Map::new();
    {
        let old_lad = {
            let mut v = rows_of(r64, &["ladder_auto.json", "ladder_auto_hi.json"]);
            v.sort_by_key(|r| r["ell"].as_u64().unwrap());
            v
        };
        let same_mwi = |x: &Value, y: &Value| -> bool {
            x["M"] == y["M"] && x["W"] == y["W"] && x["I"] == y["I"]
        };
        let (mut n, mut same) = (0, 0);
        for o in &old_lad {
            if let Some(r) = lad_all.iter().find(|r| r["ell"] == o["ell"]) {
                n += 1;
                let ok = ["desc", "asc"].iter().all(|d| {
                    same_mwi(&o[d]["find"], &r[d]["find"])
                        && same_mwi(&o[d]["ladder"], &r[d]["ladder"])
                        && same_mwi(&o[d]["minpoly"], &r[d]["minpoly"])
                        && o[d]["find_expected_Meq"] == r[d]["find_expected_Meq"]
                }) && o["p"] == r["p"];
                same += ok as usize;
            }
        }
        repro.insert(
            "ladder".into(),
            json!({"frozen_rows_matched_by_ell": n, "identical_M_W_I_and_expected_find": same}),
        );
        let old_psi = rows_of(r64, &["psi.json"]);
        let (mut n, mut same) = (0, 0);
        for o in &old_psi {
            if let Some(r) = psi_all.iter().find(|r| r["ell"] == o["ell"]) {
                n += 1;
                let ok = ["desc", "asc"].iter().all(|d| {
                    same_mwi(&o[d]["find"], &r[d]["find"]) && same_mwi(&o[d]["psi"], &r[d]["psi"])
                });
                same += ok as usize;
            }
        }
        repro.insert(
            "psi".into(),
            json!({"frozen_rows_matched_by_ell": n, "identical_M_W_I": same}),
        );
        let old_mp = rows_of(r64, &["modpoly.json"]);
        let (mut n, mut same) = (0, 0);
        for o in &old_mp {
            if let Some(r) = mp.iter().find(|r| r["ell"] == o["ell"]) {
                n += 1;
                let sum3 = |d: &Value| -> (u64, u64, u64) {
                    let g =
                        |k: &str| d["eval"][k].as_u64().unwrap() + d["roots"][k].as_u64().unwrap();
                    (g("M"), g("W"), g("I"))
                };
                let o3 = |d: &Value| -> (u64, u64, u64) {
                    (
                        d["find"]["M"].as_u64().unwrap(),
                        d["find"]["W"].as_u64().unwrap(),
                        d["find"]["I"].as_u64().unwrap(),
                    )
                };
                let ok = o3(&o["desc"]) == sum3(&r["desc"])
                    && (o["asc_reaches_crater"] != r["asc_reaches_crater"]
                        || o3(&o["asc"]) == sum3(&r["asc"]));
                same += ok as usize;
            }
        }
        repro.insert(
            "modpoly_eval_plus_roots".into(),
            json!({"frozen_rows_matched_by_ell": n, "identical_M_W_I": same}),
        );
        let old_pr = rows_of(r64, &["project.json"]);
        let (mut n, mut same) = (0, 0);
        for o in &old_pr {
            if let Some(r) = proj.iter().find(|r| r["ell"] == o["ell"]) {
                for arm in ["karatsuba", "auto", "ntt"] {
                    if r["arms"].get(arm).is_none() {
                        continue;
                    }
                    n += 1;
                    let (x, y) = (&o["arms"][arm], &r["arms"][arm]);
                    let ok = same_mwi(&x["xdbl"], &y["xdbl"])
                        && same_mwi(&x["xadd"], &y["xadd"])
                        && x["ext_mul_Meq"] == y["ext_mul_Meq"]
                        && x["find_expected_Meq"] == y["find_expected_Meq"];
                    same += ok as usize;
                }
            }
        }
        repro.insert(
            "project".into(),
            json!({"frozen_arm_rows_matched": n, "identical": same}),
        );
    }
    text.push_str(&format!(
        "\nReproduction of the frozen R6.4 counts (M, W, I and the Meq-based quantities; additions requested only here): {}\n",
        serde_json::to_string(&repro).unwrap()
    ));

    // ---- series at each weight.
    let mut fits_js: Vec<Value> = Vec::new();
    let mut fits: std::collections::HashMap<(String, String, String), Fit2> = Default::default();
    let mut order_js: Vec<Value> = Vec::new();
    let mut cross_js: Vec<Value> = Vec::new();
    let ev_pts = |a: f64| -> Vec<(f64, f64)> {
        lad_grid
            .iter()
            .map(|r| (r["ell"].as_f64().unwrap(), tc(&r["eval_pt"], a)))
            .collect()
    };
    for &a in &WEIGHTS {
        let ak = akey(a);
        text.push_str(&format!(
            "\n================ addition weight a = {a} M ================\n"
        ));
        let lad = |rows: &[Value], d: &str| -> Vec<(f64, f64)> {
            rows.iter()
                .filter(|r| d == "desc" || r["asc_reaches_crater"].as_bool() == Some(true))
                .map(|r| {
                    (
                        r["ell"].as_f64().unwrap(),
                        r[d]["find_expected_by_a"][&ak].as_f64().unwrap(),
                    )
                })
                .collect()
        };
        let pser = |arm: &str| -> Vec<(f64, f64)> {
            proj.iter()
                .filter(|r| r["arms"].get(arm).is_some())
                .map(|r| {
                    (
                        r["ell"].as_f64().unwrap(),
                        r["arms"][arm]["find_expected_by_a"][&ak].as_f64().unwrap(),
                    )
                })
                .collect()
        };
        let psi_s = |rows: &[Value], d: &str| -> Vec<(f64, f64)> {
            rows.iter()
                .filter(|r| d == "desc" || r["asc_reaches_crater"].as_bool() == Some(true))
                .map(|r| (r["ell"].as_f64().unwrap(), tc(&r[d]["find"], a)))
                .collect()
        };
        let phi_s = |rows: &[Value], d: &str, cold: bool| -> Vec<(f64, f64)> {
            rows.iter()
                .filter(|r| d == "desc" || r["asc_reaches_crater"].as_bool() == Some(true))
                .map(|r| {
                    let x = &r[d];
                    let mut v = vec![&x["eval"], &x["roots"], &x["elkies"]];
                    if cold {
                        v.push(&r["build"]);
                    }
                    (r["ell"].as_f64().unwrap(), tsum(&v, a))
                })
                .collect()
        };
        let phi_r64 = |rows: &[Value]| -> Vec<(f64, f64)> {
            rows.iter()
                .map(|r| {
                    (
                        r["ell"].as_f64().unwrap(),
                        tsum(&[&r["desc"]["eval"], &r["desc"]["roots"]], a),
                    )
                })
                .collect()
        };
        let build_s: Vec<(f64, f64)> = mp
            .iter()
            .map(|r| (r["ell"].as_f64().unwrap(), tc(&r["build"], a)))
            .collect();
        let l_desc = lad(&lad_grid, "desc");
        let l_asc = lad(&lad_grid, "asc");
        let (p_k, p_a, p_n) = (pser("karatsuba"), pser("auto"), pser("ntt"));
        // Auto chooses Karatsuba or the NTT per product by M + W alone.  With
        // additions priced the cheaper whole ladder can be the Karatsuba one
        // (its additions are fewer per product than the NTT's), so also fit
        // the per-ell minimum of the two measured arms at this weight.
        let p_b: Vec<(f64, f64)> = p_a
            .iter()
            .filter_map(|q| p_k.iter().find(|k| k.0 == q.0).map(|k| (q.0, q.1.min(k.1))))
            .collect();
        let s_desc = psi_s(&psi_grid, "desc");
        let s_asc = psi_s(&psi_grid, "asc");
        let m_am_d = phi_s(&mp, "desc", false);
        let m_am_a = phi_s(&mp, "asc", false);
        let m_co_d = phi_s(&mp, "desc", true);
        let m_co_a = phi_s(&mp, "asc", true);
        let m_am_d_all = phi_s(&mp_all, "desc", false);
        let m_am_a_all = phi_s(&mp_all, "asc", false);
        let m_co_d_all = phi_s(&mp_all, "desc", true);
        let m_co_a_all = phi_s(&mp_all, "asc", true);
        let mut push = |name: &str, range: &str, s: &[(f64, f64)]| -> Fit2 {
            let f = fit_boot(s, &format!("{name}|{range}|{ak}"));
            fits_js.push(f.js(name, range, a));
            fits.insert((name.to_string(), range.to_string(), ak.clone()), f);
            text.push_str(&format!(
                "  {:<58} {:<9} n={:<3} {}  log2c {:.2}\n",
                name,
                range,
                f.ols.n,
                ci_str(&f),
                f.ols.icpt
            ));
            f
        };
        let f_ld30 = push("ladder find desc (measured)", "l>=30", &sel(&l_desc, 30.0));
        let f_ld = push(
            "ladder find desc (measured)",
            "l>=256",
            &sel(&l_desc, 256.0),
        );
        push("ladder find asc (measured)", "l>=30", &sel(&l_asc, 30.0));
        let f_la = push("ladder find asc (measured)", "l>=256", &sel(&l_asc, 256.0));
        push("projected ladder, karatsuba", "l>=2^10", &sel(&p_k, 1024.0));
        let f_pa10 = push("projected ladder, auto", "l>=2^10", &sel(&p_a, 1024.0));
        let f_pa = push("projected ladder, auto", "l>=2^13", &sel(&p_a, 8192.0));
        push(
            "projected ladder, best of karatsuba/auto",
            "l>=2^10",
            &sel(&p_b, 1024.0),
        );
        let f_pb = push(
            "projected ladder, best of karatsuba/auto",
            "l>=2^13",
            &sel(&p_b, 8192.0),
        );
        let f_pn = if sel(&p_n, 8192.0).len() >= 3 {
            Some(push("projected ladder, ntt", "l>=2^13", &sel(&p_n, 8192.0)))
        } else {
            None
        };
        let f_sd = push("psi + CZ desc (measured)", "grid", &s_desc);
        let f_sa = push("psi + gcd asc (measured)", "grid", &s_asc);
        push("Phi amortised (eval+roots+Elkies) desc", "11..103", &m_am_d);
        push("Phi amortised (eval+roots+Elkies) asc", "11..103", &m_am_a);
        let f_cd = push(
            "Phi cold (build+eval+roots+Elkies) desc",
            "11..103",
            &m_co_d,
        );
        push("Phi cold (build+eval+roots+Elkies) asc", "11..103", &m_co_a);
        push("Phi construction alone (counted port)", "11..103", &build_s);
        push(
            "Phi eval+roots only (R6.4's row (c))",
            "11..103",
            &phi_r64(&mp),
        );
        let f_am = fits[&(
            "Phi amortised (eval+roots+Elkies) desc".to_string(),
            "11..103".to_string(),
            ak.clone(),
        )];
        // local exponents in 2-octave windows (auto)
        let mut lo = 5.0;
        let mut wins = Vec::new();
        let lmax = p_a.last().map(|p| p.0).unwrap_or(1.0);
        while lo + 2.0 <= lmax.log2() + 1e-9 {
            let v: Vec<(f64, f64)> = p_a
                .iter()
                .filter(|q| q.0.log2() >= lo - 1e-9 && q.0.log2() <= lo + 2.1)
                .copied()
                .collect();
            if v.len() >= 3 {
                let w = fit_loglog(&v);
                wins.push(json!({"a": a, "log2_ell_from": lo, "log2_ell_to": lo + 2.0,
                                 "exponent": w.slope, "ci95": [w.slope_lo, w.slope_hi]}));
            }
            lo += 1.0;
        }
        fits_js.extend(wins);

        // ---- projection vs measured (auto).
        let mut vr = Vec::new();
        for r in &lad_grid {
            let l = r["ell"].as_f64().unwrap();
            if let Some(q) = p_a.iter().find(|q| q.0 == l) {
                vr.push((
                    l,
                    q.1 / r["desc"]["find_expected_by_a"][&ak].as_f64().unwrap(),
                ));
            }
        }
        text.push_str("  projection / measured (desc, auto):");
        for (l, v) in &vr {
            text.push_str(&format!(" {l}:{v:.4}"));
        }
        text.push('\n');
        fits_js.push(json!({"series": "projection_over_measured", "a": a, "values": vr}));

        // ---- ordering table (measured rows only).
        let by = |s: &[(f64, f64)], l: f64| s.iter().find(|p| p.0 == l).map(|p| p.1);
        let mut ells: Vec<u64> = lad_all
            .iter()
            .chain(psi_all.iter())
            .chain(mp_all.iter())
            .map(|r| r["ell"].as_u64().unwrap())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        ells.sort();
        let l_all_d = lad(&lad_all, "desc");
        let l_all_a = lad(&lad_all, "asc");
        let ps_all_d = psi_s(&psi_all, "desc");
        let ps_all_a = psi_s(&psi_all, "asc");
        for &(dir, ld, pd, ma, mc) in &[
            ("desc", &l_all_d, &ps_all_d, &m_am_d_all, &m_co_d_all),
            ("asc", &l_all_a, &ps_all_a, &m_am_a_all, &m_co_a_all),
        ] {
            for &ell in &ells {
                let l = ell as f64;
                let vals = [by(ld, l), by(pd, l), by(ma, l), by(mc, l)];
                let have = vals.iter().filter(|v| v.is_some()).count();
                if have < 2 {
                    continue;
                }
                let mut rank: Vec<(usize, f64)> = vals
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| v.map(|v| (i, v)))
                    .collect();
                rank.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
                order_js.push(json!({"a": a, "dir": dir, "ell": ell,
                    "costs": {ROUTES[0]: vals[0], ROUTES[1]: vals[1], ROUTES[2]: vals[2], ROUTES[3]: vals[3]},
                    "order": rank.iter().map(|x| ROUTES[x.0]).collect::<Vec<_>>(),
                    "all_four_measured": have == 4}));
            }
        }

        // ---- crossover with rho (extrapolation).
        let f_ev = fit_loglog(&sel(&ev_pts(a), 30.0));
        let k_pa = p_a
            .last()
            .map(|q| q.1 / (q.0 * q.0 * q.0.log2()))
            .unwrap_or(f64::NAN);
        for &gm in &[6.0f64, 12.0] {
            for &rb in &[64.0f64, 128.0, 256.0] {
                // rho step: g multiplications and g additions (affine formulas
                // with batched inversions: 6 M and 6 add/sub), additions at a.
                let log2_rho =
                    (gm * (1.0 + a) * (std::f64::consts::PI / 4.0).sqrt()).log2() + rb / 2.0;
                let s = rb / 61.0;
                let log2_lmax = rb / 2.0 + 1.0;
                let rho = 2f64.powf(log2_rho);
                let ev = |x: f64| 2f64.powf(f_ev.eval_log2(x));
                let fitc = |f: Fit| move |x: f64| s * 2f64.powf(f.eval_log2(x)) + ev(x);
                let fitc_lo = |f: Fit, top: f64| {
                    move |x: f64| {
                        let lg = f.icpt + f.slope_lo * x + (f.slope - f.slope_lo) * top.log2();
                        s * 2f64.powf(lg) + ev(x)
                    }
                };
                let asym = |x: f64| s * k_pa * 2f64.powf(2.0 * x) * x + ev(x);
                let row = |name: &str, c: &dyn Fn(f64) -> f64| -> Value {
                    let x = crossover(c, rho, 1.0, 400.0);
                    let note = match x {
                        Some(v) if v > log2_lmax => "beyond ell_max = 2 sqrt(p)",
                        Some(_) => "inside the possible range",
                        None => "no crossover below 2^400",
                    };
                    json!({"model": name, "log2_ell_star": x, "note": note})
                };
                let mut rows: Vec<Value> = vec![
                    row("ladder (a), measured desc fit l>=256", &fitc(f_ld.ols)),
                    row("ladder (a), measured asc fit l>=256", &fitc(f_la.ols)),
                    row("ladder (a'), projected auto fit l>=2^10", &fitc(f_pa10.ols)),
                    row("ladder (a'), projected auto fit l>=2^13", &fitc(f_pa.ols)),
                    row(
                        "ladder (a'), projected auto l>=2^13, exponent at OLS CI low end",
                        &fitc_lo(f_pa.ols, lmax),
                    ),
                    row(
                        "ladder (a'), projected best of karatsuba/auto fit l>=2^13",
                        &fitc(f_pb.ols),
                    ),
                    row("ladder (a'), model l^2 log l calibrated at top", &asym),
                    row("psi + CZ desc (b), measured fit", &fitc(f_sd.ols)),
                    row("psi + gcd asc (b), measured fit", &fitc(f_sa.ols)),
                    row(
                        "Phi amortised desc (c), measured fit 11..103",
                        &fitc(f_am.ols),
                    ),
                    row("Phi cold desc (c), measured fit 11..103", &fitc(f_cd.ols)),
                ];
                if let Some(fnn) = f_pn {
                    rows.push(row(
                        "ladder (a'), projected ntt fit l>=2^13",
                        &fitc(fnn.ols),
                    ));
                }
                cross_js.push(
                    json!({"a": a, "g": gm, "log2_r": rb, "log2_rho_cost": log2_rho,
                                     "log2_ell_max": log2_lmax, "rows": rows}),
                );
            }
        }
        let _ = f_ld30;
    }

    // ---- how many additions per Meq each component has (a = 1 over a = 0
    // is 1 + this ratio).
    let share = |v: &Value| -> f64 {
        (v["A"].as_f64().unwrap() + v["WA"].as_f64().unwrap()) / v["Meq"].as_f64().unwrap()
    };
    let mut share_js: Vec<Value> = Vec::new();
    text.push_str(
        "\n================ additions per M-equivalent, (A + WA) / Meq ================\n",
    );
    for r in lad_grid
        .iter()
        .filter(|r| [37u64, 97, 223, 431, 863].contains(&r["ell"].as_u64().unwrap()))
    {
        let l = r["ell"].as_u64().unwrap();
        let lf = &r["desc"]["ladder"];
        let ev = &r["eval_pt"];
        text.push_str(&format!(
            "  ladder desc l={l}: ladder {:.3}, minpoly {:.3}, eval of one point {:.3}\n",
            share(lf),
            share(&r["desc"]["minpoly"]),
            share(ev)
        ));
        share_js.push(json!({"route": "ladder", "ell": l, "ladder": share(lf),
            "minpoly": share(&r["desc"]["minpoly"]), "eval_pt": share(ev)}));
    }
    for r in psi_grid
        .iter()
        .filter(|r| [37u64, 97, 131].contains(&r["ell"].as_u64().unwrap()))
    {
        let l = r["ell"].as_u64().unwrap();
        text.push_str(&format!(
            "  psi desc l={l}: psi_l recurrences {:.3}, whole find {:.3}; asc find {:.3}\n",
            share(&r["desc"]["psi"]),
            share(&r["desc"]["find"]),
            share(&r["asc"]["find"])
        ));
        share_js.push(
            json!({"route": "psi", "ell": l, "psi_recurrences": share(&r["desc"]["psi"]),
            "find_desc": share(&r["desc"]["find"]), "find_asc": share(&r["asc"]["find"])}),
        );
    }
    for r in mp_all
        .iter()
        .filter(|r| [37u64, 67, 103, 131].contains(&r["ell"].as_u64().unwrap()))
    {
        let l = r["ell"].as_u64().unwrap();
        text.push_str(&format!(
            "  Phi l={l}: construction {:.3}, evaluation {:.3}, roots {:.3}, Elkies {:.3}\n",
            share(&r["build"]),
            share(&r["desc"]["eval"]),
            share(&r["desc"]["roots"]),
            share(&r["desc"]["elkies"])
        ));
        share_js.push(
            json!({"route": "phi", "ell": l, "build": share(&r["build"]),
            "eval": share(&r["desc"]["eval"]), "roots": share(&r["desc"]["roots"]),
            "elkies": share(&r["desc"]["elkies"])}),
        );
    }
    for r in proj
        .iter()
        .filter(|r| [1031u64, 8209, 65537].contains(&r["ell"].as_u64().unwrap()))
    {
        let l = r["ell"].as_u64().unwrap();
        for arm in ["karatsuba", "auto"] {
            if let Some(x) = r["arms"].get(arm) {
                text.push_str(&format!(
                    "  projected ladder l={l} {arm}: xDBL {:.3}, xADD {:.3}\n",
                    share(&x["xdbl"]),
                    share(&x["xadd"])
                ));
                share_js.push(json!({"route": "projected", "ell": l, "arm": arm,
                    "xdbl": share(&x["xdbl"]), "xadd": share(&x["xadd"])}));
            }
        }
    }

    // ---- ordering summaries and the falsifier checks.
    text.push_str("\n================ ordering of routes at measured l ================\n");
    let mut flips: Vec<Value> = Vec::new();
    let mut summary_rows: Vec<Value> = Vec::new();
    for dir in ["desc", "asc"] {
        for acct in ["amortised", "cold"] {
            let want: [usize; 3] = if acct == "amortised" {
                [0, 1, 2]
            } else {
                [0, 1, 3]
            };
            let mut ells: Vec<u64> = order_js
                .iter()
                .filter(|o| o["dir"] == dir)
                .map(|o| o["ell"].as_u64().unwrap())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect();
            ells.sort();
            for ell in ells {
                // cheapest two among the three routes of this accounting,
                // for every weight; only l with all three measured.
                let mut per_a: Vec<(f64, Vec<usize>, f64)> = Vec::new();
                for &a in &WEIGHTS {
                    let o = order_js
                        .iter()
                        .find(|o| o["dir"] == dir && o["ell"] == ell && o["a"].as_f64() == Some(a))
                        .unwrap();
                    let c: Vec<Option<f64>> = want
                        .iter()
                        .map(|&i| o["costs"][ROUTES[i]].as_f64())
                        .collect();
                    if c.iter().any(|v| v.is_none()) {
                        per_a.clear();
                        break;
                    }
                    let mut idx: Vec<usize> = (0..3).collect();
                    idx.sort_by(|&x, &y| c[x].partial_cmp(&c[y]).unwrap());
                    let ratio = c[idx[1]].unwrap() / c[idx[0]].unwrap();
                    let ratio23 = c[idx[2]].unwrap() / c[idx[1]].unwrap();
                    per_a.push((
                        a,
                        idx.iter().map(|&i| want[i]).collect(),
                        ratio.min(ratio23),
                    ));
                }
                if per_a.is_empty() {
                    continue;
                }
                let first2: Vec<usize> = per_a[0].1[..2].to_vec();
                let changed = per_a.iter().any(|x| x.1[..2] != first2[..]);
                let full_changed = per_a.iter().any(|x| x.1 != per_a[0].1);
                let tight = per_a.iter().map(|x| x.2).fold(f64::INFINITY, f64::min);
                let line = format!(
                    "  {dir} {acct:<9} l={ell:<4} order at a=0: {} | cheapest-two changes over a: {changed} | full order changes: {full_changed} | smallest adjacent ratio {tight:.2}\n",
                    per_a[0].1.iter().map(|&i| ROUTES[i]).collect::<Vec<_>>().join(" < ")
                );
                text.push_str(&line);
                summary_rows.push(json!({"dir": dir, "accounting": acct, "ell": ell,
                    "order_at_a0": per_a[0].1.iter().map(|&i| ROUTES[i]).collect::<Vec<_>>(),
                    "order_by_a": per_a.iter().map(|x| json!({"a": x.0, "order": x.1.iter().map(|&i| ROUTES[i]).collect::<Vec<_>>()})).collect::<Vec<_>>(),
                    "cheapest_two_change": changed, "full_order_change": full_changed,
                    "smallest_adjacent_cost_ratio": tight}));
                if changed {
                    flips.push(json!({"dir": dir, "accounting": acct, "ell": ell}));
                }
            }
        }
    }
    // Fitted-curve crossings between route pairs (extrapolated outside the
    // measured range), per weight.
    text.push_str("\n================ fitted pairwise crossings (log2 l where the fits cross) ================\n");
    let mut crossings: Vec<Value> = Vec::new();
    {
        let get = |name: &str, range: &str, ak: &str| -> Fit {
            fits[&(name.to_string(), range.to_string(), ak.to_string())].ols
        };
        let pairs: [(&str, &str, &str, &str, &str, &str); 4] = [
            (
                "ladder find desc (measured)",
                "l>=30",
                "psi + CZ desc (measured)",
                "grid",
                "ladder",
                "psi",
            ),
            (
                "ladder find desc (measured)",
                "l>=30",
                "Phi cold (build+eval+roots+Elkies) desc",
                "11..103",
                "ladder",
                "Phi cold",
            ),
            (
                "ladder find desc (measured)",
                "l>=30",
                "Phi amortised (eval+roots+Elkies) desc",
                "11..103",
                "ladder",
                "Phi amortised",
            ),
            (
                "psi + CZ desc (measured)",
                "grid",
                "Phi cold (build+eval+roots+Elkies) desc",
                "11..103",
                "psi",
                "Phi cold",
            ),
        ];
        for &a in &WEIGHTS {
            let ak = akey(a);
            for (n1, r1, n2, r2, t1, t2) in pairs {
                let (f1, f2) = (get(n1, r1, &ak), get(n2, r2, &ak));
                let ds = f1.slope - f2.slope;
                let x = if ds.abs() < 1e-9 {
                    None
                } else {
                    Some((f2.icpt - f1.icpt) / ds)
                };
                text.push_str(&format!(
                    "  a={a}: {t1} vs {t2}: exponents {:.3} vs {:.3}, fits cross at log2 l = {}\n",
                    f1.slope,
                    f2.slope,
                    x.map(|v| format!("{v:.2}")).unwrap_or("-".into())
                ));
                crossings.push(json!({"a": a, "route_1": t1, "route_2": t2,
                    "exponent_1": f1.slope, "exponent_2": f2.slope, "log2_ell_cross": x}));
            }
        }
    }

    // ---- falsifier on the exponent.
    text.push_str(
        "\n================ exponent falsifier (projected auto, l >= 2^13) ================\n",
    );
    let mut exp_rows = Vec::new();
    let mut exp_falsified = false;
    let mut exp_falsified_best = false;
    for (series_name, primary) in [
        ("projected ladder, auto", true),
        ("projected ladder, best of karatsuba/auto", false),
    ] {
        for &a in &WEIGHTS {
            let f = fits[&(series_name.to_string(), "l>=2^13".to_string(), akey(a))];
            let ols_hit = f.ols.slope_lo < 1.5 || f.ols.slope_hi > 2.3;
            let boot_hit = f.boot_lo < 1.5 || f.boot_hi > 2.3;
            if primary {
                exp_falsified |= ols_hit || boot_hit;
            } else {
                exp_falsified_best |= ols_hit || boot_hit;
            }
            text.push_str(&format!(
                "  {series_name}, a={a}: {}  lower<1.5 or upper>2.3: OLS {ols_hit}, bootstrap {boot_hit}\n",
                ci_str(&f)
            ));
            exp_rows.push(json!({"series": series_name, "primary": primary, "a": a, "exponent": f.ols.slope,
                "ols_ci95": [f.ols.slope_lo, f.ols.slope_hi],
                "bootstrap_ci95": [f.boot_lo, f.boot_hi], "ols_outside_band": ols_hit, "bootstrap_outside_band": boot_hit}));
        }
    }
    text.push_str(&format!(
        "\nOrdering falsifier (cheapest two routes change with a at some measured l): {}\nExponent falsifier (CI lower < 1.5 or upper > 2.3 for some a, either interval): {}\n",
        if flips.is_empty() { "not triggered".to_string() } else { format!("TRIGGERED at {}", serde_json::to_string(&flips).unwrap()) },
        if exp_falsified { "TRIGGERED" } else { "not triggered" }
    ));
    // ---- crossover text (g = 6, all r).
    for c in &cross_js {
        if c["g"].as_f64() != Some(6.0) {
            continue;
        }
        text.push_str(&format!(
            "\ncrossover a={} g=6 r=2^{} (rho = 2^{:.2}, l_max = 2^{}):\n",
            c["a"],
            c["log2_r"],
            c["log2_rho_cost"].as_f64().unwrap(),
            c["log2_ell_max"]
        ));
        for r in c["rows"].as_array().unwrap() {
            text.push_str(&format!(
                "    {:<70} log2 l* = {:>7}  ({})\n",
                r["model"].as_str().unwrap(),
                r["log2_ell_star"]
                    .as_f64()
                    .map(|v| format!("{v:.2}"))
                    .unwrap_or("-".into()),
                r["note"].as_str().unwrap()
            ));
        }
    }
    print!("{text}");
    let out_v = json!({
        "mode": "analyze", "weights": WEIGHTS, "fits": fits_js, "ordering": order_js,
        "ordering_summary": summary_rows, "additions_per_meq": share_js, "ordering_flips": flips, "pairwise_fit_crossings": crossings,
        "exponent_falsifier": {"band": [1.5, 2.3], "rows": exp_rows, "triggered_projected_auto": exp_falsified,
                               "triggered_best_of": exp_falsified_best},
        "crossover": cross_js, "reproduction_of_frozen_r6_4": repro,
        "assumptions": {
            "addition_weight": "every F_p add/sub/neg, every fused schoolbook accumulation and every NTT-prime add/sub costs a M; a in {0, 0.1, 0.3, 1}",
            "rho_step": "g M plus g additions per group operation (affine addition with batched inversion: 6 M and 6 add/sub); cost sqrt(pi r / 4) (g + a g)",
            "logp_scale": "every find cost scaled by log2 r / 61",
            "ell_max": "ell^2 | t^2 - 4p implies ell <= 2 sqrt(p)",
            "phi_cold": "construction by the counted port of the library q-series builder, charged once per edge",
            "bootstrap": "pairs bootstrap of (log2 l, log2 cost), 2000 resamples, percentile 2.5/97.5, seed from the series name",
        },
        "text": text,
    });
    std::fs::write(out, serde_json::to_string_pretty(&out_v).unwrap()).expect("write");
    out_v
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let usage = || -> ! {
        eprintln!(
            "usage: {0} ladder <min_ell> <max_ell> <step_log2> <cap_s> <karatsuba|auto|ntt> <reps> <out>\n       {0} ladder_list <ell,ell,...> <cap_s> <karatsuba|auto|ntt> <reps> <out>\n       {0} project <min_log2> <max_log2> <step_log2> <arms,comma,separated> <out>\n       {0} psi <ell,ell,...> <cap_s> <out>\n       {0} modpoly <min_ell> <max_ell> <cap_s> <lib_check_max_ell> <out>\n       {0} selftest <ell,ell,...> <out>\n       {0} analyze <dir> <r6_4_dir> <out>",
            args[0]
        );
        std::process::exit(2)
    };
    if args.len() < 3 {
        usage();
    }
    let t0 = Instant::now();
    let (mut out, path) = match args[1].as_str() {
        "ladder" if args.len() == 9 => {
            let arm = match args[6].as_str() {
                "karatsuba" => PolyMul::Karatsuba,
                "auto" => PolyMul::Auto,
                "ntt" => PolyMul::Ntt,
                _ => usage(),
            };
            let ells = generic_ells(
                args[2].parse().unwrap(),
                args[3].parse().unwrap(),
                args[4].parse().unwrap(),
            );
            (
                mode_ladder(
                    ells,
                    args[5].parse().unwrap(),
                    arm,
                    args[7].parse().unwrap(),
                    211,
                ),
                &args[8],
            )
        }
        "ladder_list" if args.len() == 7 => {
            let arm = match args[4].as_str() {
                "karatsuba" => PolyMul::Karatsuba,
                "auto" => PolyMul::Auto,
                "ntt" => PolyMul::Ntt,
                _ => usage(),
            };
            let ells: Vec<u64> = args[2].split(',').map(|s| s.parse().unwrap()).collect();
            (
                mode_ladder(
                    ells,
                    args[3].parse().unwrap(),
                    arm,
                    args[5].parse().unwrap(),
                    211,
                ),
                &args[6],
            )
        }
        "project" if args.len() == 7 => (
            mode_project(
                args[2].parse().unwrap(),
                args[3].parse().unwrap(),
                args[4].parse().unwrap(),
                &args[5].split(',').map(String::from).collect::<Vec<_>>(),
            ),
            &args[6],
        ),
        "psi" if args.len() == 5 => {
            let ells: Vec<u64> = args[2].split(',').map(|s| s.parse().unwrap()).collect();
            (mode_psi(&ells, args[3].parse().unwrap()), &args[4])
        }
        "modpoly" if args.len() == 7 => {
            let (lo, hi): (u64, u64) = (args[2].parse().unwrap(), args[3].parse().unwrap());
            let ells: Vec<u64> = (lo.max(11)..=hi).filter(|&l| is_prime_u64(l)).collect();
            (
                mode_modpoly(&ells, args[4].parse().unwrap(), args[5].parse().unwrap()),
                &args[6],
            )
        }
        "selftest" if args.len() == 4 => {
            let ells: Vec<u64> = args[2].split(',').map(|s| s.parse().unwrap()).collect();
            (mode_selftest(&ells), &args[3])
        }
        "analyze" if args.len() == 5 => (mode_analyze(&args[2], &args[3], &args[4]), &args[4]),
        _ => usage(),
    };
    if args[1] == "analyze" {
        return;
    }
    out["argv"] = json!(args);
    out["unit"] = json!("M = F_p mults+squarings (incl. schoolbook/Karatsuba coefficient products); W = word mults mod 62-bit NTT primes, counted as 1 M; I = F_p inversions; Meq = M + W + 91 I (R6.4's unit); A = F_p additions/subtractions/negations incl. fused schoolbook accumulations and the two u128 accumulations per NTT output coefficient; WA = additions/subtractions mod the NTT primes; cost(a) = Meq + a (A + WA)");
    out["field_bits"] = json!(61);
    out["total_wall_s_contended"] = json!(t0.elapsed().as_secs_f64());
    std::fs::write(path, serde_json::to_string_pretty(&out).unwrap()).expect("write output");
}
