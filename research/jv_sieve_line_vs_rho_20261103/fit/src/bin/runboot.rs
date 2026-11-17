//! SECONDARY (declared after the data existed): rho-run-level bootstrap. The frozen bootstrap resamples rho runs only WITHIN an instance
//! group, which gives no rho variation where an instance has one run (p=1823) and little where it has two. Here the per-prime reference
//! is resampled over all that prime's runs (n_p draws with replacement), cover seeds as before. Per-prime reference only.
//! Usage: runboot <study dir> [extra rho dir with additional records, e.g. results/rho_supp]
use serde_json::Value;
use std::{fs, path::Path};
const GAE: f64 = 331.0; const NB: usize = 20000;
struct Rng(u64);
impl Rng { fn next(&mut self) -> u64 { self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15); let mut z = self.0; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^ (z >> 31) }
    fn idx(&mut self, n: usize) -> usize { (((self.next() as u128) * (n as u128)) >> 64) as usize } }
fn rows(p: &Path) -> Vec<Value> { serde_json::from_str::<Value>(&fs::read_to_string(p).unwrap()).unwrap().as_array().unwrap().clone() }
fn f(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap() }
fn pct(v: &[f64], q: f64) -> f64 { let mut s = v.to_vec(); s.sort_by(|a, b| a.partial_cmp(b).unwrap()); s[((s.len() - 1) as f64 * q).round() as usize] }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let st = Path::new(&a[1]);
    let primes = [1009u64, 1511, 1823];
    // per prime: runs as [u1, u2, rho S+ per 331 sqrt(l) (native+fp_adds)/331/sqrt(l)]
    let mut rr: Vec<Vec<[f64; 3]>> = vec![];
    for p in primes {
        let mut v = vec![];
        let mut dirs = vec![st.join("results/rho")];
        if a.len() > 2 { dirs.push(st.join(&a[2])); }
        for d in dirs { if let Ok(rd) = fs::read_dir(d) { let mut es: Vec<_> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect(); es.sort();
            for e in es { let n = e.file_name().unwrap().to_string_lossy().to_string();
                if n.starts_with(&format!("rhostrong_p{p}_")) && n.ends_with(".json") { for r in rows(&e) { let sl = f(&r, "l").sqrt(); let o = &r["ops"];
                    let nat: f64 = ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| f(o, k)).sum();
                    v.push([f(&r, "s_harness"), f(&r, "s_tuned"), (nat + f(o, "fp_adds")) / sl / GAE]); } } } } }
        rr.push(v);
    }
    let mut rng = Rng(0x2B007_2026);
    for arm in ["P", "H", "U"] {
        let cv: Vec<Vec<(f64, f64)>> = primes.iter().map(|p| rows(&st.join(format!("results/cover/{arm}_{p}.json"))).iter().map(|r| { let sl = f(r, "l").sqrt(); (f(r, "total_muls") / (GAE * sl), f(r, "total_plus") / (GAE * sl)) }).collect()).collect();
        let mean3 = |v: &Vec<[f64; 3]>| { let n = v.len() as f64; [v.iter().map(|x| x[0]).sum::<f64>() / n, v.iter().map(|x| x[1]).sum::<f64>() / n, v.iter().map(|x| x[2]).sum::<f64>() / n] };
        let stat = |refs: &Vec<[f64; 3]>, idx: &Vec<Vec<usize>>| -> Vec<[f64; 3]> { // [S/rho U1, S/rho U2, S+/rhoS+ U2] for pooled (index 0) and primes
            let mut s = vec![[0.0f64; 3]; 4];
            for pi in 0..3 { for &i in &idx[pi] { let (m, sp) = cv[pi][i]; let r = refs[pi]; let x = [(m / r[0]).ln(), (m / r[1]).ln(), (sp / (r[1] + r[2])).ln()]; for k in 0..3 { s[pi + 1][k] += x[k]; s[0][k] += x[k]; } } }
            for k in 0..3 { s[0][k] = (s[0][k] / (3 * idx[0].len()) as f64).exp(); for pi in 1..4 { s[pi][k] = (s[pi][k] / idx[pi - 1].len() as f64).exp(); } }
            s };
        let pt_refs: Vec<[f64; 3]> = rr.iter().map(mean3).collect();
        let pt = stat(&pt_refs, &(0..3).map(|pi| (0..cv[pi].len()).collect()).collect());
        let mut bs = vec![vec![vec![]; 3]; 4];
        for _ in 0..NB {
            let refs: Vec<[f64; 3]> = (0..3).map(|pi| { let v: Vec<[f64; 3]> = (0..rr[pi].len()).map(|_| rr[pi][rng.idx(rr[pi].len())]).collect(); mean3(&v) }).collect();
            let idx: Vec<Vec<usize>> = (0..3).map(|pi| (0..cv[pi].len()).map(|_| rng.idx(cv[pi].len())).collect()).collect();
            let s = stat(&refs, &idx);
            for sc in 0..4 { for k in 0..3 { bs[sc][k].push(s[sc][k]); } }
        }
        for (sc, nm) in ["pooled", "p1009", "p1511", "p1823"].iter().enumerate() {
            println!("arm {arm} {nm}: {}", ["S/rho U1", "S/rho U2", "S+/rhoS+ U2"].iter().enumerate().map(|(k, l)| format!("{l} {:.5} [{:.5}, {:.5}]", pt[sc][k], pct(&bs[sc][k], 0.025), pct(&bs[sc][k], 0.975))).collect::<Vec<_>>().join("  "));
        }
    }
}
