//! Summary of `gaudry_quartic_c4_options` runs: per arm, mean `C₄` and its
//! phases, the ratio of means against the committed arm, the paired
//! per-residual ratios, and the oracle agreement.
//!
//! ```text
//! cargo run --release --example gaudry_quartic_c4_options_summary -- \
//!     research/s5_plain_c4_options_20261019/raw/s5_p269_seed1.json \
//!     research/s5_plain_c4_options_20261019/raw/generic_d4-7.json \
//!     research/s5_plain_c4_options_20261019/raw/summary.json
//! ```
//!
//! A `+replay` arm's first solve records the shape and is not a replay; it
//! is excluded from that arm's mean and listed separately, as #1792 did.

use std::collections::BTreeMap;
use std::env;
use std::fs;

use serde::Serialize;
use serde_json::Value;

fn u(v: &Value, k: &str) -> f64 {
    v["solve"][k].as_f64().unwrap_or(0.0)
}

const PHASES: [&str; 5] = [
    "echelon_muls",
    "nf_muls",
    "charpoly_muls",
    "eigenvector_muls",
    "total_muls",
];

#[derive(Serialize)]
struct ArmSummary {
    arm: String,
    /// Residuals in the mean (a replay arm's recorded first solve excluded).
    n: usize,
    mean: BTreeMap<String, f64>,
    /// committed mean / this arm's mean, over the same residuals.
    ratio_of_means: f64,
    /// committed / this arm on each residual in the mean.
    paired_ratios: Vec<f64>,
    paired_min: f64,
    paired_max: f64,
    log2_mean_c4: f64,
    /// `log2 C₄ − 36` (`D³` with `D = 4,096`).
    bits_above_d_cubed: f64,
    agree: usize,
    solves: usize,
    first_solve_c4: Option<f64>,
    replays: Vec<String>,
    wall_s: Vec<f64>,
    order: Vec<u64>,
}

#[derive(Serialize)]
struct GenericSummary {
    equation_degree: u64,
    arm: String,
    c: f64,
    ratio_vs_committed: f64,
    planted_found: bool,
}

#[derive(Serialize)]
struct Summary {
    source: String,
    generic_source: Option<String>,
    arms: Vec<ArmSummary>,
    generic: Vec<GenericSummary>,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let src = &args[0];
    let r: Value = serde_json::from_str(&fs::read_to_string(src).expect("read")).expect("json");
    let arms: Vec<String> = r["arms"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a.as_str().unwrap().to_string())
        .collect();
    let residuals = r["residuals"].as_array().unwrap();
    let find = |res: &Value, arm: &str| -> Option<Value> {
        res["arms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["arm"] == arm)
            .cloned()
    };
    let mut out = Vec::new();
    for arm in &arms {
        let replay = arm.ends_with("+replay");
        let mut rows: Vec<(Value, Value)> = Vec::new();
        let mut first = None;
        let mut agree = 0;
        let mut solves = 0;
        let mut replays = Vec::new();
        let mut wall = Vec::new();
        let mut order = Vec::new();
        for (i, res) in residuals.iter().enumerate() {
            let (Some(a), Some(c)) = (find(res, arm), find(res, "committed")) else {
                continue;
            };
            solves += 1;
            if a["agree"].as_bool() == Some(true) {
                agree += 1;
            }
            replays.push(a["solve"]["replay"].as_str().unwrap_or("").to_string());
            wall.push(a["wall_s"].as_f64().unwrap());
            order.push(a["order"].as_u64().unwrap());
            if replay && i == 0 {
                first = Some(u(&a, "total_muls"));
                continue;
            }
            rows.push((a, c));
        }
        let n = rows.len();
        let mut mean = BTreeMap::new();
        let mut cmean = 0.0;
        for ph in PHASES {
            let m = rows.iter().map(|(a, _)| u(a, ph)).sum::<f64>() / n as f64;
            mean.insert(ph.to_string(), m);
        }
        for (_, c) in &rows {
            cmean += u(c, "total_muls") / n as f64;
        }
        let paired: Vec<f64> = rows
            .iter()
            .map(|(a, c)| u(c, "total_muls") / u(a, "total_muls"))
            .collect();
        let c4 = mean["total_muls"];
        out.push(ArmSummary {
            arm: arm.clone(),
            n,
            ratio_of_means: cmean / c4,
            paired_min: paired.iter().cloned().fold(f64::INFINITY, f64::min),
            paired_max: paired.iter().cloned().fold(0.0, f64::max),
            paired_ratios: paired,
            log2_mean_c4: c4.log2(),
            bits_above_d_cubed: c4.log2() - 36.0,
            mean,
            agree,
            solves,
            first_solve_c4: first,
            replays,
            wall_s: wall,
            order,
        });
    }
    println!("| arm | n | mean C4 | log2 | ratio of means | paired min–max | echelon | nf | charpoly | eigen | agree |");
    for a in &out {
        println!(
            "| {} | {} | {:.4e} | {:.2} | {:.2} | {:.2}–{:.2} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {}/{} |",
            a.arm,
            a.n,
            a.mean["total_muls"],
            a.log2_mean_c4,
            a.ratio_of_means,
            a.paired_min,
            a.paired_max,
            a.mean["echelon_muls"],
            a.mean["nf_muls"],
            a.mean["charpoly_muls"],
            a.mean["eigenvector_muls"],
            a.agree,
            a.solves
        );
    }
    let mut generic = Vec::new();
    if let Some(g) = args.get(1).filter(|s| s.as_str() != "-") {
        let rows: Vec<Value> =
            serde_json::from_str(&fs::read_to_string(g).expect("read")).expect("json");
        for row in &rows {
            let d = row["equation_degree"].as_u64().unwrap();
            let base = rows
                .iter()
                .find(|x| x["equation_degree"].as_u64() == Some(d) && x["arm"] == "committed")
                .map(|x| u(x, "total_muls"))
                .unwrap_or(f64::NAN);
            let c = u(row, "total_muls");
            println!(
                "generic d={d} {:<24} C = {c:.4e}  committed/arm {:.2}",
                row["arm"].as_str().unwrap(),
                base / c
            );
            generic.push(GenericSummary {
                equation_degree: d,
                arm: row["arm"].as_str().unwrap().to_string(),
                c,
                ratio_vs_committed: base / c,
                planted_found: row["planted_found"].as_bool().unwrap_or(false),
            });
        }
    }
    if let Some(path) = args.get(2) {
        let s = Summary {
            source: src.clone(),
            generic_source: args.get(1).cloned(),
            arms: out,
            generic,
        };
        fs::write(path, serde_json::to_string_pretty(&s).unwrap()).expect("write");
        println!("wrote {path}");
    }
}
