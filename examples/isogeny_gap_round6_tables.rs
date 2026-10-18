//! # Tables for research/isogeny_conductor_gap_round6_20261018 (E8, E9a)
//!
//! Reads `e8/records.jsonl` (written by
//! `examples/isogeny_gap_f4_phase_profile.rs`).
//!
//! Work per system is word XORs plus monomial tests:
//! - **full:** F4 elimination plus the reduced-basis read-off and its check;
//! - **early:** F4 elimination up to the stop, plus the affine elimination
//!   and the enumeration.
//!
//! Ratios are ratios of sums.  Work intervals bootstrap over systems (2,000
//! resamples, seed 1); yield intervals use the log ratio of two Poisson
//! counts.
//!
//! ```bash
//! cargo run --release --example isogeny_gap_round6_tables -- \
//!   --dir research/isogeny_conductor_gap_round6_20261018 \
//!   --out research/isogeny_conductor_gap_round6_20261018/TABLES.md \
//!   --json research/isogeny_conductor_gap_round6_20261018/tables.json
//! ```

use std::collections::BTreeMap;
use std::fmt::Write as _;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

fn arg(args: &[String], name: &str) -> String {
    let i = args
        .iter()
        .position(|a| a == name)
        .unwrap_or_else(|| panic!("missing {name}"));
    args[i + 1].clone()
}

struct Sys {
    solvable: bool,
    lifted: f64,
    full: f64,
    early: f64,
    stopped: bool,
}

fn work_full(r: &Value) -> f64 {
    (r["full"]["word_xors"].as_u64().unwrap() + r["full"]["extraction_tests"].as_u64().unwrap())
        as f64
}

fn work_early(r: &Value) -> f64 {
    (r["early"]["word_xors"].as_u64().unwrap()
        + r["early"]["extraction_tests"].as_u64().unwrap()
        + r["early"]["extraction_xors"].as_u64().unwrap()) as f64
}

/// Bootstrap 95 % interval of Σa / Σb over paired items.
fn boot_ratio(pairs: &[(f64, f64)], rng: &mut StdRng) -> (f64, f64, f64) {
    let ratio = |v: &[(f64, f64)]| {
        let (a, b) = v.iter().fold((0.0, 0.0), |s, p| (s.0 + p.0, s.1 + p.1));
        a / b
    };
    if pairs.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let point = ratio(pairs);
    let mut rs: Vec<f64> = (0..2000)
        .map(|_| {
            let s: Vec<(f64, f64)> = (0..pairs.len())
                .map(|_| pairs[rng.gen_range(0..pairs.len())])
                .collect();
            ratio(&s)
        })
        .filter(|r| r.is_finite())
        .collect();
    rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| rs[((rs.len() as f64 - 1.0) * p).round() as usize];
    (point, q(0.025), q(0.975))
}

/// Bootstrap 95 % interval of mean(a) / mean(b) over two independent samples.
fn boot_means(a: &[f64], b: &[f64], rng: &mut StdRng) -> (f64, f64, f64) {
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    if a.is_empty() || b.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let point = mean(a) / mean(b);
    let mut rs: Vec<f64> = (0..2000)
        .map(|_| {
            let sa: Vec<f64> = (0..a.len()).map(|_| a[rng.gen_range(0..a.len())]).collect();
            let sb: Vec<f64> = (0..b.len()).map(|_| b[rng.gen_range(0..b.len())]).collect();
            mean(&sa) / mean(&sb)
        })
        .collect();
    rs.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let q = |p: f64| rs[((rs.len() as f64 - 1.0) * p).round() as usize];
    (point, q(0.025), q(0.975))
}

