//! Score, compare and reference refutation-degree runs (`dreg_ladder` output).
//!
//! ```sh
//! # per-cell values and the registered reading, with the semi-regular reference
//! cargo run --release --example dreg_score -- score runs/*.jsonl
//! # draw-by-draw identity of a rerun against committed rows (all but `secs`)
//! cargo run --release --example dreg_score -- compare committed.jsonl rerun.jsonl
//! # the semi-regular degree of regularity of chained-system shapes
//! cargo run --release --example dreg_score -- semireg 4 5:2 6:2 7:2
//! # upper bounds on the degree-D Macaulay matrix shape
//! cargo run --release --example dreg_score -- shape 4 6:2:8 8:2:7
//! ```
//!
//! The native replacement for the degree-7 study's `score.py`, `compare.py`
//! and `semireg.py`, which stay in the repository as that study's record.
//!
//! **Reading of a cell** (the rule the refutation-degree studies register):
//! a resolved draw is its degree, exact; a full matrix without resolution
//! at `d_max` is a bound `≥ d_max + 1` and counts as that value; caps-hit
//! draws are excluded.  With at least three values, the median's middle
//! values give the reading: one degree `D` (all exact), `≥D` (all bounds),
//! or `a / b` when they differ.  Fewer than three values: not testable.

use crypto_lib::cryptanalysis::koblitz_bench::semi_regular_dreg;
use serde_json::Value;
use std::collections::BTreeMap;

/// The equation degrees of the `m`-summand chained system over GF(2^n):
/// `(m − 2)·n` cubic links' equations and `n` quadratic ones (the last link
/// closes on the known `x(R)`).
fn chain_degrees(m: usize, n: usize) -> Vec<u32> {
    std::iter::repeat_n(3, (m - 2) * n)
        .chain(std::iter::repeat_n(2, n))
        .collect()
}

fn chain_dreg(m: usize, n: usize, ell: usize) -> u32 {
    semi_regular_dreg((m - 2) * n + m * ell, &chain_degrees(m, n)).expect("reference exists")
}

/// (degree, exact) of one measured draw; `None` for caps-hit and satisfiable.
fn value(outcome: &Value) -> Option<(u64, bool)> {
    let degree = outcome.get("degree")?.as_u64()?;
    match outcome.get("kind")?.as_str()? {
        "resolved" => Some((degree, true)),
        "at_least" => Some((degree, false)),
        _ => None,
    }
}

fn show(v: (u64, bool)) -> String {
    if v.1 {
        v.0.to_string()
    } else {
        format!("≥{}", v.0)
    }
}

fn reading(values: &[(u64, bool)]) -> String {
    if values.len() < 3 {
        return "not testable".into();
    }
    let mut sorted = values.to_vec();
    sorted.sort();
    let k = sorted.len();
    let middle: Vec<(u64, bool)> = if k % 2 == 1 {
        vec![sorted[k / 2]]
    } else {
        vec![sorted[k / 2 - 1], sorted[k / 2]]
    };
    if middle.iter().all(|v| *v == middle[0]) {
        show(middle[0])
    } else {
        format!("{} / {}", show(middle[0]), show(middle[1]))
    }
}

fn lines(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| panic!("{path}: {e}")))
        .filter(|v: &Value| v.get("control").is_none())
        .collect()
}

