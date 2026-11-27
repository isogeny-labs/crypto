// Appended identically to baseline.rs and optimized.rs; only source under
// src/cryptanalysis/ecbench/isogeny.rs differs between A and B.
pub fn round141_cases(p: u64, kind: &str) -> Vec<Vec<u64>> {
    let mut tally = FieldTally::default();
    let mut f = Fp { p, tally: &mut tally };
    let mut seed = p ^ 0x140_2026_11_23u64;
    let mut v = Vec::with_capacity(32);
    for i in 0..32u64 {
        let mut next = || {
            seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
            seed % p
        };
        let (a, b, c, d) = (next(), next(), next(), next());
        let a = if a == 0 { 1 } else { a };
        let b = if b == 0 { 1 } else { b };
        let polynomial = match kind {
            "inverse_2" => vec![2],
            "inverse_3" => vec![3],
            "inverse_random" => vec![a],
            "isogeny2_general" | "isogeny3_general" => vec![a, b],
            "psi3_general" => {
                let aa = f.mul(a, a);
                vec![f.sub(0, aa), f.mul(12 % p, b), f.mul(6 % p, a), 0, 3 % p]
            },
            "quadratic_scaled_2" | "quadratic_scaled_3" => {
                let k = if kind == "quadratic_scaled_2" { 2 } else { 3 };
                let roots_sum = f.add(a, b);
                let roots_product = f.mul(a, b);
                vec![f.mul(k, roots_product),
                     f.mul(k, f.sub(0, roots_sum)), k]
            },
            "quartic_scaled_3" => vec![f.mul(3, a), f.mul(3, b),
                                        f.mul(3, c), f.mul(3, d), 3],
            "normalize_random" => vec![a, b, c, if p > 5 {4+d%(p-4)} else {1+d%(p-1)}],
            "quadratic_random_scaled" => vec![a, b, if p > 5 {4+d%(p-4)} else {1+d%(p-1)}],
            "cubic_random_scaled" => vec![a, b, c, if p > 5 {4+d%(p-4)} else {1+d%(p-1)}],
            "quartic_random_scaled" => vec![a, b, c, d, if p > 5 {4+a%(p-4)} else {1+a%(p-1)}],
            "quartic_monic" | "powmod_dense" => vec![a, b, c, d, 1],
            "gcd" => vec![a, b, c, d, 1],
            _ => panic!("unexpected kind {kind} {i}"),
        };
        v.push(polynomial);
    }
    v
}

pub fn round141_benchmark(p:u64, kind:&str, repeats:usize)->u64 {
    let inputs=round141_cases(p,kind);
    let mut tally=FieldTally::default();
    let mut field=Fp {p,tally:&mut tally};
    let mut accumulator=0u64;
    for _ in 0..repeats {
        for input in &inputs {
            let fingerprint=match kind {
                "inverse_2" | "inverse_3" | "inverse_random" =>
                    field.inv(std::hint::black_box(input[0])),
                "isogeny2_general" | "isogeny3_general" => {
                    let ell=if kind=="isogeny2_general" {2} else {3};
                    let curve=PrimeCurve {p,a:input[0],b:input[1]};
                    let mut work=FieldTally::default();
                    let phi=find_isogeny(std::hint::black_box(&curve),ell,&mut work);
                    match phi {
                        Some(phi)=>phi.x0.wrapping_add(phi.target.a.wrapping_mul(131))
                                              .wrapping_add(phi.target.b.wrapping_mul(133)),
                        None=>0,
                    }
                },
                "normalize_random" => {
                    let mut polynomial = input.clone();
                    field.make_monic(&mut polynomial);
                    polynomial.iter().fold(polynomial.len() as u64,
                        |s,&x|s.wrapping_mul(131).wrapping_add(x))
                },
                "powmod_dense" => {
                    let output=field.powmod(&[0,1],p,std::hint::black_box(input));
                    output.iter().fold(output.len() as u64,
                       |s,&x|s.wrapping_mul(131).wrapping_add(x))
                },
                "gcd" => {
                    let q=[11%p,7%p,3%p,2%p];
                    let result=field.gcd(std::hint::black_box(input),&q);
                    result.iter().fold(result.len() as u64,
                       |s,&x|s.wrapping_mul(131).wrapping_add(x))
                },
                _ => {
                    let roots=field.roots(std::hint::black_box(input));
                    roots.iter().fold(roots.len() as u64,
                       |s,&x|s.wrapping_mul(131).wrapping_add(x))
                },
            };
            accumulator=accumulator.wrapping_add(std::hint::black_box(fingerprint));
        }
    }
    std::hint::black_box(accumulator)
}

