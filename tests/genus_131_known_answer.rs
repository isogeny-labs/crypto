use crypto_lib::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
use num_bigint::BigUint;

fn power(poly: &FpPoly, exponent: usize) -> FpPoly {
    (0..exponent).fold(FpPoly::one(poly.p.clone()), |acc, _| acc.mul(poly))
}

#[test]
fn genus_131_prime_order_jacobian_known_answer() {
    const GENUS: u32 = 131;
    const FIELD: u32 = 103;
    const ORDER: u32 = 34_589;
    const SECOND_POINT_LOG: u32 = 11_442;
    const TARGET_LOG: u32 = 27_804;

    let p = BigUint::from(FIELD);
    let x = FpPoly::x(p.clone());
    let x_minus =
        |root: u32| FpPoly::from_coeffs(vec![BigUint::from(FIELD - root), 1u32.into()], p.clone());
    let x_minus_one = x_minus(1);
    let x_130 = power(&x, 130);
    let x_minus_one_130 = power(&x_minus_one, 130);
    let left = x_130.mul(&x_minus(16));
    let right = x_minus_one_130.mul(&x_minus(88)).scalar_mul(&100u32.into());
    let half = BigUint::from(52u32);
    let w = right.sub(&left).scalar_mul(&half);
    let z = right.add(&left).scalar_mul(&half);
    let f = x_130
        .mul(&x_minus_one_130)
        .mul(&power(&x_minus_one, 3))
        .add(&w.mul(&w));
    let other_f = x_130
        .mul(&x_minus_one_130)
        .mul(&power(&x, 3))
        .add(&z.mul(&z));
    assert_eq!(f, other_f, "both principal divisors lie on the same curve");
    let curve = HyperellipticCurveP::try_new(p, f, GENUS).expect("smooth genus-131 model");

    let generator = MumfordDivisorP::try_from_point(&curve, &0u32.into(), &29u32.into()).unwrap();
    let second = MumfordDivisorP::try_from_point(&curve, &1u32.into(), &59u32.into()).unwrap();
    let multiply =
        |point: &MumfordDivisorP, k: u32| point.checked_scalar_mul(&k.into(), &curve).unwrap();

    // The zeros of y-w give 130P + 133R = 0. At x=1, z=-w,
    // so y-z gives 133P - 130R = 0.
    assert!(multiply(&generator, 130)
        .checked_add(&multiply(&second, 133), &curve)
        .unwrap()
        .is_identity());
    assert!(multiply(&generator, 133)
        .checked_eq(&multiply(&second, 130), &curve)
        .unwrap());

    // Eliminating R gives [130^2 + 133^2]P = 0. The next trial divisor
    // after 185 exceeds sqrt(ORDER), so these checks prove exact order.
    assert_eq!(ORDER, 130 * 130 + 133 * 133);
    assert!((2..=185).all(|divisor| !ORDER.is_multiple_of(divisor)));
    assert!(!generator.is_identity());
    assert!(multiply(&generator, ORDER).is_identity());
    assert_eq!((130 + 133 * SECOND_POINT_LOG) % ORDER, 0);
    assert!(multiply(&generator, SECOND_POINT_LOG)
        .checked_eq(&second, &curve)
        .unwrap());

    // Build Q directly from its Mumford pair for 53P + 78R.
    let u = power(&x, 53).mul(&power(&x_minus_one, 78));
    assert_eq!(u.degree(), Some(GENUS as usize));
    let v = w.rem(&u);
    let target = MumfordDivisorP::try_from_polynomials(&curve, u, v).unwrap();
    assert_eq!(TARGET_LOG, (53 + 78 * SECOND_POINT_LOG) % ORDER);
    assert!(multiply(&generator, TARGET_LOG)
        .checked_eq(&target, &curve)
        .unwrap());
    assert!(!multiply(&generator, TARGET_LOG + 1)
        .checked_eq(&target, &curve)
        .unwrap());
}
