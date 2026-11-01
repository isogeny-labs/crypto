//! Frozen analysis for `research/jv_sieve_validation_20261101`.
//!
//! The producer writes one JSON line per attempted arm.  This program reads
//! one prime panel, retains the attempt census, checks the prospective
//! controls, and computes paired ratio-of-means bootstrap intervals in the
//! native `bitop/v1` price and at 0/0.1/1 GAE per word/hash operation.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Map, Value};

const RESAMPLES: usize = 10_000;
const BOOTSTRAP_SEED: u64 = 20_261_101;
const BAND: [f64; 3] = [0.0, 0.1, 1.0];
const EXPECTED_SEEDS: [u64; 8] = [33, 34, 35, 36, 37, 38, 39, 40];

#[derive(Clone)]
struct Obs {
    seed: u64,
    bitops: f64,
    native_bitops: f64,
    native_ops: f64,
    row: Value,
}

fn num(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().expect("finite number"),
        Value::String(s) => s.parse().expect("numeric string"),
        _ => panic!("not numeric: {v}"),
    }
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn read_json_line(path: &Path) -> Option<Value> {
    let text = fs::read_to_string(path).ok()?;
    let mut lines = text.lines().filter(|line| !line.trim().is_empty());
    let first = lines.next()?;
    if lines.next().is_some() {
        return None;
    }
    serde_json::from_str(first).ok()
}

fn obs(row: Value) -> Obs {
    let p = row["p"].as_u64().expect("p");
    let bits = 64 - p.leading_zeros() as u64;
    let counts = &row["total"]["counts"];
    let words = num(&counts["word_op"]);
    let probes = num(&counts["hash_probe"]);
    Obs {
        seed: row["seed"].as_u64().expect("seed"),
        bitops: num(&row["total_bitops"]),
        native_bitops: words * 64.0 + probes * 4.0 * bits as f64,
        native_ops: words + probes,
        row,
    }
}

fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    sorted[((sorted.len() - 1) as f64 * q).round() as usize]
}

fn paired_ratio(
    a: &[Obs],
    b: &[Obs],
    price: impl Fn(&Obs) -> f64,
    tag: u64,
) -> Value {
    assert_eq!(a.len(), b.len());
    assert!(a.len() >= 2);
    assert!(a.iter().zip(b).all(|(x, y)| x.seed == y.seed));
    let av: Vec<f64> = a.iter().map(&price).collect();
    let bv: Vec<f64> = b.iter().map(&price).collect();
    let point = mean(&av) / mean(&bv);
    let mut rng = StdRng::seed_from_u64(BOOTSTRAP_SEED ^ tag);
    let mut boot = Vec::with_capacity(RESAMPLES);
    for _ in 0..RESAMPLES {
        let mut sa = 0.0;
        let mut sb = 0.0;
        for _ in 0..av.len() {
            let i = rng.gen_range(0..av.len());
            sa += av[i];
            sb += bv[i];
        }
        boot.push(sa / sb);
    }
    boot.sort_by(|x, y| x.partial_cmp(y).expect("finite bootstrap"));
    json!({
        "ratio": point,
        "ci95": [quantile(&boot, 0.025), quantile(&boot, 0.975)],
        "pairs": av.len(),
        "bootstrap_seed": BOOTSTRAP_SEED ^ tag,
        "resamples": RESAMPLES,
    })
}

fn normalized(mut row: Value) -> Value {
    row.as_object_mut().expect("row object").remove("wall_ms");
    row
}

fn sorted(mut xs: Vec<Obs>) -> Vec<Obs> {
    xs.sort_by_key(|x| x.seed);
    xs
}

