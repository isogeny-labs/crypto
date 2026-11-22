//! Pure finite-selector resultant construction. Not registered or dispatched.
//! Caller authenticates the original irreducible field and signed domain; this
//! component checks representation and algebra over that supplied quotient only.
use crate::binary_ecc::{F2mElement, IrreduciblePoly};
use crate::cryptanalysis::binary_semaev::binary_semaev_s4;
use crate::cryptanalysis::pq_groebner_f2::{cmp_mono, F2BoolMono, F2BoolPoly, Mask};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(super) enum Failure {
    Input,
    Admission,
    Deadline,
    Allocation,
    TallyActive,
    Mismatch,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Quote {
    pub work: u64,
    pub bytes: u64,
}
/// Must charge the existing original cumulative ledger, preserve its first stop,
/// and check its existing absolute deadline. No constructor here creates a budget.
pub(super) trait OriginalAdmission {
    fn reserve(&mut self, quote: Quote) -> Result<(), Failure>;
    fn check_deadline(&mut self) -> Result<(), Failure>;
}
pub(super) struct Input<'a> {
    pub field: &'a IrreduciblePoly,
    pub b: &'a F2mElement,
    pub target_x: &'a F2mElement,
    pub codes: [&'a F2mElement; 2],
}
pub(super) struct Constructed {
    n: u32,
    modulus: u64,
    b: u64,
    target_x: u64,
    codes: [u64; 2],
    values: [u64; 8],
    pub equations: Vec<F2BoolPoly>,
}
/// Fixed-width coordinate ANFs and all eight selector values. Distinct from
/// group completeness or native matrix traces; every coordinate has eight bits.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Evidence {
    pub n: u32,
    pub modulus: u64,
    pub b: u64,
    pub target_x: u64,
    pub codes: [u64; 2],
    pub values: [u64; 8],
    pub coordinate_anfs: Vec<u8>,
}
pub(super) fn evidence(
    built: &Constructed,
    admission: &mut impl OriginalAdmission,
) -> Result<Evidence, Failure> {
    admission.reserve(Quote {
        work: 4096 + 61 * 8 * 16,
        bytes: 128,
    })?;
    admission.check_deadline()?;
    if !(2..=61).contains(&built.n)
        || built.equations.len() != built.n as usize
        || built.equations.iter().any(|p| {
            p.n_vars != 3
                || p.terms.len() > 8
                || p.terms.iter().any(|m| m.mask > 7)
                || !p.is_canonical()
        })
    {
        return Err(Failure::Input);
    }
    let mut coordinate_anfs = Vec::new();
    coordinate_anfs
        .try_reserve_exact(built.equations.len())
        .map_err(|_| Failure::Allocation)?;
    for p in &built.equations {
        let mut bits = 0u8;
        for term in &p.terms {
            bits |= 1u8 << term.mask;
        }
        coordinate_anfs.push(bits);
    }
    Ok(Evidence {
        n: built.n,
        modulus: built.modulus,
        b: built.b,
        target_x: built.target_x,
        codes: built.codes,
        values: built.values,
        coordinate_anfs,
    })
}
pub(super) fn verify_evidence(
    input: &Input<'_>,
    saved: &Evidence,
    admission: &mut impl OriginalAdmission,
) -> Result<(), Failure> {
    // Decode allocation is bounded and charged before constructing polynomials.
    admission.reserve(Quote {
        work: 16384,
        bytes: 61
            * (std::mem::size_of::<F2BoolPoly>() + 8 * std::mem::size_of::<F2BoolMono>()) as u64,
    })?;
    admission.check_deadline()?;
    if saved.n > 61 || saved.coordinate_anfs.len() != saved.n as usize {
        return Err(Failure::Input);
    }
    let mut equations = Vec::new();
    equations
        .try_reserve_exact(saved.n as usize)
        .map_err(|_| Failure::Allocation)?;
    for &bits in &saved.coordinate_anfs {
        let mut terms = Vec::new();
        terms
            .try_reserve_exact(8)
            .map_err(|_| Failure::Allocation)?;
        for bit in 0..8 {
            if bits & (1 << bit) != 0 {
                terms.push(F2BoolMono::from_mask(bit as Mask));
            }
        }
        terms.sort_unstable_by(|a, b| cmp_mono(*b, *a));
        equations.push(F2BoolPoly { terms, n_vars: 3 });
    }
    let built = Constructed {
        n: saved.n,
        modulus: saved.modulus,
        b: saved.b,
        target_x: saved.target_x,
        codes: saved.codes,
        values: saved.values,
        equations,
    };
    verify(input, &built, admission)
}

