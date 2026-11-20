#!/usr/bin/env python3
"""Round 26: compare production verifier paths to frozen baseline in actual optimized Rust."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
start = source.index("fn vmul(")
end = source.index("\nfn canonical(", start)
production = source[start:end]
for signature in ("fn vmul(", "fn vproduct(", "fn vdiv(", "fn vpower("):
    assert signature in production, signature

rust = r"""
const MAX_PRIME: u64 = (1u64 << 61) - 1;
fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) { v.pop(); }
    v
}
""" + production + r"""
// Frozen round-25 multiplication and division. Do not call the new
// implementations from these baselines.
fn old_vproduct(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = if p == MAX_PRIME {
                let product = x as u128 * y as u128;
                let folded = ((product as u64) & MAX_PRIME)
                    + (product >> 61) as u64 + r[i + j];
                let folded = (folded & MAX_PRIME) + (folded >> 61);
                if folded >= p { folded - p } else { folded }
            } else if p <= u32::MAX as u64 {
                (r[i + j] + x * y) % p
            } else {
                ((r[i + j] as u128 + x as u128 * y as u128) % p as u128) as u64
            };
        }
    }
    trim(r)
}
fn old_vdiv(a: &[u64], b: &[u64], p: u64) -> Result<(Vec<u64>, Vec<u64>), String> {
    if b.is_empty() { return Err("division by zero polynomial".into()); }
    if a.len() < b.len() { return Ok((vec![], a.to_vec())); }
    let mut r = a.to_vec();
    let mut q = vec![0; a.len() - b.len() + 1];
    let lead = *b.last().unwrap();
    if lead == 1 {
        while !r.is_empty() && r.len() >= b.len() {
            let shift = r.len() - b.len();
            let c = *r.last().unwrap();
            q[shift] = c;
            r.pop();
            for (i, &x) in b[..b.len()-1].iter().enumerate() {
                let product = vmul(c, x, p);
                let old = r[shift + i];
                r[shift + i] = if old >= product { old - product } else { old + p - product };
            }
            r = trim(r);
        }
    } else {
        let inv = vpow(lead, p - 2, p);
        while !r.is_empty() && r.len() >= b.len() {
            let shift = r.len() - b.len();
            let c = vmul(*r.last().unwrap(), inv, p);
            q[shift] = c;
            for (i, &x) in b.iter().enumerate() {
                let product = vmul(c, x, p);
                let old = r[shift + i];
                r[shift + i] = if old >= product { old - product } else { old + p - product };
            }
            r = trim(r);
        }
    }
    Ok((trim(q), r))
}
fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}
fn fixture(p: u64, kind: usize) -> Vec<(Vec<u64>, Vec<u64>)> {
    let mut seed = 0x2026_1118_2626_4242u64;
    let mut v = Vec::new();
    for _ in 0..128 {
        let mut a = (0..9).map(|_| next(&mut seed) % p).collect::<Vec<_>>();
        let mut b = (0..5).map(|_| next(&mut seed) % p).collect::<Vec<_>>();
        match kind {
            0 => b[4] = 1,
            1 => b[4] = 2,
            2 => {
                for x in &mut a { if next(&mut seed) % 4 != 0 { *x = 0; } }
                for x in &mut b { if next(&mut seed) % 4 != 0 { *x = 0; } }
                b[4] = 1;
            }
            3 => b = vec![b[0], 1],
            4 => b = vec![1],
            _ => b[4] = 1,
        }
        v.push((a,b));
    }
    v
}
#[test]
fn round26_sparse_product_and_synthetic_division_exact() {
    let mut checks = 0usize;
    for p in [3u64, 5, 487, 17497, 1000000007, 4294967291, MAX_PRIME] {
        for kind in 0..6 {
            for (a,b) in fixture(p,kind) {
                for length in 0..=a.len() {
                    let dividend = &a[..length];
                    assert_eq!(vdiv(dividend,&b,p), old_vdiv(dividend,&b,p),
                               "division p={p} kind={kind} length={length}");
                    checks+=1;
                }
                for length in 0..=a.len() {
                    assert_eq!(vproduct(&a[..length],&b,p),old_vproduct(&a[..length],&b,p),
                               "product p={p} kind={kind} length={length}");
                    checks+=1;
                }
                if kind == 2 {
                    assert_eq!(vproduct(&b,&a,p),old_vproduct(&b,&a,p));
                    checks+=1;
                }
            }
        }
    }
    println!("round26 exact actual-Rust checks={checks}");
}
#[test]
#[ignore = "production Rust AB/BA benchmark"]
fn bench_round26_verifier_actual_rust() {
    use std::hint::black_box;
    use std::time::Instant;
    fn sample(p:u64,data:&[(Vec<u64>,Vec<u64>)],kind:usize,fast:bool)->f64 {
        let reps=if kind==1 { 12 } else { 32 };
        let mut checksum=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            for (a,b) in data {
                let (a,b,p)=(black_box(a),black_box(b),black_box(p));
                let v = if kind == 2 || kind == 5 {
                    let out=if fast {vproduct(a,b,p)} else {old_vproduct(a,b,p)};
                    out.first().copied().unwrap_or(0).wrapping_add(out.len() as u64)
                } else {
                    let (q,r) = if fast {vdiv(a,b,p).unwrap()} else {old_vdiv(a,b,p).unwrap()};
                    q.first().copied().unwrap_or(0)
                        .wrapping_add(r.first().copied().unwrap_or(0))
                        .wrapping_add(q.len() as u64)
                };
                checksum = checksum.wrapping_add(black_box(v));
            }
        }
        black_box(checksum);
        start.elapsed().as_nanos() as f64 / (reps*data.len()) as f64
    }
    for p in [487u64,1000000007,MAX_PRIME] {
        for (kind,name) in [(0,"dense_monic_div"),(1,"nonmonic_div"),
                            (2,"sparse_product"),(3,"monic_linear_div"),
                            (4,"monic_constant_div"),(5,"dense_product")] {
            let data=fixture(p,kind);
            let (mut before,mut after)=(Vec::new(),Vec::new());
            for i in 0..13 {
                if i%2==0 {
                    before.push(sample(p,&data,kind,false));
                    after.push(sample(p,&data,kind,true));
                } else {
                    after.push(sample(p,&data,kind,true));
                    before.push(sample(p,&data,kind,false));
                }
            }
            before.sort_by(f64::total_cmp);
            after.sort_by(f64::total_cmp);
            println!("round26 actual RUST p={p} {name}: {:.3} -> {:.3} ns ({:.3}x)",
                     before[6],after[6],before[6]/after[6]);
        }
    }
}
"""
with tempfile.TemporaryDirectory() as td:
    rust_file=Path(td)/"round26_verifier.rs"
    executable=Path(td)/"round26_verifier"
    rust_file.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",
                    str(rust_file),"-o",str(executable)],check=True)
    subprocess.run([str(executable),"round26_sparse_product_and_synthetic_division_exact",
                    "--nocapture"],check=True)
    subprocess.run([str(executable),"bench_round26_verifier_actual_rust",
                    "--ignored","--nocapture"],check=True)