fn score(paths: &[String]) {
    // (m, n, ℓ) -> per-draw (unsat_index or order, value text, value, ffd)
    let mut cells: BTreeMap<(u64, u64, u64), Vec<(String, Option<(u64, bool)>, String)>> =
        BTreeMap::new();
    for path in paths {
        for v in lines(path) {
            let outcome = &v["outcome"];
            if outcome["kind"] == "satisfiable" {
                continue;
            }
            let key = (
                v.get("m").and_then(Value::as_u64).unwrap_or(3),
                v["n"].as_u64().expect("n"),
                v["ell"].as_u64().expect("ell"),
            );
            let val = value(outcome);
            let shown = val.map_or_else(|| outcome.to_string(), show);
            let ffd = v["ffd"].as_u64().map_or("-".into(), |f| f.to_string());
            cells.entry(key).or_default().push((shown, val, ffd));
        }
    }
    println!("| m | cell (n, ℓ) | N | S | values | reading | semi-regular D_reg | FFD |");
    println!("|--:|---|--:|--:|---|---|--:|---|");
    for ((m, n, ell), draws) in &cells {
        let (m, n, ell) = (*m as usize, *n as usize, *ell as usize);
        let vals: Vec<(u64, bool)> = draws.iter().filter_map(|d| d.1).collect();
        println!(
            "| {m} | ({n}, {ell}) | {} | {:+} | {} | {} | {} | {} |",
            (m - 2) * n + m * ell,
            n as i64 - (m * ell) as i64,
            draws
                .iter()
                .map(|d| d.0.as_str())
                .collect::<Vec<_>>()
                .join(" "),
            reading(&vals),
            chain_dreg(m, n, ell),
            draws
                .iter()
                .map(|d| d.2.as_str())
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
}

fn compare(reference: &str, rerun: &str) {
    let key = |v: &Value| {
        [
            "draw",
            "v_basis",
            "x_r",
            "solutions",
            "outcome",
            "ffd",
            "n",
            "ell",
        ]
        .iter()
        .map(|k| v.get(*k).cloned().unwrap_or(Value::Null).to_string())
        .collect::<Vec<_>>()
    };
    let (want, got) = (lines(reference), lines(rerun));
    let mut mismatches = want.len().abs_diff(got.len());
    let mut measured = 0;
    for (a, b) in want.iter().zip(&got) {
        if key(a) != key(b) {
            mismatches += 1;
        }
        if a["outcome"]["kind"] != "satisfiable" {
            measured += 1;
        }
    }
    println!(
        "{rerun}: {} rows, {measured} measured, mismatches {mismatches}",
        want.len()
    );
    if mismatches > 0 {
        std::process::exit(1);
    }
}

fn semireg(m: usize, cells: &[String]) {
    println!("| m | cell (n, ℓ) | N | S | equations | semi-regular D_reg |");
    println!("|--:|---|--:|--:|---|--:|");
    for c in cells {
        let (n, ell) = c.split_once(':').expect("cell is n:ell");
        let (n, ell): (usize, usize) = (n.parse().expect("n"), ell.parse().expect("ell"));
        println!(
            "| {m} | ({n}, {ell}) | {} | {:+} | {} cubic + {n} quadratic | {} |",
            (m - 2) * n + m * ell,
            n as i64 - (m * ell) as i64,
            (m - 2) * n,
            chain_dreg(m, n, ell)
        );
    }
}

/// `C(N, ≤ d)`: square-free monomials of degree at most `d` in `N` unknowns.
fn monomials_up_to(n_vars: u128, d: i64) -> u128 {
    let mut total = 0u128;
    let mut binom = 1u128;
    for i in 0..=d.max(-1) {
        if i as u128 > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i as u128) / (i as u128 + 1);
    }
    total
}

/// Upper bounds on the degree-`D` Macaulay matrix of the `m`-summand chain:
/// every equation times every monomial of degree `≤ D − deg`, and every
/// monomial of degree `≤ D` as a column.
fn shape(m: usize, cells: &[String]) {
    println!("| m | cell (n, ℓ) | N | degree | rows | columns |");
    println!("|--:|---|--:|--:|--:|--:|");
    for c in cells {
        let p: Vec<usize> = c.split(':').map(|x| x.parse().expect("n:ℓ:D")).collect();
        let (n, ell, d) = (p[0], p[1], p[2] as i64);
        let n_vars = ((m - 2) * n + m * ell) as u128;
        let rows = ((m - 2) * n) as u128 * monomials_up_to(n_vars, d - 3)
            + n as u128 * monomials_up_to(n_vars, d - 2);
        println!(
            "| {m} | ({n}, {ell}) | {n_vars} | {d} | {rows} | {} |",
            monomials_up_to(n_vars, d)
        );
    }
}

/// Score `dreg_closure` output against PREREGISTRATION.md of
/// `research/dreg_closure_mutantxl_20261016`: per draw the closure degree
/// and its gap to the committed XL degree; per cell the Q1 verdict
/// (below / not below / mixed), or "exploratory" when `C(N, ≤ XL − 1)`
/// exceeds the column cap (the registered testability rule); then Q1's
/// study verdict, Q2's gap counts and Q3's per-ℓ medians at m = 3, and the
/// registered descriptive cost table.  `--satisfiable` output files are
/// tallied as the soundness control.
fn closure(paths: &[String]) {
    struct Draw {
        xl: u64,
        known: bool,
        closure: Option<u64>,
        measured_to: u64,
        // At the closure's refuting degree: columns, products, word ops.
        cost: Option<(u64, u64, u64)>,
    }
    let mut cells: BTreeMap<(u64, u64, u64), Vec<Draw>> = BTreeMap::new();
    let mut max_cols = 0u128;
    let (mut sat_draws, mut sat_unsound) = (0usize, 0usize);
    let mut sat_degree: BTreeMap<u64, usize> = BTreeMap::new();
    for path in paths {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            if let Some(bad) = v["refuted_satisfiable"].as_bool() {
                sat_draws += 1;
                sat_unsound += bad as usize;
                *sat_degree
                    .entry(v["measured_to"].as_u64().unwrap())
                    .or_default() += 1;
                continue;
            }
            max_cols = max_cols.max(v["max_cols"].as_f64().unwrap() as u128);
            let key = (
                v["m"].as_u64().unwrap(),
                v["n"].as_u64().unwrap(),
                v["ell"].as_u64().unwrap(),
            );
            let o = &v["committed_outcome"];
            let closure = v["closure_degree"].as_u64();
            let cost = closure.and_then(|c| {
                v["scan"].as_array().unwrap().iter().find_map(|s| {
                    (s["degree"].as_u64() == Some(c)).then(|| {
                        (
                            s["columns"].as_u64().unwrap(),
                            s["closure_products"].as_u64().unwrap(),
                            s["closure_word_ops"].as_u64().unwrap(),
                        )
                    })
                })
            });
            cells.entry(key).or_default().push(Draw {
                xl: o["degree"].as_u64().unwrap(),
                known: o["kind"] == "resolved",
                closure,
                measured_to: v["measured_to"].as_u64().unwrap(),
                cost,
            });
        }
    }
    let med = |mut v: Vec<f64>| -> f64 {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let k = v.len();
        if k == 0 {
            f64::NAN
        } else if k % 2 == 1 {
            v[k / 2]
        } else {
            (v[k / 2 - 1] + v[k / 2]) / 2.0
        }
    };
    println!("| m | cell (n, ℓ) | N | committed XL | closure degree | gap XL − closure | Q1 |");
    println!("|--:|---|--:|---|---|---|---|");
    let (mut below, mut not_below, mut testable) = (0, 0, 0);
    let (mut gap1, mut gap2) = (0, 0);
    let mut by_ell: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    let mut cost_rows = Vec::new();
    for ((m, n, ell), draws) in &cells {
        let n_vars = (m - 2) * n + m * ell;
        // Registered rule: testable when the degree XL − 1 fits the cap.
        let xl_min = draws.iter().map(|d| d.xl).min().unwrap();
        let fits = monomials_up_to(n_vars as u128, xl_min as i64 - 1) <= max_cols;
        let k = draws
            .iter()
            .filter(|d| matches!(d.closure, Some(c) if c < d.xl))
            .count();
        let unmeasured = draws
            .iter()
            .any(|d| d.closure.is_none() && d.measured_to + 1 < d.xl - (!d.known) as u64);
        let verdict = if !fits {
            if k == draws.len() {
                "exploratory: below".to_string()
            } else {
                "exploratory".into()
            }
        } else if unmeasured {
            "untestable".into()
        } else {
            testable += 1;
            if k == draws.len() {
                below += 1;
                "below".into()
            } else if k == 0 {
                not_below += 1;
                "not below".into()
            } else {
                "mixed".into()
            }
        };
        for d in draws {
            if let Some(c) = d.closure {
                if d.known && c < d.xl {
                    if d.xl - c == 1 {
                        gap1 += 1
                    } else {
                        gap2 += 1
                    }
                }
                if *m == 3 {
                    by_ell.entry(*ell).or_default().push(c as f64);
                }
            }
        }
        let xl: Vec<String> = draws
            .iter()
            .map(|d| {
                if d.known {
                    d.xl.to_string()
                } else {
                    format!("≥{}", d.xl)
                }
            })
            .collect();
        let cl: Vec<String> = draws
            .iter()
            .map(|d| {
                d.closure
                    .map_or(format!(">{}", d.measured_to), |c| c.to_string())
            })
            .collect();
        let gaps: Vec<String> = draws
            .iter()
            .map(|d| match (d.known, d.closure) {
                (true, Some(c)) => (d.xl - c).to_string(),
                (false, Some(c)) => format!("≥{}", d.xl - c),
                _ => "–".into(),
            })
            .collect();
        println!(
            "| {m} | ({n}, {ell}) | {n_vars} | {} | {} | {} | {verdict} |",
            xl.join(" "),
            cl.join(" "),
            gaps.join(" ")
        );
        let lg = |d: u64| (monomials_up_to(n_vars as u128, d as i64) as f64).log2();
        let costed: Vec<&Draw> = draws.iter().filter(|d| d.cost.is_some()).collect();
        cost_rows.push(format!(
            "| {m} | ({n}, {ell}) | {n_vars} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |",
            med(draws.iter().map(|d| d.xl as f64).collect()),
            med(draws.iter().map(|d| lg(d.xl)).collect()),
            med(costed.iter().map(|d| lg(d.closure.unwrap())).collect()),
            med(costed
                .iter()
                .map(|d| (d.cost.unwrap().1.max(1) as f64).log2())
                .collect()),
            med(costed
                .iter()
                .map(|d| (d.cost.unwrap().2.max(1) as f64).log2())
                .collect()),
        ));
    }
    let study = if 3 * below >= 2 * testable {
        "closure lowers"
    } else if 3 * not_below >= 2 * testable {
        "no effect"
    } else {
        "mixed"
    };
    println!();
    println!("Q1: {below} of {testable} testable cells below, {not_below} not below → {study}");
    println!("Q2: gaps among resolved-XL draws refuted below XL: {gap1} of 1, {gap2} of ≥ 2");
    let medians: Vec<String> = by_ell
        .iter()
        .map(|(ell, v)| format!("ℓ={ell}: {} (n={})", med(v.clone()), v.len()))
        .collect();
    println!(
        "Q3: m = 3 closure-degree medians by ℓ: {}",
        medians.join(", ")
    );
    if sat_draws > 0 {
        let to: Vec<String> = sat_degree
            .iter()
            .map(|(d, k)| format!("{k} to degree {d}"))
            .collect();
        println!(
            "Soundness control: {sat_draws} satisfiable draws ({}), {sat_unsound} refuted",
            to.join(", ")
        );
    }
    println!();
    println!("Descriptive cost, cell medians (log2; columns are C(N, ≤D)):");
    println!();
    println!("| m | cell (n, ℓ) | N | XL degree | log2 cols at XL | log2 cols at closure | log2 closure products | log2 closure word ops |");
    println!("|--:|---|--:|--:|--:|--:|--:|--:|");
    for r in cost_rows {
        println!("{r}");
    }
}

