#!/usr/bin/env python3
"""Standalone native Rust A/B for exact field_value function extracted from production."""
from pathlib import Path
import re
import subprocess
import tempfile

SOURCE = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
start = SOURCE.index("#[inline]\nfn field_value(")
end = SOURCE.index("\nfn validate_input(", start)
field_function = SOURCE[start:end]
assert "reciprocal_64" in field_function and "let quotient" in field_function

program = r"""
use std::hint::black_box;
use std::time::Instant;
const MAX_PRIME: u64 = (1u64 << 61) - 1;
#[derive(Clone, Copy, Debug)]
enum FieldOp { Add, Sub, Mul, Square }
""" + field_function + r"""

fn rng(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn sample(items: &[(FieldOp,u64,u64)], p:u64, mu:u64) -> (f64,u64) {
    let start=Instant::now();
    let mut checksum=0u64;
    for _ in 0..32 {
        for &(op,a,b) in items {
            let result=field_value(&black_box(op),black_box(a),black_box(b),black_box(p),black_box(mu));
            checksum=checksum.wrapping_add(result ^ (result>>7));
        }
    }
    black_box(checksum);
    (start.elapsed().as_nanos() as f64/(32*items.len()) as f64,checksum)
}
fn main() {
    let mut seed=0x3fe6_897c_573a_a102u64;
    println!("p,mode,trial,variant,ns_per_op,checksum");
    let mut checks=0u64;
    for p in [65_521u64,1_000_000_007,4_294_967_291,MAX_PRIME] {
        let mu=if p<=u32::MAX as u64 {((1u128<<64)/p as u128) as u64} else {0};
        for mode in 0..4 {
            let mut items=Vec::new();
            for i in 0..4096 {
                let op=match mode {
                    0=>FieldOp::Mul, 1=>FieldOp::Square,
                    2=>match i%4 {0=>FieldOp::Add,1=>FieldOp::Sub,2=>FieldOp::Mul,_=>FieldOp::Square},
                    _=>match i%10 {0|1|2|3|4=>FieldOp::Mul,5|6|7=>FieldOp::Square,8=>FieldOp::Add,_=>FieldOp::Sub}
                };
                let a=rng(&mut seed)%p;
                let b=if matches!(op,FieldOp::Square){a}else{rng(&mut seed)%p};
                let baseline=field_value(&op,a,b,p,0);
                let after=field_value(&op,a,b,p,mu);
                let reference=match op {
                    FieldOp::Add => ((a as u128+b as u128)%p as u128) as u64,
                    FieldOp::Sub => ((a as u128+p as u128-b as u128)%p as u128) as u64,
                    _ => ((a as u128*b as u128)%p as u128) as u64,
                };
                assert_eq!(baseline,reference,"baseline p={p} a={a} b={b}");
                assert_eq!(after,reference,"optimized p={p} a={a} b={b}");
                checks+=1;
                items.push((op,a,b));
            }
            for trial in 0..13 {
                for order in 0..2 {
                    let is_after=(trial+order)%2==1;
                    let (time,checksum)=sample(&items,p,if is_after {mu}else{0});
                    println!("{p},{mode},{trial},{},{time:.4},{checksum}",
                             if is_after {"after"} else {"before"});
                }
            }
        }
    }
    eprintln!("source-exact native Rust field equivalence cases: {checks}");
}
"""
with tempfile.TemporaryDirectory() as directory:
    source_file=Path(directory)/"bench.rs"
    executable=Path(directory)/"bench"
    source_file.write_text(program)
    subprocess.run(["rustc","--edition=2021","-O","-C","target-cpu=native",
                    str(source_file),"-o",str(executable)],check=True)
    subprocess.run([str(executable)],check=True)
