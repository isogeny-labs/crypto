use super::*;
use crate::binary_ecc::{
    curve::{point_add, point_neg, scalar_mul, BinaryCurve, BinaryPoint},
    F2mElement, IrreduciblePoly,
};
use crate::cryptanalysis::{gaudry_quartic_bin as old, semaev_decomp::Gf2_128};
use num_bigint::BigUint;

fn old_element(e: old::Eb) -> Eb {
    Eb(e.0.map(u32::from))
}
fn old_point(p: old::PtB) -> PtB {
    PtB {
        x: old_element(p.x),
        y: old_element(p.y),
        inf: p.inf,
    }
}

#[test]
fn all_wide_base_elements_have_exact_inverses_and_artin_roots() {
    for m in [16, 17, 18] {
        let f = Gf2q::new(m);
        for a in 0..f.q {
            if a != 0 {
                assert_eq!(f.mul_nc(a, f.inv_nc(a)), 1);
            }
            let mut value = a;
            let mut trace = 0;
            for _ in 0..m {
                trace ^= value;
                value = f.mul_nc(value, value);
            }
            assert!(trace <= 1);
            match f.artin_schreier(a) {
                Some(t) => {
                    assert_eq!(trace, 0);
                    assert_eq!(f.mul_nc(t, t) ^ t, a);
                }
                None => assert_eq!(trace, 1),
            }
        }
    }
}

#[test]
fn excluded_linearized_family_is_reducible_on_complete_small_even_fields() {
    for m in [4, 6] {
        let mut f = Fq4::new(m);
        let z = Eb([0, 1, 0, 0]);
        for c1 in 0..f.f.q {
            for c0 in 0..f.f.q {
                f.c = [c0, c1, 0, 0];
                let d = f.pow2k(&z, 2 * m).add(&z);
                assert!(d.is_zero() || f.mat_rank(&d) < 4, "m={m}, c0={c0}, c1={c1}");
            }
        }
    }
}

#[test]
fn supported_controls_match_old_fields_points_scalars_and_counts() {
    for m in [6, 8, 10, 12, 15] {
        let narrow = old::generate_instance(m, 0x571d_0000 | u64::from(m));
        let wide = generate_instance(m, 0x571d_0000 | u64::from(m));
        assert_eq!(narrow.f.f.modulus, wide.f.f.modulus);
        assert_eq!(narrow.f.c.map(u32::from), wide.f.c);
        assert_eq!(old_element(narrow.a), wide.a);
        assert_eq!(old_element(narrow.b), wide.b);
        let mut nr = StdRng::seed_from_u64(0xc200_0000 | u64::from(m));
        let mut wr = nr.clone();
        let mut points = vec![];
        for _ in 0..32 {
            let a = narrow.f.random(&mut nr);
            let b = wide.f.random(&mut wr);
            assert_eq!(old_element(a), b);
            let c = narrow.f.random(&mut nr);
            let d = wide.f.random(&mut wr);
            assert_eq!(old_element(narrow.f.mul(&a, &c)), wide.f.mul(&b, &d));
            assert_eq!(old_element(narrow.f.sq(&a)), wide.f.sq(&b));
            assert_eq!(old_element(narrow.f.frob(&a)), wide.f.frob(&b));
            assert_eq!(u128::from(narrow.f.pack(&a)), wide.f.pack(&b));
            assert_eq!(wide.f.unpack(wide.f.pack(&b)), b);
            let p = narrow.lift_x(&a, &mut nr);
            let q = wide.lift_x(&b, &mut wr);
            assert_eq!(p.map(old_point), q);
            if let Some(p) = p {
                points.push(p);
            }
        }
        assert!(points.len() >= 2);
        let p = points[0];
        let q = points[1];
        assert_eq!(
            old_point(narrow.add(&p, &q)),
            wide.add(&old_point(p), &old_point(q))
        );
        assert_eq!(old_point(narrow.neg(&p)), wide.neg(&old_point(p)));
        for k in [0, 1, 2, 3, 17, 511, u64::MAX] {
            assert_eq!(
                old_point(narrow.mul(&p, k)),
                wide.mul(&old_point(p), u128::from(k))
            );
        }
        assert_eq!(narrow.f.f.muls(), wide.f.f.muls());
        assert_eq!(narrow.ops(), wide.ops());
    }
}

fn binary_point(curve: &CurveB, basis: &TowerBasis, p: PtB) -> BinaryPoint {
    if p.inf {
        BinaryPoint::Infinity
    } else {
        let n = 4 * curve.f.f.m;
        BinaryPoint::Affine {
            x: F2mElement::from_biguint(&BigUint::from(basis.coords(&curve.f, &p.x)), n),
            y: F2mElement::from_biguint(&BigUint::from(basis.coords(&curve.f, &p.y)), n),
        }
    }
}

