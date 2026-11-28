//! Curve-derived plane-line DLP system and CADO `bwc` format replay.
//!
//! Every matrix row comes from a certified smooth cubic's line divisors,
//! the definition `log_G(G)=1`, or the public target-point identity.
//! The target scalar is learned from the solve, then checked on the curve.

use std::collections::{BTreeMap, BTreeSet};

use crypto_lib::cryptanalysis::cado_bwc_io::{
    verify_rectangular_solution, verify_solution, write_rectangular_system, write_square_system,
};
use crypto_lib::cryptanalysis::koblitz_sparse_la::{
    solve_sparse_system, SparseRow, SparseSolveOptions, SparseSolveOutcome,
};
use crypto_lib::cryptanalysis::plane_curve_geometry::{PlaneCurve, PlaneTerm};
use crypto_lib::cryptanalysis::plane_curve_sparse::PlaneRelationColumns;
use crypto_lib::cryptanalysis::residual_walk::{Curve, Pt};

const P: u32 = 5;
const N: u64 = 7;

fn normalize(point: [u32; 3]) -> [u32; 3] {
    let leading = *point.iter().find(|&&v| v != 0).expect("projective point");
    let inverse = (1..P).find(|&v| (v * leading) % P == 1).unwrap();
    point.map(|v| (v * inverse) % P)
}

fn rank(rows: &[SparseRow<u64>], columns: usize) -> usize {
    let mut matrix: Vec<Vec<u64>> = rows
        .iter()
        .map(|row| {
            let mut dense = vec![0; columns];
            for &(column, value) in &row.entries {
                dense[column as usize] = value % N;
            }
            dense
        })
        .collect();
    let mut pivot = 0;
    for column in 0..columns {
        let Some(found) = (pivot..matrix.len()).find(|&i| matrix[i][column] != 0) else {
            continue;
        };
        matrix.swap(pivot, found);
        let inverse = (1..N)
            .find(|&v| v * matrix[pivot][column] % N == 1)
            .unwrap();
        for j in column..columns {
            matrix[pivot][j] = matrix[pivot][j] * inverse % N;
        }
        for i in pivot + 1..matrix.len() {
            let factor = matrix[i][column];
            for j in column..columns {
                matrix[i][j] = (matrix[i][j] + N - factor * matrix[pivot][j] % N) % N;
            }
        }
        pivot += 1;
    }
    pivot
}

