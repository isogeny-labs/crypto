#!/usr/bin/env python3
"""Compile unmodified production verifier helpers with rustc and AB/BA benchmark.

No shadow implementation is passed off as production: vmul/vpow/vproduct/vdiv
are extracted byte-for-byte from src/cryptanalysis/prime_root_trace.rs.
The frozen functions are the pre-optimization u128-reference formulas.
"""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
start = source.index("fn vmul(")
end = source.index("\nfn vpower(", start)
production = source[start:end]
for name in ("fn vmul(", "fn vpow(", "fn vproduct(", "fn vdiv("):
    assert name in production, name

rust = r"""
const MAX_PRIME: u64 = (1u64 << 61) - 1;
fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) { v.pop(); }
    v
}
""" + production + r"""
fn old_mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn old_pow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 { r = old_mul(r, a, p); }
        a = old_mul(a, a, p);
        e >>= 1;
    }
    r
}
fn old_product(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = ((r[i + j] as u128 + x as u128 * y as u128)
                          % p as u128) as u64;
        }
    }
    trim(r)
}
fn old_div(a: &[u64], b: &[u64], p: u64) -> Result<(Vec<u64>, Vec<u64>), String> {
    if b.is_empty() { return Err("division by zero polynomial".into()); }
    let mut r = a.to_vec();
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    let inv = old_pow(*b.last().unwrap(), p - 2, p);
    while !r.is_empty() && r.len() >= b.len() {
        let shift = r.len() - b.len();
        let c = old_mul(*r.last().unwrap(), inv, p);
        q[shift] = c;
        for (i, &x) in b.iter().enumerate() {
            r[shift + i] = ((r[shift + i] as u128 + p as u128 -
                              old_mul(c, x, p) as u128) % p as u128) as u64;
        }
        r = trim(r);
    }
    Ok((trim(q), r))
}
fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}
fn data(p: u64) -> (Vec<(u64,u64)>, Vec<(Vec<u64>,Vec<u64>)>) {
    let mut seed = 0x12bc_2026_dabe_3301u64;
    let mut pairs = Vec::with_capacity(2048);
    let mut polys = Vec::with_capacity(512);
    for _ in 0..2048 {
        pairs.push((next(&mut seed)%p, next(&mut seed)%p));
    }
    for _ in 0..512 {
        let a = (0..8).map(|_| next(&mut seed)%p).collect::<Vec<_>>();
        let mut b = (0..4).map(|_| next(&mut seed)%p).collect::<Vec<_>>();
        b[3] = next(&mut seed)%(p-1)+1;
        polys.push((a,b));
    }
    (pairs,polys)
}
#[test]
fn verifier_helpers_exact_vs_wide() {
    let mut count = 0u64;
    for p in [3,5,487,17497,1000000007,4294967291,MAX_PRIME] {
        let (xs, polys) = data(p);
        for (a,b) in xs {
            assert_eq!(vmul(a,b,p),old_mul(a,b,p));
            count+=1;
        }
        for a in [0,1,p-1,p,p+1] {
            for b in [0,1,p-1,p,p+1] {
                assert_eq!(vmul(a,b,p),old_mul(a,b,p),"p={p} a={a} b={b}");
            }
        }
        for (a,b) in polys {
            assert_eq!(vproduct(&a,&b,p),old_product(&a,&b,p));
            assert_eq!(vdiv(&a,&b,p),old_div(&a,&b,p));
            count+=2;
        }
    }
    println!("round23 production verifier exact checks: {count}");
}
#[test]
#[ignore]
fn bench_verifier_helpers_actual_rust() {
    use std::time::Instant;
    use std::hint::black_box;
    fn run(p:u64, xs:&[(u64,u64)], polys:&[(Vec<u64>,Vec<u64>)],
           op:u8, fast:bool) -> f64 {
        let reps:usize = if op==0 {96} else if op==1 {16} else {2};
        let start=Instant::now();
        let mut checksum=0u64;
        for _ in 0..reps {
            if op==0 {
                for &(a,b) in xs {
                    let (a,b,p)=(black_box(a),black_box(b),black_box(p));
                    checksum ^= if fast {vmul(a,b,p)} else {old_mul(a,b,p)};
                }
            } else {
                for (a,b) in polys {
                    if op==1 {
                        let r=if fast {vproduct(black_box(a),black_box(b),black_box(p))}
                              else {old_product(black_box(a),black_box(b),black_box(p))};
                        checksum ^= r.iter().copied().fold(0u64,u64::wrapping_add);
                    } else {
                        let (q,r)=if fast {vdiv(black_box(a),black_box(b),black_box(p)).unwrap()}
                                  else {old_div(black_box(a),black_box(b),black_box(p)).unwrap()};
                        checksum ^= q.len() as u64 ^ r.iter().copied().fold(0u64,u64::wrapping_add);
                    }
                }
            }
        }
        black_box(checksum);
        let n=if op==0 {xs.len()} else {polys.len()};
        start.elapsed().as_nanos() as f64/(reps*n) as f64
    }
    for p in [487,1000000007,4294967291,MAX_PRIME] {
        let (xs,polys)=data(p);
        for (op,name) in [(0,"vmul"),(1,"vproduct"),(2,"vdiv")] {
            let (mut b,mut a)=(Vec::new(),Vec::new());
            for i in 0..13 {
                if i%2==0 {
                    b.push(run(p,&xs,&polys,op,false));
                    a.push(run(p,&xs,&polys,op,true));
                } else {
                    a.push(run(p,&xs,&polys,op,true));
                    b.push(run(p,&xs,&polys,op,false));
                }
            }
            b.sort_by(f64::total_cmp); a.sort_by(f64::total_cmp);
            println!("round23 production Rust p={p} {name}: {:.3} -> {:.3} ns ({:.3}x)",
                      b[6],a[6],b[6]/a[6]);
        }
    }
}
"""
with tempfile.TemporaryDirectory() as td:
    src = Path(td) / "round23_verifier.rs"
    exe = Path(td) / "round23_verifier"
    src.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"verifier_helpers_exact_vs_wide","--nocapture"],check=True)
    subprocess.run([str(exe),"bench_verifier_helpers_actual_rust","--ignored","--nocapture"],check=True)
