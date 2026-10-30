//! **E30 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.28): the trivial-
//! product audit on E25's matched walks.**  Each walk runs serially with E27's
//! products and the audit counter on; the counter is read per walk.  Walks
//! (ii) and (iii) of §9.25 perform the same products, so the saving applies to
//! both.  The IC side is `quartic_pipeline_gls --karatsuba --audit`.
//!
//! ```text
//! cargo run --release --example e30_audit -- --json experiments/35_invariants_lite/walk_audit.json
//! ```

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::gaudry_quartic::{
    reset_trivial_product_savings, set_trivial_product_audit, trivial_product_savings,
};
use crypto_lib::cryptanalysis::quartic_folds::{
    generate_gls_klein_instance, rho4_folded_with, RhoFold,
};
use serde_json::json;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let json_path = args
        .iter()
        .position(|a| a == "--json")
        .map(|i| args[i + 1].clone());
    set_trivial_product_audit(true);
    let mut rows = Vec::new();
    for p in [101u64, 149, 197, 269, 397] {
        for seed in [1u64, 2] {
            let inst = generate_gls_klein_instance(p, seed, 16).unwrap().inst;
            let gls_in_fp = inst.gls.map(|(cx, cy)| (cx.in_fp(), cy.in_fp()));
            for fold in [RhoFold::Gls, RhoFold::Negation] {
                let (mut muls, mut saved) = (0f64, 0f64);
                for k in 0..16u64 {
                    reset_trivial_product_savings();
                    let w = rho4_folded_with(&inst, fold, seed * 1000 + k, true);
                    // The counter covers the walk and its [d]G check; the
                    // walk's cost is priced without the check (E25's
                    // convention), so the saving is an upper bound on the
                    // walk's own.
                    muls += (w.fp_muls - w.verify_fp_muls) as f64;
                    saved += trivial_product_savings() as f64;
                }
                println!(
                    "e30 walk p={p} seed={seed} {:?} | gls multipliers in F_p {:?} | mean (ii) {:.4e} saved {:.1} ({:.4} %)",
                    fold, gls_in_fp, muls / 16.0, saved / 16.0, 100.0 * saved / muls
                );
                rows.push(json!({"p": p, "seed": seed, "fold": format!("{fold:?}"),
                    "gls_multipliers_in_fp": gls_in_fp, "mean_walk_muls_karatsuba": muls / 16.0,
                    "mean_saved": saved / 16.0, "saved_fraction": saved / muls}));
            }
        }
    }
    if let Some(path) = json_path {
        fs::write(
            path,
            serde_json::to_string_pretty(&json!({"experiment": "e30", "rows": rows})).unwrap(),
        )
        .unwrap();
    }
}
