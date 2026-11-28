//! Independent controls for the checked Montgomery target-model adapter.

use crypto_lib::prime_hyperelliptic::{
    PrimeEllipticPointP, PrimeMontgomeryAdapterError, PrimeMontgomeryAdapterP,
    PrimeMontgomeryPointP, PrimeProjectivePointP,
};
use crypto_lib::utils::mod_inverse;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::Value;
use std::collections::BTreeSet;

fn n(value: u64) -> BigUint {
    BigUint::from(value)
}

fn add(left: u64, right: u64, p: u64) -> u64 {
    ((left as u128 + right as u128) % p as u128) as u64
}

fn sub(left: u64, right: u64, p: u64) -> u64 {
    ((left as u128 + p as u128 - (right % p) as u128) % p as u128) as u64
}

fn mul(left: u64, right: u64, p: u64) -> u64 {
    ((left as u128 * right as u128) % p as u128) as u64
}

fn inv(value: u64, p: u64) -> u64 {
    (1..p)
        .find(|candidate| mul(value, *candidate, p) == 1)
        .expect("test denominator must be invertible")
}

fn short_rhs(x: u64, a: u64, b: u64, p: u64) -> u64 {
    add(add(mul(mul(x, x, p), x, p), mul(a, x, p), p), b, p)
}

fn montgomery_rhs(x: u64, a: u64, p: u64) -> u64 {
    add(
        add(mul(mul(x, x, p), x, p), mul(a, mul(x, x, p), p), p),
        x,
        p,
    )
}

fn projectively_equivalent(
    left: &PrimeProjectivePointP,
    right: &PrimeProjectivePointP,
    p: &BigUint,
) -> bool {
    (&left.x * &right.y) % p == (&left.y * &right.x) % p
        && (&left.x * &right.z) % p == (&left.z * &right.x) % p
        && (&left.y * &right.z) % p == (&left.z * &right.y) % p
}

#[test]
fn exhaustive_small_models_are_projective_bijections() {
    for p in [5u64, 7, 11] {
        for montgomery_a in 0..p {
            for montgomery_b in 0..p {
                let singular = montgomery_b == 0 || mul(montgomery_a, montgomery_a, p) == 4 % p;
                let adapter =
                    PrimeMontgomeryAdapterP::try_new(n(p), n(montgomery_a), n(montgomery_b));
                if singular {
                    assert_eq!(
                        adapter,
                        Err(PrimeMontgomeryAdapterError::SingularMontgomeryModel)
                    );
                    continue;
                }
                let adapter = adapter.unwrap();
                adapter.validate().unwrap();
                let short_a = adapter.short_a().to_u64().unwrap();
                let short_b = adapter.short_b().to_u64().unwrap();

                let mut short_images = BTreeSet::new();
                let mut montgomery_points = BTreeSet::new();
                for x in 0..p {
                    for y in 0..p {
                        if short_rhs(x, short_a, short_b, p) == mul(y, y, p) {
                            let point = PrimeEllipticPointP::Affine { x: n(x), y: n(y) };
                            let image = adapter.short_to_montgomery(&point).unwrap();
                            assert_eq!(adapter.montgomery_to_short(&image).unwrap(), point);
                            let PrimeMontgomeryPointP::Affine { x, y } = image else {
                                panic!("finite point mapped to infinity")
                            };
                            short_images.insert((x.to_u64().unwrap(), y.to_u64().unwrap()));
                        }
                        if mul(montgomery_b, mul(y, y, p), p) == montgomery_rhs(x, montgomery_a, p)
                        {
                            montgomery_points.insert((x, y));
                        }
                    }
                }
                assert_eq!(short_images, montgomery_points);
                assert_eq!(
                    adapter
                        .short_to_montgomery(&PrimeEllipticPointP::Infinity)
                        .unwrap(),
                    PrimeMontgomeryPointP::Infinity
                );
                assert_eq!(
                    adapter
                        .montgomery_to_short(&PrimeMontgomeryPointP::Infinity)
                        .unwrap(),
                    PrimeEllipticPointP::Infinity
                );

                // Exercise every canonical homogeneous triple, including all
                // scalar representatives of the unique point at infinity.
                for x in 0..p {
                    for y in 0..p {
                        for z in 0..p {
                            if x == 0 && y == 0 && z == 0 {
                                continue;
                            }
                            let short_lhs = mul(mul(y, y, p), z, p);
                            let short_rhs_projective = add(
                                add(
                                    mul(mul(x, x, p), x, p),
                                    mul(short_a, mul(x, mul(z, z, p), p), p),
                                    p,
                                ),
                                mul(short_b, mul(mul(z, z, p), z, p), p),
                                p,
                            );
                            if short_lhs != short_rhs_projective {
                                continue;
                            }
                            let point = PrimeProjectivePointP {
                                x: n(x),
                                y: n(y),
                                z: n(z),
                            };
                            let image = adapter.short_projective_to_montgomery(&point).unwrap();
                            let round_trip =
                                adapter.montgomery_projective_to_short(&image).unwrap();
                            assert!(projectively_equivalent(
                                &point,
                                &round_trip,
                                adapter.modulus()
                            ));
                        }
                    }
                }
            }
        }
    }
}

