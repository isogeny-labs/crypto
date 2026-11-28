#!/usr/bin/env python3
"""Round 25: actual production verifier functions, Rust equivalence and AB/BA timing."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
start = source.index("fn vmul(")
end = source.index("\nfn canonical(", start)
production = source[start:end]
for required in ("fn vmul(", "fn vpow(", "fn vproduct(", "fn vsquare(", "fn vdiv(", "fn vpower(", "fn vgcd("):
    assert required in production, required

rust = r"""
const MAX_PRIME: u64 = (1u64 << 61) - 1;
fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) { v.pop(); }
    v
}
""" + production + r"""
fn old_div(a: &[u64], b: &[u64], p: u64) -> Result<(Vec<u64>, Vec<u64>), String> {
    if b.is_empty() { return Err("division by zero polynomial".into()); }
    if a.len() < b.len() { return Ok((vec![], a.to_vec())); }
    let mut r = a.to_vec();
    let mut q = vec![0; a.len() - b.len() + 1];
    let lead = *b.last().unwrap();
    let inv = if lead == 1 { 1 } else { vpow(lead, p - 2, p) };
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
    Ok((trim(q), r))
}
fn old_power(base: &[u64], mut e: u64, m: &[u64], p: u64) -> Result<Vec<u64>, String> {
    let mut r = old_div(&[1], m, p)?.1;
    let mut b = old_div(base, m, p)?.1;
    while e > 0 {
        if e & 1 == 1 {
            r = old_div(&vproduct(&r, &b, p), m, p)?.1;
        }
        e >>= 1;
        if e != 0 {
            b = old_div(&vproduct(&b, &b, p), m, p)?.1;
        }
    }
    Ok(r)
}
fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}
fn fixtures(p: u64) -> Vec<(Vec<u64>, Vec<u64>)> {
    let mut seed = 0x2026_1118_2525_a11bu64;
    let mut out = Vec::new();
    for _ in 0..128 {
        let a = (0..9).map(|_| next(&mut seed) % p).collect::<Vec<_>>();
        let mut b = (0..5).map(|_| next(&mut seed) % p).collect::<Vec<_>>();
        b[4] = 1;
        out.push((a, b));
    }
    out
}
#[test]
fn round25_symmetric_square_and_synthetic_division_exact() {
    let mut checks=0usize;
    for p in [3, 5, 487, 17497, 1000000007, 4294967291, MAX_PRIME] {
        let mut seed=0x1234_2525_9999_5555u64;
        for (a,b) in fixtures(p) {
            for degree in 0..=9 {
                let v=(0..degree).map(|_| next(&mut seed)%p).collect::<Vec<_>>();
                assert_eq!(vsquare(&v,p),vproduct(&v,&v,p),"square p={p} degree={degree}");
                checks+=1;
            }
            for divisor in [vec![1],vec![next(&mut seed)%p,1],b.clone(),
                            vec![next(&mut seed)%p,2]] {
                for len in 0..=a.len() {
                    assert_eq!(vdiv(&a[..len],&divisor,p),
                               old_div(&a[..len],&divisor,p),
                               "division p={p} dividend={len} divisor={divisor:?}");
                    checks+=1;
                }
                for e in [0, 1, 2, 3, 7, 63] {
                    assert_eq!(vpower(&[2%p,1],e,&divisor,p),
                               old_power(&[2%p,1],e,&divisor,p),
                               "power p={p} e={e} divisor={divisor:?}");
                    checks+=1;
                }
            }
        }
    }
    println!("round25 exact Rust comparisons: {checks}");
}
#[test]
#[ignore = "actual production Rust AB/BA microbench"]
fn benchmark_round25_verifier_actual_rust() {
    use std::hint::black_box;
    use std::time::Instant;
    fn sample(p:u64,data:&[(Vec<u64>,Vec<u64>)],op:usize,fast:bool)->f64 {
        let reps=if op==6 { 1 } else { 8 };
        let mut checksum=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            for (a,b) in data {
                let (a,b,p)=(black_box(a),black_box(b),black_box(p));
                let out=match op {
                    0 | 1 | 2 => {
                        let divisor=if op==1 {vec![b[0],1]} else if op==2 {
                            vec![b[0],b[1],b[2],b[3],2]
                        } else {b.clone()};
                        let (q,r)=if fast {vdiv(a,&divisor,p).unwrap()}
                                  else {old_div(a,&divisor,p).unwrap()};
                        q.len() as u64 + r.len() as u64 + q.first().copied().unwrap_or(0)
                    }
                    3 | 4 | 5 => {
                        let v=&a[..if op==3 {2} else if op==4 {4} else {8}];
                        let r=if fast {vsquare(v,p)} else {vproduct(v,v,p)};
                        r.len() as u64 + r.first().copied().unwrap_or(0)
                    }
                    _ => {
                        let r=if fast {vpower(&[a[0],1],63,b,p).unwrap()}
                              else {old_power(&[a[0],1],63,b,p).unwrap()};
                        r.len() as u64 + r.first().copied().unwrap_or(0)
                    }
                };
                checksum ^= black_box(out);
            }
        }
        black_box(checksum);
        start.elapsed().as_nanos() as f64/(reps*data.len()) as f64
    }
    for p in [487,1000000007,MAX_PRIME] {
        let data=fixtures(p);
        for (op,name) in [(0,"monic_div_deg8_by4"),(1,"monic_linear_div"),
                          (2,"nonmonic_control"),(3,"square_deg1"),
                          (4,"square_deg3"),(5,"square_deg7"),
                          (6,"poly_power_e63")] {
            let (mut before,mut after)=(Vec::new(),Vec::new());
            for i in 0..13 {
                if i%2==0 {
                    before.push(sample(p,&data,op,false));
                    after.push(sample(p,&data,op,true));
                } else {
                    after.push(sample(p,&data,op,true));
                    before.push(sample(p,&data,op,false));
                }
            }
            before.sort_by(f64::total_cmp);
            after.sort_by(f64::total_cmp);
            println!("round25 actual RUST p={p} {name}: {:.3} -> {:.3} ns ({:.3}x)",
                     before[6],after[6],before[6]/after[6]);
        }
    }
}
"""
with tempfile.TemporaryDirectory() as td:
    src = Path(td)/"round25_verifier.rs"
    exe = Path(td)/"round25_verifier"
    src.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"round25_symmetric_square_and_synthetic_division_exact","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark_round25_verifier_actual_rust","--ignored","--nocapture"],check=True)
