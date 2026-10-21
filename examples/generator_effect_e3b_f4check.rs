//! Part F of E3b (research/generator_effect_ic_20261019/E3b-PROTOCOL.md):
//! F4 run to full solves against E3's substitution.
//!
//! `cargo run --release --example generator_effect_e3b_f4check -- OUT.json E3_SESSION_DIR F4_SESSION_DIR...`
//!
//! `E3_SESSION_DIR` is E3's e3-c1 session (its `sem-grid-t{t}` full solves
//! and its capped `sem-{chain,full}-t{t}` runs).  Each `F4_SESSION_DIR`
//! holds one uncapped `sem-{chain,full}-t{t}` arm on the same curve and
//! targets.  Per run it writes the measured counts, the same counts from
//! E3's grid record of that target, and the substituted cost E3 formed
//! (`U_grid − grid mults + consistent · grid mults per call + calls ·
//! (c̄_F4 + system mults per call)`, with E3's capped means), and the
//! ratio measured / substituted of `U` and of `S(w)` at `w ∈ {0, 0.1, 1}`.
//! Every number is derived from recorded counts; nothing is rerun.
use std::collections::BTreeMap;
use std::path::Path;

use crypto_lib::cryptanalysis::ecbench::band::{calibrated_native, unpriced_total};
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::runner::read_records;
use serde_json::{json, Value};

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];

fn native(rec: &Record, key: &str) -> f64 {
    rec.phases
        .iter()
        .filter_map(|p| p.native.get(key))
        .map(|&v| v as f64)
        .sum()
}

fn counter(rec: &Record, key: &str) -> f64 {
    rec.counters.get(key).copied().unwrap_or(0) as f64
}

/// `(g, u)` of `S(w) = (g + w·u) / √r`, as `ecbench::band` prices a record.
fn priced(rec: &Record) -> (f64, f64) {
    let (c_gae, n_c) = calibrated_native(rec).expect("calibration");
    let (u, missing) = unpriced_total(rec);
    assert!(
        !missing,
        "{}: an unpriced counter has no count",
        rec.record_id
    );
    (
        rec.cost.total_gae.unwrap_or(f64::NAN) - c_gae,
        u as f64 + n_c as f64,
    )
}

fn calls(rec: &Record) -> f64 {
    rec.solver.as_ref().map_or(0.0, |s| s.calls as f64)
}

fn solver_ops(rec: &Record) -> f64 {
    rec.solver.as_ref().map_or(0.0, |s| s.ops as f64)
}

fn group_gae(rec: &Record) -> f64 {
    rec.phases.iter().map(|p| p.gae).sum()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = &args[0];
    let e3 = read_records(Path::new(&args[1])).expect("E3 records");
    let by_arm = |arm: &str| -> BTreeMap<u64, &Record> {
        e3.iter()
            .filter(|r| r.arm == arm && !r.warmup)
            .map(|r| (r.workload.target_index, r))
            .collect()
    };
    let mut runs = Vec::new();
    for dir in &args[2..] {
        let recs = read_records(Path::new(dir)).expect("F4 records");
        for rec in recs
            .iter()
            .filter(|r| !r.warmup && r.arm.starts_with("sem-"))
        {
            let t: u32 = rec.arm.rsplit("-t").next().unwrap().parse().unwrap();
            let pres = rec.arm.split('-').nth(1).unwrap();
            let target = rec.workload.target_index;
            let grids = by_arm(&format!("sem-grid-t{t}"));
            let grid = grids[&target];
            // E3's capped means for this presentation and t.
            let capped: Vec<&Record> = by_arm(&format!("sem-{pres}-t{t}")).into_values().collect();
            let capped_calls: f64 = capped.iter().map(|r| calls(r)).sum();
            let c_f4 = capped.iter().map(|r| solver_ops(r)).sum::<f64>() / capped_calls;
            let sys = capped
                .iter()
                .map(|r| native(r, "pkm_system_fp_mults_uncharged"))
                .sum::<f64>()
                / capped_calls;
            let grid_mults = native(grid, "pkm_grid_fp_mults_uncharged");
            let grid_per_call = grid_mults / calls(grid);
            let consistent = native(grid, "pkm_consistent_systems");
            let (g_grid, u_grid) = priced(grid);
            let u_subst =
                u_grid - grid_mults + consistent * grid_per_call + calls(grid) * (c_f4 + sys);
            let (g, u) = priced(rec);
            let sqrt_r = (rec.workload.curve.r as f64).sqrt();
            let by_w: Vec<Value> = WEIGHTS
                .iter()
                .map(|&w| {
                    let (sm, ss) = ((g + w * u) / sqrt_r, (g_grid + w * u_subst) / sqrt_r);
                    json!({"w": w, "s_measured": sm, "s_substituted": ss, "ratio": sm / ss})
                })
                .collect();
            let side = |r: &Record| {
                json!({
                    "targets_tried": counter(r, "targets_tried"),
                    "relations_found": counter(r, "relations_found"),
                    "group_gae": group_gae(r),
                    "g": priced(r).0,
                    "calls": calls(r),
                })
            };
            runs.push(json!({
                "session": rec.session_id,
                "arm": rec.arm,
                "t": t,
                "presentation": pres,
                "target_index": target,
                "status": rec.outcome.status,
                "measured": side(rec),
                "e3_grid": side(grid),
                "same_stream": counter(rec, "targets_tried") == counter(grid, "targets_tried")
                    && counter(rec, "relations_found") == counter(grid, "relations_found")
                    && group_gae(rec) == group_gae(grid),
                "f4_muladds": solver_ops(rec),
                "f4_muladds_per_call": solver_ops(rec) / calls(rec),
                "e3_capped_mean_per_call": c_f4,
                "system_mults_per_call": native(rec, "pkm_system_fp_mults_uncharged") / calls(rec),
                "crosscheck_disagreements": native(rec, "pkm_crosscheck_disagreements"),
                "budget_exceeded": rec.solver.as_ref().map_or(0, |s| s.budget_exceeded),
                "f4_wall_ns_per_call": rec.solver.as_ref().map_or(0.0, |s| s.wall_ns as f64) / calls(rec),
                "u_measured": u,
                "u_substituted": u_subst,
                "u_ratio": u / u_subst,
                "by_w": by_w,
            }));
        }
    }
    let doc = json!({
        "schema": "generator-effect-e3b-f4check/v1",
        "protocol": "research/generator_effect_ic_20261019/E3b-PROTOCOL.md",
        "weights": WEIGHTS,
        "runs": runs,
    });
    std::fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
