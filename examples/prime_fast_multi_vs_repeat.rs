//! Removes the extrapolation from the multi-target claim: really runs `ic_solve` (summands 3, c = 0.5)
//! once per target for T targets and compares the total with `ic_multi_solve` (table c = 2.0, sweep c = 1.0)
//! on the identical planted targets.  usage: prime_fast_multi_vs_repeat <out.jsonl> <bits> <T> <instances>
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::io::Write;
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let t_count: usize = a[3].parse().unwrap();
    let inst: u64 = a[4].parse().unwrap();
    let mut out = std::fs::File::create(&a[1]).unwrap();
    for res in [3u64, 1] {
        let Some((c, g)) = find_a3_curve(bits, res) else {
            continue;
        };
        for ins in 0..inst {
            let mut rng = StdRng::seed_from_u64(0xD00D ^ ins ^ c.n ^ t_count as u64);
            let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..c.n)).collect();
            let qs: Vec<Pt> = ks.iter().map(|&k| c.mul(Some(g), k).unwrap()).collect();
            let p = c.f.p as f64;
            let fb_rep = ((0.5 * p.cbrt()).round() as usize).max(8);
            let t0 = Instant::now();
            let mut ok_rep = 0;
            for (t, (&q, &k)) in qs.iter().zip(&ks).enumerate() {
                if matches!(ic_solve(&c, g, q, fb_rep, 6, 3, u64::MAX, t as u64), Some((x, _)) if x == k)
                {
                    ok_rep += 1;
                }
            }
            let rep_ms = t0.elapsed().as_secs_f64() * 1e3;
            let mut arms = serde_json::Map::new();
            for (name, cc, table) in [("sweep", 1.0f64, false), ("table", 2.0f64, true)] {
                let fb = ((cc * p.cbrt()).round() as usize).max(8);
                let t1 = Instant::now();
                let r = ic_multi_solve(&c, g, &qs, fb, 6, 5_000_000, ins + 1, table);
                let ms = t1.elapsed().as_secs_f64() * 1e3;
                let ok = r
                    .map(|(l, _)| l.iter().zip(&ks).filter(|(x, k)| **x == Some(**k)).count())
                    .unwrap_or(0);
                arms.insert(name.into(), json!({"ms": ms, "ok": ok}));
            }
            let row = json!({"bits":bits,"res":res,"n":c.n,"T":t_count,"inst":ins,
                "repeat_ic_solve":{"ms":rep_ms,"ok":ok_rep},"multi":arms});
            println!("{row}");
            writeln!(out, "{row}").unwrap();
        }
    }
}
