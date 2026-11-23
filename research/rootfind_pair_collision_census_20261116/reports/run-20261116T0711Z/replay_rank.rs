//! Separate dense Gauss-Jordan check of the recorded row vectors.
//! Input: one space-delimited row per line on stdin; argv: modulus, columns.

use std::io::{self, BufRead};

fn power(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    while exponent > 0 {
        if exponent & 1 != 0 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exponent >>= 1;
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3, "usage: replay_rank <prime-modulus> <columns>");
    let modulus: u64 = args[1].parse().unwrap();
    let columns: usize = args[2].parse().unwrap();
    assert!(modulus > 2 && columns > 0);
    let mut matrix = Vec::<Vec<u64>>::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let row: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        assert_eq!(row.len(), columns);
        assert!(row.iter().all(|&x| x < modulus));
        matrix.push(row);
    }
    let mut rank = 0usize;
    for col in 0..columns {
        let Some(pivot) = (rank..matrix.len()).find(|&row| matrix[row][col] != 0) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let inverse = power(matrix[rank][col], modulus - 2, modulus);
        assert_eq!(
            (matrix[rank][col] as u128 * inverse as u128 % modulus as u128) as u64,
            1,
            "modulus must be prime"
        );
        for k in col..columns {
            matrix[rank][k] =
                ((matrix[rank][k] as u128 * inverse as u128) % modulus as u128) as u64;
        }
        for row in 0..matrix.len() {
            if row == rank {
                continue;
            }
            let factor = matrix[row][col];
            if factor == 0 {
                continue;
            }
            for k in col..columns {
                let product =
                    ((factor as u128 * matrix[rank][k] as u128) % modulus as u128) as u64;
                matrix[row][k] = (matrix[row][k] + modulus - product) % modulus;
            }
        }
        rank += 1;
        if rank == matrix.len() {
            break;
        }
    }
    println!("rows={} columns={} modulus={} rank={}", matrix.len(), columns, modulus, rank);
}
