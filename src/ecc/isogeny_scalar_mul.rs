//! Algebraic scalar-multiplication speedups via low-degree isogeny bridges and
//! small-discriminant CM endomorphisms (`GLV-2`).
//!
//! This module implements two concrete algebraic mechanisms on prime-field short
//! Weierstrass curves `E: y² = x³ + a·x + b` over [`GenericField<L>`]:
//!
//! 1. **Low-Degree Isogeny Bridge to a Montgomery Ladder (`TwoIsogenyMontgomeryBridge`)**:
//!    Given three `F_p`-rational 2-torsion abscissae `(r₀, r₁, r₂)` on `E` and an odd
//!    prime subgroup order `r`, we precompute `2⁻¹ mod r` and route `[k]P` through a
//!    **single** forward 2-isogeny `φ₂: E → E_M` onto a twisted Montgomery curve
//!    `E_M: B·w² = u³ + A·u² + u`, run a constant-time branchless `XZ` differential
//!    ladder for `k' = k · 2⁻¹ mod r`, and pull back through the dual 2-isogeny `φ̂₂`:
//!    ```text
//!        φ̂₂([k'] φ₂(P)) = [2 · k'] P = [k] P.
//!    ```
//!    Crucially, this unlocks a `5M + 4S` (prepared) / `6M + 4S` (unprepared) per-bit
//!    constant-time Montgomery ladder **even when `E` itself admits NO Montgomery or
//!    twisted Edwards model over `F_p`** (i.e. when `p ≡ 3 (mod 4)` and all three
//!    2-torsion slopes `3·rᵢ² + a` are quadratic non-residues in `F_p`, so `#E = 4r`
//!    with `E(F_p)[2] ≅ Z/2 × Z/2` and zero points of order 4 on both `E` and `E^t`).
//!    Furthermore, by homogenizing Okeya–Sakurai `y`-recovery jointly with `φ̂₂`,
//!    `scalar_mul_jacobian` requires **zero field inversions** (`0I`) end-to-end, and
//!    `scalar_mul_affine` requires **one field inversion** (`1I`) total.
//!
//! 2. **Small-Discriminant CM Action GLV-2 (`CmLowDegreeEndo`, `CmLowDegreeGlv`)**:
//!    Beyond `j ∈ {0, 1728}` automorphisms, any ordinary curve whose endomorphism ring
//!    `End(E) ≅ O_D` contains a small-norm element `ψ` of norm `ℓ ∈ {2, 3}` (such as
//!    `D = −7` with `ψ = (1 + √−7)/2` of norm 2, `D = −8` with `ψ = √−2` of norm 2,
//!    or `D = −11` with `ψ = (1 + √−11)/2` of norm 3) admits an explicit inversion-free
//!    affine-to-Jacobian Vélu endomorphism (`5M + 1S` for `ℓ = 2`, `6M + 2S` for `ℓ = 3`)
//!    combined with Gauss-reduced 2D lattice Babai rounding `[k]P = [k₁]P + [k₂]ψ(P)`
//!    (`max(|k₁|, |k₂|) ≲ √r`), halving the main doubling loop length.

use super::endo_generic_field::{Fe, GenericField};
use super::endo_generic_point::{glv_combine, Affine, Curve, DoublingKind, Jacobian, TableMode};
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Zero};

/// Pre-normalized image `Q = φ₂(P) = (u_Q, w_Q)` on the twisted Montgomery codomain
/// `B·w² = u³ + A·u² + u`, enabling a `5M + 4S`/bit `XZ` ladder (`W_Q = 1`).
#[derive(Clone, Copy, Debug)]
pub struct PreparedBridgePoint<const L: usize> {
    pub u_q: Fe<L>,
    pub w_q: Fe<L>,
}

/// Degree-2 isogeny bridge `E ──φ₂──▶ E_Mont ──[k·2⁻¹ mod r]──▶ E_Mont ──φ̂₂──▶ E`.
#[derive(Clone, Debug)]
pub struct TwoIsogenyMontgomeryBridge<const L: usize> {
    pub curve: Curve<L>,
    pub order: BigUint,
    pub inv2_order: BigUint,
    pub scalar_bits: usize,
    r0: Fe<L>,
    c: Fe<L>,
    v: Fe<L>,
    d: Fe<L>,
    d2: Fe<L>,
    neg_2c: Fe<L>,
    neg_2x0: Fe<L>,
    mont_a: Fe<L>,
    a24: Fe<L>,
    roots_big: [BigUint; 3],
}

impl<const L: usize> TwoIsogenyMontgomeryBridge<L> {
    /// Construct and algebraically certify a 2-isogeny Montgomery bridge from three
    /// distinct `F_p`-rational 2-torsion abscissae `(r0, r1, r2)` of `E: y² = x³ + a·x + b`
    /// and an odd prime subgroup order `order`.
    pub fn new(
        curve: Curve<L>,
        roots: [BigUint; 3],
        order: BigUint,
    ) -> Result<Self, String> {
        let f = &curve.f;
        let p = f.modulus().clone();
        if order <= BigUint::from(3u8) || (&order & BigUint::one()).is_zero() {
            return Err("subgroup order r must be an odd integer > 3".into());
        }
        if roots[0] == roots[1] || roots[1] == roots[2] || roots[0] == roots[2] {
            return Err("2-torsion roots r0, r1, r2 must be pairwise distinct".into());
        }
        for (idx, rt) in roots.iter().enumerate() {
            if rt >= &p {
                return Err(format!("root r{idx} is not canonical (< p)"));
            }
            let rhs = (rt.modpow(&BigUint::from(3u8), &p) + &curve.a_big * rt + &curve.b_big) % &p;
            if !rhs.is_zero() {
                return Err(format!("(r{idx}, 0) is not a 2-torsion point on E"));
            }
        }
        let sum_roots = (&roots[0] + &roots[1] + &roots[2]) % &p;
        if !sum_roots.is_zero() {
            return Err("short Weierstrass 2-torsion roots must sum to 0 mod p".into());
        }

        let r0 = f.from_canonical(&roots[0])?;
        let r1 = f.from_canonical(&roots[1])?;
        let r2 = f.from_canonical(&roots[2])?;

        // Forward 2-isogeny with kernel (r0, 0):
        // Shifted Weierstrass X = x - r0 satisfies y^2 = X*(X^2 + c*X + v) with
        // c = 3*r0, v = 3*r0^2 + a, and discriminant c^2 - 4v = (r1 - r2)^2 = d^2.
        let c = f.mul_small(&r0, 3);
        let v = f.add(&f.mul(&c, &r0), &curve.a);
        let d = f.sub(&r1, &r2);
        let d2 = f.sqr(&d);
        let disc_check = f.sub(&f.sqr(&c), &f.mul_small(&v, 4));
        if d2 != disc_check {
            return Err("2-isogeny discriminant mismatch: (r1 - r2)^2 != c^2 - 4v".into());
        }

        // Codomain twisted Montgomery curve B*w^2 = u^3 + A*u^2 + u has:
        // B = d^-1, A = -2*c*d^-1, and ladder constant a24 = (A + 2)/4.
        let d_inv = f.inv(&d);
        let neg_2c = f.neg(&f.add(&c, &c));
        let mont_a = f.mul(&neg_2c, &d_inv);
        let inv4 = f.inv(&f.from_i64(4));
        let two = f.from_i64(2);
        let a24 = f.mul(&f.add(&mont_a, &two), &inv4);
        let neg_2x0 = f.neg(&f.add(&r0, &r0));

        // 2^-1 mod r = (r + 1)/2 since r is odd.
        let inv2_order = (&order + BigUint::one()) >> 1usize;
        let scalar_bits = order.bits() as usize;

        Ok(Self {
            curve,
            order,
            inv2_order,
            scalar_bits,
            r0,
            c,
            v,
            d,
            d2,
            neg_2c,
            neg_2x0,
            mont_a,
            a24,
            roots_big: roots,
        })
    }

    /// Returns `true` iff `E` itself admits a direct `F_p`-rational Montgomery (or
    /// twisted Edwards) model, which holds iff at least one `3·rᵢ² + a` is a quadratic
    /// residue in `F_p`.  When this returns `false` (all three Legendre symbols `-1`),
    /// `E` has **no** direct `F_p`-Montgomery model, yet `TwoIsogenyMontgomeryBridge`
    /// still executes `[k]P` on an `F_p`-Montgomery curve via `φ₂` and `φ̂₂`.
    pub fn has_direct_montgomery_model(&self) -> bool {
        let p = self.curve.f.modulus();
        let exp = (p - BigUint::one()) >> 1usize;
        self.roots_big.iter().any(|rt| {
            let slope = (BigUint::from(3u8) * rt * rt + &self.curve.a_big) % p;
            slope.modpow(&exp, p).is_one()
        })
    }

    /// Pre-normalize `Q = φ₂(P) = (u_Q, w_Q)` (`1` field inversion, amortized across
    /// multiple scalar multiplications of the same base or target point `P`).
    pub fn prepare_point(&self, p: &Affine<L>) -> PreparedBridgePoint<L> {
        let f = &self.curve.f;
        let x_sh = f.sub(&p.x, &self.r0);
        let x2 = f.sqr(&x_sh);
        let w_denom = f.mul(&self.d, &x2);
        let w_inv = f.inv(&w_denom);
        let quad = f.add(&f.add(&x2, &f.mul(&self.c, &x_sh)), &self.v);
        let u_num = f.mul(&quad, &x_sh);
        let y_num = f.mul(&p.y, &f.sub(&x2, &self.v));
        PreparedBridgePoint {
            u_q: f.mul(&u_num, &w_inv),
            w_q: f.mul(&y_num, &w_inv),
        }
    }

