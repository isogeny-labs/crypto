//! Development controls only. Production census requires its own frozen protocol.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

fn digit_radices(parts: &[usize]) -> Vec<usize> {
    assert!(!parts.is_empty() && parts.iter().all(|&k| (1..=27).contains(&k)));
    assert!(parts.iter().sum::<usize>() <= 27);
    parts.iter().map(|b| 1usize << b).collect()
}

fn partitions(n: usize, least: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if n == 0 {
        out.push(prefix.clone());
    } else {
        for k in least..=n {
            prefix.push(k);
            partitions(n - k, k, prefix, out);
            prefix.pop();
        }
    }
}

fn hilbert(radices: &[usize]) -> Vec<u128> {
    let mut h = vec![1];
    for &r in radices {
        assert!(r >= 2);
        let mut next = vec![0; h.len() + r - 1];
        for (i, &v) in h.iter().enumerate() {
            for j in 0..r {
                next[i + j] += v;
            }
        }
        h = next;
    }
    h
}

fn monomials(radices: &[usize]) -> Vec<Vec<Vec<usize>>> {
    let degree: usize = radices.iter().map(|r| r - 1).sum();
    let mut by_degree = vec![vec![]; degree + 1];
    for code in 0..radices.iter().product::<usize>() {
        let mut v = code;
        let mut exponents = Vec::new();
        for &r in radices {
            exponents.push(v % r);
            v /= r;
        }
        by_degree[exponents.iter().sum::<usize>()].push(exponents);
    }
    by_degree
}

fn power(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut value = 1;
    while b != 0 {
        if b & 1 != 0 {
            value = value * a % p;
        }
        a = a * a % p;
        b >>= 1;
    }
    value
}

fn rank(mut matrix: Vec<Vec<u64>>, p: u64) -> usize {
    if matrix.is_empty() {
        return 0;
    }
    let mut rank = 0;
    for col in 0..matrix[0].len() {
        if let Some(pivot) = (rank..matrix.len()).find(|&i| matrix[i][col] != 0) {
            matrix.swap(rank, pivot);
            let inv = power(matrix[rank][col], p - 2, p);
            for x in &mut matrix[rank] {
                *x = *x * inv % p;
            }
            for row in rank + 1..matrix.len() {
                let factor = matrix[row][col];
                for j in col..matrix[row].len() {
                    matrix[row][j] = (matrix[row][j] + p - factor * matrix[rank][j] % p) % p;
                }
            }
            rank += 1;
            if rank == matrix.len() {
                break;
            }
        }
    }
    rank
}

fn explicit_rank(radices: &[usize], basis: &[Vec<Vec<usize>>], d: usize, p: u64) -> usize {
    if d < 2 {
        return 0;
    }
    let target: BTreeMap<_, _> = basis[d]
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, m)| (m, i))
        .collect();
    let mut matrix = vec![vec![0; basis[d].len()]; basis[d - 2].len()];
    for (row, m) in basis[d - 2].iter().enumerate() {
        // Ordered variable pairs expand (sum z)^2 independently of Hilbert counts.
        for i in 0..radices.len() {
            for j in 0..radices.len() {
                let mut term = m.clone();
                term[i] += 1;
                term[j] += 1;
                if term.iter().zip(radices).all(|(a, r)| a < r) {
                    let col = target[&term];
                    matrix[row][col] = (matrix[row][col] + 1) % p;
                }
            }
        }
    }
    rank(matrix, p)
}

fn controls(out: &Path) -> Value {
    assert!(!out.exists(), "fresh control output required");
    std::fs::create_dir_all(out).unwrap();
    let mut rows = Vec::new();
    let mut domain_checks = 0;
    let mut characteristic_failures = 0;
    let mut partitions_checked = 0;
    for bits in 1..=6 {
        let mut parts = Vec::new();
        partitions(bits, 1, &mut vec![], &mut parts);
        for k in parts {
            partitions_checked += 1;
            let radices = digit_radices(&k);
            for x in 0..(1usize << bits) {
                let mut remaining = x;
                let mut decoded = 0;
                let mut weight = 1;
                for &r in &radices {
                    decoded += weight * (remaining % r);
                    remaining /= r;
                    weight *= r;
                }
                assert_eq!(remaining, 0);
                assert_eq!(decoded, x);
                assert_eq!(weight, 1usize << bits);
                domain_checks += 1;
            }
            let h = hilbert(&radices);
            let basis = monomials(&radices);
            assert_eq!(h, basis.iter().map(|v| v.len() as u128).collect::<Vec<_>>());
            for p in [2, 3, 101] {
                for d in 0..h.len() {
                    let observed = explicit_rank(&radices, &basis, d, p);
                    let maximal = if d < 2 {
                        0
                    } else {
                        h[d].min(h[d - 2]) as usize
                    };
                    let hypothesis = p as usize > h.len() - 1;
                    if hypothesis {
                        assert_eq!(observed, maximal);
                    }
                    if observed != maximal {
                        characteristic_failures += 1;
                    }
                    rows.push(json!({"bits":bits,"parts":k,"radices":radices,"p":p,"degree":d,"socle":h.len()-1,"source_dimension":if d<2{0}else{h[d-2] as u64},"target_dimension":h[d] as u64,"explicit_rank":observed,"maximal_rank":maximal,"large_characteristic_hypothesis":hypothesis}));
                }
            }
        }
    }
    assert!(characteristic_failures > 0);
    let summary = json!({"schema":"n83.radix-development/v1","partitions_checked":partitions_checked,"domain_checks":domain_checks,"matrix_checks":rows.len(),"nonmaximal_small_characteristic_maps":characteristic_failures,"production_census_performed":false,"rows":rows});
    std::fs::write(
        out.join("controls.json"),
        serde_json::to_string_pretty(&summary).unwrap() + "\n",
    )
    .unwrap();
    summary
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 2, "prime_radix_controls FRESH_OUTPUT");
    let mut summary = controls(Path::new(&args[1]));
    summary.as_object_mut().unwrap().remove("rows");
    println!("{summary}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn rejects_zero_part() {
        digit_radices(&[0, 2]);
    }
    #[test]
    #[should_panic]
    fn rejects_oversized_part() {
        digit_radices(&[28]);
    }
    #[test]
    #[should_panic]
    fn rejects_oversized_total() {
        digit_radices(&[14, 14]);
    }
    #[test]
    #[should_panic]
    fn rejects_empty_parts() {
        digit_radices(&[]);
    }
}

// Public bridge; development control implementation above is unchanged.
pub fn all_partitions(bits: usize) -> Vec<Vec<usize>> { let mut out = vec![]; partitions(bits, 1, &mut vec![], &mut out); out }
pub fn block_hilbert(radices: &[usize]) -> Vec<u128> { hilbert(radices) }
