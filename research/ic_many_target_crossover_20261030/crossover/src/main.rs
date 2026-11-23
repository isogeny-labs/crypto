//! Many-target crossover of compact-orbit index calculus against batch rho.
//!
//! Model (see ../README.md): IC(k) = S0 + k*d, batch rho(k) = rho1*K(k) with
//! K(k) = sum_{i<k} C(2i,i)/4^i = 2k*C(2k,k)/4^k ~ 2*sqrt(k/pi) (Kuhn-Struik).
//! The gap G = min_k IC(k)/rho(k) is the distance remaining; the window
//! {k : IC < rho} is non-empty iff G < 1.
//!
//! Reads recorded runs only; runs nothing.  Usage:
//!   crossover <ic_exponent_20260926 dir> <n83 control records dir> [seed]

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// K(k) = sum_{i<k} C(2i,i)/4^i = 2k * C(2k,k)/4^k.
fn kk(k: f64) -> f64 {
    let ki = k.round().max(1.0);
    let c = if ki <= 20000.0 {
        let mut c = 1.0f64;
        let mut i = 1.0f64;
        while i <= ki {
            c *= (2.0 * i - 1.0) / (2.0 * i);
            i += 1.0;
        }
        c
    } else {
        (1.0 / (std::f64::consts::PI * ki).sqrt()) * (1.0 - 1.0 / (8.0 * ki) + 1.0 / (128.0 * ki * ki))
    };
    2.0 * ki * c
}

#[derive(Clone, Copy, Debug)]
struct Cross {
    g_min: f64,
    k_opt: f64,
    k_lo: Option<f64>,
    k_hi: Option<f64>,
}

/// Scan integer k on a log grid for min IC/batch and the winning window.
fn cross(s0: f64, d: f64, rho1: f64) -> Cross {
    let mut g_min = f64::INFINITY;
    let mut k_opt = 1.0;
    let mut k_lo = None;
    let mut k_hi = None;
    let mut t = 0.0f64;
    let mut last_k = 0.0;
    while t < (1e12f64).ln() {
        let k = t.exp().round().max(1.0);
        t += 0.004;
        if k == last_k {
            continue;
        }
        last_k = k;
        let g = (s0 + k * d) / (rho1 * kk(k));
        if g < g_min {
            g_min = g;
            k_opt = k;
        }
        if g < 1.0 {
            if k_lo.is_none() {
                k_lo = Some(k);
            }
            k_hi = Some(k);
        }
    }
    Cross { g_min, k_opt, k_lo, k_hi }
}

fn jf(v: &Value, path: &[&str]) -> f64 {
    let mut x = v;
    for p in path {
        x = &x[*p];
    }
    x.as_f64().unwrap_or_else(|| panic!("missing {:?}", path))
}

fn jarr(v: &Value, path: &[&str]) -> Vec<f64> {
    let mut x = v;
    for p in path {
        x = &x[*p];
    }
    x.as_array().unwrap_or_else(|| panic!("missing {:?}", path)).iter().map(|e| e.as_f64().unwrap()).collect()
}

