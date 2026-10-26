//! W2a study driver: instance generation (`gen`), the resumable measurement queue
//! (`cells`), and the fits (`fit`). Protocol:
//! `research/diem_quartic_relations_20261018/PROTOCOL.md` (+ dated amendments).
//!
//! Always run pinned: `taskset -c 26,27,28 nice -n 10 cargo run --release --example
//! diem_quartic_study -j 3 -- <cmd>`.

use crypto_lib::cryptanalysis::diem_quartic::*;
#[path = "diem_support/checkpoint.rs"]
mod checkpoint;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const DIR: &str = "research/diem_quartic_relations_20261018/results";
const KMIN: u32 = 8;
const KMAX: u32 = 21;

fn n_inst(k: u32) -> u32 {
    if k <= 12 {
        5
    } else {
        3
    }
}

fn n_rho(k: u32) -> u32 {
    match k {
        0..=12 => 9,
        13..=16 => 5,
        17..=18 => 3,
        _ => 1,
    }
}

const CONFIGS: [(Arm, f64); 7] = [
    (Arm::Full, 1.0),
    (Arm::Slp, 1.0),
    (Arm::Slp, 2.0),
    (Arm::Slp, 4.0),
    (Arm::Dlp, 2.0),
    (Arm::Dlp, 4.0),
    (Arm::Dlp, 8.0),
];

fn path(name: &str) -> String {
    format!("{}/{}", DIR, name)
}

fn read_jsonl(name: &str) -> Vec<Value> {
    match fs::read_to_string(path(name)) {
        Ok(s) => s
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect(),
        Err(_) => vec![],
    }
}

fn append(lock: &Mutex<()>, name: &str, line: &str) {
    let _g = lock.lock().unwrap();
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path(name))
        .unwrap();
    writeln!(f, "{}", line).unwrap();
}

