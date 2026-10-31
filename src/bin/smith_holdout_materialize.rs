//! Materialize the preselected Smith-negative input identities from the
//! archived random-curve generator, and compare their archived verdicts.
use crypto_lib::cryptanalysis::smith_prop24::{random_curve, random_row};
use serde_json::{json, Value};
use std::fs;

fn main() {
    let path = "research/five_leads_e2e_20261019/fixtures/smith-negative-holdout-v1.json";
    let input: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let p = input["p"].as_u64().unwrap();
    let seed = input["source_seed"].as_u64().unwrap();
    for group in ["no_tractable", "trigonal_but_no_rational_isogeny"] {
        for saved in input["groups"][group].as_array().unwrap() {
            let index = saved["index"].as_u64().unwrap();
            let (f, _) = random_curve(p, seed, index);
            let fresh = random_row(p, seed, index);
            assert_eq!(fresh.cycle_type, saved["cycle_type"].as_str().unwrap());
            assert_eq!(
                fresh.n_tractable,
                saved["n_tractable"].as_u64().unwrap() as u32
            );
            assert_eq!(fresh.n_trig, saved["n_trig"].as_u64().unwrap() as u32);
            assert_eq!(fresh.n_iso, saved["n_iso"].as_u64().unwrap() as u32);
            assert!(!fresh.full_success);
            println!(
                "{}",
                json!({"schema":"smith-negative-materialized/v1","p":p,"seed":seed,
                "index":index,"group":group,"f_ascending":f,"cycle_type":fresh.cycle_type,
                "n_tractable":fresh.n_tractable,"n_trig":fresh.n_trig,"n_iso":fresh.n_iso,
                "archived_census_verdict_reproduced":true,"odd_kernel_screen_executed":false})
            );
        }
    }
}
