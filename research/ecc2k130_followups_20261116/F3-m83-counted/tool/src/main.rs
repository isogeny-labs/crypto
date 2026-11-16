//! F3 estimator: fits the frozen cost model of PROTOCOL.md section 5 to committed
//! counted rows (L7 table: PX `pipeline`/`pipeline71`, FC `ic-compact`, all w = 0) and
//! to this lane's smoke sessions, then prices the m = 83 row. Writes results.json and
//! fig_cost_vs_log2r.svg. Deterministic: same inputs, same bytes.
//!
//! Usage: f3est --l7 L7/results.json --px-specs DIR [--smoke SESSION_DIR]...
//!              --rho-ns-per-step X --ic-ns-per-gae Y --out DIR

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

const R83: f64 = 2417851639230796216685689.0;
const N83: f64 = 83.0;
const BYTES_PER_ENTRY: f64 = 16.0;
const PRICE_LO: f64 = 0.02; // USD per vCPU-hour, assumption (PROTOCOL.md section 5)
const PRICE_HI: f64 = 0.06;

#[derive(Clone, Debug)]
struct Row {
    src: String,
    n: f64,
    log2r: f64,
    c: f64,
    g_ic: f64,
    g_rho: f64,
    targets: usize,
}

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

fn args_all(args: &[String], k: &str) -> Vec<String> {
    let mut v = vec![];
    for i in 0..args.len() {
        if args[i] == k {
            if let Some(x) = args.get(i + 1) {
                v.push(x.clone());
            }
        }
    }
    v
}

fn read_json(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap_or_else(|e| panic!("{p}: {e}")))
        .unwrap_or_else(|e| panic!("{p}: {e}"))
}

fn columns_of(params: &Value) -> Option<f64> {
    let fb = params.get("factor_base")?.as_str()?;
    let i = fb.find("columns=")? + 8;
    let rest = &fb[i..];
    let end = rest.find(',').unwrap_or(rest.len());
    rest[..end].parse().ok()
}

fn committed_rows(l7: &Value, px_dir: &str) -> Vec<Row> {
    let mut rows = vec![];
    for r in l7["rows"].as_array().unwrap() {
        let src = r["source"].as_str().unwrap_or("");
        let arm = r["arm"].as_str().unwrap_or("");
        if r["regime"] != "koblitz" {
            continue;
        }
        let (Some(s0), Some(rho0)) = (r["S"][0].as_f64(), r["rho_S"][0].as_f64()) else {
            continue;
        };
        let n = r["n"].as_f64().unwrap();
        let log2r = r["log2_r"].as_f64().unwrap();
        let c = match (src, arm) {
            ("FC", "ic-compact") => r["columns"].as_f64(),
            ("PX", "pipeline") | ("PX", "pipeline71") => {
                let spec = read_json(&format!("{px_dir}/K-k{}.json", n as u32));
                spec["arms"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|a| a["name"] == arm)
                    .and_then(|a| columns_of(&a["method"]["params"]))
            }
            _ => None,
        };
        let Some(c) = c else { continue };
        let sq = (log2r / 2.0).exp2();
        rows.push(Row {
            src: format!("{src}:{arm}"),
            n,
            log2r,
            c,
            g_ic: s0 * sq,
            g_rho: rho0 * sq,
            targets: r["verified"].as_f64().unwrap_or(0.0) as usize,
        });
    }
    rows
}

