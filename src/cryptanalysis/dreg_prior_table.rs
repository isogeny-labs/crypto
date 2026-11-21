//! Calibrated prior degree for the degree-of-regularity forecaster
//! (`dreg_forecast`).  Advisory only.
//!
//! Before any solve, the forecaster's prior puts half its weight at one
//! degree, the semi-regular degree by default.  On Boolean `m = 2`
//! binary-subspace systems that degree is biased: in
//! `isogeny-labs/ml-cryptanalysis`, the semi-regular formula sits about two
//! degrees above the measured solving degree.  This table gives a measured
//! degree for the cells the factor-base sweep can request: summands
//! `m = 2`, field degree `n ∈ {11, 13, 17, 19, 23, 29, 31}`, factor-base
//! dimension `2 ≤ k ≤ 8`, and shapes `coordinate`, `gp`, `scaled` and
//! `random`.
//!
//! Each cell's degree is the mode of the exact MutantXL solving degree
//! `D_mut` over 40 systems (20 seeds × Koblitz and random curve, planted
//! targets).  The labels and the leave-one-`n`-out backtest are in
//! ml-cryptanalysis `experiments/dreg-prior-calibration-20261112`.
//! MutantXL is a calibrated proxy for F4, not F4: it matched published F4
//! degrees on 10 of 11 control cells (ml-cryptanalysis
//! `experiments/ic-dreg`).  Outside the table the caller keeps the
//! semi-regular prior, so every other cell behaves exactly as before.

use serde::Deserialize;
use std::sync::OnceLock;

const TABLE_JSON: &str = include_str!("dreg_prior_table.json");

/// Recorded on every forecast that used the table.
pub const SOURCE: &str =
    "calibrated prior: mode of exact MutantXL D_mut (an F4 proxy) per (n, k, shape), \
     ml-cryptanalysis experiments/dreg-prior-calibration-20261112";

#[derive(Deserialize)]
struct Table {
    schema: String,
    cells: Vec<Cell>,
}

#[derive(Deserialize)]
struct Cell {
    n: u32,
    k: u32,
    family: String,
    mode: u32,
    rows: u32,
}

fn table() -> &'static Table {
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(|| {
        let t: Table = serde_json::from_str(TABLE_JSON).expect("embedded dreg prior table parses");
        assert_eq!(
            t.schema, "crypto.dreg-prior-table/v1",
            "unexpected dreg prior table schema"
        );
        t
    })
}

/// The table's family for a factor-base-sweep shape, or `None` for a shape
/// the table does not cover.
pub fn family_of_shape(shape: &str) -> Option<&'static str> {
    match shape {
        "coordinate" => Some("fppr"),
        "gp" => Some("gp_unit"),
        "scaled" => Some("gp_z"),
        "random" => Some("random"),
        _ => None,
    }
}

/// The calibrated prior degree for a Boolean `m = 2` system over `GF(2^n)`
/// whose factor base is a `k`-dimensional subspace of the given sweep
/// shape.  Returns `None` outside the table; the caller then keeps the
/// semi-regular prior.
pub fn calibrated_prior_degree(n: u32, k: u32, shape: &str) -> Option<u32> {
    let family = family_of_shape(shape)?;
    table()
        .cells
        .iter()
        .find(|c| c.n == n && c.k == k && c.family == family && c.rows > 0)
        .map(|c| c.mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_table_parses_and_covers_the_sweep_domain() {
        let t = table();
        assert!(!t.cells.is_empty());
        for c in &t.cells {
            assert!(
                c.mode >= 1 && c.mode <= 9,
                "implausible degree {} at {:?}",
                c.mode,
                (c.n, c.k)
            );
            assert!(c.rows > 0);
        }
        for n in [11, 13, 17, 19, 23, 29, 31] {
            for k in 2..=8 {
                for shape in ["coordinate", "gp", "scaled", "random"] {
                    assert!(
                        calibrated_prior_degree(n, k, shape).is_some(),
                        "missing cell {n} {k} {shape}"
                    );
                }
            }
        }
    }

    #[test]
    fn requests_outside_the_table_fall_back() {
        assert_eq!(calibrated_prior_degree(47, 4, "coordinate"), None);
        assert_eq!(calibrated_prior_degree(17, 9, "coordinate"), None);
        assert_eq!(calibrated_prior_degree(17, 4, "frobenius"), None);
        assert_eq!(family_of_shape("tracezero-gp"), None);
    }
}
