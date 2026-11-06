//! SECONDARY, declared after the main protocol froze (red-team request relayed 2026-11-03): a bitop/v1-style *model* proxy.
//! Weights: field_add = 0.096, word op (lookups, rho native counters) = 0.043 F_p mults, mod-l LA op = 31.2 F_p mults (replacing the
//! record's 16 per op). Cover: unemitted F4/descent additions are not counted (lower bound). Usage: bitop <study dir> <pooled_ref: 126-run dir>
use serde_json::Value;
use std::{fs, path::Path};
const GAE: f64 = 331.0; const NB: usize = 20000;
struct Rng(u64);
impl Rng { fn next(&mut self) -> u64 { self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15); let mut z = self.0; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^ (z >> 31) }
    fn idx(&mut self, n: usize) -> usize { (((self.next() as u128) * (n as u128)) >> 64) as usize } }
fn rows(p: &Path) -> Vec<Value> { serde_json::from_str::<Value>(&fs::read_to_string(p).unwrap()).unwrap().as_array().unwrap().clone() }
fn f(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap() }
fn fb(v: &Value, k: &str) -> f64 { v["fb_line"].get(k).and_then(|x| x.as_f64()).unwrap_or(0.0) }
fn pct(v: &[f64], q: f64) -> f64 { let mut s = v.to_vec(); s.sort_by(|a, b| a.partial_cmp(b).unwrap()); s[((s.len() - 1) as f64 * q).round() as usize] }
fn cover(r: &Value) -> f64 {
    let c = f(r, "total_muls") - f(r, "la_muls") + 31.2 * f(r, "la_ops")
        + 0.096 * (f(r, "sieve_adds") + f(r, "enum_adds") + fb(r, "adds")) + 0.043 * (f(r, "sieve_lookups") + fb(r, "lookups"));
    c / (GAE * f(r, "l").sqrt())
}
fn rho(r: &Value) -> f64 {
    let o = &r["ops"];
    let nat: f64 = ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| f(o, k)).sum();
    (f(r, "muls_tuned") + 0.096 * f(o, "fp_adds") + 0.043 * nat) / (GAE * f(r, "l").sqrt())
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let st = Path::new(&a[1]);
    let primes = [1009u64, 1511, 1823];
    let mut rr: Vec<Vec<(u64, f64)>> = vec![]; // per prime: (seed, rho cost)
    for p in primes { let mut v = vec![]; for e in fs::read_dir(st.join("results/rho")).unwrap() { let e = e.unwrap().path(); let n = e.file_name().unwrap().to_string_lossy().to_string();
        if n.starts_with(&format!("rhostrong_p{p}_")) && n.ends_with(".json") { for r in rows(&e) { v.push((r["seed"].as_u64().unwrap(), rho(&r))); } } } rr.push(v); }
    let mut old = vec![];
    for d in ["inputs/old_rhostrong", "new"] { for e in fs::read_dir(Path::new(&a[2]).join(d)).unwrap() { let e = e.unwrap().path(); if e.extension().map(|x| x == "json").unwrap_or(false) { for r in rows(&e) { old.push((r["p"].as_u64().unwrap() * 1000 + r["seed"].as_u64().unwrap(), rho(&r))); } } } }
    let allrho: Vec<f64> = old.iter().map(|x| x.1).chain(rr.iter().flatten().map(|x| x.1)).collect();
    let pooled = allrho.iter().sum::<f64>() / allrho.len() as f64;
    let perp: Vec<f64> = rr.iter().map(|v| v.iter().map(|x| x.1).sum::<f64>() / v.len() as f64).collect();
    println!("bitop proxy (MODEL) rho mean per sqrt(l)*331: pooled {pooled:.4} (n={}), per prime {:?}", allrho.len(), perp);
    // instance groups for the pooled bootstrap
    let mut groups: Vec<Vec<f64>> = vec![];
    let mut keys: Vec<u64> = old.iter().map(|x| x.0).collect(); keys.sort(); keys.dedup();
    for k in keys { groups.push(old.iter().filter(|x| x.0 == k).map(|x| x.1).collect()); }
    for (pi, p) in primes.iter().enumerate() { let mut ks: Vec<u64> = rr[pi].iter().map(|x| x.0).collect(); ks.sort(); ks.dedup(); let _ = p; for k in ks { groups.push(rr[pi].iter().filter(|x| x.0 == k).map(|x| x.1).collect()); } }
    let pg: Vec<Vec<Vec<f64>>> = (0..3).map(|pi| { let mut ks: Vec<u64> = rr[pi].iter().map(|x| x.0).collect(); ks.sort(); ks.dedup(); ks.iter().map(|k| rr[pi].iter().filter(|x| x.0 == *k).map(|x| x.1).collect()).collect() }).collect();
    let mut rng = Rng(0xB17_0F_2026);
    for arm in ["P", "H", "U"] {
        let cv: Vec<Vec<f64>> = primes.iter().map(|p| rows(&st.join(format!("results/cover/{arm}_{p}.json"))).iter().map(cover).collect()).collect();
        for mode in 0..2 {
            let refs = |pooled_v: f64, per: &[f64]| -> Vec<f64> { if mode == 1 { vec![pooled_v; 3] } else { per.to_vec() } };
            let gm = |c: &Vec<Vec<f64>>, idx: &Vec<Vec<usize>>, r: &Vec<f64>| -> [f64; 4] { let mut s = [0.0; 3]; for pi in 0..3 { for &i in &idx[pi] { s[pi] += (c[pi][i] / r[pi]).ln(); } }
                [((s[0] + s[1] + s[2]) / (3 * idx[0].len()) as f64).exp(), (s[0] / idx[0].len() as f64).exp(), (s[1] / idx[1].len() as f64).exp(), (s[2] / idx[2].len() as f64).exp()] };
            let id: Vec<Vec<usize>> = (0..3).map(|pi| (0..cv[pi].len()).collect()).collect();
            let pt = gm(&cv, &id, &refs(pooled, &perp));
            let mut bs: Vec<Vec<f64>> = vec![vec![]; 4];
            for _ in 0..NB {
                let rp = if mode == 1 { let (mut s, mut n) = (0.0, 0.0); for g in &groups { for _ in 0..g.len() { s += g[rng.idx(g.len())]; n += 1.0; } } vec![s / n; 3] }
                    else { (0..3).map(|pi| { let (mut s, mut n) = (0.0, 0.0); for g in &pg[pi] { for _ in 0..g.len() { s += g[rng.idx(g.len())]; n += 1.0; } } s / n }).collect() };
                let idx: Vec<Vec<usize>> = (0..3).map(|pi| (0..cv[pi].len()).map(|_| rng.idx(cv[pi].len())).collect()).collect();
                let v = gm(&cv, &idx, &rp); for k in 0..4 { bs[k].push(v[k]); }
            }
            let nm = ["pooled", "p1009", "p1511", "p1823"];
            println!("arm {arm} ref {}: {}", if mode == 1 { "pooled" } else { "per-prime" }, (0..4).map(|k| format!("{} {:.5} [{:.5}, {:.5}]", nm[k], pt[k], pct(&bs[k], 0.025), pct(&bs[k], 0.975))).collect::<Vec<_>>().join(" | "));
        }
    }
}
