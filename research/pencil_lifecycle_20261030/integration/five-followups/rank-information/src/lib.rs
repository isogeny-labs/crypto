//! Offline linear-information diagnostic. No collector policy or cost estimate.
//! Rows encode A*x + b*k + c = 0 over the supplied prime field.
use pencil_symbolic_graph::residual::{Basis, Insert, Limits};

#[derive(Debug, PartialEq, Eq)]
pub struct Information {
    pub nuisance_rank: usize,
    pub nuisance_and_target_rank: usize,
    pub full_rank: usize,
    pub consistent: bool,
    pub target_determined: bool,
}

fn rank(p: u64, columns: usize, rows: &[Vec<u64>]) -> Result<usize, String> {
    let mut basis = Basis::new(
        p,
        columns,
        Limits {
            retained_coefficients: usize::MAX,
            working_coefficients: usize::MAX,
        },
    )
    .map_err(|e| format!("{e:?}"))?;
    for row in rows {
        let terms: Vec<_> = row.iter().take(columns).copied().enumerate().collect();
        match basis.insert(&terms).map_err(|e| format!("{e:?}"))? {
            Insert::Independent | Insert::Dependent => {}
            cap => return Err(format!("diagnostic cap: {cap:?}")),
        }
    }
    Ok(basis.rank())
}

/// For a consistent system, k is unique iff b is outside the column space of A.
/// This is rank([A b])-rank(A)=1. The full-rank comparison detects inconsistency.
/// Calling this diagnostic changes the kernel's thread-local operation counters;
/// run it outside measured producers. It does not recover or verify group scalars.
pub fn information(
    p: u64,
    nuisance_columns: usize,
    rows: &[Vec<u64>],
) -> Result<Information, String> {
    let width = nuisance_columns.checked_add(2).ok_or("column overflow")?;
    if rows.iter().any(|row| row.len() != width) {
        return Err("row width must be nuisance columns plus target and constant".into());
    }
    let a = rank(p, nuisance_columns, rows)?;
    let ab = rank(p, nuisance_columns + 1, rows)?;
    let abc = rank(p, width, rows)?;
    let consistent = abc == ab;
    Ok(Information {
        nuisance_rank: a,
        nuisance_and_target_rank: ab,
        full_rank: abc,
        consistent,
        target_determined: consistent && ab == a + 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_total_rank_can_have_different_target_information() {
        let nuisance = information(7, 1, &[vec![1, 0, 0]]).unwrap();
        let target = information(7, 1, &[vec![0, 1, 3]]).unwrap();
        assert_eq!(nuisance.full_rank, target.full_rank);
        assert!(!nuisance.target_determined);
        assert!(target.target_determined);
    }

    #[test]
    fn nuisance_rank_growth_does_not_determine_target() {
        let mut rows = vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0]];
        assert!(!information(7, 2, &rows).unwrap().target_determined);
        rows.push(vec![1, 1, 1, 3]);
        assert!(information(7, 2, &rows).unwrap().target_determined);
        rows.push(vec![2, 2, 2, 6]);
        assert_eq!(information(7, 2, &rows).unwrap().full_rank, 3);
    }

    #[test]
    fn inconsistency_is_not_target_information() {
        let info = information(7, 1, &[vec![0, 0, 1], vec![0, 1, 3]]).unwrap();
        assert!(!info.consistent);
        assert!(!info.target_determined);
    }

    #[test]
    fn row_order_scaling_and_empty_system() {
        let rows = vec![vec![1, 2, 3], vec![1, 4, 5]];
        let changed = vec![vec![3, 12, 15], vec![2, 4, 6]];
        assert_eq!(information(7, 1, &rows), information(7, 1, &changed));
        let empty = information(7, 0, &[]).unwrap();
        assert!(empty.consistent);
        assert!(!empty.target_determined);
    }

    #[test]
    fn reject_wrong_domain_and_shape() {
        assert!(information(15, 1, &[vec![1, 0, 0]]).is_err());
        assert!(information(7, 1, &[vec![1, 0]]).is_err());
        assert!(information(7, usize::MAX, &[]).is_err());
    }

    #[test]
    fn direct_solution_sets_check_the_information_predicate() {
        for rows in [
            vec![],
            vec![vec![1, 0, 0]],
            vec![vec![0, 1, 3]],
            vec![vec![1, 2, 3], vec![1, 4, 5]],
            vec![vec![0, 0, 1]],
        ] {
            let mut targets = std::collections::BTreeSet::new();
            for x in 0..7 {
                for k in 0..7 {
                    if rows
                        .iter()
                        .all(|row| (row[0] * x + row[1] * k + row[2]) % 7 == 0)
                    {
                        targets.insert(k);
                    }
                }
            }
            let info = information(7, 1, &rows).unwrap();
            assert_eq!(info.consistent, !targets.is_empty());
            assert_eq!(info.target_determined, targets.len() == 1);
        }
    }
}
