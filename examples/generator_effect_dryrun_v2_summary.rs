//! Aggregate the Amendment 1 dry run of the generator-effect study
//! (research/generator_effect_ic_20261019): six arms, per-phase S at the
//! AGENTS.md section 5a weights, the pre-registered sumset hit-rate
//! prediction against measurement, and paired bootstrap ratios.
//!
//! `cargo run --release --example generator_effect_dryrun_v2_summary -- OUT.json PREREG.jsonl PHASE_DIR:YIELD_DIR...`
//!
//! One `PHASE_DIR:YIELD_DIR` pair per curve (an `ecbench` session with the
//! per-phase readings and one with the long yield block).  Warm-up rounds
//! are dropped.  Unpriced native counters (`cost.unpriced`, `<name>_uncharged`)
//! are priced at `w` in {0, 0.1, 1} GAE per operation.
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const PHASES: [&str; 7] = [
    "setup",
    "search",
    "factor_base",
    "oracle_setup",
    "relations",
    "linear_algebra",
    "verify",
];
const RESAMPLES: usize = 10_000;

struct Run {
    workload: String,
    round: u64,
    r: f64,
    phase: BTreeMap<String, (f64, f64)>,
    total_gae: f64,
    unpriced_ops: f64,
    found: f64,
    tried: f64,
    verified: bool,
    solve_wall_ns: f64,
    /// Table lookups the record keeps at record level (the generic tables).
    lookups: f64,
}

fn parse(v: &Value) -> Option<(String, Run)> {
    if v["warmup"].as_bool()? {
        return None;
    }
    let arm = v["arm"].as_str()?.to_string();
    let unpriced: Vec<String> = v["cost"]["unpriced"]
        .as_array()?
        .iter()
        .filter_map(|x| x.as_str()?.strip_suffix("_uncharged").map(String::from))
        .collect();
    let phases = v["phases"].as_array()?;
    let mut phase = BTreeMap::new();
    let mut total_ops = 0.0;
    for p in phases {
        let mut ops = 0.0;
        if let Some(native) = p["native"].as_object() {
            for k in &unpriced {
                ops += native.get(k).and_then(Value::as_f64).unwrap_or(0.0);
            }
        }
        total_ops += ops;
        phase.insert(p["name"].as_str()?.to_string(), (p["gae"].as_f64()?, ops));
    }
    if total_ops == 0.0 {
        // Counters kept at the record level (rho, the generic tables).
        for k in &unpriced {
            total_ops += v["counters"][format!("{k}_uncharged")]
                .as_f64()
                .or_else(|| v["counters"][k.as_str()].as_f64())
                .unwrap_or(0.0);
        }
    }
    let c = &v["counters"];
    Some((
        arm,
        Run {
            workload: v["workload"]["workload_id"].as_str()?.to_string(),
            round: v["round"].as_u64()?,
            r: v["workload"]["curve"]["r"].as_f64()?,
            phase,
            total_gae: v["cost"]["total_gae"].as_f64()?,
            unpriced_ops: total_ops,
            found: c["relations_found"].as_f64().unwrap_or(0.0),
            tried: c["targets_tried"].as_f64().unwrap_or(0.0),
            verified: v["outcome"]["status"] == "verified",
            solve_wall_ns: v["time"]["solve_wall_ns"].as_f64().unwrap_or(0.0),
            lookups: c["lookups_uncharged"].as_f64().unwrap_or(0.0),
        },
    ))
}

fn s_at(x: &Run, w: f64) -> f64 {
    (x.total_gae + w * x.unpriced_ops) / x.r.sqrt()
}
fn phase_s(x: &Run, name: &str, w: f64) -> f64 {
    x.phase
        .get(name)
        .map_or(0.0, |(g, o)| (g + w * o) / x.r.sqrt())
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len().max(1) as f64
}
fn boot(rng: &mut StdRng, n: usize, stat: impl Fn(&[usize]) -> f64) -> [f64; 2] {
    let mut v: Vec<f64> = (0..RESAMPLES)
        .map(|_| {
            let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
            stat(&idx)
        })
        .collect();
    v.sort_by(|a, b| a.total_cmp(b));
    [v[RESAMPLES * 5 / 100], v[RESAMPLES * 95 / 100]]
}

