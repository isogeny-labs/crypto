//! **Optimal Ate pairing** on BLS12-381.
//!
//! `e: G1 × G2 → F_{p¹²}` — a bilinear, non-degenerate map.  We
//! compute it via the Miller loop + final exponentiation.
//!
//! ## The loop length
//!
//! BLS12-381 parameter `u = -0xd201000000010000`.  The Miller loop
//! iterates over the bits of `|u|`.  Since `u < 0`, the result is
//! conjugated at the end.
//!
//! `|u| = 0xd201000000010000` has bit length 64.
//!
//! ## Final exponentiation
//!
//! `f^((p¹² − 1) / r)`.  Decomposed via the Frobenius-orbit
//! structure into:
//!
//! 1. **Easy part**: `f^(p⁶ − 1) · f^(p² + 1)` — uses cheap
//!    Frobenius and one inversion.
//! 2. **Hard part**: `f^((p⁴ − p² + 1) / r)` — the cyclotomic
//!    polynomial Φ₁₂(p) / r.  For BLS curves this admits an
//!    efficient addition chain (Fuentes-Castañeda-Knapp-Rodríguez-
//!    Henríquez 2011).
//!
//! ## Educational scope
//!
//! This implementation **prioritises clarity over speed**.  All
//! ops use `BigUint` and explicit point doublings — a single
//! pairing takes seconds, not microseconds.  Production
//! implementations use Montgomery-form 64-bit limbs and special
//! line-function evaluation tricks; see `blst`.
//!
//! ## Twist convention (M-twist)
//!
//! The tower in [`super::fq2`], [`super::fq6`], [`super::fq12`] fixes
//! `u² = −1`, `v³ = ξ = u + 1`, and `w² = v`, so `w⁶ = ξ = u + 1`.
//! [`super::g2`] defines `E'(Fq²): y_T² = x_T³ + 4(u + 1) = x_T³ + 4·w⁶`,
//! which is the **M-type** sextic twist of `E(Fp): y² = x³ + 4`.
//! Dividing `y_T² = x_T³ + 4·w⁶` by `w⁶` yields the untwist map
//! `ψ_M: E'(Fq²) → E(Fq¹²)`:
//! - `ψ_M(x_T, y_T) = (x_T · w⁻², y_T · w⁻³) ∈ E(Fq¹²)`, where
//!   `w⁻² = ξ⁻¹ · v²` and `w⁻³ = ξ⁻¹ · v · w`.
//! - Slope on `E(Fq¹²)`: `λ_E = dy_E / dx_E = λ_{E'} · w⁻¹`, where
//!   `w⁻¹ = ξ⁻¹ · v² · w`.
//! - Untwisted chord/tangent line evaluated at `P = (x_P, y_P) ∈ E(Fp)`:
//!   `l(P) = y_P − y_T · w⁻³ − λ_{E'} · w⁻¹ · (x_P − x_T · w⁻²)`.
//! - Because the BLS12-381 parameter `x = −0xd201000000010000` is
//!   negative, `f_{x, Q}(P)` is obtained from `f_{|x|, Q}(P)` by
//!   unitary conjugation after final exponentiation.

use super::fq::{modulus, Fq};
use super::fq12::Fq12;
use super::fq2::Fq2;
use super::fq6::Fq6;
use super::g1::G1Point;
use super::g2::G2Point;
use num_bigint::BigUint;
use num_traits::One;
use std::sync::OnceLock;

/// BLS12-381 parameter `|u|` (the bit string we loop over).
fn u_abs() -> &'static BigUint {
    static U_ABS: OnceLock<BigUint> = OnceLock::new();
    U_ABS.get_or_init(|| BigUint::parse_bytes(b"d201000000010000", 16).unwrap())
}

/// Cached `ξ⁻¹ = (u + 1)⁻¹ ∈ F_{p²}` used by the M-twist untwist map.
fn xi_inv() -> &'static Fq2 {
    static XI_INV: OnceLock<Fq2> = OnceLock::new();
    XI_INV.get_or_init(|| Fq2::new(Fq::one(), Fq::one()).inverse().unwrap())
}

