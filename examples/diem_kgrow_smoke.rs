//! Selectable-`k` Diem extension-IC harness smoke.
//!
//! ```bash
//! cargo run --release --example diem_kgrow_smoke
//! ```
//!
//! Runs tiny `(p,k)` cells for `k = 2` and `k = 3`, plus one schedule-
//! derived `k`, printing JSON reports with catalog cost vs rho proxy.

use crypto_lib::cryptanalysis::diem_kgrow::{run_cell, CellRequest};

fn main() {
    let cells = [
        CellRequest {
            p: 5,
            k: Some(2),
            planted_decompositions: 6,
            trial_budget: 200_000,
            seed: 3,
            ..CellRequest::default()
        },
        CellRequest {
            p: 7,
            k: Some(3),
            planted_decompositions: 6,
            trial_budget: 2_000_000,
            seed: 1,
            ..CellRequest::default()
        },
        CellRequest {
            p: 7,
            k: None,
            k_min: 2,
            k_max: 3,
            planted_decompositions: 4,
            trial_budget: 2_000_000,
            seed: 2,
            ..CellRequest::default()
        },
    ];

    println!("=== diem_kgrow smoke (selectable-k Semaev/Diem lab slice) ===");
    for (i, req) in cells.iter().enumerate() {
        match run_cell(req) {
            Ok(rep) => {
                println!(
                    "[{i}] p={} k={} schedule_k={} recovered={}/{} fb={} ratio_log2={:.3}",
                    rep.p,
                    rep.k,
                    rep.schedule_k_of_p,
                    rep.recovered,
                    rep.planted,
                    rep.fb_size,
                    rep.ratio_log2
                );
                println!("{}", serde_json::to_string_pretty(&rep).unwrap());
            }
            Err(e) => {
                eprintln!("[{i}] FAIL: {e}");
                std::process::exit(1);
            }
        }
    }
    println!("smoke ok");
}
