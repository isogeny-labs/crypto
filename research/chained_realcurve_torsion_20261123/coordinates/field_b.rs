use crate::inherited::{psub, pxgcd, Fld};

pub const Q: u32 = 531_441;
const HALF: usize = 729;

pub fn digits(mut code: u32, length: usize) -> Vec<u32> {
    (0..length)
        .map(|_| {
            let digit = code % 3;
            code /= 3;
            digit
        })
        .collect()
}

struct Prime3;

impl Fld for Prime3 {
    type E = u32;
    fn zero(&self) -> u32 {
        0
    }
    fn one(&self) -> u32 {
        1
    }
    fn add(&self, a: &u32, b: &u32) -> u32 {
        (a + b) % 3
    }
    fn sub(&self, a: &u32, b: &u32) -> u32 {
        (a + 3 - b) % 3
    }
    fn neg(&self, a: &u32) -> u32 {
        (3 - a) % 3
    }
    fn mul(&self, a: &u32, b: &u32) -> u32 {
        a * b % 3
    }
    fn inv(&self, a: &u32) -> u32 {
        assert!(*a == 1 || *a == 2);
        *a
    }
}

struct Slow {
    modulus: Vec<u32>,
}

impl Slow {
    fn mul(&self, a: u32, b: u32) -> u32 {
        let aa = digits(a, 12);
        let bb = digits(b, 12);
        let mut product = [0_u32; 23];
        for (i, x) in aa.iter().enumerate() {
            for (j, y) in bb.iter().enumerate() {
                product[i + j] = (product[i + j] + x * y) % 3;
            }
        }
        for n in (12..23).rev() {
            let c = product[n];
            for j in 0..12 {
                product[n - 12 + j] = (product[n - 12 + j] + 6 - c * self.modulus[j]) % 3;
            }
            product[n] = 0;
        }
        product[..12].iter().rev().fold(0, |code, x| 3 * code + x)
    }

    fn pow(&self, mut a: u32, mut n: u32) -> u32 {
        let mut result = 1;
        while n != 0 {
            if n & 1 == 1 {
                result = self.mul(result, a);
            }
            n >>= 1;
            if n != 0 {
                a = self.mul(a, a);
            }
        }
        result
    }

    fn irreducible(&self) -> bool {
        let mut full = self.modulus.clone();
        full.push(1);
        let mut power = 3;
        for i in 1..=12 {
            power = self.pow(power, 3);
            if i == 4 || i == 6 {
                let difference = psub(&Prime3, &digits(power, 12), &[0, 1]);
                if pxgcd(&Prime3, &full, &difference).0.len() != 1 {
                    return false;
                }
            }
        }
        power == 3
    }
}

pub struct Field {
    pub modulus: Vec<u32>,
    pub generator: u32,
    logarithm: Vec<u32>,
    exponential: Vec<u32>,
    addition: Vec<u16>,
}

impl Field {
    pub fn new() -> Self {
        let slow = (1..Q)
            .filter(|code| !code.is_multiple_of(3))
            .map(|code| Slow {
                modulus: digits(code, 12),
            })
            .find(Slow::irreducible)
            .expect("no irreducible degree12 polynomial in complete coefficient-code order");
        let generator = (2..Q)
            .find(|&g| {
                [2, 5, 7, 13, 73]
                    .iter()
                    .all(|p| slow.pow(g, (Q - 1) / p) != 1)
            })
            .unwrap();
        let mut logarithm = vec![u32::MAX; Q as usize];
        let mut exponential = Vec::with_capacity(2 * (Q - 1) as usize);
        let mut current = 1;
        for exponent in 0..Q - 1 {
            assert_eq!(logarithm[current as usize], u32::MAX);
            logarithm[current as usize] = exponent;
            exponential.push(current);
            current = slow.mul(current, generator);
        }
        assert_eq!(current, 1);
        assert!(logarithm[1..].iter().all(|x| *x != u32::MAX));
        exponential.extend_from_within(..);
        let mut addition = vec![0; HALF * HALF];
        for a in 0..HALF {
            for b in 0..HALF {
                let (mut aa, mut bb, mut scale, mut sum) = (a, b, 1, 0);
                for _ in 0..6 {
                    sum += ((aa % 3 + bb % 3) % 3) * scale;
                    scale *= 3;
                    aa /= 3;
                    bb /= 3;
                }
                addition[a * HALF + b] = sum as u16;
            }
        }
        let field = Self {
            modulus: slow.modulus.clone(),
            generator,
            logarithm,
            exponential,
            addition,
        };
        for a in 1..Q {
            assert_eq!(field.mul(&a, &field.inv(&a)), 1);
            assert_eq!(field.pow(a, Q), a);
        }
        for a in 0..27 {
            for b in 0..27 {
                assert_eq!(field.mul(&a, &b), slow.mul(a, b));
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
        assert_eq!(self.mul(&root, &root), a);
        Some(root.min(self.neg(&root)))
    }

    pub fn subfield27(&self) -> Vec<u32> {
        let alpha = self.pow(self.generator, (Q - 1) / 26);
        assert!(alpha != 1 && self.pow(alpha, 26) == 1 && self.pow(alpha, 2) != 1);
        let alpha2 = self.mul(&alpha, &alpha);
        let mut result = Vec::new();
        for a in 0..3 {
            for b in 0..3 {
                for c in 0..3 {
                    result.push(
                        self.add(&a, &self.add(&self.mul(&b, &alpha), &self.mul(&c, &alpha2))),
                    );
                }
            }
        }
        result.sort();
        result.dedup();
        assert_eq!(result.len(), 27);
        assert!(result.iter().all(|&a| self.pow(a, 27) == a));
        for a in &result {
            for b in &result {
                for c in &result {
                    assert_eq!(
                        self.mul(a, &self.add(b, c)),
                        self.add(&self.mul(a, b), &self.mul(a, c))
                    );
                    assert_eq!(self.mul(a, &self.mul(b, c)), self.mul(&self.mul(a, b), c));
                }
            }
        }
        result
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
        u32::from(self.addition[(a % 729) as usize * HALF + (b % 729) as usize])
            + 729 * u32::from(self.addition[(a / 729) as usize * HALF + (b / 729) as usize])
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
