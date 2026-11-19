//! A shared store of factor-base logarithm tables, keyed by curve,
//! generator and materialised factor base.
//!
//! The store itself (key, file format, insert, lookup and re-verification)
//! lives in the library, [`crypto_lib::cryptanalysis::koblitz_log_store`],
//! so the `ecbench` index-calculus methods share it; this module re-exports
//! it for the `ic` commands unchanged.
//!
//! A run that finds its table here skips relation collection and linear
//! algebra and goes straight to the individual logarithm.  That makes the
//! run *amortized*, not cold: the precomputation was paid by an earlier
//! run.  Every report that uses the store says so ([`provenance`]), so an
//! amortized cost cannot pass for a cold one.
#[allow(unused_imports)]
pub use crypto_lib::cryptanalysis::koblitz_log_store::{
    dir_from, factor_base_sha256, generator, insert, key_for, key_from_parts, lookup, path_for,
    provenance, stored_doc, verify_doc, Inserted, StoreKey, FACTOR_BASE_SCHEME, KEY_SCHEME,
    STORE_ENV,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment::{self, curve, log_table_to_doc, materialize, GeneratorPoint, Solver};
    use crate::params;
    use crypto_lib::binary_ecc::BinaryPoint;
    use crypto_lib::cryptanalysis::koblitz_factor_base_search::FactorBaseSpec;
    use num_bigint::BigUint;
    use std::path::PathBuf;

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "ic-log-store-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn spec(index: usize) -> FactorBaseSpec {
        FactorBaseSpec::Factor { index }
    }

    #[test]
    fn the_key_is_deterministic_and_binds_curve_and_base() {
        let c = curve(9, 0, 1, 1).unwrap();
        let fb = materialize(&c, &spec(0)).unwrap();
        let k1 = key_for(&c, &fb).unwrap();
        let k2 = key_for(
            &curve(9, 0, 1, 1).unwrap(),
            &materialize(&c, &spec(0)).unwrap(),
        )
        .unwrap();
        assert_eq!(k1, k2);
        assert_eq!(k1.key.len(), 64);
        // The point order does not matter; the points do.
        let mut shuffled = fb.clone();
        shuffled.points.reverse();
        assert_eq!(factor_base_sha256(&shuffled), k1.factor_base_sha256);
        let mut smaller = fb.clone();
        smaller.points.pop();
        assert_ne!(factor_base_sha256(&smaller), k1.factor_base_sha256);
        // Another curve, another key.
        let c1 = curve(9, 1, 1, 1).unwrap();
        let fb1 = materialize(&c1, &spec(0)).unwrap();
        assert_ne!(key_for(&c1, &fb1).unwrap().key, k1.key);
    }

    #[test]
    fn a_different_generator_or_base_gives_a_different_key() {
        let c = curve(9, 0, 1, 1).unwrap();
        let fb = materialize(&c, &spec(0)).unwrap();
        let k = key_for(&c, &fb).unwrap();
        // Generator: [2]G in place of G.
        let g2 = match c.mul(c.generator(), &BigUint::from(2u32)) {
            BinaryPoint::Affine { x, y } => GeneratorPoint {
                x: params::hex(&x.to_biguint()),
                y: params::hex(&y.to_biguint()),
            },
            BinaryPoint::Infinity => unreachable!(),
        };
        let kg = key_from_parts(&c, &g2, &k.factor_base_sha256).unwrap();
        assert_ne!(kg.key, k.key);
        // Base: one point fewer.
        let mut fb2 = fb.clone();
        fb2.points.pop();
        assert_ne!(key_for(&c, &fb2).unwrap().key, k.key);
    }

    /// Precompute once into the store, then: a lookup is a verified hit,
    /// a second insert keeps the first file, and a corrupted log is
    /// refused rather than used.
    #[test]
    fn a_store_round_trip_reverifies_and_rejects_corruption() {
        let c = curve(9, 0, 1, 1).unwrap();
        let sp = spec(0);
        let fb = materialize(&c, &sp).unwrap();
        let opts = experiment::ic_options(Solver::PairTable, 2, 200_000, 7);
        let (table, report) =
            crypto_lib::cryptanalysis::koblitz_index_calculus::solve_factor_base_logs(
                &c, &fb, &opts,
            )
            .expect("columns");
        assert!(report.verified);
        let doc = stored_doc(&c, &sp, &fb, 2, Solver::PairTable, &table).unwrap();
        let dir = tmpdir("roundtrip");
        assert!(lookup(&dir, &c, &fb).unwrap().is_none());
        let (how, key, path) = insert(&dir, &c, &fb, &doc).unwrap();
        assert_eq!(how, Inserted::Written);
        assert_eq!(path, path_for(&dir, &key));
        let (hit, _, hit_key, _) = lookup(&dir, &c, &fb).unwrap().expect("hit");
        assert_eq!(hit_key, key);
        assert_eq!(hit.columns, table.columns);
        let (again, _, _) = insert(&dir, &c, &fb, &doc).unwrap();
        assert_eq!(again, Inserted::AlreadyPresent);
        // No temporary files left behind.
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);

        // Corrupt one logarithm in place.
        let mut bad = doc.clone();
        let log = params::number(&bad.columns[0].log).unwrap() + 1u32;
        bad.columns[0].log = (log % &c.subgroup_order).to_string();
        std::fs::remove_file(&path).unwrap();
        std::fs::write(&path, serde_json::to_string(&bad).unwrap()).unwrap();
        let err = lookup(&dir, &c, &fb).unwrap_err();
        assert!(err.contains("re-verification"), "{err}");
        // And an insert never replaces the bad file with a good one silently.
        assert!(insert(&dir, &c, &fb, &doc).is_err());
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            serde_json::to_string(&bad).unwrap()
        );

        // A table from another base under this key is refused.
        let mut other = doc.clone();
        other.factor_base_sha256 = Some("00".repeat(32));
        std::fs::remove_file(&path).unwrap();
        std::fs::write(&path, serde_json::to_string(&other).unwrap()).unwrap();
        assert!(lookup(&dir, &c, &fb)
            .unwrap_err()
            .contains("different factor base"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `ic logs --log-store` computes once and reuses after; `ic solve
    /// --log-store` finds the table by key and says it did not pay for it.
    #[test]
    fn logs_and_solve_share_the_store_and_report_the_source() {
        use clap::Parser;
        #[derive(Parser)]
        struct Logs {
            #[command(flatten)]
            a: experiment::LogsArgs,
        }
        #[derive(Parser)]
        struct Solve {
            #[command(flatten)]
            a: experiment::SolveArgs,
        }
        let dir = tmpdir("cli");
        let d = dir.to_str().unwrap();
        // Neither --database nor --log-store: refused by the parser.
        assert!(Logs::try_parse_from(["logs", "--degree", "9"]).is_err());
        let args = || {
            Logs::try_parse_from(["logs", "--degree", "9", "--log-store", d])
                .unwrap()
                .a
        };
        let first = experiment::logs(args(), true).unwrap();
        assert_eq!(first["status"], "complete");
        assert_eq!(first["log_table"]["log_table_source"], "computed");
        assert_eq!(first["log_table"]["precomputation_paid_in_this_run"], true);
        assert_eq!(first["log_table"]["store_write"], "written");
        assert!(first["counts"]["trials"].as_u64().unwrap() > 0);
        let second = experiment::logs(args(), true).unwrap();
        assert_eq!(second["log_table"]["log_table_source"], "store");
        assert_eq!(
            second["log_table"]["precomputation_paid_in_this_run"],
            false
        );
        assert_eq!(second["log_table"]["key"], first["log_table"]["key"]);
        assert_eq!(second["counts"]["trials"], 0);
        assert_eq!(second["counts"]["relations"], 0);
        let solve = Solve::try_parse_from(["solve", "--degree", "9", "--log-store", d])
            .unwrap()
            .a;
        let out = experiment::solve(solve, true).unwrap();
        assert_eq!(out["status"], "complete");
        assert_eq!(out["result"]["verified"], true);
        assert_eq!(out["log_table"]["log_table_source"], "store");
        assert_eq!(out["log_table"]["key"], first["log_table"]["key"]);
        assert_eq!(out["log_table"]["cost_basis"], "amortized");
        // Another base has no entry: an error, not a fallback.
        let other = Solve::try_parse_from([
            "solve",
            "--degree",
            "9",
            "--log-store",
            d,
            "--factor-index",
            "1",
        ])
        .unwrap()
        .a;
        if let Err(e) = experiment::solve(other, true) {
            assert!(
                e.contains("no stored logarithm table") || e.contains("factor"),
                "{e}"
            );
        } else {
            panic!("a missing store entry must not solve");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_schema_one_document_still_loads() {
        let c = curve(9, 0, 1, 1).unwrap();
        let sp = spec(0);
        let fb = materialize(&c, &sp).unwrap();
        let opts = experiment::ic_options(Solver::PairTable, 2, 200_000, 7);
        let (table, _) = crypto_lib::cryptanalysis::koblitz_index_calculus::solve_factor_base_logs(
            &c, &fb, &opts,
        )
        .expect("columns");
        let v1 = log_table_to_doc(&c, &sp, 2, Solver::PairTable, &table);
        assert_eq!(v1.schema_version, 1);
        let text = serde_json::to_string(&v1).unwrap();
        assert!(!text.contains("generator") && !text.contains("factor_base_sha256"));
        let dir = tmpdir("v1");
        let path = dir.join("v1.json");
        std::fs::write(&path, &text).unwrap();
        let loaded = experiment::load_log_table(&path).unwrap();
        assert_eq!(loaded.schema_version, 1);
        assert_eq!(verify_doc(&c, &fb, &loaded).unwrap().columns, table.columns);
        // A v1 file is not accepted as a store entry.
        let key = key_for(&c, &fb).unwrap();
        std::fs::rename(&path, path_for(&dir, &key)).unwrap();
        assert!(lookup(&dir, &c, &fb).unwrap_err().contains("schema-2"));
        // A v1 document carrying v2 fields is refused at load.
        let mut mixed = stored_doc(&c, &sp, &fb, 2, Solver::PairTable, &table).unwrap();
        mixed.schema_version = 1;
        let p2 = dir.join("mixed.json");
        std::fs::write(&p2, serde_json::to_string(&mixed).unwrap()).unwrap();
        assert!(experiment::load_log_table(&p2).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
