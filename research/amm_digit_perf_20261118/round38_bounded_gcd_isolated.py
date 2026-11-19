#!/usr/bin/env python3
"""Round 38: compile exact production rootfinder, compare frozen pre-round38 GCD, and benchmark."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
previous = r"""
impl Producer<'_> {
    // Exact pre-round38 GCD, retaining currently optimized division/field arithmetic.
    fn previous_gcd(&mut self, mut a: Poly17, mut b: Poly17) -> Result<Poly17> {
        self.gate.charge(5 * COPY_WORK)?;
        while b.len != 0 {
            self.gate.charge(3 * COPY_WORK)?;
            let r = self.division(&a, &b)?.1;
            a = b;
            b = r;
        }
        if a.len != 0 {
            let inv = self.inverse(a.coefficients[a.len as usize - 1])?;
            for i in 0..a.len as usize {
                a.coefficients[i] = self.mul(a.coefficients[i], inv)?;
            }
        }
        Ok(a)
    }
}
impl Replay {
    // Exact pre-round38 monic_gcd, independent replay remainder implementation.
    fn previous_monic_gcd(&mut self, left: Poly17, right: Poly17) -> Result<Poly17> {
        self.gate.charge(5 * COPY_WORK)?;
        let mut pair = [left, right];
        while pair[1].len != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            let next = self.remainder(pair[0], &pair[1])?;
            pair = [pair[1], next];
        }
        let mut out = pair[0];
        if out.len > 0 && out.coefficients[usize::from(out.len) - 1] != 1 {
            let inv = self.reciprocal(out.coefficients[usize::from(out.len) - 1])?;
            for i in 0..usize::from(out.len) {
                if out.coefficients[i] != 0 {
                    out.coefficients[i] = self.times(out.coefficients[i], inv)?;
                }
            }
        }
        Ok(out)
    }
}
"""
tests = r"""
#[cfg(test)]
mod round38 {
    use super::*;
    use std::{hint::black_box, time::{Duration, Instant}};
    fn deadline() -> Instant { Instant::now() + Duration::from_secs(240) }
    fn limits() -> RootBudget {
        RootBudget {work: MAX_WORK, field_work: MAX_FIELD_WORK, bytes: STORAGE_BYTES,
            events: MAX_EVENTS, split_attempts: 1000, degree: 8}
    }
    fn next(x: &mut u64) -> u64 {
        *x=x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *x ^ (*x >> 33)
    }
    fn poly(p: u64, degree: usize, rng: &mut u64, monic: bool) -> Poly17 {
        let mut x=Poly17::ZERO; x.len=(degree+1) as u8;
        for j in 0..degree {x.coefficients[j]=next(rng)%p;}
        x.coefficients[degree]=if monic {1} else {1+next(rng)%(p-1)};
        x
    }
    fn multiply(a:Poly17,b:Poly17,p:u64)->Poly17 {
        let mut x=Poly17::ZERO;
        x.len=a.len+b.len-1;
        for i in 0..usize::from(a.len) {
            for j in 0..usize::from(b.len) {
                let k=i+j;
                x.coefficients[k]=((x.coefficients[k] as u128+
                   a.coefficients[i] as u128*b.coefficients[j] as u128)%p as u128) as u64;
            }
        }
        x.trim();x
    }
    fn cases(p:u64,case_kind:usize, count:usize)->Vec<(Poly17,Poly17)> {
        let mut rng=0x20_2611_19a5_a5e5u64 ^ p;
        (0..count).map(|i| {
            if case_kind==1 {
                let common=poly(p,2,&mut rng,false);
                let a=poly(p,6,&mut rng,false);
                let b=poly(p,5,&mut rng,false);
                (multiply(common,a,p),multiply(common,b,p))
            } else {
                let a=poly(p,8,&mut rng,case_kind==2);
                let b=poly(p,7,&mut rng,case_kind==2);
                if i%8==0 { (a,Poly17::constant(1+next(&mut rng)%(p-1))) }
                else { (a,b) }
            }
        }).collect()
    }
    fn do_producer(p:u64,cases:&[(Poly17,Poly17)],fast:bool,reps:usize)->f64 {
        let mut observer=|_:&BoundedRootEvent| Ok(());
        let mut pr=Producer {
            p,gate:Gate::new(limits(),deadline()),observer:&mut observer,
            unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
            roots:[0;8],root_count:0
        };
        let mut hash=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            for &(a,b) in cases {
                pr.gate.metrics=Metrics::default();
                let out=if fast {pr.gcd(black_box(a),black_box(b)).unwrap()}
                    else {pr.previous_gcd(black_box(a),black_box(b)).unwrap()};
                hash=hash.wrapping_add(out.coefficients[0]).wrapping_add(out.len as u64);
            }
        }
        black_box(hash);
        start.elapsed().as_nanos() as f64 / (reps*cases.len()) as f64
    }
    fn do_replay(p:u64,cases:&[(Poly17,Poly17)],fast:bool,reps:usize)->f64 {
        let field=Fp64Arithmetic{p};
        let mut rep=Replay::new(&field,&[1,1],42,limits(),limits(),deadline()).unwrap();
        let mut hash=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            for &(a,b) in cases {
                rep.gate.metrics=Metrics::default();
                let out=if fast {rep.monic_gcd(black_box(a),black_box(b)).unwrap()}
                    else {rep.previous_monic_gcd(black_box(a),black_box(b)).unwrap()};
                hash=hash.wrapping_add(out.coefficients[0]).wrapping_add(out.len as u64);
            }
        }
        black_box(hash);
        start.elapsed().as_nanos() as f64 / (reps*cases.len()) as f64
    }
    #[test]
    fn round38_gcd_equivalence() {
        let mut comparisons=0usize;
        for p in [7u64,487,1_000_000_007,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            let mut replay=Replay::new(&field,&[1,1],42,limits(),limits(),deadline()).unwrap();
            let mut obs=|_:&BoundedRootEvent|Ok(());
            let mut producer=Producer {
                p,gate:Gate::new(limits(),deadline()),observer:&mut obs,
                unacknowledged:None,pending:[Poly17::ZERO;8],pending_len:0,
                roots:[0;8],root_count:0
            };
            for mode in 0..3 {
                for (a,b) in cases(p,mode,128) {
                    producer.gate.metrics=Metrics::default();
                    let old=producer.previous_gcd(a,b).unwrap();
                    producer.gate.metrics=Metrics::default();
                    let new=producer.gcd(a,b).unwrap();
                    replay.gate.metrics=Metrics::default();
                    let witness=replay.previous_monic_gcd(a,b).unwrap();
                    replay.gate.metrics=Metrics::default();
                    let verified=replay.monic_gcd(a,b).unwrap();
                    assert_eq!(old,new,"producer p={p} mode={mode} a={a:?} b={b:?}");
                    assert_eq!(old,witness,"independent replay p={p} mode={mode}");
                    assert_eq!(witness,verified,"replay p={p} mode={mode}");
                    comparisons+=3;
                }
            }
        }
        println!("ROUND38_EQ comparisons={comparisons}");
    }
    #[test]
    fn round38_full_solver_and_certificates() {
        for p in [17u64,487,1_000_000_007,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            for degree in [1usize,3,6,8] {
                let mut f=vec![1u64];
                for r in 1..=degree {
                    let mut next=vec![0u64;f.len()+1];
                    for (i,&c) in f.iter().enumerate() {
                        next[i]=((next[i] as u128+p as u128-
                             (c as u128*r as u128 % p as u128))%p as u128) as u64;
                        next[i+1]=(next[i+1]+c)%p;
                    }
                    f=next;
                }
                let mut events=Vec::new();
                let report=solve_bounded(&field,&f,42,limits(),deadline(),&mut |e|{
                    events.push(*e);Ok(())
                });
                assert_eq!(report.stop,Stop::Complete,"p={p} degree={degree}");
                let mut replay=Replay::new(&field,&f,42,limits(),limits(),deadline()).unwrap();
                for event in &events {replay.push(event).unwrap();}
                assert!(replay.finish(&report).unwrap().finite_complete);
            }
        }
        println!("ROUND38_ROOTS full producer+replay cases=16");
    }
    #[test]
    #[ignore="Compiled production-Rust paired before/after 13 trials"]
    fn bench_round38_gcd() {
        for p in [1_000_000_007u64,(1u64<<61)-1] {
            for mode in 0..3 {
                let c=cases(p,mode,64);
                for component in 0..2 {
                    let mut before=Vec::new();let mut after=Vec::new();
                    for trial in 0..13 {
                        let t=|fast| if component==0 {do_producer(p,&c,fast,5)}
                                      else {do_replay(p,&c,fast,5)};
                        if trial%2==0 {before.push(t(false)); after.push(t(true));}
                        else {after.push(t(true));before.push(t(false));}
                    }
                    before.sort_by(f64::total_cmp);after.sort_by(f64::total_cmp);
                    println!("ROUND38_RUST p={p} mode={mode} component={component} before={:.2}ns after={:.2}ns speedup={:.3}x",
                        before[6],after[6],before[6]/after[6]);
                }
            }
        }
    }
}
"""
header = """#![allow(dead_code)]
mod cryptanalysis {
  pub mod fp_backend {
    pub struct Fp64Arithmetic {pub p:u64}
    impl Fp64Arithmetic {pub fn p(&self)->u64{self.p} }
  }
  pub mod prime_root_trace {
    pub mod bounded {
"""
rust = header + source + "\n" + previous + tests + "\n} }\n}\n"
with tempfile.TemporaryDirectory() as tmp:
    path = Path(tmp)/"round38.rs"
    binary = Path(tmp)/"round38"
    path.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    for test in ["round38_gcd_equivalence","round38_full_solver_and_certificates"]:
        subprocess.run([str(binary),test,"--nocapture"],check=True)
    subprocess.run([str(binary),"bench_round38_gcd","--ignored","--nocapture"],check=True)
