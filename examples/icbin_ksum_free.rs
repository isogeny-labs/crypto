//! Lane B (research/ic_binary_generic_incumbent_20261119): run `ic.pipeline` arms with the `ksum`
//! oracle on an explicit binary curve through the library entry points that
//! `ecbench::methods::solve_ic_binary` uses (ecbench at HEAD does not dispatch `ksum` on binary curves).
//!
//! Usage: icbin_ksum_free <config.json> <out.jsonl>
//! config: {"curve": {n, modulus, a, b, group_order, r, gx, gy}, "slug": "...",
//!          "targets": [["0x..","0x.."], ...], "rounds": R, "seed_base": S,
//!          "arms": [{"name": "...", "factor_base": "...", "oracle": "..."}, ...], "max_trials": T}
//! One JSON line per (arm, target, round): counted cost and counters. Counts are deterministic;
//! the algorithm seed is derived from (seed_base, target index, round), the same for every arm.
use crypto_lib::cryptanalysis::ecbench_large_prime::binary_explicit_instance;
use crypto_lib::cryptanalysis::ic_boundary::{BinaryGroup, Calibration, GroupOps};
use crypto_lib::cryptanalysis::ic_framework::plugins::{BinaryMultiplierBase, KSumOracle};
use crypto_lib::cryptanalysis::ic_framework::stages::{InstanceCtx, Params, Targets};
use crypto_lib::cryptanalysis::ic_framework::{run_pipeline_with_base, PipelineSpec};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use serde_json::{json, Value};
use std::io::Write;

fn hex(v: &Value) -> u64 {
    u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
}

fn phase(p: &crypto_lib::cryptanalysis::ic_boundary::PhaseCost) -> Value {
    json!({"adds": p.group_ops.adds, "doubles": p.group_ops.doubles, "scalar_mults": p.group_ops.scalar_mults,
           "gae": p.gae, "native": p.native, "work_native": p.work_native})
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cfg: Value = serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let mut out = std::io::BufWriter::new(std::fs::File::create(&args[2]).unwrap());
    let c = &cfg["curve"];
    let g = |k: &str| c[k].as_u64().unwrap();
    let inst = binary_explicit_instance(
        g("n") as u32, g("modulus"), g("a"), g("b"), g("group_order"), g("r"), g("gx"), g("gy"),
    )
    .unwrap();
    let slug = cfg["slug"].as_str().unwrap();
    let rounds = cfg["rounds"].as_u64().unwrap();
    let seed_base = cfg["seed_base"].as_u64().unwrap();
    let max_trials = cfg["max_trials"].as_u64().unwrap();
    let group = BinaryGroup(&inst.fast);
    let regime = if inst.koblitz.is_some() { "koblitz" } else { "char2" };
    let targets = cfg["targets"].as_array().unwrap();
    for arm in cfg["arms"].as_array().unwrap() {
        let name = arm["name"].as_str().unwrap();
        let (fb_name, fb_params) = Params::parse_spec(arm["factor_base"].as_str().unwrap()).unwrap();
        let (or_name, or_params) = Params::parse_spec(arm["oracle"].as_str().unwrap()).unwrap();
        assert_eq!(or_name, "ksum");
        assert_eq!(fb_name, "binary-multiplier");
        let ms = or_params.u64_or("m", 2).unwrap() as u32;
        for (ti, t) in targets.iter().enumerate() {
            let target = FastPoint::affine(hex(&t[0]), hex(&t[1]));
            for round in 0..rounds {
                let seed = seed_base
                    .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    .wrapping_add((ti as u64) << 8 | round);
                let spec = PipelineSpec {
                    factor_base: fb_name.clone(),
                    factor_base_params: fb_params.clone(),
                    oracle: or_name.clone(),
                    oracle_params: or_params.clone(),
                    solver: None,
                    solver_params: Params::default(),
                    targets: Targets::parse("walk").unwrap(),
                    linalg: "incremental-gauss".into(),
                    max_trials,
                    seed,
                    solver_budget_seconds: 0,
                };
                let ctx = InstanceCtx {
                    group: &group,
                    generator: inst.generator,
                    target,
                    r: inst.r,
                    cofactor: inst.cofactor,
                    group_order: inst.group_order,
                    name: slug.to_string(),
                    field_degree: Some(inst.n),
                };
                let mut calib = Calibration { ns_per_add: 1.0, ns_per_double: 1.0, ..Default::default() };
                calib.pin(regime, slug);
                let base = BinaryMultiplierBase { instance: &inst };
                let mut oracle = KSumOracle::new(ms);
                let _ = GroupOps::default();
                let t0 = std::time::Instant::now();
                let res = run_pipeline_with_base(&ctx, &spec, &base, &mut oracle, u64::MAX, &calib, None);
                let wall = t0.elapsed().as_nanos() as u64;
                let line = match res {
                    Ok((rep, _fb)) => json!({
                        "arm": name, "target": ti, "round": round, "seed": seed, "ok": true,
                        "verified": rep.verified, "exhausted": rep.exhausted, "recovered": rep.recovered,
                        "r": rep.r, "total_gae": rep.total_gae, "s": rep.s,
                        "targets_tried": rep.decomposition.targets_tried,
                        "relations_found": rep.decomposition.relations_found,
                        "matrix_rows": rep.linear_algebra.rows, "matrix_rank": rep.linear_algebra.rank,
                        "matrix_dependent": rep.linear_algebra.dependent,
                        "phases": {
                            "factor_base": phase(&rep.factor_base.cost),
                            "oracle_setup": phase(&rep.decomposition.setup),
                            "relations": phase(&rep.decomposition.cost),
                            "linear_algebra": phase(&rep.linear_algebra.cost),
                            "verify": phase(&rep.verify)},
                        "wall_ns": wall,
                    }),
                    Err(e) => json!({"arm": name, "target": ti, "round": round, "ok": false, "error": e}),
                };
                writeln!(out, "{line}").unwrap();
            }
        }
    }
}
