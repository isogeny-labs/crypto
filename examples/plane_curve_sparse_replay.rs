//! Exact small-field replay of plane line relations entering sparse modular LA.
//! This checks matrix encoding and the native solver, not a curve DLP.

use std::collections::BTreeMap;

use crypto_lib::cryptanalysis::koblitz_sparse_la::{
    solve_sparse_system, SparseSolveOptions, SparseSolveOutcome,
};
use crypto_lib::cryptanalysis::plane_curve_geometry::{PlaneCurve, PlaneTerm};
use crypto_lib::cryptanalysis::plane_curve_sparse::PlaneRelationColumns;

fn main() {
    let curve = PlaneCurve::new(
        5,
        3,
        &[
            PlaneTerm {
                x_exp: 0,
                y_exp: 2,
                z_exp: 1,
                coeff: 1,
            },
            PlaneTerm {
                x_exp: 3,
                y_exp: 0,
                z_exp: 0,
                coeff: 4,
            },
            PlaneTerm {
                x_exp: 1,
                y_exp: 0,
                z_exp: 2,
                coeff: 4,
            },
        ],
    )
    .unwrap();
    let base = [0, 1, 0];
    let point = [0, 0, 1];
    let columns = PlaneRelationColumns::new(&curve, base, vec![point]).unwrap();
    let line = curve
        .rational_line_relation(([0, 1, 0], [0, 0, 1]), ([1, 0, 0], [0, 1, 0]))
        .unwrap()
        .unwrap();
    let line_row = columns.encode_line_relation(&line, 5).unwrap();
    assert_eq!(line_row.entries, vec![(0, 2)]);
    assert_eq!(line_row.rhs, 0);

    // An algebraic target row plants the modular vector (log P, log T)=(0,3).
    // It is a matrix fixture, not a target class on this elliptic curve.
    let descent = columns
        .encode(&BTreeMap::from([(point, 1)]), 1, 3, 5)
        .unwrap();
    let rows = [line_row, descent];
    let (outcome, _) = solve_sparse_system(
        &rows,
        columns.columns(),
        5u64,
        &SparseSolveOptions::default(),
    );
    assert_eq!(outcome, SparseSolveOutcome::Solved(vec![0, 3]));
    println!("plane-line sparse matrix replay: columns=2 rows=2 vector=[0,3] verified=true");
}
