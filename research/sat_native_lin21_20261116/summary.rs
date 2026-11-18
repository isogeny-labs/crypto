//! Tables, slopes, the pre-registered decision and the SVG figure for the lin21 pilot.
//! Standard library only: `rustc -O summary.rs && ./summary DIR`. Reads DIR/raw/targets_l*.jsonl
//! and DIR/raw/rows.jsonl (JSON lines printed by the `lin21` binary; extra keys are ignored),
//! recomputes every agreement/verification flag from the binary's own fields, and writes
//! DIR/tables.md, DIR/summary.json, DIR/results.json and DIR/fig_conflicts_vs_l.svg.

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
fn arr(line: &str, key: &str) -> Vec<u64> {
    raw(line, key)
        .map(|s| s.trim_matches(|c| c == '[' || c == ']').split(',').filter_map(|x| x.trim().parse().ok()).collect())
        .unwrap_or_default()
}

struct Target {
    idx: u64,
    kind: String,
    oracle_sat: bool,
}
struct Row {
    line: String,
    status: String,
    kind: String,
    idx: u64,
    bad: bool,
}
impl Row {
    fn f(&self, k: &str) -> f64 {
        num(&self.line, k).unwrap_or(f64::NAN)
    }
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
    } else if x.abs() >= 1.0 {
        format!("{x:.2}")
    } else {
        format!("{x:.4}")
    }
}
fn rng(v: &[f64]) -> String {
    if v.is_empty() { "n/a".into() } else { format!("{} [{}, {}]", g(med(v)), g(lo(v)), g(hi(v))) }
}