/// Score `research/dreg_closure_scale_20261016` (its PREREGISTRATION.md):
/// one `dreg_closure --to 4 --draw K` output file per draw.  An empty file
/// is a draw the memory or time limit killed (unmeasured, never evidence);
/// its cell is read from the file name (`fresh-m<M>-<n>-<ℓ>.d<K>` or
/// `committed-cell-<n>-<ℓ>-*`, which are `m = 3`).  Per draw: closure ≤ 4,
/// above 4 (degree 4 built and saturated without refuting), or unmeasured;
/// per cell flat / rises / unmeasured; the study verdict; Q5 (XL never
/// refutes at 4); Q6 (log₂ word ops ÷ log₂ columns at the refuting degree).
/// `--satisfiable` outputs are tallied as the soundness control.
fn scale(paths: &[String]) {
    #[derive(Default)]
    struct Cell {
        n_vars: u64,
        degrees: Vec<String>,
        flat: usize,
        above: usize,
        unmeasured: usize,
        xl_at_4: usize,
        ratios: Vec<f64>,
        secs: Vec<f64>,
    }
    let mut cells: BTreeMap<(u64, u64, u64), Cell> = BTreeMap::new();
    let (mut sat_draws, mut sat_bad, mut sat_unmeasured) = (0usize, 0usize, 0usize);
    for path in paths {
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let line = text.lines().find(|l| !l.trim().is_empty());
        let Some(line) = line else {
            // A killed control is an unmeasured control, never evidence.
            if name.starts_with("control-") {
                sat_unmeasured += 1;
                continue;
            }
            // Killed: read the cell from the file name.
            let key = if let Some(rest) = name.strip_prefix("fresh-m") {
                let p: Vec<u64> = rest
                    .split(['-', '.'])
                    .take(3)
                    .map(|x| x.parse().expect("fresh-m<M>-<n>-<l>"))
                    .collect();
                (p[0], p[1], p[2])
            } else if let Some(rest) = name.strip_prefix("committed-cell-") {
                let p: Vec<u64> = rest
                    .split('-')
                    .take(2)
                    .map(|x| x.parse().expect("committed-cell-<n>-<l>"))
                    .collect();
                (3, p[0], p[1])
            } else {
                panic!("{path}: empty and unnamed");
            };
            let c = cells.entry(key).or_default();
            c.unmeasured += 1;
            c.degrees.push("killed".into());
            continue;
        };
        let v: Value = serde_json::from_str(line).expect("json");
        let scan = v["scan"].as_array().expect("scan");
        if let Some(bad) = v["refuted_satisfiable"].as_bool() {
            sat_draws += 1;
            sat_bad += bad as usize;
            continue;
        }
        let key = (
            v["m"].as_u64().unwrap(),
            v["n"].as_u64().unwrap(),
            v["ell"].as_u64().unwrap(),
        );
        let c = cells.entry(key).or_default();
        c.n_vars = v["n_vars"].as_u64().unwrap();
        let at4 = scan.iter().find(|s| s["degree"] == 4);
        c.xl_at_4 += at4.is_some_and(|s| s["xl_refuted"] == true) as usize;
        c.secs
            .push(scan.iter().map(|s| s["secs"].as_f64().unwrap()).sum());
        match v["closure_degree"].as_u64() {
            Some(d) if d <= 4 => {
                c.flat += 1;
                c.degrees.push(d.to_string());
                let s = scan.iter().find(|s| s["degree"] == d).unwrap();
                let cols = s["columns"].as_f64().unwrap();
                let ops = s["closure_word_ops"].as_f64().unwrap().max(1.0);
                c.ratios.push(ops.log2() / cols.log2());
            }
            _ if at4.is_some_and(|s| s["saturated"] == true) => {
                c.above += 1;
                c.degrees.push(">4".into());
            }
            _ => {
                c.unmeasured += 1;
                c.degrees.push(format!("to {}", v["measured_to"]));
            }
        }
    }
    println!("| m | (n, ℓ) | N | closure degree per draw | cell | XL refutes at 4 | log₂ ops ÷ log₂ cols | secs per draw |");
    println!("|--:|---|--:|---|---|--:|---|---|");
    let (mut flat_max, mut rise_min) = (None::<u64>, None::<u64>);
    let mut q5 = 0;
    let mut all_ratios = Vec::new();
    for ((m, n, ell), c) in &cells {
        let n_vars = if c.n_vars > 0 {
            c.n_vars
        } else {
            (m - 2) * n + m * ell
        };
        let verdict = if c.above > 0 {
            rise_min = Some(rise_min.map_or(n_vars, |r| r.min(n_vars)));
            "rises"
        } else if c.flat >= 3 {
            flat_max = Some(flat_max.map_or(n_vars, |f| f.max(n_vars)));
            "flat"
        } else {
            "unmeasured"
        };
        q5 += c.xl_at_4;
        all_ratios.extend(c.ratios.iter().copied());
        let r: Vec<String> = c.ratios.iter().map(|x| format!("{x:.2}")).collect();
        let t: Vec<String> = c.secs.iter().map(|x| format!("{x:.0}")).collect();
        println!(
            "| {m} | ({n}, {ell}) | {n_vars} | {} | {verdict} | {} | {} | {} |",
            c.degrees.join(" "),
            c.xl_at_4,
            r.join(" "),
            t.join(" ")
        );
    }
    println!();
    match (rise_min, flat_max) {
        (Some(y), _) => println!("Study: rises at N = {y}"),
        (None, Some(x)) => println!("Study: flat to N = {x}"),
        (None, None) => println!("Study: nothing measured"),
    }
    println!("Q5: draws where XL refutes at degree 4: {q5}");
    let (lo, hi) = all_ratios
        .iter()
        .fold((f64::MAX, f64::MIN), |(a, b), &x| (a.min(x), b.max(x)));
    println!(
        "Q6: log₂ ops ÷ log₂ cols over {} refuting draws: {lo:.2}–{hi:.2} (registered band 2.0–2.4)",
        all_ratios.len()
    );
    println!(
        "Soundness control: {sat_draws} satisfiable draws, {sat_bad} refuted, {sat_unmeasured} unmeasured"
    );
}

