//! Exact small-field replay of plane line relations entering sparse modular LA.
//! This checks matrix encoding and the native solver, not a curve DLP.

use std::collections::BTreeMap;

use crypto_lib::cryptanalysis::cado_bwc_io::{verify_solution, write_square_system};
use crypto_lib::cryptanalysis::koblitz_sparse_la::{
    solve_sparse_system, SparseRow, SparseSolveOptions, SparseSolveOutcome,
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
    let mut matrix = Vec::new();
    let mut rhs = Vec::new();
    let exported = write_square_system(&rows, columns.columns(), 5, &mut matrix, &mut rhs).unwrap();
    assert_eq!((exported.dimension, exported.nonzeros), (2, 3));
    assert_eq!(
        matrix,
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
            0, 0, 0
        ]
    );
    assert_eq!(String::from_utf8(rhs).unwrap(), "2 1 5\n0\n3\n");
    // Mock CADO right-nullspace coordinates (0, 2, 1) encode x=(0,3).
    let solution_bytes: Vec<u8> = [0u64, 2, 1]
        .into_iter()
        .flat_map(u64::to_le_bytes)
        .collect();
    assert_eq!(
        verify_solution(&rows, 2, 5, &solution_bytes).unwrap(),
        vec![0, 3]
    );
    let wrong_bytes: Vec<u8> = [0u64, 1, 1]
        .into_iter()
        .flat_map(u64::to_le_bytes)
        .collect();
    assert!(verify_solution(&rows, 2, 5, &wrong_bytes).is_err());
    assert!(verify_solution(&rows, 2, 5, &[0; 9]).is_err());
    assert!(verify_solution(&rows, 2, 5, &[0; 24]).is_err());
    let negative = [SparseRow::new(vec![(0, 4)], 0, 5)];
    let mut negative_matrix = Vec::new();
    let mut negative_rhs = Vec::new();
    write_square_system(&negative, 1, 5, &mut negative_matrix, &mut negative_rhs).unwrap();
    assert_eq!(
        negative_matrix,
        vec![1, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255]
    );
    let wide = [SparseRow::new(vec![(0, 1u64 << 31)], 0, (1u64 << 61) - 1)];
    assert!(
        write_square_system(&wide, 1, (1u64 << 61) - 1, &mut Vec::new(), &mut Vec::new()).is_err()
    );
    println!("plane-line sparse matrix replay: columns=2 rows=2 vector=[0,3] verified=true");
}
