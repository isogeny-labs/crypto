//! Evaluates the labelled-base cost-model lower bound of
//! `research/labelled_bases_20261026/README.md` on a grid and prints one JSON
//! line per point (`plot.py` there only lays the records out).
//!
//! `C(B) = B·N/s + (m!·N/B^m)·max(1, s^m/(m!·N))`, reported as
//! `log2(C/√N)` for `m ∈ {2,3,4}`, `log2 s ∈ {40,50,60}`, `log2 B ∈ [0, log2 s]`
//! in steps of 0.1, at `N = 2^60` (override with `--log-n`).
//!
//! Std only: `rustc -O --edition 2021 examples/labelled_base_bound.rs`.

fn factorial(m: u32) -> f64 {
    (1..=m).map(f64::from).product()
}

/// `log2(C/√N)` of the README's bound.
fn log2_cost_over_sqrt_n(log_n: f64, m: u32, log_s: f64, log_b: f64) -> f64 {
    let (n, s, b) = (log_n.exp2(), log_s.exp2(), log_b.exp2());
    let mf = factorial(m);
    let per_target = (s.powi(m as i32) / (mf * n)).max(1.0);
    let c = b * n / s + (mf * n / b.powi(m as i32)) * per_target;
    c.log2() - log_n / 2.0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let log_n: f64 = args
        .iter()
        .position(|a| a == "--log-n")
        .map(|i| args[i + 1].parse().expect("--log-n takes a number"))
        .unwrap_or(60.0);
    for m in 2..=4u32 {
        for log_s in [40.0f64, 50.0, 60.0] {
            let steps = (log_s * 10.0).round() as u32;
            let mut min = (f64::INFINITY, 0.0);
            for k in 0..=steps {
                let log_b = f64::from(k) / 10.0;
                let y = log2_cost_over_sqrt_n(log_n, m, log_s, log_b);
                if y < min.0 {
                    min = (y, log_b);
                }
                println!(
                    "{{\"kind\":\"point\",\"log_n\":{log_n},\"m\":{m},\"log_s\":{log_s},\"log_b\":{log_b:.1},\"log2_cost_over_sqrt_n\":{y:.6}}}"
                );
            }
            // The README's claim: the minimum over B stays at or above log2(2) = 1.
            assert!(
                min.0 >= 1.0 - 1e-9,
                "bound below 2√N at m={m}, log s={log_s}"
            );
            println!(
                "{{\"kind\":\"min\",\"log_n\":{log_n},\"m\":{m},\"log_s\":{log_s},\"log_b\":{:.1},\"log2_cost_over_sqrt_n\":{:.6}}}",
                min.1, min.0
            );
        }
    }
}
