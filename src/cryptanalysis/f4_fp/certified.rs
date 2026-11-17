//! Certified structured bases alongside the regular F4 engine.
//!
//! This opt-in adapter accepts the same `Poly` values as F4. Unsupported shapes
//! return an error; callers choose a fallback explicitly. The default F4 path
//! is unchanged. All conversion and checking belongs to this API's cost.
use super::{normalise, F4Options, Ordering, Poly, TraceStats};
use ::certified_relations as algebra;
pub use algebra::quotient::compact::{Certificate as CompactCertificate, Info as CompactInfo};
pub use algebra::quotient::Info as QuotientInfo;
pub use algebra::replay::NativeKernels as ReplayKernels;
pub use algebra::Row as CertificateRow;
pub use relation_replay_checker::{Checked as ReplayChecked, Stats as ReplayCheckStats};

/// Explicit wire families; the existing vector replay API remains v1-only.
#[derive(Clone, Copy)]
pub enum ReplayForm {
    CompactSparse,
    CompactDag,
    /// Same wire families, with packed monomials throughout construction.
    PackedSparse,
    PackedDag,
}

/// Accepted only after independent wire replay and the native representation check.
/// The serialized originals and proof remain available for separate execution.
pub struct PackedReplayBasis {
    pub basis: Vec<Poly>,
    pub original_json: Vec<u8>,
    pub witness_json: Vec<u8>,
    pub replay_stats: ReplayCheckStats,
    pub native_checks: TraceStats,
    pub observed_solving_degree: Option<u32>,
    prime: u64,
    variables: usize,
    form: ReplayForm,
}

fn replay_system(original: &[algebra::Poly], n: usize, p: u64) -> algebra::System {
    algebra::System {
        prime: p,
        variables: (0..n).map(|i| format!("x{i}")).collect(),
        polynomials: original
            .iter()
            .map(|f| {
                f.iter()
                    .map(|(m, c)| algebra::Term {
                        exponents: m.clone(),
                        coefficient: *c as i64,
                    })
                    .collect()
            })
            .collect(),
    }
}
fn packed_check(
    original: &[u8],
    witness: &[u8],
    form: ReplayForm,
) -> Result<ReplayChecked, String> {
    match form {
        ReplayForm::CompactSparse | ReplayForm::PackedSparse => {
            relation_replay_checker::packed::verify_json(original, witness)
        }
        ReplayForm::CompactDag | ReplayForm::PackedDag => {
            relation_replay_checker::packed::verify_dag_json(original, witness)
        }
    }
}
fn export_replayed(checked: &ReplayChecked, p: u64) -> Result<Vec<Poly>, String> {
    checked
        .basis
        .iter()
        .map(|f| {
            let poly: algebra::Poly = f.iter().cloned().collect();
            native_polynomial(&poly, p)
        })
        .collect()
}

/// Full native-owned construction, serialization, independent p31/p53 replay,
/// conversion and native F4-boundary checking in one call. No F4 solve is run;
/// this does not extend the regular F4 solver's documented small-prime scope.
pub fn try_packed_replay_basis(
    input: &[Poly],
    n: usize,
    p: u64,
    form: ReplayForm,
    kernels: ReplayKernels,
) -> Result<PackedReplayBasis, String> {
    let original = canonical_input(input, n, p)?;
    let system = replay_system(&original, n, p);
    let original_json = serde_json::to_vec(&system).map_err(|e| e.to_string())?;
    let (witness_json, _) = match form {
        ReplayForm::CompactSparse => {
            algebra::replay::produce_native_for_packed(&system, "compact-sparse", kernels)?
        }
        ReplayForm::CompactDag => {
            algebra::replay::produce_compact_dag_for_packed(&system, kernels)?
        }
        ReplayForm::PackedSparse => algebra::packed_constructor::produce_sparse(&system, kernels)?,
        ReplayForm::PackedDag => algebra::packed_constructor::produce_dag(&system, kernels)?,
    };
    let checked = packed_check(&original_json, &witness_json, form)?;
    let basis = export_replayed(&checked, p)?;
    let native_input = original
        .iter()
        .map(|f| native_polynomial(f, p))
        .collect::<Result<Vec<_>, _>>()?;
    let mut native_checks = TraceStats::default();
    if !super::reuse::certify(
        &native_input,
        &basis,
        p,
        &F4Options::new(Ordering::Grevlex, 8),
        &mut native_checks,
    ) {
        return Err("packed replay basis failed native F4 representation checks".into());
    }
    Ok(PackedReplayBasis {
        basis,
        original_json,
        witness_json,
        replay_stats: checked.stats,
        native_checks,
        observed_solving_degree: None,
        prime: p,
        variables: n,
        form,
    })
}