type SmallPoint = Option<(u64, u64)>;

fn short_add(left: SmallPoint, right: SmallPoint, a: u64, p: u64) -> SmallPoint {
    let (x1, y1) = match left {
        Some(point) => point,
        None => return right,
    };
    let (x2, y2) = match right {
        Some(point) => point,
        None => return left,
    };
    if x1 == x2 && add(y1, y2, p) == 0 {
        return None;
    }
    let slope = if x1 == x2 && y1 == y2 {
        mul(
            add(mul(3, mul(x1, x1, p), p), a, p),
            inv(mul(2, y1, p), p),
            p,
        )
    } else {
        mul(sub(y2, y1, p), inv(sub(x2, x1, p), p), p)
    };
    let x3 = sub(sub(mul(slope, slope, p), x1, p), x2, p);
    let y3 = sub(mul(slope, sub(x1, x3, p), p), y1, p);
    Some((x3, y3))
}

fn montgomery_add(left: SmallPoint, right: SmallPoint, a: u64, b: u64, p: u64) -> SmallPoint {
    let (x1, y1) = match left {
        Some(point) => point,
        None => return right,
    };
    let (x2, y2) = match right {
        Some(point) => point,
        None => return left,
    };
    if x1 == x2 && add(y1, y2, p) == 0 {
        return None;
    }
    let slope = if x1 == x2 && y1 == y2 {
        let numerator = add(
            add(mul(3, mul(x1, x1, p), p), mul(2, mul(a, x1, p), p), p),
            1,
            p,
        );
        mul(numerator, inv(mul(2, mul(b, y1, p), p), p), p)
    } else {
        mul(sub(y2, y1, p), inv(sub(x2, x1, p), p), p)
    };
    let x3 = sub(
        sub(sub(mul(b, mul(slope, slope, p), p), a, p), x1, p),
        x2,
        p,
    );
    let y3 = sub(mul(slope, sub(x1, x3, p), p), y1, p);
    Some((x3, y3))
}

#[test]
fn adapter_commutes_with_an_independent_montgomery_group_law() {
    let (p, montgomery_a, montgomery_b) = (11u64, 3u64, 2u64);
    let adapter = PrimeMontgomeryAdapterP::try_new(n(p), n(montgomery_a), n(montgomery_b)).unwrap();
    let short_a = adapter.short_a().to_u64().unwrap();
    let mut points = vec![None];
    for x in 0..p {
        for y in 0..p {
            if mul(y, y, p) == short_rhs(x, short_a, adapter.short_b().to_u64().unwrap(), p) {
                points.push(Some((x, y)));
            }
        }
    }
    let map = |point: SmallPoint| -> SmallPoint {
        let point = match point {
            None => PrimeEllipticPointP::Infinity,
            Some((x, y)) => PrimeEllipticPointP::Affine { x: n(x), y: n(y) },
        };
        match adapter.short_to_montgomery(&point).unwrap() {
            PrimeMontgomeryPointP::Infinity => None,
            PrimeMontgomeryPointP::Affine { x, y } => {
                Some((x.to_u64().unwrap(), y.to_u64().unwrap()))
            }
        }
    };
    for &left in &points {
        for &right in &points {
            assert_eq!(
                map(short_add(left, right, short_a, p)),
                montgomery_add(map(left), map(right), montgomery_a, montgomery_b, p),
                "left={left:?}, right={right:?}"
            );
        }
    }
}

