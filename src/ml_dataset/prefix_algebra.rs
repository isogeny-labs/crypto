//! Bounded ideal-containment certificates, independent of the native F4 engine.
//! A search miss is Unknown. Only explicit polynomial identities certify a result.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

pub(super) type Poly = Vec<(Vec<u32>, u64)>;
type Row = BTreeMap<Vec<u32>, u64>;
type Combination = BTreeMap<(usize, Vec<u32>), u64>;

#[derive(Clone, Copy, Debug, Serialize)]
pub(super) struct Limits {
    pub degree: u32,
    pub rows: usize,
    pub entries: usize,
    pub operations: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            degree: 8,
            rows: 8192,
            entries: 262_144,
            operations: 10_000_000,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub(super) struct Work {
    pub generated_rows: usize,
    pub coefficient_operations: u64,
    pub peak_retained_entries: usize,
}

#[derive(Clone, Debug, Serialize)]
pub(super) struct Multiple {
    pub generator: usize,
    pub exponents: Vec<u32>,
    // Field values stay exact in JSON consumers with IEEE754 numbers.
    pub coefficient: String,
}

#[derive(Clone, Debug, Serialize)]
pub(super) struct Certificate {
    pub left_from_right: Vec<Vec<Multiple>>,
    pub right_from_left: Vec<Vec<Multiple>>,
}

#[derive(Debug, Serialize)]
pub(super) struct Audit {
    pub schema: &'static str,
    pub status: &'static str,
    pub reason: Option<String>,
    pub limits: Limits,
    pub work: Work,
    pub certificate: Option<Certificate>,
    pub native_execution_trace_verified: bool,
    pub native_instruction_count_verified: bool,
    pub exact_fall_degree_verified: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum IdentityCapKind {
    PolynomialRows,
    PolynomialTerms,
    PolynomialDegree,
    ProofEntries,
    MultiplierDegree,
    CoefficientOperations,
    SparseEntries,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub(super) enum IdentityStatus {
    Verified {
        result_degree: Option<u32>,
        max_generator_multiple_degree: Option<u32>,
        witnessed_degree_drop: bool,
    },
    UnknownCap {
        kind: IdentityCapKind,
    },
}

#[derive(Debug, Serialize)]
pub(super) struct IdentityAudit {
    pub schema: &'static str,
    pub status: IdentityStatus,
    pub limits: IdentityLimits,
    pub work: Work,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub(super) struct IdentityLimits {
    pub generator_rows: usize,
    pub total_polynomial_rows: usize,
    pub polynomial_terms: usize,
    pub polynomial_degree: u32,
    pub multiplier_degree: u32,
    pub generator_multiple_degree: u32,
    pub entries: usize,
    pub operations: u64,
}

struct Ring {
    p: u64,
    n: usize,
    limits: Limits,
    work: Work,
    // Set at the failure site; direct identity callers never classify error text.
    cap: Option<IdentityCapKind>,
}

impl Ring {
    fn charge(&mut self) -> Result<(), String> {
        if self.work.coefficient_operations >= self.limits.operations {
            self.cap = Some(IdentityCapKind::CoefficientOperations);
            return Err("coefficient operation cap".into());
        }
        self.work.coefficient_operations += 1;
        Ok(())
    }
    fn mul(&mut self, a: u64, b: u64) -> Result<u64, String> {
        self.charge()?;
        Ok(((u128::from(a) * u128::from(b)) % u128::from(self.p)) as u64)
    }
    fn add(&mut self, a: u64, b: u64) -> Result<u64, String> {
        self.charge()?;
        Ok(((u128::from(a) + u128::from(b)) % u128::from(self.p)) as u64)
    }
    fn inverse(&mut self, a: u64) -> Result<u64, String> {
        let mut e = self.p - 2;
        let mut b = a;
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b)?;
            }
            e >>= 1;
            if e > 0 {
                b = self.mul(b, b)?;
            }
        }
        Ok(r)
    }
    fn entries(&mut self, n: usize) -> Result<(), String> {
        self.work.peak_retained_entries = self.work.peak_retained_entries.max(n);
        if n > self.limits.entries {
            self.cap = Some(IdentityCapKind::SparseEntries);
            Err("retained sparse entry cap".into())
        } else {
            Ok(())
        }
    }
    fn accumulate<K: Ord + Clone>(
        &mut self,
        row: &mut BTreeMap<K, u64>,
        key: K,
        value: u64,
    ) -> Result<(), String> {
        let next = self.add(row.get(&key).copied().unwrap_or(0), value)?;
        if next == 0 {
            row.remove(&key);
        } else {
            row.insert(key, next);
        }
        Ok(())
    }
    fn axpy<K: Ord + Clone>(
        &mut self,
        row: &mut BTreeMap<K, u64>,
        other: &BTreeMap<K, u64>,
        c: u64,
    ) -> Result<(), String> {
        for (key, value) in other {
            let product = self.mul(c, *value)?;
            self.accumulate(row, key.clone(), product)?;
            // Bound each transient row as well as the complete pivot table.
            self.entries(row.len())?;
        }
        Ok(())
    }
}

fn degree(e: &[u32]) -> Result<u32, String> {
    e.iter()
        .try_fold(0u32, |a, b| a.checked_add(*b))
        .ok_or("exponent degree overflow".into())
}

fn validate(polys: &[Poly], n: usize, p: u64) -> Result<(), String> {
    let mut terms = 0usize;
    if polys.len() > 4096 {
        return Err("input polynomial count exceeds4096".into());
    }
    for poly in polys {
        let mut seen = BTreeSet::new();
        for (e, c) in poly {
            terms += 1;
            if terms > 16_384 {
                return Err("input term cap".into());
            }
            if e.len() != n || degree(e)? > 64 || *c == 0 || *c >= p || !seen.insert(e) {
                return Err(
                    "invalid polynomial dimension, degree, coefficient or duplicate term".into(),
                );
            }
        }
    }
    Ok(())
}

fn shift(e: &[u32], multiplier: &[u32]) -> Vec<u32> {
    // Inputs are validated at degree<=64 and multipliers at degree<=12.
    e.iter().zip(multiplier).map(|(a, b)| a + b).collect()
}

fn encode(combination: Combination) -> Vec<Multiple> {
    combination
        .into_iter()
        .map(|((generator, exponents), c)| Multiple {
            generator,
            exponents,
            coefficient: c.to_string(),
        })
        .collect()
}

struct Pivot {
    row: Row,
    combination: Combination,
}

/// Enumerate exact-degree monomials lazily; no combinatorial array allocation.
fn monomials(
    e: &mut [u32],
    position: usize,
    remaining: u32,
    callback: &mut impl FnMut(&[u32]) -> Result<(), String>,
) -> Result<(), String> {
    if position == e.len() {
        return if remaining == 0 { callback(e) } else { Ok(()) };
    }
    for exponent in 0..=remaining {
        e[position] = exponent;
        monomials(e, position + 1, remaining - exponent, callback)?;
    }
    Ok(())
}

fn reduce(
    ring: &mut Ring,
    row: &mut Row,
    combination: &mut Combination,
    pivots: &BTreeMap<Vec<u32>, Pivot>,
) -> Result<(), String> {
    while let Some((leading, c)) = row.last_key_value().map(|(e, c)| (e.clone(), *c)) {
        let Some(pivot) = pivots.get(&leading) else {
            break;
        };
        let negative = ring.p - c;
        ring.axpy(row, &pivot.row, negative)?;
        ring.axpy(combination, &pivot.combination, negative)?;
    }
    Ok(())
}

fn containment(
    ring: &mut Ring,
    generators: &[Poly],
    targets: &[Poly],
) -> Result<Vec<Vec<Multiple>>, String> {
    let degrees: Vec<u32> = generators
        .iter()
        .map(|f| {
            f.iter()
                .map(|(e, _)| degree(e).expect("validated degree"))
                .max()
                .unwrap_or(0)
        })
        .collect();
    let mut pivots = BTreeMap::<Vec<u32>, Pivot>::new();
    let mut retained = 0usize;
    for d in 0..=ring.limits.degree {
        for (i, f) in generators.iter().enumerate() {
            if f.is_empty() || degrees[i] > d {
                continue;
            }
            let mut e = vec![0; ring.n];
            monomials(&mut e, 0, d - degrees[i], &mut |multiplier| {
                if ring.work.generated_rows >= ring.limits.rows {
                    return Err("generated row cap".into());
                }
                ring.work.generated_rows += 1;
                let mut row: Row = f.iter().map(|(e, c)| (shift(e, multiplier), *c)).collect();
                let mut combination = BTreeMap::from([((i, multiplier.to_vec()), 1)]);
                reduce(ring, &mut row, &mut combination, &pivots)?;
                ring.entries(retained + row.len() + combination.len())?;
                if let Some((leading, c)) = row.last_key_value().map(|(e, c)| (e.clone(), *c)) {
                    let inverse = ring.inverse(c)?;
                    for c in row.values_mut() {
                        *c = ring.mul(*c, inverse)?;
                    }
                    for c in combination.values_mut() {
                        *c = ring.mul(*c, inverse)?;
                    }
                    retained += row.len() + combination.len();
                    pivots.insert(leading, Pivot { row, combination });
                }
                Ok(())
            })?;
        }
        let mut proofs = Vec::new();
        let mut proof_entries = 0;
        for target in targets {
            let mut row: Row = target.iter().cloned().collect();
            let mut combination = BTreeMap::new();
            reduce(ring, &mut row, &mut combination, &pivots)?;
            if !row.is_empty() {
                break;
            }
            for c in combination.values_mut() {
                *c = ring.p - *c;
            }
            proof_entries += combination.len();
            ring.entries(retained + proof_entries)?;
            proofs.push(encode(combination));
        }
        if proofs.len() == targets.len() {
            return Ok(proofs);
        }
    }
    Err("representation not found in bounded generator-multiple span".into())
}

/// Direct identity verification uses neither the elimination table nor F4.
fn verify_containment(
    ring: &mut Ring,
    generators: &[Poly],
    targets: &[Poly],
    proofs: &[Vec<Multiple>],
) -> Result<(), String> {
    if targets.len() != proofs.len() {
        return Err("certificate target count differs".into());
    }
    let mut total = 0usize;
    for (target, proof) in targets.iter().zip(proofs) {
        let mut sum = Row::new();
        let mut seen = BTreeSet::new();
        for multiple in proof {
            total += 1;
            ring.entries(total)?;
            let f = generators
                .get(multiple.generator)
                .ok_or("certificate generator index")?;
            let c: u64 = multiple
                .coefficient
                .parse()
                .map_err(|_| "certificate coefficient")?;
            if c == 0
                || c >= ring.p
                || c.to_string() != multiple.coefficient
                || multiple.exponents.len() != ring.n
                || degree(&multiple.exponents)? > 12
                || !seen.insert((multiple.generator, &multiple.exponents))
            {
                return Err("noncanonical certificate multiple".into());
            }
            for (e, fc) in f {
                let product = ring.mul(c, *fc)?;
                ring.accumulate(&mut sum, shift(e, &multiple.exponents), product)?;
                ring.entries(total + sum.len())?;
            }
        }
        if sum != target.iter().cloned().collect() {
            return Err("certificate polynomial identity differs".into());
        }
    }
    Ok(())
}

/// Replay one supplied generator identity using the direct containment checker.
/// `limits.degree` bounds certificate *search*, not direct identity replay: the
/// existing checker supports input degree <=64 and multiplier degree <=12.
/// Work counts only direct coefficient multiplication/accumulation, excluding
/// validation, primality, parsing and I/O. Caps do not certify an uninspected tail.
/// Neither a witnessed drop nor a cap determines any first/last fall degree.
pub(super) fn verify_identity(
    generators: &[Poly],
    target: &Poly,
    proof: &[Multiple],
    n: usize,
    p: u64,
    limits: Limits,
) -> Result<IdentityAudit, String> {
    if n > 28
        || !(5..1u64 << 62).contains(&p)
        || !crate::cryptanalysis::residual_walk::is_prime_u64(p)
    {
        return Err("unsupported certificate ring".into());
    }
    let mut ring = Ring {
        p,
        n,
        limits,
        work: Work::default(),
        cap: None,
    };
    let identity_limits = IdentityLimits {
        generator_rows: 4096,
        total_polynomial_rows: limits.rows,
        polynomial_terms: 16_384,
        polynomial_degree: 64,
        multiplier_degree: 12,
        generator_multiple_degree: 76,
        entries: limits.entries,
        operations: limits.operations,
    };
    let unknown = |kind, work| IdentityAudit {
        schema: "ml-dataset.polynomial-identity-audit/v1",
        status: IdentityStatus::UnknownCap { kind },
        limits: identity_limits,
        work,
    };
    // Check structural bounds before inspecting an arbitrarily large tail.
    if generators.len() > 4096 || generators.len().saturating_add(1) > limits.rows {
        return Ok(unknown(IdentityCapKind::PolynomialRows, ring.work));
    }
    let mut terms = 0usize;
    let mut generator_degrees = Vec::with_capacity(generators.len());
    let mut result_degree = None;
    for (index, polynomial) in generators.iter().chain(std::iter::once(target)).enumerate() {
        let mut seen = BTreeSet::new();
        let mut polynomial_degree = None;
        for (exponents, coefficient) in polynomial {
            terms += 1;
            if terms > 16_384 {
                return Ok(unknown(IdentityCapKind::PolynomialTerms, ring.work));
            }
            if exponents.len() != n
                || *coefficient == 0
                || *coefficient >= p
                || !seen.insert(exponents)
            {
                return Err("invalid polynomial dimension, coefficient or duplicate term".into());
            }
            // Typed exponents are valid even when their mathematical sum does
            // not fit u32. That exceeds the verifier bound, not the ring syntax.
            let Ok(d) = degree(exponents) else {
                return Ok(unknown(IdentityCapKind::PolynomialDegree, ring.work));
            };
            if d > 64 {
                return Ok(unknown(IdentityCapKind::PolynomialDegree, ring.work));
            }
            polynomial_degree = Some(polynomial_degree.unwrap_or(0).max(d));
        }
        if index == generators.len() {
            result_degree = polynomial_degree;
        } else {
            generator_degrees.push(polynomial_degree);
        }
    }
    if proof.len() > limits.entries {
        return Ok(unknown(IdentityCapKind::ProofEntries, ring.work));
    }
    let mut keys = BTreeSet::new();
    let mut max_generator_multiple_degree = None;
    for multiple in proof {
        let generator_degree = generator_degrees
            .get(multiple.generator)
            .ok_or("certificate generator index")?;
        let coefficient = multiple
            .coefficient
            .parse::<u64>()
            .map_err(|_| "certificate coefficient")?;
        if coefficient == 0
            || coefficient >= p
            || coefficient.to_string() != multiple.coefficient
            || multiple.exponents.len() != n
            || !keys.insert((multiple.generator, &multiple.exponents))
        {
            return Err("noncanonical certificate multiple".into());
        }
        let Ok(multiplier_degree) = degree(&multiple.exponents) else {
            return Ok(unknown(IdentityCapKind::MultiplierDegree, ring.work));
        };
        if multiplier_degree > 12 {
            return Ok(unknown(IdentityCapKind::MultiplierDegree, ring.work));
        }
        // A zero generator contributes no nonzero summand, regardless of shift.
        if let Some(generator_degree) = generator_degree {
            let summand_degree = generator_degree + multiplier_degree;
            max_generator_multiple_degree = Some(
                max_generator_multiple_degree
                    .unwrap_or(0)
                    .max(summand_degree),
            );
        }
    }
    // The shared checker takes multiple targets. This bounded copy preserves its
    // existing interface and avoids introducing another polynomial arithmetic path.
    let proofs = [proof.to_vec()];
    match verify_containment(&mut ring, generators, std::slice::from_ref(target), &proofs) {
        Ok(()) => Ok(IdentityAudit {
            schema: "ml-dataset.polynomial-identity-audit/v1",
            status: IdentityStatus::Verified {
                result_degree,
                max_generator_multiple_degree,
                witnessed_degree_drop: matches!(
                    (result_degree, max_generator_multiple_degree),
                    (Some(result), Some(summand)) if result < summand
                ),
            },
            limits: identity_limits,
            work: ring.work,
        }),
        Err(reason) => match ring.cap {
            Some(kind) => Ok(unknown(kind, ring.work)),
            None => Err(reason),
        },
    }
}

pub(super) fn equivalent(
    left: &[Poly],
    right: &[Poly],
    n: usize,
    p: u64,
    limits: Limits,
) -> Result<Audit, String> {
    if n > 28
        || !(5..1u64 << 62).contains(&p)
        || !crate::cryptanalysis::residual_walk::is_prime_u64(p)
        || limits.degree > 12
    {
        return Err("unsupported certificate ring or degree limit".into());
    }
    validate(left, n, p)?;
    validate(right, n, p)?;
    let mut ring = Ring {
        p,
        n,
        limits,
        work: Work::default(),
        cap: None,
    };
    let result = (|| {
        let left_from_right = containment(&mut ring, right, left)?;
        let right_from_left = containment(&mut ring, left, right)?;
        let certificate = Certificate {
            left_from_right,
            right_from_left,
        };
        verify_containment(&mut ring, right, left, &certificate.left_from_right)?;
        verify_containment(&mut ring, left, right, &certificate.right_from_left)?;
        Ok::<_, String>(certificate)
    })();
    let (status, reason, certificate) = match result {
        Ok(proof) => ("ideal_equality_certified", None, Some(proof)),
        Err(reason) => ("unknown", Some(reason), None),
    };
    Ok(Audit {
        schema: "ml-dataset.ideal-equality-certificate/v1",
        status,
        reason,
        limits,
        work: ring.work,
        certificate,
        native_execution_trace_verified: false,
        native_instruction_count_verified: false,
        exact_fall_degree_verified: false,
    })
}

fn trim(f: &mut Vec<u64>) {
    while f.last() == Some(&0) {
        f.pop();
    }
}

fn remainder(ring: &mut Ring, f: &[u64], g: &[u64]) -> Result<Vec<u64>, String> {
    if g.is_empty() {
        return Err("division by zero polynomial".into());
    }
    let mut r = f.to_vec();
    let inv = ring.inverse(*g.last().unwrap())?;
    trim(&mut r);
    while r.len() >= g.len() {
        let offset = r.len() - g.len();
        let c = ring.mul(*r.last().unwrap(), inv)?;
        for (i, gc) in g.iter().enumerate() {
            let product = ring.mul(c, *gc)?;
            r[i + offset] = ring.add(
                r[i + offset],
                if product == 0 { 0 } else { ring.p - product },
            )?;
        }
        trim(&mut r);
    }
    Ok(r)
}

fn gcd(ring: &mut Ring, a: &[u64], b: &[u64]) -> Result<Vec<u64>, String> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    trim(&mut a);
    trim(&mut b);
    while !b.is_empty() {
        let r = remainder(ring, &a, &b)?;
        a = b;
        b = r;
    }
    if let Some(c) = a.last() {
        let inverse = ring.inverse(*c)?;
        for c in &mut a {
            *c = ring.mul(*c, inverse)?;
        }
    }
    Ok(a)
}

