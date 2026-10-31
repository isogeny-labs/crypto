//! **E32 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.34/§9.36): tables and
//! the frozen crossover fits of the handover ladder.**
//!
//! Reads `experiments/36_handover_ladder/{ic,walk}_p*s*.json` (and E27's records
//! for the identity check G2.114) and writes `summary.json`, `fit.json` and
//! `tables.md` beside them.  Uses only `std` and `serde_json` (no library
//! code), so it can be built outside the shared target directory.
//!
//! Distance at weight `g` (§5a, one GAE `= 97`): `D(g) = (S + 97 g E) /
//! (rho + 97 g steps)`.  Fits on `x = log₂ n − 32`, `L = log₂ D`:
//! **M1** OLS on `p ≥ 269`; **M1-all** on every curve; **M2** slope fixed at
//! `−1/4` on `p ≥ 269`.  Crossover order `log₂ n* = 32 − α/β` (`∞` for
//! `β ≥ 0`).  Intervals: cluster bootstrap over curves stratified by size,
//! `10,000` replicates, SplitMix64 seeded with `32`, 95 % percentiles.
//!
//! ```text
//! e32_tables experiments/36_handover_ladder experiments/32_walk_karatsuba
//! ```

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::env;
use std::fs;

const GAE: f64 = 97.0;
const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const X0: f64 = 32.0;
const B: usize = 10_000;

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, k: usize) -> usize {
        (self.next() % k as u64) as usize
    }
}

fn read(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"))).unwrap()
}

fn flat(v: &Value, pre: &str, out: &mut BTreeMap<String, Value>) {
    match v {
        Value::Object(m) => {
            for (k, x) in m {
                let key = if pre.is_empty() { k.clone() } else { format!("{pre}.{k}") };
                flat(x, &key, out);
            }
        }
        Value::Array(a) => {
            for (i, x) in a.iter().enumerate() {
                flat(x, &format!("{pre}[{i}]"), out);
            }
        }
        _ => {
            out.insert(pre.to_string(), v.clone());
        }
    }
}

fn ignored(k: &str) -> bool {
    let head = k.split(['.', '[']).next().unwrap_or("");
    matches!(head, "wall_ms" | "solve_ms" | "experiment" | "karatsuba")
}

fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("field {k}"))
}

#[derive(Clone)]
struct Row {
    p: u64,
    seed: u64,
    log2n: f64,
    d: [f64; 3],
    v: Value,
}

fn ols(pts: &[(f64, f64)]) -> (f64, f64, f64) {
    let n = pts.len() as f64;
    let mx = pts.iter().map(|p| p.0).sum::<f64>() / n;
    let my = pts.iter().map(|p| p.1).sum::<f64>() / n;
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx).powi(2)).sum();
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let s2: f64 = pts.iter().map(|p| (p.1 - a - b * p.0).powi(2)).sum::<f64>() / (n - 2.0).max(1.0);
    (a, b, (s2 / sxx).sqrt())
}

#[derive(Clone, Copy, PartialEq)]
enum Model {
    M1,
    M2,
}

/// (alpha, beta, beta_se, log2 n*)
fn fit(model: Model, pts: &[(f64, f64)]) -> Option<(f64, f64, f64, f64)> {
    match model {
        Model::M1 => {
            let first = pts.first()?.0;
            if pts.iter().all(|p| p.0 == first) {
                return None;
            }
            let (a, b, se) = ols(pts);
            Some((a, b, se, if b < 0.0 { X0 - a / b } else { f64::INFINITY }))
        }
        Model::M2 => {
            let a = pts.iter().map(|p| p.1 + p.0 / 4.0).sum::<f64>() / pts.len() as f64;
            Some((a, -0.25, 0.0, X0 + 4.0 * a))
        }
    }
}

fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    let k = (v.len() - 1) as f64 * q;
    let (lo, hi) = (k.floor() as usize, k.ceil() as usize);
    if v[hi].is_infinite() {
        return if v[lo].is_infinite() { f64::INFINITY } else if lo == hi { v[lo] } else { f64::INFINITY };
    }
    v[lo] + (v[hi] - v[lo]) * (k - lo as f64)
}

fn num(x: f64) -> Value {
    if x.is_finite() {
        json!(x)
    } else {
        json!("inf")
    }
}

