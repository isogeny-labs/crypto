//! Narrow source bridge for the repository's exact prime-field F4 engine.
//!
//! The build script copies `src/cryptanalysis/f4_fp.rs` plus its reuse modules
//! into Cargo's generated directory.  These small support modules provide only
//! the interfaces that source needs; they do not copy or fork the F4 engine.

pub mod fx_hash {
    use std::collections::{HashMap, HashSet};
    use std::hash::{BuildHasherDefault, Hasher};

    #[derive(Clone, Default)]
    pub struct FxHasher {
        hash: u64,
    }

    impl Hasher for FxHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, bytes: &[u8]) {
            const MULTIPLIER: u64 = 0x517c_c1b7_2722_0a95;
            for byte in bytes {
                self.hash = (self.hash.rotate_left(5) ^ u64::from(*byte)).wrapping_mul(MULTIPLIER);
            }
        }
    }

    pub type FxBuildHasher = BuildHasherDefault<FxHasher>;
    pub type FxMap<K, V> = HashMap<K, V, FxBuildHasher>;
    pub type FxSet<K> = HashSet<K, FxBuildHasher>;
}

pub mod groebner_f4 {
    use std::cmp::Ordering as Comparison;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Ordering {
        Lex,
        Grlex,
        Grevlex,
    }

    pub fn cmp_monomial(left: &[u32], right: &[u32], ordering: Ordering) -> Comparison {
        debug_assert_eq!(left.len(), right.len());
        match ordering {
            Ordering::Lex => left.iter().cmp(right),
            Ordering::Grlex => left
                .iter()
                .sum::<u32>()
                .cmp(&right.iter().sum::<u32>())
                .then_with(|| cmp_monomial(left, right, Ordering::Lex)),
            Ordering::Grevlex => left
                .iter()
                .sum::<u32>()
                .cmp(&right.iter().sum::<u32>())
                .then_with(|| {
                    left.iter()
                        .zip(right)
                        .rev()
                        .find_map(|(left, right)| (left != right).then(|| right.cmp(left)))
                        .unwrap_or(Comparison::Equal)
                }),
        }
    }
}

pub mod jv_bitops {
    use serde::Serialize;
    use std::sync::atomic::{AtomicU64, Ordering};

    static FIELD_MULS: AtomicU64 = AtomicU64::new(0);
    static FIELD_ADDS: AtomicU64 = AtomicU64::new(0);
    static FIELD_INVS: AtomicU64 = AtomicU64::new(0);
    static ROW_OPS: AtomicU64 = AtomicU64::new(0);
    static WORD_OPS: AtomicU64 = AtomicU64::new(0);
    static PROBES: AtomicU64 = AtomicU64::new(0);

    #[derive(Clone, Copy, Debug)]
    pub enum Kind {
        FieldMul,
        FieldAdd,
        FieldInv,
    }

    #[derive(Clone, Copy, Debug, Default, Serialize)]
    pub struct Snapshot {
        pub field_multiplications: u64,
        pub field_additions: u64,
        pub field_inversions: u64,
        pub row_operations: u64,
        pub word_operations: u64,
        pub hash_probes: u64,
    }

    impl Snapshot {
        pub fn difference(self, earlier: Self) -> Self {
            Self {
                field_multiplications: self
                    .field_multiplications
                    .saturating_sub(earlier.field_multiplications),
                field_additions: self.field_additions.saturating_sub(earlier.field_additions),
                field_inversions: self
                    .field_inversions
                    .saturating_sub(earlier.field_inversions),
                row_operations: self.row_operations.saturating_sub(earlier.row_operations),
                word_operations: self.word_operations.saturating_sub(earlier.word_operations),
                hash_probes: self.hash_probes.saturating_sub(earlier.hash_probes),
            }
        }
    }

    pub fn snapshot() -> Snapshot {
        Snapshot {
            field_multiplications: FIELD_MULS.load(Ordering::Relaxed),
            field_additions: FIELD_ADDS.load(Ordering::Relaxed),
            field_inversions: FIELD_INVS.load(Ordering::Relaxed),
            row_operations: ROW_OPS.load(Ordering::Relaxed),
            word_operations: WORD_OPS.load(Ordering::Relaxed),
            hash_probes: PROBES.load(Ordering::Relaxed),
        }
    }

    pub fn enabled() -> bool {
        true
    }

    pub fn charge(kind: Kind, count: u64) {
        let counter = match kind {
            Kind::FieldMul => &FIELD_MULS,
            Kind::FieldAdd => &FIELD_ADDS,
            Kind::FieldInv => &FIELD_INVS,
        };
        counter.fetch_add(count, Ordering::Relaxed);
    }

    pub fn row_ops(count: u64) {
        ROW_OPS.fetch_add(count, Ordering::Relaxed);
    }

