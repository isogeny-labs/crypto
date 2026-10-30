//! The Macaulay matrix of the m = 4 PDP closure as an object (review D1).
//!
//! Replays committed decomposition draws (as `dreg_closure` does), builds the
//! dense MutantXL closure at each degree, and prints one JSON line per
//! (draw, degree) with:
//!
//! * `input`: per-degree-block profile of the unreduced Macaulay rows (what a
//!   Wiedemann-style solver would see), with `--input`;
//! * `pre`: profile of the plain XL echelon basis, before the closure;
//! * `post`: profile of the saturated closure basis (meaningless if
//!   `closure_refuted`, where the basis is the single row `1`);
//! * `base_word_ops`, `total_word_ops` (the closure's own, as in
//!   `dreg_closure`), `rounds` (products and word ops per closure round).
//!
//! ```sh
//! cargo run --release --example m4_matrix_profile -- --sat --to 4 --input \
//!     research/dreg_closure_driver_20261016/draws/m4-11-5.jsonl
//! ```
//!
//! Selectors: `--sat` replays satisfiable draws (saturating, non-refuting
//! closures), `--unsat` the unsatisfiable ones; `--draw K`, `--from D`,
//! `--to D`, `--max-cols C`.  Counted metrics only; `secs` is recorded for
//! reference and is not a measurement on a loaded host.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::inherited_f4::{
    macaulay_input_profile, take_closure_trace, trace_closure, ReducedBasis,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use num_bigint::BigUint;
use serde_json::Value;

fn monomials_up_to(n_vars: u64, d: u32) -> f64 {
    let (mut total, mut binom) = (0f64, 1f64);
    for i in 0..=d as u64 {
        if i > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i) as f64 / (i + 1) as f64;
    }
    total
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let has = |name: &str| args.iter().any(|a| a == name);
    let want_sat = has("--sat");
    let want_unsat = has("--unsat") || !want_sat;
    let with_input = has("--input");
    let from: u32 = flag("--from").map_or(3, |v| v.parse().expect("--from"));
    let to: u32 = flag("--to").map_or(4, |v| v.parse().expect("--to"));
    let max_cols: f64 = flag("--max-cols").map_or(160_000.0, |v| v.parse().expect("--max-cols"));
    let only_draw: Option<u64> = flag("--draw").map(|v| v.parse().expect("--draw"));
    let max_draws: usize = flag("--max-draws").map_or(usize::MAX, |v| v.parse().expect("--max-draws"));
    let valued = ["--from", "--to", "--max-cols", "--draw", "--max-draws"];
    let files: Vec<&String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            !a.starts_with("--") && !(*i > 0 && valued.contains(&args[i - 1].as_str()))
        })
        .map(|(_, a)| a)
        .collect();
    let mut done = 0usize;
    for file in files {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            let sat = v["outcome"]["kind"] == "satisfiable";
            if v.get("control").is_some() || (sat && !want_sat) || (!sat && !want_unsat) {
                continue;
            }
            if only_draw.is_some_and(|k| v["draw"].as_u64() != Some(k)) {
                continue;
            }
            if done >= max_draws {
                return;
            }
            done += 1;
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
            let sys = build_decomposition_system(&basis, &x_r, &b, m, &st).expect("system");
            assert_eq!(sys.n_vars as u64, v["n_vars"].as_u64().unwrap());
            let sub_vars = (m * ell) as u32;
            for d in from..=to {
                if monomials_up_to(sys.n_vars as u64, d) > max_cols {
                    eprintln!("degree {d}: C(N,<=d) over --max-cols, stopped");
                    break;
                }
                let started = std::time::Instant::now();
                let input = if with_input {
                    macaulay_input_profile(&sys.equations, sys.n_vars, d, sub_vars)
                        .map(|p| p.to_json())
                } else {
                    None
                };
                trace_closure(true);
                let Some((b, total, base, pre)) = ReducedBasis::closed_full_split(
                    &sys.equations,
                    sys.n_vars,
                    d,
                    d,
                    Some(sub_vars),
                ) else {
                    eprintln!("degree {d}: over the F4 size caps");
                    break;
                };
                let rounds = take_closure_trace();
                trace_closure(false);
                let refuted = b.is_refuted();
                let post = b.profile(sub_vars);
                let secs = started.elapsed().as_secs_f64();
                println!(
                    r#"{{"source":"{file}","cell":"{}","m":{m},"n":{n},"ell":{ell},"draw":{},"sat":{sat},"solutions":{},"n_vars":{},"degree":{d},"columns":{},"closure_refuted":{refuted},"saturated":{},"rank":{},"base_word_ops":{},"total_word_ops":{},"base_reduce_word_ops":{},"closure_products":{},"rounds":[{}],"input":{},"pre":{},"post":{},"secs":{secs:.2}}}"#,
                    v["cell"].as_str().unwrap_or(""),
                    v["draw"],
                    v["solutions"],
                    sys.n_vars,
                    b.columns(),
                    b.pending_falls() == 0,
                    b.rank(),
                    base.word_ops(),
                    total.word_ops(),
                    base.reduce_word_ops,
                    total.closure_products,
                    rounds
                        .iter()
                        .map(|r| format!(
                            r#"{{"products":{},"reduce_word_ops":{},"rows_after":{},"columns":{}}}"#,
                            r.products, r.reduce_word_ops, r.rows_after, r.columns
                        ))
                        .collect::<Vec<_>>()
                        .join(","),
                    input.unwrap_or_else(|| "null".into()),
                    pre.map_or("null".to_string(), |p| p.to_json()),
                    post.to_json(),
                );
                eprintln!(
                    "m={m} ({n},{ell}) draw {} degree {d}: cols {} rank {} ops {} ({secs:.1}s)",
                    v["draw"],
                    b.columns(),
                    b.rank(),
                    total.word_ops()
                );
            }
        }
    }
}
