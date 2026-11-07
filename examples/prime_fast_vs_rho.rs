//! Scaling probe on the native engine: prime_fast ic_solve (summands 3, frozen c = 0.5 and neighbours) vs
//! prime_fast rho_parallel (1 thread, negation).  Wall time per solve, medians over targets.
//! usage: prime_fast_vs_rho <out.json> <targets> <bits,bits,...>
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let nt: u64 = a[2].parse().unwrap();
    let mut rows = Vec::new();
    for bits in a[3].split(',').map(|s| s.parse::<u32>().unwrap()) {
        for res in [3u64, 1] {
            let Some((c, g)) = find_a3_curve(bits, res) else {
                continue;
            };
            let mut ic: Vec<(f64, Vec<f64>)> = Vec::new();
            let cs = [0.25f64, 0.5, 1.0];
            let mut icv = vec![Vec::new(); cs.len()];
            let mut rho = Vec::new();
            let mut rho_steps = Vec::new();
            for t in 0..nt {
                let mut rng = StdRng::seed_from_u64(0xBEEF ^ t ^ c.n);
                let k = rng.gen_range(2..c.n);
                let q = c.mul(Some(g), k).unwrap();
                for (i, &cc) in cs.iter().enumerate() {
                    let fb = ((cc * (c.f.p as f64).cbrt()).round() as usize).max(8);
                    let t0 = Instant::now();
                    let r = ic_solve(&c, g, q, fb, 6, 3, u64::MAX, t);
                    let ms = t0.elapsed().as_secs_f64() * 1e3;
                    if matches!(r, Some((x, _)) if x == k) {
                        icv[i].push(ms);
                    }
                }
                let mut cfg = RhoConfig::for_bits(bits, true);
                cfg.threads = 1;
                cfg.seed = t + 1;
                let t0 = Instant::now();
                let r = rho_parallel(&c, g, q, &cfg);
                let ms = t0.elapsed().as_secs_f64() * 1e3;
                if let Some(r) = r {
                    if r.log == k {
                        rho.push(ms);
                        rho_steps.push(r.total_steps as f64);
                    }
                }
            }
            let _ = &mut ic;
            let row = json!({"bits":bits,"res":res,"p":c.f.p,"n":c.n,"targets":nt,
                "ic_c":cs,"ic_ms_median":icv.iter().map(|v| if v.is_empty(){-1.0}else{med(v.clone())}).collect::<Vec<_>>(),
                "ic_ok":icv.iter().map(|v| v.len()).collect::<Vec<_>>(),
                "rho_ms_median": if rho.is_empty(){-1.0}else{med(rho.clone())}, "rho_ok":rho.len(),
                "rho_steps_median": if rho_steps.is_empty(){-1.0}else{med(rho_steps.clone())}});
            println!("{row}");
            rows.push(row);
        }
    }
    std::fs::write(&a[1], serde_json::to_string_pretty(&rows).unwrap()).unwrap();
}
