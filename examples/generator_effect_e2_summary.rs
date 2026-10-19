//! Aggregate experiment E2 of the generator-effect study
//! (research/generator_effect_ic_20261019/E2.md): the cofactor ladder.
//!
//! `cargo run --release --example generator_effect_e2_summary -- OUT.json PREREG.jsonl SESSION_DIR...`
//!
//! One `ecbench` session per curve (the same targets for every arm);
//! sessions are grouped by the cofactor `h` their curve was registered with.  For each `h` it reports, against the pre-registered
//! predictions of `generator_effect_e2_n0`:
//!
//! - the online yield ratio known-log / abscissa, measured over predicted,
//!   with the prediction from `N₀` (primary) and from the subgroup sumset;
//!   pooled over curves as `O/E` per arm (relations found over the sum of
//!   targets tried times the curve's predicted rate), so one curve reduces
//!   to `(y_kl / y_abs) / (N₀_kl / N₀_abs)`;
//! - the same for the A/A pair (two random-scalar bases, different seeds);
//! - total S at `w` ∈ {0, 0.1, 1} GAE per unpriced operation (AGENTS.md
//!   §5a), known-log over abscissa paired by target, and both over rho.
//!
//! Intervals: 10 000 bootstrap resamples, percentile 90%, seed 20261019,
//! resampling targets within each curve (stratified); the yield ratios also
//! carry a two-level interval (curves, then targets within the drawn curve).
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
const KL: &str = "ic-knownlog-random";
const AA: &str = "ic-knownlog-random-aa";
const ABS: &str = "ic-abscissa";
const RHO: &str = "rho-neg";

#[derive(Clone)]
struct Run {
    target: u64,
    phase: BTreeMap<String, (f64, f64)>,
    total_gae: f64,
    unpriced_ops: f64,
    found: f64,
    tried: f64,
    verified: bool,
    solve_wall_ns: f64,
}

/// Runs of one curve, by arm, sorted by target index.
struct Curve {
    p: u64,
    r: f64,
    arms: BTreeMap<String, Vec<Run>>,
}

fn parse(v: &Value) -> Option<(String, (u64, u64), Run)> {
    if v["warmup"].as_bool()? {
        return None;
    }
    let unpriced: Vec<String> = v["cost"]["unpriced"]
        .as_array()?
        .iter()
        .filter_map(|x| x.as_str()?.strip_suffix("_uncharged").map(String::from))
        .collect();
    let mut phase = BTreeMap::new();
    let mut total_ops = 0.0;
    for p in v["phases"].as_array()? {
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
        // Counters kept at the record level (rho).
        for k in &unpriced {
            total_ops += v["counters"][format!("{k}_uncharged")]
                .as_f64()
                .or_else(|| v["counters"][k.as_str()].as_f64())
                .unwrap_or(0.0);
        }
    }
    let c = &v["counters"];
    let w = &v["workload"];
    Some((
        v["arm"].as_str()?.to_string(),
        (
            w["curve_spec"]["p"].as_u64()?,
            w["curve_spec"]["r"].as_u64()?,
        ),
        Run {
            target: w["target_index"].as_u64()?,
            phase,
            total_gae: v["cost"]["total_gae"].as_f64()?,
            unpriced_ops: total_ops,
            found: c["relations_found"].as_f64().unwrap_or(0.0),
            tried: c["targets_tried"].as_f64().unwrap_or(0.0),
            verified: v["outcome"]["status"] == "verified",
            solve_wall_ns: v["time"]["solve_wall_ns"].as_f64().unwrap_or(0.0),
        },
    ))
}

fn s_at(x: &Run, r: f64, w: f64) -> f64 {
    (x.total_gae + w * x.unpriced_ops) / r.sqrt()
}
fn phase_s(x: &Run, r: f64, name: &str, w: f64) -> f64 {
    x.phase
        .get(name)
        .map_or(0.0, |(g, o)| (g + w * o) / r.sqrt())
}
fn mean(x: impl Iterator<Item = f64>) -> f64 {
    let (s, n) = x.fold((0.0, 0usize), |(s, n), v| (s + v, n + 1));
    s / n.max(1) as f64
}
fn ci(mut v: Vec<f64>) -> [f64; 2] {
    v.sort_by(|a, b| a.total_cmp(b));
    [v[v.len() * 5 / 100], v[v.len() * 95 / 100]]
}

fn load(dir: &str) -> Vec<Curve> {
    let text = std::fs::read_to_string(format!("{dir}/records.jsonl")).unwrap();
    let mut by: BTreeMap<(u64, u64), BTreeMap<String, Vec<Run>>> = BTreeMap::new();
    for line in text.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        if let Some((arm, key, run)) = parse(&v) {
            by.entry(key).or_default().entry(arm).or_default().push(run);
        }
    }
    by.into_iter()
        .map(|((p, r), mut arms)| {
            for runs in arms.values_mut() {
                runs.sort_by_key(|x| x.target);
            }
            Curve {
                p,
                r: r as f64,
                arms,
            }
        })
        .collect()
}

