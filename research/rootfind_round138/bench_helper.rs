// Appended to both source-exact prime-field Rust modules in CI.
// Inputs are deterministic and generated outside each timed loop.
pub fn round138_cases(p: u64, kind: &str) -> Vec<Vec<u64>> {
    let mut tally = FieldTally::default();
    let mut f = Fp { p, tally: &mut tally };
    let mut state = p ^ 0x138_2026_11_23u64;
    let mut cases = Vec::with_capacity(32);
    for i in 0..32u64 {
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % p
        };
        let (a, b, c, d) = (next(), next(), next(), next());
        cases.push(match kind {
            "psi3_j0" => round129_psi3(&mut f, 0, if a==0 { 1 } else { a }),
            "zero_quartic" => vec![0, a, b, c, 1],
            "zero_x2" => vec![0, 0, a, b, 1],
            "binomial_cubic" => vec![a, 0, 0, 1],
            "binomial_quartic_control" => vec![a, 0, 0, 0, 1],
            "psi3_general" => round129_psi3(&mut f, if a==0 { 1 } else { a }, b),
            "random_cubic" => vec![a, b, c, 1],
            "random_quartic" => vec![a, b, c, d, 1],
            "constructed_quartic" => round129_linear_factors(&mut f, &[a,b,c,d]),
            "constructed_cubic" => round129_linear_factors(&mut f, &[a,b,c]),
            _ => panic!("unknown case {kind}, i={i}")
        });
    }
    cases
}

pub fn round138_benchmark(p: u64, kind: &str, reps: usize) -> u64 {
    let mut tally = FieldTally::default();
    let mut checksum = 0u64;
    if kind.starts_with("isogeny") {
        let mut state = p ^ 0x138_150_6e6u64;
        let mut curves = Vec::with_capacity(32);
        for _ in 0..32 {
            state ^= state << 13; state ^= state >> 7; state ^= state << 17;
            let a = state % (p-1) + 1;
            state ^= state << 13; state ^= state >> 7; state ^= state << 17;
            let b = state % (p-1) + 1;
            let curve = match kind {
                "isogeny3_j0" => PrimeCurve { p, a: 0, b },
                "isogeny2_j1728" => PrimeCurve { p, a, b: 0 },
                "isogeny3_general" => PrimeCurve { p, a, b },
                "isogeny2_general" => PrimeCurve { p, a, b },
                _ => panic!("bad isogeny kind {kind}")
            };
            curves.push(curve);
        }
        let ell = if kind.starts_with("isogeny3") { 3 } else { 2 };
        for _ in 0..reps {
            for curve in &curves {
                let fingerprint = match find_isogeny(std::hint::black_box(curve),ell,&mut tally) {
                    Some(x) => x.x0.wrapping_mul(131).wrapping_add(x.target.a)
                        .wrapping_mul(131).wrapping_add(x.target.b),
                    None => 0
                };
                checksum = checksum.wrapping_add(std::hint::black_box(fingerprint));
            }
        }
        return checksum;
    }
    let mut f = Fp { p, tally: &mut tally };
    let polynomials = round138_cases(p,kind);
    for _ in 0..reps {
        for polynomial in &polynomials {
            let roots = f.roots(std::hint::black_box(polynomial));
            let fingerprint = roots.iter().fold(roots.len() as u64,
                |h,&x|h.wrapping_mul(131).wrapping_add(x));
            checksum=checksum.wrapping_add(std::hint::black_box(fingerprint));
        }
    }
    checksum
}

// Independent small-field complete enumeration, not merely root evaluation.
pub fn round138_independent_oracle() {
    for p in [5u64,7,11,13,17,19,23,31,101] {
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        for c in 0..p {
            let polynomials = [
                vec![c,0,0,1],
                vec![0,c,0,0,1],
                vec![0,0,c,0,1],
                vec![0,0,0,c,1],
                vec![0,0,0,0,1],
            ];
            for poly in polynomials {
                let got = f.roots(&poly);
                let mut expected = Vec::new();
                for x in 0..p {
                    let mut value = 0u64;
                    for &c in poly.iter().rev() {
                        let y=f.mul(value,x);
                        value=f.add(y,c);
                    }
                    if value == 0 { expected.push(x); }
                }
                assert_eq!(got,expected,"brute-force p={p}, poly={poly:?}");
            }
        }
        for t in 1..p.min(24) {
            for (ell,a,b) in [(2u64,t,0u64),(3,0,t)] {
                let iso=find_isogeny(&PrimeCurve{p,a,b},ell,&mut tally)
                    .expect("nonsingular curve and rational kernel");
                assert_eq!(iso.x0,0,"known minimal kernel root p={p} ell={ell}");
            }
        }
    }
    for p in [1_000_000_007u64,4_294_967_291] {
        let mut tally = FieldTally::default();
        let mut f = Fp{p,tally:&mut tally};
        assert_eq!(p%3,2);
        for c in 0..500u64 {
            let roots=f.roots(&[c,0,0,1]);
            assert_eq!(roots.len(),1);
            let r=roots[0];
            let r2=f.mul(r,r);let r3=f.mul(r2,r);
            assert_eq!(f.add(r3,c),0);
        }
    }
    println!("round138 independent brute-force and kernel oracles: PASS");
}
