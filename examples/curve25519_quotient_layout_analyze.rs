//! Native analyzer for `EXP-C25519-QLAYOUT-001`.
//!
//! The measurement binary is deliberately separate and hash-pinned.  This
//! program reads its JSON rows, checks the frozen reproduction/correctness
//! gates, and emits deterministic JSON, CSV, and SVG evidence.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const SEED: u64 = 2_026_112_502;
const ARMS: [&str; 5] = ["D", "T", "F", "E", "I"];

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

fn fmt_ratio(value: f64) -> String {
    format!("{value:.12}")
}

fn main() {
    if let Err(error) = run() {
        eprintln!("curve25519_quotient_layout_analyze: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let raw = arg(
        &args,
        "--raw",
        "research/curve25519_quotient_layout_20261125/raw",
    );
    let reproduction = arg(
        &args,
        "--reproduction",
        "research/curve25519_quotient_layout_20261125/reproduction",
    );
    let selftest_path = arg(
        &args,
        "--selftest",
        "research/curve25519_quotient_layout_20261125/preflight/selftest.json",
    );
    let out = arg(
        &args,
        "--out",
        "research/curve25519_quotient_layout_20261125/evidence",
    );

    let rows = read_rows(&raw)?;
    if rows.len() != 60 {
        return Err(format!(
            "expected 60 qualification rows, found {}",
            rows.len()
        ));
    }
    if rows.iter().any(|r| {
        r.schema != "curve25519.quotient-layout-row/v1"
            || r.d != 19
            || !ARMS.contains(&r.arm.as_str())
    }) {
        return Err("unexpected qualification schema, order, or arm".into());
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
    let selftest_pass = selftest["schema"] == "curve25519.quotient-layout-selftest/v1"
        && selftest["status"] == "pass"
        && selftest["all_arm_truth_agreement"] == true
        && selftest["layout_ideal_and_assignment_identity"] == true;

    let reproduction_rows = read_rows(&reproduction)?;
    if reproduction_rows.len() != 12 {
        return Err(format!(
            "expected 12 trailing-layout reproduction rows, found {}",
            reproduction_rows.len()
        ));
    }

    let mut target_pairs_exact = true;
    let mut layout_identity_mismatches = 0usize;
    let mut t_reproduction_mismatches = 0usize;
    let mut planted_bad = 0usize;
    let mut paired_csv = String::from(
        "kind,index,arm,field_ops,ratio_to_D,ratio_to_T,solving_degree_max,peak_rss_kib,rss_ratio_to_D,membership_build_coeff_ops,build_ratio_to_D,censored\n",
    );

    for kind in ["random", "planted"] {
        let count = if kind == "random" { 8 } else { 4 };
        for index in 0..count {
            let direct = row(&rows, kind, index, "D")?;
            let trailing = row(&rows, kind, index, "T")?;
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
                if ["T", "F", "E", "I"].contains(&arm)
                    && (current.target_scalar != trailing.target_scalar
                        || current.target_x != trailing.target_x
                        || current.target_y != trailing.target_y
                        || current.input_terms != trailing.input_terms
                        || current.input_degree != trailing.input_degree
                        || current.n_vars != trailing.n_vars
                        || current.n_equations != trailing.n_equations
                        || current.membership_build_coeff_ops
                            != trailing.membership_build_coeff_ops
                        || current.inconsistent != trailing.inconsistent
                        || current.censored != trailing.censored
                        || current.timed_out != trailing.timed_out)
                {
                    layout_identity_mismatches += 1;
                }
                let field_ratio_d = current.field_ops as f64 / direct.field_ops as f64;
                let field_ratio_t = current.field_ops as f64 / trailing.field_ops as f64;
                let rss_ratio = match (current.peak_rss_kib, direct.peak_rss_kib) {
                    (Some(candidate), Some(base)) => candidate as f64 / base as f64,
                    _ => f64::NAN,
                };
                let build_ratio = current.membership_build_coeff_ops as f64
                    / direct.membership_build_coeff_ops as f64;
                writeln!(
                    paired_csv,
                    "{kind},{index},{arm},{},{},{},{},{},{},{},{},{}",
                    current.field_ops,
                    fmt_ratio(field_ratio_d),
                    fmt_ratio(field_ratio_t),
                    current.solving_degree_max,
                    current.peak_rss_kib.unwrap_or(0),
                    fmt_ratio(rss_ratio),
                    current.membership_build_coeff_ops,
                    fmt_ratio(build_ratio),
                    current.censored
                )
                .unwrap();
            }

            let old_b = reproduction_rows
                .iter()
                .find(|r| r.d == 19 && r.kind == kind && r.index == index && r.arm == "B")
                .ok_or_else(|| format!("missing B reproduction {kind} i={index}"))?;
            t_reproduction_mismatches += (!reproduction_equal(trailing, old_b)) as usize;
        }
    }

    let mut cells = BTreeMap::new();
    for arm in ARMS {
        let selected: Vec<&Row> = rows
            .iter()
            .filter(|r| r.kind == "random" && r.arm == arm)
            .collect();
        let field_values: Vec<f64> = selected.iter().map(|r| r.field_ops as f64).collect();
        let ratios_d: Vec<f64> = selected
            .iter()
            .map(|candidate| {
                candidate.field_ops as f64
                    / row(&rows, "random", candidate.index, "D")
                        .unwrap()
                        .field_ops as f64
            })
            .collect();
        let ratios_t: Vec<f64> = selected
            .iter()
            .map(|candidate| {
                candidate.field_ops as f64
                    / row(&rows, "random", candidate.index, "T")
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
                "paired_ratio_to_D_geometric_mean": geometric_mean(&ratios_d),
                "paired_ratio_to_T_geometric_mean": geometric_mean(&ratios_t),
                "solving_degree_max_values": selected.iter().map(|r| r.solving_degree_max).collect::<BTreeSet<_>>(),
                "peak_rss_kib_max": selected.iter().filter_map(|r| r.peak_rss_kib).max(),
                "membership_build_coeff_ops_max": selected.iter().map(|r| r.membership_build_coeff_ops).max()
            }),
        );
    }

    let completed_triples: Vec<(&Row, &Row, &Row)> = (0..8)
        .filter_map(|index| {
            let d = row(&rows, "random", index, "D").ok()?;
            let t = row(&rows, "random", index, "T").ok()?;
            let f = row(&rows, "random", index, "F").ok()?;
            (!d.censored && !t.censored && !f.censored).then_some((d, t, f))
        })
        .collect();
    if completed_triples.is_empty() {
        return Err("no completed D/T/F random triples".into());
    }
    let ratios_fd: Vec<f64> = completed_triples
        .iter()
        .map(|(d, _, f)| f.field_ops as f64 / d.field_ops as f64)
        .collect();
    let ratios_ft: Vec<f64> = completed_triples
        .iter()
        .map(|(_, t, f)| f.field_ops as f64 / t.field_ops as f64)
        .collect();
    let (fd_lo, fd_hi) = paired_interval(&ratios_fd, SEED, 10_000);
    let (ft_lo, ft_hi) = paired_interval(&ratios_ft, SEED ^ 0x4654, 10_000);
    let degree_at_most_four = completed_triples
        .iter()
        .filter(|(_, _, f)| f.solving_degree_max <= 4)
        .count();
    let max_rss_ratio = completed_triples
        .iter()
        .map(|(d, _, f)| f.peak_rss_kib.unwrap() as f64 / d.peak_rss_kib.unwrap() as f64)
        .fold(0.0, f64::max);
    let max_build_ratio = completed_triples
        .iter()
        .map(|(d, _, f)| f.membership_build_coeff_ops as f64 / d.membership_build_coeff_ops as f64)
        .fold(0.0, f64::max);
    let permutation_build_overhead_zero = completed_triples
        .iter()
        .all(|(_, t, f)| f.membership_build_coeff_ops == t.membership_build_coeff_ops);
    let random_censored = |arm: &str| {
        (0..8)
            .filter(|&index| row(&rows, "random", index, arm).unwrap().censored)
            .count()
    };
    let d_censored = random_censored("D");
    let t_censored = random_censored("T");
    let f_censored = random_censored("F");

    let gates = serde_json::json!({
        "selftest": selftest_pass,
        "planted_correctness": planted_bad == 0,
        "target_pairs_exact": target_pairs_exact,
        "layout_counts_and_verdicts_identical": layout_identity_mismatches == 0,
        "T_canonical_B_reproduction": t_reproduction_mismatches == 0,
        "F_censoring_not_greater": f_censored <= d_censored && f_censored <= t_censored,
        "minimum_six_triples": completed_triples.len() >= 6,
        "F_over_D_upper_lt_0_75": fd_hi < 0.75,
        "F_over_T_upper_lt_0_60": ft_hi < 0.60,
        "degree_at_most_four_on_six_pairs": degree_at_most_four >= 6,
        "peak_rss_F_over_D_each_le_1_25": max_rss_ratio <= 1.25,
        "permutation_build_overhead_zero": permutation_build_overhead_zero,
        "build_F_over_D_each_le_1_25": max_build_ratio <= 1.25
    });
    let qualifies = gates
        .as_object()
        .unwrap()
        .values()
        .all(|value| value == true);
    let analysis = serde_json::json!({
        "schema": "curve25519.quotient-layout-analysis/v1",
        "study_id": "EXP-C25519-QLAYOUT-001",
        "rows": rows.len(),
        "planned": 60,
        "attempted": 60,
        "completed": rows.iter().filter(|r| !r.censored).count(),
        "censored": rows.iter().filter(|r| r.censored).count(),
        "planted_bad": planted_bad,
        "target_pairs_exact": target_pairs_exact,
        "layout_identity_mismatches": layout_identity_mismatches,
        "T_reproduction_mismatches": t_reproduction_mismatches,
        "cells": cells,
        "qualification_d19": {
            "completed_random_triples": completed_triples.len(),
            "paired_F_over_D_geometric_mean": geometric_mean(&ratios_fd),
            "paired_F_over_D_descriptive_95": [fd_lo, fd_hi],
            "paired_F_over_T_geometric_mean": geometric_mean(&ratios_ft),
            "paired_F_over_T_descriptive_95": [ft_lo, ft_hi],
            "F_degree_at_most_four_pairs": degree_at_most_four,
            "D_random_censored": d_censored,
            "T_random_censored": t_censored,
            "F_random_censored": f_censored,
            "max_F_over_D_membership_build_coeff_ops_ratio": max_build_ratio,
            "max_F_over_D_peak_rss_ratio": max_rss_ratio,
            "gates": gates
        },
        "qualifies_for_scaling": qualifies,
        "decision": if qualifies { "continue_to_gated_scaling" } else { "stop_before_scaling_and_m4" },
        "claim_boundary": "grevlex variable-layout stage diagnostic only; no total-work or subexponential ECDLP claim"
    });

    std::fs::create_dir_all(&out).map_err(|e| format!("mkdir {out}: {e}"))?;
    std::fs::write(
        Path::new(&out).join("analysis.json"),
        serde_json::to_string_pretty(&analysis).unwrap() + "\n",
    )
    .map_err(|e| format!("write analysis: {e}"))?;
    std::fs::write(Path::new(&out).join("paired.csv"), paired_csv)
        .map_err(|e| format!("write paired csv: {e}"))?;

    let plotted = ["T", "F", "E", "I"];
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
    let x = |index: usize| 180.0 + index as f64 * 185.0;
    let y = |ratio: f64| 430.0 - ratio / 1.6 * 325.0;
    let mut marks = String::new();
    for (index, &(arm, ratio)) in ratios_by_arm.iter().enumerate() {
        let color = if arm == "F" { "#d1495b" } else { "#457b9d" };
        writeln!(marks, "<line x1=\"{x:.1}\" y1=\"{base:.1}\" x2=\"{x:.1}\" y2=\"{top:.1}\" stroke=\"{color}\" stroke-width=\"40\"/><text x=\"{x:.1}\" y=\"{label:.1}\" text-anchor=\"middle\" font-size=\"15\">{ratio:.3}</text><text x=\"{x:.1}\" y=\"465\" text-anchor=\"middle\" font-size=\"16\">{arm}</text>", x=x(index), base=y(0.0), top=y(ratio), label=y(ratio)-10.0).unwrap();
    }
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="900" height="520" viewBox="0 0 900 520">
<rect width="900" height="520" fill="#ffffff"/>
<text x="450" y="34" text-anchor="middle" font-family="sans-serif" font-size="22" font-weight="bold">Quotient variable layout / direct F4 work</text>
<text x="450" y="59" text-anchor="middle" font-family="sans-serif" font-size="14">paired geometric mean, 8 random d=19 targets; lower is better</text>
<line x1="90" y1="430" x2="835" y2="430" stroke="#333"/>
<line x1="90" y1="85" x2="90" y2="430" stroke="#333"/>
<line x1="90" y1="{threshold:.1}" x2="835" y2="{threshold:.1}" stroke="#2a9d8f" stroke-width="2" stroke-dasharray="8 6"/>
<text x="830" y="102" text-anchor="end" font-family="sans-serif" font-size="14" fill="#16766c">primary F ceiling 0.75</text>
{marks}
<text x="450" y="500" text-anchor="middle" font-family="sans-serif" font-size="16">grevlex layout</text>
<text x="25" y="260" transform="rotate(-90 25 260)" text-anchor="middle" font-family="sans-serif" font-size="16">layout / D field_ops</text>
<text x="110" y="101" font-family="sans-serif" font-size="13" fill="#555">[pcc:unverified] stage diagnostic</text>
</svg>
"##,
        threshold = y(0.75),
    );
    std::fs::write(Path::new(&out).join("figure.svg"), svg)
        .map_err(|e| format!("write svg: {e}"))?;
    let export = serde_json::json!({
        "schema": "experiment.export/v1",
        "experiment_id": "EXP-C25519-QLAYOUT-001",
        "status": if qualifies { "qualified" } else { "completed_negative" },
        "flow": {"planned": 60, "attempted": 60, "completed": rows.iter().filter(|r| !r.censored).count(), "excluded": 0, "censored": rows.iter().filter(|r| r.censored).count()},
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
