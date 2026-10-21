// N4 v2 driver: instance list, IC arms (B0 frozen from N4; S/L0/L1/L; D/DL1/DL), plain and
// negation-map rho, summary. Build: rustc --edition 2021 -O v2.rs
#![allow(dead_code, unused_imports, unused_variables, unused_mut, static_mut_refs)]

#[path = "../quartic_n4_20261019/common.rs"]
mod common;
#[path = "../quartic_n4_20261019/ghs.rs"]
mod ghs;
#[path = "../quartic_n4_20261019/ic.rs"]
mod ic;
#[path = "../quartic_n4_20261019/solver.rs"]
mod solver;
#[path = "../../src/cryptanalysis/plane_quartic_pic.rs"]
mod pic;
mod ic2;
mod la2;
mod rho2;
mod solver2;
mod summ2;

use common::*;
use ghs::*;
use solver2::{Cfg, Fam, Var};

extern "C" {
    fn getloadavg(loadavg: *mut f64, nelem: i32) -> i32;
}
fn load1() -> f64 {
    let mut l = [0f64; 3];
    unsafe {
        getloadavg(l.as_mut_ptr(), 3);
    }
    l[0]
}

/// N4's ratios(): measured inv/mul, lmul/mul (min of 5 repetitions).
fn ratios(inst: &Inst) -> (f64, f64) {
    use std::hint::black_box;
    use std::time::Instant;
    let q = inst.q;
    let ell = inst.ell;
    let mut best = [f64::MAX; 3];
    for _ in 0..5 {
        let n = 2_000_000u64;
        let t = Instant::now();
        let mut x = 12345 % q;
        let y = 54321 % q;
        for _ in 0..n {
            x = black_box(x * y % q);
            x ^= 1;
        }
        black_box(x);
        best[0] = best[0].min(t.elapsed().as_secs_f64() / n as f64);
        let n2 = 200_000u64;
        let t = Instant::now();
        let mut z = 3u64;
        for _ in 0..n2 {
            z = black_box(inv_mod_u64(z % (q - 1) + 1, q));
        }
        black_box(z);
        best[1] = best[1].min(t.elapsed().as_secs_f64() / n2 as f64);
        let t = Instant::now();
        let mut x = 12345 % ell;
        let y = 54321 % ell;
        for _ in 0..n {
            x = black_box(((x as u128 * y as u128) % ell as u128) as u64);
            x ^= 1;
        }
        black_box(x);
        best[2] = best[2].min(t.elapsed().as_secs_f64() / n as f64);
    }
    (best[1] / best[0], best[2] / best[0])
}

/// Robust timing ratios under contention (deviation from N4's ratios(), see README): 600 interleaved
/// triples of short blocks (20000 F_q mults, 2000 F_q inversions, 20000 mod-ell mults); the ratio is
/// formed within each triple and the median over triples is reported.
fn ratios2(inst: &Inst, triples: usize) -> (f64, f64, f64, f64) {
    use std::hint::black_box;
    use std::time::Instant;
    let q = inst.q;
    let ell = inst.ell;
    let mut ri = Vec::new();
    let mut rl = Vec::new();
    let mut tm = Vec::new();
    let (mut x, mut z, mut w) = (12345 % q, 3u64, 12345 % ell);
    let (y, yl) = (54321 % q, 54321 % ell);
    for _ in 0..triples {
        let t = Instant::now();
        for _ in 0..20000 {
            x = black_box(x * y % q);
            x ^= 1;
        }
        let a = t.elapsed().as_secs_f64() / 20000.0;
        let t = Instant::now();
        for _ in 0..2000 {
            z = black_box(inv_mod_u64(z % (q - 1) + 1, q));
        }
        let b = t.elapsed().as_secs_f64() / 2000.0;
        let t = Instant::now();
        for _ in 0..20000 {
            w = black_box(((w as u128 * yl as u128) % ell as u128) as u64);
            w ^= 1;
        }
        let c = t.elapsed().as_secs_f64() / 20000.0;
        ri.push(b / a);
        rl.push(c / a);
        tm.push(a);
    }
    black_box((x, z, w));
    let med = |v: &mut Vec<f64>| { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v[v.len() / 2] };
    let mut tm2 = tm.clone();
    (med(&mut ri), med(&mut rl), med(&mut tm2) * 1e9, tm.iter().cloned().fold(f64::MAX, f64::min) * 1e9)
}

fn calib_cmd(q: u64, seed: u64) {
    let inst = Inst::search(q, seed, 200000).expect("no instance");
    let (r_inv, r_l, ns_med, ns_min) = ratios2(&inst, 600);
    println!("{{\"rec\":\"calib\",\"q\":{},\"seed\":{},\"r_inv\":{:.4},\"r_lmul\":{:.4},\"mul_ns_median\":{:.3},\"mul_ns_min\":{:.3},\"load1\":{:.2}}}", q, seed, r_inv, r_l, ns_med, ns_min, load1());
}

