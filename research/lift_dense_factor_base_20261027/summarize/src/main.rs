//! Native scoring of the lift-dense study (Amendment 3 replacement of the
//! historical `summarize.py`, which is kept as evidence and not executed).
//!
//! Usage: lift-dense-summarize <run-dir>
//! Reads `rows.jsonl` (confirmatory run), `lines_amend1/rows.jsonl`,
//! `lines_amend3/rows.jsonl`, `lines_amend3/u_census.jsonl`,
//! `c2pnb_native.json` and the historical `c2pnb_delta.json`; writes
//! `summary.json`. Scores PROTOCOL §4 as registered (H-A, H-B) and as
//! amended (H-A′, H-A″, H-B′), H-C, the kmax tables and the c2pnb verdicts.

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::path::Path;

fn read_jsonl(p: &Path) -> Vec<Value> {
    std::fs::read_to_string(p)
        .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("json row"))
        .collect()
}

fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64()
        .unwrap_or_else(|| panic!("missing {k} in {v}"))
}
fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64()
        .unwrap_or_else(|| panic!("missing {k} in {v}"))
}
fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v[k].as_str().unwrap_or_else(|| panic!("missing {k}"))
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
fn pstdev(x: &[f64]) -> f64 {
    let m = mean(x);
    (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / x.len() as f64).sqrt()
}
fn median(x: &[f64]) -> f64 {
    let mut y = x.to_vec();
    y.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = y.len();
    if n % 2 == 1 {
        y[n / 2]
    } else {
        0.5 * (y[n / 2 - 1] + y[n / 2])
    }
}

/// log2 of the Gaussian binomial [n, k]_2, and the expected number of
/// k-dim subspaces whose nonzero elements all lie in a uniform random
/// `size`-subset of F_q^*.
fn expect(n: u32, k: u32, size: u64) -> f64 {
    let mut g = 1.0f64;
    for i in 0..k {
        g *= (2f64.powi((n - i) as i32) - 1.0) / (2f64.powi((k - i) as i32) - 1.0);
    }
    let tot = 2f64.powi(n as i32) - 1.0;
    for i in 0..(1u64 << k) - 1 {
        g *= (size as f64 - i as f64).max(0.0) / (tot - i as f64);
    }
    g
}

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

/// One-sided Mann–Whitney for H1: x < y. Returns (U, permutation p,
/// normal-approximation p). U counts pairs with x_i < y_j (ties 1/2).
fn mann_whitney_less(x: &[f64], y: &[f64], reps: u64) -> (f64, f64, f64) {
    let ustat = |a: &[f64], b: &[f64]| {
        let mut u = 0.0;
        for &xi in a {
            for &yj in b {
                u += if xi < yj {
                    1.0
                } else if xi == yj {
                    0.5
                } else {
                    0.0
                };
            }
        }
        u
    };
    let obs = ustat(x, y);
    let mut pool: Vec<f64> = x.iter().chain(y).copied().collect();
    let mut rng = Rng(20261027);
    let mut ge = 0u64;
    for _ in 0..reps {
        for i in (1..pool.len()).rev() {
            let j = (rng.next() % (i as u64 + 1)) as usize;
            pool.swap(i, j);
        }
        if ustat(&pool[..x.len()], &pool[x.len()..]) >= obs {
            ge += 1;
        }
    }
    let (n1, n2) = (x.len() as f64, y.len() as f64);
    let mu = n1 * n2 / 2.0;
    let sd = (n1 * n2 * (n1 + n2 + 1.0) / 12.0).sqrt();
    let z = (obs - mu) / sd;
    let p_norm = 0.5 * erfc(z / std::f64::consts::SQRT_2);
    (obs, (ge + 1) as f64 / (reps + 1) as f64, p_norm)
}

