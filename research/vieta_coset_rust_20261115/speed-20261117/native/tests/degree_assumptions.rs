//! Nonempty diagonal on the prime-order toy curve, independent of the
//! timing instrument. A squarefree membership polynomial need not be divisible
//! by the repeated Vieta quadratic, so the separate diagonal branch is needed.
use vieta_coset_lab::vieta_coset::*;

#[test]
fn prime_curve_diagonal_survives_squarefree_membership() {
    let mut affine = 0;
    for x in 0..17u64 {
        for y in 0..17u64 {
            if y * y % 17 == (x * x * x + 2 * x + 2) % 17 {
                affine += 1;
            }
        }
    }
    assert_eq!(affine + 1, 19);
    for backend in [
        PolynomialBackend::Schoolbook,
        PolynomialBackend::KaratsubaNewton,
        PolynomialBackend::HalfGcd,
    ] {
        let a = Arithmetic::new(WordField(17), Limits::default())
            .unwrap()
            .with_polynomials(backend);
        let curve = Curve { a: 2, b: 2 };
        let map = a.velu_two_map(&Curve { a: 1, b: 0 }, 0).unwrap();
        let tower = a.certify_tower(&[map], 12, 42).unwrap();
        let target = a.group_add(&curve, &Some((5, 1)), &Some((5, 1))).unwrap();
        assert_eq!(target, Some((6, 3)));
        let expected = vec![((5, 1), (5, 1))];
        assert_eq!(
            a.solve_tower(&curve, &tower, &target, 42).unwrap(),
            expected
        );
        let base = a.tower_factor_base(&curve, &tower).unwrap();
        assert_eq!(a.lookup_scalar(&curve, &base, &target).unwrap(), expected);
    }
    // At slope13: s=10,v=8,Delta=0, H=Z^2+5Z+1, H(5)=0.
    // Dividing H by (Z-5)^2 leaves 15Z+10, nonzero. Off-diagonal coefficient
    // vanishing alone would reject this genuine repeated-point relation.
    assert_eq!((10u64 * 10 + 17 * 2 - 4 * 8) % 17, 0);
    assert_eq!((5u64 * 5 + 5 * 5 + 1) % 17, 0);
    assert_ne!((15u64, 10u64), (0, 0));
}
