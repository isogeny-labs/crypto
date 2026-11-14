//! L3 analysis: reads raw/rows_*.jsonl, applies PROTOCOL.md section 5, writes results.json and SVGs.
//! usage: analyze <out_dir> <rows.jsonl>...
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt::Write as _;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

fn gm_ci(logs: &[f64], seed: u64) -> (f64, f64, f64, usize) {
    let n = logs.len();
    if n == 0 {
        return (f64::NAN, f64::NAN, f64::NAN, 0);
    }
    let mean = logs.iter().sum::<f64>() / n as f64;
    let mut rng = Rng(seed);
    let mut bs: Vec<f64> = (0..2000)
        .map(|_| (0..n).map(|_| logs[(rng.next() % n as u64) as usize]).sum::<f64>() / n as f64)
        .collect();
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (mean.exp(), bs[49].exp(), bs[1949].exp(), n)
}

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { 0.5 * (v[n / 2 - 1] + v[n / 2]) }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = &args[1];
    let mut results = serde_json::Map::new();
    let mut table = String::new();
    let mut forest: Vec<(String, String, f64, f64, f64)> = Vec::new();
    let mut traces: Vec<(String, BTreeMap<String, Vec<(usize, f64)>>)> = Vec::new();
    for path in &args[2..] {
        let text = std::fs::read_to_string(path).unwrap();
        let mut header = Value::Null;
        // rows[arm][pos]
        let mut rows: BTreeMap<String, BTreeMap<usize, Value>> = BTreeMap::new();
        for line in text.lines() {
            let v: Value = serde_json::from_str(line).unwrap();
            if v["type"] == "header" {
                header = v;
                continue;
            }
            rows.entry(v["arm"].as_str().unwrap().to_string())
                .or_default()
                .insert(v["pos"].as_u64().unwrap() as usize, v);
        }
        let tag = header["tag"].as_str().unwrap().to_string();
        let nt = header["ntargets"].as_u64().unwrap() as usize;
        let nrep = header["nrepeat"].as_u64().unwrap() as usize;
        let get = |arm: &str, pos: usize, field: &str| -> Option<f64> {
            rows.get(arm).and_then(|r| r.get(&pos)).map(|v| v["total"][field].as_f64().unwrap())
        };
        let dec = |pos: usize| -> bool { rows["A0"][&pos]["decomposable"].as_bool().unwrap() };
        let complete = (1..nt).all(|p| rows.values().all(|r| r.contains_key(&p)));
        let mut cfg = serde_json::Map::new();
        cfg.insert("header".into(), header.clone());
        cfg.insert("complete_stream".into(), json!(complete));
        // all arms must match truth (the harness panics otherwise) and censoring counts
        let mut cens = serde_json::Map::new();
        let mut mism = 0;
        for (arm, r) in &rows {
            let c = r.values().filter(|v| v["censored"].as_bool().unwrap()).count();
            mism += r.values().filter(|v| !v["censored"].as_bool().unwrap() && !v["match_truth"].as_bool().unwrap()).count();
            cens.insert(arm.clone(), json!(c));
        }
        cfg.insert("censored_targets".into(), Value::Object(cens));
        cfg.insert("model_set_mismatches".into(), json!(mism));
        let npos: Vec<usize> = (1..nt).filter(|p| rows.values().all(|r| r.contains_key(p))).collect();
        let nsat = npos.iter().filter(|&&p| dec(p)).count();
        cfg.insert("stream_positions_used".into(), json!(npos.len()));
        cfg.insert("decomposable_targets".into(), json!(nsat));
        let tm: usize = npos.iter().map(|&p| rows["A0"][&p]["truth_models"].as_u64().unwrap() as usize).sum();
        let cvm: usize = npos.iter().map(|&p| rows["A0"][&p]["curve_verified_models"].as_u64().unwrap() as usize).sum();
        cfg.insert("models_total".into(), json!(tm));
        cfg.insert("models_curve_verified".into(), json!(cvm));
        // medians
        let mut med = serde_json::Map::new();
        for arm in rows.keys() {
            let mut m = serde_json::Map::new();
            for field in ["conflicts", "decisions", "propagations"] {
                let all: Vec<f64> = npos.iter().filter_map(|&p| get(arm, p, field)).collect();
                let s: Vec<f64> = npos.iter().filter(|&&p| dec(p)).filter_map(|&p| get(arm, p, field)).collect();
                let u: Vec<f64> = npos.iter().filter(|&&p| !dec(p)).filter_map(|&p| get(arm, p, field)).collect();
                m.insert(field.into(), json!({"all": median(all), "sat_targets": median(s), "unsat_targets": median(u)}));
            }
            med.insert(arm.clone(), Value::Object(m));
        }
        cfg.insert("medians_per_target".into(), Value::Object(med));
        // paired ratios
        let pairs = [("B", "A0"), ("AA", "A0"), ("B", "A1"), ("BS", "B"), ("A1", "A0"), ("CI", "A1"), ("CA", "A1"), ("CK", "A1"), ("CI", "A0"), ("CK", "A0")];
        let mut rat = serde_json::Map::new();
        for (i, (x, y)) in pairs.iter().enumerate() {
            if !rows.contains_key(*x) || !rows.contains_key(*y) {
                continue;
            }
            let mut pm = serde_json::Map::new();
            for field in ["conflicts", "decisions", "propagations"] {
                let mut sub = serde_json::Map::new();
                for (sname, filt) in [("all", 0), ("sat_targets", 1), ("unsat_targets", 2)] {
                    let logs: Vec<f64> = npos
                        .iter()
                        .filter(|&&p| filt == 0 || (filt == 1) == dec(p))
                        .map(|&p| ((get(x, p, field).unwrap() + 1.0) / (get(y, p, field).unwrap() + 1.0)).ln())
                        .collect();
                    let (g, lo, hi, n) = gm_ci(&logs, 0x5eed + i as u64);
                    sub.insert(sname.into(), json!({"gm": g, "lo95": lo, "hi95": hi, "n": n}));
                    if field == "conflicts" && sname == "all" {
                        forest.push((tag.clone(), format!("{}/{}", x, y), g, lo, hi));
                    }
                }
                pm.insert(field.into(), Value::Object(sub));
            }
            rat.insert(format!("{}/{}", x, y), Value::Object(pm));
        }
        cfg.insert("paired_gm_ratios".into(), Value::Object(rat.clone()));
        // positive control: repeat block
        let rep: Vec<usize> = (nt..nt + nrep).filter(|p| rows["B"].contains_key(p) && rows["A1"].contains_key(p)).collect();
        let logs: Vec<f64> = rep.iter().map(|&p| ((get("B", p, "conflicts").unwrap() + 1.0) / (get("A1", p, "conflicts").unwrap() + 1.0)).ln()).collect();
        let (g, lo, hi, n) = gm_ci(&logs, 0xabc);
        let firsts: Vec<f64> = (0..nrep).filter_map(|i| get("B", i, "conflicts")).collect();
        let reps: Vec<f64> = rep.iter().filter_map(|&p| get("B", p, "conflicts")).collect();
        cfg.insert("positive_control".into(), json!({"B_over_A1_repeat_gm": g, "lo95": lo, "hi95": hi, "n": n, "pass": g < 0.5,
            "B_first_occurrence_conflicts": firsts, "B_repeat_conflicts": reps}));
        // determinism: B vs CA at pos 0
        if rows.contains_key("CA") {
            let same = ["conflicts", "decisions", "propagations"].iter().all(|f| get("B", 0, f) == get("CA", 0, f));
            cfg.insert("determinism_B_eq_CA_pos0".into(), json!(same));
        }
        // learned-clause composition
        let mut lc = serde_json::Map::new();
        for arm in ["B", "BS", "CI", "CA", "CK"] {
            if let Some(r) = rows.get(arm) {
                let (mut n, mut ind, mut ro, mut act, mut ap) = (0f64, 0f64, 0f64, 0f64, 0f64);
                let mut hist = [0f64; 8];
                for p in &npos {
                    let l = &r[p]["learned"];
                    n += l["n"].as_f64().unwrap();
                    ind += l["indep"].as_f64().unwrap();
                    ro += l["r_only"].as_f64().unwrap();
                    act += l["act"].as_f64().unwrap();
                    ap += l["applic_sum"].as_f64().unwrap();
                    for (i, h) in l["r_lits_hist"].as_array().unwrap().iter().enumerate() {
                        hist[i] += h.as_f64().unwrap();
                    }
                }
                let carried: Vec<f64> = npos.iter().filter_map(|p| r[p].get("carried").and_then(|c| c.as_f64())).collect();
                lc.insert(arm.into(), json!({"learned": n, "frac_indep": ind / n, "frac_r_mentioning": ro / n, "frac_activation": act / n,
                    "mean_applicability_nonact": ap / (n - act), "r_lits_hist_nonact": hist, "carried_median": median(carried)}));
            }
        }
        cfg.insert("learned_clauses".into(), Value::Object(lc));
        // section 7 band on propagations (per target medians)
        let mut band = serde_json::Map::new();
        for arm in ["A0", "A1", "B"] {
            let p = median(npos.iter().filter_map(|&q| get(arm, q, "propagations")).collect());
            band.insert(arm.into(), json!({"0": 0.0, "0.1": 0.1 * p, "1": p}));
        }
        cfg.insert("gae_band_propagations_per_target".into(), Value::Object(band));
        // verdict per rule (applied to every config; the frozen verdict is C3)
        let r = &rat["B/A0"]["conflicts"]["all"];
        let aa = &rat["AA/A0"]["conflicts"]["all"];
        let (lo, hi) = (r["lo95"].as_f64().unwrap(), r["hi95"].as_f64().unwrap());
        let (alo, ahi) = (aa["lo95"].as_f64().unwrap(), aa["hi95"].as_f64().unwrap());
        let disjoint = hi < alo || lo > ahi;
        let ncen = rows["A0"].values().filter(|v| v["censored"].as_bool().unwrap()).count().max(rows["B"].values().filter(|v| v["censored"].as_bool().unwrap()).count());
        let verdict = if ncen * 10 > nt {
            "INCONCLUSIVE (censoring)"
        } else if hi < 0.8 && disjoint {
            "IMPROVES"
        } else if lo > 1.25 && disjoint {
            "NEGATIVE"
        } else if lo >= 0.8 && hi <= 1.25 {
            "NO-CHANGE"
        } else {
            "INCONCLUSIVE"
        };
        cfg.insert("rule_reading_B_over_A0".into(), json!(verdict));
        writeln!(table, "| {} | {} | {}/{} | {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}] | {:.3} | {} |", tag, header["template_vars"], nsat, npos.len(),
            r["gm"].as_f64().unwrap(), lo, hi, aa["gm"].as_f64().unwrap(), alo, ahi,
            rat["B/A1"]["conflicts"]["all"]["gm"].as_f64().unwrap(), rat["B/A1"]["conflicts"]["all"]["lo95"].as_f64().unwrap(), rat["B/A1"]["conflicts"]["all"]["hi95"].as_f64().unwrap(),
            g, verdict).unwrap();
        // traces
        let mut tr: BTreeMap<String, Vec<(usize, f64)>> = BTreeMap::new();
        for arm in ["A0", "A1", "B", "CI", "CK"] {
            if let Some(rr) = rows.get(arm) {
                tr.insert(arm.into(), rr.iter().map(|(p, v)| (*p, v["total"]["conflicts"].as_f64().unwrap())).collect());
            }
        }
        traces.push((tag.clone(), tr));
        results.insert(tag, Value::Object(cfg));
    }
    std::fs::write(format!("{}/results.json", out_dir), serde_json::to_string_pretty(&Value::Object(results)).unwrap()).unwrap();
    println!("| config | template vars | SAT targets | B/A0 GM [95% CI] | AA/A0 (A/A) | B/A1 (pure reuse) | pos. control B/A1 on repeats | rule reading |");
    println!("|---|---|---|---|---|---|---|---|");
    print!("{}", table);
    write_forest(&format!("{}/fig_ratios.svg", out_dir), &forest);
    write_traces(&format!("{}/fig_traces.svg", out_dir), &traces);
}

