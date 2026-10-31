//! Round 6, lead R6.4: the vertical margin, phase two.
//!
//! Round-4 lead 16.3 measured generic kernel finding (an `x`-only ladder
//! over `F_{p^k}`, `k = (ℓ − 1)/2`) only on descending edges, and R5.3
//! re-measured it with the three-prime NTT to `ℓ = 863`.  This example adds
//! three kernel-finding routes and the ascending direction, all on the same
//! `j = −3375` (`D = −7`) instances at 61-bit `p`:
//!
//! * `ladder`: (a) the `F_{p^k}` ladder (as R5.3; products by the library's
//!   `PolyMul` arm), descending from the crater **and** ascending from the
//!   floor curve it reaches;
//! * `project`: (a′) the ladder's per-step counted cost measured exactly in
//!   `F_{p^k}` for `ℓ` up to `2^16`, multiplied by the exact step count
//!   (derived from measured per-step counts; validated against `ladder`);
//! * `psi`: (b) the division polynomial `ψ_ℓ` by the standard recurrences,
//!   then equal-degree splitting (Cantor–Zassenhaus) on the crater, or
//!   `gcd(ψ_ℓ, x^{p^k} − x)` on the floor;
//! * `modpoly`: (c) roots of `Φ_ℓ(j, Y)` with the library's `Φ_ℓ mod p`
//!   (`isogeny_walk::modpoly`, q-series, wall time only) and the library's
//!   Elkies kernel, with the evaluation and the root finding counted;
//! * `analyze`: fits, the ascending/descending comparison and the §17.3
//!   crossover table.
//!
//! Instances.  `make_instance`, `finish_instance` and the generic-instance
//! filter are ported from `examples/r5_3_ntt_sqrt_velu.rs` (itself ported
//! from `examples/r4_16_3_vertical_margin.rs`) unchanged in behaviour: same
//! seeds (`0x1630 ^ ℓ`), same RNG calls, so the same `ℓ` gives the same `p`
//! as rounds 4 and 5; the descending ladder find also repeats R5.3's RNG
//! sequence, so its counts are R5.3's (checked in `analyze`).
//!
//! Unit (R5.3's): `M` = one `F_p` multiplication or squaring including every
//! coefficient product; `W` = one word product modulo a 62-bit NTT prime,
//! converted at `1 W = 1 M`; `I` = one `F_p` inversion at `91 M`;
//! `Meq = M + W + 91 I`.  Additions are not counted.  Wall times are
//! contended (several agents share four cores).
//!
//! Usage:
//!   r6_4_vertical_margin_2 ladder <min_ell> <max_ell> <step_log2> <cap_s> <karatsuba|auto|ntt> <reps> <out.json>
//!   r6_4_vertical_margin_2 project <min_log2> <max_log2> <step_log2> <out.json>
//!   r6_4_vertical_margin_2 psi <ell,ell,...> <cap_s> <out.json>
//!   r6_4_vertical_margin_2 modpoly <max_ell> <cap_s> <out.json>
//!   r6_4_vertical_margin_2 analyze <dir> <r5_3_dir> <round4_dir> <out.json>

use crypto_lib::cryptanalysis::isogeny_eval::montgomery::{xadd, xdbl, FieldArith};
use crypto_lib::cryptanalysis::isogeny_eval::poly::{self, MonicDivisor};
use crypto_lib::cryptanalysis::isogeny_eval::{OpTally, PolyMul, PrimeField, XZ};
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

fn meq_of(t: &OpTally) -> u64 {
    t.mul_equivalents(INV_COST)
}

fn tjs(t: &OpTally) -> Value {
    json!({"M": t.mul, "W": t.word, "I": t.inv, "Meq": meq_of(t)})
}

