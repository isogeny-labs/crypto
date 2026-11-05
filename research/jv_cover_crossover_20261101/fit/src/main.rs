//! Analysis for research/jv_cover_crossover_20261101.
//! Usage: jv_crossover_fit <study results dir> <predecessor results dir> <out dir>
//! Writes fit.json, fit.txt, table.md, scaling.svg into <out dir>. Every number is computed here.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

// ---------- statistics ----------
fn ols(x: &[f64], y: &[f64]) -> (f64, f64, f64, f64) {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxx: f64 = x.iter().map(|v| (v - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = x.iter().zip(y).map(|(xi, yi)| (yi - a - b * xi).powi(2)).sum();
    let df = n - 2.0;
    let se = if df > 0.0 { (rss / df / sxx).sqrt() } else { f64::NAN };
    (a, b, se, df)
}
fn t975(df: f64) -> f64 {
    let t = [
        (1.0, 12.706), (2.0, 4.303), (3.0, 3.182), (4.0, 2.776), (5.0, 2.571), (6.0, 2.447),
        (7.0, 2.365), (8.0, 2.306), (9.0, 2.262), (10.0, 2.228), (12.0, 2.179), (15.0, 2.131),
        (20.0, 2.086), (25.0, 2.060), (30.0, 2.042), (40.0, 2.021), (60.0, 2.000), (1e9, 1.960),
    ];
    if df < 1.0 { return 12.706; }
    for w in t.windows(2) {
        if df >= w[0].0 && df <= w[1].0 {
            let f = (1.0 / df - 1.0 / w[1].0) / (1.0 / w[0].0 - 1.0 / w[1].0);
            return w[1].1 + f * (w[0].1 - w[1].1);
        }
    }
    1.960
}
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn idx(&mut self, n: usize) -> usize { (self.next() as usize) % n }
}
fn pct(v: &mut Vec<f64>, q: f64) -> f64 {
    if v.is_empty() { return f64::NAN; }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}
fn mean(v: &[f64]) -> f64 { v.iter().sum::<f64>() / v.len() as f64 }
fn sd(v: &[f64]) -> f64 {
    if v.len() < 2 { return f64::NAN; }
    let m = mean(v);
    (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
}
fn gmean(v: &[f64]) -> f64 { (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp() }
fn fit_pts(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let x: Vec<f64> = pts.iter().map(|q| q.0.ln()).collect();
    let y: Vec<f64> = pts.iter().map(|q| q.1.ln()).collect();
    ols(&x, &y)
}
fn verdict(lo: f64, hi: f64) -> &'static str {
    if hi < 1.0 { "below measured rho at p" } else if lo > 1.0 { "above measured rho at p" } else { "not resolved" }
}

// ---------- IO ----------
fn rows(path: &std::path::Path) -> Vec<Value> {
    match fs::read_to_string(path) {
        Ok(s) => serde_json::from_str::<Value>(&s).ok().and_then(|v| v.as_array().cloned()).unwrap_or_default(),
        Err(_) => vec![],
    }
}
fn files_with(dir: &str, prefix: &str) -> Vec<std::path::PathBuf> {
    let mut f: Vec<_> = fs::read_dir(dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| { let n = p.file_name().unwrap().to_string_lossy().to_string(); n.starts_with(prefix) && n.ends_with(".json") })
        .collect();
    f.sort();
    f
}

#[derive(Clone)]
struct Cover { p: f64, seed: u64, s: f64, ok: bool, total_muls: u64, l: f64, c_e: f64 }
fn cover_row(r: &Value) -> Cover {
    let l = r["l"].as_f64().unwrap();
    let c_e = r["c_add_e"].as_f64().unwrap();
    let tm = r["total_muls"].as_u64().unwrap();
    Cover {
        p: r["p"].as_f64().unwrap(), seed: r["seed"].as_u64().unwrap(),
        s: tm as f64 / (c_e * l.sqrt()),
        ok: r["solved"].as_bool().unwrap() && r["correct"].as_bool().unwrap() && r["mismatches"].as_u64().unwrap() == 0,
        total_muls: tm, l, c_e,
    }
}

// a reference arm at one prime: per-instance run samples
#[derive(Clone, Default)]
struct Arm { by_seed: BTreeMap<u64, Vec<f64>>, native_per_sqrt_l: Vec<f64>, }
impl Arm {
    fn all(&self) -> Vec<f64> { self.by_seed.values().flatten().cloned().collect() }
    fn mean(&self) -> f64 { mean(&self.all()) }
    fn boot_mean(&self, rng: &mut Lcg) -> f64 {
        let mut tot = 0.0; let mut n = 0.0;
        for v in self.by_seed.values() { for _ in 0..v.len() { tot += v[rng.idx(v.len())]; n += 1.0; } }
        tot / n
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (rdir, pdir, out) = (&a[1], &a[2], &a[3]);
    let mut txt = String::new();
    macro_rules! say { ($($t:tt)*) => {{ let s = format!($($t)*); txt += &s; txt += "\n"; }} }

    // ---- predecessor ----
    let mut pred: Vec<(Cover, Vec<f64>)> = vec![];
    let mut pred_rho_all: Vec<(f64, f64)> = vec![];
    for f in files_with(pdir, "dlp_p") {
        for r in rows(&f) {
            let c = cover_row(&r);
            let rho: Vec<f64> = r["rho"].as_array().unwrap().iter().map(|x| x["s"].as_f64().unwrap()).collect();
            for s in &rho { pred_rho_all.push((c.p, *s)); }
            pred.push((c, rho));
        }
    }
    let pooled = mean(&pred_rho_all.iter().map(|x| x.1).collect::<Vec<_>>());
    say!("predecessor pooled rho_e S = {pooled:.4} (n={})", pred_rho_all.len());

    // ---- new cover rows ----
    let mut cover: BTreeMap<u64, Vec<Cover>> = BTreeMap::new(); // p -> rows
    let mut failures: Vec<String> = vec![];
    for f in files_with(rdir, "dlp_p") {
        for r in rows(&f) {
            let c = cover_row(&r);
            if !c.ok { failures.push(format!("cover p={} seed={} not ok", c.p, c.seed)); }
            cover.entry(c.p as u64).or_default().push(c);
        }
    }
    // ---- rhodp ----
    let mut rhodp: BTreeMap<u64, Arm> = BTreeMap::new();
    let mut rhodp_threads: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    let mut rhodp_native: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for f in files_with(rdir, "rhodp_p") {
        for r in rows(&f) {
            let p = r["p"].as_u64().unwrap();
            if !r["correct"].as_bool().unwrap() { failures.push(format!("rhodp p={p} seed={} run={} incorrect", r["seed"], r["run"])); continue; }
            rhodp.entry(p).or_default().by_seed.entry(r["seed"].as_u64().unwrap()).or_default().push(r["s"].as_f64().unwrap());
            rhodp_threads.entry(p).or_default().push(r["threads"].as_u64().unwrap());
            // native: one walk-index hash per step (derived) + expected DP lookups steps/2^dp_bits
            let l = r["l"].as_f64().unwrap();
            let steps = r["steps"].as_f64().unwrap();
            let dpb = r["dp_bits"].as_i64().unwrap();
            rhodp_native.entry(p).or_default().push((steps + steps / 2f64.powi(dpb as i32)) / l.sqrt());
        }
    }
    // ---- rhostrong ----
    let mut strong: BTreeMap<u64, Arm> = BTreeMap::new();
    let mut strong_tuned: BTreeMap<u64, Arm> = BTreeMap::new();
    for f in files_with(rdir, "rhostrong_p") {
        for r in rows(&f) {
            let p = r["p"].as_u64().unwrap();
            let seed = r["seed"].as_u64().unwrap();
            if !r["correct"].as_bool().unwrap() || r["capped"].as_bool().unwrap() { failures.push(format!("rhostrong p={p} seed={seed} run={} correct/capped failure", r["run"])); continue; }
            let l = r["l"].as_f64().unwrap();
            let ops = &r["ops"];
            // native count as emitted by the binary: sum of fields named in OpCounts that are native; read "native" if serialised
            let native: f64 = ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| ops[*k].as_f64().unwrap_or(f64::NAN)).sum();
            let a = strong.entry(p).or_default();
            a.by_seed.entry(seed).or_default().push(r["s_harness"].as_f64().unwrap());
            a.native_per_sqrt_l.push(native / l.sqrt());
            strong_tuned.entry(p).or_default().by_seed.entry(seed).or_default().push(r["s_tuned"].as_f64().unwrap());
        }
    }

    // ---- controls ----
    // calibration
    let mut calib = Arm::default();
    for f in files_with(rdir, "calib_p251") {
        for r in rows(&f) {
            if r["correct"].as_bool().unwrap() { calib.by_seed.entry(r["seed"].as_u64().unwrap()).or_default().push(r["s"].as_f64().unwrap()); }
        }
    }
    let pred251: Vec<f64> = pred_rho_all.iter().filter(|x| x.0 == 251.0).map(|x| x.1).collect();
    let cal = calib.all();
    let calib_json;
    if !cal.is_empty() && pred251.len() > 1 {
        let (m1, m2) = (mean(&cal), mean(&pred251));
        let (v1, v2) = (sd(&cal).powi(2) / cal.len() as f64, sd(&pred251).powi(2) / pred251.len() as f64);
        let se = (v1 + v2).sqrt();
        let df = (v1 + v2).powi(2) / (v1 * v1 / (cal.len() as f64 - 1.0) + v2 * v2 / (pred251.len() as f64 - 1.0));
        let d = m1 - m2;
        let (lo, hi) = (d - t975(df) * se, d + t975(df) * se);
        let pass = lo <= 0.0 && 0.0 <= hi;
        say!("CALIBRATION p=251 rhodp mean {m1:.4} sd {:.3} n={} vs rho_e mean {m2:.4} sd {:.3} n={}: diff {d:+.4}, Welch df {df:.1}, 95% interval [{lo:+.3}, {hi:+.3}], contains 0: {pass}; |diff|<0.3: {}", sd(&cal), cal.len(), sd(&pred251), pred251.len(), d.abs() < 0.3);
        calib_json = json!({"rhodp_mean": m1, "rhodp_sd": sd(&cal), "rhodp_n": cal.len(), "rho_e_mean": m2, "rho_e_sd": sd(&pred251), "rho_e_n": pred251.len(),
            "diff": d, "welch_df": df, "ci95": [lo, hi], "pass_interval_contains_0": pass, "abs_diff_lt_0.3": d.abs() < 0.3,
            "per_seed_rhodp_mean": calib.by_seed.iter().map(|(k, v)| (k.to_string(), mean(v))).collect::<BTreeMap<_, _>>()});
    } else { calib_json = Value::Null; say!("CALIBRATION: no data"); }
    // reproduction
    let mut repro_json = Value::Null;
    let mut repro_factor = 1.0f64;
    for f in files_with(rdir, "repro_dlp_p1009") {
        for r in rows(&f) {
            let c = cover_row(&r);
            for (pc, _) in pred.iter().filter(|(q, _)| q.p == c.p && q.seed == c.seed) {
                let same = pc.total_muls == c.total_muls;
                say!("REPRODUCTION p={} seed={}: total_muls new {} predecessor {} identical: {same}", c.p, c.seed, c.total_muls, pc.total_muls);
                repro_factor = c.total_muls as f64 / pc.total_muls as f64;
                repro_json = json!({"ratio_new_over_predecessor": c.total_muls as f64 / pc.total_muls as f64, "p": c.p, "seed": c.seed, "new": c.total_muls, "predecessor": pc.total_muls, "bit_identical": same});
            }
        }
    }

    // ---- per-prime results ----
    let nb = 20000;
    let mut rng = Lcg(20261101);
    let model = |p: f64| (p / 1329.0).powf(-2.05);
    let mut per_prime = vec![];
    let mut table = String::from("| p | seeds | S (cover, measured) per seed | geo-mean S | rhodp S mean ± sd (n) | S/rhodp geo-mean [95% boot] | verdict vs rhodp | rhostrong S mean ± sd (n) | S/rhostrong geo-mean [95% boot] | verdict vs rhostrong | model S/rho (fit 2026-10-24) |\n|---|---|---|---|---|---|---|---|---|---|---|\n");
    let mut new_points: Vec<(f64, u64, f64)> = vec![]; // (p, seed, S) cover
    for (p, cs) in &cover {
        let ss: Vec<f64> = cs.iter().filter(|c| c.ok).map(|c| c.s).collect();
        let gm = gmean(&ss);
        for c in cs.iter().filter(|c| c.ok) { new_points.push((c.p, c.seed, c.s)); }
        let mut entry = json!({"p": p, "n_seeds": ss.len(), "s_per_seed": ss, "s_geo_mean": gm, "model_ratio": model(*p as f64)});
        let mut cells: Vec<String> = vec![];
        for (name, arm) in [("rhodp", rhodp.get(p)), ("rhostrong", strong.get(p))] {
            match arm {
                Some(arm) if !arm.by_seed.is_empty() => {
                    let all = arm.all();
                    let point = gm / arm.mean();
                    let mut bs = vec![];
                    for _ in 0..nb {
                        let g: Vec<f64> = (0..ss.len()).map(|_| ss[rng.idx(ss.len())]).collect();
                        bs.push(gmean(&g) / arm.boot_mean(&mut rng));
                    }
                    let (lo, hi) = (pct(&mut bs.clone(), 0.025), pct(&mut bs.clone(), 0.975));
                    let v = verdict(lo, hi);
                    entry[name] = json!({"s_mean": arm.mean(), "s_sd": sd(&all), "n": all.len(), "n_instances": arm.by_seed.len(), "ratio_geo_mean": point, "ci95_boot": [lo, hi], "verdict": v,
                        "per_seed_ratio": ss.iter().map(|s| s / arm.mean()).collect::<Vec<_>>()});
                    cells.push(format!("{:.3} ± {:.3} ({})", arm.mean(), sd(&all), all.len()));
                    cells.push(format!("**{point:.3}** [{lo:.3}, {hi:.3}]"));
                    cells.push(v.to_string());
                }
                _ => { cells.push("missing".into()); cells.push("missing".into()); cells.push("no verdict".into()); }
            }
        }
        // strong tuned info
        if let Some(t) = strong_tuned.get(p) { entry["rhostrong_s_tuned_mean"] = json!(t.mean()); }
        table += &format!("| {p} | {} | {} | {gm:.3} | {} | {} | {} | {} | {} | {} | {:.2} |\n", ss.len(),
            ss.iter().map(|s| format!("{s:.3}")).collect::<Vec<_>>().join(", "), cells[0], cells[1], cells[2], cells[3], cells[4], cells[5], model(*p as f64));
        say!("p={p}: S per seed {:?} gm {gm:.4}; rhodp {}; rhostrong {}; model {:.3}", ss, cells[..3].join(" | "), cells[3..].join(" | "), model(*p as f64));
        per_prime.push(entry);
    }

    // ---- refit ----
    // points: (p, S/rho, kind) with rho source per variant
    struct P { p: f64, s: f64, new: bool, rho_fixed: f64 }
    let mut pts: Vec<P> = vec![];
    for (c, rho) in &pred {
        if !c.ok { continue; }
        let rf = if rho.is_empty() { pooled } else { mean(rho) };
        pts.push(P { p: c.p, s: c.s, new: false, rho_fixed: rf });
    }
    let n_pred = pts.len();
    for (p, _sd, s) in &new_points { pts.push(P { p: *p, s: *s, new: true, rho_fixed: f64::NAN }); let _ = p; }
    // variant A: predecessor own rho (p<=251) / pooled assumption (503,1009); new primes measured rhodp mean
    // variant B: as A, but predecessor primes 503 and 1009 take the mean rhodp S over all new-prime runs
    // variant C: only points with measured rho (drop 503 and 1009)
    // variant D: new primes only vs rhodp
    // variant E: new primes only vs rhostrong
    let all_new_rhodp: Vec<f64> = rhodp.iter().filter(|(p, _)| cover.contains_key(p)).flat_map(|(_, a)| a.all()).collect();
    let new_rhodp_pool = if all_new_rhodp.is_empty() { f64::NAN } else { mean(&all_new_rhodp) };
    let variants = ["A primary: measured rho wherever it exists (503, 1009 keep the pooled 1.344 assumption)",
                    "B: as A, with 503 and 1009 priced at the new-prime rhodp pooled mean",
                    "C: only points with measured rho (503 and 1009 dropped)",
                    "D: new primes only, vs rhodp",
                    "E: new primes only, vs rhostrong",
                    "F: as A, with every predecessor cover S multiplied by the reproduction ratio (new binary / predecessor binary at p = 1009 seed 1; a uniform-factor assumption)"];
    let mut fit_json = vec![];
    let mut line_params: Vec<(f64, f64)> = vec![];
    for (vi, vname) in variants.iter().enumerate() {
        // build function producing points given bootstrap rho means for new primes
        let build = |rho_new: &BTreeMap<u64, f64>, idxs: &Vec<usize>| -> Vec<(f64, f64)> {
            let mut o = vec![];
            for &i in idxs {
                let q = &pts[i];
                let r = if q.new {
                    match rho_new.get(&(q.p as u64)) { Some(r) => *r, None => continue }
                } else {
                    match vi {
                        0 | 5 => q.rho_fixed,
                        3 | 4 => continue,
                        1 => if q.p == 503.0 || q.p == 1009.0 { new_rhodp_pool } else { q.rho_fixed },
                        2 => if q.p == 503.0 || q.p == 1009.0 { continue } else { q.rho_fixed },
                        _ => continue,
                    }
                };
                o.push((q.p, q.s * (if vi == 5 && !q.new { repro_factor } else { 1.0 }) / r));
            }
            o
        };
        let mut rho_point: BTreeMap<u64, f64> = BTreeMap::new();
        for p in cover.keys() {
            let arm = if vi == 4 { strong.get(p) } else { rhodp.get(p) };
            if let Some(a) = arm { if !a.by_seed.is_empty() { rho_point.insert(*p, a.mean()); } }
        }
        let all_idx: Vec<usize> = (0..pts.len()).collect();
        let pp = build(&rho_point, &all_idx);
        let distinct = { let mut v: Vec<u64> = pp.iter().map(|x| x.0 as u64).collect(); v.sort(); v.dedup(); v.len() };
        if pp.len() < 3 || distinct < 2 { fit_json.push(json!({"variant": vname, "n_points": pp.len(), "note": "not enough points"})); line_params.push((f64::NAN, f64::NAN)); continue; }
        let (aa, bb, se, df) = fit_pts(&pp);
        let tq = t975(df);
        let pstar = |a: f64, b: f64| (-a / b).exp();
        // cluster bootstrap: resample seeds within prime (cluster = prime), resample rho runs at new primes
        let mut by_p: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
        for (i, q) in pts.iter().enumerate() { by_p.entry(q.p as u64).or_default().push(i); }
        let mut bs_b = vec![]; let mut bs_ps = vec![];
        for _ in 0..nb {
            let mut idxs = vec![];
            for v in by_p.values() { for _ in 0..v.len() { idxs.push(v[rng.idx(v.len())]); } }
            let mut rn = BTreeMap::new();
            for p in cover.keys() {
                let arm = if vi == 4 { strong.get(p) } else { rhodp.get(p) };
                if let Some(a) = arm { if !a.by_seed.is_empty() { rn.insert(*p, a.boot_mean(&mut rng)); } }
            }
            let s = build(&rn, &idxs);
            if s.len() < 3 || s.iter().all(|x| x.0 == s[0].0) { continue; }
            let (a2, b2, _, _) = fit_pts(&s);
            if b2 < -0.05 { bs_b.push(b2); bs_ps.push(pstar(a2, b2)); }
        }
        let n_bs = bs_b.len();
        let (bl, bh) = (pct(&mut bs_b.clone(), 0.025), pct(&mut bs_b.clone(), 0.975));
        let (pl, ph) = (pct(&mut bs_ps.clone(), 0.025), pct(&mut bs_ps.clone(), 0.975));
        // fitted-line value at the new primes
        let at: BTreeMap<String, f64> = cover.keys().map(|p| (p.to_string(), (aa + bb * (*p as f64).ln()).exp())).collect();
        say!("FIT {vname}: n={} slope {bb:.3} (se {se:.3}, t-CI95 [{:.3}, {:.3}], boot [{bl:.3}, {bh:.3}], {n_bs} valid resamples) p* {:.0} (boot [{pl:.0}, {ph:.0}]) MODEL extrapolation; line at new primes {:?}", pp.len(), bb - tq * se, bb + tq * se, pstar(aa, bb), at);
        fit_json.push(json!({"variant": vname, "n_points": pp.len(), "intercept": aa, "slope": bb, "slope_se": se, "df": df,
            "slope_ci95_t": [bb - tq * se, bb + tq * se], "slope_ci95_boot": [bl, bh], "p_star": pstar(aa, bb), "p_star_ci95_boot": [pl, ph], "n_valid_boot": n_bs, "line_at_new_primes": at,
            "label": "derived (slope, intercept), model (p_star, line extrapolation)"}));
        line_params.push((aa, bb));
        let _ = n_pred;
    }

    // ---- section 5a ----
    let mut s5a = vec![];
    say!("SECTION 5A (one-sided: only the rho arms emit native counts; the cover arm does not, so no band ranking is made)");
    for (p, cs) in &cover {
        let ss: Vec<f64> = cs.iter().filter(|c| c.ok).map(|c| c.s).collect();
        let gm = gmean(&ss);
        for (name, arm, nat) in [("rhodp", rhodp.get(p), rhodp_native.get(p).map(|v| mean(v))),
                                 ("rhostrong", strong.get(p), strong.get(p).map(|a| mean(&a.native_per_sqrt_l)))] {
            if let (Some(arm), Some(nat)) = (arm, nat) {
                let mut cells = vec![];
                for w in [0.0f64, 0.1, 1.0] {
                    let priced = arm.mean() + w * nat;
                    cells.push(json!({"w": w, "rho_S_priced": priced, "S_over_rho_priced_cover_at_0": gm / priced}));
                }
                say!("  p={p} {name}: native/sqrt(l) per run {nat:.4}; S/rho(rho priced, cover unpriced) at w=0,0.1,1: {}", cells.iter().map(|c| format!("{:.3}", c["S_over_rho_priced_cover_at_0"].as_f64().unwrap())).collect::<Vec<_>>().join(", "));
                s5a.push(json!({"p": p, "arm": name, "native_per_sqrt_l": nat, "band": cells}));
            }
        }
    }

    let out_json = json!({
        "labels": "measured: cover S (total_muls/(c_add_e sqrt l)), rho samples; derived: ratios, intervals, fits; model: p*, fitted line, preregistered predictions",
        "predecessor_pooled_rho_e": pooled, "calibration": calib_json, "reproduction": repro_json,
        "per_prime": per_prime, "fits": fit_json, "section_5a_one_sided": s5a, "failures": failures,
        "rhodp_threads": rhodp_threads,
    });
    fs::write(format!("{out}/fit.json"), serde_json::to_string_pretty(&out_json).unwrap()).unwrap();
    for f in &failures { say!("FAILURE {f}"); }
    fs::write(format!("{out}/fit.txt"), &txt).unwrap();
    fs::write(format!("{out}/table.md"), &table).unwrap();
    print!("{txt}");

    // ---------- SVG ----------
    let (w, h, ml, mr, mt, mb) = (820.0, 560.0, 70.0, 20.0, 70.0, 60.0);
    let xmin = 40.0f64.ln(); let xmax = 2000.0f64.ln();
    let mut all_y: Vec<f64> = vec![];
    for q in &pts { if !q.new { all_y.push(q.s / q.rho_fixed); } }
    for (p, _s, sv) in &new_points { if let Some(a) = rhodp.get(&(*p as u64)) { all_y.push(sv / a.mean()); } }
    let ymax_v = all_y.iter().cloned().fold(2.0, f64::max) * 2.0;
    let ymin_v = all_y.iter().cloned().fold(1.0, f64::min).min(0.3) / 1.5;
    let (ymin, ymax) = (ymin_v.ln(), ymax_v.ln());
    let sx = |p: f64| ml + (p.ln() - xmin) / (xmax - xmin) * (w - ml - mr);
    let sy = |v: f64| h - mb - (v.ln() - ymin) / (ymax - ymin) * (h - mt - mb);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' width='{w}' height='{h}' font-family='sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n");
    s += &format!("<text x='{ml}' y='22' font-size='15' font-weight='bold'>Cover S / rho against p, with measured rho at p = 1301-1601 (weak class, lower bound)</text>\n");
    for tick in [50.0, 100.0, 200.0, 500.0, 1000.0, 1500.0] {
        s += &format!("<line x1='{x}' y1='{}' x2='{x}' y2='{}' stroke='#ddd'/><text x='{x}' y='{}' text-anchor='middle'>{tick}</text>\n", mt, h - mb, h - mb + 18.0, x = sx(tick));
    }
    for dec in [0.1f64, 1.0, 10.0, 100.0, 1000.0] {
        for m in [1.0, 2.0, 5.0] {
            let yv = dec * m;
            if yv >= ymin_v && yv <= ymax_v { s += &format!("<line x1='{ml}' y1='{y}' x2='{}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{yv}</text>\n", w - mr, ml - 6.0, sy(yv) + 4.0, y = sy(yv)); }
        }
    }
    s += &format!("<line x1='{ml}' y1='{y}' x2='{}' y2='{y}' stroke='#c00' stroke-dasharray='6 4'/><text x='{}' y='{}' fill='#c00'>S/rho = 1</text>\n", w - mr, w - mr - 70.0, sy(1.0) - 5.0, y = sy(1.0));
    // predecessor model line (-2.05 through p*=1329) grey dashed
    let line = |a: f64, bb: f64, col: &str, dash: &str| format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{col}' stroke-width='2' stroke-dasharray='{dash}'/>\n", sx(40.0), sy((a + bb * 40f64.ln()).exp()), sx(2000.0), sy((a + bb * 2000f64.ln()).exp()));
    s += &line(2.05 * 1329f64.ln(), -2.05, "#888", "4 3");
    if line_params[0].0.is_finite() { s += &line(line_params[0].0, line_params[0].1, "#1f77b4", ""); }
    // predecessor points
    for q in pts.iter().filter(|q| !q.new) {
        let measured = q.p <= 251.0;
        if measured { s += &format!("<circle cx='{}' cy='{}' r='4' fill='#1f77b4' fill-opacity='0.7' stroke='#fff'/>\n", sx(q.p), sy(q.s / q.rho_fixed)); }
        else { s += &format!("<circle cx='{}' cy='{}' r='4.5' fill='none' stroke='#1f77b4' stroke-width='1.5'/>\n", sx(q.p), sy(q.s / q.rho_fixed)); }
    }
    // new points vs rhodp (orange squares, measured rho), vs rhostrong (green triangles)
    for (p, _sd, sv) in &new_points {
        if let Some(a) = rhodp.get(&(*p as u64)) { let (x, y) = (sx(*p), sy(sv / a.mean())); s += &format!("<rect x='{}' y='{}' width='9' height='9' fill='#e66100' stroke='#fff'/>\n", x - 4.5, y - 4.5); }
        if let Some(a) = strong.get(&(*p as u64)) { let (x, y) = (sx(*p) + 7.0, sy(sv / a.mean())); s += &format!("<polygon points='{},{} {},{} {},{}' fill='#2a9d4b' stroke='#fff'/>\n", x, y - 5.5, x - 5.0, y + 4.0, x + 5.0, y + 4.0); }
    }
    s += &format!("<text x='{}' y='{}'>p (log scale)</text><text x='14' y='{}' transform='rotate(-90 14 {})' text-anchor='middle'>S / rho (log scale)</text>\n", w / 2.0 - 40.0, h - 22.0, h / 2.0, h / 2.0);
    let ly = [("#1f77b4", "filled circle: predecessor, measured own rho_e (p &lt;= 251)"), ("#1f77b4", "open circle: predecessor, ASSUMED pooled rho 1.344 (p = 503, 1009)"),
              ("#e66100", "square: this study, vs measured rhodp mean (per seed)"), ("#2a9d4b", "triangle: this study, vs measured rhostrong mean (per seed, offset right)")];
    for (i, (c, t)) in ly.iter().enumerate() { s += &format!("<text x='{}' y='{}' fill='{c}'>{t}</text>\n", ml + 10.0, 38.0 + 14.0 * i as f64); }
    if line_params[0].0.is_finite() { s += &format!("<text x='{}' y='{}' fill='#1f77b4'>blue line: refit A, slope {:.2}, p* {:.0} (model extrapolation); grey dashed: 2026-10-24 fit</text>\n", ml + 10.0, 38.0 + 14.0 * 4.0, line_params[0].1, (-line_params[0].0 / line_params[0].1).exp()); }
    s += "</svg>\n";
    fs::write(format!("{out}/scaling.svg"), s).unwrap();
}
