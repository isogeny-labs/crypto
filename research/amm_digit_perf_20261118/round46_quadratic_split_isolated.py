#!/usr/bin/env python3
"""Round 46 quadratic split factor: compare scalar-root character paths against frozen polynomial powering and GCD."""
from pathlib import Path
import re
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source
# Keep the frozen generic exponentiation + polynomial GCD in the A/B baseline.
for label in ["PRODUCER_HELPER", "REPLAY_HELPER"]:
    pattern = rf"(?ms)^[ ]*// R46_{label}_BEGIN\n.*?^[ ]*// R46_{label}_END\n"
    before, count = re.subn(pattern, "", before, count=1)
    assert count == 1, f"missing helper {label}"
replace = {
    "PRODUCER_SPLIT": """            let mut s = self.power(&base, (self.p - 1) / 2, &h)?;
            s.len = s.len.max(1);
            s.coefficients[0] = self.sub(s.coefficients[0], 1)?;
            s.trim();
            let d = self.gcd(h, s)?;
""",
    "REPLAY_SPLIT": """                let mut value = self.modular_power(base, (self.p - 1) / 2, &polynomial)?;
                value.len = value.len.max(1);
                value.coefficients[0] = self.minus(value.coefficients[0], 1)?;
                value.trim();
                let expected = self.monic_gcd(polynomial, value)?;
""",
}
for label, old_code in replace.items():
    pattern = rf"(?ms)^[ ]*// R46_{label}_BEGIN\n.*?^[ ]*// R46_{label}_END\n"
    before, count = re.subn(pattern, lambda match: old_code, before, count=1)
    assert count == 1, f"missing split {label}"
assert "R46_" not in before
assert "fn modular_power" in source

