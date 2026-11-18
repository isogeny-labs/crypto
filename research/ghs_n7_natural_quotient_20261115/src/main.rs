use std::collections::{BTreeMap, BTreeSet};

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
        for prime in [2, 3, 7] {
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

fn binary_kernel(field: Field, map: impl Fn(u32) -> u32) -> Vec<u32> {
    let mut pivot_values = [0_u32; 32];
    let mut pivot_witnesses = [0_u32; 32];
    let mut kernel = Vec::new();
    for bit in 0..field.degree {
        let mut value = map(1 << bit);
        let mut witness = 1 << bit;
        loop {
            if value == 0 {
                kernel.push(witness);
                break;
            }
            let pivot = Field::polynomial_degree(value) as usize;
            if pivot_values[pivot] == 0 {
                pivot_values[pivot] = value;
                pivot_witnesses[pivot] = witness;
                break;
            }
            value ^= pivot_values[pivot];
            witness ^= pivot_witnesses[pivot];
        }
    }
    assert!(kernel.iter().all(|&x| map(x) == 0));
    kernel
}

fn binary_span(basis: &[u32]) -> Vec<u32> {
    (0..1 << basis.len())
        .map(|mask| {
            basis.iter().enumerate().fold(0, |sum, (bit, &value)| {
                if mask & (1 << bit) != 0 {
                    sum ^ value
                } else {
                    sum
                }
            })
        })
        .collect()
}

fn in_k_span(field: Field, candidate: u32, basis: &[u32], subfield: &[u32]) -> bool {
    match basis {
        [] => candidate == 0,
        [v] => subfield.iter().any(|&c| field.mul(c, *v) == candidate),
        [v, w] => subfield.iter().any(|&c| {
            subfield
                .iter()
                .any(|&d| field.mul(c, *v) ^ field.mul(d, *w) == candidate)
        }),
        _ => panic!("only two previously chosen vectors are needed"),
    }
}

fn subgroup_data(field: Field, q: u32, p: [bool; 4]) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let subfield_basis = binary_kernel(field, |x| field.pow(x, q) ^ x);
    let subfield = binary_span(&subfield_basis);
    let first_binary_basis = binary_kernel(field, |x| field.sigma_poly(x, q, p));
    assert_eq!(first_binary_basis.len() as u32, 3 * q.trailing_zeros());
    let mut first_k_basis = Vec::new();
    for candidate in first_binary_basis {
        if !in_k_span(field, candidate, &first_k_basis, &subfield) {
            first_k_basis.push(candidate);
        }
        if first_k_basis.len() == 3 {
            break;
        }
    }
    assert_eq!(first_k_basis.len(), 3);
    let [v0, v1, v2] = [first_k_basis[0], first_k_basis[1], first_k_basis[2]];
    let mut first = BTreeSet::new();
    let mut projective = BTreeSet::new();
    for &a in &subfield {
        for &b in &subfield {
            for &c in &subfield {
                first.insert(field.mul(a, v0) ^ field.mul(b, v1) ^ field.mul(c, v2));
            }
            projective.insert(v0 ^ field.mul(a, v1) ^ field.mul(b, v2));
        }
        projective.insert(v1 ^ field.mul(a, v2));
    }
    projective.insert(v2);
    assert_eq!(subfield.len() as u32, q);
    assert_eq!(first.len() as u32, q.pow(3));
    assert_eq!(projective.len() as u32, q * q + q + 1);
    assert!(first.iter().all(|&x| field.sigma_poly(x, q, p) == 0));
    (
        subfield,
        first.into_iter().collect(),
        projective.into_iter().collect(),
    )
}

fn main() {
    for ell in [1_u32, 2_u32] {
        let q = 1 << ell;
        let field = Field::new(7 * ell);
        let mut union = BTreeSet::new();
        let mut classical_union = BTreeSet::new();
        for (arm, p) in CUBICS.into_iter().enumerate() {
            let (subfield, first, _) = subgroup_data(field, q, p);
            let classical: BTreeSet<u32> = first.iter().copied().filter(|&x| x != 0).collect();
            let mut w1 = BTreeSet::new();
            let mut symmetric = BTreeSet::new();
            let mut presentations = 0_u64;
            let mut symmetric_presentations = 0_u64;
            let mut example_nonclassical = None;
            for &gamma1 in first.iter().filter(|&&v| v != 0) {
                let delta1 = field.pow(gamma1, q) ^ gamma1;
                assert_ne!(delta1, 0);
                for &v in first.iter().filter(|&&v| v != 0) {
                    for &d in subfield.iter().filter(|&&d| d != 0) {
                        let gamma2 = v ^ d;
                        assert!(exact_second_order(field, q, p, gamma2));
                        let delta2 = field.pow(gamma2, q) ^ gamma2;
                        let b = field.mul(field.mul(gamma1, gamma2), field.mul(gamma1, gamma2));
                        presentations += 1;
                        w1.insert(b);
                        let has_swap = subfield
                            .iter()
                            .filter(|&&c| c != 0)
                            .any(|&c| delta2 == field.mul(c, delta1));
                        if has_swap {
                            symmetric_presentations += 1;
                            symmetric.insert(b);
                            assert!(classical.contains(&b));
                        } else if !classical.contains(&b) && example_nonclassical.is_none() {
                            example_nonclassical = Some((gamma1, gamma2, b));
                        }
                    }
                }
            }
            assert_eq!(w1.len(), if q == 2 { 42 } else { 2898 });
            assert_eq!(symmetric, classical);
            assert!(classical.is_subset(&w1));
            println!("q={q} modulus=0x{:x} arm={} W1={} classical={} swap_locus={} presentations={} swap_presentations={} nonclassical_example={:?}",
                field.modulus, arm + 1, w1.len(), classical.len(), symmetric.len(), presentations, symmetric_presentations, example_nonclassical);
            union.extend(w1);
            classical_union.extend(classical);
        }
        assert_eq!(union.len(), if q == 2 { 56 } else { 3906 });
        assert_eq!(classical_union.len(), 2 * (q.pow(3) - 1) as usize);
        println!(
            "q={q} union={} classical_union={}",
            union.len(),
            classical_union.len()
        );
    }
}