fn arr_u64(a: &[u64]) -> String {
    format!(
        "[{}]",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

// ---------------------------------------------------------------------------
// gen
// ---------------------------------------------------------------------------

fn gen_one(k: u32, i: u32) -> String {
    let t = Instant::now();
    let inst = gen_instance(k, i);
    let gen_s = t.elapsed().as_secs_f64();
    let f = inst.f();
    let pts = enumerate_points(f, &inst.quartic);
    let n1 = pts.len() as u64;
    assert_eq!(n1, inst.q + 1);
    // every rational point lies on the cover (pi0 defined, image on E0); N * pi0(P) = O on all
    // points for k <= 14 and on a seeded sample of 20000 otherwise (amendment A1)
    let mut on_cover = 0u64;
    let mut cover_fail = 0u64;
    let mut order_checked = 0u64;
    let mut order_fail = 0u64;
    let mut rng = SplitMix64::new(0x4F52_4445 ^ inst.q);
    let all = k <= 14;
    for (j, p) in pts.iter().enumerate() {
        match inst.pi0(p) {
            Some(e) if inst.e_on_curve(&e) => {
                on_cover += 1;
                let do_order = all || rng.below(pts.len() as u64) < 20000;
                if do_order {
                    order_checked += 1;
                    if inst.e_mul(&e, inst.order as u128).is_some() {
                        order_fail += 1;
                    }
                }
            }
            _ => cover_fail += 1,
        }
        let _ = j;
    }
    let mut j = J::new();
    j.n("k", k)
        .n("idx", i)
        .n("q", inst.q)
        .n("ka", inst.kf.a)
        .n("kb", inst.kf.b);
    j.raw("ub", &arr_u64(&inst.ub))
        .raw("uc", &arr_u64(&inst.uc))
        .raw("quartic", &arr_u64(&inst.quartic));
    j.n("order", inst.order)
        .n("h", inst.h)
        .n("ell", inst.ell)
        .n("trace", inst.trace)
        .n("candidates", inst.candidates);
    j.raw("rejects", &arr_u64(&inst.rejects));
    j.n("n1", n1).n("n1_expected", inst.q + 1);
    j.n("on_cover", on_cover)
        .n("cover_fail", cover_fail)
        .n("order_checked", order_checked)
        .n("order_fail", order_fail);
    j.s(
        "lpoly",
        "1 - t T^3 + q^3 T^6 (inherited; live evidence N1 = q+1 and the pushforward checks)",
    );
    j.n("gen_wall_s", gen_s);
    j.done()
}

fn cmd_gen(status_only: bool) {
    let mut grid = vec![];
    for k in KMIN..=KMAX {
        for i in 0..n_inst(k) {
            grid.push((k, i));
        }
    }
    // Exact source/protocol binding prevents mixing receipts from different
    // generators. Changing these inputs requires a separately reviewed run.
    let source = serde_json::json!({
        "schema": 1,
        "driver": include_str!("diem_quartic_study.rs"),
        "checkpoint": include_str!("diem_support/checkpoint.rs"),
        "generator": include_str!("../src/cryptanalysis/diem_quartic.rs"),
        "protocol": include_str!("../research/diem_quartic_relations_20261018/PROTOCOL.md"),
        "amendments": include_str!("../research/diem_quartic_relations_20261018/AMENDMENTS-20261018.md"),
    });
    let identity = checkpoint::digest(source.to_string().as_bytes());
    let store = Arc::new(
        checkpoint::Store::open(std::path::Path::new(DIR), &identity, &grid)
            .expect("open validated generation checkpoints (another generator may own the lock)"),
    );
    let missing = store.missing().expect("validate checkpoints");
    eprintln!(
        "gen source={} durable={}/{} pending={}",
        identity,
        grid.len() - missing.len(),
        grid.len(),
        missing.len()
    );
    if status_only {
        return;
    }
    let next = Arc::new(AtomicUsize::new(0));
    let tasks = Arc::new(missing);
    let mut hs = vec![];
    for _ in 0..3 {
        let (next, store, tasks) = (next.clone(), store.clone(), tasks.clone());
        hs.push(std::thread::spawn(move || loop {
            let n = next.fetch_add(1, Ordering::SeqCst);
            if n >= tasks.len() {
                break;
            }
            let (k, i) = tasks[n];
            let line = gen_one(k, i);
            store
                .save(k, i, &line)
                .expect("durably checkpoint completed instance");
            eprintln!("gen k={} i={} durable", k, i);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    store
        .publish()
        .expect("publish complete frozen instances and SHA-256 manifest");
    eprintln!(
        "gen complete: {} durable instances; results/FROZEN.sha256 written",
        grid.len()
    );
}

// ---------------------------------------------------------------------------
// cells
// ---------------------------------------------------------------------------

fn fval(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(|x| x.as_f64()).unwrap_or(f64::NAN)
}
fn sval<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("")
}

/// Fit `log2(y) = a + b k` to the last three `(k, y)` points with distinct `k` (mean over
/// duplicates) and predict at `k_new`; `None` with fewer than two distinct `k`.
fn predict(points: &[(u32, f64)], k_new: u32) -> Option<f64> {
    let mut by: HashMap<u32, Vec<f64>> = HashMap::new();
    for &(k, y) in points {
        if y > 0.0 && k >= 9 {
            by.entry(k).or_default().push(y);
        }
    }
    let mut ks: Vec<u32> = by.keys().cloned().collect();
    ks.sort();
    if ks.len() < 2 {
        return None;
    }
    let ks = &ks[ks.len().saturating_sub(3)..];
    let xs: Vec<f64> = ks.iter().map(|&k| k as f64).collect();
    let ys: Vec<f64> = ks
        .iter()
        .map(|k| (by[k].iter().sum::<f64>() / by[k].len() as f64).log2())
        .collect();
    let n = xs.len() as f64;
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    Some((a + b * k_new as f64).exp2())
}

struct Shared {
    lock: Mutex<()>,
    kmax: u32,
}

fn run_instance(sh: &Shared, k: u32, i: u32, only_rho: bool, only_ic: bool) {
    let inst_rec = read_jsonl("instances.jsonl")
        .into_iter()
        .find(|r| fval(r, "k") as u32 == k && fval(r, "idx") as u32 == i);
    let inst = gen_instance(k, i);
    if let Some(r) = &inst_rec {
        assert_eq!(
            r["ell"].as_u64().unwrap(),
            inst.ell,
            "regenerated instance differs from the frozen list"
        );
    }
    let done_rho: HashSet<(u32, u32, u32)> = read_jsonl("rho_cells.jsonl")
        .iter()
        .map(|r| {
            (
                fval(r, "k") as u32,
                fval(r, "idx") as u32,
                fval(r, "run") as u32,
            )
        })
        .collect();
    let done_ic: HashSet<(u32, u32, String, u32)> = read_jsonl("ic_cells.jsonl")
        .iter()
        .map(|r| {
            (
                fval(r, "k") as u32,
                fval(r, "idx") as u32,
                sval(r, "arm").to_string(),
                fval(r, "kappa") as u32,
            )
        })
        .collect();
    // inversion price (once per instance, recorded)
    let rinv = measure_rho_inv(inst.q, 1_000_000);
    append(
        &sh.lock,
        "calib.jsonl",
        &format!(
            "{{\"k\":{},\"idx\":{},\"q\":{},\"rho_inv\":{}}}",
            k, i, inst.q, rinv
        ),
    );
    // ---- rho ----
    if !only_ic {
        for run in 0..n_rho(k) {
            if done_rho.contains(&(k, i, run)) {
                continue;
            }
            let rows = read_jsonl("rho_cells.jsonl");
            let pts: Vec<(u32, f64)> = rows
                .iter()
                .filter(|r| fval(r, "ok") == 1.0)
                .map(|r| (fval(r, "k") as u32, fval(r, "wall_s")))
                .collect();
            let pred = predict(&pts, k);
            if let Some(p) = pred {
                if p > 3600.0 || (p > 600.0 && i > 0) {
                    append(&sh.lock, "skipped.jsonl", &format!("{{\"what\":\"rho\",\"k\":{},\"idx\":{},\"run\":{},\"pred_wall_s\":{},\"reason\":\"{}\"}}", k, i, run, p, if p > 3600.0 { "pred>1h" } else { "pred>10min,idx>0" }));
                    continue;
                }
            }
            let o = rho_run(
                &inst,
                0xA000 + run as u64,
                Some(Instant::now() + std::time::Duration::from_secs(3600)),
            );
            append(&sh.lock, "rho_cells.jsonl", &rho_json(&inst, run, &o, rinv));
            eprintln!(
                "rho k={} i={} run={} steps={} ok={} wall={:.1}",
                k, i, run, o.steps, o.ok, o.wall_s
            );
        }
    }
    // ---- IC ----
    if !only_rho {
        for (ci, &(arm, kappa)) in CONFIGS.iter().enumerate() {
            if done_ic.contains(&(k, i, arm.name().to_string(), kappa as u32)) {
                continue;
            }
            let rows = read_jsonl("ic_cells.jsonl");
            let same: Vec<&Value> = rows
                .iter()
                .filter(|r| {
                    sval(r, "arm") == arm.name()
                        && fval(r, "kappa") == kappa
                        && sval(r, "status") != "skipped"
                })
                .collect();
            let walls: Vec<(u32, f64)> = same
                .iter()
                .filter(|r| sval(r, "status") == "ok")
                .map(|r| {
                    (
                        fval(r, "k") as u32,
                        fval(r, "rel_wall_s") + fval(r, "la_wall_s"),
                    )
                })
                .collect();
            let la_walls: Vec<(u32, f64)> = same
                .iter()
                .filter(|r| sval(r, "status") == "ok")
                .map(|r| (fval(r, "k") as u32, fval(r, "la_wall_s")))
                .collect();
            let rel_walls: Vec<(u32, f64)> = same
                .iter()
                .filter(|r| sval(r, "status") == "ok" || sval(r, "status") == "rel_only")
                .map(|r| (fval(r, "k") as u32, fval(r, "rel_wall_s")))
                .collect();
            let pred_tot = predict(&walls, k);
            let pred_la = predict(&la_walls, k);
            let pred_rel = predict(&rel_walls, k);
            let mut skip_la = false;
            let mut reason = String::new();
            if let Some(p) = pred_la {
                if p > 3600.0 || (p > 600.0 && i > 0) {
                    skip_la = true;
                }
            }
            if let Some(p) = pred_rel {
                if p > 3600.0 || (p > 600.0 && i > 0) {
                    reason = format!("relation phase predicted {:.0}s", p);
                }
            }
            let _ = pred_tot;
            if !reason.is_empty() {
                let mut j = J::new();
                j.n("k", k)
                    .n("idx", i)
                    .s("arm", arm.name())
                    .n("kappa", kappa)
                    .s("status", "skipped")
                    .s("reason", &reason);
                append(&sh.lock, "ic_cells.jsonl", &j.done());
                continue;
            }
            let seed = 0x1C00_0000u64 ^ ((k as u64) << 16) ^ ((i as u64) << 8) ^ ci as u64;
            let rp = k <= 14;
            let mut cell = run_ic_cell(&inst, arm, kappa, seed, 3600.0, rp, skip_la);
            if skip_la {
                cell.la_fail = "skipped_by_budget_rule".into();
            }
            append(&sh.lock, "ic_cells.jsonl", &cell.to_json());
            eprintln!(
                "ic k={} i={} {} kappa={} status={} rel={:.1}s la={:.1}s cross={}/{} cover={} verify_failed={} push_fail={}",
                k, i, arm.name(), kappa, cell.status, cell.rel_wall_s, cell.la_wall_s, cell.cross_ok, cell.cross_pairs, cell.coverage, cell.ctr.verify_failed, cell.push_fail
            );
        }
    }
    let _ = sh.kmax;
}

fn cmd_cells(args: &[String]) {
    let mut kmax = KMAX;
    let mut kmin = KMIN;
    let mut only_rho = false;
    let mut only_ic = false;
    let mut workers = 3usize;
    let mut j = 0;
    while j < args.len() {
        match args[j].as_str() {
            "--kmax" => {
                kmax = args[j + 1].parse().unwrap();
                j += 1;
            }
            "--kmin" => {
                kmin = args[j + 1].parse().unwrap();
                j += 1;
            }
            "--workers" => {
                workers = args[j + 1].parse().unwrap();
                j += 1;
            }
            "--rho-only" => only_rho = true,
            "--ic-only" => only_ic = true,
            _ => panic!("unknown arg {}", args[j]),
        }
        j += 1;
    }
    let mut tasks: Vec<(u32, u32)> = vec![];
    for k in kmin..=kmax {
        for i in 0..n_inst(k) {
            tasks.push((k, i));
        }
    }
    let tasks = Arc::new(tasks);
    let next = Arc::new(AtomicUsize::new(0));
    let sh = Arc::new(Shared {
        lock: Mutex::new(()),
        kmax,
    });
    let mut hs = vec![];
    for _ in 0..workers {
        let (tasks, next, sh) = (tasks.clone(), next.clone(), sh.clone());
        hs.push(std::thread::spawn(move || loop {
            let n = next.fetch_add(1, Ordering::SeqCst);
            if n >= tasks.len() {
                break;
            }
            let (k, i) = tasks[n];
            run_instance(&sh, k, i, only_rho, only_ic);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
}

// ---------------------------------------------------------------------------
// fit
// ---------------------------------------------------------------------------

fn ols(xs: &[f64], ys: &[f64]) -> (f64, f64, f64) {
    let n = xs.len() as f64;
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (y - a - b * x).powi(2))
        .sum();
    let se = if n > 2.0 {
        (rss / (n - 2.0) / sxx).sqrt()
    } else {
        f64::NAN
    };
    (a, b, se)
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn quantile(v: &mut Vec<f64>, p: f64) -> f64 {
    v.retain(|x| x.is_finite());
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = ((v.len() - 1) as f64 * p).round() as usize;
    v[idx]
}

/// A series: per `k`, the unit costs (one per instance, or one per rho run).
#[derive(Clone, Default)]
struct Series {
    units: HashMap<u32, Vec<f64>>,
}

impl Series {
    fn push(&mut self, k: u32, c: f64) {
        self.units.entry(k).or_default().push(c);
    }
    fn means(&self) -> Vec<(u32, f64, usize)> {
        let mut v: Vec<(u32, f64, usize)> = self
            .units
            .iter()
            .map(|(&k, u)| (k, mean(u), u.len()))
            .collect();
        v.sort_by_key(|x| x.0);
        v
    }
}

struct FitOut {
    a: f64,
    b: f64,
    se: f64,
    lo: f64,
    hi: f64,
    n_k: usize,
    ks: Vec<u32>,
}

/// OLS of `log2(mean cost)` on `k` over `k >= 10`, with a bootstrap interval resampling units.
fn fit_series(s: &Series, seed: u64) -> Option<FitOut> {
    let m: Vec<(u32, f64, usize)> = s
        .means()
        .into_iter()
        .filter(|x| x.0 >= 10 && x.1 > 0.0)
        .collect();
    if m.len() < 3 {
        return None;
    }
    let xs: Vec<f64> = m.iter().map(|x| x.0 as f64).collect();
    let ys: Vec<f64> = m.iter().map(|x| x.1.log2()).collect();
    let (a, b, se) = ols(&xs, &ys);
    let mut rng = SplitMix64::new(seed);
    let mut bs = vec![];
    for _ in 0..2000 {
        let mut yb = vec![];
        for x in &m {
            let u = &s.units[&x.0];
            let mut t = 0.0;
            for _ in 0..u.len() {
                t += u[rng.below(u.len() as u64) as usize];
            }
            yb.push((t / u.len() as f64).max(1e-300).log2());
        }
        bs.push(ols(&xs, &yb).1);
    }
    let lo = quantile(&mut bs.clone(), 0.025);
    let hi = quantile(&mut bs, 0.975);
    Some(FitOut {
        a,
        b,
        se,
        lo,
        hi,
        n_k: m.len(),
        ks: m.iter().map(|x| x.0).collect(),
    })
}

fn cmd_fit() {
    let calib = read_jsonl("calib.jsonl");
    let mut rinv_k: HashMap<u32, Vec<f64>> = HashMap::new();
    for c in &calib {
        rinv_k
            .entry(fval(c, "k") as u32)
            .or_default()
            .push(fval(c, "rho_inv"));
    }
    let rinv = |k: u32| -> f64 {
        let mut v = rinv_k.get(&k).cloned().unwrap_or(vec![]);
        quantile(&mut v, 0.5)
    };
    let rho = read_jsonl("rho_cells.jsonl");
    let ic = read_jsonl("ic_cells.jsonl");
    let wells = [("w9", 9.0f64), ("w1", 1.0f64)];
    let mut series: HashMap<String, Series> = HashMap::new();
    let mut unpriced_rho: HashMap<u32, Vec<f64>> = HashMap::new(); // per k, per run
    let mut gae_k: HashMap<u32, Vec<f64>> = HashMap::new();
    let mut ell_k: HashMap<u32, f64> = HashMap::new();
    let mut rho_n: HashMap<u32, (usize, usize)> = HashMap::new(); // ok, total
    for r in &rho {
        let k = fval(r, "k") as u32;
        let e = rho_n.entry(k).or_default();
        e.1 += 1;
        if fval(r, "ok") != 1.0 || fval(r, "verified") != 1.0 {
            continue;
        }
        e.0 += 1;
        ell_k.insert(k, fval(r, "ell"));
        let ri = rinv(k);
        let walk = fval(r, "fq_mul") + ri * fval(r, "fq_inv");
        let other = fval(r, "setup_fq_mul")
            + fval(r, "restart_fq_mul")
            + ri * (fval(r, "setup_fq_inv") + fval(r, "restart_fq_inv"));
        gae_k.entry(k).or_default().push(walk / fval(r, "steps"));
        for (nm, w) in &wells {
            series
                .entry(format!("rho_total_{}", nm))
                .or_default()
                .push(k, walk + other + w * fval(r, "modl_adds"));
        }
        unpriced_rho.entry(k).or_default().push(
            fval(r, "dp_checks")
                + fval(r, "dp_lookups")
                + fval(r, "dp_inserts")
                + fval(r, "part_hashes")
                + fval(r, "rng_draws"),
        );
    }
    // IC: per (arm, kappa) per instance
    let mut unpriced_ic: HashMap<(String, u32), Vec<f64>> = HashMap::new();
    let mut cover: HashMap<u32, (usize, usize)> = HashMap::new();
    let mut cross: HashMap<u32, (usize, usize)> = HashMap::new();
    for c in &ic {
        let st = sval(c, "status");
        if st != "ok" && st != "rel_only" {
            continue;
        }
        let k = fval(c, "k") as u32;
        let arm = sval(c, "arm");
        let kappa = fval(c, "kappa") as u32;
        let ri = rinv(k);
        let rel = fval(c, "fq_mul_collect")
            + fval(c, "fq_mul_fb")
            + ri * (fval(c, "fq_inv_collect") + fval(c, "fq_inv_fb"));
        let lops = fval(c, "lops");
        let name = if arm == "FULL" {
            "FULL".to_string()
        } else {
            format!("{}_k{}", arm, kappa)
        };
        let unp = fval(c, "hash_lookups")
            + fval(c, "hash_inserts")
            + fval(c, "rng_draws")
            + fval(c, "uf_ops")
            + fval(c, "elim_entries");
        {
            let e = cover.entry(k).or_default();
            e.1 += 1;
            if fval(c, "coverage") == 1.0 {
                e.0 += 1;
            }
        }
        if st == "ok" {
            let e = cross.entry(k).or_default();
            e.1 += 1;
            if fval(c, "cross_ok") == fval(c, "cross_pairs") && fval(c, "cross_pairs") > 0.0 {
                e.0 += 1;
            }
        }
        series
            .entry(format!("{}_rel", name))
            .or_default()
            .push(k, rel);
        unpriced_ic.entry((name.clone(), k)).or_default().push(unp);
        for (nm, w) in &wells {
            if st == "ok" {
                series
                    .entry(format!("{}_la_{}", name, nm))
                    .or_default()
                    .push(k, w * lops);
                series
                    .entry(format!("{}_total_{}", name, nm))
                    .or_default()
                    .push(k, rel + w * lops);
            }
        }
    }
    // fits
    let mut out = J::new();
    let mut names: Vec<String> = series.keys().cloned().collect();
    names.sort();
    let mut fits = HashMap::new();
    let mut fj = vec![];
    for (si, n) in names.iter().enumerate() {
        if let Some(f) = fit_series(&series[n], 0xF17 + si as u64) {
            fj.push(format!(
                "\"{}\":{{\"intercept\":{},\"slope\":{},\"se\":{},\"ci_lo\":{},\"ci_hi\":{},\"n_k\":{},\"ks\":{}}}",
                n,
                f.a,
                f.b,
                f.se,
                f.lo,
                f.hi,
                f.n_k,
                format_args!("[{}]", f.ks.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
            ));
            fits.insert(n.clone(), f);
        }
    }
    out.raw("fits", &format!("{{{}}}", fj.join(",")));
    // per-k table with labelled extrapolation
    let mut rows = vec![];
    for n in &names {
        let s = &series[n];
        let m = s.means();
        let measured: HashMap<u32, (f64, usize)> = m.iter().map(|x| (x.0, (x.1, x.2))).collect();
        for k in KMIN..=KMAX {
            let ell = ell_k.get(&k).cloned();
            if let Some(&(v, cnt)) = measured.get(&k) {
                let u = &s.units[&k];
                let mut bs = vec![];
                let mut rng = SplitMix64::new(k as u64 * 77 + 5);
                for _ in 0..2000 {
                    let mut t = 0.0;
                    for _ in 0..u.len() {
                        t += u[rng.below(u.len() as u64) as usize];
                    }
                    bs.push(t / u.len() as f64);
                }
                let (lo, hi) = (quantile(&mut bs.clone(), 0.025), quantile(&mut bs, 0.975));
                rows.push(format!(
                    "{{\"series\":\"{}\",\"k\":{},\"cost\":{},\"lo\":{},\"hi\":{},\"n\":{},\"source\":\"measured\",\"S\":{}}}",
                    n, k, v, lo, hi, cnt, ell.map(|e| v / e.sqrt()).unwrap_or(f64::NAN)
                ));
            } else if let Some(f) = fits.get(n) {
                let v = (f.a + f.b * k as f64).exp2();
                rows.push(format!("{{\"series\":\"{}\",\"k\":{},\"cost\":{},\"n\":0,\"source\":\"extrapolated\",\"slope\":{}}}", n, k, v, f.b));
            }
        }
    }
    fs::write(path("percost.jsonl"), rows.join("\n") + "\n").unwrap();
    // GAE band ratios (best-kappa total per family vs rho), measured k only
    let mut band = vec![];
    for (nm, wl) in &wells {
        let _ = wl;
        let rho_s = match series.get(&format!("rho_total_{}", nm)) {
            Some(s) => s.means(),
            None => continue,
        };
        for (k, rc, _) in rho_s {
            let gae = mean(&gae_k[&k]);
            let rho_unp = mean(&unpriced_rho[&k]);
            for fam in ["FULL", "SLP", "DLP"] {
                let mut best: Option<(String, f64)> = None;
                for n in &names {
                    if n.ends_with(&format!("_total_{}", nm))
                        && (n.starts_with(&format!("{}_", fam))
                            || (fam == "FULL" && n.starts_with("FULL")))
                    {
                        let base = n.trim_end_matches(&format!("_total_{}", nm)).to_string();
                        if let Some((_, v, _)) = series[n].means().into_iter().find(|x| x.0 == k) {
                            if best.as_ref().map(|b| v < b.1).unwrap_or(true) {
                                best = Some((base, v));
                            }
                        }
                    }
                }
                if let Some((base, v)) = best {
                    let unp = mean(&unpriced_ic[&(base.clone(), k)]);
                    let mut ratios = vec![];
                    for w in [0.0, 0.1, 1.0] {
                        let ic_c = v + w * gae * unp;
                        let rho_c = rc + w * gae * rho_unp;
                        ratios.push(ic_c / rho_c);
                    }
                    let all_below = ratios.iter().all(|&r| r < 1.0);
                    let all_above = ratios.iter().all(|&r| r > 1.0);
                    band.push(format!(
                        "{{\"w_ell\":\"{}\",\"k\":{},\"family\":\"{}\",\"best\":\"{}\",\"ic_total\":{},\"rho_total\":{},\"gae\":{},\"ic_unpriced\":{},\"rho_unpriced\":{},\"ratio_w0\":{},\"ratio_w01\":{},\"ratio_w1\":{},\"order\":\"{}\"}}",
                        nm,
                        k,
                        fam,
                        base,
                        v,
                        rc,
                        gae,
                        unp,
                        rho_unp,
                        ratios[0],
                        ratios[1],
                        ratios[2],
                        if all_below { "ic_below_rho_across_band" } else if all_above { "ic_above_rho_across_band" } else { "unresolved_at_boundary" }
                    ));
                }
            }
        }
    }
    fs::write(path("band.jsonl"), band.join("\n") + "\n").unwrap();
    // coverage and cross-check summary
    let mut cv = vec![];
    for k in KMIN..=KMAX {
        if let Some(c) = cover.get(&k) {
            let x = cross.get(&k).cloned().unwrap_or((0, 0));
            cv.push(format!("{{\"k\":{},\"cells\":{},\"coverage_true\":{},\"cross_cells\":{},\"cross_all_ok\":{}}}", k, c.1, c.0, x.1, x.0));
        }
    }
    out.raw("coverage", &format!("[{}]", cv.join(",")));
    let rn: Vec<String> = (KMIN..=KMAX)
        .filter_map(|k| {
            rho_n
                .get(&k)
                .map(|x| format!("{{\"k\":{},\"ok\":{},\"runs\":{}}}", k, x.0, x.1))
        })
        .collect();
    out.raw("rho_runs", &format!("[{}]", rn.join(",")));
    // crossover (point estimate and bootstrap) between the best total IC fit and rho
    for (nm, _) in &wells {
        if let Some(fr) = fits.get(&format!("rho_total_{}", nm)) {
            let mut cx = vec![];
            for n in &names {
                if n.ends_with(&format!("_total_{}", nm)) && !n.starts_with("rho") {
                    if let Some(fi) = fits.get(n) {
                        let db = fi.b - fr.b;
                        let kstar = if db.abs() < 1e-9 {
                            f64::NAN
                        } else {
                            (fr.a - fi.a) / db
                        };
                        cx.push(format!(
                            "\"{}\":{{\"slope_ic\":{},\"slope_rho\":{},\"k_star\":{}}}",
                            n, fi.b, fr.b, kstar
                        ));
                    }
                }
            }
            out.raw(
                &format!("crossover_{}", nm),
                &format!("{{{}}}", cx.join(",")),
            );
        }
    }
    fs::write(path("fits.json"), out.done()).unwrap();
    println!("{}", out.done());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");
    match cmd {
        "gen" => cmd_gen(false),
        "gen-status" => cmd_gen(true),
        "cells" => cmd_cells(&args[2..]),
        "fit" => cmd_fit(),
        _ => eprintln!("usage: diem_quartic_study gen | gen-status | cells [--kmin K --kmax K --workers N --rho-only --ic-only] | fit"),
    }
}
