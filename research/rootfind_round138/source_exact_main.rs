#![allow(dead_code)]
mod cryptanalysis {
    pub mod ic_boundary {
        #[derive(Clone, Debug)]
        pub struct PrimeCurve { pub p:u64, pub a:u64, pub b:u64 }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct PrimePoint { pub x:u64, pub y:u64, pub infinity:bool }
        impl PrimePoint {
            pub const INFINITY:Self=Self{x:0,y:0,infinity:true};
            pub fn affine(x:u64,y:u64)->Self{Self{x,y,infinity:false}}
        }
    }
}
#[path="baseline.rs"] mod baseline;
#[path="optimized.rs"] mod optimized;
use std::time::Instant;
fn median(mut v:Vec<f64>)->f64{v.sort_by(|a,b|a.total_cmp(b));v[v.len()/2]}
fn bench(p:u64,kind:&str){
    let reps=if kind.starts_with("isogeny") { 12 }
        else if kind.contains("quartic") { 12 }
        else if kind.contains("cubic") { 24 }
        else { 12 };
    let mut before=Vec::new();let mut after=Vec::new();let mut paired=Vec::new();
    let run_a=||{
        let start=Instant::now();
        let hash=baseline::round138_benchmark(p,kind,reps);
        (hash,start.elapsed().as_secs_f64()*1e9/(32*reps) as f64)
    };
    let run_b=||{
        let start=Instant::now();
        let hash=optimized::round138_benchmark(p,kind,reps);
        (hash,start.elapsed().as_secs_f64()*1e9/(32*reps) as f64)
    };
    assert_eq!(baseline::round138_benchmark(p,kind,1),
               optimized::round138_benchmark(p,kind,1),
               "different results kind={kind} p={p}");
    // Alternate before/after order; discard the first two trials.
    for trial in 0..13 {
        let ((ca,ta),(cb,tb))=if trial%2==0 {
            (run_a(),run_b())
        } else {
            let b=run_b();(run_a(),b)
        };
        assert_eq!(ca,cb,"different result trial={trial} kind={kind} p={p}");
        if trial>=2 {before.push(ta);after.push(tb);paired.push(ta/tb);}
    }
    println!("p={p} kind={kind} before_ns={:.3} after_ns={:.3} paired_speedup={:.3}x",
        median(before),median(after),median(paired));
}
fn main(){
    baseline::round138_independent_oracle();
    optimized::round138_independent_oracle();
    println!("independent source-exact Rust oracles: PASS");
    for p in [65_521u64,1_000_000_007,4_294_967_291,(1u64<<61)-1] {
        for kind in [
            "psi3_j0", "zero_quartic", "zero_x2", "binomial_cubic",
            "binomial_quartic_control",
            "isogeny2_j1728", "isogeny3_j0",
            "isogeny2_general", "isogeny3_general",
            "psi3_general", "constructed_cubic", "constructed_quartic",
            "random_cubic", "random_quartic"
        ] {bench(p,kind);}
    }
}