fn poisson_ratio(a: f64, b: f64) -> (f64, f64, f64) {
    if a == 0.0 || b == 0.0 {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let r = a / b;
    let se = (1.0 / a + 1.0 / b).sqrt();
    (r, r * (-1.96 * se).exp(), r * (1.96 * se).exp())
}

fn f(x: f64) -> String {
    if x.is_nan() {
        "–".into()
    } else {
        format!("{x:.2}")
    }
}

fn ci(t: (f64, f64, f64)) -> String {
    if t.0.is_nan() {
        "–".into()
    } else {
        format!("{} [{}, {}]", f(t.0), f(t.1), f(t.2))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = arg(&args, "--dir");
    let mut rng = StdRng::seed_from_u64(1);
    // (config, target) → system; a configuration interrupted by a host
    // restart and rerun appends its systems again, and the last copy wins.
    let mut by_target: BTreeMap<(String, u64), Sys> = BTreeMap::new();
    let mut configs: BTreeMap<String, Value> = BTreeMap::new();
    for line in std::fs::read_to_string(format!("{dir}/e8/records.jsonl"))
        .expect("records")
        .lines()
    {
        let r: Value = serde_json::from_str(line).unwrap();
        let config = r["config"].as_str().unwrap().to_string();
        if r["kind"] == "config_done" {
            configs.insert(config, r);
            continue;
        }
        let target = r["target"].as_u64().unwrap();
        by_target.insert((config, target), Sys {
            solvable: r["solutions"].as_u64().unwrap() > 0,
            lifted: r["lifted"].as_f64().unwrap(),
            full: work_full(&r),
            early: work_early(&r),
            stopped: r["early"]["early_stopped"].as_bool().unwrap(),
        });
    }
    let mut systems: BTreeMap<String, Vec<Sys>> = BTreeMap::new();
    for ((config, _), sys) in by_target {
        systems.entry(config).or_default().push(sys);
    }
    // Only finished configurations count.
    systems.retain(|k, _| configs.contains_key(k));
    let n_sys: usize = systems.values().map(|v| v.len()).sum();
    let mut md = String::new();
    writeln!(
        md,
        "# Round 6 tables (E8, E9a)\n\n{} configurations, {n_sys} systems; every early run's solution set equals the full run's (the profiler aborts otherwise).\n",
        configs.len()
    )
    .unwrap();

    writeln!(md, "## E8. Where the work goes, per configuration\n\nWork = word XORs + monomial tests.  \"solv ÷ unsolv\" is mean full work of a solvable system over an unsolvable one (P-E8a: ≥ 3).  \"early ÷ full, solvable\" pools the solvable systems (P-E8c: ≤ 0.5).  \"saving\" is 1 − Σ early / Σ full over all systems, and \"solvable share\" the share of full work in solvable systems (P-E8d: saving ≤ share).\n\n| config | systems | solvable | stopped early | solv ÷ unsolv | early ÷ full, solvable | saving | solvable share |\n|:--|--:|--:|--:|:--|:--|--:|--:|").unwrap();
    let mut rows = Vec::new();
    let mut pooled_solv: Vec<(f64, f64)> = Vec::new();
    for (config, v) in &systems {
        let solv: Vec<f64> = v.iter().filter(|s| s.solvable).map(|s| s.full).collect();
        let unsolv: Vec<f64> = v.iter().filter(|s| !s.solvable).map(|s| s.full).collect();
        let ratio_su = boot_means(&solv, &unsolv, &mut rng);
        let pairs: Vec<(f64, f64)> = v
            .iter()
            .filter(|s| s.solvable)
            .map(|s| (s.early, s.full))
            .collect();
        pooled_solv.extend(&pairs);
        let early_solv = boot_ratio(&pairs, &mut rng);
        let tot_full: f64 = v.iter().map(|s| s.full).sum();
        let tot_early: f64 = v.iter().map(|s| s.early).sum();
        let saving = 1.0 - tot_early / tot_full;
        let share = solv.iter().sum::<f64>() / tot_full;
        let stopped = v.iter().filter(|s| s.stopped).count();
        writeln!(
            md,
            "| `{config}` | {} | {} | {stopped} | {} | {} | {:.1} % | {:.1} % |",
            v.len(),
            solv.len(),
            ci(ratio_su),
            ci(early_solv),
            100.0 * saving,
            100.0 * share
        )
        .unwrap();
        rows.push(json!({"config": config, "systems": v.len(), "solvable": solv.len(),
            "stopped_early": stopped, "solvable_over_unsolvable": [ratio_su.0, ratio_su.1, ratio_su.2],
            "early_over_full_solvable": [early_solv.0, early_solv.1, early_solv.2],
            "saving": saving, "solvable_share": share, "full_work": tot_full, "early_work": tot_early}));
    }
    let pooled = boot_ratio(&pooled_solv, &mut rng);
    writeln!(
        md,
        "\nPooled over every configuration's solvable systems ({}), early ÷ full = {}.",
        pooled_solv.len(),
        ci(pooled)
    )
    .unwrap();

    writeln!(md, "\n## E9a. Trace coset against the default base\n\n\"f₂\" is the fraction of default-base points in 2E (every coset point is in 2E).  Yield is lifted decompositions over 200 shared targets.  P-E9a: yield ratio ≈ 1 / f₂ within 20 % where the default has ≥ 30 decompositions.  P-E9c (a = 1): coset F4 work per system ≥ 1.3× the default's.\n\n| curve | a | d | f₂ (default) | 1 / f₂ | yield default | yield coset | coset ÷ default | within 20 % of 1/f₂ | full F4 work per system, coset ÷ default | per relation, default ÷ coset |\n|:--|--:|--:|--:|--:|--:|--:|:--|:--|:--|--:|").unwrap();
    let mut e9 = Vec::new();
    for (config, cfg) in &configs {
        if cfg["base"] != "default" {
            continue;
        }
        let coset_key = config.replace(":default", ":coset");
        let (Some(coset_cfg), Some(dv), Some(cv)) = (
            configs.get(&coset_key),
            systems.get(config),
            systems.get(&coset_key),
        ) else {
            continue;
        };
        let _ = coset_cfg;
        let f2 = cfg["base_points_in_2e"].as_f64().unwrap() / cfg["base_points"].as_f64().unwrap();
        let yd: f64 = dv.iter().map(|s| s.lifted).sum();
        let yc: f64 = cv.iter().map(|s| s.lifted).sum();
        let y = poisson_ratio(yc, yd);
        let within = (yd >= 30.0).then(|| (y.0 * f2 - 1.0).abs() <= 0.2);
        let wd: Vec<f64> = dv.iter().map(|s| s.full).collect();
        let wc: Vec<f64> = cv.iter().map(|s| s.full).collect();
        let w = boot_means(&wc, &wd, &mut rng);
        // F4 work per lifted decomposition, default over coset (> 1: coset wins).
        let per_rel = if yd > 0.0 && yc > 0.0 {
            (wd.iter().sum::<f64>() / yd) / (wc.iter().sum::<f64>() / yc)
        } else {
            f64::NAN
        };
        writeln!(
            md,
            "| `{}` | {} | {} | {:.3} | {} | {yd:.0} | {yc:.0} | {} | {} | {} | {} |",
            cfg["curve"].as_str().unwrap(),
            cfg["a"],
            cfg["d"],
            f2,
            f(1.0 / f2),
            ci(y),
            match within {
                Some(true) => "yes",
                Some(false) => "no",
                None => "– (< 30)",
            },
            ci(w),
            f(per_rel)
        )
        .unwrap();
        e9.push(json!({"curve": cfg["curve"], "a": cfg["a"], "n": cfg["n"], "d": cfg["d"],
            "f2_default": f2, "yield_default": yd, "yield_coset": yc,
            "yield_ratio": [y.0, y.1, y.2], "within_20pct_of_inverse_f2": within,
            "full_work_per_system_coset_over_default": [w.0, w.1, w.2],
            "full_work_per_relation_default_over_coset": per_rel}));
    }
    std::fs::write(arg(&args, "--out"), &md).unwrap();
    std::fs::write(
        arg(&args, "--json"),
        serde_json::to_string_pretty(&json!({"e8": rows, "e8_pooled_early_over_full_solvable": [pooled.0, pooled.1, pooled.2], "e9a": e9}))
            .unwrap()
            + "\n",
    )
    .unwrap();
    print!("{md}");
}
