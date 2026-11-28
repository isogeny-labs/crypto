#![allow(dead_code)]
mod cryptanalysis {
    pub mod ic_boundary {
        #[derive(Clone, Debug)]
        pub struct PrimeCurve { pub p:u64, pub a:u64, pub b:u64 }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct PrimePoint { pub x:u64, pub y:u64, pub infinity:bool }
        impl PrimePoint {
            pub const INFINITY:Self=Self { x:0,y:0,infinity:true };
            pub fn affine(x:u64,y:u64)->Self {Self{x,y,infinity:false}}
        }
    }
}
#[path="baseline.rs"] mod baseline;
#[path="optimized.rs"] mod optimized;
use std::time::Instant;
fn median(mut v:Vec<f64>)->f64 {
    v.sort_by(|a,b|a.total_cmp(b));v[v.len()/2]
}
fn bench(p:u64,kind:&str) {
    let repeats=if kind.starts_with("inverse_"){512}
                else if kind.starts_with("normalize_"){100}
                else if kind.starts_with("quadratic_"){50}
                else if kind.starts_with("isogeny"){6}
                else if kind=="powmod_dense" {30} else {10};
    let base=|| {
        let t=Instant::now();
        let checksum=baseline::round141_benchmark(p,kind,repeats);
        (checksum,t.elapsed().as_secs_f64()*1e9/(32*repeats) as f64)
    };
    let candidate=|| {
        let t=Instant::now();
        let checksum=optimized::round141_benchmark(p,kind,repeats);
        (checksum,t.elapsed().as_secs_f64()*1e9/(32*repeats) as f64)
    };
    assert_eq!(baseline::round141_benchmark(p,kind,1),
               optimized::round141_benchmark(p,kind,1));
    let (mut b,mut a,mut ratios)=(vec![],vec![],vec![]);
    for trial in 0..13 {
        let ((c0,t0),(c1,t1))=if trial%2==0 {
            (base(),candidate())
        }else{
            let opt=candidate();(base(),opt)
        };
        assert_eq!(c0,c1,"checksum mismatch {kind} p={p}");
        if trial>=2 {b.push(t0);a.push(t1);ratios.push(t0/t1);}
    }
    println!("p={p} kind={kind} before_ns={:.3} after_ns={:.3} speedup={:.3}x",
             median(b),median(a),median(ratios));
}
fn main() {
    baseline::round144_exact_fixed_oracle();
    optimized::round144_exact_fixed_oracle();
    baseline::round143_fused_product_matches_exact_convolution();
    optimized::round143_fused_product_matches_exact_convolution();
    baseline::round141_oracle();
    optimized::round141_oracle();
    baseline::round142_powmod_oracle();
    optimized::round142_powmod_oracle();
    println!("native Rust independent correctness oracles PASS");
    for p in [65_521u64,1_000_000_007,4_294_967_291,(1u64<<61)-1] {
        for kind in ["powmod_dense","normalize_random","quadratic_random_scaled",
                     "cubic_random_scaled","quartic_random_scaled",
                     "isogeny3_general","isogeny2_general",
                     "inverse_random","quadratic_scaled_3",
                     "quartic_scaled_3","quartic_monic","gcd"] {
            bench(p,kind);
        }
    }
}
