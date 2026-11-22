//! Tables, ratios, the pre-registered decision and the SVG figure for the lin21 wall-gap pilot.
//! Standard library only: `rustc -O summary.rs && ./summary OUTDIR TARGETDIR`.
//! Reads TARGETDIR/targets_l*.jsonl (the lin21 pilot's target files), OUTDIR/raw/targets_l*.jsonl
//! (extension cells, if any) and OUTDIR/raw/rows.jsonl (JSON lines printed by `l21fast run`).
//! Recomputes every agreement flag from the binary's own fields and writes OUTDIR/tables.md,
//! OUTDIR/results.json and OUTDIR/fig_wall_vs_l.svg.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;

fn raw<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = line[i..].trim_start();
    let end = if rest.starts_with('[') {
        rest.find(']')? + 1
    } else if rest.starts_with('"') {
        rest[1..].find('"')? + 2
    } else {
        rest.find(|c| c == ',' || c == '}').unwrap_or(rest.len())
    };
    Some(rest[..end].trim())
}
fn num(line: &str, key: &str) -> Option<f64> {
    raw(line, key)?.parse().ok()
}
fn boolean(line: &str, key: &str) -> Option<bool> {
    match raw(line, key)? {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
fn string(line: &str, key: &str) -> Option<String> {
    Some(raw(line, key)?.trim_matches('"').to_string())
}

struct Target {
    idx: u64,
    kind: String,
    oracle_sat: bool,
}

fn med(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = s.len() / 2;
    if s.len() % 2 == 1 { s[m] } else { 0.5 * (s[m - 1] + s[m]) }
}
fn lo(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::INFINITY, f64::min)
}
fn hi(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}
fn g(x: f64) -> String {
    if x.is_nan() {
        "n/a".into()
    } else if x == x.round() && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else if x.abs() >= 100.0 {
        format!("{x:.0}")
    } else if x.abs() >= 10.0 {
        format!("{x:.1}")
    } else if x.abs() >= 1.0 {
        format!("{x:.2}")
    } else {
        format!("{x:.4}")
    }
}
fn j(x: f64) -> String {
    if x.is_finite() { format!("{x}") } else { "null".into() }
}
/// least-squares slope of y against x
fn slope(pts: &[(f64, f64)]) -> f64 {
    let n = pts.len() as f64;
    if pts.len() < 2 {
        return f64::NAN;
    }
    let (mx, my) = (pts.iter().map(|p| p.0).sum::<f64>() / n, pts.iter().map(|p| p.1).sum::<f64>() / n);
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx) * (p.0 - mx)).sum();
    sxy / sxx
}