/// Line function for doubling `T` evaluated at `P` (line through
/// `T` tangent to the curve at `T`).  Returns the resulting
/// `Fq12` value alongside the new point `2T`.
///
/// Standard affine-twist formulas (Costello-Lange-Naehrig 2010,
/// adapted to BLS12-381 curve y² = x³ + 4(u+1) over F_p²).
fn line_double(t: &G2Point, p: &G1Point) -> (Fq12, G2Point) {
    let (tx, ty) = match t {
        G2Point::Affine { x, y } => (x.clone(), y.clone()),
        G2Point::Infinity => return (Fq12::one(), G2Point::Infinity),
    };
    let (px, py) = match p {
        G1Point::Affine { x, y } => (x.clone(), y.clone()),
        G1Point::Infinity => return (Fq12::one(), t.clone()),
    };
    if ty.is_zero() {
        let vertical = fq_to_fq12(&px).sub(&fq2_to_fq12_at_w_inv2(&tx));
        return (vertical, G2Point::Infinity);
    }

    // Slope λ = (3·x²) / (2·y).
    let three = Fq2::new(Fq::new(BigUint::from(3u32)), Fq::zero());
    let two = Fq2::new(Fq::new(BigUint::from(2u32)), Fq::zero());
    let lambda = three
        .mul(&tx.square())
        .mul(&two.mul(&ty).inverse().unwrap());

    // New T' = 2T.
    let new_x = lambda.square().sub(&tx).sub(&tx);
    let new_y = lambda.mul(&tx.sub(&new_x)).sub(&ty);
    let new_t = G2Point::Affine { x: new_x, y: new_y };

    let line = line_value(&lambda, &px, &py, &tx, &ty);
    (line, new_t)
}

/// Line function for `T + Q` evaluated at `P`.  Returns the Fq12
/// value and the new point.
fn line_add(t: &G2Point, q: &G2Point, p: &G1Point) -> (Fq12, G2Point) {
    let (tx, ty) = match t {
        G2Point::Affine { x, y } => (x.clone(), y.clone()),
        G2Point::Infinity => return (Fq12::one(), q.clone()),
    };
    let (qx, qy) = match q {
        G2Point::Affine { x, y } => (x.clone(), y.clone()),
        G2Point::Infinity => return (Fq12::one(), t.clone()),
    };
    let (px, py) = match p {
        G1Point::Affine { x, y } => (x.clone(), y.clone()),
        G1Point::Infinity => return (Fq12::one(), t.clone()),
    };

    if tx == qx {
        if ty == qy {
            return line_double(t, p);
        }
        // T + (-T) = O; vertical line x_P − x_T·w⁻² through untwisted T.
        let vertical = fq_to_fq12(&px).sub(&fq2_to_fq12_at_w_inv2(&tx));
        return (vertical, G2Point::Infinity);
    }

    let lambda = qy.sub(&ty).mul(&qx.sub(&tx).inverse().unwrap());
    let new_x = lambda.square().sub(&tx).sub(&qx);
    let new_y = lambda.mul(&tx.sub(&new_x)).sub(&ty);
    let new_point = G2Point::Affine { x: new_x, y: new_y };

    let line = line_value(&lambda, &px, &py, &tx, &ty);
    (line, new_point)
}

/// **Direct M-twist line evaluation** in `Fq12`.
///
/// Untwists `T ∈ E'(Fq²)` to `ψ_M(T) = (x_T · w⁻², y_T · w⁻³) ∈ E(Fq¹²)`,
/// embeds `P ∈ E(Fp)` trivially, and evaluates
/// `l(P) = y_P − y_T · w⁻³ − (λ · w⁻¹) · (x_P − x_T · w⁻²)`.
fn line_value(lambda: &Fq2, p_x: &Fq, p_y: &Fq, t_x: &Fq2, t_y: &Fq2) -> Fq12 {
    let xp_fq12 = fq_to_fq12(p_x);
    let yp_fq12 = fq_to_fq12(p_y);
    let tx_fq12 = fq2_to_fq12_at_w_inv2(t_x);
    let ty_fq12 = fq2_to_fq12_at_w_inv3(t_y);
    let lambda_w_inv = fq2_to_fq12_at_w_inv(lambda);
    let dx = xp_fq12.sub(&tx_fq12);
    let lambda_dx = lambda_w_inv.mul(&dx);
    let dy = yp_fq12.sub(&ty_fq12);
    dy.sub(&lambda_dx)
}

