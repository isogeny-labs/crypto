//! Multi-target IC on the native engine: shared factor-base logs + one relation per target, vs T x single
//! rho and vs an idealised multi-target rho (single / sqrt(T), favourable to rho).
//! usage: prime_fast_multi <out.jsonl> <bits> <T,T,...> <c,c,...> [instances]
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::io::Write;
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let ts: Vec<usize> = a[3].split(',').map(|s| s.parse().unwrap()).collect();
    let cs: Vec<f64> = a[4].split(',').map(|s| s.parse().unwrap()).collect();
    let inst: u64 = a.get(5).map(|s| s.parse().unwrap()).unwrap_or(1);
    let table = a.get(6).is_some_and(|s| s == "table");
    let mut out = std::fs::File::create(&a[1]).unwrap();
    for res in [3u64, 1] {
        let Some((c, g)) = find_a3_curve(bits, res) else {
            continue;
        };
        // rho single-target reference
        let mut rho_ms = Vec::new();
        for t in 0..10u64 {
            let mut rng = StdRng::seed_from_u64(0xFEED ^ t ^ c.n);
            let q = c.mul(Some(g), rng.gen_range(2..c.n)).unwrap();
            let mut cfg = RhoConfig::for_bits(bits, true);
            cfg.threads = 1;
            cfg.seed = t + 1;
            let t0 = Instant::now();
            if rho_parallel(&c, g, q, &cfg).is_some() {
                rho_ms.push(t0.elapsed().as_secs_f64() * 1e3);
            }
        }
        rho_ms.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let rho1 = rho_ms[rho_ms.len() / 2];
        // single-target ic_solve (summands 3, c = 0.5) reference, 5 targets
        let mut s3_ms = Vec::new();
        for t in 0..5u64 {
            let mut rng = StdRng::seed_from_u64(0xFACE ^ t ^ c.n);
            let k = rng.gen_range(2..c.n);
            let q = c.mul(Some(g), k).unwrap();
            let fbs = ((0.5 * (c.f.p as f64).cbrt()).round() as usize).max(8);
            let t0 = Instant::now();
            if matches!(ic_solve(&c, g, q, fbs, 6, 3, u64::MAX, t), Some((x, _)) if x == k) {
                s3_ms.push(t0.elapsed().as_secs_f64() * 1e3);
            }
        }
        s3_ms.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let s3_1 = s3_ms[s3_ms.len() / 2];
        for &t_count in &ts {
            for &cc in &cs {
                for ins in 0..inst {
                    let fb = ((cc * (c.f.p as f64).cbrt()).round() as usize).max(8);
                    let mut rng = StdRng::seed_from_u64(0xC0DE ^ ins ^ c.n ^ t_count as u64);
                    let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..c.n)).collect();
                    let qs: Vec<Pt> = ks.iter().map(|&k| c.mul(Some(g), k).unwrap()).collect();
                    let t0 = Instant::now();
                    let r = ic_multi_solve(&c, g, &qs, fb, 6, 5_000_000, ins + 1, table);
                    let wall = t0.elapsed().as_secs_f64() * 1e3;
                    let Some((logs, rep)) = r else {
                        writeln!(out, "{}", json!({"bits":bits,"res":res,"T":t_count,"c":cc,"fb":fb,"inst":ins,"failed":true})).unwrap();
                        continue;
                    };
                    let ok = logs
                        .iter()
                        .zip(&ks)
                        .filter(|(l, k)| **l == Some(**k))
                        .count();
                    let mut per = rep.per_target_ms.clone();
                    per.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    let row = json!({"bits":bits,"res":res,"n":c.n,"table":table,"T":t_count,"c":cc,"fb":fb,"inst":ins,
                        "ok":ok,"determined":rep.determined_logs,"setup_ms":rep.setup_ms,
                        "per_target_median_ms":per[per.len()/2],"total_ms":wall,"ic_per_target_ms":wall/t_count as f64,
                        "rho_single_ms":rho1,"s3_single_ms":s3_1,"s3_Tx_ms":s3_1*t_count as f64,"ratio_vs_s3_Tx":wall/(s3_1*t_count as f64),"rho_T_ms":rho1*t_count as f64,
                        "rho_ideal_multi_ms":rho1*(t_count as f64).sqrt(),
                        "ratio_vs_rho_Tx":wall/(rho1*t_count as f64),"ratio_vs_rho_ideal":wall/(rho1*(t_count as f64).sqrt())});
                    println!("{row}");
                    writeln!(out, "{row}").unwrap();
                }
            }
        }
    }
}
