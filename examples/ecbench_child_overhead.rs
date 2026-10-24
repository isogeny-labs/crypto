//! Per-child overhead of `ecbench exec`, the measured child every run of
//! every session executes in a fresh process.
//!
//! ```text
//! ecbench_child_overhead inputs --spec SPEC --count K --out DIR
//! ecbench_child_overhead time --inputs DIR --rounds N --seed S \
//!     --bin before=PATH --bin after=PATH --bin after-aa=PATH --out OUT.json
//! ```
//!
//! `inputs` writes the child inputs of the first `K` measured executions
//! of `SPEC`, exactly as `ecbench verify` builds them for a replay.
//! `time` runs every input through every binary in each round, in an
//! order that alternates by round (and is shuffled within it from
//! `--seed`), and records each child's wall time from spawn to exit.  It
//! requires every binary to return the same answer, total, phases and
//! counters for an input, so the binaries differ in overhead only.  Two
//! labels on one path are an A/A control.  The output carries every
//! sample, and per binary the median with a 95% interval from a bootstrap
//! over rounds, and the ratio of each binary's median to the first's.
//! Wall time on a shared host is descriptive; instruction counts come
//! from Callgrind.  The study is
//! `research/ecbench_child_overhead_20261024/`.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use crypto_lib::cryptanalysis::ecbench::methods::MethodSpec;
use crypto_lib::cryptanalysis::ecbench::record::{ChildInput, CHILD_INPUT_SCHEMA};
use crypto_lib::cryptanalysis::ecbench::spec::{plan, Spec};
use serde_json::{json, Value};

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn need(args: &[String], name: &str) -> String {
    arg(args, name).unwrap_or_else(|| panic!("missing {name}"))
}

fn splitmix(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn inputs(args: &[String]) {
    let spec = std::fs::read_to_string(need(args, "--spec")).unwrap();
    let count: usize = need(args, "--count").parse().unwrap();
    let out = PathBuf::from(need(args, "--out"));
    std::fs::create_dir_all(&out).unwrap();
    let p = plan(Spec::from_json(&spec).unwrap()).unwrap();
    for (i, ex) in p
        .executions
        .iter()
        .filter(|e| !e.warmup)
        .take(count)
        .enumerate()
    {
        let w = &p.workloads[ex.workload];
        let arm = &p.arms[ex.arm];
        let input = ChildInput {
            schema: CHILD_INPUT_SCHEMA.into(),
            curve: w.curve_spec.for_child(p.instance(ex.workload)),
            target_seed: w.target_seed,
            target_index: w.target_index,
            target_kind: w.kind(),
            expected_workload_id: w.workload_id.clone(),
            method: MethodSpec {
                id: arm.method.id.clone(),
                params: arm.method.params.clone(),
            },
            expected_method_id: arm.method.method_id.clone(),
            algorithm_seed: ex.algorithm_seed,
        };
        let path = out.join(format!("{i:03}-{}.json", arm.name));
        std::fs::write(&path, serde_json::to_string(&input).unwrap()).unwrap();
    }
}

/// The part of a child's output that must not depend on the binary: what
/// `ecbench verify` compares on a replay (each phase's wall time is
/// timing, not a count).
fn solve_view(out: &Value) -> Value {
    let r = &out["report"];
    let mut phases = r["phases"].clone();
    for p in phases.as_array_mut().into_iter().flatten() {
        if let Some(p) = p.as_object_mut() {
            p.remove("wall_ns");
        }
    }
    json!({
        "error": out["error"],
        "recovered": r["recovered"],
        "total_gae": r["total_gae"],
        "phases": phases,
        "counters": r["counters"],
        "unpriced": r["unpriced"],
    })
}

fn run_child(bin: &Path, input: &[u8]) -> (u64, Value) {
    let t = Instant::now();
    let mut child = Command::new(bin)
        .arg("exec")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    let out = child.wait_with_output().unwrap();
    let ns = t.elapsed().as_nanos() as u64;
    assert!(out.status.success(), "{} exec failed", bin.display());
    (ns, serde_json::from_slice(&out.stdout).unwrap())
}

fn median(v: &mut [u64]) -> f64 {
    v.sort_unstable();
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2] as f64
    } else {
        (v[n / 2 - 1] as f64 + v[n / 2] as f64) / 2.0
    }
}

