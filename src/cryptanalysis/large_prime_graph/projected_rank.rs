//! Exact sparse rank of the factor-base projection after large-prime cancellation.
//!
//! LP left-nullity is NOT necessarily useful factor-base rank. This module
//! audits the already materialized, source-replayable CompleteRelation rows;
//! it neither substitutes for original curve-group verification nor changes
//! their order, witnesses, or the production relation filter.
//!
//! For W spanning ker(L^T) and S the original small coefficient matrix,
//! the useful relation rank is rank(W S), not dim ker(L^T).
//! Arithmetic below is in the prime subgroup-order field F_ell, not F_p.

use super::{add_mod, inv_mod, mul_mod, neg_mod, valid_modulus, LpGraphError};
use crate::cryptanalysis::large_prime_filter::CompleteRelation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Indices are positions in the unchanged input CompleteRelation slice.
/// A residual RHS is an inconsistency ONLY if the RHS is a known constant
/// and every unknown (including any target log) is modeled as a small
/// column. Otherwise it flags a missing column/unknown, not a false group
/// relation. In either case it is not independent small-coordinate rank.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedRankAudit {
    pub input_rows: usize,
    pub useful_small_rank: usize,
    pub augmented_rank: usize,
    pub independent_row_indices: Vec<usize>,
    pub redundant_row_indices: Vec<usize>,
    pub inconsistent_row_indices: Vec<usize>,
    /// Original, ascending input-row indices forming a basis of the augmented
    /// [small | fixed-known-RHS] row space. Always includes every independent
    /// small-coordinate row and at most one nonzero residual-RHS row.
    /// This is valid only when all unknowns are explicit small columns.
    #[serde(default)]
    pub augmented_basis_row_indices: Vec<usize>,
    /// Sparse payload retained by this exact affine row-space basis.
    #[serde(default)]
    pub augmented_basis_nonzeros: usize,
    /// Original source coefficients retained, without combining witnesses.
    #[serde(default)]
    pub augmented_basis_source_terms: usize,
    /// Field operations do not include ordered-map/search overhead.
    pub elimination_steps: u64,
    pub elimination_multiply_add_terms: u64,
    pub normalization_multiplications: u64,
    pub inverse_calls: u64,
    /// Sum of stored normalized pivot-row nonzeros, excluding row metadata.
    pub pivot_nonzeros: usize,
    /// Maximum retained-pivot plus currently reduced-row nonzero coefficients.
    pub peak_working_nonzeros: usize,
}

fn add_sparse_term(row: &mut BTreeMap<u32, u64>, column: u32, term: u64, p: u64) {
    if term == 0 {
        return;
    }
    let next = add_mod(row.get(&column).copied().unwrap_or(0), term, p);
    if next == 0 {
        row.remove(&column);
    } else {
        row.insert(column, next);
    }
}

