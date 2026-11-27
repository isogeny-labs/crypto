//! Native analyzer for `EXP-C25519-PAIRTRACE-001`.
//!
//! Reads the gated d=7 panel, verifies correctness and predecessor
//! reproduction, and emits deterministic JSON, CSV, SVG, and export evidence.

use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const SEED: u64 = 2_026_112_503;
const ARMS: [&str; 3] = ["D", "R", "P"];

#[derive(Clone, Debug, serde::Deserialize)]
struct Row {
    schema: String,
    arm: String,
    d: u64,
    kind: String,
    index: usize,
    target_scalar: u64,
    target_x: u64,
    target_y: u64,
    field_ops: u64,
    input_terms: usize,
    input_degree: u32,
    degree_reached: u32,
    solving_degree: u32,
    solving_degree_max: u32,
    censored: bool,
    timed_out: bool,
    inconsistent: bool,
    membership_build_coeff_ops: u64,
    n_vars: usize,
    n_equations: usize,
    peak_rss_kib: Option<u64>,
    max_rows: usize,
    max_cols: usize,
    max_cols_to_solution: usize,
    steps: usize,
    steps_to_solution: usize,
    planted_equations_zero: Option<bool>,
    planted_group_verified: Option<bool>,
}

#[derive(Clone, Copy)]
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn arg(args: &[String], key: &str, default: &str) -> String {
    args.windows(2)
        .find(|w| w[0] == key)
        .map(|w| w[1].clone())
        .unwrap_or_else(|| default.to_owned())
}

fn read_rows(dir: &str) -> Result<Vec<Row>, String> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("read {dir}: {e}"))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|x| x.to_str()) == Some("json"))
        .collect();
    paths.sort();
    paths
        .iter()
        .map(|path| {
            let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
            serde_json::from_slice(&bytes).map_err(|e| format!("parse {}: {e}", path.display()))
        })
        .collect()
}

fn geometric_mean(values: &[f64]) -> f64 {
    (values.iter().map(|x| x.ln()).sum::<f64>() / values.len() as f64).exp()
}

fn paired_interval(ratios: &[f64], seed: u64, draws: usize) -> (f64, f64) {
    let mut rng = Rng(seed);
    let mut samples = Vec::with_capacity(draws);
    for _ in 0..draws {
        let mut log_sum = 0.0;
        for _ in 0..ratios.len() {
            log_sum += ratios[rng.next() as usize % ratios.len()].ln();
        }
        samples.push((log_sum / ratios.len() as f64).exp());
    }
    samples.sort_by(f64::total_cmp);
    (
        samples[(draws - 1) * 25 / 1000],
        samples[(draws - 1) * 975 / 1000],
    )
}

fn row<'a>(rows: &'a [Row], kind: &str, index: usize, arm: &str) -> Result<&'a Row, String> {
    rows.iter()
        .find(|r| r.d == 7 && r.kind == kind && r.index == index && r.arm == arm)
        .ok_or_else(|| format!("missing d=7 {kind} i={index} arm={arm}"))
}

fn reproduction_equal(current: &Row, old: &Row) -> bool {
    current.target_scalar == old.target_scalar
        && current.target_x == old.target_x
        && current.target_y == old.target_y
        && current.field_ops == old.field_ops
        && current.input_terms == old.input_terms
        && current.input_degree == old.input_degree
        && current.n_vars == old.n_vars
        && current.n_equations == old.n_equations
        && current.membership_build_coeff_ops == old.membership_build_coeff_ops
        && current.degree_reached == old.degree_reached
        && current.solving_degree == old.solving_degree
        && current.solving_degree_max == old.solving_degree_max
        && current.max_rows == old.max_rows
        && current.max_cols == old.max_cols
        && current.max_cols_to_solution == old.max_cols_to_solution
        && current.steps == old.steps
        && current.steps_to_solution == old.steps_to_solution
        && current.censored == old.censored
        && current.timed_out == old.timed_out
        && current.inconsistent == old.inconsistent
}

