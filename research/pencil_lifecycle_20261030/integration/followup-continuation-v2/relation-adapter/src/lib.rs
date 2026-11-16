//! Offline adapter for GraphSolver augmented rows, not producer instrumentation.
//! GraphSolver stores [A, b, -a] as A*x+b*k=-a. The information diagnostic
//! expects A*x+b*k+c=0, hence c=a: negate only the final augmented column.
use pencil_target_information::{information, Information};

pub fn from_augmented_rows(
    modulus: u64,
    nuisance_columns: usize,
    rows: &[Vec<u64>],
) -> Result<(Vec<Vec<u64>>, Information), String> {
    // Validate prime domain and width before any modular arithmetic or indexing.
    let _ = information(modulus, nuisance_columns, rows)?;
    let constant = nuisance_columns + 1;
    let converted: Vec<Vec<u64>> = rows
        .iter()
        .map(|row| {
            let mut out: Vec<u64> = row.iter().map(|x| x % modulus).collect();
            let rhs = out[constant];
            out[constant] = if rhs == 0 { 0 } else { modulus - rhs };
            out
        })
        .collect();
    let result = information(modulus, nuisance_columns, &converted)?;
    Ok((converted, result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collector_rhs_and_equation_constant_have_opposite_signs() {
        // Additive F7 fixture: nuisance class x=2, public target k=3.
        // Verified complements: x+2+1*k=0 and 2*x+4+2*k=0.
        // Native augmented storage is [A,b,-a]. Add x=2 as a second constraint.
        let native = vec![vec![1, 1, 5], vec![1, 0, 2], vec![2, 2, 3]];
        let (equations, result) = from_augmented_rows(7, 1, &native).unwrap();
        assert_eq!(equations, vec![vec![1, 1, 2], vec![1, 0, 5], vec![2, 2, 4]]);
        assert!(result.consistent && result.target_determined);
        let solutions: Vec<_> = (0..7)
            .flat_map(|x| (0..7).map(move |k| (x, k)))
            .filter(|&(x, k)| {
                equations
                    .iter()
                    .all(|row| (row[0] * x + row[1] * k + row[2]) % 7 == 0)
            })
            .collect();
        assert_eq!(solutions, vec![(2, 3)]);
    }
    #[test]
    fn all_small_augmented_systems_preserve_exact_solutions() {
        // Exhaust all 343 single-row systems over F7 and all 49 assignments.
        for a in 0..7 {
            for b in 0..7 {
                for rhs in 0..7 {
                    let native = vec![vec![a, b, rhs]];
                    let (converted, info) = from_augmented_rows(7, 1, &native).unwrap();
                    assert_eq!(info, information(7, 1, &native).unwrap());
                    for x in 0..7 {
                        for k in 0..7 {
                            assert_eq!(
                                (a * x + b * k) % 7 == rhs,
                                (converted[0][0] * x + converted[0][1] * k + converted[0][2]) % 7
                                    == 0
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn malformed_domain_and_rows_fail_before_conversion() {
        for p in [0, 1, 4, 9] {
            assert!(from_augmented_rows(p, 1, &[vec![1, 2, 3]]).is_err());
        }
        assert!(from_augmented_rows(7, usize::MAX, &[]).is_err());
        assert!(from_augmented_rows(7, 1, &[vec![1, 2]]).is_err());
        assert_eq!(
            from_augmented_rows(7, 0, &[vec![15, 14]]).unwrap().0,
            vec![vec![1, 0]]
        );
    }
}
