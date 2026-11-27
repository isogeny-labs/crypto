//! Bounded public-input RR9 B8 full-eight-equation F4 screen.
//! Protocol: RR9_FULL8_F4_PROTOCOL_20261122.md. No signed leaves enter F4.
#![recursion_limit = "256"]
#[cfg(test)]
use prime31_algebraic_ic::f4_fp_tower::{mono_w, ONE};
use prime31_algebraic_ic::{
    check_curve,
    f4_fp_tower::{
        degree_of, f4_tower, free_of, mask_of, RPoly, SquareRule, TowerF4Options, TowerRing,
        TowerWeights,
    },
    Arithmetic, PRIME31,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::time::{Duration, Instant};

const P: u64 = 2_147_483_647;
const XS: [u64; 8] = [0, 1, 2, 3, 5, 6, 8, 9];
const RESIDUALS: [(u64, u64); 3] = [
    (1_911_579_686, 548_820_096),
    (625_037_715, 2_100_627_251),
    (1_923_055_775, 307_731_458),
];
const VARIABLES: usize = 7;
const RADIX_LEVELS: usize = 4;
const RADIX_CUTOFF_DEGREE: u32 = 1 << RADIX_LEVELS;
const MAX_FINAL_JSON_BYTES: usize = 128 * 1024 * 1024;
const MAX_BASIS_POLYNOMIALS: usize = 100_000;
const MAX_BASIS_TERMS: usize = 1_000_000;
const MAX_SERIALIZED_CERTIFICATE_TERMS: usize = 1_000_000;
const MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS: usize = 2_000_000;
const MAX_INTERNAL_PROVENANCE_TERMS: usize = 3_000_000;
const MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK: usize = 2_000_000;
type Exp = [u8; 7];
type Poly = BTreeMap<Exp, u64>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct CanonicalTerm {
    exponents: Exp,
    coefficient: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct LinearEvidence {
    raw_rows: Vec<[u64; 8]>,
    rref_rows: Vec<[u64; 8]>,
    pivot_columns: Vec<usize>,
    coefficient_rank: usize,
    augmented_rank: usize,
    unique_solution: bool,
    row_solution: Option<[u64; 7]>,
    blocker: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReplayFailure {
    blocker: &'static str,
    accumulator_peak: usize,
}

struct CertificateSuccess {
    multipliers: Vec<Poly>,
    terms_observed: usize,
    accumulator_peak: usize,
}

struct CertificateFailure {
    blocker: String,
    terms_observed: usize,
    accumulator_peak: usize,
    internal_provenance_terms_observed_lower_bound: usize,
    internal_provenance_terms_observation_kind: &'static str,
    internal_recomposition_product_work_observed_lower_bound: usize,
    internal_recomposition_product_work_observation_kind: &'static str,
    resource_reason: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AttemptObservation {
    value: usize,
    kind: &'static str,
}

impl AttemptObservation {
    fn exact(value: usize) -> Self {
        Self {
            value,
            kind: "exact",
        }
    }

    fn lower_bound(value: usize) -> Self {
        Self {
            value,
            kind: "lower_bound",
        }
    }
}

fn parse_exporter_attempt(
    blocker: &str,
    prefix: &str,
    expected_limit: usize,
) -> Option<AttemptObservation> {
    let suffix = blocker.strip_prefix(prefix)?;
    let fallback = || AttemptObservation::lower_bound(expected_limit.saturating_add(1));
    let Some(payload) = suffix.strip_prefix(": attempted ") else {
        return Some(fallback());
    };
    let Some((attempted, reported_limit)) = payload.split_once(", limit ") else {
        return Some(fallback());
    };
    if reported_limit.parse::<usize>().ok() != Some(expected_limit) {
        return Some(fallback());
    }
    if attempted.starts_with("at least ") {
        return Some(fallback());
    }
    Some(
        attempted
            .parse::<usize>()
            .ok()
            .filter(|&value| value > expected_limit)
            .map_or_else(fallback, AttemptObservation::exact),
    )
}

fn provenance_attempt(blocker: &str) -> Option<AttemptObservation> {
    const AGGREGATE_PREFIX: &str = "unit certificate aggregate provenance term budget exceeded";
    const PROOF_PREFIX: &str = "unit certificate provenance term budget exceeded";
    if blocker == "unit certificate aggregate provenance term count overflow" {
        return Some(AttemptObservation::lower_bound(
            MAX_INTERNAL_PROVENANCE_TERMS + 1,
        ));
    }
    parse_exporter_attempt(blocker, AGGREGATE_PREFIX, MAX_INTERNAL_PROVENANCE_TERMS)
        .or_else(|| parse_exporter_attempt(blocker, PROOF_PREFIX, MAX_INTERNAL_PROVENANCE_TERMS))
}

fn product_work_attempt(blocker: &str) -> Option<AttemptObservation> {
    parse_exporter_attempt(
        blocker,
        "unit certificate internal recomposition product-work budget exceeded",
        MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK,
    )
}

fn exporter_accumulator_attempt(blocker: &str) -> Option<AttemptObservation> {
    parse_exporter_attempt(
        blocker,
        "unit certificate recomposition accumulator-term budget exceeded",
        MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
    )
}

fn basis_within_output_limits(polynomials: usize, terms: usize) -> bool {
    polynomials <= MAX_BASIS_POLYNOMIALS && terms <= MAX_BASIS_TERMS
}

fn certificate_within_output_limits(terms: usize) -> bool {
    terms <= MAX_SERIALIZED_CERTIFICATE_TERMS
}

fn solver_basis_complete(
    timed_out: bool,
    oversize: bool,
    pairs_above_bound: usize,
    stopped_on_staircase: bool,
) -> bool {
    !timed_out && !oversize && pairs_above_bound == 0 && !stopped_on_staircase
}

fn evidence_ready_for_admission(
    complete_basis: bool,
    embedding_sound: bool,
    all_limits_satisfied: bool,
    basis_materialization_valid: bool,
) -> bool {
    complete_basis && embedding_sound && all_limits_satisfied && basis_materialization_valid
}

fn resource_limit_reason(
    basis_polynomials: usize,
    basis_terms: usize,
    certificate_terms: usize,
    certificate_product_accumulator_peak: usize,
    internal_provenance_terms_observed_lower_bound: usize,
    internal_recomposition_product_work_observed_lower_bound: usize,
) -> Option<&'static str> {
    if basis_polynomials > MAX_BASIS_POLYNOMIALS {
        Some("basis_polynomial_limit_exceeded")
    } else if basis_terms > MAX_BASIS_TERMS {
        Some("basis_term_limit_exceeded")
    } else if certificate_terms > MAX_SERIALIZED_CERTIFICATE_TERMS {
        Some("certificate_term_limit_exceeded")
    } else if certificate_product_accumulator_peak > MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS {
        Some("certificate_product_accumulator_limit_exceeded")
    } else if internal_provenance_terms_observed_lower_bound > MAX_INTERNAL_PROVENANCE_TERMS {
        Some("internal_provenance_term_limit_exceeded")
    } else if internal_recomposition_product_work_observed_lower_bound
        > MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK
    {
        Some("internal_recomposition_product_work_limit_exceeded")
    } else {
        None
    }
}

fn add(a: u64, b: u64) -> u64 {
    let z = a + b;
    if z >= P {
        z - P
    } else {
        z
    }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + P - b
    }
}
fn mul(a: u64, b: u64) -> u64 {
    let z = a * b;
    let t = (z & P) + (z >> 31);
    let u = (t & P) + (t >> 31);
    if u >= P {
        u - P
    } else {
        u
    }
}
fn power(mut a: u64, mut e: u64) -> u64 {
    let mut z = 1;
    while e != 0 {
        if e & 1 != 0 {
            z = mul(z, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    z
}

fn sha256_hex(bytes: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (bytes.len() as u64).wrapping_mul(8);
    let mut padded = bytes.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    for block in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in w.iter_mut().take(16).enumerate() {
            *word = u32::from_be_bytes(block[4 * i..4 * i + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (x, y) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *x = x.wrapping_add(y);
        }
    }
    let mut out = String::with_capacity(64);
    for word in h {
        write!(&mut out, "{word:08x}").unwrap();
    }
    out
}

fn radix_ring() -> (TowerRing, TowerWeights) {
    let mut rules = Vec::with_capacity(VARIABLES * RADIX_LEVELS);
    for variable in 0..VARIABLES {
        let base = variable * RADIX_LEVELS;
        for level in 0..RADIX_LEVELS {
            let next = (level + 1 < RADIX_LEVELS).then_some((base + level + 1) as u8);
            rules.push(SquareRule {
                next,
                a: 0,
                b: u64::from(next.is_some()),
                c: 0,
                d: 0,
            });
        }
    }
    let ring = TowerRing::new(P, rules, 0);
    let weights = TowerWeights::radix_blocks(VARIABLES, RADIX_LEVELS, 4);
    weights.check(&ring);
    (ring, weights)
}

fn open_top_pair_omission_mask() -> u64 {
    (0..VARIABLES).fold(0u64, |mask, variable| {
        mask | (1u64 << (variable * RADIX_LEVELS + RADIX_LEVELS - 1))
    })
}

fn radix_layout_valid(ring: &TowerRing, weights: TowerWeights, open_top_mask: u64) -> bool {
    if ring.p != P
        || ring.n_free != 0
        || ring.rules.len() != VARIABLES * RADIX_LEVELS
        || open_top_mask != open_top_pair_omission_mask()
        || (VARIABLES * RADIX_LEVELS..64).any(|index| weights.weight(index as u32) != 1)
    {
        return false;
    }
    for variable in 0..VARIABLES {
        let base = variable * RADIX_LEVELS;
        for level in 0..RADIX_LEVELS {
            let index = base + level;
            let next = (level + 1 < RADIX_LEVELS).then_some((index + 1) as u8);
            let expected = SquareRule {
                next,
                a: 0,
                b: u64::from(next.is_some()),
                c: 0,
                d: 0,
            };
            if ring.rules[index] != expected
                || weights.weight(index as u32) != 1u32 << level
                || ((open_top_mask >> index) & 1 == 1) != (level + 1 == RADIX_LEVELS)
            {
                return false;
            }
        }
    }
    true
}

fn embedded_raw_exponents(e: &Exp) -> Vec<u32> {
    let mut out = vec![0u32; VARIABLES * RADIX_LEVELS];
    for variable in 0..VARIABLES {
        out[variable * RADIX_LEVELS] = u32::from(e[variable]);
    }
    out
}

fn decode_radix_mask(mono: u128) -> Result<Exp, &'static str> {
    if free_of(mono) != [0; 4] {
        return Err("radix_monomial_has_free_exponents");
    }
    let mask = mask_of(mono);
    if mask >> (VARIABLES * RADIX_LEVELS) != 0 {
        return Err("radix_monomial_has_bits_outside_seven_blocks");
    }
    let mut out = [0u8; VARIABLES];
    for (variable, exponent) in out.iter_mut().enumerate() {
        for level in 0..RADIX_LEVELS {
            if mask >> (variable * RADIX_LEVELS + level) & 1 == 1 {
                *exponent += 1u8 << level;
            }
        }
    }
    Ok(out)
}

fn decode_weighted_monomial(mono: u128, max_degree: u32) -> Result<Exp, &'static str> {
    let out = decode_radix_mask(mono)?;
    let ordinary_degree: u32 = out.iter().map(|&x| u32::from(x)).sum();
    if ordinary_degree != degree_of(mono) {
        return Err("radix_basis_monomial_weighted_degree_mismatch");
    }
    if ordinary_degree > max_degree {
        return Err("radix_basis_monomial_above_frozen_degree_bound");
    }
    Ok(out)
}

fn canonical_poly(poly: &Poly) -> Vec<CanonicalTerm> {
    poly.iter()
        .rev()
        .map(|(&exponents, &coefficient)| CanonicalTerm {
            exponents,
            coefficient,
        })
        .collect()
}

fn canonical_rpoly(poly: &RPoly, max_degree: u32) -> Result<Vec<CanonicalTerm>, &'static str> {
    let mut decoded = Poly::new();
    for (&mono, &coefficient) in poly.monos.iter().zip(&poly.coefs) {
        let exponents = decode_weighted_monomial(mono, max_degree)?;
        let coefficient = u64::from(coefficient);
        let sum = add(decoded.get(&exponents).copied().unwrap_or(0), coefficient);
        if sum == 0 {
            decoded.remove(&exponents);
        } else {
            decoded.insert(exponents, sum);
        }
    }
    Ok(canonical_poly(&decoded))
}

fn validate_basis_monomials(basis: &[RPoly], max_degree: u32) -> Result<(), &'static str> {
    for polynomial in basis {
        for &monomial in &polynomial.monos {
            decode_weighted_monomial(monomial, max_degree)?;
        }
    }
    Ok(())
}

struct PreparedBasisEvidence {
    polynomial_count: usize,
    term_count: usize,
    within_output_limits: bool,
    encoding_result: Result<(), &'static str>,
    canonical_polynomials: Result<Vec<Vec<CanonicalTerm>>, &'static str>,
}

fn prepare_basis_evidence(basis: &[RPoly], max_degree: u32) -> PreparedBasisEvidence {
    let polynomial_count = basis.len();
    let term_count = basis
        .iter()
        .try_fold(0usize, |total, polynomial| {
            total.checked_add(polynomial.len())
        })
        .unwrap_or(usize::MAX);
    let within_output_limits = basis_within_output_limits(polynomial_count, term_count);
    let encoding_result = validate_basis_monomials(basis, max_degree);
    let canonical_polynomials = if within_output_limits {
        basis
            .iter()
            .map(|polynomial| canonical_rpoly(polynomial, max_degree))
            .collect()
    } else {
        Err("basis_evidence_output_limit_exceeded")
    };
    PreparedBasisEvidence {
        polynomial_count,
        term_count,
        within_output_limits,
        encoding_result,
        canonical_polynomials,
    }
}

fn canonical_input_rpoly(poly: &RPoly) -> Result<Vec<CanonicalTerm>, &'static str> {
    let mut decoded = Poly::new();
    for (&mono, &coefficient) in poly.monos.iter().zip(&poly.coefs) {
        let exponents = decode_radix_mask(mono)?;
        let coefficient = u64::from(coefficient);
        let sum = add(decoded.get(&exponents).copied().unwrap_or(0), coefficient);
        if sum == 0 {
            decoded.remove(&exponents);
        } else {
            decoded.insert(exponents, sum);
        }
    }
    Ok(canonical_poly(&decoded))
}
fn term(q: &mut Poly, c: u64, i: Option<usize>, j: Option<usize>) {
    if c == 0 {
        return;
    }
    let mut e = [0u8; 7];
    for k in [i, j].into_iter().flatten() {
        e[k] += 1;
    }
    let z = add(q.get(&e).copied().unwrap_or(0), c);
    if z == 0 {
        q.remove(&e);
    } else {
        q.insert(e, z);
    }
}
fn poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut z = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            z[i + j] = add(z[i + j], mul(x, y));
        }
    }
    z
}
fn h_base() -> Vec<u64> {
    XS.iter()
        .fold(vec![1], |h, &x| poly_mul(&h, &[sub(0, x), 1]))
}
fn quadrics(r: u64, s: u64, h: &[u64]) -> [Poly; 8] {
    let mut q: [Poly; 8] = std::array::from_fn(|_| BTreeMap::new());
    let d = [add(mul(r, r), PRIME31.a), r, 1];
    let vvar = |i: usize| if i == 3 { None } else { Some(i + 4) };
    for i in 0..4 {
        for j in 0..4 {
            for (k, &dk) in d.iter().enumerate() {
                if i + j + k < 8 {
                    term(&mut q[i + j + k], dk, vvar(i), vvar(j));
                }
            }
            term(&mut q[i + j], r, Some(i), Some(j));
            if i + j + 1 < 8 {
                term(&mut q[i + j + 1], P - 1, Some(i), Some(j));
            }
            term(&mut q[i + j], sub(0, mul(2, s)), Some(i), vvar(j));
        }
    }
    for i in 0..8 {
        term(&mut q[i], sub(0, h[i]), None, None);
    }
    q
}
fn canonical_input_system(case: usize, h: &[u64], q: &[Poly; 8]) -> Value {
    let residual = RESIDUALS[case];
    let input_quadrics: Vec<Vec<CanonicalTerm>> = q.iter().map(canonical_poly).collect();
    json!({
        "schema": "prime31.rr9.full8-input-system.v1",
        "p": P,
        "case": case,
        "public_residual": [residual.0, residual.1],
        "base_x": XS,
        "h8_coefficients": h,
        "input_quadrics": input_quadrics,
    })
}
fn direct_m(c: &[u64; 7], r: u64, s: u64) -> Vec<u64> {
    let a = &c[..4];
    let v = [c[4], c[5], c[6], 1];
    let d = [add(mul(r, r), PRIME31.a), r, 1];
    let dvv = poly_mul(&d, &poly_mul(&v, &v));
    let xaa = poly_mul(&[sub(0, r), 1], &poly_mul(a, a));
    let av = poly_mul(a, &v);
    (0..9)
        .map(|i| {
            let mut z = dvv[i];
            if i < xaa.len() {
                z = sub(z, xaa[i]);
            }
            if i < av.len() {
                z = sub(z, mul(mul(2, s), av[i]));
            }
            z
        })
        .collect()
}
#[cfg(test)]
fn eval(q: &Poly, c: &[u64; 7]) -> u64 {
    q.iter().fold(0, |sum, (e, &a)| {
        let mut z = a;
        for i in 0..7 {
            for _ in 0..e[i] {
                z = mul(z, c[i]);
            }
        }
        add(sum, z)
    })
}
fn eval_canonical(q: &[CanonicalTerm], c: &[u64; 7]) -> u64 {
    q.iter().fold(0, |sum, term| {
        let mut z = term.coefficient;
        for (i, &exponent) in term.exponents.iter().enumerate() {
            for _ in 0..exponent {
                z = mul(z, c[i]);
            }
        }
        add(sum, z)
    })
}
fn linear_evidence(basis: &[RPoly]) -> LinearEvidence {
    let mut rows = Vec::<[u64; 8]>::new();
    for g in basis {
        let terms = match canonical_rpoly(g, 9) {
            Ok(terms) => terms,
            Err(blocker) => {
                return LinearEvidence {
                    raw_rows: rows.clone(),
                    rref_rows: rows,
                    pivot_columns: vec![],
                    coefficient_rank: 0,
                    augmented_rank: 0,
                    unique_solution: false,
                    row_solution: None,
                    blocker: Some(blocker),
                }
            }
        };
        let polynomial_degree = terms
            .iter()
            .map(|t| t.exponents.iter().map(|&x| u32::from(x)).sum::<u32>())
            .max()
            .unwrap_or(0);
        if polynomial_degree > 1 {
            continue;
        }
        let mut row = [0u64; 8];
        for term in terms {
            let degree = term
                .exponents
                .iter()
                .map(|&x| usize::from(x))
                .sum::<usize>();
            if degree == 0 {
                row[7] = add(row[7], term.coefficient);
            } else if degree == 1 {
                let Some(i) = term.exponents.iter().position(|&x| x == 1) else {
                    return LinearEvidence {
                        raw_rows: rows.clone(),
                        rref_rows: rows,
                        pivot_columns: vec![],
                        coefficient_rank: 0,
                        augmented_rank: 0,
                        unique_solution: false,
                        row_solution: None,
                        blocker: Some("decoded_degree_one_term_is_not_a_basis_variable"),
                    };
                };
                row[i] = add(row[i], term.coefficient);
            } else {
                unreachable!("degree-at-most-one polynomial");
            }
        }
        rows.push(row);
    }
    let raw_rows = rows.clone();
    let mut pivot_columns = Vec::new();
    let mut pivot = 0usize;
    for col in 0..7 {
        let Some(at) = (pivot..rows.len()).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(pivot, at);
        let inv = power(rows[pivot][col], P - 2);
        for k in 0..8 {
            rows[pivot][k] = mul(rows[pivot][k], inv);
        }
        for i in 0..rows.len() {
            if i == pivot {
                continue;
            }
            let f = rows[i][col];
            if f != 0 {
                for k in 0..8 {
                    rows[i][k] = sub(rows[i][k], mul(f, rows[pivot][k]));
                }
            }
        }
        pivot_columns.push(col);
        pivot += 1;
    }
    let coefficient_rank = pivot_columns.len();
    let inconsistent = rows
        .iter()
        .any(|r| r[..7].iter().all(|&x| x == 0) && r[7] != 0);
    let augmented_rank = coefficient_rank + usize::from(inconsistent);
    let unique_solution = coefficient_rank == 7 && !inconsistent;
    let row_solution = unique_solution.then(|| {
        let mut solution = [0u64; 7];
        for (row, &column) in pivot_columns.iter().enumerate() {
            solution[column] = sub(0, rows[row][7]);
        }
        solution
    });
    let blocker = if inconsistent {
        Some("affine_rows_are_inconsistent")
    } else if coefficient_rank < 7 {
        Some("affine_coefficient_rank_below_seven")
    } else {
        None
    };
    LinearEvidence {
        raw_rows,
        rref_rows: rows,
        pivot_columns,
        coefficient_rank,
        augmented_rank,
        unique_solution,
        row_solution,
        blocker,
    }
}

