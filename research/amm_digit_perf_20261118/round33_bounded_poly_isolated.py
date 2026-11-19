#!/usr/bin/env python3
"""Round 33: actual compiled-Rust polynomial A/B, with frozen pre-round33 code."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text()
production = source.split("\n#[cfg(test)]\n", 1)[0]

def method(code, name):
    at = code.index("    fn " + name + "(")
    op = code.index("{", at)
    depth = 0
    for j in range(op, len(code)):
        if code[j] == "{":
            depth += 1
        elif code[j] == "}":
            depth -= 1
            if depth == 0:
                return code[at:j + 1]
    raise RuntimeError(name)

old_div = r"""    fn division(&mut self, a: &Poly17, b: &Poly17) -> Result<(Poly17, Poly17)> {
        self.gate.charge(5 * COPY_WORK)?;
        if b.len == 0 {
            return Err(Stop::InvalidInput);
        }
        let mut remainder = *a;
        let mut quotient = Poly17::ZERO;
        if a.len < b.len {
            return Ok((quotient, remainder));
        }
        quotient.len = a.len - b.len + 1;
        let inv = self.inverse(b.coefficients[b.len as usize - 1])?;
        while remainder.len >= b.len {
            self.gate.charge(COPY_WORK)?;
            let shift = (remainder.len - b.len) as usize;
            let coefficient = self.mul(remainder.coefficients[remainder.len as usize - 1], inv)?;
            quotient.coefficients[shift] = coefficient;
            for j in 0..b.len as usize {
                let v = self.mul(coefficient, b.coefficients[j])?;
                remainder.coefficients[shift + j] =
                    self.sub(remainder.coefficients[shift + j], v)?;
            }
            remainder.trim();
        }
        quotient.trim();
        Ok((quotient, remainder))
    }"""
assert method(production, "division") != old_div
before = production.replace(method(production, "division"), old_div, 1)
assert "self.square(&b)?" in before
assert "self.square_diagonals(&acc)?" in before
before = before.replace("self.square(&b)?", "self.product(&b, &b)?", 1)
before = before.replace("self.square_diagonals(&acc)?", "self.convolution(&acc, &acc)?", 1)
# Direct method-level frozen reference in the actual producer impl as well.
before_method = old_div.replace("fn division(", "fn before_division(", 1)
production = production.replace("    fn division(", before_method + "\n    fn division(", 1)

tests = r"""
#[cfg(test)]
mod round33 {
    use super::*;
    use std::{hint::black_box, time::{Instant, Duration}};
    use crate::cryptanalysis::prime_root_trace::bounded_before as old;
    fn deadline()->Instant {Instant::now()+Duration::from_secs(180)}
    fn limits()->RootBudget {
        RootBudget{work:MAX_WORK,field_work:MAX_FIELD_WORK,bytes:STORAGE_BYTES,
            events:MAX_EVENTS,split_attempts:1000,degree:8}
    }
    fn next(s:&mut u64)->u64 {
        *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *s^(*s>>29)
    }
    fn poly(p:u64,len:usize,s:&mut u64,mode:usize)->Poly17 {
        let mut out=Poly17::ZERO;
        out.len=len as u8;
        for i in 0..len {
            out.coefficients[i]=next(s)%p;
            if mode==1 && i%2==0 {out.coefficients[i]=0;}
        }
        if len>0 {out.coefficients[len-1]=if mode==2 {1} else {1+next(s)%(p-1)};}
        out
    }
    fn producer<'a>(p:u64,sink:&'a mut dyn FnMut(&BoundedRootEvent)->std::result::Result<(),()>)->Producer<'a> {
        let mut e=Producer{p,gate:Gate::new(limits(),deadline()),observer:sink,
            unacknowledged:None,pending:[Poly17::ZERO;8],
            pending_len:0,roots:[0;8],root_count:0};
        e.gate.admit().unwrap();
        e
    }
    #[test]
    fn frozen_polynomial_equivalence() {
        let mut checks=0;
        for p in [17u64,487,1_000_000_007,4_294_967_291,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            let mut seed=0x1122_3311_2026_4444;
            let mut sink=|_:&BoundedRootEvent|Ok(());
            let mut e=producer(p,&mut sink);
            let mut r=Replay::new(&field,&[1,1],42,limits(),limits(),deadline()).unwrap();
            for len in 0..=9usize {
                for mode in 0..3 {
                    for _ in 0..64 {
                        e.gate.metrics=Metrics::default();
                        r.gate.metrics=Metrics::default();
                        let a=poly(p,len,&mut seed,mode);
                        assert_eq!(e.square(&a).unwrap(),e.product(&a,&a).unwrap());
                        assert_eq!(r.square_diagonals(&a).unwrap(),r.convolution(&a,&a).unwrap());
                        checks+=2;
                        if len>0 {
                            let length=1+(next(&mut seed) as usize%len);
                            let b=poly(p,length,&mut seed,(mode+1)%3);
                            let fast=e.division(&a,&b).unwrap();
                            let frozen=e.before_division(&a,&b).unwrap();
                            assert_eq!(fast,frozen,"p={p} a={a:?} b={b:?}");
                            checks+=1;
                        }
                        if e.gate.metrics.field_work>600_000 {e.gate.metrics=Metrics::default();}
                        if r.gate.metrics.field_work>600_000 {r.gate.metrics=Metrics::default();}
                    }
                }
            }
        }
        println!("ROUND33_RUST exact producer/replay comparisons={checks}");
    }
    fn coefficients(p:u64,degree:usize)->Vec<u64> {
        let mut c=vec![1u64];
        for root in 1..=degree {
            let root=root as u64 %p;
            let mut next_c=vec![0u64;c.len()+1];
            for j in 0..c.len() {
                next_c[j]=((next_c[j] as u128+p as u128-
                    (c[j] as u128*root as u128)%p as u128)%p as u128)as u64;
                next_c[j+1]=(next_c[j+1]+c[j])%p;
            }
            c=next_c;
        }
        c
    }
    #[test]
    fn whole_rootfinder_degree_1_to_8_and_replay() {
        let mut count=0;
        for p in [17u64,487,1_000_000_007,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            for degree in 1..=8 {
                let c=coefficients(p,degree);
                let mut events=Vec::new();
                let report=solve_bounded(&field,&c,42,limits(),deadline(),&mut |e|{
                    events.push(*e); Ok(())
                });
                let old_limits=old::RootBudget{
                    work:old::MAX_WORK,field_work:old::MAX_FIELD_WORK,
                    bytes:old::STORAGE_BYTES,events:old::MAX_EVENTS,
                    split_attempts:1000,degree:8};
                let mut previous_events=Vec::new();
                let previous=old::solve_bounded(&field,&c,42,old_limits,deadline(),&mut |e|{
                    previous_events.push(format!("{e:?}"));Ok(())
                });
                assert_eq!(format!("{:?}",report.stop),format!("{:?}",previous.stop),"p={p} degree={degree}");
                assert_eq!(report.roots,previous.roots);
                assert_eq!(report.root_count,previous.root_count);
                assert_eq!(report.metrics.splits,previous.metrics.splits);
                assert_eq!(events.iter().map(|e|format!("{e:?}")).collect::<Vec<_>>(),previous_events);
                if report.root_set_complete() {
                    let mut replay=Replay::new(&field,&c,42,limits(),limits(),deadline()).unwrap();
                    for e in &events {replay.push(e).unwrap();}
                    assert!(replay.finish(&report).unwrap().finite_complete);
                }
                count+=1;
            }
        }
        println!("ROUND33_RUST full solver degree1-8 cases={count}");
    }
    fn sample(p:u64,which:usize,fast:bool,reps:usize)->f64 {
        let field=Fp64Arithmetic{p};
        let mut sink=|_:&BoundedRootEvent|Ok(());
        let mut e=producer(p,&mut sink);
        let mut r=Replay::new(&field,&[1,1],42,limits(),limits(),deadline()).unwrap();
        let mut seed=0x1122_3355_7799_4455;
        let mut inputs=Vec::new();
        for i in 0..128 {
            let a=poly(p,8,&mut seed,i%3);
            let b=poly(p,1+(i%6),&mut seed,if i%2==0 {2} else {0});
            inputs.push((a,b));
        }
        let mut sum=0u64;
        let t=Instant::now();
        for _ in 0..reps {
            for (a,b) in &inputs {
                e.gate.metrics=Metrics::default();
                r.gate.metrics=Metrics::default();
                let out=match which {
                    0=>if fast {e.square(black_box(a)).unwrap()}
                       else {e.product(black_box(a),black_box(a)).unwrap()},
                    1=>if fast {r.square_diagonals(black_box(a)).unwrap()}
                       else {r.convolution(black_box(a),black_box(a)).unwrap()},
                    2=>if fast {e.division(black_box(a),black_box(b)).unwrap().1}
                       else {e.before_division(black_box(a),black_box(b)).unwrap().1},
                    _=>unreachable!()
                };
                sum=sum.wrapping_add(out.len as u64).wrapping_add(out.coefficients[0]);
            }
        }
        black_box(sum);
        t.elapsed().as_nanos() as f64/(reps*inputs.len()) as f64
    }
    fn whole(p:u64,degree:usize,fast:bool,reps:usize)->f64 {
        let f=Fp64Arithmetic{p};
        let c=coefficients(p,degree);
        let old_limits=old::RootBudget{
            work:old::MAX_WORK,field_work:old::MAX_FIELD_WORK,
            bytes:old::STORAGE_BYTES,events:old::MAX_EVENTS,
            split_attempts:1000,degree:8};
        let mut sum=0u64;
        let t=Instant::now();
        for _ in 0..reps {
            if fast {
                let r=solve_bounded(&f,black_box(&c),42,limits(),deadline(),&mut |_|Ok(()));
                sum=sum.wrapping_add(r.root_count as u64).wrapping_add(r.metrics.field_work);
            } else {
                let r=old::solve_bounded(&f,black_box(&c),42,old_limits,deadline(),&mut |_|Ok(()));
                sum=sum.wrapping_add(r.root_count as u64).wrapping_add(r.metrics.field_work);
            }
        }
        black_box(sum);
        t.elapsed().as_nanos() as f64/reps as f64
    }
    #[test]
    #[ignore="paired actual Rust performance benchmarks"]
    fn bench_paired_actual_rust() {
        for p in [487u64,1_000_000_007,(1u64<<61)-1] {
            for (name,which) in [("producer_square",0),("replay_square",1),("division",2)] {
                let mut a=Vec::new();let mut b=Vec::new();
                for i in 0..13 {
                    if i%2==0 {a.push(sample(p,which,false,24));b.push(sample(p,which,true,24));}
                    else {b.push(sample(p,which,true,24));a.push(sample(p,which,false,24));}
                }
                a.sort_by(f64::total_cmp);b.sort_by(f64::total_cmp);
                println!("ROUND33_RUST p={p} {name}: {:.2} -> {:.2} ns ({:.3}x)",a[6],b[6],a[6]/b[6]);
            }
            for degree in [3usize,4,6,8] {
                let mut a=Vec::new();let mut b=Vec::new();
                for i in 0..13 {
                    if i%2==0 {a.push(whole(p,degree,false,4));b.push(whole(p,degree,true,4));}
                    else {b.push(whole(p,degree,true,4));a.push(whole(p,degree,false,4));}
                }
                a.sort_by(f64::total_cmp);b.sort_by(f64::total_cmp);
                println!("ROUND33_WHOLE_RUST p={p} degree={degree}: {:.2} -> {:.2} ns ({:.3}x)",a[6],b[6],a[6]/b[6]);
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
rust = header + production + "\n" + tests + "\n} pub mod bounded_before {\n" + before + "\n}}\n}\n"
with tempfile.TemporaryDirectory() as tmp:
    f=Path(tmp)/"round33.rs"
    out=Path(tmp)/"round33"
    f.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(f),"-o",str(out)],check=True)
    subprocess.run([str(out),"frozen_polynomial_equivalence","--nocapture"],check=True)
    subprocess.run([str(out),"whole_rootfinder_degree_1_to_8_and_replay","--nocapture"],check=True)
    subprocess.run([str(out),"bench_paired_actual_rust","--ignored","--nocapture"],check=True)
