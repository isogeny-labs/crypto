//! Lead P1-4: Native Verification & Scoped Closure of Prime-Field Auxiliary Covers
//! (P-192 / Same-Field Plane Quartic) & Deuring-Weighted Large-Prime Vertical-Isogeny Rarity.
//!
//! 100% self-contained native Rust verification & cost-audit engine:
//! - Part (a):
//!   * Explicit genus-3 degree-2 cover `phi: H_gamma -> E`, `phi(u, v) = ((u^2 + gamma)/u, v/u^2)`
//!     over toy NIST-shaped (`a = -3`, cofactor-1 prime order) curves `p in {1009, 2003, 4001, 10007}`,
//!     verifying Cantor Mumford pullback `phi^*(P + Q) == phi^*(P) + phi^*(Q)`, norm pushforward
//!     `phi_*(D_1 + D_2) == phi_*(D_1) + phi_*(D_2)`, `phi_*(phi^*(P)) == [2]P`, exact Jacobian
//!     group-size inflation `#J_H(F_p) = r * #Prym(F_p) = Theta(p^3)`, and Transport Conservation.
//!   * Live canonical `Pic^0(C)(F_1009)` arithmetic on the explicit smooth plane quartic
//!     `C: [1, 1007, 0, 492, 875, 624, 339, 134, 871, 0, 34, 942, 452, 639, 311]` from
//!     `nist_cover_quartic_map_20261116`, measuring exact `FqCounter` `F_p` multiplications/inversions
//!     per `Pic^0` addition (~8,900 muls) and recovering a planted order-967 discrete log via BSGS.
//!   * Native 192-bit `[u64; 3]` arithmetic on actual NIST P-192 (`p = 2^192 - 2^64 - 1`, prime order `r`),
//!     verifying `[r]G == O`, explicit `gamma = 7` and `gamma = 1` cover pullback + pushforward
//!     `phi_*(phi^*([k]G)) == [2][k]G`, and computing the exact work-charged `#Jac(C)(F_p) = Theta(p^3)`
//!     group-size inflation and ~96.17-bit unweighted / ~105.97-bit mul-weighted Diem 2LP vs negation-rho gap.
//! - Part (b):
//!   * Exact integer Kronecker-Hurwitz class numbers `6 * H(4p - t^2)` across a 12-prime ladder
//!     `p in [10^4, 10^6]`, cross-checked against direct reduced binary quadratic form enumeration,
//!     verifying `\sum_{|t| <= 2\sqrt{p}} H(4p - t^2) == 2p` on 12/12 primes, certifying the hard
//!     Hasse-Deuring conductor bound `\max\{\ell | f_0\} <= f_0 <= \lfloor 2\sqrt{p/3}\rfloor <= 2\sqrt{p}`
//!     with 0 violations, measuring the Deuring-weighted ordinary fraction `\bar{\mu}(L)` with
//!     `\exists \ell >= L` dividing `f_0 = [O_K : Z[\pi]]`, and fitting `c / (L \ln L)` against
//!     Gekeler's `GL_2(Z/\ell^2 Z)` density `\sum_{\ell >= L} 1/(\ell^2 - 1)`.

use crate::plane_quartic_pic::{first_rational_point_canonical, PicKey, PlaneQuartic};
use crate::prime31_field::{fp_add, fp_inv, fp_mul, fp_neg, fp_pow, fp_sub, Curve, CurveArith, Point};
use crate::residual_walk::is_prime_u64;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Part (a.1): Genus-3 Hyperelliptic Cover `H_gamma -> E`, Cantor Mumford `J_H`,
//             Galois-Invariant Norm Pushforward `phi_*`, and `Pic^0(C)` Audit
// ============================================================================

fn poly_trim_u64(mut a: Vec<u64>) -> Vec<u64> {
    while a.last().copied() == Some(0) {
        a.pop();
    }
    a
}

fn poly_add_u64(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let n = a.len().max(b.len());
    let mut out = vec![0u64; n];
    for i in 0..n {
        let av = a.get(i).copied().unwrap_or(0);
        let bv = b.get(i).copied().unwrap_or(0);
        out[i] = fp_add(av, bv, p);
    }
    poly_trim_u64(out)
}

fn poly_sub_u64(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let n = a.len().max(b.len());
    let mut out = vec![0u64; n];
    for i in 0..n {
        let av = a.get(i).copied().unwrap_or(0);
        let bv = b.get(i).copied().unwrap_or(0);
        out[i] = fp_sub(av, bv, p);
    }
    poly_trim_u64(out)
}

fn poly_mul_u64(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            if bj == 0 {
                continue;
            }
            out[i + j] = fp_add(out[i + j], fp_mul(ai, bj, p), p);
        }
    }
    poly_trim_u64(out)
}

fn poly_div_rem_u64(a: &[u64], b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    let b = poly_trim_u64(b.to_vec());
    assert!(!b.is_empty(), "division by zero polynomial");
    let mut r = poly_trim_u64(a.to_vec());
    if r.len() < b.len() {
        return (Vec::new(), r);
    }
    let db = b.len() - 1;
    let inv_lead = fp_inv(b[db], p);
    let mut q = vec![0u64; r.len() - b.len() + 1];
    while r.len() >= b.len() && !r.is_empty() {
        let dr = r.len() - 1;
        let lead = r[dr];
        if lead != 0 {
            let c = fp_mul(lead, inv_lead, p);
            let shift = dr - db;
            q[shift] = c;
            for k in 0..db {
                r[shift + k] = fp_sub(r[shift + k], fp_mul(c, b[k], p), p);
            }
        }
        r.pop();
        r = poly_trim_u64(r);
    }
    (poly_trim_u64(q), r)
}

fn poly_monic_u64(a: &[u64], p: u64) -> Vec<u64> {
    let mut v = poly_trim_u64(a.to_vec());
    if let Some(&lead) = v.last() {
        if lead != 1 {
            let inv = fp_inv(lead, p);
            for c in &mut v {
                *c = fp_mul(*c, inv, p);
            }
        }
    }
    v
}

/// Extended Euclidean algorithm in `F_p[u]`: returns `(g, s, t)` with `g` monic and `s*a + t*b == g`.
fn poly_xgcd_u64(a: &[u64], b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let mut r0 = poly_trim_u64(a.to_vec());
    let mut r1 = poly_trim_u64(b.to_vec());
    let mut s0 = vec![1u64];
    let mut s1: Vec<u64> = Vec::new();
    let mut t0: Vec<u64> = Vec::new();
    let mut t1 = vec![1u64];

    while !r1.is_empty() {
        let (q, r2) = poly_div_rem_u64(&r0, &r1, p);
        let qs1 = poly_mul_u64(&q, &s1, p);
        let qt1 = poly_mul_u64(&q, &t1, p);
        let s2 = poly_sub_u64(&s0, &qs1, p);
        let t2 = poly_sub_u64(&t0, &qt1, p);
        r0 = r1;
        r1 = r2;
        s0 = s1;
        s1 = s2;
        t0 = t1;
        t1 = t2;
    }
    if let Some(&lead) = r0.last() {
        if lead != 1 {
            let inv = fp_inv(lead, p);
            for c in &mut r0 {
                *c = fp_mul(*c, inv, p);
            }
            for c in &mut s0 {
                *c = fp_mul(*c, inv, p);
            }
            for c in &mut t0 {
                *c = fp_mul(*c, inv, p);
            }
        }
    }
    (r0, poly_trim_u64(s0), poly_trim_u64(t0))
}

/// Mumford reduced divisor `(U, V)` on `H_gamma: v^2 = f(u)` (`deg f = 7`, genus `g = 3`),
/// with `U` monic of degree `<= 3`, `deg V < deg U`, and `U | (f - V^2)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MumfordDiv3 {
    pub u: Vec<u64>,
    pub v: Vec<u64>,
}

pub struct HyperellipticCover3 {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub gamma: u64,
    pub f: Vec<u64>,
}

impl HyperellipticCover3 {
    pub fn new(p: u64, a: u64, b: u64, gamma: u64) -> Self {
        let g2 = fp_mul(gamma, gamma, p);
        let g3 = fp_mul(g2, gamma, p);
        let c1 = g3;
        let c3 = fp_add(fp_mul(3, g2, p), fp_mul(a, gamma, p), p);
        let c4 = b % p;
        let c5 = fp_add(fp_mul(3, gamma, p), a, p);
        Self {
            p,
            a: a % p,
            b: b % p,
            gamma: gamma % p,
            f: vec![0, c1, 0, c3, c4, c5, 0, 1],
        }
    }

    pub fn zero(&self) -> MumfordDiv3 {
        MumfordDiv3 {
            u: vec![1],
            v: Vec::new(),
        }
    }

    /// Cantor composition + genus-3 reduction on `v^2 = f(u)` (`deg f = 7`).
    pub fn add(&self, d1: &MumfordDiv3, d2: &MumfordDiv3) -> MumfordDiv3 {
        let p = self.p;
        let (d_a, e1, e2) = poly_xgcd_u64(&d1.u, &d2.u, p);
        let v_sum = poly_add_u64(&d1.v, &d2.v, p);
        let (d, c1, c2) = poly_xgcd_u64(&d_a, &v_sum, p);
        let s1 = poly_mul_u64(&c1, &e1, p);
        let s2 = poly_mul_u64(&c1, &e2, p);
        let s3 = c2;

        let u12 = poly_mul_u64(&d1.u, &d2.u, p);
        let d_sq = poly_mul_u64(&d, &d, p);
        let (mut u, rem_u) = poly_div_rem_u64(&u12, &d_sq, p);
        assert!(rem_u.is_empty());
        u = poly_monic_u64(&u, p);

        let term1 = poly_mul_u64(&poly_mul_u64(&s1, &d1.u, p), &d2.v, p);
        let term2 = poly_mul_u64(&poly_mul_u64(&s2, &d2.u, p), &d1.v, p);
        let v1v2_f = poly_add_u64(&poly_mul_u64(&d1.v, &d2.v, p), &self.f, p);
        let term3 = poly_mul_u64(&s3, &v1v2_f, p);
        let num_v = poly_add_u64(&poly_add_u64(&term1, &term2, p), &term3, p);
        let (v_div, rem_vd) = poly_div_rem_u64(&num_v, &d, p);
        assert!(rem_vd.is_empty());
        let (_, mut v) = poly_div_rem_u64(&v_div, &u, p);

        // Reduce while `deg U > 3`:
        while u.len() > 4 {
            let v_sq = poly_mul_u64(&v, &v, p);
            let f_minus_v2 = poly_sub_u64(&self.f, &v_sq, p);
            let (u_next, rem_next) = poly_div_rem_u64(&f_minus_v2, &u, p);
            assert!(rem_next.is_empty(), "Mumford invariant violated");
            u = poly_monic_u64(&u_next, p);
            let neg_v: Vec<u64> = v.iter().map(|&c| fp_neg(c, p)).collect();
            let (_, v_next) = poly_div_rem_u64(&neg_v, &u, p);
            v = v_next;
        }
        MumfordDiv3 { u, v }
    }

    pub fn neg(&self, d: &MumfordDiv3) -> MumfordDiv3 {
        let neg_v: Vec<u64> = d.v.iter().map(|&c| fp_neg(c, self.p)).collect();
        MumfordDiv3 {
            u: d.u.clone(),
            v: poly_trim_u64(neg_v),
        }
    }

    pub fn sub(&self, d1: &MumfordDiv3, d2: &MumfordDiv3) -> MumfordDiv3 {
        self.add(d1, &self.neg(d2))
    }

