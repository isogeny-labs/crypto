//! L2 falsification, model part: combine the MEASURED |D_exact| / relations-bitops
//! trajectory (l2_collect) with LEADS.md L2's descent model. Everything the model
//! produces is MODEL-ONLY (critic F6) and is labelled so in the output.
//! [pcc:unverified]
//!
//! Model (scout-graph-ic/LEADS.md L1/L2, as written):
//!   f = |D_exact| / |D_universe|, |D_universe| = 254,530 canonical classes (l1-steps-2-3 step 2)
//!   m(f) = 1 + 15 f + 45 f^2 + 15 f^3
//!   E_desc(f) = 2 c / (pi * m(f)),  c = 1.2765e9 bitops per residual, pi = 9.0e-4
//! Variants (sensitivity, also model-only):
//!   A  as written;
//!   B  f scaled by L1's measured class match rate: f_eff = f * (482/6269) / (20510/254530);
//!   C  c / 1.68 (C1 trace replay applied, LEADS.md L2 "with C1 applied").
//! Kill rule (LEADS.md L2, as written): "The lead dies if the measured marginal descent
//! saving per added source falls below the measured marginal source cost at every
//! checkpoint past rank readiness." Checkpoint = complete batch (1024 B values), the
//! collector's own checkpoint unit. Marginal = difference to the previous checkpoint
//! divided by the sources added.
//!
//! Cross-check: the measured (b_count, sources, anchors) points are compared with
//! deferred-graph50/recorded49-graph-replay.json wherever the B counts coincide.
//!
//! Run: l2_model TRAJECTORY_JSON RECORDED49_REPLAY_JSON OUT_JSON
use serde_json::{json, Value};
const U: f64 = 254_530.0;
const C: f64 = 1.2765e9;
const PI: f64 = 9.0e-4;
fn m(f: f64) -> f64 {
    1.0 + 15.0 * f + 45.0 * f * f + 15.0 * f * f * f
}
fn e(f: f64, c: f64) -> f64 {
    2.0 * c / (PI * m(f))
}
fn n(v: &Value) -> f64 {
    match v {
        Value::String(s) => s.parse().unwrap(),
        o => o.to_string().parse().unwrap(),
    }
}
fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    let t: Value = serde_json::from_slice(&std::fs::read(&a[1]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    if t["control"]["pass"] != true {
        return Err("control did not pass; refusing to model".into());
    }
    let traj = t["trajectory"].as_array().ok_or("trajectory")?;
    let i0 = traj.iter().position(|r| r["rank_ready"] == true).ok_or("no rank-ready checkpoint")?;
    let scale_b = (482.0 / 6269.0) / (20510.0 / U);
    let variants = [("A_as_written", 1.0, C), ("B_match_rate_scaled_f", scale_b, C), ("C_c1_trace_replay", 1.0, C / 1.68)];
    let (s0, d0, r0) = (n(&traj[i0]["sources"]), n(&traj[i0]["d_exact"]), n(&traj[i0]["relations_bitops"]));
    let mut rows = Vec::new();
    let mut wins = [0u64; 3];
    for k in i0 + 1..traj.len() {
        let (p, q) = (&traj[k - 1], &traj[k]);
        let (s1, s2) = (n(&p["sources"]), n(&q["sources"]));
        let (d1, d2) = (n(&p["d_exact"]), n(&q["d_exact"]));
        let (r1, r2) = (n(&p["relations_bitops"]), n(&q["relations_bitops"]));
        let ds = s2 - s1;
        let cost = (r2 - r1) / ds;
        let mut per = serde_json::Map::new();
        for (j, (name, sc, c)) in variants.iter().enumerate() {
            let save = (e(sc * d1 / U, *c) - e(sc * d2 / U, *c)) / ds;
            let cum_save = e(sc * d0 / U, *c) - e(sc * d2 / U, *c);
            if save >= cost {
                wins[j] += 1;
            }
            per.insert(name.to_string(), json!({"f":sc*d2/U,"m":m(sc*d2/U),"e_desc":e(sc*d2/U,*c),"marginal_saving_per_source":save,"saving_ge_cost":save>=cost,
                "cumulative_saving_since_rank_stop":cum_save,"cumulative_net_since_rank_stop":cum_save-(r2-r0)}));
        }
        rows.push(json!({"b_count":q["b_count"],"sources":s2,"sources_over_rank_stop":s2/s0,"d_exact":d2,"added_sources":ds,"added_anchors":d2-d1,
            "anchors_per_source":(d2-d1)/ds,"measured_marginal_cost_per_source":cost,"extra_relations_bitops_since_rank_stop":r2-r0,"model_only":per}));
    }
    let n_cp = rows.len() as u64;
    // best stop by cumulative model net (variant-wise), and the 2x end point
    let mut best = serde_json::Map::new();
    for (name, _, _) in variants.iter() {
        let b = rows.iter().max_by(|x, y| n(&x["model_only"][*name]["cumulative_net_since_rank_stop"]).partial_cmp(&n(&y["model_only"][*name]["cumulative_net_since_rank_stop"])).unwrap()).unwrap();
        let last_win = rows.iter().filter(|r| r["model_only"][*name]["saving_ge_cost"] == true).last().map(|r| r["sources"].clone());
        best.insert(name.to_string(), json!({"best_sources":b["sources"],"best_sources_over_rank_stop":b["sources_over_rank_stop"],"best_cumulative_net":b["model_only"][*name]["cumulative_net_since_rank_stop"],
            "net_at_end":rows.last().unwrap()["model_only"][*name]["cumulative_net_since_rank_stop"],"last_checkpoint_with_saving_ge_cost_sources":last_win}));
    }
    // robustness (not the rule): non-overlapping windows of 10 checkpoints past rank readiness
    let mut windows = Vec::new();
    let mut k = i0;
    while k < traj.len() - 1 {
        let k2 = (k + 10).min(traj.len() - 1);
        let (p, q) = (&traj[k], &traj[k2]);
        let ds = n(&q["sources"]) - n(&p["sources"]);
        let cost = (n(&q["relations_bitops"]) - n(&p["relations_bitops"])) / ds;
        let mut per = serde_json::Map::new();
        for (name, sc, c) in variants.iter() {
            let save = (e(sc * n(&p["d_exact"]) / U, *c) - e(sc * n(&q["d_exact"]) / U, *c)) / ds;
            per.insert(name.to_string(), json!({"marginal_saving_per_source":save,"saving_ge_cost":save>=cost}));
        }
        windows.push(json!({"from_sources":p["sources"],"to_sources":q["sources"],"anchors_per_source":(n(&q["d_exact"])-n(&p["d_exact"]))/ds,"measured_marginal_cost_per_source":cost,"model_only":per}));
        k = k2;
    }
    let win_ge: Vec<u64> = variants.iter().map(|(name, _, _)| windows.iter().filter(|w| w["model_only"][*name]["saving_ge_cost"] == true).count() as u64).collect();
    // cross-check against the recorded Run49 graph replay trajectory
    let rec: Value = serde_json::from_slice(&std::fs::read(&a[2]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let (mut xc_n, mut xc_bad) = (0u64, 0u64);
    for r in rec["trajectory"].as_array().ok_or("recorded trajectory")? {
        if let Some(m) = traj.iter().find(|x| x["b_count"] == r["b_count"]) {
            xc_n += 1;
            if m["sources"] != r["sources"] || m["d_exact"] != r["anchors"] || m["vertices"] != r["vertices"] {
                xc_bad += 1;
            }
        }
    }
    let verdict = |w: u64| if n_cp == 0 { "no checkpoint past rank readiness" } else if w == 0 { "KILL (saving < cost at every checkpoint past rank readiness)" } else { "NOT KILLED (saving >= cost at some checkpoint)" };
    let out = json!({"schema":"l2-anchor-trajectory.model/v1","pcc":"unverified","label":"MODEL-ONLY: measured trajectory combined with LEADS.md L2 descent model; not a measured descent saving",
        "constants":{"d_universe":U,"c":C,"pi":PI,"b_scale":scale_b,"c1_factor":1.68},
        "rank_stop":{"sources":s0,"d_exact":d0,"relations_bitops":r0,"f":d0/U,"e_desc_A":e(d0/U,C)},
        "checkpoints_past_rank_readiness":n_cp,
        "checkpoints_with_saving_ge_cost":{"A_as_written":wins[0],"B_match_rate_scaled_f":wins[1],"C_c1_trace_replay":wins[2]},
        "rule_verdict":{"A_as_written":verdict(wins[0]),"B_match_rate_scaled_f":verdict(wins[1]),"C_c1_trace_replay":verdict(wins[2])},
        "best_stop_by_cumulative_net":best,"windows_of_10":{"count":windows.len(),"with_saving_ge_cost":{"A_as_written":win_ge[0],"B_match_rate_scaled_f":win_ge[1],"C_c1_trace_replay":win_ge[2]},"rows":windows},"recorded49_crosscheck":{"points":xc_n,"mismatches":xc_bad},"rows":rows});
    std::fs::write(&a[3], serde_json::to_string_pretty(&out).unwrap()).map_err(|e| e.to_string())?;
    println!("{}", json!({"windows":out["windows_of_10"]["with_saving_ge_cost"],"recorded49_crosscheck":out["recorded49_crosscheck"],"best":out["best_stop_by_cumulative_net"],"checkpoints":n_cp,"wins":out["checkpoints_with_saving_ge_cost"],"verdict":out["rule_verdict"]}));
    Ok(())
}
