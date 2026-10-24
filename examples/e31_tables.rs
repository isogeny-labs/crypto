//! E31 tables (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.22): Stage A
//! (existing vs chain solve per call), the end-to-end replay check against
//! E25, `S` and the gap to the matched `⟨±ψ⟩` rho, and the §5a band.
//!
//! ```text
//! cargo run --release --example e31_tables -- experiments/31_solve_cost experiments/30_gls_v4 > experiments/31_solve_cost/tables.md
//! ```

use std::fmt::Write as _;
use std::fs;

use serde_json::Value;

const PS: [u64; 5] = [101, 149, 197, 269, 397];
const SEEDS: [u64; 2] = [1, 2];
/// Fields an E31 rerun may change: the solve's multiplications and what
/// is derived from them, wall times, the matrix entries a fallback builds
/// twice, and the solver tags.
const MAY_DIFFER: [&str; 10] = [
    "solve_muls",
    "total_muls",
    "ratio",
    "call",
    "solve_ms",
    "wall_ms",
    "macaulay_entries",
    "solver",
    "solver_fallbacks",
    "experiment",
];

fn f(v: &Value, k: &str) -> f64 {
    let mut cur = v;
    for part in k.split('.') {
        cur = &cur[part];
    }
    cur.as_f64().unwrap_or(f64::NAN)
}

fn g3(x: f64) -> String {
    if x == 0.0 || !x.is_finite() {
        return format!("{x}");
    }
    let e = x.abs().log10().floor() as i32;
    format!("{:.2}e{}", x / 10f64.powi(e), e)
}

/// Paths (dotted) where `a` and `b` differ, ignoring `MAY_DIFFER` at the
/// top level and wall-time fields anywhere.
fn diff(a: &Value, b: &Value, path: &str, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let mut keys: Vec<&String> = x.keys().chain(y.keys()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                if path.is_empty() && MAY_DIFFER.contains(&k.as_str()) {
                    continue;
                }
                if k.ends_with("_ms") {
                    continue;
                }
                let p = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{path}.{k}")
                };
                diff(
                    x.get(k).unwrap_or(&Value::Null),
                    y.get(k).unwrap_or(&Value::Null),
                    &p,
                    out,
                );
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (u, v)) in x.iter().zip(y).enumerate() {
                diff(u, v, &format!("{path}[{i}]"), out);
            }
        }
        _ => {
            if a != b {
                out.push(path.to_string());
            }
        }
    }
}