fn field(p: u64, m: PolyMul) -> Fp {
    PrimeField::counted(p)
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
    fn js(&self, name: &str, range: &str) -> Value {
        json!({"series": name, "range": range, "exponent": self.slope,
               "exponent_ci95": [self.slope_lo, self.slope_hi],
               "log2_constant": self.icpt, "n": self.n, "r2": self.r2})
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

fn per_ell(rows: &[Value], get: impl Fn(&Value) -> Option<f64>) -> Vec<(f64, f64)> {
    let mut m: std::collections::BTreeMap<u64, (f64, f64)> = Default::default();
    for r in rows {
        if let Some(y) = get(r) {
            let e = m.entry(r["ell"].as_u64().unwrap()).or_insert((0.0, 0.0));
            e.0 += y;
            e.1 += 1.0;
        }
    }
    m.into_iter().map(|(l, (s, c))| (l as f64, s / c)).collect()
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
    ladder: OpTally,
    minpoly: OpTally,
    find: OpTally,
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
        let c_find = f.tally();
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
        let ladder_cnt = f.tally().since(c_find);
        let c_mp = f.tally();
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
        let minpoly_cnt = f.tally().since(c_mp);
        let find_cnt = f.tally().since(c_find);
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

fn find_json(lf: &LadderFind) -> Value {
    let per_attempt = meq_of(&lf.ladder) as f64 / lf.attempts_final as f64;
    json!({
        "ladder": tjs(&lf.ladder), "minpoly": tjs(&lf.minpoly), "find": tjs(&lf.find),
        "attempts_total": lf.attempts_total, "attempts_final_find": lf.attempts_final,
        "horizontal_retries": lf.retries, "ladder_per_attempt_Meq": per_attempt,
        "find_expected_Meq": 2.0 * per_attempt + meq_of(&lf.minpoly) as f64,
        "find_wall_s_contended": lf.wall,
    })
}

#[allow(clippy::too_many_arguments)]
fn mode_ladder(
    min_ell: u64,
    max_ell: u64,
    step_log2: f64,
    cap_s: f64,
    arm: PolyMul,
    reps: usize,
    reps_max_ell: u64,
) -> Value {
    let mut rows = Vec::new();
    let t_start = Instant::now();
    for ell in generic_ells(min_ell, max_ell, step_log2) {
        if t_start.elapsed().as_secs_f64() > cap_s {
            eprintln!("ladder: time cap reached before ℓ={ell}");
            break;
        }
        let (mut rng, inst, n_e, n_t, ve, vt, rejected) = generic_instance(ell);
        let k = ((ell - 1) / 2) as usize;
        let f = field(inst.p, arm);
        let c_setup = f.tally();
        let (fk, cands) = find_irreducible(&f, k, &mut rng);
        let setup = f.tally().since(c_setup);
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
            "setup": tjs(&setup), "cofactor_bits": m.bits(),
            "desc": dj, "asc": aj, "crater_j": inst.jcrater, "floor_j": j_floor,
            "floor_A": a_floor, "asc_codomain_j": j_up, "asc_reaches_crater": asc_ok,
            "rep_desc_find_Meq": rep_desc, "rep_asc_find_Meq": rep_asc, "checks": checks,
        }));
    }
    json!({"mode": "ladder", "arm": format!("{arm:?}"), "rows": rows})
}

// ---------------------------------------------------------------------------
// Arm (a′): measured per-step ladder cost, projected to the exact step count.

fn mode_project(min_log2: f64, max_log2: f64, step_log2: f64) -> Value {
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
            let c0 = f.tally();
            let _ = xdbl(&fk, &pp, &a24k);
            let dbl = f.tally().since(c0);
            let c0 = f.tally();
            let _ = xadd(&fk, &pp, &qq, &dd);
            let add = f.tally().since(c0);
            let step = meq_of(&dbl) + meq_of(&add);
            let per_attempt = (cof_bits + ell_bits) as f64 * step as f64;
            let ext_mul = {
                let c0 = f.tally();
                let _ = FieldArith::mul(&fk, &pp.x, &qq.x);
                meq_of(&f.tally().since(c0))
            };
            let seq: Vec<u64> = (0..2 * k).map(|_| rng.gen_range(0..inst.p)).collect();
            let c0 = f.tally();
            let _ = berlekamp_massey(&f, &seq);
            let bm = meq_of(&f.tally().since(c0)) as f64;
            let c0 = f.tally();
            let _ = fk.inv(&pp.z);
            let invk = meq_of(&f.tally().since(c0)) as f64;
            let minpoly = invk + (2 * k) as f64 * ext_mul as f64 + (2 * k * k) as f64 + bm;
            arms.insert(
                name.into(),
                json!({"xdbl": tjs(&dbl), "xadd": tjs(&add), "step_Meq": step,
                       "ext_mul_Meq": ext_mul, "per_attempt_Meq": per_attempt,
                       "minpoly_Meq": minpoly, "bm_Meq": bm, "ext_inv_Meq": invk,
                       "find_expected_Meq": 2.0 * per_attempt + minpoly}),
            );
        }
        eprintln!(
            "project ℓ={ell} k={k} cof_bits={cof_bits} auto exp={:.3e} ntt exp={:.3e} kara exp={:.3e}",
            arms["auto"]["find_expected_Meq"].as_f64().unwrap(),
            arms["ntt"]["find_expected_Meq"].as_f64().unwrap(),
            arms["karatsuba"]["find_expected_Meq"].as_f64().unwrap()
        );
        rows.push(
            json!({"ell": ell, "k": k, "p": inst.p, "cofactor_bits": cof_bits,
                         "instance_filtered": filtered, "arms": arms}),
        );
    }
    json!({"mode": "project", "rows": rows,
           "derivation": "per attempt = (cofactor bits + bits(ell)) x (counted xDBL + xADD in F_(p^k)); minpoly = counted F_(p^k) inversion + 2k counted products + 2k^2 inner-product mults + counted BM; expected find = 2 x per attempt + minpoly (R5.3's definition)"})
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
        let c0 = f.tally();
        let (a4, a6) = weierstrass_of_montgomery(&f, inst.a);
        let psi = psi_monic(&f, a4, a6, ell);
        let c_psi = f.tally().since(c0);
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
        let c_desc = f.tally().since(c0);
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
        let c1 = f.tally();
        let (b4, b6) = weierstrass_of_montgomery(&f, a_floor);
        let psi_f = psi_monic(&f, b4, b6, ell);
        let c_psi_f = f.tally().since(c1);
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
        let c_asc = f.tally().since(c1);
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
// Arm (c): roots of Φ_ℓ(j, Y).

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