/// Score `research/dreg_closure_specialise_20261017` (its PREREGISTRATION.md):
/// one `dreg_closure --fix` output per `<cell>.d<K>.<arm>.jsonl`.  Per run the
/// closure refutes at ≤ 4, does not (degree 4 saturated), or is unmeasured
/// (empty file).  Controls are valid when arm B refutes on ≥ 7 of the 8 draws
/// and arm C does not refute on ≥ 7; then arm A (A0 and A1) favours H_sum at
/// ≥ 14 of 16 refuting, H_max at ≤ 2, and is mixed otherwise.
fn specialise(paths: &[String]) {
    // (cell, draw) -> arm -> outcome
    let mut grid: BTreeMap<(String, u64), BTreeMap<String, &'static str>> = BTreeMap::new();
    for path in paths {
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .trim_end_matches(".jsonl")
            .to_string();
        let parts: Vec<&str> = name.split('.').collect();
        assert_eq!(parts.len(), 3, "{path}: want <cell>.d<K>.<arm>.jsonl");
        let draw: u64 = parts[1].trim_start_matches('d').parse().expect("d<K>");
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let outcome = match text.lines().find(|l| !l.trim().is_empty()) {
            None => "unmeasured",
            Some(line) => {
                let v: Value = serde_json::from_str(line).expect("json");
                match v["closure_degree"].as_u64() {
                    Some(d) if d <= 4 => "refutes",
                    _ => {
                        let sat4 = v["scan"].as_array().unwrap().iter().any(|s| {
                            s["degree"] == 4
                                && s["saturated"] == true
                                && s["closure_refuted"] == false
                        });
                        if sat4 {
                            "does not"
                        } else {
                            "unmeasured"
                        }
                    }
                }
            }
        };
        grid.entry((parts[0].to_string(), draw))
            .or_default()
            .insert(parts[2].to_string(), outcome);
    }
    let arms = ["A0", "A1", "B", "C"];
    println!("| cell | draw | A0 (6,7,7) | A1 (6c,7,7) | B (6,6,6) | C (null) |");
    println!("|---|--:|---|---|---|---|");
    let mut count: BTreeMap<&str, (usize, usize, usize)> = BTreeMap::new();
    for ((cell, draw), row) in &grid {
        let cells: Vec<&str> = arms
            .iter()
            .map(|a| *row.get(*a).unwrap_or(&"missing"))
            .collect();
        for (a, o) in arms.iter().zip(&cells) {
            let e = count.entry(a).or_default();
            match *o {
                "refutes" => e.0 += 1,
                "does not" => e.1 += 1,
                _ => e.2 += 1,
            }
        }
        println!("| {cell} | {draw} | {} |", cells.join(" | "));
    }
    println!();
    for a in arms {
        let (r, n, u) = count.get(a).copied().unwrap_or_default();
        println!("{a}: {r} refute at 4, {n} do not, {u} unmeasured");
    }
    let (b_ref, _, _) = count.get("B").copied().unwrap_or_default();
    let (_, c_not, _) = count.get("C").copied().unwrap_or_default();
    let valid = b_ref >= 7 && c_not >= 7;
    println!(
        "Controls: B refutes on {b_ref} of 8 (need ≥ 7), C does not refute on {c_not} of 8 (need ≥ 7) → {}",
        if valid { "valid" } else { "NOT valid" }
    );
    let a_ref = count.get("A0").map_or(0, |c| c.0) + count.get("A1").map_or(0, |c| c.0);
    if valid {
        let v = if a_ref >= 14 {
            "H_sum favoured"
        } else if a_ref <= 2 {
            "H_max favoured"
        } else {
            "mixed"
        };
        println!("Arm A: {a_ref} of 16 refute at 4 → {v}");
    } else {
        println!("Arm A: {a_ref} of 16 refute at 4 → uninformative for H_sum vs H_max (controls not valid)");
    }
}