fn bootstrap(rows: &[Row], wi: usize, pmin: u64, model: Model, rng: &mut SplitMix) -> Value {
    let mut strata: BTreeMap<u64, Vec<(f64, f64)>> = BTreeMap::new();
    for r in rows.iter().filter(|r| r.p >= pmin) {
        strata.entry(r.p).or_default().push((r.log2n - X0, r.d[wi].log2()));
    }
    let pts: Vec<(f64, f64)> = strata.values().flatten().copied().collect();
    if pts.len() < 2 {
        return json!(null);
    }
    let Some((a, b, se, ns)) = fit(model, &pts) else {
        return json!(null);
    };
    let (mut ra, mut rb, mut rn) = (vec![], vec![], vec![]);
    let mut nonneg = 0usize;
    for _ in 0..B {
        let mut sample = Vec::with_capacity(pts.len());
        for v in strata.values() {
            for _ in 0..v.len() {
                sample.push(v[rng.below(v.len())]);
            }
        }
        if let Some((a2, b2, _, n2)) = fit(model, &sample) {
            ra.push(a2);
            rb.push(b2);
            rn.push(n2);
            if b2 >= 0.0 {
                nonneg += 1;
            }
        }
    }
    let reps = rb.len();
    let mut out = json!({
        "alpha": a, "beta": b, "beta_se": se, "log2_nstar": num(ns),
        "ci95": {
            "alpha": [pct(&mut ra, 0.025), pct(&mut ra, 0.975)],
            "beta": [pct(&mut rb, 0.025), pct(&mut rb, 0.975)],
            "log2_nstar": [num(pct(&mut rn, 0.025)), num(pct(&mut rn, 0.975))],
        },
        "replicates": reps, "frac_beta_nonneg": nonneg as f64 / reps.max(1) as f64,
        "n_curves": pts.len(), "sizes": strata.keys().collect::<Vec<_>>(),
    });
    if model == Model::M1 {
        out["beta_ols_ci95"] = json!([b - 1.96 * se, b + 1.96 * se]);
    }
    out
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn fmt_inf(v: &Value) -> String {
    match v.as_f64() {
        Some(x) => format!("{x:.2}"),
        None => "∞".into(),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let dir = args.first().cloned().unwrap_or("experiments/36_handover_ladder".into());
    let e27 = args.get(1).cloned().unwrap_or("experiments/32_walk_karatsuba".into());

    // G2.114: identity against E27.
    let summ = read(&format!("{e27}/summary.json"));
    let mut ident = vec![];
    for p in [101u64, 149, 197, 269, 397] {
        for s in [1u64, 2] {
            let mut item = json!({"p": p, "seed": s});
            let icf = format!("{dir}/ic_p{p}s{s}.json");
            if fs::metadata(&icf).is_ok() {
                let mine = read(&icf);
                let refs = read(&format!("{e27}/e2e_chain_karatsuba_p{p}s{s}.json"));
                let r = refs.as_array().unwrap().iter().find(|r| r["arm"] == "B").unwrap();
                let (mut a, mut b) = (BTreeMap::new(), BTreeMap::new());
                flat(&mine, "", &mut a);
                flat(r, "", &mut b);
                a.retain(|k, _| !ignored(k));
                b.retain(|k, _| !ignored(k));
                let keys: std::collections::BTreeSet<&String> = a.keys().chain(b.keys()).collect();
                let diff: Vec<&String> = keys.into_iter().filter(|k| a.get(*k) != b.get(*k)).collect();
                item["ic_fields_compared"] = json!(b.len());
                item["ic_diff"] = json!(diff);
            }
            let wf = format!("{dir}/walk_p{p}s{s}.json");
            if fs::metadata(&wf).is_ok() {
                let w = read(&wf);
                let r = summ["rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["p"] == p && r["seed"] == s && r["arm"] == "B")
                    .unwrap();
                item["walk_rho_equal"] = json!(w["rho_iii_mean_fp_muls"] == r["rho_e27"]);
                item["walk_steps_equal"] = json!(w["mean_steps"] == r["walk_steps"]);
            }
            ident.push(item);
        }
    }

    // Curves.
    let mut rows: Vec<Row> = vec![];
    let mut files: Vec<String> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.starts_with("ic_p") && n.ends_with(".json"))
        .collect();
    files.sort();
    for name in files {
        let key = &name[3..name.len() - 5];
        let wf = format!("{dir}/walk_{key}.json");
        if fs::metadata(&wf).is_err() {
            continue;
        }
        let ic = read(&format!("{dir}/{name}"));
        let w = read(&wf);
        let (s, e) = (f(&ic, "total_muls"), f(&ic, "macaulay_entries"));
        let (rho, steps) = (f(&w, "rho_iii_mean_fp_muls"), f(&w, "mean_steps"));
        let n = f(&ic, "n");
        let rel = ic["folded"]["relations"].as_f64().unwrap().max(1.0);
        let res = f(&ic, "residuals");
        let d = WEIGHTS.map(|g| (s + GAE * g * e) / (rho + GAE * g * steps));
        let v = json!({
            "p": ic["p"], "seed": ic["seed"], "n": ic["n"], "log2n": n.log2(), "h": ic["cofactor"],
            "cols": ic["folded"]["columns"], "relations": ic["folded"]["relations"], "residuals": res,
            "y": res / rel, "unsolved": ic["unsolved"], "C_median": ic["call"]["median"],
            "C_mean": ic["call"]["mean"], "entries_per_residual": e / res.max(1.0), "S": s, "E": e,
            "solve_share": (f(&ic, "weil_muls") + f(&ic, "solve_muls")) / s,
            "la_share": f(&ic, "la_fp_muls") / s, "rho": rho, "steps": steps,
            "steps_per_sqrt_n": steps / n.sqrt(), "muls_per_step": rho / steps,
            "ic_correct": ic["folded"]["correct"], "rows_readd_failures": ic["rows_readd_failures"],
            "off_base": ic["off_base"], "check_failures": ic["check_failures"],
            "walks_correct": w["all_correct"], "ic_wall_s": f(&ic, "wall_ms") / 1e3, "walk_wall_s": w["wall_s"],
            "D": {"0": d[0], "0.1": d[1], "1": d[2]},
        });
        rows.push(Row { p: ic["p"].as_u64().unwrap(), seed: ic["seed"].as_u64().unwrap(), log2n: n.log2(), d, v });
    }
    rows.sort_by_key(|r| (r.p, r.seed));

    let mut rng = SplitMix(32);
    let mut fits = Map::new();
    for (wi, g) in WEIGHTS.iter().enumerate() {
        fits.insert(
            g.to_string(),
            json!({
                "M1": bootstrap(&rows, wi, 269, Model::M1, &mut rng),
                "M1_all": bootstrap(&rows, wi, 0, Model::M1, &mut rng),
                "M2": bootstrap(&rows, wi, 269, Model::M2, &mut rng),
            }),
        );
    }
    let ypts: Vec<(f64, f64)> =
        rows.iter().map(|r| ((r.p as f64).log2(), f(&r.v, "y").log2())).collect();
    let (_, by, sey) = ols(&ypts);

    let mut sizes: Vec<u64> = rows.iter().map(|r| r.p).collect();
    sizes.dedup();
    let mut per_size = vec![];
    for &p in &sizes {
        let rs: Vec<&Row> = rows.iter().filter(|r| r.p == p).collect();
        let mut o = json!({"p": p, "curves": rs.len(),
            "log2n": rs.iter().map(|r| r.log2n).sum::<f64>() / rs.len() as f64,
            "y_median": median(&mut rs.iter().map(|r| f(&r.v, "y")).collect::<Vec<_>>())});
        for (wi, g) in ["0", "0.1", "1"].iter().enumerate() {
            let mut vals: Vec<f64> = rs.iter().map(|r| r.d[wi]).collect();
            let gm = (vals.iter().map(|v| v.log2()).sum::<f64>() / vals.len() as f64).exp2();
            let (mn, mx) = (vals.iter().cloned().fold(f64::MAX, f64::min), vals.iter().cloned().fold(0.0, f64::max));
            o[format!("D_{g}")] = json!({"median": median(&mut vals), "min": mn, "max": mx, "geomean": gm});
        }
        per_size.push(o);
    }
    let reached = rows.iter().map(|r| r.log2n).fold(0.0, f64::max);
    let below = rows.iter().filter(|r| r.d.iter().any(|&x| x <= 1.0)).count();

    // LA sensitivity: share extrapolated to the M2 n* at 0 GAE (LA ∝ p², the rest ∝ p).
    let la_ext = (|| {
        let pb = *sizes.last()?;
        let rs: Vec<&Row> = rows.iter().filter(|r| r.p == pb).collect();
        let share = rs.iter().map(|r| f(&r.v, "la_share")).sum::<f64>() / rs.len() as f64;
        let ns = fits["0"]["M2"]["log2_nstar"].as_f64()?;
        let pstar = (16.0 * ns.exp2()).powf(0.25);
        let ratio = share / (1.0 - share) * pstar / pb as f64;
        Some(json!({"p_from": pb, "share_from": share, "p_star": pstar, "share_at_nstar": ratio / (1.0 + ratio)}))
    })();

    let summary = json!({
        "experiment": "e32", "note": "§9.34/§9.36", "gae": GAE, "weights": WEIGHTS,
        "identity": ident, "curves": rows.iter().map(|r| r.v.clone()).collect::<Vec<_>>(),
        "per_size": per_size, "fits": fits,
        "yield_exponent_in_p": {"slope": by, "se": sey, "ci95": [by - 1.96 * sey, by + 1.96 * sey]},
        "reached_log2n": reached, "rows_at_or_below_rho_any_weight": below, "la_extrapolation": la_ext,
        "bootstrap": {"replicates": B, "rng": "SplitMix64", "seed": 32, "strata": "size", "unit": "curve"},
    });
    fs::write(format!("{dir}/summary.json"), serde_json::to_string_pretty(&summary).unwrap()).unwrap();
    fs::write(format!("{dir}/fit.json"), serde_json::to_string_pretty(&summary["fits"]).unwrap()).unwrap();

    let mut t = String::from("# E32 tables (examples/e32_tables.rs)\n\n## Per curve\n\n");
    t += "| p | seed | log₂ n | cols | y | C median | E / residual | solve % | LA % | S | rho (iii) | steps/√n | mul/step | D(0) | D(0.1) | D(1) |\n";
    t += "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    for r in &rows {
        let v = &r.v;
        t += &format!(
            "| {} | {} | {:.2} | {} | {:.0} | {:.4e} | {:.4e} | {:.2} | {:.4} | {:.3e} | {:.3e} | {:.3} | {:.1} | {:.3e} | {:.3e} | {:.3e} |\n",
            r.p, r.seed, r.log2n, v["cols"], f(v, "y"), f(v, "C_median"), f(v, "entries_per_residual"),
            100.0 * f(v, "solve_share"), 100.0 * f(v, "la_share"), f(v, "S"), f(v, "rho"),
            f(v, "steps_per_sqrt_n"), f(v, "muls_per_step"), r.d[0], r.d[1], r.d[2]
        );
    }
    t += "\n## Per size (median [min, max] over curves)\n\n| p | curves | log₂ n | y median | D(0) | D(0.1) | D(1) |\n|--:|--:|--:|--:|:--|:--|:--|\n";
    for o in &per_size {
        let cell = |g: &str| {
            let d = &o[format!("D_{g}")];
            format!("{:.3e} [{:.2e}, {:.2e}]", f(d, "median"), f(d, "min"), f(d, "max"))
        };
        t += &format!(
            "| {} | {} | {:.2} | {:.0} | {} | {} | {} |\n",
            o["p"], o["curves"], f(o, "log2n"), f(o, "y_median"), cell("0"), cell("0.1"), cell("1")
        );
    }
    t += "\n## Fits: log₂ D = α + β (log₂ n − 32); crossover log₂ n* = 32 − α/β\n\n| GAE | model | curves | β [95 %] | log₂ n* [95 %] | P(β ≥ 0) |\n|--:|:--|--:|:--|:--|--:|\n";
    for g in ["0", "0.1", "1"] {
        for m in ["M1", "M1_all", "M2"] {
            let x = &fits[g][m];
            if x.is_null() {
                continue;
            }
            t += &format!(
                "| {g} | {m} | {} | {:.4} [{:.4}, {:.4}] | {} [{}, {}] | {:.4} |\n",
                x["n_curves"], f(x, "beta"), x["ci95"]["beta"][0].as_f64().unwrap(),
                x["ci95"]["beta"][1].as_f64().unwrap(), fmt_inf(&x["log2_nstar"]),
                fmt_inf(&x["ci95"]["log2_nstar"][0]), fmt_inf(&x["ci95"]["log2_nstar"][1]),
                f(x, "frac_beta_nonneg")
            );
        }
    }
    t += &format!(
        "\nYield exponent in p (all curves): {by:.3} (OLS 95 % [{:.3}, {:.3}]).  Largest log₂ n reached: {reached:.2}; rows at or below rho at any weight: {below}.\n\n## Identity (G2.114)\n\n",
        by - 1.96 * sey, by + 1.96 * sey
    );
    for it in &ident {
        t += &format!(
            "- p = {}, seed {}: {} ic fields compared, differing {}; walk rho equal {}, steps equal {}\n",
            it["p"], it["seed"], it["ic_fields_compared"], it["ic_diff"], it["walk_rho_equal"], it["walk_steps_equal"]
        );
    }
    fs::write(format!("{dir}/tables.md"), &t).unwrap();
    print!("{t}");
}