    /// Constant-time `XZ` differential ladder + **zero-inversion** homogenized
    /// Okeya–Sakurai + dual 2-isogeny `φ̂₂` returning `[k]P` as `Jacobian<L>`.
    ///
    /// Per-bit ladder cost: `6M + 4S`. End-to-end inversions: **`0`**.
    pub fn scalar_mul_jacobian(&self, p: &Affine<L>, k: &BigUint) -> Jacobian<L> {
        let k_mod = k % &self.order;
        if k_mod.is_zero() {
            return self.curve.identity();
        }
        let kp = (&k_mod * &self.inv2_order) % &self.order;
        let f = &self.curve.f;

        // 1. Inversion-free projective forward 2-isogeny (4M + 1S):
        //    u_Q = U_Q / W_Q, w_Q = Y_Q / W_Q with W_Q = d * (x - r0)^2.
        let x_sh = f.sub(&p.x, &self.r0);
        let x2 = f.sqr(&x_sh);
        let w_q = f.mul(&self.d, &x2);
        let u_q = f.mul(
            &f.add(&f.add(&x2, &f.mul(&self.c, &x_sh)), &self.v),
            &x_sh,
        );
        let y_q = f.mul(&p.y, &f.sub(&x2, &self.v));

        // 2. Constant-time projective XZ Montgomery ladder over fixed `scalar_bits`:
        let mut x0 = f.one();
        let mut z0 = f.zero();
        let mut x1 = u_q;
        let mut z1 = w_q;
        let mut swap = 0u64;
        for bit_idx in (0..self.scalar_bits).rev() {
            let bit = u64::from(kp.bit(bit_idx as u64));
            swap ^= bit;
            Fe::cswap(&mut x0, &mut x1, swap);
            Fe::cswap(&mut z0, &mut z1, swap);
            swap = bit;

            let t0 = f.add(&x0, &z0);
            let t1 = f.sub(&x0, &z0);
            let t2 = f.add(&x1, &z1);
            let t3 = f.sub(&x1, &z1);
            let da = f.mul(&t3, &t0);
            let cb = f.mul(&t2, &t1);
            let ap = f.add(&da, &cb);
            let am = f.sub(&da, &cb);
            x1 = f.mul(&w_q, &f.sqr(&ap));
            z1 = f.mul(&u_q, &f.sqr(&am));

            let aa = f.sqr(&t0);
            let bb = f.sqr(&t1);
            let e = f.sub(&aa, &bb);
            x0 = f.mul(&aa, &bb);
            z0 = f.mul(&e, &f.add(&bb, &f.mul(&self.a24, &e)));
        }
        Fe::cswap(&mut x0, &mut x1, swap);
        Fe::cswap(&mut z0, &mut z1, swap);

        // Edge scalar kp == r - 1 (i.e. k == r - 2): R1 = [r]Q = O has Z1 = 0,
        // so [k]P = [-2]P directly on E.
        if z1.is_zero() {
            let p2 = self.curve.double(&self.curve.from_affine(p));
            return self.curve.neg(&p2);
        }

        // 3. Fused homogeneous Okeya-Sakurai + Dual 2-isogeny -> Jacobian (0 inversions!):
        //    Note A * W_Q = (-2c * d^-1) * (d * X^2) = -2c * X^2!
        let x0u = f.mul(&x0, &u_q);
        let z0w = f.mul(&z0, &w_q);
        let x0w = f.mul(&x0, &w_q);
        let z0u = f.mul(&z0, &u_q);
        let x0z0 = f.mul(&x0, &z0);
        let aw = f.mul(&self.neg_2c, &x2);
        let two_aw_u_xz = {
            let t = f.mul(&f.mul(&aw, &u_q), &x0z0);
            f.add(&t, &t)
        };
        let prod_sum = f.mul(&f.add(&x0u, &z0w), &f.add(&x0w, &z0u));
        let term1 = f.add(&prod_sum, &two_aw_u_xz);
        let diff_uw = f.sub(&x0w, &z0u);
        let num_w_prime = f.sub(&f.mul(&term1, &z1), &f.mul(&f.sqr(&diff_uw), &x1));

        let x0_2 = f.sqr(&x0);
        let z0_2 = f.sqr(&z0);
        let t_4xz = {
            let t = f.add(&x0z0, &x0z0);
            f.add(&t, &t)
        };
        let f_wyz = f.mul(&f.mul(&w_q, &y_q), &z1);
        let z_jac = f.mul(&t_4xz, &f_wyz);
        let tf2 = f.mul(&z_jac, &f_wyz);

        let x_num = f.add(
            &f.mul(&self.d, &f.add(&x0_2, &z0_2)),
            &f.mul(&self.neg_2x0, &x0z0),
        );
        let x_jac = f.mul(&x_num, &tf2);
        let y_jac = f.mul(
            &f.mul(&f.mul(&self.d2, &num_w_prime), &f.sub(&x0_2, &z0_2)),
            &tf2,
        );
        Jacobian {
            x: x_jac,
            y: y_jac,
            z: z_jac,
        }
    }

    /// Constant-time `5M + 4S`/bit `XZ` ladder on a [`PreparedBridgePoint`], returning
    /// `[k]P` in `Jacobian<L>` with **zero field inversions** (`0I`).
    pub fn scalar_mul_prepared_jacobian(
        &self,
        prep: &PreparedBridgePoint<L>,
        p_orig: &Affine<L>,
        k: &BigUint,
    ) -> Jacobian<L> {
        let k_mod = k % &self.order;
        if k_mod.is_zero() {
            return self.curve.identity();
        }
        let kp = (&k_mod * &self.inv2_order) % &self.order;
        let f = &self.curve.f;

        let mut x0 = f.one();
        let mut z0 = f.zero();
        let mut x1 = prep.u_q;
        let mut z1 = f.one();
        let mut swap = 0u64;
        for bit_idx in (0..self.scalar_bits).rev() {
            let bit = u64::from(kp.bit(bit_idx as u64));
            swap ^= bit;
            Fe::cswap(&mut x0, &mut x1, swap);
            Fe::cswap(&mut z0, &mut z1, swap);
            swap = bit;

            let t0 = f.add(&x0, &z0);
            let t1 = f.sub(&x0, &z0);
            let t2 = f.add(&x1, &z1);
            let t3 = f.sub(&x1, &z1);
            let da = f.mul(&t3, &t0);
            let cb = f.mul(&t2, &t1);
            let ap = f.add(&da, &cb);
            let am = f.sub(&da, &cb);
            x1 = f.sqr(&ap);
            z1 = f.mul(&prep.u_q, &f.sqr(&am));

            let aa = f.sqr(&t0);
            let bb = f.sqr(&t1);
            let e = f.sub(&aa, &bb);
            x0 = f.mul(&aa, &bb);
            z0 = f.mul(&e, &f.add(&bb, &f.mul(&self.a24, &e)));
        }
        Fe::cswap(&mut x0, &mut x1, swap);
        Fe::cswap(&mut z0, &mut z1, swap);

        if z1.is_zero() {
            let p2 = self.curve.double(&self.curve.from_affine(p_orig));
            return self.curve.neg(&p2);
        }

        let x0u = f.mul(&x0, &prep.u_q);
        let z0u = f.mul(&z0, &prep.u_q);
        let x0z0 = f.mul(&x0, &z0);
        let two_au_xz = {
            let t = f.mul(&f.mul(&self.mont_a, &prep.u_q), &x0z0);
            f.add(&t, &t)
        };
        let prod_sum = f.mul(&f.add(&x0u, &z0), &f.add(&x0, &z0u));
        let term1 = f.add(&prod_sum, &two_au_xz);
        let diff_uw = f.sub(&x0, &z0u);
        let num_w = f.sub(&f.mul(&term1, &z1), &f.mul(&f.sqr(&diff_uw), &x1));

        let x0_2 = f.sqr(&x0);
        let z0_2 = f.sqr(&z0);
        let t_4xz = {
            let t = f.add(&x0z0, &x0z0);
            f.add(&t, &t)
        };
        let f_wz = f.mul(&prep.w_q, &z1);
        let z_jac = f.mul(&t_4xz, &f_wz);
        let tf2 = f.mul(&z_jac, &f_wz);

        let x_num = f.add(
            &f.mul(&self.d, &f.add(&x0_2, &z0_2)),
            &f.mul(&self.neg_2x0, &x0z0),
        );
        let x_jac = f.mul(&x_num, &tf2);
        let y_jac = f.mul(
            &f.mul(&f.mul(&self.d2, &num_w), &f.sub(&x0_2, &z0_2)),
            &tf2,
        );
        Jacobian {
            x: x_jac,
            y: y_jac,
            z: z_jac,
        }
    }

    /// Constant-time `[k]P` returning `Option<Affine<L>>` (`1` inversion total).
    pub fn scalar_mul_affine(&self, p: &Affine<L>, k: &BigUint) -> Option<Affine<L>> {
        let jac = self.scalar_mul_jacobian(p, k);
        self.curve.to_affine(&jac)
    }

    /// Constant-time `[k]P` on a [`PreparedBridgePoint`] returning `Option<Affine<L>>`
    /// (`5M + 4S`/bit + `1` fused inversion total).
    pub fn scalar_mul_prepared_affine(
        &self,
        prep: &PreparedBridgePoint<L>,
        p_orig: &Affine<L>,
        k: &BigUint,
    ) -> Option<Affine<L>> {
        let jac = self.scalar_mul_prepared_jacobian(prep, p_orig, k);
        self.curve.to_affine(&jac)
    }
}