fn flat_cnt(prefix: &str, c: &Cnt, out: &mut Vec<String>) {
    out.push(format!("\"{p}.mul\":{},\"{p}.inv\":{},\"{p}.add\":{},\"{p}.lmul\":{},\"{p}.ladd\":{},\"{p}.hash\":{},\"{p}.cmp\":{}", c.mul, c.inv, c.add, c.lmul, c.ladd, c.hash, c.cmp, p = prefix));
}

/// N4 unit preamble: instance, ratios, counted E0 addition, the planted k, and the rho targets.
struct Pre {
    inst: Inst,
    k: u64,
    rng: Rng,
}
fn preamble(q: u64, seed: u64, tag: &str, print: bool) -> Pre {
    let t0 = std::time::Instant::now();
    let inst = Inst::search(q, seed, 200000).expect("no instance");
    let search_s = t0.elapsed().as_secs_f64();
    let mut rng = Rng::new(seed ^ 0x1234);
    let (gm, gi) = {
        let p = inst.ec.rand_point(&mut rng);
        let p2 = inst.ec.rand_point(&mut rng);
        let b = snap();
        let mut acc = p;
        for _ in 0..100 {
            acc = inst.ec.add(&acc, &p2);
        }
        let d = snap().since(&b);
        (d.mul as f64 / 100.0, d.inv as f64 / 100.0)
    };
    if print {
        let (r_inv, r_l) = ratios(&inst);
        println!(
            "{{\"rec\":\"ratios\",\"tag\":\"{}\",\"q\":{},\"seed\":{},\"ell\":{},\"order\":{},\"r_inv\":{:.4},\"r_lmul\":{:.4},\"gae_mul\":{:.3},\"gae_inv\":{:.3},\"load1\":{:.2},\"search_s\":{:.3}}}",
            tag, q, seed, inst.ell, inst.order, r_inv, r_l, gm, gi, load1(), search_s
        );
    }
    let k = rng.below(inst.ell - 1) + 1;
    Pre { inst, k, rng }
}

fn ic_cmd(q: u64, seed: u64, arms: &str, c_s: f64, c_d: f64, gamma: f64, block: usize) {
    let pre = preamble(q, seed, arms, true);
    let inst = &pre.inst;
    let k = pre.k;
    let t_unit = std::time::Instant::now();
    for arm in arms.split(',') {
        match arm {
            "B0" => {
                // N4's unit loop for B0, verbatim
                let mut c = 3.0f64;
                let mut attempt = 0;
                loop {
                    attempt += 1;
                    let nfb = ((c * (q as f64).powf(0.6)).ceil() as usize).min(q as usize + 1);
                    let prm = solver::Params { nfb, rows_mult: 1.2, max_desc: 100000 };
                    let rep = solver::ic_solve(inst, &prm, k, seed * 100 + attempt, true);
                    let mut f: Vec<String> = Vec::new();
                    for (n, cc, w) in &rep.phases {
                        flat_cnt(n, cc, &mut f);
                        f.push(format!("\"{}.wall\":{:.4}", n, w));
                    }
                    for (k2, v) in &rep.stats {
                        f.push(format!("\"s.{}\":{}", k2, v));
                    }
                    println!(
                        "{{\"rec\":\"ic\",\"arm\":\"B0\",\"q\":{},\"seed\":{},\"attempt\":{},\"c\":{:.2},\"nfb\":{},\"ell\":{},\"k\":{},\"k_found\":{},\"ok\":{},\"fail\":\"{}\",{}}}",
                        q, seed, attempt, c, nfb, inst.ell, k, rep.k_found.map(|x| x.to_string()).unwrap_or("null".into()), rep.ok, rep.fail, f.join(",")
                    );
                    if rep.ok || attempt >= 6 {
                        break;
                    }
                    c += 0.4;
                }
            }
            "S" | "D" => {
                let fam = if arm == "S" { Fam::S } else { Fam::D };
                let names: Vec<(&str, Var)> = if fam == Fam::S {
                    vec![("S", Var::B0Lanczos), ("L0", Var::Pm1Lanczos), ("L1", Var::SgeScalar), ("L", Var::SgeBlock)]
                } else {
                    vec![("D", Var::B0Lanczos), ("DL1", Var::SgeScalar), ("DL", Var::SgeBlock)]
                };
                let (c0, step) = if fam == Fam::S { (c_s, 0.4) } else { (c_d, 0.25) };
                let mut pending: Vec<usize> = (0..names.len()).collect();
                let mut c = c0;
                let mut attempt = 0u64;
                while !pending.is_empty() && attempt < 6 {
                    attempt += 1;
                    let cfg = Cfg { fam, c, rows_mult: 1.2, gamma, block, max_desc: 100000 };
                    let vars: Vec<Var> = pending.iter().map(|&i| names[i].1).collect();
                    let reps = solver2::run_family(inst, &cfg, &vars, k, seed * 100 + attempt, true);
                    let mut still = Vec::new();
                    for (t, rep) in reps.iter().enumerate() {
                        let i = pending[t];
                        let mut f: Vec<String> = Vec::new();
                        for (n, cc, w) in &rep.phases {
                            flat_cnt(n, cc, &mut f);
                            f.push(format!("\"{}.wall\":{:.4}", n, w));
                        }
                        for (k2, v) in &rep.stats {
                            f.push(format!("\"s.{}\":{}", k2, v));
                        }
                        println!(
                            "{{\"rec\":\"ic\",\"arm\":\"{}\",\"q\":{},\"seed\":{},\"attempt\":{},\"c\":{:.2},\"ell\":{},\"k\":{},\"k_found\":{},\"ok\":{},\"fail\":\"{}\",{}}}",
                            names[i].0, q, seed, attempt, c, inst.ell, k, rep.k_found.map(|x| x.to_string()).unwrap_or("null".into()), rep.ok, rep.fail, f.join(",")
                        );
                        if !rep.ok {
                            still.push(i);
                        }
                    }
                    pending = still;
                    c += step;
                }
            }
            _ => panic!("unknown arm {}", arm),
        }
    }
    eprintln!("ic q={} seed={} arms={} wall={:.2}s", q, seed, arms, t_unit.elapsed().as_secs_f64());
}

