//! # Tables for research/volcano_tall_layers_20261018
//!
//! Reads the study's `ecbench` sessions and the vertex tables written by
//! `crypto volcano sample`, and prints the pre-registered analysis of `PROTOCOL.md` with a JSON
//! twin.  Native; no result is computed anywhere else.
//!
//! Layout: `specs/<vid>/vertices.json` for each volcano `<vid>` (its first letter is the tier),
//! and `sessions/<vid>__<spec>/records.jsonl` for every session run from `specs/<vid>/<spec>.json`.
//!
//! The unit is one solve: a measured, non-warm-up `rho-neg` record.  ecbench gives every round
//! its own algorithm seed, so each record is an independent walk.  A solve's value is
//! `S = total_gae / √r`, divided by its volcano's mean `S` (every level weighted by its solves),
//! so volcanoes of different sizes pool without the larger one dominating a group.  Intervals
//! are 95% percentile intervals of a stratified hierarchical bootstrap (within each
//! (volcano, level) stratum, curves with replacement, then solves within curve), 4000
//! resamples, fixed seed.  Groups: crater = level 0, middle = levels 1..k-1, floor = level k.
//! The slope is the least-squares slope of normalised `S` on `level / k` (the change from
//! crater to floor, in % of the mean).  The distribution test is the two-sample
//! Kolmogorov–Smirnov statistic between crater and floor, with a p-value from 4000 permutations
//! of the crater/floor labels within each volcano.  The positive control is the paired
//! `Σ S(rho-plain) / Σ S(rho-neg)` over the control sessions' (workload, round) pairs.
//!
//! ```bash
//! cargo run --release --example volcano_tall_analyze -- \
//!   --study research/volcano_tall_layers_20261018 --margin 0.05 \
//!   --out research/volcano_tall_layers_20261018/TABLES.md \
//!   --json research/volcano_tall_layers_20261018/tables.json
//! ```

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde_json::{json, Value};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

const B: usize = 4000;

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn pct(v: &mut [f64], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[((v.len() as f64 - 1.0) * q).round() as usize]
}
fn ci(mut v: Vec<f64>) -> (f64, f64) {
    (pct(&mut v.clone(), 0.025), pct(&mut v, 0.975))
}

/// One volcano: its depth and, per level, its curves' solve values (raw S).
struct Volcano {
    id: String,
    tier: char,
    depth: u32,
    ell: u64,
    d: i64,
    m: u64,
    p: u64,
    r: u64,
    levels: BTreeMap<u32, Vec<Vec<f64>>>,
    n1: BTreeMap<u32, u64>,
}

impl Volcano {
    fn resample(&self, rng: &mut Rng) -> BTreeMap<u32, Vec<Vec<f64>>> {
        self.levels
            .iter()
            .map(|(&l, curves)| {
                let out = (0..curves.len())
                    .map(|_| {
                        let c = &curves[rng.below(curves.len())];
                        (0..c.len()).map(|_| c[rng.below(c.len())]).collect()
                    })
                    .collect();
                (l, out)
            })
            .collect()
    }
}

/// Normalised solves of one volcano, flattened per level.
fn normalise(levels: &BTreeMap<u32, Vec<Vec<f64>>>) -> BTreeMap<u32, Vec<f64>> {
    let all: Vec<f64> = levels.values().flatten().flatten().copied().collect();
    let g = mean(&all);
    levels.iter().map(|(&l, cs)| (l, cs.iter().flatten().map(|x| x / g).collect())).collect()
}

struct Stats {
    crater: f64,
    middle: f64,
    floor: f64,
    slope: f64,
}

fn stats(vols: &[(&Volcano, BTreeMap<u32, Vec<f64>>)]) -> Stats {
    let (mut c, mut m, mut f) = (Vec::new(), Vec::new(), Vec::new());
    let mut pts = Vec::new();
    for (v, lv) in vols {
        for (&l, xs) in lv {
            let tgt = if l == 0 {
                &mut c
            } else if l == v.depth {
                &mut f
            } else {
                &mut m
            };
            tgt.extend_from_slice(xs);
            let x = l as f64 / v.depth as f64;
            pts.extend(xs.iter().map(|&y| (x, y)));
        }
    }
    let n = pts.len() as f64;
    let (mx, my) = (pts.iter().map(|p| p.0).sum::<f64>() / n, pts.iter().map(|p| p.1).sum::<f64>() / n);
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx).powi(2)).sum();
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    Stats { crater: mean(&c), middle: mean(&m), floor: mean(&f), slope: 100.0 * sxy / sxx / my }
}

