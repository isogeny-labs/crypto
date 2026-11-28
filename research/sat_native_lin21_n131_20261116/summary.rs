//! Tables, decision and figures for the n131 pilot. Build: rustc -O summary.rs
//! Usage: summary OUTDIR TARGET_DIR_LIN21 TARGET_DIR_FAST
//! Reads OUTDIR/raw/{rows_small,rows_k130,dt,dtmw,selfcheck}.jsonl and the small-field target
//! files (for the pair oracle's verdict); writes OUTDIR/{results.json,tables.md,fig_*.svg}.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;

fn raw(line: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = if rest.starts_with('"') {
        1 + rest[1..].find('"')? + 1
    } else if rest.starts_with('[') {
        rest.find(']')? + 1
    } else {
        rest.find(|c| c == ',' || c == '}')?
    };
    Some(rest[..end].trim_matches('"').to_string())
}
fn num(line: &str, key: &str) -> Option<f64> {
    raw(line, key)?.parse().ok()
}
fn med(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if s.len() % 2 == 1 {
        s[s.len() / 2]
    } else {
        0.5 * (s[s.len() / 2 - 1] + s[s.len() / 2])
    }
}
fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)))
}
/// least-squares slope of y against x
fn slope(pts: &[(f64, f64)]) -> f64 {
    let n = pts.len() as f64;
    let (mx, my) = (pts.iter().map(|p| p.0).sum::<f64>() / n, pts.iter().map(|p| p.1).sum::<f64>() / n);
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx) * (p.0 - mx)).sum();
    sxy / sxx
}
fn g(x: f64) -> String {
    if x.is_nan() {
        "n/a".into()
    } else if x >= 1000.0 {
        format!("{x:.0}")
    } else if x >= 100.0 {
        format!("{x:.1}")
    } else if x >= 10.0 {
        format!("{x:.2}")
    } else {
        format!("{x:.3}")
    }
}
fn jn(x: f64) -> String {
    if x.is_finite() {
        format!("{x:.6}")
    } else {
        "null".into()
    }
}

