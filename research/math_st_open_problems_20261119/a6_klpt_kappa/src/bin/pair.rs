//! Paired A4 -> A5 comparison from the two recorded runs.csv files.
//! Usage: pair <a4_runs.csv> <a5_runs.csv>
//! Rows are matched by (csv_line, ell, run). Prints, per (p_bits, ell): runs, how
//! many share N with A4, median of A4 and A5 log_p ell^e, median paired drop, and
//! the median log_p ratio for A5; then an OLS slope of log2(ratio) on log2 p.
use std::collections::BTreeMap;

fn rows(path: &str) -> Vec<BTreeMap<String, String>> {
    let t = std::fs::read_to_string(path).unwrap();
    let mut it = t.lines();
    let h: Vec<String> = it.next().unwrap().split(',').map(String::from).collect();
    it.filter(|l| !l.is_empty())
        .map(|l| h.iter().cloned().zip(l.split(',').map(String::from)).collect())
        .collect()
}
fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (r4, r5) = (rows(&a[1]), rows(&a[2]));
    let key = |r: &BTreeMap<String, String>| (r["csv_line"].clone(), r["ell"].clone(), r["run"].clone());
    let m4: BTreeMap<_, _> = r4.iter().map(|r| (key(r), r)).collect();
    let f = |r: &BTreeMap<String, String>, k: &str| r[k].parse::<f64>().unwrap();
    let mut g: BTreeMap<(u32, u32), (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, u32, u32)> = BTreeMap::new();
    let mut ols: BTreeMap<u32, Vec<(f64, f64)>> = BTreeMap::new();
    for r in &r5 {
        let b = &m4[&key(r)];
        assert_eq!(r["status"], "ok");
        assert_eq!(b["status"], "ok");
        let k = (r["p_bits"].parse().unwrap(), r["ell"].parse().unwrap());
        let e = g.entry(k).or_default();
        e.0.push(f(b, "log_p_ell_e"));
        e.1.push(f(r, "log_p_ell_e"));
        e.2.push(f(b, "log_p_ell_e") - f(r, "log_p_ell_e"));
        e.3.push(f(r, "log_p_ratio"));
        e.4 += 1;
        e.5 += (r["N"] == b["N"]) as u32;
        let p = f(r, "p");
        ols.entry(k.1).or_default().push((p.log2(), f(r, "log_p_ratio") * p.log2()));
    }
    println!("p_bits,ell,runs,same_N_as_A4,median_A4_log_p_ell_e,median_A5_log_p_ell_e,median_paired_drop,median_A5_log_p_ratio");
    for ((bits, ell), (x4, x5, d, rt, n, sn)) in g {
        println!("{bits},{ell},{n},{sn},{:.3},{:.3},{:.3},{:.3}", med(x4), med(x5), med(d), med(rt));
    }
    println!("ell,ols_slope_log2_ratio_vs_log2_p");
    for (ell, v) in ols {
        let n = v.len() as f64;
        let (mx, my) = (v.iter().map(|t| t.0).sum::<f64>() / n, v.iter().map(|t| t.1).sum::<f64>() / n);
        let s = v.iter().map(|t| (t.0 - mx) * (t.1 - my)).sum::<f64>() / v.iter().map(|t| (t.0 - mx).powi(2)).sum::<f64>();
        println!("{ell},{s:.3}");
    }
}
