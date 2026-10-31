//! Aggregate an `ecbench` dry-run session for the generator-effect study
//! (research/generator_effect_ic_20261019): per-phase S at the AGENTS.md
//! section 5a weights, online yield, and paired bootstrap intervals.
//!
//! `cargo run --release --example generator_effect_dryrun_summary -- OUT.json SESSION_DIR...`
//!
//! Reads `records.jsonl` of each session (one curve each), drops warm-up
//! rounds, and writes one JSON document.  Unpriced native counters (the
//! record's `cost.unpriced` list, `<name>_uncharged`) are priced at
//! `w in {0, 0.1, 1}` group-addition equivalents per operation.
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const PHASES: [&str; 5] = [
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
    /// phase name -> (priced gae, unpriced operations)
    phase: BTreeMap<String, (f64, f64)>,
    /// relations phase: (gae of group work, lookups + guard probes)
    draw_probe: (f64, f64),
    total_gae: f64,
    unpriced_ops: f64,
    found: f64,
    tried: f64,
    verified: bool,
}

fn parse(v: &Value) -> Option<(String, Run)> {
    if v["warmup"].as_bool()? {
        return None;
    }
    let arm = v["arm"].as_str()?.to_string();
    let r = v["workload"]["curve"]["r"].as_f64()?;
    let unpriced: Vec<String> = v["cost"]["unpriced"]
        .as_array()?
        .iter()
        .filter_map(|x| x.as_str()?.strip_suffix("_uncharged").map(String::from))
        .collect();
    let mut phase = BTreeMap::new();
    let mut unpriced_total = 0.0;
    let mut draw_probe = (0.0, 0.0);
    let phases = v["phases"].as_array()?;
    for p in phases {
        let name = p["name"].as_str()?.to_string();
        let gae = p["gae"].as_f64()?;
        let mut ops = 0.0;
        let mut probes = 0.0;
        if let Some(native) = p["native"].as_object() {
            for (k, val) in native {
                if unpriced.contains(k) {
                    let c = val.as_f64().unwrap_or(0.0);
                    ops += c;
                    if k == "lookups" || k == "target_guard_probes" {
                        probes += c;
                    }
                }
            }
        }
        if name == "relations" {
            draw_probe = (gae, probes);
        }
        unpriced_total += ops;
        phase.insert(name, (gae, ops));
    }
    // Unpriced counters that live only in the record's counters (rho).
    let in_phases = unpriced_total;
    if arm.starts_with("rho") && in_phases == 0.0 {
        for k in &unpriced {
            unpriced_total += v["counters"][format!("{k}_uncharged")]
                .as_f64()
                .unwrap_or(0.0);
        }
    }
    let c = &v["counters"];
    Some((
        arm,
        Run {
            workload: v["workload"]["workload_id"].as_str()?.to_string(),
            round: v["round"].as_u64()?,
            r,
            phase,
            draw_probe,
            total_gae: v["cost"]["total_gae"].as_f64()?,
            unpriced_ops: unpriced_total,
            found: c["relations_found"].as_f64().unwrap_or(0.0),
            tried: c["targets_tried"].as_f64().unwrap_or(0.0),
            verified: v["outcome"]["status"] == "verified",
        },
    ))
}

fn s_at(run: &Run, w: f64) -> f64 {
    (run.total_gae + w * run.unpriced_ops) / run.r.sqrt()
}

fn phase_s(run: &Run, name: &str, w: f64) -> f64 {
    run.phase
        .get(name)
        .map_or(0.0, |(g, o)| (g + w * o) / run.r.sqrt())
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len().max(1) as f64
}

