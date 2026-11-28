//! Independent fixed-sextic point counts,using existing field/poly engines.
//! Credits external CT39 for the predictions and smooth negative control.
//! This computes spectra,not a Jacobian map or geometric-kernel certificate.
use isogeny_algos::field::{Field, Zp};
use isogeny_algos::fpr::FpR;
use isogeny_algos::poly;
use std::collections::HashSet;

fn character<F: Field>(f: &F, x: F::E) -> i64 {
    if f.is_zero(x) {
        0
    } else {
        let chi = f.pow(x, (f.size() - 1) / 2);
        if chi == f.one() {
            1
        } else {
            assert_eq!(chi, f.neg(f.one()));
            -1
        }
    }
}

fn check_characters<F: Field>(f: &F, elements: &[F::E]) {
    assert_eq!(elements.len() as u128, f.size());
    assert_eq!(
        elements.iter().copied().collect::<HashSet<_>>().len(),
        elements.len()
    );
    let squares = elements.iter().map(|&x| f.sq(x)).collect::<HashSet<_>>();
    assert_eq!(squares.len() as u128, f.size().div_ceil(2));
    for &x in elements {
        let expected = if f.is_zero(x) {
            0
        } else if squares.contains(&x) {
            1
        } else {
            -1
        };
        assert_eq!(character(f, x), expected);
    }
}

fn curve_count<F: Field>(f: &F, elements: &[F::E], coefficients: &[u64; 7]) -> i64 {
    assert_eq!(coefficients[6], 1);
    let polynomial = coefficients
        .iter()
        .map(|&c| f.from_u64(c))
        .collect::<Vec<_>>();
    assert_eq!(
        poly::gcd(f, &polynomial, &poly::derivative(f, &polynomial)),
        vec![f.one()]
    );
    2 + elements
        .iter()
        .map(|&x| 1 + character(f, poly::eval(f, &polynomial, x)))
        .sum::<i64>()
}

#[test]
fn all_three_sextic_spectra_and_smooth_base_count_impostor() {
    let prime = Zp::new(101);
    let quadratic = FpR::from_modulus(101, &[98, 0, 1]);
    let base_elements = (0..101).collect::<Vec<_>>();
    let mut extension_elements = Vec::new();
    for b in 0..101 {
        for a in 0..101 {
            extension_elements.push(quadratic.add(
                quadratic.from_u64(a),
                quadratic.mul(quadratic.from_u64(b), quadratic.gen()),
            ));
        }
    }
    assert_eq!(quadratic.modulus(), vec![98, 0, 1]);
    check_characters(&prime, &base_elements);
    check_characters(&quadratic, &extension_elements);
    println!("PASS existing Zp/FpR field characters:101/10201 complete distinct elements;Euler agrees with full square tables;quadratic basis z^2-3 constructor checked irreducible");
    let models = [
        [31, 75, 74, 54, 71, 55, 1],
        [95, 26, 19, 27, 94, 91, 1],
        [95, 93, 43, 39, 92, 17, 1],
        [31, 79, 74, 54, 71, 55, 1],
    ];
    for (i, coefficients) in models.iter().enumerate() {
        let n1 = curve_count(&prime, &base_elements, coefficients);
        let n2 = curve_count(&quadratic, &extension_elements, coefficients);
        assert_eq!(n1, 120);
        assert_eq!(n2, if i < 3 { 10436 } else { 10314 });
        let s1 = 102 - n1;
        let s2 = 10202 - n2;
        assert_eq!((s1 * s1 - s2) % 2, 0);
        let a1 = -s1;
        let a2 = (s1 * s1 - s2) / 2;
        let weil = [101 * 101, 101 * a1, a2, a1, 1];
        let order: i64 = weil.iter().sum();
        if i < 3 {
            assert_eq!(weil, [10201, 1818, 279, 18, 1]);
            assert_eq!(a1, 7 + 11);
            assert_eq!(a2, 101 + 101 + 7 * 11);
            assert_eq!(order, 109 * 113);
        } else {
            assert_eq!(weil, [10201, 1818, 218, 18, 1]);
            assert_eq!(order, 12256);
            assert_ne!(order, 109 * 113);
        }
        println!("PASS smooth sextic{i}:coefficients{coefficients:?};N1={n1};N2={n2};Weil low-first={weil:?};P(1)={order};two infinity points counted");
    }
    println!("PASS CT39 independent existing-engine Rust replay and smooth base-count negative control;no selected map,kernel,geometric isomorphism or cost conclusion");
}