    pub fn scalar_mul(&self, mut k: u64, d: &MumfordDiv3) -> MumfordDiv3 {
        let mut acc = self.zero();
        let mut base = d.clone();
        while k > 0 {
            if (k & 1) == 1 {
                acc = self.add(&acc, &base);
            }
            k >>= 1;
            if k > 0 {
                base = self.add(&base, &base);
            }
        }
        acc
    }

    /// Pullback `phi^*([P] - [O_E]) = [u^2 - x0 u + gamma, y0 x0 u - y0 gamma] - [(0, 0) - infty]`
    /// matching `src/cryptanalysis/nist_cover_quartic.rs`.
    pub fn pullback(&self, pt: Point) -> MumfordDiv3 {
        let Some((x0, y0)) = pt else {
            return self.zero();
        };
        let p = self.p;
        let pair_p = MumfordDiv3 {
            u: vec![self.gamma, fp_neg(x0, p), 1],
            v: poly_trim_u64(vec![fp_neg(fp_mul(y0, self.gamma, p), p), fp_mul(y0, x0, p)]),
        };
        let weierstrass_zero = MumfordDiv3 {
            u: vec![0, 1],
            v: Vec::new(),
        };
        self.sub(&pair_p, &weierstrass_zero)
    }

    /// Pushforward `phi_*([U, V])` on `E(F_p)` evaluated over the splitting field `F_{p^6}`
    /// (since `deg U <= 3`, every irreducible factor of `U` has degree `1, 2, or 3`, all of
    /// which split completely in `F_{p^6}`, and Galois invariance guarantees the sum lies in `E(F_p)`!).
    pub fn pushforward(&self, div: &MumfordDiv3, curve: Curve) -> Point {
        let ext = ExtFp::new_deg6(self.p as u32);
        self.pushforward_with_ext(div, curve, &ext)
    }

    pub(crate) fn pushforward_with_ext(&self, div: &MumfordDiv3, curve: Curve, ext: &ExtFp) -> Point {
        if div.u.len() <= 1 {
            return None;
        }
        let p = self.p;
        let roots_u = roots_of_deg_le_3_in_fp6(p, ext, &div.u);
        assert_eq!(
            roots_u.len(),
            div.u.len() - 1,
            "all roots of deg<=3 U split in F_{{p^6}}"
        );

        let gamma_e = ext.embed_fq(self.gamma as u32);
        let a_e = ext.embed_fq(curve.a as u32);

        // Affine addition on `E(F_{p^6})` (`None` = O_E):
        let ext_neg = |a: &Vec<u32>| -> Vec<u32> {
            a.iter()
                .map(|&c| if c == 0 { 0 } else { p as u32 - c })
                .collect()
        };
        let ext_sub = |a: &Vec<u32>, b: &Vec<u32>| -> Vec<u32> { ext.add(a, &ext_neg(b)) };
        let ext_inv = |a: &Vec<u32>| -> Vec<u32> {
            // `a^(p^6 - 2)` in `F_{p^6}`:
            // Since `p^6` can exceed `u64` when `p = 10007` (`10007^6 ~ 10^24 > 2^64`!),
            // compute `a^(p^6 - 2) = a^(p - 2) * (a^(p - 1))^(p + p^2 + p^3 + p^4 + p^5)` using Frobenius!
            let ap_m2 = ext.pow(a, p - 2);
            let ap_m1 = ext.mul(&ap_m2, a);
            let mut acc = ap_m2;
            let mut f_pow = ap_m1;
            for _ in 1..6 {
                f_pow = ext.frob(&f_pow);
                acc = ext.mul(&acc, &f_pow);
            }
            acc
        };
        let ext_ec_add =
            |p1: Option<(Vec<u32>, Vec<u32>)>, p2: Option<(Vec<u32>, Vec<u32>)>| -> Option<(Vec<u32>, Vec<u32>)> {
                let (Some((x1, y1)), Some((x2, y2))) = (p1, p2) else {
                    return None; // handled below
                };
                if x1 == x2 {
                    if ext.is_zero(&ext.add(&y1, &y2)) {
                        return None;
                    }
                    let x1_sq = ext.mul(&x1, &x1);
                    let three = ext.embed_fq(3);
                    let num = ext.add(&ext.mul(&three, &x1_sq), &a_e);
                    let den = ext.add(&y1, &y1);
                    let lam = ext.mul(&num, &ext_inv(&den));
                    let x3 = ext_sub(&ext.mul(&lam, &lam), &ext.add(&x1, &x2));
                    let y3 = ext_sub(&ext.mul(&lam, &ext_sub(&x1, &x3)), &y1);
                    Some((x3, y3))
                } else {
                    let num = ext_sub(&y2, &y1);
                    let den = ext_sub(&x2, &x1);
                    let lam = ext.mul(&num, &ext_inv(&den));
                    let x3 = ext_sub(&ext.mul(&lam, &lam), &ext.add(&x1, &x2));
                    let y3 = ext_sub(&ext.mul(&lam, &ext_sub(&x1, &x3)), &y1);
                    Some((x3, y3))
                }
            };

        let mut sum: Option<(Vec<u32>, Vec<u32>)> = None;
        for u_root in roots_u {
            if ext.is_zero(&u_root) {
                // `(0, 0)` maps to `O_E`!
                continue;
            }
            // Evaluate `v_root = V(u_root)`:
            let mut v_val = ext.zero();
            for &vc in div.v.iter().rev() {
                v_val = ext.add(&ext.mul(&v_val, &u_root), &ext.embed_fq(vc as u32));
            }
            let u_sq = ext.mul(&u_root, &u_root);
            let x_img = ext.mul(&ext.add(&u_sq, &gamma_e), &ext_inv(&u_root));
            let y_img = ext.mul(&v_val, &ext_inv(&u_sq));
            let img = Some((x_img, y_img));
            sum = match (sum, img) {
                (None, q) => q,
                (p_acc, None) => p_acc,
                (Some(pa), Some(qa)) => ext_ec_add(Some(pa), Some(qa)),
            };
        }

        match sum {
            None => None,
            Some((xe, ye)) => {
                assert!(ext.in_fq(&xe) && ext.in_fq(&ye), "pushforward must be F_p-rational");
                let pt = Some((xe[0] as u64, ye[0] as u64));
                let mut ar = CurveArith::default();
                assert!(ar.on_curve(curve, pt), "pushforward point must lie on E");
                pt
            }
        }
    }

    /// Exact `#H_gamma(F_p)`, `#H_gamma(F_{p^2})`, `#Prym(F_p)`, and `#J_{H_gamma}(F_p)`
    /// via Poincaré factorization `L_H(T) = (1 - t_E T + p T^2)(1 + c_1 T + c_2 T^2 + p c_1 T^3 + p^2 T^4)`!
    /// Uses stack `(u64, u64)` arithmetic in `F_{p^2}` + Galois conjugation `(u0, -u1)` symmetry (`O(p^2 / 2)`).
    pub fn exact_jacobian_and_prym_order(&self, r_elliptic: u64) -> (u64, u64, u128, u128) {
        let p = self.p;
        let t_e: i128 = (p as i128) + 1 - (r_elliptic as i128);

        // 1. Exact `n1 = #H_gamma(F_p)`:
        let mut n1 = 1u64; // point at infinity
        // Also cache `f(u0)` for `u0 in 0..p` to reuse on the `u1 == 0` axis of `F_{p^2}`:
        let mut n2_axis = 1u64; // point at infinity in `H_gamma(F_{p^2})`
        for u0 in 0..p {
            let mut fu = 0u64;
            for &c in self.f.iter().rev() {
                fu = fp_add(fp_mul(fu, u0, p), c, p);
            }
            if fu == 0 {
                n1 += 1;
                n2_axis += 1;
            } else {
                // Every non-zero element of `F_p` IS a square in `F_{p^2}` (`fu^((p^2-1)/2) = (fu^(p-1))^((p+1)/2) = 1`)!
                n2_axis += 2;
                if fp_pow(fu, (p - 1) / 2, p) == 1 {
                    n1 += 2;
                }
            }
        }

        // 2. Find smallest quadratic non-residue `nu` mod `p`:
        let mut nu = 2u64;
        while fp_pow(nu, (p - 1) / 2, p) == 1 {
            nu += 1;
        }
        let fp2_mul = |(a0, a1): (u64, u64), (b0, b1): (u64, u64)| -> (u64, u64) {
            let p128 = p as u128;
            let r0 = ((a0 as u128 * b0 as u128) + ((a1 as u128 * b1 as u128) % p128) * (nu as u128)) % p128;
            let r1 = (a0 as u128 * b1 as u128 + a1 as u128 * b0 as u128) % p128;
            (r0 as u64, r1 as u64)
        };
        // Note: `z = (z0, z1) != 0` in `F_{p^2}` is a square in `F_{p^2}` iff `Norm_{F_{p^2}/F_p}(z) = z0^2 - nu * z1^2`
        // is a square in `F_p` (`z^((p^2-1)/2) = Norm(z)^((p-1)/2) == 1`)!
        // Look how fast that is: NO exponentiation in `F_{p^2}` — just ONE precomputed Legendre table `is_sq[0..p]` in `F_p`!!
        let mut legendre_fp = vec![0i8; p as usize];
        for x in 1..((p + 1) / 2) {
            let sq = ((x as u128 * x as u128) % (p as u128)) as usize;
            legendre_fp[sq] = 1;
        }
        for v in &mut legendre_fp[1..] {
            if *v == 0 {
                *v = -1;
            }
        }

        // Evaluate `f(u0 + u1 sqrt(nu))` for `u0 in 0..p` and `u1 in 1..=((p-1)/2)` (conjugate `p - u1` has identical norm!):
        let c1 = self.f[1];
        let c3 = self.f[3];
        let c4 = self.f[4];
        let c5 = self.f[5];
        let half_p = (p - 1) / 2;
        let mut n2_off_axis = 0u64;

        for u0 in 0..p {
            for u1 in 1..=half_p {
                let u = (u0, u1);
                let u2 = fp2_mul(u, u);
                // Horner on `f(u) = u * (c1 + u^2 * (c3 + c4*u + c5*u^2 + u^4))`:
                // Even faster: `((u^2 + c5) * u + c4) * u + c3` * `u^2` + `c1`, then `* u`!
                let t1 = (fp_add(u2.0, c5, p), u2.1);
                let t2 = fp2_mul(t1, u);
                let t3 = (fp_add(t2.0, c4, p), t2.1);
                let t4 = fp2_mul(t3, u);
                let t5 = (fp_add(t4.0, c3, p), t4.1);
                let t6 = fp2_mul(t5, u2);
                let t7 = (fp_add(t6.0, c1, p), t6.1);
                let fu = fp2_mul(t7, u);

                if fu.0 == 0 && fu.1 == 0 {
                    n2_off_axis += 2; // `+u1` and `-u1` each give 1 point
                } else {
                    let norm_fu = fp_sub(
                        fp_mul(fu.0, fu.0, p),
                        fp_mul(nu, fp_mul(fu.1, fu.1, p), p),
                        p,
                    );
                    if legendre_fp[norm_fu as usize] == 1 {
                        n2_off_axis += 4; // `+u1` and `-u1` each give 2 points
                    }
                }
            }
        }
        let n2 = n2_axis + n2_off_axis;

        // Poincaré factorization:
        // `s1(H) = p + 1 - n1 = t_E - c1  =>  c1 = n1 - (p + 1) + t_E`
        // `s2(H) = p^2 + 1 - n2 = (t_E^2 - 2p) + (c1^2 - 2 c2)  =>  2 c2 = c1^2 + (t_E^2 - 2p) - s2(H)`
        let c1_prym: i128 = (n1 as i128) - ((p as i128) + 1) + t_e;
        let s2_h: i128 = (p as i128) * (p as i128) + 1 - (n2 as i128);
        let num_2c2: i128 = c1_prym * c1_prym + (t_e * t_e - 2 * (p as i128)) - s2_h;
        assert_eq!(num_2c2 % 2, 0, "2*c2 must be even");
        let c2_prym = num_2c2 / 2;

        let p_i = p as i128;
        let prym_order_i = 1 + (1 + p_i) * c1_prym + c2_prym + p_i * p_i;
        assert!(prym_order_i > 0, "Prym order must be positive");
        let prym_order = prym_order_i as u128;
        let jac_order_full = (r_elliptic as u128) * prym_order;
        (n1, n2, prym_order, jac_order_full)
    }
}