/// Percentile interval (default 90%) of a statistic over resampled indices.
fn boot(rng: &mut StdRng, n: usize, stat: impl Fn(&[usize]) -> f64) -> (f64, f64) {
    let mut v: Vec<f64> = (0..RESAMPLES)
        .map(|_| {
            let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
            stat(&idx)
        })
        .collect();
    v.sort_by(|a, b| a.total_cmp(b));
    (v[RESAMPLES * 5 / 100], v[RESAMPLES * 95 / 100])
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = &args[0];
    let mut rng = StdRng::seed_from_u64(20261019);
    let mut curves = Vec::new();
    for dir in &args[1..] {
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
        let r = arms["rho-neg"][0].r;
        let mut arm_json = serde_json::Map::new();
        for (arm, runs) in &arms {
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
                let draw: Vec<f64> = runs.iter().map(|x| x.draw_probe.0 / x.r.sqrt()).collect();
                let probe: Vec<f64> = runs
                    .iter()
                    .map(|x| w * x.draw_probe.1 / x.r.sqrt())
                    .collect();
                by_w.push(json!({
                    "w": w, "s_mean": mean(&total), "s_ci90": [ci.0, ci.1],
                    "phases_s": phases,
                    "relations_target_draw_s": mean(&draw),
                    "relations_probe_s": mean(&probe),
                }));
            }
            let found: f64 = runs.iter().map(|x| x.found).sum();
            let tried: f64 = runs.iter().map(|x| x.tried).sum();
            let yld = |i: &[usize]| {
                i.iter().map(|&k| runs[k].found).sum::<f64>()
                    / i.iter().map(|&k| runs[k].tried).sum::<f64>()
            };
            let yci = boot(&mut rng, n, yld);
            arm_json.insert(
                arm.clone(),
                json!({
                    "runs": n,
                    "verified": runs.iter().filter(|x| x.verified).count(),
                    "relations_found": found, "targets_tried": tried,
                    "yield": found / tried, "yield_ci90": [yci.0, yci.1],
                    "targets_per_run_mean": tried / n as f64,
                    "by_weight": by_w,
                }),
            );
        }
        // Paired ratios, knownlog over abscissa.
        let (a, k) = (&arms["ic-abscissa"], &arms["ic-knownlog"]);
        assert_eq!(a.len(), k.len());
        let mut ratios = Vec::new();
        for &w in &WEIGHTS {
            let ratio = |i: &[usize]| {
                mean(&i.iter().map(|&j| s_at(&k[j], w)).collect::<Vec<_>>())
                    / mean(&i.iter().map(|&j| s_at(&a[j], w)).collect::<Vec<_>>())
            };
            let all: Vec<usize> = (0..a.len()).collect();
            let ci = boot(&mut rng, a.len(), ratio);
            // The same ratio with the base's own cost and the matrix removed
            // from both: what is left is the online interval.
            let online = |i: &[usize], x: &[Run]| {
                mean(
                    &i.iter()
                        .map(|&j| {
                            ["relations", "verify"]
                                .iter()
                                .map(|p| phase_s(&x[j], p, w))
                                .sum::<f64>()
                        })
                        .collect::<Vec<_>>(),
                )
            };
            let ron = |i: &[usize]| online(i, k) / online(i, a);
            let cio = boot(&mut rng, a.len(), ron);
            ratios.push(json!({
                "w": w, "total_s_ratio": ratio(&all), "total_s_ratio_ci90": [ci.0, ci.1],
                "online_s_ratio": ron(&all), "online_s_ratio_ci90": [cio.0, cio.1],
            }));
        }
        let ratio_y = |ia: &[usize], ik: &[usize]| {
            let ya = ia.iter().map(|&j| a[j].found).sum::<f64>()
                / ia.iter().map(|&j| a[j].tried).sum::<f64>();
            let yk = ik.iter().map(|&j| k[j].found).sum::<f64>()
                / ik.iter().map(|&j| k[j].tried).sum::<f64>();
            yk / ya
        };
        let all: Vec<usize> = (0..a.len()).collect();
        let n = a.len();
        let mut v: Vec<f64> = (0..RESAMPLES)
            .map(|_| {
                let ia: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
                let ik: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
                ratio_y(&ia, &ik)
            })
            .collect();
        v.sort_by(|x, y| x.total_cmp(y));
        let s_ref: Vec<f64> = WEIGHTS
            .iter()
            .map(|&w| {
                mean(
                    &arms["rho-neg"]
                        .iter()
                        .map(|x| s_at(x, w))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        curves.push(json!({
            "session_dir": dir, "r": r, "log2_r": r.log2(),
            "arms": arm_json,
            "knownlog_over_abscissa": {
                "yield_ratio": ratio_y(&all, &all),
                "yield_ratio_ci90": [v[RESAMPLES * 5 / 100], v[RESAMPLES * 95 / 100]],
                "s_ratios": ratios,
            },
            "rho_neg_s_mean_by_weight": s_ref,
        }));
    }
    let doc = json!({
        "schema": "generator_effect_dryrun_summary/v1",
        "status": "SCRATCH dry run, not data; PROTOCOL.md not frozen",
        "weights_gae_per_unpriced_op": WEIGHTS,
        "bootstrap": {"resamples": RESAMPLES, "interval": "percentile 90%", "seed": 20261019},
        "curves": curves,
    });
    std::fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
