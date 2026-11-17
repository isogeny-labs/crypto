//! Native derivation of the seven-curve v2 count summary from raw NDJSON.
//! Usage: quartic_twist24_summary RAW CSV_OUT > SUMMARY_JSON
use std::collections::BTreeMap;
use std::fs;

use serde_json::{json, Value};

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn med(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(|a, b| a.total_cmp(b));
    let n = xs.len();
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    }
}
fn cost(r: &Value, i: usize) -> u64 {
    let common = u(&r["model_fp_muls"])
        + u(&r["input_fp_muls"])
        + u(&r["base_fp_muls"])
        + u(&r["table_fp_muls"]);
    let extra = match i {
        0 => 0,
        1 => u(&r["cubic_map_fp_muls"]) + u(&r["cubic_eigen_fp_muls"]),
        2 => {
            u(&r["twist_map_fp_muls"])
                + u(&r["cubic_eigen_fp_muls"])
                + u(&r["twist_geometry_fp_muls"])
                + u(&r["frob_eigen_fp_muls"])
        }
        _ => panic!("arm"),
    };
    common + extra + u(&r["stream_fp_muls_at_solve"][i]) + u(&r["verification_fp_muls"][i])
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3, "usage: quartic_twist24_summary RAW CSV_OUT");
    let raw = fs::read_to_string(&args[1]).expect("raw file");
    let records: Vec<Value> = raw
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(records.len(), 21);
    let mut groups = BTreeMap::<u64, Vec<&Value>>::new();
    for r in &records {
        groups.entry(u(&r["p"])).or_default().push(r);
    }
    assert_eq!(groups.len(), 7);
    let mut curves = Vec::new();
    let mut csv=String::from("p,r,base_abscissae,flat_columns,cubic_columns,twist24_columns,flat_residuals_median,cubic_residuals_median,twist24_residuals_median,residual_ratio_median,residual_ratio_min,residual_ratio_max,cubic_la_median,twist24_la_median,la_ratio_median,la_ratio_min,la_ratio_max,cubic_m_median,twist24_m_median,m_ratio_median,m_ratio_min,m_ratio_max,rho24_m_median,twist24_to_rho_median,gap_min,gap_max\n");
    for (p, mut rows) in groups {
        rows.sort_by_key(|r| u(&r["seed"]));
        assert_eq!(
            rows.iter().map(|r| u(&r["seed"])).collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        for r in &rows {
            for name in ["flat", "cubic", "twist24"] {
                assert!(r[name]["correct"].as_bool().unwrap());
                assert_eq!(u(&r[name]["recovered"]), 271);
            }
            assert_eq!(r["rho"].as_array().unwrap().len(), 48);
            assert!(r["rho"]
                .as_array()
                .unwrap()
                .iter()
                .all(|z| z["correct"].as_bool() == Some(true)));
        }
        let arr = |f: &dyn Fn(&Value) -> f64| rows.iter().map(|r| f(r)).collect::<Vec<_>>();
        let res_ratio = arr(&|r| {
            u(&r["cubic"]["residuals_at_solve"]) as f64
                / u(&r["twist24"]["residuals_at_solve"]) as f64
        });
        let la_ratio =
            arr(&|r| u(&r["cubic"]["la_ops"]) as f64 / u(&r["twist24"]["la_ops"]) as f64);
        let m_ratio = arr(&|r| cost(r, 1) as f64 / cost(r, 2) as f64);
        let mut rho_m = Vec::new();
        let mut gap = Vec::new();
        for r in &rows {
            let setup = u(&r["model_fp_muls"])
                + u(&r["input_fp_muls"])
                + u(&r["cubic_eigen_fp_muls"])
                + u(&r["twist_geometry_fp_muls"])
                + u(&r["frob_eigen_fp_muls"]);
            let rho = med(r["rho"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|z| z["fold"] == "Twist24")
                .map(|z| (u(&z["fp_muls"]) + setup) as f64)
                .collect());
            rho_m.push(rho);
            gap.push(cost(r, 2) as f64 / rho);
        }
        let lo = |v: &Vec<f64>| v.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = |v: &Vec<f64>| v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let row = json!({
            "p":p,"r":u(&rows[0]["n"]),
            "log2_r":(u(&rows[0]["n"]) as f64).log2(),
            "base_abscissae":u(&rows[0]["base_points"]),
            "flat_columns":u(&rows[0]["flat"]["columns"]),
            "cubic_columns":u(&rows[0]["cubic"]["columns"]),
            "twist24_columns":u(&rows[0]["twist24"]["columns"]),
            "flat_residuals_median":med(arr(&|r|u(&r["flat"]["residuals_at_solve"]) as f64)),
            "cubic_residuals_median":med(arr(&|r|u(&r["cubic"]["residuals_at_solve"]) as f64)),
            "twist24_residuals_median":med(arr(&|r|u(&r["twist24"]["residuals_at_solve"]) as f64)),
            "residual_ratio_median":med(res_ratio.clone()),"residual_ratio_min":lo(&res_ratio),"residual_ratio_max":hi(&res_ratio),
            "cubic_la_median":med(arr(&|r|u(&r["cubic"]["la_ops"]) as f64)),
            "twist24_la_median":med(arr(&|r|u(&r["twist24"]["la_ops"]) as f64)),
            "la_ratio_median":med(la_ratio.clone()),"la_ratio_min":lo(&la_ratio),"la_ratio_max":hi(&la_ratio),
            "cubic_m_median":med(arr(&|r|cost(r,1) as f64)),
            "twist24_m_median":med(arr(&|r|cost(r,2) as f64)),
            "m_ratio_median":med(m_ratio.clone()),"m_ratio_min":lo(&m_ratio),"m_ratio_max":hi(&m_ratio),
            "rho24_m_median":med(rho_m),
            "twist24_to_rho_median":med(gap.clone()),"gap_min":lo(&gap),"gap_max":hi(&gap),
        });
        let fields = [
            "p",
            "r",
            "base_abscissae",
            "flat_columns",
            "cubic_columns",
            "twist24_columns",
            "flat_residuals_median",
            "cubic_residuals_median",
            "twist24_residuals_median",
            "residual_ratio_median",
            "residual_ratio_min",
            "residual_ratio_max",
            "cubic_la_median",
            "twist24_la_median",
            "la_ratio_median",
            "la_ratio_min",
            "la_ratio_max",
            "cubic_m_median",
            "twist24_m_median",
            "m_ratio_median",
            "m_ratio_min",
            "m_ratio_max",
            "rho24_m_median",
            "twist24_to_rho_median",
            "gap_min",
            "gap_max",
        ];
        csv.push_str(
            &fields
                .iter()
                .map(|&k| row[k].to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        csv.push('\n');
        curves.push(row);
    }
    fs::write(&args[2], csv).expect("CSV output");
    println!(
        "{}",
        json!({"status":"derived","runs":records.len(),"curves":curves})
    );
}
