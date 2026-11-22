#!/usr/bin/env python3
"""Round 34: compiled production Rust, frozen pre-round34 methods, paired AB/BA timings."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text()
production = source.split("\n#[cfg(test)]\n", 1)[0]

def method(code, name):
    pos = code.index("    fn " + name + "(")
    start = code.index("{", pos)
    depth = 0
    for j in range(start, len(code)):
        if code[j] == "{":
            depth += 1
        elif code[j] == "}":
            depth -= 1
            if depth == 0:
                return code[pos:j+1]
    raise RuntimeError("unterminated " + name)

old_product = r"""    fn product(&mut self, a: &Poly17, b: &Poly17) -> Result<Poly17> {
        self.gate.charge(3 * COPY_WORK)?;
        let mut out = Poly17::ZERO;
        if a.len == 0 || b.len == 0 {
            return Ok(out);
        }
        let n = a.len as usize + b.len as usize - 1;
        if n > 17 {
            return Err(Stop::InvalidInput);
        }
        out.len = n as u8;
        for i in 0..a.len as usize {
            for j in 0..b.len as usize {
                let value = self.mul(a.coefficients[i], b.coefficients[j])?;
                out.coefficients[i + j] = self.add(out.coefficients[i + j], value)?;
            }
        }
        out.trim();
        Ok(out)
    }"""
old_reduce_monic = r"""    fn reduce_monic(&mut self, value: &mut Poly17, modulus: &Poly17) -> Result<()> {
        self.gate.charge(COPY_WORK)?;
        if modulus.len == 0 || modulus.coefficients[modulus.len as usize - 1] != 1 {
            return Err(Stop::InvalidInput);
        }
        while value.len >= modulus.len {
            self.gate.charge(COPY_WORK)?;
            let high = value.len as usize - 1;
            let coefficient = value.coefficients[high];
            let shift = (value.len - modulus.len) as usize;
            for j in 0..modulus.len as usize - 1 {
                let product = self.mul(coefficient, modulus.coefficients[j])?;
                value.coefficients[shift + j] = self.sub(value.coefficients[shift + j], product)?;
            }
            value.coefficients[high] = 0;
            value.trim();
        }
        Ok(())
    }"""
old_convolution = r"""    fn convolution(&mut self, x: &Poly17, y: &Poly17) -> Result<Poly17> {
        self.gate.charge(4 * COPY_WORK)?;
        let mut z = Poly17::ZERO;
        if x.len == 0 || y.len == 0 {
            return Ok(z);
        }
        let n = usize::from(x.len) + usize::from(y.len) - 1;
        if n > 17 {
            return Err(Stop::InvalidCertificate);
        }
        z.len = n as u8;
        // Output-diagonal order differs from producer's input-row order.
        for k in 0..n {
            self.gate.charge(32)?;
            let start = (k + 1).saturating_sub(usize::from(y.len));
            let end = usize::from(x.len).min(k + 1);
            for i in start..end {
                self.gate.charge(32)?;
                let v = self.times(x.coefficients[i], y.coefficients[k - i])?;
                z.coefficients[k] = self.plus(z.coefficients[k], v)?;
            }
        }
        z.trim();
        Ok(z)
    }"""
old_cancel_monic = r"""    fn cancel_monic(&mut self, numerator: &mut Poly17, denominator: &Poly17) -> Result<()> {
        self.gate.charge(COPY_WORK)?;
        let divisor_len = usize::from(denominator.len);
        if divisor_len == 0 || denominator.coefficients[divisor_len - 1] != 1 {
            return Err(Stop::InvalidCertificate);
        }
        // Independent descending-index traversal rather than producer trim loop.
        for high in (divisor_len - 1..usize::from(numerator.len)).rev() {
            self.gate.charge(COPY_WORK)?;
            let factor = numerator.coefficients[high];
            let offset = high + 1 - divisor_len;
            for low in 0..divisor_len - 1 {
                let term = self.times(factor, denominator.coefficients[low])?;
                numerator.coefficients[offset + low] =
                    self.minus(numerator.coefficients[offset + low], term)?;
            }
            numerator.coefficients[high] = 0;
        }
        numerator.trim();
        Ok(())
    }"""
methods = {"product": old_product, "reduce_monic": old_reduce_monic,
           "convolution": old_convolution, "cancel_monic": old_cancel_monic}
before = production
for name, old in methods.items():
    assert method(production, name) != old, name
    before = before.replace(method(before, name), old, 1)
    frozen = old.replace("fn " + name + "(", "fn frozen_" + name + "(", 1)
    production = production.replace(method(production, name),
                                    frozen + "\n" + method(production, name), 1)

tests = r"""
#[cfg(test)]
mod round34 {
    use super::*;
    use crate::cryptanalysis::prime_root_trace::bounded_before as old;
    use std::{hint::black_box, time::{Duration, Instant}};

