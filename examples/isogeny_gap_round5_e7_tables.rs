//! # E7 tables for research/isogeny_conductor_gap_round5_20261012
//!
//! **E7a** (`e7a_h72`, `e7a_h4`): per curve, the cheapest fiber-closed arm
//! (`closed-b*`) and the cheapest cofactor-cleared arm (`subgroup-b*`) by
//! mean `S`, their ratio, and each against rho.  It then gives:
//! - per cofactor, the geometric mean of subgroup ÷ closed against the
//!   prediction `h^{−1/3}`;
//! - the least-squares slope of `log₂(subgroup ÷ rho)` on `log₂ #E`, with
//!   a Student-t interval.
//!
//! **E7b** (`e7b_k0`): per curve, the default and trace-kernel arms of each
//! oracle, with:
//! - relations per trial (yield);
//! - F4 solver operations per relation, and the maximum solving degree;
//! - mean `S`;
//! - trace-kernel ÷ default for yield (95 % interval from the log ratio of
//!   two proportions) and for solver work per relation.
//!
//! Only measured (non-warm-up), verified runs count.
//!
//! ```bash
//! cargo run --release --example isogeny_gap_round5_e7_tables -- \
//!   --dir research/isogeny_conductor_gap_round5_20261012 \
//!   --out research/isogeny_conductor_gap_round5_20261012/TABLES_E7.md \
//!   --json research/isogeny_conductor_gap_round5_20261012/tables_e7.json
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

fn t975(df: usize) -> f64 {
    const T: [f64; 12] = [
        12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201, 2.179,
    ];
    if df == 0 {
        f64::NAN
    } else if df <= 12 {
        T[df - 1]
    } else {
        2.0
    }
}

#[derive(Default, Clone)]
struct Arm {
    runs: u64,
    verified: u64,
    s_sum: f64,
    relations: f64,
    trials: f64,
    solver_ops: f64,
    deg_max: u64,
    wall_s: f64,
}

impl Arm {
    fn s(&self) -> Option<f64> {
        (self.verified > 0 && self.verified == self.runs).then(|| self.s_sum / self.verified as f64)
    }
    fn json(&self) -> Value {
        json!({"runs": self.runs, "verified": self.verified, "s_mean": self.s(),
               "relations": self.relations, "trials": self.trials,
               "yield": if self.trials > 0.0 { Some(self.relations / self.trials) } else { None },
               "solver_ops": self.solver_ops,
               "solver_ops_per_relation": if self.relations > 0.0 && self.solver_ops > 0.0 { Some(self.solver_ops / self.relations) } else { None },
               "solving_degree_max": self.deg_max, "online_wall_s_mean": if self.verified > 0 { self.wall_s / self.verified as f64 } else { 0.0 }})
    }
}

/// curve slug → (log₂ #E, field bits, cofactor, arm → aggregate)
type Curves = BTreeMap<String, (f64, u64, u64, BTreeMap<String, Arm>)>;

fn load(dir: &str, name: &str) -> Option<Curves> {
    let text = std::fs::read_to_string(format!("{dir}/sessions/{name}/records.jsonl")).ok()?;
    let mut out: Curves = BTreeMap::new();
    for line in text.lines() {
        let r: Value = serde_json::from_str(line).unwrap();
        if r["warmup"] == true {
            continue;
        }
        let c = &r["workload"]["curve"];
        let n = c["group_order"].as_u64().unwrap_or(0) as f64;
        let e = out
            .entry(c["slug"].as_str().unwrap().to_string())
            .or_insert_with(|| {
                (
                    n.log2(),
                    c["field_bits"].as_u64().unwrap_or(0),
                    c["cofactor"].as_u64().unwrap_or(0),
                    BTreeMap::new(),
                )
            });
        let a =
            e.3.entry(r["arm"].as_str().unwrap().to_string())
                .or_default();
        a.runs += 1;
        if r["outcome"]["status"] != "verified" {
            continue;
        }
        a.verified += 1;
        a.s_sum += r["cost"]["s"].as_f64().unwrap_or(0.0);
        a.wall_s += r["online"]["wall_ns"].as_f64().unwrap_or(0.0) / 1e9;
        for p in r["phases"].as_array().into_iter().flatten() {
            if p["name"] == "relations" {
                a.relations += p["native"]["relations"].as_f64().unwrap_or(0.0);
                a.trials += p["native"]["trials"].as_f64().unwrap_or(0.0);
            }
        }
        if r["solver"].is_object() {
            a.solver_ops += r["solver"]["ops"].as_f64().unwrap_or(0.0);
            a.deg_max = a
                .deg_max
                .max(r["solver"]["solving_degree_max"].as_u64().unwrap_or(0));
        }
    }
    Some(out)
}

