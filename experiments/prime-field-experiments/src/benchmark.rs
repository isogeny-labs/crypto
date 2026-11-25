use std::time::Instant;

use anyhow::{ensure, Result};
use rand::{rngs::StdRng, seq::index::sample, Rng, SeedableRng};
use serde::Serialize;

use crate::poly::{Poly, PrimeField};
use crate::roots::{roots_of, RootRun};
use crate::run_jobs;

#[derive(Serialize)]
pub struct Row {
    pub case: &'static str,
    pub instance: usize,
    pub p: u64,
    pub input_degree: i64,
    pub root_count: usize,
    pub reflection_center: Option<u64>,
    pub baseline_extraction_degree: i64,
    pub reflection_extraction_degree: i64,
    pub baseline_median_us: f64,
    pub reflection_median_us: f64,
    pub flint_median_us: f64,
    pub baseline_over_reflection: f64,
}

/// Benchmark the two Rust paths and compare every root set with native FLINT.
pub fn run(repeats: usize, seed: u64, instances: usize, workers: usize) -> Result<Vec<Row>> {
    ensure!(
        repeats > 0 && instances > 0,
        "repeats and instances must be positive"
    );
    let instances: Vec<_> = (0..instances).collect();
    let batches = run_jobs(&instances, workers, |&instance| {
        run_instance(repeats, seed, instance)
    })?;
    Ok(batches.into_iter().flatten().collect())
}

fn run_instance(repeats: usize, seed: u64, instance: usize) -> Result<Vec<Row>> {
    let seed = seed.wrapping_add(instance as u64);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rows = Vec::new();
    for p in [1009, 65537, 2147483647] {
        let field = PrimeField::new(p)?;
        for degree in [16, 64, 256, 512] {
            let nonzero_roots = sample(&mut rng, (p - 1) as usize, degree / 2);
            let mut compressed = Poly::new(field, &[1]);
            for root in nonzero_roots {
                compressed = compressed.mul(&Poly::new(field, &[field.neg(root as u64 + 1), 1]));
            }
            let mut even = vec![0; degree + 1];
            for i in (0..=degree).step_by(2) {
                even[i] = compressed.coeff(i / 2);
            }
            let even = Poly::new(field, &even);
            let shifted = even.shift(p - 7);

            let mut asymmetric = Poly::new(field, &[1]);
            for root in sample(&mut rng, p as usize, degree / 4) {
                asymmetric = asymmetric.mul(&Poly::new(field, &[field.neg(root as u64), 1]));
            }
            let random: Vec<_> = (0..3 * degree / 4)
                .map(|_| rng.random_range(0..p))
                .chain([1])
                .collect();
            let asymmetric = asymmetric.mul(&Poly::new(field, &random));

            for (case, poly) in [
                ("even", even),
                ("shifted_even", shifted),
                ("asymmetric", asymmetric),
            ] {
                rows.push(time_case(repeats, seed, instance, case, &poly)?);
            }
        }
    }
    Ok(rows)
}

fn time_case(
    repeats: usize,
    seed: u64,
    instance: usize,
    case: &'static str,
    poly: &Poly,
) -> Result<Row> {
    let expected = poly.native_roots();
    let mut elapsed: [Vec<f64>; 3] = std::array::from_fn(|_| Vec::with_capacity(repeats));
    let mut diagnostics: [Option<RootRun>; 2] = [None, None];
    for repeat in 0..repeats {
        for algorithm in (0..3).map(|offset| (offset + repeat) % 3) {
            let start = Instant::now();
            let matches = if algorithm == 2 {
                let roots = poly.native_roots();
                elapsed[algorithm].push(start.elapsed().as_secs_f64() * 1e6);
                roots == expected
            } else {
                let run = roots_of(poly, algorithm == 1, seed.wrapping_add(repeat as u64))?;
                elapsed[algorithm].push(start.elapsed().as_secs_f64() * 1e6);
                let matches = run.roots == expected;
                diagnostics[algorithm] = Some(run);
                matches
            };
            ensure!(
                matches,
                "root mismatch for {case} at p={} degree={}",
                poly.field.modulus(),
                poly.degree()
            );
        }
    }

    let [baseline, reflection] = diagnostics.map(|run| run.expect("at least one repeat"));
    let [baseline_us, reflection_us, flint_us] = elapsed.map(median);
    Ok(Row {
        case,
        instance,
        p: poly.field.modulus(),
        input_degree: poly.degree(),
        root_count: expected.len(),
        reflection_center: reflection.reflection_center,
        baseline_extraction_degree: baseline.extraction_degree,
        reflection_extraction_degree: reflection.extraction_degree,
        baseline_median_us: round(baseline_us, 100.0),
        reflection_median_us: round(reflection_us, 100.0),
        flint_median_us: round(flint_us, 100.0),
        baseline_over_reflection: round(baseline_us / reflection_us, 1000.0),
    })
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_unstable_by(f64::total_cmp);
    let middle = values.len() / 2;
    if values.len().is_multiple_of(2) {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

fn round(value: f64, scale: f64) -> f64 {
    (value * scale).round() / scale
}