    fn deadline()->Instant {Instant::now()+Duration::from_secs(240)}
    fn limits()->RootBudget {
        RootBudget{work:MAX_WORK, field_work:MAX_FIELD_WORK, bytes:STORAGE_BYTES,
            events:MAX_EVENTS, split_attempts:1000, degree:8}
    }
    fn next(s:&mut u64)->u64 {
        *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *s^(*s>>29)
    }
    fn poly(p:u64, len:usize, s:&mut u64, mode:usize, monic:bool)->Poly17 {
        let mut out=Poly17::ZERO;
        out.len=len as u8;
        for i in 0..len {
            out.coefficients[i]=next(s)%p;
            if mode==1 && i%2==0 {out.coefficients[i]=0;}
            if mode==2 && i%3!=1 {out.coefficients[i]=0;}
        }
        if len>0 {out.coefficients[len-1]=if monic {1} else {1+next(s)%(p-1)};}
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
    fn exact_sparse_dense_producer_replay_equivalence() {
        let mut checks=0usize;
        for p in [17u64, 487, 1_000_000_007, 4_294_967_291, (1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            let mut seed=0x1122_3344_2026_1211;
            let mut sink=|_:&BoundedRootEvent|Ok(());
            let mut e=producer(p,&mut sink);
            let mut r=Replay::new(&field,&[1,1],42,limits(),limits(),deadline()).unwrap();
            for mode in 0..3 {
                for i in 0..256 {
                    e.gate.metrics=Metrics::default();
                    r.gate.metrics=Metrics::default();
                    let len=1+(i%9);
                    let a=poly(p,len,&mut seed,mode,false);
                    let b=poly(p,1+(i%9),&mut seed,(mode+1)%3,false);
                    assert_eq!(e.product(&a,&b).unwrap(),e.frozen_product(&a,&b).unwrap());
                    assert_eq!(r.convolution(&a,&b).unwrap(),r.frozen_convolution(&a,&b).unwrap());
                    checks+=2;
                    let mut wide=e.product(&a,&b).unwrap();
                    let mut wide2=wide;
                    let m=poly(p,1+(i%8),&mut seed,mode,true);
                    e.reduce_monic(&mut wide,&m).unwrap();
                    e.frozen_reduce_monic(&mut wide2,&m).unwrap();
                    assert_eq!(wide,wide2,"producer reduction p={p} mode={mode}");
                    let mut replay=e.product(&a,&b).unwrap();
                    let mut replay2=replay;
                    r.cancel_monic(&mut replay,&m).unwrap();
                    r.frozen_cancel_monic(&mut replay2,&m).unwrap();
                    assert_eq!(replay,replay2,"replay reduction p={p} mode={mode}");
                    checks+=2;
                }
            }
        }
        println!("ROUND34_RUST exact producer/replay comparisons={checks}");
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
    fn whole_solver_root_events_and_independent_replay() {
        let mut count=0;
        for p in [17u64,487,1_000_000_007,(1u64<<61)-1] {
            let field=Fp64Arithmetic{p};
            for degree in 1..=8 {
                let c=coefficients(p,degree);
                let mut events=Vec::new();
                let report=solve_bounded(&field,&c,42,limits(),deadline(),&mut |e|{
                    events.push(*e);Ok(())
                });
                let old_limits=old::RootBudget{work:old::MAX_WORK,field_work:old::MAX_FIELD_WORK,
                    bytes:old::STORAGE_BYTES,events:old::MAX_EVENTS,
                    split_attempts:1000,degree:8};
                let mut previous_events=Vec::new();
                let previous=old::solve_bounded(&field,&c,42,old_limits,deadline(),&mut |e|{
                    previous_events.push(format!("{e:?}"));Ok(())
                });
                assert_eq!(format!("{:?}",report.stop),format!("{:?}",previous.stop));
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
        println!("ROUND34_RUST complete root solver cases={count}");
    }
    fn sample(p:u64, kind:usize, mode:usize, fast:bool, reps:usize)->f64 {
        let field=Fp64Arithmetic{p};
        let mut sink=|_:&BoundedRootEvent|Ok(());
        let mut e=producer(p,&mut sink);
        let mut r=Replay::new(&field,&[1,1],42,limits(),limits(),deadline()).unwrap();
        let mut seed=0x1122_3355_7799_4455;
        let mut inputs=Vec::new();
        for i in 0..128 {
            let a=poly(p,8,&mut seed,mode,false);
            let b=poly(p,8,&mut seed,mode,false);
            let modulus=poly(p,2+(i%7),&mut seed,mode,true);
            inputs.push((a,b,modulus));
        }
        let mut sum=0u64;
        let t=Instant::now();
        for _ in 0..reps {
            for (a,b,m) in &inputs {
                e.gate.metrics=Metrics::default();
                r.gate.metrics=Metrics::default();
                let out=match kind {
                    0=>if fast {e.product(black_box(a),black_box(b)).unwrap()}
                       else {e.frozen_product(black_box(a),black_box(b)).unwrap()},
                    1=>if fast {r.convolution(black_box(a),black_box(b)).unwrap()}
                       else {r.frozen_convolution(black_box(a),black_box(b)).unwrap()},
                    2=>{
                        let mut z=e.frozen_product(a,b).unwrap();
                        if fast {e.reduce_monic(&mut z,black_box(m)).unwrap();}
                        else {e.frozen_reduce_monic(&mut z,black_box(m)).unwrap();}
                        z
                    },
                    3=>{
                        let mut z=e.frozen_product(a,b).unwrap();
                        if fast {r.cancel_monic(&mut z,black_box(m)).unwrap();}
                        else {r.frozen_cancel_monic(&mut z,black_box(m)).unwrap();}
                        z
                    },
                    _=>unreachable!()
                };
                sum=sum.wrapping_add(out.len as u64).wrapping_add(out.coefficients[0]);
            }
        }
        black_box(sum);
        t.elapsed().as_nanos() as f64/(reps*inputs.len()) as f64
    }
    fn whole(p:u64, degree:usize, fast:bool, reps:usize)->f64 {
        let field=Fp64Arithmetic{p};
        let c=coefficients(p,degree);
        let old_limits=old::RootBudget{work:old::MAX_WORK,field_work:old::MAX_FIELD_WORK,
            bytes:old::STORAGE_BYTES,events:old::MAX_EVENTS,
            split_attempts:1000,degree:8};
        let mut sum=0u64;
        let t=Instant::now();
        for _ in 0..reps {
            if fast {
                let r=solve_bounded(&field,black_box(&c),42,limits(),deadline(),&mut |_|Ok(()));
                sum=sum.wrapping_add(r.root_count as u64).wrapping_add(r.metrics.field_work);
            } else {
                let r=old::solve_bounded(&field,black_box(&c),42,old_limits,deadline(),&mut |_|Ok(()));
                sum=sum.wrapping_add(r.root_count as u64).wrapping_add(r.metrics.field_work);
            }
        }
        black_box(sum);
        t.elapsed().as_nanos() as f64/reps as f64
    }
    #[test]
    #[ignore="compiled production Rust alternating A/B medians"]
    fn bench_sparse_dense_and_whole() {
        for p in [487u64,1_000_000_007,(1u64<<61)-1] {
            for mode in [0usize,1,2] {
                for (name,kind) in [("product",0),("replay_convolution",1),
                    ("reduce_monic",2),("replay_cancel",3)] {
                    let mut before=Vec::new();let mut after=Vec::new();
                    for i in 0..13 {
                        if i%2==0 {
                            before.push(sample(p,kind,mode,false,12));
                            after.push(sample(p,kind,mode,true,12));
                        } else {
                            after.push(sample(p,kind,mode,true,12));
                            before.push(sample(p,kind,mode,false,12));
                        }
                    }
                    before.sort_by(f64::total_cmp);after.sort_by(f64::total_cmp);
                    println!("ROUND34_RUST p={p} mode={mode} {name}: {:.2} -> {:.2} ns ({:.3}x)",
                        before[6],after[6],before[6]/after[6]);
                }
            }
            for degree in [3usize,6,8] {
                let mut before=Vec::new();let mut after=Vec::new();
                for i in 0..13 {
                    if i%2==0 {before.push(whole(p,degree,false,4));after.push(whole(p,degree,true,4));}
                    else {after.push(whole(p,degree,true,4));before.push(whole(p,degree,false,4));}
                }
                before.sort_by(f64::total_cmp);after.sort_by(f64::total_cmp);
                println!("ROUND34_WHOLE p={p} degree={degree}: {:.2} -> {:.2} ns ({:.3}x)",
                    before[6],after[6],before[6]/after[6]);
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
    path=Path(tmp)/"round34.rs"
    binary=Path(tmp)/"round34"
    path.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"exact_sparse_dense_producer_replay_equivalence","--nocapture"],check=True)
    subprocess.run([str(binary),"whole_solver_root_events_and_independent_replay","--nocapture"],check=True)
    subprocess.run([str(binary),"bench_sparse_dense_and_whole","--ignored","--nocapture"],check=True)