#[test]
fn malformed_models_points_and_coordinate_confusion_fail_closed() {
    assert_eq!(
        PrimeMontgomeryAdapterP::try_new(n(3), n(1), n(1)),
        Err(PrimeMontgomeryAdapterError::UnsupportedCharacteristic)
    );
    assert_eq!(
        PrimeMontgomeryAdapterP::try_new(n(15), n(1), n(1)),
        Err(PrimeMontgomeryAdapterError::ModulusNotPrime)
    );
    assert_eq!(
        PrimeMontgomeryAdapterP::try_new(n(11), n(11), n(1)),
        Err(PrimeMontgomeryAdapterError::NonCanonicalCoefficient { coefficient: "A" })
    );
    assert_eq!(
        PrimeMontgomeryAdapterP::try_new(n(11), n(3), n(0)),
        Err(PrimeMontgomeryAdapterError::SingularMontgomeryModel)
    );
    assert_eq!(
        PrimeMontgomeryAdapterP::try_new(n(11), n(2), n(1)),
        Err(PrimeMontgomeryAdapterError::SingularMontgomeryModel)
    );

    let adapter = PrimeMontgomeryAdapterP::try_new(n(11), n(3), n(1)).unwrap();
    assert_eq!((adapter.short_a(), adapter.short_b()), (&n(9), &n(1)));
    let short_only = PrimeEllipticPointP::Affine { x: n(0), y: n(1) };
    adapter.validate_short_point(&short_only).unwrap();
    assert_eq!(
        adapter.validate_montgomery_point(&PrimeMontgomeryPointP::Affine { x: n(0), y: n(1) }),
        Err(PrimeMontgomeryAdapterError::PointNotOnMontgomeryModel)
    );
    assert_eq!(
        adapter.validate_short_point(&PrimeEllipticPointP::Affine { x: n(11), y: n(1) }),
        Err(PrimeMontgomeryAdapterError::PointCoordinateOutOfRange { coordinate: "x" })
    );
    assert_eq!(
        adapter.validate_short_projective(&PrimeProjectivePointP {
            x: BigUint::zero(),
            y: BigUint::zero(),
            z: BigUint::zero(),
        }),
        Err(PrimeMontgomeryAdapterError::InvalidProjectivePoint)
    );

    let infinity = PrimeProjectivePointP {
        x: BigUint::zero(),
        y: BigUint::one(),
        z: BigUint::zero(),
    };
    let image = adapter.short_projective_to_montgomery(&infinity).unwrap();
    assert_eq!(image.z, BigUint::zero());
    assert!(!image.y.is_zero());
    let round_trip = adapter.montgomery_projective_to_short(&image).unwrap();
    assert!(projectively_equivalent(
        &infinity,
        &round_trip,
        adapter.modulus()
    ));
}

fn integer(value: &Value) -> BigUint {
    let text = value.as_str().expect("catalog integer must be a string");
    let (radix, digits) = if let Some(hex) = text.strip_prefix("0x") {
        (16, hex)
    } else {
        (10, text)
    };
    BigUint::parse_bytes(digits.as_bytes(), radix).expect("valid catalog integer")
}

#[test]
fn six_live_montgomery_maps_match_the_formula_but_retain_the_prime_proof_gate() {
    let graph_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/curves/cover-links.yaml");
    let graph: Value = serde_json::from_slice(&std::fs::read(graph_path).unwrap()).unwrap();
    let normalizations = graph["maps"]
        .as_object()
        .unwrap()
        .values()
        .filter_map(|wrapper| wrapper["record"]["target_model_map"].as_object())
        .filter(|map| map["kind"] == "montgomery_from_short")
        .collect::<Vec<_>>();
    assert_eq!(normalizations.len(), 6);

    for normalization in normalizations {
        let p = integer(&normalization["target_model"]["p"]);
        let a = integer(&normalization["montgomery_A"]);
        let b = integer(&normalization["montgomery_B"]);
        let shift = integer(&normalization["shift_A_over_3"]);
        let short_a = integer(&normalization["source_model"]["a"]);
        let short_b = integer(&normalization["source_model"]["b"]);
        let inv3 = mod_inverse(&n(3), &p).unwrap();
        let expected_shift = &a * &inv3 % &p;
        assert_eq!(shift, expected_shift);
        let b2 = &b * &b % &p;
        let b3 = &b2 * &b % &p;
        assert_eq!(
            (&short_a * &b2 + (&a * &a % &p) * &inv3) % &p,
            BigUint::one()
        );
        assert_eq!(
            (&short_b * &b3 + &shift) % &p,
            (n(2) * &shift * &shift * &shift) % &p
        );
        assert!(matches!(
            PrimeMontgomeryAdapterP::try_new(p, a, b),
            Err(PrimeMontgomeryAdapterError::UnsupportedPrimalityProofRange { .. })
        ));
    }
}