fn load(path: &str) -> Option<Vec<Value>> {
    let s = fs::read_to_string(path).ok()?;
    serde_json::from_str::<Vec<Value>>(&s).ok()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args.get(1).map_or("experiments/31_solve_cost", |s| s.as_str());
    let e25 = args.get(2).map_or("experiments/30_gls_v4", |s| s.as_str());
    let mut o = String::new();

    // ── Stage A ──
    if let Some(stage) = load(&format!("{dir}/stage_a.json")) {
        o.push_str("### E31 Stage A — per call, existing vs chain (A) vs chain with sparsest pivots (B)\n\n");
        o.push_str("Same calls (the first 1,000 targets of E25 arm A's stream per curve), one thread, interleaved per call. `C` = Weil restriction + solve (E25's per-call count), `F_p` multiplications.\n\n");
        o.push_str("| p | seed | agree A / B | M, charpoly agree A / B | C median existing | C median A | C median B | existing / A | existing / B | NF existing / A | charpoly existing / A | eigvec existing / A | echelon A / B | fallbacks A / B | wall median µs existing / A / B | wall existing / A | wall existing / B |\n");
        o.push_str("|--:|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|--:|\n");
        for c in &stage {
            let s = |name: &str| -> &Value {
                c["summaries"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|x| x["solver"] == name)
                    .unwrap()
            };
            let (e, a, b) = (s("macaulay"), s("chain"), s("chain-sparsest"));
            let _ = writeln!(
                o,
                "| {} | {} | {} / {} of {} | {} / {} of {} | {} | {} | {} | {:.2} | {:.2} | {:.1} | {:.2} | {:.1} | {:.2} | {} / {} | {:.0} / {:.0} / {:.0} | {:.2} | {:.2} |",
                c["p"], c["seed"], c["agree"][0], c["agree"][1], c["calls"],
                c["quotient_agree"][0], c["quotient_agree"][1], c["quotient_checked"][0],
                e["call_median"], a["call_median"], b["call_median"],
                f(e, "call_median") / f(a, "call_median"),
                f(e, "call_median") / f(b, "call_median"),
                f(e, "normal_forms_mean") / f(a, "normal_forms_mean"),
                f(e, "charpoly_mean") / f(a, "charpoly_mean"),
                f(e, "eigenvectors_mean") / f(a, "eigenvectors_mean"),
                f(a, "echelon_mean") / f(b, "echelon_mean"),
                a["fallbacks"], b["fallbacks"],
                f(e, "wall_us_median"), f(a, "wall_us_median"), f(b, "wall_us_median"),
                f(e, "wall_us_median") / f(a, "wall_us_median"),
                f(e, "wall_us_median") / f(b, "wall_us_median"),
            );
        }
        o.push_str("\nPer-call phase means (all calls, fallbacks charged):\n\n| p | seed | solver | echelon | normal forms | charpoly | roots | eigenvectors | other (Weil, triangularise, checks) | C mean |\n|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|\n");
        for c in &stage {
            for s in c["summaries"].as_array().unwrap() {
                let _ = writeln!(
                    o,
                    "| {} | {} | {} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} |",
                    c["p"],
                    c["seed"],
                    s["solver"].as_str().unwrap(),
                    f(s, "echelon_mean"),
                    f(s, "normal_forms_mean"),
                    f(s, "charpoly_mean"),
                    f(s, "roots_mean"),
                    f(s, "eigenvectors_mean"),
                    f(s, "other_mean"),
                    f(s, "call_mean"),
                );
            }
        }
        o.push('\n');
    }

    // ── End to end ──
    for solver in ["chain", "chain-sparsest"] {
        let mut rows = Vec::new();
        for p in PS {
            for s in SEEDS {
                let (Some(old), Some(new)) = (
                    load(&format!("{e25}/p{p}s{s}.json")),
                    load(&format!("{dir}/e2e_{solver}_p{p}s{s}.json")),
                ) else {
                    continue;
                };
                for arm in ["A", "B"] {
                    let pick = |v: &Vec<Value>| v.iter().find(|r| r["arm"] == arm).cloned();
                    if let (Some(a), Some(b)) = (pick(&old), pick(&new)) {
                        rows.push((p, s, arm, a, b));
                    }
                }
            }
        }
        if rows.is_empty() {
            continue;
        }
        let _ = writeln!(
            o,
            "### E31 end to end — E25's curves rerun with `--solver {solver}`\n"
        );
        o.push_str("Replay check: every field of the E25 record other than the solve's multiplications and what is derived from them (`solve_muls`, `total_muls`, `ratio`, `call`), wall times, `macaulay_entries` (a fallback builds the matrix twice) and the solver tags.\n\n");
        o.push_str("| p | seed | arm | other fields identical | fallbacks | C median E25 / E31 | solves share E31 | S E25 | S E31 | S E25 / S E31 | rho ±ψ | S / rho E25 | S / rho E31 | correct (log, walks) |\n");
        o.push_str("|--:|--:|:--|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|\n");
        let mut band = String::new();
        band.push_str("\n| p | seed | arm | GAE (F_p mults) | S E25 / S E31 at 0 | at 0.1 | at 1 | S / rho E31 at 0 | at 0.1 | at 1 | E31 below E25 at every weight |\n|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|\n");
        for (p, s, arm, a, b) in &rows {
            let mut d = Vec::new();
            diff(a, b, "", &mut d);
            let ident = if d.is_empty() {
                "yes".to_string()
            } else {
                format!("**no**: {}", d.join(", "))
            };
            let share = (f(b, "weil_muls") + f(b, "solve_muls")) / f(b, "total_muls");
            let _ = writeln!(
                o,
                "| {p} | {s} | {arm} | {ident} | {} | {} / {} | {:.1} % | {} | {} | {:.2} | {} | {} | {} | {}, {} |",
                b["solver_fallbacks"],
                g3(f(a, "call.median")),
                g3(f(b, "call.median")),
                100.0 * share,
                g3(f(a, "total_muls")),
                g3(f(b, "total_muls")),
                f(a, "total_muls") / f(b, "total_muls"),
                g3(f(b, "rho_matched_muls")),
                g3(f(a, "ratio")),
                g3(f(b, "ratio")),
                b["folded"]["correct"],
                b["rho"].as_array().unwrap().iter().all(|w| w["correct"] == true),
            );
            let gae = f(b, "fp_muls_per_add");
            let s_at = |r: &Value, g: f64| f(r, "total_muls") + g * gae * f(r, "macaulay_entries");
            let rho_at =
                |r: &Value, g: f64| f(r, "rho_matched_muls") + g * gae * f(r, "rho_matched_steps");
            let rs: Vec<f64> = [0.0, 0.1, 1.0].iter().map(|&g| s_at(a, g) / s_at(b, g)).collect();
            let gr: Vec<f64> = [0.0, 0.1, 1.0].iter().map(|&g| s_at(b, g) / rho_at(b, g)).collect();
            let _ = writeln!(
                band,
                "| {p} | {s} | {arm} | {gae:.1} | {:.2} | {:.2} | {:.3} | {} | {} | {} | {} |",
                rs[0],
                rs[1],
                rs[2],
                g3(gr[0]),
                g3(gr[1]),
                g3(gr[2]),
                if rs.iter().all(|&x| x > 1.0) { "yes" } else { "**no**" },
            );
        }
        let _ = writeln!(
            o,
            "\n#### §5a band — unpriced native work at 0, 0.1 and 1 GAE (`--solver {solver}`)\n\nOne operation per Macaulay-matrix entry (as recorded per run) for the index calculus, one per step for the matched walk, as E25's table priced them."
        );
        o.push_str(&band);
        o.push('\n');
    }
    print!("{o}");
}