    pub fn words(count: u64) {
        WORD_OPS.fetch_add(count, Ordering::Relaxed);
    }

    pub fn probes(probes: u64, words_per_key: u64) {
        PROBES.fetch_add(probes, Ordering::Relaxed);
        WORD_OPS.fetch_add(probes.saturating_mul(words_per_key), Ordering::Relaxed);
    }

    pub fn sort_cost(items: usize) -> u64 {
        if items <= 1 {
            0
        } else {
            items as u64 * u64::from(usize::BITS - (items - 1).leading_zeros())
        }
    }
}

fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    (u128::from(left) * u128::from(right) % u128::from(modulus)) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

pub mod residual_walk {
    pub fn is_prime_u64(candidate: u64) -> bool {
        if candidate < 2 {
            return false;
        }
        for prime in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
            if candidate == prime {
                return true;
            }
            if candidate.is_multiple_of(prime) {
                return false;
            }
        }
        let shifts = (candidate - 1).trailing_zeros();
        let odd = (candidate - 1) >> shifts;
        for witness in [2u64, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022] {
            let witness = witness % candidate;
            if witness == 0 {
                continue;
            }
            let mut value = super::pow_mod(witness, odd, candidate);
            if value == 1 || value == candidate - 1 {
                continue;
            }
            let mut composite = true;
            for _ in 1..shifts {
                value = super::mul_mod(value, value, candidate);
                if value == candidate - 1 {
                    composite = false;
                    break;
                }
            }
            if composite {
                return false;
            }
        }
        true
    }
}

pub mod glv_invariant_base {
    use std::time::Instant;

    fn add(left: u64, right: u64, p: u64) -> u64 {
        ((u128::from(left) + u128::from(right)) % u128::from(p)) as u64
    }

    fn sub(left: u64, right: u64, p: u64) -> u64 {
        add(left, if right == 0 { 0 } else { p - right }, p)
    }

    fn mul(left: u64, right: u64, p: u64) -> u64 {
        super::mul_mod(left, right, p)
    }

    fn inverse(value: u64, p: u64) -> u64 {
        super::pow_mod(value, p - 2, p)
    }

    fn trim(polynomial: &mut Vec<u64>) {
        while polynomial.last() == Some(&0) {
            polynomial.pop();
        }
    }

    fn remainder(numerator: &[u64], denominator: &[u64], p: u64, operations: &mut u64) -> Vec<u64> {
        let mut denominator = denominator.to_vec();
        trim(&mut denominator);
        if denominator.is_empty() {
            let mut output = numerator.to_vec();
            trim(&mut output);
            return output;
        }
        let mut output = numerator.to_vec();
        trim(&mut output);
        let degree = denominator.len() - 1;
        let inverse_lead = inverse(denominator[degree], p);
        *operations += 1;
        while output.len() >= denominator.len() {
            let output_degree = output.len() - 1;
            let factor = mul(output[output_degree], inverse_lead, p);
            *operations += 1;
            let shift = output_degree - degree;
            for index in 0..degree {
                output[shift + index] =
                    sub(output[shift + index], mul(factor, denominator[index], p), p);
                *operations += 1;
            }
            output.pop();
            trim(&mut output);
        }
        output
    }

    fn multiply_mod(
        left: &[u64],
        right: &[u64],
        modulus: &[u64],
        p: u64,
        operations: &mut u64,
    ) -> Vec<u64> {
        if left.is_empty() || right.is_empty() {
            return Vec::new();
        }
        let mut product = vec![0; left.len() + right.len() - 1];
        for (i, left) in left.iter().copied().enumerate() {
            for (j, right) in right.iter().copied().enumerate() {
                if left != 0 && right != 0 {
                    product[i + j] = add(product[i + j], mul(left, right, p), p);
                    *operations += 1;
                }
            }
        }
        remainder(&product, modulus, p, operations)
    }

