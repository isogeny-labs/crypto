#![allow(dead_code)]
mod cryptanalysis {
    pub mod ic_boundary {
        #[derive(Clone, Debug)]
        pub struct PrimeCurve { pub p: u64, pub a: u64, pub b: u64 }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct PrimePoint { pub x: u64, pub y: u64, pub infinity: bool }
        impl PrimePoint {
            pub const INFINITY: Self = Self { x:0,y:0,infinity:true };
            pub fn affine(x:u64,y:u64)->Self { Self {x,y,infinity:false} }
        }
    }
}
#[path="baseline.rs"] mod baseline;
#[path="optimized.rs"] mod optimized;
use std::time::Instant;

fn median(mut v:Vec<f64>)->f64 {
    v.sort_by(|a,b|a.total_cmp(b));
    v[v.len()/2]
}

fn bench(p:u64,kind:&str) {
    let repeats = if kind=="iso3_b0" { 6 } else if kind=="even_four_roots" { 8 } else { 12 };
    let base = || {
        let t = Instant::now();
        let checksum = baseline::round139_benchmark(p,kind,repeats);
        (checksum,t.elapsed().as_secs_f64()*1e9/(32*repeats) as f64)
    };
    let optimized = || {
        let t = Instant::now();
        let checksum = optimized::round139_benchmark(p,kind,repeats);
        (checksum,t.elapsed().as_secs_f64()*1e9/(32*repeats) as f64)
    };
    assert_eq!(baseline::round139_benchmark(p,kind,1),
               optimized::round139_benchmark(p,kind,1),
               "baseline and candidate differ p={p} kind={kind}");
    let mut before = Vec::new();
    let mut after = Vec::new();
    let mut speedups = Vec::new();
    for trial in 0..13 {
        let ((c0,t0),(c1,t1))=if trial%2==0 {
            (base(),optimized())
        } else {
            let b=optimized();
            (base(),b)
        };
        assert_eq!(c0,c1,"checksum mismatch p={p} kind={kind} trial={trial}");
        if trial>=2 {
            before.push(t0);
            after.push(t1);
            speedups.push(t0/t1);
        }
    }
    println!("p={p} kind={kind} before_ns={:.3} after_ns={:.3} speedup={:.3}x",
        median(before),median(after),median(speedups));
}
fn main() {
    baseline::round139_independent_oracle();
    optimized::round139_independent_oracle();
    println!("source-exact Rust independent oracles: PASS");
    for p in [65_521u64,1_000_000_007,1_000_000_009,4_294_967_291,(1u64<<61)-1] {
        for kind in ["even_four_roots", "even_random", "even_scaled",
                     "binomial_quartic", "psi3_b0", "iso3_b0",
                     "psi3_general", "random_quartic"] {
            bench(p,kind);
        }
    }
}
