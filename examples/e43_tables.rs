//! **E43 tables** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.58–§9.59):
//! controls, Stage A and end to end for FV (`chain-fifo-profile`) and FW
//! (`chain-fifo-tails`) against A (`chain-counted`), F (`chain-fifo`), V
//! (`chain-profile`), W (`chain-tails`), E25 (`macaulay`) and the matched rho
//! (walk (iii)), every arm on the fresh stream `E43E0001`, at `0`, `0.1` and
//! `1` GAE, with σ1 (lead scans priced, every arm; E25 takes A's) and σ4
//! (per-call structural work priced, every arm, E40's Stage 0 counts); the
//! negative control FW′, W′ on `E43E0002`.  Reads recorded JSON only; writes
//! `tables.md`, `summary.json` (the values the report plots) and the
//! verdicts G2.258–G2.274.
//!
//! ```text
//! ./e43_tables experiments/50_compose_fw
//! ```

#![recursion_limit = "256"]

use std::env;
use std::fmt::Write as _;
use std::fs;

use serde_json::{json, Map, Value};

const GAE: f64 = 97.0;
const GS: [f64; 3] = [0.0, 0.1, 1.0];
const GLS: [(u64, u64); 10] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
];
const E24: [(u64, u64); 12] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
    (521, 1),
    (521, 2),
];
const E40: &str = "experiments/47_macaulay_entries";
const WEAK: &str = "equal at 0, ahead at 0.1 and 1";
/// Arms: short name and solver.
const ARMS: [(&str, &str); 7] = [
    ("A", "chain-counted"),
    ("F", "chain-fifo"),
    ("V", "chain-profile"),
    ("W", "chain-tails"),
    ("FV", "chain-fifo-profile"),
    ("FW", "chain-fifo-tails"),
    ("E25", "macaulay"),
];
/// Comparisons `(reference, arm)`, reported as `S_ref / S_arm`.
const PAIRS: [(&str, &str); 17] = [
    ("A", "FV"),
    ("F", "FV"),
    ("V", "FV"),
    ("W", "FV"),
    ("E25", "FV"),
    ("A", "FW"),
    ("F", "FW"),
    ("V", "FW"),
    ("W", "FW"),
    ("E25", "FW"),
    ("FV", "FW"),
    ("A", "F"),
    ("A", "V"),
    ("V", "W"),
    ("A", "W"),
    ("E25", "A"),
    ("E25", "F"),
];

fn load(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .unwrap()
}

fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("field {k}"))
}

fn sci(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let e = v.abs().log10().floor() as i32;
    format!("{:.2}·10^{}", v / 10f64.powi(e), e)
}

fn range(v: &[f64]) -> (f64, f64) {
    v.iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| {
            (lo.min(x), hi.max(x))
        })
}

fn arm_of(recs: &Value, a: Option<&str>) -> Value {
    let arr = recs.as_array().unwrap();
    match a {
        Some(a) => arr
            .iter()
            .find(|r| r["arm"] == a)
            .unwrap_or_else(|| panic!("arm {a}"))
            .clone(),
        None => arr[0].clone(),
    }
}

fn timing(k: &str) -> bool {
    k.ends_with("_ms") || k == "ms" || k.contains("wall")
}

const STORAGE: [&str; 3] = [
    "macaulay_stored_entries",
    "profile_offset_ops",
    "tail_extent_ops",
];

/// Fields a chain arm's record may change against A's (§9.58).
fn allow_vs_a(k: &str) -> bool {
    timing(k)
        || STORAGE.contains(&k)
        || matches!(
            k,
            "solver"
                | "solve_muls"
                | "total_muls"
                | "call"
                | "ratio"
                | "pivot_selection"
                | "pivot_selection_ops"
                | "lead_scan_reads"
        )
}

/// Fields FV and FW may change against F's.
fn allow_vs_f(k: &str) -> bool {
    timing(k) || STORAGE.contains(&k) || matches!(k, "solver" | "lead_scan_reads")
}

/// Fields FW may change against FV's (and W against V's).
fn allow_vs_fv(k: &str) -> bool {
    timing(k)
        || matches!(
            k,
            "solver" | "lead_scan_reads" | "macaulay_stored_entries" | "tail_extent_ops"
        )
}

fn strip(v: &Value, allow: fn(&str) -> bool) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.iter()
                .filter(|(k, _)| !allow(k))
                .map(|(k, x)| (k.clone(), strip(x, allow)))
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.iter().map(|x| strip(x, allow)).collect()),
        x => x.clone(),
    }
}

fn diff_fields(a: &Value, b: &Value, allow: fn(&str) -> bool) -> Vec<String> {
    let (ao, bo) = (a.as_object().unwrap(), b.as_object().unwrap());
    let mut keys: Vec<&String> = ao.keys().chain(bo.keys()).collect();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .filter(|k| {
            !allow(k) && ao.get(*k).map(|x| strip(x, allow)) != bo.get(*k).map(|x| strip(x, allow))
        })
        .cloned()
        .collect()
}

/// The charged cells-and-operations `K` of a record (§9.58).
fn k_of(x: &Value) -> f64 {
    match x.get("macaulay_stored_entries").and_then(Value::as_f64) {
        Some(st) => st + f(x, "profile_offset_ops") + x["tail_extent_ops"].as_f64().unwrap_or(0.0),
        None => f(x, "macaulay_entries") + x["pivot_selection_ops"].as_f64().unwrap_or(0.0),
    }
}

