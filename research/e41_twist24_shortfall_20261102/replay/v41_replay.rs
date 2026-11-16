//! E41 validator's replay driver (not producer code).  Built as an example
//! inside the validator's clean export of b0e307d9b8, so it links exactly the
//! library code under test (`twist24_shortfall::rho_j0_e41`).  It emits, for
//! each selected walk, the record `examples/e41_run.rs` writes for it
//! (`record()`: label, jumps, walk, shortfall with `part_hist` cleared,
//! serialized through `serde_json::Value`), so the validator's own comparator
//! (`validate_e41.rs compare`) can compare it byte for byte with the committed
//! record.  It takes no decision and computes no statistic.
//!
//! Input lines (selection file, one walk each):
//!   school|kar <p> <label> <seed>    main stream, schoolbook | Karatsuba
//!   aa <p> O24AA <seed>              A/A stream, schoolbook
//!   ctl <p> O24F-broken <seed>       positive control F-slash, schoolbook
//!   e34 <p> <RepArm> <true|false> <seed>   E34 Stage R walk, fix off; emits the walk only
//!   s0 <p> <RepArm> <seed>           Stage 0 walk (fix off); emits {walk, shortfall} with part_hist kept
//! Output: the input line, a tab, the JSON text.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;

use crypto_lib::cryptanalysis::quartic_folds::{construct_twist24, run_e29_kill, Twist24Instance};
use crypto_lib::cryptanalysis::twist24_rep::RepArm;
use crypto_lib::cryptanalysis::twist24_shortfall::{rho_j0_e41, Jumps};
use serde_json::{json, Value};

fn instance(p: u64) -> Twist24Instance {
    let kill = run_e29_kill(p, 1).expect("E18's curve");
    let b0 = kill.canonical_b0.expect("a canonical model");
    construct_twist24(p, b0, 1).expect("E29's canonical model")
}

fn arm_of(name: &str) -> RepArm {
    match name {
        "Order6Built" => RepArm::Order6Built,
        "Order6Cheap" => RepArm::Order6Cheap,
        "Order24Built" => RepArm::Order24Built,
        "Order24Cheap" => RepArm::Order24Cheap,
        "Order24CheapMulLean" => RepArm::Order24CheapMulLean,
        other => panic!("unknown arm {other}"),
    }
}

fn label_of(label: &str) -> (RepArm, Jumps) {
    match label {
        "O6B" => (RepArm::Order6Built, Jumps::E34),
        "O6C" => (RepArm::Order6Cheap, Jumps::E34),
        "O24" | "O24AA" => (RepArm::Order24Cheap, Jumps::E34),
        "O24L" => (RepArm::Order24CheapMulLean, Jumps::E34),
        "O24F" => (RepArm::Order24Cheap, Jumps::PairSums),
        "O24F-broken" => (RepArm::Order24Cheap, Jumps::PairSumsBroken),
        other => panic!("unknown label {other}"),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let sel = fs::read_to_string(&args[0]).expect("selection file");
    let mut insts: HashMap<u64, Twist24Instance> = HashMap::new();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in sel.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.is_empty() || t[0].starts_with('#') {
            continue;
        }
        let p: u64 = t[1].parse().unwrap();
        let inst = insts.entry(p).or_insert_with(|| instance(p));
        let text = match t[0] {
            "school" | "kar" | "aa" | "ctl" => {
                let kar = t[0] == "kar";
                let (arm, jumps) = label_of(t[2]);
                let seed: u64 = t[3].parse().unwrap();
                let (w, mut sf) = rho_j0_e41(inst, arm, seed, kar, jumps);
                sf.part_hist.clear();
                let v = json!({"label": t[2], "jumps": jumps, "walk": w, "shortfall": sf});
                serde_json::to_string(&v).unwrap()
            }
            "e34" => {
                let arm = arm_of(t[2]);
                let kar = t[3] == "true";
                let seed: u64 = t[4].parse().unwrap();
                let (w, _) = rho_j0_e41(inst, arm, seed, kar, Jumps::E34);
                let v: Value = serde_json::to_value(&w).unwrap();
                serde_json::to_string(&v).unwrap()
            }
            "s0" => {
                let arm = arm_of(t[2]);
                let seed: u64 = t[3].parse().unwrap();
                let (w, sf) = rho_j0_e41(inst, arm, seed, false, Jumps::E34);
                let v = json!({"walk": w, "shortfall": sf});
                serde_json::to_string(&v).unwrap()
            }
            other => panic!("unknown run {other}"),
        };
        writeln!(out, "{line}\t{text}").unwrap();
        out.flush().unwrap();
    }
}