fn main() {
    if let Err(error) = run() {
        eprintln!("curve25519_pair_trace_analyze: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let raw = arg(
        &args,
        "--raw",
        "research/curve25519_pair_trace_20261125/raw/d7",
    );
    let reproduction_new = arg(
        &args,
        "--reproduction-new",
        "research/curve25519_pair_trace_20261125/preflight/d-reproduction/new",
    );
    let reproduction_old = arg(
        &args,
        "--reproduction-old",
        "research/curve25519_pair_trace_20261125/preflight/d-reproduction/old",
    );
    let selftest_path = arg(
        &args,
        "--selftest",
        "research/curve25519_pair_trace_20261125/preflight/selftest.json",
    );
    let out = arg(
        &args,
        "--out",
        "research/curve25519_pair_trace_20261125/evidence",
    );

    let rows = read_rows(&raw)?;
    if rows.len() != 24 {
        return Err(format!("expected 24 d=7 rows, found {}", rows.len()));
    }
    if rows.iter().any(|r| {
        r.schema != "curve25519.pair-trace-row/v1" || r.d != 7 || !ARMS.contains(&r.arm.as_str())
    }) {
        return Err("unexpected row schema, order, or arm".into());
    }
    let mut unique = HashSet::new();
    for r in &rows {
        if !unique.insert((r.kind.clone(), r.index, r.arm.clone())) {
            return Err(format!("duplicate row {} i={} {}", r.kind, r.index, r.arm));
        }
    }

    let selftest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&selftest_path).map_err(|e| format!("read {selftest_path}: {e}"))?,
    )
    .map_err(|e| format!("parse {selftest_path}: {e}"))?;
    let selftest_ok = selftest["status"] == "pass"
        && selftest["all_arm_truth_agreement"] == true
        && selftest["all_split_input_equations_quadratic"] == true;

    let planted_bad = rows
        .iter()
        .filter(|r| {
            r.kind == "planted"
                && (r.planted_equations_zero != Some(true)
                    || r.planted_group_verified != Some(true))
        })
        .count();

    let mut target_pairs_exact = true;
    for (kind, count) in [("random", 4usize), ("planted", 4usize)] {
        for index in 0..count {
            let d = row(&rows, kind, index, "D")?;
            for arm in ["R", "P"] {
                let other = row(&rows, kind, index, arm)?;
                target_pairs_exact &= (d.target_scalar, d.target_x, d.target_y)
                    == (other.target_scalar, other.target_x, other.target_y);
            }
        }
    }

    let new_rows = read_rows(&reproduction_new)?;
    let old_rows = read_rows(&reproduction_old)?;
    if new_rows.len() != 8 || old_rows.len() != 8 {
        return Err(format!(
            "expected 8/8 reproduction rows, found {}/{}",
            new_rows.len(),
            old_rows.len()
        ));
    }
    let mut reproduction_mismatches = 0usize;
    for (kind, count) in [("random", 4usize), ("planted", 4usize)] {
        for index in 0..count {
            let current = row(&new_rows, kind, index, "D")?;
            let old = row(&old_rows, kind, index, "D")?;
            if !reproduction_equal(current, old) {
                reproduction_mismatches += 1;
            }
        }
    }

    let ratios_pd: Vec<f64> = (0..4)
        .map(|index| {
            Ok(row(&rows, "random", index, "P")?.field_ops as f64
                / row(&rows, "random", index, "D")?.field_ops as f64)
        })
        .collect::<Result<_, String>>()?;
    let ratios_pr: Vec<f64> = (0..4)
        .map(|index| {
            Ok(row(&rows, "random", index, "P")?.field_ops as f64
                / row(&rows, "random", index, "R")?.field_ops as f64)
        })
        .collect::<Result<_, String>>()?;
    let ratios_rd: Vec<f64> = (0..4)
        .map(|index| {
            Ok(row(&rows, "random", index, "R")?.field_ops as f64
                / row(&rows, "random", index, "D")?.field_ops as f64)
        })
        .collect::<Result<_, String>>()?;
    let (pd_lo, pd_hi) = paired_interval(&ratios_pd, SEED, 10_000);
    let (pr_lo, pr_hi) = paired_interval(&ratios_pr, SEED ^ 0x5052, 10_000);
    let pd_gm = geometric_mean(&ratios_pd);
    let pr_gm = geometric_mean(&ratios_pr);
    let rd_gm = geometric_mean(&ratios_rd);

    let p_degree_at_most_four = (0..4)
        .filter(|&index| row(&rows, "random", index, "P").is_ok_and(|r| r.solving_degree_max <= 4))
        .count();
    let d_censored = (0..4)
        .filter(|&index| row(&rows, "random", index, "D").is_ok_and(|r| r.censored))
        .count();
    let p_censored = (0..4)
        .filter(|&index| row(&rows, "random", index, "P").is_ok_and(|r| r.censored))
        .count();
    let max_rss_ratio = (0..4)
        .map(|index| {
            let p = row(&rows, "random", index, "P")?
                .peak_rss_kib
                .ok_or("P row lacks peak RSS")?;
            let d = row(&rows, "random", index, "D")?
                .peak_rss_kib
                .ok_or("D row lacks peak RSS")?;
            Ok(p as f64 / d as f64)
        })
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .fold(0.0f64, f64::max);

    let gates = serde_json::json!({
        "selftest": selftest_ok,
        "planted_correctness": planted_bad == 0,
        "target_pairs_exact": target_pairs_exact,
        "D_predecessor_reproduction": reproduction_mismatches == 0,
        "P_censoring_not_greater": p_censored <= d_censored,
        "minimum_three_pairs": ratios_pd.len() >= 3,
        "P_over_D_upper_lt_1_00": pd_hi < 1.0,
        "degree_at_most_four_on_three_pairs": p_degree_at_most_four >= 3,
        "peak_rss_P_over_D_each_le_1_50": max_rss_ratio <= 1.5
    });
    let qualifies = gates
        .as_object()
        .unwrap()
        .values()
        .all(|value| value == &serde_json::Value::Bool(true));

    let mut cells = serde_json::Map::new();
    for arm in ARMS {
        let random: Vec<&Row> = rows
            .iter()
            .filter(|r| r.kind == "random" && r.arm == arm)
            .collect();
        let ops: Vec<f64> = random.iter().map(|r| r.field_ops as f64).collect();
        let degrees: Vec<u32> = random.iter().map(|r| r.solving_degree_max).collect();
        cells.insert(
            arm.to_string(),
            serde_json::json!({
                "random_rows": random.len(),
                "censored": random.iter().filter(|r| r.censored).count(),
                "field_ops_arithmetic_mean": ops.iter().sum::<f64>() / ops.len() as f64,
                "field_ops_geometric_mean": geometric_mean(&ops),
                "n_vars": random[0].n_vars,
                "n_equations": random[0].n_equations,
                "input_degree": random[0].input_degree,
                "solving_degree_max_values": degrees,
                "peak_rss_kib_max": random.iter().filter_map(|r| r.peak_rss_kib).max()
            }),
        );
    }

    let analysis = serde_json::json!({
        "schema": "curve25519.pair-trace-analysis/v1",
        "study_id": "EXP-C25519-PAIRTRACE-001",
        "stage": "d7_screen",
        "planned": 24,
        "attempted": rows.len(),
        "completed": rows.iter().filter(|r| !r.censored).count(),
        "censored": rows.iter().filter(|r| r.censored).count(),
        "planted_bad": planted_bad,
        "target_pairs_exact": target_pairs_exact,
        "D_reproduction_mismatches": reproduction_mismatches,
        "cells": cells,
        "qualification_d7": {
            "paired_P_over_D_geometric_mean": pd_gm,
            "paired_P_over_D_descriptive_95": [pd_lo, pd_hi],
            "paired_P_over_R_geometric_mean": pr_gm,
            "paired_P_over_R_descriptive_95": [pr_lo, pr_hi],
            "paired_R_over_D_geometric_mean": rd_gm,
            "P_degree_at_most_four_pairs": p_degree_at_most_four,
            "D_random_censored": d_censored,
            "P_random_censored": p_censored,
            "max_P_over_D_peak_rss_ratio": max_rss_ratio,
            "gates": gates
        },
        "qualifies_for_d9": qualifies,
        "decision": if qualifies { "continue_to_d9" } else { "stop_before_d9_d19_and_m4" },
        "claim_boundary": "d=7 relation-system stage diagnostic only; no total-work or subexponential ECDLP claim"
    });

    std::fs::create_dir_all(&out).map_err(|e| format!("mkdir {out}: {e}"))?;
    std::fs::write(
        Path::new(&out).join("analysis.json"),
        serde_json::to_string_pretty(&analysis).unwrap() + "\n",
    )
    .map_err(|e| format!("write analysis: {e}"))?;

    let mut csv = String::from(
        "index,D_field_ops,R_field_ops,P_field_ops,R_over_D,P_over_D,P_over_R,P_over_D_rss\n",
    );
    for index in 0..4 {
        let d = row(&rows, "random", index, "D")?;
        let r = row(&rows, "random", index, "R")?;
        let p = row(&rows, "random", index, "P")?;
        writeln!(
            csv,
            "{index},{},{},{},{:.12},{:.12},{:.12},{:.12}",
            d.field_ops,
            r.field_ops,
            p.field_ops,
            r.field_ops as f64 / d.field_ops as f64,
            p.field_ops as f64 / d.field_ops as f64,
            p.field_ops as f64 / r.field_ops as f64,
            p.peak_rss_kib.unwrap() as f64 / d.peak_rss_kib.unwrap() as f64
        )
        .unwrap();
    }
    std::fs::write(Path::new(&out).join("paired.csv"), csv)
        .map_err(|e| format!("write paired csv: {e}"))?;

    let y = |ratio: f64| 430.0 - ratio.log10() / 3.6 * 325.0;
    let mut marks = String::new();
    for (index, (arm, ratio)) in [("R", rd_gm), ("P", pd_gm)].into_iter().enumerate() {
        let x = 300.0 + index as f64 * 300.0;
        let color = if arm == "P" { "#d1495b" } else { "#457b9d" };
        writeln!(marks, "<line x1=\"{x:.1}\" y1=\"430\" x2=\"{x:.1}\" y2=\"{top:.1}\" stroke=\"{color}\" stroke-width=\"70\"/><text x=\"{x:.1}\" y=\"{label:.1}\" text-anchor=\"middle\" font-size=\"17\">{ratio:.2}x</text><text x=\"{x:.1}\" y=\"465\" text-anchor=\"middle\" font-size=\"18\">{arm}</text>", top=y(ratio), label=y(ratio)-12.0).unwrap();
    }
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="900" height="520" viewBox="0 0 900 520">
<rect width="900" height="520" fill="#ffffff"/>
<text x="450" y="34" text-anchor="middle" font-family="sans-serif" font-size="22" font-weight="bold">Quadratized split / direct F4 work at d=7</text>
<text x="450" y="59" text-anchor="middle" font-family="sans-serif" font-size="14">paired geometric mean; log10 scale; lower is better</text>
<line x1="90" y1="430" x2="835" y2="430" stroke="#2a9d8f" stroke-width="2" stroke-dasharray="8 6"/>
<line x1="90" y1="85" x2="90" y2="430" stroke="#333"/>
<text x="830" y="420" text-anchor="end" font-family="sans-serif" font-size="14" fill="#16766c">frozen P ceiling 1.00x</text>
{marks}
<text x="450" y="500" text-anchor="middle" font-family="sans-serif" font-size="16">split arm</text>
<text x="25" y="260" transform="rotate(-90 25 260)" text-anchor="middle" font-family="sans-serif" font-size="16">arm / D field_ops (log10)</text>
<text x="110" y="101" font-family="sans-serif" font-size="13" fill="#555">[pcc:unverified] stage diagnostic</text>
</svg>
"##
    );
    std::fs::write(Path::new(&out).join("figure.svg"), svg)
        .map_err(|e| format!("write svg: {e}"))?;

    let export = serde_json::json!({
        "schema": "experiment.export/v1",
        "experiment_id": "EXP-C25519-PAIRTRACE-001",
        "status": if qualifies { "d7_qualified" } else { "completed_negative" },
        "flow": {"planned": 24, "attempted": rows.len(), "completed": rows.iter().filter(|r| !r.censored).count(), "excluded": 0, "censored": rows.iter().filter(|r| r.censored).count()},
        "primary_metric": "F_p multiply-adds in F4 elimination",
        "decision": if qualifies { "continue_to_d9" } else { "stop_before_d9_d19_and_m4" },
        "label": "[pcc:unverified]"
    });
    std::fs::write(
        Path::new(&out).join("experiment.export.json"),
        serde_json::to_string_pretty(&export).unwrap() + "\n",
    )
    .map_err(|e| format!("write export: {e}"))?;

    println!("{}", serde_json::to_string(&analysis).unwrap());
    Ok(())
}
