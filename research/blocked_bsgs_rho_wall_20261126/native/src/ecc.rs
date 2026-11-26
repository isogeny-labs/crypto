pub mod field {
    use num_bigint::BigUint;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct FieldElement {
        pub value: BigUint,
        pub modulus: BigUint,
    }

    impl FieldElement {
        pub fn new(value: BigUint, modulus: BigUint) -> Self {
            assert!(modulus > BigUint::from(2u8));
            Self {
                value: value % &modulus,
                modulus,
            }
        }

        pub fn inv(&self) -> Option<Self> {
            if self.value == BigUint::from(0u8) {
                return None;
            }
            let exponent = &self.modulus - BigUint::from(2u8);
            Some(Self::new(
                self.value.modpow(&exponent, &self.modulus),
                self.modulus.clone(),
            ))
        }
    }
}

pub mod point {
    use num_bigint::BigUint;

    use super::field::FieldElement;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Point {
        Infinity,
        Affine { x: FieldElement, y: FieldElement },
    }

    fn add_mod(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        (a + b) % p
    }

    fn sub_mod(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        if a >= b {
            (a - b) % p
        } else {
            (p - ((b - a) % p)) % p
        }
    }

    fn mul_mod(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        (a * b) % p
    }

    fn inv_mod(a: &BigUint, p: &BigUint) -> Option<BigUint> {
        if a == &BigUint::from(0u8) {
            None
        } else {
            Some(a.modpow(&(p - BigUint::from(2u8)), p))
        }
    }

    impl Point {
        pub fn add(&self, other: &Self, a: &FieldElement) -> Self {
            match (self, other) {
                (Self::Infinity, q) | (q, Self::Infinity) => q.clone(),
                (Self::Affine { x: x1, y: y1 }, Self::Affine { x: x2, y: y2 }) => {
                    assert_eq!(x1.modulus, x2.modulus);
                    assert_eq!(x1.modulus, y1.modulus);
                    assert_eq!(x1.modulus, y2.modulus);
                    assert_eq!(x1.modulus, a.modulus);
                    let p = &x1.modulus;
                    if x1.value == x2.value
                        && add_mod(&y1.value, &y2.value, p) == BigUint::from(0u8)
                    {
                        return Self::Infinity;
                    }
                    let slope = if x1.value == x2.value && y1.value == y2.value {
                        if y1.value == BigUint::from(0u8) {
                            return Self::Infinity;
                        }
                        let three_x2 =
                            mul_mod(&BigUint::from(3u8), &mul_mod(&x1.value, &x1.value, p), p);
                        let num = add_mod(&three_x2, &a.value, p);
                        let den = mul_mod(&BigUint::from(2u8), &y1.value, p);
                        mul_mod(&num, &inv_mod(&den, p).expect("nonzero tangent"), p)
                    } else {
                        let num = sub_mod(&y2.value, &y1.value, p);
                        let den = sub_mod(&x2.value, &x1.value, p);
                        mul_mod(&num, &inv_mod(&den, p).expect("distinct x"), p)
                    };
                    let x3 = sub_mod(
                        &sub_mod(&mul_mod(&slope, &slope, p), &x1.value, p),
                        &x2.value,
                        p,
                    );
                    let y3 = sub_mod(
                        &mul_mod(&slope, &sub_mod(&x1.value, &x3, p), p),
                        &y1.value,
                        p,
                    );
                    Self::Affine {
                        x: FieldElement::new(x3, p.clone()),
                        y: FieldElement::new(y3, p.clone()),
                    }
                }
            }
        }

        pub fn scalar_mul(&self, k: &BigUint, a: &FieldElement) -> Self {
            let mut scalar = k.clone();
            let mut base = self.clone();
            let mut out = Self::Infinity;
            while scalar != BigUint::from(0u8) {
                if (&scalar & BigUint::from(1u8)) == BigUint::from(1u8) {
                    out = out.add(&base, a);
                }
                scalar >>= 1usize;
                if scalar != BigUint::from(0u8) {
                    base = base.add(&base, a);
                }
            }
            out
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