/// One smoke session: mean total GAE per arm over verified records, columns from the spec.
fn smoke_row(dir: &str, checks: &mut Vec<Value>) -> Row {
    let spec = read_json(&format!("{dir}/spec.json"));
    let n = spec["workloads"]["curves"][0]["n"].as_f64().unwrap();
    let c = spec["arms"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "ic-pipeline")
        .and_then(|a| columns_of(&a["method"]["params"]))
        .unwrap();
    let text = fs::read_to_string(format!("{dir}/records.jsonl")).unwrap();
    let mut by_arm: BTreeMap<String, Vec<(String, f64, bool, f64)>> = BTreeMap::new();
    let mut r_val = 0.0f64;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let rec: Value = serde_json::from_str(line).unwrap();
        let arm = rec["arm"].as_str().unwrap_or("?").to_string();
        if rec["warmup"].as_bool() == Some(true) {
            continue;
        }
        let gae = rec["cost"]["total_gae"].as_f64().unwrap_or(f64::NAN);
        let s = rec["cost"]["s"].as_f64().unwrap_or(f64::NAN);
        if s.is_finite() && gae.is_finite() && s > 0.0 {
            r_val = (gae / s).powi(2);
        }
        let ok = rec["outcome"]["status"] == "verified" && rec["outcome"]["matches_target"] == true;
        let wall = rec["time"]["solve_wall_ns"].as_f64().unwrap_or(f64::NAN);
        let tgt = format!("{:06}", rec["workload"]["target_index"].as_u64().unwrap_or(u64::MAX));
        by_arm.entry(arm).or_default().push((tgt, gae, ok, wall));
    }
    let mean = |a: &str| -> (f64, usize, usize, f64) {
        let v = by_arm.get(a).cloned().unwrap_or_default();
        let ok: Vec<_> = v.iter().filter(|x| x.2).collect();
        let g = ok.iter().map(|x| x.1).sum::<f64>() / ok.len().max(1) as f64;
        let w = ok.iter().map(|x| x.3).sum::<f64>() / ok.len().max(1) as f64;
        (g, ok.len(), v.len(), w)
    };
    let (gi, oki, ni, wi) = mean("ic-pipeline");
    let (gr, okr, nr, wr) = mean("rho-strong");
    let (ga, oka, na, _) = mean("rho-strong-aa");
    // A/A: identical GAE target by target, in record order.
    let mut rho = by_arm.get("rho-strong").cloned().unwrap_or_default();
    let mut aa = by_arm.get("rho-strong-aa").cloned().unwrap_or_default();
    rho.sort_by(|x, y| x.0.cmp(&y.0));
    aa.sort_by(|x, y| x.0.cmp(&y.0));
    let aa_exact = rho.len() == aa.len()
        && rho.iter().zip(aa.iter()).all(|(x, y)| x.0 == y.0 && x.1 == y.1);
    checks.push(json!({
        "session": dir, "n": n, "columns": c, "r_from_records": r_val,
        "ic": {"verified": oki, "records": ni, "mean_gae": gi, "mean_wall_ns": wi, "ns_per_gae": wi / gi},
        "rho": {"verified": okr, "records": nr, "mean_gae": gr, "mean_wall_ns": wr, "ns_per_gae": wr / gr},
        "rho_aa": {"verified": oka, "records": na, "mean_gae": ga},
        "aa_exact": aa_exact,
        "ratio_ic_over_rho_w0": gi / gr,
    }));
    Row {
        src: "F3:smoke".into(),
        n,
        log2r: r_val.log2(),
        c,
        g_ic: gi,
        g_rho: gr,
        targets: oki,
    }
}

fn model(a: f64, b: f64, n: f64, r: f64, c: f64) -> f64 {
    a * n * c * c + b * r / (n * n * c)
}

/// Weighted least squares for relative error: min sum ((a x1 + b x2)/y - 1)^2.
fn fit(rows: &[Row]) -> (f64, f64) {
    let (mut saa, mut sab, mut sbb, mut sa, mut sb) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for r in rows {
        let rr = r.log2r.exp2();
        let a = r.n * r.c * r.c / r.g_ic;
        let b = rr / (r.n * r.n * r.c) / r.g_ic;
        saa += a * a;
        sab += a * b;
        sbb += b * b;
        sa += a;
        sb += b;
    }
    let det = saa * sbb - sab * sab;
    ((sa * sbb - sb * sab) / det, (saa * sb - sab * sa) / det)
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 11
    }
}

fn pct(v: &mut [f64], p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * p).round() as usize]
}

