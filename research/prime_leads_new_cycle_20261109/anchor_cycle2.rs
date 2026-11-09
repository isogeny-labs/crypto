//! H3: anchor direction, not the presence/count of anchors, controls isolation.
//!
//! PREREGISTERED finite-model cycle 2; no result is claimed by this source.
//! Compile only after the parent commits this source and its exact dependency.
//! Dependency: the prior std-only `lead_controls` finite Z/19Z model, not the
//! production PrimeCurve, KSumTable, PKM pipeline or published five-sum records.
//!
//! Fixed fixture: scalar labels [1,1,1,1,1], the same 16 five-sum candidates,
//! retained collision equations of predicted rank 4, and the nonzero target 5.
//! Its kept query must evaluate to 5. Equations use five base-log variables and
//! one target variable. Known labels are used by the independent verifier, not
//! silently supplied to the recovery system as five anchor equations.
//!
//! H3 prediction: for a single anchor a, target isolation occurs exactly when
//! a dot [1;5] != 0 in F19. A zero-dot anchor lies in the existing collision row
//! span and contributes no new information, even if its byte representation is
//! nonzero. A nonzero-dot anchor removes the one-dimensional scalar ambiguity.
//! The base collision rank is held fixed at 4 throughout this cycle.
//!
//! Exactly two tests are fixed before execution:
//! 1. Named directional/degenerate controls: e0 resolves; e0-e1, zero, and
//!    3e0+16e1 do not; e0+e1 resolves. Adding a dependent anchor cannot make the
//!    dependent-only system resolve; adding an independent anchor can.
//! 2. Exhaust exactly 19*19 = 361 anchors [a,b,0,0,0]. Predict 342 isolated and
//!    19 ambiguous cases, and rank 6 versus 5 respectively for the query system.
//!    Left-pivot rank and independent right-pivot inverse-free reference rank
//!    must agree for every collision/query/anchor/target-unit matrix.
//! Stop on any mismatch. Preserve failures rather than weakening expectations.
//! This is a finite linear-information control, not a new ECDLP complexity,
//! global novelty, scientific-panel, timing, cost, pricing or execution claim.

extern crate lead_controls;

use lead_controls::{rank, reference_rank, Row, MODULUS};

pub const FIXED_LABELS: [u8; 5] = [1; 5];
pub const FIXED_TARGET: u8 = 5;
pub const ANCHOR_DIRECTIONS: usize = 19 * 19;
pub const PREDICTED_ISOLATED: usize = 19 * 18;
pub const PREDICTED_AMBIGUOUS: usize = 19;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Isolation {
    pub collision_rank: usize,
    pub query_rank_before_anchors: usize,
    pub equation_rank: usize,
    pub with_target_unit_rank: usize,
    pub target_isolated: bool,
}

fn agreed_rank(rows: &[Row], columns: usize) -> Result<usize, String> {
    let primary = rank(rows, columns)?;
    if primary != reference_rank(rows, columns)? {
        return Err("cycle-2 independent rank disagreement".into());
    }
    Ok(primary)
}