fn load(p: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))).unwrap()
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn pct(v: &mut Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// One measurement set: 32 targets, one process, everything in `ic price` units.
struct Set {
    shared: f64,
    shared_work_only: f64, // shared minus the workflow constructions
    d_total: f64,          // sum over the 32 targets of per-target descent units
    trials: Vec<f64>,
    cold_gae: Option<Vec<f64>>, // counted GAE of 32 cold single-target rho runs (set M1 only)
    canon: f64,         // canonical rho step in units
    batch_total_units: f64,
    batch_gae: f64,
    canon_uncharged: f64, // rho canonicalisations not in `gae` (one per step)
    ic_collect_adds: f64, // summands scanned (each one batched addition)
    ic_pairs: f64,        // stored pairs (each built by one addition)
}

struct Size {
    name: String,
    n: u32,
    log2_r: f64,
    r: f64,
    sets: Vec<Set>,
}

fn read_size(dir: &Path) -> Size {
    let mut sets = vec![];
    let mut meta = None;
    for j in 1..=4 {
        let single = dir.join("measure").join(format!("M{j}.price.json"));
        let double = dir.join("measure").join(format!("M{j}.price-double.json"));
        let v = load(if double.exists() { &double } else { &single });
        assert_eq!(v["targets"].as_u64().unwrap(), 32);
        let r = jf(&v, &["r"]);
        let sr = r.sqrt();
        meta = Some((v["curve"].as_str().unwrap().to_string(), v["n"].as_u64().unwrap() as u32, jf(&v, &["log2_r"]), r));
        let shared = jf(&v, &["median", "shared_units"]);
        let constr = jf(&v, &["median", "groups", "constructions"]);
        sets.push(Set {
            shared,
            shared_work_only: shared - constr,
            d_total: jf(&v, &["median", "per_target_units"]) * 32.0,
            trials: jarr(&v, &["counts", "descent", "trials_per_target"]),
            cold_gae: if v.get("rho_cold").is_some() { Some(jarr(&v, &["rho_cold", "gae_per_target"])) } else { None },
            canon: jf(&v, &["step_prices", "canonical_step_units"]),
            batch_total_units: jf(&v, &["rho_batch", "s_priced_per_target"]) * sr * 32.0,
            batch_gae: jf(&v, &["rho_batch", "gae"]),
            canon_uncharged: jf(&v, &["rho_batch", "counters", "canonicalisations_uncharged"]),
            ic_collect_adds: jf(&v, &["counts", "collect", "summands_scanned"]),
            ic_pairs: jf(&v, &["counts", "build", "stored_pairs"]),
        });
    }
    let (name, n, log2_r, r) = meta.unwrap();
    Size { name, n, log2_r, r, sets }
}

/// Point estimates for one set: (S0, d, rho1) in units.
fn point(s: &Set) -> (f64, f64, f64) {
    (s.shared, s.d_total / 32.0, s.batch_total_units / kk(32.0))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exp = PathBuf::from(&args[1]).join("runs");
    let n83 = PathBuf::from(&args[2]);
    let seed: u64 = args.get(3).map(|s| s.parse().unwrap()).unwrap_or(20261030);
    let mut rng = Rng(seed | 1);
    const B: usize = 2000;

    let mut dirs: Vec<PathBuf> = fs::read_dir(&exp)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.join("measure").join("M1.price.json").exists())
        .collect();
    let mut sizes: Vec<Size> = dirs.drain(..).map(|d| read_size(&d)).collect();
    sizes.sort_by(|a, b| a.log2_r.partial_cmp(&b.log2_r).unwrap());

    println!("## Counted-unit table (k = 32 batch runs, `ic price` units; 4 target sets per size)\n");
    println!("| curve | log2 r | S0 units | d units | rho1 units | S0/rho1 | d/rho1 | KS check | G=min IC/rho [95% boot] | k_opt | G, S0 work-only | G, rho step=1.0 | G, rho step=2.0 |");
    println!("|:--|--:|--:|--:|--:|--:|--:|--:|:--|--:|--:|--:|--:|");

    // per-size bootstrap draws of ln G for the trend fit
    let mut lng_draws: Vec<Vec<f64>> = vec![];
    let mut rows_json = vec![];
    for sz in &sizes {
        // point estimate: mean over the four sets of S0, d, rho1 (S0 is deterministic per recipe)
        let pts: Vec<_> = sz.sets.iter().map(point).collect();
        let s0 = mean(&pts.iter().map(|p| p.0).collect::<Vec<_>>());
        let d = mean(&pts.iter().map(|p| p.1).collect::<Vec<_>>());
        let rho1 = mean(&pts.iter().map(|p| p.2).collect::<Vec<_>>());
        let c = cross(s0, d, rho1);
        let s0w = mean(&sz.sets.iter().map(|s| s.shared_work_only).collect::<Vec<_>>());
        let cw = cross(s0w, d, rho1);
        let canon = mean(&sz.sets.iter().map(|s| s.canon).collect::<Vec<_>>());
        // section 5a band on the rho arm: step = 1 + w native ops (w = 0, 0.1, 1); the
        // clock-priced IC arm has no unpriced counter, so only the rho step moves.
        let rho_step1 = rho1 / canon * 1.0;
        let rho_step2 = rho1 / canon * 2.0;
        let c1 = cross(s0, d, rho_step1);
        let c2 = cross(s0, d, rho_step2);
        let ks = sz.sets.iter().find_map(|s| s.cold_gae.as_ref().map(|c| s.batch_total_units / (mean(c) * s.canon * kk(32.0)))).unwrap();

        // bootstrap: pick a set, resample its 32 targets for d (by trials share) and rho1 (cold gae)
        let mut gs = vec![];
        let mut lgs = vec![];
        for _ in 0..B {
            let s = &sz.sets[rng.below(4)];
            let tsum: f64 = s.trials.iter().sum();
            let mut dd = 0.0;
            for _ in 0..32 {
                let i = rng.below(32);
                dd += s.d_total * s.trials[i] / tsum;
            }
            let cb = cross(s.shared, dd / 32.0, s.batch_total_units / kk(32.0));
            gs.push(cb.g_min);
            lgs.push(cb.g_min.ln());
        }
        let lo = pct(&mut gs.clone(), 0.025);
        let hi = pct(&mut gs.clone(), 0.975);
        println!(
            "| {} | {:.1} | {:.3e} | {:.3e} | {:.3e} | {:.1} | {:.3} | {:.2} | {:.2} [{:.2}, {:.2}] | {:.0} | {:.2} | {:.2} | {:.2} |",
            sz.name, sz.log2_r, s0, d, rho1, s0 / rho1, d / rho1, ks, c.g_min, lo, hi, c.k_opt, cw.g_min, c1.g_min, c2.g_min
        );
        rows_json.push(serde_json::json!({
            "curve": sz.name, "n": sz.n, "log2_r": sz.log2_r, "S0": s0, "d": d, "rho1": rho1,
            "sigma": s0/rho1, "delta": d/rho1, "ks_check": ks, "G": c.g_min, "G_lo": lo, "G_hi": hi,
            "k_opt": c.k_opt, "G_work_only": cw.g_min, "G_rho_step_1": c1.g_min, "G_rho_step_2": c2.g_min,
            "window_lo": c.k_lo, "window_hi": c.k_hi, "r": sz.r,
        }));
        lng_draws.push(lgs);
    }

    // AGENTS.md section 5a: price the unpriced work on every arm at 0, 0.1 and 1 GAE per native operation, from counts.
    // IC counts used: one batched addition per summand scanned, per stored pair and per descent trial (lower bounds: linear
    // algebra, selection, projections and constructions emit no GAE count).  Rho counts: `rho_batch.gae` (additions) plus one
    // canonicalisation per step at weight w.  Variant A prices w on rho only (IC at its additions alone, the IC-favourable
    // reading); variant B also prices one lookup per IC addition at w (symmetric).
    println!("\n## Count-based section 5a band: G = min_k IC(k)/batch rho(k) from native operation counts (IC counts are lower bounds)\n");
    println!("| curve | log2 r | S0c/rho1c (w=0) | d_c/rho1c (w=0) | G A w=0 [min,max over 4 sets] | G A w=0.1 | G A w=1 | G B w=0 | G B w=0.1 | G B w=1 | k_opt A w=0 |");
    println!("|:--|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|");
    let mut band_json = vec![];
    for sz in &sizes {
        let kk32 = kk(32.0);
        let ga = |s: &Set, w: f64, sym: bool| -> Cross {
            let f = if sym { 1.0 + w } else { 1.0 };
            let s0 = (s.ic_collect_adds + s.ic_pairs) * f;
            let d = s.trials.iter().sum::<f64>() / 32.0 * f;
            let rho1 = (s.batch_gae + w * s.canon_uncharged) / kk32;
            cross(s0, d, rho1)
        };
        let mean_g = |w: f64, sym: bool| mean(&sz.sets.iter().map(|s| ga(s, w, sym).g_min).collect::<Vec<_>>());
        let gs0: Vec<f64> = sz.sets.iter().map(|s| ga(s, 0.0, false).g_min).collect();
        let (lo, hi) = (gs0.iter().cloned().fold(f64::INFINITY, f64::min), gs0.iter().cloned().fold(0.0, f64::max));
        let s0c = mean(&sz.sets.iter().map(|s| s.ic_collect_adds + s.ic_pairs).collect::<Vec<_>>());
        let dc = mean(&sz.sets.iter().map(|s| s.trials.iter().sum::<f64>() / 32.0).collect::<Vec<_>>());
        let r1c = mean(&sz.sets.iter().map(|s| s.batch_gae / kk32).collect::<Vec<_>>());
        let kopt = mean(&sz.sets.iter().map(|s| ga(s, 0.0, false).k_opt).collect::<Vec<_>>());
        println!(
            "| {} | {:.1} | {:.1} | {:.3} | {:.2} [{:.2}, {:.2}] | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.0} |",
            sz.name, sz.log2_r, s0c / r1c, dc / r1c, mean_g(0.0, false), lo, hi, mean_g(0.1, false), mean_g(1.0, false),
            mean_g(0.0, true), mean_g(0.1, true), mean_g(1.0, true), kopt
        );
        band_json.push(serde_json::json!({"curve": sz.name, "log2_r": sz.log2_r, "sigma_c": s0c / r1c, "delta_c": dc / r1c,
            "G_A": [mean_g(0.0, false), mean_g(0.1, false), mean_g(1.0, false)], "G_B": [mean_g(0.0, true), mean_g(0.1, true), mean_g(1.0, true)]}));
    }

    // recipe sweep: same S0/d accounting on every (columns, descent arity) point of the sweep, rho1 from the size's measure sets
    let mut best_sweep: Vec<(f64, f64, String, f64)> = vec![];
    println!("\n## Recipe sweep: G for every recorded (columns, descent summands) point (sweep seed set W; rho1 from the size's measure sets)\n");
    println!("| curve | recipe (cols-m) | S0 units | d units | S0/rho1 | d/rho1 | G | k_opt |");
    println!("|:--|:--|--:|--:|--:|--:|--:|--:|");
    for sz in &sizes {
        let rho1 = mean(&sz.sets.iter().map(|s| s.batch_total_units / kk(32.0)).collect::<Vec<_>>());
        let sdir = exp.join(sz.name.replace(" / GF(2^", "n").replace(")", "").replace("K_0", "k0").replace("K_1", "k1").replace(" ", "")).join("sweep");
        let mut files: Vec<PathBuf> = fs::read_dir(&sdir).map(|rd| rd.map(|e| e.unwrap().path()).filter(|p| p.to_string_lossy().ends_with(".price.json")).collect()).unwrap_or_default();
        files.sort();
        for f in files {
            let v = load(&f);
            if v["status"].as_str() != Some("complete") { continue; }
            let s0 = jf(&v, &["median", "shared_units"]);
            let d = jf(&v, &["median", "per_target_units"]);
            let c = cross(s0, d, rho1);
            let tag = f.file_stem().unwrap().to_string_lossy().trim_end_matches(".price").to_string();
            println!("| {} | {} | {:.3e} | {:.3e} | {:.1} | {:.3} | {:.2} | {:.0} |", sz.name, tag, s0, d, s0 / rho1, d / rho1, c.g_min, c.k_opt);
            match best_sweep.last_mut() {
                Some(b) if b.0 == sz.log2_r => {
                    if c.g_min < b.1 {
                        *b = (sz.log2_r, c.g_min, tag, c.k_opt);
                    }
                }
                _ => best_sweep.push((sz.log2_r, c.g_min, tag, c.k_opt)),
            }
        }
    }
    println!("\n### Best recorded recipe per size (minimum over the sweep; optimistic by selection)\n");
    println!("| log2 r | recipe | G | k_opt |\n|--:|:--|--:|--:|");
    for b in &best_sweep {
        println!("| {:.1} | {} | {:.2} | {:.0} |", b.0, b.2, b.1, b.3);
    }

    // trend: ln G = a + b*log2 r over all sizes and over n >= 37 (constructions-dominated small sizes dropped)
    println!("\n## Trend of the gap G = min_k IC(k)/batch-rho(k) with log2 r\n");
    for (label, min_n) in [("all nine sizes", 0u32), ("n >= 37 (seven sizes)", 37)] {
        let idx: Vec<usize> = (0..sizes.len()).filter(|&i| sizes[i].n >= min_n).collect();
        let xs: Vec<f64> = idx.iter().map(|&i| sizes[i].log2_r).collect();
        let mut slopes = vec![];
        let mut p73 = vec![];
        let mut p83 = vec![];
        for b in 0..B {
            let ys: Vec<f64> = idx.iter().map(|&i| lng_draws[i][b]).collect();
            let (a, s) = ols(&xs, &ys);
            slopes.push(s);
            p73.push((a + s * 56.3).exp());
            p83.push((a + s * 52.9).exp());
        }
        let pts: Vec<f64> = idx.iter().map(|&i| rows_json[i]["G"].as_f64().unwrap().ln()).collect();
        let (a, s) = ols(&xs, &pts);
        println!(
            "- {label}: slope of ln G per bit of log2 r = {:.3} [{:.3}, {:.3}] (point {:.3}); extrapolated G at log2 r = 56.3 (n=73): {:.1} [{:.1}, {:.1}], at 52.9 (n=83): {:.1} [{:.1}, {:.1}]",
            s,
            pct(&mut slopes.clone(), 0.025),
            pct(&mut slopes.clone(), 0.975),
            s,
            (a + s * 56.3).exp(),
            pct(&mut p73.clone(), 0.025),
            pct(&mut p73.clone(), 0.975),
            (a + s * 52.9).exp(),
            pct(&mut p83.clone(), 0.025),
            pct(&mut p83.clone(), 0.975)
        );
    }

    // ---- n = 83 anchor: same-runner wall/CPU records (not counted units)
    println!("\n## n = 83 anchor (recorded wall/CPU seconds, one runner per row; NOT counted units)\n");
    println!("| run / recipe | S0 CPU s | d mean ms [boot] | rho1 plain mean s [boot] | S0/rho1 | d/rho1 | G [95% boot] | k_opt | d-cap k=(1.128 rho1/d)^2 |");
    println!("|:--|--:|--:|--:|--:|--:|:--|--:|--:|");
    let mut anchors = vec![];
    for run in ["run-38855595549", "run-38878565324"] {
        for kdir in ["k3300", "k600"] {
            let base = n83.join("records").join(run).join(kdir);
            let an = load(&base.join("analysis.json"));
            let user: f64 = an["setup"]["ic"]["time_v_user_seconds"].as_str().unwrap().parse().unwrap();
            let tms: Vec<f64> = fs::read_to_string(base.join("ic").join("records.jsonl"))
                .unwrap()
                .lines()
                .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                .filter_map(|v| v.get("target_ms").and_then(|x| x.as_f64()).filter(|_| v.get("probes").is_some()))
                .collect();
            let mut rho_s = vec![];
            for i in 0..16 {
                let v = load(&base.join("rho_plain").join(format!("w{:02}.json", i)));
                rho_s.push(v["target_ms"].as_f64().unwrap() / 1000.0);
            }
            assert_eq!(tms.len(), 16);
            let d = mean(&tms) / 1000.0;
            let rho1 = mean(&rho_s);
            let c = cross(user, d, rho1);
            let mut gs = vec![];
            let (mut dl, mut rl): (Vec<f64>, Vec<f64>) = (vec![], vec![]);
            for _ in 0..B {
                let dd = (0..16).map(|_| tms[rng.below(16)]).sum::<f64>() / 16.0 / 1000.0;
                let rr = (0..16).map(|_| rho_s[rng.below(16)]).sum::<f64>() / 16.0;
                gs.push(cross(user, dd, rr).g_min);
                dl.push(dd);
                rl.push(rr);
            }
            println!(
                "| {run} {kdir} | {:.0} | {:.1} [{:.1}, {:.1}] | {:.3} [{:.3}, {:.3}] | {:.0} | {:.4} | {:.2} [{:.2}, {:.2}] | {:.0} | {:.0} |",
                user,
                d * 1000.0,
                pct(&mut dl.clone(), 0.025) * 1000.0,
                pct(&mut dl.clone(), 0.975) * 1000.0,
                rho1,
                pct(&mut rl.clone(), 0.025),
                pct(&mut rl.clone(), 0.975),
                user / rho1,
                d / rho1,
                c.g_min,
                pct(&mut gs.clone(), 0.025),
                pct(&mut gs.clone(), 0.975),
                c.k_opt,
                (1.1284 * rho1 / d).powi(2)
            );
            anchors.push(serde_json::json!({"run": run, "recipe": kdir, "S0_cpu_s": user, "d_s": d, "rho1_s": rho1, "G": c.g_min, "k_opt": c.k_opt}));
        }
    }

    // ---- n = 83: total seconds for k targets, every arm recorded on the same runner
    println!("\n## n = 83 total CPU seconds for k targets, all recorded arms (same runner per row; BL = precomputed-table rho)\n");
    println!("| run / recipe | k | IC | batch rho (KS) | BL time-matched (a) | BL wall-equal (a2) | BL memory-matched (b) | cheapest |");
    println!("|:--|--:|--:|--:|--:|--:|--:|:--|");
    for run in ["run-38878565324", "run-38855595549"] {
        for kdir in ["k3300", "k600"] {
            let base = n83.join("records").join(run).join(kdir);
            let an = load(&base.join("analysis.json"));
            let ms = |arm: &str| -> Option<f64> {
                let a = an["arms"][arm]["online_ms"].as_array()?;
                Some(a.iter().map(|x| x.as_f64().unwrap()).sum::<f64>() / a.len() as f64 / 1000.0)
            };
            let setup = |arm: &str| -> Option<f64> { an["setup"][arm]["process_cpu_seconds"].as_f64() };
            let ic_s0: f64 = an["setup"]["ic"]["time_v_user_seconds"].as_str().unwrap().parse().unwrap();
            let ic_d = ms("ic").unwrap();
            let rho1 = ms("rho_plain_fold").unwrap();
            for k in [1.0, 10.0, 100.0, 1e3, 1e4, 1e5, 1e6, 1e7] {
                let mut row: Vec<(String, f64)> = vec![("IC".into(), ic_s0 + k * ic_d), ("batch".into(), rho1 * kk(k))];
                for (lab, arm) in [("BL(a)", "bl_a_time_matched"), ("BL(a2)", "bl_a2_wall_equal"), ("BL(b)", "bl_b_memory_matched")] {
                    if let (Some(s), Some(o)) = (setup(arm), ms(arm)) {
                        row.push((lab.into(), s + k * o));
                    }
                }
                let best = row.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0.clone();
                let get = |l: &str| row.iter().find(|r| r.0 == l).map(|r| format!("{:.0}", r.1)).unwrap_or("-".into());
                println!("| {run} {kdir} | {k:.0} | {} | {} | {} | {} | {} | {} |", get("IC"), get("batch"), get("BL(a)"), get("BL(a2)"), get("BL(b)"), best);
            }
        }
    }

    // ---- n = 73 anchor: ledger numbers only, rho1 scaled from the n = 83 EPYC run
    println!("\n## n = 73 anchor (ledger wall seconds; mixed hosts; rho1 scaled from n = 83, extrapolated)\n");
    let n_prime_73 = 86_020_738_150_056_119f64 / (2.0 * 73.0);
    let n_prime_83 = 8_569_786_107_849_059f64 / (2.0 * 83.0);
    let scale = (n_prime_73 / n_prime_83).sqrt();
    let base = n83.join("records").join("run-38878565324").join("k3300");
    let rho83: f64 = mean(&(0..16).map(|i| load(&base.join("rho_plain").join(format!("w{:02}.json", i)))["target_ms"].as_f64().unwrap() / 1000.0).collect::<Vec<_>>());
    let rho73 = rho83 * scale;
    println!("rho1(73) = {:.2} s = rho1(83) {:.3} s x sqrt(N'73/N'83) = x{:.3}\n", rho73, rho83, scale);
    println!("| S0 s (K600 rank, 1 thread) | d s | G | k_opt |");
    println!("|--:|--:|--:|--:|");
    for d in [0.249, 0.326, 0.544] {
        let c = cross(18398.0, d, rho73);
        println!("| 18398 | {d} | {:.1} | {:.0} |", c.g_min, c.k_opt);
    }

    // graph: gap G against log2 r (log y); the line G = 1 is where amortised IC would start to win
    {
        let (w, h, ml, mb, mt) = (760.0, 460.0, 70.0, 50.0, 30.0);
        let xr = |x: f64| ml + (x - 15.0) / (60.0 - 15.0) * (w - ml - 20.0);
        let yr = |g: f64| mt + (1.0 - (g.ln() - (0.5f64).ln()) / ((60.0f64).ln() - (0.5f64).ln())) * (h - mt - mb);
        let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
        for g in [0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0] {
            s += &format!("<line x1='{ml}' x2='{}' y1='{y}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{g}</text>", w - 20.0, ml - 6.0, yr(g) + 4.0, y = yr(g));
        }
        for x in (15..=60).step_by(5) {
            s += &format!("<text x='{}' y='{}' text-anchor='middle'>{x}</text>", xr(x as f64), h - mb + 18.0);
        }
        s += &format!("<line x1='{ml}' x2='{}' y1='{y}' y2='{y}' stroke='#c00' stroke-width='2'/><text x='{}' y='{}' fill='#c00'>G = 1: amortised IC beats batch rho</text>", w - 20.0, ml + 6.0, yr(1.0) - 6.0, y = yr(1.0));
        for r in &rows_json {
            let (x, g, lo, hi) = (r["log2_r"].as_f64().unwrap(), r["G"].as_f64().unwrap(), r["G_lo"].as_f64().unwrap(), r["G_hi"].as_f64().unwrap());
            s += &format!("<line x1='{x0}' x2='{x0}' y1='{}' y2='{}' stroke='#1f77b4'/><circle cx='{x0}' cy='{}' r='4' fill='#1f77b4'/>", yr(lo), yr(hi), yr(g), x0 = xr(x));
        }
        for b in &best_sweep {
            s += &format!("<rect x='{}' y='{}' width='8' height='8' fill='#ff7f0e'/>", xr(b.0) - 4.0, yr(b.1) - 4.0);
        }
        for a in &anchors {
            let g = a["G"].as_f64().unwrap();
            if g < 10.0 {
                s += &format!("<path d='M{x} {} l6 6 l-6 6 l-6 -6 z' fill='#2ca02c'/>", yr(g) - 6.0, x = xr(52.9));
            }
        }
        s += &format!("<text x='{ml}' y='16'>Gap G = min over k of IC(k)/batch-rho(k), against log2 r. Blue: k=32 recipe, 95% bootstrap. Orange: best swept recipe. Green: n=83 wall (K3300).</text><text x='{}' y='{}' text-anchor='middle'>log2 r (n = 19..61 counted units; n = 83 recorded seconds)</text></svg>", (ml + w) / 2.0, h - 8.0);
        fs::write("g_vs_log2r.svg", s).unwrap();
    }
    let out = serde_json::json!({"sizes": rows_json, "n83_anchors": anchors, "rho73_s": rho73, "count_band": band_json, "best_sweep": best_sweep.iter().map(|b| serde_json::json!({"log2_r": b.0, "G": b.1, "recipe": b.2, "k_opt": b.3})).collect::<Vec<_>>()});
    fs::write("crossover_results.json", serde_json::to_string_pretty(&out).unwrap()).unwrap();
}

fn ols(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxx: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let s = sxy / sxx;
    (my - s * mx, s)
}