fn rho_model(n: f64, r: f64) -> f64 {
    (std::f64::consts::PI * r / (4.0 * n)).sqrt()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let l7 = read_json(&arg(&args, "--l7").expect("--l7"));
    let px = arg(&args, "--px-specs").expect("--px-specs");
    let out = arg(&args, "--out").expect("--out");
    // Rate probe: every record exhausted its 10^6-step cap (koblitz_strong_rho.rs: the cap
    // is max(10^6, factor x ideal); factor 0 gives 10^6). ns per step = median wall / 10^6.
    let probe = arg(&args, "--probe").expect("--probe");
    let mut probe_ns: Vec<f64> = vec![];
    let mut probe_status: Vec<String> = vec![];
    for line in fs::read_to_string(format!("{probe}/records.jsonl")).unwrap().lines() {
        let rec: Value = serde_json::from_str(line).unwrap();
        probe_status.push(rec["outcome"]["status"].as_str().unwrap_or("?").to_string());
        probe_ns.push(rec["time"]["solve_wall_ns"].as_f64().unwrap() / 1.0e6);
    }
    let probe_all = probe_ns.clone();
    let rho_ns = pct(&mut probe_ns, 0.5);
    // The IC arm has no wide implementation: its ns per GAE is assumed equal (PROTOCOL 5).
    let ic_ns = rho_ns;
    let mut rows = committed_rows(&l7, &px);
    let mut smoke_checks = vec![];
    for d in args_all(&args, "--smoke") {
        rows.push(smoke_row(&d, &mut smoke_checks));
    }
    // Rows in the mitm regime only: C >= 8 (below that the arm is a collision walk and
    // the table term is negligible; they are reported but not fitted).
    let fit_rows: Vec<Row> = rows.iter().filter(|r| r.c >= 8.0).cloned().collect();
    let (a, b) = fit(&fit_rows);
    let resid: Vec<f64> = fit_rows
        .iter()
        .map(|r| (r.g_ic / model(a, b, r.n, r.log2r.exp2(), r.c)).ln())
        .collect();
    let sd = (resid.iter().map(|x| x * x).sum::<f64>() / (resid.len() as f64 - 2.0)).sqrt();
    // k_rho on the rows with log2 r >= 30 (small curves carry fixed overheads).
    let k_rho: Vec<f64> = rows
        .iter()
        .filter(|r| r.log2r >= 30.0)
        .map(|r| r.g_rho / rho_model(r.n, r.log2r.exp2()))
        .collect();
    // Drift of ln(measured/model) with log2 r on the fitted rows: a systematic the
    // two-term model leaves; applied at m = 83 as a separate reading, not as the estimate.
    let xm = fit_rows.iter().map(|r| r.log2r).sum::<f64>() / fit_rows.len() as f64;
    let (mut sxy, mut sxx) = (0.0, 0.0);
    for (r, e) in fit_rows.iter().zip(resid.iter()) {
        sxy += (r.log2r - xm) * e;
        sxx += (r.log2r - xm).powi(2);
    }
    let drift = sxy / sxx;
    let drift_factor_83 = (drift * (R83.log2() - xm)).exp();
    let k_rho_mean = k_rho.iter().sum::<f64>() / k_rho.len() as f64;
    let k_lo = k_rho.iter().cloned().fold(f64::INFINITY, f64::min);
    let k_hi = k_rho.iter().cloned().fold(0.0, f64::max);
    let g_rho83 = k_rho_mean * rho_model(N83, R83);
    let c0 = (0.6 * (R83 / N83.powi(3)).cbrt()).round();
    let c_opt = |a: f64, b: f64| (b * R83 / (2.0 * a * N83.powi(3))).cbrt();
    let cap_c = |bytes: f64| (bytes / (BYTES_PER_ENTRY * N83)).sqrt().floor();
    let gib = (1u64 << 30) as f64;
    let scenarios: Vec<(&str, Box<dyn Fn(f64, f64) -> f64>)> = vec![
        ("rule_c0", Box::new(move |_, _| c0)),
        ("model_optimum", Box::new(move |a, b| c_opt(a, b))),
        ("table_cap_16GiB", Box::new(move |a, b| c_opt(a, b).min(cap_c(16.0 * gib)))),
        ("table_cap_128GiB", Box::new(move |a, b| c_opt(a, b).min(cap_c(128.0 * gib)))),
        ("table_cap_1TiB", Box::new(move |a, b| c_opt(a, b).min(cap_c(1024.0 * gib)))),
    ];
    // Bootstrap over fitted rows for the (a, b) uncertainty.
    let mut rng = Lcg(20261116305);
    let mut boot: Vec<(f64, f64)> = vec![];
    while boot.len() < 4000 {
        let s: Vec<Row> = (0..fit_rows.len())
            .map(|_| fit_rows[(rng.next() % fit_rows.len() as u64) as usize].clone())
            .collect();
        let (x, y) = fit(&s);
        if x > 0.0 && y > 0.0 && x.is_finite() && y.is_finite() {
            boot.push((x, y));
        }
    }
    let z = 1.96 * sd;
    let mut sc_out = vec![];
    for (name, cf) in &scenarios {
        let c = cf(a, b);
        let g = model(a, b, N83, R83, c);
        let mut gb: Vec<f64> = boot.iter().map(|(x, y)| model(*x, *y, N83, R83, cf(*x, *y))).collect();
        let lo = pct(&mut gb, 0.025) * (-z).exp();
        let hi = pct(&mut gb, 0.975) * z.exp();
        let table_bytes = N83 * c * c * BYTES_PER_ENTRY;
        let core_h = |g: f64| g * ic_ns / 3.6e12;
        sc_out.push(json!({
            "scenario": name, "columns": c,
            "table_entries": N83 * c * c, "table_bytes": table_bytes, "table_TiB": table_bytes / (1024.0 * gib),
            "ic_gae": g, "ic_gae_log2": g.log2(), "ic_gae_95": [lo, hi],
            "ratio_to_rho": g / g_rho83, "ratio_to_rho_95": [lo / g_rho83, hi / g_rho83],
            "ic_gae_with_drift": g * drift_factor_83, "ratio_to_rho_with_drift": g * drift_factor_83 / g_rho83,
            "ic_core_hours_per_target": core_h(g), "ic_core_hours_95": [core_h(lo), core_h(hi)],
            "usd_per_target": [core_h(g) * PRICE_LO, core_h(g) * PRICE_HI],
        }));
    }
    let rho_core_h = g_rho83 * rho_ns / 3.6e12;
    let row_json: Vec<Value> = rows
        .iter()
        .map(|r| {
            let m = model(a, b, r.n, r.log2r.exp2(), r.c);
            json!({"src": r.src, "n": r.n, "log2_r": r.log2r, "columns": r.c, "ic_gae": r.g_ic,
                   "rho_gae": r.g_rho, "ratio_w0": r.g_ic / r.g_rho, "model_gae": m,
                   "measured_over_model": r.g_ic / m, "fitted": r.c >= 8.0, "verified_targets": r.targets})
        })
        .collect();
    let res = json!({
        "schema": "f3.m83_estimate/v1",
        "pcc": "[pcc:unverified]",
        "protocol": "research/ecc2k130_followups_20261116/F3-m83-counted/PROTOCOL.md section 5",
        "instance": {"n": 83, "r": "2417851639230796216685689", "log2_r": R83.log2()},
        "model": {"ic": "alpha*n*C^2 + beta*r/(n^2*C)", "alpha": a, "beta": b,
                  "fitted_rows": fit_rows.len(), "log_resid_sd": sd,
                  "drift_ln_per_log2r_bit": drift, "drift_factor_at_m83": drift_factor_83,
                  "rho": "k_rho*sqrt(pi*r/(4n))", "k_rho_mean": k_rho_mean, "k_rho_range": [k_lo, k_hi]},
        "rho_m83": {"gae": g_rho83, "gae_log2": g_rho83.log2(), "ns_per_step": rho_ns,
                    "core_hours_per_target": rho_core_h,
                    "usd_per_target": [rho_core_h * PRICE_LO, rho_core_h * PRICE_HI]},
        "ic_ns_per_gae_assumed": ic_ns,
        "probe": {"session": probe, "ns_per_step_each": probe_all, "status_each": probe_status,
                  "label": "L0, shared 4-vCPU container, cores 0-2, provisional"},
        "price_usd_per_vcpu_hour_assumed": [PRICE_LO, PRICE_HI],
        "scenarios": sc_out,
        "smoke": smoke_checks,
        "rows": row_json,
    });
    fs::create_dir_all(&out).unwrap();
    fs::write(format!("{out}/results.json"), serde_json::to_string_pretty(&res).unwrap() + "\n").unwrap();
    fs::write(format!("{out}/fig_cost_vs_log2r.svg"), svg(&rows, a, b, k_rho_mean, &res)).unwrap();
    println!("{}", serde_json::to_string_pretty(&json!({"model": res["model"], "rho_m83": res["rho_m83"], "scenarios": res["scenarios"], "smoke": res["smoke"]})).unwrap());
}

