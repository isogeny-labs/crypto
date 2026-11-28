#![allow(dead_code)]
mod cryptanalysis {
  pub mod ic_boundary {
    #[derive(Clone,Debug)]
    pub struct PrimeCurve{pub p:u64,pub a:u64,pub b:u64}
    #[derive(Clone,Copy,Debug,PartialEq,Eq)]
    pub struct PrimePoint{pub x:u64,pub y:u64,pub infinity:bool}
    impl PrimePoint{
      pub const INFINITY:Self=Self{x:0,y:0,infinity:true};
      pub fn affine(x:u64,y:u64)->Self{Self{x,y,infinity:false}}
    }
  }
}
#[path="baseline.rs"]mod baseline;
#[path="optimized.rs"]mod optimized;
use std::time::Instant;
fn med(mut x:Vec<f64>)->f64{x.sort_by(|a,b|a.total_cmp(b));x[x.len()/2]}
fn bench(p:u64,kind:&str){
    let sqrt=kind=="sqrt_residue"||kind=="sqrt_nonresidue";
    let inputs=if sqrt{
        baseline::round134_generate_cases(p,kind=="sqrt_nonresidue")
    }else{vec![]};
    let polys=if !sqrt && kind!="isogeny3" {
        baseline::round136_cases(p,kind)
    }else{vec![]};
    let reps=if kind=="isogeny3"{3}else if kind=="cubic"{5}else{70};
    let a=|n:usize|->u64{
        if sqrt {baseline::round134_sqrt_benchmark(p,&inputs,n)}
        else if kind=="isogeny3"{baseline::round129_benchmark("isogeny3",p,n)}
        else{baseline::round134_complete_roots_benchmark(p,&polys,n)}
    };
    let b=|n:usize|->u64{
        if sqrt {optimized::round134_sqrt_benchmark(p,&inputs,n)}
        else if kind=="isogeny3"{optimized::round129_benchmark("isogeny3",p,n)}
        else{optimized::round134_complete_roots_benchmark(p,&polys,n)}
    };
    assert_eq!(a(1),b(1),"checksum p={p} kind={kind}");
    let mut x=Vec::new();let mut y=Vec::new();let mut ratios=Vec::new();
    for trial in 0..13{
        let ma=||{let t=Instant::now();let c=a(reps);(c,t.elapsed().as_secs_f64()*1e9/(32*reps) as f64)};
        let mb=||{let t=Instant::now();let c=b(reps);(c,t.elapsed().as_secs_f64()*1e9/(32*reps) as f64)};
        let ((ca,ta),(cb,tb))=if trial%2==0{(ma(),mb())}else{
            let q=mb();(ma(),q)
        };
        assert_eq!(ca,cb,"A/B mismatch p={p} kind={kind}");
        x.push(ta);y.push(tb);ratios.push(ta/tb);
    }
    println!("p={p} kind={kind} before_ns={:.3} after_ns={:.3} paired_speedup={:.3}x",med(x),med(y),med(ratios));
}
fn main(){
    baseline::round136_oracle();optimized::round136_oracle();
    baseline::round135_sqrt_oracle();optimized::round135_sqrt_oracle();
    println!("source-exact baseline and optimized oracles: PASS");
    for p in [167_772_161u64,998_244_353,2_013_265_921,65_537] {
        for kind in ["sqrt_residue","sqrt_nonresidue","quadratic","quad_random","isogeny3"]{bench(p,kind);}
    }
    for p in [65_519u64,1_000_000_007,4_294_967_291,(1u64<<61)-1] {
        for kind in ["quadratic","quad_nonsquare","quad_random","cubic","isogeny3"]{bench(p,kind);}
    }
}
