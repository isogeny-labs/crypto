//! Sparse left-kernel vector: same contract as the dense elimination, on rows
//! with only a few nonzeros per relation.

use std::collections::HashMap;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use ecdl_tensor_ic::field::{add_mod, mul_mod};
use ecdl_tensor_ic::{left_kernel_vector, left_kernel_vector_sparse};

const R: u64 = 10007;

fn random_rows(rng: &mut StdRng, ncols: usize, k: usize, nnz: usize) -> Vec<HashMap<usize, u64>> {
    (0..k)
        .map(|_| {
            let mut row = HashMap::new();
            for _ in 0..nnz {
                row.insert(rng.gen_range(0..ncols), rng.gen_range(0..R));
            }
            row
        })
        .collect()
}

fn check(rows: &[HashMap<usize, u64>], ncols: usize, c: &[u64]) -> Result<(), String> {
    if c.len() != rows.len() {
        return Err(format!(
            "combination has {} entries, want {}",
            c.len(),
            rows.len()
        ));
    }
    if !c.iter().any(|&v| v != 0) {
        return Err("zero combination".into());
    }
    let mut sum = vec![0u64; ncols];
    for (j, &cj) in c.iter().enumerate() {
        for (&col, &v) in &rows[j] {
            sum[col] = add_mod(sum[col], mul_mod(cj, v, R), R);
        }
    }
    match sum.iter().position(|&s| s != 0) {
        Some(col) => Err(format!("column {} sums to {}", col, sum[col])),
        None => Ok(()),
    }
}

// The sparse elimination must return a valid dependency wherever the dense one
// does, and never an invalid one.
#[test]
fn sparse_agrees_with_dense_on_random_systems() {
    let mut rng = StdRng::seed_from_u64(211);
    for trial in 0..300 {
        let ncols = 1 + rng.gen_range(0..18);
        let k = ncols + 1;
        let nnz = 1 + rng.gen_range(0..4);
        let rows = random_rows(&mut rng, ncols, k, nnz);
        let dense = left_kernel_vector(&rows, ncols, R).expect("dense found no dependency");
        check(&rows, ncols, &dense).expect("dense combination is invalid");
        let sparse = match left_kernel_vector_sparse(&rows, ncols, R) {
            Some(c) => c,
            None => panic!("trial {}: sparse found no dependency", trial),
        };
        if let Err(e) = check(&rows, ncols, &sparse) {
            panic!("trial {}: sparse combination invalid: {}", trial, e);
        }
    }
}

// Rank-deficient-by-several systems: some rows are linear combinations of the
// others, so the dependency can involve more than the last row.
#[test]
fn sparse_finds_dependencies_in_deficient_systems() {
    let mut rng = StdRng::seed_from_u64(223);
    for _ in 0..50 {
        let ncols = 4 + rng.gen_range(0..8);
        let mut rows = random_rows(&mut rng, ncols, 3, 2);
        // row 3 = row 0 + 2 * row 1
        let mut combo: HashMap<usize, u64> = HashMap::new();
        for (&c, &v) in &rows[0] {
            *combo.entry(c).or_insert(0) = v;
        }
        for (&c, &v) in &rows[1] {
            let e = combo.entry(c).or_insert(0);
            *e = add_mod(*e, mul_mod(2, v, R), R);
        }
        combo.retain(|_, &mut v| v != 0);
        rows.push(combo);
        let c = left_kernel_vector_sparse(&rows, ncols, R).expect("no dependency");
        check(&rows, ncols, &c).expect("invalid combination");
    }
}

// A system at the size a 20-bit solve produces: |F| ~ sqrt(2r) ~ 1400 per
// factor base, N ~ 2000 columns. The dense version allocates O(k(ncols + k))
// words here and does O(k^3) work; the sparse one must stay fast.
#[test]
fn sparse_handles_a_thousand_columns_quickly() {
    let mut rng = StdRng::seed_from_u64(227);
    let ncols = 2000;
    let rows = random_rows(&mut rng, ncols, ncols + 1, 3);
    let started = Instant::now();
    let c = left_kernel_vector_sparse(&rows, ncols, R).expect("no dependency");
    let elapsed = started.elapsed();
    check(&rows, ncols, &c).expect("invalid combination");
    assert!(
        elapsed.as_secs_f64() < 60.0,
        "sparse elimination took {:?} for {} columns",
        elapsed,
        ncols
    );
    eprintln!(
        "2000 columns, {} rows, 3 nonzeros: {:?}",
        rows.len(),
        elapsed
    );
}
