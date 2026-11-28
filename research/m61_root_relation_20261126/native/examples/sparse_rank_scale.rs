//! Planted sparse linear-algebra scale probe; rows are synthetic, not E relations.
use ecdlp_pieces::sparse_wiedemann::TallSparseRows;
use ecdlp_pieces::R;
use std::time::Instant;

fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % R as u128) as u64
}
fn add(a: u64, b: u64) -> u64 {
    let sum = a + b;
    if sum >= R {
        sum - R
    } else {
        sum
    }
}
fn random(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn case(n: usize) {
    let started = Instant::now();
    let mut state = 2026 + n as u64;
    let mut planted = (0..n)
        .map(|_| 1 + random(&mut state) % (R - 1))
        .collect::<Vec<_>>();
    planted.push(1); // fixed free column
    let mut independent = Vec::with_capacity(n);
    for i in 0..n {
        // Unit upper triangular, with a nonzero superdiagonal chain and
        // up to four extra diagonals: a nonsingular planted minor.
        let mut row = vec![(i, 1_u64)];
        for offset in 1..=5 {
            if i + offset < n {
                let coefficient = if offset == 1 {
                    1
                } else {
                    1 + random(&mut state) % (R - 1)
                };
                row.push((i + offset, coefficient));
            }
        }
        let dot = row.iter().fold(0, |sum, &(column, coefficient)| {
            add(sum, mul(coefficient, planted[column]))
        });
        if dot != 0 {
            row.push((n, R - dot));
        }
        independent.push(row);
    }
    let mut rows = Vec::with_capacity(8 * n);
    for copy in 0..8 {
        for row in &independent {
            let weight = if copy == 0 {
                1
            } else {
                1 + random(&mut state) % (R - 1)
            };
            rows.push(
                row.iter()
                    .map(|&(column, coefficient)| (column, mul(weight, coefficient)))
                    .collect(),
            );
        }
    }
    let source_nonzeros = rows.iter().map(Vec::len).sum::<usize>();
    let prepared_ms = started.elapsed().as_millis();
    let tall = TallSparseRows::new(n + 1, rows, R).unwrap();
    let certificate = tall.rank_certified_nullspace(n, 32, 8).unwrap();
    assert_eq!(certificate.vector, planted);
    println!(
        "n={n} rows={} source_nonzeros={source_nonzeros} compressed_nonzeros={} compression_attempts={} setup_ms={prepared_ms} total_ms={}",
        8 * n,
        certificate.compressed_nonzeros,
        certificate.compression_attempts_used,
        started.elapsed().as_millis()
    );
}

fn main() {
    for n in [64, 256, 512, 1024, 2446] {
        case(n);
    }
}
