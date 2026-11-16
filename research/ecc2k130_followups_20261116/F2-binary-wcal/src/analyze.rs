//! `binwcal analyze --raw DIR --out DIR --l2 L2DIR --l4 L4DIR --full-prices FILE`
//!
//! Reads every `raw/measure/*.json` and `raw/linz/*.json`, applies the frozen
//! reading rule of PROTOCOL.md §6, checks the controls of §5, and re-reads
//! L2's m = 2 trade and L4's G-vs-I order at the measured prices (§7).
//! Writes `results.json`, `tables.md` and two SVG figures.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use serde_json::{json, Value};

use crate::{arg, Sm, WORDS};

const BOOT: usize = 2000;

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = v.len();
    if k % 2 == 1 { v[k / 2] } else { 0.5 * (v[k / 2 - 1] + v[k / 2]) }
}

/// Median and percentile-bootstrap 95% interval of the median.
fn med_ci(x: &[f64], rng: &mut Sm) -> (f64, f64, f64) {
    let mut v = x.to_vec();
    let m = median(&mut v);
    let mut boots = Vec::with_capacity(BOOT);
    let mut buf = vec![0.0; x.len()];
    for _ in 0..BOOT {
        for b in buf.iter_mut() {
            *b = x[rng.below(x.len())];
        }
        boots.push(median(&mut buf));
    }
    boots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (m, boots[(BOOT as f64 * 0.025) as usize], boots[(BOOT as f64 * 0.975) as usize])
}

fn fmin(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::INFINITY, f64::min)
}

/// One process's samples, `ops[op][round]` in ns per op.
struct Proc {
    n: u32,
    imp: String,
    rep: u64,
    label: String,
    ops: BTreeMap<String, Vec<f64>>,
}

fn load_procs(dir: &Path) -> Result<Vec<Proc>, String> {
    let mut out = Vec::new();
    let mut names: Vec<_> = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    names.sort();
    for p in names {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&p).map_err(|e| e.to_string())?)
            .map_err(|e| format!("{}: {e}", p.display()))?;
        let mut ops = BTreeMap::new();
        for (k, o) in v["ops"].as_object().ok_or("ops")? {
            ops.insert(k.clone(), o["ns_per_op"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect());
        }
        out.push(Proc {
            n: v["n"].as_u64().unwrap() as u32,
            imp: v["impl"].as_str().unwrap().to_string(),
            rep: v["rep"].as_u64().unwrap(),
            label: v["label"].as_str().unwrap_or("").to_string(),
            ops,
        });
    }
    Ok(out)
}

/// Pooled reading of `num/den` over the processes of one (n, impl):
/// point = median of the per-process medians of the paired per-round ratio,
/// interval = union of the per-process bootstrap 95% intervals,
/// best = min(num) / min(den) over all rounds of all processes (best-of-N).
#[derive(Clone, Copy, Debug)]
struct Reading {
    point: f64,
    lo: f64,
    hi: f64,
    best: f64,
}

fn reading(procs: &[&Proc], num: &str, den: &str, rng: &mut Sm) -> Reading {
    let mut pts = Vec::new();
    let (mut lo, mut hi) = (f64::INFINITY, 0.0f64);
    let (mut nmin, mut dmin) = (f64::INFINITY, f64::INFINITY);
    for p in procs {
        let a = &p.ops[num];
        let b = &p.ops[den];
        let r: Vec<f64> = a.iter().zip(b).map(|(x, y)| x / y).collect();
        let (m, l, h) = med_ci(&r, rng);
        pts.push(m);
        lo = lo.min(l);
        hi = hi.max(h);
        nmin = nmin.min(fmin(a));
        dmin = dmin.min(fmin(b));
    }
    Reading { point: median(&mut pts), lo, hi, best: nmin / dmin }
}

fn rjson(r: &Reading) -> Value {
    json!({"point": r.point, "ci95_union": [r.lo, r.hi], "best_of_n": r.best})
}

fn ns_median(procs: &[&Proc], op: &str) -> (f64, f64, f64) {
    let mut all: Vec<f64> = procs.iter().flat_map(|p| p.ops[op].iter().cloned()).collect();
    let mn = fmin(&all);
    let mx = all.iter().cloned().fold(0.0, f64::max);
    (median(&mut all), mn, mx)
}

fn sci(x: f64) -> String {
    format!("{x:.3e}")
}

/// Nearest measured row width at or above `w` words.
fn width_at_least(w: usize) -> usize {
    *WORDS.iter().find(|&&x| x >= w).unwrap_or(&4096)
}

