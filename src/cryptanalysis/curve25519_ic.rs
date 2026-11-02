//! Exact Curve25519 model binding for prime-field index-calculus research.
//!
//! The prime-field Semaev/PKM machinery in this crate is written for short
//! Weierstrass curves, while RFC 7748 specifies Curve25519 as
//!
//! `v^2 = u^3 + A*u^2 + u`, `A = 486662`, over `F_(2^255-19)`.
//!
//! The translation `x = u + A/3`, `y = v` gives
//! `y^2 = x^3 + a*x + b`, with
//! `a = 1 - A^2/3` and `b = 2*A^3/27 - A/3`. This module makes that
//! binding executable at the full field size and evaluates the standard
//! third Semaev polynomial there. It is a representation adapter, not a
//! claim that index calculus beats generic rho on Curve25519.
//!
//! The accompanying study and replay artifacts are in
//! `research/curve25519_index_calculus_20261102/`.

use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::cryptanalysis::identity_certificate::{Expr, IdentityStatement};
use crate::ecc::curve::CurveParams;
use crate::ecc::curve25519;

const RFC_BASE_V_DECIMAL: &[u8] =
    b"14781619447589544791020593568409986887264606134616475288964881837755586237401";

/// The exact model and prime-order subgroup used by the study.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curve25519IcModel {
    pub p: BigUint,
    pub montgomery_a: BigUint,
    pub subgroup_order: BigUint,
    pub cofactor: u32,
    pub base_u: BigUint,
    pub base_v: BigUint,
    pub short_a: BigUint,
    pub short_b: BigUint,
    /// `A/3 mod p`; `x = u + x_shift`.
    pub x_shift: BigUint,
    pub base_x: BigUint,
    pub base_y: BigUint,
}

/// Capacity of the binary split-torus factor-base constructions at this
/// field prime. These are arithmetic limits, not measured solver results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PkmBinaryTowerLimits {
    /// Maximum `t` for a Kummer base `g * mu_(2^t)` in `F_p`.
    pub kummer_height_from_p_minus_1: u32,
    pub kummer_abscissae: u64,
    /// Maximum `t` for the corresponding norm-one/Dickson binary tower.
    pub torus_height_from_p_plus_1: u32,
    pub torus_abscissae: u64,
}

fn add_mod(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    (a + b) % p
}

