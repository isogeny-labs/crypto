//! **E41 Stage 0 (counts only): the order-24 walk's step shortfall, by cause.**
//!
//! Part `records`: E34's stored Stage R walks (`experiments/38_twist24_cheap_rep/stage_r_p*.json`,
//! schoolbook, the cheap arms; their steps equal the built arms') against the ideal
//! `√(π(n−1)/(2m))`.  Part `run`: E34's walk with [`rho_j0_instrumented`] on fresh seeds
//! (`41000 + k`), the cheap order-6 and order-24 arms, schoolbook, serial.
//!
//! ```text
//! e41_stage0 --part records --json experiments/48_twist24_shortfall/stage0_records.json
//! e41_stage0 --part run --walks 64 --json experiments/48_twist24_shortfall/stage0_run.json
//! ```

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::quartic_folds::{construct_twist24, run_e29_kill};
use crypto_lib::cryptanalysis::twist24_rep::RepArm;
use crypto_lib::cryptanalysis::twist24_shortfall::{rho_j0_instrumented, Shortfall, CYCLE_BINS};
use serde_json::{json, Value};

const PS: [u64; 8] = [241, 337, 541, 601, 733, 769, 997, 1069];
const E34: &str = "experiments/38_twist24_cheap_rep";

/// Mean and standard error.
fn mean_se(xs: &[f64]) -> (f64, f64) {
    let k = xs.len() as f64;
    let m = xs.iter().sum::<f64>() / k;
    let v = xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (k - 1.0).max(1.0);
    (m, (v / k).sqrt())
}

fn ideal(n: u64, m: u64) -> f64 {
    (std::f64::consts::PI * ((n - 1) / m) as f64 / 2.0).sqrt()
}

fn records() -> Value {
    let mut per_arm: Vec<(String, Vec<f64>)> = vec![
        ("Order6Cheap".into(), vec![]),
        ("Order24Cheap".into(), vec![]),
    ];
    let mut curves = Vec::new();
    let (mut pool6, mut pool24) = (0.0, 0.0);
    for &p in &PS {
        let path = format!("{E34}/stage_r_p{p}.json");
        let v: Value = serde_json::from_str(&fs::read_to_string(&path).expect(&path)).expect(&path);
        let c = &v[0];
        let n = c["n"].as_u64().unwrap();
        let mut row = json!({"p": p, "n": n});
        for (arm, all) in per_arm.iter_mut() {
            let m = if arm.starts_with("Order24") { 24 } else { 6 };
            let id = ideal(n, m);
            let xs: Vec<f64> = c["walks"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|w| w["arm"] == arm.as_str() && w["karatsuba"] == false)
                .map(|w| w["run"]["steps"].as_u64().unwrap() as f64 / id)
                .collect();
            let esc: f64 = c["walks"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|w| w["arm"] == arm.as_str() && w["karatsuba"] == false)
                .map(|w| w["run"]["cycle_escapes"].as_u64().unwrap() as f64)
                .sum::<f64>()
                / xs.len() as f64;
            let (mu, se) = mean_se(&xs);
            let steps_sum: f64 = xs.iter().sum::<f64>() * id;
            if m == 6 {
                pool6 += steps_sum / (n as f64).sqrt();
            } else {
                pool24 += steps_sum / (n as f64).sqrt();
            }
            println!("e41 S0 records p={p} {arm} walks {} steps/ideal {mu:.4} ± {se:.4} escapes/walk {esc:.2}", xs.len());
            row[arm.as_str()] = json!({"walks": xs.len(), "ideal": id, "steps_over_ideal": mu, "se": se, "escapes_per_walk": esc});
            all.extend(xs);
        }
        curves.push(row);
    }
    let mut pooled = json!({});
    for (arm, xs) in &per_arm {
        let (mu, se) = mean_se(xs);
        println!(
            "e41 S0 records pooled {arm} walks {} steps/ideal {mu:.4} ± {se:.4}",
            xs.len()
        );
        pooled[arm.as_str()] = json!({"walks": xs.len(), "steps_over_ideal": mu, "se": se});
    }
    println!(
        "e41 S0 records steps ratio order6/order24 pooled per sqrt(n) {:.4}",
        pool6 / pool24
    );
    pooled["steps_ratio_6_over_24_per_sqrt_n"] = json!(pool6 / pool24);
    json!({"source": E34, "arithmetic": "schoolbook", "curves": curves, "pooled": pooled})
}

