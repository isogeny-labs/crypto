//! EXPERIMENT driver for research/f4_early_stop_20261018: F4 early-stop rules on the
//! Nagao six-quadric systems, same residual jobs as `jv_cover --exp ccov`.
use crypto_lib::cryptanalysis::jv_cover::run_stop_study;
use std::env;
use std::fs;

fn main() {
    let a: Vec<String> = env::args().skip(1).collect();
    let (mut sizes, mut residuals, mut constructed, mut threads) = (vec![101u64], 100usize, 20usize, 2usize);
    let mut chunks = vec![1usize, 2, 4, 8];
    let mut json: Option<String> = None;
    let mut i = 0;
    while i < a.len() {
        let v = a.get(i + 1).cloned().unwrap_or_default();
        match a[i].as_str() {
            "--sizes" => sizes = v.split(',').map(|x| x.parse().unwrap()).collect(),
            "--residuals" => residuals = v.parse().unwrap(),
            "--constructed" => constructed = v.parse().unwrap(),
            "--threads" => threads = v.parse().unwrap(),
            "--chunks" => chunks = v.split(',').map(|x| x.parse().unwrap()).collect(),
            "--json" => json = Some(v),
            o => panic!("unknown {o}"),
        }
        i += 2;
    }
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().ok();
    let mut all = Vec::new();
    for &p in &sizes {
        let rows = run_stop_study(p, 1, residuals, constructed, &chunks);
        eprintln!("p={p}: {} systems", rows.len());
        all.push(serde_json::json!({ "p": p, "systems": rows }));
    }
    if let Some(j) = json {
        fs::write(j, serde_json::to_string(&all).unwrap()).unwrap();
    }
}
