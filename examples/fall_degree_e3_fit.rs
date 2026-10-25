//! E3's registered model comparison (`research/fall_degree_bounds_20261008/
//! PREREGISTRATION.md` §E3 and Amendment 2): fit `LOG: d = a + b·log₂ n` and
//! `LIN: d = a + n/c` (ordinary least squares on the per-`n` median `d_F`)
//! to `n ≤ 22`, then compare mean absolute error on `n ∈ {24…32}`.
//!
//! Inputs: the committed Sage medians at `n ≤ 16` (`RESULTS.md`, E2 table,
//! "Semaev" columns, random base) and the native E3 medians at `n ≥ 18`
//! (lower median of the rand and sat draws pooled), read from `e3/n*.jsonl`.
//!
//! Usage: `fall_degree_e3_fit <study_dir>`
use std::collections::BTreeMap;

fn ols(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let k = xs.len() as f64;
    let (sx, sy) = (xs.iter().sum::<f64>(), ys.iter().sum::<f64>());
    let sxx: f64 = xs.iter().map(|x| x * x).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let b = (k * sxy - sx * sy) / (k * sxx - sx * sx);
    ((sy - b * sx) / k, b)
}

fn main() {
    let dir = std::path::PathBuf::from(std::env::args().nth(1).expect("study dir"));
    let mut med: BTreeMap<u32, f64> = [(8, 3.0), (10, 3.0), (12, 3.0), (14, 3.0), (16, 3.0)].into();
    let mut native: BTreeMap<u32, Vec<u64>> = BTreeMap::new();
    for e in std::fs::read_dir(dir.join("e3")).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if !(name.starts_with('n') && name.ends_with(".jsonl")) {
            continue;
        }
        for l in std::fs::read_to_string(&p)
            .unwrap()
            .lines()
            .filter(|l| l.starts_with('{'))
        {
            let r: serde_json::Value = serde_json::from_str(l).unwrap();
            native
                .entry(r["n"].as_u64().unwrap() as u32)
                .or_default()
                .push(r["d_last"].as_u64().unwrap_or(7));
        }
    }
    for (n, mut v) in native {
        v.sort_unstable();
        med.insert(n, v[(v.len() + 1) / 2 - 1] as f64);
    }
    let fit: Vec<(f64, f64)> = med
        .iter()
        .filter(|(&n, _)| n <= 22)
        .map(|(&n, &d)| (n as f64, d))
        .collect();
    let xs: Vec<f64> = fit.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = fit.iter().map(|p| p.1).collect();
    let (a_log, b_log) = ols(&xs.iter().map(|x| x.log2()).collect::<Vec<_>>(), &ys);
    let (a_lin, b_lin) = ols(&xs, &ys);
    println!("medians: {med:?}");
    println!(
        "LOG: d = {a_log:.3} + {b_log:.3}·log2 n;  LIN: d = {a_lin:.3} + n/{:.2}",
        1.0 / b_lin
    );
    let (mut e_log, mut e_lin, mut k) = (0.0, 0.0, 0.0);
    println!("| n | median d_F | LOG | LIN |");
    println!("|--:|--:|--:|--:|");
    for (&n, &d) in med.iter().filter(|(&n, _)| (24..=32).contains(&n)) {
        let (pl, pn) = (a_log + b_log * (n as f64).log2(), a_lin + b_lin * n as f64);
        e_log += (pl - d).abs();
        e_lin += (pn - d).abs();
        k += 1.0;
        println!("| {n} | {d} | {pl:.2} | {pn:.2} |");
    }
    println!(
        "mean |error| on n in 24..32: LOG {:.3}, LIN {:.3} -> preferred: {}",
        e_log / k,
        e_lin / k,
        if (e_log - e_lin).abs() < 1e-9 {
            "tie"
        } else if e_log < e_lin {
            "LOG"
        } else {
            "LIN"
        }
    );
}
