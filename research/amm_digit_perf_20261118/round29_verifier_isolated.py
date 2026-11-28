#!/usr/bin/env python3
"""Compile exact production Rust verifier arithmetic against frozen pre-round29 functions."""
from pathlib import Path
import subprocess
import tempfile

source = Path("src/cryptanalysis/prime_root_trace.rs").read_text()
production = source[source.index("fn vmul("):source.index("\nfn canonical(")]
baseline = Path("research/amm_digit_perf_20261118/round29_verifier_baseline.rs").read_text()
checks = r'''
fn rng(s: &mut u64) -> u64 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *s ^ (*s >> 29)
}
fn fixtures(p: u64, op: usize, kind: usize) -> Vec<(Vec<u64>,Vec<u64>)> {
    let mut s = 0x2026_1119_1234u64 + (op as u64)*31 + (kind as u64)*77;
    let mut result = Vec::new();
    for _ in 0..128 {
        let mut a = (0..12).map(|_| rng(&mut s)%p).collect::<Vec<_>>();
        let mut b = (0..7).map(|_| rng(&mut s)%p).collect::<Vec<_>>();
        if op != 1 {
            match kind {
                0 => { a.fill(0); a[0]=1; a[4]=5; a[8]=7; a[11]=2; },
                1 => { a.fill(0); a[3]=1; a[6]=5; a[9]=2; a[11]=7; },
                2 => { a.fill(0); a[0]=1; a[3]=2; a[6]=3; a[9]=4; a[11]=5; },
                3 => { a.fill(0); a[0]=1; a[11]=5; },
                4 => { a.fill(0); a[0]=1; a[5]=3; a[11]=5; },
                5 => { a=vec![3,5,2,0,4,1,1]; },
                6 => { a=vec![5,2,8,9]; },
                _ => unreachable!()
            }
            b=a.clone();
        } else {
            b=match kind {
                0 => vec![3,0,0,0,0,0,1], 1=>vec![1,0,0,1],
                2=>vec![p-1,0,0,0,1], 3=>vec![3,0,0,0,0,1],
                4=>vec![1,4,2,5,6,1],5=>vec![1,0,0,0,0,0,0,1],
                6=>vec![3,0,0,4,0,0,1],_=>unreachable!()
            };
        }
        for x in a.iter_mut() { *x %= p; }
        for x in b.iter_mut() { *x %= p; }
        if a.last() == Some(&0) { *a.last_mut().unwrap()=1; }
        if b.last() == Some(&0) { *b.last_mut().unwrap()=1; }
        result.push((a,b));
    }
    result
}
#[test]
fn round29_exact_frozen_comparisons() {
    let mut count=0u64;
    for p in [3u64,5,487,17497,1_000_000_007,4_294_967_291,MAX_PRIME] {
        for op in 0..3 {
            for kind in 0..7 {
                for (a,b) in fixtures(p,op,kind) {
                    for n in [0,3,8,12] {
                        let a=trim(a[..n.min(a.len())].to_vec());
                        match op {
                            0 => {
                                assert_eq!(vsquare(&a,p),before_vsquare(&a,p),
                                    "square p={p} kind={kind} len={n}");
                            },
                            1 => {
                                assert_eq!(vdiv(&a,&b,p),before_vdiv(&a,&b,p),
                                    "division p={p} kind={kind} len={n}");
                            },
                            2 => {
                                let old=vproduct(&a,&a,p);
                                let fast=if a.len()>=6 && a[0]!=0 && a[1]==0 && a[2]==0
                                    && *a.last().unwrap()!=0
                                    && a[3..a.len()-1].iter().all(|&x|x==0)
                                { vsquare(&a,p) } else { vproduct(&a,&a,p) };
                                assert_eq!(fast,old,"power-step p={p} kind={kind} len={n}");
                            },
                            _ => unreachable!(),
                        }
                        count+=1;
                    }
                }
            }
        }
    }
    println!("round29 Rust frozen-reference comparisons={count}");
}
#[test]
#[ignore = "paired compiled production Rust benchmark"]
fn round29_paired_actual_rust() {
    use std::{hint::black_box, time::Instant};
    fn sample(p:u64,op:usize,kind:usize, data:&[(Vec<u64>,Vec<u64>)],fast:bool)->f64 {
        let reps=if op==1 {80} else {110};
        let mut checksum=0u64;
        let start=Instant::now();
        for _ in 0..reps {
            for (a,b) in data {
                let a=black_box(a);
                let b=black_box(b);
                let p=black_box(p);
                let v=match op {
                    0 => { let v=if fast {vsquare(a,p)} else {before_vsquare(a,p)};
                        v.len() as u64+v.last().copied().unwrap_or(0) },
                    1 => { let(q,r)=if fast {vdiv(a,b,p).unwrap()} else {before_vdiv(a,b,p).unwrap()};
                        q.len() as u64+r.len() as u64+q.last().copied().unwrap_or(0) },
                    2 => { let v=if fast && a.len()>=6 && a[0]!=0 && a[1]==0 && a[2]==0
                                    && *a.last().unwrap()!=0
                                    && a[3..a.len()-1].iter().all(|&x|x==0)
                                {vsquare(a,p)} else {vproduct(a,a,p)};
                        v.len() as u64+v.last().copied().unwrap_or(0) },
                    _ => unreachable!(),
                };
                checksum=checksum.wrapping_add(black_box(v));
            }
        }
        black_box(checksum);
        start.elapsed().as_nanos() as f64/(reps*data.len()) as f64
    }
    let names=[
        "four_term_0_4_8_11","four_term_3_6_9_11","five_term_control",
        "binomial_0_11","trinomial_control","dense_7_control","dense_4_control"
    ];
    for p in [1_000_000_007u64,MAX_PRIME] {
        for op in 0..3 {
            for kind in 0..7 {
                let data=fixtures(p,op,kind);
                let (mut before,mut after)=(Vec::new(),Vec::new());
                for i in 0..13 {
                    if i%2==0 {
                        before.push(sample(p,op,kind,&data,false));
                        after.push(sample(p,op,kind,&data,true));
                    }else{
                        after.push(sample(p,op,kind,&data,true));
                        before.push(sample(p,op,kind,&data,false));
                    }
                }
                before.sort_by(f64::total_cmp);
                after.sort_by(f64::total_cmp);
                println!("round29 RUST p={p} op={op} kind={} {:.3} -> {:.3} ns ({:.3}x)",
                    names[kind],before[6],after[6],before[6]/after[6]);
            }
        }
    }
}
'''
rust = "const MAX_PRIME:u64=(1u64<<61)-1;\nfn trim(mut a:Vec<u64>)->Vec<u64>{while a.last()==Some(&0){a.pop();}a}\n" + production + "\n" + baseline + checks
with tempfile.TemporaryDirectory() as temp:
    path = Path(temp)/"round29_verifier.rs"
    binary = Path(temp)/"round29_verifier"
    path.write_text(rust)
    subprocess.run(["rustc","--edition=2021","-O","--test",str(path),"-o",str(binary)],check=True)
    subprocess.run([str(binary),"round29_exact_frozen_comparisons","--nocapture"],check=True)
    subprocess.run([str(binary),"round29_paired_actual_rust","--ignored","--nocapture"],check=True)
