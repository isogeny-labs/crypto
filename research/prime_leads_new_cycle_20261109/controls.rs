//! Preregistered finite CONTROL MODELS, not the production curve/table backends.
//!
//! Scope: arithmetic is Z/19Z with generator 1. H1 models a bounded generic
//! shift attachment. H2 models a signed five-sum table and its collision rows.
//! Neither is a Hecke collector, PrimeCurve/KSumTable backend test, published
//! five-sum replay, scientific panel, timing experiment, or complete-cost claim.
//! No production checkpoint is called or replaced, and no science entrypoint is
//! launched. These synthetic software checks do not admit comparisons or prices.
//!
//! Fixtures and stop conditions are fixed BEFORE execution:
//! - H1: four label sets, all 19 targets, caps 0..=19: exactly 1,520 cases.
//!   Independent translated-set/minimum-distance reference must match every
//!   scalar, probe count, and miss. Permutation must preserve these outputs;
//!   duplicate/corrupt/noncanonical labels and targets must refuse.
//! - H2: three five-label fixtures, exactly 16 signed candidates each (48 total),
//!   first sign positive; no partial-sum pruning. Identity final sums are
//!   excluded from this model's table. Collision retention off/on must preserve
//!   attempts, kept representatives and lookup results. Every emitted collision
//!   row must annihilate the independent scalar labels. Nonzero target recovery
//!   must remain impossible without an anchor even when coefficient rank rises.
//!   The deliberately repeated-label fixture predicts rank 4 and resolution
//!   after one anchor; all-zero labels predict zero inserts/collisions/rank.
//! - Any assertion failure stops this cycle. Preserve it; do not relax fixtures
//!   or expected outcomes after observing results. No result is claimed here.
//!
//! The repeated/all-zero fixtures are deliberate model degeneracies, not valid
//! production factor bases. Lookup coverage, equation rank and target recovery
//! are separate facts. Checking known labels does not mean those labels are
//! supplied as equations to the recovery system. Inverses/ranks are finite model
//! arithmetic, not production scalar implementation validation.

use std::collections::{BTreeMap, BTreeSet};

