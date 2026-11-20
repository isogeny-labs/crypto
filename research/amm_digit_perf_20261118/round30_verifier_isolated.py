#!/usr/bin/env python3
"""Exact production-Rust root-verifier arithmetic vs frozen pre-round30 code."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
production = source[source.index("fn vmul("):source.index("\nfn canonical(")]
baseline = Path("research/amm_digit_perf_20261118/round30_verifier_baseline.rs").read_text()
checks = r'''
fn rng(s: &mut u64) -> u64 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *s ^ (*s >> 29)
}
fn cases(p:u64, kind:usize) -> Vec<(Vec<u64>,Vec<u64>)> {
    let mut seed=0x2026_1119_3030u64 + kind as u64 * 117;
    let mut all=Vec::new();
    for _ in 0..96 {
        let mut a=(0..16).map(|_|rng(&mut seed)%p).collect::<Vec<_>>();
        a[15]=1+rng(&mut seed)%(p-1);
        let b=match kind {
            0=>vec![3%p,0,0,0,0,0,0,1],             // binomial x^7+c
            1=>vec![3%p,0,0,5%p,0,0,0,1],           // sparse trinomial
            2=>vec![3%p,0,1],                        // monic quadratic
            3=>vec![3%p,p-1],                        // nonmonic linear
            4=>vec![3%p,2%p,4%p,5%p,1],             // dense monic
            5=>vec![3%p,2%p,4%p,5%p,p-1],           // dense nonmonic
            6=>vec![5%p,0,0,3%p,0,0,1],             // trinomial x^6+c*x^3+d
            7=>vec![3%p,0,0,0,1],                   // binomial x^4+c
            8=>vec![0,0,1],                         // monomial x^2
            9=>vec![3%p,0,2%p,0,1],                 // quadratic in x^2
            10=>vec![1,1],                          // monic linear control
            _=>unreachable!()
        };
        all.push((a,b));
    }
    all
}
fn square_cases(p:u64,kind:usize)->Vec<Vec<u64>>{
    let mut seed=0x2026_1119_3119u64 + kind as u64 * 391;
    let mut all=Vec::new();
    for _ in 0..96 {
        let mut a=(0..9).map(|_|rng(&mut seed)%p).collect::<Vec<_>>();
        match kind {
            0=>{a.fill(0);for j in [0,2,4,6,8]{a[j]=1+j as u64%p;}},
            1=>{a.fill(0);for j in [0,3,5,7,8]{a[j]=1+j as u64%p;}},
            2=>{a.fill(0);for j in [0,4,8]{a[j]=1+j as u64%p;}},
            3=>{a[8]=1;},
            4=>{a.fill(0);},
            _=>unreachable!()
        }
        a=trim(a);
        all.push(a);
    }
    all
}
#[test]
fn round30_exact_frozen_comparisons() {
    let mut count=0u64;
    for p in [3u64,5,7,17,487,17497,1_000_000_007,4_294_967_291,MAX_PRIME] {
        for kind in 0..11 {
            for (a,b) in cases(p,kind) {
                for length in [0,1,3,8,12,16] {
                    let a=trim(a[..length].to_vec());
                    assert_eq!(vdiv(&a,&b,p),before_vdiv(&a,&b,p),
                       "division p={p} kind={kind} len={length}");
                    count+=1;
                }
            }
        }
        for kind in 0..5 {
            for a in square_cases(p,kind) {
                assert_eq!(vsquare(&a,p),before_vsquare(&a,p),
                   "square p={p} kind={kind}");
                count+=1;
            }
        }
    }
    println!("round30 exact compiled Rust comparisons={count}");
}
#[test]
#[ignore = "paired compiled production Rust benchmark"]
fn round30_paired_actual_rust() {
    use std::{hint::black_box, time::Instant};
    fn sample(p:u64,kind:usize,fast:bool, data:&[(Vec<u64>,Vec<u64>)],reps:usize)->f64 {
        let mut checksum=0u64;
        let begin=Instant::now();
        for _ in 0..reps {
            for (a,b) in data {
                let a=black_box(a); let b=black_box(b); let p=black_box(p);
                let (q,r)=if fast {vdiv(a,b,p).unwrap()} else {before_vdiv(a,b,p).unwrap()};
                checksum=checksum.wrapping_add(q.len() as u64 + r.len() as u64
                  + q.last().copied().unwrap_or(0));
            }
        }
        black_box(checksum);
        begin.elapsed().as_nanos() as f64/(reps*data.len()) as f64
    }
    fn square_sample(p:u64,kind:usize,fast:bool,data:&[Vec<u64>],reps:usize)->f64{
        let mut checksum=0u64;let begin=Instant::now();
        for _ in 0..reps {
            for a in data {
                let a=black_box(a);let p=black_box(p);
                let r=if fast {vsquare(a,p)} else {before_vsquare(a,p)};
                checksum=checksum.wrapping_add(r.len() as u64+r.last().copied().unwrap_or(0));
            }
        }
        black_box(checksum);
        begin.elapsed().as_nanos() as f64/(reps*data.len()) as f64
    }
    for p in [1_000_000_007u64,MAX_PRIME] {
        for kind in 0..11 {
            let data=cases(p,kind);let mut bs=Vec::new();let mut fs=Vec::new();
            for trial in 0..13 {
                if trial%2==0 {
                    bs.push(sample(p,kind,false,&data,100));
                    fs.push(sample(p,kind,true,&data,100));
                } else {
                    fs.push(sample(p,kind,true,&data,100));
                    bs.push(sample(p,kind,false,&data,100));
                }
            }
            bs.sort_by(f64::total_cmp);fs.sort_by(f64::total_cmp);
            println!("round30 RUST division p={p} kind={kind}: {:.3} -> {:.3} ns ({:.3}x)",
                    bs[6],fs[6],bs[6]/fs[6]);
        }
        for kind in 0..5 {
            let data=square_cases(p,kind);let(mut bs,mut fs)=(Vec::new(),Vec::new());
            for trial in 0..13{
                if trial%2==0 {
                    bs.push(square_sample(p,kind,false,&data,120));
                    fs.push(square_sample(p,kind,true,&data,120));
                } else {
                    fs.push(square_sample(p,kind,true,&data,120));
                    bs.push(square_sample(p,kind,false,&data,120));
                }
            }
            bs.sort_by(f64::total_cmp);fs.sort_by(f64::total_cmp);
            println!("round30 RUST square p={p} kind={kind}: {:.3} -> {:.3} ns ({:.3}x)",
                    bs[6],fs[6],bs[6]/fs[6]);
        }
    }
}
''';
rust = "const MAX_PRIME:u64=(1u64<<61)-1;\nfn trim(mut a:Vec<u64>)->Vec<u64>{while a.last()==Some(&0){a.pop();}a}\n" + production + "\n" + baseline + checks
with tempfile.TemporaryDirectory() as temp:
    path=Path(temp)/"round30.rs"
    binary=Path(temp)/"round30"
    path.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"round30_exact_frozen_comparisons","--nocapture"],check=True)
    subprocess.run([str(binary),"round30_paired_actual_rust","--ignored","--nocapture"],check=True)
