use crate::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly,
};
use crate::cryptanalysis::trace_zero_word_field::*;
fn fixture(n: u32) -> BinaryCurve {
    if n == 131 {
        return BinaryCurve::certicom_challenge("ecc2k-130").unwrap().0;
    }
    BinaryCurve {
        m: n,
        irreducible: IrreduciblePoly {
            degree: n,
            low_terms: if n == 5 {
                vec![0, 2]
            } else {
                vec![0, 1, 2, 45]
            },
        },
        a: F2mElement::zero(n),
        b: F2mElement::one(n),
        generator: BinaryPoint::Infinity,
        order: if n == 5 {
            11u32.into()
        } else {
            num_bigint::BigUint::parse_bytes(b"2417851639230796216685689", 10).unwrap()
        },
        cofactor: 4u32.into(),
    }
}
#[test]
fn compact_arithmetic_matches_general_backend_small_exhaustive_and_wide_samples() {
    let mut seed = 20261115u64;
    for n in [5, 83, 131] {
        let c = fixture(n);
        let f = WordField::new(&c);
        let count = if n == 5 { 32 } else { 128 };
        for i in 0..count {
            let a = if n == 5 {
                F2mElement::from_biguint(&(i as u32).into(), n)
            } else {
                let mut raw = [0; 3];
                for w in &mut raw {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    *w = seed;
                }
                F2mElement::from_words(&raw, n)
            };
            let ar = element(&a);
            assert_eq!(f.square(ar), element(&a.square(&c.irreducible)));
            assert_eq!(f.half(ar), element(&a.half_trace(&c.irreducible).unwrap()));
            assert_eq!(f.trace(ar), a.trace(&c.irreducible));
            assert_eq!(
                f.inv(ar),
                a.flt_inverse(&c.irreducible).map(|x| element(&x))
            );
            for j in 0..if n == 5 { 32 } else { 4 } {
                let b = if n == 5 {
                    F2mElement::from_biguint(&(j as u32).into(), n).add(&a)
                } else {
                    let mut raw = [0; 3];
                    for w in &mut raw {
                        seed ^= seed << 13;
                        seed ^= seed >> 7;
                        seed ^= seed << 17;
                        *w = seed;
                    }
                    F2mElement::from_words(&raw, n)
                };
                assert_eq!(f.add(ar, element(&b)), element(&a.add(&b)));
                assert_eq!(f.mul(ar, element(&b)), element(&a.mul(&b, &c.irreducible)));
            }
        }
    }
}
#[test]
fn compact_admission_and_batch_residuals_match_all_small_curve_points_and_offcurve_inputs() {
    let c = fixture(5);
    let f = WordField::new(&c);
    let mut points = vec![BinaryPoint::Infinity];
    for x in 0..32u32 {
        for y in 0..32u32 {
            let p = BinaryPoint::Affine {
                x: F2mElement::from_biguint(&x.into(), 5),
                y: F2mElement::from_biguint(&y.into(), 5),
            };
            assert_eq!(f.admit(&p), c.is_in_4e(&p) == Some(true));
            if c.is_on_curve(&p) {
                points.push(p);
            }
        }
    }
    for target in &points {
        let got = f.residuals(&c, target, &points);
        let want: Vec<_> = points
            .iter()
            .map(|p| WordPoint::from_point(&point_add(&c, target, &point_neg(p))))
            .collect();
        assert_eq!(got, want);
    }
}

#[test]
fn every_product_monomial_matches_reference_for_specialized_moduli() {
    for n in [83, 131] {
        let c = fixture(n);
        let f = WordField::new(&c);
        for k in 0..=2 * n - 2 {
            let i = k.min(n - 1);
            let j = k - i;
            let a = F2mElement::from_bit_positions(&[i], n);
            let b = F2mElement::from_bit_positions(&[j], n);
            assert_eq!(
                f.mul(element(&a), element(&b)),
                element(&a.mul(&b, &c.irreducible)),
                "n{n}/z^{k}"
            );
        }
    }
}
#[test]
fn generator_multiples_match_general_arithmetic_on_every_basis_vector() {
    for n in [5, 83, 131] {
        let c = fixture(n);
        let f = WordField::new(&c);
        let z = F2mElement::from_bit_positions(&[1], n);
        for k in 0..n {
            let a = F2mElement::from_bit_positions(&[k], n);
            assert_eq!(f.times_x(element(&a)), element(&a.mul(&z, &c.irreducible)));
        }
    }
}
#[test]
fn degree_alone_never_selects_a_specialized_modulus() {
    for n in [83, 131] {
        let mut c = fixture(n);
        c.irreducible.low_terms = vec![0, 1];
        let f = WordField::new(&c);
        for k in 0..n {
            let a = F2mElement::from_bit_positions(&[k], n);
            let b = F2mElement::from_bit_positions(&[n - 1], n);
            assert_eq!(
                f.mul(element(&a), element(&b)),
                element(&a.mul(&b, &c.irreducible))
            );
            assert_eq!(f.square(element(&a)), element(&a.square(&c.irreducible)));
        }
    }
}