/// `S(g) = total_muls + 97 g (K + extra)`.
fn s_of(x: &Value, extra: f64) -> Vec<f64> {
    GS.iter()
        .map(|g| f(x, "total_muls") + GAE * g * (k_of(x) + extra))
        .collect()
}

/// Order of arm `x` against reference `a` (`ahead`: x cheaper).
fn order(a: &[f64], x: &[f64]) -> &'static str {
    let ahead = (0..3).all(|g| x[g] < a[g]);
    let behind = (0..3).all(|g| x[g] > a[g]);
    let weak = x[0] == a[0] && x[1] < a[1] && x[2] < a[2];
    let weak_behind = x[0] == a[0] && x[1] > a[1] && x[2] > a[2];
    if ahead {
        "ahead"
    } else if weak {
        WEAK
    } else if behind {
        "behind"
    } else if weak_behind {
        "equal at 0, behind at 0.1 and 1"
    } else {
        "unresolved"
    }
}

fn rat(r: &[f64], x: &[f64]) -> Vec<f64> {
    (0..3).map(|g| r[g] / x[g]).collect()
}

fn fmt3(b: &[(f64, f64)]) -> String {
    format!(
        "`{:.4}`–`{:.4}` / `{:.4}`–`{:.4}` / `{:.4}`–`{:.4}`",
        b[0].0, b[0].1, b[1].0, b[1].1, b[2].0, b[2].1
    )
}

fn inside(b: (f64, f64), lo: f64, hi: f64) -> bool {
    b.0 >= lo && b.1 <= hi
}

/// A three-weight band `[(lo, hi); 3]` inside registered bounds (`None`: the
/// value must be exactly 1).
fn inside3(b: &[(f64, f64)], want: [Option<(f64, f64)>; 3]) -> bool {
    (0..3).all(|g| match want[g] {
        Some((lo, hi)) => inside(b[g], lo, hi),
        None => b[g].0 == 1.0 && b[g].1 == 1.0,
    })
}

fn key(r: &str, x: &str) -> String {
    format!("{r}/{x}")
}