fn main() {
    let mut target = Pt::affine(1, 2);
    let mut cado_dir = None;
    let mut cado_sol = None;
    let mut overdetermined = false;
    let mut args = std::env::args_os().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--target" {
            let x = args.next().expect("--target needs x and y");
            let y = args.next().expect("--target needs x and y");
            target = Pt::affine(
                x.to_str()
                    .unwrap()
                    .parse()
                    .expect("target x must be an integer"),
                y.to_str()
                    .unwrap()
                    .parse()
                    .expect("target y must be an integer"),
            );
        } else if flag == "--cado-dir" {
            cado_dir = Some(std::path::PathBuf::from(
                args.next().expect("--cado-dir needs a path"),
            ));
        } else if flag == "--cado-sol" {
            cado_sol = Some(std::path::PathBuf::from(
                args.next().expect("--cado-sol needs a path"),
            ));
        } else if flag == "--overdetermined" {
            overdetermined = true;
        } else {
            panic!("usage: plane_curve_ecdlp_system [--target X Y] [--overdetermined] [--cado-dir PATH] [--cado-sol PATH]");
        }
    }
    // y²z = x³ + 2xz² + z³, with seven F_5-rational points.
    let curve = PlaneCurve::new(
        P,
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
                coeff: 3,
            },
            PlaneTerm {
                x_exp: 0,
                y_exp: 0,
                z_exp: 3,
                coeff: 4,
            },
        ],
    )
    .unwrap();
    assert_eq!(curve.certify_geometric_smoothness(), Ok(true));
    let generator = Pt::affine(0, 1);
    // The public target is supplied by coordinates, never by its scalar.
    let group = Curve::new(P as u64, 2, 1, N, generator);
    assert!(
        target.x < P as u64 && target.y < P as u64,
        "target coordinates must be canonical residues"
    );
    assert!(group.is_on_curve(&generator) && group.is_on_curve(&target));
    assert_eq!(group.mul(&generator, N), Pt::INFINITY);
    assert_ne!(generator, Pt::INFINITY);
    let base = [0, 1, 0];
    let points: Vec<_> = (0..P)
        .flat_map(|x| (0..P).map(move |y| (x, y)))
        .filter(|&(x, y)| group.is_on_curve(&Pt::affine(x as u64, y as u64)))
        .map(|(x, y)| normalize([x, y, 1]))
        .collect();
    assert_eq!(points.len(), 6);
    assert_eq!(points.iter().copied().collect::<BTreeSet<_>>().len(), 6);
    let columns = PlaneRelationColumns::new_certified(&curve, base, points).unwrap();
    assert_eq!(columns.columns(), 7);

    let line_at_infinity = ([1, 0, 0], [0, 1, 0]);
    let mut candidate_rows = Vec::new();
    let mut split_lines = 0;
    let mut seen_lines = BTreeSet::new();
    for a in 0..P {
        for b in 0..P {
            for c in 0..P {
                if [a, b, c] == [0; 3] {
                    continue;
                }
                let coefficients = normalize([a, b, c]);
                if !seen_lines.insert(coefficients) {
                    continue;
                }
                let basis: Vec<_> = (0..P)
                    .flat_map(|x| (0..P).flat_map(move |y| (0..P).map(move |z| [x, y, z])))
                    .filter(|&v| v != [0; 3])
                    .filter(|&v| {
                        (coefficients[0] * v[0] + coefficients[1] * v[1] + coefficients[2] * v[2])
                            % P
                            == 0
                    })
                    .map(normalize)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .take(2)
                    .collect();
                let relation = curve
                    .certified_rational_line_relation((basis[0], basis[1]), line_at_infinity)
                    .unwrap();
                if let Some(relation) = relation {
                    split_lines += 1;
                    candidate_rows.push(columns.encode_line_relation(&relation, N).unwrap());
                }
            }
        }
    }
    assert_eq!(seen_lines.len(), 31);
    assert!(split_lines >= 7);
    let g = normalize([generator.x as u32, generator.y as u32, 1]);
    let q = normalize([target.x as u32, target.y as u32, 1]);
    // These equations define the logarithm origin and the public target.
    candidate_rows.push(columns.encode(&BTreeMap::from([(g, 1)]), 0, 1, N).unwrap());
    candidate_rows.push(columns.encode(&BTreeMap::from([(q, 1)]), -1, 0, N).unwrap());

    let mut rows = Vec::new();
    for row in &candidate_rows {
        let mut trial = rows.clone();
        trial.push(row.clone());
        if rank(&trial, columns.columns()) > rows.len() {
            rows.push(row.clone());
        }
        if rows.len() == columns.columns() {
            break;
        }
    }
    assert_eq!(rows.len(), columns.columns(), "curve rows lack full rank");
    let (outcome, _) =
        solve_sparse_system(&rows, columns.columns(), N, &SparseSolveOptions::default());
    let SparseSolveOutcome::Solved(solution) = outcome else {
        panic!("native sparse solve failed: {outcome:?}");
    };
    let recovered = solution[columns.target_column() as usize];
    assert_eq!(group.mul(&generator, recovered), target);
    if target == Pt::affine(1, 2) {
        assert_eq!(recovered, 5);
    }
    if overdetermined {
        let extra = candidate_rows[..split_lines]
            .iter()
            .find(|row| !row.entries.is_empty() && !rows.contains(row))
            .expect("no additional curve-derived line relation")
            .clone();
        rows.push(extra);
        assert_eq!(rank(&rows, columns.columns()), columns.columns());
    }
    let mut matrix = Vec::new();
    let mut rhs = Vec::new();
    let exported = if overdetermined {
        write_rectangular_system(&rows, columns.columns(), N, &mut matrix, &mut rhs)
    } else {
        write_square_system(&rows, columns.columns(), N, &mut matrix, &mut rhs)
    }
    .unwrap();
    assert_eq!(exported.dimension, 7);
    let cado_words: Vec<u8> = solution
        .iter()
        .copied()
        .chain(std::iter::once(N - 1))
        .flat_map(u64::to_le_bytes)
        .collect();
    let checked = if overdetermined {
        verify_rectangular_solution(&rows, columns.columns(), N, &cado_words)
    } else {
        verify_solution(&rows, columns.columns(), N, &cado_words)
    };
    assert_eq!(checked.unwrap(), solution);
    if let Some(directory) = cado_dir {
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("matrix.bin"), &matrix).unwrap();
        std::fs::write(directory.join("rhs.txt"), &rhs).unwrap();
    }
    if let Some(path) = cado_sol {
        let words = std::fs::read(path).unwrap();
        let external = if overdetermined {
            verify_rectangular_solution(&rows, columns.columns(), N, &words)
        } else {
            verify_solution(&rows, columns.columns(), N, &words)
        }
        .unwrap();
        assert_eq!(external[columns.target_column() as usize], recovered);
        assert_eq!(group.mul(&generator, recovered), target);
        println!("external CADO solution verified: log={recovered}");
    }
    println!(
        "curve-derived plane DLP: points=7 split_lines={split_lines} rank=7 rows={} log={recovered} verified=true cado_nonzeros={}",
        rows.len(),
        exported.nonzeros
    );
}
