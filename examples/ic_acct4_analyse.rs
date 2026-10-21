//! Analysis of `research/ic_acct4_inversion_20261021`: does executing the
//! priced inversion and residue tests leave the hyperelliptic study's counts
//! unchanged, and does it close the instruction gap the Callgrind
//! diagnostic saw?
//!
//! ```text
//! cargo run --release --example ic_acct4_analyse
//! ```
//!
//! Run from the repository root.  Reads the study's `raw/` files and the
//! committed `research/hec_bitop_20261020/raw/g{2,3}.jsonl`; writes
//! `analysis/analysis.json` and `analysis/tables.md`.
//!
//! * **Equivalence.** For every re-run row, the per-run `bitop/v1` totals,
//!   the per-phase sums, the recovered key and the verification flags of
//!   both arms are compared with the committed row of the same `p`.
//! * **Instructions per counted gate.** Each arm's marginal figure is the
//!   extra Callgrind instructions of two more runs of that arm over the
//!   extra `bitop/v1` those runs count (see `raw/callgrind.json`).
//! * **Algorithm totals.** Each re-run row's mean total with and without
//!   the checked arithmetic's operand re-validation, and IC/rho for both
//!   (the self-check policy of `docs/units/BITOPS.md`).

use std::fmt::Write as _;
use std::path::Path;

use serde_json::{json, Value};

const STUDY: &str = "research/ic_acct4_inversion_20261021";
const COMMITTED: &str = "research/hec_bitop_20261020/raw";

fn read_lines(path: &Path) -> Vec<Value> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .collect()
}

/// The fields a faithful re-run must reproduce exactly.
fn fingerprint(row: &Value) -> Value {
    let arm = |a: &Value| {
        json!({
            "bitops_per_run": a["bitops_per_run"],
            "phases_summed_over_runs": a["phases_summed_over_runs"],
            "validation_subset_summed_over_runs": a["validation_subset_summed_over_runs"],
            "verified": a["verified"],
            "seeds": a["seeds"],
        })
    };
    json!({
        "p": row["p"], "genus": row["genus"], "n": row["n"], "k": row["k"],
        "factor_base_size": row["factor_base_size"],
        "ic": arm(&row["ic"]), "rho": arm(&row["rho"]),
    })
}

fn num(v: &Value) -> f64 {
    v.as_f64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .unwrap_or_else(|| panic!("not a number: {v}"))
}