fn mode_modpoly(max_ell: u64, cap_s: f64) -> Value {
    let mut rows = Vec::new();
    let t_start = Instant::now();
    let ells: Vec<u64> = (11..=max_ell).filter(|&l| is_prime_u64(l)).collect();
    for ell in ells {
        if t_start.elapsed().as_secs_f64() > cap_s {
            eprintln!("modpoly: time cap reached before ℓ={ell}");
            break;
        }
        let (_, inst, _, _, _, _, _) = generic_instance(ell);
        let mut rng = StdRng::seed_from_u64(0x6405 ^ ell);
        let p = inst.p;
        let f = field(p, PolyMul::Auto);
        let lf = lib_field(p);
        let w0 = Instant::now();
        let phi = modpoly::modular_polynomial(&lf, ell).expect("Φ_ℓ");
        let t_phi = w0.elapsed().as_secs_f64();
        let c: Vec<Vec<u64>> = phi
            .c
            .iter()
            .map(|r| r.iter().map(|v| lf.to_big(v).to_u64().unwrap()).collect())
            .collect();
        let jc = inst.jcrater;
        // Descending.
        let c0 = f.tally();
        let py = eval_phi(&f, &c, jc);
        assert_eq!(py.last(), Some(&1), "Φ_ℓ monic in Y");
        let c_eval = f.tally().since(c0);
        let mut g = rational_part(&f, &py);
        let n_rat = g.len() - 1;
        let mut crater_mult = 0;
        while horner(&f, &g, jc) == 0 {
            g = poly_divexact(&f, &g, &[f.neg(jc), 1]);
            crater_mult += 1;
        }
        let (j_floor, splits) = cz_root(&f, &g, &mut rng);
        let c_desc = f.tally().since(c0);
        assert_eq!(horner(&f, &py, j_floor), 0);
        // Ascending.
        let c1 = f.tally();
        let pf = eval_phi(&f, &c, j_floor);
        let gf = rational_part(&f, &pf);
        let n_rat_f = gf.len() - 1;
        let (j_up, asc_splits) = if n_rat_f == 1 {
            (f.neg(gf[0]), 0)
        } else {
            cz_root(&f, &gf, &mut rng)
        };
        let c_asc = f.tally().since(c1);
        let asc_ok = j_up == jc;
        // Library Elkies kernels for both edges (wall, uncounted).
        let e_crater = Model {
            a: lf.from_i64(-35),
            b: lf.from_i64(-98),
        };
        let jf = lf.from_u64(j_floor);
        let w1 = Instant::now();
        let el_desc = kernel::elkies_isogeny(&lf, &e_crater, &phi, &jf);
        let t_el_desc = w1.elapsed().as_secs_f64();
        // Floor model y² = x³ + 3κx + 2κ, κ = j/(1728 − j).
        let kap = f.mul(j_floor, inv(&f, f.sub(1728, j_floor)));
        let e_floor = Model {
            a: lf.from_u64(f.mul(3, kap)),
            b: lf.from_u64(f.mul(2, kap)),
        };
        let w2 = Instant::now();
        let el_asc = kernel::elkies_isogeny(&lf, &e_floor, &phi, &lf.from_u64(jc));
        let t_el_asc = w2.elapsed().as_secs_f64();
        let jc_of = |r: &Result<kernel::Isogeny, kernel::EdgeError>| -> Value {
            match r {
                Ok(iso) => json!(lf.to_big(&iso.codomain.j(&lf).unwrap()).to_u64()),
                Err(e) => json!(format!("{e:?}")),
            }
        };
        eprintln!(
            "modpoly ℓ={ell} Φ {t_phi:.2}s desc={} asc={} (rational roots {n_rat}/{n_rat_f}) asc_ok={asc_ok} elkies {}/{}",
            meq_of(&c_desc),
            meq_of(&c_asc),
            el_desc.is_ok(),
            el_asc.is_ok()
        );
        rows.push(json!({
            "ell": ell, "p": p, "chi_m7": chi_minus7(ell),
            "phi_mod_p_wall_s_contended": t_phi, "phi_checks_passed": phi.checks_passed,
            "desc": {"eval": tjs(&c_eval), "find": tjs(&c_desc), "rational_roots": n_rat,
                     "crater_root_multiplicity": crater_mult, "cz_splits": splits,
                     "elkies_ok": el_desc.is_ok(), "elkies_codomain_j": jc_of(&el_desc),
                     "elkies_wall_s_contended": t_el_desc},
            "asc": {"find": tjs(&c_asc), "rational_roots": n_rat_f, "cz_splits": asc_splits,
                    "elkies_ok": el_asc.is_ok(), "elkies_codomain_j": jc_of(&el_asc),
                    "elkies_wall_s_contended": t_el_asc},
            "crater_j": jc, "floor_j": j_floor, "asc_reaches_crater": asc_ok,
        }));
    }
    json!({"mode": "modpoly", "rows": rows,
           "counted": "evaluation of the library's Phi_l mod p at j plus root finding (gcd with Y^p - Y, Cantor-Zassenhaus); Phi_l mod p (q-series) and Elkies are library code on a 256-bit field, wall time only",
           "sutherland_crt": "not built"})
}

