//! summarize <rawdir> <outdir>: aggregate raw JSONL into results.json, tables (markdown) and SVG graphs.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

fn load(path: &str) -> Vec<Value> {
    match fs::read_to_string(path) {
        Ok(s) => s.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).expect("json")).collect(),
        Err(_) => vec![],
    }
}
fn u(v: &Value, k: &str) -> i64 { v[k].as_i64().unwrap_or_else(|| panic!("field {}", k)) }
fn b(v: &Value, k: &str) -> bool { v[k].as_bool().unwrap_or(false) }
fn s<'a>(v: &'a Value, k: &str) -> &'a str { v[k].as_str().unwrap() }

#[derive(Default, Clone)]
struct Cell { n: usize, rec: usize, short_sum: i64, short2: usize, genuine: i64, det_ok: bool, wp_ok: bool, pred: bool, tr: f64, wx: f64, vanish_ok: bool }

pub fn summarize(raw: &str, out: &str) {
    let mut md = String::new();
    let mut res = serde_json::Map::new();
    // ---------- existence ----------
    let mut ex_rows = vec![];
    let mut l_ex: BTreeMap<usize, f64> = BTreeMap::new();
    md += "## Existence threshold (random R, ordered decompositions)\n\n| n | N | l_min median | l_min q25–q75 | min–max | mean x1 evals |\n|---:|---:|---:|---|---|---:|\n";
    let mut ex_count_md = String::from("\n### Mean cumulative ordered count C(l) vs 2^{2l-n}\n\n| n | l | measured C(l) | predicted 2^{2l-n} | ratio |\n|---:|---:|---:|---:|---:|\n");
    for n in [13usize, 19, 29, 37, 53] {
        let v = load(&format!("{}/exist_n{}.jsonl", raw, n));
        if v.is_empty() { continue; }
        let mut lm: Vec<i64> = v.iter().map(|r| u(r, "l_min")).collect();
        lm.sort();
        let q = |p: f64| lm[((lm.len() - 1) as f64 * p).round() as usize];
        let med = if lm.len() % 2 == 1 { lm[lm.len() / 2] as f64 } else { (lm[lm.len() / 2 - 1] + lm[lm.len() / 2]) as f64 / 2.0 };
        l_ex.insert(n, med);
        let ev: f64 = v.iter().map(|r| u(r, "x1_evals") as f64).sum::<f64>() / v.len() as f64;
        md += &format!("| {} | {} | {} | {}–{} | {}–{} | {:.3e} |\n", n, v.len(), med, q(0.25), q(0.75), lm[0], lm[lm.len() - 1], ev);
        let lmax = u(&v[0], "lmax") as usize;
        let mut cum = vec![0f64; lmax + 1];
        for r in &v { let h = r["hist_ordered"].as_array().unwrap(); let mut c = 0u64; for l in 0..=lmax { c += h[l].as_u64().unwrap(); cum[l] += c as f64; } }
        let mut counts = vec![];
        for l in 1..=lmax {
            let m = cum[l] / v.len() as f64;
            let p = 2f64.powi(2 * l as i32 - n as i32);
            counts.push(json!({"l": l, "mean_cum_ordered": m, "pred": p}));
            if p >= 0.05 { ex_count_md += &format!("| {} | {} | {:.3} | {:.3} | {:.2} |\n", n, l, m, p, m / p); }
        }
        let hist_lmin: BTreeMap<String, usize> = lm.iter().fold(BTreeMap::new(), |mut a, x| { *a.entry(x.to_string()).or_insert(0) += 1; a });
        ex_rows.push(json!({"n": n, "N": v.len(), "l_min_median": med, "l_min_q25": q(0.25), "l_min_q75": q(0.75), "l_min_hist": hist_lmin, "mean_x1_evals": ev, "cum_counts": counts}));
    }
    md += &ex_count_md;
    res.insert("existence".into(), json!(ex_rows));
    // heuristic median for unmeasured n: smallest l with 1-exp(-2^{2l-n-1}) >= 1/2
    let heur_med = |n: usize| -> usize { (1..=n).find(|&l| 1.0 - (-(2f64.powi(2 * l as i32 - n as i32 - 1))).exp() >= 0.5).unwrap() };

    // ---------- lattice ----------
    let mut lat: Vec<Value> = vec![];
    for n in [13usize, 19, 29, 37, 53, 61, 131] { lat.extend(load(&format!("{}/lat_n{}.jsonl", raw, n))); }
    lat.extend(load(&format!("{}/lat_n131_half.jsonl", raw)));
    let highk = { let mut h = load(&format!("{}/highk_n61.jsonl", raw)); h.extend(load(&format!("{}/highk_n131.jsonl", raw))); h };
    let mut cells: BTreeMap<(usize, usize, String, usize), Cell> = BTreeMap::new();
    let mut all_det = true; let mut all_wp = true; let mut all_vanish = true;
    let mut unplanted_genuine = vec![];
    let nskip = lat.iter().chain(highk.iter()).filter(|r| s(r, "arm").starts_with("skip")).count();
    res.insert("skipped_cells_fb_lt2".into(), json!(nskip));
    for r in lat.iter().chain(highk.iter()).filter(|r| !s(r, "arm").starts_with("skip")) {
        let key = (u(r, "n") as usize, u(r, "k") as usize, s(r, "arm").to_string(), u(r, "l") as usize);
        let c = cells.entry(key.clone()).or_insert(Cell { det_ok: true, wp_ok: true, vanish_ok: true, ..Default::default() });
        c.n += 1;
        if b(r, "recovered") { c.rec += 1; }
        let sr = u(r, "short_rows");
        c.short_sum += sr;
        if sr >= 2 { c.short2 += 1; }
        c.genuine += u(r, "genuine");
        c.det_ok &= b(r, "det_identity"); all_det &= b(r, "det_identity");
        c.wp_ok &= b(r, "weak_popov"); all_wp &= b(r, "weak_popov");
        c.vanish_ok &= b(r, "short_rows_vanish_at_planted"); all_vanish &= b(r, "short_rows_vanish_at_planted");
        c.pred = b(r, "pred_ge1_short");
        c.tr += u(r, "transforms") as f64; c.wx += u(r, "word_xors") as f64;
        if key.2.ends_with('u') && u(r, "genuine") > 0 { unplanted_genuine.push(r.clone()); }
    }
    // thresholds
    let mut thr: BTreeMap<(usize, usize, String), (i64, i64, i64, i64, usize)> = BTreeMap::new(); // l_rec, l_rec8, l_short2, l_det, N
    for ((n, k, arm, l), c) in &cells {
        let e = thr.entry((*n, *k, arm.clone())).or_insert((-1, -1, -1, -1, c.n));
        let l = *l as i64;
        if c.rec * 2 >= c.n && c.rec > 0 { e.0 = e.0.max(l); }
        if c.rec * 10 >= 8 * c.n && c.rec > 0 { e.1 = e.1.max(l); }
        if c.short2 * 2 >= c.n && c.short2 > 0 { e.2 = e.2.max(l); }
        if c.pred { e.3 = e.3.max(l); }
    }
    // positive control: s3p recovery >= 8/10 at all l <= l_det (n>=29, k>=2)
    let mut pos_ok = true; let mut pos_fail = vec![];
    for ((n, k, arm, l), c) in &cells {
        if arm == "s3p" && *n >= 29 && *k >= 2 && *k <= 5 && c.pred && c.rec * 10 < 8 * c.n { pos_ok = false; pos_fail.push(format!("n{} k{} l{} {}/{}", n, k, l, c.rec, c.n)); }
    }
    md += "\n## Lattice thresholds (largest l; recovery >= 5/N, >= 8/N; >= 2 short rows in half the trials; floor(det/N) < kn)\n\n| n | k | arm | N | l_rec | l_rec8 | l_short2 | l_det | (l_rec-1)/n |\n|---:|---:|---|---:|---:|---:|---:|---:|---:|\n";
    let mut thr_json = vec![];
    for ((n, k, arm), (lr, lr8, ls2, ld, nn)) in &thr {
        md += &format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", n, k, arm, nn, lr, lr8, ls2, ld, if *lr > 0 { format!("{:.4}", (*lr - 1) as f64 / *n as f64) } else { "-".into() });
        thr_json.push(json!({"n": n, "k": k, "arm": arm, "N": nn, "l_rec": lr, "l_rec8": lr8, "l_short2": ls2, "l_det": ld}));
    }
    res.insert("lattice_thresholds".into(), json!(thr_json));
    let cells_json: Vec<Value> = cells.iter().map(|((n, k, arm, l), c)| json!({"n": n, "k": k, "arm": arm, "l": l, "N": c.n, "recovered": c.rec, "mean_short": c.short_sum as f64 / c.n as f64, "ge2_short": c.short2, "genuine": c.genuine, "pred_ge1_short": c.pred, "mean_transforms": c.tr / c.n as f64, "mean_word_xors": c.wx / c.n as f64})).collect();
    res.insert("lattice_cells".into(), json!(cells_json));

    // best over k per (n, arm)
    let best = |n: usize, arm: &str, kmax: usize| -> (i64, usize) {
        let mut bl = -1; let mut bk = 0;
        for ((nn, k, a), t) in &thr { if *nn == n && a == arm && *k <= kmax && t.0 > bl { bl = t.0; bk = *k; } }
        (bl, bk)
    };
    md += "\n## Gap: existence vs lattice recovery (s3p, best k <= 8)\n\n| n | l_ex (median l_min) | source | best l_rec s3p (k) | gap Δ = l_ex - l_rec | L-ratio (l_ex-1)/(l_rec-1) | (l_rec-1)/n | best genp | best gsp | best e3p | best k2p |\n|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|\n";
    let mut gap_json = vec![];
    let mut svg_pts: Vec<(usize, f64, f64, f64, f64, f64)> = vec![];
    for n in [13usize, 19, 29, 37, 53, 61, 131] {
        let (lex, src) = match l_ex.get(&n) { Some(m) => (*m, "measured"), None => (heur_med(n) as f64, "heuristic 2^{2l-n-1}") };
        let (lr, kk) = best(n, "s3p", 8);
        let g = best(n, "genp", 8).0; let gs = best(n, "gsp", 8).0; let e = best(n, "e3p", 8).0; let k2 = best(n, "k2p", 8).0;
        let ratio = if lr > 1 { (lex - 1.0) / (lr - 1) as f64 } else { f64::INFINITY };
        md += &format!("| {} | {} | {} | {} (k={}) | {} | {:.2} | {:.4} | {} | {} | {} | {} |\n", n, lex, src, lr, kk, lex - lr as f64, ratio, (lr - 1) as f64 / n as f64, g, gs, e, k2);
        gap_json.push(json!({"n": n, "l_ex": lex, "l_ex_source": src, "l_rec_s3p": lr, "k_best": kk, "gap": lex - lr as f64, "L_ratio": ratio, "genp": g, "gsp": gs, "e3p": e, "k2p": k2}));
        svg_pts.push((n, (lex - 1.0) / n as f64, (lr - 1).max(0) as f64 / n as f64, (g - 1).max(0) as f64 / n as f64, (k2 - 1).max(0) as f64 / n as f64, (e - 1).max(0) as f64 / n as f64));
    }
    res.insert("gap".into(), json!(gap_json));

    // controls
    let aa_a = fs::read(format!("{}/aa_lat_n61_k3_a.jsonl", raw)).unwrap_or_default();
    let aa_b = fs::read(format!("{}/aa_lat_n61_k3_b.jsonl", raw)).unwrap_or_default();
    let aa = !aa_a.is_empty() && aa_a == aa_b;
    md += &format!("\n## Controls\n\n- reducer identity (sum row deg = det deg) in every lattice: **{}**\n- weak Popov property in every lattice: **{}**\n- every short row vanishes at the planted root over F_2[t]: **{}**\n- positive control (s3p >= 8/N at every l with floor(det/N) < kn, n >= 29, 2 <= k <= 5): **{}** {:?}\n- genuine roots on unplanted arms: **{}** cells\n- A/A (n = 61, k = 3 run twice, byte-identical): **{}**\n", all_det, all_wp, all_vanish, pos_ok, pos_fail, unplanted_genuine.len(), aa);
    res.insert("controls".into(), json!({"det_identity_all": all_det, "weak_popov_all": all_wp, "short_rows_vanish_all": all_vanish, "positive_control": pos_ok, "positive_control_failures": pos_fail, "unplanted_genuine_rows": unplanted_genuine.len(), "aa_identical": aa}));

    // univariate control
    let mut uni = vec![];
    for d in [2, 3] { for n in [29, 61, 131] { uni.extend(load(&format!("{}/uni_d{}_n{}.jsonl", raw, d, n))); } }
    let mut ucell: BTreeMap<(usize, usize, usize), (i64, i64, bool, i64)> = BTreeMap::new(); // l_rec(all 5), l_det, ok, l_rec_any
    let mut uagg: BTreeMap<(usize, usize, usize, usize), (usize, usize, bool, bool)> = BTreeMap::new();
    for r in &uni {
        let e = uagg.entry((u(r, "n") as usize, u(r, "d") as usize, u(r, "k") as usize, u(r, "l") as usize)).or_insert((0, 0, false, true));
        e.0 += 1; if b(r, "recovered") { e.1 += 1; } e.2 = b(r, "pred_ge1_short"); e.3 &= b(r, "det_identity");
    }
    let mut uni_ok = true;
    for ((n, d, k, l), (nn, rec, pred, det)) in &uagg {
        let e = ucell.entry((*n, *d, *k)).or_insert((-1, -1, true, -1));
        if *rec == *nn { e.0 = e.0.max(*l as i64); }
        if *rec * 2 >= *nn && *rec > 0 { e.3 = e.3.max(*l as i64); }
        if *pred { e.1 = e.1.max(*l as i64); if *rec < *nn { e.2 = false; uni_ok = false; } }
        if !det { uni_ok = false; }
    }
    md += "\n## Univariate control (rigorous: one short row once floor(det/N) < kn)\n\n| n | d | k | l_det | largest l with 5/5 | largest l with >= 3/5 | all l <= l_det recovered |\n|---:|---:|---:|---:|---:|---:|---|\n";
    let mut uj = vec![];
    for ((n, d, k), (lr, ld, ok, la)) in &ucell { md += &format!("| {} | {} | {} | {} | {} | {} | {} |\n", n, d, k, ld, lr, la, ok); uj.push(json!({"n": n, "d": d, "k": k, "l_det": ld, "l_rec_all": lr, "l_rec_half": la, "ok": ok})); }
    res.insert("univariate".into(), json!({"cells": uj, "control_ok": uni_ok}));

    // kobaug
    let mut kob = vec![];
    for n in [29, 61, 131] { kob.extend(load(&format!("{}/kobaug_n{}.jsonl", raw, n)).into_iter().filter(|r| !s(r, "arm").starts_with("skip"))); }
    let keq = kob.iter().filter(|r| b(r, "profiles_equal")).count();
    let kh = kob.iter().all(|r| b(r, "H_eq_g2_mod_f"));
    let kimp = kob.iter().filter(|r| u(r, "short_aug") >= 2 && u(r, "short_plain") < 2).count();
    md += &format!("\n## Koblitz conjugate augmentation (K1)\n\n- lattices compared: {}; reduced profiles identical: {}; H ≡ g^2 mod f coefficientwise: {}; cells where augmentation creates >= 2 short rows from < 2: {}\n\n| n | k | l | trial | added rows | rank plain/aug | sumdeg plain/aug | short plain/aug | equal |\n|---:|---:|---:|---:|---:|---|---|---|---|\n", kob.len(), keq, kh, kimp);
    for r in &kob { md += &format!("| {} | {} | {} | {} | {} | {}/{} | {}/{} | {}/{} | {} |\n", u(r, "n"), u(r, "k"), u(r, "l"), u(r, "trial"), u(r, "added_rows"), u(r, "rank_plain"), u(r, "rank_aug"), u(r, "sumdeg_plain"), u(r, "sumdeg_aug"), u(r, "short_plain"), u(r, "short_aug"), b(r, "profiles_equal")); }
    res.insert("koblitz_aug".into(), json!({"n_lattices": kob.len(), "profiles_equal": keq, "H_eq_g2_mod_f_all": kh, "improving_cells": kimp}));

    fs::create_dir_all(out).unwrap();
    fs::write(format!("{}/results.json", out), serde_json::to_string_pretty(&Value::Object(res)).unwrap()).unwrap();
    fs::write(format!("{}/SUMMARY_TABLES.md", out), md).unwrap();
    fs::write(format!("{}/gap.svg", out), svg_gap(&svg_pts)).unwrap();
    fs::write(format!("{}/recovery_n131.svg", out), svg_rec(&cells, 131)).unwrap();
    fs::write(format!("{}/recovery_n61.svg", out), svg_rec(&cells, 61)).unwrap();
    println!("summarized {} lattice rows, {} highk rows, {} uni rows, {} kobaug rows", lat.len(), highk.len(), uni.len(), kob.len());
}

