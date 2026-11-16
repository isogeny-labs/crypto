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

fn incidence_summary(counts: &[u16]) -> (usize, u64, u64, u128, BTreeMap<u16, usize>) {
    let mut histogram = BTreeMap::new();
    let mut union = 0;
    let mut first_moment = 0_u64;
    let mut second_moment = 0_u64;
    for &count in counts.iter().filter(|&&count| count != 0) {
        union += 1;
        first_moment += u64::from(count);
        second_moment += u64::from(count) * u64::from(count);
        *histogram.entry(count).or_insert(0) += 1;
    }
    let lower =
        (u128::from(first_moment) * u128::from(first_moment)).div_ceil(u128::from(second_moment));
    (union, first_moment, second_moment, lower, histogram)
}

fn incidence_census() {
    for (ell, expected_single, expected_union) in
        [(1, 42, 56), (2, 2898, 3906), (3, 182938, 237839)]
    {
        let q = 1 << ell;
        let field = Field::new(7 * ell);
        let mut total = vec![0_u16; field.size as usize];
        for (index, p) in CUBICS.into_iter().enumerate() {
            let (subfield, first, projective) = subgroup_data(field, q, p);
            let mut counts = vec![0_u16; field.size as usize];
            for &r in &projective {
                let mut line = BTreeSet::new();
                for &w in first.iter().filter(|&&w| w != 0) {
                    for &c in subfield.iter().filter(|&&c| c != 0) {
                        line.insert(field.mul(r, w ^ c));
                    }
                }
                assert_eq!(line.len() as u32, (q.pow(3) - 1) * (q - 1));
                for z in line {
                    counts[z as usize] += 1;
                    total[z as usize] += 1;
                }
            }
            let (union, first_moment, second_moment, lower, histogram) = incidence_summary(&counts);
            assert_eq!(union, expected_single);
            assert!(counts
                .iter()
                .enumerate()
                .all(|(z, &count)| { count == 0 || field.relative_trace(z as u32, q) == 0 }));
            println!(
                "q={q} cubic={} lines={} set_size={} union={union} M={first_moment} E={second_moment} cauchy_lower={lower} histogram={histogram:?}",
                index + 1,
                projective.len(),
                (q.pow(3) - 1) * (q - 1)
            );
        }
        let (union, first_moment, second_moment, lower, histogram) = incidence_summary(&total);
        assert_eq!(union, expected_union);
        println!(
            "q={q} both_cubics union={union} M={first_moment} E={second_moment} cauchy_lower={lower} histogram={histogram:?}"
        );
    }
}

fn bitset_insert(bits: &mut [u8], value: u32) {
    let index = (value / 8) as usize;
    bits[index] |= 1 << (value % 8);
}

fn q16_product_census() {
    let q = 16;
    let field = Field::new(28);
    let bitset_len = (field.size / 8) as usize;
    let mut cubic_bitsets = Vec::new();
    let mut cubic_counts = Vec::new();
    for (index, p) in CUBICS.into_iter().enumerate() {
        let (subfield, first, projective) = subgroup_data(field, q, p);
        let mut bits = vec![0_u8; bitset_len];
        for &r in &projective {
            for &w in first.iter().filter(|&&w| w != 0) {
                for &c in subfield.iter().filter(|&&c| c != 0) {
                    bitset_insert(&mut bits, field.mul(r, w ^ c));
                }
            }
        }
        let count: u64 = bits.iter().map(|&byte| u64::from(byte.count_ones())).sum();
        println!(
            "q=16 modulus=0x{:x} cubic={} lines={} set_size={} unique={count}",
            field.modulus,
            index + 1,
            projective.len(),
            (q.pow(3) - 1) * (q - 1)
        );
        cubic_counts.push(count);
        cubic_bitsets.push(bits);
    }
    let union: u64 = cubic_bitsets[0]
        .iter()
        .zip(&cubic_bitsets[1])
        .map(|(&a, &b)| u64::from((a | b).count_ones()))
        .sum();
    let overlap = cubic_counts[0] + cubic_counts[1] - union;
    let trace_zero_nonzero = u64::from(q.pow(6) - 1);
    let complement = trace_zero_nonzero - union;
    let predicted = u64::from((q - 1) * (q.pow(3) - 1));
    println!(
        "q=16 union={union} overlap={overlap} trace_zero_nonzero={trace_zero_nonzero} complement={complement} nonsplit_prediction={predicted} prediction_holds={}",
        complement == predicted
    );
}