/// A resample: for each curve slot, the curve drawn and its target indices.
type Draw = Vec<(usize, Vec<usize>)>;

fn identity(curves: &[Curve]) -> Draw {
    curves
        .iter()
        .enumerate()
        .map(|(c, x)| (c, (0..x.arms[KL].len()).collect()))
        .collect()
}
fn stratified(rng: &mut StdRng, curves: &[Curve]) -> Draw {
    curves
        .iter()
        .enumerate()
        .map(|(c, x)| {
            let n = x.arms[KL].len();
            (c, (0..n).map(|_| rng.gen_range(0..n)).collect())
        })
        .collect()
}
fn two_level(rng: &mut StdRng, curves: &[Curve]) -> Draw {
    (0..curves.len())
        .map(|_| {
            let c = rng.gen_range(0..curves.len());
            let n = curves[c].arms[KL].len();
            (c, (0..n).map(|_| rng.gen_range(0..n)).collect())
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let prereg: Vec<Value> = std::fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let pre = |p: u64, r: f64| {
        prereg
            .iter()
            .find(|x| x["p"].as_u64() == Some(p) && x["r"].as_f64() == Some(r))
            .unwrap_or_else(|| panic!("no pre-registration for p = {p}"))
    };
    // Predicted relations per target tried, by arm: from the sumset, or the
    // class-cancelling count N0 as a rate (N0 / r).
    let rate = |p: &Value, arm: &str, key: &str| {
        let b = p["bases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|b| b["arm"] == arm)
            .unwrap();
        match key {
            "n0" => b["n0_over_r"].as_f64().unwrap(),
            _ => b["predicted_hit_rate"].as_f64().unwrap(),
        }
    };
    let mut rng = StdRng::seed_from_u64(20261019);
    let mut by_h: BTreeMap<u64, (Vec<Curve>, Vec<String>)> = BTreeMap::new();
    for dir in &args[2..] {
        for c in load(dir) {
            let h = pre(c.p, c.r)["h"].as_u64().unwrap();
            let e = by_h.entry(h).or_default();
            e.0.push(c);
            e.1.push(dir.clone());
        }
    }
    let mut ladder = Vec::new();
    for (h, (curves, dirs)) in &by_h {
        let pres: Vec<&Value> = curves.iter().map(|c| pre(c.p, c.r)).collect();
        // O/E of one arm over a draw, against prediction `key`.
        let oe = |d: &Draw, arm: &str, key: &str| {
            let (mut o, mut e) = (0.0, 0.0);
            for (c, idx) in d {
                let runs = &curves[*c].arms[arm];
                let pr = rate(pres[*c], arm, key);
                for &i in idx {
                    o += runs[i].found;
                    e += runs[i].tried * pr;
                }
            }
            o / e
        };
        let yld = |d: &Draw, arm: &str| {
            let (mut o, mut t) = (0.0, 0.0);
            for (c, idx) in d {
                for &i in idx {
                    o += curves[*c].arms[arm][i].found;
                    t += curves[*c].arms[arm][i].tried;
                }
            }
            o / t
        };
        let all = identity(&curves);
        let strat: Vec<Draw> = (0..RESAMPLES)
            .map(|_| stratified(&mut rng, &curves))
            .collect();
        let two: Vec<Draw> = (0..RESAMPLES)
            .map(|_| two_level(&mut rng, &curves))
            .collect();
        let mut yield_ratios = serde_json::Map::new();
        for (name, num, den, pred_key) in [
            ("knownlog_over_abscissa_n0", KL, ABS, "n0"),
            ("knownlog_over_abscissa_sumset", KL, ABS, "sumset"),
            ("aa_over_knownlog_n0", AA, KL, "n0"),
            ("aa_over_knownlog_sumset", AA, KL, "sumset"),
        ] {
            let mp = |d: &Draw| oe(d, num, pred_key) / oe(d, den, pred_key);
            let pred: Vec<f64> = pres
                .iter()
                .map(|p| rate(p, num, pred_key) / rate(p, den, pred_key))
                .collect();
            let pred_mean = mean(pred.iter().copied());
            yield_ratios.insert(
                name.into(),
                json!({
                    "predicted_per_curve": pred,
                    "predicted_mean": pred_mean,
                    "measured_over_predicted": mp(&all),
                    "measured_over_predicted_ci90": ci(strat.iter().map(mp).collect()),
                    "measured_over_predicted_ci90_two_level": ci(two.iter().map(mp).collect()),
                    "measured_ratio_pooled": yld(&all, num) / yld(&all, den),
                    "measured_ratio_pooled_ci90": ci(strat.iter().map(|d| yld(d, num) / yld(d, den)).collect()),
                    "measured_ratio_per_curve": curves.iter().enumerate().map(|(c, x)| {
                        let d: Draw = vec![(c, (0..x.arms[KL].len()).collect())];
                        yld(&d, num) / yld(&d, den)
                    }).collect::<Vec<_>>(),
                }),
            );
        }
        let mut arms = serde_json::Map::new();
        for arm in [RHO, ABS, KL, AA] {
            let mut by_w = Vec::new();
            for &w in &WEIGHTS {
                let ms = |d: &Draw| {
                    mean(d.iter().flat_map(|(c, idx)| {
                        let x = &curves[*c];
                        idx.iter().map(move |&i| s_at(&x.arms[arm][i], x.r, w))
                    }))
                };
                let mut phases = serde_json::Map::new();
                for ph in PHASES {
                    phases.insert(
                        ph.into(),
                        json!(mean(curves.iter().flat_map(|x| {
                            x.arms[arm].iter().map(move |run| phase_s(run, x.r, ph, w))
                        }))),
                    );
                }
                by_w.push(json!({
                    "w": w, "s_mean": ms(&all),
                    "s_ci90": ci(strat.iter().map(ms).collect()),
                    "phases_s": phases,
                }));
            }
            let runs = curves.iter().flat_map(|x| x.arms[arm].iter());
            let n = curves.iter().map(|x| x.arms[arm].len()).sum::<usize>();
            let mut entry = json!({
                "runs": n,
                "verified": curves.iter().flat_map(|x| x.arms[arm].iter()).filter(|x| x.verified).count(),
                "solve_wall_s_total": runs.map(|x| x.solve_wall_ns).sum::<f64>() / 1e9,
                "by_weight": by_w,
            });
            if arm != RHO {
                entry["relations_found"] = json!(curves
                    .iter()
                    .flat_map(|x| x.arms[arm].iter())
                    .map(|x| x.found)
                    .sum::<f64>());
                entry["targets_tried"] = json!(curves
                    .iter()
                    .flat_map(|x| x.arms[arm].iter())
                    .map(|x| x.tried)
                    .sum::<f64>());
                entry["yield"] = json!(yld(&all, arm));
                entry["yield_ci90"] = json!(ci(strat.iter().map(|d| yld(d, arm)).collect()));
                for key in ["sumset", "n0"] {
                    entry[format!("measured_over_predicted_{key}")] = json!(oe(&all, arm, key));
                    entry[format!("measured_over_predicted_{key}_ci90")] =
                        json!(ci(strat.iter().map(|d| oe(d, arm, key)).collect()));
                }
            }
            arms.insert(arm.into(), entry);
        }
        // Paired total-S ratios through the band (ratio of means over the
        // same targets).
        let mut s_ratios = Vec::new();
        for (a, b) in [(KL, ABS), (KL, RHO), (ABS, RHO), (AA, KL)] {
            let mut rows = Vec::new();
            let (mut below, mut above) = (true, true);
            for &w in &WEIGHTS {
                let ratio = |d: &Draw| {
                    let m = |arm: &str| {
                        mean(d.iter().flat_map(|(c, idx)| {
                            let x = &curves[*c];
                            idx.iter().map(move |&i| s_at(&x.arms[arm][i], x.r, w))
                        }))
                    };
                    m(a) / m(b)
                };
                let iv = ci(strat.iter().map(ratio).collect());
                below &= iv[1] < 1.0;
                above &= iv[0] > 1.0;
                rows.push(json!({"w": w, "ratio": ratio(&all), "ci90": iv}));
            }
            s_ratios.push(json!({
                "a": a, "b": b, "a_over_b": rows,
                "a_below_b_at_every_weight": below,
                "a_above_b_at_every_weight": above,
            }));
        }
        ladder.push(json!({
            "h": h, "session_dirs": dirs, "curves": curves.len(),
            "curve_list": pres.iter().map(|p| json!({"p": p["p"], "r": p["r"], "group_order": p["group_order"], "size": p["size"]})).collect::<Vec<_>>(),
            "yield_ratios": yield_ratios, "arms": arms, "total_s_ratios": s_ratios,
        }));
    }
    let doc = json!({
        "schema": "generator_effect_e2_summary/v1",
        "weights_gae_per_unpriced_op": WEIGHTS,
        "bootstrap": {"resamples": RESAMPLES, "interval": "percentile 90%", "seed": 20261019,
                      "strata": "targets within curve; two_level: curves, then targets"},
        "ladder": ladder,
    });
    std::fs::write(&args[0], serde_json::to_string_pretty(&doc).unwrap()).unwrap();
}
