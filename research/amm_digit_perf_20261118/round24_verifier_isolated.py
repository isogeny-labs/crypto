#!/usr/bin/env python3
"""Round 24: compile exact production verifier functions in an isolated Rust test.

Reference functions preserve the old monic/short-division and power-loop code.
Reported timing is actual compiled production Rust, not a C++ reproduction.
"""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
start = source.index("fn vmul(")
end = source.index("\nfn canonical(", start)
production = source[start:end]
for name in ("fn vmul(", "fn vpow(", "fn vproduct(", "fn vdiv(", "fn vpower(", "fn vmonic(", "fn vgcd("):
    assert name in production, name

rust = r"""
const MAX_PRIME: u64 = (1u64 << 61) - 1;
fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) { v.pop(); }
    v
}
""" + production + r"""
fn old_div(a: &[u64], b: &[u64], p: u64) -> Result<(Vec<u64>, Vec<u64>), String> {
    if b.is_empty() {
        return Err("division by zero polynomial".into());
    }
    let mut r = a.to_vec();
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    let inv = vpow(*b.last().unwrap(), p - 2, p);
    while !r.is_empty() && r.len() >= b.len() {
        let shift = r.len() - b.len();
        let c = vmul(*r.last().unwrap(), inv, p);
        q[shift] = c;
        for (i, &x) in b.iter().enumerate() {
            let product = vmul(c, x, p);
            let old = r[shift + i];
            r[shift + i] = if old >= product { old - product }
                           else { old + p - product };
        }
        r = trim(r);
    }
    Ok((trim(q), r))
}
fn old_monic(mut a: Vec<u64>, p: u64) -> Vec<u64> {
    if let Some(&lead) = a.last() {
        let inv = vpow(lead, p - 2, p);
        for c in &mut a {
            *c = vmul(*c, inv, p);
        }
    }
    a
}
fn old_power(base: &[u64], mut e: u64, m: &[u64], p: u64) -> Result<Vec<u64>, String> {
    let mut r = old_div(&[1], m, p)?.1;
    let mut b = old_div(base, m, p)?.1;
    while e > 0 {
        if e & 1 == 1 {
            r = old_div(&vproduct(&r, &b, p), m, p)?.1;
        }
        b = old_div(&vproduct(&b, &b, p), m, p)?.1;
        e >>= 1;
    }
    Ok(r)
}
fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}
fn fixtures(p: u64) -> Vec<(Vec<u64>, Vec<u64>)> {
    let mut seed = 0x2026_1118_2424_a11bu64;
    let mut out = Vec::new();
    for _ in 0..128 {
        let a = (0..7).map(|_| next(&mut seed) % p).collect::<Vec<_>>();
        let mut b = (0..4).map(|_| next(&mut seed) % p).collect::<Vec<_>>();
        b[3] = 1;
        out.push((a, b));
    }
    out
}
#[test]
fn exact_round24_verifier_reference() {
    let mut checks = 0usize;
    for p in [3, 5, 487, 17497, 1000000007, 4294967291, MAX_PRIME] {
        for (a, mut b) in fixtures(p) {
            for lead in [1, (p-1).max(1)] {
                b[3] = lead;
                assert_eq!(vdiv(&a, &b, p), old_div(&a, &b, p));
                assert_eq!(vdiv(&a[..2], &b, p), old_div(&a[..2], &b, p));
                assert_eq!(vmonic(b.clone(), p), old_monic(b.clone(), p));
                for exponent in [0, 1, 2, 7, 31, 63] {
                    assert_eq!(vpower(&[2,1], exponent, &b, p),
                               old_power(&[2,1], exponent, &b, p),
                               "p={p} e={exponent}");
                    checks += 1;
                }
            }
        }
    }
    println!("round24 exact production-verifier comparisons: {checks} polynomial powers plus division/normalization");
}
#[test]
#[ignore]
fn benchmark_round24_verifier_actual_rust() {
    use std::hint::black_box;
    use std::time::Instant;
    fn sample(p: u64, data: &[(Vec<u64>,Vec<u64>)], op: usize, fast: bool) -> f64 {
        let reps = if op == 4 { 1 } else { 12 };
        let mut checksum = 0u64;
        let start = Instant::now();
        for _ in 0..reps {
            for (a, b) in data {
                let (a,b,p) = (black_box(a),black_box(b),black_box(p));
                match op {
                    0 | 1 | 2 => {
                        let mut b = b.clone();
                        if op == 1 { b[3] = p-1; }
                        let a = if op == 2 { &a[..2] } else { &a[..] };
                        let (q,r) = if fast { vdiv(a, &b, p).unwrap() }
                                    else { old_div(a, &b, p).unwrap() };
                        checksum ^= (q.len() + r.len()) as u64
                            ^ q.first().copied().unwrap_or(0);
                    }
                    3 => {
                        let r = if fast { vmonic(b.clone(),p) } else {old_monic(b.clone(),p)};
                        checksum ^= r[0];
                    }
                    _ => {
                        let r = if fast {vpower(a,63,b,p).unwrap()}
                                else {old_power(a,63,b,p).unwrap()};
                        checksum ^= r.first().copied().unwrap_or(0);
                    }
                }
            }
        }
        black_box(checksum);
        start.elapsed().as_nanos() as f64 / (reps * data.len()) as f64
    }
    for p in [487,1000000007,MAX_PRIME] {
        let data=fixtures(p);
        for (op,name) in [(0,"monic_div"),(1,"nonmonic_div_control"),
                          (2,"short_div"),(3,"monic_normalization"),
                          (4,"poly_power_e63")] {
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
            println!("round24 RUST p={p} {name}: {:.3} -> {:.3} ns ({:.3}x)",
                     before[6],after[6],before[6]/after[6]);
        }
    }
}
"""
with tempfile.TemporaryDirectory() as td:
    src = Path(td) / "round24_verifier.rs"
    exe = Path(td) / "round24_verifier"
    src.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(src),"-o",str(exe)],check=True)
    subprocess.run([str(exe),"exact_round24_verifier_reference","--nocapture"],check=True)
    subprocess.run([str(exe),"benchmark_round24_verifier_actual_rust",
                    "--ignored","--nocapture"],check=True)