fn product_mod(ring: &mut Ring, a: &[u64], b: &[u64], f: &[u64]) -> Result<Vec<u64>, String> {
    if a.is_empty() || b.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, ac) in a.iter().enumerate() {
        for (j, bc) in b.iter().enumerate() {
            let c = ring.mul(*ac, *bc)?;
            out[i + j] = ring.add(out[i + j], c)?;
        }
    }
    remainder(ring, &out, f)
}

fn power_mod(ring: &mut Ring, a: &[u64], mut e: u64, f: &[u64]) -> Result<Vec<u64>, String> {
    let mut b = remainder(ring, a, f)?;
    let mut r = vec![1];
    while e > 0 {
        if e & 1 == 1 {
            r = product_mod(ring, &r, &b, f)?;
        }
        e >>= 1;
        if e > 0 {
            b = product_mod(ring, &b, &b, f)?;
        }
    }
    Ok(r)
}

/// Deterministic capped splitter. Gcd(f,X^p-X) certifies distinct-root completeness;
/// no claim is made about the original native factorization trace or its costs.
pub(super) fn roots(
    coefficients: &[u64],
    p: u64,
    work: &mut Work,
    operation_limit: u64,
) -> Result<Vec<u64>, String> {
    if coefficients.len() > 65
        || coefficients.iter().any(|c| *c >= p)
        || !(5..1u64 << 62).contains(&p)
        || !crate::cryptanalysis::residual_walk::is_prime_u64(p)
    {
        return Err("unsupported root verifier input".into());
    }
    let mut f = coefficients.to_vec();
    trim(&mut f);
    if f.len() <= 1 {
        return Err("root verifier requires nonconstant polynomial".into());
    }
    let mut ring = Ring {
        p,
        n: 1,
        limits: Limits {
            operations: operation_limit,
            ..Limits::default()
        },
        work: Work::default(),
        cap: None,
    };
    let result = (|| {
        let mut xp = power_mod(&mut ring, &[0, 1], p, &f)?;
        xp.resize(xp.len().max(2), 0);
        xp[1] = ring.add(xp[1], p - 1)?;
        trim(&mut xp);
        let g = gcd(&mut ring, &f, &xp)?;
        let expected = g.len().saturating_sub(1);
        let mut pending = vec![g];
        let mut roots = Vec::new();
        while let Some(h) = pending.pop() {
            if h.len() <= 1 {
                continue;
            }
            if h.len() == 2 {
                let inv = ring.inverse(h[1])?;
                roots.push(ring.mul(if h[0] == 0 { 0 } else { p - h[0] }, inv)?);
                continue;
            }
            let mut factors = None;
            for delta in 0..p.min(256) {
                // First isolate a possible zero of x+delta, then quadratic characters.
                let linear = vec![delta, 1];
                let mut d = gcd(&mut ring, &h, &linear)?;
                if d.len() <= 1 || d.len() == h.len() {
                    let mut s = power_mod(&mut ring, &linear, (p - 1) / 2, &h)?;
                    if s.is_empty() {
                        s.push(0);
                    }
                    s[0] = ring.add(s[0], p - 1)?;
                    trim(&mut s);
                    d = gcd(&mut ring, &h, &s)?;
                }
                if d.len() > 1 && d.len() < h.len() {
                    let mut r = h.clone();
                    let mut q = vec![0; h.len() - d.len() + 1];
                    // d is monic. Check exact quotient, including its remainder.
                    while r.len() >= d.len() {
                        let offset = r.len() - d.len();
                        let c = *r.last().unwrap();
                        q[offset] = c;
                        for (i, dc) in d.iter().enumerate() {
                            let product = ring.mul(c, *dc)?;
                            r[i + offset] = ring
                                .add(r[i + offset], if product == 0 { 0 } else { p - product })?;
                        }
                        trim(&mut r);
                    }
                    if !r.is_empty() {
                        return Err("root factor quotient failed".into());
                    }
                    trim(&mut q);
                    factors = Some((d, q));
                    break;
                }
            }
            let (a, b) = factors.ok_or("deterministic root split cap")?;
            pending.push(a);
            pending.push(b);
        }
        roots.sort_unstable();
        roots.dedup();
        if roots.len() != expected {
            return Err("distinct-root completeness failed".into());
        }
        for x in &roots {
            let mut value = 0;
            for c in f.iter().rev() {
                value = ring.mul(value, *x)?;
                value = ring.add(value, *c)?;
            }
            if value != 0 {
                return Err("root evaluation failed".into());
            }
        }
        Ok(roots)
    })();
    *work = ring.work;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn roots(coefficients: &[u64], p: u64) -> Result<Vec<u64>, String> {
        super::roots(
            coefficients,
            p,
            &mut Work::default(),
            Limits::default().operations,
        )
    }
    fn poly(terms: &[(&[u32], u64)]) -> Poly {
        terms.iter().map(|(e, c)| (e.to_vec(), *c)).collect()
    }
    fn multiple(generator: usize, exponents: &[u32], coefficient: u64) -> Multiple {
        Multiple {
            generator,
            exponents: exponents.to_vec(),
            coefficient: coefficient.to_string(),
        }
    }
    #[test]
    fn identity_replay_certifies_cancellation_without_search_minimality() {
        // y = y*x^2 + (xy+y) - x*(xy+y), with a degree-three summand.
        let generators = vec![
            poly(&[(&[2, 0], 1)]),
            poly(&[(&[1, 1], 1), (&[0, 1], 1)]),
            vec![],
        ];
        let proof = vec![
            multiple(0, &[0, 1], 1),
            multiple(1, &[0, 0], 1),
            multiple(1, &[1, 0], 126),
            multiple(2, &[12, 0], 1),
        ];
        let audit = verify_identity(
            &generators,
            &poly(&[(&[0, 1], 1)]),
            &proof,
            2,
            127,
            Limits {
                degree: 0,
                ..Limits::default()
            },
        )
        .unwrap();
        assert_eq!(
            audit.status,
            IdentityStatus::Verified {
                result_degree: Some(1),
                max_generator_multiple_degree: Some(3),
                witnessed_degree_drop: true,
            }
        );
        assert_eq!(audit.work.coefficient_operations, 10);
        assert_eq!(audit.limits.generator_multiple_degree, 76);
        assert!(serde_json::to_value(&audit).unwrap()["limits"]
            .get("degree")
            .is_none());
    }
    #[test]
    fn identity_zero_results_and_generators_do_not_claim_degree_drops() {
        let x = poly(&[(&[1], 1)]);
        let audit = verify_identity(
            &[x.clone(), x],
            &vec![],
            &[multiple(0, &[0], 1), multiple(1, &[0], 126)],
            1,
            127,
            Limits::default(),
        )
        .unwrap();
        assert_eq!(
            audit.status,
            IdentityStatus::Verified {
                result_degree: None,
                max_generator_multiple_degree: Some(1),
                witnessed_degree_drop: false,
            }
        );
        let zero = verify_identity(
            &[vec![]],
            &vec![],
            &[multiple(0, &[12], 1)],
            1,
            127,
            Limits::default(),
        )
        .unwrap();
        assert_eq!(
            zero.status,
            IdentityStatus::Verified {
                result_degree: None,
                max_generator_multiple_degree: None,
                witnessed_degree_drop: false,
            }
        );
        assert_eq!(zero.work.coefficient_operations, 0);
        assert!(matches!(
            verify_identity(&[], &vec![], &[], 0, 127, Limits::default())
                .unwrap()
                .status,
            IdentityStatus::Verified {
                result_degree: None,
                ..
            }
        ));
    }
    #[test]
    fn identity_corruption_and_duplicate_encodings_are_hard_errors() {
        let x = poly(&[(&[1], 1)]);
        let proof = multiple(0, &[0], 1);
        for bad_proof in [
            vec![multiple(0, &[0], 2)],
            vec![multiple(1, &[0], 1)],
            vec![multiple(0, &[0, 0], 1)],
            vec![multiple(0, &[0], 0)],
            vec![multiple(0, &[0], 127)],
            vec![proof.clone(), proof.clone()],
            vec![Multiple {
                coefficient: "01".into(),
                ..proof.clone()
            }],
            vec![Multiple {
                coefficient: "+1".into(),
                ..proof.clone()
            }],
        ] {
            assert!(verify_identity(
                std::slice::from_ref(&x),
                &x,
                &bad_proof,
                1,
                127,
                Limits::default()
            )
            .is_err());
        }
        for bad_poly in [
            poly(&[(&[1], 1), (&[1], 2)]),
            poly(&[(&[1], 0)]),
            poly(&[(&[1], 127)]),
            poly(&[(&[1, 0], 1)]),
        ] {
            assert!(verify_identity(
                &[bad_poly],
                &x,
                std::slice::from_ref(&proof),
                1,
                127,
                Limits::default()
            )
            .is_err());
        }
        assert!(verify_identity(
            std::slice::from_ref(&x),
            &x,
            &[proof],
            1,
            15,
            Limits::default()
        )
        .is_err());
        // Well-formed source or target substitutions must still fail the identity.
        let two_x = poly(&[(&[1], 2)]);
        assert!(verify_identity(
            std::slice::from_ref(&two_x),
            &x,
            &[multiple(0, &[0], 1)],
            1,
            127,
            Limits::default(),
        )
        .is_err());
        assert!(verify_identity(
            &[x],
            &two_x,
            &[multiple(0, &[0], 1)],
            1,
            127,
            Limits::default(),
        )
        .is_err());
    }
    #[test]
    fn identity_typed_caps_retain_partial_work_and_do_not_leak_verified_labels() {
        let x = poly(&[(&[1], 1)]);
        let proof = vec![multiple(0, &[0], 1)];
        for (limits, kind, operations) in [
            (
                Limits {
                    operations: 1,
                    ..Limits::default()
                },
                IdentityCapKind::CoefficientOperations,
                1,
            ),
            (
                Limits {
                    entries: 1,
                    ..Limits::default()
                },
                IdentityCapKind::SparseEntries,
                2,
            ),
            (
                Limits {
                    entries: 0,
                    ..Limits::default()
                },
                IdentityCapKind::ProofEntries,
                0,
            ),
            (
                Limits {
                    rows: 1,
                    ..Limits::default()
                },
                IdentityCapKind::PolynomialRows,
                0,
            ),
        ] {
            let audit =
                verify_identity(std::slice::from_ref(&x), &x, &proof, 1, 127, limits).unwrap();
            assert_eq!(audit.status, IdentityStatus::UnknownCap { kind });
            assert_eq!(audit.work.coefficient_operations, operations);
            let value = serde_json::to_value(audit).unwrap();
            assert!(value["status"].get("result_degree").is_none());
            assert!(value["status"].get("witnessed_degree_drop").is_none());
        }
        // A new call has no stale cap state from any preceding attempt.
        assert!(matches!(
            verify_identity(
                std::slice::from_ref(&x),
                &x,
                &proof,
                1,
                127,
                Limits::default()
            )
            .unwrap()
            .status,
            IdentityStatus::Verified { .. }
        ));
        assert_eq!(
            verify_identity(
                &[poly(&[(&[65], 1)])],
                &vec![],
                &[],
                1,
                127,
                Limits::default()
            )
            .unwrap()
            .status,
            IdentityStatus::UnknownCap {
                kind: IdentityCapKind::PolynomialDegree
            }
        );
        assert_eq!(
            verify_identity(
                &[x],
                &vec![],
                &[multiple(0, &[13], 1)],
                1,
                127,
                Limits::default()
            )
            .unwrap()
            .status,
            IdentityStatus::UnknownCap {
                kind: IdentityCapKind::MultiplierDegree
            }
        );
        let many_terms: Poly = (0..16_385u32)
            .map(|i| (vec![i % 16, (i / 16) % 16, (i / 256) % 16, i / 4096], 1))
            .collect();
        assert_eq!(
            verify_identity(&[many_terms], &vec![], &[], 4, 127, Limits::default())
                .unwrap()
                .status,
            IdentityStatus::UnknownCap {
                kind: IdentityCapKind::PolynomialTerms
            }
        );
    }
    #[test]
    fn identity_oversized_degree_sums_are_caps_and_search_limit_is_ignored() {
        for (generators, target) in [
            (vec![poly(&[(&[u32::MAX, 1], 1)])], vec![]),
            (vec![vec![]], poly(&[(&[u32::MAX, 1], 1)])),
        ] {
            assert_eq!(
                verify_identity(&generators, &target, &[], 2, 127, Limits::default())
                    .unwrap()
                    .status,
                IdentityStatus::UnknownCap {
                    kind: IdentityCapKind::PolynomialDegree
                },
            );
        }
        assert_eq!(
            verify_identity(
                &[vec![]],
                &vec![],
                &[multiple(0, &[u32::MAX, 1], 1)],
                2,
                127,
                Limits::default()
            )
            .unwrap()
            .status,
            IdentityStatus::UnknownCap {
                kind: IdentityCapKind::MultiplierDegree
            },
        );
        let x = poly(&[(&[1], 1)]);
        let proof = vec![multiple(0, &[0], 1)];
        let audit = verify_identity(
            std::slice::from_ref(&x),
            &x,
            &proof,
            1,
            127,
            Limits {
                degree: u32::MAX,
                ..Limits::default()
            },
        )
        .unwrap();
        assert!(matches!(audit.status, IdentityStatus::Verified { .. }));
    }
    #[test]
    fn identity_wide_coefficients_match_bigint_and_decimal_proof_encoding() {
        use num_bigint::BigUint;
        let p = (1u64 << 61) - 1;
        let c = p - 2;
        let a = p - 3;
        let expected: u64 = ((BigUint::from(c) * BigUint::from(a)) % BigUint::from(p))
            .try_into()
            .unwrap();
        let proof = vec![multiple(0, &[1], c)];
        let audit = verify_identity(
            &[poly(&[(&[1], a)])],
            &poly(&[(&[2], expected)]),
            &proof,
            1,
            p,
            Limits::default(),
        )
        .unwrap();
        assert_eq!(
            audit.status,
            IdentityStatus::Verified {
                result_degree: Some(2),
                max_generator_multiple_degree: Some(2),
                witnessed_degree_drop: false,
            }
        );
        assert_eq!(audit.work.coefficient_operations, 2);
        assert_eq!(
            serde_json::to_value(proof).unwrap()[0]["coefficient"],
            c.to_string()
        );
    }
    #[test]
    fn certificates_check_both_containments_and_cancellation() {
        // <x+y, x-y> = <x,y> in odd characteristic.
        let left = vec![
            poly(&[(&[1, 0], 1), (&[0, 1], 1)]),
            poly(&[(&[1, 0], 1), (&[0, 1], 126)]),
        ];
        let right = vec![poly(&[(&[1, 0], 1)]), poly(&[(&[0, 1], 1)])];
        let audit = equivalent(&left, &right, 2, 127, Limits::default()).unwrap();
        assert_eq!(audit.status, "ideal_equality_certified");
        let mut proof = audit.certificate.unwrap();
        proof.left_from_right[0][0].coefficient = "2".into();
        let mut ring = Ring {
            p: 127,
            n: 2,
            limits: Limits::default(),
            work: Work::default(),
            cap: None,
        };
        assert!(verify_containment(&mut ring, &right, &left, &proof.left_from_right).is_err());
    }
    #[test]
    fn strict_containment_and_search_caps_are_unknown() {
        let x = vec![poly(&[(&[1], 1)])];
        let xx = vec![poly(&[(&[2], 1)])];
        assert_eq!(
            equivalent(&x, &xx, 1, 127, Limits::default())
                .unwrap()
                .status,
            "unknown"
        );
        for limits in [
            Limits {
                rows: 0,
                ..Limits::default()
            },
            Limits {
                entries: 0,
                ..Limits::default()
            },
            Limits {
                operations: 0,
                ..Limits::default()
            },
        ] {
            assert_eq!(
                equivalent(&x, &x, 1, 127, limits).unwrap().status,
                "unknown"
            );
        }
        assert_eq!(
            equivalent(&[], &[vec![]], 0, 127, Limits::default())
                .unwrap()
                .status,
            "ideal_equality_certified"
        );
    }
    #[test]
    fn malformed_ring_and_terms_are_rejected() {
        for (poly, n, p) in [
            (poly(&[(&[1], 1)]), 1, 15),
            (poly(&[(&[1], 0)]), 1, 127),
            (poly(&[(&[1], 127)]), 1, 127),
            (poly(&[(&[1], 1), (&[1], 2)]), 1, 127),
            (poly(&[(&[1], 1)]), 2, 127),
            (poly(&[(&[u32::MAX], 1)]), 1, 127),
        ] {
            assert!(equivalent(&[poly], &[], n, p, Limits::default()).is_err());
        }
    }
    #[test]
    fn bounded_span_miss_can_be_followed_by_an_exact_higher_degree_certificate() {
        // y = (1-x)(xy+y) + y*x^2. Degree2 generator multiples cannot
        // certify y here, while degree3 can; a low-bound miss is not a lower
        // bound on first/last fall degree or proof of ideal nonmembership.
        let left = vec![poly(&[(&[2, 0], 1)]), poly(&[(&[1, 1], 1), (&[0, 1], 1)])];
        let mut right = left.clone();
        right.push(poly(&[(&[0, 1], 1)]));
        let low = equivalent(
            &left,
            &right,
            2,
            127,
            Limits {
                degree: 2,
                ..Limits::default()
            },
        )
        .unwrap();
        assert_eq!(low.status, "unknown");
        let high = equivalent(
            &left,
            &right,
            2,
            127,
            Limits {
                degree: 3,
                ..Limits::default()
            },
        )
        .unwrap();
        assert_eq!(high.status, "ideal_equality_certified");
        assert!(high.certificate.unwrap().right_from_left[2]
            .iter()
            .any(|m| m.exponents.iter().any(|e| *e > 0)));
    }
    #[test]
    fn root_operation_caps_preserve_partial_verification_work() {
        let mut work = Work::default();
        assert!(super::roots(&[1, 125, 1], 127, &mut work, 5).is_err());
        assert_eq!(work.coefficient_operations, 5);
    }
    #[test]
    fn wide_field_certificate_has_independent_bigint_identity() {
        use num_bigint::BigUint;
        let p = (1u64 << 61) - 1;
        let left = vec![poly(&[(&[1], p - 1), (&[0], p - 2)])];
        let right = vec![poly(&[(&[1], 1), (&[0], 2)])];
        let audit = equivalent(&left, &right, 1, p, Limits::default()).unwrap();
        let proof = audit.certificate.unwrap();
        for (sources, targets, proofs) in [
            (&left, &right, &proof.right_from_left),
            (&right, &left, &proof.left_from_right),
        ] {
            for (target, multiples) in targets.iter().zip(proofs) {
                let mut sums = BTreeMap::<Vec<u32>, BigUint>::new();
                for multiple in multiples {
                    for (e, c) in &sources[multiple.generator] {
                        *sums.entry(shift(e, &multiple.exponents)).or_default() +=
                            BigUint::from(*c) * multiple.coefficient.parse::<BigUint>().unwrap();
                    }
                }
                let actual: BTreeMap<_, _> = sums
                    .into_iter()
                    .map(|(e, c)| (e, c % BigUint::from(p)))
                    .filter(|(_, c)| *c != BigUint::from(0u32))
                    .collect();
                assert_eq!(
                    actual,
                    target
                        .iter()
                        .map(|(e, c)| (e.clone(), BigUint::from(*c)))
                        .collect()
                );
            }
        }
    }
    #[test]
    fn root_completeness_matches_exhaustive_fields_and_repeated_roots() {
        for p in [5, 7, 11, 127] {
            for a in 0..p.min(8) {
                for b in 0..p.min(8) {
                    let f = vec![a, b, 1];
                    let expected: Vec<_> =
                        (0..p).filter(|x| (x * x + b * x + a) % p == 0).collect();
                    assert_eq!(roots(&f, p).unwrap(), expected);
                }
            }
        }
        let p = (1u64 << 61) - 1;
        assert_eq!(roots(&[1, p - 2, 1], p).unwrap(), vec![1]);
        assert_eq!(roots(&[0, p - 1, 0, 1], p).unwrap(), vec![0, 1, p - 1]);
    }
}