const COLORS: [&str; 8] = ["#2a6fdb", "#d1495b", "#00798c", "#edae49", "#66a182", "#8d6a9f", "#333333", "#e07a5f"];

fn svg_gap(pts: &[(usize, f64, f64, f64, f64, f64)]) -> String {
    let (w, h, ml, mb, mt, mr) = (720.0, 440.0, 70.0, 60.0, 40.0, 190.0);
    let pw = w - ml - mr; let ph = h - mt - mb;
    let xs = |n: f64| ml + (n - 10.0) / (135.0 - 10.0) * pw;
    let ys = |v: f64| mt + ph - v / 0.6 * ph;
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>", w, h);
    s += &format!("<text x='{}' y='22' font-size='14' font-weight='bold'>Binary S3: existence threshold vs exact F2[t] lattice reach (L/n, L = l-1)</text>", ml);
    for i in 0..=6 { let v = i as f64 * 0.1; s += &format!("<line x1='{}' x2='{}' y1='{y}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{:.1}</text>", ml, ml + pw, ml - 6.0, ys(v) + 4.0, v, y = ys(v)); }
    for n in [13, 19, 29, 37, 53, 61, 131] { s += &format!("<text x='{}' y='{}' text-anchor='middle'>{}</text>", xs(n as f64), mt + ph + 18.0, n); }
    s += &format!("<text x='{}' y='{}' text-anchor='middle'>n (extension degree)</text>", ml + pw / 2.0, h - 15.0);
    s += &format!("<text x='18' y='{}' transform='rotate(-90 18 {})' text-anchor='middle'>(l-1)/n</text>", mt + ph / 2.0, mt + ph / 2.0);
    for (v, lab) in [(1.0 / 6.0, "1/6 owner estimate"), (0.25, "1/4 decision line"), (0.5, "1/2 counting")] { s += &format!("<line x1='{}' x2='{}' y1='{y}' y2='{y}' stroke='#999' stroke-dasharray='4 3'/><text x='{}' y='{}' fill='#666'>{}</text>", ml, ml + pw, ml + pw + 4.0, ys(v) + 4.0, lab, y = ys(v)); }
    let series: [(&str, Box<dyn Fn(&(usize, f64, f64, f64, f64, f64)) -> f64>); 5] = [
        ("existence (median l_min)", Box::new(|p| p.1)), ("s3p lattice (best k)", Box::new(|p| p.2)), ("generic null genp", Box::new(|p| p.3)), ("k2p Frobenius-orbit", Box::new(|p| p.4)), ("e3p symmetric vars", Box::new(|p| p.5))];
    for (i, (lab, f)) in series.iter().enumerate() {
        let path: Vec<String> = pts.iter().map(|p| format!("{:.1},{:.1}", xs(p.0 as f64), ys(f(p)))).collect();
        s += &format!("<polyline fill='none' stroke='{}' stroke-width='2' points='{}'/>", COLORS[i], path.join(" "));
        for p in pts { s += &format!("<circle cx='{:.1}' cy='{:.1}' r='3' fill='{}'/>", xs(p.0 as f64), ys(f(p)), COLORS[i]); }
        s += &format!("<rect x='{}' y='{}' width='12' height='3' fill='{}'/><text x='{}' y='{}'>{}</text>", ml + pw + 4.0, mt + 150.0 + i as f64 * 18.0, COLORS[i], ml + pw + 20.0, mt + 155.0 + i as f64 * 18.0, lab);
    }
    s += "</svg>";
    s
}