/// Score `research/dreg_closure_slope_20261018` (its PREREGISTRATION.md):
/// one `dreg_closure --sparse D --draw K` output per `<cell>.d<K>.D<d>.jsonl`.
/// An empty file is a run the memory or time limit killed; `"capped":true` is
/// a run that hit the entry cap; both are unmeasured, never evidence.  Per
/// draw: the first scanned degree that refutes, `≥ D+1` when degree `D`
/// saturated without refuting and nothing higher was measured, or
/// unmeasured.  Per cell: resolved at the median of ≥ 3 exact degrees, or
/// bounded below.  Study: the least-squares slope of cell degree against
/// `m·ℓ` over the resolved cells with the committed degree-4 anchors, the
/// registered verdict, P1–P5, and the derived distance table.
fn slope(paths: &[String]) {
    // (m, n, ℓ) → draw → degree → record
    type Rec = (
        bool, /*measured*/
        bool, /*refuted*/
        bool, /*saturated*/
        f64,  /*secs*/
        bool, /*capped*/
    );
    let mut cells: BTreeMap<(u64, u64, u64), BTreeMap<u64, BTreeMap<u64, Rec>>> = BTreeMap::new();
    let mut kinds: BTreeMap<(u64, u64, u64), String> = BTreeMap::new();
    let (mut check_runs, mut check_measured, mut check_refuted) = (0usize, 0usize, 0usize);
    let (mut ctrl_runs, mut ctrl_bad, mut ctrl_unmeasured) = (0usize, 0usize, 0usize);
    for path in paths {
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let line = text.lines().find(|l| !l.trim().is_empty());
        // `<kind>-m<M>-<n>-<ℓ>.d<K>.D<d>.jsonl`
        let parse_name = || -> Option<(String, u64, u64, u64, u64, u64)> {
            let (kind, rest) = name.split_once("-m")?;
            let mut it = rest.split(['-', '.']);
            let m = it.next()?.parse().ok()?;
            let n = it.next()?.parse().ok()?;
            let ell = it.next()?.parse().ok()?;
            let k = it.next()?.strip_prefix('d')?.parse().ok()?;
            let d = it.next()?.strip_prefix('D')?.parse().ok()?;
            Some((kind.to_string(), m, n, ell, k, d))
        };
        let Some((kind, m, n, ell, k, d)) = parse_name() else {
            panic!("{path}: unparsable file name");
        };
        let rec: Rec = match line {
            None => (false, false, false, 0.0, false),
            Some(l) => {
                let v: Value = serde_json::from_str(l).expect("json");
                assert_eq!(v["degree"].as_u64(), Some(d), "{path}: degree");
                assert_eq!(v["m"].as_u64(), Some(m), "{path}: m");
                let capped = v["capped"].as_bool().unwrap_or(false);
                (
                    !capped,
                    v["closure_refuted"].as_bool().unwrap_or(false),
                    v["saturated"].as_bool().unwrap_or(false),
                    v["secs"].as_f64().unwrap_or(0.0),
                    capped,
                )
            }
        };
        match kind.as_str() {
            "check" => {
                check_runs += 1;
                check_measured += rec.0 as usize;
                check_refuted += (rec.0 && rec.1) as usize;
            }
            "control" => {
                ctrl_runs += 1;
                ctrl_bad += (rec.0 && rec.1) as usize;
                ctrl_unmeasured += (!rec.0) as usize;
            }
            _ => {
                cells
                    .entry((m, n, ell))
                    .or_default()
                    .entry(k)
                    .or_default()
                    .insert(d, rec);
                kinds.insert((m, n, ell), kind.clone());
            }
        }
    }

    // Committed anchors: dense closure 4 on every draw (dreg_closure_scale_20261016, dreg_closure_mutantxl_20261016).
    let anchors: [(u64, u64, u64); 8] = [
        (4, 12, 3),
        (4, 14, 3),
        (3, 11, 5),
        (3, 13, 5),
        (3, 15, 5),
        (4, 13, 4),
        (3, 16, 6),
        (3, 18, 6),
    ];
    println!("| m | (n, ℓ) | m·ℓ | N | degree per draw | cell | secs per draw |");
    println!("|--:|---|--:|--:|---|---|---|");
    for (m, n, ell) in anchors {
        println!(
            "| {m} | ({n}, {ell}) | {} | {} | 4 4 4 4 (committed, dense) | resolved at 4 | — |",
            m * ell,
            m * ell + (m - 2) * n
        );
    }
    let mut points: Vec<(f64, f64)> = anchors
        .iter()
        .map(|&(m, _, l)| ((m * l) as f64, 4.0))
        .collect();
    let mut first_six: Option<u64> = None;
    let mut p1_ok = 0usize;
    let mut p1_total = 0usize;
    for ((m, n, ell), draws) in &cells {
        let mut exact: Vec<u64> = Vec::new();
        let mut bounds: Vec<u64> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        let mut secs: Vec<String> = Vec::new();
        // The committed m·ℓ = 21 cells start their scan at 5: the dense closure
        // at 4 is committed (saturated, not refuting). Fresh cells start at 4.
        let scan_start = if kinds.get(&(*m, *n, *ell)).is_some_and(|k| k == "committed") {
            5
        } else {
            4
        };
        for degs in draws.values() {
            let mut label = String::from("unmeasured");
            let mut best_bound = 0u64;
            let mut found: Option<u64> = None;
            let mut total = 0.0;
            for (&d, &(measured, refuted, saturated, s, _)) in degs {
                total += s;
                if measured && refuted {
                    found = Some(d);
                    break;
                }
                if measured && saturated {
                    best_bound = best_bound.max(d + 1);
                }
            }
            // "refutes and no lower scanned degree did": every lower degree must
            // be measured and saturated without refuting, else the degree is only
            // an upper bound and the draw is not exact.
            let gap = found.is_some_and(|d| {
                (scan_start..d).any(|dd| {
                    !degs
                        .get(&dd)
                        .is_some_and(|&(measured, refuted, saturated, _, _)| {
                            measured && saturated && !refuted
                        })
                })
            });
            if let (Some(d), true) = (found, gap) {
                label = format!("≤{d} (lower degree unmeasured)");
            } else if let Some(d) = found {
                exact.push(d);
                label = d.to_string();
            } else if best_bound > 0 {
                bounds.push(best_bound);
                label = format!("≥{best_bound}");
            }
            labels.push(label);
            secs.push(format!("{total:.0}"));
        }
        if m * ell == 20 || m * ell == 21 {
            // A draw bounded below by 6 or more is "not 5": a violation.
            p1_total += exact.len() + bounds.len();
            p1_ok += exact.iter().filter(|&&d| d == 5).count();
        }
        let verdict = if exact.len() >= 3 {
            let mut e = exact.clone();
            e.sort_unstable();
            let med = e[e.len() / 2];
            points.push(((m * ell) as f64, med as f64));
            if med >= 6 {
                first_six = Some(first_six.map_or(m * ell, |f| f.min(m * ell)));
            }
            format!("resolved at {med}")
        } else {
            let mut lows: Vec<u64> = exact
                .iter()
                .copied()
                .chain(bounds.iter().copied())
                .collect();
            lows.sort_unstable();
            if lows.len() >= 3 {
                format!("bounded below by {}", lows[lows.len() - 3])
            } else {
                "unmeasured".to_string()
            }
        };
        println!(
            "| {m} | ({n}, {ell}) | {} | {} | {} | {verdict} | {} |",
            m * ell,
            m * ell + (m - 2) * n,
            labels.join(" "),
            secs.join(" ")
        );
    }
    println!();
    // Least squares over resolved cells (anchors included), one point per cell.
    let k = points.len() as f64;
    let (sx, sy) = points
        .iter()
        .fold((0.0, 0.0), |(a, b), &(x, y)| (a + x, b + y));
    let (mx, my) = (sx / k, sy / k);
    let sxx: f64 = points.iter().map(|&(x, _)| (x - mx) * (x - mx)).sum();
    let sxy: f64 = points.iter().map(|&(x, y)| (x - mx) * (y - my)).sum();
    let s = if sxx > 0.0 { sxy / sxx } else { f64::NAN };
    let fitted = points.iter().filter(|&&(x, _)| x > 18.0).count();
    println!(
        "Slope s = {s:.4} degrees per summand unknown (m·ℓ), least squares over {} cells ({} anchors at degree 4, {fitted} measured here)",
        points.len(),
        anchors.len()
    );
    let verdict = if fitted == 0 {
        "nothing resolved above the anchors: undecided"
    } else if s < 0.03 {
        "grows slowly (s < 0.03): compute the crossover"
    } else if s >= 0.10 {
        "closed at every size (s ≥ 0.10)"
    } else {
        "undecided (0.03 ≤ s < 0.10)"
    };
    println!("Registered verdict: {verdict}");
    println!(
        "P1 (every resolved draw at m·ℓ = 20, 21 reads 5): {p1_ok} of {p1_total}{}",
        if p1_total > 0 && p1_ok == p1_total {
            " → holds"
        } else if p1_total > 0 {
            " → refuted"
        } else {
            " → unmeasured"
        }
    );
    match first_six {
        Some(x) => println!("P2 (a resolved cell with m·ℓ ≤ 30 reads 6): holds, first at m·ℓ = {x}"),
        None => println!("P2 (a resolved cell with m·ℓ ≤ 30 reads 6): refuted on the resolved cells (or unmeasured)"),
    }
    println!(
        "P3 (s ≥ 0.10): {}",
        if fitted == 0 {
            "unmeasured".to_string()
        } else if s >= 0.10 {
            "holds".to_string()
        } else {
            format!("refuted (s = {s:.4})")
        }
    );
    println!(
        "P4 (sparse degree-4 replays of committed dense refutations refute): {check_refuted} of {check_measured} measured ({check_runs} runs){}",
        if check_runs == 0 {
            " → not run"
        } else if check_measured > check_refuted {
            " → FAILS: instrument failure"
        } else if check_measured < check_runs {
            " → incomplete (capped or killed runs are unmeasured)"
        } else {
            " → holds"
        }
    );
    println!(
        "P5 (no satisfiable draw refuted): {ctrl_runs} control runs, {ctrl_bad} refuted, {ctrl_unmeasured} unmeasured{}",
        if ctrl_bad > 0 { " → FAILS: instrument failure" } else { "" }
    );

    // Derived distance table (cost model of PREREGISTRATION.md, w = 1 column touches):
    //   rho = sqrt(pi r / (4·2n)), r = 2^n / cofactor; relations 2^ℓ/(2n); attempts max(1, m!·2^{n−mℓ});
    //   oracle ≥ C(N, ≤D), N = m·ℓ + (m−2)·n (chained) — and N = m·ℓ for the eliminated system.
    println!();
    println!("Derived (cost model, w = 1; extrapolated degree 4 + s·(m·ℓ − 18) at ℓ = ⌈n/m⌉, read from the fit over m·ℓ ≤ 30):");
    println!("| n | m | ℓ | log₂ budget per solve | admissible D (chained N) | admissible D (N = m·ℓ) | extrapolated degree | gap, degrees (chained) |");
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|");
    let log2_cols = |nv: u64, d: u64| -> f64 {
        let mut total = 0f64;
        let mut binom = 1f64;
        for i in 0..=d {
            if i > nv {
                break;
            }
            total += binom;
            binom = binom * (nv - i) as f64 / (i + 1) as f64;
        }
        total.log2()
    };
    let admissible = |nv: u64, budget: f64| -> Option<u64> {
        (2..=40u64).rev().find(|&d| log2_cols(nv, d) <= budget)
    };
    for &(n, cof) in &[
        (73u64, 4u64),
        (109, 4),
        (127, 4),
        (131, 4),
        (163, 2),
        (233, 4),
    ] {
        for m in 3..=6u64 {
            let ell = n.div_ceil(m);
            let rho = 0.5
                * ((std::f64::consts::PI / (8.0 * n as f64)).log2() + n as f64
                    - (cof as f64).log2());
            let fact: f64 = (1..=m).map(|i| i as f64).product();
            let attempts = (fact.log2() + n as f64 - (m * ell) as f64).max(0.0);
            let relations = ell as f64 - (2.0 * n as f64).log2();
            let budget = rho - relations - attempts;
            let n_ch = m * ell + (m - 2) * n;
            let a_ch = admissible(n_ch, budget);
            let a_el = admissible(m * ell, budget);
            let extrap = 4.0 + s.max(0.0) * ((m * ell) as f64 - 18.0);
            let gap = a_ch.map_or("—".to_string(), |a| format!("{:+.1}", extrap - a as f64));
            println!(
                "| {n} | {m} | {ell} | {budget:.1} | {} | {} | {extrap:.1} | {gap} |",
                a_ch.map_or("none".to_string(), |a| a.to_string()),
                a_el.map_or("none".to_string(), |a| a.to_string())
            );
        }
    }
    if s.is_finite() && s > 0.0 {
        // The crossover: the first n at which the extrapolated degree fits the chained budget, per m.
        for m in 3..=6u64 {
            let hit = (40..=20000u64).find(|&n| {
                let ell = n.div_ceil(m);
                let rho = 0.5 * ((std::f64::consts::PI / (8.0 * n as f64)).log2() + n as f64 - 2.0);
                let fact: f64 = (1..=m).map(|i| i as f64).product();
                let attempts = (fact.log2() + n as f64 - (m * ell) as f64).max(0.0);
                let budget = rho - (ell as f64 - (2.0 * n as f64).log2()) - attempts;
                let extrap = (4.0 + s * ((m * ell) as f64 - 18.0)).ceil() as u64;
                log2_cols(m * ell + (m - 2) * n, extrap) <= budget
            });
            match hit {
                Some(n) => println!("Crossover under the fit, m = {m}: first n = {n} (derived; the fit is read beyond its range)"),
                None => println!("Crossover under the fit, m = {m}: none below n = 20000 (derived)"),
            }
        }
    }
}