fn summarize(label: &str, sfs: &[(u64, Shortfall)]) -> Value {
    let k = sfs.len() as f64;
    let xs: Vec<f64> = sfs.iter().map(|(s, f)| *s as f64 / f.ideal_steps).collect();
    let stored: Vec<f64> = sfs
        .iter()
        .map(|(_, f)| f.stored as f64 / f.ideal_steps)
        .collect();
    let (mu, se) = mean_se(&xs);
    let (smu, sse) = mean_se(&stored);
    let tot_steps: f64 = sfs.iter().map(|(s, _)| *s as f64).sum();
    let tot_ideal: f64 = sfs.iter().map(|(_, f)| f.ideal_steps).sum();
    let red: f64 = sfs.iter().map(|(_, f)| f.redirects as f64).sum::<f64>();
    let fr: f64 = sfs.iter().map(|(_, f)| f.fruitless as f64).sum::<f64>();
    let ties: f64 = sfs.iter().map(|(_, f)| f.tie_inputs as f64).sum::<f64>();
    let rehit: f64 = sfs.iter().map(|(_, f)| f.escape_rehits as f64).sum::<f64>();
    let mut lens = [0u64; CYCLE_BINS];
    for (_, f) in sfs {
        for (i, v) in f.fruitless_len.iter().enumerate() {
            lens[i] += v;
        }
    }
    let lam: f64 = sfs
        .iter()
        .map(|(_, f)| f.final_cycle as f64 / f.ideal_steps)
        .sum::<f64>()
        / k;
    let mu_t: f64 = sfs
        .iter()
        .map(|(_, f)| f.final_tail as f64 / f.ideal_steps)
        .sum::<f64>()
        / k;
    println!(
        "e41 S0 {label} | walks {} | steps/ideal {mu:.4} ± {se:.4} (ratio of sums {:.4}) | stored/ideal {smu:.4} ± {sse:.4} | redirects/step {:.5} | fruitless per 1e6 steps {:.1} (lengths {:?}, escape rehits {}) | tie inputs {} | tail/ideal {mu_t:.3} cycle/ideal {lam:.3}",
        sfs.len(),
        tot_steps / tot_ideal,
        red / tot_steps,
        fr / tot_steps * 1e6,
        lens,
        rehit,
        ties
    );
    json!({
        "walks": sfs.len(), "steps_over_ideal": mu, "se": se, "ratio_of_sums": tot_steps / tot_ideal,
        "stored_over_ideal": smu, "stored_se": sse, "redirects_per_step": red / tot_steps,
        "fruitless": fr, "fruitless_per_step": fr / tot_steps, "fruitless_len": lens,
        "escape_rehits": rehit, "tie_inputs": ties, "tail_over_ideal": mu_t, "cycle_over_ideal": lam,
        "excess_from_fruitless": (fr + k) / tot_ideal,
    })
}

fn run(walks: u64, sizes: &[u64], json_path: Option<&str>) -> Value {
    let arms = [RepArm::Order6Cheap, RepArm::Order24Cheap];
    let mut all: Vec<Vec<(u64, Shortfall)>> = vec![vec![], vec![]];
    let mut curves = Vec::new();
    for &p in sizes {
        let kill = run_e29_kill(p, 1).expect("E18's curve");
        let b0 = kill.canonical_b0.expect("a canonical model");
        let inst = construct_twist24(p, b0, 1).expect("E29's canonical model");
        let mut row = json!({"p": p, "b0": b0, "n": inst.core.n});
        let mut walks_out = Vec::new();
        for (ai, &arm) in arms.iter().enumerate() {
            let mut here = Vec::new();
            for k in 0..walks {
                let (w, sf) = rho_j0_instrumented(&inst, arm, 41000 + k, false);
                assert!(w.run.correct, "p={p} {arm:?} seed {} wrong log", 41000 + k);
                walks_out.push(json!({"arm": arm, "seed": 41000 + k, "steps": w.run.steps, "correct": w.run.correct,
                    "fp_muls": w.run.fp_muls, "native": w.ops.total(), "shortfall": sf}));
                here.push((w.run.steps, sf));
            }
            row[format!("{arm:?}")] = summarize(&format!("p={p} {arm:?}"), &here);
            all[ai].extend(here);
        }
        row["walks"] = json!(walks_out);
        curves.push(row);
        if let Some(path) = json_path {
            fs::write(
                path,
                serde_json::to_string(&json!({"curves": curves})).unwrap(),
            )
            .expect("write");
        }
    }
    let pooled = json!({
        "Order6Cheap": summarize("pooled Order6Cheap", &all[0]),
        "Order24Cheap": summarize("pooled Order24Cheap", &all[1]),
    });
    json!({"seeds": format!("41000..{}", 41000 + walks), "arithmetic": "schoolbook", "curves": curves, "pooled": pooled})
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (mut partname, mut walks, mut json_path) = ("records".to_string(), 64u64, None::<String>);
    let mut sizes = PS.to_vec();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--part" => partname = args[i + 1].clone(),
            "--walks" => walks = args[i + 1].parse().unwrap(),
            "--json" => json_path = Some(args[i + 1].clone()),
            "--sizes" => sizes = args[i + 1].split(',').map(|v| v.parse().unwrap()).collect(),
            other => panic!("unknown argument {other}"),
        }
        i += 2;
    }
    let out = match partname.as_str() {
        "records" => records(),
        "run" => run(walks, &sizes, json_path.as_deref()),
        other => panic!("unknown part {other}"),
    };
    if let Some(path) = json_path {
        fs::write(path, serde_json::to_string_pretty(&out).unwrap()).expect("write");
    }
}