/// Simple line plot, log2 y axis. series: (name, colour, points).
fn svg(path: &str, title: &str, xl: &str, yl: &str, series: &[(String, &str, Vec<(f64, f64)>)]) {
    let all: Vec<(f64, f64)> = series.iter().flat_map(|s| s.2.iter().cloned()).filter(|p| p.1.is_finite() && p.1 > 0.0).collect();
    if all.is_empty() {
        return;
    }
    let (x0, x1) = minmax(&all.iter().map(|p| p.0).collect::<Vec<_>>());
    let (y0, y1) = minmax(&all.iter().map(|p| p.1.log2()).collect::<Vec<_>>());
    let (y0, y1) = (y0.floor(), y1.ceil().max(y0.floor() + 1.0));
    let (w, h, ml, mr, mt, mb) = (760.0, 440.0, 70.0, 190.0, 40.0, 50.0);
    let px = |x: f64| ml + (x - x0) / (x1 - x0).max(1e-9) * (w - ml - mr);
    let py = |y: f64| h - mb - (y.log2() - y0) / (y1 - y0) * (h - mt - mb);
    let mut s = String::new();
    write!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"12\"><rect width=\"100%\" height=\"100%\" fill=\"white\"/>").unwrap();
    write!(s, "<text x=\"{ml}\" y=\"20\" font-size=\"14\">{title}</text>").unwrap();
    let mut yt = y0;
    let step = ((y1 - y0) / 10.0).ceil().max(1.0);
    while yt <= y1 + 1e-9 {
        let y = h - mb - (yt - y0) / (y1 - y0) * (h - mt - mb);
        write!(s, "<line x1=\"{ml}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" stroke=\"#ddd\"/><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">2^{yt:.0}</text>", w - mr, ml - 6.0, y + 4.0).unwrap();
        yt += step;
    }
    let mut xt = x0;
    while xt <= x1 + 1e-9 {
        write!(s, "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{xt:.0}</text>", px(xt), h - mb + 18.0).unwrap();
        xt += 1.0;
    }
    write!(s, "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{xl}</text>", (ml + w - mr) / 2.0, h - 10.0).unwrap();
    write!(s, "<text x=\"14\" y=\"{:.1}\" transform=\"rotate(-90 14 {:.1})\" text-anchor=\"middle\">{yl}</text>", h / 2.0, h / 2.0).unwrap();
    for (i, (name, col, pts)) in series.iter().enumerate() {
        let p: Vec<String> = pts.iter().filter(|p| p.1.is_finite() && p.1 > 0.0).map(|p| format!("{:.1},{:.1}", px(p.0), py(p.1))).collect();
        write!(s, "<polyline fill=\"none\" stroke=\"{col}\" stroke-width=\"1.8\" points=\"{}\"/>", p.join(" ")).unwrap();
        for q in &p {
            let (a, b) = q.split_once(',').unwrap();
            write!(s, "<circle cx=\"{a}\" cy=\"{b}\" r=\"3\" fill=\"{col}\"/>").unwrap();
        }
        let ly = mt + 16.0 * i as f64;
        write!(s, "<line x1=\"{:.1}\" y1=\"{ly}\" x2=\"{:.1}\" y2=\"{ly}\" stroke=\"{col}\" stroke-width=\"2\"/><text x=\"{:.1}\" y=\"{:.1}\">{name}</text>", w - mr + 10.0, w - mr + 30.0, w - mr + 36.0, ly + 4.0).unwrap();
    }
    s.push_str("</svg>\n");
    fs::write(path, s).unwrap();
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out = &a[1];
    let read = |p: String| fs::read_to_string(p).unwrap_or_default();
    let small = read(format!("{out}/raw/rows_small.jsonl"));
    let k130 = read(format!("{out}/raw/rows_k130.jsonl"));
    // oracle verdicts of the small-field targets
    let mut oracle: BTreeMap<(u64, u64), (bool, String)> = BTreeMap::new();
    for dir in &a[2..] {
        for l in 5..=13 {
            for line in read(format!("{dir}/targets_l{l}.jsonl")).lines() {
                if let (Some(x), Some(o), Some(k)) = (num(line, "x_r"), raw(line, "oracle_sat"), raw(line, "kind")) {
                    oracle.insert((l, x as u64), (o == "true", k));
                }
            }
        }
    }
    let mut md = String::new();
    let mut js = String::from("{\n");
    let solver_arms = ["merged", "mw", "del", "leaf", "delleaf", "merged-b"];

    // ---------------- small-field series
    writeln!(md, "# Tables (written by summary.rs; every number [pcc:unverified])\n").unwrap();
    writeln!(md, "## Small-field series (curve y^2 + xy = x^3 + x^2 + 1, n = 3l + 2, non-decomposable uniform targets)\n").unwrap();
    writeln!(md, "Arms: `merged` = fastcdcl.rs unmodified; `mw` = the copy with W = 1 and both switches off (control); `del` = keep the 64 newest learnt clauses; `leaf` = learning-free leaf; `delleaf` = both; `merged-b` = second run of `merged` (A/A). `scan` = the lin21 harness's plain scan; `fastleaf` = generic-width port of the baseline leaf of scan_leaf.rs (n odd only).\n").unwrap();
    writeln!(md, "| l | n | targets | arm | median wall ms | median conflicts | wall / scan | wall / fastleaf | clause-prop share | conflicts = merged (targets) |\n|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut ls: Vec<u64> = small.lines().filter_map(|x| num(x, "l")).map(|x| x as u64).collect();
    ls.sort();
    ls.dedup();
    let (mut verdict_bad, mut rows_checked, mut models, mut models_bad, mut fl_agree, mut fl_rows) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ratio_fast: BTreeMap<&str, Vec<(f64, f64)>> = BTreeMap::new();
    let mut ratio_scan: BTreeMap<&str, Vec<(f64, f64)>> = BTreeMap::new();
    let mut js_small = Vec::new();
    let mut aa_lines = String::new();
    for &l in &ls {
        let rows: Vec<&str> = small.lines().filter(|x| num(x, "l") == Some(l as f64)).collect();
        let n = num(rows[0], "n").unwrap();
        // checks on all rows
        for r in &rows {
            let x = num(r, "x_r").unwrap() as u64;
            let Some((osat, _)) = oracle.get(&(l, x)) else { continue };
            let arm = raw(r, "arm").unwrap();
            let sat = raw(r, "status").unwrap() == "sat";
            if arm == "fastleaf" {
                fl_rows += 1;
                fl_agree += (sat == *osat) as u64;
                continue;
            }
            rows_checked += 1;
            verdict_bad += (sat != *osat) as u64;
            if raw(r, "in_base").as_deref() == Some("true") || raw(r, "in_base").as_deref() == Some("false") {
                models += 1;
                let ok = ["in_base", "liftable", "sums_to_target", "chain_ok"].iter().all(|k| raw(r, k).as_deref() == Some("true"));
                models_bad += !ok as u64;
            }
        }
        let nd = |r: &str| {
            let x = num(r, "x_r").unwrap() as u64;
            matches!(oracle.get(&(l, x)), Some((false, k)) if k == "uniform")
        };
        let by = |arm: &str, key: &str| -> Vec<(u64, f64)> {
            rows.iter().filter(|r| nd(r)).filter(|r| raw(r, "arm").as_deref() == Some(arm)).map(|r| (num(r, "x_r").unwrap() as u64, num(r, key).unwrap_or(f64::NAN))).collect()
        };
        let vals = |arm: &str, key: &str| -> Vec<f64> { by(arm, key).iter().map(|p| p.1).collect() };
        let scan = med(&vals("scan", "wall_ms"));
        let fast = med(&vals("fastleaf", "wall_ms"));
        let nt = vals("scan", "wall_ms").len();
        let mconf: BTreeMap<u64, f64> = by("merged", "conflicts").into_iter().collect();
        writeln!(md, "| {l} | {n} | {nt} | scan | {} | - | 1 | {} | - | - |", g(scan), g(scan / fast)).unwrap();
        if fast.is_finite() {
            writeln!(md, "| {l} | {n} | {nt} | fastleaf | {} ({} ns/leaf) | - | {} | 1 | - | - |", g(fast), g(med(&vals("fastleaf", "ns_per_leaf"))), g(fast / scan)).unwrap();
        }
        let mut o = format!("{{\"l\":{l},\"n\":{n},\"targets\":{nt},\"scan_ms\":{},\"fastleaf_ms\":{},\"fastleaf_ns_per_leaf\":{},\"arms\":{{", jn(scan), jn(fast), jn(med(&vals("fastleaf", "ns_per_leaf"))));
        for (i, arm) in solver_arms.iter().enumerate() {
            let w = med(&vals(arm, "wall_ms"));
            let c = med(&vals(arm, "conflicts"));
            let sh = med(&vals(arm, "clause_prop_share"));
            let eq = by(arm, "conflicts").iter().filter(|(x, c)| mconf.get(x) == Some(c)).count();
            writeln!(md, "| {l} | {n} | {nt} | {arm} | {} | {} | {} | {} | {:.1} % | {eq} of {nt} |", g(w), c, g(w / scan), g(w / fast), 100.0 * sh).unwrap();
            if fast.is_finite() {
                ratio_fast.entry(arm).or_default().push((l as f64, w / fast));
            }
            ratio_scan.entry(arm).or_default().push((l as f64, w / scan));
            write!(o, "{}\"{arm}\":{{\"wall_ms\":{},\"conflicts\":{},\"over_scan\":{},\"over_fastleaf\":{},\"clause_prop_share\":{},\"conflicts_equal_merged\":{eq}}}", if i > 0 { "," } else { "" }, jn(w), jn(c), jn(w / scan), jn(w / fast), jn(sh)).unwrap();
        }
        // A/A: merged-b / merged per target
        let m1: BTreeMap<u64, f64> = by("merged", "wall_ms").into_iter().collect();
        let aa: Vec<f64> = by("merged-b", "wall_ms").iter().filter_map(|(x, w)| m1.get(x).map(|w1| w / w1)).collect();
        let (lo, hi) = minmax(&aa);
        writeln!(aa_lines, "| {l} | {} | {} | {} |", g(med(&aa)), g(lo), g(hi)).unwrap();
        write!(o, "}},\"aa_merged_b_over_merged\":{{\"median\":{},\"min\":{},\"max\":{}}}}}", jn(med(&aa)), jn(lo), jn(hi)).unwrap();
        js_small.push(o);
    }
    writeln!(md, "\nA/A band, per-target wall ratio `merged-b` / `merged`:\n\n| l | median | min | max |\n|---|---|---|---|\n{aa_lines}").unwrap();
    // ADDED AFTER THE MEASURED RUN (deviation 1 in README.md): two counts, no existing number changes.
    {
        let mut eq: BTreeMap<String, (u64, u64)> = BTreeMap::new();
        let mut mc: BTreeMap<(u64, u64), f64> = BTreeMap::new();
        for r in small.lines().filter(|r| raw(r, "arm").as_deref() == Some("merged")) {
            mc.insert((num(r, "l").unwrap() as u64, num(r, "x_r").unwrap() as u64), num(r, "conflicts").unwrap());
        }
        for r in small.lines() {
            let arm = raw(r, "arm").unwrap();
            if !["mw", "del", "leaf", "delleaf", "merged-b"].contains(&arm.as_str()) {
                continue;
            }
            let e = eq.entry(arm).or_default();
            e.1 += 1;
            e.0 += (mc.get(&(num(r, "l").unwrap() as u64, num(r, "x_r").unwrap() as u64)) == num(r, "conflicts").as_ref()) as u64;
        }
        let s: Vec<String> = eq.iter().map(|(a, (e, t))| format!("{a} {e} of {t}")).collect();
        writeln!(md, "Conflict count equal to `merged` on ALL targets (decomposable ones included), per arm: {}.\n", s.join("; ")).unwrap();
        let ku: Vec<&str> = k130.lines().filter(|r| raw(r, "kind").as_deref() == Some("uniform") && raw(r, "arm").map(|a| a.starts_with("solver")) == Some(true)).collect();
        let learnt = ku.iter().filter(|r| num(r, "learnt_clauses").unwrap_or(0.0) > 0.0).count();
        let cps = ku.iter().filter_map(|r| num(r, "clause_prop_share")).fold(0.0f64, f64::max);
        writeln!(md, "ECC2K-130 solver rows on uniform targets: {} rows, {learnt} with a learnt clause; largest clause-propagation share {:.2} %.\n", ku.len(), 100.0 * cps).unwrap();
    }
    // slopes and decision
    writeln!(md, "## Slopes of log2(solver wall / reference wall) per unit of l (least squares on cell medians)\n").unwrap();
    writeln!(md, "| arm | vs fastleaf, l = 9, 11, 13 | ratio at l = 13 (fastleaf) | vs scan, l = 9..13 | ratio at l = 13 (scan) |\n|---|---|---|---|---|").unwrap();
    let mut dec_flat = false;
    let mut dec_all_above = true;
    let mut have = false;
    let mut js_sl = Vec::new();
    for arm in ["merged", "mw", "del", "leaf", "delleaf"] {
        let pf: Vec<(f64, f64)> = ratio_fast.get(arm).map(|v| v.iter().filter(|p| p.0 >= 9.0 && p.0 <= 13.0).map(|p| (p.0, p.1.log2())).collect()).unwrap_or_default();
        let ps: Vec<(f64, f64)> = ratio_scan.get(arm).map(|v| v.iter().filter(|p| p.0 >= 9.0 && p.0 <= 13.0).map(|p| (p.0, p.1.log2())).collect()).unwrap_or_default();
        let sf = if pf.len() >= 2 { slope(&pf) } else { f64::NAN };
        let ss = if ps.len() >= 2 { slope(&ps) } else { f64::NAN };
        let r13f = pf.iter().find(|p| p.0 == 13.0).map(|p| p.1.exp2()).unwrap_or(f64::NAN);
        let r13s = ps.iter().find(|p| p.0 == 13.0).map(|p| p.1.exp2()).unwrap_or(f64::NAN);
        writeln!(md, "| {arm} | {sf:+.3} | {} | {ss:+.3} | {} |", g(r13f), g(r13s)).unwrap();
        js_sl.push(format!("\"{arm}\":{{\"slope_vs_fastleaf\":{},\"ratio_l13_fastleaf\":{},\"slope_vs_scan\":{},\"ratio_l13_scan\":{}}}", jn(sf), jn(r13f), jn(ss), jn(r13s)));
        if ["del", "leaf", "delleaf"].contains(&arm) && pf.len() == 3 {
            have = true;
            if sf.abs() <= 0.03 && r13f <= 60.0 {
                dec_flat = true;
            }
            if !(sf > 0.05) {
                dec_all_above = false;
            }
        }
    }
    let decision = if !have {
        "not evaluated (cells l = 9, 11, 13 incomplete)"
    } else if verdict_bad > 0 || models_bad > 0 {
        "void (exactness check failed)"
    } else if dec_flat {
        "ratio flattens"
    } else if dec_all_above {
        "does not flatten"
    } else {
        "inconclusive"
    };
    writeln!(md, "\n**Small-field decision by the pre-registered rule: {decision}.**\n").unwrap();
    writeln!(md, "Checks, small-field: {rows_checked} scan and solver rows compared with the pair oracle, {verdict_bad} verdicts differ; {models} SAT models, {models_bad} fail a model check; fast-leaf port verdict equals the pair oracle on {fl_agree} of {fl_rows} rows.\n").unwrap();

    // ---------------- ECC2K-130 series
    writeln!(md, "## ECC2K-130 series (y^2 + xy = x^3 + 1 over F_2[z]/(z^131 + z^8 + z^3 + z^2 + 1), V = degree < d)\n").unwrap();
    writeln!(md, "Uniform (non-decomposable) targets. `A` = nonzero abscissae in V that lift; pair count = C(A, 2) (the panel's unit); `pairs` enumerates point pairs (both signs, i <= j).\n").unwrap();
    writeln!(md, "| d | targets | A | C(A,2) | median conflicts | conflicts / A | conflicts / C(A,2) | solver ms | fastleaf ms (ns/leaf) | pairs ms (point pairs) | solver / fastleaf | solver / pairs | A/A solver-b / solver median [min, max] |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut ds: Vec<u64> = k130.lines().filter_map(|x| num(x, "d")).map(|x| x as u64).collect();
    ds.sort();
    ds.dedup();
    let (mut kbad, mut ktargets, mut kplanted_sat, mut kplanted, mut kmodels, mut kmodels_bad, mut khits_bad) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut js_k = Vec::new();
    let mut ser: [Vec<(f64, f64)>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for &d in &ds {
        let rows: Vec<&str> = k130.lines().filter(|x| num(x, "d") == Some(d as f64)).collect();
        // verdict agreement per target
        let mut per: BTreeMap<String, Vec<(String, String, String)>> = BTreeMap::new();
        for r in &rows {
            per.entry(raw(r, "x_r").unwrap()).or_default().push((raw(r, "arm").unwrap(), raw(r, "status").unwrap(), raw(r, "kind").unwrap()));
            if raw(r, "hits_verified").as_deref() == Some("false") {
                khits_bad += 1;
            }
            if raw(r, "in_base").as_deref() == Some("true") || raw(r, "in_base").as_deref() == Some("false") {
                kmodels += 1;
                kmodels_bad += !["in_base", "liftable", "sums_to_target"].iter().all(|k| raw(r, k).as_deref() == Some("true")) as u64;
            }
        }
        for v in per.values() {
            ktargets += 1;
            let want = if v[0].2 == "planted" { "sat" } else { "unsat" };
            if v[0].2 == "planted" {
                kplanted += 1;
                kplanted_sat += v.iter().all(|x| x.1 == "sat") as u64;
            }
            if v.iter().any(|x| x.1 != want) {
                kbad += 1;
            }
        }
        let vals = |arm: &str, key: &str| -> Vec<f64> { rows.iter().filter(|r| raw(r, "kind").as_deref() == Some("uniform") && raw(r, "arm").as_deref() == Some(arm)).filter_map(|r| num(r, key)).collect() };
        let pair = |arm: &str, key: &str| -> BTreeMap<String, f64> { rows.iter().filter(|r| raw(r, "kind").as_deref() == Some("uniform") && raw(r, "arm").as_deref() == Some(arm)).filter_map(|r| Some((raw(r, "x_r")?, num(r, key)?))).collect() };
        let (aa, ca2) = (num(rows[0], "A").unwrap(), num(rows[0], "pair_count_CA2").unwrap());
        let conf = med(&vals("solver", "conflicts"));
        let (ws, wf, wp) = (med(&vals("solver", "wall_ms")), med(&vals("fastleaf", "wall_ms")), med(&vals("pairs", "wall_ms")));
        let s1 = pair("solver", "wall_ms");
        let ab: Vec<f64> = pair("solver-b", "wall_ms").iter().filter_map(|(x, w)| s1.get(x).map(|w1| w / w1)).collect();
        let (lo, hi) = minmax(&ab);
        let nt = vals("solver", "wall_ms").len();
        let np = vals("pairs", "wall_ms").len();
        writeln!(
            md,
            "| {d} | {nt} | {aa} | {ca2} | {conf} | {:.3} | {:.5} | {} | {} ({}) | {} ({}; {np} targets) | {} | {} | {} [{}, {}] |",
            conf / aa, conf / ca2, g(ws), g(wf), g(med(&vals("fastleaf", "ns_per_leaf"))), g(wp), med(&vals("pairs", "point_pairs_tested")), g(ws / wf), g(ws / wp), g(med(&ab)), g(lo), g(hi)
        )
        .unwrap();
        ser[0].push((d as f64, ws));
        ser[1].push((d as f64, wf));
        ser[2].push((d as f64, wp));
        js_k.push(format!(
            "{{\"d\":{d},\"targets\":{nt},\"A\":{aa},\"pair_count_CA2\":{ca2},\"median_conflicts\":{},\"conflicts_over_A\":{},\"conflicts_over_CA2\":{},\"solver_ms\":{},\"fastleaf_ms\":{},\"fastleaf_ns_per_leaf\":{},\"pairs_ms\":{},\"pairs_targets\":{np},\"point_pairs_tested\":{},\"solver_over_fastleaf\":{},\"solver_over_pairs\":{},\"aa_median\":{},\"aa_min\":{},\"aa_max\":{},\"clause_prop_share\":{},\"learnt_clauses_median\":{}}}",
            jn(conf), jn(conf / aa), jn(conf / ca2), jn(ws), jn(wf), jn(med(&vals("fastleaf", "ns_per_leaf"))), jn(wp), jn(med(&vals("pairs", "point_pairs_tested"))), jn(ws / wf), jn(ws / wp), jn(med(&ab)), jn(lo), jn(hi),
            jn(med(&vals("solver", "clause_prop_share"))), jn(med(&vals("solver", "learnt_clauses")))
        ));
    }
    if ser[0].len() >= 2 {
        let sl = |v: &Vec<(f64, f64)>| slope(&v.iter().filter(|p| p.1.is_finite()).map(|p| (p.0, p.1.log2())).collect::<Vec<_>>());
        writeln!(md, "\nSlopes of log2(median wall) per unit of d over the cells above: solver {:+.3}, fastleaf {:+.3}, pairs {:+.3}.", sl(&ser[0]), sl(&ser[1]), sl(&ser[2])).unwrap();
    }
    writeln!(md, "\nChecks, ECC2K-130: {ktargets} targets; {kbad} targets with an arm whose verdict differs from the expected one (uniform: UNSAT on every arm, planted: SAT on every arm); planted targets SAT on all arms: {kplanted_sat} of {kplanted}; {kmodels} solver models, {kmodels_bad} fail the lift-and-sum check; rows with an unverified scan/pair hit: {khits_bad}.\n").unwrap();

    // ---------------- differential tests
    writeln!(md, "## Differential tests (seeds 991 and 992, not used for measured rows)\n").unwrap();
    let dt = read(format!("{out}/raw/dt.jsonl"));
    let s = |k: &str| dt.lines().filter_map(|x| num(x, k)).sum::<f64>();
    let diffs: Vec<u64> = (0..5)
        .map(|i| dt.lines().filter_map(|x| raw(x, "conflict_count_diffs_vs_merged")).map(|v| v.trim_matches(|c| c == '[' || c == ']').split(',').nth(i).unwrap().trim().parse::<u64>().unwrap()).sum())
        .collect();
    writeln!(md, "- n <= 63, (n, l) = (10,3), (12,3), (13,3), (14,4), (15,4), (17,5), (20,6): {} fresh instances ({} planted, {} decomposable by the pair oracle); arms mw, del, del1, leaf, delleaf against unmodified fastcdcl and the pair oracle: {} verdict mismatches; {} models checked, {} bad, {} non-liftable on admissible targets; control (`mw`) conflict counts differing from fastcdcl: {}; two-word control differing: {}; instances whose conflict count differs from fastcdcl per arm [mw, del, del1, leaf, delleaf]: {:?}.", s("instances"), s("planted"), s("decomposable"), s("verdict_mismatches"), s("models_checked"), s("bad_models"), s("nonliftable_models_rule_ok"), s("ctrl_conflict_diffs_vs_merged"), s("w2_ctrl_diffs_vs_merged"), diffs).unwrap();
    let dm = read(format!("{out}/raw/dtmw.jsonl"));
    let s2 = |k: &str| dm.lines().filter_map(|x| num(x, k)).sum::<f64>();
    writeln!(md, "- multi-word, (W, n, d) from (1, 17, 5) to (3, 131, 6), {} cells: {} targets ({} decomposable); solver arms mw, delleaf, del1, leaf against point-pair enumeration and the fast-leaf port: {} verdict mismatches; {} models checked, {} bad; {} unverified hits.", dm.lines().count(), s2("targets"), s2("decomposable"), s2("verdict_mismatches"), s2("models_checked"), s2("bad_models"), s2("bad_hits")).unwrap();
    writeln!(md, "- selfcheck: {}", read(format!("{out}/raw/selfcheck.jsonl")).trim()).unwrap();

    // ---------------- figures
    let cols = ["#444444", "#999999", "#1f77b4", "#2ca02c", "#d62728"];
    let mk = |m: &BTreeMap<&str, Vec<(f64, f64)>>, suffix: &str| -> Vec<(String, &str, Vec<(f64, f64)>)> {
        ["merged", "mw", "del", "leaf", "delleaf"].iter().enumerate().filter_map(|(i, a)| m.get(a).map(|v| (format!("{a} / {suffix}"), cols[i], v.clone()))).collect()
    };
    svg(&format!("{out}/fig_small_ratio_fastleaf.svg"), "Small fields: solver wall / fast-leaf scan wall (medians, odd l)", "l (n = 3l + 2)", "ratio", &mk(&ratio_fast, "fastleaf"));
    svg(&format!("{out}/fig_small_ratio_scan.svg"), "Small fields: solver wall / plain scan wall (medians)", "l (n = 3l + 2)", "ratio", &mk(&ratio_scan, "scan"));
    svg(
        &format!("{out}/fig_k130_wall.svg"),
        "ECC2K-130 field (n = 131): median wall per non-decomposable target",
        "d (subspace dimension)",
        "ms",
        &[("solver (delleaf)".to_string(), "#d62728", ser[0].clone()), ("fast scan leaf".to_string(), "#2ca02c", ser[1].clone()), ("point-pair enumeration".to_string(), "#1f77b4", ser[2].clone())],
    );

    // ---------------- results.json
    writeln!(js, "\"tag\":\"[pcc:unverified] local pilot, shared container\",").unwrap();
    writeln!(js, "\"small_decision\":\"{decision}\",").unwrap();
    writeln!(js, "\"small_checks\":{{\"rows_vs_oracle\":{rows_checked},\"verdict_mismatches\":{verdict_bad},\"models\":{models},\"models_bad\":{models_bad},\"fastleaf_rows\":{fl_rows},\"fastleaf_agree_oracle\":{fl_agree}}},").unwrap();
    writeln!(js, "\"small_slopes\":{{{}}},", js_sl.join(",")).unwrap();
    writeln!(js, "\"small\":[\n{}\n],", js_small.join(",\n")).unwrap();
    writeln!(js, "\"k130_checks\":{{\"targets\":{ktargets},\"targets_with_wrong_verdict\":{kbad},\"planted\":{kplanted},\"planted_sat_all_arms\":{kplanted_sat},\"models\":{kmodels},\"models_bad\":{kmodels_bad},\"unverified_hit_rows\":{khits_bad}}},").unwrap();
    writeln!(js, "\"k130\":[\n{}\n],", js_k.join(",\n")).unwrap();
    for (name, body) in [("dt", &dt), ("dtmw", &dm), ("rows_small", &small), ("rows_k130", &k130)] {
        let lines: Vec<&str> = body.lines().filter(|x| x.starts_with('{')).collect();
        writeln!(js, "\"{name}\":[\n{}\n]{}", lines.join(",\n"), if name == "rows_k130" { "" } else { "," }).unwrap();
    }
    js.push_str("}\n");
    fs::write(format!("{out}/results.json"), js).unwrap();
    fs::write(format!("{out}/tables.md"), &md).unwrap();
    print!("{md}");
}