fn main() {
    let study = Path::new(STUDY);
    let mut out = String::new();

    // ── equivalence
    let mut eq_rows = Vec::new();
    let mut all_equal = true;
    writeln!(out, "## Equivalence with the committed study\n").unwrap();
    writeln!(out, "| genus | p | IC runs | rho runs | identical |").unwrap();
    writeln!(out, "|--:|--:|--:|--:|:--|").unwrap();
    for g in [2, 3] {
        let committed = read_lines(&Path::new(COMMITTED).join(format!("g{g}.jsonl")));
        for row in read_lines(&study.join(format!("raw/g{g}_rerun.jsonl"))) {
            let old = committed
                .iter()
                .find(|r| r["p"] == row["p"])
                .unwrap_or_else(|| panic!("no committed genus-{g} row for p = {}", row["p"]));
            let same = fingerprint(&row) == fingerprint(old);
            all_equal &= same;
            let runs = |a: &str| row[a]["bitops_per_run"].as_array().map_or(0, Vec::len);
            writeln!(
                out,
                "| {g} | {} | {} | {} | {} |",
                row["p"],
                runs("ic"),
                runs("rho"),
                if same { "yes" } else { "**no**" }
            )
            .unwrap();
            eq_rows.push(json!({"genus": g, "p": row["p"], "identical": same}));
        }
    }

    // ── instructions per counted gate
    let cg: Value = serde_json::from_str(
        &std::fs::read_to_string(study.join("raw/callgrind.json")).expect("raw/callgrind.json"),
    )
    .expect("callgrind.json");
    let runs = cg["runs"].as_array().expect("runs");
    let get = |tag: &str| {
        runs.iter()
            .find(|r| r["tag"] == tag)
            .unwrap_or_else(|| panic!("no run {tag}"))
    };
    let marginal = |base: &Value, more: &Value, arm: &str| {
        (num(&more["ir"]) - num(&base["ir"])) / (num(&more[arm]) - num(&base[arm]))
    };
    let before = &cg["recorded_before"]["ic_over_rho_instr_per_gate"];
    writeln!(
        out,
        "\n## Instructions per counted gate (Callgrind, marginal)\n"
    )
    .unwrap();
    writeln!(
        out,
        "| row | IC | rho | IC/rho after | IC/rho before (recorded) |"
    )
    .unwrap();
    writeln!(out, "|:--|--:|--:|--:|--:|").unwrap();
    let mut ipg = Vec::new();
    for (g, p, key) in [(2, 41, "g2_p41_fbwalk"), (3, 23, "g3_p23_fbwalk")] {
        let (a, b, c) = (
            get(&format!("g{g}a")),
            get(&format!("g{g}b")),
            get(&format!("g{g}c")),
        );
        let ic = marginal(a, b, "ic_bitops");
        let rho = marginal(a, c, "rho_bitops");
        writeln!(
            out,
            "| genus {g}, p = {p} | {ic:.3} | {rho:.3} | {:.3} | {:.2} |",
            ic / rho,
            num(&before[key])
        )
        .unwrap();
        ipg.push(json!({
            "genus": g, "p": p, "ic_instr_per_gate": ic, "rho_instr_per_gate": rho,
            "ic_over_rho": ic / rho, "ic_over_rho_recorded_before": before[key],
        }));
    }

    // ── algorithm totals (self-check policy)
    writeln!(
        out,
        "\n## Mean bitop/v1 per run, with and without operand re-validation\n"
    )
    .unwrap();
    writeln!(
        out,
        "| genus | p | IC total | IC algorithm | rho total | rho algorithm | IC/rho total | IC/rho algorithm |"
    )
    .unwrap();
    writeln!(out, "|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    let mut alg = Vec::new();
    for g in [2, 3] {
        for row in read_lines(&study.join(format!("raw/g{g}_rerun.jsonl"))) {
            let (ic_all, ic_alg) = (
                num(&row["ic"]["bitops_mean"]),
                num(&row["ic"]["algorithm_bitops_mean"]),
            );
            let (rho_all, rho_alg) = (
                num(&row["rho"]["bitops_mean"]),
                num(&row["rho"]["algorithm_bitops_mean"]),
            );
            writeln!(
                out,
                "| {g} | {} | {ic_all:.4e} | {ic_alg:.4e} | {rho_all:.4e} | {rho_alg:.4e} | {:.3} | {:.3} |",
                row["p"],
                ic_all / rho_all,
                ic_alg / rho_alg
            )
            .unwrap();
            alg.push(json!({
                "genus": g, "p": row["p"],
                "ic_total": ic_all, "ic_algorithm": ic_alg,
                "rho_total": rho_all, "rho_algorithm": rho_alg,
                "ic_over_rho_total": ic_all / rho_all,
                "ic_over_rho_algorithm": ic_alg / rho_alg,
            }));
        }
    }

    let analysis = json!({
        "schema": "ic-acct4-analysis/v1",
        "all_rows_identical": all_equal,
        "equivalence": eq_rows,
        "instructions_per_gate": ipg,
        "algorithm_totals": alg,
    });
    std::fs::create_dir_all(study.join("analysis")).unwrap();
    std::fs::write(
        study.join("analysis/analysis.json"),
        serde_json::to_string_pretty(&analysis).unwrap() + "\n",
    )
    .unwrap();
    std::fs::write(study.join("analysis/tables.md"), &out).unwrap();
    print!("{out}");
    assert!(all_equal, "a re-run row differs from the committed study");
}