fn replay_unit_identity(
    quadrics: &[Poly; 8],
    multipliers: &[Poly],
    max_product_degree: u32,
    max_accumulator_terms: usize,
) -> Result<(bool, usize), ReplayFailure> {
    if multipliers.len() != quadrics.len() {
        return Err(ReplayFailure {
            blocker: "unit_certificate_does_not_have_eight_multipliers",
            accumulator_peak: 0,
        });
    }
    let mut sum = Poly::new();
    let mut peak_terms = 0usize;
    for (multiplier, quadric) in multipliers.iter().zip(quadrics) {
        for (a_exp, &a_coefficient) in multiplier {
            for (b_exp, &b_coefficient) in quadric {
                let mut exponent = [0u8; VARIABLES];
                for i in 0..VARIABLES {
                    exponent[i] = a_exp[i].checked_add(b_exp[i]).ok_or(ReplayFailure {
                        blocker: "unit_certificate_exponent_addition_overflow",
                        accumulator_peak: peak_terms,
                    })?;
                }
                if exponent.iter().map(|&x| u32::from(x)).sum::<u32>() > max_product_degree {
                    return Err(ReplayFailure {
                        blocker: "unit_certificate_product_exceeds_frozen_degree_bound",
                        accumulator_peak: peak_terms,
                    });
                }
                let coefficient = mul(a_coefficient, b_coefficient);
                let next = add(sum.get(&exponent).copied().unwrap_or(0), coefficient);
                if next == 0 {
                    sum.remove(&exponent);
                } else {
                    sum.insert(exponent, next);
                }
                peak_terms = peak_terms.max(sum.len());
                if sum.len() > max_accumulator_terms {
                    return Err(ReplayFailure {
                        blocker: "unit_certificate_product_accumulator_term_budget_exceeded",
                        accumulator_peak: peak_terms,
                    });
                }
            }
        }
    }
    let zero = [0u8; VARIABLES];
    Ok((sum.len() == 1 && sum.get(&zero) == Some(&1), peak_terms))
}