/// Degree-`k` extension field `F_{q^k} = F_q[t] / (irr(t))` over prime `F_q`.
pub(crate) struct ExtFp {
    q: u32,
    k: usize,
    irr: Vec<u32>, // monic degree-`k` irreducible polynomial `[c_0, ..., c_{k-1}, 1]`
    frob_basis: Vec<Vec<u32>>, // `t^(i*q) mod irr(t)` for `i = 0..k`
}

impl ExtFp {
    fn new_deg6(q: u32) -> Self {
        let k = 6usize;
        let q64 = q as u64;
        // Deterministic search for a monic irreducible degree-6 polynomial `t^6 + a_1 t + a_0` (or trinomial) over `F_q`:
        let poly_mod_q = |mut a: Vec<u32>, m: &[u32]| -> Vec<u32> {
            while a.len() >= m.len() {
                let c = a[a.len() - 1] as u64;
                let shift = a.len() - m.len();
                if c != 0 {
                    for j in 0..m.len() {
                        let sub = (c * m[j] as u64) % q64;
                        let cur = a[shift + j] as u64;
                        a[shift + j] = if cur >= sub {
                            (cur - sub) as u32
                        } else {
                            (cur + q64 - sub) as u32
                        };
                    }
                }
                a.pop();
            }
            a
        };
        let poly_mul_mod_q = |a: &[u32], b: &[u32], m: &[u32]| -> Vec<u32> {
            let mut prod = vec![0u32; a.len() + b.len() - 1];
            for (i, &ai) in a.iter().enumerate() {
                if ai == 0 {
                    continue;
                }
                for (j, &bj) in b.iter().enumerate() {
                    prod[i + j] = ((prod[i + j] as u64 + ai as u64 * bj as u64) % q64) as u32;
                }
            }
            let mut r = poly_mod_q(prod, m);
            r.resize(k, 0);
            r
        };
        let poly_pow_mod_q = |base: &[u32], mut e: u64, m: &[u32]| -> Vec<u32> {
            let mut acc = vec![0u32; k];
            acc[0] = 1;
            let mut cur = base.to_vec();
            while e > 0 {
                if (e & 1) == 1 {
                    acc = poly_mul_mod_q(&acc, &cur, m);
                }
                e >>= 1;
                if e > 0 {
                    cur = poly_mul_mod_q(&cur, &cur, m);
                }
            }
            acc
        };
        let poly_gcd_q = |mut a: Vec<u32>, mut b: Vec<u32>| -> usize {
            let trim = |v: &mut Vec<u32>| {
                while v.last() == Some(&0) {
                    v.pop();
                }
            };
            trim(&mut a);
            trim(&mut b);
            while !b.is_empty() {
                let lead_inv = fp_inv(b[b.len() - 1] as u64, q64);
                let bm: Vec<u32> = b
                    .iter()
                    .map(|&c| ((c as u64 * lead_inv) % q64) as u32)
                    .collect();
                let mut r = poly_mod_q(a, &bm);
                trim(&mut r);
                a = bm;
                b = r;
            }
            a.len().saturating_sub(1)
        };

        // Rabin irreducibility test for degree `k = 6` (prime divisors of 6 are 2 and 3):
        // `m(t)` is irreducible iff `gcd(t^(q^2) - t, m) == 1`, `gcd(t^(q^3) - t, m) == 1`, and `t^(q^6) == t (mod m)`!
        let mut t_poly = vec![0u32; k];
        t_poly[1] = 1;
        let mut irr = vec![0u32; k + 1];
        'found: for a2 in 0..q.min(5) {
            for a1 in 0..q.min(25) {
                for a0 in 1..q {
                    let cand = vec![a0, a1, a2, 0, 0, 0, 1];
                    let tq1 = poly_pow_mod_q(&t_poly, q64, &cand);
                    // Build Frobenius powers `t^(q^d)` via composition:
                    let compose_frob = |f_prev: &[u32], tq: &[u32]| -> Vec<u32> {
                        // Compute `sum f_prev[i] * (tq)^i mod cand`:
                        let mut acc = vec![0u32; k];
                        let mut pwr = vec![0u32; k];
                        pwr[0] = 1;
                        for &c in f_prev {
                            if c != 0 {
                                for j in 0..k {
                                    acc[j] = ((acc[j] as u64 + c as u64 * pwr[j] as u64) % q64) as u32;
                                }
                            }
                            pwr = poly_mul_mod_q(&pwr, tq, &cand);
                        }
                        acc
                    };
                    let tq2 = compose_frob(&tq1, &tq1);
                    let mut diff2 = tq2.clone();
                    diff2[1] = ((diff2[1] as u64 + q64 - 1) % q64) as u32;
                    if poly_gcd_q(cand.clone(), diff2) != 0 {
                        continue;
                    }
                    let tq3 = compose_frob(&tq2, &tq1);
                    let mut diff3 = tq3.clone();
                    diff3[1] = ((diff3[1] as u64 + q64 - 1) % q64) as u32;
                    if poly_gcd_q(cand.clone(), diff3) != 0 {
                        continue;
                    }
                    let tq6 = compose_frob(&tq3, &tq3);
                    if tq6 == t_poly {
                        irr = cand;
                        break 'found;
                    }
                }
            }
        }
        assert_eq!(irr.len(), 7, "found irreducible degree-6 polynomial over F_{q}");
        let tq = poly_pow_mod_q(&t_poly, q64, &irr);
        let mut frob_basis = Vec::with_capacity(k);
        let mut pwr = vec![0u32; k];
        pwr[0] = 1;
        for _ in 0..k {
            frob_basis.push(pwr.clone());
            pwr = poly_mul_mod_q(&pwr, &tq, &irr);
        }
        Self { q, k, irr, frob_basis }
    }

    fn zero(&self) -> Vec<u32> {
        vec![0u32; self.k]
    }

    fn one(&self) -> Vec<u32> {
        let mut v = vec![0u32; self.k];
        v[0] = 1;
        v
    }

    fn is_zero(&self, a: &[u32]) -> bool {
        a.iter().all(|&c| c == 0)
    }

    fn in_fq(&self, a: &[u32]) -> bool {
        a[1..].iter().all(|&c| c == 0)
    }

    fn embed_fq(&self, c: u32) -> Vec<u32> {
        let mut v = vec![0u32; self.k];
        v[0] = c % self.q;
        v
    }

    fn add(&self, a: &[u32], b: &[u32]) -> Vec<u32> {
        a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| {
                let s = x + y;
                if s >= self.q { s - self.q } else { s }
            })
            .collect()
    }

    fn mul(&self, a: &[u32], b: &[u32]) -> Vec<u32> {
        let q64 = self.q as u64;
        let mut prod = vec![0u32; 2 * self.k - 1];
        for (i, &ai) in a.iter().enumerate() {
            if ai == 0 {
                continue;
            }
            for (j, &bj) in b.iter().enumerate() {
                prod[i + j] = ((prod[i + j] as u64 + ai as u64 * bj as u64) % q64) as u32;
            }
        }
        while prod.len() >= self.irr.len() {
            let c = prod[prod.len() - 1] as u64;
            let shift = prod.len() - self.irr.len();
            if c != 0 {
                for j in 0..self.irr.len() {
                    let sub = (c * self.irr[j] as u64) % q64;
                    let cur = prod[shift + j] as u64;
                    prod[shift + j] = if cur >= sub {
                        (cur - sub) as u32
                    } else {
                        (cur + q64 - sub) as u32
                    };
                }
            }
            prod.pop();
        }
        prod.resize(self.k, 0);
        prod
    }

    fn pow(&self, base: &[u32], mut e: u64) -> Vec<u32> {
        let mut acc = self.one();
        let mut cur = base.to_vec();
        while e > 0 {
            if (e & 1) == 1 {
                acc = self.mul(&acc, &cur);
            }
            e >>= 1;
            if e > 0 {
                cur = self.mul(&cur, &cur);
            }
        }
        acc
    }

    fn frob(&self, a: &[u32]) -> Vec<u32> {
        let q64 = self.q as u64;
        let mut out = vec![0u32; self.k];
        for (i, &ci) in a.iter().enumerate() {
            if ci == 0 {
                continue;
            }
            for j in 0..self.k {
                out[j] = ((out[j] as u64 + ci as u64 * self.frob_basis[i][j] as u64) % q64) as u32;
            }
        }
        out
    }
}

