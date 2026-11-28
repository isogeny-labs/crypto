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
#[path="baseline.rs"]mod baseline;
#[path="optimized.rs"]mod optimized;
use std::time::Instant;
fn median(mut xs:Vec<f64>)->f64 {
    xs.sort_by(|a,b|a.total_cmp(b));xs[xs.len()/2]
}
fn benchmark(kind:&str,p:u64,op:&str) {
    let is_sqrt = kind=="residue" || kind=="nonresidue";
    let inputs=if is_sqrt{
        baseline::round134_generate_cases(p,kind=="nonresidue")
    }else{vec![]};
    let polynomials=if !is_sqrt && kind!="isogeny3" {
        baseline::round134_polynomial_cases(p,kind)
    }else{vec![]};
    let reps=if kind=="isogeny3"{3}else{30};
    let (baseline_call, optimized_call) = (|n:usize|->u64 {
       if is_sqrt { baseline::round134_sqrt_benchmark(p,&inputs,n) }
       else if kind=="isogeny3" { baseline::round129_benchmark("isogeny3",p,n) }
       else { baseline::round134_complete_roots_benchmark(p,&polynomials,n) }
    }, |n:usize|->u64 {
       if is_sqrt { optimized::round134_sqrt_benchmark(p,&inputs,n) }
       else if kind=="isogeny3" { optimized::round129_benchmark("isogeny3",p,n) }
       else { optimized::round134_complete_roots_benchmark(p,&polynomials,n) }
    });
    let count=32*reps;
    assert_eq!(baseline_call(1),optimized_call(1),
        "baseline/candidate mismatch p={p}, kind={kind}");
    let mut before=Vec::new();let mut after=Vec::new();let mut ratios=Vec::new();
    for trial in 0..13 {
        let measure_a=||{
            let t=Instant::now();let c=baseline_call(reps);
            (c,t.elapsed().as_secs_f64()*1e9/(count as f64))
        };
        let measure_b=||{
            let t=Instant::now();let c=optimized_call(reps);
            (c,t.elapsed().as_secs_f64()*1e9/(count as f64))
        };
        let ((a,ta),(b,tb))=if trial%2==0 {
            (measure_a(),measure_b())
        }else {
            let b=measure_b();let a=measure_a();(a,b)
        };
        assert_eq!(a,b,"checksum mismatch p={p} kind={kind} trial={trial}");
        before.push(ta);after.push(tb);ratios.push(ta/tb);
    }
    println!("p={p} kind={kind} op={op} before_ns={:.3} after_ns={:.3} paired_speedup={:.3}x",
        median(before),median(after),median(ratios));
}
fn main() {
    baseline::round134_sqrt_oracle();
    optimized::round134_sqrt_oracle();
    baseline::round133_independent_oracle();
    optimized::round133_independent_oracle();
    println!("independent source-exact roots oracle: PASS");
    for p in [65_521u64,998_244_353,1_000_000_009,2_013_265_921,1_000_000_007] {
        for kind in ["residue","nonresidue","quadratic","repeated","cubic","isogeny3"] {
            benchmark(kind,p,"full");
        }
    }
}