fn full_point_check(c: &[u64; 7], residual: (u64, u64)) -> bool {
    let (r, s) = residual;
    let mut ar = Arithmetic::default();
    let mut total = None;
    for &x in &XS {
        let z = add(add(mul(mul(x, x), x), mul(PRIME31.a, x)), PRIME31.b);
        let Some(y) = ar.sqrt(z, P) else {
            return false;
        };
        let ax = c[..4].iter().rev().fold(0, |v, &a| add(mul(v, x), a));
        let vx = [c[4], c[5], c[6], 1]
            .iter()
            .rev()
            .fold(0, |v, &a| add(mul(v, x), a));
        let f = |yy| add(mul(sub(x, r), ax), mul(add(yy, s), vx));
        let selected = match (f(y) == 0, f(sub(0, y)) == 0) {
            (true, false) => Some((x, y)),
            (false, true) => Some((x, sub(0, y))),
            _ => return false,
        };
        total = ar.add(PRIME31, total, selected);
    }
    total == Some(residual)
}
fn run(case: usize) -> Value {
    let started = Instant::now();
    let residual = RESIDUALS[case];
    check_curve(PRIME31, &mut Arithmetic::default()).unwrap();
    let h = h_base();
    let q = quadrics(residual.0, residual.1, &h);
    let canonical_input_quadrics: Vec<Vec<CanonicalTerm>> = q.iter().map(canonical_poly).collect();
    let input_system = canonical_input_system(case, &h, &q);
    let input_system_bytes = serde_json::to_vec(&input_system).unwrap();
    let input_system_sha256 = sha256_hex(&input_system_bytes);

    let (ring, weights) = radix_ring();
    let inputs: Vec<_> = q
        .iter()
        .map(|g| {
            let raw: Vec<_> = g
                .iter()
                .map(|(e, &c)| (embedded_raw_exponents(e), c))
                .collect();
            ring.from_raw(&raw)
        })
        .collect();
    let embedded_inputs = inputs
        .iter()
        .map(canonical_input_rpoly)
        .collect::<Result<Vec<_>, _>>();
    let input_embedding_roundtrip = embedded_inputs
        .as_ref()
        .is_ok_and(|got| got == &canonical_input_quadrics);

    let mut opts = TowerF4Options::new(9)
        .with_budget(Duration::from_secs(60))
        .with_max_nnz(3_000_000)
        .with_max_dense(1_000_000);
    let requested_open_top_mask = open_top_pair_omission_mask();
    let radix_layout_valid = radix_layout_valid(&ring, weights, requested_open_top_mask);
    opts = opts
        .with_weights(weights)
        .omitting_zero_top_pairs(requested_open_top_mask);
    opts.record_trace = true;
    let f4_start = Instant::now();
    let rep = f4_tower(&inputs, &ring, &opts);
    let f4_ns = f4_start.elapsed().as_nanos();

    let prepared_basis = prepare_basis_evidence(&rep.basis, 9);
    let basis_polynomial_count = prepared_basis.polynomial_count;
    let basis_term_count = prepared_basis.term_count;
    let basis_within_limits = prepared_basis.within_output_limits;
    let basis_encoding_result = prepared_basis.encoding_result;
    let basis_blocker = prepared_basis.canonical_polynomials.as_ref().err().copied();
    let basis_polynomials = prepared_basis.canonical_polynomials.unwrap_or_default();
    let decoded_basis_counts_match = basis_within_limits
        && basis_polynomials.len() == basis_polynomial_count
        && basis_polynomials.iter().map(Vec::len).sum::<usize>() == basis_term_count;
    let basis_materialization_valid = basis_blocker.is_none() && decoded_basis_counts_match;
    let trace_degrees_bounded = rep.trace.iter().all(|step| step.degree <= 9)
        && rep
            .stopped_step
            .as_ref()
            .is_none_or(|step| step.degree <= 9)
        && rep.degree_reached <= 9
        && rep.solving_degree_max <= 9;
    let open_top_receipt_valid = rep.open_top_pair_omission_mask == requested_open_top_mask
        && rep.open_top_pair_omission_mask.count_ones() as usize == VARIABLES
        && rep.open_top_reductions == 0;
    let embedding_sound = radix_layout_valid
        && input_embedding_roundtrip
        && basis_encoding_result.is_ok()
        && trace_degrees_bounded
        && open_top_receipt_valid;
    let complete_basis = solver_basis_complete(
        rep.timed_out,
        rep.oversize,
        rep.pairs_above_bound,
        rep.staircase_at_stop.is_some(),
    );

    let linear = if basis_within_limits {
        linear_evidence(&rep.basis)
    } else {
        LinearEvidence {
            raw_rows: vec![],
            rref_rows: vec![],
            pivot_columns: vec![],
            coefficient_rank: 0,
            augmented_rank: 0,
            unique_solution: false,
            row_solution: None,
            blocker: Some("basis_evidence_output_limit_exceeded"),
        }
    };
    let candidate =
        if complete_basis && embedding_sound && !rep.inconsistent && linear.unique_solution {
            linear.row_solution
        } else {
            None
        };
    let direct_identity = candidate
        .as_ref()
        .map(|c| direct_m(c, residual.0, residual.1) == h);
    let basis_check = candidate
        .as_ref()
        .map(|c| basis_polynomials.iter().all(|g| eval_canonical(g, c) == 0));
    let point_check = candidate.as_ref().map(|c| full_point_check(c, residual));
    let positive_replay_passed =
        direct_identity == Some(true) && basis_check == Some(true) && point_check == Some(true);

    let negative_certificate_result: Result<CertificateSuccess, CertificateFailure> = (|| {
        if !rep.inconsistent {
            return Err(CertificateFailure {
                blocker: "solver_did_not_observe_a_unit_basis".into(),
                terms_observed: 0,
                accumulator_peak: 0,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: None,
            });
        }
        let Some(trace) = rep.learned.as_ref() else {
            return Err(CertificateFailure {
                blocker: "f4_refutation_trace_was_not_available".into(),
                terms_observed: 0,
                accumulator_peak: 0,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: None,
            });
        };
        let encoded = match trace.unit_certificate(&inputs, &ring, weights, 9) {
            Ok(encoded) => encoded,
            Err(blocker) => {
                let provenance = provenance_attempt(&blocker);
                let product_work = product_work_attempt(&blocker);
                let accumulator = exporter_accumulator_attempt(&blocker);
                return Err(CertificateFailure {
                    blocker,
                    terms_observed: 0,
                    accumulator_peak: accumulator.map_or(0, |observation| observation.value),
                    internal_provenance_terms_observed_lower_bound: provenance
                        .map_or(0, |observation| observation.value),
                    internal_provenance_terms_observation_kind: provenance
                        .map_or("not_observed", |observation| observation.kind),
                    internal_recomposition_product_work_observed_lower_bound: product_work
                        .map_or(0, |observation| observation.value),
                    internal_recomposition_product_work_observation_kind: product_work
                        .map_or("not_observed", |observation| observation.kind),
                    resource_reason: provenance
                        .map(|_| "internal_provenance_term_limit_exceeded")
                        .or_else(|| {
                            product_work
                                .map(|_| "internal_recomposition_product_work_limit_exceeded")
                        })
                        .or_else(|| {
                            accumulator.map(|_| "certificate_product_accumulator_limit_exceeded")
                        }),
                });
            }
        };
        let encoded_terms: usize = encoded.iter().map(RPoly::len).sum();
        if encoded.len() != q.len() {
            return Err(CertificateFailure {
                blocker: "unit_certificate_does_not_have_eight_multipliers".into(),
                terms_observed: encoded_terms,
                accumulator_peak: 0,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: None,
            });
        }
        if !certificate_within_output_limits(encoded_terms) {
            return Err(CertificateFailure {
                blocker: "unit_certificate_serialized_term_budget_exceeded".into(),
                terms_observed: encoded_terms,
                accumulator_peak: 0,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: Some("certificate_term_limit_exceeded"),
            });
        }
        let decoded: Vec<Poly> = encoded
            .iter()
            .map(|multiplier| {
                canonical_rpoly(multiplier, 7)
                    .map(|terms| {
                        terms
                            .into_iter()
                            .map(|term| (term.exponents, term.coefficient))
                            .collect()
                    })
                    .map_err(str::to_owned)
            })
            .collect::<Result<_, _>>()
            .map_err(|blocker| CertificateFailure {
                blocker,
                terms_observed: encoded_terms,
                accumulator_peak: 0,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: None,
            })?;
        let decoded_terms: usize = decoded.iter().map(Poly::len).sum();
        if decoded_terms != encoded_terms {
            return Err(CertificateFailure {
                blocker: "unit_certificate_decode_changed_term_count".into(),
                terms_observed: decoded_terms,
                accumulator_peak: 0,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: None,
            });
        }
        match replay_unit_identity(&q, &decoded, 9, MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS) {
            Ok((true, accumulator_peak)) => Ok(CertificateSuccess {
                multipliers: decoded,
                terms_observed: decoded_terms,
                accumulator_peak,
            }),
            Ok((false, accumulator_peak)) => Err(CertificateFailure {
                blocker: "exported_multipliers_do_not_replay_to_the_constant_one".into(),
                terms_observed: decoded_terms,
                accumulator_peak,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: None,
            }),
            Err(failure) => Err(CertificateFailure {
                blocker: failure.blocker.into(),
                terms_observed: decoded_terms,
                accumulator_peak: failure.accumulator_peak,
                internal_provenance_terms_observed_lower_bound: 0,
                internal_provenance_terms_observation_kind: "not_observed",
                internal_recomposition_product_work_observed_lower_bound: 0,
                internal_recomposition_product_work_observation_kind: "not_observed",
                resource_reason: (failure.blocker
                    == "unit_certificate_product_accumulator_term_budget_exceeded")
                    .then_some("certificate_product_accumulator_limit_exceeded"),
            }),
        }
    })();
    let negative_certificate_replayed = negative_certificate_result.is_ok();
    let negative_certificate_blocker = negative_certificate_result
        .as_ref()
        .err()
        .map(|failure| failure.blocker.clone());
    let certificate_terms_observed = match &negative_certificate_result {
        Ok(success) => success.terms_observed,
        Err(failure) => failure.terms_observed,
    };
    let certificate_product_accumulator_peak = match &negative_certificate_result {
        Ok(success) => success.accumulator_peak,
        Err(failure) => failure.accumulator_peak,
    };
    let internal_provenance_terms_observed_lower_bound = negative_certificate_result
        .as_ref()
        .err()
        .map_or(0, |failure| {
            failure.internal_provenance_terms_observed_lower_bound
        });
    let internal_provenance_terms_observation_kind = negative_certificate_result
        .as_ref()
        .err()
        .map_or("not_observed", |failure| {
            failure.internal_provenance_terms_observation_kind
        });
    let internal_recomposition_product_work_observed_lower_bound = negative_certificate_result
        .as_ref()
        .err()
        .map_or(0, |failure| {
            failure.internal_recomposition_product_work_observed_lower_bound
        });
    let internal_recomposition_product_work_observation_kind = negative_certificate_result
        .as_ref()
        .err()
        .map_or("not_observed", |failure| {
            failure.internal_recomposition_product_work_observation_kind
        });
    let certificate_within_limits = certificate_within_output_limits(certificate_terms_observed)
        && negative_certificate_blocker.as_deref()
            != Some("unit_certificate_serialized_term_budget_exceeded");
    let certificate_product_accumulator_within_limit = certificate_product_accumulator_peak
        <= MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS
        && negative_certificate_blocker.as_deref()
            != Some("unit_certificate_product_accumulator_term_budget_exceeded");
    let internal_provenance_within_limit =
        internal_provenance_terms_observed_lower_bound <= MAX_INTERNAL_PROVENANCE_TERMS;
    let internal_recomposition_product_work_within_limit =
        internal_recomposition_product_work_observed_lower_bound
            <= MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK;
    let all_pre_serialization_limits_satisfied = basis_within_limits
        && certificate_within_limits
        && certificate_product_accumulator_within_limit
        && internal_provenance_within_limit
        && internal_recomposition_product_work_within_limit;
    let resource_reason = resource_limit_reason(
        basis_polynomial_count,
        basis_term_count,
        certificate_terms_observed,
        certificate_product_accumulator_peak,
        internal_provenance_terms_observed_lower_bound,
        internal_recomposition_product_work_observed_lower_bound,
    )
    .or_else(|| {
        negative_certificate_result
            .as_ref()
            .err()
            .and_then(|failure| failure.resource_reason)
    });
    let negative_unit_identity =
        negative_certificate_result
            .as_ref()
            .map_or(Value::Null, |success| {
                let canonical_multipliers: Vec<Vec<CanonicalTerm>> =
                    success.multipliers.iter().map(canonical_poly).collect();
                json!({
                    "schema": "prime31.rr9.full8-unit-identity.v1",
                    "case": case,
                    "public_residual": [residual.0, residual.1],
                    "input_system_sha256": input_system_sha256,
                    "multipliers": canonical_multipliers,
                })
            });
    let evidence_ready = evidence_ready_for_admission(
        complete_basis,
        embedding_sound,
        all_pre_serialization_limits_satisfied,
        basis_materialization_valid,
    );
    let admissible_positive = case == 0
        && evidence_ready
        && !rep.inconsistent
        && candidate.is_some()
        && positive_replay_passed;
    let admissible_negative =
        case != 0 && evidence_ready && rep.inconsistent && negative_certificate_replayed;
    let outcome = if resource_reason.is_some() {
        "resource_censored"
    } else if rep.timed_out {
        "censored_timeout"
    } else if rep.oversize {
        "censored_oversize"
    } else if rep.staircase_at_stop.is_some() {
        "censored_staircase_stop"
    } else if rep.pairs_above_bound != 0 {
        "censored_degree_bound"
    } else if !embedding_sound {
        "invalid_radix_embedding_evidence"
    } else if !basis_materialization_valid {
        "invalid_basis_evidence"
    } else if rep.inconsistent && admissible_negative {
        "negative_verified"
    } else if rep.inconsistent && negative_certificate_replayed {
        "positive_control_refuted"
    } else if rep.inconsistent {
        "negative_observation_certificate_unavailable"
    } else if candidate.is_none() {
        "complete_without_unique_affine_solution"
    } else if !positive_replay_passed {
        "candidate_replay_failed"
    } else if case == 0 {
        "positive_verified"
    } else {
        "negative_control_has_solution"
    };
    let status = if resource_reason.is_some() {
        "resource_censored"
    } else if admissible_positive || admissible_negative {
        "admissible"
    } else if outcome.starts_with("censored_") {
        "censored"
    } else {
        "inadmissible"
    };
    let tower_weights: Vec<u32> = (0..VARIABLES * RADIX_LEVELS)
        .map(|j| weights.weight(j as u32))
        .collect();
    let blocks: Vec<Vec<usize>> = (0..VARIABLES)
        .map(|variable| {
            (0..RADIX_LEVELS)
                .map(|level| variable * RADIX_LEVELS + level)
                .collect()
        })
        .collect();
    let carry_rules: Vec<Value> = (0..VARIABLES)
        .flat_map(|variable| {
            (0..RADIX_LEVELS).map(move |level| {
                let tower_index = variable * RADIX_LEVELS + level;
                json!({
                    "tower_index": tower_index,
                    "ordinary_variable": variable,
                    "level": level,
                    "weight": 1usize << level,
                    "square_maps_to_tower_index": (level + 1 < RADIX_LEVELS)
                        .then_some(tower_index + 1),
                    "top_square_maps_to_zero": level + 1 == RADIX_LEVELS,
                })
            })
        })
        .collect();
    let trace_available = rep.learned.is_some();
    let trace_kept_rows = rep.learned.as_ref().map(|trace| trace.kept_rows());
    let linear_blocker = if !complete_basis || !embedding_sound {
        Some("basis_or_radix_embedding_is_incomplete")
    } else {
        linear.blocker
    };
    let linear_extraction = if basis_within_limits {
        json!({
            "schema": "prime31.rr9.full8-linear-extraction.v1",
            "raw_rows": linear.raw_rows,
            "rref_rows": linear.rref_rows,
            "pivot_columns": linear.pivot_columns,
            "coefficient_rank": linear.coefficient_rank,
            "augmented_rank": linear.augmented_rank,
            "unique_solution": linear.unique_solution,
            "recovered_coefficients": candidate,
            "blocker": linear_blocker,
        })
    } else {
        json!({
            "schema": "prime31.rr9.full8-linear-extraction.v1",
            "raw_rows": [], "rref_rows": [], "pivot_columns": [],
            "coefficient_rank": 0, "augmented_rank": 0,
            "unique_solution": false, "recovered_coefficients": Value::Null,
            "blocker": "basis_evidence_output_limit_exceeded",
        })
    };
    let basis_evidence = if basis_within_limits {
        json!({
            "schema": "prime31.rr9.full8-basis-evidence.v1",
            "polynomial_count": basis_polynomial_count,
            "term_count": basis_term_count,
            "canonical_polynomials": basis_polynomials,
            "blocker": basis_blocker,
        })
    } else {
        json!({
            "schema": "prime31.rr9.full8-basis-evidence.v1",
            "polynomial_count": 0, "term_count": 0,
            "canonical_polynomials": [],
            "blocker": "basis_evidence_output_limit_exceeded",
        })
    };
    let document = json!({
        "schema": "prime31.rr9.full8-f4-screen.v2", "case": case,
        "public_residual": [residual.0, residual.1], "base_x": XS,
        "p": P, "curve_a": PRIME31.a, "curve_b": PRIME31.b, "n": PRIME31.n,
        "h8_coefficients": h, "input_quadrics": 8,
        "input_system": input_system,
        "input_system_hash_encoding": "sha256(compact serde_json UTF-8 of emitted input_system; object keys lexicographic)",
        "input_system_canonical_json_bytes": input_system_bytes.len(),
        "input_system_sha256": input_system_sha256,
        "no_sign_mask_or_secret_scalar_passed": true,
        "f4_wall_ns": f4_ns.to_string(), "full_wall_ns": started.elapsed().as_nanos().to_string(),
        "status": status, "outcome": outcome,
        "admissible_positive": admissible_positive,
        "admissible_negative": admissible_negative,
        "complete_basis": complete_basis, "inconsistent": rep.inconsistent,
        "timed_out": rep.timed_out, "oversize": rep.oversize,
        "pairs_above_bound": rep.pairs_above_bound,
        "staircase_at_stop": rep.staircase_at_stop,
        "basis_size": rep.basis.len(), "degree_reached": rep.degree_reached,
        "solving_degree_max": rep.solving_degree_max,
        "max_rows": rep.max_rows, "max_cols": rep.max_cols,
        "max_nnz": rep.max_nnz, "max_dense_entries": rep.max_dense_entries,
        "muladds": rep.muladds, "steps": rep.steps,
        "evidence_limits": {
            "schema": "prime31.rr9.full8-evidence-limits.v1",
            "max_final_json_bytes": MAX_FINAL_JSON_BYTES,
            "max_basis_polynomials": MAX_BASIS_POLYNOMIALS,
            "max_basis_terms": MAX_BASIS_TERMS,
            "max_certificate_terms": MAX_SERIALIZED_CERTIFICATE_TERMS,
            "max_certificate_product_accumulator_terms": MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
            "max_internal_provenance_terms": MAX_INTERNAL_PROVENANCE_TERMS,
            "max_internal_recomposition_product_work": MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK,
            "basis_polynomials_observed": basis_polynomial_count,
            "basis_terms_observed": basis_term_count,
            "certificate_terms_observed": certificate_terms_observed,
            "certificate_product_accumulator_peak": certificate_product_accumulator_peak,
            "internal_provenance_terms_observed_lower_bound": internal_provenance_terms_observed_lower_bound,
            "internal_provenance_terms_observation_kind": internal_provenance_terms_observation_kind,
            "internal_recomposition_product_work_observed_lower_bound": internal_recomposition_product_work_observed_lower_bound,
            "internal_recomposition_product_work_observation_kind": internal_recomposition_product_work_observation_kind,
            "basis_within_limits": basis_within_limits,
            "certificate_within_limits": certificate_within_limits,
            "certificate_product_accumulator_within_limit": certificate_product_accumulator_within_limit,
            "internal_provenance_within_limit": internal_provenance_within_limit,
            "internal_recomposition_product_work_within_limit": internal_recomposition_product_work_within_limit,
            "all_pre_serialization_limits_satisfied": all_pre_serialization_limits_satisfied,
            "final_json_size_checked_before_write": true,
            "final_json_within_limit": true,
            "attempted_final_pretty_json_bytes": Value::Null,
        },
        "radix_embedding": {
            "schema": "prime31.rr9.full8-radix-embedding.v1",
            "ordinary_variables": VARIABLES,
            "tower_variables": VARIABLES * RADIX_LEVELS,
            "free_variables": 0,
            "levels_per_variable": RADIX_LEVELS,
            "blocks": blocks,
            "level_weights": [1, 2, 4, 8],
            "tower_weights": tower_weights,
            "carry_rules": carry_rules,
            "artificial_top_relation_exponent": RADIX_CUTOFF_DEGREE,
            "frozen_max_weighted_degree": 9,
            "requested_open_top_pair_omission_mask": requested_open_top_mask,
            "open_top_pair_omission_mask": rep.open_top_pair_omission_mask,
            "open_top_rules_selected": rep.open_top_pair_omission_mask.count_ones(),
            "open_top_pairs_omitted": rep.open_top_pairs_omitted,
            "open_top_reductions": rep.open_top_reductions,
            "open_top_receipt_valid": open_top_receipt_valid,
            "exact_block_rule_weight_layout_valid": radix_layout_valid,
            "input_embedding_roundtrip": input_embedding_roundtrip,
            "all_reported_step_degrees_bounded": trace_degrees_bounded,
            "all_basis_monomials_decoded_below_top_relation": basis_encoding_result.is_ok(),
            "top_rule_reductions_excluded_by_degree_bound": trace_degrees_bounded
                && rep.open_top_reductions == 0,
            "admissible": embedding_sound,
        },
        "linear_extraction": linear_extraction,
        "basis_evidence": basis_evidence,
        "recovered_coefficients": candidate,
        "original_eight_coefficients_checked": direct_identity,
        "candidate_vanishes_on_f4_basis": basis_check,
        "full_signed_point_sum_checked": point_check,
        "positive_replay_passed": positive_replay_passed,
        "negative_unit_identity": negative_unit_identity,
        "negative_certificate": {
            "schema": "prime31.rr9.full8-negative-certificate.v1",
            "available": negative_certificate_replayed,
            "replayed": negative_certificate_replayed,
            "internal_trace_available": trace_available,
            "internal_trace_kept_rows": trace_kept_rows,
            "blocker": negative_certificate_blocker,
        },
        "resource_refusal": Value::Null,
        "pcc": "unverified"
    });
    if let Some(reason) = resource_reason {
        resource_censored_document(document, reason, None)
    } else {
        document
    }
}

