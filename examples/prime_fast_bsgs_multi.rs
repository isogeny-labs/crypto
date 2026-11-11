//! Measured multi-target baby-step giant-step on the native engine (negation map, batched giant steps
//! across targets), for the same planted targets as `prime_fast_multi`.  Replaces the assumed
//! sqrt(T) multi-target rho reference.  Baby table: x(jG) -> j for 1 <= j <= B, covering +-j;
//! giant step M = 2B + 1; all T targets step together with one batched inversion per round.
//! usage: prime_fast_bsgs_multi <out.jsonl> <bits> <T> <instances>
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let t_count: usize = a[3].parse().unwrap();
    let inst: u64 = a[4].parse().unwrap();
    let tuned = a.get(5).is_some_and(|s| s == "tuned");
    let mut out = std::fs::File::create(&a[1]).unwrap();
    for res in [3u64, 1] {
        let Some((c, g)) = find_a3_curve(bits, res) else {
            continue;
        };
        for ins in 0..inst {
            let mut rng = StdRng::seed_from_u64(0xC0DE ^ ins ^ c.n ^ t_count as u64);
            let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..c.n)).collect();
            let qs: Vec<Pt> = ks.iter().map(|&k| c.mul(Some(g), k).unwrap()).collect();
            let t0 = Instant::now();
            let b = (((t_count as f64) * (c.n as f64) / 4.0).sqrt().ceil() as u64).max(16);
            let m = 2 * b + 1;
            // baby steps: tuned = batched adds over lanes + open-addressing table; else HashMap
            let mut table: HashMap<u64, u32> = HashMap::new();
            let cap = ((2 * b) as usize).next_power_of_two().max(16);
            let shift = 64 - cap.trailing_zeros();
            let mut open: Vec<(u64, u32)> = if tuned { vec![(0, 0); cap] } else { Vec::new() };
            if tuned {
                let lanes_n = 1024usize.min(b as usize);
                let mut lanes: Vec<Option<Pt>> = Vec::with_capacity(lanes_n);
                let mut cur = Some(g);
                for _ in 0..lanes_n {
                    lanes.push(cur);
                    cur = c.add(cur, Some(g));
                }
                let step: Vec<Option<Pt>> = vec![c.mul(Some(g), lanes_n as u64); lanes_n];
                let mut scratch0 = BatchAddScratch::default();
                let mut base = 0u64;
                'outer: loop {
                    for (i, lane) in lanes.iter().enumerate() {
                        let j = base + i as u64 + 1;
                        if j > b {
                            break 'outer;
                        }
                        if let Some(pt) = lane {
                            let mut h =
                                (pt.x.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> shift) as usize;
                            while open[h].1 != 0 {
                                h = (h + 1) & (cap - 1);
                            }
                            open[h] = (pt.x, j as u32);
                        }
                    }
                    c.add_many_batched(&mut lanes, &step, &mut scratch0);
                    base += lanes_n as u64;
                }
            } else {
                table.reserve(b as usize);
                let mut cur = Some(g);
                for j in 1..=b {
                    if let Some(p) = cur {
                        table.insert(p.x, j as u32);
                    }
                    cur = c.add(cur, Some(g));
                }
            }
            let mg = c.mul(Some(g), m).unwrap();
            let neg_mg = c.neg(mg);
            let mut lanes: Vec<Option<Pt>> = qs.iter().map(|&q| Some(q)).collect();
            let mut found: Vec<Option<u64>> = vec![None; t_count];
            let mut remaining = t_count;
            let mut scratch = BatchAddScratch::default();
            let rounds = c.n / m + 2;
            let mut round = 0u64;
            // collect lookups of the current lanes, then step all unfinished lanes
            loop {
                for (t, lane) in lanes.iter().enumerate() {
                    if found[t].is_some() {
                        continue;
                    }
                    if let Some(p) = lane {
                        let hit = if tuned {
                            let mut h = (p.x.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> shift) as usize;
                            let mut r = None;
                            while open[h].1 != 0 {
                                if open[h].0 == p.x {
                                    r = Some(open[h].1);
                                    break;
                                }
                                h = (h + 1) & (cap - 1);
                            }
                            r
                        } else {
                            table.get(&p.x).copied()
                        };
                        if let Some(j) = hit {
                            // lane = Q - i*M*G = +-jG
                            let jg = c.mul(Some(g), j as u64).unwrap();
                            let sign_plus = jg.y == p.y;
                            let base = round * m;
                            // Q = i*M*G + (+-j)G
                            let k = if sign_plus {
                                base + j as u64
                            } else {
                                base + c.n - j as u64
                            };
                            let k = k % c.n;
                            if c.mul(Some(g), k) == Some(qs[t]) {
                                found[t] = Some(k);
                                remaining -= 1;
                            }
                        }
                    } else {
                        // lane hit infinity: Q = i*M*G exactly
                        let k = (round * m) % c.n;
                        if c.mul(Some(g), k) == Some(qs[t]) {
                            found[t] = Some(k);
                            remaining -= 1;
                        }
                    }
                }
                if remaining == 0 || round > rounds {
                    break;
                }
                let rights: Vec<Option<Pt>> = vec![Some(neg_mg); t_count];
                c.add_many_batched(&mut lanes, &rights, &mut scratch);
                round += 1;
            }
            let ms = t0.elapsed().as_secs_f64() * 1e3;
            let ok = found
                .iter()
                .zip(&ks)
                .filter(|(f, k)| **f == Some(**k))
                .count();
            let row = json!({"bits":bits,"res":res,"n":c.n,"tuned":tuned,"T":t_count,"inst":ins,"B":b,"rounds":round,
                "ok":ok,"bsgs_total_ms":ms,"bsgs_per_target_ms":ms/t_count as f64});
            println!("{row}");
            writeln!(out, "{row}").unwrap();
        }
    }
}
