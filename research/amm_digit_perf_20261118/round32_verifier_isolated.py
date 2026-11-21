#!/usr/bin/env python3
"""Round 32: compile the exact production-Rust root-verifier helpers and compare
Euclidean public-input inversion with frozen Fermat inversion. No mock functions
are substituted for optimized production arithmetic.
"""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
production = source[source.index("fn vmul("):source.index("\nfn canonical(")]
old_div = production[production.index("fn vdiv("):production.index("\nfn vpower(")]
old_div = old_div.replace("fn vdiv(", "fn before_vdiv(", 1)
old_div = old_div.replace("vinv(b[0], p)", "vpow(b[0], p - 2, p)")
old_div = old_div.replace("vinv(b[1], p)", "vpow(b[1], p - 2, p)")
old_div = old_div.replace("vinv(lead, p)", "vpow(lead, p - 2, p)")
assert "vinv(" not in old_div, "baseline must use frozen Fermat inversion"
old_monic = production[production.index("fn vmonic("):production.index("\nfn vgcd(")]
old_monic = old_monic.replace("fn vmonic(", "fn before_vmonic(", 1)
old_monic = old_monic.replace("vinv(lead, p)", "vpow(lead, p - 2, p)")
assert "vinv(" not in old_monic

checks = r'''
fn next(s: &mut u64) -> u64 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *s ^ (*s >> 29)
}
fn primes() -> [u64; 9] {
    [3, 5, 7, 17, 487, 17_497, 1_000_000_007, 4_294_967_291, MAX_PRIME]
}
fn sample_dividends(p:u64) -> Vec<Vec<u64>> {
    let mut seed = 0x3212_2026_1119_0042;
    let mut v = Vec::new();
    for len in [0usize,1,2,3,5,8,12,16] {
        for _ in 0..48 {
            let mut a = (0..len).map(|_|next(&mut seed)%p).collect::<Vec<_>>();
            if let Some(high) = a.last_mut() {
                *high = 1 + next(&mut seed)%(p-1);
            }
            v.push(a);
        }
    }
    v
}
fn divisors(p:u64)->Vec<Vec<u64>> {
    vec![
        vec![1],vec![p-1],vec![2%p],vec![0,1],vec![3%p,1],
        vec![3%p,p-1],vec![3%p,2%p,1],vec![3%p,2%p,p-1],
        vec![3%p,0,0,1],vec![3%p,0,2%p,0,1],
        vec![3%p,2%p,4%p,5%p,1],vec![3%p,2%p,4%p,5%p,p-1]
    ]
}
#[test]
fn exact_euclidean_inversion_against_frozen_fermat() {
    let mut cases=0u64;
    for p in primes() {
        let mut seed=0x3212_2026_1119_9919;
        for &x in &[0,1,2,p-1] {
            assert_eq!(vinv(x,p),vpow(x,p-2,p),"p={p} a={x}");
            cases+=1;
        }
        for _ in 0..2048 {
            let x=next(&mut seed)%p;
            let v=vinv(x,p);
            assert_eq!(v,vpow(x,p-2,p),"p={p} a={x}");
            if x>0 {assert_eq!(vmul(x,v,p),1);}
            cases+=1;
        }
    }
    println!("ROUND32_RUST inversion exact comparisons={cases}");
}
#[test]
fn exact_division_and_monic_normalization() {
    let mut count=0u64;
    for p in primes() {
        let dividends=sample_dividends(p);
        for b in divisors(p) {
            for a in &dividends {
                let old=before_vdiv(a,&b,p).unwrap();
                let new=vdiv(a,&b,p).unwrap();
                assert_eq!(new,old,"p={p} b={b:?} a={a:?}");
                count+=1;
            }
        }
        for a in dividends {
            assert_eq!(vmonic(a.clone(),p),before_vmonic(a,p));
            count+=1;
        }
    }
    println!("ROUND32_RUST division/normalization exact comparisons={count}");
}
#[test]
#[ignore = "paired compiled actual production-Rust benchmarks"]
fn benchmark_euclid_vs_fermat() {
    use std::{hint::black_box, time::Instant};
    fn trial_inv(xs:&[u64],p:u64,fast:bool,reps:usize)->f64 {
        let mut sum=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            for &x in xs {
                let x=black_box(x);let p=black_box(p);
                sum=sum.wrapping_add(if fast {vinv(x,p)} else {vpow(x,p-2,p)});
            }
        }
        black_box(sum);
        start.elapsed().as_nanos() as f64/(reps*xs.len()) as f64
    }
    fn trial_div(xs:&[Vec<u64>],b:&[u64],p:u64,fast:bool,reps:usize)->f64 {
        let mut sum=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            for a in xs {
                let a=black_box(a);let b=black_box(b);let p=black_box(p);
                let (q,r)=if fast {vdiv(a,b,p).unwrap()} else {before_vdiv(a,b,p).unwrap()};
                sum=sum.wrapping_add(q.first().copied().unwrap_or(0))
                    .wrapping_add(r.len() as u64);
            }
        }
        black_box(sum);
        start.elapsed().as_nanos() as f64/(reps*xs.len()) as f64
    }
    fn trial_monic(xs:&[Vec<u64>],p:u64,fast:bool,reps:usize)->f64 {
        let mut sum=0u64;
        let start=Instant::now();
        for a in xs {
            for _ in 0..reps {
                let a=black_box(a.clone());let p=black_box(p);
                let q=if fast {vmonic(a,p)} else {before_vmonic(a,p)};
                sum=sum.wrapping_add(q.first().copied().unwrap_or(0));
            }
        }
        black_box(sum);
        start.elapsed().as_nanos() as f64/(reps*xs.len()) as f64
    }
    for p in [487u64, 1_000_000_007, 4_294_967_291, MAX_PRIME] {
        let mut seed=0x3212_2026_1119_0111;
        let vals=(0..512).map(|_|1+next(&mut seed)%(p-1)).collect::<Vec<_>>();
        let polys=(0..128).map(|_|{
            let mut a=(0..12).map(|_|next(&mut seed)%p).collect::<Vec<_>>();
            *a.last_mut().unwrap()=1+next(&mut seed)%(p-1);
            a
        }).collect::<Vec<_>>();
        let normalizations=polys.iter().map(|a|a[..8].to_vec()).collect::<Vec<_>>();
        let tests=[
            ("inverse",Vec::<u64>::new()),
            ("constant_division",vec![3%p]),
            ("nonmonic_linear_division",vec![3%p,p-1]),
            ("nonmonic_dense_division",vec![3%p,2%p,4%p,5%p,p-1]),
            ("monic_normalization",Vec::<u64>::new()),
        ];
        for (kind,b) in tests {
            let mut before=Vec::new();let mut after=Vec::new();
            let timed=|fast:bool|->f64 {
                match kind {
                    "inverse"=>trial_inv(&vals,p,fast,100),
                    "monic_normalization"=>trial_monic(&normalizations,p,fast,80),
                    _=>trial_div(&polys,&b,p,fast,60)
                }
            };
            for turn in 0..13 {
                if turn%2==0 {before.push(timed(false));after.push(timed(true));}
                else {after.push(timed(true));before.push(timed(false));}
            }
            before.sort_by(f64::total_cmp);
            after.sort_by(f64::total_cmp);
            println!("ROUND32_RUST p={p} {kind}: {:.3} -> {:.3} ns/op ({:.3}x)",
                     before[6],after[6],before[6]/after[6]);
        }
    }
}
''';
rust = (
    "#![allow(dead_code)]\n"
    "const MAX_PRIME:u64=(1u64<<61)-1;\n"
    "fn trim(mut v:Vec<u64>)->Vec<u64>{while v.last()==Some(&0){v.pop();}v}\n"
    + production + "\n" + old_div + "\n" + old_monic + "\n" + checks
)
with tempfile.TemporaryDirectory() as tmp:
    file=Path(tmp)/"round32.rs"
    out=Path(tmp)/"round32-test"
    file.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(file),"-o",str(out)],check=True)
    subprocess.run([str(out),"exact_euclidean_inversion_against_frozen_fermat","--nocapture"],check=True)
    subprocess.run([str(out),"exact_division_and_monic_normalization","--nocapture"],check=True)
    subprocess.run([str(out),"benchmark_euclid_vs_fermat","--ignored","--nocapture"],check=True)