/// `target_free FILES…`: the registered reading of the target-free study
/// (`research/target_free_relations_20261018/PREREGISTRATION.md`) from
/// `dreg_target_free` records.  A record is *measured* when F4 finished
/// (neither `timed_out` nor `oversize`); an unfinished record still bounds
/// the solving degree from below by the highest degree that added a basis
/// element (`solving_degree`), and that bound is used only to rule *lower*
/// out, never to read a value.  Per (cell, basis, twists): with ≥ 3 measured
/// draws on both arms the medians decide lower / equal / higher; otherwise
/// the cell is unmeasured by the registered rule, and the reading of the
/// draws that exist is printed beside it, labelled.
fn target_free(paths: &[String]) {
    // (n, ℓ, basis, twists) → arm → draw → (measured, solving_degree, degree_reached, satisfiable, secs)
    type Rec = (bool, u64, u64, bool, f64);
    let mut cells: BTreeMap<(u64, u64, String, String), BTreeMap<String, BTreeMap<u64, Rec>>> =
        BTreeMap::new();
    let mut files = 0usize;
    for path in paths {
        let text = std::fs::read_to_string(path).unwrap_or_default();
        files += 1;
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            let key = (
                v["n"].as_u64().expect("n"),
                v["ell"].as_u64().expect("ell"),
                v["basis"].as_str().expect("basis").to_string(),
                v["twists"]
                    .as_array()
                    .expect("twists")
                    .iter()
                    .map(|t| t.as_u64().unwrap().to_string())
                    .collect::<Vec<_>>()
                    .join(""),
            );
            let arm = v["arm"].as_str().expect("arm").to_string();
            let draw = v["draw"].as_u64().expect("draw");
            let measured = !v["timed_out"].as_bool().unwrap_or(false)
                && !v["oversize"].as_bool().unwrap_or(false);
            let rec: Rec = (
                measured,
                v["solving_degree"].as_u64().unwrap_or(0),
                v["degree_reached"].as_u64().unwrap_or(0),
                v["basis_len"].as_u64().unwrap_or(0) != 1,
                v["secs"].as_f64().unwrap_or(0.0),
            );
            cells
                .entry(key)
                .or_default()
                .entry(arm)
                .or_default()
                .insert(draw, rec);
        }
    }
    println!(
        "Records: {files} files, {} (cell, basis, twists) groups.\n",
        cells.len()
    );
    println!("| (n, ℓ) | basis | twists | target-free measured / draws | target-free solving (measured; bounds ≥) | target m = 3 measured / draws | target solving (measured) | unsat. target draws | registered reading | single-arm reading (fewer than 3 draws) |");
    println!("|---|---|---|--:|---|--:|---|--:|---|---|");
    let median = |xs: &[u64]| -> Option<u64> {
        if xs.is_empty() {
            return None;
        }
        let mut v = xs.to_vec();
        v.sort_unstable();
        Some(v[v.len() / 2])
    };
    // (n, ℓ, basis) → twists → (registered reading, bounded "not lower")
    let mut readings: BTreeMap<(u64, u64, String), BTreeMap<String, (String, bool)>> =
        BTreeMap::new();
    let (mut p4_total, mut p4_ok) = (0usize, 0usize);
    let (mut p4_sat_total, mut p4_sat_ok) = (0usize, 0usize);
    for ((n, ell, basis, tw), arms) in &cells {
        let empty = BTreeMap::new();
        let tf = arms.get("target_free").unwrap_or(&empty);
        let tg = arms.get("target_m3").unwrap_or(&empty);
        let tf_meas: Vec<u64> = tf.values().filter(|r| r.0).map(|r| r.1).collect();
        let tf_bound: Vec<u64> = tf.values().filter(|r| !r.0).map(|r| r.1).collect();
        let tg_meas: Vec<u64> = tg.values().filter(|r| r.0).map(|r| r.1).collect();
        for r in tg.values().filter(|r| r.0) {
            if n == &16 || n == &18 {
                if r.3 {
                    p4_sat_total += 1;
                    p4_sat_ok += (r.1 == 4) as usize;
                } else {
                    p4_total += 1;
                    p4_ok += (r.1 == 4) as usize;
                }
            }
        }
        let unsat = tg.values().filter(|r| r.0 && !r.3).count();
        let fmt = |m: &[u64], b: &[u64]| -> String {
            let mut s = m.iter().map(u64::to_string).collect::<Vec<_>>().join(", ");
            if !b.is_empty() {
                if !s.is_empty() {
                    s.push_str("; ");
                }
                s.push_str("bounds ≥ ");
                s.push_str(&b.iter().map(u64::to_string).collect::<Vec<_>>().join(", "));
            }
            if s.is_empty() {
                s.push('—');
            }
            s
        };
        let tg_med = median(&tg_meas);
        let registered = if tf_meas.len() >= 3 && tg_meas.len() >= 3 {
            let (a, b) = (median(&tf_meas).unwrap(), tg_med.unwrap());
            if a < b {
                "lower"
            } else if a == b {
                "equal"
            } else {
                "higher"
            }
        } else {
            "unmeasured"
        }
        .to_string();
        // Bounded reading from every draw: "not lower" when every target-free draw
        // (measured value or lower bound) is at least the target arm's median.
        let not_lower = tg_med.is_some_and(|b| !tf.is_empty() && tf.values().all(|r| r.1 >= b));
        let single = match (tg_med, median(&tf_meas)) {
            (Some(b), Some(a)) if registered == "unmeasured" => {
                let w = if a < b {
                    "lower"
                } else if a == b {
                    "equal"
                } else {
                    "higher"
                };
                format!(
                    "{w} ({} vs {} measured draws)",
                    tf_meas.len(),
                    tg_meas.len()
                )
            }
            (Some(_), None) if not_lower => {
                "not lower (bounded: every target-free draw ≥ target median)".to_string()
            }
            (Some(_), None) => "no measured target-free draw".to_string(),
            _ => "—".to_string(),
        };
        println!(
            "| ({n}, {ell}) | {basis} | [{}] | {} / {} | {} | {} / {} | {} | {unsat} | {registered} | {single} |",
            tw.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(","),
            tf_meas.len(),
            tf.len(),
            fmt(&tf_meas, &tf_bound),
            tg_meas.len(),
            tg.len(),
            fmt(&tg_meas, &[]),
        );
        readings
            .entry((*n, *ell, basis.clone()))
            .or_default()
            .insert(tw.clone(), (registered, not_lower));
    }
    println!();
    // Study verdict.
    let lower_cells = |basis: &str| -> Vec<(u64, u64)> {
        readings
            .iter()
            .filter(|((_, _, b), tws)| b == basis && tws.values().any(|(r, _)| r == "lower"))
            .map(|((n, ell, _), _)| (*n, *ell))
            .collect()
    };
    let gp_lower = lower_cells("gp");
    let random_lower = lower_cells("random");
    let gp_measured_cells = readings
        .iter()
        .filter(|((_, _, b), tws)| b == "gp" && tws.values().any(|(r, _)| r != "unmeasured"))
        .count();
    let gp_cells = readings.keys().filter(|(_, _, b)| b == "gp").count();
    let gp_not_lower_bounded = readings
        .iter()
        .filter(|((_, _, b), tws)| b == "gp" && tws.values().all(|(_, nl)| *nl))
        .count();
    let verdict = if gp_lower
        .iter()
        .filter(|c| !random_lower.contains(c))
        .count()
        >= 2
    {
        "lead survives"
    } else if gp_lower.is_empty() && gp_measured_cells == gp_cells && gp_cells > 0 {
        "killed"
    } else if gp_lower.is_empty() && gp_not_lower_bounded == gp_cells && gp_cells > 0 {
        "killed on the bounded reading (no cell reads lower for gp on any draw, measured or bounded); undecided by the registered rule (fewer than 3 measured draws per arm)"
    } else {
        "undecided"
    };
    println!(
        "Registered verdict: {verdict}. gp cells reading lower: {:?}; random cells reading lower: {:?}; gp cells with a registered reading: {gp_measured_cells} of {gp_cells}; gp cells bounded not-lower on every twist: {gp_not_lower_bounded} of {gp_cells}.",
        gp_lower, random_lower
    );
    let outcome = |basis: &str, tw: &str, pred: &dyn Fn(&str) -> bool, name: &str| {
        let mut rows: Vec<String> = Vec::new();
        let (mut ok, mut bad, mut unm) = (0, 0, 0);
        for ((n, ell, b), tws) in &readings {
            if b != basis {
                continue;
            }
            for (t, (r, nl)) in tws {
                if tw != "*" && t != tw {
                    continue;
                }
                if r == "unmeasured" {
                    unm += 1;
                    rows.push(format!(
                        "({n},{ell})[{t}]: unmeasured{}",
                        if *nl { ", bounded not lower" } else { "" }
                    ));
                } else if pred(r) {
                    ok += 1;
                    rows.push(format!("({n},{ell})[{t}]: {r}"));
                } else {
                    bad += 1;
                    rows.push(format!("({n},{ell})[{t}]: {r} (against)"));
                }
            }
        }
        let verdict = if bad > 0 {
            "fails"
        } else if ok > 0 && unm == 0 {
            "holds"
        } else if ok > 0 {
            "holds on the measured cells, others unmeasured"
        } else {
            "unmeasured"
        };
        println!("{name}: {verdict} — {}", rows.join("; "));
    };
    outcome(
        "gp",
        "0000",
        &|r| r == "higher" || r == "equal",
        "P1 (gp plain reads higher or equal at every measured cell)",
    );
    outcome(
        "gp",
        "0123",
        &|r| r == "equal",
        "P2 (gp twisted reads equal at every measured cell)",
    );
    outcome(
        "random",
        "*",
        &|r| r != "lower",
        "P3 (random never reads lower)",
    );
    println!(
        "P4 (target arm F4 solving degree = 4 on unsatisfiable draws at (16,4), (18,4)): {} — {p4_ok} of {p4_total} unsatisfiable measured target draws read 4{}",
        if p4_total == 0 { "unmeasured" } else if p4_ok == p4_total { "holds" } else { "fails" },
        if p4_sat_total > 0 { format!("; satisfiable target draws (outside P4): {p4_sat_ok} of {p4_sat_total} read 4") } else { String::new() }
    );
}

