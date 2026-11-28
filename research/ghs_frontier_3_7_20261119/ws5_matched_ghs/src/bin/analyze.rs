use ghs_matched_frontier_gate::{classify, selftest, Components, GRID, WEIGHTS};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::Path;

fn finite(value: &Value, label: &str) -> Result<f64, String> {
    let value = value
        .as_f64()
        .ok_or_else(|| format!("{label} is not numeric"))?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!("{label} is negative or non-finite"));
    }
    Ok(value)
}

fn run_analysis(path: &Path) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    let input: Value = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    if input["schema"] != "ghs.matched-frontier.analysis-input/v1" {
        return Err("wrong analysis-input schema".into());
    }
    let sizes = input["sizes"]
        .as_array()
        .ok_or_else(|| "sizes is not an array".to_owned())?;
    if sizes.len() != GRID.len() {
        return Err("analysis size census incomplete".into());
    }
    let mut verdict_rows = Vec::new();
    let mut output = Vec::new();
    for (size, (_, expected_q, _, _, _)) in sizes.iter().zip(GRID) {
        if size["q"].as_u64() != Some(expected_q) {
            return Err("analysis q order changed".into());
        }
        let steps = finite(&size["steps_over_sqrt_ell"], "steps_over_sqrt_ell")?;
        let admitted = (0.75..=1.15).contains(&steps);
        let calibration = size["calibration"]
            .as_object()
            .ok_or_else(|| "calibration is not an object".to_owned())?;
        let r_inv = finite(&calibration["r_inv"], "r_inv")?;
        let r_lmul = finite(&calibration["r_lmul"], "r_lmul")?;
        if calibration["triples_per_seed"].as_u64() != Some(600)
            || calibration["seed_count"].as_u64() != Some(3)
            || r_inv <= 0.0
            || r_lmul <= 0.0
        {
            return Err("calibration contract incomplete".into());
        }
        let candidates = size["candidates"]
            .as_array()
            .ok_or_else(|| "candidates is not an array".to_owned())?;
        if candidates.is_empty() {
            return Err("no complete candidates".into());
        }
        let rho = Components::from_value(&size["rho_components"], "rho_components")?;
        let mut intervals = Vec::new();
        let mut weight_rows = Vec::new();
        for weight in WEIGHTS {
            let rho_cost = rho.price(r_inv, r_lmul, weight);
            if rho_cost <= 0.0 {
                return Err("zero rho cost".into());
            }
            let mut priced = candidates
                .iter()
                .map(|candidate| {
                    let arm = candidate["arm"]
                        .as_str()
                        .ok_or_else(|| "candidate arm missing".to_owned())?;
                    if candidate["complete"] != true || candidate["scalar_verified"] != true {
                        return Err(format!("candidate incomplete: {arm}"));
                    }
                    let cost =
                        Components::from_value(&candidate["components"], "candidate components")?
                            .price(r_inv, r_lmul, weight);
                    Ok((arm, cost))
                })
                .collect::<Result<Vec<_>, String>>()?;
            priced.sort_by(|a, b| {
                a.1.partial_cmp(&b.1)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| a.0.cmp(b.0))
            });
            let selected = priced[0];
            let point = selected.1 / rho_cost;
            let ci = size["ci95_by_weight"]
                .as_object()
                .and_then(|rows| rows.get(&format!("{weight}")))
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    format!("missing independently recomputed CI at q={expected_q}, w={weight}")
                })?;
            if ci.len() != 2 {
                return Err("CI must have two endpoints".into());
            }
            let lower = finite(&ci[0], "CI lower")?;
            let upper = finite(&ci[1], "CI upper")?;
            if lower > upper || point < lower || point > upper {
                return Err("point estimate is outside its supplied interval".into());
            }
            intervals.push((lower, upper));
            weight_rows.push(serde_json::json!({
                "weight": weight,
                "selected_ic_arm": selected.0,
                "ic_over_rho": point,
                "ci95": [lower, upper]
            }));
        }
        verdict_rows.push((admitted, intervals));
        output.push(serde_json::json!({
            "q": expected_q,
            "admitted": admitted,
            "steps_over_sqrt_ell": steps,
            "weights": weight_rows
        }));
    }
    Ok(serde_json::json!({
        "schema": "ghs.matched-frontier.analysis/v1",
        "input_sha256": format!("{:x}", Sha256::digest(bytes)),
        "scope": "restricted-ghs-weak-plane-quartic",
        "scope_warning": "This applies only to E_0/F_(q^3) with order 4*ell and is not a generic or standardized-curve result.",
        "verdict": classify(&verdict_rows),
        "sizes": output
    }))
}

fn run() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    match args.as_slice() {
        [_, command] if command == "selftest" => {
            selftest()?;
            println!("GHS M3 native analysis self-test PASS");
        }
        [_, command, input, output] if command == "run" => {
            let output_path = Path::new(output);
            if output_path.exists() {
                return Err("refusing to overwrite analysis output".into());
            }
            let value = run_analysis(Path::new(input))?;
            let bytes = serde_json::to_vec_pretty(&value).map_err(|error| error.to_string())?;
            fs::write(output_path, [bytes, b"\n".to_vec()].concat())
                .map_err(|error| error.to_string())?;
        }
        _ => return Err("usage: ghs-m3-analyze selftest | run INPUT OUTPUT".into()),
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ghs-m3-analyze: refused: {error}");
        std::process::exit(2);
    }
}
