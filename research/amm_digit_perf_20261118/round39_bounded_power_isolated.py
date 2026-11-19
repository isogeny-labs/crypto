#!/usr/bin/env python3
"""Round 39: compare the committed Rust producer/replay against exact pre-round39 power and linear-root code."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_bounded.rs").read_text().split("\n#[cfg(test)]\n", 1)[0]
before = source

def swap_once(text, newer, former):
    if text.count(newer) != 1:
        raise ValueError(f"expected unique code block, got {text.count(newer)}: {newer[:70]}")
    return text.replace(newer, former, 1)

before = swap_once(before, """        let mut acc = Poly17::constant(1);
        // The identity is reduced modulo any positive-degree polynomial.
        // A degree-zero modulus is exceptional: 1 mod c = 0.
        if monic.len == 1 {
            self.reduce_monic(&mut acc, &monic)?;
        }
        let mut acc_initialized = false;
        while exponent != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            if exponent & 1 != 0 {
                if acc_initialized {
                    let mut product = self.product(&acc, &b)?;
                    self.reduce_monic(&mut product, &monic)?;
                    acc = product;
                } else {
                    // First set bit: 1 * b mod m == b, already reduced.
                    acc = b;
                    acc_initialized = true;
                }
            }
            exponent >>= 1;
            if exponent != 0 {
                let mut product = self.square(&b)?;
                self.reduce_monic(&mut product, &monic)?;
                b = product;
            }
        }""", """        let mut acc = Poly17::constant(1);
        self.reduce_monic(&mut acc, &monic)?;
        while exponent != 0 {
            self.gate.charge(4 * COPY_WORK)?;
            if exponent & 1 != 0 {
                let mut product = self.product(&acc, &b)?;
                self.reduce_monic(&mut product, &monic)?;
                acc = product;
            }
            exponent >>= 1;
            if exponent != 0 {
                let mut product = self.square(&b)?;
                self.reduce_monic(&mut product, &monic)?;
                b = product;
            }
        }""")
before = swap_once(before, """                let root = if h.coefficients[1] == 1 {
                    // Split factors are normally monic: x + c has root -c.
                    self.sub(0, h.coefficients[0])?
                } else {
                    let inv = self.inverse(h.coefficients[1])?;
                    let value = self.mul(h.coefficients[0], inv)?;
                    self.sub(0, value)?
                };""", """                let inv = self.inverse(h.coefficients[1])?;
                let value = self.mul(h.coefficients[0], inv)?;
                let root = self.sub(0, value)?;""")
before = swap_once(before, """        // The top exponent bit is always 1. Initialize from the already
        // reduced base instead of squaring 1 and multiplying by base.
        let mut acc = if exponent == 0 { Poly17::constant(1) } else { base };
        if exponent == 0 && monic.len == 1 {
            self.cancel_monic(&mut acc, &monic)?;
        }
        let remaining_bits = (64 - exponent.leading_zeros()).saturating_sub(1);
        for bit in (0..remaining_bits).rev() {
            self.gate.charge(4 * COPY_WORK)?;
            let mut square = self.square_diagonals(&acc)?;
            self.cancel_monic(&mut square, &monic)?;
            acc = square;
            if exponent & (1u64 << bit) != 0 {
                let mut product = self.convolution(&acc, &base)?;
                self.cancel_monic(&mut product, &monic)?;
                acc = product;
            }
        }""", """        let mut acc = Poly17::constant(1);
        self.cancel_monic(&mut acc, &monic)?;
        for bit in (0..64 - exponent.leading_zeros()).rev() {
            self.gate.charge(4 * COPY_WORK)?;
            let mut square = self.square_diagonals(&acc)?;
            self.cancel_monic(&mut square, &monic)?;
            acc = square;
            if exponent & (1u64 << bit) != 0 {
                let mut product = self.convolution(&acc, &base)?;
                self.cancel_monic(&mut product, &monic)?;
                acc = product;
            }
        }""")
before = swap_once(before, """                let v = if polynomial.coefficients[1] == 1 {
                    root
                } else {
                    self.times(polynomial.coefficients[1], root)?
                };
                if self.plus(v, polynomial.coefficients[0])? != 0 {""", """                let v = self.times(polynomial.coefficients[1], root)?;
                if self.plus(v, polynomial.coefficients[0])? != 0 {""")

header = """#![allow(dead_code)]
mod cryptanalysis {
 pub mod fp_backend {
  pub struct Fp64Arithmetic {pub p:u64}
  impl Fp64Arithmetic {pub fn p(&self)->u64{self.p}}
 }
 pub mod prime_root_trace {
  pub mod after {
"""
middle = "\n  } pub mod before {\n"
tail = "\n  }\n }\n}\n"
probes = r"""
pub fn round39_probe(p:u64,coefficients:&[u64],delta:u64,exponent:u64)->(Vec<u64>,Vec<u64>){
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
    let generated=producer.power(&base,exponent,&modulus).unwrap();
    let mut replay=Replay::new(&field,&[1],42,limits,limits,deadline).unwrap();
    let checked=replay.modular_power(base,exponent,&modulus).unwrap();
    (generated.coefficients().to_vec(),checked.coefficients().to_vec())
}
"""

tests = r"""
#[cfg(test)]
mod round39 {
    use super::cryptanalysis::{fp_backend::Fp64Arithmetic, prime_root_trace::{after,before}};
    use std::{hint::black_box,time::{Instant,Duration}};

    fn polynomial(p:u64, degree:usize)->Vec<u64> {
        let mut f=vec![1u64];
        for root in 1..=degree {
            let mut next=vec![0;f.len()+1];
            for (i,&coefficient) in f.iter().enumerate(){
                next[i]=((next[i] as u128 + p as u128 -
                    coefficient as u128*root as u128 % p as u128)%p as u128) as u64;
                next[i+1]=(next[i+1]+coefficient)%p;
            }
            f=next;
        }
        f
    }
    fn budgets_after()->after::RootBudget {
        after::RootBudget {work:after::MAX_WORK,field_work:after::MAX_FIELD_WORK,
            bytes:after::STORAGE_BYTES,events:after::MAX_EVENTS,split_attempts:1000,degree:8}
    }
    fn budgets_before()->before::RootBudget {
        before::RootBudget {work:before::MAX_WORK,field_work:before::MAX_FIELD_WORK,
            bytes:before::STORAGE_BYTES,events:before::MAX_EVENTS,split_attempts:1000,degree:8}
    }
    fn run_after(p:u64,f:&[u64],seed:u64,compare:bool)->(u64,Vec<String>){
        let field=Fp64Arithmetic{p};
        let deadline=Instant::now()+Duration::from_secs(120);
        let limits=budgets_after();
        let mut events=Vec::new();
        let report=after::solve_bounded(&field,black_box(f),seed,limits,deadline,
            &mut |e|{events.push(*e);Ok(())});
        assert!(report.root_set_complete()||report.all_field_elements(),"after stop={:?}",report.stop);
        let mut replay=after::Replay::new(&field,f,seed,limits,limits,deadline).unwrap();
        for e in &events {replay.push(e).unwrap();}
        let verified=replay.finish(&report).unwrap();
        let digest=report.root_count as u64 + verified.checked_root_count as u64+
           report.roots[0] + events.len() as u64;
        let kinds=if compare {events.iter().map(|e|format!("{e:?}")).collect()} else {Vec::new()};
        (black_box(digest),kinds)
    }
    fn run_before(p:u64,f:&[u64],seed:u64,compare:bool)->(u64,Vec<String>){
        let field=Fp64Arithmetic{p};
        let deadline=Instant::now()+Duration::from_secs(120);
        let limits=budgets_before();
        let mut events=Vec::new();
        let report=before::solve_bounded(&field,black_box(f),seed,limits,deadline,
            &mut |e|{events.push(*e);Ok(())});
        assert!(report.root_set_complete()||report.all_field_elements(),"before stop={:?}",report.stop);
        let mut replay=before::Replay::new(&field,f,seed,limits,limits,deadline).unwrap();
        for e in &events {replay.push(e).unwrap();}
        let verified=replay.finish(&report).unwrap();
        let digest=report.root_count as u64 + verified.checked_root_count as u64+
           report.roots[0] + events.len() as u64;
        let kinds=if compare {events.iter().map(|e|format!("{e:?}")).collect()} else {Vec::new()};
        (black_box(digest),kinds)
    }
    #[test]
    fn polynomial_power_edge_exponents_match_frozen() {
        let mut comparisons=0usize;
        for p in [17u64,487,1_000_000_007,(1u64<<61)-1] {
            for degree in 0usize..=8 {
                let mut m=if degree==0 {vec![3]}else{vec![0;degree+1]};
                if degree>0 {
                    m[0]=3;
                    if degree>1 {m[1]=2;}
                    m[degree]=1;
                }
                for delta in [0u64,1,5] {
                    for exponent in [0u64,1,2,3,7,(p-1)/2,p] {
                        let old=before::round39_probe(p,&m,delta,exponent);
                        let new=after::round39_probe(p,&m,delta,exponent);
                        assert_eq!(old,new,"p={p} deg={degree} delta={delta} exp={exponent}");
                        comparisons+=2;
                    }
                }
            }
        }
        println!("ROUND39_POWER_EQ producer_and_replay_checks={comparisons}");
    }
    #[test]
    fn roots_and_certificate_events_are_equal() {
        let mut count=0usize;
        for p in [17u64,487,1_000_000_007,(1u64<<61)-1] {
            for degree in [0usize,1,2,3,4,6,8] {
                for seed in [1u64,42,0x5291ae77] {
                    let poly=if degree==0{vec![3]}else{polynomial(p,degree)};
                    let old=run_before(p,&poly,seed,true);
                    let new=run_after(p,&poly,seed,true);
                    assert_eq!(old,new,"p={p} degree={degree} seed={seed}");
                    count+=1;
                }
            }
            let old=run_before(p,&[],42,true);
            let new=run_after(p,&[],42,true);
            assert_eq!(old,new,"zero polynomial p={p}");
            count+=1;
        }
        println!("ROUND39_EQ complete_root_and_certificate_cases={count}");
    }
    fn timed(p:u64,f:&[u64],seed:u64,old:bool,reps:usize)->f64{
        let start=Instant::now();let mut digest=0u64;
        for _ in 0..reps {
            digest=digest.wrapping_add(if old {
                run_before(p,f,seed,false).0
            } else {
                run_after(p,f,seed,false).0
            });
        }
        black_box(digest);
        start.elapsed().as_nanos() as f64/reps as f64
    }
    #[test]
    #[ignore="Native production Rust complete solver plus replay A/B"]
    fn bench_full_root_extraction() {
        for p in [487u64,1_000_000_007,(1u64<<61)-1] {
            for degree in [3usize,6,8] {
                let f=polynomial(p,degree);
                let mut old=Vec::new();let mut new=Vec::new();
                for trial in 0..13 {
                    if trial%2==0 {
                        old.push(timed(p,&f,42,true,3));new.push(timed(p,&f,42,false,3));
                    } else {
                        new.push(timed(p,&f,42,false,3));old.push(timed(p,&f,42,true,3));
                    }
                }
                old.sort_by(f64::total_cmp);new.sort_by(f64::total_cmp);
                println!("ROUND39_WHOLE p={p} degree={degree} before_ns={:.1} after_ns={:.1} speedup={:.4}x",
                    old[6],new[6],old[6]/new[6]);
            }
        }
    }
}
"""
with tempfile.TemporaryDirectory() as folder:
    path=Path(folder)/"round39.rs"
    binary=Path(folder)/"round39"
    path.write_text(header+source+probes+middle+before+probes+tail+tests)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"polynomial_power_edge_exponents_match_frozen","--nocapture"],check=True)
    subprocess.run([str(binary),"roots_and_certificate_events_are_equal","--nocapture"],check=True)
    subprocess.run([str(binary),"bench_full_root_extraction","--ignored","--nocapture"],check=True)