fn phase_count(row: &Value, phase: &str, kind: &str) -> f64 {
    num(&row["phases"][phase]["counts"][kind])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut panel: Option<PathBuf> = None;
    let mut protocol: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--panel" => panel = Some(PathBuf::from(&args[i + 1])),
            "--protocol" => protocol = Some(PathBuf::from(&args[i + 1])),
            "--out" => out = Some(PathBuf::from(&args[i + 1])),
            other => panic!("unknown argument {other}"),
        }
        i += 2;
    }
    let panel = panel.expect("--panel");
    let protocol = protocol.expect("--protocol");
    let out = out.expect("--out");
    let p: u64 = panel
        .file_name()
        .and_then(|x| x.to_str())
        .and_then(|x| x.strip_prefix('p'))
        .expect("panel directory p<prime>")
        .parse()
        .expect("prime");

    let protocol_value = read_json(&protocol);
    let protocol_sha256 = protocol_value["freeze"]["protocol_sha256"]
        .as_str()
        .expect("frozen protocol hash");
    let attempts = read_json(&panel.join("attempts.json"));
    let attempt_rows = attempts.as_array().expect("attempt array");
    let symmetry_status = read_json(&panel.join("symmetry-status.json"));
    let symmetry_rows: Vec<Value> = fs::read_to_string(panel.join("raw/symmetry.jsonl"))
        .unwrap_or_default()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    let symmetry_pass = symmetry_status["exit_code"].as_i64() == Some(0)
        && symmetry_status["output_valid"].as_bool() == Some(true)
        && symmetry_rows.len() == 8
        && symmetry_rows.iter().enumerate().all(|(i, row)| {
            row["p"].as_u64() == Some(p)
                && row["seed"].as_u64() == Some(EXPECTED_SEEDS[i])
                && row["j_in_Fp2"].as_bool() == Some(false)
                && row["j_in_Fp3"].as_bool() == Some(false)
        });
    let producer_complete = symmetry_pass
        && attempt_rows.len() == 33
        && attempt_rows.iter().all(|r| {
            r["exit_code"].as_i64() == Some(0) && r["output_valid"].as_bool() == Some(true)
        });

    let mut ic = Vec::new();
    let mut ic_aa = Vec::new();
    let mut r3 = Vec::new();
    let mut r3_aa = Vec::new();
    let mut r0 = Vec::new();
    let raw = panel.join("raw");
    if raw.exists() {
        let mut paths: Vec<PathBuf> = fs::read_dir(&raw)
            .expect("raw directory")
            .filter_map(|e| e.ok().map(|x| x.path()))
            .filter(|x| x.extension().and_then(|y| y.to_str()) == Some("jsonl"))
            .collect();
        paths.sort();
        for path in paths {
            let Some(row) = read_json_line(&path) else { continue };
            let name = path.file_name().unwrap().to_str().unwrap();
            let x = obs(row);
            if name.starts_with("ic_aa_") {
                ic_aa.push(x);
            } else if name.starts_with("ic_s") {
                ic.push(x);
            } else if name.starts_with("rho_R3_run0_") {
                r3.push(x);
            } else if name.starts_with("rho_R3_run1_") {
                r3_aa.push(x);
            } else if name.starts_with("rho_R0_run0_") {
                r0.push(x);
            }
        }
    }
    let (ic, ic_aa, r3, r3_aa, r0) = (
        sorted(ic),
        sorted(ic_aa),
        sorted(r3),
        sorted(r3_aa),
        sorted(r0),
    );
    let expected: Vec<u64> = EXPECTED_SEEDS.to_vec();
    let seeds_ok = [&ic, &r3, &r3_aa, &r0]
        .iter()
        .all(|xs| xs.iter().map(|x| x.seed).collect::<Vec<_>>() == expected)
        && ic_aa.iter().map(|x| x.seed).collect::<Vec<_>>() == vec![33];

    let ic_correct = ic.iter().chain(&ic_aa).all(|x| {
        x.row["solved"].as_bool() == Some(true)
            && x.row["correct"].as_bool() == Some(true)
            && x.row["verified_on_challenge"].as_bool() == Some(true)
            && x.row["challenge_non_weak"].as_bool() == Some(true)
    });
    let rho_correct = r3.iter().chain(&r3_aa).chain(&r0).all(|x| {
        x.row["correct"].as_bool() == Some(true)
            && x.row["verified_on_challenge"].as_bool() == Some(true)
            && x.row["capped"].as_bool() == Some(false)
    });
    let completeness = ic.iter().chain(&ic_aa).all(|x| {
        x.row["foreign_hook_calls"].as_u64() == Some(0)
            && x.row["other_modulus_ops"].as_u64() == Some(0)
            && num(&x.row["phases"]["unattributed"]["bitops"]) == 0.0
    });
    let route_phases = ic.iter().chain(&ic_aa).all(|x| {
        num(&x.row["phases"]["walk"]["bitops"]) > 0.0
            && x.row["path_steps"].as_u64().unwrap_or(0) > 0
            && phase_count(&x.row, "descent", "field_add") > 0.0
            && phase_count(&x.row, "descent", "field_mul") > 0.0
            && phase_count(&x.row, "linear_algebra", "modr_add") > 0.0
            && phase_count(&x.row, "linear_algebra", "modr_mul") > 0.0
    });
    let hash_check = |x: &Obs, factor: u64| {
        let ops = &x.row["ops"];
        ops["hash"].as_u64()
            == Some(
                x.row["steps"].as_u64().unwrap()
                    + factor * ops["dp_check"].as_u64().unwrap(),
            )
    };
    let corrected_hashes = r3
        .iter()
        .chain(&r3_aa)
        .all(|x| hash_check(x, 3))
        && r0.iter().all(|x| hash_check(x, 1));
    let ic_aa_identical = ic_aa.len() == 1
        && ic
            .iter()
            .find(|x| x.seed == ic_aa[0].seed)
            .is_some_and(|x| normalized(x.row.clone()) == normalized(ic_aa[0].row.clone()));

    let ratios_available = seeds_ok && ic.len() == 8 && r3.len() == 8;
    let raw_ratio = ratios_available.then(|| paired_ratio(&ic, &r3, |x| x.bitops, p << 8));
    let gae = r3.first().map(|x| {
        let pr = &x.row["fq3_prices"];
        let bits = 64 - p.leading_zeros() as u64;
        2.0 * num(&pr["mul"]["bitops"])
            + num(&pr["sq"]["bitops"])
            + num(&pr["inv"]["bitops"])
            + 42.0 * 13.0 * bits as f64
    });
    let mut band = Vec::new();
    if ratios_available {
        let gae = gae.expect("R3 GAE price");
        for (k, w) in BAND.into_iter().enumerate() {
            let price = |x: &Obs| x.bitops - x.native_bitops + w * gae * x.native_ops;
            let ic_mean = mean(&ic.iter().map(&price).collect::<Vec<_>>());
            let rho_mean = mean(&r3.iter().map(&price).collect::<Vec<_>>());
            band.push(json!({
                "w_gae": w,
                "gae_bitops": gae,
                "ic_mean": ic_mean,
                "rho_mean": rho_mean,
                "result": paired_ratio(&ic, &r3, price, (p << 8) ^ 0x40 ^ k as u64),
            }));
        }
    }
    let rho_aa = (r3.len() == 8 && r3_aa.len() == 8)
        .then(|| paired_ratio(&r3_aa, &r3, |x| x.bitops, (p << 8) ^ 0xAA));
    let rho_aa_pass = rho_aa.as_ref().is_some_and(|x| {
        let point = num(&x["ratio"]);
        let lo = num(&x["ci95"][0]);
        let hi = num(&x["ci95"][1]);
        (0.50..=2.00).contains(&point) && lo <= 1.0 && hi >= 1.0
    });
    let r0_mean = (!r0.is_empty()).then(|| {
        mean(
            &r0.iter()
                .map(|x| num(&x.row["steps_over_sqrt_l"]))
                .collect::<Vec<_>>(),
        )
    });
    let r3_mean = (!r3.is_empty()).then(|| {
        mean(
            &r3.iter()
                .map(|x| num(&x.row["steps_over_sqrt_l"]))
                .collect::<Vec<_>>(),
        )
    });
    let positive_pass = r0_mean.is_some_and(|x| (0.90..=1.70).contains(&x))
        && r3_mean.is_some_and(|x| (0.55..=1.30).contains(&x));
    let controls_pass = producer_complete
        && seeds_ok
        && ic_correct
        && rho_correct
        && completeness
        && route_phases
        && corrected_hashes
        && ic_aa_identical
        && rho_aa_pass
        && positive_pass;

    let mut intervals: Vec<(f64, f64)> = Vec::new();
    if let Some(x) = &raw_ratio {
        intervals.push((num(&x["ci95"][0]), num(&x["ci95"][1])));
    }
    for x in &band {
        intervals.push((
            num(&x["result"]["ci95"][0]),
            num(&x["result"]["ci95"][1]),
        ));
    }
    let verdict = if !controls_pass || intervals.len() != 4 {
        "withheld"
    } else if intervals.iter().all(|(_, hi)| *hi < 1.0) {
        "below_strong_rho"
    } else if intervals.iter().all(|(lo, _)| *lo > 1.0) {
        "above_strong_rho"
    } else {
        "unresolved"
    };

    let mut phase_means = Map::new();
    if let Some(first) = ic.first() {
        for name in first.row["phases"].as_object().unwrap().keys() {
            phase_means.insert(
                name.clone(),
                json!(mean(
                    &ic.iter()
                        .map(|x| num(&x.row["phases"][name]["bitops"]))
                        .collect::<Vec<_>>()
                )),
            );
        }
    }
    let result = json!({
        "schema": "jv_sieve_validation_analysis/v1",
        "p": p,
        "protocol_sha256": protocol_sha256,
        "attempts": {
            "planned": 33,
            "recorded": attempt_rows.len(),
            "failed": attempt_rows.iter().filter(|x| x["exit_code"].as_i64() != Some(0) || x["output_valid"].as_bool() != Some(true)).count(),
            "producer_complete": producer_complete,
        },
        "run_counts": {"ic": ic.len(), "ic_aa": ic_aa.len(), "rho_r3": r3.len(), "rho_r3_aa": r3_aa.len(), "rho_r0": r0.len()},
        "controls": {
            "seeds_exact": {"pass": seeds_ok, "expected": EXPECTED_SEEDS},
            "correctness": {"pass": ic_correct && rho_correct, "ic": ic_correct, "rho": rho_correct},
            "counter_completeness": {"pass": completeness},
            "walk_f4_la_present": {"pass": route_phases},
            "corrected_hash_accounting": {"pass": corrected_hashes, "R3_identity": "hash = steps + 3*dp_check", "R0_identity": "hash = steps + dp_check"},
            "strongest_rho_symmetry": {"pass": symmetry_pass, "rows": symmetry_rows.len(), "rule": "j is in neither Fp2 nor Fp3 for every seed"},
            "ic_aa": {"pass": ic_aa_identical, "seed": 33, "rule": "all non-wall producer fields identical"},
            "rho_aa": {"pass": rho_aa_pass, "accept_point": [0.50, 2.00], "accept_ci_contains": 1.0, "result": rho_aa},
            "positive_rho": {"pass": positive_pass, "R0_mean_steps_sqrt_l": r0_mean, "R0_window": [0.90, 1.70], "R3_mean_steps_sqrt_l": r3_mean, "R3_window": [0.55, 1.30]},
        },
        "controls_pass": controls_pass,
        "comparison": {
            "arm": "full-route IC including walk, F4 and linear algebra",
            "reference": "R3 corrected-hash strong/equal-advice rho run 0",
            "bitop_v1": raw_ratio,
            "gae_band": band,
            "ic_phase_bitops_mean": phase_means,
            "verdict": verdict,
        },
    });
    fs::write(&out, serde_json::to_string_pretty(&result).unwrap() + "\n")
        .unwrap_or_else(|e| panic!("{}: {e}", out.display()));
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
