use crate::base27::F27;
use crate::inherited::Fld;

pub const Q: u32 = 729;

pub struct Field {
    pub generator: u32,
    logarithm: Vec<u32>,
    exponential: Vec<u32>,
    addition: Vec<u16>,
}

impl Field {
    fn raw_mul(a: u32, b: u32) -> u32 {
        let f = F27;
        let (a0, a1, b0, b1) = (a % 27, a / 27, b % 27, b / 27);
        let real = f.add(&f.mul(&a0, &b0), &f.mul(&2, &f.mul(&a1, &b1)));
        let imaginary = f.add(&f.mul(&a0, &b1), &f.mul(&a1, &b0));
        real + 27 * imaginary
    }

    fn raw_pow(mut a: u32, mut n: u32) -> u32 {
        let mut result = 1;
        while n != 0 {
            if n & 1 == 1 {
                result = Self::raw_mul(result, a);
            }
            n >>= 1;
            if n != 0 {
                a = Self::raw_mul(a, a);
            }
        }
        result
    }

    pub fn new() -> Self {
        let base = F27;
        assert_eq!(
            base.pow(2, 13),
            2,
            "quadratic extension requires nonsquare2"
        );
        let generator = (2..Q)
            .find(|&g| {
                [2, 7, 13]
                    .iter()
                    .all(|p| Self::raw_pow(g, (Q - 1) / p) != 1)
            })
            .unwrap();
        let mut logarithm = vec![u32::MAX; Q as usize];
        let mut exponential = Vec::new();
        let mut value = 1;
        for exponent in 0..Q - 1 {
            assert_eq!(logarithm[value as usize], u32::MAX);
            logarithm[value as usize] = exponent;
            exponential.push(value);
            value = Self::raw_mul(value, generator);
        }
        assert_eq!(value, 1);
        assert!(logarithm[1..].iter().all(|&x| x != u32::MAX));
        exponential.extend_from_within(..);
        let mut addition = vec![0; (Q * Q) as usize];
        for a in 0..Q {
            for b in 0..Q {
                let sum = base.add(&(a % 27), &(b % 27)) + 27 * base.add(&(a / 27), &(b / 27));
                let (mut aa, mut bb, mut weight, mut reference) = (a, b, 1, 0);
                for _ in 0..6 {
                    reference += ((aa % 3 + bb % 3) % 3) * weight;
                    aa /= 3;
                    bb /= 3;
                    weight *= 3;
                }
                assert_eq!(sum, reference);
                addition[(a * Q + b) as usize] = sum as u16;
            }
        }
        let field = Self {
            generator,
            logarithm,
            exponential,
            addition,
        };
        for a in 0..Q {
            for b in 0..Q {
                assert_eq!(field.mul(&a, &b), Self::raw_mul(a, b));
            }
            assert_eq!(field.pow(a, Q), a);
            if a != 0 {
                let (a0, a1) = (a % 27, a / 27);
                let norm = base.sub(&base.mul(&a0, &a0), &base.mul(&2, &base.mul(&a1, &a1)));
                assert_ne!(norm, 0);
                let inverse_norm = base.inv(&norm);
                let reference =
                    base.mul(&a0, &inverse_norm) + 27 * base.neg(&base.mul(&a1, &inverse_norm));
                assert_eq!(field.inv(&a), reference);
                assert_eq!(field.mul(&a, &field.inv(&a)), 1);
            }
            if let Some(root) = field.sqrt(a) {
                assert_eq!(field.mul(&root, &root), a);
            } else {
                assert_eq!(field.pow(a, (Q - 1) / 2), 2);
            }
        }
        for a in [1, 3, 9, 27, 81, 243] {
            for b in [1, 3, 9, 27, 81, 243] {
                for c in [1, 3, 9, 27, 81, 243] {
                    assert_eq!(
                        field.mul(&a, &field.add(&b, &c)),
                        field.add(&field.mul(&a, &b), &field.mul(&a, &c))
                    );
                }
            }
        }
        field
    }

    pub fn pow(&self, mut a: u32, mut n: u32) -> u32 {
        let mut result = 1;
        while n != 0 {
            if n & 1 == 1 {
                result = self.mul(&result, &a);
            }
            n >>= 1;
            if n != 0 {
                a = self.mul(&a, &a);
            }
        }
        result
    }

    pub fn sqrt(&self, a: u32) -> Option<u32> {
        if a == 0 {
            return Some(0);
        }
        let exponent = self.logarithm[a as usize];
        if exponent & 1 == 1 {
            return None;
        }
        let root = self.exponential[(exponent / 2) as usize];
        Some(root.min(self.neg(&root)))
    }
}

impl Fld for Field {
    type E = u32;
    fn zero(&self) -> u32 {
        0
    }
    fn one(&self) -> u32 {
        1
    }
    fn add(&self, a: &u32, b: &u32) -> u32 {
        u32::from(self.addition[(a * Q + b) as usize])
    }
    fn sub(&self, a: &u32, b: &u32) -> u32 {
        self.add(a, &self.neg(b))
    }
    fn neg(&self, a: &u32) -> u32 {
        self.add(a, a)
    }
    fn mul(&self, a: &u32, b: &u32) -> u32 {
        if *a == 0 || *b == 0 {
            return 0;
        }
        self.exponential[(self.logarithm[*a as usize] + self.logarithm[*b as usize]) as usize]
    }
    fn inv(&self, a: &u32) -> u32 {
        assert_ne!(*a, 0);
        self.exponential[(Q - 1 - self.logarithm[*a as usize]) as usize]
    }
}