/// A caller-owned finite coefficient-space check. Anchor RHS scalar values are
/// assumed supplied separately; the question here is whether the target unit
/// vector lies in the coefficient row span. This does not attest the source,
/// correctness or public availability of any anchor in a cryptographic system.
pub fn isolation_with_anchors(
    collisions: &[Row],
    query: &[u8],
    anchors: &[Row],
) -> Result<Isolation, String> {
    if query.is_empty() || query.iter().any(|&x| x >= MODULUS) {
        return Err("invalid cycle-2 query row".into());
    }
    let base_columns = query.len();
    let collision_rank = agreed_rank(collisions, base_columns)?;
    agreed_rank(anchors, base_columns)?;
    let columns = base_columns + 1;
    let mut equations: Vec<Row> = collisions
        .iter()
        .map(|row| row.iter().copied().chain([0]).collect())
        .collect();
    equations.push(query.iter().copied().chain([MODULUS - 1]).collect());
    let query_rank_before_anchors = agreed_rank(&equations, columns)?;
    equations.extend(
        anchors
            .iter()
            .map(|anchor| anchor.iter().copied().chain([0]).collect()),
    );
    let equation_rank = agreed_rank(&equations, columns)?;
    let mut target_unit = vec![0; columns];
    target_unit[columns - 1] = 1;
    equations.push(target_unit);
    let with_target_unit_rank = agreed_rank(&equations, columns)?;
    Ok(Isolation {
        collision_rank,
        query_rank_before_anchors,
        equation_rank,
        with_target_unit_rank,
        target_isolated: equation_rank == with_target_unit_rank,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lead_controls::{dot, five_sum_table, probe};

    fn fixture() -> (Vec<Row>, Row) {
        let table = five_sum_table(&FIXED_LABELS, true).unwrap();
        assert_eq!(table.candidates, 16);
        let query = probe(&table, FIXED_TARGET).unwrap().unwrap();
        assert_eq!(dot(&query, &FIXED_LABELS).unwrap(), FIXED_TARGET);
        assert_eq!(agreed_rank(&table.collisions, 5).unwrap(), 4);
        for collision in &table.collisions {
            assert_eq!(dot(collision, &FIXED_LABELS).unwrap(), 0);
        }
        (table.collisions, query)
    }

    #[test]
    fn anchor_direction_and_dependent_anchor_controls() {
        let (collisions, query) = fixture();
        let cases = [
            (vec![1, 0, 0, 0, 0], true),
            (vec![1, 18, 0, 0, 0], false),
            (vec![0, 0, 0, 0, 0], false),
            (vec![1, 1, 0, 0, 0], true),
            (vec![3, 16, 0, 0, 0], false),
        ];
        for (anchor, expected) in cases {
            let observed = isolation_with_anchors(&collisions, &query, &[anchor]).unwrap();
            assert_eq!(observed.collision_rank, 4);
            assert_eq!(observed.query_rank_before_anchors, 5);
            assert_eq!(observed.target_isolated, expected);
            assert_eq!(observed.equation_rank, if expected { 6 } else { 5 });
            assert_eq!(observed.with_target_unit_rank, 6);
        }
        let dependent = vec![vec![1, 18, 0, 0, 0], vec![3, 16, 0, 0, 0]];
        let observed = isolation_with_anchors(&collisions, &query, &dependent).unwrap();
        assert!(!observed.target_isolated);
        assert_eq!(observed.equation_rank, 5);
        let mut resolved = dependent;
        resolved.push(vec![0, 1, 0, 0, 0]);
        let observed = isolation_with_anchors(&collisions, &query, &resolved).unwrap();
        assert!(observed.target_isolated);
        assert_eq!(observed.equation_rank, 6);
        assert!(isolation_with_anchors(&collisions, &query, &[vec![1, 0]]).is_err());
        assert!(isolation_with_anchors(&collisions, &query, &[vec![19; 5]]).is_err());
    }

    #[test]
    fn all_361_two_coordinate_anchor_directions_follow_pairing_prediction() {
        let (collisions, query) = fixture();
        let mut isolated = 0;
        let mut ambiguous = 0;
        for a in 0..MODULUS {
            for b in 0..MODULUS {
                let anchor = vec![a, b, 0, 0, 0];
                // Independent prediction uses the known scalar kernel vector,
                // not row-space membership or either elimination algorithm.
                let predicted = dot(&anchor, &FIXED_LABELS).unwrap() != 0;
                let observed = isolation_with_anchors(&collisions, &query, &[anchor]).unwrap();
                assert_eq!(observed.collision_rank, 4);
                assert_eq!(observed.query_rank_before_anchors, 5);
                assert_eq!(observed.target_isolated, predicted, "a={a}, b={b}");
                assert_eq!(observed.equation_rank, if predicted { 6 } else { 5 });
                assert_eq!(observed.with_target_unit_rank, 6);
                if observed.target_isolated {
                    isolated += 1;
                } else {
                    ambiguous += 1;
                }
            }
        }
        assert_eq!(isolated + ambiguous, ANCHOR_DIRECTIONS);
        assert_eq!(isolated, PREDICTED_ISOLATED);
        assert_eq!(ambiguous, PREDICTED_AMBIGUOUS);
    }
}