/// Audit useful rank and RHS consistency of replayed LP-free rows.
///
/// The input must have canonical strictly increasing, distinct small columns,
/// 1 <= coefficient < ell, and 0 <= rhs < ell. This is the contract of the
/// existing replay normalization. Invalid rows are refused, not repaired by
/// silently changing a source witness. Odd composite moduli are not rank
/// fields; if a needed pivot is not invertible the function fails closed.
///
/// Any target scalar coefficient must first be represented as an explicit
/// unknown column in the input small matrix, NOT hidden inside rhs; a
/// residual RHS with an omitted target column is not proof of invalidity.
///
/// A sparse echelon basis is kept with leading coefficient 1. Original
/// independent row indices are retained, so downstream callers can preserve
/// complete witnesses and not accidentally replace them with reduced rows.
/// Field reduction is exact, but fill and ordered-map work can dominate.
/// No ECDLP speed or full solution claim follows from this diagnostic.
pub fn audit_projected_rank(
    relations: &[CompleteRelation],
    ell: u64,
) -> Result<ProjectedRankAudit, LpGraphError> {
    if !valid_modulus(ell) {
        return Err(LpGraphError::InvalidModulus);
    }
    // Validate the entire batch before starting any arithmetic.
    for (row_index, relation) in relations.iter().enumerate() {
        if relation.rhs >= ell
            || relation.small.iter().any(|&(_, a)| a == 0 || a >= ell)
            || relation.small.windows(2).any(|w| w[0].0 >= w[1].0)
        {
            return Err(LpGraphError::NonCanonicalRow { row: row_index });
        }
    }

    let mut audit = ProjectedRankAudit {
        input_rows: relations.len(),
        ..ProjectedRankAudit::default()
    };
    // Pivot -> (normalized sparse coefficients, correspondingly reduced RHS).
    let mut pivots: BTreeMap<u32, (BTreeMap<u32, u64>, u64)> = BTreeMap::new();
    for (row_index, relation) in relations.iter().enumerate() {
        let mut current: BTreeMap<u32, u64> = relation.small.iter().copied().collect();
        let mut rhs = relation.rhs;
        audit.peak_working_nonzeros = audit
            .peak_working_nonzeros
            .max(audit.pivot_nonzeros + current.len());
        let mut selected = false;
        while let Some((&column, &coefficient)) = current.iter().next() {
            if let Some((basis, basis_rhs)) = pivots.get(&column) {
                // The pivot row has coefficient 1 at column. All its other
                // columns are >= column; progress is strict as this pivot
                // disappears, even if later fill occurs.
                for (&c, &b) in basis {
                    let term = neg_mod(mul_mod(coefficient, b, ell), ell);
                    add_sparse_term(&mut current, c, term, ell);
                }
                rhs = add_mod(
                    rhs,
                    neg_mod(mul_mod(coefficient, *basis_rhs, ell), ell),
                    ell,
                );
                audit.elimination_steps += 1;
                audit.elimination_multiply_add_terms += basis.len() as u64;
                audit.peak_working_nonzeros = audit
                    .peak_working_nonzeros
                    .max(audit.pivot_nonzeros + current.len());
            } else {
                let inverse = inv_mod(coefficient, ell)
                    .ok_or(LpGraphError::NonInvertible { value: coefficient })?;
                audit.inverse_calls += 1;
                audit.normalization_multiplications += current.len() as u64 + 1;
                let normalized: BTreeMap<u32, u64> = current
                    .into_iter()
                    .map(|(c, a)| (c, mul_mod(a, inverse, ell)))
                    .collect();
                let normalized_rhs = mul_mod(rhs, inverse, ell);
                audit.pivot_nonzeros += normalized.len();
                pivots.insert(column, (normalized, normalized_rhs));
                audit.independent_row_indices.push(row_index);
                audit.peak_working_nonzeros =
                    audit.peak_working_nonzeros.max(audit.pivot_nonzeros);
                selected = true;
                break;
            }
        }
        if !selected {
            if rhs == 0 {
                audit.redundant_row_indices.push(row_index);
            } else {
                audit.inconsistent_row_indices.push(row_index);
            }
        }
    }
    audit.useful_small_rank = pivots.len();
    // Adding a single RHS column increases matrix rank by at most one.
    audit.augmented_rank =
        audit.useful_small_rank + usize::from(!audit.inconsistent_row_indices.is_empty());
    // The original pivot rows span the S-projection. Every remaining
    // augmented residual is either zero or (0,...,0,delta), so retaining
    // the FIRST contradictory source row (if present) spans the entire
    // one-dimensional RHS residual. Never replace source-bearing original
    // relations by reduced rows, as that would discard their provenance.
    audit.augmented_basis_row_indices = audit.independent_row_indices.clone();
    if let Some(&first) = audit.inconsistent_row_indices.first() {
        audit.augmented_basis_row_indices.push(first);
    }
    audit.augmented_basis_row_indices.sort_unstable();
    audit.augmented_basis_nonzeros = audit
        .augmented_basis_row_indices
        .iter()
        .map(|&i| relations[i].small.len())
        .sum();
    audit.augmented_basis_source_terms = audit
        .augmented_basis_row_indices
        .iter()
        .map(|&i| relations[i].sources.len())
        .sum();
    debug_assert_eq!(audit.augmented_basis_row_indices.len(), audit.augmented_rank);
    debug_assert_eq!(
        audit.input_rows,
        audit.independent_row_indices.len()
            + audit.redundant_row_indices.len()
            + audit.inconsistent_row_indices.len()
    );
    Ok(audit)
}

/// Return an exact augmented-row-space basis using unchanged original rows.
///
/// The selection first audits sparse factor-base rank over prime `F_ell`.
/// It retains the pivot originals and at most one additional row with a
/// nonzero residual known RHS. Every original equation is a linear consequence
/// of the retained affine equations; unsatisfiability, if any, is preserved.
///
/// The returned rows are clones, including their original source witnesses;
/// callers must charge the rank audit, output cloning, sparse fill and later
/// solver costs. This is opt-in and does NOT change the production LP filter.
/// All target/other unknowns must be explicit small columns, not hidden in RHS.
pub fn select_augmented_basis(
    relations: &[CompleteRelation],
    ell: u64,
) -> Result<(Vec<CompleteRelation>, ProjectedRankAudit), LpGraphError> {
    let audit = audit_projected_rank(relations, ell)?;
    let retained = audit
        .augmented_basis_row_indices
        .iter()
        .map(|&i| relations[i].clone())
        .collect();
    Ok((retained, audit))
}
