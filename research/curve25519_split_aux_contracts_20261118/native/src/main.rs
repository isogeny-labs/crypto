use num_bigint::BigUint;
use num_traits::Zero;

pub mod ecc {
    pub mod field {
        use num_bigint::BigUint;

        #[derive(Clone, Debug, PartialEq)]
        pub struct FieldElement {
            pub value: BigUint,
            pub modulus: BigUint,
        }

        impl FieldElement {
            pub fn new(value: BigUint, modulus: BigUint) -> Self {
                Self {
                    value: value % &modulus,
                    modulus,
                }
            }
        }
    }

    pub mod point {
        use num_bigint::BigUint;
        use num_traits::Zero;

        use super::field::FieldElement;

        #[derive(Clone, Debug, PartialEq)]
        pub enum Point {
            Infinity,
            Affine { x: FieldElement, y: FieldElement },
        }

        fn sub_mod(a: &BigUint, b: &BigUint, modulus: &BigUint) -> BigUint {
            if a >= b {
                (a - b) % modulus
            } else {
                modulus - ((b - a) % modulus)
            }
        }

        fn inv(value: &BigUint, modulus: &BigUint) -> BigUint {
            value.modpow(&(modulus - 2u32), modulus)
        }

        impl Point {
            pub fn x_coord(&self) -> Option<&BigUint> {
                match self {
                    Point::Infinity => None,
                    Point::Affine { x, .. } => Some(&x.value),
                }
            }

            pub fn neg_vartime(&self) -> Point {
                match self {
                    Point::Infinity => Point::Infinity,
                    Point::Affine { x, y } => Point::Affine {
                        x: x.clone(),
                        y: FieldElement::new(
                            if y.value.is_zero() {
                                BigUint::zero()
                            } else {
                                &y.modulus - &y.value
                            },
                            y.modulus.clone(),
                        ),
                    },
                }
            }

            pub fn add_vartime(&self, other: &Point, a: &FieldElement) -> Point {
                match (self, other) {
                    (Point::Infinity, point) | (point, Point::Infinity) => point.clone(),
                    (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
                        let modulus = &x1.modulus;
                        if x1.value == x2.value {
                            if y1.value != y2.value || y1.value.is_zero() {
                                return Point::Infinity;
                            }
                            let numerator =
                                (BigUint::from(3u32) * &x1.value * &x1.value + &a.value) % modulus;
                            let denominator = (BigUint::from(2u32) * &y1.value) % modulus;
                            let lambda = numerator * inv(&denominator, modulus) % modulus;
                            let x3 = sub_mod(
                                &(&lambda * &lambda % modulus),
                                &(BigUint::from(2u32) * &x1.value % modulus),
                                modulus,
                            );
                            let y3 = sub_mod(
                                &(&lambda * sub_mod(&x1.value, &x3, modulus) % modulus),
                                &y1.value,
                                modulus,
                            );
                            return Point::Affine {
                                x: FieldElement::new(x3, modulus.clone()),
                                y: FieldElement::new(y3, modulus.clone()),
                            };
                        }

                        let numerator = sub_mod(&y2.value, &y1.value, modulus);
                        let denominator = sub_mod(&x2.value, &x1.value, modulus);
                        let lambda = numerator * inv(&denominator, modulus) % modulus;
                        let x3 = sub_mod(
                            &sub_mod(&(&lambda * &lambda % modulus), &x1.value, modulus),
                            &x2.value,
                            modulus,
                        );
                        let y3 = sub_mod(
                            &(&lambda * sub_mod(&x1.value, &x3, modulus) % modulus),
                            &y1.value,
                            modulus,
                        );
                        Point::Affine {
                            x: FieldElement::new(x3, modulus.clone()),
                            y: FieldElement::new(y3, modulus.clone()),
                        }
                    }
                }
            }

            pub fn scalar_mul_vartime(&self, scalar: &BigUint, a: &FieldElement) -> Point {
                let mut scalar = scalar.clone();
                let mut addend = self.clone();
                let mut result = Point::Infinity;
                while !scalar.is_zero() {
                    if scalar.bit(0) {
                        result = result.add_vartime(&addend, a);
                    }
                    addend = addend.add_vartime(&addend, a);
                    scalar >>= 1usize;
                }
                result
            }
        }
    }

    pub mod curve {
        use num_bigint::BigUint;

        use super::field::FieldElement;
        use super::point::Point;

        #[derive(Clone, Debug)]
        pub struct CurveParams {
            pub name: &'static str,
            pub p: BigUint,
            pub a: BigUint,
            pub b: BigUint,
            pub gx: BigUint,
            pub gy: BigUint,
            pub n: BigUint,
            pub h: u32,
        }