/// Complementary error function (Numerical Recipes erfcc, |rel err| < 1.2e-7).
fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let r = t
        * (-z * z - 1.26551223
            + t * (1.00002368
                + t * (0.37409196
                    + t * (0.09678418
                        + t * (-0.18628806
                            + t * (0.27886807
                                + t * (-1.13520398
                                    + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))))
            .exp();
    if x >= 0.0 {
        r
    } else {
        2.0 - r
    }
}

/// u-count rule. Registered (H-B): n=2 {0, q-1}, n=3 {0, 2(q-1)}, n=4
/// multiple of q-1. Amended (H-B′): n=3 {0, q-1, 2(q-1), q^2-1}.
fn ucount_ok(n: u64, d: u64, c: u64, amended: bool) -> bool {
    let q1 = (1u64 << d) - 1;
    match n {
        2 => c == 0 || c == q1,
        3 => {
            if amended {
                c % q1 == 0 && [0, 1, 2, q1 + 2].contains(&(c / q1))
            } else {
                c == 0 || c == 2 * q1
            }
        }
        _ => c % q1 == 0,
    }
}

fn main() {
    let run = std::path::PathBuf::from(std::env::args().nth(1).expect("run dir"));
    let rows = read_jsonl(&run.join("rows.jsonl"));
    let kind = |r: &Value| r["kind"].as_str().unwrap_or("").to_string();
    let kmax: Vec<&Value> = rows.iter().filter(|r| kind(r) == "kmax").collect();
    let lines: Vec<&Value> = rows.iter().filter(|r| kind(r) == "line").collect();
    let ym2: Vec<&Value> = rows.iter().filter(|r| kind(r) == "yield_m2").collect();
    let mut out = Map::new();
    out.insert(
        "run".into(),
        json!(run.file_name().unwrap().to_string_lossy()),
    );
    out.insert("scorer".into(), json!("native summarize/ (Amendment 3)"));

    // ---- H-A (registered Poisson rule) and H-B / H-B′ on the confirmatory lines
    let mut ha_fail = vec![];
    let (mut hb_fail, mut hbp_fail) = (vec![], vec![]);
    let mut cells: BTreeMap<String, Value> = BTreeMap::new();
    for r in &lines {
        let (nn, d) = (u(r, "N"), u(r, "d"));
        let key = format!("N{nn}_d{d}");
        let n = nn / d;
        let c = u(r, "u_count");
        let ci = u(r, "curve").to_string();
        if !ucount_ok(n, d, c, false) {
            hb_fail.push(json!([key, ci, "u_count", c]));
        }
        if !ucount_ok(n, d, c, true) {
            hbp_fail.push(json!([key, ci, "u_count", c]));
        }
        let cell = cells
            .entry(key.clone())
            .or_insert_with(|| json!({"N": nn, "d": d, "m": u(r, "m"), "curves": {}}));
        let cur = cell["curves"]
            .as_object_mut()
            .unwrap()
            .entry(ci.clone())
            .or_insert_with(|| json!({"Tr_a": u(r, "Tr_a"), "u_count": c}));
        if r["u"].is_null() {
            cur["twisted"] = Value::Null;
            continue;
        }
        let line = s(r, "line");
        cur[line] = json!({"lifting": u(r, "lifting"), "nonzero": u(r, "line_nonzero"),
            "yield": f(r, "yield_ordered_mean"), "pred": f(r, "yield_pred"), "targets": u(r, "targets")});
        if line == "twisted" {
            let want = if u(r, "Tr_a") == 0 {
                u(r, "line_nonzero")
            } else {
                0
            };
            if u(r, "lifting") != want {
                hb_fail.push(json!([key, ci, "lifting", u(r, "lifting"), want]));
                hbp_fail.push(json!([key, ci, "lifting", u(r, "lifting"), want]));
            }
        }
        if u(r, "fb_points") > 0 {
            let se = (f(r, "yield_pred").max(1e-12) / f(r, "targets")).sqrt();
            if (f(r, "yield_ordered_mean") - f(r, "yield_pred")).abs() > 4.0 * se {
                ha_fail.push(json!([
                    key,
                    ci,
                    line,
                    f(r, "yield_ordered_mean"),
                    f(r, "yield_pred")
                ]));
            }
        }
    }
    for cell in cells.values_mut() {
        let m = cell["m"].as_i64().unwrap() as i32;
        let mut ratios = vec![];
        let mut have = 0;
        let ncur = cell["curves"].as_object().unwrap().len();
        for cur in cell["curves"].as_object().unwrap().values() {
            if cur["u_count"].as_u64().unwrap() > 0 {
                have += 1;
            }
            let (tw, sd) = (&cur["twisted"], &cur["standard"]);
            if cur["Tr_a"] == 0 && tw.is_object() && sd.is_object() && f(sd, "yield") > 0.0 {
                ratios.push(json!({"measured": f(tw, "yield") / f(sd, "yield"),
                    "predicted": f(tw, "pred") / f(sd, "pred"),
                    "lift_ratio_pow_m": (f(tw, "lifting") / f(sd, "lifting")).powi(m)}));
            }
        }
        cell["yield_ratio_tr0"] = json!(ratios);
        cell["u_exists_fraction"] = json!(have as f64 / ncur as f64);
    }
    out.insert("lines".into(), json!(cells));
    out.insert(
        "H_A".into(),
        json!({"rule": "Poisson SE (as registered)", "failures": ha_fail,
        "verdict": if ha_fail.is_empty() { "pass" } else { "FAIL" }}),
    );
    out.insert(
        "H_B".into(),
        json!({"rule": "n=3 count in {0, 2(q-1)} (as registered; wrong, Amendment 3)",
        "failures": hb_fail, "verdict": if hb_fail.is_empty() { "pass" } else { "FAIL" }}),
    );

    // ---- H-A′ (Amendment 1) and H-A″ (Amendment 3): empirical variance
    let score_amended = |path: &Path, use_excl: bool| -> Value {
        let a: Vec<Value> = read_jsonl(path)
            .into_iter()
            .filter(|r| kind(r) == "line")
            .collect();
        let orig: BTreeMap<String, f64> = lines
            .iter()
            .filter(|r| !r["u"].is_null())
            .map(|r| {
                (
                    format!("{}_{}_{}_{}", r["N"], r["d"], r["curve"], s(r, "line")),
                    f(r, "yield_ordered_mean"),
                )
            })
            .collect();
        let mut reproduced = true;
        let (mut fails, mut zs) = (vec![], vec![]);
        let mut pooled: BTreeMap<String, (f64, f64, f64)> = BTreeMap::new();
        for r in &a {
            if r["u"].is_null() {
                continue;
            }
            let k = format!("{}_{}_{}_{}", r["N"], r["d"], r["curve"], s(r, "line"));
            if orig.get(&k) != Some(&f(r, "yield_ordered_mean")) {
                reproduced = false;
            }
            if u(r, "fb_points") == 0 {
                continue;
            }
            let t = f(r, "targets");
            let mean_ = f(r, "yield_ordered_mean");
            let var = (f(r, "yield_ordered_m2") - mean_ * mean_).max(0.0);
            let pred = if use_excl {
                f(r, "yield_pred_excl")
            } else {
                f(r, "yield_pred")
            };
            let se = if var > 0.0 { (var / t).sqrt() } else { 1e-12 };
            let z = (mean_ - pred) / se;
            zs.push(z);
            if z.abs() > 4.0 {
                fails.push(json!([
                    r["N"],
                    r["d"],
                    r["curve"],
                    s(r, "line"),
                    (z * 100.0).round() / 100.0
                ]));
            }
            let e = pooled
                .entry(format!("N{}_d{}", r["N"], r["d"]))
                .or_insert((0.0, 0.0, 0.0));
            e.0 += mean_ * t;
            e.1 += pred * t;
            e.2 += var * t;
        }
        let mut pool_fail = vec![];
        let mut pj = Map::new();
        for (k, (me, pr, va)) in &pooled {
            let z = (me - pr) / va.sqrt();
            if z.abs() > 4.0 {
                pool_fail.push(json!([k, z]));
            }
            pj.insert(k.clone(), json!({"ratio": me / pr, "z": z}));
        }
        let ok = reproduced && fails.is_empty() && pool_fail.is_empty();
        json!({"reproduces_confirmatory_means": reproduced, "row_failures": fails, "pooled": pj,
            "pooled_failures": pool_fail, "z_mean": mean(&zs), "z_sd": pstdev(&zs), "rows": zs.len(),
            "verdict": if ok { "pass" } else { "FAIL" }})
    };
    out.insert(
        "H_A_prime".into(),
        score_amended(&run.join("lines_amend1/rows.jsonl"), false),
    );
    let a3 = run.join("lines_amend3/rows.jsonl");
    out.insert("H_A_dprime".into(), score_amended(&a3, true));
    // H-B′ also covers the amend3 line rows (same curves) and the census
    let census = read_jsonl(&run.join("lines_amend3/u_census.jsonl"));
    let cviol: u64 = census.iter().map(|c| u(c, "violations")).sum();
    out.insert(
        "H_B_prime".into(),
        json!({"rule": "n=2 {0,q-1}; n=3 {0,q-1,2(q-1),q^2-1}; n=4 multiple of q-1",
        "line_failures": hbp_fail, "census": census, "census_violations": cviol,
        "verdict": if hbp_fail.is_empty() && cviol == 0 { "pass" } else { "FAIL" }}),
    );

    // ---- m = 2 instrument check (post-hoc, not a decision rule)
    let ym: Vec<Value> = ym2
        .iter()
        .map(|r| {
            let order = u(r, "order") as f64;
            let fix = |a: &Value| {
                let k = f(a, "k");
                let p = 2.0 * k * k / (order - 2.0);
                json!({"k": a["k"], "mean": a["mean"], "pred": a["pred"], "pred_excl_O": p, "ratio_excl_O": f(a, "mean") / p})
            };
            json!({"n": r["n"], "label": r["label"], "order": r["order"], "l": r["l"], "targets": r["targets"],
                "dense": fix(&r["dense"]), "random": r["random"].as_array().unwrap().iter().map(fix).collect::<Vec<_>>()})
        })
        .collect();
    out.insert("yield_m2".into(), json!(ym));

    // ---- kmax, counts, H-C
    let mut byn: BTreeMap<u64, Vec<&Value>> = BTreeMap::new();
    for r in &kmax {
        byn.entry(u(r, "n")).or_default().push(r);
    }
    let mut km = Map::new();
    let mut hc = Map::new();
    for (n, rs) in &byn {
        let mut o = Map::new();
        for arm in ["curve", "null", "planted"] {
            let a: Vec<&&Value> = rs.iter().filter(|r| s(r, "arm") == arm).collect();
            let mut aj = Map::new();
            aj.insert(
                "k_exact".into(),
                json!(a.iter().map(|r| r["k_exact"].clone()).collect::<Vec<_>>()),
            );
            aj.insert(
                "k_budget".into(),
                json!(a.iter().map(|r| u(r, "k_budget")).collect::<Vec<_>>()),
            );
            if arm == "curve" {
                aj.insert(
                    "struct_k".into(),
                    json!(a.iter().map(|r| u(r, "struct_k")).collect::<Vec<_>>()),
                );
            }
            if arm == "planted" {
                let pk = u(a[0], "planted_k");
                aj.insert("planted_k".into(), json!(pk));
                let bd =
                    a.iter().filter(|r| u(r, "k_budget") >= pk).count() as f64 / a.len() as f64;
                aj.insert("budget_detect".into(), json!(bd));
                if !a[0]["k_exact"].is_null() {
                    let ed =
                        a.iter().filter(|r| u(r, "k_exact") >= pk).count() as f64 / a.len() as f64;
                    aj.insert("exact_detect".into(), json!(ed));
                }
            }
            aj.insert(
                "rho_star_first".into(),
                json!(a
                    .iter()
                    .map(|r| r["rho_star"]
                        .as_array()
                        .and_then(|v| v.first())
                        .and_then(|x| x.as_str())
                        .map(|x| x.parse::<f64>().unwrap()))
                    .collect::<Vec<_>>()),
            );
            if a[0]["subspace_counts"]
                .as_array()
                .is_some_and(|v| !v.is_empty())
            {
                let mut cr = Map::new();
                for k in 2u32..6 {
                    let v: Vec<f64> = a
                        .iter()
                        .map(|r| {
                            let c = r["subspace_counts"].as_array().unwrap();
                            if c.len() > k as usize {
                                c[k as usize].as_f64().unwrap()
                                    / expect(*n as u32, k, c[1].as_u64().unwrap())
                            } else {
                                0.0
                            }
                        })
                        .collect();
                    cr.insert(k.to_string(), json!(v));
                }
                aj.insert("count_ratio".into(), Value::Object(cr));
            }
            o.insert(arm.into(), Value::Object(aj));
        }
        if let (Some(x), Some(y)) = (o["curve"].get("count_ratio"), o["null"].get("count_ratio")) {
            let xs: Vec<f64> = x["4"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect();
            let ys: Vec<f64> = y["4"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect();
            let (uu, pp, pn) = mann_whitney_less(&xs, &ys, 50_000);
            hc.insert(
                n.to_string(),
                json!({"n_curves": xs.len(), "n_nulls": ys.len(),
                "median_curve": median(&xs), "median_null": median(&ys), "mean_curve": mean(&xs),
                "mean_null": mean(&ys), "sd_curve": pstdev(&xs), "sd_null": pstdev(&ys),
                "U": uu, "p_perm": pp, "p_norm": pn}),
            );
        }
        km.insert(n.to_string(), Value::Object(o));
    }
    let v11 = hc.get("11").map(|h| {
        if f(h, "p_perm") < 0.01 {
            "repulsion confirmed"
        } else {
            "not confirmed"
        }
    });
    out.insert("kmax".into(), Value::Object(km));
    out.insert("H_C".into(), Value::Object(hc));
    out.insert("H_C_verdict_N11".into(), json!(v11));

    // ---- c2pnb (native) and the cost delta
    let c2: Value = serde_json::from_str(
        &std::fs::read_to_string(run.join("c2pnb_native.json")).expect("c2pnb_native.json"),
    )
    .unwrap();
    let hist: Value =
        serde_json::from_str(&std::fs::read_to_string(run.join("c2pnb_lines.json")).unwrap())
            .unwrap();
    let mut c2v = vec![];
    for (r, h) in c2.as_array().unwrap().iter().zip(hist.as_array().unwrap()) {
        let same = r["curve"] == h["curve"]
            && r["Tr_a"] == h["Tr_a"]
            && r["a_in_F16"] == h["a_in_F16"]
            && r["b_in_F16"] == h["b_in_F16"]
            && r["u_kernel_T"] == h["u_kernel_T"]
            && r["u_all_lift"] == h["u_all_lift"];
        let ns = u(r, "samples");
        let verdict = if u(r, "Tr_a") == 0
            && u(r, "u_all_lift") > 0
            && r["sampled_lift_twisted"].as_u64() == Some(ns)
        {
            "applicable"
        } else if u(r, "Tr_a") == 1 && r["sampled_lift_twisted"].as_u64() == Some(0) {
            "inapplicable (trace obstruction: line lies on the twist)"
        } else {
            "UNEXPECTED"
        };
        let mut rr = r.clone();
        rr["matches_historical_python"] = json!(same);
        rr["verdict"] = json!(verdict);
        c2v.push(rr);
    }
    out.insert("c2pnb_lines".into(), json!(c2v));
    // closed form of the upstream 2LP optimum: T* ∝ sqrt(N^3 / p), so the
    // margin change is ½(log2(p'/p) − 3 log2(N'/N)) with p' = 16 p, N' = 2N
    let delta = 0.5 * (16f64.log2() - 3.0 * 2f64.log2());
    out.insert(
        "c2pnb_delta_native".into(),
        json!({"metric": "2LP time (closed form)", "delta_bits": delta,
        "label": "model output, not a measurement; c_trial assumed unchanged"}),
    );
    let dh: Value =
        serde_json::from_str(&std::fs::read_to_string(run.join("c2pnb_delta.json")).unwrap())
            .unwrap();
    out.insert(
        "c2pnb_delta_historical_python".into(),
        json!(dh
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| {
                let m = s(r, "metric");
                ["time", "TxM-a-p0", "mesh-a", "MM1-a"].contains(&m)
            })
            .collect::<Vec<_>>()),
    );

    std::fs::write(
        run.join("summary.json"),
        serde_json::to_string_pretty(&Value::Object(out.clone())).unwrap(),
    )
    .unwrap();
    for k in ["H_A", "H_B", "H_A_prime", "H_A_dprime", "H_B_prime"] {
        let h = &out[k];
        println!(
            "{k}: {} {}",
            h["verdict"],
            match k {
                "H_A_prime" | "H_A_dprime" => format!(
                    "row_failures {} pooled_failures {} z {:.2}±{:.2} reproduced {}",
                    h["row_failures"],
                    h["pooled_failures"],
                    f(h, "z_mean"),
                    f(h, "z_sd"),
                    h["reproduces_confirmatory_means"]
                ),
                "H_B_prime" => format!(
                    "line_failures {} census_violations {}",
                    h["line_failures"], h["census_violations"]
                ),
                _ => format!("{} failures", h["failures"].as_array().unwrap().len()),
            }
        );
    }
    for (n, h) in out["H_C"].as_object().unwrap() {
        println!(
            "H_C n={n}: curves {:.3}±{:.3} nulls {:.3}±{:.3} p_perm {:.2e} p_norm {:.2e}",
            f(h, "mean_curve"),
            f(h, "sd_curve"),
            f(h, "mean_null"),
            f(h, "sd_null"),
            f(h, "p_perm"),
            f(h, "p_norm")
        );
    }
    for r in out["c2pnb_lines"].as_array().unwrap() {
        println!(
            "{} Tr_a={} u={} lift {}/{} std {} | matches_python {} | {}",
            r["curve"],
            r["Tr_a"],
            r["u_all_lift"],
            r["sampled_lift_twisted"],
            r["samples"],
            r["sampled_lift_standard"],
            r["matches_historical_python"],
            r["verdict"]
        );
    }
    println!("c2pnb 2LP time delta (closed form): {delta:+.2} bits");
}