fn word(e: &F2mElement, n: u32) -> Result<u64, Failure> {
    if e.m_value() != n || e.raw_bits().len() != 1 || e.raw_bits()[0] >> n != 0 {
        return Err(Failure::Input);
    }
    Ok(e.raw_bits()[0])
}
fn inspect(
    input: &Input<'_>,
    admission: &mut impl OriginalAdmission,
) -> Result<(u32, u64, [u64; 4]), Failure> {
    // Fixed metadata admission precedes validation, including tally inspection.
    admission.reserve(Quote {
        work: 4096,
        bytes: 0,
    })?;
    admission.check_deadline()?;
    let n = input.field.degree;
    if !(2..=61).contains(&n)
        || input.field.low_terms.is_empty()
        || input.field.low_terms.len() > 61
    {
        return Err(Failure::Input);
    }
    if input.field.low_terms[0] != 0
        || input.field.low_terms.iter().any(|t| *t >= n)
        || input.field.low_terms.windows(2).any(|w| w[0] >= w[1])
    {
        return Err(Failure::Input);
    }
    let mut modulus = 1u64 << n;
    for t in &input.field.low_terms {
        modulus |= 1u64 << t;
    }
    let words = [
        word(input.b, n)?,
        word(input.target_x, n)?,
        word(input.codes[0], n)?,
        word(input.codes[1], n)?,
    ];
    if words[0] == 0 || words[2] >= words[3] {
        return Err(Failure::Input);
    }
    // New read-only accessor is supplied as a separate external patch hunk.
    if crate::binary_ecc::operation_tally::current_thread_active_or_borrowed() {
        return Err(Failure::TallyActive);
    }
    Ok((n, modulus, words))
}
fn construction_quote(n: u32, terms: usize) -> Quote {
    // n<=61: inline field values; mul/square use20word stack scratch.
    // Reduction folds<=n-1; each fold visits<=terms low coefficients.
    // 128 setup +32 per fold/term models software/hardware paths conservatively.
    let field = 128 + 32 * u64::from(n) * (terms as u64 + 1);
    Quote {
        work: 112 * field + 64 * 32 + 128 * u64::from(n) + 4096,
        bytes: 16384
            + u64::from(n)
                * (std::mem::size_of::<F2BoolPoly>() as u64
                    + 8 * std::mem::size_of::<F2BoolMono>() as u64),
    }
}
pub(super) fn build(
    input: &Input<'_>,
    admission: &mut impl OriginalAdmission,
) -> Result<Constructed, Failure> {
    let (n, modulus, w) = inspect(input, admission)?;
    admission.reserve(construction_quote(n, input.field.low_terms.len()))?;
    admission.check_deadline()?;
    let mut values = [0u64; 8];
    for (a, value) in values.iter_mut().enumerate() {
        admission.check_deadline()?;
        *value = word(
            &binary_semaev_s4(
                input.codes[a & 1],
                input.codes[(a >> 1) & 1],
                input.codes[(a >> 2) & 1],
                input.target_x,
                input.b,
                input.field,
            ),
            n,
        )?;
        admission.check_deadline()?;
    }
    let mut equations = Vec::new();
    equations
        .try_reserve_exact(n as usize)
        .map_err(|_| Failure::Allocation)?;
    for bit in 0..n {
        admission.check_deadline()?;
        let mut coefficients = [false; 8];
        for a in 0..8 {
            coefficients[a] = values[a] & (1u64 << bit) != 0;
        }
        for variable in 0..3 {
            for a in 0..8 {
                if a & (1 << variable) != 0 {
                    coefficients[a] ^= coefficients[a ^ (1 << variable)];
                }
            }
        }
        let mut terms = Vec::new();
        terms
            .try_reserve_exact(8)
            .map_err(|_| Failure::Allocation)?;
        for (a, set) in coefficients.into_iter().enumerate() {
            if set {
                terms.push(F2BoolMono::from_mask(a as Mask));
            }
        }
        terms.sort_unstable_by(|a, b| cmp_mono(*b, *a));
        equations.push(F2BoolPoly { terms, n_vars: 3 });
    }
    admission.check_deadline()?;
    Ok(Constructed {
        n,
        modulus,
        b: w[0],
        target_x: w[1],
        codes: [w[2], w[3]],
        values,
        equations,
    })
}
/// Independent bitwise polynomial product/reduction; never calls native mul,
/// square, resultant or its optimized quadratic-resultant formula.
fn reference_mul(a: u64, b: u64, n: u32, modulus: u64) -> u64 {
    let mut product = 0u128;
    for i in 0..n {
        if b & (1u64 << i) != 0 {
            product ^= (a as u128) << i;
        }
    }
    for i in (n..=(2 * n - 2)).rev() {
        if product & (1u128 << i) != 0 {
            product ^= (modulus as u128) << (i - n);
        }
    }
    product as u64
}
fn reference_coefficients(x: u64, y: u64, b: u64, n: u32, modulus: u64) -> [u64; 3] {
    let xy = reference_mul(x, y, n, modulus);
    [
        reference_mul(x ^ y, x ^ y, n, modulus),
        xy,
        reference_mul(xy, xy, n, modulus) ^ b,
    ]
}
fn reference_determinant(x: [u64; 4], b: u64, n: u32, modulus: u64) -> u64 {
    let [a, c, d] = reference_coefficients(x[0], x[1], b, n, modulus);
    let [e, f, g] = reference_coefficients(x[2], x[3], b, n, modulus);
    let matrix = [[a, c, d, 0], [0, a, c, d], [e, f, g, 0], [0, e, f, g]];
    let mut answer = 0;
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    if i == j || i == k || i == l || j == k || j == l || k == l {
                        continue;
                    }
                    let mut term = 1;
                    for (row, col) in [i, j, k, l].into_iter().enumerate() {
                        term = reference_mul(term, matrix[row][col], n, modulus);
                    }
                    answer ^= term;
                }
            }
        }
    }
    answer
}
pub(super) fn verify(
    input: &Input<'_>,
    built: &Constructed,
    admission: &mut impl OriginalAdmission,
) -> Result<(), Failure> {
    let (n, modulus, w) = inspect(input, admission)?;
    // Eight*(six coefficient products+24permutations*four products)=816
    // fixed reference multiplications, each<=2n bit-loop bodies. Plus256
    // permutation candidates/evaluation and bounded canonical/evaluation passes.
    admission.reserve(Quote {
        work: 816 * 32 * u64::from(n) + 8 * 256 * 16 + 256 * u64::from(n) + 4096,
        bytes: 8192,
    })?;
    admission.check_deadline()?;
    if (built.n, built.modulus, built.b, built.target_x, built.codes)
        != (n, modulus, w[0], w[1], [w[2], w[3]])
        || built.equations.len() != n as usize
    {
        return Err(Failure::Mismatch);
    }
    for p in &built.equations {
        if p.n_vars != 3
            || p.terms.len() > 8
            || p.terms.iter().any(|m| m.mask & !7 != 0)
            || !p.is_canonical()
        {
            return Err(Failure::Mismatch);
        }
    }
    for a in 0..8 {
        admission.check_deadline()?;
        let expected = reference_determinant(
            [
                w[2 + (a & 1)],
                w[2 + ((a >> 1) & 1)],
                w[2 + ((a >> 2) & 1)],
                w[1],
            ],
            w[0],
            n,
            modulus,
        );
        if built.values[a] != expected {
            return Err(Failure::Mismatch);
        }
        for (bit, p) in built.equations.iter().enumerate() {
            if u64::from(p.eval(a as Mask)) != (expected >> bit) & 1 {
                return Err(Failure::Mismatch);
            }
        }
        admission.check_deadline()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Admission {
        work: u64,
        bytes: u64,
        calls: usize,
        reject: Option<usize>,
        ticks: usize,
        expire: Option<usize>,
    }
    impl Admission {
        fn new() -> Self {
            Self {
                work: 0,
                bytes: 0,
                calls: 0,
                reject: None,
                ticks: 0,
                expire: None,
            }
        }
    }
    impl OriginalAdmission for Admission {
        fn reserve(&mut self, q: Quote) -> Result<(), Failure> {
            self.calls += 1;
            if self.reject == Some(self.calls) {
                return Err(Failure::Admission);
            }
            self.work += q.work;
            self.bytes += q.bytes;
            Ok(())
        }
        fn check_deadline(&mut self) -> Result<(), Failure> {
            self.ticks += 1;
            if self.expire.is_some_and(|at| self.ticks >= at) {
                Err(Failure::Deadline)
            } else {
                Ok(())
            }
        }
    }
    #[test]
    fn arbitrary_b_all_three_bit_assignments_match_independent_determinant() {
        for (n, low) in [(3, vec![0, 1]), (4, vec![0, 1]), (5, vec![0, 2])] {
            let field = IrreduciblePoly {
                degree: n,
                low_terms: low,
            };
            for b in [1, 2, 3] {
                for target in [0, 1, 2, 3, (1u64 << n) - 1] {
                    let b = F2mElement::from_words(&[b], n);
                    let target = F2mElement::from_words(&[target], n);
                    let x0 = F2mElement::from_words(&[0], n);
                    let x1 = F2mElement::from_words(&[3], n);
                    let input = Input {
                        field: &field,
                        b: &b,
                        target_x: &target,
                        codes: [&x0, &x1],
                    };
                    let mut admission = Admission::new();
                    let built = build(&input, &mut admission).unwrap();
                    assert_eq!(built.equations.len(), n as usize);
                    verify(&input, &built, &mut admission).unwrap();
                    assert_eq!(admission.calls, 4);
                    assert!(admission.work > 0 && admission.bytes > 0);
                }
            }
        }
    }
    #[test]
    fn payload_original_b_and_anf_mutations_are_rejected() {
        let field = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let b = F2mElement::from_words(&[2], 3);
        let other_b = F2mElement::from_words(&[3], 3);
        let target = F2mElement::from_words(&[5], 3);
        let zero = F2mElement::zero(3);
        let x = F2mElement::from_words(&[3], 3);
        let input = Input {
            field: &field,
            b: &b,
            target_x: &target,
            codes: [&zero, &x],
        };
        let mut a = Admission::new();
        let mut built = build(&input, &mut a).unwrap();
        built.values[7] ^= 1;
        assert_eq!(verify(&input, &built, &mut a), Err(Failure::Mismatch));
        built.values[7] ^= 1;
        // Keep canonical shape but change a genuine coefficient: replay must
        // reject semantic ANF drift, not merely an out-of-range mask.
        if let Some(i) = built.equations[0].terms.iter().position(|m| m.mask == 0) {
            built.equations[0].terms.remove(i);
        } else {
            built.equations[0].terms.push(F2BoolMono::from_mask(0));
            built.equations[0]
                .terms
                .sort_unstable_by(|a, b| cmp_mono(*b, *a));
        }
        assert!(built.equations[0].is_canonical());
        assert_eq!(verify(&input, &built, &mut a), Err(Failure::Mismatch));
        built.equations[0].terms.clear();
        built.equations[0].terms.push(F2BoolMono::from_mask(8));
        assert_eq!(verify(&input, &built, &mut a), Err(Failure::Mismatch));
        let built = build(&input, &mut a).unwrap();
        let wrong = Input {
            b: &other_b,
            ..input
        };
        assert_eq!(verify(&wrong, &built, &mut a), Err(Failure::Mismatch));
    }
    #[test]
    fn admission_deadline_and_active_tally_refuse_before_unpriced_field_work() {
        let field = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let b = F2mElement::from_words(&[2], 3);
        let zero = F2mElement::zero(3);
        let one = F2mElement::one(3);
        let input = Input {
            field: &field,
            b: &b,
            target_x: &one,
            codes: [&zero, &one],
        };
        let mut a = Admission::new();
        a.reject = Some(2);
        assert!(matches!(build(&input, &mut a), Err(Failure::Admission)));
        assert_eq!(a.work, 4096);
        assert_eq!(a.bytes, 0);
        let mut a = Admission::new();
        a.expire = Some(2);
        assert!(matches!(build(&input, &mut a), Err(Failure::Deadline)));
        assert!(a.work > 4096); // admission is cumulative, no refund on deadline
        let tally = crate::binary_ecc::operation_tally::OperationTally::begin();
        let mut a = Admission::new();
        assert!(matches!(build(&input, &mut a), Err(Failure::TallyActive)));
        assert!(tally.finish().is_empty());
    }
    #[test]
    fn malformed_modulus_and_selector_shapes_reject_before_arithmetic() {
        let field = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1, 1],
        };
        let b = F2mElement::from_words(&[2], 3);
        let zero = F2mElement::zero(3);
        let one = F2mElement::one(3);
        let input = Input {
            field: &field,
            b: &b,
            target_x: &one,
            codes: [&zero, &one],
        };
        assert!(matches!(
            build(&input, &mut Admission::new()),
            Err(Failure::Input)
        ));
        let field = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let input = Input {
            field: &field,
            codes: [&one, &one],
            ..input
        };
        assert!(matches!(
            build(&input, &mut Admission::new()),
            Err(Failure::Input)
        ));
    }
    #[test]
    fn n2_and_dense_n61_match_fixed_independent_quotient_reference() {
        // Constants independently generated by Python polynomial long division,
        // permutation determinant and subset-sum ANF. Dense n61 is a quotient
        // test only: it is NOT asserted to define an irreducible field/curve.
        for (n, low_terms, b, target, codes, expected_values, expected_anf) in [
            (
                2u32,
                vec![0, 1],
                2u64,
                3u64,
                [0, 1],
                [2, 1, 1, 2, 1, 2, 2, 2],
                vec![150, 151],
            ),
            (
                61u32,
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                ],
                1152921504606847029u64,
                576460752303423779u64,
                [4660, 1152921504606846985],
                [
                    990939322866816419,
                    80572196268344519,
                    80572196268344519,
                    2289954560958776008,
                    80572196268344519,
                    2289954560958776008,
                    2289954560958776008,
                    1821526970034409635,
                ],
                vec![
                    233, 233, 150, 232, 0, 127, 126, 1, 255, 232, 105, 0, 127, 0, 151, 104, 1, 232,
                    0, 126, 1, 151, 105, 104, 1, 0, 151, 151, 1, 0, 1, 128, 0, 232, 150, 126, 105,
                    22, 150, 126, 254, 105, 1, 254, 22, 150, 128, 23, 104, 254, 254, 150, 150, 0,
                    151, 23, 1, 232, 23, 151, 104,
                ],
            ),
        ] {
            let field = IrreduciblePoly {
                degree: n,
                low_terms,
            };
            let b = F2mElement::from_words(&[b], n);
            let target = F2mElement::from_words(&[target], n);
            let codes = [
                F2mElement::from_words(&[codes[0]], n),
                F2mElement::from_words(&[codes[1]], n),
            ];
            let input = Input {
                field: &field,
                b: &b,
                target_x: &target,
                codes: [&codes[0], &codes[1]],
            };
            let mut admission = Admission::new();
            let built = build(&input, &mut admission).unwrap();
            assert_eq!(built.values, expected_values);
            let actual: Vec<u8> = built
                .equations
                .iter()
                .map(|p| {
                    p.terms
                        .iter()
                        .fold(0u8, |acc, m| acc | (1u8 << (m.mask as u32)))
                })
                .collect();
            assert_eq!(actual, expected_anf);
            let before = (admission.work, admission.bytes);
            verify(&input, &built, &mut admission).unwrap();
            assert_eq!(
                admission.work - before.0,
                4096 + 816 * 32 * u64::from(n) + 8 * 256 * 16 + 256 * u64::from(n) + 4096
            );
            assert_eq!(admission.bytes - before.1, 8192);
        }
    }
    #[test]
    fn modulus_target_and_ordered_code_substitution_are_rejected() {
        let field = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let changed_field = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 2],
        };
        let b = F2mElement::from_words(&[2], 3);
        let target = F2mElement::from_words(&[5], 3);
        let other_target = F2mElement::from_words(&[6], 3);
        let zero = F2mElement::zero(3);
        let one = F2mElement::one(3);
        let code = F2mElement::from_words(&[3], 3);
        let input = Input {
            field: &field,
            b: &b,
            target_x: &target,
            codes: [&zero, &code],
        };
        let built = build(&input, &mut Admission::new()).unwrap();
        for changed in [
            Input {
                field: &changed_field,
                b: &b,
                target_x: &target,
                codes: [&zero, &code],
            },
            Input {
                field: &field,
                b: &b,
                target_x: &other_target,
                codes: [&zero, &code],
            },
            Input {
                field: &field,
                b: &b,
                target_x: &target,
                codes: [&one, &code],
            },
        ] {
            assert_eq!(
                verify(&changed, &built, &mut Admission::new()),
                Err(Failure::Mismatch)
            );
        }
    }
    #[test]
    fn verifier_refusals_keep_initial_and_later_charges_without_refund() {
        let field = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let b = F2mElement::from_words(&[2], 3);
        let zero = F2mElement::zero(3);
        let one = F2mElement::one(3);
        let input = Input {
            field: &field,
            b: &b,
            target_x: &one,
            codes: [&zero, &one],
        };
        let built = build(&input, &mut Admission::new()).unwrap();
        // Fresh admission fixtures isolate verifier gates; this is not a
        // production retry or renewed original execution budget.
        for reject in [1, 2] {
            let mut a = Admission::new();
            a.reject = Some(reject);
            assert_eq!(verify(&input, &built, &mut a), Err(Failure::Admission));
            assert_eq!(a.work, if reject == 1 { 0 } else { 4096 });
            assert_eq!(a.bytes, 0);
        }
        for expire in [1, 2, 3, 4] {
            let mut a = Admission::new();
            a.expire = Some(expire);
            assert_eq!(verify(&input, &built, &mut a), Err(Failure::Deadline));
            // 1: after metadata; 2: after full admission; 3: before first
            // determinant; 4: after first determinant and ANF verification.
            assert_eq!(a.work, if expire == 1 { 4096 } else { 120064 });
            assert_eq!(a.bytes, if expire == 1 { 0 } else { 8192 });
        }
    }
}
