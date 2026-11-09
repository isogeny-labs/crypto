//! Summary of the amendment-6 screening scan: per prime, the spread of
//! `columns / p` over the curve chain, the top-5% over median factor, the
//! trials-per-relation saving that factor implies at the measured slope
//! (model), the checks S1–S3, and the stage-2 selection.
//!
//! ```text
//! cargo run --release --example gaudry_quartic_d4_columns_summary -- scan/columns.jsonl curves.tsv scan/summary.json
//! ```

use std::collections::{BTreeMap, HashSet};
use std::{env, fs};

const SLOPE: f64 = 4.28; // measured: ln(trials/relation) on ln(columns/p), amendment 5 data

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn sd(v: &[f64]) -> f64 {
    let m = mean(v);
    (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}
fn quantile(sorted: &[f64], q: f64) -> f64 {
    let i = q * (sorted.len() as f64 - 1.0);
    let (lo, hi) = (i.floor() as usize, i.ceil() as usize);
    sorted[lo] + (sorted[hi] - sorted[lo]) * (i - lo as f64)
}

fn main() {
    let a: Vec<String> = env::args().skip(1).collect();
    let rows: Vec<serde_json::Value> = fs::read_to_string(&a[0])
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .filter(|v: &serde_json::Value| v.get("columns").is_some())
        .collect();
    let used: HashSet<(u64, u64)> = fs::read_to_string(&a[1])
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            Some((f.first()?.parse().ok()?, f.get(3)?.parse().ok()?))
        })
        .collect();
    let mut by_p: BTreeMap<u64, Vec<&serde_json::Value>> = BTreeMap::new();
    for r in &rows {
        by_p.entry(r["p"].as_u64().unwrap()).or_default().push(r);
    }
    let mut out = Vec::new();
    let mut pooled_top = Vec::new();
    println!("p  n  mean(c/p)  sd  median  max/median  top5%/median  trials-saving(model)");
    for (p, v) in &by_p {
        let ratios: Vec<f64> = v.iter().map(|r| r["ratio"].as_f64().unwrap()).collect();
        let mut s = ratios.clone();
        s.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let med = quantile(&s, 0.5);
        let t95 = quantile(&s, 0.95);
        let top: Vec<f64> = s.iter().copied().filter(|&x| x >= t95).collect();
        let top_mean = mean(&top);
        let saving = (top_mean / med).powf(SLOPE);
        pooled_top.push(top_mean / med);
        println!(
            "{p} {} {:.3} {:.3} {:.3} {:.2} {:.2} {:.2}",
            v.len(),
            mean(&ratios),
            sd(&ratios),
            med,
            s[s.len() - 1] / med,
            top_mean / med,
            saving
        );
        out.push(serde_json::json!({"p": p, "n": v.len(), "mean_ratio": mean(&ratios),
            "sd_ratio": sd(&ratios), "median_ratio": med, "max_over_median": s[s.len()-1]/med,
            "top5_mean_over_median": top_mean/med, "trials_saving_model": saving}));
    }
    // Stage 2 selection.
    let mut sel = serde_json::Map::new();
    for p in [101u64, 149] {
        let mut v: Vec<&serde_json::Value> = by_p[&p]
            .iter()
            .copied()
            .filter(|r| !used.contains(&(p, r["instance_seed"].as_u64().unwrap())))
            .collect();
        v.sort_by_key(|r| (r["columns"].as_u64().unwrap(), r["instance_seed"].as_u64().unwrap()));
        let pick = |s: &[&serde_json::Value]| -> Vec<serde_json::Value> {
            s.iter().map(|r| serde_json::json!({"instance_seed": r["instance_seed"], "start": r["start"], "columns": r["columns"], "cofactor": r["cofactor"], "r": r["r"]})).collect()
        };
        let n = v.len();
        sel.insert(
            p.to_string(),
            serde_json::json!({"fewest": pick(&v[..3]), "most": pick(&v[n - 3..])}),
        );
    }
    println!("stage-2 selection: {}", serde_json::to_string(&sel).unwrap());
    fs::write(&a[2], serde_json::to_string_pretty(&serde_json::json!({"per_p": out, "stage2": sel, "slope_used": SLOPE})).unwrap()).unwrap();
}
