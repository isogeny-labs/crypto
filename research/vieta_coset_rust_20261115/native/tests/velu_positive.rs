//! Nonempty genuine normalized-Velu fiber on a separate prime-order curve.
use vieta_coset_lab::vieta_coset::*;
#[test]
fn velu_prime_curve_positive_relation() {
    let a = Arithmetic::new(WordField(17), Limits::default()).unwrap();
    let map = a.velu_two_map(&Curve { a: 1, b: 0 }, 0).unwrap();
    let tower = a.certify_tower(&[map], 12, 42).unwrap();
    assert_eq!(tower.abscissae(), [5, 7]);
    let target_curve = Curve { a: 2, b: 2 };
    let base = a.tower_factor_base(&target_curve, &tower).unwrap();
    assert_eq!(base.len(), 4);
    let target = Some((7, 11));
    assert_eq!(
        a.group_add(&target_curve, &Some((5, 1)), &Some((7, 6)))
            .unwrap(),
        target
    );
    let expected = vec![((5, 1), (7, 6)), ((7, 6), (5, 1))];
    assert_eq!(
        a.lookup_scalar(&target_curve, &base, &target).unwrap(),
        expected
    );
    assert_eq!(
        a.solve_tower(&target_curve, &tower, &target, 42).unwrap(),
        expected
    );
}

#[test]
fn rational_two_torsion_has_nonempty_distinct_and_diagonal_controls() {
    let setup = Arithmetic::new(WordField(17), Limits::default()).unwrap();
    let tower = setup
        .certify_tower(
            &[RationalMap {
                numerator: vec![0, 0, 1],
                denominator: vec![1],
            }],
            4,
            42,
        )
        .unwrap();
    let target = Some((0, 0));
    for (coefficient, expected) in [
        (13, vec![((2, 0), (15, 0)), ((15, 0), (2, 0))]),
        (
            4,
            vec![
                ((2, 4), (2, 4)),
                ((2, 13), (2, 13)),
                ((15, 1), (15, 1)),
                ((15, 16), (15, 16)),
            ],
        ),
    ] {
        let curve = Curve {
            a: coefficient,
            b: 0,
        };
        let base = setup.tower_factor_base(&curve, &tower).unwrap();
        assert!(!base.is_empty());
        assert_eq!(
            setup.lookup_scalar(&curve, &base, &target).unwrap(),
            expected
        );
        for mode in [
            PolynomialBackend::KaratsubaNewton,
            PolynomialBackend::HalfGcd,
        ] {
            let query = Arithmetic::new(WordField(17), Limits::default())
                .unwrap()
                .with_polynomials(mode);
            assert_eq!(
                query.solve_tower(&curve, &tower, &target, 42).unwrap(),
                expected
            );
        }
        let limited = Arithmetic::new(
            WordField(17),
            Limits {
                max_muls: 1,
                ..Limits::default()
            },
        )
        .unwrap()
        .with_polynomials(PolynomialBackend::HalfGcd);
        assert!(matches!(
            limited.solve_tower(&curve, &tower, &target, 42),
            Err(Error::Budget(_))
        ));
    }
}