fn write_forest(path: &str, rows: &[(String, String, f64, f64, f64)]) {
    let w = 760.0;
    let rh = 18.0;
    let h = 60.0 + rh * rows.len() as f64;
    let (x0, x1) = (260.0, 730.0);
    let (lmin, lmax) = ((1.0f64 / 64.0).log2(), 4.0f64.log2());
    let sx = |v: f64| x0 + (v.max(1.0 / 64.0).min(4.0).log2() - lmin) / (lmax - lmin) * (x1 - x0);
    let mut s = String::new();
    write!(s, "<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='11'><rect width='100%' height='100%' fill='white'/>").unwrap();
    write!(s, "<text x='10' y='16' font-size='13'>Paired geometric-mean conflict ratio per target, 95% bootstrap CI (log2 axis) [pcc:unverified]</text>").unwrap();
    for t in [1.0 / 64.0, 1.0 / 16.0, 0.25, 0.5, 0.8, 1.0, 1.25, 2.0, 4.0] {
        let x = sx(t);
        let col = if t == 1.0 { "#444" } else { "#ddd" };
        write!(s, "<line x1='{x}' y1='30' x2='{x}' y2='{}' stroke='{col}'/><text x='{x}' y='{}' text-anchor='middle'>{}</text>", h - 20.0, h - 6.0, if t < 0.1 { format!("1/{}", (1.0 / t) as i32) } else { format!("{t}") }).unwrap();
    }
    for (i, (tag, pair, g, lo, hi)) in rows.iter().enumerate() {
        let y = 40.0 + rh * i as f64;
        let col = match pair.as_str() { "B/A0" => "#c0392b", "AA/A0" | "BS/B" => "#7f8c8d", "B/A1" => "#2471a3", _ => "#16a085" };
        write!(s, "<text x='10' y='{}'>{} {}</text><line x1='{}' y1='{y}' x2='{}' y2='{y}' stroke='{col}' stroke-width='2'/><circle cx='{}' cy='{y}' r='3.5' fill='{col}'/>", y + 4.0, tag, pair, sx(*lo), sx(*hi), sx(*g)).unwrap();
    }
    s.push_str("</svg>");
    std::fs::write(path, s).unwrap();
}

