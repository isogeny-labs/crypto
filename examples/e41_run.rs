//! **E41 (§9.52): the order-24 walk's step shortfall — the registered runs.**
//!
//! ```text
//! e41_run --stage identity --json experiments/48_twist24_shortfall/identity.json
//! e41_run --stage main --stream main --arith school --walks 128 --json experiments/48_twist24_shortfall/main_school.json
//! e41_run --stage main --stream aa --arith school --walks 128 --json experiments/48_twist24_shortfall/aa_school.json
//! e41_run --stage main --stream main --arith karatsuba --walks 32 --json experiments/48_twist24_shortfall/main_karatsuba.json
//! e41_run --stage control --json experiments/48_twist24_shortfall/control.json
//! ```
//! `identity`: with the fix off, every E34 Stage R record (`experiments/38_twist24_cheap_rep/stage_r_p*.json`,
//! 8 curves × 5 arms × 16 seeds × 2 arithmetics) is recomputed and compared as serialized JSON.
//! `main`: arms O6B, O6C, O24, O24L, O24F on the main stream (seeds `48000 + k`), O24 alone on the
//! A/A stream (`49000 + k`).  `control`: F̸ at `p = 241, 541`, seeds `48000`–`48015`.  Serial, one core.

use std::env;
use std::fs;

use crypto_lib::cryptanalysis::quartic_folds::{construct_twist24, run_e29_kill, Twist24Instance};
use crypto_lib::cryptanalysis::twist24_rep::RepArm;
use crypto_lib::cryptanalysis::twist24_shortfall::{rho_j0_e41, Jumps};
use serde_json::{json, Value};

const PS: [u64; 8] = [241, 337, 541, 601, 733, 769, 997, 1069];
const E34: &str = "experiments/38_twist24_cheap_rep";
const E34_ARMS: [RepArm; 5] = [
    RepArm::Order6Built,
    RepArm::Order6Cheap,
    RepArm::Order24Built,
    RepArm::Order24Cheap,
    RepArm::Order24CheapMulLean,
];

fn instance(p: u64) -> Twist24Instance {
    let kill = run_e29_kill(p, 1).expect("E18's curve");
    let b0 = kill.canonical_b0.expect("a canonical model");
    construct_twist24(p, b0, 1).expect("E29's canonical model")
}

fn record(
    inst: &Twist24Instance,
    label: &str,
    arm: RepArm,
    seed: u64,
    kar: bool,
    jumps: Jumps,
) -> Value {
    let (w, mut sf) = rho_j0_e41(inst, arm, seed, kar, jumps);
    sf.part_hist.clear();
    json!({"label": label, "jumps": jumps, "walk": w, "shortfall": sf})
}

fn identity() -> Value {
    let (mut compared, mut same_bytes, mut same_value) = (0u64, 0u64, 0u64);
    let mut diffs = Vec::new();
    for p in PS {
        let path = format!("{E34}/stage_r_p{p}.json");
        let stored: Value = serde_json::from_str(&fs::read_to_string(&path).expect(&path)).unwrap();
        let inst = instance(p);
        for w in stored[0]["walks"].as_array().unwrap() {
            let arm = *E34_ARMS
                .iter()
                .find(|a| json!(a) == w["arm"])
                .expect("an E34 arm");
            let kar = w["karatsuba"].as_bool().unwrap();
            let seed = w["run"]["seed"].as_u64().unwrap();
            let (mine, _) = rho_j0_e41(&inst, arm, seed, kar, Jumps::E34);
            let mine_v = serde_json::to_value(&mine).unwrap();
            compared += 1;
            let eq_v = mine_v == *w;
            let eq_b = serde_json::to_string(&mine_v).unwrap() == serde_json::to_string(w).unwrap();
            same_value += u64::from(eq_v);
            same_bytes += u64::from(eq_b);
            if !(eq_v && eq_b) {
                diffs.push(json!([p, arm, kar, seed]));
            }
        }
        println!("e41 identity p={p}: compared {compared}, byte-identical {same_bytes}");
    }
    json!({"compared": compared, "byte_identical": same_bytes, "value_identical": same_value, "differences": diffs,
           "source": E34})
}

fn main_stage(stream: &str, kar: bool, walks: u64, json_path: Option<&str>) -> Value {
    let arms: Vec<(&str, RepArm, Jumps)> = if stream == "main" {
        vec![
            ("O6B", RepArm::Order6Built, Jumps::E34),
            ("O6C", RepArm::Order6Cheap, Jumps::E34),
            ("O24", RepArm::Order24Cheap, Jumps::E34),
            ("O24L", RepArm::Order24CheapMulLean, Jumps::E34),
            ("O24F", RepArm::Order24Cheap, Jumps::PairSums),
        ]
    } else {
        vec![("O24AA", RepArm::Order24Cheap, Jumps::E34)]
    };
    let base = if stream == "main" { 48000 } else { 49000 };
    let mut curves = Vec::new();
    for p in PS {
        let inst = instance(p);
        let mut out = Vec::new();
        // Interleaved: seed-major, arm-minor.
        for k in 0..walks {
            for &(label, arm, jumps) in &arms {
                out.push(record(&inst, label, arm, base + k, kar, jumps));
            }
        }
        let wrong = out
            .iter()
            .filter(|r| r["walk"]["run"]["correct"] != true)
            .count();
        println!(
            "e41 main stream={stream} karatsuba={kar} p={p}: {} walks, wrong logs {wrong}",
            out.len()
        );
        curves.push(json!({"p": p, "b0": inst.b0, "n": inst.core.n, "walks": out}));
        if let Some(path) = json_path {
            fs::write(path, serde_json::to_string(&curves).unwrap()).expect("write");
        }
    }
    json!(curves)
}

fn control() -> Value {
    let mut curves = Vec::new();
    for p in [241u64, 541] {
        let inst = instance(p);
        let out: Vec<Value> = (0..16)
            .map(|k| {
                record(
                    &inst,
                    "O24F-broken",
                    RepArm::Order24Cheap,
                    48000 + k,
                    false,
                    Jumps::PairSumsBroken,
                )
            })
            .collect();
        let wrong = out
            .iter()
            .filter(|r| r["walk"]["run"]["correct"] != true)
            .count();
        println!("e41 control p={p}: 16 broken-fix walks, flagged wrong {wrong}");
        curves.push(json!({"p": p, "n": inst.core.n, "walks": out}));
    }
    json!(curves)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (mut stage, mut stream, mut kar, mut walks) =
        (String::new(), "main".to_string(), false, 128u64);
    let mut json_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--stage" => stage = args[i + 1].clone(),
            "--stream" => stream = args[i + 1].clone(),
            "--arith" => kar = args[i + 1] == "karatsuba",
            "--walks" => walks = args[i + 1].parse().unwrap(),
            "--json" => json_path = Some(args[i + 1].clone()),
            other => panic!("unknown argument {other}"),
        }
        i += 2;
    }
    let out = match stage.as_str() {
        "identity" => identity(),
        "main" => main_stage(&stream, kar, walks, json_path.as_deref()),
        "control" => control(),
        other => panic!("unknown stage {other}"),
    };
    if let Some(path) = json_path {
        fs::write(path, serde_json::to_string(&out).unwrap()).expect("write");
    }
}
