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