/// Complete Renes–Costello–Batina (ePrint 2015/1060, Algorithms 1 & 3) constant-time
/// projective short-Weierstrass Montgomery ladder over the **exact same** [`Curve<L>`]
/// and [`GenericField<L>`], providing a zero-confounder constant-time baseline against
/// [`TwoIsogenyMontgomeryBridge`].
#[derive(Clone, Debug)]
pub struct RcbWeierstrassLadder<const L: usize> {
    pub curve: Curve<L>,
    b3: Fe<L>,
    pub scalar_bits: usize,
}

#[derive(Clone, Copy, Debug)]
struct RcbProj<const L: usize> {
    x: Fe<L>,
    y: Fe<L>,
    z: Fe<L>,
}

impl<const L: usize> RcbWeierstrassLadder<L> {
    pub fn new(curve: Curve<L>, scalar_bits: usize) -> Self {
        let b3 = curve.f.mul_small(&curve.b, 3);
        Self {
            curve,
            b3,
            scalar_bits,
        }
    }

    #[inline]
    fn cswap_proj(a: &mut RcbProj<L>, b: &mut RcbProj<L>, swap: u64) {
        Fe::cswap(&mut a.x, &mut b.x, swap);
        Fe::cswap(&mut a.y, &mut b.y, swap);
        Fe::cswap(&mut a.z, &mut b.z, swap);
    }

