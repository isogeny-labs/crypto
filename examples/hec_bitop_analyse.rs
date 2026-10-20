//! Analysis of `research/hec_bitop_20261020` (PROTOCOL.md "Measures"): per row
//! the bitop/v1 ratio IC/rho with a bootstrap interval, and per genus the slope
//! of `log₂(IC/rho)` against `log₂ N` with a two-stage bootstrap interval, from
//! the JSON lines `hyperelliptic_ic_vs_rho_bitops --json` writes.
//!
//! ```sh
//! cargo run --release --example hec_bitop_analyse -- --out analysis.json raw/g2.jsonl raw/g3.jsonl raw/g4.jsonl
//! ```
//!
//! Everything printed is computed here from the recorded per-run counts; the
//! in-situ ratio is copied as recorded, for reference only.

use std::collections::BTreeMap;
use std::env;
use std::fs;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const RESAMPLES: usize = 2000;
const SEED: u64 = 20261020;

struct Row {
    genus: u64,
    p: u64,
    n: f64,
    ic: Vec<f64>,
    rho: Vec<f64>,
    ic_verified: bool,
    rho_verified: bool,
    ic_validation: f64,
    rho_validation: f64,
    in_situ: Option<f64>,
}

fn num(v: &Value) -> f64 {
    match v {
        Value::String(s) => s.parse().expect("numeric string"),
        Value::Number(n) => n.to_string().parse().expect("number"),
        _ => panic!("not a number: {v}"),
    }
}

fn validation_total(arm: &Value) -> f64 {
    arm["validation_subset_summed_over_runs"]
        .as_object()
        .map(|phases| phases.values().map(|ph| num(&ph["bitops"])).sum())
        .unwrap_or(0.0)
}