// ---------------------------------------------------------------------------
// Mode: fits, the ascending/descending comparison, the crossover table.

/// Mean of `log2 y` at the mean of `log2 ℓ` with its 95% t-interval (the
/// level of a fitted line where it is best determined).
fn level_ci(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let f = fit_loglog(pts);
    let n = pts.len() as f64;
    let xs: Vec<f64> = pts.iter().map(|p| p.0.log2()).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1.log2()).collect();
    let mx = xs.iter().sum::<f64>() / n;
    let sse: f64 = xs
        .iter()
        .zip(&ys)
        .map(|(x, y)| (y - f.icpt - f.slope * x).powi(2))
        .sum();
    let s = (sse / (n - 2.0)).sqrt();
    let lvl = f.icpt + f.slope * mx;
    let h = t975(n - 2.0) * s / n.sqrt();
    (mx, lvl, lvl - h, lvl + h)
}

/// Paired comparison: mean of `log2(asc/desc)` over ℓ with a 95%
/// t-interval.
fn paired(desc: &[(f64, f64)], asc: &[(f64, f64)]) -> Value {
    let mut d = Vec::new();
    for (l, y) in desc {
        if let Some((_, z)) = asc.iter().find(|q| q.0 == *l) {
            d.push((z / y).log2());
        }
    }
    let n = d.len() as f64;
    if d.len() < 3 {
        return json!({"n": d.len()});
    }
    let m = d.iter().sum::<f64>() / n;
    let sd = (d.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
    let h = t975(n - 1.0) * sd / n.sqrt();
    json!({"n": d.len(), "mean_log2_ratio_asc_over_desc": m, "ci95": [m - h, m + h],
           "ratio": 2f64.powf(m), "ratio_ci95": [2f64.powf(m - h), 2f64.powf(m + h)],
           "excludes_zero": m - h > 0.0 || m + h < 0.0})
}

fn compare(name: &str, desc: &[(f64, f64)], asc: &[(f64, f64)], text: &mut String) -> Value {
    let fd = fit_loglog(desc);
    let fa = fit_loglog(asc);
    // Common ℓ set for the level comparison.
    let common: Vec<f64> = desc
        .iter()
        .filter(|p| asc.iter().any(|q| q.0 == p.0))
        .map(|p| p.0)
        .collect();
    let dsel: Vec<(f64, f64)> = desc
        .iter()
        .filter(|p| common.contains(&p.0))
        .copied()
        .collect();
    let asel: Vec<(f64, f64)> = asc
        .iter()
        .filter(|p| common.contains(&p.0))
        .copied()
        .collect();
    let ld = level_ci(&dsel);
    let la = level_ci(&asel);
    let disjoint_level = ld.3 < la.2 || la.3 < ld.2;
    let disjoint_exp = fd.slope_hi < fa.slope_lo || fa.slope_hi < fd.slope_lo;
    let pr = paired(&dsel, &asel);
    text.push_str(&format!(
        "  {name}: desc exponent {:.3} [{:.3}, {:.3}] | asc {:.3} [{:.3}, {:.3}] (disjoint: {disjoint_exp}); level at log2 ell = {:.2}: desc {:.3} [{:.3}, {:.3}] | asc {:.3} [{:.3}, {:.3}] (disjoint: {disjoint_level}); paired asc/desc = {:.3} [{:.3}, {:.3}] over n={}\n",
        fd.slope, fd.slope_lo, fd.slope_hi, fa.slope, fa.slope_lo, fa.slope_hi,
        ld.0, ld.1, ld.2, ld.3, la.1, la.2, la.3,
        pr["ratio"].as_f64().unwrap_or(f64::NAN),
        pr["ratio_ci95"][0].as_f64().unwrap_or(f64::NAN),
        pr["ratio_ci95"][1].as_f64().unwrap_or(f64::NAN),
        pr["n"]
    ));
    json!({"arm": name, "desc_fit": fd.js("desc", "all"), "asc_fit": fa.js("asc", "all"),
           "exponent_intervals_disjoint": disjoint_exp,
           "level_log2_ell": ld.0, "desc_level": [ld.1, ld.2, ld.3], "asc_level": [la.1, la.2, la.3],
           "level_intervals_disjoint": disjoint_level, "paired": pr})
}

fn mode_analyze(dir: &str, r53: &str, r4: &str, out: &str) -> Value {
    let mut text = String::new();
    let mut fits = Vec::new();
    let push = |fits: &mut Vec<Value>, f: Fit, name: &str, range: &str| {
        fits.push(f.js(name, range));
        f
    };
    // ---- arm (a): measured ladder.
    let lad = rows_of(
        dir,
        &[
            "ladder_auto.json",
            "ladder_auto_hi.json",
            "ladder_auto_top.json",
        ],
    );
    let r53rows = rows_of(r53, &["generic_ntt.json"]);
    let (mut matched, mut same) = (0, 0);
    for r in &lad {
        if let Some(q) = r53rows.iter().find(|q| q["ell"] == r["ell"]) {
            matched += 1;
            if q["p"] == r["p"] && q["ladder"]["Meq"] == r["desc"]["ladder"]["Meq"] {
                same += 1;
            }
        }
    }
    text.push_str(&format!(
        "R5.3 reproduction (descending ladder, Auto): {matched} rows with the same ell, {same} with the same p and the same counted ladder Meq\n"
    ));
    let asc_ok: Vec<&Value> = lad
        .iter()
        .filter(|r| r["asc_reaches_crater"].as_bool() == Some(true))
        .collect();
    text.push_str(&format!(
        "ladder rows {}; ascending reached the crater in {}\n",
        lad.len(),
        asc_ok.len()
    ));
    let g = |rows: &[&Value], dir: &str, key: &str| -> Vec<(f64, f64)> {
        rows.iter()
            .map(|r| (r["ell"].as_f64().unwrap(), r[dir][key].as_f64().unwrap()))
            .collect()
    };
    let lad_all: Vec<&Value> = lad.iter().collect();
    let l_desc = g(&lad_all, "desc", "find_expected_Meq");
    let l_asc = g(&asc_ok, "asc", "find_expected_Meq");
    let _f_ld = push(
        &mut fits,
        fit_loglog(&sel(&l_desc, 30.0)),
        "ladder find expected, desc, Auto (measured)",
        "ell >= 30",
    );
    let f_ld_hi = push(
        &mut fits,
        fit_loglog(&sel(&l_desc, 256.0)),
        "ladder find expected, desc, Auto (measured)",
        "ell >= 256",
    );
    push(
        &mut fits,
        fit_loglog(&sel(&l_asc, 30.0)),
        "ladder find expected, asc, Auto (measured)",
        "ell >= 30",
    );
    let f_la_hi = push(
        &mut fits,
        fit_loglog(&sel(&l_asc, 256.0)),
        "ladder find expected, asc, Auto (measured)",
        "ell >= 256",
    );
    // Realised repeated finds (ℓ ≤ 211): per-ℓ means.
    let rep = |key: &str| -> Vec<(f64, f64)> {
        lad.iter()
            .filter_map(|r| {
                let v = r[key].as_array()?;
                if v.is_empty() {
                    return None;
                }
                let m = v.iter().map(|x| x.as_f64().unwrap()).sum::<f64>() / v.len() as f64;
                Some((r["ell"].as_f64().unwrap(), m))
            })
            .collect()
    };
    let rep_d = rep("rep_desc_find_Meq");
    let rep_a = rep("rep_asc_find_Meq");
    // ---- arm (a′): projection.
    let proj = rows_of(dir, &["project.json"]);
    let pser = |arm: &str| -> Vec<(f64, f64)> {
        proj.iter()
            .map(|r| {
                (
                    r["ell"].as_f64().unwrap(),
                    r["arms"][arm]["find_expected_Meq"].as_f64().unwrap(),
                )
            })
            .collect()
    };
    let (p_k, p_a, p_n) = (pser("karatsuba"), pser("auto"), pser("ntt"));
    let lmax_proj = p_a.last().map(|p| p.0).unwrap_or(0.0);
    for (nm, s) in [("karatsuba", &p_k), ("auto", &p_a), ("ntt", &p_n)] {
        push(
            &mut fits,
            fit_loglog(&sel(s, 30.0)),
            &format!("projected find expected, {nm}"),
            "ell >= 30",
        );
        push(
            &mut fits,
            fit_loglog(&sel(s, 1024.0)),
            &format!("projected find expected, {nm}"),
            "ell >= 2^10",
        );
    }
    let f_pa_top = push(
        &mut fits,
        fit_loglog(&sel(&p_a, 8192.0)),
        "projected find expected, auto",
        "ell >= 2^13",
    );
    let f_pn_top = push(
        &mut fits,
        fit_loglog(&sel(&p_n, 8192.0)),
        "projected find expected, ntt",
        "ell >= 2^13",
    );
    let f_pa_10 = fit_loglog(&sel(&p_a, 1024.0));
    // Local exponents over 2-octave windows.
    let mut local = Vec::new();
    text.push_str(
        "\nProjected find, local exponents over 2-octave windows (karatsuba | auto | ntt):\n",
    );
    let mut lo = 5.0;
    while lo + 2.0 <= lmax_proj.log2() + 1e-9 {
        let win = |s: &[(f64, f64)]| -> Option<Fit> {
            let v: Vec<(f64, f64)> = s
                .iter()
                .filter(|p| p.0.log2() >= lo - 1e-9 && p.0.log2() <= lo + 2.0 + 0.1)
                .copied()
                .collect();
            if v.len() >= 3 {
                Some(fit_loglog(&v))
            } else {
                None
            }
        };
        if let (Some(a), Some(b), Some(c)) = (win(&p_k), win(&p_a), win(&p_n)) {
            text.push_str(&format!(
                "  log2 ell {lo:>4.1}..{:>4.1}: {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}]\n",
                lo + 2.0, a.slope, a.slope_lo, a.slope_hi, b.slope, b.slope_lo, b.slope_hi, c.slope, c.slope_lo, c.slope_hi
            ));
            local.push(json!({"log2_ell_from": lo, "log2_ell_to": lo + 2.0,
                "karatsuba": [a.slope, a.slope_lo, a.slope_hi], "auto": [b.slope, b.slope_lo, b.slope_hi],
                "ntt": [c.slope, c.slope_lo, c.slope_hi]}));
        }
        lo += 1.0;
    }
    // Validation of the projection against measured descending rows.
    let mut val_ratios = Vec::new();
    for r in &lad {
        let l = r["ell"].as_f64().unwrap();
        if let Some(q) = p_a.iter().find(|q| q.0 == l) {
            val_ratios.push((l, q.1 / r["desc"]["find_expected_Meq"].as_f64().unwrap()));
        }
    }
    text.push_str("\nProjection / measured (desc, Auto, expected find):");
    for (l, v) in &val_ratios {
        text.push_str(&format!(" {l}:{v:.4}"));
    }
    text.push('\n');
    // ---- arm (b): ψ.
    let psi = rows_of(dir, &["psi.json", "psi_hi.json", "psi_top.json"]);
    let psi_ok: Vec<&Value> = psi
        .iter()
        .filter(|r| r["asc_reaches_crater"].as_bool() == Some(true))
        .collect();
    let gp = |rows: &[&Value], d: &str, key: &str| -> Vec<(f64, f64)> {
        rows.iter()
            .map(|r| {
                (
                    r["ell"].as_f64().unwrap(),
                    r[d][key]["Meq"].as_f64().unwrap(),
                )
            })
            .collect()
    };
    let psi_all: Vec<&Value> = psi.iter().collect();
    let s_desc = gp(&psi_all, "desc", "find");
    let s_asc = gp(&psi_ok, "asc", "find");
    let s_psi = gp(&psi_all, "desc", "psi");
    let f_sd = push(
        &mut fits,
        fit_loglog(&sel(&s_desc, 11.0)),
        "psi route find, desc (measured)",
        "all",
    );
    let f_sa = push(
        &mut fits,
        fit_loglog(&sel(&s_asc, 11.0)),
        "psi route find, asc (measured)",
        "all",
    );
    push(
        &mut fits,
        fit_loglog(&sel(&s_desc, 50.0)),
        "psi route find, desc (measured)",
        "ell >= 50",
    );
    push(
        &mut fits,
        fit_loglog(&sel(&s_asc, 50.0)),
        "psi route find, asc (measured)",
        "ell >= 50",
    );
    push(
        &mut fits,
        fit_loglog(&sel(&s_psi, 11.0)),
        "psi_ell by recurrences (measured)",
        "all",
    );
    // ---- arm (c): Φ.
    let mp = rows_of(dir, &["modpoly.json"]);
    let mp_ok: Vec<&Value> = mp
        .iter()
        .filter(|r| r["asc_reaches_crater"].as_bool() == Some(true))
        .collect();
    let mp_all: Vec<&Value> = mp.iter().collect();
    let m_desc = gp(&mp_all, "desc", "find");
    let m_asc = gp(&mp_ok, "asc", "find");
    let m_phi: Vec<(f64, f64)> = mp
        .iter()
        .map(|r| {
            (
                r["ell"].as_f64().unwrap(),
                r["phi_mod_p_wall_s_contended"].as_f64().unwrap(),
            )
        })
        .collect();
    let f_md = push(
        &mut fits,
        fit_loglog(&m_desc),
        "Phi route counted (eval + roots), desc",
        "all",
    );
    push(
        &mut fits,
        fit_loglog(&m_asc),
        "Phi route counted (eval + roots), asc",
        "all",
    );
    push(
        &mut fits,
        fit_loglog(&sel(&m_phi, 20.0)),
        "library Phi_ell mod p (q-series), SECONDS contended",
        "ell >= 20",
    );
    let mp_elk = mp
        .iter()
        .filter(|r| {
            r["desc"]["elkies_ok"].as_bool() == Some(true)
                && r["asc"]["elkies_ok"].as_bool() == Some(true)
        })
        .count();
    text.push_str(&format!(
        "\nPhi rows {}; ascending reached the crater in {}; library Elkies succeeded in both directions in {}\n",
        mp.len(),
        mp_ok.len(),
        mp_elk
    ));

    text.push_str(
        "\nFits: log2(cost) = log2_constant + exponent * log2(ell), OLS, 95% t-interval\n",
    );
    for f in &fits {
        text.push_str(&format!(
            "  {:<66} {:<10} exponent {:.3} [{:.3}, {:.3}]  log2 c {:.2}  n={} r2={:.4}\n",
            f["series"].as_str().unwrap(),
            f["range"].as_str().unwrap(),
            f["exponent"].as_f64().unwrap(),
            f["exponent_ci95"][0].as_f64().unwrap(),
            f["exponent_ci95"][1].as_f64().unwrap(),
            f["log2_constant"].as_f64().unwrap(),
            f["n"].as_u64().unwrap(),
            f["r2"].as_f64().unwrap()
        ));
    }

    // ---- ascending vs descending.
    text.push_str("\nAscending vs descending at equal ell (counted Meq):\n");
    let mut cmp = vec![compare(
        "ladder (a), expected find, Auto",
        &sel(&l_desc, 30.0),
        &sel(&l_asc, 30.0),
        &mut text,
    )];
    if rep_d.len() >= 3 && rep_a.len() >= 3 {
        cmp.push(compare(
            "ladder (a), realised repeated finds (mean per ell)",
            &rep_d,
            &rep_a,
            &mut text,
        ));
    }
    if s_desc.len() >= 3 && s_asc.len() >= 3 {
        cmp.push(compare("psi (b), find", &s_desc, &s_asc, &mut text));
    }
    if m_desc.len() >= 3 && m_asc.len() >= 3 {
        cmp.push(compare(
            "Phi (c), counted eval + roots",
            &m_desc,
            &m_asc,
            &mut text,
        ));
    }

    // ---- falsifier 1: a generic exponent below 2 with NTT.
    let below2 = |f: &Fit| f.slope_hi < 2.0;
    text.push_str(&format!(
        "\nFalsifier 1 (generic exponent < 2 with NTT): measured Auto desc ell>=256 {:.3} [{:.3}, {:.3}] below 2: {}; projected auto ell>=2^13 {:.3} [{:.3}, {:.3}] below 2: {}; projected ntt ell>=2^13 {:.3} [{:.3}, {:.3}] below 2: {}\n",
        f_ld_hi.slope, f_ld_hi.slope_lo, f_ld_hi.slope_hi, below2(&f_ld_hi),
        f_pa_top.slope, f_pa_top.slope_lo, f_pa_top.slope_hi, below2(&f_pa_top),
        f_pn_top.slope, f_pn_top.slope_lo, f_pn_top.slope_hi, below2(&f_pn_top)
    ));

    // ---- crossover table.
    let r53a = load(&format!("{r53}/analysis.json"));
    let r4gen = load(&format!("{r4}/generic.json"))["rows"]
        .as_array()
        .unwrap()
        .clone();
    let r4evalh = per_ell(&r4gen, |r| {
        r["eval_2pt_from_h"]["M"].as_f64().map(|m| m / 2.0)
    });
    let f_evalh = fit_loglog(&sel(&r4evalh, 30.0));
    let mut table = Vec::new();
    text.push_str("\nDerived (extrapolated) crossover: smallest ell with find+evaluate >= g * sqrt(pi r / 4) F_p mults\n");
    let k_pa = p_a
        .last()
        .map(|q| q.1 / (q.0 * q.0 * q.0.log2()))
        .unwrap_or(f64::NAN);
    for &gm in &[6.0f64, 12.0] {
        for &rb in &[64.0f64, 128.0, 256.0] {
            let log2_rho = (gm * (std::f64::consts::PI / 4.0).sqrt()).log2() + rb / 2.0;
            let s = rb / 61.0;
            let log2_lmax = rb / 2.0 + 1.0;
            let rho = 2f64.powf(log2_rho);
            let ev = |x: f64| 2f64.powf(f_evalh.eval_log2(x)) + 91.0;
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
            let mut rows: Vec<Value> = Vec::new();
            // Carried over from R5.3 (rational kernel, and its generic NTT fit).
            if let Some(t) = r53a["crossover"].as_array().unwrap().iter().find(|t| {
                t["g_mults_per_group_op"].as_f64() == Some(gm) && t["log2_r"].as_f64() == Some(rb)
            }) {
                for r in t["rows"].as_array().unwrap() {
                    let m = r["model"].as_str().unwrap();
                    if m.starts_with("rational") || m.contains("NTT fit (ell>=256, central)") {
                        let mut r = r.clone();
                        r["model"] = json!(format!("[R5.3] {m}"));
                        rows.push(r);
                    }
                }
            }
            rows.push(row(
                "generic (a) ladder desc, measured Auto fit ell>=256",
                &fitc(f_ld_hi),
            ));
            rows.push(row(
                "generic (a) ladder asc, measured Auto fit ell>=256",
                &fitc(f_la_hi),
            ));
            rows.push(row(
                "generic (a') ladder, projected Auto fit ell>=2^10",
                &fitc(f_pa_10),
            ));
            rows.push(row(
                "generic (a') ladder, projected Auto fit ell>=2^13",
                &fitc(f_pa_top),
            ));
            rows.push(row(
                "generic (a') ladder, projected Auto fit ell>=2^13, exponent at CI low end",
                &fitc_lo(f_pa_top, lmax_proj),
            ));
            rows.push(row(
                "generic (a') ladder, projected NTT fit ell>=2^13",
                &fitc(f_pn_top),
            ));
            rows.push(row(
                "generic (a') ladder, model l^2 log l calibrated at top projected Auto",
                &asym,
            ));
            rows.push(row("generic (b) psi + CZ desc, measured fit", &fitc(f_sd)));
            rows.push(row("generic (b) psi + gcd asc, measured fit", &fitc(f_sa)));
            rows.push(row(
                "generic (c) Phi eval+roots desc, LOWER BOUND (Phi_ell mod p not counted)",
                &fitc(f_md),
            ));
            text.push_str(&format!(
                "  g={gm} M/op, r=2^{rb}: rho = 2^{log2_rho:.2} M, ell_max = 2^{log2_lmax:.0}\n"
            ));
            for r in &rows {
                text.push_str(&format!(
                    "    {:<84} log2 ell* = {:>7}  ({})\n",
                    r["model"].as_str().unwrap(),
                    r["log2_ell_star"]
                        .as_f64()
                        .map(|v| format!("{v:.2}"))
                        .unwrap_or("-".into()),
                    r["note"].as_str().unwrap()
                ));
            }
            table.push(
                json!({"g_mults_per_group_op": gm, "log2_r": rb, "log2_rho_mults": log2_rho,
                              "log2_ell_max": log2_lmax, "logp_scale": s, "rows": rows}),
            );
        }
    }
    print!("{text}");
    let out_v = json!({
        "mode": "analyze", "fits": fits, "projected_local_exponents": local,
        "asc_vs_desc": cmp, "crossover": table, "projection_over_measured": val_ratios,
        "r53_reproduction": {"matched": matched, "same_p_and_ladder": same},
        "eval_from_h_fit": f_evalh.js("round4 eval per point from h", "ell >= 30"),
        "assumptions": {
            "rho_group_ops": "sqrt(pi r / 4)", "g": "6 or 12 M per rho step",
            "logp_scale": "every find cost scaled by log2 r / 61 (ladder bits, CZ and Frobenius exponents are linear in log p)",
            "ell_max": "ell^2 | t^2 - 4p implies ell <= 2 sqrt(p)",
            "phi_route": "counted part only: Phi_ell mod p construction (library q-series) not counted, so its row is a lower bound on cost and an upper bound on ell*",
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
            "usage: {0} ladder <min_ell> <max_ell> <step_log2> <cap_s> <karatsuba|auto|ntt> <reps> <out>\n       {0} project <min_log2> <max_log2> <step_log2> <out>\n       {0} psi <ell,ell,...> <cap_s> <out>\n       {0} modpoly <max_ell> <cap_s> <out>\n       {0} analyze <dir> <r5_3_dir> <round4_dir> <out>",
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
            (
                mode_ladder(
                    args[2].parse().unwrap(),
                    args[3].parse().unwrap(),
                    args[4].parse().unwrap(),
                    args[5].parse().unwrap(),
                    arm,
                    args[7].parse().unwrap(),
                    211,
                ),
                &args[8],
            )
        }
        "project" if args.len() == 6 => (
            mode_project(
                args[2].parse().unwrap(),
                args[3].parse().unwrap(),
                args[4].parse().unwrap(),
            ),
            &args[5],
        ),
        "psi" if args.len() == 5 => {
            let ells: Vec<u64> = args[2].split(',').map(|s| s.parse().unwrap()).collect();
            (mode_psi(&ells, args[3].parse().unwrap()), &args[4])
        }
        "modpoly" if args.len() == 5 => (
            mode_modpoly(args[2].parse().unwrap(), args[3].parse().unwrap()),
            &args[4],
        ),
        "analyze" if args.len() == 6 => (
            mode_analyze(&args[2], &args[3], &args[4], &args[5]),
            &args[5],
        ),
        _ => usage(),
    };
    if args[1] == "analyze" {
        return;
    }
    out["argv"] = json!(args);
    out["unit"] = json!("M = F_p mults+squarings (incl. schoolbook/Karatsuba coefficient products); W = word mults mod 62-bit NTT primes, counted as 1 M; I = F_p inversions; Meq = M + W + 91 I");
    out["field_bits"] = json!(61);
    out["total_wall_s_contended"] = json!(t0.elapsed().as_secs_f64());
    std::fs::write(path, serde_json::to_string_pretty(&out).unwrap()).expect("write output");
}
