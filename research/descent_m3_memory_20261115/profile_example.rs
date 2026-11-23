// Profiling driver used for this note (copy to examples/zz_descent_m3_profile.rs and `cargo build --release --example zz_descent_m3_profile`; not built in-tree).
// Usage: zz_descent_m3_profile <k> <budget_s> [x_R] [chain]  (curve n21 b=1797947, z^21+z^2+1, V = <1, z, ..., z^(k-1)>)
use crypto_lib::binary_ecc::f2m::IrreduciblePoly;
use crypto_lib::cryptanalysis::pq_descent_symbolic::{descend, FieldBoolPoly};
use crypto_lib::cryptanalysis::pq_f4_f2::groebner_basis_f4;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use std::time::{Duration, Instant};

fn rss() -> (u64, u64) {
    let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let get = |k: &str| s.lines().find(|l| l.starts_with(k)).and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok()).unwrap_or(0u64);
    (get("VmRSS:"), get("VmHWM:"))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let k: usize = args[1].parse().unwrap();
    let budget: u64 = args[2].parse().unwrap();
    let x_r: u64 = args.get(3).map(|s| s.parse().unwrap()).unwrap_or(232919);
    let gf = Gf2::new(&IrreduciblePoly { degree: 21, low_terms: vec![0, 2] });
    let b = 1797947u64;
    let basis: Vec<u64> = (0..k).map(|i| 1u64 << i).collect();
    let t = Instant::now();
    let mut sys = descend(&gf, b, x_r, &basis, 3).unwrap();
    if args.get(4).map(|s| s.as_str()) == Some("chain") {
        let n = 21u32; let kk = k as u32;
        let std: Vec<u64> = (0..n).map(|i| 1u64 << i).collect();
        let x1 = FieldBoolPoly::linear(0, &basis);
        let x2 = FieldBoolPoly::linear(kk, &basis);
        let x3 = FieldBoolPoly::linear(2 * kk, &basis);
        let u = FieldBoolPoly::linear(3 * kk, &std);
        let xr = FieldBoolPoly::constant(x_r);
        let s3 = |p: &FieldBoolPoly, q: &FieldBoolPoly, x: &FieldBoolPoly| {
            let a = p.add(q).sqr(&gf).mul(&x.sqr(&gf), &gf);
            let pq = p.mul(q, &gf);
            a.add(&pq.mul(x, &gf)).add(&pq.sqr(&gf)).add(&FieldBoolPoly::constant(b))
        };
        let nv = (3 * kk + n) as usize;
        let mut eqs = s3(&x1, &x2, &u).split(n, nv);
        eqs.extend(s3(&u, &x3, &xr).split(n, nv));
        eqs.retain(|p| !p.terms.is_empty());
        sys.equations = eqs; sys.n_vars = nv;
    }
    let terms: Vec<usize> = sys.equations.iter().map(|p| p.terms.len()).collect();
    let deg = sys.equations.iter().flat_map(|p| p.terms.iter().map(|t| t.degree())).max().unwrap();
    let mut all: Vec<u64> = sys.equations.iter().flat_map(|p| p.terms.iter().map(|t| t.mask as u64)).collect();
    all.sort_unstable(); all.dedup();
    println!("k={k} n_vars={} eqs={} field_monos={} max_deg={deg} terms/eq min={} max={} sum={} distinct_monos={} descent_ms={} rss_kb={:?}",
        sys.n_vars, sys.equations.len(), sys.field_monomials, terms.iter().min().unwrap(), terms.iter().max().unwrap(), terms.iter().sum::<usize>(), all.len(), t.elapsed().as_millis(), rss());
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let s2 = stop.clone();
    let t0 = Instant::now();
    let h = std::thread::spawn(move || {
        while !s2.load(std::sync::atomic::Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(2000));
            let (r, hwm) = rss();
            eprintln!("t={:.1}s rss_mb={} hwm_mb={}", t0.elapsed().as_secs_f64(), r / 1024, hwm / 1024);
        }
    });
    let (gb, st) = groebner_basis_f4(sys.equations.clone(), sys.n_vars, Some(Duration::from_secs(budget)));
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    h.join().ok();
    println!("gb_len={} wall_s={:.1} hwm_kb={:?}", gb.len(), t0.elapsed().as_secs_f64(), rss());
    println!("{:#?}", st);
}