fn ks(a: &[f64], b: &[f64]) -> f64 {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.sort_by(|x, y| x.total_cmp(y));
    b.sort_by(|x, y| x.total_cmp(y));
    let (mut i, mut j, mut d) = (0usize, 0usize, 0f64);
    while i < a.len() && j < b.len() {
        let x = a[i].min(b[j]);
        while i < a.len() && a[i] <= x {
            i += 1;
        }
        while j < b.len() && b[j] <= x {
            j += 1;
        }
        d = d.max((i as f64 / a.len() as f64 - j as f64 / b.len() as f64).abs());
    }
    d
}

fn reading(lo: f64, hi: f64, null: f64, margin_lo: f64, margin_hi: f64) -> &'static str {
    let contains = lo <= null && null <= hi;
    let inside = margin_lo <= lo && hi <= margin_hi;
    match (contains, inside) {
        (true, true) => "equivalent within the margin",
        (true, false) => "no difference detected; interval wider than the margin",
        (false, true) => "difference detected, smaller than the margin",
        (false, false) => "difference detected",
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str| args.iter().position(|a| a == k).map(|i| args[i + 1].clone());
    let study = get("--study").expect("--study DIR");
    let margin: f64 = get("--margin").map(|s| s.parse().unwrap()).unwrap_or(0.05);
    let mut vids: Vec<String> = std::fs::read_dir(format!("{study}/specs"))
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("vertices.json").exists())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    vids.sort();
    let mut sessions: Vec<String> = std::fs::read_dir(format!("{study}/sessions"))
        .map(|rd| rd.filter_map(|e| e.ok()).filter(|e| e.path().join("records.jsonl").exists()).map(|e| e.file_name().to_string_lossy().to_string()).collect())
        .unwrap_or_default();
    sessions.sort();
    let mut vols = Vec::new();
    // control pairs per tier: (workload, round) -> (neg S, plain S), grouped by curve
    let mut control: BTreeMap<char, BTreeMap<String, BTreeMap<(String, u64), (Option<f64>, Option<f64>)>>> = BTreeMap::new();
    let mut unverified: Vec<String> = Vec::new();
    for vid in &vids {
        let meta: Value = serde_json::from_str(&std::fs::read_to_string(format!("{study}/specs/{vid}/vertices.json")).unwrap()).unwrap();
        let mut level_of = BTreeMap::new();
        let mut n1 = BTreeMap::new();
        for v in meta["vertices"].as_array().unwrap() {
            let l = v["level"].as_u64().unwrap() as u32;
            level_of.insert((v["a"].as_u64().unwrap(), v["b"].as_u64().unwrap()), l);
            n1.insert(l, v["n1"].as_u64().unwrap());
        }
        let tier = vid.chars().next().unwrap();
        let mut per_curve: BTreeMap<(u32, (u64, u64)), Vec<f64>> = BTreeMap::new();
        for s in sessions.iter().filter(|s| s.split("__").next() == Some(vid.as_str())) {
            let is_control = s.ends_with("__control");
            let text = std::fs::read_to_string(format!("{study}/sessions/{s}/records.jsonl")).unwrap();
            for line in text.lines() {
                let r: Value = serde_json::from_str(line).unwrap();
                if r["warmup"].as_bool().unwrap_or(false) {
                    continue;
                }
                let arm = r["arm"].as_str().unwrap();
                if r["outcome"]["status"] != "verified" {
                    unverified.push(format!("{s} {arm} {} round {}: {}", r["workload"]["workload_id"], r["round"], r["outcome"]["status"]));
                    continue;
                }
                let cs = &r["workload"]["curve_spec"];
                let ab = (cs["a"].as_u64().unwrap(), cs["b"].as_u64().unwrap());
                let rr = cs["r"].as_f64().unwrap();
                let sv = r["cost"]["total_gae"].as_f64().unwrap() / rr.sqrt();
                if is_control {
                    let key = (r["workload"]["workload_id"].as_str().unwrap().to_string(), r["round"].as_u64().unwrap());
                    let e = control.entry(tier).or_default().entry(format!("{vid}:{ab:?}")).or_default().entry(key).or_insert((None, None));
                    match arm {
                        "rho-neg" => e.0 = Some(sv),
                        "rho-plain" => e.1 = Some(sv),
                        _ => {}
                    }
                } else if arm == "rho-neg" {
                    let l = *level_of.get(&ab).expect("curve not in vertices.json");
                    per_curve.entry((l, ab)).or_default().push(sv);
                }
            }
        }
        if per_curve.is_empty() {
            continue;
        }
        let mut levels: BTreeMap<u32, Vec<Vec<f64>>> = BTreeMap::new();
        for ((l, _), v) in per_curve {
            levels.entry(l).or_default().push(v);
        }
        vols.push(Volcano {
            id: vid.clone(),
            tier,
            depth: meta["depth"].as_u64().unwrap() as u32,
            ell: meta["ell"].as_u64().unwrap(),
            d: meta["d"].as_i64().unwrap(),
            m: meta["m"].as_u64().unwrap(),
            p: meta["p"].as_u64().unwrap(),
            r: meta["r"].as_u64().unwrap(),
            levels,
            n1,
        });
    }
    let (mlo, mhi) = (1.0 - margin, 1.0 / (1.0 - margin));
    let mut md = format!(
        "# Tables (generated by `examples/volcano_tall_analyze.rs`)\n\nUnit: one `rho-neg` solve, `S = total_gae / √r`, normalised by its volcano's mean. 95% stratified hierarchical bootstrap (curves within (volcano, level), then solves), {B} resamples. Pre-registered equivalence margin: ratios in [{mlo:.3}, {mhi:.3}], crater-to-floor slope in [−{:.1}%, +{:.1}%].\n\n",
        100.0 * margin,
        100.0 * margin
    );
    let mut out = json!({"margin": margin, "volcanoes": [], "pooled": [], "control": [], "unverified": unverified});
    // ---------------------------------------------------------------- volcano table
    writeln!(md, "## Volcanoes\n\n| id | ℓ | D | depth | m | p | r (bits) | curves | solves | mean S | crater S | floor S |\n|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for v in &vols {
        let all: Vec<f64> = v.levels.values().flatten().flatten().copied().collect();
        let cr: Vec<f64> = v.levels.get(&0).into_iter().flatten().flatten().copied().collect();
        let fl: Vec<f64> = v.levels.get(&v.depth).into_iter().flatten().flatten().copied().collect();
        let nc: usize = v.levels.values().map(|c| c.len()).sum();
        writeln!(md, "| {} | {} | {} | {} | {} | {} | {:.1} | {nc} | {} | {:.4} | {:.4} | {:.4} |", v.id, v.ell, v.d, v.depth, v.m, v.p, (v.r as f64).log2(), all.len(), mean(&all), mean(&cr), mean(&fl)).unwrap();
        let lv: Vec<Value> = v
            .levels
            .iter()
            .map(|(l, cs)| {
                let xs: Vec<f64> = cs.iter().flatten().copied().collect();
                json!({"level": l, "curves": cs.len(), "solves": xs.len(), "mean_s": mean(&xs), "n1": v.n1.get(l)})
            })
            .collect();
        out["volcanoes"].as_array_mut().unwrap().push(json!({"id": v.id, "tier": v.tier.to_string(), "ell": v.ell, "d": v.d, "depth": v.depth, "m": v.m, "p": v.p, "r": v.r, "solves": all.len(), "mean_s": mean(&all), "levels": lv}));
    }
    md.push('\n');
    // ---------------------------------------------------------------- pooled analysis
    let mut tiers: Vec<(String, Vec<&Volcano>)> = Vec::new();
    let tier_set: std::collections::BTreeSet<char> = vols.iter().map(|v| v.tier).collect();
    for t in &tier_set {
        tiers.push((format!("tier {t}"), vols.iter().filter(|v| v.tier == *t).collect()));
    }
    if tier_set.len() > 1 {
        tiers.push(("all volcanoes".into(), vols.iter().collect()));
    }
    for v in &vols {
        tiers.push((format!("volcano {}", v.id), vec![v]));
    }
    writeln!(md, "## Level comparisons\n\n| scope | volcanoes | solves (crater / middle / floor) | comparison | estimate | 95% interval | reading |\n|---|--:|---|---|--:|---|---|").unwrap();
    let mut rng = Rng(0x7A11_B0CA);
    for (name, vs) in &tiers {
        let point = stats(&vs.iter().map(|v| (*v, normalise(&v.levels))).collect::<Vec<_>>());
        let boots: Vec<Stats> = (0..B).map(|_| stats(&vs.iter().map(|v| (*v, normalise(&v.resample(&mut rng)))).collect::<Vec<_>>())).collect();
        let count = |f: &dyn Fn(&Volcano, u32) -> bool| -> usize { vs.iter().map(|v| v.levels.iter().filter(|(l, _)| f(v, **l)).map(|(_, cs)| cs.iter().map(|c| c.len()).sum::<usize>()).sum::<usize>()).sum() };
        let ns = format!("{} / {} / {}", count(&|_, l| l == 0), count(&|v, l| l > 0 && l < v.depth), count(&|v, l| l == v.depth));
        let mut entry = json!({"scope": name, "volcanoes": vs.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), "rows": []});
        for (cmp, pt, f) in [
            ("floor / crater", point.floor / point.crater, (|s: &Stats| s.floor / s.crater) as fn(&Stats) -> f64),
            ("middle / crater", point.middle / point.crater, |s: &Stats| s.middle / s.crater),
            ("floor / middle", point.floor / point.middle, |s: &Stats| s.floor / s.middle),
        ] {
            let (lo, hi) = ci(boots.iter().map(f).collect());
            let rd = reading(lo, hi, 1.0, mlo, mhi);
            writeln!(md, "| {name} | {} | {ns} | {cmp} | {pt:.4} | [{lo:.4}, {hi:.4}] | {rd} |", vs.len()).unwrap();
            entry["rows"].as_array_mut().unwrap().push(json!({"comparison": cmp, "estimate": pt, "lo": lo, "hi": hi, "reading": rd}));
        }
        let (lo, hi) = ci(boots.iter().map(|s| s.slope).collect());
        let rd = reading(lo, hi, 0.0, -100.0 * margin, 100.0 * margin);
        writeln!(md, "| {name} | {} | {ns} | crater→floor slope, % | {:.2} | [{lo:.2}, {hi:.2}] | {rd} |", vs.len(), point.slope).unwrap();
        entry["rows"].as_array_mut().unwrap().push(json!({"comparison": "slope_percent", "estimate": point.slope, "lo": lo, "hi": hi, "reading": rd}));
        // distribution: KS crater vs floor, permutation within volcano
        if !name.starts_with("volcano") {
            let norm: Vec<(Vec<f64>, Vec<f64>)> = vs
                .iter()
                .map(|v| {
                    let n = normalise(&v.levels);
                    (n.get(&0).cloned().unwrap_or_default(), n.get(&v.depth).cloned().unwrap_or_default())
                })
                .collect();
            let flat = |k: usize| -> Vec<f64> { norm.iter().flat_map(|x| if k == 0 { x.0.clone() } else { x.1.clone() }).collect() };
            let d0 = ks(&flat(0), &flat(1));
            let mut ge = 0usize;
            for _ in 0..B {
                let (mut a, mut b) = (Vec::new(), Vec::new());
                for (c, f) in &norm {
                    let mut pool: Vec<f64> = c.iter().chain(f.iter()).copied().collect();
                    for i in (1..pool.len()).rev() {
                        let j = rng.below(i + 1);
                        pool.swap(i, j);
                    }
                    a.extend_from_slice(&pool[..c.len()]);
                    b.extend_from_slice(&pool[c.len()..]);
                }
                if ks(&a, &b) >= d0 {
                    ge += 1;
                }
            }
            let pv = (ge as f64 + 1.0) / (B as f64 + 1.0);
            let (mut a, mut b) = (flat(0), flat(1));
            let q = |v: &mut Vec<f64>| (pct(v, 0.1), pct(v, 0.5), pct(v, 0.9));
            let (qa, qb) = (q(&mut a), q(&mut b));
            writeln!(md, "| {name} | {} | {ns} | KS crater vs floor (p by permutation) | D = {d0:.4} | p = {pv:.3} | quantiles 10/50/90%: crater {:.3}/{:.3}/{:.3}, floor {:.3}/{:.3}/{:.3} |", vs.len(), qa.0, qa.1, qa.2, qb.0, qb.1, qb.2).unwrap();
            entry["ks"] = json!({"d": d0, "p": pv, "crater_q": [qa.0, qa.1, qa.2], "floor_q": [qb.0, qb.1, qb.2]});
            // raw (unnormalised) mean S with interval: the size check
            let raw = |lv: &[BTreeMap<u32, Vec<Vec<f64>>>]| -> f64 { mean(&lv.iter().flat_map(|m| m.values().flatten().flatten().copied()).collect::<Vec<_>>()) };
            let pt = raw(&vs.iter().map(|v| v.levels.clone()).collect::<Vec<_>>());
            let (lo, hi) = ci((0..B / 4).map(|_| raw(&vs.iter().map(|v| v.resample(&mut rng)).collect::<Vec<_>>())).collect());
            writeln!(md, "| {name} | {} | {ns} | mean raw S of rho.negation (floor √(π/4) = 0.8862) | {pt:.4} | [{lo:.4}, {hi:.4}] | descriptive |", vs.len()).unwrap();
            entry["raw_s"] = json!({"estimate": pt, "lo": lo, "hi": hi});
        }
        out["pooled"].as_array_mut().unwrap().push(entry);
    }
    md.push('\n');
    // ---------------------------------------------------------------- positive control
    writeln!(md, "## Positive control: rho.plain against rho.negation on the same workloads and seeds\n\n| scope | curves | pairs | Σ S(plain) / Σ S(neg) | 95% interval | reading |\n|---|--:|--:|--:|---|---|").unwrap();
    for (t, curves) in &control {
        let cs: Vec<Vec<(f64, f64)>> = curves.values().map(|m| m.values().filter_map(|(a, b)| Some(((*a)?, (*b)?))).collect::<Vec<_>>()).filter(|v: &Vec<(f64, f64)>| !v.is_empty()).collect();
        let ratio = |cs: &[Vec<(f64, f64)>]| -> f64 {
            let (sa, sb) = cs.iter().flatten().fold((0.0, 0.0), |acc, p| (acc.0 + p.0, acc.1 + p.1));
            sb / sa
        };
        let pt = ratio(&cs);
        let (lo, hi) = ci((0..B)
            .map(|_| {
                let rs: Vec<Vec<(f64, f64)>> = (0..cs.len())
                    .map(|_| {
                        let c = &cs[rng.below(cs.len())];
                        (0..c.len()).map(|_| c[rng.below(c.len())]).collect()
                    })
                    .collect();
                ratio(&rs)
            })
            .collect());
        let pairs: usize = cs.iter().map(|c| c.len()).sum();
        let rd = if lo > mhi { "difference detected, larger than the margin" } else { "CONTROL FAILED: the design does not resolve the known difference" };
        writeln!(md, "| tier {t} | {} | {pairs} | {pt:.4} | [{lo:.4}, {hi:.4}] | {rd} |", cs.len()).unwrap();
        out["control"].as_array_mut().unwrap().push(json!({"tier": t.to_string(), "curves": cs.len(), "pairs": pairs, "ratio": pt, "lo": lo, "hi": hi, "reading": rd}));
    }
    writeln!(md, "\nMeasured records that did not verify (excluded above, listed here): {}", if out["unverified"].as_array().unwrap().is_empty() { "none".to_string() } else { out["unverified"].as_array().unwrap().len().to_string() }).unwrap();
    for u in out["unverified"].as_array().unwrap() {
        writeln!(md, "- {}", u.as_str().unwrap()).unwrap();
    }
    print!("{md}");
    if let Some(p) = get("--out") {
        std::fs::write(&p, &md).unwrap();
    }
    if let Some(p) = get("--json") {
        std::fs::write(&p, serde_json::to_string_pretty(&out).unwrap() + "\n").unwrap();
    }
}
