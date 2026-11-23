//! **E37 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.44, G2.174): the
//! matched rho on E24's curves, walk (iii).**
//!
//! E27's Stage W (`e27_walk.rs`) on E24's curves
//! (`generate_klein_instance(p, seed, 16)`, the matched fold of E24, the
//! negation walk, E24's seeds `1000·seed + k`, `k < 16`): (i) schoolbook
//! products, serial; (ii) Karatsuba products, serial; (iii) Karatsuba
//! products with batched norm inversions (`rho4_folded_batched`).  (i) is
//! compared field for field with E24's recorded walks, (ii) and (iii) with
//! (i).  Walk cost excludes the `[d]G = Q` check (E25's convention).
//!
//! ```text
//! ./e37_walk_e24 --json experiments/44_pivot_rules_confirm/walk_e24.json
//! ```

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::quartic_folds::{
    generate_klein_instance, rho4_folded_batched, rho4_folded_with, RhoFold, RhoRun4F,
};
use rayon::prelude::*;
use serde_json::{json, Value};

fn mean(ws: &[RhoRun4F], f: impl Fn(&RhoRun4F) -> u64) -> f64 {
    ws.iter().map(|w| f(w) as f64).sum::<f64>() / ws.len().max(1) as f64
}

fn walk_muls(w: &RhoRun4F) -> u64 {
    w.fp_muls - w.verify_fp_muls
}

fn same_walk(a: &RhoRun4F, b: &RhoRun4F) -> bool {
    (
        a.steps,
        a.group_ops,
        a.cycle_escapes,
        a.recovered,
        a.correct,
        a.visit_digest,
    ) == (
        b.steps,
        b.group_ops,
        b.cycle_escapes,
        b.recovered,
        b.correct,
        b.visit_digest,
    )
}

/// Does a recorded E24 walk equal walk (i), field for field?
fn equals_record(rec: &Value, w: &RhoRun4F) -> bool {
    rec["steps"].as_u64() == Some(w.steps)
        && rec["group_ops"].as_u64() == Some(w.group_ops - w.verify_group_ops)
        && rec["fp_muls"].as_u64() == Some(w.fp_muls - w.verify_fp_muls)
        && rec["cycle_escapes"].as_u64() == Some(w.cycle_escapes)
        && rec["correct"].as_bool() == Some(w.correct)
        && (rec.get("recovered").is_none() || rec["recovered"].as_u64() == w.recovered)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let sizes: Vec<u64> = vec![101, 149, 197, 269, 397, 521];
    let seeds: Vec<u64> = vec![1, 2];
    let walks = 16u64;
    let pool = 16usize;
    let e24_dir = "experiments/29_quartic_pipeline_v4";
    let json_path = args
        .iter()
        .position(|a| a == "--json")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let jobs: Vec<(u64, u64)> = sizes
        .iter()
        .flat_map(|&p| seeds.iter().map(move |&s| (p, s)))
        .collect();
    let fold = RhoFold::Negation;
    let rows: Vec<Value> = jobs
        .par_iter()
        .map(|&(p, seed)| {
            let ki = generate_klein_instance(p, seed, 16).expect("E24's curve");
            let inst = ki.inst;
            let e24: Value = serde_json::from_str(
                &fs::read_to_string(format!("{e24_dir}/p{p}s{seed}.json")).expect("E24 record"),
            )
            .unwrap();
            let recorded: Vec<Value> = e24[0]["rho"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|r| r["fold"].as_str() == Some("Negation"))
                .collect();
            let ws: Vec<u64> = (0..walks).map(|k| seed * 1000 + k).collect();
            let school: Vec<RhoRun4F> = ws
                .par_iter()
                .map(|&s| rho4_folded_with(&inst, fold, s, false))
                .collect();
            let kara: Vec<RhoRun4F> = ws
                .par_iter()
                .map(|&s| rho4_folded_with(&inst, fold, s, true))
                .collect();
            let (batched, stats) =
                rho4_folded_batched(&inst, fold, &ws, pool, seed * 1_000_000 + 500_000, true);
            let replay_ok = recorded.len() == school.len()
                && school.iter().all(|w| {
                    recorded
                        .iter()
                        .find(|r| r["seed"].as_u64() == Some(w.seed))
                        .is_some_and(|r| equals_record(r, w))
                });
            let kara_ok = school.iter().zip(&kara).all(|(a, b)| same_walk(a, b));
            let batch_ok = school.iter().zip(&batched).all(|(a, b)| same_walk(a, b));
            let m = [
                mean(&school, walk_muls),
                mean(&kara, walk_muls),
                mean(&batched, walk_muls),
            ];
            let steps = mean(&school, |w| w.steps);
            let all_correct = school.iter().all(|w| w.correct);
            println!(
                "e37 walk p={p} seed={seed} replay(i)=E24 {replay_ok} (ii)=(i) {kara_ok} (iii)=(i) {batch_ok} correct {all_correct} | steps {steps:.0} | muls (i) {:.4e} (ii) {:.4e} (iii) {:.4e} | (i)/(iii) {:.4}",
                m[0], m[1], m[2], m[0] / m[2]
            );
            json!({"p": p, "seed": seed, "n": inst.n, "bits": (inst.n as f64).log2(),
                   "fold": "Negation", "replay_matches_e24": replay_ok, "e24_records": recorded.len(),
                   "karatsuba_identical": kara_ok, "batched_identical": batch_ok,
                   "all_correct": all_correct, "mean_steps": steps,
                   "mean_fp_muls": {"schoolbook": m[0], "karatsuba": m[1], "batched": m[2]},
                   "mean_fp_muls_note": "walk only: the [d]G = Q check is excluded",
                   "batch": stats,
                   "walks": {"schoolbook": school, "karatsuba": kara, "batched": batched}})
        })
        .collect();
    let out = json!({
        "experiment": "e37",
        "stage": "walk (iii) on E24's curves",
        "pool": pool,
        "walks_per_curve": walks,
        "curves": rows,
    });
    if let Some(path) = json_path {
        fs::write(&path, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
    }
}