fn resource_censored_document(
    document: Value,
    reason: &'static str,
    attempted_final_pretty_json_bytes: Option<usize>,
) -> Value {
    let mut limits = document["evidence_limits"].clone();
    let limits_object = limits.as_object_mut().expect("evidence_limits object");
    limits_object.insert(
        "attempted_final_pretty_json_bytes".into(),
        attempted_final_pretty_json_bytes.map_or(Value::Null, |value| json!(value)),
    );
    limits_object.insert(
        "final_json_within_limit".into(),
        json!(attempted_final_pretty_json_bytes.is_none()),
    );
    let resource_refusal = json!({
        "schema": "prime31.rr9.full8-resource-refusal.v1",
        "reason": reason,
        "basis_polynomials_observed": limits["basis_polynomials_observed"],
        "basis_terms_observed": limits["basis_terms_observed"],
        "certificate_terms_observed": limits["certificate_terms_observed"],
        "certificate_product_accumulator_peak": limits["certificate_product_accumulator_peak"],
        "internal_provenance_terms_observed_lower_bound": limits["internal_provenance_terms_observed_lower_bound"],
        "internal_provenance_terms_observation_kind": limits["internal_provenance_terms_observation_kind"],
        "internal_recomposition_product_work_observed_lower_bound": limits["internal_recomposition_product_work_observed_lower_bound"],
        "internal_recomposition_product_work_observation_kind": limits["internal_recomposition_product_work_observation_kind"],
        "attempted_final_pretty_json_bytes": attempted_final_pretty_json_bytes,
        "suppressed_evidence": true,
    });
    json!({
        "schema": "prime31.rr9.full8-f4-screen.v2",
        "case": document["case"],
        "public_residual": document["public_residual"],
        "base_x": document["base_x"],
        "p": document["p"],
        "curve_a": document["curve_a"],
        "curve_b": document["curve_b"],
        "n": document["n"],
        "h8_coefficients": document["h8_coefficients"],
        "input_quadrics": document["input_quadrics"],
        "input_system": document["input_system"],
        "input_system_hash_encoding": document["input_system_hash_encoding"],
        "input_system_canonical_json_bytes": document["input_system_canonical_json_bytes"],
        "input_system_sha256": document["input_system_sha256"],
        "no_sign_mask_or_secret_scalar_passed": document["no_sign_mask_or_secret_scalar_passed"],
        "status": "resource_censored",
        "outcome": "resource_censored",
        "admissible_positive": false,
        "admissible_negative": false,
        "complete_basis": document["complete_basis"],
        "inconsistent": document["inconsistent"],
        "timed_out": document["timed_out"],
        "oversize": document["oversize"],
        "pairs_above_bound": document["pairs_above_bound"],
        "staircase_at_stop": document["staircase_at_stop"],
        "degree_reached": document["degree_reached"],
        "solving_degree_max": document["solving_degree_max"],
        "radix_embedding": document["radix_embedding"],
        "evidence_limits": limits,
        "resource_refusal": resource_refusal,
        "pcc": "unverified",
    })
}

