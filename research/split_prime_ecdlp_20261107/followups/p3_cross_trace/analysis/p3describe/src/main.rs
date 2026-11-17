//! p3describe: descriptive statistics for P3 that decide nothing (PROTOCOL.md "Descriptive").
//! (1) percentile of each published curve's z_691 (../../../data/recount.json) within the census
//!     z_691 of each prime; (2) the distribution of per-curve rho cost (mean ops / sqrt n), runs A and B,
//!     as deciles and the share above 2.0, to document the instrument finding raised by the A/A control.
use serde_json::Value;
use std::fs;
fn lines(p: &str) -> Vec<Value> { fs::read_to_string(p).unwrap().lines().filter(|l| !l.is_empty()).map(|l| serde_json::from_str(l).unwrap()).collect() }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (run, recount, out) = (&a[1], &a[2], &a[3]);
    let rec: Value = serde_json::from_str(&fs::read_to_string(recount).unwrap()).unwrap();
    let published: Vec<(String, f64)> = rec["curves"].as_array().unwrap().iter().map(|c| {
        let z = c["cutoffs"].as_array().unwrap().iter().find(|r| r["cutoff"].as_i64() == Some(691)).unwrap()["z"].as_f64().unwrap();
        (c["name"].as_str().unwrap().to_string(), z) }).collect();
    let mut o = String::new();
    for pr in ["p24", "p28"] {
        let cov = lines(&format!("{run}/{pr}/cov.jsonl"));
        let mut zs: Vec<f64> = cov.iter().filter_map(|r| r.get("z_691").and_then(|x| x.as_f64())).collect();
        zs.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let n = zs.len() as f64;
        o += &format!("== {pr}: census z_691, {} curves; min {:.3}, median {:.3}, max {:.3}\n", zs.len(), zs[0], zs[zs.len() / 2], zs[zs.len() - 1]);
        o += "published curve\tz_691\tpercentile (share of census curves with z <= it)\n";
        for (name, z) in &published { let k = zs.iter().filter(|&&v| v <= *z).count() as f64; o += &format!("{name}\t{z:+.3}\t{:.2} %\n", 100.0 * k / n); }
        for run_ab in ["rhoA", "rhoB"] {
            let mut v: Vec<f64> = lines(&format!("{run}/{pr}/{run_ab}.jsonl")).iter().filter(|r| r["kind"] == "rho").map(|r| r["mean_over_sqrt_n"].as_f64().unwrap()).collect();
            v.sort_by(|x, y| x.partial_cmp(y).unwrap());
            let dec: Vec<String> = (0..=10).map(|i| format!("{:.3}", v[((v.len() - 1) * i) / 10])).collect();
            let hi = v.iter().filter(|&&x| x > 2.0).count();
            o += &format!("{run_ab} mean_ops/sqrt(n) deciles: {}; share > 2.0: {}/{}\n", dec.join(" "), hi, v.len());
        }
    }
    fs::write(format!("{out}/p3_descriptive.txt"), &o).unwrap();
    print!("{o}");
}