/// `s4_free FILES…`: the registered reading of the eliminated target-free
/// `S₄` study (`research/target_free_s4_20261018/PREREGISTRATION.md`) from
/// `dreg_eliminated_s4` records.  A record is measured when F4 finished.
/// Per (cell, basis, twists): the target-free value is the single
/// deterministic draw on `gp` and the median of ≥ 3 measured draws on
/// `random`; the target value is the median of ≥ 3 measured target draws;
/// lower / equal / higher, else unmeasured (with the bounded reading beside
/// it).  Also the top-form deficiency (`top_forms − top_rank`) per arm and
/// the within-target-free shape reading (gp against the random median).
fn s4_free(paths: &[String]) {
    // (n, ℓ, basis, twists) → arm → draw → (measured, solving, deficiency, top_degree)
    type Rec = (bool, u64, u64, u64);
    let mut cells: BTreeMap<(u64, u64, String, String), BTreeMap<String, BTreeMap<u64, Rec>>> =
        BTreeMap::new();
    for path in paths {
        let text = std::fs::read_to_string(path).unwrap_or_default();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            let tw = v["twists"]
                .as_array()
                .expect("twists")
                .iter()
                .map(|t| t.as_u64().unwrap().to_string())
                .collect::<Vec<_>>()
                .join("");
            let key = (
                v["n"].as_u64().expect("n"),
                v["ell"].as_u64().expect("ell"),
                v["basis"].as_str().expect("basis").to_string(),
                tw,
            );
            let measured = !v["timed_out"].as_bool().unwrap_or(false)
                && !v["oversize"].as_bool().unwrap_or(false);
            let rec: Rec = (
                measured,
                v["solving_degree"].as_u64().unwrap_or(0),
                v["top_forms"].as_u64().unwrap_or(0) - v["top_rank"].as_u64().unwrap_or(0),
                v["top_degree"].as_u64().unwrap_or(0),
            );
            cells
                .entry(key)
                .or_default()
                .entry(v["arm"].as_str().expect("arm").to_string())
                .or_default()
                .insert(v["draw"].as_u64().expect("draw"), rec);
        }
    }
    let median = |xs: &[u64]| -> Option<u64> {
        let mut v = xs.to_vec();
        v.sort_unstable();
        v.get(v.len() / 2).copied()
    };
    let cmp = |a: u64, b: u64| -> &'static str {
        if a < b {
            "lower"
        } else if a == b {
            "equal"
        } else {
            "higher"
        }
    };
    let list = |rs: &BTreeMap<u64, Rec>, f: &dyn Fn(&Rec) -> u64| -> String {
        let s: Vec<String> = rs
            .values()
            .map(|r| {
                if r.0 {
                    f(r).to_string()
                } else {
                    format!("≥{}", r.1)
                }
            })
            .collect();
        if s.is_empty() {
            "—".into()
        } else {
            s.join(", ")
        }
    };
    println!("| (n, ℓ) | basis | twists | target-free solving (≥ = unfinished) | target-free top deficiency | target solving | target top deficiency | target-free value | target median | registered reading | bounded reading |");
    println!("|---|---|---|---|---|---|---|--:|--:|---|---|");
    // (n, ℓ, twists) → basis → (registered reading, target-free value)
    let mut by_cell: BTreeMap<
        (u64, u64, String),
        BTreeMap<String, (String, Option<u64>, Vec<u64>)>,
    > = BTreeMap::new();
    for ((n, ell, basis, tw), arms) in &cells {
        let empty = BTreeMap::new();
        let tf = arms.get("target_free_s4").unwrap_or(&empty);
        let tg = arms.get("target_s4").unwrap_or(&empty);
        let tf_meas: Vec<u64> = tf.values().filter(|r| r.0).map(|r| r.1).collect();
        let tg_meas: Vec<u64> = tg.values().filter(|r| r.0).map(|r| r.1).collect();
        let need = if basis == "gp" { 1 } else { 3 };
        let tf_val = (tf_meas.len() >= need).then(|| median(&tf_meas)).flatten();
        let tg_val = (tg_meas.len() >= 3).then(|| median(&tg_meas)).flatten();
        let reading = match (tf_val, tg_val) {
            (Some(a), Some(b)) => cmp(a, b).to_string(),
            _ => "unmeasured".to_string(),
        };
        let bounded = match median(&tg_meas) {
            Some(b) if !tf.is_empty() && tf.values().all(|r| r.1 >= b) => "not lower".to_string(),
            Some(_) if tf.is_empty() => "—".to_string(),
            Some(_) => "open".to_string(),
            None => "—".to_string(),
        };
        let defs: Vec<u64> = tf.values().map(|r| r.2).collect();
        println!(
            "| ({n}, {ell}) | {basis} | [{}] | {} | {} | {} | {} | {} | {} | {reading} | {bounded} |",
            tw.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(","),
            list(tf, &|r| r.1),
            if defs.is_empty() { "—".into() } else { defs.iter().map(u64::to_string).collect::<Vec<_>>().join(", ") },
            list(tg, &|r| r.1),
            tg.values().map(|r| r.2.to_string()).collect::<Vec<_>>().join(", "),
            tf_val.map_or("—".into(), |v| v.to_string()),
            tg_val.map_or("—".into(), |v| v.to_string()),
        );
        by_cell
            .entry((*n, *ell, tw.clone()))
            .or_default()
            .insert(basis.clone(), (reading, tf_val, defs));
    }
    println!();
    let lower = |basis: &str| -> Vec<(u64, u64, String)> {
        by_cell
            .iter()
            .filter(|(_, b)| b.get(basis).is_some_and(|x| x.0 == "lower"))
            .map(|(k, _)| k.clone())
            .collect()
    };
    let (gl, rl) = (lower("gp"), lower("random"));
    let gp_read = by_cell
        .values()
        .filter(|b| b.get("gp").is_some_and(|x| x.0 != "unmeasured"))
        .count();
    let survives = gl.iter().filter(|c| !rl.contains(c)).count() >= 2;
    let verdict = if survives {
        "lead survives"
    } else if gl.is_empty() && gp_read > 0 {
        "killed"
    } else {
        "undecided"
    };
    println!("Registered verdict (primary): {verdict}. gp (cell, twists) reading lower: {gl:?}; random reading lower: {rl:?}; gp readings measured: {gp_read}.");
    // Secondary: shape effect inside the target-free arm (gp value one below the random median).
    let mut shape = Vec::new();
    for ((n, ell, tw), b) in &by_cell {
        if let (Some((_, Some(g), _)), Some((_, Some(r), _))) = (b.get("gp"), b.get("random")) {
            shape.push(format!(
                "({n},{ell})[{tw}]: gp {g} vs random {r} → {}",
                if g < r { "shape lower" } else { cmp(*g, *r) }
            ));
        }
    }
    let shape_lower = shape.iter().filter(|s| s.ends_with("shape lower")).count();
    println!(
        "Secondary (shape inside target-free): {} — {}",
        if shape_lower >= 2 {
            "shape effect at ≥ 2 cells"
        } else if shape.is_empty() {
            "unmeasured"
        } else {
            "fewer than 2 cells"
        },
        if shape.is_empty() {
            "no cell with both values".into()
        } else {
            shape.join("; ")
        }
    );
    // Predictions.
    let mut p1 = (0, 0);
    let mut p2 = (0, 0, 0);
    for b in by_cell.values() {
        for (basis, (r, _, _)) in b {
            let _ = basis;
            if r != "unmeasured" {
                p1.0 += 1;
                p1.1 += (r == "higher") as usize;
            }
        }
        if let (Some((_, _, g)), Some((_, _, r))) = (b.get("gp"), b.get("random")) {
            if let (Some(gd), Some(rd)) = (g.first(), median(r)) {
                p2.0 += 1;
                p2.1 += (*gd > rd) as usize;
                p2.2 += (*gd == rd) as usize;
            }
        }
    }
    println!(
        "P1 (target-free reads higher than target at every measured (cell, basis, twists)): {} — {} of {} higher",
        if p1.0 == 0 { "unmeasured" } else if p1.1 == p1.0 { "holds" } else { "fails" },
        p1.1,
        p1.0
    );
    println!(
        "P2 (gp target-free top deficiency above the random median at every cell with both): {} — {} of {} above, {} equal",
        if p2.0 == 0 { "unmeasured" } else if p2.1 == p2.0 { "holds" } else { "fails" },
        p2.1,
        p2.0,
        p2.2
    );
    println!(
        "P3 (secondary shape effect at ≥ 2 cells): {}",
        if shape_lower >= 2 {
            "holds"
        } else if shape.is_empty() {
            "unmeasured"
        } else {
            "fails"
        }
    );
    let twisted: Vec<String> = by_cell
        .iter()
        .filter(|((_, _, tw), _)| tw == "0123")
        .filter_map(|((n, ell, _), b)| {
            let g = b.get("gp")?.2.first()?;
            let plain = by_cell.get(&(*n, *ell, "0000".to_string()))?;
            let r = median(&plain.get("random")?.2)?;
            Some(format!(
                "({n},{ell}): gp twisted deficiency {g} vs random plain median {r} → {}",
                if *g <= r {
                    "at or below random"
                } else {
                    "above random"
                }
            ))
        })
        .collect();
    println!(
        "P4 (twisting [0,1,2,3] brings the gp deficiency to the random level or below): {} — {}",
        if twisted.is_empty() {
            "unmeasured"
        } else if twisted.iter().all(|t| t.ends_with("at or below random")) {
            "holds"
        } else {
            "fails"
        },
        if twisted.is_empty() {
            "no twisted gp record".into()
        } else {
            twisted.join("; ")
        }
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("score") => score(&args[1..]),
        Some("compare") if args.len() == 3 => compare(&args[1], &args[2]),
        Some("semireg") if args.len() >= 3 => semireg(args[1].parse().expect("m"), &args[2..]),
        Some("shape") if args.len() >= 3 => shape(args[1].parse().expect("m"), &args[2..]),
        Some("closure") => closure(&args[1..]),
        Some("scale") => scale(&args[1..]),
        Some("specialise") => specialise(&args[1..]),
        Some("slope") => slope(&args[1..]),
        Some("target_free") => target_free(&args[1..]),
        Some("s4_free") => s4_free(&args[1..]),
        _ => {
            eprintln!(
                "usage: dreg_score score FILES… | compare REF RERUN | semireg M n:ℓ… | shape M n:ℓ:D… | closure FILES… | scale FILES… | specialise FILES… | slope FILES… | target_free FILES… | s4_free FILES…"
            );
            std::process::exit(2);
        }
    }
}