fn sub_mod(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    let a = a % p;
    let b = b % p;
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

fn mul_mod(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    (a * b) % p
}

fn square_mod(a: &BigUint, p: &BigUint) -> BigUint {
    mul_mod(a, a, p)
}

fn two_adicity(mut n: BigUint) -> u32 {
    assert!(!n.is_zero(), "two-adicity is undefined at zero");
    let mut v = 0;
    while !n.bit(0) {
        n >>= 1usize;
        v += 1;
    }
    v
}

impl Curve25519IcModel {
    /// RFC 7748 Curve25519, restricted to its specified prime-order subgroup.
    pub fn rfc7748() -> Self {
        let p = curve25519::p();
        let montgomery_a = BigUint::from(486_662u32);
        let three_inv = BigUint::from(3u32).modpow(&(&p - 2u32), &p);
        let twenty_seven_inv = BigUint::from(27u32).modpow(&(&p - 2u32), &p);
        let x_shift = mul_mod(&montgomery_a, &three_inv, &p);

        // Depress u^3 + A*u^2 + u by u = x - A/3.
        let a_squared_over_three = mul_mod(&square_mod(&montgomery_a, &p), &three_inv, &p);
        let short_a = sub_mod(&BigUint::one(), &a_squared_over_three, &p);
        let a_cubed = mul_mod(&square_mod(&montgomery_a, &p), &montgomery_a, &p);
        let two_a_cubed_over_twenty_seven = mul_mod(
            &mul_mod(&BigUint::from(2u32), &a_cubed, &p),
            &twenty_seven_inv,
            &p,
        );
        let short_b = sub_mod(&two_a_cubed_over_twenty_seven, &x_shift, &p);

        let subgroup_order = (BigUint::one() << 252usize)
            + BigUint::parse_bytes(b"14def9dea2f79cd65812631a5cf5d3ed", 16)
                .expect("fixed RFC 7748 order suffix");
        let base_u = BigUint::from(9u32);
        let base_v = BigUint::parse_bytes(RFC_BASE_V_DECIMAL, 10)
            .expect("fixed RFC 7748 base-point ordinate");
        let base_x = add_mod(&base_u, &x_shift, &p);
        let base_y = base_v.clone();

        Self {
            p,
            montgomery_a,
            subgroup_order,
            cofactor: 8,
            base_u,
            base_v,
            short_a,
            short_b,
            x_shift,
            base_x,
            base_y,
        }
    }

    /// Parameters consumable by the crate's exact `BigUint` short-Weierstrass
    /// point arithmetic. `n` is the prime subgroup order and `h = 8`.
    pub fn short_weierstrass_params(&self) -> CurveParams {
        CurveParams {
            name: "Curve25519-short-Weierstrass-IC",
            p: self.p.clone(),
            a: self.short_a.clone(),
            b: self.short_b.clone(),
            gx: self.base_x.clone(),
            gy: self.base_y.clone(),
            n: self.subgroup_order.clone(),
            h: self.cofactor,
        }
    }

    /// Map a finite Montgomery point to the short-Weierstrass model.
    pub fn montgomery_to_short(&self, u: &BigUint, v: &BigUint) -> (BigUint, BigUint) {
        (add_mod(u, &self.x_shift, &self.p), v % &self.p)
    }

    /// Inverse of [`Self::montgomery_to_short`] on finite points.
    pub fn short_to_montgomery(&self, x: &BigUint, y: &BigUint) -> (BigUint, BigUint) {
        (sub_mod(x, &self.x_shift, &self.p), y % &self.p)
    }

    pub fn is_on_montgomery(&self, u: &BigUint, v: &BigUint) -> bool {
        let lhs = square_mod(v, &self.p);
        let u2 = square_mod(u, &self.p);
        let u3 = mul_mod(&u2, u, &self.p);
        let rhs = add_mod(
            &add_mod(&u3, &mul_mod(&self.montgomery_a, &u2, &self.p), &self.p),
            u,
            &self.p,
        );
        lhs == rhs
    }

    pub fn is_on_short(&self, x: &BigUint, y: &BigUint) -> bool {
        let lhs = square_mod(y, &self.p);
        let x2 = square_mod(x, &self.p);
        let rhs = add_mod(
            &add_mod(
                &mul_mod(&x2, x, &self.p),
                &mul_mod(&self.short_a, x, &self.p),
                &self.p,
            ),
            &self.short_b,
            &self.p,
        );
        lhs == rhs
    }

    /// The standard third Semaev polynomial for the exact short model.
    /// It vanishes iff signs can be chosen on points with these abscissae
    /// so that their sum is the identity (over the algebraic closure).
    pub fn semaev_s3(&self, x1: &BigUint, x2: &BigUint, x3: &BigUint) -> BigUint {
        let p = &self.p;
        let d = sub_mod(x1, x2, p);
        let sum = add_mod(x1, x2, p);
        let product = mul_mod(x1, x2, p);

        let term1 = mul_mod(&square_mod(&d, p), &square_mod(x3, p), p);
        let middle_inner = add_mod(
            &mul_mod(&sum, &add_mod(&product, &self.short_a, p), p),
            &mul_mod(&BigUint::from(2u32), &self.short_b, p),
            p,
        );
        let term2 = mul_mod(&BigUint::from(2u32), &mul_mod(&middle_inner, x3, p), p);
        let product_minus_a = sub_mod(&product, &self.short_a, p);
        let term3 = sub_mod(
            &square_mod(&product_minus_a, p),
            &mul_mod(&BigUint::from(4u32), &mul_mod(&self.short_b, &sum, p), p),
            p,
        );
        sub_mod(&add_mod(&term1, &term3, p), &term2, p)
    }

    /// Exact binary-tower capacities implied by `p - 1` and `p + 1`.
    pub fn pkm_binary_tower_limits(&self) -> PkmBinaryTowerLimits {
        let minus = two_adicity(&self.p - BigUint::one());
        let plus = two_adicity(&self.p + BigUint::one());
        PkmBinaryTowerLimits {
            kummer_height_from_p_minus_1: minus,
            kummer_abscissae: 1u64 << minus,
            torus_height_from_p_plus_1: plus,
            torus_abscissae: 1u64 << plus,
        }
    }

    /// Machine-readable exact-model handoff. Values not quoted directly from
    /// RFC 7748 are labelled `derived` in the record.
    pub fn evidence_manifest(&self) -> serde_json::Value {
        let limits = self.pkm_binary_tower_limits();
        serde_json::json!({
            "schema": "curve25519-index-calculus-model/v1",
            "status": "exact_model_adapter_not_an_attack",
            "standard_source": "RFC 7748 section 4.1",
            "curve": {
                "name": "Curve25519",
                "field_prime": self.p.to_string(),
                "montgomery_equation": "v^2 = u^3 + 486662*u^2 + u",
                "prime_subgroup_order": self.subgroup_order.to_string(),
                "cofactor": self.cofactor,
                "base_u": self.base_u.to_string(),
                "base_v": self.base_v.to_string()
            },
            "short_weierstrass_adapter": {
                "provenance": "derived_exactly_mod_p",
                "map": "x = u + A/3; y = v",
                "inverse": "u = x - A/3; v = y",
                "x_shift_A_over_3": self.x_shift.to_string(),
                "a": self.short_a.to_string(),
                "b": self.short_b.to_string(),
                "base_x": self.base_x.to_string(),
                "base_y": self.base_y.to_string()
            },
            "pkm_binary_tower_limits": {
                "provenance": "derived_exactly_from_p_plus_or_minus_1",
                "v2_p_minus_1": limits.kummer_height_from_p_minus_1,
                "max_kummer_abscissae": limits.kummer_abscissae,
                "v2_p_plus_1": limits.torus_height_from_p_plus_1,
                "max_binary_torus_abscissae": limits.torus_abscissae,
                "scope": "binary Kummer and norm-one/Dickson towers only; auxiliary-isogeny and mixed-radix PKM constructions remain separate"
            },
            "implementation_boundary": {
                "exact_field_arithmetic": "BigUint",
                "semaev_polynomial": "S3 on the exact short-Weierstrass model",
                "existing_end_to_end_pkm_harness": "single-word p < 2^63; not wired to this exact model",
                "performance_measurement": false
            }
        })
    }
}

fn expr_pow(x: Expr, exponent: usize) -> Expr {
    Expr::product((0..exponent).map(|_| x.clone()).collect())
}

/// Integer-polynomial identity behind the Montgomery translation:
///
/// `27(u^3 + A*u^2 + u)`
/// `= (3u+A)^3 + 3(3-A^2)(3u+A) + 2A^3 - 9A`.
///
/// Dividing by 27 in a field of characteristic other than 3 gives the short
/// model used above. The committed `identity.certificate/v1` record evaluates
/// this statement at derived points independently of Curve25519's constants.
pub fn montgomery_translation_identity() -> IdentityStatement {
    let u = Expr::var("u");
    let a = Expr::var("A");
    let three_u_plus_a = Expr::plus(Expr::times(Expr::Const(3), u.clone()), a.clone());
    let lhs = Expr::times(
        Expr::Const(27),
        Expr::sum(vec![
            expr_pow(u.clone(), 3),
            Expr::product(vec![a.clone(), expr_pow(u.clone(), 2)]),
            u,
        ]),
    );
    let three_minus_a2 = Expr::plus(Expr::Const(3), Expr::minus(expr_pow(a.clone(), 2)));
    let rhs = Expr::sum(vec![
        expr_pow(three_u_plus_a.clone(), 3),
        Expr::product(vec![Expr::Const(3), three_minus_a2, three_u_plus_a]),
        Expr::times(Expr::Const(2), expr_pow(a.clone(), 3)),
        Expr::times(Expr::Const(-9), a),
    ]);
    IdentityStatement {
        name: "depress the Curve25519 Montgomery cubic by x = u + A/3".to_string(),
        source: "research/curve25519_index_calculus_20261102/README.md; direct integer expansion of the Montgomery-to-short-Weierstrass translation".to_string(),
        variables: vec!["A".to_string(), "u".to_string()],
        degree_bound: 3,
        lhs,
        rhs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::identity_certificate::{
        check_with_extra, IdentityCertificate, Verdict,
    };
    use crate::ecc::point::Point;

    #[test]
    fn rfc_base_maps_to_the_exact_short_model_and_back() {
        let model = Curve25519IcModel::rfc7748();
        assert!(model.is_on_montgomery(&model.base_u, &model.base_v));
        assert!(model.is_on_short(&model.base_x, &model.base_y));
        let short = model.montgomery_to_short(&model.base_u, &model.base_v);
        assert_eq!(short, (model.base_x.clone(), model.base_y.clone()));
        assert_eq!(
            model.short_to_montgomery(&short.0, &short.1),
            (model.base_u.clone(), model.base_v.clone())
        );
    }

    #[test]
    fn rfc_base_generates_the_declared_prime_order_subgroup() {
        let model = Curve25519IcModel::rfc7748();
        let curve = model.short_weierstrass_params();
        let generator = curve.generator();
        assert!(curve.is_on_curve(&generator));
        assert_eq!(
            generator.scalar_mul_vartime(&model.subgroup_order, &curve.a_fe()),
            Point::Infinity
        );
    }

    #[test]
    fn exact_s3_vanishes_on_a_zero_sum_triple() {
        let model = Curve25519IcModel::rfc7748();
        let curve = model.short_weierstrass_params();
        let g = curve.generator();
        let two_g = g.double_vartime(&curve.a_fe());
        let minus_three_g = g.add_vartime(&two_g, &curve.a_fe()).neg_vartime();
        let xs = [
            g.x_coord().unwrap(),
            two_g.x_coord().unwrap(),
            minus_three_g.x_coord().unwrap(),
        ];
        assert!(model.semaev_s3(xs[0], xs[1], xs[2]).is_zero());
        assert!(model.semaev_s3(xs[2], xs[0], xs[1]).is_zero());
    }

    #[test]
    fn curve25519_binary_pkm_towers_are_shallow() {
        let limits = Curve25519IcModel::rfc7748().pkm_binary_tower_limits();
        assert_eq!(limits.kummer_height_from_p_minus_1, 2);
        assert_eq!(limits.kummer_abscissae, 4);
        assert_eq!(limits.torus_height_from_p_plus_1, 1);
        assert_eq!(limits.torus_abscissae, 2);
    }

    #[test]
    fn committed_translation_certificate_replays() {
        let cert: IdentityCertificate = serde_json::from_str(include_str!(
            "../../research/curve25519_index_calculus_20261102/montgomery-translation.identity.json"
        ))
        .expect("committed identity certificate parses");
        assert_eq!(cert.statement, montgomery_translation_identity());
        assert!(matches!(
            check_with_extra(&cert, Some((0x2551_9c1c, 32))).expect("certificate is well formed"),
            Verdict::Accept { .. }
        ));
    }

    #[test]
    fn committed_manifest_matches_native_derivation() {
        let committed: serde_json::Value = serde_json::from_str(include_str!(
            "../../research/curve25519_index_calculus_20261102/model.json"
        ))
        .expect("committed model manifest parses");
        assert_eq!(committed, Curve25519IcModel::rfc7748().evidence_manifest());
    }
}