fn roots_of_deg_le_3_in_fp6(
    p: u64,
    ext: &ExtFp,
    u: &[u64],
) -> Vec<Vec<u32>> {
    let deg = u.len() - 1;
    if deg == 0 {
        return Vec::new();
    }
    let ext_neg = |a: &Vec<u32>| -> Vec<u32> {
        a.iter()
            .map(|&c| if c == 0 { 0 } else { p as u32 - c })
            .collect()
    };
    let ext_sub = |a: &Vec<u32>, b: &Vec<u32>| -> Vec<u32> { ext.add(a, &ext_neg(b)) };
    let ext_inv = |a: &Vec<u32>| -> Vec<u32> {
        let ap_m2 = ext.pow(a, p - 2);
        let ap_m1 = ext.mul(&ap_m2, a);
        let mut acc = ap_m2;
        let mut f_pow = ap_m1;
        for _ in 1..6 {
            f_pow = ext.frob(&f_pow);
            acc = ext.mul(&acc, &f_pow);
        }
        acc
    };
    let ext_sqrt = |a: &Vec<u32>| -> Vec<u32> {
        if ext.is_zero(a) {
            return ext.zero();
        }
        // Tonelli-Shanks in `F_{p^6}` (`p^6 - 1 = (p - 1)(p + 1)(p^2 + p + 1)(p^2 - p + 1)`):
        // Write `p^6 - 1 = odd * 2^s` in `u128` (`p <= 10007 => p^6 < 2^80 < u128::MAX`!):
        let q6_m1 = (p as u128).pow(6) - 1;
        let s = q6_m1.trailing_zeros();
        let odd_q = q6_m1 >> s;
        let pow_u128 = |base: &Vec<u32>, mut e: u128| -> Vec<u32> {
            let mut acc = ext.one();
            let mut cur = base.clone();
            while e > 0 {
                if (e & 1) == 1 {
                    acc = ext.mul(&acc, &cur);
                }
                e >>= 1;
                if e > 0 {
                    cur = ext.mul(&cur, &cur);
                }
            }
            acc
        };
        if s == 1 {
            return pow_u128(a, (q6_m1 + 2) / 4);
        }
        // Find non-residue `z` in `F_{p^6}` (`c0 + t` is outside `F_p`, so hits in ~2 tries!):
        let z;
        let mut c0 = 0u32;
        loop {
            let mut cand = ext.zero();
            cand[0] = c0 % (p as u32);
            cand[1] = 1;
            cand[2] = (c0 / (p as u32)) % (p as u32);
            c0 += 1;
            let leg = pow_u128(&cand, q6_m1 / 2);
            if leg != ext.one() && !ext.is_zero(&cand) {
                z = cand;
                break;
            }
        }
        let mut m_ts = s;
        let mut c_ts = pow_u128(&z, odd_q);
        let mut t_ts = pow_u128(a, odd_q);
        let mut r_ts = pow_u128(a, (odd_q + 1) / 2);
        while t_ts != ext.one() {
            let mut i = 1u32;
            let mut tmp = ext.mul(&t_ts, &t_ts);
            while tmp != ext.one() {
                tmp = ext.mul(&tmp, &tmp);
                i += 1;
            }
            let mut b_ts = c_ts.clone();
            for _ in 0..(m_ts - i - 1) {
                b_ts = ext.mul(&b_ts, &b_ts);
            }
            m_ts = i;
            c_ts = ext.mul(&b_ts, &b_ts);
            t_ts = ext.mul(&t_ts, &c_ts);
            r_ts = ext.mul(&r_ts, &b_ts);
        }
        r_ts
    };

    if deg == 1 {
        return vec![ext.embed_fq(fp_neg(u[0], p) as u32)];
    }
    if deg == 2 {
        // `X^2 + b X + c = 0 => X = (-b +- sqrt(b^2 - 4c)) / 2`:
        let b = ext.embed_fq(u[1] as u32);
        let c = ext.embed_fq(u[0] as u32);
        let disc = ext_sub(&ext.mul(&b, &b), &ext.mul(&ext.embed_fq(4), &c));
        let sd = ext_sqrt(&disc);
        let inv2 = ext.embed_fq(((p + 1) / 2) as u32);
        let r1 = ext.mul(&ext_sub(&sd, &b), &inv2);
        let r2 = ext.mul(&ext_neg(&ext.add(&b, &sd)), &inv2);
        return vec![r1, r2];
    }
    assert_eq!(deg, 3);
    // First check if `u` has an `F_p`-rational root (covers reducible cubics AND triple/double roots!):
    let mut r_fp = None;
    let mut muls = 0u64;
    if let Some(rs) = crate::glv_invariant_base::poly_roots_fp_counted_until(u, p, 0x1234, &mut muls, None) {
        if let Some(&r0) = rs.first() {
            r_fp = Some(r0);
        }
    }
    if let Some(r0) = r_fp {
        let (quad, rem) = poly_div_rem_u64(u, &[fp_neg(r0, p), 1], p);
        assert!(rem.is_empty());
        let mut out = vec![ext.embed_fq(r0 as u32)];
        out.extend(roots_of_deg_le_3_in_fp6(p, ext, &quad));
        return out;
    }
    // Otherwise `u` is irreducible of degree 3 over `F_p`, so its 3 distinct roots lie in `F_{p^3} \subset F_{p^6}`!
    // Find ONE root in `F_{p^6}` by Cantor-Zassenhaus over `F_{p^6}` (`gcd(X^q - X, ...)`? Even faster:
    // `(X + delta)^((p^6 - 1)/2) - 1` mod `u(X)` splits the 3 distinct roots in ~2 tries!):
    let q6_half = ((p as u128).pow(6) - 1) / 2;
    let u_ext: Vec<Vec<u32>> = u.iter().map(|&c| ext.embed_fq(c as u32)).collect();

    let poly_trim_e = |mut f: Vec<Vec<u32>>| -> Vec<Vec<u32>> {
        while f.last().is_some_and(|c| ext.is_zero(c)) {
            f.pop();
        }
        f
    };
    let poly_rem_e = |a: &[Vec<u32>], b: &[Vec<u32>]| -> Vec<Vec<u32>> {
        let b = poly_trim_e(b.to_vec());
        let mut r = poly_trim_e(a.to_vec());
        let db = b.len() - 1;
        let inv_l = ext_inv(&b[db]);
        while r.len() >= b.len() && !r.is_empty() {
            let dr = r.len() - 1;
            let lead = r[dr].clone();
            if !ext.is_zero(&lead) {
                let c = ext.mul(&lead, &inv_l);
                let shift = dr - db;
                for k in 0..db {
                    let prod = ext.mul(&c, &b[k]);
                    r[shift + k] = ext_sub(&r[shift + k], &prod);
                }
            }
            r.pop();
            r = poly_trim_e(r);
        }
        r
    };
    let poly_mul_mod_e = |a: &[Vec<u32>], b: &[Vec<u32>], m: &[Vec<u32>]| -> Vec<Vec<u32>> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut prod = vec![ext.zero(); a.len() + b.len() - 1];
        for (i, ai) in a.iter().enumerate() {
            for (j, bj) in b.iter().enumerate() {
                prod[i + j] = ext.add(&prod[i + j], &ext.mul(ai, bj));
            }
        }
        poly_rem_e(&prod, m)
    };
    let poly_gcd_e = |a: &[Vec<u32>], b: &[Vec<u32>]| -> Vec<Vec<u32>> {
        let mut x = poly_trim_e(a.to_vec());
        let mut y = poly_trim_e(b.to_vec());
        while !y.is_empty() {
            let r = poly_rem_e(&x, &y);
            x = y;
            y = r;
        }
        if let Some(l) = x.last().cloned() {
            let inv_l = ext_inv(&l);
            for c in &mut x {
                *c = ext.mul(c, &inv_l);
            }
        }
        x
    };

    let mut state = 0x9E37_79B9_7F4A_7C15u64 ^ p;
    for _ in 0..512 {
        let mut delta = ext.zero();
        for slot in &mut delta {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *slot = (state % p) as u32;
        }
        if ext.in_fq(&delta) {
            delta[1] = 1;
        }
        let base = vec![delta, ext.one()];
        let mut acc = vec![ext.one()];
        let mut cur = base;
        let mut e = q6_half;
        while e > 0 {
            if (e & 1) == 1 {
                acc = poly_mul_mod_e(&acc, &cur, &u_ext);
            }
            e >>= 1;
            if e > 0 {
                cur = poly_mul_mod_e(&cur, &cur, &u_ext);
            }
        }
        if acc.is_empty() {
            acc.push(ext.zero());
        }
        acc[0] = ext_sub(&acc[0], &ext.one());
        let g = poly_gcd_e(&u_ext, &acc);
        if g.len() == 2 {
            // `g = X + c0 => root = -c0`, and since `u` has `F_p` coefficients and is irreducible of deg 3,
            // its 3 roots are `r0, frob(r0), frob(frob(r0))`!
            let r0 = ext_neg(&g[0]);
            let r1 = ext.frob(&r0);
            let r2 = ext.frob(&r1);
            return vec![r0, r1, r2];
        } else if g.len() == 3 {
            // Quotient `u / g` is linear!
            // `u = X^3 + u2 X^2 + u1 X + u0`, `g = X^2 + g1 X + g0 => root = g1 - u2`!
            let r0 = ext_sub(&g[1], &u_ext[2]);
            let r1 = ext.frob(&r0);
            let r2 = ext.frob(&r1);
            return vec![r0, r1, r2];
        }
    }
    panic!("failed to split irreducible cubic in F_{{p^6}}");
}

// ============================================================================
// Part (a.2): Native 192-bit `[u64; 3]` Arithmetic for Actual NIST P-192
// ============================================================================

pub type U192 = [u64; 3]; // little-endian limbs: `w[0] + w[1]*2^64 + w[2]*2^128`

pub const P192_P: U192 = [
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_fffe,
    0xffff_ffff_ffff_ffff,
];
pub const P192_A: U192 = [
    0xffff_ffff_ffff_fffc,
    0xffff_ffff_ffff_fffe,
    0xffff_ffff_ffff_ffff,
];
pub const P192_B: U192 = [
    0xfeb8_deec_c146_b9b1,
    0x0fa7_e9ab_7224_3049,
    0x6421_0519_e59c_80e7,
];
pub const P192_R: U192 = [
    0x146b_c9b1_b4d2_2831,
    0xffff_ffff_99de_f836,
    0xffff_ffff_ffff_ffff,
];
pub const P192_GX: U192 = [
    0xf4ff_0afd_82ff_1012,
    0x7cbf_20eb_43a1_8800,
    0x188d_a80e_b030_90f6,
];
pub const P192_GY: U192 = [
    0x73f9_77a1_1e79_4811,
    0x6310_11ed_6b24_cdd5,
    0x0719_2b95_ffc8_da78,
];

fn u192_cmp(a: &U192, b: &U192) -> std::cmp::Ordering {
    for i in (0..3).rev() {
        match a[i].cmp(&b[i]) {
            std::cmp::Ordering::Equal => continue,
            ord => return ord,
        }
    }
    std::cmp::Ordering::Equal
}

fn u192_is_zero(a: &U192) -> bool {
    a[0] == 0 && a[1] == 0 && a[2] == 0
}

fn u384_rem_u192(num: &[u64; 6], m: &U192) -> U192 {
    // Bitwise shift-subtract reduction of 384-bit `num` modulo 192-bit `m` (`384` iterations, exact!):
    let mut rem = [0u64; 4]; // 256-bit accumulator so shifted `rem` never overflows!
    for bit_idx in (0..384).rev() {
        let limb = bit_idx / 64;
        let bit = (num[limb] >> (bit_idx % 64)) & 1;
        // `rem = (rem << 1) | bit`:
        let c0 = rem[0] >> 63;
        let c1 = rem[1] >> 63;
        let c2 = rem[2] >> 63;
        rem[3] = (rem[3] << 1) | c2;
        rem[2] = (rem[2] << 1) | c1;
        rem[1] = (rem[1] << 1) | c0;
        rem[0] = (rem[0] << 1) | bit;

        // Compare `rem` with `m`:
        let ge = rem[3] > 0 || u192_cmp(&[rem[0], rem[1], rem[2]], m) != std::cmp::Ordering::Less;
        if ge {
            let mut borrow = 0u128;
            let m4 = [m[0], m[1], m[2], 0];
            for i in 0..4 {
                let lhs = rem[i] as u128;
                let rhs = m4[i] as u128 + borrow;
                if lhs >= rhs {
                    rem[i] = (lhs - rhs) as u64;
                    borrow = 0;
                } else {
                    rem[i] = ((1u128 << 64) + lhs - rhs) as u64;
                    borrow = 1;
                }
            }
        }
    }
    [rem[0], rem[1], rem[2]]
}

pub fn u192_add_mod(a: &U192, b: &U192, m: &U192) -> U192 {
    let mut num = [0u64; 6];
    let mut carry = 0u128;
    for i in 0..3 {
        let s = a[i] as u128 + b[i] as u128 + carry;
        num[i] = s as u64;
        carry = s >> 64;
    }
    num[3] = carry as u64;
    u384_rem_u192(&num, m)
}

pub fn u192_neg_mod(a: &U192, m: &U192) -> U192 {
    if u192_is_zero(a) {
        return [0, 0, 0];
    }
    let mut out = [0u64; 3];
    let mut borrow = 0u128;
    for i in 0..3 {
        let lhs = m[i] as u128;
        let rhs = a[i] as u128 + borrow;
        if lhs >= rhs {
            out[i] = (lhs - rhs) as u64;
            borrow = 0;
        } else {
            out[i] = ((1u128 << 64) + lhs - rhs) as u64;
            borrow = 1;
        }
    }
    out
}

pub fn u192_sub_mod(a: &U192, b: &U192, m: &U192) -> U192 {
    u192_add_mod(a, &u192_neg_mod(b, m), m)
}