fn best<'a>(arms: &'a BTreeMap<String, Arm>, prefix: &str) -> Option<(f64, &'a str)> {
    arms.iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .filter_map(|(k, a)| a.s().map(|s| (s, k.as_str())))
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = arg(&args, "--dir");
    let mut md = String::new();
    let mut js = serde_json::Map::new();

    // ── E7a ──
    writeln!(
        md,
        "## E7a. Cofactor-cleared base against the fiber-closed table\n"
    )
    .unwrap();
    for name in ["e7a_h4", "e7a_h72"] {
        let Some(curves) = load(&dir, name) else {
            writeln!(md, "`{name}`: no session.\n").unwrap();
            continue;
        };
        writeln!(md, "### `{name}`\n\n| curve | bits | h | log₂ #E | rho S | best closed (arm : S) | best subgroup (arm : S) | subgroup ÷ closed | subgroup ÷ rho | closed ÷ rho |\n|:--|--:|--:|--:|--:|:--|:--|--:|--:|--:|").unwrap();
        let (mut xs, mut ys, mut lr) = (Vec::new(), Vec::new(), Vec::new());
        let mut rows = Vec::new();
        let mut h_seen = 0;
        for (slug, (lg, bits, h, arms)) in &curves {
            h_seen = *h;
            let rho = arms.get("rho-neg").and_then(|a| a.s());
            let (c, s) = (best(arms, "closed-"), best(arms, "subgroup-"));
            let cell =
                |b: Option<(f64, &str)>| b.map_or("–".into(), |(v, k)| format!("{k} : {v:.1}"));
            let (sc, sr, cr) = match (c, s, rho) {
                (Some(c), Some(s), Some(r)) => (s.0 / c.0, s.0 / r, c.0 / r),
                _ => (f64::NAN, f64::NAN, f64::NAN),
            };
            if sr.is_finite() {
                xs.push(*lg);
                ys.push(sr.log2());
                lr.push(sc.ln());
            }
            writeln!(
                md,
                "| `{slug}` | {bits} | {h} | {lg:.2} | {} | {} | {} | {sc:.3} | {sr:.2} | {cr:.2} |",
                rho.map_or("–".into(), |r| format!("{r:.2}")),
                cell(c),
                cell(s)
            )
            .unwrap();
            rows.push(json!({"slug": slug, "bits": bits, "cofactor": h, "log2_order": lg, "rho_s": rho,
                "best_closed": c.map(|(v, k)| json!({"arm": k, "s": v})),
                "best_subgroup": s.map(|(v, k)| json!({"arm": k, "s": v})),
                "subgroup_over_closed": sc, "subgroup_over_rho": sr, "closed_over_rho": cr,
                "arms": arms.iter().map(|(k, a)| (k.clone(), a.json())).collect::<BTreeMap<_, _>>()}));
        }
        let n = xs.len();
        let gm = if lr.is_empty() {
            f64::NAN
        } else {
            (lr.iter().sum::<f64>() / lr.len() as f64).exp()
        };
        let pred = (h_seen as f64).powf(-1.0 / 3.0);
        let fit = if n >= 3 {
            let mx = xs.iter().sum::<f64>() / n as f64;
            let my = ys.iter().sum::<f64>() / n as f64;
            let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
            let b = xs
                .iter()
                .zip(&ys)
                .map(|(x, y)| (x - mx) * (y - my))
                .sum::<f64>()
                / sxx;
            let a = my - b * mx;
            let rss: f64 = xs
                .iter()
                .zip(&ys)
                .map(|(x, y)| (y - a - b * x).powi(2))
                .sum();
            let hw = t975(n - 2) * (rss / (n as f64 - 2.0) / sxx).sqrt();
            Some((b, hw, a))
        } else {
            None
        };
        writeln!(
            md,
            "\nGeometric mean of subgroup ÷ closed over {n} curves: **{gm:.3}** (prediction h^(−1/3) = {pred:.3}; within a factor 1.5: {}).{}\n",
            if gm.is_finite() && gm / pred <= 1.5 && pred / gm <= 1.5 { "yes" } else { "no" },
            fit.map_or(String::new(), |(b, hw, _)| format!(" Slope of log₂(subgroup ÷ rho) on log₂ #E: **{b:.3}** (95 % interval {:.3} to {:.3}; prediction 1/6).", b - hw, b + hw))
        )
        .unwrap();
        js.insert(name.into(), json!({"cofactor": h_seen, "curves": rows, "geomean_subgroup_over_closed": gm,
            "prediction": pred, "slope": fit.map(|(b, hw, a)| json!({"slope": b, "low": b - hw, "high": b + hw, "intercept": a}))}));
    }

    // ── E7b ──
    writeln!(
        md,
        "## E7b. Trace-kernel subspace base against the default\n"
    )
    .unwrap();
    if let Some(curves) = load(&dir, "e7b_k0") {
        writeln!(md, "| curve | arm | runs (verified) | relations / trials | yield | solver ops per relation | max degree | S | online wall (s) |\n|:--|:--|:--|:--|--:|--:|--:|--:|--:|").unwrap();
        let mut comps = Vec::new();
        for (slug, (_, _, _, arms)) in &curves {
            for (k, a) in arms {
                writeln!(
                    md,
                    "| `{slug}` | {k} | {} ({}) | {:.0} / {:.0} | {} | {} | {} | {} | {:.2} |",
                    a.runs,
                    a.verified,
                    a.relations,
                    a.trials,
                    if a.trials > 0.0 {
                        format!("{:.4}", a.relations / a.trials)
                    } else {
                        "–".into()
                    },
                    if a.solver_ops > 0.0 && a.relations > 0.0 {
                        format!("{:.0}", a.solver_ops / a.relations)
                    } else {
                        "–".into()
                    },
                    if a.deg_max > 0 {
                        a.deg_max.to_string()
                    } else {
                        "–".into()
                    },
                    a.s().map_or("–".into(), |s| format!("{s:.2}")),
                    if a.verified > 0 {
                        a.wall_s / a.verified as f64
                    } else {
                        0.0
                    }
                )
                .unwrap();
            }
            for fam in ["f4", "mitm"] {
                let (Some(d), Some(t)) = (
                    arms.get(&format!("{fam}-blind")),
                    arms.get(&format!("{fam}-tracekernel")),
                ) else {
                    continue;
                };
                if d.relations == 0.0 || t.relations == 0.0 {
                    continue;
                }
                let yr = (t.relations / t.trials) / (d.relations / d.trials);
                let se = (1.0 / t.relations - 1.0 / t.trials + 1.0 / d.relations - 1.0 / d.trials)
                    .max(0.0)
                    .sqrt();
                let ops = if d.solver_ops > 0.0 && t.solver_ops > 0.0 {
                    Some((d.solver_ops / d.relations) / (t.solver_ops / t.relations))
                } else {
                    None
                };
                let s_ratio = match (d.s(), t.s()) {
                    (Some(a), Some(b)) => Some(a / b),
                    _ => None,
                };
                comps.push(json!({"slug": slug, "oracle": fam, "yield_ratio": yr,
                    "yield_ratio_low": yr * (-1.96 * se).exp(), "yield_ratio_high": yr * (1.96 * se).exp(),
                    "solver_ops_per_relation_default_over_trace_kernel": ops, "s_default_over_trace_kernel": s_ratio,
                    "max_degree_default": d.deg_max, "max_degree_trace_kernel": t.deg_max}));
            }
        }
        writeln!(md, "\n| curve | oracle | yield, trace-kernel ÷ default | 95 % interval | solver ops per relation, default ÷ trace-kernel | S, default ÷ trace-kernel | max degree (default, trace-kernel) |\n|:--|:--|--:|:--|--:|--:|:--|").unwrap();
        for c in &comps {
            writeln!(
                md,
                "| `{}` | {} | {:.2} | {:.2} to {:.2} | {} | {} | {}, {} |",
                c["slug"].as_str().unwrap(),
                c["oracle"].as_str().unwrap(),
                c["yield_ratio"].as_f64().unwrap(),
                c["yield_ratio_low"].as_f64().unwrap(),
                c["yield_ratio_high"].as_f64().unwrap(),
                c["solver_ops_per_relation_default_over_trace_kernel"]
                    .as_f64()
                    .map_or("–".into(), |v| format!("{v:.2}")),
                c["s_default_over_trace_kernel"]
                    .as_f64()
                    .map_or("–".into(), |v| format!("{v:.2}")),
                c["max_degree_default"],
                c["max_degree_trace_kernel"]
            )
            .unwrap();
        }
        js.insert(
            "e7b_k0".into(),
            json!({"comparisons": comps, "curves": curves.iter().map(|(k, c)| (k.clone(), c.3.iter().map(|(a, x)| (a.clone(), x.json())).collect::<BTreeMap<_, _>>())).collect::<BTreeMap<_, _>>()}),
        );
    } else {
        writeln!(md, "`e7b_k0`: no session.").unwrap();
    }
    // ── §5a band ──
    // From `ecbench band` (`sessions/<name>.band.json`): per curve and
    // weight, the cheapest closed and cleared arms and rho, and whether the
    // cleared base stays cheaper than the closed table at every weight.
    writeln!(md, "\n## E7a under the §5a pricing band\n").unwrap();
    let mut band_json = serde_json::Map::new();
    for name in ["e7a_h4", "e7a_h72"] {
        let Ok(text) = std::fs::read_to_string(format!("{dir}/sessions/{name}.band.json")) else {
            continue;
        };
        let b: Value = serde_json::from_str(&text).unwrap();
        let weights: Vec<f64> = b["weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w.as_f64().unwrap())
            .collect();
        let mut by_curve: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
        for r in b["rows"].as_array().unwrap() {
            by_curve
                .entry(r["curve"].as_str().unwrap().to_string())
                .or_default()
                .push(r);
        }
        writeln!(md, "### `{name}`\n\n| curve | weight | best closed | best cleared | rho | cleared ÷ closed | cleared ÷ rho |\n|:--|--:|--:|--:|--:|--:|--:|").unwrap();
        let mut holds = true;
        let mut rows = Vec::new();
        for (curve, rs) in &by_curve {
            for (wi, w) in weights.iter().enumerate() {
                let pick = |prefix: &str| {
                    rs.iter()
                        .filter(|r| {
                            r["arm"].as_str().unwrap().starts_with(prefix)
                                && r["verified"] == r["runs"]
                        })
                        .map(|r| r["mean_s"][wi].as_f64().unwrap())
                        .fold(f64::INFINITY, f64::min)
                };
                let (c, g, rho) = (pick("closed-"), pick("subgroup-"), pick("rho-neg"));
                holds &= g < c;
                writeln!(
                    md,
                    "| `{curve}` | {w} | {c:.2} | {g:.2} | {rho:.3} | {:.3} | {:.2} |",
                    g / c,
                    g / rho
                )
                .unwrap();
                rows.push(json!({"curve": curve, "weight": w, "best_closed": c, "best_cleared": g, "rho": rho,
                    "cleared_over_closed": g / c, "cleared_over_rho": g / rho}));
            }
        }
        writeln!(md, "\nCleared base cheaper than the closed table on every curve at every weight: **{}**.\n", if holds { "yes" } else { "no" }).unwrap();
        band_json.insert(name.into(), json!({"weights": weights, "rows": rows, "cleared_beats_closed_at_every_weight": holds}));
    }
    js.insert("band".into(), Value::Object(band_json));
    std::fs::write(arg(&args, "--out"), &md).unwrap();
    std::fs::write(
        arg(&args, "--json"),
        serde_json::to_string_pretty(&Value::Object(js)).unwrap() + "\n",
    )
    .unwrap();
    print!("{md}");
}
