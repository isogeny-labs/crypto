//! Standalone arithmetic-only checker. This module authenticates NO curve,
//! source equations, manifest, variable map, capture watermark, or native row.
//! A separate authenticated-system adapter must compare these exact ordered
//! generators and variable identities before exporting any training record.
//!
//! Canonical wire: ASCII `BQI1` followed by single-space-separated unsigned
//! decimal tokens and exactly one final newline. No leading zeros (except 0).
//! Tokens: N, generator_count, each generator polynomial, target polynomial,
//! proof_count, each proof's generator_index then multiplier monomial.
//! Polynomial: term_count then monomials in strict lexicographic order.
//! Monomial: variable_count then strictly increasing indices in [0,N).
//! Proof: strict (generator_index, multiplier lexicographic) order.
//! BQI1 fixes F2 and quotient vi^2+vi; all coefficients are implicitly one.
//! Empty polynomial is zero; empty monomial is one. Duplicate input terms are
//! malformed; only internally generated repeated products cancel by parity.
use std::{cmp::Ordering, mem::size_of, time::Instant};

type Mono = Vec<u32>;
type Poly = Vec<Mono>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Resource {
    EncodedBytes,
    Validation,
    TermVisits,
    UnionSteps,
    ParitySteps,
    SparseEntries,
    ProofRows,
    AllocationBytes,
    DeadlineProbes,
}
const RESOURCES: usize = 9;
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub variables: u32,
    pub generators: u32,
    pub polynomial_terms: u32,
    pub proof_terms: u32,
    pub accumulator_entries: usize,
    pub caps: [u64; RESOURCES],
}
impl Limits {
    /// Frozen proposed software limits; no exponential-in-N allocation.
    pub const fn software() -> Self {
        Self {
            variables: 4096,
            generators: 4096,
            polynomial_terms: 16384,
            proof_terms: 4096,
            accumulator_entries: 16384,
            caps: [
                4 * 1024 * 1024,
                20_000_000,
                2_000_000,
                20_000_000,
                20_000_000,
                2_000_000,
                8192,
                64 * 1024 * 1024,
                80_000_000,
            ],
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Charges {
    pub used: [u64; RESOURCES],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syntax {
    Header,
    Integer,
    Separator,
    Trailing,
    Variable,
    CanonicalMonomial,
    CanonicalPolynomial,
    CanonicalProof,
    Generator,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exhaustion {
    Resource(Resource),
    Variables,
    Generators,
    PolynomialTerms,
    ProofTerms,
    AccumulatorEntries,
    ArithmeticOverflow,
    Allocation,
    Deadline,
    /// Original-owner admission refused this exact request. Its ancestor may
    /// have reserved before a later postcheck failed; consult that owner ledger.
    Admission { resource: Resource, amount: u64 },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Malformed(Syntax),
    EqualityMismatch,
    SourceMismatch,
    Unknown(Exhaustion),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Degrees {
    pub target: Option<usize>,
    pub max_quotient_summand: Option<usize>,
    pub max_before_quotient_summand: Option<usize>,
}
#[derive(Clone, Copy, Debug)]
pub struct Audit {
    pub result: Result<Degrees, Failure>,
    pub charges: Charges,
    /// Always false. Arithmetic equality is not original-system admission.
    pub source_authenticated: bool,
    pub native_operation_witness: bool,
}
struct Meter<'a> {
    limits: Limits,
    charges: Charges,
    deadline: Instant,
    admission: Option<&'a mut dyn FnMut(Resource, u64) -> Result<(), ()>>,
}
impl Meter<'_> {
    fn charge(&mut self, resource: Resource, amount: u64) -> Result<(), Failure> {
        let probe = Resource::DeadlineProbes as usize;
        let next = self.charges.used[probe]
            .checked_add(1)
            .ok_or(Failure::Unknown(Exhaustion::ArithmeticOverflow))?;
        if next > self.limits.caps[probe] {
            return Err(Failure::Unknown(Exhaustion::Resource(
                Resource::DeadlineProbes,
            )));
        }
        self.admit(Resource::DeadlineProbes, 1)?;
        self.charges.used[probe] = next;
        if Instant::now() >= self.deadline {
            return Err(Failure::Unknown(Exhaustion::Deadline));
        }
        let i = resource as usize;
        let next = self.charges.used[i]
            .checked_add(amount)
            .ok_or(Failure::Unknown(Exhaustion::ArithmeticOverflow))?;
        if next > self.limits.caps[i] {
            return Err(Failure::Unknown(Exhaustion::Resource(resource)));
        }
        self.admit(resource, amount)?;
        self.charges.used[i] = next;
        Ok(())
    }
    fn admit(&mut self, resource: Resource, amount: u64) -> Result<(), Failure> {
        if let Some(admission) = self.admission.as_mut() {
            admission(resource, amount).map_err(|()|
                Failure::Unknown(Exhaustion::Admission { resource, amount }))?;
        }
        Ok(())
    }
    fn vector<T>(&mut self, count: usize) -> Result<Vec<T>, Failure> {
        let bytes = count
            .checked_mul(size_of::<T>())
            .ok_or(Failure::Unknown(Exhaustion::ArithmeticOverflow))?;
        self.charge(
            Resource::AllocationBytes,
            u64::try_from(bytes).map_err(|_| Failure::Unknown(Exhaustion::ArithmeticOverflow))?,
        )?;
        let mut result = Vec::new();
        result
            .try_reserve_exact(count)
            .map_err(|_| Failure::Unknown(Exhaustion::Allocation))?;
        Ok(result)
    }
}
fn cmp(a: &[u32], b: &[u32], meter: &mut Meter) -> Result<Ordering, Failure> {
    for (&x, &y) in a.iter().zip(b) {
        meter.charge(Resource::Validation, 1)?;
        if x != y {
            return Ok(x.cmp(&y));
        }
    }
    meter.charge(Resource::Validation, 1)?;
    Ok(a.len().cmp(&b.len()))
}
struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}
impl Parser<'_> {
    fn integer(&mut self, meter: &mut Meter) -> Result<u32, Failure> {
        meter.charge(Resource::Validation, 1)?;
        if self.bytes.get(self.at) != Some(&b' ') {
            return Err(Failure::Malformed(Syntax::Separator));
        }
        self.at += 1;
        let start = self.at;
        let mut value = 0u32;
        while let Some(&byte) = self.bytes.get(self.at) {
            meter.charge(Resource::Validation, 1)?;
            if !byte.is_ascii_digit() {
                break;
            }
            if self.at > start && self.bytes[start] == b'0' {
                return Err(Failure::Malformed(Syntax::Integer));
            }
            value = value
                .checked_mul(10)
                .and_then(|n| n.checked_add(u32::from(byte - b'0')))
                .ok_or(Failure::Malformed(Syntax::Integer))?;
            self.at += 1;
        }
        if self.at == start {
            return Err(Failure::Malformed(Syntax::Integer));
        }
        Ok(value)
    }
    fn mono(&mut self, n: u32, meter: &mut Meter) -> Result<Mono, Failure> {
        let count = self.integer(meter)?;
        if count > n {
            return Err(Failure::Malformed(Syntax::CanonicalMonomial));
        }
        let mut result = meter.vector(count as usize)?;
        for _ in 0..count {
            meter.charge(Resource::Validation, 1)?;
            let value = self.integer(meter)?;
            if value >= n {
                return Err(Failure::Malformed(Syntax::Variable));
            }
            if result.last().is_some_and(|old| *old >= value) {
                return Err(Failure::Malformed(Syntax::CanonicalMonomial));
            }
            result.push(value);
        }
        Ok(result)
    }
    fn poly(&mut self, n: u32, meter: &mut Meter) -> Result<Poly, Failure> {
        let count = self.integer(meter)?;
        if count > meter.limits.polynomial_terms {
            return Err(Failure::Unknown(Exhaustion::PolynomialTerms));
        }
        let mut result: Poly = meter.vector(count as usize)?;
        for _ in 0..count {
            meter.charge(Resource::TermVisits, 1)?;
            let mono = self.mono(n, meter)?;
            if let Some(previous) = result.last() {
                if cmp(previous, &mono, meter)? != Ordering::Less {
                    return Err(Failure::Malformed(Syntax::CanonicalPolynomial));
                }
            }
            result.push(mono);
        }
        Ok(result)
    }
}
fn union(a: &[u32], b: &[u32], meter: &mut Meter) -> Result<Mono, Failure> {
    let capacity = a
        .len()
        .checked_add(b.len())
        .ok_or(Failure::Unknown(Exhaustion::ArithmeticOverflow))?;
    let mut result = meter.vector(capacity)?;
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        meter.charge(Resource::UnionSteps, 1)?;
        let value = if j == b.len() || i < a.len() && a[i] < b[j] {
            let v = a[i];
            i += 1;
            v
        } else if i == a.len() || b[j] < a[i] {
            let v = b[j];
            j += 1;
            v
        } else {
            let v = a[i];
            i += 1;
            j += 1;
            v
        };
        result.push(value);
    }
    Ok(result)
}
fn toggle(accumulator: &mut Poly, product: Mono, meter: &mut Meter) -> Result<(), Failure> {
    let (mut low, mut high) = (0, accumulator.len());
    while low < high {
        meter.charge(Resource::ParitySteps, 1)?;
        let mid = low + (high - low) / 2;
        match cmp(&accumulator[mid], &product, meter)? {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
            Ordering::Equal => {
                meter.charge(Resource::ParitySteps, (accumulator.len() - mid) as u64)?;
                accumulator.remove(mid);
                return Ok(());
            }
        }
    }
    if accumulator.len() >= meter.limits.accumulator_entries {
        return Err(Failure::Unknown(Exhaustion::AccumulatorEntries));
    }
    meter.charge(Resource::SparseEntries, 1)?;
    meter.charge(Resource::ParitySteps, (accumulator.len() - low + 1) as u64)?;
    // All outer slots reserved before the first product; insert cannot grow.
    accumulator.insert(low, product);
    Ok(())
}
struct ProofTerm {
    generator: usize,
    multiplier: Mono,
}
fn arithmetic(bytes: &[u8], expected: Option<(u32, &[Poly])>, meter: &mut Meter) -> Result<Degrees, Failure> {
    meter.charge(Resource::EncodedBytes, bytes.len() as u64)?;
    meter.charge(Resource::Validation, 5)?;
    if !bytes.starts_with(b"BQI1 ") {
        return Err(Failure::Malformed(Syntax::Header));
    }
    let mut parser = Parser { bytes, at: 4 };
    let n = parser.integer(meter)?;
    if n > meter.limits.variables {
        return Err(Failure::Unknown(Exhaustion::Variables));
    }
    let count = parser.integer(meter)?;
    if count > meter.limits.generators {
        return Err(Failure::Unknown(Exhaustion::Generators));
    }
    let mut generators: Vec<Poly> = meter.vector(count as usize)?;
    for _ in 0..count {
        meter.charge(Resource::Validation, 1)?;
        generators.push(parser.poly(n, meter)?);
    }
    if let Some((variables, equations)) = expected {
        meter.charge(Resource::Validation, 1)?;
        if n != variables || generators.len() != equations.len() {
            return Err(Failure::SourceMismatch);
        }
        for (actual, expected) in generators.iter().zip(equations) {
            meter.charge(Resource::Validation, 1)?;
            if actual.len() != expected.len() { return Err(Failure::SourceMismatch); }
            for (actual, expected) in actual.iter().zip(expected) {
                if cmp(actual, expected, meter)? != Ordering::Equal {
                    return Err(Failure::SourceMismatch);
                }
            }
        }
    }
    let target = parser.poly(n, meter)?;
    let count = parser.integer(meter)?;
    if count > meter.limits.proof_terms {
        return Err(Failure::Unknown(Exhaustion::ProofTerms));
    }
    let mut proof: Vec<ProofTerm> = meter.vector(count as usize)?;
    for _ in 0..count {
        meter.charge(Resource::ProofRows, 1)?;
        let generator = parser.integer(meter)? as usize;
        if generator >= generators.len() {
            return Err(Failure::Malformed(Syntax::Generator));
        }
        let multiplier = parser.mono(n, meter)?;
        if expected.is_some() && !multiplier.is_empty() { return Err(Failure::SourceMismatch); }
        if let Some(previous) = proof.last() {
            meter.charge(Resource::Validation, 1)?;
            if generator < previous.generator
                || generator == previous.generator
                    && cmp(&previous.multiplier, &multiplier, meter)? != Ordering::Less
            {
                return Err(Failure::Malformed(Syntax::CanonicalProof));
            }
        }
        proof.push(ProofTerm {
            generator,
            multiplier,
        });
    }
    meter.charge(Resource::Validation, 1)?;
    if bytes.get(parser.at..) != Some(b"\n".as_slice()) {
        return Err(Failure::Malformed(Syntax::Trailing));
    }
    let mut accumulator: Poly = meter.vector(meter.limits.accumulator_entries)?;
    let mut degrees = Degrees {
        target: None,
        max_quotient_summand: None,
        max_before_quotient_summand: None,
    };
    for term in &target {
        meter.charge(Resource::TermVisits, 1)?;
        degrees.target = Some(degrees.target.unwrap_or(0).max(term.len()));
    }
    let mut summand: Poly = meter.vector(meter.limits.accumulator_entries)?;
    for term in &proof {
        meter.charge(Resource::ProofRows, 1)?;
        for source in &generators[term.generator] {
            meter.charge(Resource::TermVisits, 1)?;
            let degree = source
                .len()
                .checked_add(term.multiplier.len())
                .ok_or(Failure::Unknown(Exhaustion::ArithmeticOverflow))?;
            degrees.max_before_quotient_summand =
                Some(degrees.max_before_quotient_summand.unwrap_or(0).max(degree));
            let product = union(source, &term.multiplier, meter)?;
            toggle(&mut summand, product, meter)?;
        }
        // Degrees refer to the fully normalized generator-multiple polynomial,
        // after repeated union products within this summand have cancelled.
        for product in summand.drain(..) {
            meter.charge(Resource::TermVisits, 1)?;
            degrees.max_quotient_summand =
                Some(degrees.max_quotient_summand.unwrap_or(0).max(product.len()));
            toggle(&mut accumulator, product, meter)?;
        }
    }
    meter.charge(Resource::Validation, 1)?;
    if accumulator.len() != target.len() {
        return Err(Failure::EqualityMismatch);
    }
    for (a, b) in accumulator.iter().zip(&target) {
        if cmp(a, b, meter)? != Ordering::Equal {
            return Err(Failure::EqualityMismatch);
        }
    }
    Ok(degrees)
}
/// Deadline belongs to caller's original operation; this function never renews
/// it. All failures retain accepted charges. Limit refusals are UNKNOWN, never
/// nonmembership. Resource counters are modeled work/requested allocation,
/// not hardware instruction counts or exact allocator consumption.
pub fn verify_untrusted(bytes: &[u8], limits: Limits, deadline: Instant) -> Audit {
    let mut meter = Meter {
        limits,
        charges: Charges::default(),
        deadline,
        admission: None,
    };
    let result = arithmetic(bytes, None, &mut meter);
    Audit {
        result,
        charges: meter.charges,
        source_authenticated: false,
        native_operation_witness: false,
    }
}

/// Incremental caller-owned admission before each accepted resource charge and
/// before the corresponding computation/allocation. Local caps and the original
/// absolute deadline remain unchanged. Callback contract: every successful
/// return must enforce that SAME original deadline after its reservation and
/// postchecks. This function does not impose a second unmetered clock probe
/// around an arbitrary callback; the native bridge satisfies this contract via
/// Meter::pay. Successful admissions contribute to the
/// same nine counters as verify_untrusted. A refusal retains the exact request
/// in Unknown(Admission); an ancestor may already have reserved before returning
/// a postcheck failure, so this is not a claim that the parent spent zero.
/// The caller must retain its original ledger, including that refused request.
pub fn verify_untrusted_admitted(
    bytes: &[u8], limits: Limits, deadline: Instant,
    admission: &mut impl FnMut(Resource, u64) -> Result<(), ()>,
) -> Audit {
    let mut meter = Meter { limits, charges: Charges::default(), deadline,
        admission: Some(admission) };
    let result = arithmetic(bytes, None, &mut meter);
    Audit { result, charges: meter.charges, source_authenticated: false,
        native_operation_witness: false }
}

/// Private to the checked-map parent module. Expected source is supplied only
/// after checked-map reconstruction; comparison itself does not mint authority.
pub(super) fn verify_expected(bytes: &[u8], variables: u32, equations: &[Poly], limits: Limits, deadline: Instant) -> Audit {
    let mut meter = Meter { limits, charges: Charges::default(), deadline, admission: None };
    let result = arithmetic(bytes, Some((variables, equations)), &mut meter);
    Audit { result, charges: meter.charges, source_authenticated: false, native_operation_witness: false }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    fn wire(n: u32, g: &[Poly], target: &Poly, proof: &[(u32, Mono)]) -> Vec<u8> {
        fn mono(out: &mut String, m: &Mono) {
            out.push_str(&format!(" {}", m.len()));
            for x in m {
                out.push_str(&format!(" {x}"));
            }
        }
        fn poly(out: &mut String, p: &Poly) {
            out.push_str(&format!(" {}", p.len()));
            for m in p {
                mono(out, m);
            }
        }
        let mut out = format!("BQI1 {n} {}", g.len());
        for p in g {
            poly(&mut out, p);
        }
        poly(&mut out, target);
        out.push_str(&format!(" {}", proof.len()));
        for (i, m) in proof {
            out.push_str(&format!(" {i}"));
            mono(&mut out, m);
        }
        out.push('\n');
        out.into_bytes()
    }
    fn check(bytes: &[u8]) -> Audit {
        verify_untrusted(
            bytes,
            Limits::software(),
            Instant::now() + Duration::from_secs(10),
        )
    }
    fn sample() -> Vec<u8> {
        wire(
            2,
            &[vec![vec![0], vec![1]], vec![vec![], vec![1]]],
            &vec![vec![], vec![0]],
            &[(0, vec![]), (1, vec![])],
        )
    }
    #[test]
    fn cross_generator_cancellation_and_restrictions() {
        let a = check(&sample());
        assert_eq!(
            a.result,
            Ok(Degrees {
                target: Some(1),
                max_quotient_summand: Some(1),
                max_before_quotient_summand: Some(1)
            })
        );
        assert!(!a.source_authenticated && !a.native_operation_witness);
        let bad = wire(
            2,
            &[vec![vec![0], vec![1]], vec![vec![], vec![1]]],
            &vec![vec![0]],
            &[(0, vec![]), (1, vec![])],
        );
        assert_eq!(check(&bad).result, Err(Failure::EqualityMismatch));
    }
    #[test]
    fn overlap_and_within_summand_cancellation() {
        let a = check(&wire(1, &[vec![vec![0]]], &vec![vec![0]], &[(0, vec![0])]));
        assert_eq!(
            a.result,
            Ok(Degrees {
                target: Some(1),
                max_quotient_summand: Some(1),
                max_before_quotient_summand: Some(2)
            })
        );
        let a = check(&wire(
            2,
            &[vec![vec![0], vec![0, 1]]],
            &vec![],
            &[(0, vec![1])],
        ));
        assert_eq!(
            a.result,
            Ok(Degrees {
                target: None,
                max_quotient_summand: None,
                max_before_quotient_summand: Some(3)
            })
        );
    }
    #[test]
    fn constants_zeros_and_more_than_word_width() {
        assert!(check(&wire(0, &[], &vec![], &[])).result.is_ok());
        assert!(check(&wire(
            0,
            &[vec![vec![]], vec![]],
            &vec![vec![]],
            &[(0, vec![]), (1, vec![])]
        ))
        .result
        .is_ok());
        assert!(check(&wire(
            130,
            &[vec![vec![0, 64, 129]]],
            &vec![vec![0, 64, 129]],
            &[(0, vec![64])]
        ))
        .result
        .is_ok());
    }
    #[test]
    fn malformed_canonical_inputs_reject() {
        let cases = [
            wire(2, &[vec![vec![1, 0]]], &vec![], &[]),
            wire(2, &[vec![vec![0, 0]]], &vec![], &[]),
            wire(2, &[vec![vec![2]]], &vec![], &[]),
            wire(2, &[vec![vec![0], vec![0]]], &vec![], &[]),
            wire(2, &[vec![vec![1], vec![0]]], &vec![], &[]),
            wire(
                2,
                &[vec![vec![0]]],
                &vec![vec![0]],
                &[(0, vec![]), (0, vec![])],
            ),
            wire(2, &[vec![vec![0]]], &vec![vec![0]], &[(1, vec![])]),
            b"BQI2 0 0 0 0\n".to_vec(),
            b"BQI1 00 0 0 0\n".to_vec(),
            b"BQI1 +0 0 0 0\n".to_vec(),
            b"BQI1 0 0 0 0 \n".to_vec(),
            b"BQI1 0 0 0 0\nextra".to_vec(),
            b"BQI1 4294967296 0 0 0\n".to_vec(),
        ];
        for bytes in cases {
            assert!(
                matches!(check(&bytes).result, Err(Failure::Malformed(_))),
                "{bytes:?}"
            );
        }
    }
    #[test]
    fn ordered_generator_and_multiplier_mutations_mismatch() {
        let target = vec![vec![0]];
        let generators = vec![vec![vec![0]], vec![vec![1]]];
        assert!(check(&wire(2, &generators, &target, &[(0, vec![])]))
            .result
            .is_ok());
        assert_eq!(
            check(&wire(
                2,
                &[generators[1].clone(), generators[0].clone()],
                &target,
                &[(0, vec![])]
            ))
            .result,
            Err(Failure::EqualityMismatch)
        );
        assert_eq!(
            check(&wire(2, &generators, &target, &[(0, vec![1])])).result,
            Err(Failure::EqualityMismatch)
        );
    }
    #[test]
    fn every_counter_exact_and_one_under_retains_prefix() {
        let bytes = sample();
        let accepted = check(&bytes);
        assert!(accepted.result.is_ok());
        let mut exact = Limits::software();
        exact.caps = accepted.charges.used;
        assert!(
            verify_untrusted(&bytes, exact, Instant::now() + Duration::from_secs(10))
                .result
                .is_ok()
        );
        for i in 0..RESOURCES {
            assert!(accepted.charges.used[i] > 0);
            let mut limits = Limits::software();
            limits.caps[i] = accepted.charges.used[i] - 1;
            let refused =
                verify_untrusted(&bytes, limits, Instant::now() + Duration::from_secs(10));
            assert!(matches!(refused.result, Err(Failure::Unknown(_))));
            assert!(refused.charges.used[i] <= limits.caps[i]);
            assert!(refused.charges.used[Resource::DeadlineProbes as usize] > 0);
        }
    }
    #[test]
    fn dimensions_live_entries_and_expired_deadline_are_unknown() {
        let bytes = sample();
        let mut limits = Limits::software();
        limits.variables = 1;
        assert_eq!(
            verify_untrusted(&bytes, limits, Instant::now() + Duration::from_secs(10)).result,
            Err(Failure::Unknown(Exhaustion::Variables))
        );
        let mut limits = Limits::software();
        limits.accumulator_entries = 0;
        assert_eq!(
            verify_untrusted(&bytes, limits, Instant::now() + Duration::from_secs(10)).result,
            Err(Failure::Unknown(Exhaustion::AccumulatorEntries))
        );
        let a = verify_untrusted(&bytes, Limits::software(), Instant::now());
        assert_eq!(a.result, Err(Failure::Unknown(Exhaustion::Deadline)));
        assert_eq!(a.charges.used[Resource::DeadlineProbes as usize], 1);
    }
    #[test]
    fn count_caps_and_allocation_overflow_are_unknown() {
        let bytes = sample();
        for (limits, expected) in [
            (
                Limits {
                    generators: 0,
                    ..Limits::software()
                },
                Exhaustion::Generators,
            ),
            (
                Limits {
                    polynomial_terms: 0,
                    ..Limits::software()
                },
                Exhaustion::PolynomialTerms,
            ),
            (
                Limits {
                    proof_terms: 0,
                    ..Limits::software()
                },
                Exhaustion::ProofTerms,
            ),
            (
                Limits {
                    accumulator_entries: usize::MAX,
                    ..Limits::software()
                },
                Exhaustion::ArithmeticOverflow,
            ),
        ] {
            let audit = verify_untrusted(&bytes, limits, Instant::now() + Duration::from_secs(10));
            assert_eq!(audit.result, Err(Failure::Unknown(expected)));
            assert!(audit.charges.used[Resource::EncodedBytes as usize] > 0);
        }
    }
    // Independent test oracle: evaluate truth tables with bit tests. Production
    // arithmetic above uses sorted variable sets, never these truth-table masks.
    fn eval(p: &Poly, assignment: u32) -> bool {
        p.iter().fold(false, |v, m| {
            v ^ m.iter().all(|i| assignment & (1u32 << i) != 0)
        })
    }
    #[test]
    fn small_truth_tables_reference_all_two_variable_polynomials() {
        let monomials = [vec![], vec![0], vec![0, 1], vec![1]];
        for selector in 0..16u32 {
            let generator: Poly = monomials
                .iter()
                .enumerate()
                .filter(|(i, _)| selector & (1u32 << i) != 0)
                .map(|(_, m)| m.clone())
                .collect();
            for multiplier in &monomials {
                // Derive reference target ANF by Boolean Mobius transform of
                // evaluation values, independent of production union/parity.
                let mut table = [false; 4];
                for a in 0..4 {
                    table[a] =
                        eval(&generator, a as u32) && eval(&vec![multiplier.clone()], a as u32);
                }
                for bit in [1usize, 2] {
                    for a in 0..4 {
                        if a & bit != 0 {
                            table[a] ^= table[a ^ bit];
                        }
                    }
                }
                let mut target = Vec::new();
                for (mask, coefficient) in table.iter().enumerate() {
                    if *coefficient {
                        target.push(
                            (0..2u32)
                                .filter(|bit| mask & (1usize << bit) != 0)
                                .collect::<Mono>(),
                        );
                    }
                }
                target.sort();
                assert!(check(&wire(
                    2,
                    &[generator.clone()],
                    &target,
                    &[(0, multiplier.clone())]
                ))
                .result
                .is_ok());
                let mut mutated = target.clone();
                if mutated.first() == Some(&vec![]) {
                    mutated.remove(0);
                } else {
                    mutated.insert(0, vec![]);
                }
                assert_eq!(
                    check(&wire(
                        2,
                        &[generator.clone()],
                        &mutated,
                        &[(0, multiplier.clone())]
                    ))
                    .result,
                    Err(Failure::EqualityMismatch)
                );
            }
        }
    }
    #[test]
    fn expected_source_rejects_valid_foreign_and_reordered_generator_identities(){
        let generators=vec![vec![vec![0]],vec![vec![1]]];
        let bytes=wire(2,&generators,&generators[0],&[(0,vec![])]);
        let deadline=Instant::now()+Duration::from_secs(10);
        assert!(verify_untrusted(&bytes,Limits::software(),deadline).result.is_ok());
        assert!(verify_expected(&bytes,2,&generators,Limits::software(),deadline).result.is_ok());
        let reordered=vec![generators[1].clone(),generators[0].clone()];
        assert_eq!(verify_expected(&bytes,2,&reordered,Limits::software(),deadline).result,Err(Failure::SourceMismatch));
        assert_eq!(verify_expected(&bytes,3,&generators,Limits::software(),deadline).result,Err(Failure::SourceMismatch));
        let foreign=vec![vec![vec![]],vec![vec![1]]];
        let foreign_bytes=wire(2,&foreign,&foreign[0],&[(0,vec![])]);
        assert!(verify_untrusted(&foreign_bytes,Limits::software(),deadline).result.is_ok());
        assert_eq!(verify_expected(&foreign_bytes,2,&generators,Limits::software(),deadline).result,Err(Failure::SourceMismatch));
        let multiplied=wire(2,&generators,&vec![vec![0,1]],&[(0,vec![1])]);
        assert!(verify_untrusted(&multiplied,Limits::software(),deadline).result.is_ok());
        assert_eq!(verify_expected(&multiplied,2,&generators,Limits::software(),deadline).result,Err(Failure::SourceMismatch));
    }

}