/// Recheck public proof, original, basis and receipt mutations from the input.
pub fn verify_packed_replay_basis(report: &PackedReplayBasis, input: &[Poly]) -> bool {
    let Ok(original) = canonical_input(input, report.variables, report.prime) else {
        return false;
    };
    let Ok(bytes) = serde_json::to_vec(&replay_system(&original, report.variables, report.prime))
    else {
        return false;
    };
    if bytes != report.original_json || report.observed_solving_degree.is_some() {
        return false;
    }
    let Ok(checked) = packed_check(&bytes, &report.witness_json, report.form) else {
        return false;
    };
    let Ok(basis) = export_replayed(&checked, report.prime) else {
        return false;
    };
    if basis != report.basis
        || serde_json::to_value(&checked.stats).ok()
            != serde_json::to_value(&report.replay_stats).ok()
    {
        return false;
    }
    let Ok(native_input) = original
        .iter()
        .map(|f| native_polynomial(f, report.prime))
        .collect::<Result<Vec<_>, _>>()
    else {
        return false;
    };
    let mut native_checks = TraceStats::default();
    super::reuse::certify(
        &native_input,
        &basis,
        report.prime,
        &F4Options::new(Ordering::Grevlex, 8),
        &mut native_checks,
    ) && native_checks == report.native_checks
}

/// Exact bounded replay of a serialized original system and certificate using
/// separate arithmetic, without invoking a basis solver or external process.
/// Currently p=2^31-1 and3..16 variables; parsing and every acceptance check are
/// included in this call. Unsupported or malformed certificates return Err.
pub fn verify_replay_certificate_json(
    original: &[u8],
    witness: &[u8],
) -> Result<ReplayChecked, String> {
    relation_replay_checker::verify_json(original, witness)
}

/// Complete basis with an acyclic chain of checked local membership identities.
/// Local stored term counts are a different representation from expanded
/// original-input cofactors; neither is a runtime or regularity observation.
pub struct CompactCertifiedBasis {
    pub basis: Vec<Poly>,
    pub certificate: CompactCertificate,
    pub info: CompactInfo,
    pub native_checks: TraceStats,
    pub observed_solving_degree: Option<u32>,
    prime: u64,
    variables: usize,
}

/// A complete monic reduced grevlex basis and original-input certificates.
/// `verify_basis` rechecks a mutated/retained result against the supplied input.
pub struct CertifiedBasis {
    pub basis: Vec<Poly>,
    pub certificates: Vec<CertificateRow>,
    pub quotient: QuotientInfo,
    pub native_checks: TraceStats,
    /// No Macaulay/F4 solving-degree observation is made by this algorithm.
    pub observed_solving_degree: Option<u32>,
    prime: u64,
    variables: usize,
}