pub fn round141_oracle() {
    for p in [3u64,5,7,11,13,17,19,23,31,101,257,65_521,
              1_000_000_007,1_000_000_009,4_294_967_291,(1u64<<61)-1] {
        let mut tally=FieldTally::default();
        let mut field=Fp {p,tally:&mut tally};
        for a in [2u64,3] {
            if a>=p {continue;}
            let inv=field.inv(a);
            assert_eq!(inv,field.pow(a,p-2),"Fermat reference p={p} a={a}");
            assert_eq!(field.mul(a,inv),1,"multiplicative identity p={p} a={a}");
        }
        if p<=31 {
            for r in 0..p {
                for s in 0..p {
                    for k in [2u64,3] {
                        if k>=p {continue;}
                        let b=field.sub(0,field.add(r,s));
                        let c=field.mul(r,s);
                        let poly=[field.mul(k,c),field.mul(k,b),k];
                        let actual=field.roots(&poly);
                        let expected:Vec<u64>=(0..p).filter(|&x| {
                            let mut t=0;
                            for &c in poly.iter().rev(){let product=field.mul(t,x);t=field.add(product,c);}
                            t==0
                        }).collect();
                        assert_eq!(actual,expected,"scaled quadratic p={p} r={r} s={s}");
                    }
                }
            }
        }
    }
    // Independent coefficient identity and random-leading quadratic oracle.
    for p in [3u64, 5, 7, 11, 17, 101, 65_521, 1_000_000_007,
              4_294_967_291, (1u64 << 61)-1] {
        let mut tally=FieldTally::default();
        let mut field=Fp {p,tally:&mut tally};
        for i in 1..257u64 {
            let lead=if p>5 {4+i%(p-4)} else {1+i%(p-1)};
            let original=vec![(17*i)%p, (37*i)%p, lead];
            let mut normalized=original.clone();
            field.make_monic(&mut normalized);
            assert_eq!(normalized.last(),Some(&1), "p={p}, i={i}");
            for j in 0..original.len()-1 {
                assert_eq!(field.mul(lead,normalized[j]), original[j],
                           "nonmonic normalization p={p}, i={i}, index={j}");
            }
            if p <= 101 {
                let brute:Vec<u64>=(0..p).filter(|&x| {
                    let mut accum=0;
                    for &coefficient in original.iter().rev() {
                        let product=field.mul(accum,x);
                        accum=field.add(product,coefficient);
                    }
                    accum==0
                }).collect();
                assert_eq!(field.roots(&original),brute,
                           "random nonmonic quadratic p={p}, i={i}");
            }
        }
    }
    // Random full-width leading coefficients, compared against independent
    // Fermat inversion. This probes beyond the small deterministic coefficients.
    for p in [65_521u64, 1_000_000_007, 4_294_967_291, (1u64 << 61)-1] {
        let mut tally=FieldTally::default();
        let mut field=Fp {p,tally:&mut tally};
        let mut state=p^0x141_5054_2026u64;
        for _ in 0..4096 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let lead=1+state%(p-1);
            let mut polynomial=vec![1,lead];
            field.make_monic(&mut polynomial);
            assert_eq!(polynomial[0],field.pow(lead,p-2),
                       "random full-width leading inverse p={p}, lead={lead}");
        }
    }
    println!("round141 independent inversion and quadratic oracles PASS");
}

pub fn round142_powmod_oracle() {
    for p in [3u64, 5, 7, 17, 101, 65_521, 1_000_000_007,
              4_294_967_291, (1u64<<61)-1] {
        let mut tally=FieldTally::default();
        let mut f=Fp {p,tally:&mut tally};
        let mut state=p^0x142_2026_11_23u64;
        for degree in [3usize,4] {
            for _ in 0..64 {
                let mut modulus=Vec::with_capacity(degree+1);
                for _ in 0..degree {
                    state^=state<<13; state^=state>>7; state^=state<<17;
                    modulus.push(state%p);
                }
                modulus.push(1);
                for exponent in [0u64,1,2,3,11,31,257,(p-1)/2,p-1,p] {
                    let observed=f.powmod(&[0,1],exponent,&modulus);
                    let mut wanted=vec![1];
                    let mut base=f.rem(&[0,1],&modulus);
                    let mut e=exponent;
                    while e!=0 {
                        if e&1==1 { wanted=f.mulmod(&wanted,&base,&modulus); }
                        e>>=1;
                        if e!=0 { base=f.mulmod(&base,&base,&modulus); }
                    }
                    assert_eq!(observed,wanted,"p={p} degree={degree} exp={exponent}");
                }
            }
        }
    }
    println!("round142 independent polynomial exponentiation oracle PASS");
}