#[test]
fn full_width_coordinates_and_scalars_match_independent_binary_arithmetic() {
    for m in [16, 17, 18] {
        let curve = generate_instance(m, 1);
        let basis = TowerBasis::new(&curve.f);
        let n = 4 * m;
        let irr = IrreduciblePoly {
            degree: n,
            low_terms: (0..n).filter(|&i| basis.modulus >> i & 1 == 1).collect(),
        };
        let independent = Gf2_128::new(&irr);
        let mut rng = StdRng::seed_from_u64(0xc218_0000 | u64::from(m));
        for _ in 0..32 {
            let a = curve.f.random(&mut rng);
            let b = curve.f.random(&mut rng);
            assert_eq!(
                basis.coords(&curve.f, &curve.f.mul_nc(&a, &b)),
                independent.mul(basis.coords(&curve.f, &a), basis.coords(&curve.f, &b))
            );
            assert_eq!(curve.f.unpack(curve.f.pack(&a)), a);
        }
        // Use a real order-two subgroup for the independent arithmetic model;
        // arbitrary curve points/scalars below are not reduced by this metadata.
        let order = group_order_bin(&curve, &mut rng).expect("unique Hasse multiple");
        assert!(order.is_multiple_of(2));
        let binary = BinaryCurve {
            m: n,
            irreducible: irr,
            a: F2mElement::from_biguint(&BigUint::from(basis.coords(&curve.f, &curve.a)), n),
            b: F2mElement::from_biguint(&BigUint::from(basis.coords(&curve.f, &curve.b)), n),
            generator: binary_point(&curve, &basis, curve.torsion()),
            order: BigUint::from(2u32),
            cofactor: BigUint::from(order / 2),
        };
        assert!(binary.is_on_curve(&binary.generator));
        assert_eq!(
            scalar_mul(&binary, &binary.generator, &BigUint::from(2u32)),
            BinaryPoint::Infinity
        );
        let mut points = vec![];
        while points.len() < 4 {
            let x = curve.f.random(&mut rng);
            if points.is_empty() && curve.f.pack(&x) < (1u128 << (n - 1)) {
                continue;
            }
            if let Some(p) = curve.lift_x(&x, &mut rng) {
                points.push(p);
            }
        }
        let p = points[0];
        let q = points[1];
        let bp = binary_point(&curve, &basis, p);
        let bq = binary_point(&curve, &basis, q);
        assert!(binary.is_on_curve(&bp) && binary.is_on_curve(&bq));
        assert_eq!(
            binary_point(&curve, &basis, curve.add(&p, &q)),
            point_add(&binary, &bp, &bq)
        );
        assert_eq!(binary_point(&curve, &basis, curve.neg(&p)), point_neg(&bp));
        for k in [
            0,
            1,
            17,
            (1u128 << 65) + 3,
            (1u128 << 70) + (1u128 << 64) + 123,
        ] {
            assert_eq!(
                binary_point(&curve, &basis, curve.mul(&p, k)),
                scalar_mul(&binary, &bp, &BigUint::from(k))
            );
        }
        assert!(curve.mul(&p, order).inf);
        let sum = points.iter().fold(PtB::INF, |s, p| curve.add(&s, p));
        assert!(!sum.inf);
        assert_eq!(
            curve.s5(&[points[0].x, points[1].x, points[2].x, points[3].x, sum.x]),
            Eb::ZERO
        );
        assert!(
            points
                .iter()
                .any(|p| curve.point_key(p) > u128::from(u64::MAX)),
            "high coordinate bits must be exercised"
        );
    }
}

#[test]
#[should_panic(expected = "coefficient outside base field")]
fn packing_rejects_out_of_range_coefficients() {
    let f = Fq4::new(16);
    let invalid = Eb([f.f.q, 0, 0, 0]);
    f.pack(&invalid);
}
#[test]
#[should_panic(expected = "packed element exceeds field width")]
fn unpacking_rejects_excess_high_bits() {
    let f = Fq4::new(18);
    f.unpack(1u128 << 72);
}

#[test]
fn bounded_prime_check_matches_exact_u64_reference_and_rejects_composites() {
    let mut rng = StdRng::seed_from_u64(0xc273);
    for _ in 0..256 {
        let n = rng.gen::<u64>();
        assert_eq!(
            is_prime(u128::from(n)),
            crate::cryptanalysis::residual_walk::is_prime_u64(n)
        );
    }
    assert!(is_prime(18446744073709551557));
    assert!(!is_prime(3825123056546413051));
    assert!(!is_prime((1u128 << 65) + 2));
}
