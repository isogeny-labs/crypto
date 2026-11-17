//! F1 analysis: applies PROTOCOL.md section 5 to raw/rows_*.jsonl and writes results.json,
//! raw/analysis_table.md and figures/*.svg.
//! usage: analyze <out_dir> <cell>=<rows.jsonl>[+<rows.jsonl>...] ...
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
    let mut bs: Vec<f64> = (0..2000).map(|_| (0..n).map(|_| logs[(rng.next() % n as u64) as usize]).sum::<f64>() / n as f64).collect();
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (mean.exp(), bs[49].exp(), bs[1949].exp(), n)
}
fn mean(v: &[f64]) -> f64 {
    if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 }
}
/// agreement of two phase strings over positions where both are set, restricted to a class
fn agree(a: &str, b: &str, cls: &str, want: char) -> Option<f64> {
    let (mut n, mut k) = (0u64, 0u64);
    for ((x, y), c) in a.bytes().zip(b.bytes()).zip(cls.bytes()) {
        if x == b'.' || y == b'.' || (want != '*' && c as char != want) {
            continue;
        }
        n += 1;
        if x == y {
            k += 1;
        }
    }
    if n == 0 { None } else { Some(k as f64 / n as f64) }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = &args[1];
    let mut results = serde_json::Map::new();
    let mut table = String::new();
    let mut forest: Vec<(String, String, f64, f64, f64)> = Vec::new();
    let mut agr_plot: Vec<(String, String, f64, f64)> = Vec::new();
    for spec in &args[2..] {
        let (cell, files) = spec.split_once('=').unwrap();
        let mut header = Value::Null;
        let mut rows: BTreeMap<String, BTreeMap<usize, Value>> = BTreeMap::new();
        for path in files.split('+') {
            for line in std::fs::read_to_string(path).unwrap().lines() {
                let v: Value = serde_json::from_str(line).unwrap();
                if v["type"] == "header" {
                    header = v;
                    continue;
                }
                let prev = rows.entry(v["arm"].as_str().unwrap().to_string()).or_default().insert(v["pos"].as_u64().unwrap() as usize, v);
                assert!(prev.is_none(), "duplicate row");
            }
        }
        let tag = header["tag"].as_str().unwrap().to_string();
        let nt = header["ntargets"].as_u64().unwrap() as usize;
        let vclass = header["template_var_class"].as_str().unwrap().to_string();
        let get = |arm: &str, pos: usize, field: &str| -> Option<f64> { rows.get(arm).and_then(|r| r.get(&pos)).map(|v| v["total"][field].as_f64().unwrap()) };
        let dec = |pos: usize| -> bool { rows["A0"][&pos]["decomposable"].as_bool().unwrap() };
        let mut cfg = serde_json::Map::new();
        let mut hdr = header.clone();
        hdr.as_object_mut().unwrap().remove("template_var_class");
        cfg.insert("header".into(), hdr);
        cfg.insert("template_vars_core".into(), json!(vclass.bytes().filter(|&c| c == b'c').count()));
        cfg.insert("template_vars_rdep".into(), json!(vclass.bytes().filter(|&c| c == b'r').count()));
        let mut cens = serde_json::Map::new();
        let mut mism = 0;
        let mut nrows = 0;
        for (arm, r) in &rows {
            nrows += r.len();
            cens.insert(arm.clone(), json!(r.values().filter(|v| v["censored"].as_bool().unwrap()).count()));
            mism += r.values().filter(|v| !v["censored"].as_bool().unwrap() && !v["match_truth"].as_bool().unwrap()).count();
        }
        cfg.insert("rows".into(), json!(nrows));
        cfg.insert("censored_targets".into(), Value::Object(cens));
        cfg.insert("model_set_mismatches".into(), json!(mism));
        let npos: Vec<usize> = (1..nt).filter(|p| rows.values().all(|r| r.contains_key(p))).collect();
        cfg.insert("stream_positions_used".into(), json!(npos.len()));
        cfg.insert("decomposable_targets".into(), json!(npos.iter().filter(|&&p| dec(p)).count()));
        // determinism: at position 0 the unseeded W arms must equal A0
        let mut det = serde_json::Map::new();
        for arm in ["WP", "WF", "WR", "WT", "WL"] {
            if let (Some(a), Some(b)) = (get(arm, 0, "conflicts"), get("A0", 0, "conflicts")) {
                det.insert(arm.into(), json!(a == b));
            }
        }
        cfg.insert("pos0_equals_A0".into(), Value::Object(det));
        // means per target
        let mut mp = serde_json::Map::new();
        for arm in rows.keys() {
            let mut m = serde_json::Map::new();
            for field in ["conflicts", "decisions", "propagations"] {
                let v: Vec<f64> = npos.iter().filter_map(|&p| get(arm, p, field)).collect();
                m.insert(field.into(), json!(mean(&v)));
            }
            let wall: Vec<f64> = npos.iter().filter_map(|&p| rows[arm].get(&p).map(|v| v["wall_ms"].as_f64().unwrap())).collect();
            m.insert("wall_ms_note_only".into(), json!(mean(&wall)));
            mp.insert(arm.clone(), Value::Object(m));
        }
        cfg.insert("mean_per_target".into(), Value::Object(mp));
        // paired ratios
        let pairs = [("WP", "A0"), ("AA", "A0"), ("WS", "A0"), ("WR", "A0"), ("WF", "A0"), ("WT", "A0"), ("WL", "A0"), ("B", "A0"), ("WP", "WR"), ("WL", "WT"), ("WT", "WP")];
        let mut rat = serde_json::Map::new();
        writeln!(table, "\n### {cell} ({tag}), positions 1..{}, {} used, {} decomposable\n", nt - 1, npos.len(), npos.iter().filter(|&&p| dec(p)).count()).unwrap();
        writeln!(table, "| pair | conflicts gm [95% CI] | SAT targets | UNSAT targets | propagations gm [95% CI] |").unwrap();
        writeln!(table, "|---|---|---|---|---|").unwrap();
        for (i, (x, y)) in pairs.iter().enumerate() {
            if !rows.contains_key(*x) || !rows.contains_key(*y) {
                continue;
            }
            let mut pm = serde_json::Map::new();
            let mut cells: Vec<String> = Vec::new();
            for field in ["conflicts", "decisions", "propagations"] {
                let mut sub = serde_json::Map::new();
                for (sname, filt) in [("all", 0), ("sat_targets", 1), ("unsat_targets", 2)] {
                    let logs: Vec<f64> = npos
                        .iter()
                        .filter(|&&p| filt == 0 || (filt == 1) == dec(p))
                        .map(|&p| ((get(x, p, field).unwrap() + 1.0) / (get(y, p, field).unwrap() + 1.0)).ln())
                        .collect();
                    let (g, lo, hi, n) = gm_ci(&logs, 0xF1_0000 + i as u64 * 31 + filt as u64);
                    sub.insert(sname.into(), json!({"gm": g, "lo": lo, "hi": hi, "n": n}));
                    if field != "decisions" && (field == "conflicts" || filt == 0) {
                        cells.push(if n == 0 { "–".into() } else { format!("{g:.3} [{lo:.3}, {hi:.3}] (n={n})") });
                    }
                    if field == "conflicts" && filt == 0 {
                        forest.push((cell.to_string(), format!("{x}/{y}"), g, lo, hi));
                    }
                }
                pm.insert(field.into(), Value::Object(sub));
            }
            writeln!(table, "| {x}/{y} | {} | {} | {} | {} |", cells[0], cells[1], cells[2], cells[3]).unwrap();
            rat.insert(format!("{x}/{y}"), Value::Object(pm));
        }
        cfg.insert("paired_ratios".into(), Value::Object(rat));
        // verdict (section 5)
        let r = |k: &str| cfg["paired_ratios"][k]["conflicts"]["all"].clone();
        let wp = r("WP/A0");
        let ws = r("WS/A0");
        let (lo, hi) = (wp["lo"].as_f64().unwrap(), wp["hi"].as_f64().unwrap());
        let raw_verdict = if hi < 1.0 {
            "IMPROVES"
        } else if lo > 1.0 {
            "WORSENS"
        } else if lo >= 0.8 && hi <= 1.25 {
            "NO-CHANGE"
        } else {
            "INCONCLUSIVE"
        };
        cfg.insert("verdict_rule_WP_vs_A0".into(), json!(raw_verdict));
        cfg.insert("positive_control_WS_hi_lt_1".into(), json!(ws["hi"].as_f64().map(|h| h < 1.0)));
        // phase-transfer diagnostics
        let ph = |arm: &str, pos: usize, key: &str| -> Option<String> { rows.get(arm).and_then(|r| r.get(&pos)).and_then(|v| v[key].as_str().map(|s| s.to_string())) };
        let mut diag = serde_json::Map::new();
        let defs: [(&str, Box<dyn Fn(usize) -> Option<(String, String)>>); 5] = [
            ("WP_seed_vs_A0_final_same_target", Box::new(|p| Some((ph("WP", p, "seed_phases")?, ph("A0", p, "final_phases")?)))),
            ("A0_final_prev_vs_A0_final", Box::new(|p| Some((ph("A0", p - 1, "final_phases")?, ph("A0", p, "final_phases")?)))),
            ("AA_final_vs_A0_final_same_target_AA", Box::new(|p| Some((ph("AA", p, "final_phases")?, ph("A0", p, "final_phases")?)))),
            ("WR_seed_vs_A0_final_same_target", Box::new(|p| Some((ph("WR", p, "seed_phases")?, ph("A0", p, "final_phases")?)))),
            ("WP_seed_vs_A0_first_model_sat_targets", Box::new(|p| Some((ph("WP", p, "seed_phases")?, ph("A0", p, "first_model")?)))),
        ];
        for (name, f) in defs.iter() {
            let mut sub = serde_json::Map::new();
            for (cname, want) in [("all", '*'), ("core_vars", 'c'), ("rdep_vars", 'r')] {
                let v: Vec<f64> = npos.iter().filter_map(|&p| f(p)).filter_map(|(a, b)| agree(&a, &b, &vclass, want)).collect();
                sub.insert(cname.into(), json!({"mean": mean(&v), "n_targets": v.len()}));
                if cname != "all" && !v.is_empty() {
                    agr_plot.push((cell.to_string(), format!("{name} [{cname}]"), mean(&v), 0.0));
                }
            }
            diag.insert((*name).into(), Value::Object(sub));
        }
        // correlation: does higher seed agreement go with a lower WP/A0 ratio?
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for &p in &npos {
            if let (Some(a), Some(b)) = (ph("WP", p, "seed_phases"), ph("A0", p, "final_phases")) {
                if let Some(ag) = agree(&a, &b, &vclass, '*') {
                    xs.push(ag);
                    ys.push(((get("WP", p, "conflicts").unwrap() + 1.0) / (get("A0", p, "conflicts").unwrap() + 1.0)).ln());
                }
            }
        }
        if xs.len() > 2 {
            let (mx, my) = (mean(&xs), mean(&ys));
            let cov: f64 = xs.iter().zip(&ys).map(|(a, b)| (a - mx) * (b - my)).sum();
            let vx: f64 = xs.iter().map(|a| (a - mx).powi(2)).sum();
            let vy: f64 = ys.iter().map(|b| (b - my).powi(2)).sum();
            diag.insert("pearson_seed_agreement_vs_log_WP_over_A0".into(), json!({"r": cov / (vx * vy).sqrt(), "n": xs.len()}));
        }
        cfg.insert("phase_diagnostics".into(), Value::Object(diag));
        // WL / WT carried clause statistics
        if let Some(wl) = rows.get("WL") {
            let co: Vec<f64> = npos.iter().filter_map(|p| wl.get(p).and_then(|v| v["carried_out"].as_f64())).collect();
            let ci: Vec<f64> = npos.iter().filter_map(|p| wl.get(p).and_then(|v| v["carried_in"].as_f64())).collect();
            let chk: f64 = wl.values().filter_map(|v| v["taint"]["checked"].as_f64()).sum();
            let der: Vec<f64> = npos.iter().filter_map(|p| wl.get(p).map(|v| v["taint"]["derived"].as_f64().unwrap())).collect();
            let dc: Vec<f64> = npos.iter().filter_map(|p| wl.get(p).map(|v| v["taint"]["derived_clean"].as_f64().unwrap())).collect();
            cfg.insert("WL_carry".into(), json!({"carried_in_mean": mean(&ci), "carried_out_last": co.last(), "derived_mean": mean(&der), "derived_clean_mean": mean(&dc), "clean_share": mean(&dc) / mean(&der), "clauses_checked_against_core": chk}));
        }
        // section 5a band on propagations
        let mut band = serde_json::Map::new();
        for arm in rows.keys() {
            let v: Vec<f64> = npos.iter().filter_map(|&p| get(arm, p, "propagations")).collect();
            let m = mean(&v);
            band.insert(arm.clone(), json!({"0": 0.0, "0.1": 0.1 * m, "1": m}));
        }
        cfg.insert("gae_band_propagations_per_target".into(), Value::Object(band));
        results.insert(cell.to_string(), Value::Object(cfg));
    }
    std::fs::write(format!("{out_dir}/results.json"), serde_json::to_string_pretty(&Value::Object(results)).unwrap()).unwrap();
    std::fs::write(format!("{out_dir}/raw/analysis_table.md"), table).unwrap();
    // forest plot
    let n = forest.len();
    let (w, rowh, left, right) = (760.0, 22.0, 190.0, 40.0);
    let h = 60.0 + rowh * n as f64;
    let (lmin, lmax) = (0.25f64.ln(), 4.0f64.ln());
    let xmap = |v: f64| left + (v.max(0.25).min(4.0).ln() - lmin) / (lmax - lmin) * (w - left - right);
    let mut s = String::new();
    writeln!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"12\">").unwrap();
    writeln!(s, "<rect width=\"100%\" height=\"100%\" fill=\"white\"/><text x=\"10\" y=\"18\" font-size=\"14\">F1: paired geometric-mean conflict ratio, 95% bootstrap CI (log scale). [pcc:unverified]</text>").unwrap();
    for t in [0.25, 0.5, 1.0, 2.0, 4.0] {
        let x = xmap(t);
        writeln!(s, "<line x1=\"{x}\" y1=\"30\" x2=\"{x}\" y2=\"{}\" stroke=\"{}\"/><text x=\"{x}\" y=\"{}\" text-anchor=\"middle\">{t}</text>", h - 20.0, if t == 1.0 { "#444" } else { "#ddd" }, h - 6.0).unwrap();
    }
    for (i, (cell, pair, g, lo, hi)) in forest.iter().enumerate() {
        let y = 40.0 + rowh * i as f64;
        let col = if pair == "WP/A0" { "#c0392b" } else if pair.starts_with("WS") || pair.starts_with("WR") || pair.starts_with("AA") { "#2c7fb8" } else { "#555" };
        writeln!(s, "<text x=\"10\" y=\"{}\">{cell} {pair}</text><line x1=\"{}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"{col}\" stroke-width=\"2\"/><circle cx=\"{}\" cy=\"{y}\" r=\"4\" fill=\"{col}\"/>", y + 4.0, xmap(*lo), xmap(*hi), xmap(*g)).unwrap();
    }
    writeln!(s, "</svg>").unwrap();
    std::fs::write(format!("{out_dir}/figures/fig_ratios.svg"), s).unwrap();
    // agreement bars
    let n = agr_plot.len();
    let h = 60.0 + rowh * n as f64;
    let (left, w) = (430.0, 800.0);
    let mut s = String::new();
    writeln!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"11\">").unwrap();
    writeln!(s, "<rect width=\"100%\" height=\"100%\" fill=\"white\"/><text x=\"10\" y=\"18\" font-size=\"14\">F1: phase agreement (fraction of variables with equal phase). Dashed: 0.5. [pcc:unverified]</text>").unwrap();
    let xm = |v: f64| left + v * (w - left - 30.0);
    writeln!(s, "<line x1=\"{}\" y1=\"30\" x2=\"{}\" y2=\"{}\" stroke=\"#888\" stroke-dasharray=\"4\"/>", xm(0.5), xm(0.5), h - 20.0).unwrap();
    for (i, (cell, name, v, _)) in agr_plot.iter().enumerate() {
        let y = 34.0 + rowh * i as f64;
        writeln!(s, "<text x=\"10\" y=\"{}\">{cell} {name}</text><rect x=\"{left}\" y=\"{y}\" width=\"{}\" height=\"14\" fill=\"{}\"/><text x=\"{}\" y=\"{}\">{v:.3}</text>", y + 11.0, xm(*v) - left, if name.contains("core") { "#2c7fb8" } else { "#e6550d" }, xm(*v) + 4.0, y + 11.0).unwrap();
    }
    writeln!(s, "</svg>").unwrap();
    std::fs::write(format!("{out_dir}/figures/fig_phase_agreement.svg"), s).unwrap();
}
