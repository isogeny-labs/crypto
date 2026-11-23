//! Tests for `cryptanalysis::large_prime_graph::block_wiedemann`.

use crypto_lib::cryptanalysis::large_prime_graph::block_wiedemann::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Random sparse `n × n` matrix with `w` entries per row; `dead` rows are
/// zero, so the rank is at most `n − dead`.
fn random_matrix(rng: &mut Rng, n: usize, w: usize, dead: usize, p: u64) -> SparseMatrix {
    let rows = (0..n)
        .map(|i| {
            if i < dead {
                Vec::new()
            } else {
                (0..w)
                    .map(|_| (rng.below(n as u64) as usize, 1 + rng.below(p - 1)))
                    .collect()
            }
        })
        .collect();
    SparseMatrix::new(n, rows, p).unwrap()
}

/// Dense rank for cross-checks.
fn dense_rank(a: &SparseMatrix, p: u64) -> usize {
    let n = a.n;
    let mut m: Vec<Vec<u64>> = a
        .rows
        .iter()
        .map(|r| {
            let mut d = vec![0u64; n];
            for &(c, x) in r {
                d[c] = x;
            }
            d
        })
        .collect();
    let mul = |a: u64, b: u64| ((a as u128 * b as u128) % p as u128) as u64;
    let pw = |mut b: u64, mut e: u64| {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = mul(r, b);
            }
            b = mul(b, b);
            e >>= 1;
        }
        r
    };
    let mut rank = 0;
    for col in 0..n {
        let Some(pv) = (rank..n).find(|&i| m[i][col] != 0) else {
            continue;
        };
        m.swap(rank, pv);
        let iv = pw(m[rank][col], p - 2);
        let prow = m[rank].clone();
        for (i, row) in m.iter_mut().enumerate() {
            if i != rank && row[col] != 0 {
                let f = mul(row[col], iv);
                for (x, &y) in row.iter_mut().zip(&prow) {
                    *x = (*x + p - mul(f, y)) % p;
                }
            }
        }
        rank += 1;
    }
    rank
}

const PRIMES: [u64; 3] = [65537, 1_000_003, (1u64 << 61) - 1];

#[test]
fn kernel_vectors_verify_for_every_block_shape() {
    let mut rng = Rng(0x1234_5678);
    for &p in &PRIMES {
        for &(m, n) in &[(1, 1), (2, 2), (4, 4), (2, 4), (4, 2), (8, 8)] {
            for trial in 0..6 {
                let dim = 30 + rng.below(90) as usize;
                let a = random_matrix(&mut rng, dim, 4, 1 + trial % 3, p);
                let mut found = None;
                for seed in 0..4 {
                    if let Ok(r) = kernel_vector(
                        &a,
                        p,
                        BwParams {
                            m,
                            n,
                            seed,
                            extra: 8,
                        },
                    ) {
                        found = Some(r);
                        break;
                    }
                }
                let (w, stats) =
                    found.unwrap_or_else(|| panic!("no kernel vector p={p} m={m} n={n} dim={dim}"));
                assert!(w.iter().any(|&t| t != 0));
                assert!(a.mul_vec(&w, p).iter().all(|&t| t == 0));
                assert_eq!(stats.sequence_spmv as usize, n * stats.sequence_length);
                assert!(stats.generator_degree <= stats.sequence_length);
            }
        }
    }
}

#[test]
fn solves_nonsingular_and_consistent_singular_systems() {
    let mut rng = Rng(77);
    for &p in &PRIMES {
        for &(m, n) in &[(1, 1), (4, 4), (2, 8)] {
            // Nonsingular: random sparse plus a nonzero diagonal.
            let dim = 60;
            let mut a = random_matrix(&mut rng, dim, 3, 0, p);
            for (i, r) in a.rows.iter_mut().enumerate() {
                r.push((i, 1 + rng.below(p - 1)));
            }
            let a = SparseMatrix::new(dim, a.rows.clone(), p).unwrap();
            if dense_rank(&a, p) < dim {
                continue;
            }
            let x0: Vec<u64> = (0..dim).map(|_| rng.below(p)).collect();
            let b = a.mul_vec(&x0, p);
            let (x, _) = solve(
                &a,
                &b,
                p,
                BwParams {
                    m,
                    n,
                    seed: 3,
                    extra: 8,
                },
                6,
            )
            .unwrap_or_else(|e| panic!("{e:?} p={p} m={m} n={n}"));
            assert_eq!(x, x0, "unique solution");
            // Singular but consistent: some solution, verified.
            let s = random_matrix(&mut rng, dim, 4, 5, p);
            let b = s.mul_vec(&x0, p);
            let (x, _) = solve(
                &s,
                &b,
                p,
                BwParams {
                    m,
                    n,
                    seed: 5,
                    extra: 8,
                },
                8,
            )
            .unwrap();
            assert_eq!(s.mul_vec(&x, p), b);
        }
    }
}

#[test]
fn nonsingular_matrix_has_no_kernel_vector() {
    let p = 1_000_003;
    let dim = 40;
    let rows = (0..dim)
        .map(|i| vec![(i, 2u64), ((i + 1) % dim, 1)])
        .collect();
    let a = SparseMatrix::new(dim, rows, p).unwrap();
    assert_eq!(dense_rank(&a, p), dim);
    assert_eq!(
        kernel_vector(&a, p, BwParams::default()).unwrap_err(),
        BwError::NoKernelVector
    );
}

#[test]
fn product_counts_follow_the_block_schedule() {
    // Sequence products n·L with L = ⌈N/m⌉ + ⌈N/n⌉ + extra: total ≈ N(1 + n/m).
    let p = (1u64 << 61) - 1;
    let mut rng = Rng(9);
    let a = random_matrix(&mut rng, 400, 5, 3, p);
    for b in [1usize, 2, 4, 8] {
        let (_, s) = kernel_vector(
            &a,
            p,
            BwParams {
                m: b,
                n: b,
                seed: 1,
                extra: 4,
            },
        )
        .unwrap();
        assert_eq!(s.sequence_length, 2 * 400usize.div_ceil(b) + 4);
        assert_eq!(s.sequence_spmv as usize, b * s.sequence_length);
        eprintln!(
            "b={b} L={} seq_spmv={} total_spmv={} proj={} gen={} deg={}",
            s.sequence_length,
            s.sequence_spmv,
            s.spmv,
            s.projection_mults,
            s.generator_mults,
            s.generator_degree
        );
    }
}

#[test]
fn rejects_bad_input() {
    assert_eq!(
        SparseMatrix::new(2, vec![vec![]], 7).unwrap_err(),
        BwError::NotSquare { rows: 1, n: 2 }
    );
    assert_eq!(
        SparseMatrix::new(1, vec![vec![(3, 1)]], 7).unwrap_err(),
        BwError::ColumnOutOfRange { column: 3 }
    );
    assert_eq!(
        SparseMatrix::new(1, vec![vec![]], 8).unwrap_err(),
        BwError::InvalidModulus
    );
    let a = SparseMatrix::new(1, vec![vec![]], 7).unwrap();
    assert_eq!(
        kernel_vector(
            &a,
            7,
            BwParams {
                m: 0,
                ..Default::default()
            }
        )
        .unwrap_err(),
        BwError::BadParams
    );
}