fn svg(rows: &[Row], a: f64, b: f64, k_rho: f64, res: &Value) -> String {
    let (w, h, l, rgt, t, bt) = (760.0, 480.0, 70.0, 20.0, 40.0, 50.0);
    let (x0, x1, y0, y1) = (10.0, 84.0, 0.0, 64.0);
    let px = |x: f64| l + (x - x0) / (x1 - x0) * (w - l - rgt);
    let py = |y: f64| h - bt - (y - y0) / (y1 - y0) * (h - t - bt);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='11'>\n<rect width='100%' height='100%' fill='white'/>\n");
    s += "<text x='70' y='22' font-size='13'>F3: counted cost per target (log2 GAE) vs log2 r, a=0/a=1 Koblitz, w = 0 [pcc:unverified]</text>\n";
    for g in (0..=64).step_by(8) {
        let y = py(g as f64);
        s += &format!("<line x1='{l}' x2='{}' y1='{y}' y2='{y}' stroke='#eee'/><text x='{}' y='{}' text-anchor='end'>{g}</text>\n", w - rgt, l - 6.0, y + 4.0);
    }
    for g in (10..=80).step_by(10) {
        let x = px(g as f64);
        s += &format!("<line x1='{x}' x2='{x}' y1='{t}' y2='{}' stroke='#eee'/><text x='{x}' y='{}' text-anchor='middle'>{g}</text>\n", h - bt, h - bt + 16.0);
    }
    s += &format!("<text x='{}' y='{}' text-anchor='middle'>log2 r</text><text x='16' y='{}' transform='rotate(-90 16 {})' text-anchor='middle'>log2 GAE per target</text>\n", (l + w) / 2.0, h - 12.0, h / 2.0, h / 2.0);
    for r in rows {
        let col = match r.src.as_str() {
            "F3:smoke" => "#d62728",
            s if s.starts_with("PX") => "#1f77b4",
            _ => "#2ca02c",
        };
        s += &format!("<circle cx='{}' cy='{}' r='3.5' fill='{col}'/>\n", px(r.log2r), py(r.g_ic.log2()));
        s += &format!("<rect x='{}' y='{}' width='5' height='5' fill='none' stroke='#555'/>\n", px(r.log2r) - 2.5, py(r.g_rho.log2()) - 2.5);
    }
    // m=83 predictions
    let lr = R83.log2();
    s += &format!("<rect x='{}' y='{}' width='7' height='7' fill='#555'/>\n", px(lr) - 3.5, py((k_rho * rho_model(N83, R83)).log2()) - 3.5);
    for (i, sc) in res["scenarios"].as_array().unwrap().iter().enumerate() {
        let g = sc["ic_gae"].as_f64().unwrap();
        let lo = sc["ic_gae_95"][0].as_f64().unwrap();
        let hi = sc["ic_gae_95"][1].as_f64().unwrap();
        let x = px(lr) - 24.0 + 9.0 * i as f64;
        s += &format!("<line x1='{x}' x2='{x}' y1='{}' y2='{}' stroke='#9467bd'/><circle cx='{x}' cy='{}' r='3' fill='#9467bd'/>\n", py(lo.log2()), py(hi.log2().min(y1)), py(g.log2()));
    }
    let _ = (a, b);
    s += "<g transform='translate(80,60)'><circle cx='0' cy='0' r='3.5' fill='#1f77b4'/><text x='8' y='4'>PX pipeline / pipeline71 IC (committed)</text>\
<circle cx='0' cy='16' r='3.5' fill='#2ca02c'/><text x='8' y='20'>FC ic-compact IC (committed)</text>\
<circle cx='0' cy='32' r='3.5' fill='#d62728'/><text x='8' y='36'>F3 smoke IC (this lane, L0)</text>\
<rect x='-2.5' y='45.5' width='5' height='5' fill='none' stroke='#555'/><text x='8' y='52'>matched rho (same session)</text>\
<rect x='-3.5' y='60.5' width='7' height='7' fill='#555'/><text x='8' y='68'>m=83 rho, model x k_rho</text>\
<circle cx='0' cy='80' r='3' fill='#9467bd'/><text x='8' y='84'>m=83 IC model, 95% band: c0 rule, optimum, 16 GiB, 128 GiB, 1 TiB table caps (left to right)</text></g>\n";
    s += "</svg>\n";
    s
}