fn main() {
    let dir = env::args()
        .nth(1)
        .unwrap_or("experiments/50_compose_fw".into());
    let mut md = String::new();
    let mut verdict = Map::new();
    writeln!(md, "# E43 tables (§9.58, §9.59)\n\nGenerated by `examples/e43_tables.rs` from recorded JSON. `S(g) = total_muls + 97 g · K`; `K` = `macaulay_entries` (+ selection, `0` for F) for A, F and E25, `macaulay_stored_entries + profile_offset_ops` for V and FV, `macaulay_stored_entries + profile_offset_ops + tail_extent_ops` for W and FW; rho(g) = rho + 97 g · steps (walk (iii)). Ratios are `S_ref / S_arm` (above 1: the arm is cheaper).\n").unwrap();

    // σ4: E40's per-call structural work (common to every arm).
    let e40s0 = load(&format!("{E40}/stage0.json"));
    let struct_of = |set: &str, p: u64, seed: u64| -> f64 {
        let c = e40s0
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["set"] == set && c["p"] == p && c["seed"] == seed)
            .unwrap();
        [
            "enumerated_monomials",
            "cols",
            "selection_tests",
            "term_writes",
        ]
        .iter()
        .map(|k| f(&c["fields"][k], "mean"))
        .sum()
    };

    // ── Existing configurations and the components' control replays ──
    let ide = load(&format!("{dir}/identity_existing_solvers.json"));
    let ide_runs = ide["runs"].as_array().unwrap();
    let ide_ok = ide_runs.len() == 11 && ide_runs.iter().all(|r| r["identical"] == true);
    writeln!(md, "## Existing configurations under the E43 binaries\n\n| config | reference | identical |\n|:--|:--|:--|").unwrap();
    for r in ide_runs {
        writeln!(
            md,
            "| `{}` | {} | {} |",
            r["config"].as_str().unwrap(),
            r["reference"].as_str().unwrap(),
            r["identical"]
        )
        .unwrap();
    }
    let rc = load(&format!("{dir}/replay_controls.json"));
    let rc_runs = rc["runs"].as_array().unwrap();
    let rc_ok = rc_runs.len() == 3 && rc_runs.iter().all(|r| r["identical"] == true);
    writeln!(md, "\n## The components' control runs rebuilt at the E43 commit\n\n| control | reference | byte-identical |\n|:--|:--|:--|").unwrap();
    for r in rc_runs {
        writeln!(
            md,
            "| {} | `{}` | {} |",
            r["control"].as_str().unwrap(),
            r["reference"].as_str().unwrap(),
            r["identical"]
        )
        .unwrap();
    }

    // ── Controls ──
    let ck = load(&format!("{dir}/checks.json"));
    let ck_ok = ck["g2_259_e43_checks_holds"] == true;
    let g260 = ck["g2_260_holds"] == true;
    writeln!(md, "\n## Controls (G2.259–G2.260, stream `0xE43C_0001`, {} calls)\n\n| check | count | registered |\n|:--|--:|:--|", ck["calls"]).unwrap();
    for (k, reg) in [
        ("fv_equals_f", "320/320"),
        ("fw_equals_f", "320/320"),
        ("fv_counter_ok", "320/320"),
        ("fw_counter_ok", "320/320"),
        ("pc1_v_detected", "320/320 (rule lost: V ≠ F)"),
        ("pc1_w_detected", "320/320 (rule lost: W ≠ F)"),
        ("pc2_fv_detected", "320/320 (FV lead cell dropped)"),
        (
            "pc2_fw_detected",
            "320/320 (FW caps at build end, fill check off)",
        ),
        ("pc3_fv_detected", "320/320 (FV counter short)"),
        ("pc3_fw_detected", "320/320 (FW counter short)"),
        ("g0f_equals_f", "320/320 (G0 under F)"),
        ("g0f_counter_ok", "320/320"),
        ("f_digest_differs_from_a", "— (contested calls)"),
    ] {
        writeln!(md, "| `{k}` | {} | {reg} |", ck[k]).unwrap();
    }
    writeln!(md, "| perturbed coefficient, FV / FW | {} / {} of {} non-empty | every non-empty |\n| re-allocations per call: G0 under F / FW / W | {:.2} / {:.2} / {:.2} | G0 `[150, 500]` (descriptive) |",
        ck["perturbed_fv_changed"], ck["perturbed_fw_changed"], ck["nonempty"],
        f(&ck, "g0f_reallocs_per_call"), f(&ck, "fw_reallocs_per_call"), f(&ck, "w_reallocs_per_call")).unwrap();

    // ── Stage A ──
    let sa = load(&format!("{dir}/stage_a.json"));
    let mut sa_ok = true;
    let mut sa_rows = Vec::new();
    writeln!(md, "\n## Stage A (10 curves × 1,000 calls, stream `seed ^ 0xE43A_0001`)\n\n| p | seed | identity | FW reallocs/call [W] | FW clean cells | FV ops | FV lead | FW tail ops | ΔU | δ_FV | δ_FW | break-even | fb (all chain arms) |\n|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    let names = [
        "existing",
        "chain-counted",
        "chain-fifo",
        "chain-profile",
        "chain-tails",
        "chain-fifo-profile",
        "chain-fifo-tails",
    ];
    for c in sa.as_array().unwrap() {
        let calls = c["calls"].as_u64().unwrap();
        let ckc = &c["checks"];
        let agree_ok = ckc["agree"]
            .as_array()
            .unwrap()
            .iter()
            .all(|x| x.as_u64() == Some(calls));
        let eq = |k: &str| ckc[k].as_u64() == Some(calls);
        let ok = agree_ok
            && [
                "v_equals_a",
                "w_equals_a",
                "fv_equals_f",
                "fw_equals_f",
                "fv_stored_equals_v",
                "fv_lead_equals_f_minus_offsets",
                "fw_offsets_equal_fv",
                "fw_built_equals_w",
                "fw_counter_ok",
                "tail_identity",
                "fw_tail_measure_equals_solve",
                "fw_lead_le_fv",
                "same_fallbacks",
            ]
            .iter()
            .all(|k| eq(k));
        sa_ok &= ok;
        let s = c["summaries"].as_array().unwrap();
        let sum = |name: &str| s.iter().find(|x| x["solver"] == name).unwrap();
        let rmap: Map<String, Value> = c["ratios"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| (x[0].as_str().unwrap().to_string(), x[1].clone()))
            .collect();
        let rmap1: Map<String, Value> = c["ratios_sigma1"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| (x[0].as_str().unwrap().to_string(), x[1].clone()))
            .collect();
        let cn = calls as f64;
        let fw_r = f(c, "fw_reallocs") / cn;
        let w_r = f(c, "w_reallocs") / cn;
        let du = (f(c, "updates_f") - f(c, "updates_a")) / cn;
        let (fv, fw) = (sum("chain-fifo-profile"), sum("chain-fifo-tails"));
        let fbs: Vec<u64> = names[1..]
            .iter()
            .map(|n| sum(n)["fallbacks"].as_u64().unwrap())
            .collect();
        writeln!(md, "| {} | {} | {} | {:.2} [{:.2}] | {:.0} [{}, {}] | {:.0} | {:.0} | {:.0} | {:.1} | {:.0} | {:.0} | {:.0} | {:?} |",
            c["p"], c["seed"], ok, fw_r, w_r, f(fw, "cells_clean_mean"), fw["cells_clean_min"], fw["cells_clean_max"],
            f(fv, "offset_ops_mean"), f(fv, "lead_scan_mean"), f(fw, "tail_ops_mean"), du,
            f(c, "delta_fv_mean"), f(c, "delta_fw_mean"), f(c, "break_even"), fbs).unwrap();
        let mut row = json!({"p": c["p"], "seed": c["seed"], "identity_ok": ok,
            "fw_reallocs_per_call": fw_r, "w_reallocs_per_call": w_r, "fw_reallocs_max": c["fw_reallocs_max"],
            "fw_cells_clean": f(fw, "cells_clean_mean"), "w_cells_clean": f(sum("chain-tails"), "cells_clean_mean"),
            "fv_cells_clean": f(fv, "cells_clean_mean"), "v_cells_clean": f(sum("chain-profile"), "cells_clean_mean"),
            "built_mean": f(c, "built_mean"),
            "fv_offset_ops": f(fv, "offset_ops_mean"), "v_offset_ops": f(sum("chain-profile"), "offset_ops_mean"),
            "fv_lead": f(fv, "lead_scan_mean"), "fw_lead": f(fw, "lead_scan_mean"), "f_lead": f(sum("chain-fifo"), "lead_scan_mean"),
            "fw_tail_ops": f(fw, "tail_ops_mean"), "w_tail_ops": f(sum("chain-tails"), "tail_ops_mean"),
            "delta_u": du, "delta_fv": c["delta_fv_mean"], "delta_fv_min": c["delta_fv_min"], "delta_fv_max": c["delta_fv_max"],
            "delta_fw": c["delta_fw_mean"], "delta_fw_min": c["delta_fw_min"], "delta_fw_max": c["delta_fw_max"],
            "delta_reads": c["delta_reads_mean"], "break_even": c["break_even"],
            "fallbacks": fbs,
            "c_mean": names.iter().map(|n| f(sum(n), "call_mean")).collect::<Vec<_>>(),
            "s_mean": names.iter().map(|n| sum(n)["s_mean"].clone()).collect::<Vec<_>>()});
        for (r, x) in PAIRS {
            let full = |t: &str| {
                ARMS.iter()
                    .find(|(s, _)| *s == t)
                    .map(|(_, n)| if *n == "macaulay" { "existing" } else { n })
                    .unwrap()
                    .to_string()
            };
            let lbl = format!("{}/{}", full(r), full(x));
            if let Some(v) = rmap.get(&lbl) {
                row[key(r, x)] = v.clone();
                row[format!("{}_sigma1", key(r, x))] = rmap1[&lbl].clone();
            }
        }
        sa_rows.push(row);
    }
    writeln!(md, "\nStage A ratios (`S_ref / S_arm` of the per-curve means, ranges over the ten curves):\n\n| ratio | 0 / 0.1 / 1 GAE |\n|:--|:--|").unwrap();
    let sa_band = |k: &str| -> Vec<(f64, f64)> {
        (0..3)
            .map(|g| {
                range(
                    &sa_rows
                        .iter()
                        .map(|r| r[k][g].as_f64().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    };
    let sa_col =
        |k: &str| -> (f64, f64) { range(&sa_rows.iter().map(|r| f(r, k)).collect::<Vec<_>>()) };
    let mut sa_bands = Map::new();
    for (r, x) in PAIRS {
        if sa_rows[0].get(key(r, x)).is_some() {
            let b = sa_band(&key(r, x));
            writeln!(md, "| `S_{r} / S_{x}` | {} |", fmt3(&b)).unwrap();
            sa_bands.insert(key(r, x), json!(b));
        }
    }

    // ── End to end ──
    let s27 = load("experiments/32_walk_karatsuba/summary.json");
    let w24 = load("experiments/44_pivot_rules_confirm/walk_e24.json");
    let mut rows: Vec<Value> = Vec::new();
    let mut identity = Vec::new();
    let mut health_ok = true;
    let mut unsolved_max = 0f64;
    for (set, curves) in [("gls", &GLS[..]), ("e24", &E24[..])] {
        for &(p, seed) in curves {
            let recs: Vec<Value> = ARMS
                .iter()
                .map(|(_, s)| load(&format!("{dir}/e2e/{set}_{s}_p{p}s{seed}.json")))
                .collect();
            let arms: Vec<Option<&str>> = if set == "gls" {
                vec![Some("A"), Some("B")]
            } else {
                vec![None]
            };
            for a in arms {
                let (rho, steps) = if set == "gls" {
                    let r = s27["rows"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|r| r["p"] == p && r["seed"] == seed && r["arm"] == a.unwrap())
                        .unwrap();
                    (f(r, "rho_e27"), f(r, "walk_steps"))
                } else {
                    let c = w24["curves"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|c| c["p"] == p && c["seed"] == seed)
                        .unwrap();
                    (f(&c["mean_fp_muls"], "batched"), f(c, "mean_steps"))
                };
                let rho_g: Vec<f64> = GS.iter().map(|g| rho + GAE * g * steps).collect();
                let xs: Vec<Value> = recs.iter().map(|r| arm_of(r, a)).collect();
                let x = |n: &str| &xs[ARMS.iter().position(|(s, _)| *s == n).unwrap()];
                let calls = f(x("A"), "residuals");
                let st = struct_of(set, p, seed) * calls;
                let lead = |n: &str| -> f64 {
                    // E25 emits no lead scans; it is taken as A's (E39–E42).
                    let n = if n == "E25" { "A" } else { n };
                    f(x(n), "lead_scan_reads")
                };
                let mut row = json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"),
                    "calls": calls, "rho": rho, "walk_steps": steps, "rho_g": rho_g});
                for (n, _) in ARMS {
                    let s = s_of(x(n), 0.0);
                    row[format!("S_{n}")] = json!(s);
                    row[format!("S_{n}_sigma1")] = json!(s_of(x(n), lead(n)));
                    row[format!("S_{n}_sigma4")] = json!(s_of(x(n), st));
                    row[format!("{n}_over_rho")] = json!(rat(&s, &rho_g));
                    row[format!("fallbacks_{n}")] = x(n)["solver_fallbacks"].clone();
                    row[format!("K_{n}_per_call")] = json!(k_of(x(n)) / calls);
                    row[format!("lead_{n}_per_call")] =
                        json!(x(n)["lead_scan_reads"].as_f64().map(|l| l / calls));
                    health_ok &= x(n)["folded"]["correct"] == true
                        && x(n)["rho"]
                            .as_array()
                            .is_some_and(|w| w.iter().all(|w| w["correct"] == true))
                        && f(x(n), "residuals") == calls;
                    let uns = f(x(n), "unsolved") / calls;
                    unsolved_max = unsolved_max.max(uns);
                }
                let snap = row.clone();
                let sv = |n: &str, suf: &str| -> Vec<f64> {
                    snap[format!("S_{n}{suf}")]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_f64().unwrap())
                        .collect()
                };
                for (r, a_) in PAIRS {
                    for suf in ["", "_sigma1", "_sigma4"] {
                        row[format!("{}{suf}", key(r, a_))] = json!(rat(&sv(r, suf), &sv(a_, suf)));
                        row[format!("order_{a_}_vs_{r}{suf}")] =
                            json!(order(&sv(r, suf), &sv(a_, suf)));
                    }
                }
                // Interactions per call and the break-even (§9.58).
                row["delta_fv"] = json!((k_of(x("FV")) - k_of(x("V"))) / calls);
                row["delta_fw"] = json!((k_of(x("FW")) - k_of(x("W"))) / calls);
                row["break_even"] =
                    json!((f(x("A"), "total_muls") - f(x("F"), "total_muls")) / (GAE * calls));
                row["fw_cells_per_call"] = json!(f(x("FW"), "macaulay_stored_entries") / calls);
                row["w_cells_per_call"] = json!(f(x("W"), "macaulay_stored_entries") / calls);
                row["fw_tail_per_call"] = json!(f(x("FW"), "tail_extent_ops") / calls);
                row["w_tail_per_call"] = json!(f(x("W"), "tail_extent_ops") / calls);
                row["fv_offset_per_call"] = json!(f(x("FV"), "profile_offset_ops") / calls);
                row["v_offset_per_call"] = json!(f(x("V"), "profile_offset_ops") / calls);
                // Identity of the records.
                let mut id = json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—")});
                for n in ["F", "V", "W", "FV", "FW"] {
                    id[format!("{n}_vs_A")] = json!(diff_fields(x("A"), x(n), allow_vs_a));
                }
                for n in ["FV", "FW"] {
                    id[format!("{n}_vs_F")] = json!(diff_fields(x("F"), x(n), allow_vs_f));
                }
                id["FW_vs_FV"] = json!(diff_fields(x("FV"), x("FW"), allow_vs_fv));
                id["W_vs_V"] = json!(diff_fields(x("V"), x("W"), allow_vs_fv));
                identity.push(id);
                rows.push(row);
            }
        }
    }
    let ident_ok = identity.iter().all(|i| {
        i.as_object()
            .unwrap()
            .iter()
            .filter(|(k, _)| k.contains("_vs_"))
            .all(|(_, v)| v.as_array().unwrap().is_empty())
    });
    let fb_same = rows.iter().all(|r| {
        ["F", "V", "W", "FV", "FW"]
            .iter()
            .all(|n| r[format!("fallbacks_{n}")] == r["fallbacks_A"])
    });
    let rr = |k: &str, g: usize| -> (f64, f64) {
        range(
            &rows
                .iter()
                .map(|r| r[k][g].as_f64().unwrap())
                .collect::<Vec<_>>(),
        )
    };
    let band = |k: &str| -> Vec<(f64, f64)> { (0..3).map(|g| rr(k, g)).collect() };
    let col = |k: &str| -> (f64, f64) { range(&rows.iter().map(|r| f(r, k)).collect::<Vec<_>>()) };
    let orders = |k: &str| -> Vec<String> {
        let mut o: Vec<String> = rows
            .iter()
            .map(|r| r[k].as_str().unwrap().to_string())
            .collect();
        o.sort();
        o.dedup();
        o
    };
    let all_order = |k: &str, want: &str| rows.iter().all(|r| r[k] == want);
    let count_order = |k: &str, want: &str| rows.iter().filter(|r| r[k] == want).count();

    writeln!(md, "\n## End to end (32 rows, fresh stream `E43E0001`, Karatsuba, every arm)\n\nRecord identity (F, V, W, FV, FW = A outside the registered fields; FV, FW = F outside timing, tag, lead scans and the storage fields; FW = FV and W = V outside timing, tag, lead scans, stored cells and tail ops): **{ident_ok}**; every log recovered, every walk correct, the same residuals for every arm: **{health_ok}**; the same fallbacks for every chain arm on every row: **{fb_same}**; unsolved at most `{:.4} %` of calls.\n", 100.0 * unsolved_max).unwrap();
    writeln!(md, "### Band tables (`S_ref / S_arm`, ranges over the 32 rows; order on all 32 rows)\n\n| arm | reference | `0` GAE | `0.1` GAE | `1` GAE | order (primary) | σ1 | σ4 |\n|:--|:--|:--|:--|:--|:--|:--|:--|").unwrap();
    let mut bands = Map::new();
    for (r, x) in PAIRS {
        let b = band(&key(r, x));
        let fo = |suf: &str| -> String {
            let o = orders(&format!("order_{x}_vs_{r}{suf}"));
            if o.len() == 1 {
                format!("{} on 32/32", o[0])
            } else {
                o.iter()
                    .map(|v| format!("{v} {}", count_order(&format!("order_{x}_vs_{r}{suf}"), v)))
                    .collect::<Vec<_>>()
                    .join("; ")
            }
        };
        writeln!(
            md,
            "| {x} | {r} | `{:.4}`–`{:.4}` | `{:.4}`–`{:.4}` | `{:.4}`–`{:.4}` | {} | {} | {} |",
            b[0].0,
            b[0].1,
            b[1].0,
            b[1].1,
            b[2].0,
            b[2].1,
            fo(""),
            fo("_sigma1"),
            fo("_sigma4")
        )
        .unwrap();
        bands.insert(key(r, x), json!(b));
        bands.insert(
            format!("{}_sigma1", key(r, x)),
            json!(band(&format!("{}_sigma1", key(r, x)))),
        );
        bands.insert(
            format!("{}_sigma4", key(r, x)),
            json!(band(&format!("{}_sigma4", key(r, x)))),
        );
    }
    writeln!(md, "\n### Distance to the matched rho (`S_X / rho`, ranges over the 32 rows)\n\n| arm | `0` GAE | `0.1` GAE | `1` GAE | rows with rho behind the arm at any weight |\n|:--|:--|:--|:--|--:|").unwrap();
    for (n, _) in ARMS {
        let k = format!("{n}_over_rho");
        let b = band(&k);
        let below = rows
            .iter()
            .filter(|r| (0..3).any(|g| r[&k][g].as_f64().unwrap() < 1.0))
            .count();
        writeln!(
            md,
            "| {n} | `{}`–`{}` | `{}`–`{}` | `{}`–`{}` | {below} |",
            sci(b[0].0),
            sci(b[0].1),
            sci(b[1].0),
            sci(b[1].1),
            sci(b[2].0),
            sci(b[2].1)
        )
        .unwrap();
        bands.insert(k, json!(b));
    }
    let below_rho = rows
        .iter()
        .filter(|r| {
            ARMS.iter()
                .any(|(n, _)| (0..3).any(|g| r[format!("{n}_over_rho")][g].as_f64().unwrap() < 1.0))
        })
        .count();

    writeln!(md, "\n### Every row: composed arms against each component and rho\n\n| set | p | seed | arm | calls | δ_FV | δ_FW | break-even | W/FV 1 | W/FW 0.1 | W/FW 1 | F/FW 0.1 | F/FW 1 | V/FW 1 | A/FW 0 | A/FW 1 | FW/rho 0 | FW/rho 0.1 | FW/rho 1 | FV/rho 1 | W/rho 1 |\n|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for r in &rows {
        let g = |k: &str, i: usize| r[k][i].as_f64().unwrap();
        writeln!(md, "| {} | {} | {} | {} | {} | {:.0} | {:.0} | {:.0} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {} | {} | {} | {} | {} |",
            r["set"].as_str().unwrap(), r["p"], r["seed"], r["arm"].as_str().unwrap(), r["calls"],
            f(r, "delta_fv"), f(r, "delta_fw"), f(r, "break_even"),
            g("W/FV", 2), g("W/FW", 1), g("W/FW", 2), g("F/FW", 1), g("F/FW", 2), g("V/FW", 2), g("A/FW", 0), g("A/FW", 2),
            sci(g("FW_over_rho", 0)), sci(g("FW_over_rho", 1)), sci(g("FW_over_rho", 2)), sci(g("FV_over_rho", 2)), sci(g("W_over_rho", 2))).unwrap();
    }

    // ── Negative control: FW′ and W′ on E43E0002 (GLS) ──
    let mut neg = Vec::new();
    let mut neg_ok = true;
    for &(p, seed) in &GLS {
        let paths = [
            format!("{dir}/e2e/gls_chain-fifo-tails_p{p}s{seed}.json"),
            format!("{dir}/e2e/gls_chain-fifo-tails_E43E0002_p{p}s{seed}.json"),
            format!("{dir}/e2e/gls_chain-tails_E43E0002_p{p}s{seed}.json"),
        ];
        if !paths.iter().all(|p| fs::metadata(p).is_ok()) {
            neg_ok = false;
            continue;
        }
        let (rfw, rfw2, rw2) = (load(&paths[0]), load(&paths[1]), load(&paths[2]));
        for a in ["A", "B"] {
            let (xfw, xfw2, xw2) = (
                arm_of(&rfw, Some(a)),
                arm_of(&rfw2, Some(a)),
                arm_of(&rw2, Some(a)),
            );
            let (sfw, sfw2, sw2) = (s_of(&xfw, 0.0), s_of(&xfw2, 0.0), s_of(&xw2, 0.0));
            neg.push(json!({"p": p, "seed": seed, "arm": a,
                "order_fw2_vs_fw": order(&sfw, &sfw2), "fw_over_fw2": rat(&sfw, &sfw2),
                "w2_over_fw2": rat(&sw2, &sfw2), "order_fw2_vs_w2": order(&sw2, &sfw2),
                "fw2_identity_fields": diff_fields(&xw2, &xfw2, allow_vs_a)}));
        }
    }
    let (fw2_ranked, w2_band, ff_band, fw2_ident, fw2_orders) = if neg_ok && !neg.is_empty() {
        let o: Vec<&str> = neg
            .iter()
            .map(|r| r["order_fw2_vs_fw"].as_str().unwrap())
            .collect();
        let ranked =
            o.iter().all(|&x| x == "ahead" || x == WEAK) || o.iter().all(|&x| x == "behind");
        let nb = |k: &str| -> Vec<(f64, f64)> {
            (0..3)
                .map(|g| {
                    range(
                        &neg.iter()
                            .map(|r| r[k][g].as_f64().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect()
        };
        let ident = neg
            .iter()
            .all(|r| r["fw2_identity_fields"].as_array().unwrap().is_empty());
        let mut os: Vec<String> = o.iter().map(|s| s.to_string()).collect();
        os.sort();
        let mut counts: Vec<(String, usize)> = Vec::new();
        for s in os {
            match counts.last_mut() {
                Some((l, n)) if *l == s => *n += 1,
                _ => counts.push((s, 1)),
            }
        }
        (ranked, nb("w2_over_fw2"), nb("fw_over_fw2"), ident, counts)
    } else {
        (
            true,
            vec![(0.0, 0.0); 3],
            vec![(0.0, 0.0); 3],
            false,
            vec![],
        )
    };
    let fw2_vs_w2 = neg_ok && neg.iter().all(|r| r["order_fw2_vs_w2"] == "ahead");
    writeln!(md, "\n## Negative control (stream `E43E0002`, 20 GLS rows)\n\nFW′ against FW (unpaired across streams): orders {fw2_orders:?}; FW′ ranked against FW: **{fw2_ranked}**; `S_FW / S_FW′` {}.\nFW′ against W′ (paired): `S_W′ / S_FW′` {}; FW′ ahead of W′ at every weight on every row: **{fw2_vs_w2}**; FW′ = W′ outside the vs-A fields: **{fw2_ident}**.\n",
        fmt3(&ff_band), fmt3(&w2_band)).unwrap();

    // ── Verdicts ──
    let b = |k: &str| band(k);
    let i15 = Some((1.44, 1.56));
    // Stage A: (G2.262–G2.264).
    let (fr0, fr1) = sa_col("fw_reallocs_per_call");
    let (fc0, fc1) = sa_col("fw_cells_clean");
    let (fo0, fo1) = sa_col("fv_offset_ops");
    let (fl0, fl1) = sa_col("fv_lead");
    let (ft0, ft1) = sa_col("fw_tail_ops");
    let (dv0, dv1) = sa_col("delta_fv");
    let g261 = fr0 >= 3.0 && fr1 <= 60.0 && fc0 >= 9.25e4 && fc1 <= 9.60e4;
    let g262 = sa_ok
        && fo0 >= 2.08e4
        && fo1 <= 2.40e4
        && fl0 >= 5.7e3
        && fl1 <= 6.1e3
        && dv0 >= -1500.0
        && dv1 <= 1500.0;
    let g263 = sa_ok && ft0 >= 5.48e3 && ft1 <= 5.60e3;
    let sab = |k: &str| sa_band(k);
    let g264_kill = sa_rows.iter().any(|r| {
        let s = |i: usize, g: usize| r["s_mean"][i][g].as_f64().unwrap();
        // indices in `names`: 1 A, 2 F, 5 FV, 6 FW.
        [5usize, 6].iter().any(|&x| {
            (0..3).any(|g| s(x, g) > s(1, g))
                || s(x, 0) != s(2, 0)
                || s(x, 1) > s(2, 1)
                || s(x, 2) > s(2, 2)
        })
    });
    let g264 = !g264_kill
        && inside3(&sab("A/FV"), [i15, Some((1.37, 1.46)), Some((1.35, 1.42))])
        && inside3(&sab("F/FV"), [None, Some((1.25, 1.32)), Some((1.33, 1.40))])
        && inside3(
            &sab("V/FV"),
            [i15, Some((1.10, 1.16)), Some((1.004, 1.030))],
        )
        && inside3(
            &sab("W/FV"),
            [i15, Some((1.08, 1.13)), Some((0.972, 0.997))],
        )
        && inside3(&sab("A/FW"), [i15, Some((1.40, 1.50)), Some((1.39, 1.47))])
        && inside3(&sab("F/FW"), [None, Some((1.28, 1.36)), Some((1.38, 1.45))])
        && inside3(
            &sab("V/FW"),
            [i15, Some((1.13, 1.19)), Some((1.035, 1.065))],
        )
        && inside3(
            &sab("W/FW"),
            [i15, Some((1.10, 1.16)), Some((1.004, 1.030))],
        )
        && inside3(
            &sab("FV/FW"),
            [None, Some((1.010, 1.040)), Some((1.015, 1.050))],
        );
    let g265 = inside3(&b("A/FV"), [i15, Some((1.38, 1.46)), Some((1.36, 1.43))])
        && inside3(&b("F/FV"), [None, Some((1.26, 1.32)), Some((1.35, 1.41))])
        && inside3(&b("V/FV"), [i15, Some((1.10, 1.16)), Some((1.004, 1.030))])
        && inside3(&b("W/FV"), [i15, Some((1.08, 1.13)), Some((0.972, 0.997))])
        && inside3(
            &b("E25/FV"),
            [Some((3.5, 4.1)), Some((1.92, 2.09)), Some((1.41, 1.50))],
        );
    let g266 = inside3(&b("A/FW"), [i15, Some((1.41, 1.50)), Some((1.40, 1.48))])
        && inside3(&b("F/FW"), [None, Some((1.29, 1.36)), Some((1.39, 1.46))])
        && inside3(&b("V/FW"), [i15, Some((1.13, 1.19)), Some((1.035, 1.065))])
        && inside3(&b("W/FW"), [i15, Some((1.10, 1.16)), Some((1.004, 1.030))])
        && inside3(
            &b("E25/FW"),
            [Some((3.5, 4.1)), Some((1.97, 2.14)), Some((1.46, 1.55))],
        );
    let (edv0, edv1) = col("delta_fv");
    let fw_under_break_even = rows.iter().all(|r| f(r, "delta_fw") < f(r, "break_even"));
    let g267 = edv0 >= -1500.0
        && edv1 <= 1500.0
        && fw_under_break_even
        && inside3(
            &b("FV/FW"),
            [None, Some((1.010, 1.040)), Some((1.015, 1.050))],
        );
    let g268 = inside3(
        &b("A/F"),
        [
            Some((1.43, 1.57)),
            Some((1.085, 1.115)),
            Some((1.010, 1.015)),
        ],
    ) && inside3(&b("A/V"), [None, Some((1.18, 1.38)), Some((1.28, 1.48))])
        && inside3(&b("V/W"), [None, Some((1.015, 1.035)), Some((1.02, 1.05))]);
    let equal_f_at0 = rows
        .iter()
        .all(|r| r["S_FV"][0] == r["S_F"][0] && r["S_FW"][0] == r["S_F"][0]);
    let strict_all = |x: &str| {
        ["A", "V", "W", "F"]
            .iter()
            .all(|r| all_order(&format!("order_{x}_vs_{r}"), "ahead"))
    };
    let strong_fv = strict_all("FV");
    let strong_fw = strict_all("FW");
    let g269 = equal_f_at0 && !strong_fv && !strong_fw;
    let comp_rank = |x: &str| {
        ["A", "V", "W", "E25"]
            .iter()
            .all(|r| all_order(&format!("order_{x}_vs_{r}"), "ahead"))
            && all_order(&format!("order_{x}_vs_F"), WEAK)
    };
    let fw_comp = comp_rank("FW");
    let fv_comp = comp_rank("FV");
    let fv_pred = ["A", "V", "E25"]
        .iter()
        .all(|r| all_order(&format!("order_FV_vs_{r}"), "ahead"))
        && all_order("order_FV_vs_F", WEAK)
        && rows.iter().all(|r| {
            let s = |n: &str, g: usize| r[format!("S_{n}")][g].as_f64().unwrap();
            s("FV", 0) < s("W", 0) && s("FV", 1) < s("W", 1) && s("FV", 2) > s("W", 2)
        });
    let g270 = fw_comp && fv_pred;
    let g271 = below_rho == 0
        && inside(b("W/FW")[2], 1.004, 1.030)
        && rows
            .iter()
            .all(|r| r["FW_over_rho"][0] == r["F_over_rho"][0]);
    let sens_same = rows.iter().all(|r| {
        PAIRS
            .iter()
            .filter(|(_, x)| *x == "FV" || *x == "FW")
            .all(|(rf, x)| {
                let o = &r[format!("order_{x}_vs_{rf}")];
                let o1 = &r[format!("order_{x}_vs_{rf}_sigma1")];
                let o4 = &r[format!("order_{x}_vs_{rf}_sigma4")];
                o == o1 && o == o4
            })
    });
    let g272 = sens_same;
    let g273 = neg_ok
        && !fw2_ranked
        && fw2_ident
        && inside3(&w2_band, [i15, Some((1.10, 1.16)), Some((1.004, 1.030))]);
    let g274 = health_ok && fb_same && unsolved_max <= 0.001 && sa_ok;
    let g258 = sa_ok && ident_ok && ide_ok;
    let g259 = ck_ok && rc_ok;
    for (k, v) in [
        ("G2.258_identity", g258),
        ("G2.259_controls", g259),
        ("G2.260_g0_reallocs_descriptive", g260),
        ("G2.261_fw_reallocs_descriptive", g261),
        ("G2.262_fv_counters", g262),
        ("G2.263_fw_counters", g263),
        ("G2.264_stage_a_bands", g264),
        ("G2.264_kill_fired", g264_kill),
        ("G2.265_e2e_fv", g265),
        ("G2.266_e2e_fw", g266),
        ("G2.267_additivity", g267),
        ("G2.268_components_replicated", g268),
        ("G2.269_strong_rank_not_met_by_construction", g269),
        ("G2.270_composition_rank_as_predicted", g270),
        ("G2.271_rho", g271),
        ("G2.272_sensitivities", g272),
        ("G2.273_negative", g273),
        ("G2.274_health", g274),
        ("stage_a_identity", sa_ok),
        ("e2e_identity", ident_ok),
        ("existing_configs_identical", ide_ok),
        ("component_controls_replayed", rc_ok),
        ("e43_checks", ck_ok),
        ("FW_delta_under_break_even_every_row", fw_under_break_even),
        ("strong_rank_FV", strong_fv),
        ("strong_rank_FW", strong_fw),
        ("FW_equal_F_at_0_every_row", equal_f_at0),
    ] {
        verdict.insert(k.into(), json!(v));
    }
    let gate = g258 && g259 && !g264_kill;
    verdict.insert(
        "FW_composition_rank_registered_rule".into(),
        json!(gate && fw_comp),
    );
    verdict.insert(
        "FV_composition_rank_registered_rule".into(),
        json!(gate && fv_comp),
    );
    for x in ["FV", "FW"] {
        for r in ["A", "F", "V", "W", "E25"] {
            let k = format!("order_{x}_vs_{r}");
            verdict.insert(k.clone(), json!(orders(&k)));
        }
    }
    verdict.insert("order_FW_vs_FV".into(), json!(orders("order_FW_vs_FV")));
    writeln!(
        md,
        "\n## Verdicts\n\n```\n{}\n```",
        serde_json::to_string_pretty(&verdict).unwrap()
    )
    .unwrap();
    let summary = json!({
        "verdict": verdict, "stage_a": sa_rows, "stage_a_bands": sa_bands, "rows": rows,
        "negative": neg, "e2e_identity": identity, "bands": bands,
        "negative_bands": {"fw_over_fw2": ff_band, "w2_over_fw2": w2_band},
    });
    fs::write(format!("{dir}/tables.md"), md).unwrap();
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();
    eprintln!("{}", serde_json::to_string_pretty(&verdict).unwrap());
}