/// Embed `a ∈ Fp` into `Fq12` at the constant slot (`c0_fq6.c0.c0`).
fn fq_to_fq12(a: &Fq) -> Fq12 {
    let a_fq2 = Fq2::new(a.clone(), Fq::zero());
    let c0_fq6 = Fq6::new(a_fq2, Fq2::zero(), Fq2::zero());
    Fq12::new(c0_fq6, Fq6::zero())
}

/// Embed `a · w⁻²` into `Fq12`: since `w⁶ = ξ` and `w² = v`,
/// `w⁻² = v⁻¹ = ξ⁻¹ · v²` (`c0_fq6.c2`).
fn fq2_to_fq12_at_w_inv2(a: &Fq2) -> Fq12 {
    let c0_fq6 = Fq6::new(Fq2::zero(), Fq2::zero(), a.mul(xi_inv()));
    Fq12::new(c0_fq6, Fq6::zero())
}

/// Embed `a · w⁻³` into `Fq12`: since `w⁶ = ξ`,
/// `w⁻³ = ξ⁻¹ · w³ = ξ⁻¹ · v · w` (`c1_fq6.c1`).
fn fq2_to_fq12_at_w_inv3(a: &Fq2) -> Fq12 {
    let c1_fq6 = Fq6::new(Fq2::zero(), a.mul(xi_inv()), Fq2::zero());
    Fq12::new(Fq6::zero(), c1_fq6)
}

/// Embed `a · w⁻¹` into `Fq12`: since `w⁶ = ξ`,
/// `w⁻¹ = ξ⁻¹ · w⁵ = ξ⁻¹ · v² · w` (`c1_fq6.c2`).
fn fq2_to_fq12_at_w_inv(a: &Fq2) -> Fq12 {
    let c1_fq6 = Fq6::new(Fq2::zero(), Fq2::zero(), a.mul(xi_inv()));
    Fq12::new(Fq6::zero(), c1_fq6)
}

/// Compute the Miller-loop value `f_{|u|, Q}(P)` for BLS12-381.
fn miller_loop(p: &G1Point, q: &G2Point) -> Fq12 {
    if p.is_infinity() || q.is_infinity() {
        return Fq12::one();
    }
    let u = u_abs();
    let bits = u.bits();
    let mut f = Fq12::one();
    let mut t = q.clone();
    for i in (0..(bits - 1)).rev() {
        f = f.square();
        let (line, new_t) = line_double(&t, p);
        f = f.mul(&line);
        t = new_t;
        if u.bit(i) {
            let (line, new_t) = line_add(&t, q, p);
            f = f.mul(&line);
            t = new_t;
        }
    }
    f
}

fn p_squared() -> &'static BigUint {
    static P2: OnceLock<BigUint> = OnceLock::new();
    P2.get_or_init(|| {
        let p = modulus();
        &p * &p
    })
}

fn hard_part_exponent() -> &'static BigUint {
    static EXP: OnceLock<BigUint> = OnceLock::new();
    EXP.get_or_init(|| {
        let p2 = p_squared();
        let p4 = p2 * p2;
        let r = super::fq::scalar_modulus();
        (&p4 - p2 + BigUint::one()) / &r
    })
}

/// Final exponentiation: `f^((p¹² − 1) / r)`.
fn final_exponentiation(f: &Fq12) -> Fq12 {
    let Some(f_inv) = f.inverse() else {
        return Fq12::zero();
    };
    // Easy part: f^(p⁶ − 1) · f^(p² + 1).
    // Since Fq12 = Fq6[w]/(w² − v) is a quadratic extension of Fq6,
    // raising to p⁶ is exact conjugation a + b·w ↦ a − b·w.
    let f_p6 = f.conjugate();
    let f_easy_1 = f_p6.mul(&f_inv);
    let f_easy_p2 = f_easy_1.pow(p_squared());
    let f_easy = f_easy_p2.mul(&f_easy_1);

    // Hard part: raise to (p⁴ − p² + 1) / r.
    f_easy.pow(hard_part_exponent())
}