fn write_traces(path: &str, traces: &[(String, BTreeMap<String, Vec<(usize, f64)>>)]) {
    let pw = 360.0;
    let ph = 220.0;
    let n = traces.len();
    let w = 20.0 + pw * n as f64;
    let h = ph + 70.0;
    let mut s = String::new();
    write!(s, "<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='11'><rect width='100%' height='100%' fill='white'/>").unwrap();
    write!(s, "<text x='10' y='16' font-size='13'>Conflicts per target (full enumeration) by stream position, log10(c+1); positions 40-44 repeat 0-4 [pcc:unverified]</text>").unwrap();
    let cols = [("A0", "#c0392b"), ("A1", "#e67e22"), ("B", "#2471a3"), ("CI", "#16a085"), ("CK", "#8e44ad")];
    for (pi, (tag, tr)) in traces.iter().enumerate() {
        let ox = 20.0 + pw * pi as f64;
        let oy = 40.0;
        let ymax = tr.values().flat_map(|v| v.iter().map(|p| (p.1 + 1.0).log10())).fold(1.0f64, f64::max).ceil();
        let xmax = tr.values().flat_map(|v| v.iter().map(|p| p.0)).max().unwrap_or(1) as f64;
        let sx = |x: f64| ox + 30.0 + x / xmax * (pw - 50.0);
        let sy = |y: f64| oy + ph - 20.0 - y / ymax * (ph - 30.0);
        write!(s, "<text x='{}' y='{}'>{}</text><rect x='{}' y='{}' width='{}' height='{}' fill='none' stroke='#999'/>", ox + 30.0, oy + 4.0, tag, ox + 30.0, oy + 10.0, pw - 50.0, ph - 30.0).unwrap();
        for d in 0..=(ymax as i32) {
            write!(s, "<text x='{}' y='{}' text-anchor='end'>1e{}</text>", ox + 26.0, sy(d as f64) + 4.0, d).unwrap();
        }
        for (arm, col) in cols {
            if let Some(v) = tr.get(arm) {
                let pts: Vec<String> = v.iter().map(|(x, y)| format!("{:.1},{:.1}", sx(*x as f64), sy((y + 1.0).log10()))).collect();
                write!(s, "<polyline fill='none' stroke='{col}' stroke-width='1.3' points='{}'/>", pts.join(" ")).unwrap();
            }
        }
    }
    for (i, (arm, col)) in cols.iter().enumerate() {
        write!(s, "<rect x='{}' y='{}' width='10' height='10' fill='{col}'/><text x='{}' y='{}'>{arm}</text>", 30.0 + 70.0 * i as f64, h - 18.0, 44.0 + 70.0 * i as f64, h - 9.0).unwrap();
    }
    s.push_str("</svg>");
    std::fs::write(path, s).unwrap();
}
