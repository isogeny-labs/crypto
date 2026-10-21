//! Recompute the two-sample KS D and p-value for every arm of one isoclass_rho
//! run directory, with (a) the historical tie-blind merge (reproduces the
//! recorded values) and (b) the tie-consuming merge from #2174's fixed stats.rs.
//! usage: ks_recompute <trials.csv> <recorded.tsv>   (recorded.tsv: arm\tks_d\tks_p)
//! out (TSV): arm n m rec_d rec_p buggy_d buggy_p fixed_d fixed_p
use std::collections::HashMap;
use std::fs;

fn pval(d: f64, n: usize, m: usize) -> f64 {
    let ne = (n * m) as f64 / (n + m) as f64;
    let lam = (ne.sqrt() + 0.12 + 0.11 / ne.sqrt()) * d;
    let mut p = 0.0;
    for k in 1..100 {
        p += 2.0 * (-1f64).powi(k - 1) * (-2.0 * (k as f64).powi(2) * lam * lam).exp();
    }
    p.clamp(0.0, 1.0)
}
fn ks_buggy(a: &[f64], b: &[f64]) -> (f64, f64) {
    let (mut x, mut y) = (a.to_vec(), b.to_vec());
    x.sort_by(|p, q| p.partial_cmp(q).unwrap());
    y.sort_by(|p, q| p.partial_cmp(q).unwrap());
    let (mut i, mut j, mut d) = (0usize, 0usize, 0f64);
    while i < x.len() && j < y.len() {
        if x[i] <= y[j] { i += 1 } else { j += 1 }
        d = d.max((i as f64 / x.len() as f64 - j as f64 / y.len() as f64).abs());
    }
    (d, pval(d, x.len(), y.len()))
}
fn ks_fixed(a: &[f64], b: &[f64]) -> (f64, f64) {
    let (mut x, mut y) = (a.to_vec(), b.to_vec());
    x.sort_by(|p, q| p.partial_cmp(q).unwrap());
    y.sort_by(|p, q| p.partial_cmp(q).unwrap());
    let (mut i, mut j, mut d) = (0usize, 0usize, 0f64);
    while i < x.len() || j < y.len() {
        let v = match (x.get(i), y.get(j)) {
            (Some(a), Some(b)) => a.min(*b),
            (Some(a), None) => *a,
            (None, Some(b)) => *b,
            _ => break,
        };
        while i < x.len() && x[i] == v { i += 1 }
        while j < y.len() && y[j] == v { j += 1 }
        d = d.max((i as f64 / x.len() as f64 - j as f64 / y.len() as f64).abs());
    }
    (d, pval(d, x.len(), y.len()))
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let trials = fs::read_to_string(&args[1]).unwrap();
    let rec = fs::read_to_string(&args[2]).unwrap();
    let mut by: HashMap<String, Vec<f64>> = HashMap::new();
    let mut kind: HashMap<String, String> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for line in trials.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        if !by.contains_key(f[0]) { order.push(f[0].to_string()); }
        by.entry(f[0].to_string()).or_default().push(f[6].parse().unwrap());
        kind.insert(f[0].to_string(), f[1].to_string());
    }
    let mut kc: HashMap<&str, usize> = HashMap::new();
    for a in &order { *kc.entry(kind[a].as_str()).or_default() += 1; }
    let main_kind = kc.iter().max_by_key(|(k, c)| (**c, std::cmp::Reverse(**k))).unwrap().0.to_string();
    let main_arms: Vec<&String> = order.iter().filter(|a| kind[*a] == main_kind).collect();
    for line in rec.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        let arm = f[0];
        let (rd, rp): (f64, f64) = (f[1].parse().unwrap(), f[2].parse().unwrap());
        let it = &by[arm];
        let others: Vec<f64> = main_arms.iter().filter(|k| k.as_str() != arm).flat_map(|k| by[*k].iter().cloned()).collect();
        if others.is_empty() || it.is_empty() { continue; }
        let (bd, bp) = ks_buggy(it, &others);
        let (fd, fp) = ks_fixed(it, &others);
        println!("{arm}\t{}\t{}\t{rd}\t{rp}\t{bd}\t{bp}\t{fd}\t{fp}", it.len(), others.len());
    }
}
