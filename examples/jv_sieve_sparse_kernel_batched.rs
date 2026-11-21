//! Sparse solver integration for the extension-field IC relation matrix.
pub use crypto_lib::cryptanalysis;
#[path = "../src/cryptanalysis/jv_sparse_kernel.rs"]
mod sparse_kernel;
#[path = "../research/extension_ic_graph_20261115/sparse_kernel_driver.rs"]
mod sparse_kernel_driver;
#[path = "../research/extension_ic_graph_20261115/sparse_kernel_input.rs"]
mod sparse_kernel_input;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("prepare") if a.len() == 4 => {
            sparse_kernel_input::prepare(&a[1..3], std::path::Path::new(&a[3]))
        }
        Some(method) if a.len() == 3 => sparse_kernel_driver::run(
            method,
            std::path::Path::new(&a[1]),
            std::path::Path::new(&a[2]),
        ),
        _ => Err(
            "usage: WORKER prepare FULL_RAW QUADRATIC_RAW NEW_INPUT | METHOD INPUT NEW_OUTPUT"
                .into(),
        ),
    }
}

#[cfg(test)]
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/sparse_kernel.rs"]
mod scalar_reference;
#[cfg(test)]
mod batching_controls {
    use super::*;
    use crypto_lib::cryptanalysis::jv_bitops as work;
    fn compare(matrix: serde_json::Value, seed: u64) {
        let old: scalar_reference::Matrix = serde_json::from_value(matrix.clone()).unwrap();
        let new: sparse_kernel::Matrix = serde_json::from_value(matrix).unwrap();
        work::claim();
        work::set_moduli(1009, new.modulus);
        work::enable(true);
        let before = work::snapshot();
        let ub = work::uniform_snapshot();
        let scalar = scalar_reference::randomized_lanczos(&old, seed, 3).unwrap();
        let st = work::snapshot().since(&before);
        let su = work::uniform_snapshot().since(&ub).unwrap();
        work::enable(false);
        work::enable(true);
        let before = work::snapshot();
        let ub = work::uniform_snapshot();
        let batched = sparse_kernel::randomized_lanczos(&new, seed, 3).unwrap();
        let bt = work::snapshot().since(&before);
        let bu = work::uniform_snapshot().since(&ub).unwrap();
        work::enable(false);
        assert_eq!(
            serde_json::to_value(scalar).unwrap(),
            serde_json::to_value(batched).unwrap()
        );
        assert_eq!(work::prims_json(&st.total()), work::prims_json(&bt.total()));
        assert_eq!(
            work::uniform_json(&su.sum(&work::Phase::ALL).unwrap()),
            work::uniform_json(&bu.sum(&work::Phase::ALL).unwrap())
        );
        assert_eq!(work::foreign(), 0);
    }
    #[test]
    fn all_trace_fields_and_every_counter_match_scalar_reference() {
        for (l, n) in [(101, 4), (1009, 8), (263807419614139553, 12)] {
            let rows = (0..n)
                .map(|j| vec![(j, 1), (n, l - (3 * j + 2) as u64)])
                .collect::<Vec<_>>();
            for surplus in [false, true] {
                let mut cohort = rows.clone();
                if surplus {
                    cohort.push(Vec::new());
                    cohort.push(rows[0].clone());
                }
                for seed in [443, 444, 445] {
                    compare(
                        serde_json::json!({"modulus":l,"columns":n+1,"rows":cohort}),
                        seed,
                    );
                }
            }
        }
    }
    #[test]
    fn refused_attempt_prefixes_and_counters_match_too() {
        compare(
            serde_json::json!({"modulus":101,"columns":4,"rows":vec![vec![(0,1),(1,1),(2,1),(3,100)];3]}),
            443,
        );
    }
}
