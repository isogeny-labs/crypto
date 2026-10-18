//! Boolean F4 on a committed refutation-degree draw: the solving degree a
//! fall-aware Gröbner engine reaches, as a second instrument beside the
//! MutantXL closure ([`crate::cryptanalysis::koblitz_bench::sparse_closure_refutes`]).
//!
//! ```sh
//! cargo run --release --example dreg_f4_degree -- --draw 0 --budget-secs 1200 FILE.jsonl
//! ```
//!
//! Replays the draw exactly as `dreg_closure` does (same subspace basis,
//! same `x(R)`, same field, same chained `S₃` system) and runs
//! [`groebner_basis_f4`] with the field equations under a wall budget.  One
//! JSON line per draw: whether the reduced basis is `{1}` (refuted), the
//! highest step degree processed, the solving degree (highest degree that
//! added a basis element), the largest matrix, and the wall time.  A timed
//! out run is unmeasured, never a refutation.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_f4_f2::groebner_basis_f4;
use num_bigint::BigUint;
use serde_json::Value;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| -> Option<String> {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let only_draw: Option<u64> = flag("--draw").map(|v| v.parse().expect("--draw K"));
    let budget: u64 = flag("--budget-secs").map_or(1200, |v| v.parse().expect("--budget-secs S"));
    let satisfiable = args.iter().any(|a| a == "--satisfiable");
    let valued = ["--draw", "--budget-secs"];
    let files: Vec<&String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            !valued.contains(&a.as_str())
                && a.as_str() != "--satisfiable"
                && !(*i > 0 && valued.contains(&args[i - 1].as_str()))
        })
        .map(|(_, a)| a)
        .collect();
    for file in files {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            let sat = v["outcome"]["kind"] == "satisfiable";
            if v.get("control").is_some() || sat != satisfiable {
                continue;
            }
            if only_draw.is_some_and(|k| v["draw"].as_u64() != Some(k)) {
                continue;
            }
            let m = v.get("m").and_then(Value::as_u64).unwrap_or(3) as usize;
            let n = v["n"].as_u64().expect("n") as u32;
            let ell = v["ell"].as_u64().expect("ell") as usize;
            let irr = find_irreducible_sparse(n).expect("field");
            let st = FieldStructure::new(n, &irr);
            let b = F2mElement::one(n);
            let basis: Vec<F2mElement> = v["v_basis"]
                .as_array()
                .expect("v_basis")
                .iter()
                .map(|w| F2mElement::from_biguint(&BigUint::from(w.as_u64().unwrap()), n))
                .collect();
            let x_r = F2mElement::from_biguint(&BigUint::from(v["x_r"].as_u64().unwrap()), n);
            assert_eq!(basis.len(), ell);
            let sys = build_decomposition_system(&basis, &x_r, &b, m, &st).expect("system");
            assert_eq!(
                sys.n_vars as u64,
                v["n_vars"].as_u64().unwrap(),
                "{file}: rebuilt shape"
            );
            let (gb, stats) = groebner_basis_f4(
                sys.equations.clone(),
                sys.n_vars,
                Some(Duration::from_secs(budget)),
            );
            let refuted = !stats.timed_out
                && gb.len() == 1
                && gb[0].terms.len() == 1
                && gb[0].terms[0].mask == 0;
            println!(
                r#"{{"source":"{file}","cell":"{}","m":{m},"n":{n},"ell":{ell},"draw":{},"n_vars":{},"committed_outcome":{},"engine":"f4","budget_secs":{budget},"timed_out":{},"refuted":{refuted},"basis_len":{},"degree_reached":{},"solving_degree":{},"max_poly_degree":{},"steps":{},"matrix_rows_max":{},"matrix_cols_max":{},"word_xors":{},"peak_matrix_bytes":{},"secs":{:.3}}}"#,
                v["cell"].as_str().unwrap_or(""),
                v["draw"],
                sys.n_vars,
                v["outcome"],
                stats.timed_out,
                gb.len(),
                stats.degree_reached,
                stats.solving_degree,
                stats.max_poly_degree,
                stats.steps,
                stats.matrix_rows_max,
                stats.matrix_cols_max,
                stats.word_xors,
                stats.peak_matrix_bytes,
                stats.wall_ns as f64 / 1e9
            );
            eprintln!(
                "m={m} ({n},{ell}) draw {}: refuted={refuted} timed_out={} degree_reached={} solving_degree={} ({:.1}s)",
                v["draw"],
                stats.timed_out,
                stats.degree_reached,
                stats.solving_degree,
                stats.wall_ns as f64 / 1e9
            );
        }
    }
}