header = """#![allow(dead_code)]
mod cryptanalysis {
 pub mod fp_backend {
  pub struct Fp64Arithmetic {pub p:u64}
  impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
 }
 pub mod bounded {
  pub mod after {
"""
middle = "\n  } pub mod before {\n"
tail = "\n  }\n }\n}\n"
probe = r"""
pub fn round46_quad_probe(p:u64, coefficients:&[u64], delta:u64, exponent:u64)
    ->(Vec<u64>,Vec<u64>,u64,u64,u64,u64){
    let field=Fp64Arithmetic{p};
    let modulus=Poly17::from_coefficients(&field,coefficients).unwrap();
    let mut base=Poly17::ZERO;
    base.len=2;
    base.coefficients[0]=delta%p;
    base.coefficients[1]=1;
    let limits=RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
        bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8};
    let deadline=Instant::now()+std::time::Duration::from_secs(120);
    let mut observer=|_:&BoundedRootEvent|Ok(());
    let mut producer=Producer {
        p,gate:Gate::new(limits,deadline),observer:&mut observer,
        unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
        roots:[0;8],root_count:0
    };
    let producer_start=Instant::now();
    let generated=producer.power(&base,exponent,&modulus).unwrap();
    let producer_ns=producer_start.elapsed().as_nanos() as u64;
    let pwork=producer.gate.metrics.field_work;
    let mut replay=Replay::new(&field,&[1],42,limits,limits,deadline).unwrap();
    let replay_start=Instant::now();
    let checked=replay.modular_power(base,exponent,&modulus).unwrap();
    let replay_ns=replay_start.elapsed().as_nanos() as u64;
    (generated.coefficients().to_vec(),checked.coefficients().to_vec(),
     pwork,replay.gate.metrics.field_work,producer_ns,replay_ns)
}
"""
tests=r"""
#[cfg(test)]
mod round46_quad {
    use super::cryptanalysis::{fp_backend::Fp64Arithmetic,bounded::{after,before}};
    use std::{hint::black_box,time::{Instant,Duration}};
    fn polynomial(p:u64, degree:usize)->Vec<u64> {
        let mut f=vec![1u64];
        for root in 1..=degree {
            let mut next=vec![0;f.len()+1];
            for (i,&c) in f.iter().enumerate() {
                let m=(c as u128 * root as u128 % p as u128) as u64;
                next[i]=((next[i] as u128 + p as u128 - m as u128)%p as u128) as u64;
                next[i+1]=(next[i+1]+c)%p;
            }
            f=next;
        }
        f
    }
    fn budgets_after()->after::RootBudget{
        after::RootBudget{work:after::MAX_WORK,field_work:after::MAX_FIELD_WORK,
          bytes:after::STORAGE_BYTES,events:after::MAX_EVENTS,split_attempts:1000,degree:8}
    }
    fn budgets_before()->before::RootBudget{
        before::RootBudget{work:before::MAX_WORK,field_work:before::MAX_FIELD_WORK,
          bytes:before::STORAGE_BYTES,events:before::MAX_EVENTS,split_attempts:1000,degree:8}
    }
    fn solve_after(p:u64,f:&[u64],seed:u64)->(Vec<u64>,Vec<String>,u64,u64) {
        let field=Fp64Arithmetic{p};let deadline=Instant::now()+Duration::from_secs(120);
        let limits=budgets_after();let mut events=Vec::new();
        let report=after::solve_bounded(&field,black_box(f),seed,limits,deadline,
            &mut |e|{events.push(*e);Ok(())});
        assert!(report.root_set_complete(),"new stop={:?} degree={}",report.stop,f.len()-1);
        let mut replay=after::Replay::new(&field,f,seed,limits,limits,deadline).unwrap();
        for e in &events{replay.push(e).unwrap();}
        let verified=replay.finish(&report).unwrap();
        assert!(verified.finite_complete);
        (report.roots[..report.root_count as usize].to_vec(),
         events.iter().map(|e|format!("{e:?}")).collect(),
         report.metrics.field_work,verified.replay_metrics.field_work)
    }
    fn solve_before(p:u64,f:&[u64],seed:u64)->(Vec<u64>,Vec<String>,u64,u64) {
        let field=Fp64Arithmetic{p};let deadline=Instant::now()+Duration::from_secs(120);
        let limits=budgets_before();let mut events=Vec::new();
        let report=before::solve_bounded(&field,black_box(f),seed,limits,deadline,
            &mut |e|{events.push(*e);Ok(())});
        assert!(report.root_set_complete(),"old stop={:?} degree={}",report.stop,f.len()-1);
        let mut replay=before::Replay::new(&field,f,seed,limits,limits,deadline).unwrap();
        for e in &events{replay.push(e).unwrap();}
        let verified=replay.finish(&report).unwrap();
        assert!(verified.finite_complete);
        (report.roots[..report.root_count as usize].to_vec(),
         events.iter().map(|e|format!("{e:?}")).collect(),
         report.metrics.field_work,verified.replay_metrics.field_work)
    }
    #[test]
    fn exact_powers_and_independent_replay_match_frozen() {
        let p=1_000_000_007u64;
        let mut comparisons=0;
        for degree in [2usize,3,6,8] {
            let mut modulus=polynomial(p,degree);
            for exp in [(p-1)/2,p,(p-1)/2-1,19] {
                for delta in [0,1,7,999_999_937] {
                    let x=before::round46_quad_probe(p,&modulus,delta,exp);
                    let y=after::round46_quad_probe(p,&modulus,delta,exp);
                    assert_eq!(x.0,y.0,"producer degree={degree} exp={exp}");
                    assert_eq!(x.1,y.1,"replay degree={degree} exp={exp}");
                    assert_eq!(y.0,y.1,"producer-replay degree={degree} exp={exp}");
                    if exp==p || exp==(p-1)/2 {
                        if y.2 > x.2 { println!("ROUND46_QUAD_PRODUCER_FIELD_WORK_MORE degree={degree} exp={exp} old={} new={}", x.2, y.2); }
                        if y.3 > x.3 { println!("ROUND46_QUAD_REPLAY_FIELD_WORK_MORE degree={degree} exp={exp} old={} new={}", x.3, y.3); }
                    }
                    comparisons+=3;
                }
            }
            // Nontrivial nonmonic inputs exercise the modulus normalizer.
            modulus[degree]=17;
            for exp in [(p-1)/2,p] {
                let x=before::round46_quad_probe(p,&modulus,42,exp);
                let y=after::round46_quad_probe(p,&modulus,42,exp);
                assert_eq!((x.0,x.1),(y.0,y.1));
                comparisons+=2;
            }
        }
        println!("ROUND46_QUAD_POWER_EXACT comparisons={comparisons}");
    }
    #[test]
    fn randomized_dense_sparse_and_nonmonic_powers_match_frozen() {
        let mut state=0x2026_1119_4242_0001u64;
        let mut comparisons=0usize;
        for p in [487u64,1_000_000_007u64,1_000_000_009u64,(1u64<<61)-1] {
        for degree in 1usize..=8 {
            for trial in 0..24 {
                let mut f=vec![0u64;degree+1];
                for j in 0..degree {
                    state=state.wrapping_mul(6364136223846793005)
                               .wrapping_add(1442695040888963407);
                    f[j]=if trial%4==0 && j%2==0{0}else{state%p};
                }
                f[degree]=if trial%3==0 {17} else {1};
                state=state.wrapping_mul(6364136223846793005)
                           .wrapping_add(1442695040888963407);
                let delta=state%p;
                for exp in [(p-1)/2,p] {
                    let old=before::round46_quad_probe(p,&f,delta,exp);
                    let new=after::round46_quad_probe(p,&f,delta,exp);
                    assert_eq!(old.0,new.0,
                        "producer degree={degree} trial={trial} exp={exp}");
                    assert_eq!(old.1,new.1,
                        "replay degree={degree} trial={trial} exp={exp}");
                    assert_eq!(new.0,new.1,
                        "cross-check degree={degree} trial={trial} exp={exp}");
                    if new.2 > old.2 || new.3 > old.3 { println!("ROUND46_QUAD_RANDOM_FIELD_WORK_MORE p={p} degree={degree} trial={trial} exp={exp} producer={}:{} replay={}:{}",old.2,new.2,old.3,new.3); }
                    comparisons+=3;
                }
            }
        }
        }
        println!("ROUND46_QUAD_RANDOM_EXACT comparisons={comparisons}");
    }
    #[test]
    fn complete_root_and_certificate_events_match_frozen() {
        let p=1_000_000_007u64;let mut n=0;
        for degree in [2usize,3,4,6,8] {
            let f=polynomial(p,degree);
            for seed in [42u64,0x5291ae77] {
                let old=solve_before(p,&f,seed);let new=solve_after(p,&f,seed);
                assert_eq!(old.0,new.0,"root set changed");
                assert_eq!(old.1,new.1,"event stream changed");
                if new.2 > old.2 { println!("ROUND46_QUAD_COMPLETE_PRODUCER_WORK_MORE degree={degree} old={} new={}",old.2,new.2); }
                if new.3 > old.3 { println!("ROUND46_QUAD_COMPLETE_REPLAY_WORK_MORE degree={degree} old={} new={}",old.3,new.3); }
                n+=1;
            }
        }
        println!("ROUND46_QUAD_COMPLETE_EXACT checked_cases={n}");
    }
    fn median(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);v[v.len()/2]}
    fn probe_timed(p:u64, f:&[u64], delta:u64, exp:u64, old:bool, reps:usize)->f64{
        let start=Instant::now();let mut digest=0u64;
        for _ in 0..reps {
            let v=if old{before::round46_quad_probe(p,f,black_box(delta),exp)}
                  else{after::round46_quad_probe(p,f,black_box(delta),exp)};
            digest^=v.0.first().copied().unwrap_or(0)^v.1.first().copied().unwrap_or(0)
                  ^v.2^v.3;
        }
        black_box(digest);start.elapsed().as_nanos() as f64/reps as f64
    }
    fn probe_parts(p:u64,f:&[u64],delta:u64,exp:u64,old:bool)->(f64,f64){
        let mut producer=0f64;let mut replay=0f64;
        for _ in 0..6 {
            let v=if old{before::round46_quad_probe(p,f,black_box(delta),exp)}
                  else{after::round46_quad_probe(p,f,black_box(delta),exp)};
            producer+=v.4 as f64;replay+=v.5 as f64;
        }
        (producer/6.0,replay/6.0)
    }
    fn solve_timed(p:u64,f:&[u64],old:bool,reps:usize)->f64{
        let start=Instant::now();let mut digest=0u64;
        for _ in 0..reps {
            let v=if old{solve_before(p,f,42)}else{solve_after(p,f,42)};
            digest^=v.2^v.3^v.0.first().copied().unwrap_or(0);
        }
        black_box(digest);start.elapsed().as_nanos() as f64/reps as f64
    }
    #[test]
    fn exact_quadratic_discriminant_classes_and_events() {
        for p in [3u64, 487, 1_000_000_007, 1_000_000_009, (1u64<<61)-1] {
            // (x-r)^2, (x-2)(x-3), x^2+1, x^2+x+5
            let r=if p==3 {1} else {23};
            let repeated=vec![(r*r)%p,(p-(2*r)%p)%p,1];
            let split=vec![6%p,(p-5%p)%p,1];
            for f in [repeated,split,vec![1,0,1],vec![5%p,1,1]] {
                let old=before::round46_quad_probe(p,&f,0,p);
                let new=after::round46_quad_probe(p,&f,0,p);
                assert_eq!(old.0,new.0,"producer p={p} f={f:?}");
                assert_eq!(old.1,new.1,"verifier p={p} f={f:?}");
                assert_eq!(new.0,new.1,"cross-check p={p} f={f:?}");
                let old_complete=solve_before(p,&f,42);
                let new_complete=solve_after(p,&f,42);
                assert_eq!(old_complete.0,new_complete.0,"roots p={p} f={f:?}");
                assert_eq!(old_complete.1,new_complete.1,"events p={p} f={f:?}");
            }
        }
        println!("ROUND46_QUAD_DISCRIMINANT_CLASSES checked=20");
    }
    #[test]
    fn randomized_distinct_quadratic_root_events_match_frozen() {
        let mut checked=0usize;
        for p in [7u64,487,1_000_000_007,(1u64<<61)-1] {
            for j in 1u64..=12 {
                let r0=j%p;
                let r1=(j*17+3)%p;
                if r0==r1 {continue;}
                let c=((r0 as u128*r1 as u128)%p as u128) as u64;
                let b=(p-((r0+r1)%p))%p;
                let poly=[c,b,1u64];
                for seed in [0u64,42,0x5291ae77] {
                    let old=solve_before(p,&poly,seed);
                    let new=solve_after(p,&poly,seed);
                    assert_eq!(old.0,new.0,"p={p} roots={r0},{r1} seed={seed}");
                    assert_eq!(old.1,new.1,"p={p} roots={r0},{r1} seed={seed}");
                    checked+=1;
                }
            }
        }
        println!("ROUND46_QUAD_RANDOM_ROOT_EVENTS checked={checked}");
    }
    #[test]
    #[ignore="Paired production Rust end-to-end split and higher-degree controls"]
    fn bench_complete_degree_controls() {
        for p in [487u64,1_000_000_007,1_000_000_009,(1u64<<61)-1] {
            for degree in [2usize,3,4,6,8] {
                let poly=polynomial(p,degree);
                let old_result=solve_before(p,&poly,42);
                let new_result=solve_after(p,&poly,42);
                assert_eq!(old_result.0,new_result.0);
                assert_eq!(old_result.1,new_result.1);
                let mut old=Vec::new();let mut new=Vec::new();
                for trial in 0..13 {
                    if trial%2==0 {
                        old.push(solve_timed(p,&poly,true,2));
                        new.push(solve_timed(p,&poly,false,2));
                    } else {
                        new.push(solve_timed(p,&poly,false,2));
                        old.push(solve_timed(p,&poly,true,2));
                    }
                }
                let before=median(old);let after=median(new);
                println!("ROUND46_NATIVE_COMPLETE p={p} degree={degree} before_ns={before:.1} after_ns={after:.1} speedup={:.4}x producer_work={}:{} replay_work={}:{}",before/after,old_result.2,new_result.2,old_result.3,new_result.3);
            }
        }
    }
    #[test]
    #[ignore="Benchmark split, irreducible and repeated quadratics"]
    fn bench_quadratic_classes_complete() {
        for p in [487u64, 1_000_000_007, 1_000_000_009, (1u64<<61)-1] {
            let nonsquare_c=if p==1_000_000_009 {11} else {1};
            let r=23u64;
            let repeated=vec![r*r%p, p-2*r, 1];
            let split=polynomial(p,2);
            let irreducible=vec![nonsquare_c,0,1];
            for (name,f) in [("split",split),("irreducible",irreducible),("repeated",repeated)] {
                let old_result=solve_before(p,&f,42);
                let new_result=solve_after(p,&f,42);
                assert_eq!(old_result.0,new_result.0);
                assert_eq!(old_result.1,new_result.1);
                let mut old=Vec::new();let mut new=Vec::new();
                for trial in 0..13 {
                    if trial%2==0 {
                        old.push(solve_timed(p,&f,true,4));
                        new.push(solve_timed(p,&f,false,4));
                    } else {
                        new.push(solve_timed(p,&f,false,4));
                        old.push(solve_timed(p,&f,true,4));
                    }
                }
                let a=median(old);let b=median(new);
                println!("ROUND46_QUAD_CLASS p={p} class={name} before_ns={a:.1} after_ns={b:.1} speedup={:.4}x",a/b);
                println!("ROUND46_QUAD_WORK p={p} class={name} producer_before={} producer_after={} replay_before={} replay_after={}",old_result.2,new_result.2,old_result.3,new_result.3);
            }
        }
    }
    #[test]
    #[ignore="Production Rust A/B paired time comparison"]
    fn bench_quadratic_actual_rust(){
        for p in [487u64, 1_000_000_007u64, 1_000_000_009u64] {
        for degree in [2usize,3,4,6,8]{
            let f=polynomial(p,degree);
            for exp in [(p-1)/2,p]{
                let delta = if exp==p {0} else {13};
                let mut old=Vec::new();let mut new=Vec::new();
                for trial in 0..13{
                    if trial%2==0{
                        old.push(probe_timed(p,&f,delta,exp,true,6));
                        new.push(probe_timed(p,&f,delta,exp,false,6));
                    }else{
                        new.push(probe_timed(p,&f,delta,exp,false,6));
                        old.push(probe_timed(p,&f,delta,exp,true,6));
                    }
                }
                let a=median(old);let b=median(new);
                println!("ROUND46_QUAD_NATIVE_POWER p={p} degree={degree} exponent={exp} before_ns={a:.1} after_ns={b:.1} speedup={:.4}x",a/b);
                let mut bp=Vec::new();let mut ap=Vec::new();let mut br=Vec::new();let mut ar=Vec::new();
                for trial in 0..13 {
                    let (oldP,oldR,newP,newR)=if trial%2==0 {
                        let old=probe_parts(p,&f,delta,exp,true);
                        let new=probe_parts(p,&f,delta,exp,false);
                        (old.0,old.1,new.0,new.1)
                    } else {
                        let new=probe_parts(p,&f,delta,exp,false);
                        let old=probe_parts(p,&f,delta,exp,true);
                        (old.0,old.1,new.0,new.1)
                    };
                    bp.push(oldP);br.push(oldR);ap.push(newP);ar.push(newR);
                }
                let (bp,ap,br,ar)=(median(bp),median(ap),median(br),median(ar));
                println!("ROUND46_QUAD_NATIVE_PRODUCER p={p} degree={degree} exponent={exp} before_ns={bp:.1} after_ns={ap:.1} speedup={:.4}x",bp/ap);
                println!("ROUND46_QUAD_NATIVE_REPLAY p={p} degree={degree} exponent={exp} before_ns={br:.1} after_ns={ar:.1} speedup={:.4}x",br/ar);
            }
            let mut old=Vec::new();let mut new=Vec::new();
            for trial in 0..13 {
                if trial%2==0 {
                    old.push(solve_timed(p,&f,true,2));new.push(solve_timed(p,&f,false,2));
                }else{
                    new.push(solve_timed(p,&f,false,2));old.push(solve_timed(p,&f,true,2));
                }
            }
            let a=median(old);let b=median(new);
            println!("ROUND46_QUAD_NATIVE_COMPLETE p={p} degree={degree} before_ns={a:.1} after_ns={b:.1} speedup={:.4}x",a/b);
        }
    }
    }
}
"""
with tempfile.TemporaryDirectory() as tmp:
    path=Path(tmp)/"round46_quad.rs"
    binary=Path(tmp)/"round46_quad"
    path.write_text(header+source+probe+middle+before+probe+tail+tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    for test in ["exact_powers_and_independent_replay_match_frozen",
                 "randomized_dense_sparse_and_nonmonic_powers_match_frozen",
                 "complete_root_and_certificate_events_match_frozen"]:
        subprocess.run([str(binary),test,"--nocapture"],check=True)
    subprocess.run([str(binary),"exact_quadratic_discriminant_classes_and_events","--nocapture"],check=True)
    subprocess.run([str(binary),"randomized_distinct_quadratic_root_events_match_frozen","--nocapture"],check=True)
    subprocess.run([str(binary),"bench_quadratic_classes_complete","--ignored","--nocapture"],check=True)
    subprocess.run([str(binary),"bench_complete_degree_controls","--ignored","--nocapture"],check=True)