fn load(dir: &str) -> BTreeMap<String, Vec<Run>> {
    let text = std::fs::read_to_string(format!("{dir}/records.jsonl")).unwrap();
    let mut arms: BTreeMap<String, Vec<Run>> = BTreeMap::new();
    for line in text.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        if let Some((arm, run)) = parse(&v) {
            arms.entry(arm).or_default().push(run);
        }
    }
    for runs in arms.values_mut() {
        runs.sort_by(|a, b| (&a.workload, a.round).cmp(&(&b.workload, b.round)));
    }
    arms
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let prereg: Vec<Value> = std::fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let mut rng = StdRng::seed_from_u64(20261019);
    let mut curves = Vec::new();
    for pair in &args[2..] {
        let (pdir, ydir) = pair.split_once(':').unwrap();
        let phase_arms = load(pdir);
        let yield_arms = load(ydir);
        let r = phase_arms["rho-neg"][0].r;
        // ---- per-arm S, per phase
        let mut arm_json = serde_json::Map::new();
        for (arm, runs) in &phase_arms {
            let n = runs.len();
            let mut by_w = Vec::new();
            for &w in &WEIGHTS {
                let total: Vec<f64> = runs.iter().map(|x| s_at(x, w)).collect();
                let ci = boot(&mut rng, n, |i| {
                    mean(&i.iter().map(|&k| total[k]).collect::<Vec<_>>())
                });
                let mut phases = serde_json::Map::new();
                for ph in PHASES {
                    let x: Vec<f64> = runs.iter().map(|x| phase_s(x, ph, w)).collect();
                    phases.insert(ph.into(), json!(mean(&x)));
                }
                by_w.push(
                    json!({"w": w, "s_mean": mean(&total), "s_ci90": ci, "phases_s": phases}),
                );
            }
            arm_json.insert(
                arm.clone(),
                json!({
                    "runs": n,
                    "verified": runs.iter().filter(|x| x.verified).count(),
                    "relations_found": runs.iter().map(|x| x.found).sum::<f64>(),
                    "targets_tried": runs.iter().map(|x| x.tried).sum::<f64>(),
                    "by_weight": by_w,
                }),
            );
        }
        // ---- paired total-S ratios of every arm against every other (section 5a)
        let names: Vec<&String> = phase_arms.keys().collect();
        let mut order = Vec::new();
        for a in &names {
            for b in &names {
                if a >= b {
                    continue;
                }
                let (ra, rb) = (&phase_arms[*a], &phase_arms[*b]);
                if ra.len() != rb.len() {
                    continue;
                }
                let mut rows = Vec::new();
                let mut below = true;
                let mut above = true;
                for &w in &WEIGHTS {
                    let all: Vec<usize> = (0..ra.len()).collect();
                    let ratio = |i: &[usize]| {
                        mean(&i.iter().map(|&j| s_at(&ra[j], w)).collect::<Vec<_>>())
                            / mean(&i.iter().map(|&j| s_at(&rb[j], w)).collect::<Vec<_>>())
                    };
                    let ci = boot(&mut rng, ra.len(), ratio);
                    below &= ci[1] < 1.0;
                    above &= ci[0] > 1.0;
                    rows.push(json!({"w": w, "ratio": ratio(&all), "ci90": ci}));
                }
                order.push(json!({
                    "a": a, "b": b, "a_over_b": rows,
                    "a_below_b_at_every_weight": below,
                    "a_above_b_at_every_weight": above,
                }));
            }
        }
        // ---- multi-target model from measured parts.  The first key is the
        // measured run.  Each further key under the same G costs the arm's
        // measured per-key online S: for the known-log arms a run stops at its
        // first relation, so (relations + verify) of one run is exactly that
        // (walk set-up included).  The abscissa run collects ~B relations, so
        // its per-key cost is not read from its own run; it is taken equal to
        // the random-scalar arm's measured one (same relation law, walk and
        // probe; the yield ratio below is the check on that assumption).
        let key_cost = |runs: &[Run], i: &[usize], w: f64| {
            mean(
                &i.iter()
                    .map(|&j| phase_s(&runs[j], "relations", w) + phase_s(&runs[j], "verify", w))
                    .collect::<Vec<_>>(),
            )
        };
        let ia = &phase_arms["ic-abscissa"];
        let ik = &phase_arms["ic-knownlog"];
        let ir = &phase_arms["ic-knownlog-random"];
        let first = |runs: &[Run], i: &[usize], w: f64| {
            mean(&i.iter().map(|&j| s_at(&runs[j], w)).collect::<Vec<_>>())
        };
        let mut multi = Vec::new();
        for &w in &WEIGHTS {
            for t in [1.0f64, 4.0, 16.0, 64.0] {
                let cost = |which: usize, i: &[usize]| match which {
                    0 => (first(ia, i, w) + (t - 1.0) * key_cost(ir, i, w)) / t,
                    1 => (first(ik, i, w) + (t - 1.0) * key_cost(ik, i, w)) / t,
                    _ => (first(ir, i, w) + (t - 1.0) * key_cost(ir, i, w)) / t,
                };
                let all: Vec<usize> = (0..ia.len()).collect();
                let rho = &phase_arms["rho-neg"];
                let rho_s = |i: &[usize]| first(rho, i, w);
                let b_s = |i: &[usize]| first(&phase_arms["bsgs-neg-B"], i, w);
                let t_s = |i: &[usize]| first(&phase_arms["bsgs-neg-tbl"], i, w);
                // BSGS reuses its baby table across keys: per further key it pays
                // the giant steps (the search phase) and their lookups only.
                let bsgs_cost = |arm: &str, i: &[usize]| {
                    let x = &phase_arms[arm];
                    let per_key = mean(
                        &i.iter()
                            .map(|&j| {
                                phase_s(&x[j], "search", w) + w * x[j].lookups / x[j].r.sqrt()
                            })
                            .collect::<Vec<_>>(),
                    );
                    (first(x, i, w) + (t - 1.0) * per_key) / t
                };
                let ci_ratio = boot(&mut rng, ia.len(), |i| cost(2, i) / cost(0, i));
                let ci_rho = boot(&mut rng, ia.len(), |i| cost(2, i) / rho_s(i));
                let ci_abs_rho = boot(&mut rng, ia.len(), |i| cost(0, i) / rho_s(i));
                multi.push(json!({
                    "w": w, "T": t,
                    "abscissa_s_per_key": cost(0, &all),
                    "knownlog_s_per_key": cost(1, &all),
                    "knownlog_random_s_per_key": cost(2, &all),
                    "rho_neg_s": rho_s(&all), "bsgs_neg_B_s": b_s(&all), "bsgs_neg_tbl_s": t_s(&all),
                    "random_over_abscissa": cost(2, &all) / cost(0, &all),
                    "random_over_abscissa_ci90": ci_ratio,
                    "random_over_rho": cost(2, &all) / rho_s(&all),
                    "random_over_rho_ci90": ci_rho,
                    "abscissa_over_rho": cost(0, &all) / rho_s(&all),
                    "abscissa_over_rho_ci90": ci_abs_rho,
                    "bsgs_neg_B_s_per_key": bsgs_cost("bsgs-neg-B", &all),
                    "bsgs_neg_tbl_s_per_key": bsgs_cost("bsgs-neg-tbl", &all),
                    "random_over_bsgs_B": cost(2, &all) / bsgs_cost("bsgs-neg-B", &all),
                    "random_over_bsgs_tbl": cost(2, &all) / bsgs_cost("bsgs-neg-tbl", &all),
                    "random_over_bsgs_tbl_ci90": boot(&mut rng, ia.len(), |i| cost(2, i) / bsgs_cost("bsgs-neg-tbl", i)),
                    "random_over_bsgs_B_ci90": boot(&mut rng, ia.len(), |i| cost(2, i) / bsgs_cost("bsgs-neg-B", i)),
                }));
            }
        }
        let per_key_online = json!({
            "knownlog_random": key_cost(ir, &(0..ir.len()).collect::<Vec<_>>(), 0.0),
            "knownlog_progression": key_cost(ik, &(0..ik.len()).collect::<Vec<_>>(), 0.0),
            "w": 0.0,
        });
        // ---- yield block: measured against the pre-registered sumset law
        let mut yields = serde_json::Map::new();
        for (arm, runs) in &yield_arms {
            if runs[0].tried == 0.0 {
                continue;
            }
            let n = runs.len();
            let found: f64 = runs.iter().map(|x| x.found).sum();
            let tried: f64 = runs.iter().map(|x| x.tried).sum();
            let ci = boot(&mut rng, n, |i| {
                i.iter().map(|&k| runs[k].found).sum::<f64>()
                    / i.iter().map(|&k| runs[k].tried).sum::<f64>()
            });
            let pre = prereg
                .iter()
                .find(|p| p["r"].as_f64() == Some(r) && p["arm"].as_str() == Some(arm.as_str()));
            let mut entry = json!({
                "targets": n, "relations_found": found, "targets_tried": tried,
                "yield": found / tried, "yield_ci90": ci,
                "verified": runs.iter().filter(|x| x.verified).count(),
                "solve_wall_s_total": runs.iter().map(|x| x.solve_wall_ns).sum::<f64>() / 1e9,
            });
            if let Some(p) = pre {
                let pred = p["predicted_hit_rate"].as_f64().unwrap();
                entry["predicted"] = json!(pred);
                entry["predicted_sumset"] = p["sumset"].clone();
                entry["measured_over_predicted"] = json!(found / tried / pred);
                entry["measured_over_predicted_ci90"] = json!([ci[0] / pred, ci[1] / pred]);
                entry["prediction_inside_ci90"] = json!(ci[0] <= pred && pred <= ci[1]);
            }
            yields.insert(arm.clone(), entry);
        }
        // yield ratios against the abscissa baseline
        let mut yratios = serde_json::Map::new();
        if let Some(base) = yield_arms.get("ic-abscissa") {
            for (arm, runs) in &yield_arms {
                if arm == "ic-abscissa" || runs[0].tried == 0.0 {
                    continue;
                }
                let yld = |x: &[Run], i: &[usize]| {
                    i.iter().map(|&k| x[k].found).sum::<f64>()
                        / i.iter().map(|&k| x[k].tried).sum::<f64>()
                };
                let (na, nk) = (base.len(), runs.len());
                let all_a: Vec<usize> = (0..na).collect();
                let all_k: Vec<usize> = (0..nk).collect();
                let mut v: Vec<f64> = (0..RESAMPLES)
                    .map(|_| {
                        let ia: Vec<usize> = (0..na).map(|_| rng.gen_range(0..na)).collect();
                        let ik: Vec<usize> = (0..nk).map(|_| rng.gen_range(0..nk)).collect();
                        yld(runs, &ik) / yld(base, &ia)
                    })
                    .collect();
                v.sort_by(|x, y| x.total_cmp(y));
                yratios.insert(
                    arm.clone(),
                    json!({
                        "over_abscissa": yld(runs, &all_k) / yld(base, &all_a),
                        "ci90": [v[RESAMPLES * 5 / 100], v[RESAMPLES * 95 / 100]],
                    }),
                );
            }
        }
        curves.push(json!({
            "phase_dir": pdir, "yield_dir": ydir, "r": r, "log2_r": r.log2(),
            "arms": arm_json, "pairwise_total_s": order,
            "multi_target_model": multi, "per_key_online_s": per_key_online,
            "yield_block": yields, "yield_ratio_to_abscissa": yratios,
        }));
    }
    let doc = json!({
        "schema": "generator_effect_dryrun_v2_summary/v1",
        "status": "SCRATCH dry run, not data; PROTOCOL.md not frozen",
        "weights_gae_per_unpriced_op": WEIGHTS,
        "bootstrap": {"resamples": RESAMPLES, "interval": "percentile 90%", "seed": 20261019},
        "curves": curves,
    });
    std::fs::write(&args[0], serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