pub const MODULUS: u8 = 19;
pub const H1_CASES: usize = 4 * 19 * 20;
pub const H2_CANDIDATES_PER_FIXTURE: usize = 16;
pub const H2_TOTAL_CANDIDATES: usize = 3 * H2_CANDIDATES_PER_FIXTURE;
pub const H1_LABEL_SETS: [&[u8]; 4] = [&[], &[1], &[1, 4, 7], &[2, 5, 11, 17]];
pub const H2_LABEL_SETS: [[u8; 5]; 3] = [[1, 2, 4, 7, 11], [1, 1, 1, 1, 1], [0, 0, 0, 0, 0]];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Label {
    pub point: u8,
    pub scalar: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Attachment {
    pub recovered: Option<u8>,
    pub probes: u8,
}

/// A bounded cyclic control with the same public convention Q + jP, P = 1.
/// Identity recognition occurs only inside an attempted probe, so cap zero
/// returns a miss even for the identity target. The scalar is checked last.
pub fn attach(labels: &[Label], target: u8, cap: u8) -> Result<Attachment, String> {
    if target >= MODULUS {
        return Err("noncanonical target".into());
    }
    let mut known = BTreeMap::new();
    for label in labels {
        if label.point >= MODULUS || label.scalar >= MODULUS {
            return Err("noncanonical label".into());
        }
        if label.point != label.scalar {
            return Err("label scalar equation fails in the cyclic model".into());
        }
        if known.insert(label.point, label.scalar).is_some() {
            return Err("duplicate label point".into());
        }
    }
    let limit = cap.min(MODULUS);
    let mut shifted = target;
    for shift in 0..limit {
        let scalar = if shifted == 0 {
            Some(0)
        } else {
            known.get(&shifted).copied()
        };
        if let Some(found) = scalar {
            let recovered = (found + MODULUS - shift) % MODULUS;
            if recovered != target {
                return Err("final scalar equation fails".into());
            }
            return Ok(Attachment {
                recovered: Some(recovered),
                probes: shift + 1,
            });
        }
        shifted = (shifted + 1) % MODULUS;
    }
    Ok(Attachment {
        recovered: None,
        probes: limit,
    })
}

/// Independent set construction, without the producer's shifted-target loop.
pub fn attachment_reference(label_points: &[u8], target: u8, cap: u8) -> Attachment {
    assert!(target < MODULUS && label_points.iter().all(|&x| x < MODULUS));
    let limit = cap.min(MODULUS);
    let anchors: BTreeSet<u8> = label_points.iter().copied().chain([0]).collect();
    let covered: BTreeSet<u8> = anchors
        .iter()
        .flat_map(|&d| (0..limit).map(move |j| (d + MODULUS - j) % MODULUS))
        .collect();
    let first_distance = anchors
        .iter()
        .map(|&d| (d + MODULUS - target) % MODULUS)
        .filter(|&distance| distance < limit)
        .min();
    assert_eq!(covered.contains(&target), first_distance.is_some());
    Attachment {
        recovered: covered.contains(&target).then_some(target),
        probes: first_distance.map_or(limit, |distance| distance + 1),
    }
}

pub type Row = Vec<u8>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiveSumTable {
    /// One canonical signed representative per nonidentity negation orbit.
    pub kept: BTreeMap<u8, Row>,
    pub candidates: usize,
    pub insert_attempts: usize,
    pub identity_candidates: usize,
    pub collisions: Vec<Row>,
}

pub fn dot(row: &[u8], labels: &[u8]) -> Result<u8, String> {
    if row.len() != labels.len() || row.iter().chain(labels).any(|&x| x >= MODULUS) {
        return Err("noncanonical or dimension-mismatched scalar row".into());
    }
    Ok(row.iter().zip(labels).fold(0_u16, |sum, (&a, &b)| {
        (sum + u16::from(a) * u16::from(b)) % u16::from(MODULUS)
    }) as u8)
}

/// This model enumerates 16 sign choices, not the production table's recursive
/// partial-sum traversal, packing, key function, group operations or counters.
pub fn five_sum_table(labels: &[u8; 5], retain_collisions: bool) -> Result<FiveSumTable, String> {
    if labels.iter().any(|&x| x >= MODULUS) {
        return Err("noncanonical five-sum labels".into());
    }
    let mut table = FiveSumTable {
        kept: BTreeMap::new(),
        candidates: 0,
        insert_attempts: 0,
        identity_candidates: 0,
        collisions: Vec::new(),
    };
    for mask in 0_u8..16 {
        let mut row = vec![1; 5];
        for (i, entry) in row.iter_mut().enumerate().skip(1) {
            if mask & (1 << (i - 1)) != 0 {
                *entry = MODULUS - 1;
            }
        }
        table.candidates += 1;
        let sum = dot(&row, labels)?;
        if sum == 0 {
            table.identity_candidates += 1;
            continue;
        }
        let key = sum.min(MODULUS - sum);
        if key != sum {
            for entry in &mut row {
                *entry = (MODULUS - *entry) % MODULUS;
            }
        }
        table.insert_attempts += 1;
        if let Some(previous) = table.kept.get(&key) {
            if retain_collisions {
                table.collisions.push(
                    row.iter()
                        .zip(previous)
                        .map(|(&a, &b)| (a + MODULUS - b) % MODULUS)
                        .collect(),
                );
            }
        } else {
            table.kept.insert(key, row);
        }
    }
    Ok(table)
}

pub fn probe(table: &FiveSumTable, target: u8) -> Result<Option<Row>, String> {
    if target >= MODULUS {
        return Err("noncanonical five-sum target".into());
    }
    if target == 0 {
        return Ok(None);
    }
    let key = target.min(MODULUS - target);
    Ok(table.kept.get(&key).map(|row| {
        if key == target {
            row.clone()
        } else {
            row.iter().map(|&x| (MODULUS - x) % MODULUS).collect()
        }
    }))
}

fn canonical_matrix(rows: &[Row], columns: usize) -> Result<(), String> {
    if rows
        .iter()
        .any(|row| row.len() != columns || row.iter().any(|&x| x >= MODULUS))
    {
        return Err("noncanonical or ragged matrix".into());
    }
    Ok(())
}

/// Left-pivot RREF with inverse found by finite search.
pub fn rank(rows: &[Row], columns: usize) -> Result<usize, String> {
    canonical_matrix(rows, columns)?;
    let mut a = rows.to_vec();
    let mut next = 0;
    for column in 0..columns {
        let Some(pivot) = (next..a.len()).find(|&r| a[r][column] != 0) else {
            continue;
        };
        a.swap(next, pivot);
        let inverse = (1..MODULUS)
            .find(|&x| (u16::from(x) * u16::from(a[next][column])) % 19 == 1)
            .expect("nonzero element of F19 has an inverse");
        for value in &mut a[next] {
            *value = (u16::from(*value) * u16::from(inverse) % 19) as u8;
        }
        let pivot_row = a[next].clone();
        for (r, row) in a.iter_mut().enumerate() {
            if r == next {
                continue;
            }
            let factor = row[column];
            for (entry, &basis) in row.iter_mut().zip(&pivot_row) {
                *entry = ((u16::from(*entry) + 19 - u16::from(factor) * u16::from(basis) % 19) % 19)
                    as u8;
            }
        }
        next += 1;
    }
    Ok(next)
}

/// Independent arithmetic path: right pivots, unnormalized cross-products,
/// no inverse search, and elimination only below each pivot.
pub fn reference_rank(rows: &[Row], columns: usize) -> Result<usize, String> {
    canonical_matrix(rows, columns)?;
    let mut a = rows.to_vec();
    let mut next = 0;
    for column in (0..columns).rev() {
        let Some(pivot) = (next..a.len()).find(|&r| a[r][column] != 0) else {
            continue;
        };
        a.swap(next, pivot);
        let pivot_row = a[next].clone();
        let diagonal = pivot_row[column];
        for row in a.iter_mut().skip(next + 1) {
            let factor = row[column];
            for (entry, &basis) in row.iter_mut().zip(&pivot_row) {
                *entry = ((u16::from(diagonal) * u16::from(*entry) + 361
                    - u16::from(factor) * u16::from(basis))
                    % 19) as u8;
            }
        }
        next += 1;
    }
    Ok(next)
}

/// Collision equations have target coefficient zero. A successful lookup adds
/// [coefficients | -1]; its value equation is homogeneous in the unknown logs.
/// The anchor, when present, supplies e0 with its known scalar on the RHS.
/// This function tests coefficient-row isolation of the target variable only;
/// it is not a complete production equation/certificate checker.
pub fn target_isolation(
    collisions: &[Row],
    query: &[u8],
    anchor_first: bool,
) -> Result<(usize, bool), String> {
    canonical_matrix(collisions, query.len())?;
    if query.iter().any(|&x| x >= MODULUS) || query.is_empty() {
        return Err("invalid query row".into());
    }
    let columns = query.len() + 1;
    let mut equations: Vec<Row> = collisions
        .iter()
        .map(|row| row.iter().copied().chain([0]).collect())
        .collect();
    equations.push(query.iter().copied().chain([MODULUS - 1]).collect());
    if anchor_first {
        let mut anchor = vec![0; columns];
        anchor[0] = 1;
        equations.push(anchor);
    }
    let base_rank = rank(&equations, columns)?;
    if base_rank != reference_rank(&equations, columns)? {
        return Err("independent rank disagreement".into());
    }
    let mut target_unit = vec![0; columns];
    target_unit[columns - 1] = 1;
    equations.push(target_unit);
    let augmented_rank = rank(&equations, columns)?;
    if augmented_rank != reference_rank(&equations, columns)? {
        return Err("independent augmented rank disagreement".into());
    }
    Ok((base_rank, augmented_rank == base_rank))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(points: &[u8]) -> Vec<Label> {
        points
            .iter()
            .map(|&x| Label {
                point: x,
                scalar: x,
            })
            .collect()
    }

    #[test]
    fn attachment_matches_independent_translated_sets_for_1520_cases() {
        let mut cases = 0;
        for points in H1_LABEL_SETS {
            for cap in 0..=MODULUS {
                for target in 0..MODULUS {
                    assert_eq!(
                        attach(&labels(points), target, cap).unwrap(),
                        attachment_reference(points, target, cap),
                        "labels={points:?}, target={target}, cap={cap}"
                    );
                    cases += 1;
                }
            }
        }
        assert_eq!(cases, H1_CASES);
        assert_eq!(attach(&labels(&[]), 0, 0).unwrap().recovered, None);
        assert_eq!(attach(&labels(&[]), 0, 1).unwrap().recovered, Some(0));
        assert_eq!(attach(&labels(&[1]), 18, 2).unwrap().recovered, Some(18));
        for target in 0..MODULUS {
            assert_eq!(attach(&[], target, 255).unwrap().recovered, Some(target));
        }
    }

    #[test]
    fn attachment_label_permutations_preserve_results() {
        for points in H1_LABEL_SETS {
            let original = labels(points);
            let mut reverse = original.clone();
            reverse.reverse();
            let mut rotate = original.clone();
            if !rotate.is_empty() {
                rotate.rotate_left(1);
            }
            for target in 0..MODULUS {
                for cap in 0..=MODULUS {
                    let expected = attach(&original, target, cap).unwrap();
                    assert_eq!(attach(&reverse, target, cap).unwrap(), expected);
                    assert_eq!(attach(&rotate, target, cap).unwrap(), expected);
                }
            }
        }
    }

    #[test]
    fn attachment_refuses_duplicate_corrupt_and_noncanonical_inputs() {
        assert!(attach(&labels(&[1, 1]), 0, 0).is_err());
        assert!(attach(
            &[Label {
                point: 1,
                scalar: 2
            }],
            0,
            0
        )
        .is_err());
        assert!(attach(
            &[Label {
                point: 19,
                scalar: 0
            }],
            0,
            0
        )
        .is_err());
        assert!(attach(
            &[Label {
                point: 0,
                scalar: 19
            }],
            0,
            0
        )
        .is_err());
        assert!(attach(&[], 19, 19).is_err());
    }

    #[test]
    fn five_sum_retention_preserves_coverage_and_collision_equations() {
        let mut candidates = 0;
        for scalar_labels in H2_LABEL_SETS {
            let without = five_sum_table(&scalar_labels, false).unwrap();
            let with = five_sum_table(&scalar_labels, true).unwrap();
            assert_eq!(with.candidates, H2_CANDIDATES_PER_FIXTURE);
            candidates += with.candidates;
            assert_eq!(without.kept, with.kept);
            assert_eq!(without.candidates, with.candidates);
            assert_eq!(without.insert_attempts, with.insert_attempts);
            assert_eq!(without.identity_candidates, with.identity_candidates);
            assert!(without.collisions.is_empty());
            assert_eq!(
                with.kept.len() + with.collisions.len(),
                with.insert_attempts
            );
            assert_eq!(
                with.insert_attempts + with.identity_candidates,
                with.candidates
            );
            for row in &with.collisions {
                assert_eq!(dot(row, &scalar_labels).unwrap(), 0);
            }
            assert_eq!(
                rank(&with.collisions, 5),
                reference_rank(&with.collisions, 5)
            );
            for target in 0..MODULUS {
                let a = probe(&without, target).unwrap();
                let b = probe(&with, target).unwrap();
                assert_eq!(a, b);
                if let Some(row) = b {
                    assert_eq!(dot(&row, &scalar_labels).unwrap(), target);
                }
            }
            if scalar_labels == [0; 5] {
                assert_eq!(with.identity_candidates, 16);
                assert_eq!(with.insert_attempts, 0);
                assert!(with.kept.is_empty() && with.collisions.is_empty());
                assert_eq!(rank(&with.collisions, 5).unwrap(), 0);
            }
        }
        assert_eq!(candidates, H2_TOTAL_CANDIDATES);
    }

    #[test]
    fn collision_rank_is_not_unanchored_target_recovery() {
        for scalar_labels in H2_LABEL_SETS {
            let table = five_sum_table(&scalar_labels, true).unwrap();
            let coefficient_rank = rank(&table.collisions, 5).unwrap();
            assert_eq!(
                coefficient_rank,
                reference_rank(&table.collisions, 5).unwrap()
            );
            if scalar_labels != [0; 5] {
                assert!(coefficient_rank < 5);
            }
            for target in 1..MODULUS {
                if let Some(query) = probe(&table, target).unwrap() {
                    assert!(
                        !target_isolation(&table.collisions, &query, false)
                            .unwrap()
                            .1
                    );
                    if scalar_labels == [1; 5] {
                        assert_eq!(coefficient_rank, 4);
                        assert!(target_isolation(&table.collisions, &query, true).unwrap().1);
                    }
                }
            }
        }
    }

    #[test]
    fn independent_rank_and_collision_tamper_controls() {
        let known = [
            (vec![], 3, 0),
            (vec![vec![0, 0, 0]], 3, 0),
            (vec![vec![1, 2, 3], vec![2, 4, 6]], 3, 1),
            (vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 1]], 3, 3),
        ];
        for (rows, columns, expected) in known {
            assert_eq!(rank(&rows, columns).unwrap(), expected);
            assert_eq!(reference_rank(&rows, columns).unwrap(), expected);
        }
        let table = five_sum_table(&[1; 5], true).unwrap();
        let mut corrupted = table.collisions[0].clone();
        corrupted[0] = (corrupted[0] + 1) % MODULUS;
        assert_ne!(dot(&corrupted, &[1; 5]).unwrap(), 0);
        assert!(dot(&[1, 2], &[1; 5]).is_err());
        assert!(dot(&[19], &[1]).is_err());
        assert!(rank(&[vec![1, 2]], 3).is_err());
        assert!(reference_rank(&[vec![19]], 1).is_err());
        assert!(five_sum_table(&[19, 0, 0, 0, 0], true).is_err());
        assert!(probe(&table, 19).is_err());
        assert!(target_isolation(&[], &[], false).is_err());
    }
}
