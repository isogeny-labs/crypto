#![allow(dead_code)]
mod cryptanalysis {
    pub mod ic_boundary {
        #[derive(Clone, Debug)]
        pub struct PrimeCurve { pub p: u64, pub a: u64, pub b: u64 }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct PrimePoint { pub x: u64, pub y: u64, pub infinity: bool }
        impl PrimePoint {
            pub const INFINITY: Self=Self{x:0,y:0,infinity:true};
            pub fn affine(x:u64,y:u64)->Self{Self{x,y,infinity:false}}
        }
    }
}
#[path="baseline.rs"] mod baseline;
#[path="optimized.rs"] mod optimized;
use std::time::Instant;
fn med(mut x:Vec<f64>)->f64{x.sort_by(|a,b|a.total_cmp(b));x[x.len()/2]}
fn bench(p:u64,kind:&str){
    let gcd=kind=="gcd";
    let iso=kind=="isogeny3"||kind=="isogeny2";
    let cases=if !gcd && !iso{baseline::round137_cases(p,kind)}else{vec![]};
    let reps=if gcd {300} else if iso {4} else if kind=="quartic"||kind=="quartic_scaled" {6} else {10};
    let a=|r:usize| -> u64 {
        if gcd {baseline::round137_gcd_benchmark(p,r)}
        else if iso {baseline::round129_benchmark(kind,p,r)}
        else {baseline::round134_complete_roots_benchmark(p,&cases,r)}
    };
    let b=|r:usize| -> u64 {
        if gcd {optimized::round137_gcd_benchmark(p,r)}
        else if iso {optimized::round129_benchmark(kind,p,r)}
        else {optimized::round134_complete_roots_benchmark(p,&cases,r)}
    };
    assert_eq!(a(1),b(1),"checksum mismatch p={p} kind={kind}");
    let mut before=vec![];let mut after=vec![];let mut ratio=vec![];
    for trial in 0..13 {
        let measure_a=||{let t=Instant::now();let c=a(reps);(c,t.elapsed().as_secs_f64()*1e9/(32*reps) as f64)};
        let measure_b=||{let t=Instant::now();let c=b(reps);(c,t.elapsed().as_secs_f64()*1e9/(32*reps) as f64)};
        let ((ca,ta),(cb,tb))=if trial%2==0{(measure_a(),measure_b())}
            else{let q=measure_b();(measure_a(),q)};
        assert_eq!(ca,cb,"checksum mismatch p={p} kind={kind} trial={trial}");
        if trial>=2{before.push(ta);after.push(tb);ratio.push(ta/tb);}
    }
    println!("p={p} kind={kind} before_ns={:.3} after_ns={:.3} paired_speedup={:.3}x",med(before),med(after),med(ratio));
}
fn main(){
    baseline::round137_independent_gcd_oracle();
    optimized::round137_independent_gcd_oracle();
    println!("both source-exact independent oracles: PASS");
    for p in [65_521u64,1_000_000_007,4_294_967_291,(1u64<<61)-1] {
        for kind in ["gcd","cubic","quartic","quartic_scaled","random_cubic","random_quartic","psi3","isogeny2","isogeny3"]{
            bench(p,kind);
        }
    }
}