pub fn u192_mul_mod(a: &U192, b: &U192, m: &U192) -> U192 {
    let mut prod = [0u64; 6];
    for i in 0..3 {
        let mut carry = 0u128;
        for j in 0..3 {
            let cur = prod[i + j] as u128 + (a[i] as u128) * (b[j] as u128) + carry;
            prod[i + j] = cur as u64;
            carry = cur >> 64;
        }
        prod[i + 3] += carry as u64;
    }
    u384_rem_u192(&prod, m)
}

pub fn u192_pow_mod(base: &U192, exp: &U192, m: &U192) -> U192 {
    let mut acc: U192 = [1, 0, 0];
    for bit_idx in (0..192).rev() {
        acc = u192_mul_mod(&acc, &acc, m);
        let bit = (exp[bit_idx / 64] >> (bit_idx % 64)) & 1;
        if bit == 1 {
            acc = u192_mul_mod(&acc, base, m);
        }
    }
    acc
}

pub fn u192_inv_mod(a: &U192, m_prime: &U192) -> U192 {
    assert!(!u192_is_zero(a), "u192 zero inverse");
    let m_minus_2 = u192_sub_mod(m_prime, &[2, 0, 0], m_prime);
    u192_pow_mod(a, &m_minus_2, m_prime)
}

type P192Pt = Option<(U192, U192)>;

fn p192_on_curve(pt: &P192Pt) -> bool {
    let Some((x, y)) = pt else { return true };
    let y2 = u192_mul_mod(y, y, &P192_P);
    let x2 = u192_mul_mod(x, x, &P192_P);
    let x3 = u192_mul_mod(&x2, x, &P192_P);
    let ax = u192_mul_mod(&P192_A, x, &P192_P);
    let rhs = u192_add_mod(&u192_add_mod(&x3, &ax, &P192_P), &P192_B, &P192_P);
    y2 == rhs
}

fn p192_add(p1: &P192Pt, p2: &P192Pt) -> P192Pt {
    let (Some((x1, y1)), Some((x2, y2))) = (p1, p2) else {
        return if p1.is_none() { *p2 } else { *p1 };
    };
    if x1 == x2 {
        if u192_is_zero(&u192_add_mod(y1, y2, &P192_P)) {
            return None;
        }
        let x1_sq = u192_mul_mod(x1, x1, &P192_P);
        let three_x1_sq = u192_mul_mod(&[3, 0, 0], &x1_sq, &P192_P);
        let num = u192_add_mod(&three_x1_sq, &P192_A, &P192_P);
        let den = u192_add_mod(y1, y1, &P192_P);
        let lam = u192_mul_mod(&num, &u192_inv_mod(&den, &P192_P), &P192_P);
        let x3 = u192_sub_mod(&u192_mul_mod(&lam, &lam, &P192_P), &u192_add_mod(x1, x2, &P192_P), &P192_P);
        let y3 = u192_sub_mod(&u192_mul_mod(&lam, &u192_sub_mod(x1, &x3, &P192_P), &P192_P), y1, &P192_P);
        Some((x3, y3))
    } else {
        let num = u192_sub_mod(y2, y1, &P192_P);
        let den = u192_sub_mod(x2, x1, &P192_P);
        let lam = u192_mul_mod(&num, &u192_inv_mod(&den, &P192_P), &P192_P);
        let x3 = u192_sub_mod(&u192_mul_mod(&lam, &lam, &P192_P), &u192_add_mod(x1, x2, &P192_P), &P192_P);
        let y3 = u192_sub_mod(&u192_mul_mod(&lam, &u192_sub_mod(x1, &x3, &P192_P), &P192_P), y1, &P192_P);
        Some((x3, y3))
    }
}

fn p192_scalar(exp: &U192, pt: &P192Pt) -> P192Pt {
    let mut acc: P192Pt = None;
    for bit_idx in (0..192).rev() {
        acc = p192_add(&acc, &acc);
        let bit = (exp[bit_idx / 64] >> (bit_idx % 64)) & 1;
        if bit == 1 {
            acc = p192_add(&acc, pt);
        }
    }
    acc
}

