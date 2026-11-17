//! Aggregates `rows.jsonl` from the twist-obstruction harness into
//! `summary.json`: per-family totals, the decision-rule checks, and the
//! observed off-parity fraction with a 95 % Clopper–Pearson interval
//! against the independent-lift heuristic 1 - p_E^m.

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

fn ln_gamma(x: f64) -> f64 {
    // Lanczos, g = 7, n = 9.
    const C: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        return (std::f64::consts::PI / (std::f64::consts::PI * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = C[0];
    let t = x + 7.5;
    for (i, c) in C.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

/// P[Bin(n, p) <= k].
fn binom_cdf(k: u64, n: u64, p: f64) -> f64 {
    if p <= 0.0 {
        return 1.0;
    }
    if p >= 1.0 {
        return if k >= n { 1.0 } else { 0.0 };
    }
    let lc = ln_gamma(n as f64 + 1.0);
    (0..=k.min(n))
        .map(|i| {
            (lc - ln_gamma(i as f64 + 1.0) - ln_gamma((n - i) as f64 + 1.0)
                + i as f64 * p.ln()
                + (n - i) as f64 * (1.0 - p).ln())
            .exp()
        })
        .sum()
}

fn bisect(f: impl Fn(f64) -> f64, target: f64) -> f64 {
    // f decreasing in p on [0, 1]
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f(mid) > target {
            lo = mid
        } else {
            hi = mid
        }
    }
    0.5 * (lo + hi)
}

fn clopper_pearson(k: u64, n: u64) -> (f64, f64) {
    let lo = if k == 0 {
        0.0
    } else {
        bisect(|p| binom_cdf(k - 1, n, p), 0.975)
    };
    let hi = if k == n {
        1.0
    } else {
        bisect(|p| binom_cdf(k, n, p), 0.025)
    };
    (lo, hi)
}

fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or_else(|| panic!("missing {}", k))
}

const TALLY: [&str; 11] = [
    "solutions",
    "pure",
    "pure_with_2tor",
    "off_O",
    "off_T",
    "violation",
    "witness_fail",
    "obs_off",
    "pred_off",
    "obs_minus_pred",
    "pred_minus_obs",
];

fn main() {
    // Known values: CP(5, 10) = [0.1871, 0.8129]; CP(0, 3000) upper = 0.00123.
    let (a, b) = clopper_pearson(5, 10);
    assert!(
        (a - 0.1871).abs() < 1e-3 && (b - 0.8129).abs() < 1e-3,
        "{} {}",
        a,
        b
    );
    assert!((clopper_pearson(0, 3000).1 - 0.00123).abs() < 1e-5);
    let dir = std::env::args().nth(1).expect("run directory");
    let text = std::fs::read_to_string(format!("{}/rows.jsonl", dir)).unwrap();
    let rows: Vec<Value> = text
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();

    let mut census = Vec::new();
    let mut census_tot = BTreeMap::<&str, u64>::new();
    // key: (arm, n, m)
    let mut fam: BTreeMap<(String, u64, u64), Map<String, Value>> = BTreeMap::new();
    let mut acc: BTreeMap<(String, u64, u64), BTreeMap<String, f64>> = BTreeMap::new();

    for r in &rows {
        if r["family"] == "census" {
            let mismatch = (u(r, "sigma_minus") != u(r, "twist_count_Fq")) as u64
                + (u(r, "phi_image_ok") + 2 != u(r, "sigma_minus")) as u64
                + (u(r, "E_Fq") + u(r, "sigma_minus") != 2 * (1u64 << u(r, "n")) + 2) as u64;
            for k in [
                "pairs",
                "nontor_rational",
                "nontor_irrational",
                "tor_rational",
                "tor_irrational",
            ] {
                *census_tot.entry(k).or_default() += u(r, k);
            }
            *census_tot.entry("count_mismatches").or_default() += mismatch;
            census.push(json!({
                "n": r["n"], "a": r["a"], "b": r["b"], "E_Fq": r["E_Fq"], "twist_Fq": r["twist_count_Fq"],
                "pairs": r["pairs"], "nontor_rational": r["nontor_rational"],
                "tor_irrational": r["tor_irrational"], "count_mismatches": mismatch
            }));
            continue;
        }
        let key = (r["arm"].as_str().unwrap().to_string(), u(r, "n"), u(r, "m"));
        let a = acc.entry(key.clone()).or_default();
        let m = u(r, "m") as i32;
        let cm = &r["curve_model"];
        let tm = &r["twist_model"];
        let p_e = u(r, "v_E_type") as f64 / u(r, "v_nonzero") as f64;
        let sols = u(cm, "solutions") as f64;
        *a.entry("targets".into()).or_default() += 1.0;
        for k in TALLY {
            *a.entry(format!("curve.{}", k)).or_default() += u(cm, k) as f64;
            *a.entry(format!("twist.{}", k)).or_default() += u(tm, k) as f64;
        }
        *a.entry("targets_off_set_mismatch".into()).or_default() += ((u(cm, "obs_minus_pred")
            + u(cm, "pred_minus_obs")
            + u(tm, "obs_minus_pred")
            + u(tm, "pred_minus_obs"))
            > 0) as u64 as f64;
        *a.entry("targets_pred_nonempty".into()).or_default() +=
            (u(cm, "pred_off") > 0) as u64 as f64;
        *a.entry("xset_mismatch_targets".into()).or_default() +=
            (!r["xsets_equal"].as_bool().unwrap()) as u64 as f64;
        *a.entry("heuristic_weighted".into()).or_default() += sols * (1.0 - p_e.powi(m));
        *a.entry("p_E_sum".into()).or_default() += p_e;
        if r["poly_checked"].as_bool().unwrap() {
            *a.entry("poly_targets".into()).or_default() += 1.0;
            *a.entry("poly_roots".into()).or_default() += u(r, "poly_roots") as f64;
            *a.entry("poly_minus_points".into()).or_default() += u(r, "poly_minus_points") as f64;
            *a.entry("points_minus_poly".into()).or_default() += u(r, "points_minus_poly") as f64;
            for s in r["poly_minus_points_sample"].as_array().unwrap() {
                let k: Vec<u64> = s
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_u64().unwrap())
                    .collect();
                let tx = u(r, "target_x");
                // Formal-resultant degeneracy: x1 = x2 and x3 = x_R.
                let degenerate = k.len() == 3 && k[0] == k[1] && k[2] == tx;
                *a.entry(
                    if degenerate {
                        "poly_extra_degenerate"
                    } else {
                        "poly_extra_unexplained"
                    }
                    .into(),
                )
                .or_default() += 1.0;
                fam.entry(key.clone())
                    .or_default()
                    .entry("poly_extra_examples")
                    .or_insert(json!([]))
                    .as_array_mut()
                    .unwrap()
                    .push(json!({"target_x": tx, "a": r["a"], "b": r["b"], "tuple": k}));
            }
        }
    }

    let mut families = Vec::new();
    let mut gate = BTreeMap::<&str, f64>::new();
    for (key, a) in &acc {
        let g = |k: &str| *a.get(k).unwrap_or(&0.0);
        let n_sol = g("curve.solutions") as u64;
        let n_off = g("curve.obs_off") as u64;
        let (lo, hi) = clopper_pearson(n_off, n_sol);
        let mut f = fam.remove(key).unwrap_or_default();
        f.insert("arm".into(), json!(key.0));
        f.insert("n".into(), json!(key.1));
        f.insert("m".into(), json!(key.2));
        for (k, v) in a {
            f.insert(k.clone(), json!(*v));
        }
        f.insert("mean_p_E".into(), json!(g("p_E_sum") / g("targets")));
        f.insert(
            "observed_off_fraction".into(),
            json!(if n_sol > 0 {
                n_off as f64 / n_sol as f64
            } else {
                0.0
            }),
        );
        f.insert("observed_off_cp95".into(), json!([lo, hi]));
        f.insert(
            "heuristic_off_fraction".into(),
            json!(if n_sol > 0 {
                g("heuristic_weighted") / n_sol as f64
            } else {
                0.0
            }),
        );
        f.insert(
            "violation_rate_upper95".into(),
            json!(clopper_pearson(0, n_sol).1),
        );
        for k in [
            "curve.violation",
            "twist.violation",
            "curve.witness_fail",
            "twist.witness_fail",
            "targets_off_set_mismatch",
            "xset_mismatch_targets",
            "points_minus_poly",
            "poly_extra_unexplained",
        ] {
            *gate.entry(k).or_default() += g(k);
        }
        families.push(Value::Object(f));
    }
    for (k, v) in &census_tot {
        if ["nontor_rational", "tor_irrational", "count_mismatches"].contains(k) {
            gate.insert(k, *v as f64);
        }
    }
    let c1_ok = acc
        .iter()
        .filter(|(k, _)| k.0 == "C1_planted_xR_in_V")
        .all(|(_, a)| {
            a["targets_pred_nonempty"] == a["targets"] && a["targets_off_set_mismatch"] == 0.0
        });
    let c2_off_t: f64 = acc
        .iter()
        .filter(|(k, _)| k.0 == "C2_planted_T_pairs")
        .map(|(_, a)| a["curve.off_T"])
        .sum();
    let consistent = gate.values().all(|v| *v == 0.0);
    let out = json!({
        "run_dir": dir,
        "rows": rows.len(),
        "decision": {
            "gate_counters": gate,
            "positive_control_C1_detected": c1_ok,
            "positive_control_C2_off_T_total": c2_off_t,
            "valid": c1_ok && c2_off_t > 0.0,
            "consistent_with_T1_T2_T3": consistent && c1_ok && c2_off_t > 0.0,
        },
        "census_totals": census_tot,
        "census": census,
        "families": families,
    });
    std::fs::write(
        format!("{}/summary.json", dir),
        serde_json::to_string_pretty(&out).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&out["decision"]).unwrap()
    );
}
