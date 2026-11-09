use full_trace_zero_backend::binary_ecc::{
    curve::{point_add, point_neg, scalar_mul},
    BinaryPoint, F2mElement, TraceZeroVariety,
};
use num_bigint::BigUint;
use trace_zero_leads::fixture;

#[test]
fn tau_minus_one_is_a_bijection_with_a_checked_inverse_on_the_small_kernel() {
    for n in [5, 7] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let g = (1..1u32 << n)
            .find_map(|raw| {
                v.lift_kummer_x(&F2mElement::from_biguint(&BigUint::from(raw), n), false)
                    .ok()
            })
            .expect("nonidentity kernel point");
        let points: Vec<_> = (0..c.order.to_u32_digits()[0])
            .map(|i| scalar_mul(&c, &g, &BigUint::from(i)))
            .collect();
        assert!(points.iter().skip(1).all(|p| *p != BinaryPoint::Infinity));
        let image: Vec<_> = points
            .iter()
            .map(|p| v.cofactor_image(p).unwrap())
            .collect();
        for (p, q) in points.iter().zip(&image) {
            assert!(v.contains(q));
            assert_eq!(v.inverse_cofactor_image(q).unwrap(), *p);
            assert_eq!(image.iter().filter(|r| *r == q).count(), 1);
        }
        // Complete relation-existence correspondence for a finite signed base.
        let base = [&points[1], &points[2], &points[points.len() - 1]];
        let mapped: Vec<_> = base.iter().map(|p| v.cofactor_image(p).unwrap()).collect();
        let sums = |b: &[BinaryPoint]| {
            let mut out = Vec::new();
            for p in b {
                for q in b {
                    for r in b {
                        out.push(point_add(&c, &point_add(&c, p, q), r));
                    }
                }
            }
            out
        };
        let old = sums(&base.iter().map(|p| (*p).clone()).collect::<Vec<_>>());
        let new = sums(&mapped);
        for (t, dt) in points.iter().zip(&image) {
            assert_eq!(old.contains(t), new.contains(dt));
        }
    }
}

#[test]
fn wide_tau_transport_replays_both_directions_and_rejects_torsion_cosets() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let p = &c.generator;
        let target = if n == 131 {
            full_trace_zero_backend::binary_ecc::BinaryCurve::certicom_challenge("ECC2K-130")
                .unwrap()
                .1
        } else {
            scalar_mul(&c, p, &BigUint::from(17u32))
        };
        let q = &target;
        let dp = v.cofactor_image(p).unwrap();
        let dq = v.cofactor_image(q).unwrap();
        for (original, mapped) in [(p, &dp), (q, &dq)] {
            assert_eq!(v.inverse_cofactor_image(mapped).unwrap(), *original);
            assert_eq!(
                v.cofactor_image(&v.inverse_cofactor_image(original).unwrap())
                    .unwrap(),
                *original
            );
        }
        let target = point_add(&c, &point_add(&c, p, q), &point_neg(p));
        let mapped_sum = point_add(&c, &point_add(&c, &dp, &dq), &point_neg(&dp));
        assert_eq!(v.cofactor_image(&target).unwrap(), mapped_sum);
        assert_eq!(v.inverse_cofactor_image(&mapped_sum).unwrap(), target);
        let t2 = BinaryPoint::Affine {
            x: F2mElement::zero(n),
            y: F2mElement::one(n),
        };
        assert!(v.inverse_cofactor_image(&t2).is_err());
        assert!(v.inverse_cofactor_image(&point_add(&c, p, &t2)).is_err());
        assert_eq!(
            v.inverse_cofactor_image(&BinaryPoint::Infinity).unwrap(),
            BinaryPoint::Infinity
        );
        println!("TAU_TRANSPORT n={n} fixture_G_target both directions and original relation replay; n131 target=public_Q,n83 target=17G; cardinality preserved, no relation-finding comparison");
    }
}
