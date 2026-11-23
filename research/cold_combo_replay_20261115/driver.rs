//! One public target and one DL variant per process. No measured results here.
#![allow(dead_code)]
#[path = "../quartic_n4_20261019/common.rs"] mod common;
#[path = "../quartic_n4_20261019/ghs.rs"] mod ghs;
#[path = "../quartic_n4_20261019/ic.rs"] mod ic;
#[path = "../quartic_n4_20261019/solver.rs"] mod solver;
#[path = "../../src/cryptanalysis/plane_quartic_pic.rs"] mod pic;
#[cfg(not(combo))]
#[path = "../quartic_dl1_cd_sqrt_20261113/src/ic2.rs"] mod ic2;
#[cfg(combo)]
#[path = "../ic_gw_fasthash_20261114/src/ic2.rs"] mod ic2;
#[path = "../ic_gw_fasthash_20261114/src/la2.rs"] mod la2;
#[path = "solver_public.rs"] mod solver2;
use ghs::{EP, Inst};
use solver2::{Cfg, Fam, Var};

const PHASES: [&str; 7] = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"];

fn point_json(p: EP) -> String {
    match p {
        EP::Inf => "null".into(),
        EP::A(x, y) => format!("{{\"x\":{x:?},\"y\":{y:?}}}"),
    }
}

fn checked_target(inst: &Inst, coords: &[u64]) -> Result<EP, &'static str> {
    if coords.len() != 6 || coords.iter().any(|&x| x >= inst.q) {
        return Err("need six canonical target coordinates");
    }
    let p = EP::A([coords[0], coords[1], coords[2]], [coords[3], coords[4], coords[5]]);
    if !inst.ec.on(&p) { return Err("target is off curve"); }
    if inst.ec.mul(&p, inst.ell as u128) != EP::Inf { return Err("target is outside subgroup"); }
    Ok(p)
}

fn replay_vector(inst: &Inst, target: EP, k: u64) -> String {
    let EP::A(gx, gy) = inst.g else { panic!("infinite generator"); };
    let EP::A(tx, ty) = target else { panic!("infinite target"); };
    format!("[{},{},{:?},{:?},{:?},{gx:?},{gy:?},{tx:?},{ty:?},{},{}]\n",
        inst.q, inst.kf.c, inst.ec.a2, inst.ec.a4, inst.ec.a6, inst.ell, k)
}

fn write_replay(path: &str, text: &str) {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().write(true).create_new(true).open(path)
        .expect("replay destination must not exist");
    f.write_all(text.as_bytes()).expect("write replay input");
}

fn solve_public(inst: &Inst, target: &EP, seed: u64, replay_path: &str) -> bool {
    // Timed cold solve: all attempts, allocation, controls, final scalar check,
    // and destruction of per-attempt solver state. Input reconstruction is outside.
    let start_cpu = ic2::cpu_now();
    let start_wall = std::time::Instant::now();
    let mut phase_cpu = [0.0; 7];
    let mut attempts = 0;
    let mut answer = None;
    let mut reports = Vec::new();
    for attempt in 1..=6u64 {
        attempts = attempt;
        let cfg = Cfg { fam: Fam::D, c: 1.7 + 0.25 * (attempt - 1) as f64,
            rows_mult: 1.2, gamma: 4.0, block: 4, max_desc: 100000 };
        let mut reps = solver2::run_family(inst, &cfg, &[Var::SgeBlock], target,
            seed.checked_mul(100).unwrap().checked_add(attempt).unwrap(), true);
        assert_eq!(reps.len(), 1);
        let rep = reps.pop().unwrap();
        for (name, _, _, cpu) in &rep.phases {
            let i = PHASES.iter().position(|&p| p == name).expect("unknown phase");
            phase_cpu[i] += cpu;
        }
        assert!(rep.stats.iter().all(|(k, v)| k != "downstream_shared" || *v == 0.0));
        if rep.ok {
            let k = rep.k_found.expect("success without scalar");
            assert_eq!(inst.ec.mul(&inst.g, k as u128), *target);
            answer = Some(k);
            reports.push(rep);
            break;
        }
        reports.push(rep);
    }
    let total_cpu = ic2::cpu_now() - start_cpu;
    let total_wall = start_wall.elapsed().as_secs_f64();
    // External PARI replay uses exact integers, with no JSON/double conversion.
    write_replay(replay_path, &replay_vector(inst, *target, answer.unwrap_or(0)));
    // Retain every unsuccessful attempt and every counted field, outside timing.
    for (i, rep) in reports.iter().enumerate() {
        let mut fields = Vec::new();
        for (phase, cnt, wall, cpu) in &rep.phases {
            fields.push(format!("\"{phase}\":{{\"cpu\":{cpu:.9},\"wall\":{wall:.9},\"mul\":{},\"inv\":{},\"add\":{},\"lmul\":{},\"ladd\":{},\"hash\":{},\"cmp\":{}}}",
                cnt.mul, cnt.inv, cnt.add, cnt.lmul, cnt.ladd, cnt.hash, cnt.cmp));
        }
        let stats = rep.stats.iter().map(|(k,v)| format!("\"{k}\":{v}"))
            .collect::<Vec<_>>().join(",");
        println!("{{\"rec\":\"attempt\",\"attempt\":{},\"ok\":{},\"fail\":{:?},\"phases\":{{{}}},\"stats\":{{{stats}}}}}",
            i + 1, rep.ok, rep.fail, fields.join(","));
    }
    let phases: String = PHASES.iter().zip(phase_cpu).map(|(p, t)| format!("\"{p}\":{t:.9}"))
        .collect::<Vec<_>>().join(",");
    println!("{{\"rec\":\"cold_solve\",\"engine\":\"{}\",\"q\":{},\"seed\":{},\"target\":{},\"attempts\":{},\"ok\":{},\"k_found\":{},\"total_cpu\":{:.9},\"total_wall\":{:.9},\"phase_cpu\":{{{}}},\"unattributed_cpu\":{:.9},\"independent_replay\":false}}",
        if cfg!(combo) { "COMBO" } else { "REF" }, inst.q, seed, point_json(*target), attempts,
        answer.is_some(), answer.map_or("null".into(), |k| k.to_string()), total_cpu, total_wall,
        phases, total_cpu - phase_cpu.iter().sum::<f64>());
    answer.is_some()
}

