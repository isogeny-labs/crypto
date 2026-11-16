//! Arm (c)'s frozen selection rule (PROTOCOL.md §4.4): for each crater
//! curve, the isomorphic model with the smallest mean `S` (weight 0) over its
//! calibration runs, ties to the lower model index; a model with any missing
//! or unverified calibration run is not eligible. The same rule applied to the
//! rho reference picks the rho-selected model of the negative control.
//! Only `cal`-stage runs are read; held-out data does not exist yet.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::analyse::{parse_run, workload_key};

pub fn select(
    records: &[String],
    labels: &[String],
    expected_runs: usize,
) -> Result<Value, String> {
    // workload key -> (class, unit, model)
    let mut lab: BTreeMap<String, (String, String, u64)> = BTreeMap::new();
    let mut tier = String::new();
    for t in labels {
        for line in t.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            if v["stage"] != "cal" {
                return Err(format!(
                    "select reads cal-stage labels only, got {}",
                    v["stage"]
                ));
            }
            tier = v["tier"].as_str().unwrap_or("").into();
            let w = &v["workload"];
            let l = &v["labels"][0];
            lab.insert(
                workload_key(&w["p"], &w["a"], &w["b"], &w["gx"], &w["gy"]),
                (
                    v["class"].as_str().unwrap_or("").into(),
                    l["unit"].as_str().unwrap_or("").into(),
                    l["model"].as_u64().ok_or("model")?,
                ),
            );
        }
    }
    // (class, unit) -> method -> model -> (sum S, runs, all verified)
    type Acc = BTreeMap<(String, String), BTreeMap<String, BTreeMap<u64, (f64, usize, bool)>>>;
    let mut acc: Acc = BTreeMap::new();
    for text in records {
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let r = parse_run(line)?;
            if r.warmup || !(r.arm == "ic-mitm" || r.arm == "rho-neg") {
                continue;
            }
            let Some((class, unit, model)) = lab.get(&r.key) else {
                return Err(format!("a calibration record has no cal label: {}", r.key));
            };
            let e = acc
                .entry((class.clone(), unit.clone()))
                .or_default()
                .entry(r.arm.clone())
                .or_default()
                .entry(*model)
                .or_insert((0.0, 0, true));
            e.0 += r.total_gae / r.sqrt_r;
            e.1 += 1;
            e.2 &= r.ok;
        }
    }
    let mut out = Vec::new();
    for ((class, unit), methods) in &acc {
        let pick = |m: &str| -> Result<(u64, Value), String> {
            let models = methods
                .get(m)
                .ok_or(format!("{unit}: no {m} calibration runs"))?;
            let mut best: Option<(f64, u64)> = None;
            let mut means = BTreeMap::new();
            for (&model, &(sum, n, ok)) in models {
                let eligible = ok && n == expected_runs;
                let mean = sum / n as f64;
                means.insert(
                    model.to_string(),
                    json!({"mean_s": mean, "runs": n, "eligible": eligible}),
                );
                if eligible && best.is_none_or(|(b, _)| mean < b) {
                    best = Some((mean, model));
                }
            }
            let (_, m) = best.ok_or(format!("{unit}: no eligible {m} model"))?;
            Ok((m, json!(means)))
        };
        let (ic_model, ic_means) = pick("ic-mitm")?;
        let (rho_model, rho_means) = pick("rho-neg")?;
        let in_sample = |means: &Value, m: u64| {
            means[m.to_string()]["mean_s"].as_f64().unwrap_or(f64::NAN)
                / means["0"]["mean_s"].as_f64().unwrap_or(f64::NAN)
        };
        let (ic_in, rho_in) = (
            in_sample(&ic_means, ic_model),
            in_sample(&rho_means, rho_model),
        );
        let vertex: u64 = unit
            .trim_start_matches('c')
            .parse()
            .map_err(|_| format!("bad unit {unit}"))?;
        out.push(json!({"class": class, "unit": unit, "vertex": vertex, "ic_model": ic_model, "rho_model": rho_model,
                        "in_sample_ratio": {"ic-mitm": ic_in, "rho-neg": rho_in},
                        "calibration": {"ic-mitm": ic_means, "rho-neg": rho_means}}));
    }
    Ok(
        json!({"schema": "repsel.selection/v1", "tier": tier, "rule": "argmin mean S (w = 0) over eligible models; ties to the lower model index",
              "expected_runs_per_model": expected_runs, "selection": out}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(a: u64, arm: &str, round: u64, total: f64, ok: bool) -> String {
        json!({"arm": arm, "round": round, "warmup": round == 0,
               "workload": {"target_index": 0, "planted": 5, "curve_spec": {"p": 101, "a": a, "b": 1, "gx": 2, "gy": 3, "r": 100}},
               "outcome": {"status": if ok {"verified"} else {"exhausted"}, "matches_target": ok, "matches_planted": ok, "recovered": "5"},
               "cost": {"total_gae": total, "unpriced": []}, "counters": {}, "phases": [], "detail": {}}).to_string()
    }

    fn lab(a: u64, model: u64) -> String {
        json!({"tier": "24", "class": "t24-s1", "stage": "cal", "workload": {"p": 101, "a": a, "b": 1, "gx": 2, "gy": 3},
               "labels": [{"arm": "cal", "unit": "c7", "model": model, "extra_ops": 0}]}).to_string()
    }

    #[test]
    fn argmin_with_ties_to_the_lower_model_and_ineligible_models_skipped() {
        let labels = vec![[lab(10, 0), lab(11, 1), lab(12, 2), lab(13, 3)].join("\n")];
        let mut rs = Vec::new();
        for (a, ic, rho, ok) in [
            (10, 50.0, 9.0, true),
            (11, 40.0, 9.0, true),
            (12, 40.0, 8.0, true),
            (13, 10.0, 7.0, false),
        ] {
            for round in 0..3 {
                rs.push(rec(a, "ic-mitm", round, ic, ok || round == 0));
                rs.push(rec(a, "rho-neg", round, rho, ok || round == 0));
            }
        }
        let s = select(&[rs.join("\n")], &labels, 2).unwrap();
        let pick = &s["selection"][0];
        assert_eq!(pick["ic_model"], 1); // 1 and 2 tie at 40; 3 is cheaper but has an unverified run
        assert_eq!(pick["rho_model"], 2);
        assert_eq!(pick["vertex"], 7);
    }
}