#[derive(Default)]
struct CountingWriter {
    bytes: usize,
}

impl std::io::Write for CountingWriter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.bytes = self.bytes.checked_add(buffer.len()).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::FileTooLarge, "JSON byte count overflow")
        })?;
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn pretty_json_stdout_bytes(document: &Value) -> usize {
    let mut counter = CountingWriter::default();
    serde_json::to_writer_pretty(&mut counter, document).expect("count JSON serialization");
    counter
        .bytes
        .checked_add(1)
        .expect("JSON newline byte count overflow")
}

fn bounded_pretty_json(document: Value, case: usize, max_bytes: usize) -> String {
    let attempted_bytes = pretty_json_stdout_bytes(&document);
    if attempted_bytes <= max_bytes {
        return serde_json::to_string_pretty(&document).unwrap();
    }
    debug_assert_eq!(document["case"], json!(case));
    let fallback =
        resource_censored_document(document, "final_json_limit_exceeded", Some(attempted_bytes));
    let fallback_bytes = pretty_json_stdout_bytes(&fallback);
    assert!(
        fallback_bytes <= max_bytes,
        "bounded refusal JSON itself exceeds the output cap"
    );
    serde_json::to_string_pretty(&fallback).unwrap()
}

fn main() {
    let case: usize = std::env::args()
        .nth(1)
        .expect("case 0, 1 or 2")
        .parse()
        .unwrap();
    assert!(case < 3);
    println!(
        "{}",
        bounded_pretty_json(run(case), case, MAX_FINAL_JSON_BYTES)
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    fn exponents_of_degree(degree: u8) -> Vec<Exp> {
        fn visit(variable: usize, remaining: u8, current: &mut Exp, out: &mut Vec<Exp>) {
            if variable + 1 == VARIABLES {
                current[variable] = remaining;
                out.push(*current);
                return;
            }
            for exponent in 0..=remaining {
                current[variable] = exponent;
                visit(variable + 1, remaining - exponent, current, out);
            }
        }
        let mut out = Vec::new();
        visit(0, degree, &mut [0u8; VARIABLES], &mut out);
        out
    }

    fn ordinary_grevlex(a: &Exp, b: &Exp) -> std::cmp::Ordering {
        let da: u16 = a.iter().map(|&x| u16::from(x)).sum();
        let db: u16 = b.iter().map(|&x| u16::from(x)).sum();
        da.cmp(&db).then_with(|| {
            for i in (0..VARIABLES).rev() {
                match a[i].cmp(&b[i]) {
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Less,
                }
            }
            std::cmp::Ordering::Equal
        })
    }

    fn weighted_mono(exponents: &Exp, weights: TowerWeights) -> u128 {
        let mut mask = 0u64;
        for (variable, &exponent) in exponents.iter().enumerate() {
            assert!(u32::from(exponent) < RADIX_CUTOFF_DEGREE);
            for level in 0..RADIX_LEVELS {
                if exponent >> level & 1 == 1 {
                    mask |= 1u64 << (variable * RADIX_LEVELS + level);
                }
            }
        }
        mono_w(mask, [0; 4], weights)
    }

    #[test]
    fn public_quadrics_match_direct_norm() {
        let h = h_base();
        for &(r, s) in &RESIDUALS {
            let q = quadrics(r, s, &h);
            for seed in 0..8u64 {
                let c: [u64; 7] = std::array::from_fn(|j| (seed * 71 + j as u64 * 29 + 3) % P);
                let m = direct_m(&c, r, s);
                assert_eq!(m[8], 1);
                for i in 0..8 {
                    assert_eq!(eval(&q[i], &c), sub(m[i], h[i]));
                }
            }
        }
    }

    #[test]
    fn sha256_and_canonical_input_hashes_match_independent_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let h = h_base();
        assert_eq!(
            h,
            vec![
                0,
                2_147_470_687,
                31_572,
                2_147_454_631,
                13_319,
                2_147_480_297,
                468,
                2_147_483_613,
                1,
            ]
        );
        let expected = [
            "05ede9c68813a0e3ef4da239a2adb5bcbfaa7e563b56a6c42439b29be1bf0707",
            "90ea3b65be87c566c1e141927d143f3fd46e3b12807d034805bfb19f8ba49b2c",
            "4a16e53d8a3efe2dc874661a9fc568ed526beeb42d818981bd48ebebe3699755",
        ];
        for case in 0..3 {
            let (r, s) = RESIDUALS[case];
            let q = quadrics(r, s, &h);
            let system = canonical_input_system(case, &h, &q);
            assert_eq!(
                sha256_hex(&serde_json::to_vec(&system).unwrap()),
                expected[case]
            );
        }
    }

    #[test]
    fn radix_blocks_round_trip_and_preserve_bounded_products() {
        let (mut ring, weights) = radix_ring();
        let open_top_mask = open_top_pair_omission_mask();
        assert!(radix_layout_valid(&ring, weights, open_top_mask));
        assert_eq!(ring.n_vars(), VARIABLES * RADIX_LEVELS);
        assert_eq!(ring.n_free, 0);
        for variable in 0..VARIABLES {
            for exponent in 0..RADIX_CUTOFF_DEGREE as u8 {
                let mut e = [0u8; VARIABLES];
                e[variable] = exponent;
                let mono = weighted_mono(&e, weights);
                assert_eq!(decode_radix_mask(mono), Ok(e));
                assert_eq!(degree_of(mono), u32::from(exponent));
            }
        }
        let products = [
            ([3, 0, 0, 0, 0, 0, 0], [3, 0, 0, 0, 0, 0, 0]),
            ([5, 0, 0, 0, 0, 0, 0], [4, 0, 0, 0, 0, 0, 0]),
            ([2, 1, 1, 0, 0, 0, 0], [1, 2, 0, 1, 1, 0, 0]),
        ];
        for (a, b) in products {
            let want: Exp = std::array::from_fn(|i| a[i] + b[i]);
            assert!(want.iter().map(|&x| u32::from(x)).sum::<u32>() <= 9);
            let g = RPoly {
                monos: vec![weighted_mono(&a, weights)],
                coefs: vec![1],
            };
            let got = ring.mul_mono_w(&g, weighted_mono(&b, weights), weights);
            assert_eq!(got.coefs, vec![1]);
            assert_eq!(decode_weighted_monomial(got.monos[0], 9), Ok(want));
        }
        let mut eight = [0u8; VARIABLES];
        eight[0] = 8;
        let g = RPoly {
            monos: vec![weighted_mono(&eight, weights)],
            coefs: vec![1],
        };
        assert!(
            ring.mul_mono_w(&g, weighted_mono(&eight, weights), weights)
                .is_empty(),
            "the artificial x^16=0 boundary must never be processed by the bounded run"
        );

        ring.rules[0].b = 0;
        assert!(!radix_layout_valid(&ring, weights, open_top_mask));
        ring.rules[0].b = 1;
        let mut wrong_weights = vec![1u32; VARIABLES * RADIX_LEVELS];
        for variable in 0..VARIABLES {
            wrong_weights[variable * RADIX_LEVELS..(variable + 1) * RADIX_LEVELS]
                .copy_from_slice(&[1, 2, 4, 8]);
        }
        wrong_weights[0] = 2;
        assert!(!radix_layout_valid(
            &ring,
            TowerWeights::from_slice(&wrong_weights),
            open_top_mask,
        ));
    }

    #[test]
    fn radix_embedding_exhausts_degree_nine_order_and_products() {
        let (ring, weights) = radix_ring();
        let by_degree: Vec<Vec<Exp>> = (0..=9).map(exponents_of_degree).collect();
        assert_eq!(by_degree.iter().map(Vec::len).sum::<usize>(), 11_440);

        let mut ordinary: Vec<Exp> = by_degree.iter().flatten().copied().collect();
        ordinary.sort_by(|a, b| ordinary_grevlex(b, a));
        let mut encoded: Vec<(u128, Exp)> = ordinary
            .iter()
            .copied()
            .map(|e| (weighted_mono(&e, weights), e))
            .collect();
        encoded.sort_by_key(|(mono, _)| std::cmp::Reverse(*mono));
        assert_eq!(
            encoded.iter().map(|(_, e)| *e).collect::<Vec<_>>(),
            ordinary
        );
        for &(mono, exponent) in &encoded {
            assert_eq!(decode_weighted_monomial(mono, 9), Ok(exponent));
        }

        let mut checked = 0usize;
        for a_degree in 0..=9 {
            for b_degree in 0..=9 - a_degree {
                for a in &by_degree[a_degree] {
                    let g = RPoly {
                        monos: vec![weighted_mono(a, weights)],
                        coefs: vec![1],
                    };
                    for b in &by_degree[b_degree] {
                        let want: Exp = std::array::from_fn(|i| a[i] + b[i]);
                        let got = ring.mul_mono_w(&g, weighted_mono(b, weights), weights);
                        assert_eq!(got.coefs, vec![1]);
                        assert_eq!(got.monos.len(), 1);
                        assert_eq!(decode_weighted_monomial(got.monos[0], 9), Ok(want));
                        checked += 1;
                    }
                }
            }
        }
        assert_eq!(checked, 817_190);
    }

    #[test]
    fn linear_rows_expose_rank_solution_and_inconsistency() {
        let (_, weights) = radix_ring();
        let solution = [3, 5, 7, 11, 13, 17, 19];
        let mut basis = Vec::new();
        for (variable, &constant) in solution.iter().enumerate() {
            let mut exponent = [0u8; VARIABLES];
            exponent[variable] = 1;
            basis.push(RPoly {
                monos: vec![weighted_mono(&exponent, weights), ONE],
                coefs: vec![1, sub(0, constant) as u32],
            });
        }
        let evidence = linear_evidence(&basis);
        assert_eq!(evidence.coefficient_rank, 7);
        assert_eq!(evidence.augmented_rank, 7);
        assert_eq!(evidence.pivot_columns, (0..7).collect::<Vec<_>>());
        assert!(evidence.unique_solution);
        assert_eq!(evidence.row_solution, Some(solution));
        assert_eq!(evidence.blocker, None);

        basis.push(RPoly {
            monos: vec![ONE],
            coefs: vec![1],
        });
        let inconsistent = linear_evidence(&basis);
        assert_eq!(inconsistent.coefficient_rank, 7);
        assert_eq!(inconsistent.augmented_rank, 8);
        assert!(!inconsistent.unique_solution);
        assert_eq!(inconsistent.row_solution, None);
        assert_eq!(inconsistent.blocker, Some("affine_rows_are_inconsistent"));
    }

    #[test]
    fn ordinary_unit_identity_replay_rejects_mutations_and_degree_overflow() {
        let mut quadrics: [Poly; 8] = std::array::from_fn(|_| Poly::new());
        let mut x = [0u8; VARIABLES];
        x[0] = 1;
        quadrics[0].insert(x, 1);
        quadrics[1].insert(x, 1);
        quadrics[1].insert([0; VARIABLES], 1);
        let mut multipliers = vec![Poly::new(); 8];
        multipliers[0].insert([0; VARIABLES], P - 1);
        multipliers[1].insert([0; VARIABLES], 1);
        assert_eq!(
            replay_unit_identity(
                &quadrics,
                &multipliers,
                9,
                MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
            ),
            Ok((true, 2)),
        );

        multipliers[1].insert([0; VARIABLES], 2);
        assert_eq!(
            replay_unit_identity(
                &quadrics,
                &multipliers,
                9,
                MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
            )
            .map(|(valid, _)| valid),
            Ok(false),
        );
        multipliers[1].insert([0; VARIABLES], 1);
        multipliers.swap(0, 1);
        assert_eq!(
            replay_unit_identity(
                &quadrics,
                &multipliers,
                9,
                MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
            )
            .map(|(valid, _)| valid),
            Ok(false),
        );
        multipliers.swap(0, 1);
        multipliers[1].clear();
        multipliers[1].insert(x, 1);
        assert_eq!(
            replay_unit_identity(
                &quadrics,
                &multipliers,
                9,
                MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
            )
            .map(|(valid, _)| valid),
            Ok(false),
        );

        quadrics = std::array::from_fn(|_| Poly::new());
        multipliers = vec![Poly::new(); 8];
        let mut degree_eight = [0u8; VARIABLES];
        degree_eight[0] = 8;
        multipliers[0].insert(degree_eight, 1);
        let mut quadratic = [0u8; VARIABLES];
        quadratic[0] = 2;
        quadrics[0].clear();
        quadrics[0].insert(quadratic, 1);
        assert_eq!(
            replay_unit_identity(
                &quadrics,
                &multipliers,
                9,
                MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
            ),
            Err(ReplayFailure {
                blocker: "unit_certificate_product_exceeds_frozen_degree_bound",
                accumulator_peak: 0,
            })
        );
    }

    #[test]
    fn exporter_budget_attempts_preserve_exact_counts_and_lower_bound_fallbacks() {
        assert_eq!(
            provenance_attempt(
                "unit certificate aggregate provenance term budget exceeded: attempted 3000123, limit 3000000"
            ),
            Some(AttemptObservation::exact(3_000_123)),
        );
        assert_eq!(
            provenance_attempt(
                "unit certificate provenance term budget exceeded: attempted at least 3000001, limit 3000000"
            ),
            Some(AttemptObservation::lower_bound(3_000_001)),
        );
        assert_eq!(
            provenance_attempt("unit certificate aggregate provenance term count overflow"),
            Some(AttemptObservation::lower_bound(3_000_001)),
        );
        assert_eq!(
            product_work_attempt(
                "unit certificate internal recomposition product-work budget exceeded: attempted 2000456, limit 2000000"
            ),
            Some(AttemptObservation::exact(2_000_456)),
        );
        assert_eq!(
            product_work_attempt(
                "unit certificate internal recomposition product-work budget exceeded: attempted at least 2000001, limit 2000000"
            ),
            Some(AttemptObservation::lower_bound(2_000_001)),
        );
        assert_eq!(
            product_work_attempt(
                "unit certificate internal recomposition product-work budget exceeded: attempted at least 2000042, limit 2000000"
            ),
            Some(AttemptObservation::lower_bound(2_000_001)),
        );
        assert_eq!(
            product_work_attempt(
                "unit certificate internal recomposition product-work budget exceeded: malformed"
            ),
            Some(AttemptObservation::lower_bound(2_000_001)),
        );
        assert_eq!(
            product_work_attempt(
                "unit certificate internal recomposition product-work budget exceeded: attempted 2000456, limit 7"
            ),
            Some(AttemptObservation::lower_bound(2_000_001)),
        );
        assert_eq!(
            exporter_accumulator_attempt(
                "unit certificate recomposition accumulator-term budget exceeded: attempted 2000042, limit 2000000"
            ),
            Some(AttemptObservation::exact(2_000_042)),
        );
        assert_eq!(provenance_attempt("unrelated exporter failure"), None);
    }

    #[test]
    fn evidence_limits_accept_boundaries_and_refuse_oversized_outputs() {
        assert!(basis_within_output_limits(
            MAX_BASIS_POLYNOMIALS,
            MAX_BASIS_TERMS,
        ));
        assert!(!basis_within_output_limits(
            MAX_BASIS_POLYNOMIALS + 1,
            MAX_BASIS_TERMS,
        ));
        assert!(!basis_within_output_limits(
            MAX_BASIS_POLYNOMIALS,
            MAX_BASIS_TERMS + 1,
        ));
        assert!(certificate_within_output_limits(
            MAX_SERIALIZED_CERTIFICATE_TERMS,
        ));
        assert!(!certificate_within_output_limits(
            MAX_SERIALIZED_CERTIFICATE_TERMS + 1,
        ));
        let at_limits = (
            MAX_BASIS_POLYNOMIALS,
            MAX_BASIS_TERMS,
            MAX_SERIALIZED_CERTIFICATE_TERMS,
            MAX_CERTIFICATE_PRODUCT_ACCUMULATOR_TERMS,
            MAX_INTERNAL_PROVENANCE_TERMS,
            MAX_INTERNAL_RECOMPOSITION_PRODUCT_WORK,
        );
        assert_eq!(
            resource_limit_reason(
                at_limits.0,
                at_limits.1,
                at_limits.2,
                at_limits.3,
                at_limits.4,
                at_limits.5,
            ),
            None,
        );
        assert_eq!(
            resource_limit_reason(
                at_limits.0 + 1,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX
            ),
            Some("basis_polynomial_limit_exceeded"),
        );
        assert!(solver_basis_complete(false, false, 0, false));
        let oversized_basis: Vec<RPoly> = (0..=MAX_BASIS_POLYNOMIALS)
            .map(|_| RPoly {
                monos: Vec::new(),
                coefs: Vec::new(),
            })
            .collect();
        let prepared = prepare_basis_evidence(&oversized_basis, 9);
        assert_eq!(prepared.polynomial_count, MAX_BASIS_POLYNOMIALS + 1);
        assert_eq!(prepared.term_count, 0);
        assert!(!prepared.within_output_limits);
        assert_eq!(prepared.encoding_result, Ok(()));
        assert_eq!(
            prepared.canonical_polynomials,
            Err("basis_evidence_output_limit_exceeded"),
        );
        let complete_with_suppressed_evidence = solver_basis_complete(false, false, 0, false);
        assert!(
            complete_with_suppressed_evidence,
            "the run's cap-independent basis validation must preserve completion",
        );
        assert!(prepared.encoding_result.is_ok());
        assert!(
            !evidence_ready_for_admission(complete_with_suppressed_evidence, false, true, true,),
            "invalid embedding evidence must block admission without rewriting complete_basis",
        );
        assert_eq!(
            resource_limit_reason(
                at_limits.0,
                at_limits.1 + 1,
                at_limits.2,
                at_limits.3,
                at_limits.4,
                at_limits.5
            ),
            Some("basis_term_limit_exceeded"),
        );
        assert_eq!(
            resource_limit_reason(
                at_limits.0,
                at_limits.1,
                at_limits.2 + 1,
                at_limits.3,
                at_limits.4,
                at_limits.5
            ),
            Some("certificate_term_limit_exceeded"),
        );
        assert_eq!(
            resource_limit_reason(
                at_limits.0,
                at_limits.1,
                at_limits.2,
                at_limits.3 + 1,
                at_limits.4,
                at_limits.5
            ),
            Some("certificate_product_accumulator_limit_exceeded"),
        );
        assert_eq!(
            resource_limit_reason(
                at_limits.0,
                at_limits.1,
                at_limits.2,
                at_limits.3,
                at_limits.4 + 1,
                at_limits.5
            ),
            Some("internal_provenance_term_limit_exceeded"),
        );
        assert_eq!(
            resource_limit_reason(
                at_limits.0,
                at_limits.1,
                at_limits.2,
                at_limits.3,
                at_limits.4,
                at_limits.5 + 1
            ),
            Some("internal_recomposition_product_work_limit_exceeded"),
        );

        let mut quadrics: [Poly; 8] = std::array::from_fn(|_| Poly::new());
        let mut x = [0u8; VARIABLES];
        x[0] = 1;
        quadrics[0].insert([0; VARIABLES], 1);
        quadrics[0].insert(x, 1);
        let mut multipliers = vec![Poly::new(); 8];
        multipliers[0].insert([0; VARIABLES], 1);
        assert_eq!(
            replay_unit_identity(&quadrics, &multipliers, 9, 1),
            Err(ReplayFailure {
                blocker: "unit_certificate_product_accumulator_term_budget_exceeded",
                accumulator_peak: 2,
            }),
        );

        let oversized = json!({
            "schema": "prime31.rr9.full8-f4-screen.v2",
            "case": 2,
            "complete_basis": complete_with_suppressed_evidence,
            "payload": "x".repeat(32_768),
            "evidence_limits": {
                "schema": "prime31.rr9.full8-evidence-limits.v1",
                "basis_polynomials_observed": prepared.polynomial_count,
                "basis_terms_observed": prepared.term_count,
                "certificate_terms_observed": 0,
                "certificate_product_accumulator_peak": 0,
                "internal_provenance_terms_observed_lower_bound": 0,
                "internal_provenance_terms_observation_kind": "not_observed",
                "internal_recomposition_product_work_observed_lower_bound": 0,
                "internal_recomposition_product_work_observation_kind": "not_observed",
                "attempted_final_pretty_json_bytes": Value::Null,
                "final_json_within_limit": true,
            },
        });
        assert_eq!(
            pretty_json_stdout_bytes(&oversized),
            serde_json::to_string_pretty(&oversized).unwrap().len() + 1,
        );
        let pre_refusal =
            resource_censored_document(oversized.clone(), "basis_polynomial_limit_exceeded", None);
        assert_eq!(pre_refusal["status"], "resource_censored");
        assert_eq!(pre_refusal["complete_basis"], true);
        assert_eq!(
            pre_refusal["resource_refusal"]["basis_polynomials_observed"],
            MAX_BASIS_POLYNOMIALS + 1,
        );
        assert_eq!(
            pre_refusal["resource_refusal"]["attempted_final_pretty_json_bytes"],
            Value::Null,
        );
        assert_eq!(
            pre_refusal["resource_refusal"]["internal_provenance_terms_observation_kind"],
            "not_observed",
        );
        assert_eq!(
            pre_refusal["resource_refusal"]["internal_recomposition_product_work_observation_kind"],
            "not_observed",
        );
        let mut exact_attempt = oversized.clone();
        exact_attempt["evidence_limits"]["internal_provenance_terms_observed_lower_bound"] =
            json!(3_000_123);
        exact_attempt["evidence_limits"]["internal_provenance_terms_observation_kind"] =
            json!("exact");
        let exact_refusal = resource_censored_document(
            exact_attempt,
            "internal_provenance_term_limit_exceeded",
            None,
        );
        assert_eq!(
            exact_refusal["resource_refusal"]["internal_provenance_terms_observed_lower_bound"],
            3_000_123,
        );
        assert_eq!(
            exact_refusal["resource_refusal"]["internal_provenance_terms_observation_kind"],
            "exact",
        );
        let encoded = bounded_pretty_json(oversized, 2, 8192);
        assert!(
            encoded.len() + 1 <= 8192,
            "trailing newline is part of the cap"
        );
        let refusal: Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(refusal["status"], "resource_censored");
        assert_eq!(refusal["outcome"], "resource_censored");
        assert_eq!(refusal["complete_basis"], true);
        assert_eq!(
            refusal["resource_refusal"]["reason"],
            "final_json_limit_exceeded"
        );
        assert_eq!(
            refusal["resource_refusal"]["attempted_final_pretty_json_bytes"],
            refusal["evidence_limits"]["attempted_final_pretty_json_bytes"],
        );
        assert!(
            refusal["resource_refusal"]["attempted_final_pretty_json_bytes"]
                .as_u64()
                .unwrap()
                > 8192,
        );
        assert_eq!(refusal["admissible_positive"], false);
        assert_eq!(refusal["admissible_negative"], false);
    }
}
