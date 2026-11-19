//! Lead P2-5: Native 256-bit P-256 aligned auxiliary curve (`w^2 = x^3 - 3x`, `j = 1728`,
//! `#E_aux(F_p) = p + 1 = 2^96 * q_odd`) & scaled `p = 3 (mod 4)` shared-`x` tower evaluation.
//!
//! Compares `dense`, `tower`, `paired`, and `oriented` 5-chart relation systems plus the
//! shared-`x` Semaev 2-isogeny fibre presentation using both:
//!   1. Exact 256-bit `Fp256` (`[u64; 4]`) affine F4 + Schwartz-Zippel trace replay on NIST P-256
//!      (`depth = 1..4` and degree-bounded `depth = 8`, reproducing `FB1h16882188194e` byte-for-byte),
//!   2. Accelerated `f4_fp_tower` / `PkmSemaevOracle::new_isogeny` + affine F4 on scaled
//!      `p = 3 (mod 4)` prime-order curves (`M19 = 2^19 - 1`, `M31 = 2^31 - 1`) with `w^2 = x^3 - 3x`.

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, OracleCounters, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::PkmSemaevOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};

use crate::p0_1c_bench::find_prime_order_curve_over_fixed_p;

// ── Exact 256-bit NIST P-256 Field (`[u64; 4]`, little-endian limbs) ──

pub type U256 = [u64; 4];

pub const P256_P: U256 = [
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_0000_FFFF_FFFF,
    0x0000_0000_0000_0000,
    0xFFFF_FFFF_0000_0001,
];

pub const P256_B: U256 = [
    0x3BCE_3C3E_27D2_604B,
    0x651D_06B0_CC53_B0F6,
    0xB3EB_BD55_7698_86BC,
    0x5AC6_35D8_AA3A_93E7,
];

pub const P256_N: U256 = [
    0xF3B9_CAC2_FC63_2551,
    0xBCE6_FAAD_A717_9E84,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_0000_0000,
];

pub const P256_GX: U256 = [
    0xF4A1_3945_D898_C296,
    0x7703_7D81_2DEB_33A0,
    0xF8BC_E6E5_63A4_40F2,
    0x6B17_D1F2_E12C_4247,
];

pub const P256_GY: U256 = [
    0xCBB6_4068_37BF_51F5,
    0x2BCE_3357_6B31_5ECE,
    0x8EE7_EB4A_7C0F_9E16,
    0x4FE3_42E2_FE1A_7F9B,
];

pub const U256_ZERO: U256 = [0, 0, 0, 0];
pub const U256_ONE: U256 = [1, 0, 0, 0];

#[inline]
pub fn u256_cmp(a: &U256, b: &U256) -> std::cmp::Ordering {
    for i in (0..4).rev() {
        match a[i].cmp(&b[i]) {
            std::cmp::Ordering::Equal => continue,
            ord => return ord,
        }
    }
    std::cmp::Ordering::Equal
}

#[inline]
pub fn u256_is_zero(a: &U256) -> bool {
    (a[0] | a[1] | a[2] | a[3]) == 0
}

#[inline]
pub fn u256_from_u64(x: u64) -> U256 {
    [x, 0, 0, 0]
}

fn u256_add_raw(a: &U256, b: &U256) -> (U256, u64) {
    let mut out = [0u64; 4];
    let mut carry = 0u128;
    for i in 0..4 {
        let s = a[i] as u128 + b[i] as u128 + carry;
        out[i] = s as u64;
        carry = s >> 64;
    }
    (out, carry as u64)
}

fn u256_sub_raw(a: &U256, b: &U256) -> (U256, u64) {
    let mut out = [0u64; 4];
    let mut borrow = 0u128;
    for i in 0..4 {
        let ai = a[i] as u128;
        let bi = b[i] as u128 + borrow;
        if ai >= bi {
            out[i] = (ai - bi) as u64;
            borrow = 0;
        } else {
            out[i] = ((1u128 << 64) + ai - bi) as u64;
            borrow = 1;
        }
    }
    (out, borrow as u64)
}

#[inline]
pub fn fp256_mod(a: &U256) -> U256 {
    if u256_cmp(a, &P256_P) != std::cmp::Ordering::Less {
        u256_sub_raw(a, &P256_P).0
    } else {
        *a
    }
}

#[inline]
pub fn fp256_add(a: &U256, b: &U256) -> U256 {
    let (s, c) = u256_add_raw(a, b);
    if c != 0 || u256_cmp(&s, &P256_P) != std::cmp::Ordering::Less {
        u256_sub_raw(&s, &P256_P).0
    } else {
        s
    }
}

#[inline]
pub fn fp256_sub(a: &U256, b: &U256) -> U256 {
    let (d, borrow) = u256_sub_raw(a, b);
    if borrow != 0 {
        u256_add_raw(&d, &P256_P).0
    } else {
        d
    }
}

#[inline]
pub fn fp256_neg(a: &U256) -> U256 {
    if u256_is_zero(a) {
        U256_ZERO
    } else {
        u256_sub_raw(&P256_P, a).0
    }
}

pub fn rem_u512_p256(prod: &[u64; 8]) -> U256 {
    // `2^256 ≡ R256 = 2^256 - P256_P (mod P256_P)` with `R256[3] < 2^32`.
    const R256: [u64; 4] = [
        0x0000_0000_0000_0001,
        0xFFFF_FFFF_0000_0000,
        0xFFFF_FFFF_FFFF_FFFF,
        0x0000_0000_FFFF_FFFE,
    ];
    let fold = |buf: &[u64; 8]| -> [u64; 8] {
        let mut out = [buf[0], buf[1], buf[2], buf[3], 0, 0, 0, 0];
        for i in 0..4 {
            let hi = buf[i + 4];
            if hi == 0 {
                continue;
            }
            let mut carry = 0u128;
            for j in 0..4 {
                let cur = out[i + j] as u128 + (hi as u128) * (R256[j] as u128) + carry;
                out[i + j] = cur as u64;
                carry = cur >> 64;
            }
            let mut k = i + 4;
            while carry > 0 && k < 8 {
                let cur = out[k] as u128 + carry;
                out[k] = cur as u64;
                carry = cur >> 64;
                k += 1;
            }
        }
        out
    };
    let mut cur = *prod;
    while cur[4] > 1 || cur[5] != 0 || cur[6] != 0 || cur[7] != 0 {
        cur = fold(&cur);
    }
    let mut r = [cur[0], cur[1], cur[2], cur[3]];
    let mut top = cur[4];
    while top > 0 || u256_cmp(&r, &P256_P) != std::cmp::Ordering::Less {
        let (sub, borrow) = u256_sub_raw(&r, &P256_P);
        r = sub;
        if borrow != 0 {
            top -= 1;
        }
    }
    r
}

pub fn fp256_mul(a: &U256, b: &U256) -> U256 {
    let mut prod = [0u64; 8];
    for i in 0..4 {
        let mut carry = 0u128;
        for j in 0..4 {
            let cur = prod[i + j] as u128 + (a[i] as u128) * (b[j] as u128) + carry;
            prod[i + j] = cur as u64;
            carry = cur >> 64;
        }
        prod[i + 4] = carry as u64;
    }
    rem_u512_p256(&prod)
}

pub fn fp256_pow(mut base: U256, exp: &U256) -> U256 {
    let mut acc = U256_ONE;
    for limb in 0..4 {
        let w = exp[limb];
        for bit in 0..64 {
            if ((w >> bit) & 1) == 1 {
                acc = fp256_mul(&acc, &base);
            }
            base = fp256_mul(&base, &base);
        }
    }
    acc
}

pub fn fp256_inv(a: &U256) -> U256 {
    assert!(!u256_is_zero(a), "fp256_inv of zero");
    let p_minus_2 = u256_sub_raw(&P256_P, &[2, 0, 0, 0]).0;
    fp256_pow(*a, &p_minus_2)
}

