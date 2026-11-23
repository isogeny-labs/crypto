#!/usr/bin/env python3
"""Rust source-exact 32-bit Barrett cross-check plus paired A/B microbench.

Extracts the actual producer field arithmetic functions. The sparse polynomial
loops are covered by the Rust unit tests and separate local C++ reproductions.
"""
from pathlib import Path
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[2]
source = (repo / "src/cryptanalysis/prime_root_trace.rs").read_text()

def extract(signature):
    begin = source.index(signature)
    brace = source.index("{", begin)
    nesting = 0
    for i in range(brace, len(source)):
        if source[i] == "{":
            nesting += 1
        elif source[i] == "}":
            nesting -= 1
            if nesting == 0:
                return source[begin:i + 1]
    raise ValueError("unclosed source function " + signature)

old = extract("fn field_value(op: &FieldOp, a: u64, b: u64, p: u64)")
new = extract("fn field_value_cached(op: &FieldOp, a: u64, b: u64, p: u64, reciprocal32: u64)")
main = r"""
use std::hint::black_box;
use std::time::Instant;
const MAX_PRIME: u64 = (1u64 << 61) - 1;
#[derive(Clone,Copy,Debug)]
enum FieldOp { Add, Sub, Mul, Square }
@@FIELD@@
@@CACHED@@
fn next(s:&mut u64)->u64 {
    *s=s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*s
}
fn measure(p:u64,recip:u64,pairs:&[(u64,u64)],fast:bool)->f64 {
    let start=Instant::now();let mut sum=0u64;
    for _ in 0..32 {for &(a,b) in pairs {
        let (a,b)=(black_box(a),black_box(b));
        sum^=if fast {field_value_cached(&FieldOp::Mul,a,b,p,recip)}
              else {field_value(&FieldOp::Mul,a,b,p)};
    }}
    black_box(sum);
    start.elapsed().as_nanos() as f64/(32*pairs.len()) as f64
}
fn main(){
    println!("prime,baseline_ns,candidate_ns,speedup");
    let mut seed=0x7520_2611_2026_0001u64;
    for p in [3,65521,998244353,1000000007,4294967291,MAX_PRIME] {
        let recip=if p<=u32::MAX as u64 {((1u128<<64)/p as u128) as u64} else {0};
        let mut pairs=Vec::new();
        for _ in 0..8192 {
            let a=next(&mut seed)%p;let b=next(&mut seed)%p;
            for op in [FieldOp::Add,FieldOp::Sub,FieldOp::Mul,FieldOp::Square] {
                assert_eq!(field_value(&op,a,b,p),field_value_cached(&op,a,b,p,recip));
                assert_eq!(field_value(&op,a+p,b,p),field_value_cached(&op,a+p,b,p,recip));
            }
            pairs.push((a,b));
        }
        let(mut before,mut after)=(Vec::new(),Vec::new());
        for trial in 0..13{
            if trial%2==0 {before.push(measure(p,recip,&pairs,false));after.push(measure(p,recip,&pairs,true));}
            else {after.push(measure(p,recip,&pairs,true));before.push(measure(p,recip,&pairs,false));}
        }
        before.sort_by(f64::total_cmp);after.sort_by(f64::total_cmp);
        println!("{p},{:.4},{:.4},{:.3}",before[6],after[6],before[6]/after[6]);
    }
    println!("native Rust: 393216 exact field-operation comparisons passed");
}
"""
rust = main.replace("@@FIELD@@", old).replace("@@CACHED@@", new)
with tempfile.TemporaryDirectory() as directory:
    file = Path(directory) / "round75.rs"
    binary = Path(directory) / "round75"
    file.write_text(rust)
    subprocess.run(["rustc", "--edition=2021", "-O", "-C", "target-cpu=native", "-o", str(binary), str(file)], check=True)
    subprocess.run([str(binary)], check=True)