fn prime(q: u64) -> bool {
    if q < 2 { return false; }
    let mut d = 2;
    while d * d <= q {
        if q % d == 0 { return false; }
        d += 1;
    }
    true
}

#[cfg(not(replay_selftest))]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 { panic!("fixture Q SEED LABEL [REPLAY_FILE] | solve Q SEED X0 X1 X2 Y0 Y1 Y2 REPLAY_FILE"); }
    let q: u64 = args[1].parse().unwrap();
    let seed: u64 = args[2].parse().unwrap();
    assert!(q >= 7 && q < (1u64 << 31) && q % 12 == 7 && prime(q));
    assert!(seed <= (u64::MAX - 6) / 100);
    let inst = Inst::search(q, seed, 200000).expect("no instance within bound");
    match args[0].as_str() {
        "fixture" => {
            assert!(args.len() == 4 || args.len() == 5);
            let label: u64 = args[3].parse().unwrap();
            assert!(label > 0 && label < inst.ell);
            let target = inst.ec.mul(&inst.g, label as u128);
            if args.len() == 5 { write_replay(&args[4], &replay_vector(&inst, target, label)); }
            println!("{{\"rec\":\"public_fixture\",\"q\":{q},\"seed\":{seed},\"extension_c\":{},\"a2\":{:?},\"a4\":{:?},\"a6\":{:?},\"ell\":{},\"generator\":{},\"target\":{}}}",
                inst.kf.c, inst.ec.a2, inst.ec.a4, inst.ec.a6, inst.ell, point_json(inst.g), point_json(target));
        }
        "solve" => {
            assert_eq!(args.len(), 10);
            assert!(!std::path::Path::new(&args[9]).exists(), "replay file exists");
            let coords: Vec<u64> = args[3..9].iter().map(|s| s.parse().unwrap()).collect();
            let target = checked_target(&inst, &coords).expect("invalid public target");
            if !solve_public(&inst, &target, seed, &args[9]) { std::process::exit(2); }
        }
        _ => panic!("unknown command"),
    }
}

#[cfg(replay_selftest)]
fn main() { tests::run(); }

#[cfg(replay_selftest)]
mod tests {
    use super::*;
    fn canonical_point_serialization() {
        assert_eq!(point_json(EP::Inf), "null");
        assert_eq!(point_json(EP::A([1,2,3],[4,5,6])), "{\"x\":[1, 2, 3],\"y\":[4, 5, 6]}");
    }
    fn public_target_boundary() {
        let inst = Inst::search(8191, 1, 200000).unwrap();
        let target = inst.ec.mul(&inst.g, 17);
        let EP::A(x, y) = target else { panic!("unexpected identity"); };
        let coords = [x[0], x[1], x[2], y[0], y[1], y[2]];
        assert_eq!(checked_target(&inst, &coords), Ok(target));
        assert!(checked_target(&inst, &[8191; 6]).is_err());
        assert!(checked_target(&inst, &coords[..5]).is_err());
        // Boundary checks only. No cost measurement or algorithm experiment.
        assert!(!prime(91));
        assert!(prime(8191));
    }
    pub fn run() {
        canonical_point_serialization();
        public_target_boundary();
        println!("development self-checks passed; no solver run");
    }
}
