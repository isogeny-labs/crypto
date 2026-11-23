#!/usr/bin/env python3
"""Compile and test the *exact* production trace field_value body in isolation.

The full prime_root_trace.rs module requires its original dependency tree.
This script extracts only field_value (unaltered), supplies its enum/constant,
and builds release Rust unit tests with rustc, without mimicking the source.
"""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
start = source.index("/// Accelerated arithmetic for canonical public field elements")
end = source.index("\nfn validate_input(", start)
body = source[start:end]
assert "fn field_value(op: &FieldOp" in body
prefix = """\
const MAX_PRIME: u64 = (1u64 << 61) - 1;
#[derive(Clone, Copy, Debug)]
enum FieldOp { Add, Sub, Mul, Square }
"""
test = r"""
#[test]
fn trace_field_matches_original() {
    let mut seed = 0x7452_4132_2026_1118u64;
    for p in [3, 487, 17497, 998244353, 1000000007,
              MAX_PRIME - 30, MAX_PRIME] {
        for _ in 0..8192 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = seed % p;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let b = seed % p;
            for op in [FieldOp::Add, FieldOp::Sub, FieldOp::Mul, FieldOp::Square] {
                let expected = original(op, a, b, p);
                assert_eq!(field_value(&op, a, b, p), expected,
                           "p={p} a={a} b={b} op={op:?}");
                assert_eq!(field_value(&op, a + p, b, p),
                           original(op, a + p, b, p),
                           "noncanonical p={p} a={a} b={b} op={op:?}");
            }
        }
    }
}
#[inline(never)]
fn original(op: FieldOp, a: u64, b: u64, p: u64) -> u64 {
    match op {
        FieldOp::Add => ((a as u128 + b as u128) % p as u128) as u64,
        FieldOp::Sub => ((a as u128 + p as u128 - b as u128) % p as u128) as u64,
        FieldOp::Mul | FieldOp::Square =>
            ((a as u128 * b as u128) % p as u128) as u64,
    }
}
#[test]
#[ignore]
fn paired_trace_field_rust_bench() {
    use std::hint::black_box;
    use std::time::Instant;
    fn sample(op: FieldOp, p: u64, xs: &[(u64,u64)], fast: bool) -> f64 {
        let start = Instant::now();
        let mut check = 0u64;
        for _ in 0..48 {
            for &(a,b) in xs {
                let (a,b,p) = (black_box(a),black_box(b),black_box(p));
                check = check.wrapping_add(if fast {
                    field_value(&op,a,b,p)
                } else {
                    original(op,a,b,p)
                });
            }
        }
        black_box(check);
        start.elapsed().as_nanos() as f64/(48*xs.len()) as f64
    }
    let mut seed = 0xb17f_135e_1023u64;
    for p in [487,1000000007,MAX_PRIME-30,MAX_PRIME] {
        let mut xs=Vec::new();
        for _ in 0..4096 {
            seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a=seed%p;
            seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let b=seed%p;
            xs.push((a,b));
        }
        for op in [FieldOp::Add,FieldOp::Sub,FieldOp::Mul,FieldOp::Square] {
            let (mut before,mut after)=(Vec::new(),Vec::new());
            for i in 0..13 {
                if i%2==0 {
                    before.push(sample(op,p,&xs,false));
                    after.push(sample(op,p,&xs,true));
                } else {
                    after.push(sample(op,p,&xs,true));
                    before.push(sample(op,p,&xs,false));
                }
            }
            before.sort_by(f64::total_cmp);
            after.sort_by(f64::total_cmp);
            println!("round22 production Rust trace-field p={p} op={op:?}: {:.3} -> {:.3} ns ({:.3}x)",
                     before[6],after[6],before[6]/after[6]);
        }
    }
}
"""
with tempfile.TemporaryDirectory() as td:
    source_path = Path(td) / "round22_trace_field.rs"
    binary_path = Path(td) / "round22_trace_field"
    source_path.write_text(prefix + "\n" + body + "\n" + test)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(source_path),
                    "-o",str(binary_path)],check=True)
    subprocess.run([str(binary_path),"trace_field_matches_original","--nocapture"],check=True)
    subprocess.run([str(binary_path),"paired_trace_field_rust_bench","--ignored",
                    "--nocapture"],check=True)