/// Compute `e(P, Q) ∈ F_{p¹²}` for `P ∈ G1, Q ∈ G2`.
///
/// Conjugates after final exponentiation to account for the negative
/// BLS12-381 parameter `u = −0xd201000000010000`.
pub fn pairing(p: &G1Point, q: &G2Point) -> Fq12 {
    let f = miller_loop(p, q);
    let f_exp = final_exponentiation(&f);
    f_exp.conjugate()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Untwist sanity check: `ψ_M(G2) = (x_G2 · w⁻², y_G2 · w⁻³)` lies on
    /// `E(Fq¹²): Y² = X³ + 4`.
    #[test]
    fn untwisted_g2_generator_lies_on_e_fq12() {
        let G2Point::Affine { x, y } = G2Point::generator() else {
            panic!("G2 generator is affine");
        };
        let x_e = fq2_to_fq12_at_w_inv2(&x);
        let y_e = fq2_to_fq12_at_w_inv3(&y);
        let lhs = y_e.square();
        let rhs = x_e
            .square()
            .mul(&x_e)
            .add(&fq_to_fq12(&Fq::new(BigUint::from(4u32))));
        assert_eq!(lhs, rhs, "ψ_M(G2) must satisfy Y² = X³ + 4 in Fq12");
    }

    /// Non-degeneracy and `r`-torsion: `e(G1, G2) ≠ 1` and `e(G1, G2)^r = 1`.
    #[test]
    fn pairing_nonzero() {
        let g1 = G1Point::generator();
        let g2 = G2Point::generator();
        let e = pairing(&g1, &g2);
        assert_ne!(
            e,
            Fq12::one(),
            "e(G1, G2) should be a non-trivial root of unity"
        );
        let r = super::super::fq::scalar_modulus();
        let test = e.pow(&r);
        assert_eq!(
            test,
            Fq12::one(),
            "pairing output must be an r-th root of unity"
        );
    }

    /// Pairing of infinity is one in either argument.
    #[test]
    fn pairing_with_infinity_is_one() {
        let g1 = G1Point::generator();
        let g2 = G2Point::generator();
        assert_eq!(pairing(&G1Point::Infinity, &g2), Fq12::one());
        assert_eq!(pairing(&g1, &G2Point::Infinity), Fq12::one());
    }

    /// **Bilinearity in G1**: `e([2]P, Q) == e(P, Q)²`.
    #[test]
    fn pairing_bilinear_in_g1() {
        let g1 = G1Point::generator();
        let g2 = G2Point::generator();
        let g1_doubled = g1.double();
        let e_p_q = pairing(&g1, &g2);
        let e_2p_q = pairing(&g1_doubled, &g2);
        let e_p_q_sq = e_p_q.square();
        assert_ne!(e_p_q, Fq12::one());
        assert_eq!(e_2p_q, e_p_q_sq, "e([2]P, Q) must equal e(P, Q)²");
    }

    /// **Bilinearity in G2**: `e(P, [2]Q) == e(P, Q)²` and `e(P, −Q) == e(P, Q)⁻¹`.
    #[test]
    fn pairing_bilinear_in_g2() {
        let g1 = G1Point::generator();
        let g2 = G2Point::generator();
        let g2_doubled = g2.double();
        let e_p_q = pairing(&g1, &g2);
        let e_p_2q = pairing(&g1, &g2_doubled);
        let e_p_q_sq = e_p_q.square();
        assert_ne!(e_p_q, Fq12::one());
        assert_eq!(e_p_2q, e_p_q_sq, "e(P, [2]Q) must equal e(P, Q)²");
        let e_p_neg_q = pairing(&g1, &g2.neg());
        assert_eq!(e_p_q.mul(&e_p_neg_q), Fq12::one());
    }
}
