use std::collections::BTreeSet;

#[derive(Clone, Copy)]
struct Field {
    degree: u32,
    modulus: u32,
    size: u32,
}

impl Field {
    fn new(degree: u32) -> Self {
        let size = 1 << degree;
        let modulus = (size | 1..2 * size)
            .step_by(2)
            .find(|&candidate| Self::is_irreducible(degree, candidate))
            .expect("irreducible polynomial exists");
        Self {
            degree,
            modulus,
            size,
        }
    }

    fn with_modulus(degree: u32, modulus: u32) -> Self {
        Self {
            degree,
            modulus,
            size: 1 << degree,
        }
    }

    fn mul(self, mut a: u32, mut b: u32) -> u32 {
        let mut out = 0;
        while b != 0 {
            if b & 1 != 0 {
                out ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & self.size != 0 {
                a ^= self.modulus;
            }
        }
        out
    }

    fn pow(self, mut a: u32, mut e: u32) -> u32 {
        let mut out = 1;
        while e != 0 {
            if e & 1 != 0 {
                out = self.mul(out, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        out
    }

    fn inv(self, a: u32) -> u32 {
        assert_ne!(a, 0);
        self.pow(a, self.size - 2)
    }

    fn sigma_poly(self, x: u32, q: u32, coefficients: [bool; 4]) -> u32 {
        let mut value = x;
        let mut out = 0;
        for coefficient in coefficients {
            if coefficient {
                out ^= value;
            }
            value = self.pow(value, q);
        }
        out
    }

    fn relative_trace(self, mut x: u32, q: u32) -> u32 {
        let mut out = 0;
        for _ in 0..7 {
            out ^= x;
            x = self.pow(x, q);
        }
        out
    }

    fn absolute_trace(self, mut x: u32) -> u32 {
        let mut out = 0;
        for _ in 0..self.degree {
            out ^= x;
            x = self.mul(x, x);
        }
        assert!(out <= 1);
        out
    }

    fn polynomial_degree(x: u32) -> u32 {
        31 - x.leading_zeros()
    }

    fn polynomial_remainder(mut a: u32, b: u32) -> u32 {
        let b_degree = Self::polynomial_degree(b);
        while a != 0 && Self::polynomial_degree(a) >= b_degree {
            a ^= b << (Self::polynomial_degree(a) - b_degree);
        }
        a
    }

    fn polynomial_gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let remainder = Self::polynomial_remainder(a, b);
            a = b;
            b = remainder;
        }
        a
    }

    fn is_irreducible(degree: u32, modulus: u32) -> bool {
        let ring = Self::with_modulus(degree, modulus);
        let mut powers = vec![2];
        for _ in 0..degree {
            powers.push(ring.mul(*powers.last().unwrap(), *powers.last().unwrap()));
        }
        if powers[degree as usize] != 2 {
            return false;
        }
        for prime in [2, 7] {
            if degree % prime == 0
                && Self::polynomial_gcd(powers[(degree / prime) as usize] ^ 2, modulus) != 1
            {
                return false;
            }
        }
        true
    }
}

const CUBICS: [[bool; 4]; 2] = [[true, true, false, true], [true, false, true, true]];

fn exact_second_order(field: Field, q: u32, p: [bool; 4], gamma: u32) -> bool {
    gamma != 0
        && field.relative_trace(gamma, q) != 0
        && field.pow(gamma, q) != gamma
        && field.sigma_poly(field.pow(gamma, q) ^ gamma, q, p) == 0
}

fn subgroup_data(field: Field, q: u32, p: [bool; 4]) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let subfield: Vec<_> = (0..field.size).filter(|&x| field.pow(x, q) == x).collect();
    let first: Vec<_> = (0..field.size)
        .filter(|&x| field.sigma_poly(x, q, p) == 0)
        .collect();
    assert_eq!(subfield.len() as u32, q);
    assert_eq!(first.len() as u32, q.pow(3));
    let mut projective = BTreeSet::new();
    for &v in first.iter().filter(|&&v| v != 0) {
        projective.insert(
            subfield
                .iter()
                .filter(|&&c| c != 0)
                .map(|&c| field.mul(c, v))
                .min()
                .unwrap(),
        );
    }
    assert_eq!(projective.len() as u32, q * q + q + 1);
    (subfield, first, projective.into_iter().collect())
}

fn projective_membership(field: Field, q: u32, p: [bool; 4], z: u32, inverses: &[u32]) -> bool {
    if z == 0 || field.relative_trace(z, q) != 0 {
        return false;
    }
    inverses
        .iter()
        .any(|&inv| exact_second_order(field, q, p, field.mul(z, inv)))
}

fn direct_locus(
    field: Field,
    q: u32,
    p: [bool; 4],
    subfield: &[u32],
    first: &[u32],
) -> BTreeSet<u32> {
    let mut out = BTreeSet::new();
    for &v in first.iter().filter(|&&v| v != 0) {
        for &w in first.iter().filter(|&&w| w != 0) {
            for &c in subfield.iter().filter(|&&c| c != 0) {
                let gamma = w ^ c;
                assert!(exact_second_order(field, q, p, gamma));
                out.insert(field.mul(v, gamma));
            }
        }
    }
    out
}

fn curve_order(field: Field, a: u32, b: u32) -> u32 {
    let mut order = 2; // point at infinity and the unique point with x = 0
    for x in 1..field.size {
        let x2 = field.mul(x, x);
        let rhs = x ^ a ^ field.mul(b, field.inv(x2));
        if field.absolute_trace(rhs) == 0 {
            order += 2;
        }
    }
    order
}

fn main() {
    for ell in [1, 2] {
        let q = 1 << ell;
        let field = Field::new(7 * ell);
        let mut union = BTreeSet::new();
        let mut examples = Vec::new();
        for (index, p) in CUBICS.into_iter().enumerate() {
            let (subfield, first, projective) = subgroup_data(field, q, p);
            let inverses: Vec<_> = projective.iter().map(|&v| field.inv(v)).collect();
            let direct = direct_locus(field, q, p, &subfield, &first);
            assert!(direct.iter().all(|&z| field.relative_trace(z, q) == 0));
            let predicted: BTreeSet<_> = (1..field.size)
                .filter(|&z| projective_membership(field, q, p, z, &inverses))
                .collect();
            assert_eq!(direct, predicted);
            let z = *direct.first().unwrap();
            let b = field.mul(z, z);
            let a = (0..field.size)
                .find(|&candidate| field.absolute_trace(candidate) == 1)
                .unwrap();
            examples.push((index, a, b, curve_order(field, a, b)));
            println!(
                "q={q} modulus=0x{:x} cubic={} dimV=3 projective={} W1={} exact=true",
                field.modulus,
                index + 1,
                projective.len(),
                direct.len()
            );
            union.extend(direct);
        }
        let trace_zero_nonzero = (1..field.size)
            .filter(|&z| field.relative_trace(z, q) == 0)
            .count();
        println!(
            "q={q} W1_union={} trace_zero_nonzero={} examples={examples:?}",
            union.len(),
            trace_zero_nonzero
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn irreducible_moduli_and_field_inverses() {
        for degree in [7, 14] {
            let field = Field::new(degree);
            for x in 1..field.size {
                assert_eq!(field.mul(x, field.inv(x)), 1);
            }
        }
    }

    #[test]
    fn small_field_membership_matches_direct_products() {
        for ell in [1, 2] {
            let q = 1 << ell;
            let field = Field::new(7 * ell);
            for p in CUBICS {
                let (subfield, first, projective) = subgroup_data(field, q, p);
                let inverses: Vec<_> = projective.iter().map(|&v| field.inv(v)).collect();
                let direct = direct_locus(field, q, p, &subfield, &first);
                for z in 1..field.size {
                    assert_eq!(
                        direct.contains(&z),
                        projective_membership(field, q, p, z, &inverses)
                    );
                }
            }
        }
    }
}