fn rho_cmd(q: u64, seed: u64, i0: usize, i1: usize, which: &str) {
    let mut pre = preamble(q, seed, which, true);
    let inst = &pre.inst;
    for i in 0..i1 {
        let kk = pre.rng.below(inst.ell - 1) + 1;
        if i < i0 {
            continue;
        }
        for w in which.split(',') {
            let r = match w {
                "plain" => rho2::rho_plain(inst, kk, seed * 1000 + i as u64),
                "neg" => rho2::rho_neg(inst, kk, seed * 1000 + i as u64),
                _ => panic!("rho variant"),
            };
            let mut f = Vec::new();
            flat_cnt("c", &r.cnt, &mut f);
            println!(
                "{{\"rec\":\"rho\",\"arm\":\"{}\",\"q\":{},\"seed\":{},\"i\":{},\"ell\":{},\"ok\":{},\"steps\":{},\"adds\":{},\"dps\":{},\"restarts\":{},\"lookahead\":{},\"cycles\":{},\"cycle_adds\":{},\"wall\":{:.4},\"load1\":{:.2},{}}}",
                w, q, seed, i, inst.ell, r.ok, r.steps, r.adds, r.dps, r.restarts, r.lookahead, r.cycles, r.cycle_adds, r.wall, load1(), f.join(",")
            );
        }
    }
}

fn instances_cmd(spec: &[String]) {
    // spec: q:nseeds ...
    for s in spec {
        let mut it = s.split(':');
        let q: u64 = it.next().unwrap().parse().unwrap();
        let ns: u64 = it.next().unwrap().parse().unwrap();
        for seed in 1..=ns {
            let t0 = std::time::Instant::now();
            let inst = Inst::search(q, seed, 200000).expect("no instance");
            println!(
                "{{\"q\":{},\"seed\":{},\"ell\":{},\"order\":{},\"tries\":{},\"f\":[{}],\"search_s\":{:.3}}}",
                q, seed, inst.ell, inst.order, inst.tries, inst.f.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","), t0.elapsed().as_secs_f64()
            );
        }
    }
}

fn qlist(kmin: u32, kmax: u32) {
    for k in kmin..=kmax {
        let mut q = (1u64 << k) - 1;
        while !(q % 12 == 7 && is_prime(q)) {
            q -= 1;
        }
        println!("{} {}", k, q);
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("ic") => ic_cmd(
            a[2].parse().unwrap(),
            a[3].parse().unwrap(),
            &a[4],
            a.get(5).map(|s| s.parse().unwrap()).unwrap_or(3.0),
            a.get(6).map(|s| s.parse().unwrap()).unwrap_or(2.0),
            a.get(7).map(|s| s.parse().unwrap()).unwrap_or(4.0),
            a.get(8).map(|s| s.parse().unwrap()).unwrap_or(4),
        ),
        Some("rho") => rho_cmd(a[2].parse().unwrap(), a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap(), &a[6]),
        Some("calib") => calib_cmd(a[2].parse().unwrap(), a[3].parse().unwrap()),
        Some("instances") => instances_cmd(&a[2..]),
        Some("qlist") => qlist(a[2].parse().unwrap(), a[3].parse().unwrap()),
        Some("summarize") => summ2::run(&a[2..]),
        Some("replaycmp") => {
            let same = summ2::replay_cmp(&a[2], &a[3]);
            std::process::exit(if same { 0 } else { 1 });
        }
        _ => eprintln!("usage: v2 ic <q> <seed> <B0,S,D> [cS cD gamma block] | rho <q> <seed> <i0> <i1> <plain,neg> | instances q:n.. | qlist kmin kmax | summarize ..."),
    }
}
