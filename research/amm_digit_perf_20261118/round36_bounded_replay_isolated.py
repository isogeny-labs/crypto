#!/usr/bin/env python3
"""Round 36: compile exact production bounded root replay, run frozen baseline A/B tests."""
from pathlib import Path
import subprocess
import tempfile

production = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
old_methods = r"""
impl Replay {
    // Frozen pre-round36 independent verifier implementations.
    fn before_remainder(&mut self, numerator: Poly17, denominator: &Poly17) -> Result<Poly17> {
        self.gate.charge(4 * COPY_WORK)?;
        if denominator.len == 0 { return Err(Stop::InvalidCertificate); }
        let mut r = numerator;
        if r.len < denominator.len { return Ok(r); }
        let inverse = self.reciprocal(denominator.coefficients[usize::from(denominator.len) - 1])?;
        for high in (usize::from(denominator.len) - 1..usize::from(numerator.len)).rev() {
            self.gate.charge(COPY_WORK)?;
            let c = self.times(r.coefficients[high], inverse)?;
            let shift = high + 1 - usize::from(denominator.len);
            for j in 0..usize::from(denominator.len) {
                let v = self.times(c, denominator.coefficients[j])?;
                r.coefficients[shift + j] = self.minus(r.coefficients[shift + j], v)?;
            }
        }
        r.trim();
        Ok(r)
    }
    fn before_square_diagonals(&mut self, x: &Poly17) -> Result<Poly17> {
        self.gate.charge(4 * COPY_WORK)?;
        let mut z = Poly17::ZERO;
        if x.len == 0 { return Ok(z); }
        let len = usize::from(x.len);
        let n = 2 * len - 1;
        if n > 17 { return Err(Stop::InvalidCertificate); }
        z.len = n as u8;
        for k in 0..n {
            self.gate.charge(32)?;
            let start = (k + 1).saturating_sub(len);
            let end = len.min(k + 1);
            for i in start..end {
                let j = k - i;
                if i > j { break; }
                self.gate.charge(32)?;
                let v = self.times(x.coefficients[i], x.coefficients[j])?;
                let term = if i == j { v } else { self.plus(v, v)? };
                z.coefficients[k] = self.plus(z.coefficients[k], term)?;
            }
        }
        z.trim();
        Ok(z)
    }
    fn before_monic_gcd(&mut self, left: Poly17, right: Poly17) -> Result<Poly17> {
        self.gate.charge(5 * COPY_WORK)?;
        let mut pair = [left, right];
        while pair[1].len != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            let next = self.before_remainder(pair[0], &pair[1])?;
            pair = [pair[1], next];
        }
        let mut out = pair[0];
        if out.len > 0 {
            let inv = self.reciprocal(out.coefficients[usize::from(out.len) - 1])?;
            for i in 0..usize::from(out.len) {
                out.coefficients[i] = self.times(out.coefficients[i], inv)?;
            }
        }
        Ok(out)
    }
}
"""
def method(code, name):
    start = code.index("    fn " + name + "(")
    brace = code.index("{", start)
    depth = 0
    for i in range(brace, len(code)):
        if code[i] == "{": depth += 1
        elif code[i] == "}":
            depth -= 1
            if depth == 0: return code[start:i+1]
    raise RuntimeError(name)

# Frozen pre-change verifier is built from the *same* production source,
# reverting only the three edited functions. All other optimizations stay.
before = production
for name in ["remainder", "square_diagonals", "monic_gcd"]:
    old = method(old_methods, "before_" + name)
    old = old.replace("fn before_" + name + "(", "fn " + name + "(", 1)
    if name == "monic_gcd":
        old = old.replace("self.before_remainder(", "self.remainder(")
    before = before.replace(method(before, name), old, 1)