    #[inline]
    fn rcb_add(&self, p: &RcbProj<L>, q: &RcbProj<L>) -> RcbProj<L> {
        let f = &self.curve.f;
        let a = &self.curve.a;
        let b3 = &self.b3;

        let mut t0 = f.mul(&p.x, &q.x);
        let mut t1 = f.mul(&p.y, &q.y);
        let mut t2 = f.mul(&p.z, &q.z);
        let mut t3 = f.add(&p.x, &p.y);
        let mut t4 = f.add(&q.x, &q.y);
        t3 = f.mul(&t3, &t4);
        t4 = f.add(&t0, &t1);
        t3 = f.sub(&t3, &t4);
        t4 = f.add(&p.x, &p.z);
        let mut t5 = f.add(&q.x, &q.z);
        t4 = f.mul(&t4, &t5);
        t5 = f.add(&t0, &t2);
        t4 = f.sub(&t4, &t5);
        t5 = f.add(&p.y, &p.z);
        let mut x3 = f.add(&q.y, &q.z);
        t5 = f.mul(&t5, &x3);
        x3 = f.add(&t1, &t2);
        t5 = f.sub(&t5, &x3);
        let mut z3 = f.mul(a, &t4);
        x3 = f.mul(b3, &t2);
        z3 = f.add(&x3, &z3);
        x3 = f.sub(&t1, &z3);
        z3 = f.add(&t1, &z3);
        let mut y3 = f.mul(&x3, &z3);
        t1 = f.add(&t0, &t0);
        t1 = f.add(&t1, &t0);
        t2 = f.mul(a, &t2);
        t4 = f.mul(b3, &t4);
        t1 = f.add(&t1, &t2);
        t2 = f.sub(&t0, &t2);
        t2 = f.mul(a, &t2);
        t4 = f.add(&t4, &t2);
        t0 = f.mul(&t1, &t4);
        y3 = f.add(&y3, &t0);
        t0 = f.mul(&t5, &t4);
        x3 = f.mul(&t3, &x3);
        x3 = f.sub(&x3, &t0);
        t0 = f.mul(&t3, &t1);
        z3 = f.mul(&t5, &z3);
        z3 = f.add(&z3, &t0);

        RcbProj {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    #[inline]
    fn rcb_double(&self, p: &RcbProj<L>) -> RcbProj<L> {
        let f = &self.curve.f;
        let a = &self.curve.a;
        let b3 = &self.b3;

        let mut t0 = f.sqr(&p.x);
        let t1 = f.sqr(&p.y);
        let mut t2 = f.sqr(&p.z);
        let mut t3 = f.mul(&p.x, &p.y);
        t3 = f.add(&t3, &t3);
        let mut z3 = f.mul(&p.x, &p.z);
        z3 = f.add(&z3, &z3);
        let mut x3 = f.mul(a, &z3);
        let mut y3 = f.mul(b3, &t2);
        y3 = f.add(&x3, &y3);
        x3 = f.sub(&t1, &y3);
        y3 = f.add(&t1, &y3);
        y3 = f.mul(&x3, &y3);
        x3 = f.mul(&t3, &x3);
        z3 = f.mul(b3, &z3);
        t2 = f.mul(a, &t2);
        t3 = f.sub(&t0, &t2);
        t3 = f.mul(a, &t3);
        t3 = f.add(&t3, &z3);
        z3 = f.add(&t0, &t0);
        t0 = f.add(&z3, &t0);
        t0 = f.add(&t0, &t2);
        t0 = f.mul(&t0, &t3);
        y3 = f.add(&y3, &t0);
        t2 = f.mul(&p.y, &p.z);
        t2 = f.add(&t2, &t2);
        t0 = f.mul(&t2, &t3);
        x3 = f.sub(&x3, &t0);
        z3 = f.mul(&t2, &t1);
        z3 = f.add(&z3, &z3);
        z3 = f.add(&z3, &z3);

        RcbProj {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Constant-time complete short-Weierstrass Montgomery ladder returning `Jacobian<L>`
    /// (`0` inversions: homogeneous `(X : Y : Z)` converts to Jacobian `(X·Z : Y·Z² : Z)`
    /// in `2M + 1S`).
    pub fn scalar_mul_jacobian(&self, p: &Affine<L>, k: &BigUint) -> Jacobian<L> {
        let f = &self.curve.f;
        let mut r0 = RcbProj {
            x: f.zero(),
            y: f.one(),
            z: f.zero(),
        };
        let mut r1 = RcbProj {
            x: p.x,
            y: p.y,
            z: f.one(),
        };
        let mut swap = 0u64;
        for bit_idx in (0..self.scalar_bits).rev() {
            let bit = u64::from(k.bit(bit_idx as u64));
            swap ^= bit;
            Self::cswap_proj(&mut r0, &mut r1, swap);
            swap = bit;

            let sum = self.rcb_add(&r0, &r1);
            let dbl = self.rcb_double(&r0);
            r1 = sum;
            r0 = dbl;
        }
        Self::cswap_proj(&mut r0, &mut r1, swap);

        let z2 = f.sqr(&r0.z);
        Jacobian {
            x: f.mul(&r0.x, &r0.z),
            y: f.mul(&r0.y, &z2),
            z: r0.z,
        }
    }

    /// Constant-time complete short-Weierstrass ladder returning `Option<Affine<L>>`
    /// (`1` inversion).
    pub fn scalar_mul_affine(&self, p: &Affine<L>, k: &BigUint) -> Option<Affine<L>> {
        let jac = self.scalar_mul_jacobian(p, k);
        self.curve.to_affine(&jac)
    }
}

// ── Small-Discriminant CM Action GLV-2 ───────────────────────────────────────

/// Explicit inversion-free low-degree Vélu CM endomorphism `ψ: E → E` mapping an
/// affine point `P = (x, y)` directly to Jacobian `(X' : Y' : Z')` on `E`.
#[derive(Clone, Copy, Debug)]
pub enum CmLowDegreeEndo<const L: usize> {
    /// Degree-2 CM endomorphism (e.g. `D = −7` with `ψ = (1 + √−7)/2`, or `D = −8`
    /// with `ψ = √−2`), costing **`5M + 1S + 0I`**:
    /// `d = x − x₀`, `X' = (x·d + v)·d`, `Y' = y·(d² − v)·d`, `Z' = d·u⁻¹`.
    Deg2 { x0: Fe<L>, v: Fe<L>, u_inv: Fe<L> },
    /// Degree-3 CM endomorphism (e.g. `D = −11` with `ψ = (1 + √−11)/2`), costing
    /// **`6M + 2S + 0I`**:
    /// `d = x − x₀`, `d₂ = d²`, `vd = v·d`,
    /// `X' = x·d₂ + vd + u₃`, `Y' = y·(d·d₂ − vd − 2·u₃)`, `Z' = d·u⁻¹`.
    Deg3 {
        x0: Fe<L>,
        v: Fe<L>,
        u3: Fe<L>,
        two_u3: Fe<L>,
        u_inv: Fe<L>,
    },
}

impl<const L: usize> CmLowDegreeEndo<L> {
    pub fn new_deg2(curve: &Curve<L>, x0_big: &BigUint, u_inv_big: &BigUint) -> Result<Self, String> {
        let f = &curve.f;
        let x0 = f.from_canonical(x0_big)?;
        let u_inv = f.from_canonical(u_inv_big)?;
        let v = f.add(&f.mul_small(&f.sqr(&x0), 3), &curve.a);
        Ok(Self::Deg2 { x0, v, u_inv })
    }

    pub fn new_deg3(curve: &Curve<L>, x0_big: &BigUint, u_inv_big: &BigUint) -> Result<Self, String> {
        let f = &curve.f;
        let x0 = f.from_canonical(x0_big)?;
        let u_inv = f.from_canonical(u_inv_big)?;
        let x0_2 = f.sqr(&x0);
        let v = f.mul_small(&f.add(&f.mul_small(&x0_2, 3), &curve.a), 2);
        let fx0 = f.add(&f.add(&f.mul(&x0_2, &x0), &f.mul(&curve.a, &x0)), &curve.b);
        let u3 = f.mul_small(&fx0, 4);
        let two_u3 = f.add(&u3, &u3);
        Ok(Self::Deg3 {
            x0,
            v,
            u3,
            two_u3,
            u_inv,
        })
    }

    #[inline]
    pub fn apply(&self, f: &GenericField<L>, p: &Affine<L>) -> Jacobian<L> {
        match self {
            Self::Deg2 { x0, v, u_inv } => {
                let d = f.sub(&p.x, x0);
                let dd = f.sqr(&d);
                let x_jac = f.mul(&f.add(&f.mul(&p.x, &d), v), &d);
                let y_jac = f.mul(&f.mul(&p.y, &f.sub(&dd, v)), &d);
                let z_jac = f.mul(&d, u_inv);
                Jacobian {
                    x: x_jac,
                    y: y_jac,
                    z: z_jac,
                }
            }
            Self::Deg3 {
                x0,
                v,
                u3,
                two_u3,
                u_inv,
            } => {
                let d = f.sub(&p.x, x0);
                let d2 = f.sqr(&d);
                let d3 = f.mul(&d2, &d);
                let vd = f.mul(v, &d);
                let x_jac = f.add(&f.add(&f.mul(&p.x, &d2), &vd), u3);
                let y_jac = f.mul(&p.y, &f.sub(&f.sub(&d3, &vd), two_u3));
                let z_jac = f.mul(&d, u_inv);
                Jacobian {
                    x: x_jac,
                    y: y_jac,
                    z: z_jac,
                }
            }
        }
    }
}

/// Compute a short Gauss/half-GCD basis `[[v10, v11], [v20, v21]]` of the GLV
/// kernel lattice `L = {(x, y) ∈ Z² : x + y·λ ≡ 0 (mod r)}` with `|det| = r`.
pub fn gauss_reduce_glv_basis(order: &BigUint, lambda: &BigUint) -> [[BigInt; 2]; 2] {
    let n = BigInt::from(order.clone());
    let lam = BigInt::from(lambda % order);
    let sqrt_n = order.sqrt();
    let bound = BigInt::from(sqrt_n);

    let (mut r0, mut r1) = (n.clone(), lam);
    let (mut t0, mut t1) = (BigInt::zero(), BigInt::one());

    while r1 > bound {
        let q = &r0 / &r1;
        let r2 = &r0 - &q * &r1;
        let t2 = &t0 - &q * &t1;
        r0 = r1;
        r1 = r2;
        t0 = t1;
        t1 = t2;
    }

    // First short vector: (r1, -t1). Candidate neighbours: (r0, -t0) and (r2, -t2).
    let v1 = [r1.clone(), -&t1];
    let cand_a = [r0.clone(), -&t0];
    let cand_b = if r1.is_zero() {
        cand_a.clone()
    } else {
        let q = &r0 / &r1;
        [&r0 - &q * &r1, -(&t0 - &q * &t1)]
    };
    let norm_sq = |v: &[BigInt; 2]| &v[0] * &v[0] + &v[1] * &v[1];
    let mut v2 = if norm_sq(&cand_a) <= norm_sq(&cand_b) {
        cand_a
    } else {
        cand_b
    };

    // Full Lagrange/Gauss 2D lattice reduction pass for minimal orthogonality defect.
    let mut a = v1;
    let mut b = v2;
    loop {
        if norm_sq(&b) < norm_sq(&a) {
            core::mem::swap(&mut a, &mut b);
        }
        let dot = &a[0] * &b[0] + &a[1] * &b[1];
        let na = norm_sq(&a);
        let mu = round_div_bigint(&dot, &na);
        if mu.is_zero() {
            v2 = b;
            return [a, v2];
        }
        b = [&b[0] - &mu * &a[0], &b[1] - &mu * &a[1]];
    }
}

#[inline]
fn round_div_bigint(num: &BigInt, den: &BigInt) -> BigInt {
    let (num, den) = if den.sign() == Sign::Minus {
        (-num, -den)
    } else {
        (num.clone(), den.clone())
    };
    (num * 2u8 + &den).div_floor(&(den * 2u8))
}

/// Prepared small-discriminant CM GLV-2 context (`D ∈ {−7, −8, −11, ...}`).
#[derive(Clone, Debug)]
pub struct CmLowDegreeGlv<const L: usize> {
    pub curve: Curve<L>,
    pub endo: CmLowDegreeEndo<L>,
    pub lambda: BigUint,
    pub order: BigUint,
    pub basis: [[BigInt; 2]; 2],
    det: BigInt,
    two_det: BigInt,
}

#[inline]
fn stack_wnaf_bigint(mag: &BigUint, w: u32, out: &mut [i8; 260]) -> usize {
    let mut limbs = [0u64; 4];
    for (i, &d) in mag.to_u64_digits().iter().take(4).enumerate() {
        limbs[i] = d;
    }
    let mask = (1u64 << w) - 1;
    let half = 1i64 << (w - 1);
    let full = 1i64 << w;
    let mut len = 0usize;
    while (limbs[0] | limbs[1] | limbs[2] | limbs[3]) != 0 {
        if (limbs[0] & 1) == 1 {
            let mut d = (limbs[0] & mask) as i64;
            if d >= half {
                d -= full;
                let add_v = (-d) as u64;
                let (l0, c1) = limbs[0].overflowing_add(add_v);
                let (l1, c2) = limbs[1].overflowing_add(c1 as u64);
                let (l2, c3) = limbs[2].overflowing_add(c2 as u64);
                limbs[0] = l0;
                limbs[1] = l1;
                limbs[2] = l2;
                limbs[3] = limbs[3].wrapping_add(c3 as u64);
            } else {
                limbs[0] &= !mask;
            }
            out[len] = d as i8;
        } else {
            out[len] = 0;
        }
        limbs[0] = (limbs[0] >> 1) | (limbs[1] << 63);
        limbs[1] = (limbs[1] >> 1) | (limbs[2] << 63);
        limbs[2] = (limbs[2] >> 1) | (limbs[3] << 63);
        limbs[3] >>= 1;
        len += 1;
    }
    len
}

impl<const L: usize> CmLowDegreeGlv<L> {
    pub fn new(
        curve: Curve<L>,
        endo: CmLowDegreeEndo<L>,
        order: BigUint,
        lambda: BigUint,
    ) -> Result<Self, String> {
        let lambda = lambda % &order;
        if lambda.is_zero() {
            return Err("GLV eigenvalue lambda must be non-zero mod r".into());
        }
        let basis = gauss_reduce_glv_basis(&order, &lambda);
        let det = &basis[0][0] * &basis[1][1] - &basis[0][1] * &basis[1][0];
        if det.magnitude() != &order {
            return Err("Gauss-reduced GLV basis determinant is not ±r".into());
        }
        let n_int = BigInt::from(order.clone());
        let lam_int = BigInt::from(lambda.clone());
        for row in &basis {
            if !(&row[0] + &row[1] * &lam_int).mod_floor(&n_int).is_zero() {
                return Err("GLV basis vector does not satisfy v0 + v1*lambda == 0 (mod r)".into());
            }
        }
        let two_det = if det.sign() == Sign::Minus {
            -&det << 1usize
        } else {
            &det << 1usize
        };
        Ok(Self {
            curve,
            endo,
            lambda,
            order,
            basis,
            det,
            two_det,
        })
    }

    #[inline]
    fn round_div_det(&self, num: BigInt) -> BigInt {
        let adj = if self.det.sign() == Sign::Minus {
            (-num << 1usize) - &self.det
        } else {
            (num << 1usize) + &self.det
        };
        adj.div_floor(&self.two_det)
    }

    /// Babai nearest-plane decomposition `k ≡ k₁ + k₂·λ (mod r)` with `|k₁|, |k₂| ≲ √r`.
    #[inline]
    pub fn decompose(&self, k: &BigUint) -> (BigInt, BigInt) {
        let k_int = BigInt::from(k % &self.order);
        let r1 = self.round_div_det(&k_int * &self.basis[1][1]);
        let r2 = self.round_div_det(-&k_int * &self.basis[0][1]);
        let k1 = &k_int - &r1 * &self.basis[0][0] - &r2 * &self.basis[1][0];
        let k2 = -(&r1 * &self.basis[0][1]) - &r2 * &self.basis[1][1];
        (k1, k2)
    }

    /// Zero-inversion, zero-heap-allocation isomorphic `Π`-scaled `madd` GLV-2 kernel
    /// for `w = 4` (`1, 3, 5, 7` odd multiples of `P` and `ψ(P)`):
    /// lifts all 8 table entries simultaneously to a common denominator `Π = ∏ Zᵢ` on the
    /// isomorphic curve `ỹ² = x̃³ + (Π⁴·a)x̃ + (Π⁶·b)` in `0` inversions so every Shamir
    /// table addition uses `madd-2007-bl` (`7M + 4S`) instead of `add-2007-bl` (`11M + 5S`),
    /// then maps back to `E` at the end with one multiply `Z = Z̃ · Π`.
    fn scalar_mul_w4_isomorphic_madd(
        &self,
        p: &Affine<L>,
        k1: &BigInt,
        k2: &BigInt,
    ) -> Jacobian<L> {
        let c = &self.curve;
        let f = &c.f;
        let neg1 = k1.sign() == Sign::Minus;
        let neg2 = k2.sign() == Sign::Minus;
        let mut d1 = [0i8; 260];
        let mut d2 = [0i8; 260];
        let len1 = stack_wnaf_bigint(k1.magnitude(), 4, &mut d1);
        let len2 = stack_wnaf_bigint(k2.magnitude(), 4, &mut d2);
        let top = len1.max(len2);
        if top == 0 {
            return c.identity();
        }

        let p_jac = c.from_affine(p);
        let p2 = c.double(&p_jac);
        let p3 = c.add_affine(&p2, p);
        let p5 = c.add(&p3, &p2);
        let p7 = c.add(&p5, &p2);

        let q1 = self.endo.apply(f, p);
        let q2 = c.double(&q1);
        let q3 = c.add(&q1, &q2);
        let q5 = c.add(&q3, &q2);
        let q7 = c.add(&q5, &q2);

        // If any rare small-order table entry hit identity, fall back safely:
        let raw = [p3, p5, p7, q1, q3, q5, q7];
        if raw.iter().any(|pt| pt.is_identity()) {
            return glv_combine(c, p, &q1, k1, k2, 4, TableMode::Jacobian);
        }

        // Prefix/suffix products over the 7 non-unit Z coordinates (18M, 0 inversions!):
        let mut pref = [f.one(); 7];
        pref[0] = raw[0].z;
        for i in 1..7 {
            pref[i] = f.mul(&pref[i - 1], &raw[i].z);
        }
        let pi = pref[6];
        let mut suf = f.one();
        let mut cofac = [f.one(); 7];
        for i in (1..7).rev() {
            cofac[i] = f.mul(&pref[i - 1], &suf);
            suf = f.mul(&suf, &raw[i].z);
        }
        cofac[0] = suf;

        let pi2 = f.sqr(&pi);
        let pi3 = f.mul(&pi2, &pi);
        let pi4 = f.sqr(&pi2);
        let a_tilde = f.mul(&c.a, &pi4);

        // Rescale 1P (`Z = 1`, cofactor = Π) and the 7 Jacobian entries onto `Ẽ`:
        let mut tab1 = [*p; 4];
        tab1[0] = Affine {
            x: f.mul(&p.x, &pi2),
            y: f.mul(&p.y, &pi3),
        };
        for j in 0..3 {
            let cf2 = f.sqr(&cofac[j]);
            let cf3 = f.mul(&cf2, &cofac[j]);
            tab1[j + 1] = Affine {
                x: f.mul(&raw[j].x, &cf2),
                y: f.mul(&raw[j].y, &cf3),
            };
        }
        let mut tab2 = [*p; 4];
        for j in 0..4 {
            let cf2 = f.sqr(&cofac[3 + j]);
            let cf3 = f.mul(&cf2, &cofac[3 + j]);
            tab2[j] = Affine {
                x: f.mul(&raw[3 + j].x, &cf2),
                y: f.mul(&raw[3 + j].y, &cf3),
            };
        }

        let dbl_tilde = |pt: &Jacobian<L>| -> Jacobian<L> {
            if pt.is_identity() || pt.y.is_zero() {
                return c.identity();
            }
            match c.dbl {
                DoublingKind::AZero => c.double(pt),
                DoublingKind::AMinus3 => {
                    // `ã = -3 · Π⁴` ⇒ `3X² + ã·Z⁴ = 3(X − Π²·Z²)(X + Π²·Z²)` (`4M + 5S`)!
                    let delta = f.sqr(&pt.z);
                    let pi2_delta = f.mul(&pi2, &delta);
                    let gamma = f.sqr(&pt.y);
                    let beta = f.mul(&pt.x, &gamma);
                    let t = f.mul(&f.sub(&pt.x, &pi2_delta), &f.add(&pt.x, &pi2_delta));
                    let alpha = f.add(&f.add(&t, &t), &t);
                    let beta2 = f.add(&beta, &beta);
                    let beta4 = f.add(&beta2, &beta2);
                    let beta8 = f.add(&beta4, &beta4);
                    let x3 = f.sub(&f.sqr(&alpha), &beta8);
                    let z3 = f.sub(&f.sub(&f.sqr(&f.add(&pt.y, &pt.z)), &gamma), &delta);
                    let g2 = f.sqr(&gamma);
                    let g4 = f.add(&g2, &g2);
                    let g8 = f.add(&g4, &g4);
                    let g8 = f.add(&g8, &g8);
                    let y3 = f.sub(&f.mul(&alpha, &f.sub(&beta4, &x3)), &g8);
                    Jacobian {
                        x: x3,
                        y: y3,
                        z: z3,
                    }
                }
                _ => {
                    // `dbl-2007-bl` with `ã = Π⁴·a` (`2M + 8S`, 0 extra ops vs `E`!):
                    let xx = f.sqr(&pt.x);
                    let yy = f.sqr(&pt.y);
                    let yyyy = f.sqr(&yy);
                    let zz = f.sqr(&pt.z);
                    let s_tmp = f.sub(&f.sub(&f.sqr(&f.add(&pt.x, &yy)), &xx), &yyyy);
                    let s = f.add(&s_tmp, &s_tmp);
                    let m = f.add(&f.add(&f.add(&xx, &xx), &xx), &f.mul(&a_tilde, &f.sqr(&zz)));
                    let x3 = f.sub(&f.sqr(&m), &f.add(&s, &s));
                    let y4_2 = f.add(&yyyy, &yyyy);
                    let y4_4 = f.add(&y4_2, &y4_2);
                    let y4_8 = f.add(&y4_4, &y4_4);
                    let y3 = f.sub(&f.mul(&m, &f.sub(&s, &x3)), &y4_8);
                    let z3 = f.sub(&f.sub(&f.sqr(&f.add(&pt.y, &pt.z)), &yy), &zz);
                    Jacobian {
                        x: x3,
                        y: y3,
                        z: z3,
                    }
                }
            }
        };

        let pick = |tab: &[Affine<L>; 4], d: i8, flip: bool| -> Affine<L> {
            let neg = (d < 0) ^ flip;
            let idx = ((d.unsigned_abs() as usize) - 1) >> 1;
            let mut e = tab[idx];
            if neg {
                e.y = f.neg(&e.y);
            }
            e
        };

        let mut acc = c.identity();
        for i in (0..top).rev() {
            if !acc.is_identity() {
                acc = dbl_tilde(&acc);
            }
            let a_d = if i < len1 { d1[i] } else { 0 };
            if a_d != 0 {
                acc = c.add_affine(&acc, &pick(&tab1, a_d, neg1));
            }
            let b_d = if i < len2 { d2[i] } else { 0 };
            if b_d != 0 {
                acc = c.add_affine(&acc, &pick(&tab2, b_d, neg2));
            }
        }

        if acc.is_identity() {
            return acc;
        }
        Jacobian {
            x: acc.x,
            y: acc.y,
            z: f.mul(&acc.z, &pi),
        }
    }

    /// Execute `[k]P = [k₁]P + [k₂]ψ(P)` using our explicit `5M+1S` / `6M+2S` CM Vélu
    /// endomorphism and zero-allocation `Π`-isomorphic `madd` Shamir table (`w = 4`,
    /// `TableMode::Jacobian`) or batched affine table (`TableMode::Affine`).
    pub fn scalar_mul_jacobian(
        &self,
        p: &Affine<L>,
        k: &BigUint,
        w: u32,
        table: TableMode,
    ) -> Jacobian<L> {
        let k_mod = k % &self.order;
        if k_mod.is_zero() {
            return self.curve.identity();
        }
        let (k1, k2) = self.decompose(&k_mod);
        if w == 4 && matches!(table, TableMode::Jacobian) && self.order.bits() <= 256 {
            return self.scalar_mul_w4_isomorphic_madd(p, &k1, &k2);
        }
        let psi_p = self.endo.apply(&self.curve.f, p);
        glv_combine(&self.curve, p, &psi_p, &k1, &k2, w, table)
    }

    pub fn scalar_mul_affine(
        &self,
        p: &Affine<L>,
        k: &BigUint,
        w: u32,
        table: TableMode,
    ) -> Option<Affine<L>> {
        let jac = self.scalar_mul_jacobian(p, k, w, table);
        self.curve.to_affine(&jac)
    }
}

// ── Horizontal Cl(O_D) Volcano Multi-Curve Orbit DLP ────────────────────────

/// Single-word Shoup prime-field helper (`p < 2^31`) for fast horizontal CM volcano
/// isogeny transport and x-only negation-folded BSGS.
#[derive(Clone, Copy, Debug)]
struct CraterFp {
    p: u64,
}

impl CraterFp {
    #[inline(always)]
    fn add(self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p { s - self.p } else { s }
    }
    #[inline(always)]
    fn sub(self, a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + self.p - b }
    }
    #[inline(always)]
    fn neg(self, a: u64) -> u64 {
        if a == 0 { 0 } else { self.p - a }
    }
    #[inline(always)]
    fn mul(self, a: u64, b: u64) -> u64 {
        (a * b) % self.p
    }
    #[inline]
    fn inv(self, a: u64) -> u64 {
        let (mut old_r, mut r) = (a, self.p);
        let (mut old_s, mut s) = (1i64, 0i64);
        while r != 0 {
            let q = old_r / r;
            (old_r, r) = (r, old_r - q * r);
            (old_s, s) = (s, old_s - (q as i64) * s);
        }
        if old_s < 0 {
            (old_s + self.p as i64) as u64
        } else {
            old_s as u64
        }
    }
}

/// One curve `E_i: y² = x³ + aᵢx + bᵢ` in a horizontal `ℓ = 3` CM volcano crater,
/// with an independent subgroup generator `g` of prime order `r` and optional
/// horizontal 3-isogeny kernel `ker_to_next` mapping `E_i → E_{i+1}`.
#[derive(Clone, Copy, Debug)]
pub struct CraterCurveNode {
    pub a: u64,
    pub b: u64,
    pub g: (u64, u64),
    pub j_invariant: u64,
    pub ker_to_next: Option<u64>,
}

/// An ECDLP target `(curve_idx, P, Q = [k]P)` residing on one of the `H` non-isomorphic
/// curves in a horizontal CM crater `Cl(O_D)`.
#[derive(Clone, Copy, Debug)]
pub struct CraterDlpInstance {
    pub curve_idx: usize,
    pub base: (u64, u64),
    pub target: (u64, u64),
}

/// **Horizontal `Cl(O_D)` Isogeny Volcano Multi-Curve DLP Solver**:
/// transports ECDLP instances `(E_c, P_i, Q_i = [k_i]P_i)` across explicit horizontal
/// degree-3 Vélu isogenies `Φ_c: E_c → E_{H-1}` onto one canonical crater curve and
/// solves all targets against **one shared negation-folded BSGS baby-step table** via
/// `k_i ≡ log_{G*}(Φ_c(Q_i)) · log_{G*}(Φ_c(P_i))⁻¹ (mod r)`, avoiding `H` separate
/// per-curve baby-step tables even when every curve uses an independent base point!
#[derive(Clone, Debug)]
pub struct HorizontalCmVolcanoDlp {
    pub p: u64,
    pub order: u64,
    pub nodes: Vec<CraterCurveNode>,
}

impl HorizontalCmVolcanoDlp {
    fn curve_add(f: CraterFp, a: u64, p1: Option<(u64, u64)>, p2: Option<(u64, u64)>) -> Option<(u64, u64)> {
        let (Some((x1, y1)), Some((x2, y2))) = (p1, p2) else {
            return p1.or(p2);
        };
        let lam = if x1 == x2 {
            if f.add(y1, y2) == 0 {
                return None;
            }
            let num = f.add(f.mul(3, f.mul(x1, x1)), a);
            f.mul(num, f.inv(f.add(y1, y1)))
        } else {
            f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
        };
        let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
        Some((x3, y3))
    }

    pub fn scalar_mul_node(&self, curve_idx: usize, pt: (u64, u64), mut k: u64) -> Option<(u64, u64)> {
        let f = CraterFp { p: self.p };
        let a = self.nodes[curve_idx].a;
        let mut acc = None;
        let mut base = Some(pt);
        while k > 0 {
            if k & 1 == 1 {
                acc = Self::curve_add(f, a, acc, base);
            }
            k >>= 1;
            if k > 0 {
                base = Self::curve_add(f, a, base, base);
            }
        }
        acc
    }

    /// Evaluate the horizontal Vélu 3-isogeny chain `Φ_c: E_c → E_{H-1}` on `pt`.
    #[inline]
    pub fn transport_to_canonical(&self, mut curve_idx: usize, mut pt: (u64, u64)) -> (u64, u64) {
        let f = CraterFp { p: self.p };
        let last = self.nodes.len() - 1;
        while curve_idx < last {
            let node = &self.nodes[curve_idx];
            let x0 = node.ker_to_next.expect("non-terminal crater node has ker_to_next");
            let v = f.mul(2, f.add(f.mul(3, f.mul(x0, x0)), node.a));
            let fx0 = f.add(
                f.add(f.mul(f.mul(x0, x0), x0), f.mul(node.a, x0)),
                node.b,
            );
            let u3 = f.mul(4, fx0);
            let d = f.sub(pt.0, x0);
            let di = f.inv(d);
            let di2 = f.mul(di, di);
            let di3 = f.mul(di2, di);
            let nx = f.add(f.add(pt.0, f.mul(v, di)), f.mul(u3, di2));
            let factor = f.sub(f.sub(1, f.mul(v, di2)), f.mul(f.mul(2, u3), di3));
            let ny = f.mul(pt.1, factor);
            pt = (nx, ny);
            curve_idx += 1;
        }
        pt
    }

    /// Batch-add `acc_pts[i] += step_pt` across `K` points using 1 shared field inversion.
    fn batch_step_inplace(
        f: CraterFp,
        a: u64,
        pts: &mut [Option<(u64, u64)>],
        step: (u64, u64),
    ) {
        let mut dens = [1u64; 64];
        let mut pref = [1u64; 64];
        let mut active = [usize::MAX; 64];
        let mut cnt = 0usize;
        for (i, slot) in pts.iter().enumerate() {
            if let Some((x1, _)) = *slot {
                if x1 != step.0 {
                    let d = f.sub(step.0, x1);
                    dens[cnt] = d;
                    pref[cnt] = if cnt == 0 { d } else { f.mul(pref[cnt - 1], d) };
                    active[cnt] = i;
                    cnt += 1;
                }
            }
        }
        let mut invs = [0u64; 64];
        if cnt > 0 {
            let mut inv_acc = f.inv(pref[cnt - 1]);
            for k in (1..cnt).rev() {
                invs[k] = f.mul(inv_acc, pref[k - 1]);
                inv_acc = f.mul(inv_acc, dens[k]);
            }
            invs[0] = inv_acc;
        }
        let mut ptr = 0usize;
        for i in 0..pts.len() {
            if ptr < cnt && active[ptr] == i {
                let (x1, y1) = pts[i].unwrap();
                let lam = f.mul(f.sub(step.1, y1), invs[ptr]);
                let x3 = f.sub(f.sub(f.mul(lam, lam), x1), step.0);
                let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                pts[i] = Some((x3, y3));
                ptr += 1;
            } else {
                pts[i] = Self::curve_add(f, a, pts[i], Some(step));
            }
        }
    }

    /// Negation-folded x-only BSGS on a single curve `(a, g)` solving a slice of targets
    /// `[Q_0, ..., Q_{k-1}]` with respect to generator `g` of prime order `r`:
    /// baby-step table stores `j ∈ [0, m)` (`m = ⌈√(r/2)⌉`), giant step jumps by `2m·g`.
    fn bsgs_solve_on_curve(
        f: CraterFp,
        a: u64,
        r: u64,
        g: (u64, u64),
        targets: &[(u64, u64)],
    ) -> Vec<u64> {
        let m = (((r as f64) * 0.5).sqrt().ceil() as u64) + 1;
        // Open-addressed power-of-two hash table keyed by `x + 1` (0 = empty):
        let cap = ((m as usize) * 2).next_power_of_two();
        let mask = cap - 1;
        let mut keys = vec![0u64; cap];
        let mut vals = vec![(0u64, 0u32); cap]; // (y, j)

        // Build baby steps `j·g` for `j = 1..=m` using 16-way batched chord additions:
        const LANES: usize = 16;
        let mut lane_pts: [Option<(u64, u64)>; LANES] = [None; LANES];
        let mut cur = None;
        for l in 0..LANES {
            cur = Self::curve_add(f, a, cur, Some(g));
            lane_pts[l] = cur;
        }
        let jump16 = cur.expect("16g != O");
        let mut base_j = 1u32;
        while (base_j as u64) <= m {
            for l in 0..LANES {
                let j = base_j + (l as u32);
                if (j as u64) > m {
                    break;
                }
                if let Some((bx, by)) = lane_pts[l] {
                    let key = bx + 1;
                    let mut slot = ((bx.wrapping_mul(0x9E37_79B9_7F4A_7C15)) as usize) & mask;
                    while keys[slot] != 0 && keys[slot] != key {
                        slot = (slot + 1) & mask;
                    }
                    if keys[slot] == 0 {
                        keys[slot] = key;
                        vals[slot] = (by, j);
                    }
                }
            }
            base_j += LANES as u32;
            if (base_j as u64) <= m {
                Self::batch_step_inplace(f, a, &mut lane_pts, jump16);
            }
        }

        // Giant step `-2m·g`? With negation folding: `Q - i·(2m·g) = ±j·g` ⇒ `k = 2m·i ± j`!
        let mut mg = None;
        let mut base_g = Some(g);
        let mut rem = 2 * m;
        while rem > 0 {
            if rem & 1 == 1 {
                mg = Self::curve_add(f, a, mg, base_g);
            }
            rem >>= 1;
            if rem > 0 {
                base_g = Self::curve_add(f, a, base_g, base_g);
            }
        }
        let neg_2mg = mg.map(|(x, y)| (x, f.neg(y))).expect("2mg != O");
        let max_i = (r / (2 * m)) + 2;

        let mut out = vec![0u64; targets.len()];
        let mut solved = vec![false; targets.len()];
        let mut rem_targets = targets.len();
        let mut walkers: Vec<Option<(u64, u64)>> = targets.iter().map(|&t| Some(t)).collect();

        for i in 0..=max_i {
            for idx in 0..targets.len() {
                if solved[idx] {
                    continue;
                }
                let Some((wx, wy)) = walkers[idx] else {
                    out[idx] = (2 * m * i) % r;
                    solved[idx] = true;
                    rem_targets -= 1;
                    continue;
                };
                let key = wx + 1;
                let mut slot = ((wx.wrapping_mul(0x9E37_79B9_7F4A_7C15)) as usize) & mask;
                while keys[slot] != 0 {
                    if keys[slot] == key {
                        let (by, j) = vals[slot];
                        let k_val = if wy == by {
                            (2 * m * i + (j as u64)) % r
                        } else {
                            (2 * m * i + r - (j as u64)) % r
                        };
                        out[idx] = k_val;
                        solved[idx] = true;
                        rem_targets -= 1;
                        break;
                    }
                    slot = (slot + 1) & mask;
                }
            }
            if rem_targets == 0 {
                break;
            }
            for chunk in walkers.chunks_mut(32) {
                Self::batch_step_inplace(f, a, chunk, neg_2mg);
            }
        }
        out
    }

    /// **Baseline Arm A**: Solve multi-curve ECDLP instances independently on each curve
    /// `E_c` (building a separate BSGS baby-step table per distinct curve `E_c`).
    pub fn solve_independent_per_curve(&self, instances: &[CraterDlpInstance]) -> Vec<u64> {
        let f = CraterFp { p: self.p };
        let r_mod = CraterFp { p: self.order };
        let mut ans = vec![0u64; instances.len()];
        for c_idx in 0..self.nodes.len() {
            let mut idxs = Vec::new();
            let mut pts = Vec::new();
            for (k, inst) in instances.iter().enumerate() {
                if inst.curve_idx == c_idx {
                    idxs.push(k);
                    pts.push(inst.base);
                    pts.push(inst.target);
                }
            }
            if idxs.is_empty() {
                continue;
            }
            let node = &self.nodes[c_idx];
            let logs = Self::bsgs_solve_on_curve(f, node.a, self.order, node.g, &pts);
            for (t, &orig_idx) in idxs.iter().enumerate() {
                let log_p = logs[2 * t];
                let log_q = logs[2 * t + 1];
                ans[orig_idx] = r_mod.mul(log_q, r_mod.inv(log_p));
            }
        }
        ans
    }

    /// **CM Orbit Arm B**: Push all `(P_i, Q_i)` across horizontal 3-isogenies `Φ_c` onto
    /// the canonical curve `E_{H-1}` and solve **every instance across all `H` curves in
    /// one shared BSGS baby-step table** via `k_i = log(Φ(Q_i)) · log(Φ(P_i))⁻¹ mod r`!
    pub fn solve_via_horizontal_cm_orbit(&self, instances: &[CraterDlpInstance]) -> Vec<u64> {
        let f = CraterFp { p: self.p };
        let r_mod = CraterFp { p: self.order };
        let mut pushed = Vec::with_capacity(2 * instances.len());
        for inst in instances {
            pushed.push(self.transport_to_canonical(inst.curve_idx, inst.base));
            pushed.push(self.transport_to_canonical(inst.curve_idx, inst.target));
        }
        let canon = self.nodes.last().expect("non-empty crater");
        let logs = Self::bsgs_solve_on_curve(f, canon.a, self.order, canon.g, &pushed);
        let mut ans = Vec::with_capacity(instances.len());
        for i in 0..instances.len() {
            ans.push(r_mod.mul(logs[2 * i + 1], r_mod.inv(logs[2 * i])));
        }
        ans
    }
}

/// Certified 256-bit prime-field curve fixtures for benchmarking and regression tests.
pub mod fixtures {
    use super::*;

    fn hex(s: &str) -> BigUint {
        BigUint::parse_bytes(s.as_bytes(), 16).expect("valid hex constant")
    }

    /// Certified 256-bit curve (`j = −32768 ∉ {0, 1728}`, `#E(F_p) = 4·r` with 254-bit
    /// prime `r`, `p ≡ 7 (mod 8)`) whose three rational 2-torsion slopes `3·rᵢ² + a`
    /// all have Legendre symbol `-1`: `E` admits **no** direct `F_p`-Montgomery or
    /// twisted Edwards model, yet bridges onto a Montgomery ladder via `φ₂`.
    pub fn non_montgomery_256_bridge() -> (TwoIsogenyMontgomeryBridge<4>, Affine<4>) {
        let p = hex("b58cba3b0443a61571836fcfc1d099f2862f24f3526a4c9c2a7af6808a1d782f");
        let a = hex("b58cba3b0443a61571836fcfc1d099f2862f24f3526a4c9c2a7af6808a1d7727");
        let b = hex("b58cba3b0443a61571836fcfc1d099f2862f24f3526a4c9c2a7af6808a1d7191");
        let r = hex("2d632e8ec110e9855c60dbf3f074267c3d6b2630b61eb6ba979011487c8bafd7");
        let r0 = hex("65e59e7079c36468d21bdd4e4046ec0f24c337d66323c3a8e3b6e2435f657810");
        let r1 = hex("0be61d3b5f7b1117d9fc8e0958635ac5c359ebf32b9c7067567b9655890ec6d6");
        let r2 = hex("43c0fe8f2b053094c56b04782926531d9e120129c3aa188bf0487de7a1a93949");
        let gx = hex("38bf107ff459588dafd0169d8c1da7587973a312bb4beafb559287eb73b8d8e7");
        let gy = hex("a39710ee3fa602460d33f8669cc91cba3661f3ce7059b9d4ae3db28343a36988");

        let curve = Curve::<4>::new(&p, &a, &b).expect("valid 256-bit curve");
        let g = curve.affine(&gx, &gy).expect("valid generator");
        let bridge =
            TwoIsogenyMontgomeryBridge::new(curve, [r0, r1, r2], r).expect("valid 2-isogeny bridge");
        (bridge, g)
    }

    /// Certified 256-bit CM curve with discriminant `D = −7` (`y² = x³ − 35x − 98`,
    /// 254-bit prime subgroup `r`) equipped with the degree-2 endomorphism
    /// `ψ = (1 + √−7)/2` (`5M + 1S`).
    pub fn cm_dm7_deg2_256() -> (CmLowDegreeGlv<4>, Affine<4>) {
        let p = hex("86e63d4339662de596ce574e80eeae2e491b841bd22d9d1da5961eb8b3ab82cd");
        let a = hex("86e63d4339662de596ce574e80eeae2e491b841bd22d9d1da5961eb8b3ab82aa");
        let b = hex("86e63d4339662de596ce574e80eeae2e491b841bd22d9d1da5961eb8b3ab826b");
        let r = hex("21b98f50ce598b7965b395d3a03bab8b5e35037fb866452e4a9e35dd1075048f");
        let gx = hex("2b6162e39d709765f1885c9e3762ff1888cbf2e1f27793234b8da1ea253cf931");
        let gy = hex("2d5c648a6c2540b0428380ad297813163dced645ec9cb6cf3d13c05baf31300b");
        let x0 = hex("04ba39882a279aacb8b079630ab9db8dc21e2eee94f835c9fef4cb988871d3bd");
        let u_inv = hex("04ba39882a279aacb8b079630ab9db8dc21e2eee94f835c9fef4cb988871d3c1");
        let lam = hex("1ed38de3cf4518d5394856bd49d26ddcd7bc3bcf4ba0cdfbf7553cedf13a6825");

        let curve = Curve::<4>::new(&p, &a, &b).expect("valid D=-7 curve");
        let g = curve.affine(&gx, &gy).expect("valid D=-7 generator");
        let endo = CmLowDegreeEndo::new_deg2(&curve, &x0, &u_inv).expect("valid deg-2 endo");
        let glv = CmLowDegreeGlv::new(curve, endo, r, lam).expect("valid D=-7 GLV");
        (glv, g)
    }

    /// Certified 256-bit CM curve with discriminant `D = −8` (`a = −3`, 255-bit prime
    /// subgroup `r`) equipped with the degree-2 endomorphism `ψ = √−2` (`5M + 1S`).
    pub fn cm_dm8_deg2_256() -> (CmLowDegreeGlv<4>, Affine<4>) {
        let p = hex("f968629150d8c4c667397e8bfffeea56e53b830c1764cddfc83c0e0710901423");
        let a = &p - BigUint::from(3u8);
        let b = hex("b06044e583e30ceea41b656686ac677002b948371433674e4029ff1705ad3257");
        let r = hex("7cb43148a86c6263339cbf45ffff752c5e5a64980168b6f95f8dfba6d8598891");
        let gx = hex("994827f24158f61747d16bc28b5efb4a678662e3046b1cf83ca38931caf2e521");
        let gy = hex("5ad3792d3795c742480a4ef7bcf95bdbe5263ff09bb510facf4352ca93d28566");
        let x0 = hex("7dfb9eed150fe4aa753823db84c449e24b16a14be9db92eec01dff59965723f5");
        let u_inv = hex("c7b8c1dc81852a197de8f012f005322d043a528b58b6660a5335551b9bf929e6");
        let lam = hex("415cc0647687ba9e2e6310c5120b52ab5f4ce9fe6e08ede6a9ea16048c369d56");

        let curve = Curve::<4>::new(&p, &a, &b).expect("valid D=-8 curve");
        let g = curve.affine(&gx, &gy).expect("valid D=-8 generator");
        let endo = CmLowDegreeEndo::new_deg2(&curve, &x0, &u_inv).expect("valid deg-2 endo");
        let glv = CmLowDegreeGlv::new(curve, endo, r, lam).expect("valid D=-8 GLV");
        (glv, g)
    }

    /// Certified 256-bit prime-order CM curve with discriminant `D = −11` (`a = −3`,
    /// `#E(F_p) = r` 256-bit prime, `j = −32768`) equipped with the degree-3
    /// endomorphism `ψ = (1 + √−11)/2` (`6M + 2S`).
    pub fn cm_dm11_deg3_256() -> (CmLowDegreeGlv<4>, Affine<4>) {
        let p = hex("cd3f8021be942e8b2307a4e888617c18b9c1bf2c69981af5ad596c7186348333");
        let a = &p - BigUint::from(3u8);
        let b = hex("89b1a19d701831608529310e6152529544b2fc94d4a0c4882164632a563088d4");
        let r = hex("cd3f8021be942e8b2307a4e888617c17ce5569fb0235d4b43e7eeef2acb07d49");
        let gx = hex("2b6a5318095a4b7d63afa693a9f117df05bc041574b0e313092256babbf36f24");
        let gy = hex("51970d1e2f1cec3fab1a5c0d64226bf4d781cfa5af63a7ffbdc69e8a1e94c35f");
        let x0 = hex("177823150fc877149a2c8c6c6d11d75d6012d6b570e4cbe576187a7fe3987cae");
        let u_inv = hex("73d19c3eddbd03b487599ea03d33470951c89ccb6fd5fde82dddaa2950dec688");
        let lam = hex("6e80d32c65da8a8d729cfc08a278298f999f791d5d0e7b2dfbc52b431be8310a");

        let curve = Curve::<4>::new(&p, &a, &b).expect("valid D=-11 curve");
        let g = curve.affine(&gx, &gy).expect("valid D=-11 generator");
        let endo = CmLowDegreeEndo::new_deg3(&curve, &x0, &u_inv).expect("valid deg-3 endo");
        let glv = CmLowDegreeGlv::new(curve, endo, r, lam).expect("valid D=-11 GLV");
        (glv, g)
    }

    /// Certified 6-curve horizontal `ℓ = 3` CM volcano crater over `p = 38,394,667`
    /// with prime subgroup order `r = 5,484,301` (`#E = 7·r`), consisting of six
    /// pairwise non-isomorphic curves (`j ∈ {35448289, 22973782, 30442456, 30748310,
    /// 25577789, 23758938}`) linked by exact un-rescaled horizontal Vélu 3-isogenies
    /// `E₀ → E₁ → E₂ → E₃ → E₄ → E₅`.
    pub fn horizontal_cm_crater_6curves() -> HorizontalCmVolcanoDlp {
        HorizontalCmVolcanoDlp {
            p: 38_394_667,
            order: 5_484_301,
            nodes: vec![
                CraterCurveNode {
                    a: 28_106_635,
                    b: 14_821_600,
                    g: (36_669_201, 38_102_472),
                    j_invariant: 35_448_289,
                    ker_to_next: Some(32_431_268),
                },
                CraterCurveNode {
                    a: 6_357_181,
                    b: 9_024_023,
                    g: (4_317_379, 37_224_232),
                    j_invariant: 22_973_782,
                    ker_to_next: Some(5_620_462),
                },
                CraterCurveNode {
                    a: 36_609_651,
                    b: 33_584_435,
                    g: (20_706_674, 15_235_783),
                    j_invariant: 30_442_456,
                    ker_to_next: Some(3_090_066),
                },
                CraterCurveNode {
                    a: 18_245_400,
                    b: 32_709_641,
                    g: (16_625_304, 31_083_937),
                    j_invariant: 30_748_310,
                    ker_to_next: Some(14_033_600),
                },
                CraterCurveNode {
                    a: 20_109_486,
                    b: 5_619_296,
                    g: (28_408_492, 30_745_995),
                    j_invariant: 25_577_789,
                    ker_to_next: Some(20_851_881),
                },
                CraterCurveNode {
                    a: 16_058_937,
                    b: 1_629_565,
                    g: (6_203_666, 15_118_711),
                    j_invariant: 23_758_938,
                    ker_to_next: None,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::*;
    use super::*;
    use crate::ecc::endo_generic_point::{jacobian_to_biguint, scalar_mul_wnaf, textbook_mul};
    use num_bigint::RandBigInt;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn two_isogeny_montgomery_bridge_matches_weierstrass_across_edge_and_random_scalars() {
        let (bridge, g) = non_montgomery_256_bridge();
        // Verify algebraically that E itself has NO direct F_p Montgomery model!
        assert!(
            !bridge.has_direct_montgomery_model(),
            "fixture must have no direct F_p Montgomery model"
        );
        assert!(bridge.curve.is_on_curve(&g));

        let rcb = RcbWeierstrassLadder::new(bridge.curve.clone(), bridge.scalar_bits);
        let r = &bridge.order;

        // Test on multiple base points: G, [3]G, -[5]G
        let g3 = bridge
            .curve
            .to_affine(&scalar_mul_wnaf(
                &bridge.curve,
                &g,
                &BigUint::from(3u8),
                4,
                TableMode::Affine,
            ))
            .unwrap();
        let neg_g5 = bridge.curve.neg_affine(
            &bridge
                .curve
                .to_affine(&scalar_mul_wnaf(
                    &bridge.curve,
                    &g,
                    &BigUint::from(5u8),
                    4,
                    TableMode::Affine,
                ))
                .unwrap(),
        );

        let mut rng = StdRng::seed_from_u64(0x2026_1118);
        let mut scalars = vec![
            BigUint::zero(),
            BigUint::one(),
            BigUint::from(2u8),
            BigUint::from(3u8),
            BigUint::from(4u8),
            BigUint::from(7u8),
            r - BigUint::from(3u8),
            r - BigUint::from(2u8), // hits Z1 == 0 Okeya-Sakurai edge case!
            r - BigUint::one(),
            r.clone(),
            r + BigUint::one(),
            r * BigUint::from(2u8),
        ];
        for _ in 0..48 {
            scalars.push(rng.gen_biguint_below(r));
        }

        for base in [g, g3, neg_g5] {
            let prep = bridge.prepare_point(&base);
            for k in &scalars {
                let k_mod = k % r;
                let expected = jacobian_to_biguint(
                    &bridge.curve,
                    &scalar_mul_wnaf(&bridge.curve, &base, &k_mod, 5, TableMode::Affine),
                );
                let got_unprep =
                    jacobian_to_biguint(&bridge.curve, &bridge.scalar_mul_jacobian(&base, k));
                let got_prep = jacobian_to_biguint(
                    &bridge.curve,
                    &bridge.scalar_mul_prepared_jacobian(&prep, &base, k),
                );
                let got_rcb =
                    jacobian_to_biguint(&bridge.curve, &rcb.scalar_mul_jacobian(&base, &k_mod));
                assert_eq!(got_unprep, expected, "unprepared bridge mismatch at k={k}");
                assert_eq!(got_prep, expected, "prepared bridge mismatch at k={k}");
                assert_eq!(got_rcb, expected, "RCB ladder mismatch at k={k}");
            }
        }
        // Spot-check independent BigUint textbook oracle as well:
        let k_spot = &scalars[15];
        assert_eq!(
            jacobian_to_biguint(&bridge.curve, &bridge.scalar_mul_jacobian(&g, k_spot)),
            textbook_mul(&bridge.curve, &g, k_spot)
        );
    }

    fn verify_cm_glv_fixture(glv: &CmLowDegreeGlv<4>, g: &Affine<4>, label: &str) {
        assert!(glv.curve.is_on_curve(g), "{label}: generator not on curve");
        // 1. Verify [lambda]G == psi(G) exactly:
        let psi_g = jacobian_to_biguint(&glv.curve, &glv.endo.apply(&glv.curve.f, g));
        let lam_g = jacobian_to_biguint(
            &glv.curve,
            &scalar_mul_wnaf(&glv.curve, g, &glv.lambda, 5, TableMode::Affine),
        );
        assert_eq!(psi_g, lam_g, "{label}: psi(G) != [lambda]G");

        // 2. Verify edge + 48 random 256-bit scalars across both Affine and Jacobian tables:
        let r = &glv.order;
        let max_half_bits = (r.bits() / 2) + 3;
        let mut rng = StdRng::seed_from_u64(0x5eed_cafe);
        let mut scalars = vec![
            BigUint::zero(),
            BigUint::one(),
            BigUint::from(2u8),
            glv.lambda.clone(),
            &glv.lambda + BigUint::one(),
            r - BigUint::from(2u8),
            r - BigUint::one(),
            r.clone(),
            r + BigUint::from(5u8),
        ];
        for _ in 0..48 {
            scalars.push(rng.gen_biguint_below(r));
        }

        for k in &scalars {
            let (k1, k2) = glv.decompose(k);
            assert!(
                k1.bits() <= max_half_bits && k2.bits() <= max_half_bits,
                "{label}: sub-scalars too wide: ({}, {}) > {max_half_bits}",
                k1.bits(),
                k2.bits()
            );
            let expected = jacobian_to_biguint(
                &glv.curve,
                &scalar_mul_wnaf(&glv.curve, g, &(k % r), 5, TableMode::Affine),
            );
            let got_aff = jacobian_to_biguint(
                &glv.curve,
                &glv.scalar_mul_jacobian(g, k, 4, TableMode::Affine),
            );
            let got_jac = jacobian_to_biguint(
                &glv.curve,
                &glv.scalar_mul_jacobian(g, k, 4, TableMode::Jacobian),
            );
            assert_eq!(got_aff, expected, "{label} affine-table GLV mismatch at k={k}");
            assert_eq!(got_jac, expected, "{label} jacobian-table GLV mismatch at k={k}");
        }
    }

    #[test]
    fn cm_dm7_deg2_glv_matches_weierstrass() {
        let (glv, g) = cm_dm7_deg2_256();
        verify_cm_glv_fixture(&glv, &g, "CM D=-7 deg-2");
    }

    #[test]
    fn cm_dm8_deg2_glv_matches_weierstrass() {
        let (glv, g) = cm_dm8_deg2_256();
        verify_cm_glv_fixture(&glv, &g, "CM D=-8 deg-2");
    }

    #[test]
    fn cm_dm11_deg3_glv_matches_weierstrass() {
        let (glv, g) = cm_dm11_deg3_256();
        verify_cm_glv_fixture(&glv, &g, "CM D=-11 deg-3");
    }

    #[test]
    fn horizontal_cm_volcano_crater_dlp_recovers_all_multi_curve_scalars() {
        use rand::Rng;
        let crater = horizontal_cm_crater_6curves();
        // Verify all 6 j-invariants are pairwise distinct (non-isomorphic over F_p):
        let mut js: Vec<u64> = crater.nodes.iter().map(|n| n.j_invariant).collect();
        js.sort_unstable();
        js.dedup();
        assert_eq!(js.len(), 6, "all 6 crater curves must be non-isomorphic");

        // Verify independent base points + 24 random ECDLP instances across all 6 curves:
        let mut rng = StdRng::seed_from_u64(0x2026_1119);
        let mut instances = Vec::with_capacity(24);
        let mut secrets = Vec::with_capacity(24);
        for i in 0..24usize {
            let c_idx = i % 6;
            let base_mult = rng.gen_range(2..crater.order);
            let secret = rng.gen_range(1..crater.order);
            let base = crater
                .scalar_mul_node(c_idx, crater.nodes[c_idx].g, base_mult)
                .unwrap();
            let target = crater.scalar_mul_node(c_idx, base, secret).unwrap();
            instances.push(CraterDlpInstance {
                curve_idx: c_idx,
                base,
                target,
            });
            secrets.push(secret);
        }

        let got_indep = crater.solve_independent_per_curve(&instances);
        let got_orbit = crater.solve_via_horizontal_cm_orbit(&instances);
        assert_eq!(got_indep, secrets);
        assert_eq!(got_orbit, secrets);
    }
}