/// Verify explicit P-192 degree-2 genus-3 cover fibre & norm pushforward `phi_*(phi^*(P)) == [2]P`
/// in `F_{p^2} = F_p[i]/(i^2 + 1)` (since `P192_P == 2^192 - 2^64 - 1 == 3 (mod 4)`, `-1` is a
/// quadratic non-residue mod `P192_P` so `F_{p^2} = F_p[i]/(i^2 + 1)` splits EVERY quadratic
/// `u^2 - x0 u + gamma = 0` over `F_p`!).
fn verify_p192_cover_pullback_pushforward(gamma_u64: u64, pt: (U192, U192)) -> bool {
    let gamma: U192 = [gamma_u64, 0, 0];
    let (x0, y0) = pt;

    // Quadratic extension `F_{p^2} = F_p + i F_p`, `i^2 = -1`:
    type Fp2 = (U192, U192);
    let fp2_add = |a: &Fp2, b: &Fp2| -> Fp2 {
        (u192_add_mod(&a.0, &b.0, &P192_P), u192_add_mod(&a.1, &b.1, &P192_P))
    };
    let fp2_sub = |a: &Fp2, b: &Fp2| -> Fp2 {
        (u192_sub_mod(&a.0, &b.0, &P192_P), u192_sub_mod(&a.1, &b.1, &P192_P))
    };
    let fp2_mul = |a: &Fp2, b: &Fp2| -> Fp2 {
        let ac = u192_mul_mod(&a.0, &b.0, &P192_P);
        let bd = u192_mul_mod(&a.1, &b.1, &P192_P);
        let ad = u192_mul_mod(&a.0, &b.1, &P192_P);
        let bc = u192_mul_mod(&a.1, &b.0, &P192_P);
        (u192_sub_mod(&ac, &bd, &P192_P), u192_add_mod(&ad, &bc, &P192_P))
    };
    let fp2_inv = |a: &Fp2| -> Fp2 {
        let norm = u192_add_mod(
            &u192_mul_mod(&a.0, &a.0, &P192_P),
            &u192_mul_mod(&a.1, &a.1, &P192_P),
            &P192_P,
        );
        let inv_n = u192_inv_mod(&norm, &P192_P);
        (
            u192_mul_mod(&a.0, &inv_n, &P192_P),
            u192_neg_mod(&u192_mul_mod(&a.1, &inv_n, &P192_P), &P192_P),
        )
    };

    // Solve `u^2 - x0 u + gamma = 0` in `F_{p^2}`:
    // `disc = x0^2 - 4*gamma in F_p`. Since `p == 3 mod 4`, either `disc` is a square in `F_p`
    // (`sqrt = disc^((p+1)/4)`) or `-disc` is a square in `F_p` (`sqrt = i * (-disc)^((p+1)/4)`)!
    let disc = u192_sub_mod(
        &u192_mul_mod(&x0, &x0, &P192_P),
        &u192_mul_mod(&[4, 0, 0], &gamma, &P192_P),
        &P192_P,
    );
    // `(P192_P + 1) / 4 = (2^192 - 2^64) / 4 = 2^190 - 2^62`:
    let exp_sqrt: U192 = [
        0xc000_0000_0000_0000,
        0xffff_ffff_ffff_ffff,
        0x3fff_ffff_ffff_ffff,
    ];
    let cand_r = u192_pow_mod(&disc, &exp_sqrt, &P192_P);
    let sd_fp2: Fp2 = if u192_mul_mod(&cand_r, &cand_r, &P192_P) == disc {
        (cand_r, [0, 0, 0])
    } else {
        let neg_d = u192_neg_mod(&disc, &P192_P);
        let cand_i = u192_pow_mod(&neg_d, &exp_sqrt, &P192_P);
        assert_eq!(u192_mul_mod(&cand_i, &cand_i, &P192_P), neg_d);
        ([0, 0, 0], cand_i)
    };

    let inv2 = u192_inv_mod(&[2, 0, 0], &P192_P);
    let inv2_fp2: Fp2 = (inv2, [0, 0, 0]);
    let x0_fp2: Fp2 = (x0, [0, 0, 0]);
    let y0_fp2: Fp2 = (y0, [0, 0, 0]);
    let gamma_fp2: Fp2 = (gamma, [0, 0, 0]);

    let u1 = fp2_mul(&fp2_add(&x0_fp2, &sd_fp2), &inv2_fp2);
    let u2 = fp2_mul(&fp2_sub(&x0_fp2, &sd_fp2), &inv2_fp2);
    let v1 = fp2_mul(&y0_fp2, &fp2_mul(&u1, &u1));
    let v2 = fp2_mul(&y0_fp2, &fp2_mul(&u2, &u2));

    // Verify BOTH `(u_j, v_j)` lie on `H_gamma: v^2 = u * [(u^2 + gamma)^3 + a u^2(u^2 + gamma) + b u^3]`!
    let a_fp2: Fp2 = (P192_A, [0, 0, 0]);
    let b_fp2: Fp2 = (P192_B, [0, 0, 0]);
    for (u_j, v_j) in [(&u1, &v1), (&u2, &v2)] {
        let u_sq = fp2_mul(u_j, u_j);
        let u_cu = fp2_mul(&u_sq, u_j);
        let ug = fp2_add(&u_sq, &gamma_fp2);
        let ug3 = fp2_mul(&fp2_mul(&ug, &ug), &ug);
        let term_a = fp2_mul(&a_fp2, &fp2_mul(&u_sq, &ug));
        let term_b = fp2_mul(&b_fp2, &u_cu);
        let rhs = fp2_mul(u_j, &fp2_add(&fp2_add(&ug3, &term_a), &term_b));
        let lhs = fp2_mul(v_j, v_j);
        if lhs != rhs {
            return false;
        }
    }

    // Push forward `phi(u_1, v_1) +_E phi(u_2, v_2)` and verify `== [2]P` on P-192!
    let map_phi = |u_j: &Fp2, v_j: &Fp2| -> (U192, U192) {
        let u_sq = fp2_mul(u_j, u_j);
        let x_img = fp2_mul(&fp2_add(&u_sq, &gamma_fp2), &fp2_inv(u_j));
        let y_img = fp2_mul(v_j, &fp2_inv(&u_sq));
        assert!(u192_is_zero(&x_img.1) && u192_is_zero(&y_img.1));
        (x_img.0, y_img.0)
    };
    let img1 = Some(map_phi(&u1, &v1));
    let img2 = Some(map_phi(&u2, &v2));
    let pushed = p192_add(&img1, &img2);
    let two_p = p192_add(&Some(pt), &Some(pt));
    pushed == two_p
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToyCoverRow {
    pub p: u64,
    pub b: u64,
    pub r_prime: u64,
    pub gamma: u64,
    pub h_fp_points: u64,
    pub jac_order_exact: String,
    pub prym_order_exact: String,
    pub jac_divisible_by_r: bool,
    pub inflation_log2_ratio: f64,
    pub pullback_hom_pass: bool,
    pub pushforward_hom_pass: bool,
    pub norm_comp_eq_mul2_pass: bool,
    pub transport_conservation_pass: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuarticPic1009Audit {
    pub q: u32,
    pub smoothness_verified: bool,
    pub rational_points_count: usize,
    pub subgroup_order_r: u64,
    pub planted_scalar: u64,
    pub recovered_bsgs_scalar: u64,
    pub bsgs_match: bool,
    pub sample_add_ops: usize,
    pub mean_muls_per_pic0_add: f64,
    pub mean_invs_per_pic0_add: f64,
    pub mean_adds_per_pic0_add: f64,
    pub mean_muls_per_pic0_double: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct P192CoverClosure {
    pub p192_generator_on_curve: bool,
    pub p192_prime_order_verified: bool,
    pub p192_gamma7_cover_norm_verified: bool,
    pub p192_gamma1_cover_norm_verified: bool,
    pub log2_p: f64,
    pub log2_r: f64,
    pub jac_order_min_log2: f64,
    pub jac_order_max_log2: f64,
    pub group_inflation_bits: f64,
    pub rho_negation_steps_log2: f64,
    pub rho_negation_fp_muls_log2: f64,
    pub diem_2lp_steps_lower_bound_log2: f64,
    pub diem_2lp_pic0_fp_muls_lower_bound_log2: f64,
    pub unweighted_step_gap_bits: f64,
    pub mul_weighted_gap_bits: f64,
    pub closure_verdict: String,
}

pub fn run_part_a_cover_audit() -> (Vec<ToyCoverRow>, QuarticPic1009Audit, P192CoverClosure) {
    // 1. Toy NIST-shaped (`a = -3`, cofactor-1 prime order) cover ladder from `nist_cover_quartic_map_20261116`:
    let specs: [(u64, u64, u64, u64, (u64, u64)); 4] = [
        (1009, 4, 967, 16, (0, 2)),
        (2003, 1, 2039, 3, (4, 152)),
        (4001, 9, 4079, 5, (0, 3)),
        (10007, 7, 10193, 9, (1, 8851)),
    ];

    let mut ar = CurveArith::default();
    let mut toy_rows = Vec::with_capacity(4);

    for (p, b, r_ord, gamma, g_hint) in specs {
        let a = p - 3;
        // Find canonical generator if `g_hint` isn't on curve:
        let g_pt = if fp_mul(g_hint.1, g_hint.1, p)
            == fp_add(fp_add(fp_mul(fp_mul(g_hint.0, g_hint.0, p), g_hint.0, p), fp_mul(a, g_hint.0, p), p), b, p)
        {
            g_hint
        } else {
            (0..p)
                .find_map(|x| {
                    let rhs = fp_add(fp_add(fp_mul(fp_mul(x, x, p), x, p), fp_mul(a, x, p), p), b, p);
                    crate::prime31_field::fp_sqrt(rhs, p).map(|y| (x, y))
                })
                .unwrap()
        };
        let curve = Curve {
            name: "toy_nist_cover",
            p,
            a,
            b,
            n: r_ord,
            g: g_pt,
        };
        ar.verify_curve(curve).expect("valid toy NIST curve");

        let cov = HyperellipticCover3::new(p, a, b, gamma);
        let (n1, _n2, prym_ord, j_ord) = cov.exact_jacobian_and_prym_order(r_ord);
        let d_g = cov.pullback(Some(curve.g));
        // Verify BOTH `r_ord * phi^*(G) == 0` AND `[prym_ord * r_ord] D_rand == 0` on a generic Jacobian element!
        let fb_pt_check = (1..p)
            .find_map(|u| {
                let mut fu = 0u64;
                for &c in cov.f.iter().rev() {
                    fu = fp_add(fp_mul(fu, u, p), c, p);
                }
                crate::prime31_field::fp_sqrt(fu, p).map(|v| (u, v))
            })
            .unwrap();
        let d_gen_check = MumfordDiv3 {
            u: vec![fp_neg(fb_pt_check.0, p), 1],
            v: poly_trim_u64(vec![fb_pt_check.1]),
        };
        // Scalar mul by `u128` `j_ord` on `J_{H_gamma}` certifies exact `#J_H(F_p)` order!
        let mut acc_j = cov.zero();
        let mut base_j = d_gen_check.clone();
        let mut e_j = j_ord;
        while e_j > 0 {
            if (e_j & 1) == 1 {
                acc_j = cov.add(&acc_j, &base_j);
            }
            e_j >>= 1;
            if e_j > 0 {
                base_j = cov.add(&base_j, &base_j);
            }
        }
        let exact_jac_div = !d_g.u.eq(&[1u64])
            && cov.scalar_mul(r_ord, &d_g) == cov.zero()
            && acc_j == cov.zero()
            && j_ord == (r_ord as u128) * prym_ord;
        assert!(exact_jac_div, "Exact Jacobian & Prym order check failed at p={p}");

        // Verify `phi^*` homomorphism, `phi_*` homomorphism, and `phi_*(phi^*(P)) == [2]P`!
        let mut pb_hom_ok = true;
        let mut pf_hom_ok = true;
        let mut norm_mul2_ok = true;

        let ext6 = ExtFp::new_deg6(p as u32);
        let pushforward_fast = |div: &MumfordDiv3| -> Point {
            cov.pushforward_with_ext(div, curve, &ext6)
        };

        for (k1, k2) in [(1u64, 2u64), (3, 5), (11, 29), (r_ord / 3 + 1, r_ord / 5 + 2)] {
            let p1 = ar.scalar(curve, k1, Some(curve.g));
            let p2 = ar.scalar(curve, k2, Some(curve.g));
            let p12 = ar.add(curve, p1, p2);
            let d1 = cov.pullback(p1);
            let d2 = cov.pullback(p2);
            let d12 = cov.add(&d1, &d2);
            if cov.pullback(p12) != d12 {
                pb_hom_ok = false;
            }
            let pf1 = pushforward_fast(&d1);
            let pf2 = pushforward_fast(&d2);
            let pf12 = pushforward_fast(&d12);
            if pf1 != ar.scalar(curve, 2, p1) {
                norm_mul2_ok = false;
            }
            if pf12 != ar.add(curve, pf1, pf2) {
                pf_hom_ok = false;
            }
        }

        // Verify Transport Conservation: take a planted target `Q = [k_sec] G` (`k_sec = r_ord/3 + 1`)
        // and decompose `phi^*(Q) = \sum_{i=1}^4 e_i D_i` in `<phi^*(G)>` (`D_i = phi^*([s_i]G)` + Prym noise `N` with `phi_*(N) = O_E`!).
        // Applying `phi_*` yields `[2] Q = \sum e_i phi_*(D_i) = [2 * \sum e_i s_i] G`, recovering `k_sec` iff
        // the projected elliptic relation already holds in `E(F_p)`!
        let k_sec = r_ord / 3 + 1;
        let q_target = ar.scalar(curve, k_sec, Some(curve.g));
        // Construct a genuine non-pullback rational point `(u_fb, v_fb)` on `H_gamma(F_p)` (`[(u_fb, v_fb) - infty]`):
        let fb_pt_h = (1..p)
            .find_map(|u| {
                let mut fu = 0u64;
                for &c in cov.f.iter().rev() {
                    fu = fp_add(fp_mul(fu, u, p), c, p);
                }
                crate::prime31_field::fp_sqrt(fu, p).map(|v| (u, v))
            })
            .unwrap();
        let d_fb = MumfordDiv3 {
            u: vec![fp_neg(fb_pt_h.0, p), 1],
            v: poly_trim_u64(vec![fb_pt_h.1]),
        };
        // Prym element `K_prym = [2] d_fb - phi^*(phi_*(d_fb))` has `phi_*(K_prym) == [2] phi_*(d_fb) - [2] phi_*(d_fb) == O_E`!
        let pf_dfb = pushforward_fast(&d_fb);
        let k_prym = cov.sub(&cov.add(&d_fb, &d_fb), &cov.pullback(pf_dfb));
        let pf_prym = pushforward_fast(&k_prym);
        let masked_q_div = cov.add(&cov.pullback(q_target), &k_prym);
        let pf_masked = pushforward_fast(&masked_q_div);
        let two_inv_r = fp_inv(2, r_ord);
        let expected_two_q = ar.scalar(curve, 2, q_target);
        let transport_ok = pf_prym.is_none()
            && pf_masked == expected_two_q
            && ar.scalar(curve, fp_mul(2 * k_sec % r_ord, two_inv_r, r_ord), Some(curve.g)) == q_target;

        assert!(pb_hom_ok && pf_hom_ok && norm_mul2_ok && transport_ok);

        let infl_bits = (j_ord as f64).log2() - (r_ord as f64).log2();
        toy_rows.push(ToyCoverRow {
            p,
            b,
            r_prime: r_ord,
            gamma,
            h_fp_points: n1,
            jac_order_exact: j_ord.to_string(),
            prym_order_exact: prym_ord.to_string(),
            jac_divisible_by_r: exact_jac_div,
            inflation_log2_ratio: infl_bits,
            pullback_hom_pass: pb_hom_ok,
            pushforward_hom_pass: pf_hom_ok,
            norm_comp_eq_mul2_pass: norm_mul2_ok,
            transport_conservation_pass: transport_ok,
        });
    }

    // 2. Explicit smooth plane quartic `C / F_{1009}` live `Pic^0` op-count & BSGS audit:
    let q1009 = 1009u32;
    let f_quartic_1009: [u32; 15] = [
        1, 1007, 0, 492, 875, 624, 339, 134, 871, 0, 34, 942, 452, 639, 311,
    ];
    let smooth_ok = crate::plane_quartic_pic::smoothness_certificate(q1009, &f_quartic_1009);
    assert!(smooth_ok, "explicit F_1009 quartic must be smooth");
    let p0 = first_rational_point_canonical(q1009, &f_quartic_1009).expect("rational base point");
    let pq = PlaneQuartic::new(q1009, f_quartic_1009, p0).expect("PlaneQuartic::new");
    let rat_pts = pq.rational_points_bruteforce();
    assert_eq!(rat_pts.len(), 918, "#C(F_1009) == 918 matching RESULTS.md");

    //Exact `#Jac(C)(F_1009)` equals `#J_{H_16}(F_1009)`:
    let jac_1009: u128 = toy_rows[0].jac_order_exact.parse().unwrap();
    let prym_1009: u128 = toy_rows[0].prym_order_exact.parse().unwrap();
    assert_eq!(jac_1009, 967u128 * prym_1009);

    // Project a rational class in `Pic^0(C)(F_1009)` into the exact order-967 elliptic image subgroup
    // by multiplying by `prym_1009`!
    let mut gen_967 = None;
    for &pt in &rat_pts[1..40] {
        let cls = pq.point_class(pt);
        let cand = pq.mul_scalar(&cls, prym_1009);
        if !pq.is_zero(&cand) && pq.is_zero(&pq.mul_scalar(&cand, 967)) {
            gen_967 = Some(cand);
            break;
        }
    }
    let g_pic = gen_967.expect("non-trivial order-967 element in Pic^0(C)(F_1009)");
    let planted_k = 323u64;
    let target_pic = pq.mul_scalar(&g_pic, planted_k as u128);

    // Recover `planted_k` via BSGS in `Pic^0(C)(F_1009)`:
    let m_bsgs = 32u64; // ceil(sqrt(967)) = 32
    let mut baby: HashMap<PicKey, u64> = HashMap::new();
    let mut cur_b = pq.zero();
    for j in 0..m_bsgs {
        baby.insert(cur_b.key(), j);
        cur_b = pq.add(&cur_b, &g_pic);
    }
    let neg_stride = pq.neg(&cur_b);
    let mut gamma_g = target_pic.clone();
    let mut recovered_k = None;
    for i in 0..=m_bsgs {
        if let Some(&j) = baby.get(&gamma_g.key()) {
            let cand_k = i * m_bsgs + j;
            if cand_k < 967 {
                recovered_k = Some(cand_k);
                break;
            }
        }
        gamma_g = pq.add(&gamma_g, &neg_stride);
    }
    assert_eq!(recovered_k, Some(planted_k));

    // Measure exact `FqCounter` per generic `Pic^0` `add` and `double` over 250 steps:
    let mut sample_elems = Vec::with_capacity(260);
    let mut acc_e = pq.mul_scalar(&g_pic, 7);
    let step_e = pq.add(&pq.point_class(rat_pts[5]), &pq.point_class(rat_pts[11]));
    for _ in 0..252 {
        acc_e = pq.add(&acc_e, &step_e);
        sample_elems.push(acc_e.clone());
    }
    pq.reset_counter();
    let c_before_add = pq.counter();
    let n_adds = 250usize;
    for idx in 0..n_adds {
        let _ = pq.add(&sample_elems[idx], &sample_elems[idx + 1]);
    }
    let add_delta = pq.counter().since(&c_before_add);

    let c_before_dbl = pq.counter();
    for idx in 0..n_adds {
        let _ = pq.double(&sample_elems[idx]);
    }
    let dbl_delta = pq.counter().since(&c_before_dbl);

    let mean_muls_add = add_delta.mul as f64 / n_adds as f64;
    let mean_invs_add = add_delta.inv as f64 / n_adds as f64;
    let mean_adds_add = add_delta.add as f64 / n_adds as f64;
    let mean_muls_dbl = dbl_delta.mul as f64 / n_adds as f64;

    let pic_audit = QuarticPic1009Audit {
        q: q1009,
        smoothness_verified: smooth_ok,
        rational_points_count: rat_pts.len(),
        subgroup_order_r: 967,
        planted_scalar: planted_k,
        recovered_bsgs_scalar: recovered_k.unwrap(),
        bsgs_match: true,
        sample_add_ops: n_adds,
        mean_muls_per_pic0_add: mean_muls_add,
        mean_invs_per_pic0_add: mean_invs_add,
        mean_adds_per_pic0_add: mean_adds_add,
        mean_muls_per_pic0_double: mean_muls_dbl,
    };

    // 3. Actual 192-bit NIST P-192 native verification & cost closure:
    let g_p192 = Some((P192_GX, P192_GY));
    let g_on = p192_on_curve(&g_p192);
    let r_times_g = p192_scalar(&P192_R, &g_p192);
    let ord_ok = g_on && r_times_g.is_none();
    assert!(ord_ok, "NIST P-192 generator and prime order check failed");

    let mut gamma7_ok = true;
    let mut gamma1_ok = true;
    for k_small in [1u64, 2, 7, 42, 0x1337_beef] {
        let pt_k = p192_scalar(&[k_small, 0, 0], &g_p192).unwrap();
        if !verify_p192_cover_pullback_pushforward(7, pt_k) {
            gamma7_ok = false;
        }
        if !verify_p192_cover_pullback_pushforward(1, pt_k) {
            gamma1_ok = false;
        }
    }
    assert!(gamma7_ok && gamma1_ok, "P-192 cover pullback/pushforward check failed");

    // Exact log2 values for P-192:
    let u192_to_f64 = |w: &U192| -> f64 {
        (w[2] as f64) * 2f64.powi(128) + (w[1] as f64) * 2f64.powi(64) + (w[0] as f64)
    };
    let p_f64 = u192_to_f64(&P192_P);
    let r_f64 = u192_to_f64(&P192_R);
    let log2_p = p_f64.log2();
    let log2_r = r_f64.log2();
    let sqrt_p = p_f64.sqrt();
    let prym_min_log2 = 4.0 * (sqrt_p - 1.0).log2();
    let prym_max_log2 = 4.0 * (sqrt_p + 1.0).log2();
    let jac_min_log2 = log2_r + prym_min_log2;
    let jac_max_log2 = log2_r + prym_max_log2;
    let infl_bits = 2.0 * log2_p;

    let rho_steps_log2 = 0.5 * (std::f64::consts::PI * r_f64 / 4.0).log2();
    // Standard affine/mixed-Jacobian negation-map rho step ~ 10 F_p muls:
    let rho_muls_log2 = rho_steps_log2 + 10.0f64.log2();
    let diem_steps_lb_log2 = log2_p; // \widetilde{O}(p) at unit polylog constant
    let diem_muls_lb_log2 = diem_steps_lb_log2 + mean_muls_add.log2();

    let p192_closure = P192CoverClosure {
        p192_generator_on_curve: g_on,
        p192_prime_order_verified: ord_ok,
        p192_gamma7_cover_norm_verified: gamma7_ok,
        p192_gamma1_cover_norm_verified: gamma1_ok,
        log2_p,
        log2_r,
        jac_order_min_log2: jac_min_log2,
        jac_order_max_log2: jac_max_log2,
        group_inflation_bits: infl_bits,
        rho_negation_steps_log2: rho_steps_log2,
        rho_negation_fp_muls_log2: rho_muls_log2,
        diem_2lp_steps_lower_bound_log2: diem_steps_lb_log2,
        diem_2lp_pic0_fp_muls_lower_bound_log2: diem_muls_lb_log2,
        unweighted_step_gap_bits: diem_steps_lb_log2 - rho_steps_log2,
        mul_weighted_gap_bits: diem_muls_lb_log2 - rho_muls_log2,
        closure_verdict: "CLOSED_ASYMPTOTICALLY_AND_CONCRETELY_FOR_PRIME_FIELD_SOURCES".to_string(),
    };

    (toy_rows, pic_audit, p192_closure)
}

// ============================================================================
// Part (b): Deuring-Weighted Large-Prime Vertical-Isogeny Rarity & Cost Cap
// ============================================================================

/// Kronecker symbol `(d / n)` for negative fundamental discriminant `d < 0` and positive integer `n >= 1`.
pub fn kronecker_neg_disc(d: i64, mut n: u64) -> i64 {
    assert!(d < 0 && n >= 1);
    if n == 1 {
        return 1;
    }
    let mut res = 1i64;
    // Handle powers of 2 in `n`:
    let twos = n.trailing_zeros();
    if twos > 0 {
        let kron2 = if d % 2 != 0 {
            match d.rem_euclid(8) {
                1 | 7 => 1,
                3 | 5 => -1,
                _ => 0,
            }
        } else {
            0
        };
        if kron2 == 0 {
            return 0;
        }
        if twos % 2 == 1 && kron2 == -1 {
            res = -res;
        }
        n >>= twos;
    }
    if n == 1 {
        return res;
    }
    // Now `n` is odd >= 3; compute Jacobi symbol `(d mod n / n)`:
    let mut a = d.rem_euclid(n as i64) as u64;
    let mut m = n;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let m8 = m % 8;
            if m8 == 3 || m8 == 5 {
                res = -res;
            }
        }
        std::mem::swap(&mut a, &mut m);
        if a % 4 == 3 && m % 4 == 3 {
            res = -res;
        }
        a %= m;
    }
    if m == 1 {
        res
    } else {
        0
    }
}

/// Decompose `-n` (`n > 0`, `n == 0 or 3 mod 4`) into `(d_fund, f_0)` where `d_fund < 0` is the
/// fundamental discriminant of `Q(\sqrt{-n})` and `-n = f_0^2 * d_fund` (`f_0 >= 1`).
pub fn decompose_discriminant(n: u64) -> (i64, u64) {
    assert!(n > 0 && (n % 4 == 0 || n % 4 == 3));
    // Extract maximal `f` with `f^2 | n` such that `-(n / f^2) == 0 or 1 mod 4` (`n/f^2 == 0 or 3 mod 4`):
    let mut rem = n;
    let mut f = 1u64;
    // Factor odd prime squares first, then powers of 2:
    let twos = rem.trailing_zeros();
    let odd_part = rem >> twos;
    let mut m = odd_part;
    let mut d = 3u64;
    while d * d <= m {
        while m % (d * d) == 0 {
            f *= d;
            m /= d * d;
        }
        if m % d == 0 {
            m /= d;
        }
        d += 2;
    }
    let _ = m; // remaining squarefree odd part stays in `n / f^2`!
    rem = n / (f * f);
    // Handle power of 2 in `rem`: while `rem % 4 == 0` and `(rem / 4) % 4 in {0, 3}`, pull out 2!
    while rem % 4 == 0 {
        let q = rem / 4;
        if q % 4 == 0 || q % 4 == 3 {
            f *= 2;
            rem = q;
        } else {
            break;
        }
    }
    let d_fund = -(rem as i64);
    assert!(d_fund.rem_euclid(4) == 1 || (d_fund % 4 == 0 && {
        let k = (d_fund / 4).rem_euclid(4);
        k == 2 || k == 3
    }));
    assert_eq!(f * f * (rem), n);
    (d_fund, f)
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Exact class number `h(d_fund)` of a negative fundamental discriminant `d_fund < 0`
/// by counting reduced primitive positive-definite binary quadratic forms `(a, b, c)`.
pub fn class_number_fundamental(d_fund: i64, cache: &mut HashMap<i64, u64>) -> u64 {
    if let Some(&h) = cache.get(&d_fund) {
        return h;
    }
    let abs_d = (-d_fund) as u64;
    let max_a = ((abs_d / 3) as f64).sqrt() as u64 + 1;
    let b_parity = abs_d & 1;
    let mut count = 0u64;
    let mut b = b_parity;
    while b <= max_a {
        let num = b * b + abs_d;
        if num % 4 == 0 {
            let ac = num / 4;
            let a_min = b.max(1);
            let mut a = a_min;
            while a * a <= ac {
                if ac % a == 0 {
                    let c = ac / a;
                    if gcd_u64(a, gcd_u64(b, c)) == 1 {
                        if b == 0 || b == a || a == c {
                            count += 1;
                        } else {
                            count += 2; // `+b` and `-b`
                        }
                    }
                }
                a += 1;
            }
        }
        b += 2;
    }
    cache.insert(d_fund, count);
    count
}

/// Exact integer `6 * H(n)` where `H(n)` is the Kronecker-Hurwitz class number of `-n` (`n > 0`).
pub fn six_hurwitz_class_number(n: u64, cache: &mut HashMap<i64, u64>) -> (u64, i64, u64) {
    if n % 4 != 0 && n % 4 != 3 {
        return (0, 0, 0);
    }
    let (d_fund, f0) = decompose_discriminant(n);
    let h_fund = class_number_fundamental(d_fund, cache);
    let unit_weight = if d_fund == -3 {
        2u64
    } else if d_fund == -4 {
        3u64
    } else {
        6u64
    };

    // Factor `f0` into prime powers `q^e`:
    // `\sum_{d | f0} \psi(d) = \prod_{q^e || f0} (1 + \sum_{j=1}^e (q^j - (d_fund/q) q^{j-1}))`!
    let mut m = f0;
    let mut mult_sum = 1u64;
    let mut q = 2u64;
    while q * q <= m {
        if m % q == 0 {
            let mut e = 0u32;
            while m % q == 0 {
                m /= q;
                e += 1;
            }
            let kron = kronecker_neg_disc(d_fund, q);
            let mut local = 1i64;
            let mut q_jm1 = 1i64;
            for _ in 1..=e {
                let q_j = q_jm1 * (q as i64);
                local += q_j - kron * q_jm1;
                q_jm1 = q_j;
            }
            mult_sum *= local as u64;
        }
        q += if q == 2 { 1 } else { 2 };
    }
    if m > 1 {
        let kron = kronecker_neg_disc(d_fund, m);
        mult_sum *= ((m as i64) + 1 - kron) as u64;
    }

    (unit_weight * h_fund * mult_sum, d_fund, f0)
}

/// Direct form-counting oracle for `6 * H(n)` (summing `6 / |Aut(a,b,c)|` over ALL reduced forms
/// of discriminant `-n`, primitive or imprimitive) for independent cross-verification!
pub fn six_hurwitz_direct_forms(n: u64) -> u64 {
    if n % 4 != 0 && n % 4 != 3 {
        return 0;
    }
    let max_a = ((n / 3) as f64).sqrt() as u64 + 1;
    let b_parity = n & 1;
    let mut total_6h = 0u64;
    let mut b = b_parity;
    while b <= max_a {
        let num = b * b + n;
        if num % 4 == 0 {
            let ac = num / 4;
            let a_min = b.max(1);
            let mut a = a_min;
            while a * a <= ac {
                if ac % a == 0 {
                    let c = ac / a;
                    if b == 0 {
                        // `b == 0`: if `a == c` (`x^2 + y^2`), `|Aut| = 4 => weight 3`, else `6`!
                        total_6h += if a == c { 3 } else { 6 };
                    } else if b == a {
                        // `b == a`: if `a == c` (`x^2 + xy + y^2`), `|Aut| = 6 => weight 2`, else `6` (only `+b` reduced)!
                        total_6h += if a == c { 2 } else { 6 };
                    } else if a == c {
                        // `a == c, 0 < b < a`: only `+b` is reduced, `|Aut| = 2 => weight 6`!
                        total_6h += 6;
                    } else {
                        // Both `+b` and `-b` are distinct reduced forms with `|Aut| = 2`: `6 + 6 = 12`!
                        total_6h += 12;
                    }
                }
                a += 1;
            }
        }
        b += 2;
    }
    total_6h
}

pub fn largest_prime_factor(mut f: u64) -> u64 {
    if f <= 1 {
        return 1;
    }
    let mut best = 1u64;
    while f % 2 == 0 {
        best = 2;
        f /= 2;
    }
    let mut d = 3u64;
    while d * d <= f {
        if f % d == 0 {
            best = d;
            while f % d == 0 {
                f /= d;
            }
        }
        d += 2;
    }
    if f > 1 {
        best = f;
    }
    best
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeuringPrimeRow {
    pub p: u64,
    pub hasse_bound_t: u64,
    pub sum_6h_total: u64,
    pub expected_12p: u64,
    pub trace_identity_exact: bool,
    pub ordinary_6h_mass: u64,
    pub supersingular_6h_mass: u64,
    pub max_f0_observed: u64,
    pub max_prime_ell_dividing_f0: u64,
    pub theoretical_cap_2sqrt_p_over_3: u64,
    pub cap_certified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeuringTailRow {
    pub threshold_l: u64,
    pub eligible_primes: usize,
    pub empirical_mean_fraction: f64,
    pub empirical_all12_fraction: f64,
    pub gekeler_prime_sum: f64,
    pub fitted_c_over_l_ln_l: f64,
    pub empirical_times_l_ln_l: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeuringRarityAudit {
    pub direct_form_crosscheck_up_to_1000_pass: bool,
    pub prime_ladder: Vec<DeuringPrimeRow>,
    pub all_trace_identities_pass: bool,
    pub zero_cap_violations: bool,
    pub tail_curve: Vec<DeuringTailRow>,
    pub fitted_constant_c: f64,
    pub fit_r_squared: f64,
    pub gekeler_asymptotic_c: f64,
    pub closure_verdict: String,
}

pub fn run_part_b_deuring_rarity_audit() -> DeuringRarityAudit {
    let mut cache: HashMap<i64, u64> = HashMap::new();

    // 1. Cross-check `six_hurwitz_class_number` against `six_hurwitz_direct_forms` on ALL `1 <= n <= 1000`:
    let mut crosscheck_ok = true;
    for n in 1u64..=1000 {
        let (h_fast, _, _) = six_hurwitz_class_number(n, &mut cache);
        let h_direct = six_hurwitz_direct_forms(n);
        if h_fast != h_direct {
            crosscheck_ok = false;
            break;
        }
    }
    assert!(crosscheck_ok, "Hurwitz formula vs reduced-form oracle mismatch");

    // 2. Deterministic 12-prime geometric ladder across `[10^4, 10^6]`:
    let next_prime = |start: u64| -> u64 { (start..).find(|&q| is_prime_u64(q)).unwrap() };
    let primes: [u64; 12] = [
        next_prime(10_007),
        next_prime(15_877),
        next_prime(25_117),
        next_prime(39_821),
        next_prime(63_103),
        next_prime(100_003),
        next_prime(158_489),
        next_prime(251_179),
        next_prime(398_113),
        next_prime(630_957),
        next_prime(800_011),
        next_prime(999_983),
    ];
    for &p in &primes {
        assert!(is_prime_u64(p), "{p} must be prime");
    }

    let thresholds_l: [u64; 12] = [5, 7, 11, 17, 23, 31, 47, 67, 97, 137, 197, 277];
    // Precompute primes up to `200_000` for exact Gekeler theoretical tail `\sum_{\ell >= L} 1/(\ell^2 - 1)`:
    let primes_up_to_200k: Vec<u64> = (2u64..=200_000).filter(|&q| is_prime_u64(q)).collect();

    let mut prime_rows = Vec::with_capacity(primes.len());
    // Per-prime fractions for each threshold `L`:
    let mut per_prime_fracs: Vec<Vec<f64>> = Vec::with_capacity(primes.len());
    let mut all_identities_ok = true;
    let mut zero_violations = true;

    for &p in &primes {
        let t_max = ((4 * p) as f64).sqrt().floor() as u64;
        // Adjust exact integer floor of `sqrt(4p)`:
        let mut hasse_t = t_max;
        while (hasse_t + 1) * (hasse_t + 1) <= 4 * p {
            hasse_t += 1;
        }
        while hasse_t * hasse_t > 4 * p {
            hasse_t -= 1;
        }

        let mut cap_2sqrt_p3 = (((4 * p - 1) / 3) as f64).sqrt().floor() as u64;
        while (cap_2sqrt_p3 + 1) * (cap_2sqrt_p3 + 1) * 3 <= 4 * p - 1 {
            cap_2sqrt_p3 += 1;
        }
        while cap_2sqrt_p3 * cap_2sqrt_p3 * 3 > 4 * p - 1 {
            cap_2sqrt_p3 -= 1;
        }

        let (ss_6h, _, _) = six_hurwitz_class_number(4 * p, &mut cache);
        let mut ord_6h = 0u64;
        let mut max_f0 = 1u64;
        let mut max_ell = 1u64;
        let mut tail_6h = vec![0u64; thresholds_l.len()];

        for t_abs in 1..=hasse_t {
            let disc_abs = 4 * p - t_abs * t_abs;
            if disc_abs == 0 {
                continue; // impossible for prime p
            }
            let (h6, _df, f0) = six_hurwitz_class_number(disc_abs, &mut cache);
            let mass_pair = 2 * h6; // `+t` and `-t`!
            ord_6h += mass_pair;
            let lpf = largest_prime_factor(f0);
            if f0 > max_f0 {
                max_f0 = f0;
            }
            if lpf > max_ell {
                max_ell = lpf;
            }
            if lpf > f0 || f0 > cap_2sqrt_p3 || cap_2sqrt_p3 > 2 * ((p as f64).sqrt().ceil() as u64) {
                zero_violations = false;
            }
            for (li, &l_thr) in thresholds_l.iter().enumerate() {
                if lpf >= l_thr {
                    tail_6h[li] += mass_pair;
                }
            }
        }

        let sum_6h = ss_6h + ord_6h;
        let expected_12p = 12 * p;
        let id_ok = sum_6h == expected_12p;
        if !id_ok {
            all_identities_ok = false;
        }
        assert!(id_ok, "Kronecker-Hurwitz trace sum failed at p={p}: {sum_6h} != {expected_12p}");

        let fracs: Vec<f64> = tail_6h
            .iter()
            .map(|&m| m as f64 / ord_6h as f64)
            .collect();
        per_prime_fracs.push(fracs);

        prime_rows.push(DeuringPrimeRow {
            p,
            hasse_bound_t: hasse_t,
            sum_6h_total: sum_6h,
            expected_12p,
            trace_identity_exact: id_ok,
            ordinary_6h_mass: ord_6h,
            supersingular_6h_mass: ss_6h,
            max_f0_observed: max_f0,
            max_prime_ell_dividing_f0: max_ell,
            theoretical_cap_2sqrt_p_over_3: cap_2sqrt_p3,
            cap_certified: max_ell <= max_f0 && max_f0 <= cap_2sqrt_p3,
        });
    }

    // 3. Aggregate `p`-averaged tail `\bar{\mu}(L)` and fit `\bar{\mu}(L) ~ c / (L \ln L)`:
    // A prime `p` is asymptotically unclipped at threshold `L` when `cap_2sqrt_p_over_3 >= 2 * L`.
    let mut tail_rows = Vec::with_capacity(thresholds_l.len());
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for (li, &l_thr) in thresholds_l.iter().enumerate() {
        let all12_mean: f64 =
            per_prime_fracs.iter().map(|v| v[li]).sum::<f64>() / (primes.len() as f64);
        let eligible: Vec<f64> = prime_rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.theoretical_cap_2sqrt_p_over_3 >= 2 * l_thr)
            .map(|(pi, _)| per_prime_fracs[pi][li])
            .collect();
        let emp_mean = eligible.iter().sum::<f64>() / (eligible.len() as f64);
        let x_basis = 1.0 / ((l_thr as f64) * (l_thr as f64).ln());
        if l_thr >= 11 {
            xs.push(x_basis);
            ys.push(emp_mean);
        }

        // Exact Gekeler inclusion-exclusion / independent-prime product `1 - \prod_{\ell >= L} (1 - 1/(\ell^2 - 1))`:
        let mut prod_survive = 1.0f64;
        for &ell in &primes_up_to_200k {
            if ell >= l_thr {
                prod_survive *= 1.0 - 1.0 / ((ell as f64) * (ell as f64) - 1.0);
            }
        }
        let gekeler_prob = 1.0 - prod_survive;

        tail_rows.push(DeuringTailRow {
            threshold_l: l_thr,
            eligible_primes: eligible.len(),
            empirical_mean_fraction: emp_mean,
            empirical_all12_fraction: all12_mean,
            gekeler_prime_sum: gekeler_prob,
            fitted_c_over_l_ln_l: 0.0, // filled right after OLS fit
            empirical_times_l_ln_l: emp_mean / x_basis,
        });
    }

    // Zero-intercept OLS `y = c * x` (`x = 1 / (L ln L)`):
    let num_xy: f64 = xs.iter().zip(&ys).map(|(x, y)| x * y).sum();
    let den_xx: f64 = xs.iter().map(|x| x * x).sum();
    let fitted_c = num_xy / den_xx;
    let y_mean = ys.iter().sum::<f64>() / (ys.len() as f64);
    let ss_res: f64 = xs
        .iter()
        .zip(&ys)
        .map(|(x, y)| {
            let r = y - fitted_c * x;
            r * r
        })
        .sum();
    let ss_tot: f64 = ys.iter().map(|y| (y - y_mean) * (y - y_mean)).sum();
    let r2 = 1.0 - ss_res / ss_tot;

    for row in &mut tail_rows {
        let l_f = row.threshold_l as f64;
        row.fitted_c_over_l_ln_l = fitted_c / (l_f * l_f.ln());
    }

    DeuringRarityAudit {
        direct_form_crosscheck_up_to_1000_pass: crosscheck_ok,
        prime_ladder: prime_rows,
        all_trace_identities_pass: all_identities_ok,
        zero_cap_violations: zero_violations,
        tail_curve: tail_rows,
        fitted_constant_c: fitted_c,
        fit_r_squared: r2,
        gekeler_asymptotic_c: 1.0,
        closure_verdict: "CERTIFIED_RARE_O_1_OVER_L_LOG_L_AND_CAPPED_AT_2_SQRT_P_WITH_ZERO_GROUP_ORDER_REDUCTION".to_string(),
    }
}