fn binary_rank(values: impl IntoIterator<Item = u32>) -> u32 {
    let mut pivots = [0_u32; 32];
    let mut rank = 0;
    for mut value in values {
        while value != 0 {
            let pivot = Field::polynomial_degree(value) as usize;
            if pivots[pivot] == 0 {
                pivots[pivot] = value;
                rank += 1;
                break;
            }
            value ^= pivots[pivot];
        }
    }
    rank
}

fn rank_pair_census() {
    for ell in [1, 2, 3, 4] {
        let q = 1 << ell;
        let field = Field::new(7 * ell);
        let k_binary = binary_kernel(field, |x| field.pow(x, q) ^ x);
        for (index, p) in CUBICS.into_iter().enumerate() {
            let (subfield, first, projective) = subgroup_data(field, q, p);
            let mut v_basis = Vec::new();
            for candidate in first {
                if !in_k_span(field, candidate, &v_basis, &subfield) {
                    v_basis.push(candidate);
                }
                if v_basis.len() == 3 {
                    break;
                }
            }
            assert_eq!(v_basis.len(), 3);
            let h_basis: Vec<_> = [1_u32, v_basis[0], v_basis[1], v_basis[2]]
                .into_iter()
                .flat_map(|h| k_binary.iter().map(move |&c| field.mul(h, c)))
                .collect();
            assert_eq!(binary_rank(h_basis.iter().copied()), 4 * ell);
            let subspaces: Vec<Vec<u32>> = projective
                .iter()
                .map(|&r| h_basis.iter().map(|&h| field.mul(r, h)).collect())
                .collect();
            let mut histogram = BTreeMap::<u32, u32>::new();
            for i in 0..subspaces.len() {
                for j in i + 1..subspaces.len() {
                    let rank = binary_rank(subspaces[i].iter().chain(&subspaces[j]).copied());
                    assert_eq!(rank % ell, 0);
                    let intersection_dim = 8 - rank / ell;
                    *histogram.entry(intersection_dim).or_insert(0) += 1;
                }
            }
            let expected_pairs = (projective.len() * (projective.len() - 1) / 2) as u32;
            assert_eq!(histogram.values().sum::<u32>(), expected_pairs);
            println!(
                "q={q} modulus=0x{:x} cubic={} projective={} pairs={expected_pairs} intersection_dims={histogram:?}",
                field.modulus,
                index + 1,
                projective.len()
            );
        }
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--rank-pairs") {
        rank_pair_census();
        return;
    }
    if std::env::args().any(|arg| arg == "--q16-products") {
        q16_product_census();
        return;
    }
    if std::env::args().any(|arg| arg == "--incidence") {
        incidence_census();
        return;
    }
    if std::env::args().any(|arg| arg == "--q8-direct") {
        let q = 8;
        let field = Field::new(21);
        let mut union = BTreeSet::new();
        for (index, p) in CUBICS.into_iter().enumerate() {
            let (subfield, first, projective) = subgroup_data(field, q, p);
            let direct = direct_locus(field, q, p, &subfield, &first);
            assert!(direct.iter().all(|&z| field.relative_trace(z, q) == 0));
            println!(
                "q={q} modulus=0x{:x} cubic={} projective={} W1={}",
                field.modulus,
                index + 1,
                projective.len(),
                direct.len()
            );
            union.extend(direct);
        }
        let trace_zero_nonzero = q.pow(6) - 1;
        let missing = trace_zero_nonzero - union.len() as u32;
        println!(
            "q={q} W1_union={} trace_zero_nonzero={trace_zero_nonzero} missing={missing} conjectured_missing={}",
            union.len(),
            (q - 1) * (q.pow(3) - 1)
        );
        return;
    }
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