tests = r"""
#[cfg(test)]
mod round36 {
    use super::*;
    use std::{hint::black_box, time::{Duration, Instant}};
    fn limits()->RootBudget { RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,
        bytes:STORAGE_BYTES,events:MAX_EVENTS,split_attempts:1000,degree:8} }
    fn deadline()->Instant { Instant::now()+Duration::from_secs(180) }
    fn next(s:&mut u64)->u64 { *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *s^(*s>>30) }
    fn poly(p:u64,len:usize,s:&mut u64,mode:usize,monic:bool)->Poly17 {
        let mut x=Poly17::ZERO;
        x.len=len as u8;
        for i in 0..len {
            let v=next(s)%p;
            x.coefficients[i]=if (mode==1 && i%2==0) || (mode==2 && i%3!=1) {0} else {v};
        }
        if len>0 {x.coefficients[len-1]=if monic{1}else{1+next(s)%(p-1)};}
        x
    }
    fn replay(p:u64)->Replay {
        let field=Fp64Arithmetic{p};
        Replay::new(&field,&[1,1],42,limits(),limits(),deadline()).unwrap()
    }
    #[test]
    fn exact_replay_equivalence_and_budget_refusals() {
        let mut checks=0usize;
        for p in [17u64,487,1_000_000_007,4_294_967_291,(1u64<<61)-1] {
            let mut r=replay(p);
            let mut seed=0x1234_5678_abc1_2299;
            for mode in 0..3 {
                for degree in 1..=9 {
                    for _ in 0..32 {
                        let a=poly(p,degree,&mut seed,mode,false);
                        let b=poly(p,1+next(&mut seed) as usize%degree,&mut seed,mode,mode!=2);
                        r.gate.metrics=Metrics::default();
                        let expected=r.before_remainder(a,&b).unwrap();
                        r.gate.metrics=Metrics::default();
                        let actual=r.remainder(a,&b).unwrap();
                        assert_eq!(actual,expected,"remainder p={p} mode={mode} a={a:?} b={b:?}");
                        r.gate.metrics=Metrics::default();
                        let expected=r.before_square_diagonals(&a).unwrap();
                        r.gate.metrics=Metrics::default();
                        let actual=r.square_diagonals(&a).unwrap();
                        assert_eq!(actual,expected,"square p={p} mode={mode}");
                        r.gate.metrics=Metrics::default();
                        let expected=r.before_monic_gcd(a,b).unwrap();
                        r.gate.metrics=Metrics::default();
                        let actual=r.monic_gcd(a,b).unwrap();
                        assert_eq!(actual,expected,"gcd p={p} mode={mode}");
                        checks+=3;
                    }
                }
            }
        }
        println!("ROUND36_RUST equivalence comparisons={checks}");
    }
    fn coefficients(p:u64,degree:usize)->Vec<u64> {
        let mut c=vec![1u64];
        for root in 1..=degree {
            let root=root as u64 %p;
            let mut next_c=vec![0u64;c.len()+1];
            for j in 0..c.len() {
                let prod=(u128::from(c[j])*u128::from(root))%u128::from(p);
                next_c[j]=((u128::from(next_c[j])+u128::from(p)-prod)%u128::from(p)) as u64;
                next_c[j+1]=(next_c[j+1]+c[j])%p;
            }
            c=next_c;
        }
        c
    }
    #[test]
    fn full_degree_one_to_eight_certificate_replay() {
        let mut count=0usize;
        for p in [17u64,487,1_000_000_007,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            for degree in 1..=8 {
                let coefficients=coefficients(p,degree);
                let mut events=Vec::new();
                let report=solve_bounded(&field,&coefficients,42,limits(),deadline(),&mut |e|{
                    events.push(*e);Ok(())
                });
                assert_eq!(report.stop,Stop::Complete,"p={p} degree={degree}");
                let mut r=Replay::new(&field,&coefficients,42,limits(),limits(),deadline()).unwrap();
                for event in &events { r.push(event).unwrap(); }
                let checked=r.finish(&report).unwrap();
                assert_eq!(checked.checked_root_count as usize,degree);
                count+=1;
            }
        }
        println!("ROUND36_RUST full root + independent replay cases={count}");
    }
    fn whole_replay(p:u64,degree:usize,fast:bool,reps:usize)->f64 {
        use crate::cryptanalysis::prime_root_trace::bounded_before as old;
        let field=Fp64Arithmetic{p};
        let c=coefficients(p,degree);
        let mut sum=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            if fast {
                let mut events=Vec::new();
                let report=solve_bounded(&field,black_box(&c),42,limits(),deadline(),&mut |event|{
                    events.push(*event);Ok(())
                });
                assert_eq!(report.stop,Stop::Complete);
                let mut replay=Replay::new(&field,&c,42,limits(),limits(),deadline()).unwrap();
                for e in &events {replay.push(e).unwrap();}
                sum=sum.wrapping_add(replay.finish(&report).unwrap().checked_root_count as u64);
            } else {
                let limits=old::RootBudget{work:old::MAX_WORK,field_work:old::MAX_FIELD_WORK,
                    bytes:old::STORAGE_BYTES,events:old::MAX_EVENTS,
                    split_attempts:1000,degree:8};
                let mut events=Vec::new();
                let report=old::solve_bounded(&field,black_box(&c),42,limits,deadline(),&mut |event|{
                    events.push(*event);Ok(())
                });
                assert_eq!(report.stop,old::Stop::Complete);
                let mut replay=old::Replay::new(&field,&c,42,limits,limits,deadline()).unwrap();
                for e in &events {replay.push(e).unwrap();}
                sum=sum.wrapping_add(replay.finish(&report).unwrap().checked_root_count as u64);
            }
        }
        black_box(sum);
        start.elapsed().as_nanos() as f64/reps as f64
    }
    #[test]
    #[ignore="full root solve + certificate replay before/after"]
    fn bench_round36_complete_replay() {
        for p in [487u64,1_000_000_007,(1u64<<61)-1] {
            for degree in [3usize,6,8] {
                let mut before=Vec::new();let mut after=Vec::new();
                for i in 0..13 {
                    if i%2==0 {
                        before.push(whole_replay(p,degree,false,6));
                        after.push(whole_replay(p,degree,true,6));
                    } else {
                        after.push(whole_replay(p,degree,true,6));
                        before.push(whole_replay(p,degree,false,6));
                    }
                }
                before.sort_by(f64::total_cmp);after.sort_by(f64::total_cmp);
                println!("ROUND36_WHOLE p={p} degree={degree} solve+replay: {:.2} -> {:.2} ns ({:.3}x)",
                    before[6],after[6],before[6]/after[6]);
            }
        }
    }
    fn sample(p:u64,mode:usize,kind:usize,fast:bool,reps:usize)->f64 {
        let mut seed=0x1122_9911_7223_4555;
        let mut work=Vec::new();
        for i in 0..128 {
            let a=poly(p,6+i%3,&mut seed,mode,false);
            let b=poly(p,if mode==3{1}else{2+i%5},&mut seed,mode,mode!=3);
            work.push((a,b));
        }
        let mut r=replay(p);
        let mut sum=0u64;
        let t=Instant::now();
        for _ in 0..reps {
            for (a,b) in &work {
                r.gate.metrics=Metrics::default();
                let out=match kind {
                    0=>if fast{r.remainder(black_box(*a),black_box(b)).unwrap()}
                       else{r.before_remainder(black_box(*a),black_box(b)).unwrap()},
                    1=>if fast{r.square_diagonals(black_box(a)).unwrap()}
                       else{r.before_square_diagonals(black_box(a)).unwrap()},
                    2=>if fast{r.monic_gcd(black_box(*a),black_box(*b)).unwrap()}
                       else{r.before_monic_gcd(black_box(*a),black_box(*b)).unwrap()},
                    _=>unreachable!()
                };
                sum=sum.wrapping_add(out.coefficients[0]).wrapping_add(out.len as u64);
            }
        }
        black_box(sum);
        t.elapsed().as_nanos() as f64/(reps*work.len()) as f64
    }
    #[test]
    #[ignore="production Rust paired A/B microbenchmarks"]
    fn bench_round36_replay() {
        for p in [487u64,1_000_000_007,(1u64<<61)-1] {
            for mode in 0usize..4 {
                for (name,kind) in [("remainder",0usize),("square",1),("gcd",2)] {
                    let mut before=Vec::new();let mut after=Vec::new();
                    for i in 0..13 {
                        if i%2==0 {
                            before.push(sample(p,mode,kind,false,8));
                            after.push(sample(p,mode,kind,true,8));
                        } else {
                            after.push(sample(p,mode,kind,true,8));
                            before.push(sample(p,mode,kind,false,8));
                        }
                    }
                    before.sort_by(f64::total_cmp);after.sort_by(f64::total_cmp);
                    println!("ROUND36_RUST p={p} mode={mode} {name}: {:.2} -> {:.2} ns ({:.3}x)",
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
        impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
    }
    pub mod prime_root_trace {
        pub mod bounded {
"""
rust = header + production + "\n" + old_methods + tests + "\n} pub mod bounded_before {\n" + before + "\n} }\n}\n"
with tempfile.TemporaryDirectory() as tmp:
    path = Path(tmp)/"round36.rs"
    binary = Path(tmp)/"round36"
    path.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    for test in ["exact_replay_equivalence_and_budget_refusals",
                 "full_degree_one_to_eight_certificate_replay"]:
        subprocess.run([str(binary),test,"--nocapture"],check=True)
    subprocess.run([str(binary),"bench_round36_replay","--ignored","--nocapture"],check=True)
    subprocess.run([str(binary),"bench_round36_complete_replay","--ignored","--nocapture"],check=True)
