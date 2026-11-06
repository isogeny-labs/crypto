//! Native summarizer for a frozen GLS symmetry NDJSON run.
use std::collections::BTreeMap;
use std::fs;
#[derive(Clone)]
struct R {
    seed: u64,
    arm: String,
    trials: u64,
    adds: u64,
    muls: u64,
    extras: u64,
    row: u64,
    ns: u64,
}
fn n(s: &str, key: &str) -> u64 {
    let pattern = format!("\"{}\":", key);
    let t = &s[s
        .find(&pattern)
        .unwrap_or_else(|| panic!("missing {}", key))
        + pattern.len()..];
    t.chars()
        .take_while(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}
fn median(mut x: Vec<u64>) -> u64 {
    x.sort_unstable();
    x[x.len() / 2]
}
fn cost(x: &R, w: f64) -> f64 {
    x.muls as f64 + w * x.extras as f64
}
fn main() {
    let path = std::env::args().nth(1).expect("NDJSON path");
    let raw = fs::read_to_string(path).unwrap();
    let mut cells: BTreeMap<(u64, u64), Vec<R>> = BTreeMap::new();
    for line in raw.lines() {
        if !(line.contains("\"kind\":\"solve\"") || line.contains("\"kind\":\"rho\"")) {
            continue;
        }
        let arm = if line.contains("\"arm\":\"signed-flat\"") {
            "flat"
        } else if line.contains("\"arm\":\"psi-folded\"") {
            "fold"
        } else {
            "rho"
        };
        let muls = n(line, "f2_mul");
        let extras = n(line, "f2_inv")
            + n(line, "probes")
            + n(line, "mod_inv")
            + if arm == "rho" {
                n(line, "canonicalisations")
            } else {
                n(line, "psi_eval") + n(line, "row_ops") + n(line, "mod_square_tests")
            };
        cells
            .entry((n(line, "p"), n(line, "b")))
            .or_default()
            .push(R {
                seed: n(line, "seed"),
                arm: arm.to_string(),
                trials: if arm == "rho" { 0 } else { n(line, "trials") },
                adds: n(line, "group_add"),
                muls,
                extras,
                row: if arm == "rho" { 0 } else { n(line, "row_ops") },
                ns: n(line, "total_ns"),
            });
    }
    println!("p,b,arm,n,median_trials,median_group_add,median_f2_mul,median_extra_ops,median_row_ops,median_ns,min_ns,max_ns,cost_w0,cost_w0p1,cost_w1,flat_over_arm_w0,flat_over_arm_w0p1,flat_over_arm_w1");
    for ((p, b), rows) in cells {
        assert_eq!(rows.len(), 9);
        let flat: Vec<&R> = rows.iter().filter(|x| x.arm == "flat").collect();
        let fcost = [0.0, 0.1, 1.0]
            .map(|w| median(flat.iter().map(|x| cost(x, w).round() as u64).collect()) as f64);
        for arm in ["flat", "fold", "rho"] {
            let a: Vec<&R> = rows.iter().filter(|x| x.arm == arm).collect();
            assert_eq!(a.len(), 3);
            for seed in 0..3 {
                assert_eq!(a[seed as usize].seed, seed);
            }
            let mut ns: Vec<u64> = a.iter().map(|x| x.ns).collect();
            ns.sort_unstable();
            let cc = [0.0, 0.1, 1.0]
                .map(|w| median(a.iter().map(|x| cost(x, w).round() as u64).collect()) as f64);
            println!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{:.1},{:.1},{:.1},{:.6},{:.6},{:.6}",
                p,
                b,
                arm,
                a.len(),
                median(a.iter().map(|x| x.trials).collect()),
                median(a.iter().map(|x| x.adds).collect()),
                median(a.iter().map(|x| x.muls).collect()),
                median(a.iter().map(|x| x.extras).collect()),
                median(a.iter().map(|x| x.row).collect()),
                ns[1],
                ns[0],
                ns[2],
                cc[0],
                cc[1],
                cc[2],
                fcost[0] / cc[0],
                fcost[1] / cc[1],
                fcost[2] / cc[2]
            );
        }
    }
}
