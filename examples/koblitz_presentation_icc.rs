//! **Curve-vs-presentation variance split** for `koblitz_presentation_sweep`
//! output: the Rust port of `research/notes/koblitz-isogeny/presentation_icc.py`
//! (same model, estimator, metrics and percentile CI; the bootstrap draws
//! come from a different RNG, so CI endpoints differ in the last digits).
//!
//! Two-way crossed random effects without replication,
//! `Y[E,V] = μ + u_E + w_V + e`, by ANOVA mean squares on the complete
//! curves × subspaces grid; `ICC_E = σ²_u / (σ²_u + σ²_w + σ²_e)`; 95 % CI
//! by 2000 bootstrap resamples of curves.
//!
//! Metrics (per cell): yield per `|F|^m`; `ln µs` per call; work per call
//! (Gröbner reductions for m = 2, `S₄` roots for the m = 3 e₃ engine);
//! `ln` projected solve `(unknowns + 4) / yield × time per call`.
//!
//! Run: `cargo run --release --example koblitz_presentation_icc <pres.jsonl>...`

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_isogeny_cost::field_for;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

type Metric = (&'static str, fn(&Value) -> f64);

fn f(r: &Value, k: &str) -> f64 {
    r[k].as_f64().unwrap_or(0.0)
}

fn m_of(r: &Value) -> i32 {
    r["m"].as_i64().unwrap_or(2) as i32
}

const METRICS: [Metric; 4] = [
    ("yield_per_Fm", |r| {
        (f(r, "relations") / f(r, "probes")) / f(r, "factor_base_points").powi(m_of(r))
    }),
    ("log_us_per_call", |r| {
        (f(r, "groebner_ns") / f(r, "groebner_calls") / 1e3).ln()
    }),
    ("work_per_call", |r| {
        let work = if m_of(r) == 3 {
            f(r, "sym_roots")
        } else {
            f(r, "reductions")
        };
        work / f(r, "groebner_calls")
    }),
    ("log_projected_solve", |r| {
        let y = (f(r, "relations") / f(r, "probes")).max(1e-9);
        ((f(r, "unknowns") + 4.0) / y * f(r, "groebner_ns") / f(r, "groebner_calls")).ln()
    }),
];

/// Post-hoc diagnostics (`KOBLITZ_ICC_DIAG=1`), not pre-registered: the
/// factor-base size, and the m = 3 work split into pins per call (`x₃`
/// values tried, PinX3) and roots per pin.
const DIAGNOSTICS: [Metric; 4] = [
    ("diag_factor_base_points", |r| f(r, "factor_base_points")),
    ("diag_pins_per_call", |r| {
        f(r, "sym_e3_visited") / f(r, "groebner_calls")
    }),
    ("diag_roots_per_pin", |r| {
        f(r, "sym_roots") / f(r, "sym_e3_visited").max(1.0)
    }),
    ("diag_work_per_call_per_F", |r| {
        f(r, "sym_roots") / f(r, "groebner_calls") / f(r, "factor_base_points")
    }),
];

/// `(ICC_E, σ²_u, σ²_w, σ²_e)` of a curves × subspaces grid.
fn icc(grid: &[Vec<f64>]) -> (f64, f64, f64, f64) {
    let (a, b) = (grid.len() as f64, grid[0].len() as f64);
    let gm = grid.iter().flatten().sum::<f64>() / (a * b);
    let rm: Vec<f64> = grid.iter().map(|r| r.iter().sum::<f64>() / b).collect();
    let cm: Vec<f64> = (0..grid[0].len())
        .map(|j| grid.iter().map(|r| r[j]).sum::<f64>() / a)
        .collect();
    let ss_r = b * rm.iter().map(|x| (x - gm).powi(2)).sum::<f64>();
    let ss_c = a * cm.iter().map(|x| (x - gm).powi(2)).sum::<f64>();
    let ss_t: f64 = grid.iter().flatten().map(|x| (x - gm).powi(2)).sum();
    let (ms_r, ms_c) = (ss_r / (a - 1.0), ss_c / (b - 1.0));
    let ms_e = (ss_t - ss_r - ss_c) / ((a - 1.0) * (b - 1.0));
    let su = ((ms_r - ms_e) / b).max(0.0);
    let sw = ((ms_c - ms_e) / a).max(0.0);
    let tot = su + sw + ms_e;
    (if tot > 0.0 { su / tot } else { 0.0 }, su, sw, ms_e)
}

/// Absolute trace `Tr(a) = Σ a^{2^i}` of a one-word element of `F_{2^n}`.
fn abs_trace(a: u64, n: u32) -> u64 {
    let irr = field_for(n).expect("field");
    let bits: Vec<u32> = (0..n).filter(|i| (a >> i) & 1 == 1).collect();
    let mut x = F2mElement::from_bit_positions(&bits, n);
    let mut t = F2mElement::zero(n);
    for _ in 0..n {
        t = t.add(&x);
        x = x.square(&irr);
    }
    t.raw_bits().first().copied().unwrap_or(0)
}

/// Post-hoc: curve-mean `factor_base_points` grouped by volcano depth and by
/// `Tr(a₆)`, with a curve-bootstrap 95 % interval for the group difference.
fn f_by_group(n: u32, curves: &[u64], by: &BTreeMap<u64, BTreeMap<u64, &Value>>) -> Value {
    let mean_f = |c: &u64| {
        let v: Vec<f64> = by[c].values().map(|r| f(r, "factor_base_points")).collect();
        v.iter().sum::<f64>() / v.len() as f64
    };
    let keys: [(&str, Box<dyn Fn(&u64) -> u64>); 2] = [
        (
            "depth",
            Box::new(|c| by[c].values().next().unwrap()["depth"].as_u64().unwrap()),
        ),
        ("trace_a6", Box::new(|c| abs_trace(*c, n))),
    ];
    let mut out = serde_json::Map::new();
    for (name, key) in keys {
        let mut groups: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
        for c in curves {
            groups.entry(key(c)).or_default().push(mean_f(c));
        }
        let summary: Vec<Value> = groups
            .iter()
            .map(|(k, v)| {
                let m = v.iter().sum::<f64>() / v.len() as f64;
                let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>()
                    / (v.len().max(2) - 1) as f64)
                    .sqrt();
                json!({"key": k, "curves": v.len(), "mean_F": m, "sd_F": sd})
            })
            .collect();
        let diff = if groups.len() == 2 {
            let g: Vec<&Vec<f64>> = groups.values().collect();
            let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
            let point = mean(g[1]) - mean(g[0]);
            let mut rng = StdRng::seed_from_u64(11);
            let mut boots: Vec<f64> = (0..2000)
                .map(|_| {
                    let rs = |v: &Vec<f64>, rng: &mut StdRng| -> Vec<f64> {
                        (0..v.len()).map(|_| v[rng.gen_range(0..v.len())]).collect()
                    };
                    let (a, b) = (rs(g[0], &mut rng), rs(g[1], &mut rng));
                    mean(&b) - mean(&a)
                })
                .collect();
            boots.sort_by(f64::total_cmp);
            json!({"key1_minus_key0": point, "ci95": [boots[50], boots[1949]]})
        } else {
            Value::Null
        };
        out.insert(name.into(), json!({"groups": summary, "difference": diff}));
    }
    Value::Object(out)
}

