//! Fixed algebraic development contracts, not a query or timing campaign.
use prime31_aux_matched::collector::{Recovery, RecoveryMode};

fn alpha(row: &[i64], logs: &[u64], beta: u64, d: u64, n: u64) -> u64 {
    let dot = row
        .iter()
        .zip(logs)
        .fold(0i128, |s, (&a, &x)| s + i128::from(a) * i128::from(x));
    (dot - i128::from(beta) * i128::from(d)).rem_euclid(i128::from(n)) as u64
}

#[test]
fn every_prefix_retains_rank_scalar_duplicates_and_augmented_work() {
    // Includes early target recovery below full A rank, new A pivots after
    // recovery, signed/repeated rows, a zero row and dependent multiples.
    let rows: [([i64; 4], u64); 9] = [
        ([1, 0, 0, 0], 1),
        ([2, 0, 0, 0], 2),
        ([1, 0, 0, 0], 3),
        ([0, 1, 0, 0], 2),
        ([0, 0, 0, 0], 0),
        ([-1, 2, 0, 0], 5),
        ([0, 0, 1, 0], 0),
        ([0, 0, 0, -2], 1),
        ([1, 0, 0, 0], 3),
    ];
    for n in [13, 101, 2_147_446_831] {
        let d = 7;
        let logs = [9, 3, 5, 11];
        let mut legacy = Recovery::new(4, n);
        let mut reduced = Recovery::new_with_mode(4, n, RecoveryMode::AugmentedOnly);
        for (i, (row, beta)) in rows.iter().enumerate() {
            let a = alpha(row, &logs, *beta, d, n);
            let before = legacy.insert(row, a, *beta).unwrap();
            let after = reduced.insert(row, a, *beta).unwrap();
            assert_eq!(before, after, "scalar at prefix{} mod{n}", i + 1);
            assert_eq!(legacy.rank_a(), reduced.rank_a());
            assert_eq!(legacy.rank_augmented(), reduced.rank_augmented());
            assert_eq!(legacy.duplicates, reduced.duplicates);
            assert_eq!(legacy.useful_dependencies, reduced.useful_dependencies);
            // rank_a is obtained from cached pivot state, so it introduces
            // no hidden extra scans of the retained augmented matrix.
            assert_eq!(legacy.work().1, reduced.work().1);
            assert_eq!(before, (i >= 2).then_some(d));
        }
        assert_eq!(reduced.rank_a(), 4);
        assert_eq!(reduced.rank_augmented(), 5);
        assert_eq!(reduced.useful_dependencies, 1);
        assert!(legacy.work().0["row_mul_subs"].as_u64().unwrap() > 0);
        assert_eq!(reduced.work().0["schema"], "prime31-recovery-work/v2");
        assert_eq!(reduced.work().0["a_matrix_materialized"], false);
        assert!(reduced.work().0["separate_matrix_counters"].is_null());
    }
}

#[test]
fn inconsistent_and_malformed_rows_reject_in_both_modes() {
    for mode in [RecoveryMode::DualMatrices, RecoveryMode::AugmentedOnly] {
        let mut r = Recovery::new_with_mode(2, 13, mode);
        assert_eq!(r.insert(&[1, 0], 2, 1).unwrap(), None);
        assert!(r.insert(&[1, 0], 3, 1).is_err());
        assert!(r.insert(&[1], 2, 1).is_err());
        assert!(r.insert(&[1, 0], 13, 1).is_err());
        assert!(r.insert(&[1, 0], 2, 13).is_err());
        assert_eq!(r.rank_a(), 1);
        assert_eq!(r.rank_augmented(), 1);
        assert_eq!(r.insert(&[1, 0], 8, 2).unwrap(), Some(7));
        assert!(r.insert(&[0, 0], 5, 1).is_err());
        assert_eq!(r.rank_a(), 1);
        assert_eq!(r.rank_augmented(), 2);
    }
}

#[test]
fn target_can_be_pinned_before_any_factor_base_pivot() {
    for c in [0, 1, 4] {
        let mut old = Recovery::new(c, 13);
        let mut new = Recovery::new_with_mode(c, 13, RecoveryMode::AugmentedOnly);
        let zero = vec![0; c];
        assert_eq!(old.insert(&zero, 6, 1).unwrap(), Some(7));
        assert_eq!(new.insert(&zero, 6, 1).unwrap(), Some(7));
        assert_eq!(new.rank_a(), 0);
        assert_eq!(new.rank_augmented(), 1);
        assert_eq!(old.work().1, new.work().1);
    }
}