        impl CurveParams {
            pub fn generator(&self) -> Point {
                Point::Affine {
                    x: FieldElement::new(self.gx.clone(), self.p.clone()),
                    y: FieldElement::new(self.gy.clone(), self.p.clone()),
                }
            }

            pub fn a_fe(&self) -> FieldElement {
                FieldElement::new(self.a.clone(), self.p.clone())
            }
        }
    }

    pub mod curve25519 {
        use num_bigint::BigUint;
        use num_traits::One;

        pub fn p() -> BigUint {
            (BigUint::one() << 255usize) - BigUint::from(19u32)
        }
    }
}

pub mod cryptanalysis {
    pub mod identity_certificate {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum Expr {
            Var(String),
            Const(i64),
            Neg(Box<Expr>),
            Add(Vec<Expr>),
            Mul(Vec<Expr>),
        }

        impl Expr {
            pub fn var(name: &str) -> Expr {
                Expr::Var(name.to_string())
            }

            pub fn plus(left: Expr, right: Expr) -> Expr {
                Expr::Add(vec![left, right])
            }

            pub fn minus(value: Expr) -> Expr {
                Expr::Neg(Box::new(value))
            }

            pub fn times(coefficient: Expr, value: Expr) -> Expr {
                Expr::Mul(vec![coefficient, value])
            }

            pub fn sum(terms: Vec<Expr>) -> Expr {
                Expr::Add(terms)
            }

            pub fn product(factors: Vec<Expr>) -> Expr {
                Expr::Mul(factors)
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct IdentityStatement {
            pub name: String,
            pub source: String,
            pub variables: Vec<String>,
            pub degree_bound: usize,
            pub lhs: Expr,
            pub rhs: Expr,
        }
    }
}

#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/curve25519_ic.rs"]
mod exact_model;

use ecc::point::Point;
use exact_model::Curve25519IcModel;

fn main() {
    let model = Curve25519IcModel::rfc7748();
    let curve = model.short_weierstrass_params();
    let a = curve.a_fe();
    let generator = curve.generator();

    let p = generator.scalar_mul_vartime(&BigUint::from(2u32), &a);
    let q = generator.scalar_mul_vartime(&BigUint::from(3u32), &a);
    let auxiliary = generator.scalar_mul_vartime(&BigUint::from(5u32), &a);
    let t = generator.scalar_mul_vartime(&BigUint::from(7u32), &a);
    let target = generator.scalar_mul_vartime(&BigUint::from(12u32), &a);

    assert_eq!(p.add_vartime(&q, &a), auxiliary);
    assert_eq!(auxiliary.add_vartime(&t, &a), target);
    assert!(model
        .semaev_s3(
            p.x_coord().expect("2G is finite"),
            q.x_coord().expect("3G is finite"),
            auxiliary.x_coord().expect("5G is finite"),
        )
        .is_zero());
    assert!(model
        .semaev_s3(
            auxiliary.x_coord().expect("5G is finite"),
            t.x_coord().expect("7G is finite"),
            target.x_coord().expect("12G is finite"),
        )
        .is_zero());

    let minus_p = p.neg_vartime();
    let minus_q = q.neg_vartime();
    assert_eq!(minus_p.x_coord(), p.x_coord());
    assert_eq!(minus_q.x_coord(), q.x_coord());
    let opposite_auxiliary = minus_p.add_vartime(&minus_q, &a);
    assert_eq!(opposite_auxiliary.x_coord(), auxiliary.x_coord());
    assert_ne!(opposite_auxiliary.add_vartime(&t, &a), target);

    let inverse_pair_auxiliary = p.add_vartime(&p.neg_vartime(), &a);
    assert_eq!(inverse_pair_auxiliary, Point::Infinity);
    assert!(inverse_pair_auxiliary.x_coord().is_none());
    assert_eq!(inverse_pair_auxiliary.add_vartime(&t, &a), t);

    println!(
        "{}",
        serde_json::json!({
            "schema": "curve25519-split-aux-contract-check/v1",
            "model_source": "src/cryptanalysis/curve25519_ic.rs",
            "point_arithmetic": "study-local-minimal-BigUint-affine-adapter",
            "field_prime": model.p.to_string(),
            "finite_split": {
                "scalars": {"p": 2, "q": 3, "auxiliary": 5, "t": 7, "target": 12},
                "exact_joins": true,
                "s3_left_zero": true,
                "s3_right_zero": true,
                "negated_inputs_same_abscissae": true,
                "negated_fixed_signed_relation": false
            },
            "identity_auxiliary": {
                "pair": "[2]G + [-2]G",
                "is_infinity": true,
                "has_affine_x": false,
                "exact_join": true
            },
            "performance_measurement": false,
            "status": "checked_witnesses_not_a_general_identity"
        })
    );
}