fn time(args: &[String]) {
    let dir = PathBuf::from(need(args, "--inputs"));
    let rounds: usize = need(args, "--rounds").parse().unwrap();
    let mut rng: u64 = need(args, "--seed").parse().unwrap();
    let out_path = need(args, "--out");
    let bins: Vec<(String, PathBuf)> = args
        .windows(2)
        .filter(|w| w[0] == "--bin")
        .map(|w| {
            let (label, path) = w[1].split_once('=').expect("--bin LABEL=PATH");
            (label.to_string(), PathBuf::from(path))
        })
        .collect();
    assert!(bins.len() >= 2, "at least two --bin");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    let inputs: Vec<Vec<u8>> = files.iter().map(|f| std::fs::read(f).unwrap()).collect();

    // One untimed pass: every binary agrees on every solve, and the page
    // cache holds every binary before the clock starts.
    for (i, input) in inputs.iter().enumerate() {
        let first = solve_view(&run_child(&bins[0].1, input).1);
        for (label, path) in &bins[1..] {
            let v = solve_view(&run_child(path, input).1);
            assert_eq!(
                v,
                first,
                "{label} disagrees with {} on {}",
                bins[0].0,
                files[i].display()
            );
        }
    }

    // samples[b][round] = wall times of every input in that round.
    let mut samples = vec![vec![Vec::new(); rounds]; bins.len()];
    for round in 0..rounds {
        let mut order: Vec<usize> = (0..bins.len()).collect();
        if round % 2 == 1 {
            order.reverse();
        }
        for input in &inputs {
            for i in (1..order.len()).rev() {
                let j = (splitmix(&mut rng) % (i as u64 + 1)) as usize;
                order.swap(i, j);
            }
            for &b in &order {
                let (ns, _) = run_child(&bins[b].1, input);
                samples[b][round].push(ns);
            }
        }
    }

    // Bootstrap over rounds: resample whole rounds, the same draw for every
    // binary, so the intervals keep the pairing.  Summarised over every
    // input, and over each input alone.
    let samples = &samples;
    let reps = 2000;
    let n_bins = bins.len();
    let ci = |v: &mut Vec<f64>| -> [f64; 2] {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        [v[v.len() * 25 / 1000], v[v.len() * 975 / 1000 - 1]]
    };
    let mut summarise = |sel: &[usize]| -> Value {
        let take = |b: usize, picked: &[usize]| -> Vec<u64> {
            picked
                .iter()
                .flat_map(|&r| sel.iter().map(move |&i| samples[b][r][i]))
                .collect()
        };
        let every: Vec<usize> = (0..rounds).collect();
        let med: Vec<f64> = (0..n_bins).map(|b| median(&mut take(b, &every))).collect();
        let mut boot = vec![Vec::with_capacity(reps); n_bins];
        for _ in 0..reps {
            let pick: Vec<usize> = (0..rounds)
                .map(|_| (splitmix(&mut rng) % rounds as u64) as usize)
                .collect();
            for (b, out) in boot.iter_mut().enumerate() {
                out.push(median(&mut take(b, &pick)));
            }
        }
        let arms: Vec<Value> = (0..n_bins)
            .map(|b| {
                let ratios: serde_json::Map<String, Value> = (0..n_bins)
                    .filter(|&a| a != b)
                    .map(|a| {
                        let mut r: Vec<f64> = (0..reps).map(|k| boot[b][k] / boot[a][k]).collect();
                        (
                            bins[a].0.clone(),
                            json!({"ratio": med[b] / med[a], "ci95": ci(&mut r)}),
                        )
                    })
                    .collect();
                json!({
                    "label": bins[b].0,
                    "n": take(b, &every).len(),
                    "median_ns": med[b],
                    "median_ci95_ns": ci(&mut boot[b].clone()),
                    "ratio_to": ratios,
                })
            })
            .collect();
        json!(arms)
    };
    let all: Vec<usize> = (0..inputs.len()).collect();
    let pooled = summarise(&all);
    let by_input: serde_json::Map<String, Value> = files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            (
                f.file_name().unwrap().to_string_lossy().into_owned(),
                summarise(&[i]),
            )
        })
        .collect();
    let host = Command::new("uname")
        .arg("-a")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let binaries: Vec<Value> = bins
        .iter()
        .enumerate()
        .map(|(b, (label, path))| {
            json!({
                "label": label,
                "binary": path.display().to_string(),
                "binary_sha256": crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex(
                    &std::fs::read(path).unwrap()),
                "samples_ns_by_round": samples[b],
            })
        })
        .collect();
    let doc = json!({
        "schema": "ecbench.child_overhead/v1",
        "inputs": files.iter().map(|f| f.file_name().unwrap().to_string_lossy().into_owned()).collect::<Vec<_>>(),
        "rounds": rounds,
        "bootstrap": {"unit": "round", "reps": reps, "interval": "percentile 2.5-97.5"},
        "solves_identical_across_binaries": true,
        "host": host,
        "pooled": pooled,
        "by_input": by_input,
        "binaries": binaries,
    });
    std::fs::write(
        &out_path,
        serde_json::to_string_pretty(&doc).unwrap() + "\n",
    )
    .unwrap();
    for arm in doc["pooled"].as_array().unwrap() {
        println!(
            "{}: median {:.3} ms",
            arm["label"].as_str().unwrap(),
            arm["median_ns"].as_f64().unwrap() / 1e6
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("inputs") => inputs(&args),
        Some("time") => time(&args),
        _ => panic!("usage: inputs … | time …"),
    }
}