    fn power_mod(
        base: &[u64],
        mut exponent: u64,
        modulus: &[u64],
        p: u64,
        operations: &mut u64,
    ) -> Vec<u64> {
        let mut result = remainder(&[1], modulus, p, operations);
        let mut base = remainder(base, modulus, p, operations);
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = multiply_mod(&result, &base, modulus, p, operations);
            }
            exponent >>= 1;
            if exponent != 0 {
                base = multiply_mod(&base, &base, modulus, p, operations);
            }
        }
        result
    }

    fn gcd(mut left: Vec<u64>, mut right: Vec<u64>, p: u64, operations: &mut u64) -> Vec<u64> {
        trim(&mut left);
        trim(&mut right);
        while !right.is_empty() {
            let next = remainder(&left, &right, p, operations);
            left = right;
            right = next;
        }
        if let Some(lead) = left.last().copied() {
            let inverse = inverse(lead, p);
            *operations += 1;
            for coefficient in &mut left {
                *coefficient = mul(*coefficient, inverse, p);
                *operations += 1;
            }
        }
        left
    }

    fn quotient_exact(
        numerator: &[u64],
        denominator: &[u64],
        p: u64,
        operations: &mut u64,
    ) -> Vec<u64> {
        let mut work = numerator.to_vec();
        trim(&mut work);
        let mut denominator = denominator.to_vec();
        trim(&mut denominator);
        let degree = denominator.len() - 1;
        let inverse_lead = inverse(denominator[degree], p);
        *operations += 1;
        let mut quotient = vec![0; work.len() - degree];
        while work.len() >= denominator.len() {
            let work_degree = work.len() - 1;
            let shift = work_degree - degree;
            let factor = mul(work[work_degree], inverse_lead, p);
            quotient[shift] = factor;
            for index in 0..=degree {
                work[shift + index] =
                    sub(work[shift + index], mul(factor, denominator[index], p), p);
                *operations += 1;
            }
            trim(&mut work);
        }
        debug_assert!(work.is_empty());
        trim(&mut quotient);
        quotient
    }

    fn splitmix64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = *state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    pub fn poly_roots_fp_counted_until(
        polynomial: &[u64],
        p: u64,
        seed: u64,
        operations: &mut u64,
        deadline: Option<Instant>,
    ) -> Option<Vec<u64>> {
        let expired = || deadline.is_some_and(|deadline| Instant::now() >= deadline);
        if expired() {
            return None;
        }
        let mut polynomial = polynomial.to_vec();
        trim(&mut polynomial);
        if polynomial.len() <= 1 {
            return Some(Vec::new());
        }
        let mut x_to_p_minus_x = power_mod(&[0, 1], p, &polynomial, p, operations);
        if x_to_p_minus_x.len() < 2 {
            x_to_p_minus_x.resize(2, 0);
        }
        x_to_p_minus_x[1] = sub(x_to_p_minus_x[1], 1, p);
        trim(&mut x_to_p_minus_x);
        let rational_part = gcd(polynomial, x_to_p_minus_x, p, operations);
        let mut state = seed ^ 0x524f_4f54;
        let mut stack = vec![rational_part];
        let mut roots = Vec::new();
        while let Some(factor) = stack.pop() {
            if expired() {
                return None;
            }
            match factor.len() {
                0 | 1 => {}
                2 => {
                    *operations += 2;
                    roots.push(sub(0, mul(factor[0], inverse(factor[1], p), p), p));
                }
                _ => {
                    let shift = splitmix64(&mut state) % p;
                    let mut splitter = power_mod(&[shift, 1], (p - 1) / 2, &factor, p, operations);
                    if splitter.is_empty() {
                        splitter.push(0);
                    }
                    splitter[0] = sub(splitter[0], 1, p);
                    trim(&mut splitter);
                    let divisor = gcd(factor.clone(), splitter, p, operations);
                    if divisor.len() <= 1 || divisor.len() == factor.len() {
                        stack.push(factor);
                        continue;
                    }
                    let quotient = quotient_exact(&factor, &divisor, p, operations);
                    stack.push(divisor);
                    stack.push(quotient);
                }
            }
        }
        roots.sort_unstable();
        roots.dedup();
        (!expired()).then_some(roots)
    }
}

include!(concat!(env!("OUT_DIR"), "/f4_fp_bridge.rs"));
pub use f4_fp_host::f4_fp;

#[cfg(test)]
mod bridge_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn counted_root_bridge_finds_distinct_rational_roots() {
        // (X - 3)(X - 5) = X^2 - 8X + 15 over F_31.
        let mut operations = 0;
        let roots = glv_invariant_base::poly_roots_fp_counted_until(
            &[15, 23, 1],
            31,
            7,
            &mut operations,
            None,
        )
        .unwrap();
        assert_eq!(roots, vec![3, 5]);
        assert!(operations != 0);
    }

    #[test]
    fn exact_repository_f4_visits_toy_solutions() {
        let polynomial =
            f4_fp::normalise(&[(vec![2], 1), (vec![0], 30)], 31, f4_fp::Ordering::Grevlex);
        let options =
            f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 3).with_budget(Duration::from_secs(5));
        let mut solutions = Vec::new();
        let report = f4_fp::solve_visit(&[polynomial], 1, 31, &options, &[None], |assignment| {
            solutions.push(assignment[0]);
            false
        })
        .unwrap();
        solutions.sort_unstable();
        assert!(report.complete);
        assert!(!report.timed_out);
        assert_eq!(solutions, vec![1, 30]);
    }
}
