//! # Tables for research/isogeny_conductor_gap_round5_20261012 (E6)
//!
//! Reads `e6/records.jsonl` (written by
//! `examples/isogeny_gap_pushforward_yield.rs`).  For every curve and base
//! size, it divides each quotient arm's decompositions by the direct arm's,
//! over the same targets.  The 95 % interval comes from the log ratio of
//! two Poisson counts (`se = √(1/a + 1/b)`).
//!
//! It also pools by subgroup order: the sum of quotient decompositions over
//! the sum of direct ones, across every curve and base size where the arm
//! exists.  The F4 rows split by whether the arm's F4 target had a
//! solution.
//!
//! ```bash
//! cargo run --release --example isogeny_gap_round5_tables -- \
//!   --dir research/isogeny_conductor_gap_round5_20261012 \
//!   --out research/isogeny_conductor_gap_round5_20261012/TABLES.md \
//!   --json research/isogeny_conductor_gap_round5_20261012/tables.json
//! ```

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde_json::{json, Value};

fn arg(args: &[String], name: &str) -> String {
    let i = args
        .iter()
        .position(|a| a == name)
        .unwrap_or_else(|| panic!("missing {name}"));
    args[i + 1].clone()
}

fn ratio_ci(a: f64, b: f64) -> (f64, f64, f64) {
    if a == 0.0 || b == 0.0 {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let r = a / b;
    let se = (1.0 / a + 1.0 / b).sqrt();
    (r, r * (-1.96 * se).exp(), r * (1.96 * se).exp())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = arg(&args, "--dir");
    let recs: Vec<Value> = std::fs::read_to_string(format!("{dir}/e6/records.jsonl"))
        .expect("records")
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let lift_failures: u64 = recs
        .iter()
        .map(|r| r["lift_failures"].as_u64().unwrap())
        .sum();
    let mut md = String::new();
    writeln!(
        md,
        "## E6. Pushed-forward base against the direct base, matched columns\n\n{} arm records; {} decompositions failed to lift back to E.\n",
        recs.len(),
        lift_failures
    )
    .unwrap();
    // Direct counts by (pair, role, c).
    let key = |r: &Value| {
        (
            r["pair"].as_str().unwrap().to_string(),
            r["role"].as_str().unwrap().to_string(),
            r["base_size"].as_u64().unwrap(),
        )
    };
    let direct: BTreeMap<_, f64> = recs
        .iter()
        .filter(|r| r["arm"] == "direct")
        .map(|r| (key(r), r["decompositions"].as_f64().unwrap()))
        .collect();
    writeln!(md, "### Per curve\n\n| pair | role | c | arm | \\|H\\| | columns | decompositions | ÷ direct | 95 % interval |\n|:--|:--|--:|:--|--:|--:|--:|--:|:--|").unwrap();
    let mut pooled: BTreeMap<(String, u64), (f64, f64, u64)> = BTreeMap::new();
    let mut rows = Vec::new();
    for r in &recs {
        let k = key(r);
        let dec = r["decompositions"].as_f64().unwrap();
        let h = r["subgroup_order"].as_u64().unwrap();
        let arm = r["arm"].as_str().unwrap();
        let d = direct[&k];
        let (ratio, lo, hi) = if arm == "direct" {
            (1.0, 1.0, 1.0)
        } else {
            ratio_ci(dec, d)
        };
        writeln!(
            md,
            "| {} | {} | {} | {arm} | {h} | {} | {dec:.0} | {} | {} |",
            k.0,
            k.1,
            k.2,
            r["columns"],
            if arm == "direct" {
                "–".into()
            } else {
                format!("{ratio:.2}")
            },
            if arm == "direct" {
                "–".into()
            } else {
                format!("{lo:.2} to {hi:.2}")
            },
        )
        .unwrap();
        if arm != "direct" {
            // Pool by arm family (C2, C4, E[2], E[h]) and subgroup order.
            let fam = arm.trim_start_matches("quotient ").to_string();
            let e = pooled.entry((fam, h)).or_insert((0.0, 0.0, 0));
            e.0 += dec;
            e.1 += d;
            e.2 += 1;
        }
        rows.push(
            json!({"pair": k.0, "role": k.1, "base_size": k.2, "arm": arm, "subgroup_order": h,
            "columns": r["columns"], "decompositions": dec, "direct_decompositions": d,
            "ratio": if arm == "direct" { Value::Null } else { json!(ratio) },
            "ratio_low": if arm == "direct" { Value::Null } else { json!(lo) },
            "ratio_high": if arm == "direct" { Value::Null } else { json!(hi) },
            "targets_hit": r["targets_hit"], "targets": r["targets"], "f4": r["f4"]}),
        );
    }
    writeln!(md, "\n### Pooled by subgroup\n\n| arm | \\|H\\| | curves × sizes | Σ quotient ÷ Σ direct | 95 % interval | contains \\|H\\| |\n|:--|--:|--:|--:|:--|:--|").unwrap();
    let mut pooled_json = Vec::new();
    for ((fam, h), (q, d, n)) in &pooled {
        let (r, lo, hi) = ratio_ci(*q, *d);
        let contains = lo <= *h as f64 && *h as f64 <= hi;
        writeln!(
            md,
            "| {fam} | {h} | {n} | {r:.2} | {lo:.2} to {hi:.2} | {} |",
            if contains { "yes" } else { "no" }
        )
        .unwrap();
        pooled_json.push(
            json!({"arm": fam, "subgroup_order": h, "cases": n, "quotient": q, "direct": d,
            "ratio": r, "low": lo, "high": hi, "contains_h": contains}),
        );
    }
    // Exponent: least squares of ln(ratio) on ln|H| over the pooled rows,
    // weighted by 1/se² of each pooled log ratio.
    let pts: Vec<(f64, f64, f64)> = pooled
        .iter()
        .map(|((_, h), (q, d, _))| ((*h as f64).ln(), (q / d).ln(), 1.0 / (1.0 / q + 1.0 / d)))
        .collect();
    let sw: f64 = pts.iter().map(|p| p.2).sum();
    let mx = pts.iter().map(|p| p.2 * p.0).sum::<f64>() / sw;
    let my = pts.iter().map(|p| p.2 * p.1).sum::<f64>() / sw;
    let sxx: f64 = pts.iter().map(|p| p.2 * (p.0 - mx).powi(2)).sum();
    let sxy: f64 = pts.iter().map(|p| p.2 * (p.0 - mx) * (p.1 - my)).sum();
    let slope = sxy / sxx;
    let intercept = my - slope * mx;
    let slope_se = (1.0 / sxx).sqrt();
    writeln!(
        md,
        "\nWeighted least squares of ln(ratio) on ln|H| over the {} pooled rows: exponent **{slope:.3}** (± {:.3} at 95 %, from the Poisson weights alone), ratio at |H| = 1 fitted as {:.2}.",
        pts.len(),
        1.96 * slope_se,
        intercept.exp()
    )
    .unwrap();
    // F4: identical cost on unsolvable targets?
    writeln!(md, "\n### F4 on one random target per arm\n\n| c | target solvable | arms | solving degree | widest matrix | multiplications |\n|--:|:--|--:|:--|:--|:--|").unwrap();
    let mut f4g: BTreeMap<(u64, bool), Vec<(u64, u64, u64)>> = BTreeMap::new();
    for r in &recs {
        let f = &r["f4"];
        f4g.entry((r["base_size"].as_u64().unwrap(), f["inconsistent"] == false))
            .or_default()
            .push((
                f["solving_degree_max"].as_u64().unwrap(),
                f["max_cols_to_solution"].as_u64().unwrap(),
                f["field_ops"].as_u64().unwrap(),
            ));
    }
    let span = |v: &[u64]| {
        let (a, b) = (v.iter().min().unwrap(), v.iter().max().unwrap());
        if a == b {
            a.to_string()
        } else {
            format!("{a}–{b}")
        }
    };
    for ((c, solvable), v) in &f4g {
        let degs: Vec<u64> = v.iter().map(|x| x.0).collect();
        let cols: Vec<u64> = v.iter().map(|x| x.1).collect();
        let ops: Vec<u64> = v.iter().map(|x| x.2).collect();
        writeln!(
            md,
            "| {c} | {} | {} | {} | {} | {} |",
            if *solvable { "yes" } else { "no" },
            v.len(),
            span(&degs),
            span(&cols),
            span(&ops)
        )
        .unwrap();
    }
    std::fs::write(arg(&args, "--out"), &md).unwrap();
    std::fs::write(
        arg(&args, "--json"),
        serde_json::to_string_pretty(
            &json!({"rows": rows, "pooled": pooled_json, "lift_failures": lift_failures, "exponent_fit": {"slope": slope, "slope_half_width_95": 1.96 * slope_se, "ratio_at_h1": intercept.exp()}}),
        )
        .unwrap()
            + "\n",
    )
    .unwrap();
    print!("{md}");
}