const SAT_ARMS: [&str; 4] = ["lin", "lin-nolex", "lin21", "lin21-pu"];
const CAP_MS: f64 = 600_000.0;

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    // targets
    let mut targets: BTreeMap<(u64, u64), Target> = BTreeMap::new();
    let mut target_lines: Vec<String> = Vec::new();
    for l in 3..=20u64 {
        let Ok(txt) = fs::read_to_string(format!("{dir}/raw/targets_l{l}.jsonl")) else { continue };
        for line in txt.lines().filter(|x| !x.trim().is_empty()) {
            let key = (num(line, "l").unwrap() as u64, num(line, "x_r").unwrap() as u64);
            let t = Target {
                idx: num(line, "idx").unwrap() as u64,
                kind: string(line, "kind").unwrap(),
                oracle_sat: boolean(line, "oracle_sat").unwrap(),
            };
            assert!(targets.insert(key, t).is_none(), "duplicate target");
            target_lines.push(line.to_string());
        }
    }
    // rows, keyed (l, arm) -> x_r -> row
    let txt = fs::read_to_string(format!("{dir}/raw/rows.jsonl")).expect("rows.jsonl");
    let mut by: BTreeMap<(u64, String), BTreeMap<u64, Row>> = BTreeMap::new();
    let (mut n_rows, mut n_bad, mut n_cens, mut n_sat, mut n_sat_ok) = (0, 0, 0, 0, 0);
    let mut row_lines: Vec<String> = Vec::new();
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
        n_bad += bad as u32;
        n_cens += (status == "censored") as u32;
        if status == "sat" {
            n_sat += 1;
            n_sat_ok += verified as u32;
        }
        row_lines.push(line.to_string());
        let r = Row { line: line.to_string(), status, kind: t.kind.clone(), idx: t.idx, bad };
        assert!(by.entry((l, arm)).or_default().insert(xr, r).is_none(), "duplicate row");
    }
    let ls: Vec<u64> = {
        let mut v: Vec<u64> = by.keys().map(|k| k.0).collect();
        v.dedup();
        v
    };
    let lmax = *ls.last().unwrap();
    let refut = |l: u64, arm: &str| -> Vec<(u64, &Row)> {
        by.get(&(l, arm.to_string()))
            .map(|m| m.iter().filter(|(_, r)| r.kind == "uniform" && r.status == "unsat" && !r.bad).map(|(k, r)| (*k, r)).collect())
            .unwrap_or_default()
    };
    let col = |rs: &[(u64, &Row)], k: &str| -> Vec<f64> { rs.iter().map(|(_, r)| r.f(k)).collect() };

    let mut o = String::new();
    let mut js = String::from("{\n");
    writeln!(o, "Generated by `summary.rs` from `raw/targets_l*.jsonl` and `raw/rows.jsonl`.\n").unwrap();
    writeln!(o, "Rows: {n_rows}; verdict disagreements with the pair oracle, unverified SAT models or errors: {n_bad}; censored at the cap: {n_cens}; SAT models lifted to points summing to the target: {n_sat_ok} of {n_sat}. [pcc:unverified]\n").unwrap();
    writeln!(js, " \"integrity\": {{\"rows\": {n_rows}, \"bad\": {n_bad}, \"censored\": {n_cens}, \"sat_models\": {n_sat}, \"sat_verified\": {n_sat_ok}}},").unwrap();

    // table 1
    writeln!(o, "## Conflicts on non-decomposable uniform targets: median [min, max] [pcc:unverified]\n").unwrap();
    writeln!(o, "| l | n | arm | targets | conflicts | / 2^l | / C(2^l,2) | decisions | GJE calls | gate linearisations | wall ms |").unwrap();
    writeln!(o, "|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut medc: BTreeMap<(u64, &str), f64> = BTreeMap::new();
    writeln!(js, " \"cells\": {{").unwrap();
    for &l in &ls {
        let (p, c2) = ((1u64 << l) as f64, ((1u64 << l) * ((1u64 << l) - 1) / 2) as f64);
        for arm in SAT_ARMS {
            let rs = refut(l, arm);
            let ncens = by.get(&(l, arm.to_string())).map(|m| m.values().filter(|r| r.kind == "uniform" && r.status == "censored").count()).unwrap_or(0);
            if rs.is_empty() && ncens == 0 {
                continue;
            }
            if rs.is_empty() {
                writeln!(o, "| {l} | {} | {arm} | 0 (+{ncens} censored) | censored | | | | | | > {CAP_MS} |", 3 * l + 2).unwrap();
                continue;
            }
            let c = col(&rs, "conflicts");
            let m = med(&c);
            medc.insert((l, arm), m);
            let cens = if ncens > 0 { format!(" (+{ncens} censored)") } else { String::new() };
            writeln!(o, "| {l} | {} | {arm} | {}{cens} | {} | {:.3} [{:.3}, {:.3}] | {:.4} [{:.4}, {:.4}] | {} | {} | {} | {} |",
                3 * l + 2, rs.len(), rng(&c), m / p, lo(&c) / p, hi(&c) / p, m / c2, lo(&c) / c2, hi(&c) / c2,
                g(med(&col(&rs, "decisions"))), g(med(&col(&rs, "gje_calls"))), g(med(&col(&rs, "gate_linearisations"))),
                rng(&col(&rs, "wall_ms"))).unwrap();
            writeln!(js, "  \"{l}/{arm}\": {{\"targets\": {}, \"censored\": {ncens}, \"median_conflicts\": {m}, \"min\": {}, \"max\": {}, \"per_2l\": {}, \"per_pairs\": {}, \"median_wall_ms\": {}}},",
                rs.len(), lo(&c), hi(&c), m / p, m / c2, med(&col(&rs, "wall_ms"))).unwrap();
        }
    }

    // slopes
    writeln!(o, "\n## Least-squares slope of log2(median) against l [pcc:unverified]\n").unwrap();
    writeln!(o, "| arm | l range | slope of log2 conflicts (bits per unit of l) | slope of log2 wall ms |").unwrap();
    writeln!(o, "|---|---|---|---|").unwrap();
    let slope = |pts: &[(f64, f64)]| -> f64 {
        let n = pts.len() as f64;
        let (sx, sy) = (pts.iter().map(|p| p.0).sum::<f64>(), pts.iter().map(|p| p.1).sum::<f64>());
        let (sxx, sxy) = (pts.iter().map(|p| p.0 * p.0).sum::<f64>(), pts.iter().map(|p| p.0 * p.1).sum::<f64>());
        (n * sxy - sx * sy) / (n * sxx - sx * sx)
    };
    let mut slopes = String::new();
    for arm in ["lin", "lin-nolex", "lin21", "lin21-pu", "scan"] {
        let mut pc = Vec::new();
        let mut pw = Vec::new();
        for &l in &ls {
            let rs = refut(l, arm);
            if rs.is_empty() {
                continue;
            }
            let key = if arm == "scan" { "leaves" } else { "conflicts" };
            pc.push((l as f64, med(&col(&rs, key)).log2()));
            pw.push((l as f64, med(&col(&rs, "wall_ms")).log2()));
        }
        if pc.len() >= 2 {
            let (a, b) = (slope(&pc), slope(&pw));
            let what = if arm == "scan" { " (leaves)" } else { "" };
            writeln!(o, "| {arm} | {}..{} | {a:.3}{what} | {b:.3} |", pc[0].0, pc[pc.len() - 1].0).unwrap();
            write!(slopes, "\"{arm}\": {{\"conflicts\": {a}, \"wall\": {b}}}, ").unwrap();
        }
    }

    // scan
    writeln!(o, "\n## Plain native 2+1 scan (no SAT), non-decomposable uniform targets, medians [pcc:unverified]\n").unwrap();
    writeln!(o, "| l | targets | x1 visited | leaves | leaves with consistent (s,p) | field mul | field inv | Artin-Schreier solves | shifts | row XORs | wall ms (median of 21 repeats): median [min, max] |").unwrap();
    writeln!(o, "|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    for &l in &ls {
        let rs = refut(l, "scan");
        if rs.is_empty() {
            continue;
        }
        let m = |k: &str| g(med(&col(&rs, k)));
        writeln!(o, "| {l} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |", rs.len(), m("x1_visited"), m("leaves"), m("leaves_consistent"),
            m("field_muls"), m("field_invs"), m("as_solves"), m("xtimes"), m("row_xors"), rng(&col(&rs, "wall_ms"))).unwrap();
        writeln!(js, "  \"{l}/scan\": {{\"targets\": {}, \"median_leaves\": {}, \"median_wall_ms\": {}}},", rs.len(), med(&col(&rs, "leaves")), med(&col(&rs, "wall_ms"))).unwrap();
    }
    writeln!(js, "  \"_\": null\n }},\n \"slopes\": {{{slopes}\"_\": null}},").unwrap();

    // paired
    writeln!(o, "\n## Paired ratios on common non-decomposable targets: median of per-target ratios [min, max] [pcc:unverified]\n").unwrap();
    writeln!(o, "| l | common targets lin & lin21 | conflicts lin / lin21 | 2^(l-3) | wall lin / lin21 | 2^(l-4) | wall lin21 / scan | wall lin / scan | conflicts lin21-pu / lin21 | D = lin21 conflicts minus scan count [min, max] |").unwrap();
    writeln!(o, "|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut pair: BTreeMap<u64, (usize, f64)> = BTreeMap::new(); // l -> (common, ratio of medians)
    writeln!(js, " \"paired\": {{").unwrap();
    for &l in &ls {
        let m = |arm: &str| -> BTreeMap<u64, &Row> { refut(l, arm).into_iter().collect() };
        let (a, b, s, pu) = (m("lin"), m("lin21"), m("scan"), m("lin21-pu"));
        let ratio = |x: &BTreeMap<u64, &Row>, y: &BTreeMap<u64, &Row>, k: &str| -> Vec<f64> {
            x.iter().filter_map(|(i, r)| y.get(i).map(|q| r.f(k) / q.f(k))).collect()
        };
        let rc = ratio(&a, &b, "conflicts");
        let common: Vec<u64> = a.keys().filter(|i| b.contains_key(i)).cloned().collect();
        let rom = if common.is_empty() { f64::NAN } else {
            med(&common.iter().map(|i| a[i].f("conflicts")).collect::<Vec<_>>()) / med(&common.iter().map(|i| b[i].f("conflicts")).collect::<Vec<_>>())
        };
        pair.insert(l, (common.len(), rom));
        let dd: Vec<f64> = b.iter().filter_map(|(i, r)| s.get(i).map(|q| r.f("conflicts") - (1.0 + q.f("x1_visited") - q.f("x1_with_roots") + q.f("leaves")))).collect();
        let dstr = if dd.is_empty() { "n/a".to_string() } else { format!("[{}, {}]", g(lo(&dd)), g(hi(&dd))) };
        let (rw, r21s, rls, rpu) = (ratio(&a, &b, "wall_ms"), ratio(&b, &s, "wall_ms"), ratio(&a, &s, "wall_ms"), ratio(&pu, &b, "conflicts"));
        writeln!(o, "| {l} | {} | {} | {} | {} | {} | {} | {} | {} | {dstr} |", common.len(), rng(&rc), 1u64 << (l - 3), rng(&rw), 1u64 << (l - 4), rng(&r21s), rng(&rls), rng(&rpu)).unwrap();
        writeln!(js, "  \"{l}\": {{\"common\": {}, \"ratio_of_medians_conflicts\": {}, \"median_wall_lin_over_lin21\": {}, \"median_wall_lin21_over_scan\": {}, \"D_min\": {}, \"D_max\": {}}},",
            common.len(), jn(rom), jn(med(&rw)), jn(med(&r21s)), jn(lo(&dd)), jn(hi(&dd))).unwrap();
    }
    writeln!(js, "  \"_\": null\n }},").unwrap();

    // planted
    writeln!(o, "\n## Planted targets and decomposable uniform draws [pcc:unverified]\n").unwrap();
    writeln!(o, "| l | arm | targets | SAT and lifted | conflicts: median [min, max] | wall ms: median [min, max] |").unwrap();
    writeln!(o, "|---|---|---|---|---|---|").unwrap();
    for &l in &ls {
        for arm in ["lin", "lin-nolex", "lin21", "lin21-pu", "scan"] {
            let Some(m) = by.get(&(l, arm.to_string())) else { continue };
            let rs: Vec<&Row> = m.values().filter(|r| r.kind != "uniform").collect();
            if rs.is_empty() {
                continue;
            }
            let ok: Vec<&Row> = rs.iter().cloned().filter(|r| r.status == "sat" && !r.bad).collect();
            let c: Vec<f64> = ok.iter().map(|r| r.f("conflicts")).filter(|x| !x.is_nan()).collect();
            let w: Vec<f64> = ok.iter().map(|r| r.f("wall_ms")).collect();
            writeln!(o, "| {l} | {arm} | {} | {} | {} | {} |", rs.len(), ok.len(), rng(&c), rng(&w)).unwrap();
        }
    }

    // histogram
    writeln!(o, "\n## lin21: learnt clauses by number of x1 priority literals, summed over non-decomposable uniform targets [pcc:unverified]\n").unwrap();
    writeln!(o, "The priority set of `lin21` is the l bits of x1 only. A clause of width w < l excludes 2^(l-w) values of x1.\n").unwrap();
    write!(o, "| l | targets |").unwrap();
    for w in 0..=lmax {
        write!(o, " w={w} |").unwrap();
    }
    writeln!(o, " clauses | conflicts |").unwrap();
    writeln!(o, "|---|---|{}---|---|", "---|".repeat(lmax as usize + 1)).unwrap();
    writeln!(js, " \"lin21_width_hist\": {{").unwrap();
    for &l in &ls {
        let rs = refut(l, "lin21");
        let mut h = vec![0u64; lmax as usize + 1];
        for (_, r) in &rs {
            for (w, k) in arr(&r.line, "learnt_priority_width").iter().enumerate() {
                h[w] += k;
            }
        }
        write!(o, "| {l} | {} |", rs.len()).unwrap();
        for (w, x) in h.iter().enumerate() {
            if w as u64 <= l { write!(o, " {x} |").unwrap() } else { write!(o, " |").unwrap() }
        }
        writeln!(o, " {} | {} |", h.iter().sum::<u64>(), g(col(&rs, "conflicts").iter().sum::<f64>())).unwrap();
        writeln!(js, "  \"{l}\": {h:?},").unwrap();
    }
    writeln!(js, "  \"_\": null\n }},").unwrap();

    // decision
    let big: Vec<u64> = ls.iter().cloned().filter(|&l| l >= 7 && medc.contains_key(&(l, "lin21"))).collect();
    let cond_a = !big.is_empty() && big.iter().all(|&l| medc[&(l, "lin21")] <= 4.0 * (1u64 << l) as f64);
    let lstar = ls.iter().cloned().filter(|l| pair[l].0 >= 4).max();
    let (dec, ratio, thr) = match lstar {
        None => ("inconclusive", f64::NAN, f64::NAN),
        Some(l) => {
            let r = pair[&l].1;
            let thr = (1u64 << (l - 3)) as f64;
            (if r < 4.0 { "fails" } else if cond_a && r >= thr { "lead holds" } else { "inconclusive" }, r, thr)
        }
    };
    writeln!(o, "\n## Decision by the pre-registered rule\n\nCondition A (lin21 median conflicts <= 4 * 2^l at every measured l >= 7; measured l = {big:?}): {cond_a}. L* (largest l with >= 4 common uncensored non-decomposable targets in lin and lin21): {}; median(lin) / median(lin21) conflicts there: {}; threshold 2^(L*-3) = {}. **Decision: {dec}.** [pcc:unverified]",
        lstar.map(|l| l.to_string()).unwrap_or("none".into()), g(ratio), g(thr)).unwrap();
    writeln!(js, " \"decision\": {{\"condA\": {cond_a}, \"L_star\": {}, \"ratio_of_medians\": {}, \"threshold\": {}, \"decision\": \"{dec}\"}}\n}}",
        lstar.map(|l| l.to_string()).unwrap_or("null".into()), jn(ratio), jn(thr)).unwrap();

    fs::write(format!("{dir}/tables.md"), &o).unwrap();
    fs::write(format!("{dir}/summary.json"), &js).unwrap();
    // results.json: the raw lines, verbatim, as two arrays
    let res = format!("{{\n\"schema\": \"sat_native_lin21.pilot/v1\",\n\"targets\": [\n{}\n],\n\"rows\": [\n{}\n]\n}}\n", target_lines.join(",\n"), row_lines.join(",\n"));
    fs::write(format!("{dir}/results.json"), res).unwrap();
    print!("{o}");

    // ---- SVG figure: conflicts vs l, log2 axis
    let (w, h, ml, mr, mt, mb) = (860.0, 560.0, 70.0, 250.0, 40.0, 50.0);
    let (x0, x1) = (ls[0] as f64 - 0.3, lmax as f64 + 0.3);
    let (y0, y1) = (4.0, (2.0 * lmax as f64).max(20.0)); // log2 range
    let px = |l: f64| ml + (l - x0) / (x1 - x0) * (w - ml - mr);
    let py = |v: f64| h - mb - (v.log2() - y0) / (y1 - y0) * (h - mt - mb);
    let mut s = String::new();
    writeln!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"sans-serif\" font-size=\"12\">").unwrap();
    writeln!(s, "<rect width=\"{w}\" height=\"{h}\" fill=\"white\"/>").unwrap();
    writeln!(s, "<text x=\"{}\" y=\"20\" font-size=\"13\">3-summand PDP refutation in LinCdcl: chained pair encoding (lin) vs 2+1 leaf (lin21); median, bar = min..max</text>", ml - 40.0).unwrap();
    let mut e = y0 as i64;
    while e as f64 <= y1 {
        let y = py(2f64.powi(e as i32));
        writeln!(s, "<line x1=\"{ml}\" y1=\"{y:.1}\" x2=\"{}\" y2=\"{y:.1}\" stroke=\"#ddd\"/><text x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">2^{e}</text>", w - mr, ml - 6.0, y + 4.0).unwrap();
        e += 2;
    }
    for &l in &ls {
        let x = px(l as f64);
        writeln!(s, "<line x1=\"{x:.1}\" y1=\"{mt}\" x2=\"{x:.1}\" y2=\"{}\" stroke=\"#eee\"/><text x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{l}</text>", h - mb, h - mb + 18.0).unwrap();
    }
    writeln!(s, "<rect x=\"{ml}\" y=\"{mt}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"black\"/>", w - ml - mr, h - mt - mb).unwrap();
    writeln!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">l = dim V   (n = 3l + 2)</text>", ml + (w - ml - mr) / 2.0, h - 10.0).unwrap();
    writeln!(s, "<text transform=\"translate(18,{}) rotate(-90)\" text-anchor=\"middle\">conflicts per non-decomposable target</text>", mt + (h - mt - mb) / 2.0).unwrap();
    let mut legend: Vec<(String, String, String)> = Vec::new();
    let mut tsv = String::from("series\tl\tmedian\tmin\tmax\n");
    let refs: [(&str, &str, Box<dyn Fn(f64) -> f64>); 3] = [
        ("2^l", "2,3", Box::new(|l| 2f64.powf(l))),
        ("4 * 2^l (rule bound)", "1,5", Box::new(|l| 4.0 * 2f64.powf(l))),
        ("C(2^l, 2)", "7,4", Box::new(|l| 2f64.powf(l) * (2f64.powf(l) - 1.0) / 2.0)),
    ];
    for (name, dash, f) in refs.iter() {
        let pts: Vec<String> = ls.iter().map(|&l| format!("{:.1},{:.1}", px(l as f64), py(f(l as f64)))).collect();
        writeln!(s, "<polyline points=\"{}\" fill=\"none\" stroke=\"black\" stroke-dasharray=\"{dash}\"/>", pts.join(" ")).unwrap();
        legend.push((name.to_string(), "black".into(), dash.to_string()));
    }
    for (arm, color) in [("lin", "#1f77b4"), ("lin-nolex", "#7f7f7f"), ("lin21", "#d62728"), ("lin21-pu", "#ff7f0e"), ("scan", "#2ca02c")] {
        let key = if arm == "scan" { "leaves" } else { "conflicts" };
        let mut pts = Vec::new();
        for &l in &ls {
            let rs = refut(l, arm);
            if rs.is_empty() {
                continue;
            }
            let c = col(&rs, key);
            let (x, off) = (px(l as f64), if arm == "lin21-pu" { 4.0 } else { 0.0 });
            writeln!(s, "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{color}\" stroke-width=\"2\"/>", x + off, py(lo(&c)), x + off, py(hi(&c))).unwrap();
            writeln!(s, "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{color}\"/>", x + off, py(med(&c))).unwrap();
            pts.push(format!("{:.1},{:.1}", x + off, py(med(&c))));
            writeln!(tsv, "{arm}\t{l}\t{}\t{}\t{}", med(&c), lo(&c), hi(&c)).unwrap();
        }
        if pts.is_empty() {
            continue;
        }
        writeln!(s, "<polyline points=\"{}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.5\"/>", pts.join(" ")).unwrap();
        legend.push((if arm == "scan" { "plain 2+1 scan: leaves solved".into() } else { format!("{arm}: conflicts") }, color.into(), "".into()));
    }
    for (i, (name, color, dash)) in legend.iter().enumerate() {
        let y = mt + 14.0 + 20.0 * i as f64;
        writeln!(s, "<line x1=\"{}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"{color}\" stroke-width=\"1.5\" stroke-dasharray=\"{dash}\"/><text x=\"{}\" y=\"{}\">{name}</text>", w - mr + 12.0, w - mr + 44.0, w - mr + 50.0, y + 4.0).unwrap();
    }
    writeln!(s, "</svg>").unwrap();
    fs::write(format!("{dir}/fig_conflicts_vs_l.svg"), s).unwrap();
    fs::write(format!("{dir}/fig_data.tsv"), tsv).unwrap();
}

fn jn(x: f64) -> String {
    if x.is_finite() { format!("{x}") } else { "null".into() }
}
