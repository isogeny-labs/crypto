//! Fit the cost exponent of the linear two-summand algebraic IC ladder
//! (`research/ic_m2_linear_ladder_20261024/PROTOCOL.md`) from its rung files.
//!
//! Usage: `ic_m2_ladder_fit <f4_analysis.json> <out.json> <rung.json>...`
//!
//! Per rung: means of IC trials, IC wall, rho group additions and rho wall,
//! and the IC/rho wall ratio.  Slopes of log2(mean) against n by least
//! squares over the small-cofactor rungs (h ≤ 4), with a seeded bootstrap
//! that resamples targets within each rung.  The F4 arm's per-size solver
//! word XORs are read from its committed analysis for the same fit.

use serde_json::{json, Value};
use std::fs;

fn f(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap(),
        Value::String(s) => s.parse().unwrap(),
        _ => f64::NAN,
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn slope(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let (mx, my) = (mean(xs), mean(ys));
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let b = sxy / sxx;
    (b, my - b * mx)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f4 = serde_json::from_str::<Value>(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let mut rungs = Vec::new();
    for path in &args[3..] {
        let d: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        let rows = d["rows"].as_array().unwrap();
        let col = |k: &str| rows.iter().map(|r| f(&r[k])).collect::<Vec<f64>>();
        let all_ok = rows.iter().all(|r| r["ic_verified"] == true && r["rho_verified"] == true);
        rungs.push(json!({
            "file": path, "a": d["a"], "n": d["n"], "d": d["d"], "cofactor": d["cofactor"],
            "log2_r": d["log2_r"], "targets": rows.len(), "all_verified": all_ok,
            "ic_trials": col("ic_trials"), "ic_s": col("ic_s"), "ic_relations": col("ic_relations"),
            "ic_gauss_xors": col("ic_gauss_xors"), "ic_group_additions": col("ic_group_additions"),
            "rho_group_additions": col("rho_group_additions"), "rho_s": col("rho_s"),
            "mean_ic_trials": mean(&col("ic_trials")), "mean_ic_s": mean(&col("ic_s")),
            "mean_rho_s": mean(&col("rho_s")), "mean_rho_adds": mean(&col("rho_group_additions")),
            "ic_over_rho_wall": mean(&col("ic_s")) / mean(&col("rho_s")),
            "ic_trials_over_rho_adds": mean(&col("ic_trials")) / mean(&col("rho_group_additions")),
        }));
    }
    let small: Vec<&Value> = rungs.iter().filter(|r| f(&r["cofactor"]) <= 4.0).collect();
    let ns: Vec<f64> = small.iter().map(|r| f(&r["n"])).collect();
    let fit_key = |key: &str, rng: &mut Rng| {
        let ys: Vec<f64> = small.iter().map(|r| mean(&r[key].as_array().unwrap().iter().map(f).collect::<Vec<_>>()).log2()).collect();
        let (b, a0) = slope(&ns, &ys);
        let mut boots = Vec::with_capacity(10_000);
        for _ in 0..10_000 {
            let ys_b: Vec<f64> = small.iter().map(|r| {
                let v: Vec<f64> = r[key].as_array().unwrap().iter().map(f).collect();
                let s: Vec<f64> = (0..v.len()).map(|_| v[(rng.next() % v.len() as u64) as usize]).collect();
                mean(&s).log2()
            }).collect();
            boots.push(slope(&ns, &ys_b).0);
        }
        boots.sort_by(|x, y| x.partial_cmp(y).unwrap());
        json!({"slope_log2_per_n": b, "intercept": a0, "boot95": [boots[250], boots[9750]], "rungs": ns.len()})
    };
    let mut rng = Rng(20261024);
    let fits = json!({
        "ic_trials": fit_key("ic_trials", &mut rng),
        "ic_wall": fit_key("ic_s", &mut rng),
        "rho_group_additions": fit_key("rho_group_additions", &mut rng),
        "rho_wall": fit_key("rho_s", &mut rng),
    });
    // F4 arm: per-size median solver word XORs and S, same n range.
    let mut f4_pts = Vec::new();
    for p in f4["per_size"].as_array().unwrap() {
        let arm = &p["arms"]["ic-f4"];
        if arm["median_word_xors"].is_number() {
            f4_pts.push((f(&p["m"]), f(&arm["median_word_xors"]), f(&arm["median_s_lower"]), f(&p["rho_median_s"]), f(&p["r"])));
        }
    }
    let (b4, _) = slope(&f4_pts.iter().map(|p| p.0).collect::<Vec<_>>(), &f4_pts.iter().map(|p| p.1.log2()).collect::<Vec<_>>());
    let out = json!({
        "schema": "ic-m2-linear-ladder-fit/v1",
        "rungs": rungs, "fit_small_cofactor": fits,
        "model_slope_ic_trials": 2.0 / 3.0, "model_slope_rho": 0.5,
        "f4_reference": {"source": args[1], "points": f4_pts.iter().map(|p| json!({"n": p.0, "median_word_xors": p.1, "median_s_lower": p.2, "rho_median_s": p.3, "r": p.4})).collect::<Vec<_>>(),
                          "slope_log2_word_xors_per_n": b4},
    });
    fs::write(&args[2], serde_json::to_string_pretty(&out).unwrap()).unwrap();
    println!("{}", serde_json::to_string_pretty(&json!({"fits": out["fit_small_cofactor"], "f4_slope": b4})).unwrap());
}
