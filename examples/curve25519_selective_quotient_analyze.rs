//! Native analyzer for `EXP-C25519-SELCUT-001`.
//!
//! The measurement binary is deliberately separate and hash-pinned.  This
//! program reads its JSON rows, checks the frozen reproduction/correctness
//! gates, and emits deterministic JSON, CSV, and SVG evidence.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const SEED: u64 = 20_261_125;
const ARMS: [&str; 6] = ["D", "L", "B", "N", "U", "A"];

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
    input_degree: u32,
    degree_reached: u32,
    solving_degree: u32,
    solving_degree_max: u32,
    censored: bool,
    timed_out: bool,
    inconsistent: bool,
    membership_build_coeff_ops: u64,
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
        (samples[(draws - 1) * 25 / 1000]),
        (samples[(draws - 1) * 975 / 1000]),
    )
}

fn row<'a>(rows: &'a [Row], kind: &str, index: usize, arm: &str) -> Result<&'a Row, String> {
    rows.iter()
        .find(|r| r.d == 19 && r.kind == kind && r.index == index && r.arm == arm)
        .ok_or_else(|| format!("missing d=19 {kind} i={index} arm={arm}"))
}

fn reproduction_equal(current: &Row, old: &Row) -> bool {
    current.target_scalar == old.target_scalar
        && current.target_x == old.target_x
        && current.target_y == old.target_y
        && current.field_ops == old.field_ops
        && current.input_degree == old.input_degree
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

fn fmt_ratio(value: f64) -> String {
    format!("{value:.12}")
}

fn main() {
    if let Err(error) = run() {
        eprintln!("curve25519_selective_quotient_analyze: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let raw = arg(
        &args,
        "--raw",
        "research/curve25519_selective_quotient_20261125/raw",
    );
    let predecessor = arg(
        &args,
        "--predecessor",
        "research/curve25519_symmetric_remainder_20261125/raw",
    );
    let selftest_path = arg(
        &args,
        "--selftest",
        "research/curve25519_selective_quotient_20261125/preflight/selftest.json",
    );
    let out = arg(
        &args,
        "--out",
        "research/curve25519_selective_quotient_20261125/evidence",
    );

    let rows = read_rows(&raw)?;
    if rows.len() != 72 {
        return Err(format!("expected 72 rows, found {}", rows.len()));
    }
    if rows.iter().any(|r| {
        r.schema != "curve25519.selective-quotient-row/v1"
            || r.d != 19
            || !ARMS.contains(&r.arm.as_str())
    }) {
        return Err("unexpected schema, order, or arm".into());
    }
    let mut unique = HashSet::new();
    for r in &rows {
        if !unique.insert((r.kind.clone(), r.index, r.arm.clone())) {
            return Err(format!("duplicate key {} i={} {}", r.kind, r.index, r.arm));
        }
    }

    let selftest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&selftest_path).map_err(|e| format!("read {selftest_path}: {e}"))?,
    )
    .map_err(|e| format!("parse {selftest_path}: {e}"))?;
    let selftest_pass = selftest["schema"] == "curve25519.selective-quotient-selftest/v1"
        && selftest["status"] == "pass"
        && selftest["all_arm_truth_agreement"] == true;

    let predecessor_rows = read_rows(&predecessor)?;
    let mut target_pairs_exact = true;
    let mut d_reproduction_mismatches = 0usize;
    let mut a_reproduction_mismatches = 0usize;
    let mut planted_bad = 0usize;
    let mut paired_csv = String::from(
        "kind,index,arm,field_ops,ratio_to_D,solving_degree_max,peak_rss_kib,rss_ratio_to_D,membership_build_coeff_ops,build_ratio_to_D,censored\n",
    );

    for kind in ["random", "planted"] {
        let count = if kind == "random" { 8 } else { 4 };
        for index in 0..count {
            let direct = row(&rows, kind, index, "D")?;
            for arm in ARMS {
                let current = row(&rows, kind, index, arm)?;
                target_pairs_exact &= (current.target_scalar, current.target_x, current.target_y)
                    == (direct.target_scalar, direct.target_x, direct.target_y);
                if kind == "planted"
                    && (current.planted_equations_zero != Some(true)
                        || current.planted_group_verified != Some(true))
                {
                    planted_bad += 1;
                }
                let field_ratio = current.field_ops as f64 / direct.field_ops as f64;
                let rss_ratio = match (current.peak_rss_kib, direct.peak_rss_kib) {
                    (Some(candidate), Some(base)) => candidate as f64 / base as f64,
                    _ => f64::NAN,
                };
                let build_ratio = current.membership_build_coeff_ops as f64
                    / direct.membership_build_coeff_ops as f64;
                writeln!(
                    paired_csv,
                    "{kind},{index},{arm},{},{},{},{},{},{},{},{}",
                    current.field_ops,
                    fmt_ratio(field_ratio),
                    current.solving_degree_max,
                    current.peak_rss_kib.unwrap_or(0),
                    fmt_ratio(rss_ratio),
                    current.membership_build_coeff_ops,
                    fmt_ratio(build_ratio),
                    current.censored
                )
                .unwrap();
            }

            let old_direct = predecessor_rows
                .iter()
                .find(|r| r.d == 19 && r.kind == kind && r.index == index && r.arm == "direct")
                .ok_or_else(|| format!("missing predecessor direct {kind} i={index}"))?;
            let old_quotient = predecessor_rows
                .iter()
                .find(|r| r.d == 19 && r.kind == kind && r.index == index && r.arm == "quotient")
                .ok_or_else(|| format!("missing predecessor quotient {kind} i={index}"))?;
            d_reproduction_mismatches += (!reproduction_equal(direct, old_direct)) as usize;
            d_reproduction_mismatches += (direct.membership_build_coeff_ops
                != old_direct.membership_build_coeff_ops)
                as usize;
            let all = row(&rows, kind, index, "A")?;
            a_reproduction_mismatches += (!reproduction_equal(all, old_quotient)) as usize;
            a_reproduction_mismatches += (all.membership_build_coeff_ops
                != old_quotient.membership_build_coeff_ops)
                as usize;
        }
    }

    let mut cells = BTreeMap::new();
    for arm in ARMS {
        let selected: Vec<&Row> = rows
            .iter()
            .filter(|r| r.kind == "random" && r.arm == arm)
            .collect();
        let field_values: Vec<f64> = selected.iter().map(|r| r.field_ops as f64).collect();
        let ratios: Vec<f64> = selected
            .iter()
            .map(|candidate| {
                candidate.field_ops as f64
                    / row(&rows, "random", candidate.index, "D")
                        .unwrap()
                        .field_ops as f64
            })
            .collect();
        cells.insert(
            arm,
            serde_json::json!({
                "random_rows": selected.len(),
                "censored": selected.iter().filter(|r| r.censored).count(),
                "field_ops_arithmetic_mean": field_values.iter().sum::<f64>() / field_values.len() as f64,
                "field_ops_geometric_mean": geometric_mean(&field_values),
                "paired_ratio_to_D_geometric_mean": geometric_mean(&ratios),
                "solving_degree_max_values": selected.iter().map(|r| r.solving_degree_max).collect::<BTreeSet<_>>(),
                "peak_rss_kib_max": selected.iter().filter_map(|r| r.peak_rss_kib).max(),
                "membership_build_coeff_ops_max": selected.iter().map(|r| r.membership_build_coeff_ops).max()
            }),
        );
    }

    let completed_pairs: Vec<(&Row, &Row)> = (0..8)
        .filter_map(|index| {
            let d = row(&rows, "random", index, "D").ok()?;
            let b = row(&rows, "random", index, "B").ok()?;
            (!d.censored && !b.censored).then_some((d, b))
        })
        .collect();
    let ratios: Vec<f64> = completed_pairs
        .iter()
        .map(|(d, b)| b.field_ops as f64 / d.field_ops as f64)
        .collect();
    let (ratio_lo, ratio_hi) = paired_interval(&ratios, SEED, 10_000);
    let degree_at_most_four = completed_pairs
        .iter()
        .filter(|(_, b)| b.solving_degree_max <= 4)
        .count();
    let max_rss_ratio = completed_pairs
        .iter()
        .map(|(d, b)| b.peak_rss_kib.unwrap() as f64 / d.peak_rss_kib.unwrap() as f64)
        .fold(0.0, f64::max);
    let max_build_ratio = completed_pairs
        .iter()
        .map(|(d, b)| b.membership_build_coeff_ops as f64 / d.membership_build_coeff_ops as f64)
        .fold(0.0, f64::max);
    let u_degree_drop_pairs = (0..8)
        .filter(|&index| {
            row(&rows, "random", index, "U").unwrap().solving_degree_max
                < row(&rows, "random", index, "D").unwrap().solving_degree_max
        })
        .count();
    let d_censored = (0..8)
        .filter(|&index| row(&rows, "random", index, "D").unwrap().censored)
        .count();
    let b_censored = (0..8)
        .filter(|&index| row(&rows, "random", index, "B").unwrap().censored)
        .count();

    let gates = serde_json::json!({
        "selftest": selftest_pass,
        "planted_correctness": planted_bad == 0,
        "target_pairs_exact": target_pairs_exact,
        "D_predecessor_reproduction": d_reproduction_mismatches == 0,
        "A_predecessor_reproduction": a_reproduction_mismatches == 0,
        "U_degree_drop_at_most_two": u_degree_drop_pairs <= 2,
        "B_censoring_not_greater": b_censored <= d_censored,
        "minimum_six_pairs": completed_pairs.len() >= 6,
        "ratio_upper_lt_0_75": ratio_hi < 0.75,
        "degree_at_most_four_on_six_pairs": degree_at_most_four >= 6,
        "peak_rss_ratio_each_le_1_25": max_rss_ratio <= 1.25,
        "build_ratio_each_le_1_25": max_build_ratio <= 1.25
    });
    let qualifies = gates
        .as_object()
        .unwrap()
        .values()
        .all(|value| value == true);
    let analysis = serde_json::json!({
        "schema": "curve25519.selective-quotient-analysis/v1",
        "study_id": "EXP-C25519-SELCUT-001",
        "rows": rows.len(),
        "planned": 72,
        "attempted": 72,
        "completed": rows.iter().filter(|r| !r.censored).count(),
        "censored": rows.iter().filter(|r| r.censored).count(),
        "planted_bad": planted_bad,
        "target_pairs_exact": target_pairs_exact,
        "D_predecessor_reproduction_mismatches": d_reproduction_mismatches,
        "A_predecessor_reproduction_mismatches": a_reproduction_mismatches,
        "cells": cells,
        "qualification_d19": {
            "completed_random_pairs": completed_pairs.len(),
            "paired_B_over_D_geometric_mean": geometric_mean(&ratios),
            "paired_B_over_D_descriptive_95": [ratio_lo, ratio_hi],
            "B_degree_at_most_four_pairs": degree_at_most_four,
            "U_degree_drop_pairs": u_degree_drop_pairs,
            "D_random_censored": d_censored,
            "B_random_censored": b_censored,
            "max_B_over_D_membership_build_coeff_ops_ratio": max_build_ratio,
            "max_B_over_D_peak_rss_ratio": max_rss_ratio,
            "gates": gates
        },
        "qualifies_for_scaling": qualifies,
        "decision": if qualifies { "continue_to_gated_scaling" } else { "stop_before_scaling_and_m4" },
        "claim_boundary": "stage diagnostic only; no total-work or subexponential ECDLP claim"
    });

    std::fs::create_dir_all(&out).map_err(|e| format!("mkdir {out}: {e}"))?;
    std::fs::write(
        Path::new(&out).join("analysis.json"),
        serde_json::to_string_pretty(&analysis).unwrap() + "\n",
    )
    .map_err(|e| format!("write analysis: {e}"))?;
    std::fs::write(Path::new(&out).join("paired.csv"), paired_csv)
        .map_err(|e| format!("write paired csv: {e}"))?;

    let plotted = ["L", "B", "N", "U", "A"];
    let ratios_by_arm: Vec<(&str, f64)> = plotted
        .iter()
        .map(|&arm| {
            let arm_ratios: Vec<f64> = (0..8)
                .map(|index| {
                    row(&rows, "random", index, arm).unwrap().field_ops as f64
                        / row(&rows, "random", index, "D").unwrap().field_ops as f64
                })
                .collect();
            (arm, geometric_mean(&arm_ratios))
        })
        .collect();
    let x = |index: usize| 155.0 + index as f64 * 145.0;
    let y = |ratio: f64| 430.0 - ratio / 2.25 * 325.0;
    let mut marks = String::new();
    for (index, &(arm, ratio)) in ratios_by_arm.iter().enumerate() {
        let color = if arm == "B" { "#d1495b" } else { "#457b9d" };
        writeln!(marks, "<line x1=\"{x:.1}\" y1=\"{base:.1}\" x2=\"{x:.1}\" y2=\"{top:.1}\" stroke=\"{color}\" stroke-width=\"34\"/><text x=\"{x:.1}\" y=\"{label:.1}\" text-anchor=\"middle\" font-size=\"15\">{ratio:.3}</text><text x=\"{x:.1}\" y=\"465\" text-anchor=\"middle\" font-size=\"16\">{arm}</text>", x=x(index), base=y(0.0), top=y(ratio), label=y(ratio)-10.0).unwrap();
    }
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="900" height="520" viewBox="0 0 900 520">
<rect width="900" height="520" fill="#ffffff"/>
<text x="450" y="34" text-anchor="middle" font-family="sans-serif" font-size="22" font-weight="bold">Selective quotient cuts / direct F4 work</text>
<text x="450" y="59" text-anchor="middle" font-family="sans-serif" font-size="14">paired geometric mean, 8 random d=19 targets; lower is better</text>
<line x1="90" y1="430" x2="835" y2="430" stroke="#333"/>
<line x1="90" y1="85" x2="90" y2="430" stroke="#333"/>
<line x1="90" y1="{threshold:.1}" x2="835" y2="{threshold:.1}" stroke="#2a9d8f" stroke-width="2" stroke-dasharray="8 6"/>
<text x="830" y="{threshold_label:.1}" text-anchor="end" font-family="sans-serif" font-size="14" fill="#16766c">primary B ceiling 0.75</text>
{marks}
<text x="450" y="500" text-anchor="middle" font-family="sans-serif" font-size="16">cut policy</text>
<text x="25" y="260" transform="rotate(-90 25 260)" text-anchor="middle" font-family="sans-serif" font-size="16">candidate / D field_ops</text>
<text x="110" y="101" font-family="sans-serif" font-size="13" fill="#555">[pcc:unverified] stage diagnostic</text>
</svg>
"##,
        threshold = y(0.75),
        threshold_label = y(0.75) - 8.0,
    );
    std::fs::write(Path::new(&out).join("figure.svg"), svg)
        .map_err(|e| format!("write svg: {e}"))?;
    let export = serde_json::json!({
        "schema": "experiment.export/v1",
        "experiment_id": "EXP-C25519-SELCUT-001",
        "status": if qualifies { "qualified" } else { "completed_negative" },
        "flow": {"planned": 72, "attempted": 72, "completed": rows.iter().filter(|r| !r.censored).count(), "excluded": 0, "censored": rows.iter().filter(|r| r.censored).count()},
        "primary_metric": "F_p multiply-adds in F4 elimination",
        "decision": if qualifies { "continue_to_gated_scaling" } else { "stop_before_scaling_and_m4" },
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