pub fn run(args: &[String]) -> Result<(), String> {
    let raw = arg(args, "--raw").ok_or("--raw")?;
    let out = arg(args, "--out").ok_or("--out")?;
    let l2 = arg(args, "--l2").ok_or("--l2")?;
    let l4 = arg(args, "--l4").ok_or("--l4")?;
    let full_prices = arg(args, "--full-prices");
    let mut rng = Sm(0xF2_2026_1116);
    let procs = load_procs(&Path::new(&raw).join("measure"))?;
    let mut groups: BTreeMap<(u32, String), Vec<&Proc>> = BTreeMap::new();
    for p in &procs {
        groups.entry((p.n, p.imp.clone())).or_default().push(p);
    }
    let mut md = String::new();
    let mut prices = Vec::new();
    let mut controls = Vec::new();
    let price_ops = ["dbl", "mul_lat", "mul_tp", "sqr_lat", "sqr_tp", "inv_lat", "fadd", "xor_w1", "xor_w4", "xor_w16", "xor_w64", "xor_w97", "xor_w256", "xor_w4096", "null"];
    writeln!(md, "## Table 1: price in GAE (point [union of per-process bootstrap 95% intervals]; best-of-N)\n").unwrap();
    writeln!(md, "| n | impl | GA ns (median) | op | price GAE | 95% union | best-of-N |\n|--:|--|--:|--|--:|--|--:|").unwrap();
    let mut xor64_ns_all = Vec::new();
    let mut by_n_small_ga: BTreeMap<u32, f64> = BTreeMap::new();
    let mut readings: BTreeMap<(u32, String, String, String), Reading> = BTreeMap::new();
    for ((n, imp), ps) in &groups {
        let (ga_ns, ga_min, ga_max) = ns_median(ps, "ga");
        if imp == "small" || imp == "wide" {
            by_n_small_ga.insert(*n, ga_ns);
        }
        let mut row = serde_json::Map::new();
        for w in WORDS {
            let op = format!("xor_w{w}");
            if !price_ops.contains(&op.as_str()) {
                let r = reading(ps, &op, "ga", &mut rng);
                readings.insert((*n, imp.clone(), op.clone(), "ga".into()), r);
                row.insert(op, rjson(&r));
            }
        }
        for op in price_ops.iter() {
            let r = reading(ps, op, "ga", &mut rng);
            readings.insert((*n, imp.clone(), op.to_string(), "ga".into()), r);
            row.insert(op.to_string(), rjson(&r));
            writeln!(md, "| {n} | {imp} | {ga_ns:.1} | {op} | {} | [{}, {}] | {} |", sci(r.point), sci(r.lo), sci(r.hi), sci(r.best)).unwrap();
        }
        // word-XOR-denominated prices (L4's unit), at every width.
        let mut wordunits = serde_json::Map::new();
        for op in ["mul_lat", "mul_tp", "sqr_lat", "sqr_tp", "inv_lat", "fadd", "ga"] {
            let mut per_w = serde_json::Map::new();
            for w in WORDS {
                let r = reading(ps, op, &format!("xor_w{w}"), &mut rng);
                readings.insert((*n, imp.clone(), op.to_string(), format!("xor_w{w}")), r);
                per_w.insert(format!("W{w}"), rjson(&r));
            }
            wordunits.insert(op.to_string(), Value::Object(per_w));
        }
        let mut ns = serde_json::Map::new();
        for op in ps[0].ops.keys() {
            let (m, lo, hi) = ns_median(ps, op);
            ns.insert(op.clone(), json!({"median": m, "min": lo, "max": hi}));
        }
        for p in ps.iter() {
            let mut v = p.ops["xor_w64"].clone();
            xor64_ns_all.push((p.n, p.imp.clone(), p.rep, median(&mut v)));
        }
        // Controls (PROTOCOL §5).
        let aa = reading(ps, "ga_aa", "ga", &mut rng);
        let pc1 = reading(ps, "ga4", "ga", &mut rng);
        let pc2 = reading(ps, "mul8_lat", "mul_lat", &mut rng);
        let nc = reading(ps, "null", "ga", &mut rng);
        let aa_pass = aa.lo >= 0.95 && aa.hi <= 1.05;
        let pc1_pass = pc1.point >= 3.6 && pc1.point <= 4.4;
        let pc2_pass = pc2.point >= 7.2 && pc2.point <= 8.8;
        let nc_pass = nc.hi < 0.01;
        let floor_rel = (1.0 - aa.lo).max(aa.hi - 1.0);
        controls.push(json!({"n": n, "impl": imp, "processes": ps.len(),
            "aa_ga_dup_over_ga": rjson(&aa), "aa_pass": aa_pass, "relative_noise_floor": floor_rel,
            "pc1_ga4_over_ga": rjson(&pc1), "pc1_pass": pc1_pass,
            "pc2_mul8_over_mul": rjson(&pc2), "pc2_pass": pc2_pass,
            "nc_null_over_ga": rjson(&nc), "nc_pass": nc_pass,
            "ga_ns": {"median": ga_ns, "min": ga_min, "max": ga_max}}));
        prices.push(json!({"n": n, "impl": imp, "label": ps[0].label, "processes": ps.len(),
            "ga_ns_median": ga_ns, "gae": Value::Object(row), "word_xor_units": Value::Object(wordunits), "ns": Value::Object(ns)}));
    }
    // Labelled extrapolation (PROTOCOL §4 limits): n = 131 w3 prices rescaled by GA_w3 / GA_wide at n = 83.
    let mut bridged = Vec::new();
    if let (Some(w3_83), Some(wide_83), Some(w3_131)) = (groups.get(&(83, "w3".to_string())), groups.get(&(83, "wide".to_string())), groups.get(&(131, "w3".to_string()))) {
        let k = ns_median(w3_83, "ga").0 / ns_median(wide_83, "ga").0;
        for op in price_ops.iter() {
            let r = readings[&(131, "w3".to_string(), op.to_string(), "ga".to_string())];
            bridged.push(json!({"op": op, "price_w3_unit": rjson(&r), "scale_ga_w3_over_ga_wide_at_83": k,
                "extrapolated_price_ecbench_quality_ga": {"point": r.point * k, "ci95_union": [r.lo * k, r.hi * k]}}));
        }
        writeln!(md, "\nExtrapolation (labelled): GA_w3/GA_wide at n = 83 = {k:.2}; n = 131 prices × {k:.2} estimate the price against an ecbench-quality addition.").unwrap();
    }
    // Drift control: ns per word at W = 64 does not touch the field.
    let xs: Vec<f64> = xor64_ns_all.iter().map(|x| x.3).collect();
    let drift = xs.iter().cloned().fold(0.0, f64::max) / fmin(&xs);
    let drift_pass = drift <= 1.5;

    writeln!(md, "\n## Table 2: controls\n\n| n | impl | A/A GA'/GA [95% union] | pass | PC1 4·GA/GA | pass | PC2 8·mul/mul | pass | NC null/GA (upper) | pass |\n|--:|--|--|--|--:|--|--:|--|--:|--|").unwrap();
    for c in &controls {
        let g = |k: &str| c[k]["point"].as_f64().unwrap();
        let ci = |k: &str| (c[k]["ci95_union"][0].as_f64().unwrap(), c[k]["ci95_union"][1].as_f64().unwrap());
        let (al, ah) = ci("aa_ga_dup_over_ga");
        writeln!(md, "| {} | {} | {:.3} [{:.3}, {:.3}] | {} | {:.2} | {} | {:.2} | {} | {} | {} |",
            c["n"], c["impl"].as_str().unwrap(), g("aa_ga_dup_over_ga"), al, ah, c["aa_pass"], g("pc1_ga4_over_ga"), c["pc1_pass"],
            g("pc2_mul8_over_mul"), c["pc2_pass"], sci(ci("nc_null_over_ga").1), c["nc_pass"]).unwrap();
    }
    writeln!(md, "\nDrift control: max/min over processes of the median ns per word at W = 64: {drift:.3} (pass ≤ 1.5: {drift_pass}).").unwrap();

    // ---- linz: L2's own Macaulay kernel, ns per counted word op.
    let mut linz = Vec::new();
    let l2raw = Path::new(&l2).join("raw");
    let mut linz_files: Vec<_> = std::fs::read_dir(Path::new(&raw).join("linz")).map_err(|e| e.to_string())?
        .filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
    linz_files.sort();
    for f in linz_files {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&f).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        let (n, m, d) = (v["n"].as_u64().unwrap() as u32, v["m"].as_u64().unwrap(), v["d"].as_u64().unwrap());
        // Exactness: our counts equal L2's committed per-target counts.
        let theirs: Value = serde_json::from_str(&std::fs::read_to_string(l2raw.join(format!("linz_n{n}_m{m}_d{d}.json"))).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        let mut mismatches = 0u64;
        let mut checked = 0u64;
        for r in v["rows"].as_array().unwrap() {
            let t = r["target"].as_u64().unwrap() as usize;
            let dd = r["D"].as_u64().unwrap();
            let th = &theirs["rows"][t][format!("D{dd}")];
            checked += 1;
            if th["word_ops"] != r["word_ops"] || th["refuted"] != r["refuted"] || theirs["rows"][t]["xR"] != r["xR"] {
                mismatches += 1;
            }
        }
        let ga = *by_n_small_ga.get(&n).ok_or(format!("no GA measurement at n = {n}"))?;
        for dd in [2u64, 3, 4] {
            let mut per_rep = Vec::new();
            let mut cols = 0u64;
            for rep in 0..v["reps"].as_u64().unwrap() {
                let (mut ns, mut wo) = (0u64, 0u64);
                for r in v["rows"].as_array().unwrap() {
                    if r["D"].as_u64() == Some(dd) && r["rep"].as_u64() == Some(rep) {
                        ns += r["ns"].as_u64().unwrap();
                        wo += r["word_ops"].as_u64().unwrap();
                        cols = r["cols"].as_u64().unwrap();
                    }
                }
                if wo > 0 {
                    per_rep.push(ns as f64 / wo as f64);
                }
            }
            if per_rep.is_empty() {
                continue;
            }
            let lo = fmin(&per_rep);
            let hi = per_rep.iter().cloned().fold(0.0, f64::max);
            linz.push(json!({"n": n, "m": m, "d": d, "D": dd, "cols": cols, "words": cols.div_ceil(64),
                "ns_per_word_op": {"per_rep": per_rep, "min": lo, "max": hi},
                "gae_per_word_op_allin": {"min": lo / ga, "max": hi / ga},
                "ga_ns_median": ga, "count_check": {"checked": checked, "mismatches": mismatches}}));
        }
    }

    // ---- L2 re-read (PROTOCOL §7a).
    let l2res: Value = serde_json::from_str(&std::fs::read_to_string(Path::new(&l2).join("results.json")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let mut l2rows = Vec::new();
    writeln!(md, "\n## Table 3: L2 m = 2 linearisation filter at the measured word-op price (application of a provisional local calibration)\n").unwrap();
    writeln!(md, "| n | d | D | words/row | net GAE/target at 0 | mean word ops | break-even GAE/word op | measured price, pure row XOR [lo, hi] | measured price, L2 kernel all-in [lo, hi] | price / break-even (min) | reading |\n|--:|--:|--:|--:|--:|--:|--:|--|--|--:|--|").unwrap();
    for c in l2res["linearisation"].as_array().unwrap() {
        if c["m"].as_u64() != Some(2) {
            continue;
        }
        let (n, d, dd) = (c["n"].as_u64().unwrap() as u32, c["d"].as_u64().unwrap(), c["D"].as_u64().unwrap());
        let net0 = c["net_gae_at_price_0_0.1_1"][0].as_f64().unwrap();
        let wo = c["mean_word_ops"].as_f64().unwrap();
        let be = c["breakeven_gae_per_word_op"].as_f64().unwrap();
        let lz = linz.iter().find(|x| x["n"].as_u64() == Some(n as u64) && x["d"].as_u64() == Some(d) && x["D"].as_u64() == Some(dd));
        let words = lz.map(|x| x["words"].as_u64().unwrap() as usize).unwrap_or(1);
        let w = width_at_least(words);
        let pure = readings.get(&(n, "small".into(), format!("xor_w{w}"), "ga".into())).copied();
        let allin = lz.map(|x| (x["gae_per_word_op_allin"]["min"].as_f64().unwrap(), x["gae_per_word_op_allin"]["max"].as_f64().unwrap()));
        let lo = pure.map(|r| r.lo).unwrap_or(f64::NAN).min(allin.map(|a| a.0).unwrap_or(f64::INFINITY));
        let hi = pure.map(|r| r.hi).unwrap_or(f64::NAN).max(allin.map(|a| a.1).unwrap_or(0.0));
        let reading = if net0 <= 0.0 {
            "no saving at any price (filter refutes nothing): loses"
        } else if lo > be {
            "filter LOSES at the measured price (whole interval above break-even)"
        } else if hi < be {
            "filter WINS at the measured price"
        } else {
            "unresolved: break-even inside the measured interval"
        };
        let net_at = |p: f64| net0 - wo * p;
        writeln!(md, "| {n} | {d} | {dd} | {words} (W{w}) | {net0:.3} | {} | {} | [{}, {}] | {} | {} | {reading} |",
            sci(wo), sci(be), pure.map(|r| sci(r.lo)).unwrap_or("-".into()), pure.map(|r| sci(r.hi)).unwrap_or("-".into()),
            allin.map(|a| format!("[{}, {}]", sci(a.0), sci(a.1))).unwrap_or("-".into()),
            if be > 0.0 { format!("{:.1}", lo / be) } else { "inf".into() }).unwrap();
        l2rows.push(json!({"n": n, "d": d, "D": dd, "words": words, "measured_width": w, "net_gae_at_0": net0, "mean_word_ops": wo,
            "breakeven": be, "price_pure_xor": pure.map(|r| rjson(&r)), "price_allin_kernel": allin.map(|a| json!([a.0, a.1])),
            "price_interval_used": [lo, hi], "net_gae_at_price_lo_hi": [net_at(lo), net_at(hi)],
            "net_gae_at_10x_hi": net_at(10.0 * hi), "reading": reading,
            "s5a_ranked_at_calibrated_price": false,
            "s5a_note": "the filter saves at w = 0 whenever net0 > 0, so the order at 0, w_cal, 10·w_cal is not one order"}));
    }

    // ---- L4 re-read (PROTOCOL §7b): I native word XORs vs G field ops priced in word XORs.
    let mut l4rows = Vec::new();
    writeln!(md, "\n## Table 4: L4 G (gcd with L_V) vs I (inherited-f4) at measured field-op prices in word XORs\n").unwrap();
    writeln!(md, "| cell n:l:m | targets | mul price in word ops (point; range over lat/tp, W 4–4096, 95% union) | I/G at P1 / P4 / P2n (L4) | I/G measured point | I/G measured range | reading | secondary range, W ≥ 16 |\n|--|--:|--|--|--:|--|--|--|").unwrap();
    let l4res: Value = serde_json::from_str(&std::fs::read_to_string(Path::new(&l4).join("results.json")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let mut gfiles: Vec<_> = std::fs::read_dir(Path::new(&l4).join("raw")).map_err(|e| e.to_string())?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.file_name().unwrap().to_string_lossy().starts_with("g-K-") && p.extension().is_some_and(|x| x == "jsonl"))
        .collect();
    gfiles.sort();
    // Group target lines by (n, l, m).
    let mut cells: BTreeMap<(u32, u32, u32), Vec<Value>> = BTreeMap::new();
    for f in &gfiles {
        for line in std::fs::read_to_string(f).map_err(|e| e.to_string())?.lines() {
            let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            cells.entry((v["n"].as_u64().unwrap() as u32, v["l"].as_u64().unwrap() as u32, v["m"].as_u64().unwrap() as u32)).or_default().push(v);
        }
    }
    for ((n, l, m), ts) in &cells {
        let rd = |op: &str, w: usize| readings.get(&(*n, "small".to_string(), op.to_string(), format!("xor_w{w}"))).copied();
        if rd("mul_lat", 64).is_none() {
            continue;
        }
        // Point: latency variant at W = 64.  Range: every variant's interval.
        let pt = |op: &str| rd(op, 64).unwrap().point;
        let mut lo_p: BTreeMap<&str, f64> = BTreeMap::new();
        let mut hi_p: BTreeMap<&str, f64> = BTreeMap::new();
        for (key, ops) in [("mul", ["mul_lat", "mul_tp"]), ("sqr", ["sqr_lat", "sqr_tp"]), ("inv", ["inv_lat", "inv_lat"]), ("add", ["fadd", "fadd"])] {
            if key == "add" {
                // PROTOCOL §4 limits: low end 1 word op per word of the field (by construction),
                // high end the measured black_box-latency fadd.
                let mut hi = 0.0f64;
                for w in WORDS.iter().filter(|&&w| w >= 4) {
                    hi = hi.max(rd("fadd", *w).unwrap().hi);
                }
                lo_p.insert(key, n.div_ceil(64) as f64);
                hi_p.insert(key, hi);
                continue;
            }
            let (mut lo, mut hi) = (f64::INFINITY, 0.0f64);
            for op in ops {
                for w in WORDS.iter().filter(|&&w| w >= 4) {
                    let r = rd(op, *w).unwrap();
                    lo = lo.min(r.lo);
                    hi = hi.max(r.hi);
                }
            }
            lo_p.insert(key, lo);
            hi_p.insert(key, hi);
        }
        let p_point: BTreeMap<&str, f64> = [("mul", pt("mul_lat")), ("sqr", pt("sqr_lat")), ("inv", pt("inv_lat")), ("add", n.div_ceil(64) as f64)].into_iter().collect();
        let gcost = |g: &Value, p: &BTreeMap<&str, f64>| -> f64 {
            ["mul", "sqr", "inv", "add"].iter().map(|k| g["g"][*k].as_f64().unwrap() * p[k]).sum()
        };
        // Secondary (descriptive): the same range over W >= 16 only.
        let mut lo16 = lo_p.clone();
        let mut hi16 = hi_p.clone();
        for (key, ops) in [("mul", ["mul_lat", "mul_tp"]), ("sqr", ["sqr_lat", "sqr_tp"]), ("inv", ["inv_lat", "inv_lat"])] {
            let (mut lo, mut hi) = (f64::INFINITY, 0.0f64);
            for op in ops {
                for w in WORDS.iter().filter(|&&w| w >= 16) {
                    let r = rd(op, *w).unwrap();
                    lo = lo.min(r.lo);
                    hi = hi.max(r.hi);
                }
            }
            lo16.insert(key, lo);
            hi16.insert(key, hi);
        }
        // Amendment 1 (descriptive, after the data): I/G at four named corners, add = 1 word op.
        let corner = |mul: &str, sqr: &str, w: usize| -> BTreeMap<&str, f64> {
            [("mul", rd(mul, w).unwrap().point), ("sqr", rd(sqr, w).unwrap().point), ("inv", rd("inv_lat", w).unwrap().point), ("add", n.div_ceil(64) as f64)].into_iter().collect()
        };
        let corners = [("lat_W64", corner("mul_lat", "sqr_lat", 64)), ("tp_W64", corner("mul_tp", "sqr_tp", 64)),
            ("lat_W4096", corner("mul_lat", "sqr_lat", 4096)), ("tp_W4096", corner("mul_tp", "sqr_tp", 4096))];
        let mut csum = [0.0f64; 4];
        let (mut l16lo, mut l16hi) = (0.0, 0.0);
        let (mut lsum_pt, mut lsum_lo, mut lsum_hi, mut k) = (0.0, 0.0, 0.0, 0usize);
        let mut budget = 0;
        for t in ts {
            let Some(i) = t["engines"]["inherited-f4"]["ops"].as_array() else { continue };
            let iv = i.iter().map(|x| x.as_f64().unwrap()).fold(f64::INFINITY, f64::min);
            if t["engines"]["inherited-f4"]["verdict"].as_str() == Some("budget") {
                budget += 1;
            }
            lsum_pt += (iv / gcost(t, &p_point)).ln();
            lsum_lo += (iv / gcost(t, &hi_p)).ln();
            lsum_hi += (iv / gcost(t, &lo_p)).ln();
            for (ci, (_, c)) in corners.iter().enumerate() {
                csum[ci] += (iv / gcost(t, c)).ln();
            }
            l16lo += (iv / gcost(t, &hi16)).ln();
            l16hi += (iv / gcost(t, &lo16)).ln();
            k += 1;
        }
        if k == 0 {
            continue;
        }
        let (ig_pt, ig_lo, ig_hi) = ((lsum_pt / k as f64).exp(), (lsum_lo / k as f64).exp(), (lsum_hi / k as f64).exp());
        let l4cell = l4res["summary"]["cells"].as_array().unwrap().iter().find(|c| c["n"].as_u64() == Some(*n as u64) && c["l"].as_u64() == Some(*l as u64) && c["m"].as_u64() == Some(*m as u64));
        let band = l4cell.and_then(|c| {
            let i = c["i_ops_geomean"].as_f64()?;
            Some([i / c["g_P1"].as_f64()?, i / c["g_P4"].as_f64()?, i / c["g_P2n"].as_f64()?])
        });
        let reading = if budget > 0 && ig_lo > 1.0 {
            "G cheaper than I at every measured price (I is a budget lower bound)".to_string()
        } else if budget > 0 {
            "I hit its budget (lower bound); unresolved".to_string()
        } else if ig_lo > 1.0 {
            "G cheaper than I across the measured range".to_string()
        } else if ig_hi < 1.0 {
            "I cheaper than G across the measured range".to_string()
        } else {
            "unresolved inside the measured range".to_string()
        };
        writeln!(md, "| {n}:{l}:{m} | {k} | {:.1} ({:.1}–{:.1}) | {} | {:.3} | [{:.3}, {:.3}] | {reading} | [{:.3}, {:.3}] |",
            p_point["mul"], lo_p["mul"], hi_p["mul"],
            band.map(|b| format!("{:.3} / {:.3} / {:.3}", b[0], b[1], b[2])).unwrap_or("-".into()), ig_pt, ig_lo, ig_hi,
            (l16lo / k as f64).exp(), (l16hi / k as f64).exp()).unwrap();
        l4rows.push(json!({"n": n, "l": l, "m": m, "targets": k, "budget_hits": budget,
            "prices_word_ops_point": p_point, "prices_word_ops_lo": lo_p, "prices_word_ops_hi": hi_p,
            "i_over_g_band_L4": band, "i_over_g_measured": {"point": ig_pt, "lo": ig_lo, "hi": ig_hi}, "reading": reading,
            "amendment1_corners_i_over_g": corners.iter().enumerate().map(|(ci, (name, c))| json!({"corner": name, "prices": c, "i_over_g": (csum[ci] / k as f64).exp()})).collect::<Vec<_>>(),
            "secondary_w_ge_16": {"i_over_g_lo": (l16lo / k as f64).exp(), "i_over_g_hi": (l16hi / k as f64).exp(), "mul_lo": lo16["mul"], "mul_hi": hi16["mul"]}}));
    }

    writeln!(md, "\n### Table 4b (Amendment 1, descriptive): I/G at named price corners (add = 1 word op)\n\n| cell | lat, W64 | tp, W64 | lat, W4096 | tp, W4096 |\n|--|--:|--:|--:|--:|").unwrap();
    for r in &l4rows {
        let c = r["amendment1_corners_i_over_g"].as_array().unwrap();
        writeln!(md, "| {}:{}:{} | {:.3} | {:.3} | {:.3} | {:.3} |", r["n"], r["l"], r["m"],
            c[0]["i_over_g"].as_f64().unwrap(), c[1]["i_over_g"].as_f64().unwrap(), c[2]["i_over_g"].as_f64().unwrap(), c[3]["i_over_g"].as_f64().unwrap()).unwrap();
    }
    // ---- cross-check with the committed full_prices.json (another host).
    let mut xcheck = Vec::new();
    if let Some(fp) = full_prices {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&fp).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        for (slug, row) in v["instances"].as_object().unwrap() {
            let n: u32 = slug.trim_start_matches("icv1-f2m").split('-').next().unwrap().parse().unwrap_or(0);
            let Some(ps) = groups.get(&(n, "small".to_string())) else { continue };
            let mut rr = serde_json::Map::new();
            for (theirs, ours) in [("field_mul", "mul_lat"), ("field_sqr", "sqr_lat"), ("field_inv", "inv_lat"), ("word_xor", "xor_w4096"), ("double", "dbl")] {
                let r = reading(ps, ours, "ga", &mut rng);
                rr.insert(theirs.into(), json!({"full_prices": row[theirs], "ours": r.point, "ratio_ours_over_theirs": r.point / row[theirs].as_f64().unwrap()}));
            }
            xcheck.push(json!({"slug": slug, "n": n, "units": rr}));
        }
    }

    let results = json!({
        "schema": "binwcal.results/v1",
        "pcc": "unverified",
        "method": "best-of-N and median-of-rounds paired timing, one process per (n, impl, rep), core 3 only (taskset -c 3); provisional local calibration",
        "prices": prices, "controls": controls,
        "drift_control": {"xor_w64_ns_median_per_process": xor64_ns_all.iter().map(|x| json!({"n": x.0, "impl": x.1, "rep": x.2, "ns": x.3})).collect::<Vec<_>>(),
            "max_over_min": drift, "pass": drift_pass},
        "bridged_131": bridged, "linz_allin": linz, "l2_reread": l2rows, "l4_reread": l4rows, "full_prices_crosscheck": xcheck,
    });
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    std::fs::write(Path::new(&out).join("results.json"), serde_json::to_string_pretty(&results).unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(Path::new(&out).join("tables.md"), &md).map_err(|e| e.to_string())?;
    std::fs::write(Path::new(&out).join("fig_prices_vs_n.svg"), svg_prices(&groups, &readings)).map_err(|e| e.to_string())?;
    std::fs::write(Path::new(&out).join("fig_l2_breakeven.svg"), svg_l2(&l2rows)).map_err(|e| e.to_string())?;
    eprintln!("wrote results.json, tables.md, 2 SVGs to {out}");
    Ok(())
}

// ---------------------------------------------------------------- SVG
const COLORS: [&str; 7] = ["#1f77b4", "#d62728", "#2ca02c", "#9467bd", "#ff7f0e", "#8c564b", "#7f7f7f"];

fn svg_frame(title: &str, xlabel: &str, ylabel: &str, w: f64, h: f64) -> String {
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"12\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n<text x=\"{}\" y=\"20\" text-anchor=\"middle\" font-size=\"14\">{title}</text>\n<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{xlabel}</text>\n<text transform=\"translate(16,{}) rotate(-90)\" text-anchor=\"middle\">{ylabel}</text>\n",
        w / 2.0, w / 2.0, h - 8.0, h / 2.0)
}

fn svg_prices(groups: &BTreeMap<(u32, String), Vec<&Proc>>, rd: &BTreeMap<(u32, String, String, String), Reading>) -> String {
    let (w, h, l, r, t, b) = (760.0, 480.0, 70.0, 170.0, 40.0, 50.0);
    let (ymin, ymax) = (-5.0f64, 1.0f64);
    let xs: Vec<u32> = groups.keys().map(|k| k.0).collect();
    let (xmin, xmax) = (0.0f64, (*xs.iter().max().unwrap_or(&131) as f64 + 5.0).log2());
    let xmap = |n: f64| l + (n.log2() - xmin.max(3.0)) / (xmax - xmin.max(3.0)) * (w - l - r);
    let ymap = |v: f64| t + (ymax - v.log10()) / (ymax - ymin) * (h - t - b);
    let mut s = svg_frame("Measured price per op in GAE vs n (binary Koblitz, core 3, provisional)", "field degree n (log scale)", "GAE per op (log10)", w, h);
    for e in (ymin as i32)..=(ymax as i32) {
        let y = ymap(10f64.powi(e));
        writeln!(s, "<line x1=\"{l}\" x2=\"{}\" y1=\"{y}\" y2=\"{y}\" stroke=\"#ddd\"/><text x=\"{}\" y=\"{}\" text-anchor=\"end\">1e{e}</text>", w - r, l - 4.0, y + 4.0).unwrap();
    }
    let mut ns: Vec<u32> = xs.clone();
    ns.dedup();
    for n in &ns {
        let x = xmap(*n as f64);
        writeln!(s, "<line x1=\"{x}\" x2=\"{x}\" y1=\"{t}\" y2=\"{}\" stroke=\"#eee\"/><text x=\"{x}\" y=\"{}\" text-anchor=\"middle\">{n}</text>", h - b, h - b + 14.0).unwrap();
    }
    let ops = ["inv_lat", "mul_lat", "sqr_lat", "xor_w64", "xor_w4096", "fadd", "null"];
    for (ci, op) in ops.iter().enumerate() {
        let mut pts = Vec::new();
        for ((n, imp), _) in groups {
            if imp == "w3" && *n != 131 {
                continue;
            }
            if let Some(r) = rd.get(&(*n, imp.clone(), op.to_string(), "ga".into())) {
                pts.push((*n, *r));
            }
        }
        let path: Vec<String> = pts.iter().map(|(n, r)| format!("{:.1},{:.1}", xmap(*n as f64), ymap(r.point.max(1e-6)))).collect();
        writeln!(s, "<polyline fill=\"none\" stroke=\"{}\" stroke-width=\"1.6\" points=\"{}\"/>", COLORS[ci], path.join(" ")).unwrap();
        for (n, r) in &pts {
            let x = xmap(*n as f64);
            writeln!(s, "<line x1=\"{x}\" x2=\"{x}\" y1=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\"/><circle cx=\"{x:.1}\" cy=\"{:.1}\" r=\"2.5\" fill=\"{}\"/>",
                ymap(r.lo.max(1e-6)), ymap(r.hi.max(1e-6)), COLORS[ci], ymap(r.point.max(1e-6)), COLORS[ci]).unwrap();
        }
        let ly = t + 18.0 * ci as f64 + 10.0;
        writeln!(s, "<line x1=\"{}\" x2=\"{}\" y1=\"{ly}\" y2=\"{ly}\" stroke=\"{}\" stroke-width=\"2\"/><text x=\"{}\" y=\"{}\">{op}</text>", w - r + 10.0, w - r + 30.0, COLORS[ci], w - r + 34.0, ly + 4.0).unwrap();
    }
    s.push_str("</svg>\n");
    s
}

fn svg_l2(rows: &[Value]) -> String {
    let (w, h, l, r, t, b) = (760.0, 420.0, 70.0, 20.0, 40.0, 90.0);
    let (ymin, ymax) = (-9.0f64, 0.0f64);
    let rows: Vec<&Value> = rows.iter().filter(|x| x["breakeven"].as_f64().unwrap() > 0.0).collect();
    let k = rows.len().max(1) as f64;
    let ymap = |v: f64| t + (ymax - v.max(1e-9).log10()) / (ymax - ymin) * (h - t - b);
    let mut s = svg_frame("L2 m=2 filter: break-even (x) vs measured word-op price (bar), GAE per word op", "cell n/d/D", "GAE per word op (log10)", w, h);
    for e in (ymin as i32)..=(ymax as i32) {
        let y = ymap(10f64.powi(e));
        writeln!(s, "<line x1=\"{l}\" x2=\"{}\" y1=\"{y}\" y2=\"{y}\" stroke=\"#ddd\"/><text x=\"{}\" y=\"{}\" text-anchor=\"end\">1e{e}</text>", w - r, l - 4.0, y + 4.0).unwrap();
    }
    for (i, c) in rows.iter().enumerate() {
        let x = l + (i as f64 + 0.5) / k * (w - l - r);
        let lo = c["price_interval_used"][0].as_f64().unwrap();
        let hi = c["price_interval_used"][1].as_f64().unwrap();
        let be = c["breakeven"].as_f64().unwrap();
        writeln!(s, "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"{:.1}\" fill=\"#1f77b4\" opacity=\"0.6\"/>", x - 5.0, ymap(hi), (ymap(lo) - ymap(hi)).max(1.5)).unwrap();
        writeln!(s, "<text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#d62728\" font-size=\"14\">x</text>", ymap(be) + 5.0).unwrap();
        writeln!(s, "<text transform=\"translate({x:.1},{}) rotate(60)\" font-size=\"10\">{}/{}/{}</text>", h - b + 8.0, c["n"], c["d"], c["D"]).unwrap();
    }
    s.push_str("</svg>\n");
    s
}