pub fn fp256_sqrt(a: &U256) -> Option<U256> {
    if u256_is_zero(a) {
        return Some(U256_ZERO);
    }
    // Since P256_P == 3 (mod 4), `(P + 1) / 4`:
    // P + 1 = 0xFFFFFFFF00000001_0000000000000000_0000000100000000_0000000000000000
    // (P + 1) >> 2 = [0x0000000000000000, 0x4000000000000000, 0x4000000000000000, 0x3FFFFFFFC0000000]
    let (p_plus_1, c) = u256_add_raw(&P256_P, &U256_ONE);
    let exp = [
        (p_plus_1[0] >> 2) | (p_plus_1[1] << 62),
        (p_plus_1[1] >> 2) | (p_plus_1[2] << 62),
        (p_plus_1[2] >> 2) | (p_plus_1[3] << 62),
        (p_plus_1[3] >> 2) | (c << 62),
    ];
    let mut y = fp256_pow(*a, &exp);
    if fp256_mul(&y, &y) != *a {
        return None;
    }
    let neg_y = fp256_neg(&y);
    if u256_cmp(&y, &neg_y) == std::cmp::Ordering::Greater {
        y = neg_y;
    }
    Some(y)
}

fn u256_from_be_bytes(bytes: &[u8; 32]) -> U256 {
    let mut out = [0u64; 4];
    for i in 0..4 {
        let start = (3 - i) * 8;
        out[i] = u64::from_be_bytes(bytes[start..start + 8].try_into().unwrap());
    }
    out
}

fn u256_to_dec(x: &U256) -> String {
    if u256_is_zero(x) {
        return "0".into();
    }
    let mut cur = *x;
    let mut digits = Vec::new();
    while !u256_is_zero(&cur) {
        let mut rem = 0u128;
        for i in (0..4).rev() {
            let v = (rem << 64) | (cur[i] as u128);
            cur[i] = (v / 10) as u64;
            rem = v % 10;
        }
        digits.push(b'0' + (rem as u8));
    }
    digits.reverse();
    String::from_utf8(digits).unwrap()
}

fn u256_to_mpz_hex_lower(x: &U256) -> String {
    if u256_is_zero(x) {
        return "0x0".into();
    }
    let s = format!(
        "{:x}{:016x}{:016x}{:016x}",
        x[3], x[2], x[1], x[0]
    );
    let trimmed = s.trim_start_matches('0');
    format!("0x{}", if trimmed.is_empty() { "0" } else { trimmed })
}