/// Percentile 95 % interval of `ICC_E` from `boots` resamples of the curves.
fn boot_ci(grid: &[Vec<f64>], boots: usize, rng: &mut StdRng) -> (f64, f64) {
    let mut v: Vec<f64> = (0..boots)
        .map(|_| {
            let g: Vec<Vec<f64>> = (0..grid.len())
                .map(|_| grid[rng.gen_range(0..grid.len())].clone())
                .collect();
            icc(&g).0
        })
        .collect();
    v.sort_by(f64::total_cmp);
    (v[boots / 40], v[boots - 1 - boots / 40])
}

/// One standard normal draw (Box–Muller).
fn normal(rng: &mut StdRng) -> f64 {
    let (u1, u2): (f64, f64) = (rng.gen_range(1e-12..1.0), rng.gen());
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

/// Operating characteristics of the pre-registered rules (`KOBLITZ_ICC_POWER=1`):
/// simulate `a × b` grids from the fitted model with the observed `σ²_w`, `σ²_e`
/// and a planted curve share `rho`, and count how often the rules say H0
/// (upper bound < 0.05) and H1 (point ≥ 0.10 with lower bound > 0).
fn power(sw: f64, se: f64, a: usize, b: usize, rho: f64, sims: usize, rng: &mut StdRng) -> Value {
    let su = rho / (1.0 - rho) * (sw + se);
    let (mut h0, mut h1, mut points) = (0, 0, Vec::with_capacity(sims));
    for _ in 0..sims {
        let u: Vec<f64> = (0..a).map(|_| su.sqrt() * normal(rng)).collect();
        let w: Vec<f64> = (0..b).map(|_| sw.sqrt() * normal(rng)).collect();
        let grid: Vec<Vec<f64>> = (0..a)
            .map(|i| {
                (0..b)
                    .map(|j| u[i] + w[j] + se.sqrt() * normal(rng))
                    .collect()
            })
            .collect();
        let point = icc(&grid).0;
        let (lo, hi) = boot_ci(&grid, 400, rng);
        h0 += (hi < 0.05) as usize;
        h1 += (point >= 0.10 && lo > 0.0) as usize;
        points.push(point);
    }
    points.sort_by(f64::total_cmp);
    json!({"curves": a, "subspaces": b, "icc_true": rho, "sims": sims,
           "p_h0": h0 as f64 / sims as f64, "p_h1": h1 as f64 / sims as f64,
           "icc_median": points[sims / 2]})
}

fn main() {
    let mut groups: BTreeMap<(u64, u64, u64, i64), Vec<Value>> = BTreeMap::new();
    for path in std::env::args().skip(1) {
        for line in std::fs::read_to_string(&path).expect("read").lines() {
            let r: Value = serde_json::from_str(line).expect("json");
            if r.get("skipped").is_some()
                || r["inconsistent"].as_u64().unwrap_or(0) > 0
                || !r["bsgs_ok"].as_bool().unwrap_or(true)
            {
                continue;
            }
            let key = (
                r["n"].as_u64().unwrap(),
                r["a2"].as_u64().unwrap(),
                r["l"].as_u64().unwrap(),
                r["m"].as_i64().unwrap_or(2),
            );
            groups.entry(key).or_default().push(r);
        }
    }
    let mut out = serde_json::Map::new();
    for ((n, a2, l, m), rs) in groups {
        let seeds: BTreeSet<u64> = rs
            .iter()
            .map(|r| r["basis_seed"].as_u64().unwrap())
            .collect();
        let mut by: BTreeMap<u64, BTreeMap<u64, &Value>> = BTreeMap::new();
        for r in &rs {
            by.entry(r["a6"].as_u64().unwrap())
                .or_default()
                .insert(r["basis_seed"].as_u64().unwrap(), r);
        }
        let curves: Vec<u64> = by
            .iter()
            .filter(|(_, v)| seeds.iter().all(|s| v.contains_key(s)))
            .map(|(c, _)| *c)
            .collect();
        let mut res = serde_json::Map::new();
        res.insert("curves".into(), json!(curves.len()));
        res.insert("subspaces".into(), json!(seeds.len()));
        let diag = std::env::var("KOBLITZ_ICC_DIAG").is_ok_and(|v| v == "1");
        let metrics: Vec<Metric> = METRICS
            .iter()
            .chain(if diag { DIAGNOSTICS.iter() } else { [].iter() })
            .copied()
            .collect();
        let mut fitted: Vec<(&str, f64, f64)> = Vec::new();
        for (name, metric) in metrics {
            let grid: Vec<Vec<f64>> = curves
                .iter()
                .map(|c| seeds.iter().map(|s| metric(by[c][s])).collect())
                .collect();
            let (point, su, sw, se) = icc(&grid);
            let mut rng = StdRng::seed_from_u64(7);
            let (lo, hi) = boot_ci(&grid, 2000, &mut rng);
            let tot = su + sw + se;
            res.insert(
                name.into(),
                json!({"icc_curve": point, "ci95": [lo, hi],
                       "var_curve": su, "var_subspace": sw, "var_residual": se,
                       "share_subspace": if tot > 0.0 { sw / tot } else { 0.0 }}),
            );
            fitted.push((name, sw, se));
        }
        if std::env::var("KOBLITZ_ICC_POWER").is_ok_and(|v| v == "1") {
            let mut rng = StdRng::seed_from_u64(13);
            let mut pw = serde_json::Map::new();
            for (name, sw, se) in fitted.iter().take(METRICS.len()) {
                let rows: Vec<Value> = [4usize, 8, 32]
                    .iter()
                    .flat_map(|&b| {
                        [0.0, 0.05, 0.10, 0.20]
                            .map(|rho| power(*sw, *se, curves.len(), b, rho, 200, &mut rng))
                    })
                    .collect();
                pw.insert((*name).into(), Value::Array(rows));
            }
            res.insert("power".into(), Value::Object(pw));
        }
        if std::env::var("KOBLITZ_ICC_DIAG").is_ok_and(|v| v == "1") {
            res.insert("diag_F_by_group".into(), f_by_group(n as u32, &curves, &by));
        }
        out.insert(format!("n={n} a2={a2} l={l} m={m}"), Value::Object(res));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&Value::Object(out)).unwrap()
    );
}