fn canonical_input(input: &[Poly], n: usize, p: u64) -> Result<Vec<algebra::Poly>, String> {
    if !(1..=16).contains(&n) || p > i64::MAX as u64 || !algebra::prime(p) {
        return Err("certified relations require 1..16 variables and a prime <= i64::MAX".into());
    }
    if input.is_empty() || input.len() > 2000 {
        return Err("certified relations input equation budget".into());
    }
    let mut result = Vec::with_capacity(input.len());
    for f in input {
        if f.len() > 5000 {
            return Err("certified relations input term budget".into());
        }
        let mut out = algebra::Poly::new();
        for (m, c) in f {
            if m.len() != n || m.iter().map(|e| u64::from(*e)).sum::<u64>() > 16 {
                return Err("certified relations exponent shape/degree budget".into());
            }
            let m: Vec<usize> = m.iter().map(|e| *e as usize).collect();
            let c = ((out.get(&m).copied().unwrap_or(0) as u128 + u128::from(*c % p))
                % u128::from(p)) as u64;
            if c == 0 {
                out.remove(&m);
            } else {
                out.insert(m, c);
            }
        }
        result.push(out);
    }
    Ok(result)
}
fn native_polynomial(poly: &algebra::Poly, p: u64) -> Result<Poly, String> {
    let terms = poly
        .iter()
        .map(|(m, c)| {
            let exponents = m
                .iter()
                .map(|e| u32::try_from(*e))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "certified relation exponent overflow".to_string())?;
            Ok((exponents, *c))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(normalise(&terms, p, Ordering::Grevlex))
}
fn export(rows: &[CertificateRow], p: u64) -> Result<Vec<Poly>, String> {
    rows.iter()
        .map(|row| native_polynomial(&row.poly, p))
        .collect()
}

/// Try a complete certified quotient basis. This does not call `f4` or a
/// reference basis oracle. Coordinate propagation uses minimum-depth waves.
/// Recognition, graph/unit failures and proof budgets
/// are reported explicitly. Field admission also applies to the 53-bit prime;
/// that does not extend the regular F4 engine's documented small-prime scope.
pub fn try_quotient_basis(input: &[Poly], n: usize, p: u64) -> Result<CertifiedBasis, String> {
    let original = canonical_input(input, n, p)?;
    let native_input = original
        .iter()
        .map(|f| native_polynomial(f, p))
        .collect::<Result<Vec<_>, _>>()?;
    let (certificates, quotient) = algebra::quotient::solve(&original, n, p)?;
    let basis = export(&certificates, p)?;
    let mut native_checks = TraceStats::default();
    if basis.is_empty()
        || !super::reuse::certify(
            &native_input,
            &basis,
            p,
            &F4Options::new(Ordering::Grevlex, 8),
            &mut native_checks,
        )
    {
        return Err("certified relation basis failed native F4 boundary checks".into());
    }
    Ok(CertifiedBasis {
        basis,
        certificates,
        quotient,
        native_checks,
        observed_solving_degree: None,
        prime: p,
        variables: n,
    })
}

/// Original-input membership plus input/S-pair checks, and agreement of the
/// exported F4 representation with the retained certificate rows. This is an
/// arithmetic check by the caller, not an independent signed execution proof.
pub fn verify_basis(report: &CertifiedBasis, input: &[Poly]) -> bool {
    let Ok(original) = canonical_input(input, report.variables, report.prime) else {
        return false;
    };
    // Certificate rows are public for inspection. Reject malformed mutations
    // before the algebra kernel sees monomial arithmetic on trusted shapes.
    let n = report.variables;
    if report.certificates.iter().any(|row| {
        row.poly.len() > 5000
            || row.proof.len() > 50000
            || row.poly.iter().any(|(m, c)| {
                m.len() != n
                    || *c == 0
                    || *c >= report.prime
                    || m.iter()
                        .try_fold(0usize, |s, e| s.checked_add(*e))
                        .is_none_or(|d| d > 64)
            })
            || row.proof.iter().any(|((i, m), c)| {
                *i >= original.len()
                    || m.len() != n
                    || *c == 0
                    || *c >= report.prime
                    || m.iter().any(|e| e.checked_add(16).is_none())
            })
    }) {
        return false;
    }
    if !algebra::check_reference(
        &report.certificates,
        &original,
        report.variables,
        report.prime,
    ) {
        return false;
    }
    let Ok(expected) = export(&report.certificates, report.prime) else {
        return false;
    };
    if expected != report.basis || expected.is_empty() {
        return false;
    }
    let Ok(native_input) = original
        .iter()
        .map(|f| native_polynomial(f, report.prime))
        .collect::<Result<Vec<_>, _>>()
    else {
        return false;
    };
    super::reuse::certify(
        &native_input,
        &report.basis,
        report.prime,
        &F4Options::new(Ordering::Grevlex, 8),
        &mut TraceStats::default(),
    )
}

/// Opt-in compact membership certificates. All local identities, original
/// input containment, reducedness and S-pairs are checked before exporting;
/// the native F4 boundary checker also checks the exported representation.
pub fn try_compact_quotient_basis(
    input: &[Poly],
    n: usize,
    p: u64,
) -> Result<CompactCertifiedBasis, String> {
    compact_basis(input, n, p, false)
}

/// Eager all-vertices ablation with the same native checks as the lazy route.
pub fn try_compact_quotient_basis_all_vertices(
    input: &[Poly],
    n: usize,
    p: u64,
) -> Result<CompactCertifiedBasis, String> {
    compact_basis(input, n, p, true)
}

fn compact_basis(
    input: &[Poly],
    n: usize,
    p: u64,
    all_vertices: bool,
) -> Result<CompactCertifiedBasis, String> {
    let original = canonical_input(input, n, p)?;
    let native_input = original
        .iter()
        .map(|f| native_polynomial(f, p))
        .collect::<Result<Vec<_>, _>>()?;
    let output = if all_vertices {
        algebra::quotient::compact::solve_all_vertices(&original, n, p)?
    } else {
        algebra::quotient::compact::solve(&original, n, p)?
    };
    let basis = export(&output.certificate.basis, p)?;
    let mut native_checks = TraceStats::default();
    if !super::reuse::certify(
        &native_input,
        &basis,
        p,
        &F4Options::new(Ordering::Grevlex, 8),
        &mut native_checks,
    ) {
        return Err("compact certificate basis failed native F4 boundary checks".into());
    }
    Ok(CompactCertifiedBasis {
        basis,
        certificate: output.certificate,
        info: output.info,
        native_checks,
        observed_solving_degree: None,
        prime: p,
        variables: n,
    })
}

/// Recheck membership/basis arithmetic and native representation after public
/// certificate mutation. Construction diagnostics are not a signed verdict.
pub fn verify_compact_basis(report: &CompactCertifiedBasis, input: &[Poly]) -> bool {
    let Ok(original) = canonical_input(input, report.variables, report.prime) else {
        return false;
    };
    if !algebra::quotient::compact::verify(
        &report.certificate,
        &original,
        report.variables,
        report.prime,
    ) {
        return false;
    }
    let Ok(expected) = export(&report.certificate.basis, report.prime) else {
        return false;
    };
    if expected != report.basis {
        return false;
    }
    let Ok(native_input) = original
        .iter()
        .map(|f| native_polynomial(f, report.prime))
        .collect::<Result<Vec<_>, _>>()
    else {
        return false;
    };
    super::reuse::certify(
        &native_input,
        &report.basis,
        report.prime,
        &F4Options::new(Ordering::Grevlex, 8),
        &mut TraceStats::default(),
    )
}

/// Bounded affine closure/Hilbert traces for the same polynomial representation.
/// The analysis separately computes its completion reference and reports that
/// discovery cost; the reference never supplies rows to the traced closure.
pub fn trace_relations(
    input: &[Poly],
    n: usize,
    p: u64,
    max_degree: usize,
    candidate_limit: usize,
) -> Result<serde_json::Value, String> {
    let original = canonical_input(input, n, p)?;
    let system = algebra::System {
        prime: p,
        variables: (0..n).map(|i| format!("x{i}")).collect(),
        polynomials: original
            .iter()
            .map(|f| {
                f.iter()
                    .map(|(m, c)| algebra::Term {
                        exponents: m.clone(),
                        coefficient: *c as i64,
                    })
                    .collect()
            })
            .collect(),
    };
    algebra::analyze(&system, max_degree, candidate_limit)
}