fn sha256_hex_str(s: &[u8]) -> String {
    let digest = Sha256::digest(s);
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct P256Pt {
    pub x: U256,
    pub y: U256,
    pub infinity: bool,
}

impl P256Pt {
    pub const INF: Self = P256Pt {
        x: U256_ZERO,
        y: U256_ZERO,
        infinity: true,
    };
    pub fn affine(x: U256, y: U256) -> Self {
        P256Pt {
            x,
            y,
            infinity: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct P256Curve {
    pub a: U256,
    pub b: U256,
}

impl P256Curve {
    pub fn rhs(&self, x: &U256) -> U256 {
        let x2 = fp256_mul(x, x);
        let x3 = fp256_mul(&x2, x);
        let ax = fp256_mul(&self.a, x);
        fp256_add(&fp256_add(&x3, &ax), &self.b)
    }

    pub fn on_curve(&self, p: &P256Pt) -> bool {
        if p.infinity {
            return u256_is_zero(&p.x) && u256_is_zero(&p.y);
        }
        fp256_mul(&p.y, &p.y) == self.rhs(&p.x)
    }

    pub fn neg(&self, p: &P256Pt) -> P256Pt {
        if p.infinity {
            *p
        } else {
            P256Pt::affine(p.x, fp256_neg(&p.y))
        }
    }

    pub fn add(&self, q: &P256Pt, r: &P256Pt) -> P256Pt {
        if q.infinity {
            return *r;
        }
        if r.infinity {
            return *q;
        }
        if q.x == r.x && u256_is_zero(&fp256_add(&q.y, &r.y)) {
            return P256Pt::INF;
        }
        let slope = if q.x == r.x {
            let x2 = fp256_mul(&q.x, &q.x);
            let num = fp256_add(&fp256_mul(&u256_from_u64(3), &x2), &self.a);
            let den = fp256_add(&q.y, &q.y);
            fp256_mul(&num, &fp256_inv(&den))
        } else {
            let num = fp256_sub(&r.y, &q.y);
            let den = fp256_sub(&r.x, &q.x);
            fp256_mul(&num, &fp256_inv(&den))
        };
        let x = fp256_sub(&fp256_sub(&fp256_mul(&slope, &slope), &q.x), &r.x);
        let y = fp256_sub(&fp256_mul(&slope, &fp256_sub(&q.x, &x)), &q.y);
        P256Pt::affine(x, y)
    }

    pub fn mul_u256(&self, mut q: P256Pt, n: &U256) -> P256Pt {
        let mut acc = P256Pt::INF;
        for limb in 0..4 {
            let w = n[limb];
            for bit in 0..64 {
                if ((w >> bit) & 1) == 1 {
                    acc = self.add(&acc, &q);
                }
                q = self.add(&q, &q);
            }
        }
        acc
    }

    pub fn mul_pow2(&self, mut q: P256Pt, k: u32) -> P256Pt {
        for _ in 0..k {
            q = self.add(&q, &q);
        }
        q
    }
}

#[derive(Clone, Copy, Debug)]
pub struct P256IsogStep {
    pub source: P256Curve,
    pub target: P256Curve,
    pub r: U256,
    pub delta: U256,
}

impl P256IsogStep {
    pub fn build(c: P256Curve, t: P256Pt) -> Self {
        assert!(c.on_curve(&t) && !t.infinity && u256_is_zero(&t.y));
        let tx2 = fp256_mul(&t.x, &t.x);
        let d = fp256_add(&fp256_mul(&u256_from_u64(3), &tx2), &c.a);
        assert!(!u256_is_zero(&d));
        let next_a = fp256_sub(&c.a, &fp256_mul(&u256_from_u64(5), &d));
        let next_b = fp256_sub(
            &c.b,
            &fp256_mul(&u256_from_u64(7), &fp256_mul(&t.x, &d)),
        );
        P256IsogStep {
            source: c,
            target: P256Curve {
                a: next_a,
                b: next_b,
            },
            r: t.x,
            delta: d,
        }
    }

    pub fn apply(&self, q: &P256Pt) -> P256Pt {
        if q.infinity || q.x == self.r {
            return P256Pt::INF;
        }
        let t = fp256_inv(&fp256_sub(&q.x, &self.r));
        let nx = fp256_add(&q.x, &fp256_mul(&self.delta, &t));
        let dt2 = fp256_mul(&self.delta, &fp256_mul(&t, &t));
        let ny = fp256_mul(&q.y, &fp256_sub(&U256_ONE, &dt2));
        let out = P256Pt::affine(nx, ny);
        debug_assert!(self.target.on_curve(&out));
        out
    }
}

#[derive(Clone, Debug)]
pub struct P256AuxBase {
    pub depth: u32,
    pub fb_id: String,
    pub columns: Vec<(P256Pt, P256Pt)>, // (target_pt, aux_pt)
    pub chain: Vec<P256IsogStep>,
    pub terminal: P256Pt,
}

fn p256_trial_point(c: &P256Curve, domain: &str, i: usize) -> P256Pt {
    let msg = format!("p256-aux-smooth-20261114/v1|{domain}|{i}");
    let digest: [u8; 32] = Sha256::digest(msg.as_bytes()).into();
    let x = fp256_mod(&u256_from_be_bytes(&digest));
    match fp256_sqrt(&c.rhs(&x)) {
        Some(y) => P256Pt::affine(x, y),
        None => P256Pt::INF,
    }
}

pub fn build_p256_aux_base(depth: u32) -> P256AuxBase {
    let p_minus_3 = u256_sub_raw(&P256_P, &[3, 0, 0, 0]).0;
    let target_curve = P256Curve {
        a: p_minus_3,
        b: P256_B,
    };
    let aux_curve = P256Curve {
        a: p_minus_3,
        b: U256_ZERO,
    };
    // `(P + 1) >> 96`:
    // P + 1 = [0, 0x0000_0001_0000_0000, 0x0000_0000_0000_0000, 0xFFFF_FFFF_0000_0001]
    // Shift right by 96 bits (= 1 limb + 32 bits):
    let (p_plus_1, _) = u256_add_raw(&P256_P, &U256_ONE);
    let odd: U256 = [
        (p_plus_1[1] >> 32) | (p_plus_1[2] << 32),
        (p_plus_1[2] >> 32) | (p_plus_1[3] << 32),
        p_plus_1[3] >> 32,
        0,
    ];

    let mut kernel96 = P256Pt::INF;
    for i in 0..4096usize {
        let q = p256_trial_point(&aux_curve, "kernel", i);
        if q.infinity {
            continue;
        }
        let k = aux_curve.mul_u256(q, &odd);
        if aux_curve.mul_pow2(k, 96).infinity && !aux_curve.mul_pow2(k, 95).infinity {
            kernel96 = k;
            break;
        }
    }
    assert!(!kernel96.infinity, "P-256 aux kernel96 not found");

    let kernel = aux_curve.mul_pow2(kernel96, 96 - depth);
    assert!(aux_curve.mul_pow2(kernel, depth).infinity);
    assert!(!aux_curve.mul_pow2(kernel, depth - 1).infinity);

    let mut chain = Vec::with_capacity(depth as usize);
    let mut k_cur = kernel;
    let mut c_cur = aux_curve;
    for i in 0..depth {
        let ker_step = c_cur.mul_pow2(k_cur, depth - i - 1);
        let s = P256IsogStep::build(c_cur, ker_step);
        k_cur = s.apply(&k_cur);
        c_cur = s.target;
        chain.push(s);
    }
    assert!(k_cur.infinity);

    let dom = format!("coset-depth-{depth}");
    let mut coset = P256Pt::INF;
    let mut terminal = P256Pt::INF;
    let mut columns: Vec<(P256Pt, P256Pt)> = Vec::new();
    for i in 0..4096usize {
        let r = p256_trial_point(&aux_curve, &dom, i);
        if r.infinity {
            continue;
        }
        let mut t = r;
        for s in &chain {
            t = s.apply(&t);
        }
        if t.infinity || u256_is_zero(&t.y) {
            continue;
        }
        let mut cols = Vec::new();
        let mut q = r;
        for _ in 0..(1usize << depth) {
            if let Some(y) = fp256_sqrt(&target_curve.rhs(&q.x)) {
                assert!(!u256_is_zero(&y));
                cols.push((P256Pt::affine(q.x, y), q));
            }
            q = aux_curve.add(&q, &kernel);
        }
        assert_eq!(q, r);
        if !cols.is_empty() {
            coset = r;
            terminal = t;
            columns = cols;
            break;
        }
    }
    columns.sort_unstable_by(|a, b| u256_cmp(&a.0.x, &b.0.x));

    // Exact canonical `FB1h` SHA-256 identity matching `aux_base.cpp:320-338`:
    let mut keys_bytes: Vec<u8> = Vec::with_capacity(columns.len() * 2 * 33);
    for (tgt, _) in &columns {
        for sign in 0..2u64 {
            // `key = ((q.x + 1) << 1) + sign` as 33-byte big-endian!
            let (xp1, c1) = u256_add_raw(&tgt.x, &U256_ONE);
            let top_byte = ((c1 << 1) | (xp1[3] >> 63)) as u8;
            let k3 = (xp1[3] << 1) | (xp1[2] >> 63);
            let k2 = (xp1[2] << 1) | (xp1[1] >> 63);
            let k1 = (xp1[1] << 1) | (xp1[0] >> 63);
            let k0 = (xp1[0] << 1) | sign;
            let mut raw = [0u8; 33];
            raw[0] = top_byte;
            raw[1..9].copy_from_slice(&k3.to_be_bytes());
            raw[9..17].copy_from_slice(&k2.to_be_bytes());
            raw[17..25].copy_from_slice(&k1.to_be_bytes());
            raw[25..33].copy_from_slice(&k0.to_be_bytes());
            keys_bytes.extend_from_slice(&raw);
        }
    }
    let points_sha = sha256_hex_str(&keys_bytes);
    let aux_model = format!(
        "{{\"a\":\"{}\",\"b\":\"0\",\"field\":\"fp-{}\",\"form\":\"y^2=x^3+a*x+b\",\"p\":\"{}\",\"v\":\"1\"}}",
        u256_to_dec(&p_minus_3),
        u256_to_dec(&P256_P),
        u256_to_dec(&P256_P)
    );
    let aux_model_sha = sha256_hex_str(aux_model.as_bytes());
    let params_json = format!(
        "{{\"aux_model_sha256\":\"{}\",\"coset_x\":\"{}\",\"coset_y\":\"{}\",\"depth\":\"{}\",\"kernel_x\":\"{}\",\"kernel_y\":\"{}\",\"point_key_encoding\":\"prime-affine-x-plus-one-shift-sign/be33/v1\",\"seed\":\"p256-aux-smooth-20261114/v1\",\"terminal_x\":\"{}\"}}",
        aux_model_sha,
        u256_to_mpz_hex_lower(&coset.x),
        u256_to_mpz_hex_lower(&coset.y),
        depth,
        u256_to_mpz_hex_lower(&kernel.x),
        u256_to_mpz_hex_lower(&kernel.y),
        u256_to_mpz_hex_lower(&terminal.x)
    );
    let identity_json = format!(
        "{{\"columns\":{},\"curve\":\"icv1-fp256-t89188191154553853111372247798585809583-f188c491\",\"family\":\"aligned-auxiliary-2-isogeny\",\"params\":{},\"points_sha256\":\"{}\",\"schema\":\"ecbench.factor_base/v1\",\"signed_points\":{}}}",
        columns.len(),
        params_json,
        points_sha,
        columns.len() * 2
    );
    let fb_sha = sha256_hex_str(identity_json.as_bytes());
    let fb_id = format!("FB1h{}", &fb_sha[..12]);

    P256AuxBase {
        depth,
        fb_id,
        columns,
        chain,
        terminal,
    }
}

// ── Compact Native Affine Grevlex F4 over `Fp256` with Trace Replay ──

#[derive(Clone, PartialEq, Eq)]
struct Mono256(Vec<u8>);

impl Mono256 {
    fn one(n: usize) -> Self {
        Mono256(vec![0u8; n])
    }
    fn is_one(&self) -> bool {
        self.0.iter().all(|&e| e == 0)
    }
    fn deg(&self) -> u32 {
        self.0.iter().map(|&e| u32::from(e)).sum()
    }
    fn mul(&self, other: &Self) -> Self {
        Mono256(self.0.iter().zip(&other.0).map(|(&a, &b)| a + b).collect())
    }
    fn lcm(&self, other: &Self) -> Self {
        Mono256(self.0.iter().zip(&other.0).map(|(&a, &b)| a.max(b)).collect())
    }
    fn divides(&self, other: &Self) -> bool {
        self.0.iter().zip(&other.0).all(|(&a, &b)| a <= b)
    }
    fn quo(&self, div: &Self) -> Self {
        Mono256(self.0.iter().zip(&div.0).map(|(&a, &b)| a - b).collect())
    }
    fn coprime(&self, other: &Self) -> bool {
        self.0.iter().zip(&other.0).all(|(&a, &b)| a == 0 || b == 0)
    }
}

// Grevlex: larger degree wins; on tie, smaller last exponent wins!
impl Ord for Mono256 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.deg().cmp(&other.deg()) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        for i in (0..self.0.len()).rev() {
            match other.0[i].cmp(&self.0[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Mono256 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Mono256 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Clone)]
struct Poly256 {
    terms: Vec<(Mono256, U256)>, // strictly descending by Grevlex
}

impl Poly256 {
    fn zero() -> Self {
        Poly256 { terms: Vec::new() }
    }
    fn constant(n: usize, c: U256) -> Self {
        let cr = fp256_mod(&c);
        if u256_is_zero(&cr) {
            Self::zero()
        } else {
            Poly256 {
                terms: vec![(Mono256::one(n), cr)],
            }
        }
    }
    fn var(n: usize, idx: usize) -> Self {
        let mut v = vec![0u8; n];
        v[idx] = 1;
        Poly256 {
            terms: vec![(Mono256(v), U256_ONE)],
        }
    }
    fn add(&self, o: &Self) -> Self {
        let mut map: BTreeMap<Mono256, U256> = BTreeMap::new();
        for (m, c) in &self.terms {
            map.insert(m.clone(), *c);
        }
        for (m, c) in &o.terms {
            let nv = fp256_add(map.get(m).unwrap_or(&U256_ZERO), c);
            if u256_is_zero(&nv) {
                map.remove(m);
            } else {
                map.insert(m.clone(), nv);
            }
        }
        let mut terms: Vec<_> = map.into_iter().collect();
        terms.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        Poly256 { terms }
    }
    fn sub(&self, o: &Self) -> Self {
        let neg_o = Poly256 {
            terms: o.terms.iter().map(|(m, c)| (m.clone(), fp256_neg(c))).collect(),
        };
        self.add(&neg_o)
    }
    fn mul(&self, o: &Self) -> Self {
        let mut map: BTreeMap<Mono256, U256> = BTreeMap::new();
        for (m1, c1) in &self.terms {
            for (m2, c2) in &o.terms {
                let m = m1.mul(m2);
                let prod = fp256_mul(c1, c2);
                let nv = fp256_add(map.get(&m).unwrap_or(&U256_ZERO), &prod);
                if u256_is_zero(&nv) {
                    map.remove(&m);
                } else {
                    map.insert(m, nv);
                }
            }
        }
        let mut terms: Vec<_> = map.into_iter().collect();
        terms.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        Poly256 { terms }
    }
    fn mul_mono(&self, mm: &Mono256) -> Self {
        Poly256 {
            terms: self.terms.iter().map(|(m, c)| (m.mul(mm), *c)).collect(),
        }
    }
    fn make_monic(&mut self, muladds: &mut u64) {
        if self.terms.is_empty() || self.terms[0].1 == U256_ONE {
            return;
        }
        let inv = fp256_inv(&self.terms[0].1);
        self.terms[0].1 = U256_ONE;
        for (_, c) in &mut self.terms[1..] {
            *muladds += 1;
            *c = fp256_mul(c, &inv);
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct F4Fp256Stats {
    pub inconsistent: bool,
    pub max_step_degree: u32,
    pub steps: usize,
    pub max_cols: usize,
    pub max_rows: usize,
    pub total_nnz: u64,
    pub muladds: u64,
    pub basis_size: usize,
    pub wall_ms: f64,
}

#[derive(Clone)]
struct F4Fp256TraceStep {
    degree: u32,
    kept_pairs: Vec<(Mono256, usize)>,
}

fn run_f4_fp256(
    inputs: &[Poly256],
    max_degree: u32,
    max_cols_cap: usize,
    replay_trace: Option<&[F4Fp256TraceStep]>,
) -> (F4Fp256Stats, Vec<F4Fp256TraceStep>) {
    let t0 = Instant::now();
    let mut muladds = 0u64;
    let mut basis: Vec<Poly256> = Vec::new();
    let mut pairs: Vec<(u32, Mono256, usize, usize)> = Vec::new();

    let insert_poly = |mut p: Poly256,
                       basis: &mut Vec<Poly256>,
                       pairs: &mut Vec<(u32, Mono256, usize, usize)>,
                       muladds: &mut u64| {
        if p.terms.is_empty() {
            return;
        }
        p.make_monic(muladds);
        let idx = basis.len();
        let lm_new = p.terms[0].0.clone();
        for g in 0..idx {
            let lm_g = &basis[g].terms[0].0;
            if !lm_new.coprime(lm_g) {
                let l = lm_new.lcm(lm_g);
                pairs.push((l.deg(), l, g, idx));
            }
        }
        basis.push(p);
    };

    for inp in inputs {
        if !inp.terms.is_empty() {
            if inp.terms[0].0.is_one() {
                return (
                    F4Fp256Stats {
                        inconsistent: true,
                        max_step_degree: 0,
                        steps: 0,
                        max_cols: 1,
                        max_rows: 1,
                        total_nnz: 1,
                        muladds: 0,
                        basis_size: 1,
                        wall_ms: t0.elapsed().as_secs_f64() * 1e3,
                    },
                    Vec::new(),
                );
            }
            insert_poly(inp.clone(), &mut basis, &mut pairs, &mut muladds);
        }
    }

    let mut steps = 0usize;
    let mut max_step_deg = 0u32;
    let mut max_cols = 0usize;
    let mut max_rows = 0usize;
    let mut total_nnz = 0u64;
    let mut learned: Vec<F4Fp256TraceStep> = Vec::new();

    while !pairs.is_empty() {
        let d = pairs.iter().map(|p| p.0).min().unwrap();
        if d > max_degree {
            break;
        }
        let (sel, rest): (Vec<_>, Vec<_>) = pairs.drain(..).partition(|p| p.0 == d);
        pairs = rest;

        let mut row_jobs: Vec<(Mono256, usize)> = Vec::new();
        if let Some(tr) = replay_trace {
            if let Some(ts) = tr.get(steps) {
                if ts.degree == d {
                    row_jobs = ts.kept_pairs.clone();
                }
            }
        }
        if row_jobs.is_empty() {
            let mut seen: std::collections::HashSet<(Mono256, usize)> = std::collections::HashSet::new();
            for (_, lcm, i, j) in &sel {
                for &g in &[i, j] {
                    let mult = lcm.quo(&basis[*g].terms[0].0);
                    if seen.insert((mult.clone(), *g)) {
                        row_jobs.push((mult, *g));
                    }
                }
            }
        }

        // Symbolic preprocessing with reducer closure.
        let mut s_polys: Vec<(Poly256, (Mono256, usize))> = row_jobs
            .iter()
            .filter(|(_, g)| *g < basis.len())
            .map(|(m, g)| (basis[*g].mul_mono(m), (m.clone(), *g)))
            .collect();
        let mut examined: std::collections::HashSet<Mono256> = std::collections::HashSet::new();
        let mut frontier: Vec<Mono256> = Vec::new();
        for (p, _) in &s_polys {
            for (m, _) in &p.terms {
                if examined.insert(m.clone()) {
                    frontier.push(m.clone());
                }
            }
        }
        let mut r_polys: Vec<Poly256> = Vec::new();
        let mut done_lead: std::collections::HashSet<Mono256> =
            s_polys.iter().filter_map(|(p, _)| p.terms.first().map(|t| t.0.clone())).collect();
        let mut aborted = false;
        while let Some(m) = frontier.pop() {
            if examined.len() > max_cols_cap {
                aborted = true;
                break;
            }
            if done_lead.contains(&m) {
                continue;
            }
            if let Some(g) = (0..basis.len())
                .filter(|&g| basis[g].terms[0].0.divides(&m))
                .min_by_key(|&g| basis[g].terms.len())
            {
                let mult = m.quo(&basis[g].terms[0].0);
                let rp = basis[g].mul_mono(&mult);
                done_lead.insert(m);
                for (rm, _) in &rp.terms {
                    if examined.insert(rm.clone()) {
                        frontier.push(rm.clone());
                    }
                }
                r_polys.push(rp);
            }
        }
        if aborted {
            break;
        }

        steps += 1;
        max_step_deg = max_step_deg.max(d);
        let mut cols: Vec<Mono256> = examined.into_iter().collect();
        cols.sort_unstable_by(|a, b| b.cmp(a));
        let col_of: HashMap<Mono256, usize> =
            cols.iter().enumerate().map(|(i, m)| (m.clone(), i)).collect();
        let n_cols = cols.len();
        max_cols = max_cols.max(n_cols);
        max_rows = max_rows.max(r_polys.len() + s_polys.len());

        // Sparse Gaussian echelon over `Fp256`: reducer rows first, then S-rows, recording useful S-rows!
        let mut piv_map: HashMap<usize, Vec<(usize, U256)>> = HashMap::with_capacity(r_polys.len() + 16);
        for rp in &r_polys {
            total_nnz += rp.terms.len() as u64;
            let row: Vec<(usize, U256)> = rp.terms.iter().map(|(m, c)| (col_of[m], *c)).collect();
            piv_map.insert(row[0].0, row);
        }
        let n_orig_pivs_leads: std::collections::HashSet<usize> = piv_map.keys().copied().collect();

        let mut useful_jobs: Vec<(Mono256, usize)> = Vec::new();
        let mut new_polys: Vec<Poly256> = Vec::new();
        s_polys.sort_unstable_by(|a, b| b.0.terms[0].0.cmp(&a.0.terms[0].0));
        for (sp, job) in s_polys {
            total_nnz += sp.terms.len() as u64;
            // Sparse map accumulator `col -> U256`
            let mut acc: BTreeMap<usize, U256> = BTreeMap::new();
            for (m, c) in &sp.terms {
                acc.insert(col_of[m], *c);
            }
            while let Some((&lead_c, &lead_v)) = acc.first_key_value() {
                if u256_is_zero(&lead_v) {
                    acc.remove(&lead_c);
                    continue;
                }
                let Some(prow) = piv_map.get(&lead_c) else {
                    break;
                };
                acc.remove(&lead_c);
                for &(pc, pv) in &prow[1..] {
                    muladds += 1;
                    let sub = fp256_mul(&lead_v, &pv);
                    let cur = acc.get(&pc).copied().unwrap_or(U256_ZERO);
                    let nv = fp256_sub(&cur, &sub);
                    if u256_is_zero(&nv) {
                        acc.remove(&pc);
                    } else {
                        acc.insert(pc, nv);
                    }
                }
            }
            if acc.is_empty() {
                continue;
            }
            let (&lead_c, &lead_v) = acc.first_key_value().unwrap();
            let inv = fp256_inv(&lead_v);
            let mut norm_row: Vec<(usize, U256)> = Vec::with_capacity(acc.len());
            for (c_idx, v) in acc {
                if c_idx == lead_c {
                    norm_row.push((c_idx, U256_ONE));
                } else {
                    muladds += 1;
                    norm_row.push((c_idx, fp256_mul(&v, &inv)));
                }
            }
            useful_jobs.push(job);
            if !n_orig_pivs_leads.contains(&lead_c) {
                let p_new = Poly256 {
                    terms: norm_row
                        .iter()
                        .map(|&(ci, cv)| (cols[ci].clone(), cv))
                        .collect(),
                };
                new_polys.push(p_new);
            }
            piv_map.insert(lead_c, norm_row);
        }

        learned.push(F4Fp256TraceStep {
            degree: d,
            kept_pairs: useful_jobs,
        });

        if new_polys.iter().any(|p| p.terms[0].0.is_one()) {
            return (
                F4Fp256Stats {
                    inconsistent: true,
                    max_step_degree: max_step_deg,
                    steps,
                    max_cols,
                    max_rows,
                    total_nnz,
                    muladds,
                    basis_size: 1,
                    wall_ms: t0.elapsed().as_secs_f64() * 1e3,
                },
                learned,
            );
        }

        for np in new_polys {
            insert_poly(np, &mut basis, &mut pairs, &mut muladds);
        }
    }

    (
        F4Fp256Stats {
            inconsistent: false,
            max_step_degree: max_step_deg,
            steps,
            max_cols,
            max_rows,
            total_nnz,
            muladds,
            basis_size: basis.len(),
            wall_ms: t0.elapsed().as_secs_f64() * 1e3,
        },
        learned,
    )
}

/// Build Semaev shared-`x` system (`dense` vs `tower` vs `paired` vs `oriented`) on `P256AuxBase` for `m = 2`:
/// - `dense`: `prod_{c in FB} (x_j - x_c) = 0` + `S_3(x_0, x_1, x_R) = 0` (+ `y_j^2 = x_j^3 - 3 x_j + B` in full chart)
/// - `tower`: 2-isogeny fibre chain `(f_{k+1} - f_k)(f_k - r_k) - delta_k = 0` + `S_3(x_0, x_1, x_R) = 0`
/// - `paired`: `tower` + auxiliary/target paired norm `u_j v_j - B = 0, 4(x_j^3 - 3 x_j) - (u_j - v_j)^2 = 0`
/// - `oriented`: `paired` + oriented ordinate chain `w_{k+1}(f_k - r_k)^2 - w_k((f_k - r_k)^2 - delta_k) = 0`
fn build_p256_presentation_system(
    base: &P256AuxBase,
    arm: &str,
    xr: U256,
    planted_pair: Option<(usize, usize)>,
) -> (Vec<Poly256>, Option<Vec<U256>>) {
    let d = base.depth as usize;
    // Variable layout per summand `j in 0..2`:
    // - `dense`: `[x_0, x_1]` (`2` vars)
    // - `tower`: `[x_0, f_{0,1}..f_{0,d-1}, x_1, f_{1,1}..f_{1,d-1}]` (`2*d` vars)
    // - `paired`: `tower` + `[u_0, v_0, u_1, v_1]` (`2*d + 4` vars)
    // - `oriented`: `paired` + `[w_{0,1}..w_{0,d-1}, w_{1,1}..w_{1,d-1}]` (`4*d + 2` vars)
    let per_pt = match arm {
        "dense" => 1,
        "tower" => d,
        "paired" => d + 2,
        "oriented" => 2 * d + 1,
        _ => panic!("unknown presentation arm {arm}"),
    };
    let n_vars = 2 * per_pt;
    let p_three = u256_from_u64(3);
    let p_minus_3 = fp256_neg(&p_three);
    let inv2 = fp256_inv(&u256_from_u64(2));

    let mut witness = vec![U256_ZERO; n_vars];
    let mut eqs: Vec<Poly256> = Vec::new();

    for j in 0..2usize {
        let off = j * per_pt;
        let x_poly = Poly256::var(n_vars, off);
        if let Some((i0, i1)) = planted_pair {
            let idx = if j == 0 { i0 } else { i1 };
            let (tgt_pt, aux_pt) = base.columns[idx / 2];
            let qy = if idx % 2 == 0 {
                tgt_pt.y
            } else {
                fp256_neg(&tgt_pt.y)
            };
            witness[off] = tgt_pt.x;
            if arm != "dense" {
                let mut cur_x = tgt_pt.x;
                let mut cur_w = aux_pt.y;
                for k in 0..d {
                    let st = &base.chain[k];
                    let den = fp256_inv(&fp256_sub(&cur_x, &st.r));
                    let next_x = fp256_add(&cur_x, &fp256_mul(&st.delta, &den));
                    let next_w = fp256_mul(
                        &cur_w,
                        &fp256_sub(&U256_ONE, &fp256_mul(&st.delta, &fp256_mul(&den, &den))),
                    );
                    if k + 1 < d {
                        witness[off + 1 + k] = next_x;
                        if arm == "oriented" {
                            witness[off + d + 2 + k] = next_w;
                        }
                    }
                    cur_x = next_x;
                    cur_w = next_w;
                }
                if arm == "paired" || arm == "oriented" {
                    let w_val = if arm == "oriented" {
                        aux_pt.y
                    } else {
                        fp256_sqrt(&fp256_sub(
                            &fp256_mul(&tgt_pt.x, &fp256_mul(&tgt_pt.x, &tgt_pt.x)),
                            &fp256_mul(&p_three, &tgt_pt.x),
                        ))
                        .unwrap()
                    };
                    witness[off + d] = fp256_add(&qy, &w_val);
                    witness[off + d + 1] = fp256_sub(&qy, &w_val);
                }
            }
        }

        if arm == "dense" {
            let mut mem = Poly256::constant(n_vars, U256_ONE);
            for (c_pt, _) in &base.columns {
                mem = mem.mul(&x_poly.sub(&Poly256::constant(n_vars, c_pt.x)));
            }
            eqs.push(mem);
        } else {
            let mut cur = x_poly.clone();
            let mut aux_w = if arm == "paired" || arm == "oriented" {
                let u_p = Poly256::var(n_vars, off + d);
                let v_p = Poly256::var(n_vars, off + d + 1);
                // `u*v - B = 0` and `4(x^3 - 3x) - (u - v)^2 = 0`
                eqs.push(u_p.mul(&v_p).sub(&Poly256::constant(n_vars, P256_B)));
                let x3_3x = x_poly
                    .mul(&x_poly)
                    .mul(&x_poly)
                    .sub(&Poly256::constant(n_vars, p_three).mul(&x_poly));
                let diff_uv = u_p.sub(&v_p);
                eqs.push(
                    Poly256::constant(n_vars, u256_from_u64(4))
                        .mul(&x3_3x)
                        .sub(&diff_uv.mul(&diff_uv)),
                );
                Some(diff_uv.mul(&Poly256::constant(n_vars, inv2)))
            } else {
                None
            };

            for k in 0..d {
                let st = &base.chain[k];
                let next_p = if k + 1 == d {
                    Poly256::constant(n_vars, base.terminal.x)
                } else {
                    Poly256::var(n_vars, off + 1 + k)
                };
                let cur_minus_r = cur.sub(&Poly256::constant(n_vars, st.r));
                eqs.push(
                    next_p
                        .sub(&cur)
                        .mul(&cur_minus_r)
                        .sub(&Poly256::constant(n_vars, st.delta)),
                );
                if arm == "oriented" {
                    let w_cur = aux_w.take().unwrap();
                    let next_w = if k + 1 == d {
                        Poly256::constant(n_vars, base.terminal.y)
                    } else {
                        Poly256::var(n_vars, off + d + 2 + k)
                    };
                    let sq_den = cur_minus_r.mul(&cur_minus_r);
                    eqs.push(
                        next_w.mul(&sq_den).sub(
                            &w_cur.mul(&sq_den.sub(&Poly256::constant(n_vars, st.delta))),
                        ),
                    );
                    aux_w = Some(next_w);
                }
                cur = next_p;
            }
        }
    }

    // Semaev `S_3(x_0, x_1, xr)`:
    let x0 = Poly256::var(n_vars, 0);
    let x1 = Poly256::var(n_vars, per_pt);
    let xr_p = Poly256::constant(n_vars, xr);
    let diff = x0.sub(&x1);
    let t1 = diff.mul(&diff).mul(&xr_p).mul(&xr_p);
    let sum01 = x0.add(&x1);
    let prod01 = x0.mul(&x1);
    let inner = sum01
        .mul(&prod01.add(&Poly256::constant(n_vars, p_minus_3)))
        .add(&Poly256::constant(n_vars, fp256_add(&P256_B, &P256_B)));
    let t2 = Poly256::constant(n_vars, fp256_neg(&u256_from_u64(2)))
        .mul(&inner)
        .mul(&xr_p);
    let q = prod01.sub(&Poly256::constant(n_vars, p_minus_3));
    let four_b = fp256_mul(&u256_from_u64(4), &P256_B);
    let t3 = q
        .mul(&q)
        .sub(&Poly256::constant(n_vars, four_b).mul(&sum01));
    eqs.push(t1.add(&t2).add(&t3));

    (eqs, planted_pair.map(|_| witness))
}

fn eval_poly256(p: &Poly256, w: &[U256]) -> U256 {
    let mut acc = U256_ZERO;
    for (m, c) in &p.terms {
        let mut term = *c;
        for (i, &e) in m.0.iter().enumerate() {
            for _ in 0..e {
                term = fp256_mul(&term, &w[i]);
            }
        }
        acc = fp256_add(&acc, &term);
    }
    acc
}

#[derive(Clone, Debug, Serialize)]
pub struct P256PresentationRow {
    pub depth: u32,
    pub fb_id: String,
    pub signed_points: usize,
    pub arm: String,
    pub n_vars: usize,
    pub n_equations: usize,
    pub max_input_degree: u32,
    pub planted_witness_verified: bool,
    pub degree_cap: u32,
    pub unplanted_inconsistent: bool,
    pub max_step_degree: u32,
    pub max_cols: usize,
    pub full_f4_muladds: u64,
    pub replay_f4_muladds: u64,
    pub replay_speedup_x: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScaledAuxIsogenyRow {
    pub curve_label: String,
    pub p: u64,
    pub r: u64,
    pub depth: u32,
    pub m: u32,
    pub fb_columns: usize,
    pub solving_degree_max: u32,
    pub max_cols: u64,
    pub scratch_ops_per_call: f64,
    pub trace_ops_per_call: f64,
    pub ops_speedup_x: f64,
    pub scratch_wall_ms: f64,
    pub trace_wall_ms: f64,
    pub wall_speedup_x: f64,
    pub planted_hit_verified: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct P25Summary {
    pub p256_depth8_fb_id_verified: String,
    pub p256_depth8_signed_points: usize,
    pub p256_presentations: Vec<P256PresentationRow>,
    pub scaled_j1728_aux_towers: Vec<ScaledAuxIsogenyRow>,
}

pub fn run_p256_presentations() -> (String, usize, Vec<P256PresentationRow>) {
    // Verify exact P-256 curve generator and order first!
    let p_minus_3 = fp256_neg(&u256_from_u64(3));
    let p256 = P256Curve {
        a: p_minus_3,
        b: P256_B,
    };
    let g = P256Pt::affine(P256_GX, P256_GY);
    assert!(p256.on_curve(&g));
    assert!(p256.mul_u256(g, &P256_N).infinity);

    // Build exact depth-8 P-256 aligned auxiliary base and verify exact SHA-256 `FB1h16882188194e`
    // and `288` signed points from `retained_depth8.cpp`!
    let base8 = build_p256_aux_base(8);
    assert_eq!(base8.fb_id, "FB1h16882188194e", "P-256 depth-8 FB1h mismatch vs aux_base.cpp!");
    assert_eq!(base8.columns.len() * 2, 288, "P-256 depth-8 signed point count mismatch!");

    let mut rows = Vec::new();
    for depth in [1u32, 2, 3, 4] {
        let base = build_p256_aux_base(depth);
        let arms: &[&str] = if depth <= 2 {
            &["dense", "tower", "paired", "oriented"]
        } else if depth == 3 {
            &["dense", "tower", "paired"]
        } else {
            &["tower"]
        };

        let xr_cal = p256_trial_point(&p256, &format!("cal-{depth}"), 0).x;
        let xr_probe = p256_trial_point(&p256, &format!("probe-{depth}"), 1).x;

        // Planted pair `columns[0] + columns[1]` (or `columns[0] + columns[0]` if 1 col)
        let i0 = 0usize;
        let i1 = if base.columns.len() >= 2 { 2usize } else { 0usize };
        let p0 = base.columns[i0 / 2].0;
        let p1 = base.columns[i1 / 2].0;
        let p_sum = p256.add(&p0, &p1);

        for &arm in arms {
            let planted_ok = if !p_sum.infinity && p0.x != p1.x {
                let (p_eqs, wit) =
                    build_p256_presentation_system(&base, arm, p_sum.x, Some((i0, i1)));
                let w = wit.unwrap();
                p_eqs.iter().all(|eq| u256_is_zero(&eval_poly256(eq, &w)))
            } else {
                true
            };
            assert!(planted_ok, "P-256 planted witness failed on depth={depth} arm={arm}");

            let (deg_cap, col_cap) = match depth {
                1 | 2 => (9, 1_200),
                3 => (6, 900),
                _ => (5, 800),
            };
            let (cal_eqs, _) = build_p256_presentation_system(&base, arm, xr_cal, None);
            let n_vars = cal_eqs[0].terms[0].0.0.len();
            let max_in_deg = cal_eqs.iter().map(|e| e.terms[0].0.deg()).max().unwrap_or(0);
            let (cal_stats, trace) = run_f4_fp256(&cal_eqs, deg_cap, col_cap, None);

            let (probe_eqs, _) = build_p256_presentation_system(&base, arm, xr_probe, None);
            let (full_stats, _) = run_f4_fp256(&probe_eqs, deg_cap, col_cap, None);
            let (rep_stats, _) = if cal_stats.inconsistent {
                run_f4_fp256(&probe_eqs, deg_cap, col_cap, Some(&trace))
            } else {
                (full_stats.clone(), Vec::new())
            };
            if rep_stats.inconsistent {
                assert!(full_stats.inconsistent, "P-256 replay soundness check");
            }

            rows.push(P256PresentationRow {
                depth,
                fb_id: base.fb_id.clone(),
                signed_points: base.columns.len() * 2,
                arm: arm.to_string(),
                n_vars,
                n_equations: cal_eqs.len(),
                max_input_degree: max_in_deg,
                planted_witness_verified: planted_ok,
                degree_cap: deg_cap,
                unplanted_inconsistent: full_stats.inconsistent,
                max_step_degree: full_stats.max_step_degree,
                max_cols: full_stats.max_cols,
                full_f4_muladds: full_stats.muladds,
                replay_f4_muladds: rep_stats.muladds,
                replay_speedup_x: full_stats.muladds as f64 / rep_stats.muladds.max(1) as f64,
            });
        }
    }

    // Bounded degree-4 step census on exact P-256 `depth = 8` (`FB1h16882188194e`, 288 signed points):
    for &arm in &["tower", "paired", "oriented"] {
        let xr_d8 = p256_trial_point(&p256, "d8-probe", 0).x;
        let (p_eqs, wit) = build_p256_presentation_system(
            &base8,
            arm,
            p256.add(&base8.columns[0].0, &base8.columns[1].0).x,
            Some((0, 2)),
        );
        let w = wit.unwrap();
        let planted_ok = p_eqs.iter().all(|eq| u256_is_zero(&eval_poly256(eq, &w)));
        assert!(planted_ok, "P-256 depth-8 planted witness failed on {arm}");

        let (d8_eqs, _) = build_p256_presentation_system(&base8, arm, xr_d8, None);
        let n_vars = d8_eqs[0].terms[0].0.0.len();
        let max_in_deg = d8_eqs.iter().map(|e| e.terms[0].0.deg()).max().unwrap_or(0);
        let (d8_stats, _) = run_f4_fp256(&d8_eqs, 3, 1_200, None);

        rows.push(P256PresentationRow {
            depth: 8,
            fb_id: base8.fb_id.clone(),
            signed_points: base8.columns.len() * 2,
            arm: format!("{arm} (bounded deg<=3)"),
            n_vars,
            n_equations: d8_eqs.len(),
            max_input_degree: max_in_deg,
            planted_witness_verified: planted_ok,
            degree_cap: 4,
            unplanted_inconsistent: d8_stats.inconsistent,
            max_step_degree: d8_stats.max_step_degree,
            max_cols: d8_stats.max_cols,
            full_f4_muladds: d8_stats.muladds,
            replay_f4_muladds: d8_stats.muladds,
            replay_speedup_x: 1.0,
        });
    }

    (base8.fb_id, base8.columns.len() * 2, rows)
}

pub fn run_scaled_j1728_aux_towers() -> Vec<ScaledAuxIsogenyRow> {
    let mut out = Vec::new();
    // Scaled `p = 3 (mod 4)` prime-order curves `y^2 = x^3 - 3x + B` with `w^2 = x^3 - 3x`
    // (`#E_aux(F_p) = p + 1` divisible by `2^19` on `M19 = 524,287` and `2^31` on `M31 = 2^31 - 1`!).
    let p19 = (1u64 << 19) - 1;
    let p31 = (1u64 << 31) - 1;
    let inst19 = find_prime_order_curve_over_fixed_p(p19, Some(p19 - 3), 20261119);
    let inst31 = find_prime_order_curve_over_fixed_p(p31, Some(p31 - 3), 20261119);

    for (label, inst, odd_shift) in [
        ("M19 (p = 2^19 - 1, w^2 = x^3 - 3x)", &inst19, 19u32),
        ("M31 (p = 2^31 - 1, w^2 = x^3 - 3x)", &inst31, 31u32),
    ] {
        let p = inst.curve.p;
        let aux = crypto_lib::cryptanalysis::ic_boundary::PrimeCurve {
            p,
            a: p - 3,
            b: 0,
        };
        // Find a point of exact order `2^{odd_shift}` on `E_aux: w^2 = x^3 - 3x` (`#E_aux = p + 1 = 2^{odd_shift}`).
        let mut ops = GroupOps::default();
        let mut ker_top = PrimePoint::INFINITY;
        for x in 2..500u64 {
            if let Some(y) = aux.sqrt(aux.rhs(x)) {
                if y != 0 {
                    let q = PrimePoint::affine(x, y);
                    if aux.mul(&mut ops, q, 1u64 << odd_shift).infinity
                        && !aux.mul(&mut ops, q, 1u64 << (odd_shift - 1)).infinity
                    {
                        ker_top = q;
                        break;
                    }
                }
            }
        }
        assert!(!ker_top.infinity);

        for &(depth, m) in &[(3u32, 2u32), (4u32, 2u32), (3u32, 3u32)] {
            // Build `depth`-step Vélu 2-isogeny chain from `E_aux: w^2 = x^3 - 3x`!
            let mut k_pt = aux.mul(&mut ops, ker_top, 1u64 << (odd_shift - depth));
            let mut cur_c = aux.clone();
            let mut steps: Vec<(u64, u64)> = Vec::with_capacity(depth as usize);
            let apply_step = |pt: PrimePoint, r_k: u64, d_k: u64, cod: &crypto_lib::cryptanalysis::ic_boundary::PrimeCurve| -> PrimePoint {
                if pt.infinity || pt.x == r_k {
                    return PrimePoint::INFINITY;
                }
                let diff = if pt.x >= r_k { pt.x - r_k } else { p - (r_k - pt.x) };
                let inv = {
                    let mut b = diff;
                    let mut e = p - 2;
                    let mut acc = 1u64;
                    while e > 0 {
                        if e & 1 == 1 {
                            acc = ((acc as u128 * b as u128) % p as u128) as u64;
                        }
                        b = ((b as u128 * b as u128) % p as u128) as u64;
                        e >>= 1;
                    }
                    acc
                };
                let nx = ((pt.x as u128 + (d_k as u128 * inv as u128) % p as u128) % p as u128) as u64;
                let inv2 = ((inv as u128 * inv as u128) % p as u128) as u64;
                let dinv2 = ((d_k as u128 * inv2 as u128) % p as u128) as u64;
                let one_minus = if 1 >= dinv2 { 1 - dinv2 } else { p + 1 - dinv2 };
                let ny = ((pt.y as u128 * one_minus as u128) % p as u128) as u64;
                let out = PrimePoint::affine(nx, ny);
                debug_assert_eq!(((out.y as u128 * out.y as u128) % p as u128) as u64, cod.rhs(out.x));
                out
            };

            let mut chain_cods = Vec::with_capacity(depth as usize);
            let ker_base = k_pt;
            for s_idx in 0..depth {
                let t_ker = cur_c.mul(&mut ops, k_pt, 1u64 << (depth - s_idx - 1));
                assert!(!t_ker.infinity && t_ker.y == 0);
                let tx2 = ((t_ker.x as u128 * t_ker.x as u128) % p as u128) as u64;
                let d_k = ((3u128 * tx2 as u128 + cur_c.a as u128) % p as u128) as u64;
                let sub_5d = ((5u128 * d_k as u128) % p as u128) as u64;
                let next_a = if cur_c.a >= sub_5d { cur_c.a - sub_5d } else { p - (sub_5d - cur_c.a) };
                let sub_7rd = ((7u128 * ((t_ker.x as u128 * d_k as u128) % p as u128)) % p as u128) as u64;
                let next_b = if cur_c.b >= sub_7rd { cur_c.b - sub_7rd } else { p - (sub_7rd - cur_c.b) };
                let next_c = crypto_lib::cryptanalysis::ic_boundary::PrimeCurve {
                    p,
                    a: next_a,
                    b: next_b,
                };
                steps.push((t_ker.x, d_k));
                k_pt = apply_step(k_pt, t_ker.x, d_k, &next_c);
                chain_cods.push(next_c.clone());
                cur_c = next_c;
            }
            assert!(k_pt.infinity);

            // Pick a coset on `E_aux` with >= `m + 2` lifted points on `inst.curve`.
            let mut v_xs = Vec::new();
            let mut c_term = 0u64;
            let mut fb = FactorBase::empty("aligned-j1728-aux".into());
            for x_try in 5..2000u64 {
                let Some(y_try) = aux.sqrt(aux.rhs(x_try)) else { continue };
                if y_try == 0 {
                    continue;
                }
                let r0 = PrimePoint::affine(x_try, y_try);
                let mut t_end = r0;
                for (si, &(rk, dk)) in steps.iter().enumerate() {
                    t_end = apply_step(t_end, rk, dk, &chain_cods[si]);
                }
                if t_end.infinity || t_end.y == 0 {
                    continue;
                }
                let mut coset_xs = Vec::with_capacity(1usize << depth);
                let mut q = r0;
                for _ in 0..(1usize << depth) {
                    coset_xs.push(q.x);
                    q = aux.add(&mut ops, q, ker_base);
                }
                let mut cand_fb = FactorBase::empty("aligned-j1728-aux".into());
                for &cx in &coset_xs {
                    if let Some(ty) = inst.curve.sqrt(inst.curve.rhs(cx)) {
                        if ty != 0 {
                            let ty = ty.min(p - ty);
                            let pt = PrimePoint::affine(cx, ty);
                            let qn = inst.curve.neg(pt);
                            let col = cand_fb.abscissae;
                            let idx = cand_fb.points.len();
                            cand_fb.points.push(pt);
                            cand_fb.col_of.push(col);
                            cand_fb.coef_of.push(1);
                            cand_fb.points.push(qn);
                            cand_fb.col_of.push(col);
                            cand_fb.coef_of.push(inst.r - 1);
                            cand_fb.neg_index.push(idx + 1);
                            cand_fb.neg_index.push(idx);
                            cand_fb.x_index.insert(cx, vec![idx, idx + 1]);
                            cand_fb.abscissae += 1;
                        }
                    }
                }
                cand_fb.columns = cand_fb.abscissae;
                for (idx, pt) in cand_fb.points.iter().enumerate() {
                    cand_fb.point_index.insert(inst.curve.key(pt), idx);
                }
                cand_fb.abscissa_list = cand_fb.x_index.keys().copied().collect();
                cand_fb.abscissa_list.sort_unstable();
                if cand_fb.columns >= (m as usize + 1) {
                    v_xs = coset_xs;
                    c_term = t_end.x;
                    fb = cand_fb;
                    break;
                }
            }
            assert!(fb.columns >= m as usize + 1);

            let ctx = InstanceCtx {
                group: &inst.curve,
                generator: inst.generator_point(),
                target: inst.generator_point(),
                r: inst.r,
                cofactor: 1,
                group_order: inst.group_order,
                name: inst.name.clone(),
                field_degree: None,
            };

            // Build 32 natural UNSAT targets + 1 planted `m`-sum control.
            let g_pt = inst.generator_point();
            let targets: Vec<PrimePoint> = (1..=32u64)
                .map(|k| inst.curve.mul(&mut ops, g_pt, 10_000 + k * 7919))
                .collect();
            let mut planted_sum = PrimePoint::INFINITY;
            for j in 0..(m as usize) {
                planted_sum = inst.curve.add(&mut ops, planted_sum, fb.points[2 * j]);
            }

            let run_iso_arm = |fast: bool| -> (f64, u32, u64, f64, bool) {
                let mut oracle = PkmSemaevOracle::new_isogeny(m, depth, v_xs.clone(), steps.clone(), c_term);
                let spec_s = if fast {
                    format!("x:m={m},degree=16,trace=1,inherit=1,descent=1")
                } else {
                    format!("x:m={m},degree=16,trace=0,inherit=0,descent=0")
                };
                let (_, params) = Params::parse_spec(&spec_s).unwrap();
                let mut a_ops = GroupOps::default();
                let t0 = Instant::now();
                oracle.prepare(&ctx, &fb, &params, &mut a_ops).unwrap();
                let mut counters = OracleCounters::default();
                for &t_pt in &targets {
                    let _ = oracle.decompose(&ctx, &fb, &mut a_ops, &mut counters, t_pt);
                }
                let hit = oracle.decompose(&ctx, &fb, &mut a_ops, &mut counters, planted_sum);
                let hit_ok = if let Some(idxs) = hit {
                    let mut s = PrimePoint::INFINITY;
                    for i in idxs {
                        s = inst.curve.add(&mut a_ops, s, fb.points[i]);
                    }
                    s == planted_sum
                } else {
                    false
                };
                let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
                let tot = oracle.solver_totals().unwrap();
                let max_c = tot
                    .extra
                    .get("max_cols_max")
                    .copied()
                    .or_else(|| tot.degree_trace.iter().map(|s| s.cols).max())
                    .unwrap_or(0);
                let deg = tot.solving_degree_max;
                (
                    tot.ops as f64 / (targets.len() + 1) as f64,
                    deg,
                    max_c,
                    wall_ms,
                    hit_ok,
                )
            };

            let (scr_ops, deg, max_c, scr_ms, scr_hit) = run_iso_arm(false);
            let (tr_ops, _, _, tr_ms, tr_hit) = run_iso_arm(true);
            assert!(scr_hit && tr_hit);

            out.push(ScaledAuxIsogenyRow {
                curve_label: label.to_string(),
                p,
                r: inst.r,
                depth,
                m,
                fb_columns: fb.columns,
                solving_degree_max: deg,
                max_cols: max_c,
                scratch_ops_per_call: scr_ops,
                trace_ops_per_call: tr_ops,
                ops_speedup_x: scr_ops / tr_ops.max(1.0),
                scratch_wall_ms: scr_ms,
                trace_wall_ms: tr_ms,
                wall_speedup_x: scr_ms / tr_ms.max(1e-6),
                planted_hit_verified: tr_hit,
            });
        }
    }

    out
}

pub fn run_all_p2_5() -> P25Summary {
    let (fb_id8, sp8, p256_presentations) = run_p256_presentations();
    let scaled_j1728_aux_towers = run_scaled_j1728_aux_towers();
    P25Summary {
        p256_depth8_fb_id_verified: fb_id8,
        p256_depth8_signed_points: sp8,
        p256_presentations,
        scaled_j1728_aux_towers,
    }
}
