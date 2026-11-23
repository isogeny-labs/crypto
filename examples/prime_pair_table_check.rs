//! Equivalence gate for the word-sized pair-sum table.
//!
//! `build_pair_sum_table` used to compute every pair sum with the BigUint
//! affine add (`Point::add`), i.e. one modular exponentiation per pair.  On
//! fields below 2^63 it now computes them in u64 with one extended-Euclid
//! inversion per pair.  This example re-derives the table with the BigUint
//! add and asserts the two agree exactly -- same abscissae, same index
//! pairs, same count -- so a table-stage speedup can never silently drop or
//! invent relations.
//!
//! ```bash
//! cargo run --release --example prime_pair_table_check -- 16 120
//! cargo run --release --example prime_pair_table_check -- 36 512
//! ```

use crypto_lib::cryptanalysis::ec_index_calculus::{build_factor_base, build_pair_sum_table};
use crypto_lib::cryptanalysis::research_bench::bench_curves_a_minus_3;
use crypto_lib::ecc::point::Point;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let bits: u32 = arguments.get(1).and_then(|a| a.parse().ok()).unwrap_or(16);
    let fb_size: usize = arguments.get(2).and_then(|a| a.parse().ok()).unwrap_or(120);
    let class = arguments.get(3).cloned().unwrap_or_else(|| "p256".into());

    let Some((_, curve)) = bench_curves_a_minus_3()
        .into_iter()
        .find(|(b, c)| *b == bits && c.name.contains(class.as_str()))
    else {
        eprintln!("no a=-3 bench curve at {bits} bits matching {class:?}");
        std::process::exit(2);
    };

    let factor_base = build_factor_base(&curve, fb_size);
    let table = build_pair_sum_table(&curve, &factor_base);
    let a_fe = curve.a_fe();

    // Reference build: the BigUint affine add this replaced.
    let mut want: Vec<(u64, u32, u32)> = Vec::new();
    for i in 0..factor_base.len() {
        for j in i..factor_base.len() {
            if let Point::Affine { x, .. } = factor_base[i].point.add(&factor_base[j].point, &a_fe)
            {
                want.push((
                    x.value.to_u64_digits().first().copied().unwrap_or(0),
                    i as u32,
                    j as u32,
                ));
            }
        }
    }

    let mut got: Vec<(u64, u32, u32)> = table.entries_word();
    let word_path_used = table.uses_word_path();
    want.sort_unstable();
    got.sort_unstable();
    let identical = got == want && table.pairs == want.len() && word_path_used;

    println!(
        "{}",
        serde_json::json!({
            "bits": bits,
            "curve": curve.name,
            "fb_size": factor_base.len(),
            "word_path_used": word_path_used,
            "pairs_word": got.len(),
            "pairs_biguint": want.len(),
            "identical": identical,
            "claim_boundary": "synthetic table-equivalence check; no timing claim",
        })
    );
    if !identical {
        std::process::exit(1);
    }
}
