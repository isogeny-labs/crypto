//! A second elimination implementation checks production recovery decisions on
//! fixed small modular systems. Known logarithms occur only in test fixture
//! generation; the Recovery interface receives rows, alpha, and beta.

use prime31_aux_matched::collector::Recovery;

fn power(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut out = 1;
    while e != 0 {
        if e & 1 != 0 {
            out = out * a % p;
        }
        a = a * a % p;
        e >>= 1;
    }
    out
}

/// Batch Gauss-Jordan with a fixed left-to-right column traversal. The
/// production collector instead inserts rows into its maintained sparse RREF.
fn independent_rref(mut matrix: Vec<Vec<u64>>, columns: usize, p: u64) -> (usize, Vec<Vec<u64>>) {
    let mut rank = 0;
    for column in 0..columns {
        let Some(pivot) = (rank..matrix.len()).find(|&i| matrix[i][column] != 0) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let inverse = power(matrix[rank][column], p - 2, p);
        for x in &mut matrix[rank] {
            *x = *x * inverse % p;
        }
        for i in 0..matrix.len() {
            if i == rank {
                continue;
            }
            let scale = matrix[i][column];
            for j in 0..matrix[i].len() {
                matrix[i][j] = (matrix[i][j] + p - scale * matrix[rank][j] % p) % p;
            }
        }
        rank += 1;
    }
    (rank, matrix)
}

fn independently_pinned(matrix: &[Vec<u64>], columns: usize, target_column: usize) -> Option<u64> {
    matrix.iter().find_map(|row| {
        (row[target_column] == 1 && (0..columns).all(|i| i == target_column || row[i] == 0))
            .then_some(row[columns])
    })
}

#[test]
fn every_rank_and_target_pin_matches_independent_batch_rref() {
    const N: u64 = 109;
    const C: usize = 4;
    let base_logs = [7i64, 13, 2, 99];
    let target_log = 37i64;
    let mut recovery = Recovery::new(C, N);
    let mut a_rows = Vec::new();
    let mut augmented_rows = Vec::new();
    let mut fixture = vec![
        (vec![1i64, 1, 0, 0], 1u64),
        (vec![1i64, 1, 0, 0], 3u64),
        (vec![0i64; C], 0u64),
        (vec![0i64; C], 2u64),
    ];
    for k in 0..40i64 {
        fixture.push((
            (0..C)
                .map(|j| ((k * (j as i64 + 3) + j as i64 * 5) % 9) - 4)
                .collect(),
            ((17 * k + 11) as u64) % N,
        ));
    }
    for (index, (row, beta)) in fixture.into_iter().enumerate() {
        let alpha = (row.iter().zip(base_logs).map(|(&a, l)| a * l).sum::<i64>()
            - beta as i64 * target_log)
            .rem_euclid(N as i64) as u64;
        let got = recovery.insert(&row, alpha, beta).unwrap();
        let reduced: Vec<u64> = row.iter().map(|&x| x.rem_euclid(N as i64) as u64).collect();
        let mut augmented = reduced.clone();
        augmented.push((N - beta) % N);
        augmented.push(alpha);
        a_rows.push(reduced);
        augmented_rows.push(augmented);
        let (rank_a, _) = independent_rref(a_rows.clone(), C, N);
        let (rank_augmented, rref) = independent_rref(augmented_rows.clone(), C + 1, N);
        let independent = independently_pinned(&rref, C + 1, C);
        assert_eq!(recovery.rank_a(), rank_a, "A rank at prefix {index}");
        assert_eq!(
            recovery.rank_augmented(),
            rank_augmented,
            "augmented rank at prefix {index}"
        );
        assert_eq!(got, independent, "target pin at prefix {index}");
        assert!(got.is_none() || got == Some(target_log as u64));
        if index == 1 {
            assert_eq!(rank_a, 1);
            assert_eq!(rank_augmented, 2);
            assert_eq!(
                got,
                Some(37),
                "duplicate A rows can pin target before full base rank"
            );
        }
    }
    assert!(recovery.duplicates >= 2);
    assert_eq!(recovery.useful_dependencies, 1);
}

#[test]
fn a_zero_beta_dependency_never_becomes_a_target_pin() {
    let mut recovery = Recovery::new(3, 109);
    for (row, alpha) in [
        ([1, -1, 0], 7u64),
        ([2, -2, 0], 14),
        ([-3, 3, 0], 88),
        ([0, 0, 0], 0),
    ] {
        assert_eq!(recovery.insert(&row, alpha, 0).unwrap(), None);
    }
    assert_eq!(recovery.rank_a(), 1);
    assert_eq!(recovery.rank_augmented(), 1);
    assert_eq!(recovery.useful_dependencies, 0);
}