fn read_row(v: &Value) -> Row {
    let runs = |arm: &Value| -> Vec<f64> {
        arm["bitops_per_run"]
            .as_array()
            .expect("bitops_per_run")
            .iter()
            .map(num)
            .collect()
    };
    Row {
        genus: num(&v["genus"]) as u64,
        p: num(&v["p"]) as u64,
        n: num(&v["n"]),
        ic: runs(&v["ic"]),
        rho: runs(&v["rho"]),
        ic_verified: v["ic"]["verified"].as_bool().unwrap_or(false),
        rho_verified: v["rho"]["verified"].as_bool().unwrap_or(false),
        ic_validation: validation_total(&v["ic"]),
        rho_validation: validation_total(&v["rho"]),
        in_situ: v["in_situ_reference"].get("s_ic_over_s_rho").map(num),
    }
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn resample(rng: &mut StdRng, x: &[f64]) -> f64 {
    (0..x.len())
        .map(|_| x[rng.gen_range(0..x.len())])
        .sum::<f64>()
        / x.len() as f64
}

/// Percentile interval of `xs` (sorted in place).
fn interval(xs: &mut [f64]) -> [f64; 2] {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let at = |q: f64| xs[((q * (xs.len() - 1) as f64).round()) as usize];
    [at(0.025), at(0.975)]
}

/// Least-squares slope and intercept of `ys` on `xs`; `None` when the `xs`
/// do not vary.
fn fit(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let (mx, my) = (mean(xs), mean(ys));
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    if sxx <= 0.0 {
        return None;
    }
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let slope = sxy / sxx;
    Some((slope, my - slope * mx))
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let out = match args.iter().position(|a| a == "--out") {
        Some(i) => {
            let path = args.remove(i + 1);
            args.remove(i);
            Some(path)
        }
        None => None,
    };
    let mut rows = Vec::new();
    for path in &args {
        for line in fs::read_to_string(path).expect("read input").lines() {
            if !line.trim().is_empty() {
                rows.push(read_row(&serde_json::from_str(line).expect("json line")));
            }
        }
    }
    rows.sort_by_key(|r| (r.genus, r.p));
    let mut rng = StdRng::seed_from_u64(SEED);

    println!("| g | p | N | log2 N | IC runs | rho runs | IC/rho bitop/v1 [95%] | excl. validation | in-situ (full-width rho) | verified |");
    println!("|--:|--:|--:|--:|--:|--:|:--|--:|--:|:--|");
    let mut row_json = Vec::new();
    for r in &rows {
        let ratio = mean(&r.ic) / mean(&r.rho);
        let mut boots: Vec<f64> = (0..RESAMPLES)
            .map(|_| resample(&mut rng, &r.ic) / resample(&mut rng, &r.rho))
            .collect();
        let ci = interval(&mut boots);
        let excl = (r.ic.iter().sum::<f64>() - r.ic_validation)
            / (r.rho.iter().sum::<f64>() - r.rho_validation)
            * (r.rho.len() as f64 / r.ic.len() as f64);
        let ok = r.ic_verified && r.rho_verified;
        println!(
            "| {} | {} | {} | {:.2} | {} | {} | {:.4} [{:.4}, {:.4}] | {:.4} | {} | {} |",
            r.genus,
            r.p,
            r.n,
            r.n.log2(),
            r.ic.len(),
            r.rho.len(),
            ratio,
            ci[0],
            ci[1],
            excl,
            r.in_situ
                .map(|x| format!("{x:.4}"))
                .unwrap_or_else(|| "—".into()),
            if ok { "yes" } else { "NO" },
        );
        row_json.push(json!({
            "genus": r.genus, "p": r.p, "n": r.n, "log2_n": r.n.log2(),
            "ic_runs": r.ic.len(), "rho_runs": r.rho.len(),
            "ic_bitops_mean": mean(&r.ic), "rho_bitops_mean": mean(&r.rho),
            "ic_over_rho": ratio, "ic_over_rho_ci95": ci,
            "ic_over_rho_excluding_validation": excl,
            "in_situ_s_ic_over_s_rho": r.in_situ,
            "verified": ok,
        }));
    }

    // Per genus: slope of log2(IC/rho) on log2 N over verified rows, with a
    // two-stage bootstrap (rows with replacement, then runs within each row).
    let mut by_genus: BTreeMap<u64, Vec<&Row>> = BTreeMap::new();
    for r in rows.iter().filter(|r| r.ic_verified && r.rho_verified) {
        by_genus.entry(r.genus).or_default().push(r);
    }
    println!();
    println!("| genus | rows | slope of log2(IC/rho) on log2 N [95%] | degenerate resamples |");
    println!("|--:|--:|:--|--:|");
    let mut fits = serde_json::Map::new();
    for (g, rs) in &by_genus {
        let xs: Vec<f64> = rs.iter().map(|r| r.n.log2()).collect();
        let ys: Vec<f64> = rs
            .iter()
            .map(|r| (mean(&r.ic) / mean(&r.rho)).log2())
            .collect();
        let Some((slope, intercept)) = fit(&xs, &ys) else {
            continue;
        };
        let mut slopes = Vec::with_capacity(RESAMPLES);
        let mut degenerate = 0usize;
        for _ in 0..RESAMPLES {
            let pick: Vec<&Row> = (0..rs.len())
                .map(|_| rs[rng.gen_range(0..rs.len())])
                .collect();
            let bx: Vec<f64> = pick.iter().map(|r| r.n.log2()).collect();
            let by: Vec<f64> = pick
                .iter()
                .map(|r| (resample(&mut rng, &r.ic) / resample(&mut rng, &r.rho)).log2())
                .collect();
            match fit(&bx, &by) {
                Some((s, _)) => slopes.push(s),
                None => degenerate += 1,
            }
        }
        let ci = interval(&mut slopes);
        println!(
            "| {g} | {} | {slope:.3} [{:.3}, {:.3}] | {degenerate} |",
            rs.len(),
            ci[0],
            ci[1]
        );
        fits.insert(
            g.to_string(),
            json!({"rows": rs.len(), "slope": slope, "intercept": intercept,
                   "slope_ci95": ci, "degenerate_resamples": degenerate}),
        );
    }

    if let Some(path) = out {
        let doc = json!({
            "schema": "hec-bitop-analysis/v1",
            "unit": "bitop/v1",
            "resamples": RESAMPLES,
            "seed": SEED,
            "rows": row_json,
            "fits": fits,
        });
        fs::write(&path, serde_json::to_string_pretty(&doc).unwrap() + "\n").expect("write --out");
        eprintln!("wrote {path}");
    }
}