const ARMS: [&str; 5] = ["scan", "lin21", "lin21-fast", "lin21-fast-b", "lin21-prof"];
const MAIN_LS: [u64; 4] = [9, 10, 11, 12];

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let dir = a.get(1).expect("OUTDIR").clone();
    let tdir = a.get(2).expect("TARGETDIR").clone();
    let mut targets: BTreeMap<(u64, u64), Target> = BTreeMap::new();
    let mut target_lines: Vec<String> = Vec::new();
    for l in 3..=20u64 {
        for d in [&tdir, &format!("{dir}/raw")] {
            let Ok(txt) = fs::read_to_string(format!("{d}/targets_l{l}.jsonl")) else { continue };
            for line in txt.lines().filter(|x| !x.trim().is_empty()) {
                let key = (num(line, "l").unwrap() as u64, num(line, "x_r").unwrap() as u64);
                let t = Target {
                    idx: num(line, "idx").unwrap() as u64,
                    kind: string(line, "kind").unwrap(),
                    oracle_sat: boolean(line, "oracle_sat").unwrap(),
                };
                if targets.insert(key, t).is_none() {
                    target_lines.push(line.to_string());
                }
            }
        }
    }
    let txt = fs::read_to_string(format!("{dir}/raw/rows.jsonl")).expect("rows.jsonl");
    // (l, arm) -> x_r -> line
    let mut by: BTreeMap<(u64, String), BTreeMap<u64, String>> = BTreeMap::new();
    let (mut n_rows, mut n_bad, mut n_cens, mut n_sat, mut n_sat_ok) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut row_lines: Vec<String> = Vec::new();
    let mut ls: Vec<u64> = Vec::new();
    for line in txt.lines().filter(|x| !x.trim().is_empty()) {
        let (l, xr) = (num(line, "l").unwrap() as u64, num(line, "x_r").unwrap() as u64);
        let arm = string(line, "arm").unwrap();
        let status = string(line, "status").unwrap();
        let t = targets.get(&(l, xr)).expect("row without target");
        let verified = ["in_base", "liftable", "sums_to_target", "chain_ok"].iter().all(|k| boolean(line, k) == Some(true));
        let bad = match status.as_str() {
            "sat" => !(t.oracle_sat && verified),
            "unsat" => t.oracle_sat,
            "censored" => false,
            _ => true,
        };
        n_rows += 1;
        n_bad += bad as u64;
        n_cens += (status == "censored") as u64;
        if status == "sat" {
            n_sat += 1;
            n_sat_ok += verified as u64;
        }
        row_lines.push(line.to_string());
        if !ls.contains(&l) {
            ls.push(l);
        }
        assert!(by.entry((l, arm)).or_default().insert(xr, line.to_string()).is_none(), "duplicate row");
    }
    ls.sort();
    let f = |l: u64, arm: &str, xr: u64, k: &str| -> f64 {
        by.get(&(l, arm.to_string())).and_then(|m| m.get(&xr)).and_then(|ln| num(ln, k)).unwrap_or(f64::NAN)
    };
    let st = |l: u64, arm: &str, xr: u64| -> Option<String> {
        by.get(&(l, arm.to_string())).and_then(|m| m.get(&xr)).and_then(|ln| string(ln, "status"))
    };
    // cross-arm verdict agreement per target, and the profiling copy against the original
    let (mut n_targets_run, mut cross_mismatch, mut prof_checked, mut prof_mismatch) = (0u64, 0u64, 0u64, 0u64);
    let (mut conf_same_all, mut conf_cmp_all) = (0u64, 0u64);
    for (&(l, xr), t) in &targets {
        let ss: Vec<String> = ARMS.iter().filter_map(|arm| st(l, arm, xr)).collect();
        if ss.is_empty() {
            continue;
        }
        n_targets_run += 1;
        let want = if t.oracle_sat { "sat" } else { "unsat" };
        if ss.iter().any(|s| s != want && s != "censored") {
            cross_mismatch += 1;
        }
        if st(l, "lin21-prof", xr).is_some() && st(l, "lin21", xr).as_deref().is_some_and(|s| s != "censored") {
            prof_checked += 1;
            if ["conflicts", "decisions", "gje_calls", "propagations"].iter().any(|k| f(l, "lin21-prof", xr, k) != f(l, "lin21", xr, k)) {
                prof_mismatch += 1;
            }
        }
        let (c0, c1) = (f(l, "lin21", xr, "conflicts"), f(l, "lin21-fast", xr, "conflicts"));
        if c0.is_finite() && c1.is_finite() {
            conf_cmp_all += 1;
            conf_same_all += (c0 == c1) as u64;
        }
    }

    let mut md = String::new();
    let mut js = String::new();
    writeln!(md, "# Tables: lin21 wall-gap pilot (written by summary.rs)\n").unwrap();
    writeln!(md, "Every number is an unverified pilot assertion [pcc:unverified]. Wall times are single runs on a shared machine.\n").unwrap();
    writeln!(md, "Rows {n_rows}; censored {n_cens}; rows whose verdict differs from the pair oracle or whose SAT model fails a check: {n_bad}; targets with any arm disagreeing with the oracle: {cross_mismatch} of {n_targets_run}; SAT models {n_sat}, of which lift to points summing to the target: {n_sat_ok}. Profiling copy against the unmodified solver (conflicts, decisions, propagations, GJE calls): {prof_mismatch} mismatches in {prof_checked} targets. Conflicts of lin21-fast equal to lin21 on {conf_same_all} of {conf_cmp_all} targets (all kinds).\n").unwrap();

    // per-l tables over non-decomposable uniform targets
    struct Cell {
        l: u64,
        n_t: usize,
        w_scan: f64,
        w_lin: f64,
        w_fast: f64,
        w_fast_b: f64,
        r_fast_scan: f64,
        r_lin_scan: f64,
        r_lin_fast: f64,
        p_fast_scan: Vec<f64>,
        p_lin_fast: Vec<f64>,
        aa: Vec<f64>,
        c_lin: f64,
        c_fast: f64,
        c_same: usize,
        gw_scan: f64,
        gw_fast_rows: f64,
        gw_fast_cols: f64,
        gw_fast_copy: f64,
        gw_prof: f64,
        n_prof: usize,
        rx_scan: f64,
        rx_fast: f64,
        rx_prof: f64,
    }
    let mut cells: Vec<Cell> = Vec::new();
    for &l in &ls {
        let xs: Vec<u64> = targets
            .iter()
            .filter(|(k, t)| k.0 == l && t.kind == "uniform")
            .map(|(k, _)| k.1)
            .filter(|&xr| ["scan", "lin21", "lin21-fast", "lin21-fast-b"].iter().all(|a| st(l, a, xr).is_some_and(|s| s != "censored")))
            .collect();
        if xs.is_empty() {
            continue;
        }
        let col = |arm: &str, k: &str| -> Vec<f64> { xs.iter().map(|&x| f(l, arm, x, k)).collect() };
        let (ws, wl, wf, wb) = (col("scan", "wall_ms"), col("lin21", "wall_ms"), col("lin21-fast", "wall_ms"), col("lin21-fast-b", "wall_ms"));
        let pairs = |a: &[f64], b: &[f64]| -> Vec<f64> { a.iter().zip(b).map(|(x, y)| x / y).collect() };
        let (cl, cf) = (col("lin21", "conflicts"), col("lin21-fast", "conflicts"));
        let prof_x: Vec<u64> = xs.iter().cloned().filter(|&x| st(l, "lin21-prof", x).is_some()).collect();
        let pg: Vec<f64> = prof_x.iter().map(|&x| f(l, "lin21-prof", x, "gauss_words")).collect();
        let pr: Vec<f64> = prof_x.iter().map(|&x| f(l, "lin21-prof", x, "row_xors")).collect();
        cells.push(Cell {
            l,
            n_t: xs.len(),
            w_scan: med(&ws),
            w_lin: med(&wl),
            w_fast: med(&wf),
            w_fast_b: med(&wb),
            r_fast_scan: med(&wf) / med(&ws),
            r_lin_scan: med(&wl) / med(&ws),
            r_lin_fast: med(&wl) / med(&wf),
            p_fast_scan: pairs(&wf, &ws),
            p_lin_fast: pairs(&wl, &wf),
            aa: pairs(&wb, &wf),
            c_lin: med(&cl),
            c_fast: med(&cf),
            c_same: cl.iter().zip(&cf).filter(|(a, b)| a == b).count(),
            gw_scan: med(&col("scan", "gauss_words")),
            gw_fast_rows: med(&col("lin21-fast", "gauss_words")),
            gw_fast_cols: med(&col("lin21-fast", "col_xor_words")),
            gw_fast_copy: med(&col("lin21-fast", "mat_copy_words")),
            gw_prof: med(&pg),
            n_prof: prof_x.len(),
            rx_scan: med(&col("scan", "row_xors")),
            rx_fast: med(&col("lin21-fast", "row_xors")),
            rx_prof: med(&pr),
        });
    }
    writeln!(md, "## Wall time per target, non-decomposable uniform targets (median ms)\n").unwrap();
    writeln!(md, "Ratios in the first three ratio columns are ratios of medians (the rule's quantity). Paired = median [min, max] of the per-target ratio. A/A = second lin21-fast run / first, same process, the original lin21 run between them.\n").unwrap();
    writeln!(md, "| l | n | targets | scan | lin21 | lin21-fast | lin21-fast (2nd) | fast / scan | lin21 / scan | lin21 / fast | fast / scan paired | lin21 / fast paired | A/A paired |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    for c in &cells {
        let pr = |v: &[f64]| format!("{} [{}, {}]", g(med(v)), g(lo(v)), g(hi(v)));
        writeln!(
            md,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            c.l, 3 * c.l + 2, c.n_t, g(c.w_scan), g(c.w_lin), g(c.w_fast), g(c.w_fast_b), g(c.r_fast_scan), g(c.r_lin_scan),
            g(c.r_lin_fast), pr(&c.p_fast_scan), pr(&c.p_lin_fast), pr(&c.aa)
        )
        .unwrap();
    }
    writeln!(md, "\n## Conflicts and Gauss work, non-decomposable uniform targets (medians)\n").unwrap();
    writeln!(md, "Gauss work in 64-bit words touched by row operations. scan: 2 words per row XOR (column + combination). lin21-fast: variable range + mask word per row XOR; its column-index XORs and checkpoint copies are listed separately. lin21 (original): variable words + combination words per row XOR, counted by the profiling copy on the targets it ran on.\n").unwrap();
    writeln!(md, "| l | conflicts lin21 | conflicts lin21-fast | equal on | row XORs scan | row XORs fast | row XORs lin21 | words scan | words fast rows | words fast col index | words fast copies | words lin21 | lin21 targets counted | fast rows+cols / scan | lin21 / fast rows+cols |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    for c in &cells {
        let fw = c.gw_fast_rows + c.gw_fast_cols;
        writeln!(
            md,
            "| {} | {} | {} | {} of {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            c.l, g(c.c_lin), g(c.c_fast), c.c_same, c.n_t, g(c.rx_scan), g(c.rx_fast), g(c.rx_prof), g(c.gw_scan), g(c.gw_fast_rows),
            g(c.gw_fast_cols), g(c.gw_fast_copy), g(c.gw_prof), c.n_prof, g(fw / c.gw_scan), g(c.gw_prof / fw)
        )
        .unwrap();
    }
    // slopes
    let pts = |sel: &dyn Fn(&Cell) -> f64, lmin: u64| -> Vec<(f64, f64)> {
        cells.iter().filter(|c| c.l >= lmin).map(|c| (c.l as f64, sel(c).log2())).collect()
    };
    let s_ratio9 = slope(&pts(&|c| c.r_fast_scan, 9));
    let s_ratio_all = slope(&pts(&|c| c.r_fast_scan, 0));
    let s_fast = slope(&pts(&|c| c.w_fast, 9));
    let s_scan = slope(&pts(&|c| c.w_scan, 9));
    let s_lin = slope(&pts(&|c| c.w_lin, 9));
    writeln!(md, "\n## Slopes (bits per unit of l, least squares on medians)\n").unwrap();
    writeln!(md, "- log2(lin21-fast / scan), l >= 9: {s_ratio9:.3}; all l: {s_ratio_all:.3}").unwrap();
    writeln!(md, "- log2(wall), l >= 9: scan {s_scan:.3}, lin21-fast {s_fast:.3}, lin21 {s_lin:.3}").unwrap();

    // decision (pre-registered): primary cells l = 9..12
    let main: Vec<&Cell> = cells.iter().filter(|c| MAIN_LS.contains(&c.l)).collect();
    let complete = main.len() == MAIN_LS.len();
    let clean = n_bad == 0 && cross_mismatch == 0 && n_cens == 0;
    let closes = complete && clean && main.iter().all(|c| c.r_fast_scan <= 30.0);
    let partly = complete && main.iter().all(|c| c.r_lin_fast >= 10.0);
    let fails = main.last().is_some_and(|c| c.r_lin_fast < 3.0);
    let decision = if !complete {
        "incomplete"
    } else if closes {
        "gap closes"
    } else if fails {
        "fails"
    } else if partly {
        "partly"
    } else {
        "inconclusive"
    };
    let worst = main.iter().map(|c| c.r_fast_scan).fold(f64::NEG_INFINITY, f64::max);
    let aa_lo = main.iter().map(|c| lo(&c.aa)).fold(f64::INFINITY, f64::min);
    let aa_hi = main.iter().map(|c| hi(&c.aa)).fold(f64::NEG_INFINITY, f64::max);
    let aa_meds: Vec<f64> = main.iter().map(|c| med(&c.aa)).collect();
    writeln!(md, "\n## Decision by the pre-registered rule (l = 9..12)\n").unwrap();
    writeln!(md, "**{decision}** [pcc:unverified]. Largest lin21-fast / scan ratio of medians over l = 9..12: {}; threshold 30. Verdicts clean: {clean}. lin21 / lin21-fast ratio of medians: {}. A/A band over l = 9..12: per-target second/first in [{}, {}], per-l medians {}.", g(worst), main.iter().map(|c| format!("l={} {}", c.l, g(c.r_lin_fast))).collect::<Vec<_>>().join(", "), g(aa_lo), g(aa_hi), aa_meds.iter().map(|x| g(*x)).collect::<Vec<_>>().join(", ")).unwrap();
    // added after the measured run (README, Deviations): the protocol's A/A-band statement, computed here
    for c in &main {
        let (a, b) = (c.r_fast_scan * lo(&c.aa), c.r_fast_scan * hi(&c.aa));
        let (ma, mb) = (c.r_fast_scan * med(&c.aa), hi(&c.p_fast_scan));
        writeln!(md, "\n- l = {}: ratio {} x per-target A/A [min, max] = [{}, {}]; 30 inside: {}. Ratio x A/A median = {}; largest single-target lin21-fast / scan = {}.", c.l, g(c.r_fast_scan), g(a), g(b), a <= 30.0 && 30.0 <= b, g(ma), g(mb)).unwrap();
    }
    for c in cells.iter().filter(|c| c.l > 12) {
        writeln!(md, "\nExtension cell l = {} ({} targets): lin21-fast / scan = {}, lin21 / lin21-fast = {}; inside the 30x threshold: {}.", c.l, c.n_t, g(c.r_fast_scan), g(c.r_lin_fast), c.r_fast_scan <= 30.0).unwrap();
    }

    // phase shares of lin21-fast and of the profiling copy, summed over uniform targets
    writeln!(md, "\n## Time shares by phase (sum over the non-decomposable uniform targets, % of the summed phase timers)\n").unwrap();
    for arm in ["lin21-fast", "lin21-prof"] {
        let mut names: Vec<String> = Vec::new();
        let mut lines: Vec<String> = Vec::new();
        for &l in &ls {
            let Some(m) = by.get(&(l, arm.to_string())) else { continue };
            let mut sum: Vec<f64> = Vec::new();
            for (xr, ln) in m {
                if targets[&(l, *xr)].kind != "uniform" {
                    continue;
                }
                if names.is_empty() {
                    names = raw(ln, "prof_names").unwrap().trim_matches(|c| c == '[' || c == ']').split(',').map(|s| s.trim().trim_matches('"').to_string()).collect();
                }
                let v: Vec<f64> = raw(ln, "prof_ns").unwrap().trim_matches(|c| c == '[' || c == ']').split(',').map(|s| s.trim().parse().unwrap()).collect();
                if sum.is_empty() {
                    sum = vec![0.0; v.len()];
                }
                for (a, b) in sum.iter_mut().zip(&v) {
                    *a += b;
                }
            }
            if sum.is_empty() {
                continue;
            }
            let tot: f64 = sum.iter().sum();
            lines.push(format!("| {l} | {} |", sum.iter().map(|x| format!("{:.1}", 100.0 * x / tot)).collect::<Vec<_>>().join(" | ")));
        }
        if lines.is_empty() {
            continue;
        }
        writeln!(md, "{arm}:\n\n| l | {} |\n|---|{}", names.join(" | "), "---|".repeat(names.len())).unwrap();
        for ln in lines {
            writeln!(md, "{ln}").unwrap();
        }
        writeln!(md).unwrap();
    }

    // all targets (planted, uniform-dec): status and conflicts
    writeln!(md, "## Decomposable targets (planted and uniform-dec): verdicts and conflicts\n").unwrap();
    writeln!(md, "| l | targets | all arms SAT with lifted models | conflicts lin21 (sum) | conflicts lin21-fast (sum) | wall lin21 ms (sum) | wall lin21-fast ms (sum) | wall scan ms (sum, full scan) |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|").unwrap();
    for &l in &ls {
        let xs: Vec<u64> = targets.iter().filter(|(k, t)| k.0 == l && t.oracle_sat).map(|(k, _)| k.1).filter(|&x| st(l, "lin21-fast", x).is_some()).collect();
        if xs.is_empty() {
            continue;
        }
        let ok = xs
            .iter()
            .filter(|&&x| {
                ["scan", "lin21", "lin21-fast", "lin21-fast-b"].iter().all(|a| {
                    by.get(&(l, a.to_string())).and_then(|m| m.get(&x)).is_some_and(|ln| string(ln, "status").as_deref() == Some("sat") && boolean(ln, "sums_to_target") == Some(true) && boolean(ln, "liftable") == Some(true))
                })
            })
            .count();
        let sum = |arm: &str, k: &str| -> f64 { xs.iter().map(|&x| f(l, arm, x, k)).sum() };
        writeln!(md, "| {l} | {} | {ok} | {} | {} | {} | {} | {} |", xs.len(), g(sum("lin21", "conflicts")), g(sum("lin21-fast", "conflicts")), g(sum("lin21", "wall_ms")), g(sum("lin21-fast", "wall_ms")), g(sum("scan", "wall_ms"))).unwrap();
    }
    fs::write(format!("{dir}/tables.md"), &md).unwrap();

    // results.json
    writeln!(js, "{{").unwrap();
    writeln!(js, "  \"note\": \"unverified pilot assertions [pcc:unverified]; wall times are single runs on a shared machine\",").unwrap();
    writeln!(js, "  \"decision\": \"{decision}\",").unwrap();
    writeln!(js, "  \"rule\": \"gap closes: lin21-fast median wall <= 30x scan at every l in 9..12 with identical verdicts; partly: >= 10x faster than lin21 at every l in 9..12 but above 30x; fails: < 3x faster than lin21 at l = 12\",").unwrap();
    writeln!(js, "  \"checks\": {{\"rows\": {n_rows}, \"censored\": {n_cens}, \"bad_rows\": {n_bad}, \"targets_run\": {n_targets_run}, \"targets_with_disagreement\": {cross_mismatch}, \"sat_models\": {n_sat}, \"sat_models_lifted\": {n_sat_ok}, \"prof_copy_checked\": {prof_checked}, \"prof_copy_mismatch\": {prof_mismatch}, \"conflicts_equal_targets\": {conf_same_all}, \"conflicts_compared_targets\": {conf_cmp_all}}},").unwrap();
    writeln!(js, "  \"slopes_bits_per_l\": {{\"ratio_fast_over_scan_l9plus\": {}, \"ratio_fast_over_scan_all_l\": {}, \"wall_scan_l9plus\": {}, \"wall_fast_l9plus\": {}, \"wall_lin21_l9plus\": {}}},", j(s_ratio9), j(s_ratio_all), j(s_scan), j(s_fast), j(s_lin)).unwrap();
    writeln!(js, "  \"aa_band_l9_12\": {{\"min\": {}, \"max\": {}}},", j(aa_lo), j(aa_hi)).unwrap();
    writeln!(js, "  \"cells\": [").unwrap();
    for (i, c) in cells.iter().enumerate() {
        writeln!(
            js,
            "    {{\"l\": {}, \"n\": {}, \"targets\": {}, \"median_wall_ms\": {{\"scan\": {}, \"lin21\": {}, \"lin21_fast\": {}, \"lin21_fast_b\": {}}}, \"ratio_of_medians\": {{\"fast_over_scan\": {}, \"lin21_over_scan\": {}, \"lin21_over_fast\": {}}}, \"paired_fast_over_scan\": {{\"median\": {}, \"min\": {}, \"max\": {}}}, \"paired_lin21_over_fast\": {{\"median\": {}, \"min\": {}, \"max\": {}}}, \"aa_second_over_first\": {{\"median\": {}, \"min\": {}, \"max\": {}}}, \"median_conflicts\": {{\"lin21\": {}, \"lin21_fast\": {}, \"equal_targets\": {}}}, \"median_row_xors\": {{\"scan\": {}, \"lin21_fast\": {}, \"lin21\": {}}}, \"median_gauss_words\": {{\"scan\": {}, \"lin21_fast_rows\": {}, \"lin21_fast_col_index\": {}, \"lin21_fast_copies\": {}, \"lin21\": {}, \"lin21_targets_counted\": {}}}}}{}",
            c.l, 3 * c.l + 2, c.n_t, j(c.w_scan), j(c.w_lin), j(c.w_fast), j(c.w_fast_b), j(c.r_fast_scan), j(c.r_lin_scan), j(c.r_lin_fast),
            j(med(&c.p_fast_scan)), j(lo(&c.p_fast_scan)), j(hi(&c.p_fast_scan)), j(med(&c.p_lin_fast)), j(lo(&c.p_lin_fast)), j(hi(&c.p_lin_fast)),
            j(med(&c.aa)), j(lo(&c.aa)), j(hi(&c.aa)), j(c.c_lin), j(c.c_fast), c.c_same, j(c.rx_scan), j(c.rx_fast), j(c.rx_prof),
            j(c.gw_scan), j(c.gw_fast_rows), j(c.gw_fast_cols), j(c.gw_fast_copy), j(c.gw_prof), c.n_prof,
            if i + 1 < cells.len() { "," } else { "" }
        )
        .unwrap();
    }
    writeln!(js, "  ],").unwrap();
    writeln!(js, "  \"targets\": [").unwrap();
    let tl: Vec<&String> = target_lines.iter().filter(|t| ls.contains(&(num(t, "l").unwrap() as u64))).collect();
    for (i, t) in tl.iter().enumerate() {
        let key = (num(t, "l").unwrap() as u64, num(t, "x_r").unwrap() as u64);
        let _ = targets[&key].idx;
        writeln!(js, "    {t}{}", if i + 1 < tl.len() { "," } else { "" }).unwrap();
    }
    writeln!(js, "  ],").unwrap();
    writeln!(js, "  \"rows\": [").unwrap();
    for (i, r) in row_lines.iter().enumerate() {
        writeln!(js, "    {r}{}", if i + 1 < row_lines.len() { "," } else { "" }).unwrap();
    }
    writeln!(js, "  ]\n}}").unwrap();
    fs::write(format!("{dir}/results.json"), &js).unwrap();

    // SVG: wall per target against l, log2 axis, three arms
    let (w, h, ml, mr, mt, mb) = (760.0, 460.0, 70.0, 150.0, 40.0, 50.0);
    let (lmin, lmax) = (cells.first().map(|c| c.l).unwrap_or(5) as f64, cells.last().map(|c| c.l).unwrap_or(12) as f64);
    let ymin = cells.iter().map(|c| c.w_scan.log2()).fold(f64::INFINITY, f64::min).floor();
    let ymax = cells.iter().map(|c| c.w_lin.log2()).fold(f64::NEG_INFINITY, f64::max).ceil();
    let px = |l: f64| ml + (l - lmin) / (lmax - lmin).max(1.0) * (w - ml - mr);
    let py = |y: f64| h - mb - (y - ymin) / (ymax - ymin) * (h - mt - mb);
    let mut s = String::new();
    writeln!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" font-family=\"sans-serif\" font-size=\"12\">").unwrap();
    writeln!(s, "<rect width=\"{w}\" height=\"{h}\" fill=\"#ffffff\"/>").unwrap();
    writeln!(s, "<text x=\"{ml}\" y=\"20\" font-size=\"14\" fill=\"#111\">Median wall time per non-decomposable target (ms, log scale) [pcc:unverified]</text>").unwrap();
    let mut y = ymin;
    while y <= ymax {
        let yy = py(y);
        writeln!(s, "<line x1=\"{ml}\" y1=\"{yy:.1}\" x2=\"{:.1}\" y2=\"{yy:.1}\" stroke=\"#e3e3e3\"/>", w - mr).unwrap();
        let v = 2f64.powf(y);
        let lab = if v >= 1.0 { format!("{v:.0}") } else { format!("{v:.3}") };
        writeln!(s, "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{lab}</text>", ml - 6.0, yy + 4.0).unwrap();
        y += 2.0;
    }
    for c in &cells {
        writeln!(s, "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#444\">{}</text>", px(c.l as f64), h - mb + 18.0, c.l).unwrap();
    }
    writeln!(s, "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#111\">l (subspace dimension), n = 3l + 2</text>", (ml + w - mr) / 2.0, h - 10.0).unwrap();
    let series: [(&str, &str, &dyn Fn(&Cell) -> f64); 3] = [
        ("lin21 (original solver)", "#b3261e", &|c| c.w_lin),
        ("lin21-fast", "#1a5fb4", &|c| c.w_fast),
        ("plain scan", "#2e7d32", &|c| c.w_scan),
    ];
    for (name, colr, sel) in series {
        let pts: Vec<String> = cells.iter().map(|c| format!("{:.1},{:.1}", px(c.l as f64), py(sel(c).log2()))).collect();
        writeln!(s, "<polyline fill=\"none\" stroke=\"{colr}\" stroke-width=\"2\" points=\"{}\"/>", pts.join(" ")).unwrap();
        for c in &cells {
            writeln!(s, "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{colr}\"/>", px(c.l as f64), py(sel(c).log2())).unwrap();
        }
        if let Some(c) = cells.last() {
            writeln!(s, "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{colr}\">{name}</text>", px(c.l as f64) + 8.0, py(sel(c).log2()) + 4.0).unwrap();
        }
    }
    writeln!(s, "</svg>").unwrap();
    fs::write(format!("{dir}/fig_wall_vs_l.svg"), &s).unwrap();
    println!("decision: {decision}; worst fast/scan over l=9..12: {}; bad rows {n_bad}; disagreements {cross_mismatch}", g(worst));
}