fn svg_rec(cells: &BTreeMap<(usize, usize, String, usize), Cell>, n: usize) -> String {
    let (w, h, ml, mb, mt, mr) = (720.0, 420.0, 60.0, 55.0, 40.0, 150.0);
    let pw = w - ml - mr; let ph = h - mt - mb;
    let lmax = cells.keys().filter(|k| k.0 == n && k.3 < 60).map(|k| k.3).max().unwrap_or(10) as f64;
    let xs = |l: f64| ml + (l - 2.0) / (lmax - 2.0).max(1.0) * pw;
    let ys = |v: f64| mt + ph - v * ph;
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>", w, h);
    s += &format!("<text x='{}' y='22' font-size='14' font-weight='bold'>n = {}: planted-root recovery of S3 (s3p) vs l, by multiplicity k</text>", ml, n);
    for i in 0..=4 { let v = i as f64 * 0.25; s += &format!("<line x1='{}' x2='{}' y1='{y}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{:.2}</text>", ml, ml + pw, ml - 6.0, ys(v) + 4.0, v, y = ys(v)); }
    let mut l = 2.0; while l <= lmax { s += &format!("<text x='{}' y='{}' text-anchor='middle'>{}</text>", xs(l), mt + ph + 16.0, l); l += if lmax > 40.0 { 4.0 } else { 2.0 }; }
    s += &format!("<text x='{}' y='{}' text-anchor='middle'>l (factor base: deg x &lt; l)</text>", ml + pw / 2.0, h - 12.0);
    let l6 = n as f64 / 6.0 + 1.0;
    s += &format!("<line x1='{x}' x2='{x}' y1='{}' y2='{}' stroke='#999' stroke-dasharray='4 3'/><text x='{}' y='{}' fill='#666'>L/n = 1/6</text>", mt, mt + ph, xs(l6) + 3.0, mt + 12.0, x = xs(l6));
    for k in 1..=8usize {
        let pts: Vec<(f64, f64)> = cells.iter().filter(|(key, _)| key.0 == n && key.1 == k && key.2 == "s3p" && key.3 < 60).map(|(key, c)| (key.3 as f64, c.rec as f64 / c.n as f64)).collect();
        if pts.is_empty() { continue; }
        let col = COLORS[(k - 1) % 8];
        let path: Vec<String> = pts.iter().map(|p| format!("{:.1},{:.1}", xs(p.0), ys(p.1))).collect();
        s += &format!("<polyline fill='none' stroke='{}' stroke-width='2' points='{}'/>", col, path.join(" "));
        s += &format!("<rect x='{}' y='{}' width='12' height='3' fill='{}'/><text x='{}' y='{}'>k = {}</text>", ml + pw + 6.0, mt + 20.0 + k as f64 * 18.0, col, ml + pw + 22.0, mt + 25.0 + k as f64 * 18.0, k);
    }
    s += "</svg>";
    s
}
