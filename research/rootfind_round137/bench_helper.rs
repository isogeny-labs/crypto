// Appended to both source-exact production isogeny.rs versions by CI.
// The GCD result is normalized: a field-valued scalar is not meaningful.
pub fn round137_gcd_benchmark(p: u64, reps: usize) -> u64 {
    let mut tally = FieldTally::default();
    let mut field = Fp { p, tally: &mut tally };
    let mut cases = Vec::new();
    let mut state = p ^ 0x137_2026_11_23u64;
    for _ in 0..32 {
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % p
        };
        let a = vec![next(), next(), next(), next(), next()];
        let b = vec![next(), next(), next(), next()];
        cases.push((a,b));
    }
    let mut checksum = 0u64;
    for _ in 0..reps {
        for (a,b) in &cases {
            let mut d = field.gcd(std::hint::black_box(a), std::hint::black_box(b));
            if d.len() > 1 { field.make_monic(&mut d); }
            else if !d.is_empty() { d[0] = 1; }
            let fingerprint = d.iter().fold(d.len() as u64,
                |h,&x|h.wrapping_mul(131).wrapping_add(x));
            checksum = checksum.wrapping_add(std::hint::black_box(fingerprint));
        }
    }
    checksum
}

pub fn round137_cases(p: u64, kind: &str) -> Vec<Vec<u64>> {
    let mut tally = FieldTally::default();
    let mut f = Fp { p, tally: &mut tally };
    let mut state = p ^ 0x137_5047_2026u64;
    let mut cases = Vec::new();
    for i in 0..32u64 {
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % p
        };
        let (a,b,c,d) = (next(),next(),next(),next());
        let poly = match kind {
            "cubic" => round129_linear_factors(&mut f, &[a,b,c]),
            "quartic" => round129_linear_factors(&mut f, &[a,b,c,d]),
            "random_cubic" => vec![a,b,c,1],
            "random_quartic" => vec![a,b,c,d,1],
            "psi3" => round129_psi3(&mut f, a,b),
            "quartic_scaled" => round129_linear_factors(&mut f, &[a,b,c,d])
                .iter().map(|&x| f.mul(x, 3 % p)).collect(),
            _ => panic!("unsupported round137 case {kind}, i={i}"),
        };
        cases.push(poly);
    }
    cases
}

pub fn round137_independent_gcd_oracle() {
    for p in [3u64,5,7,11,13,17,19,31,101,65_521,65_519,
              1_000_000_007,4_294_967_291,(1u64<<61)-1] {
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        let mut state = p ^ 0x137_0ac1_2026u64;
        for _ in 0..250 {
            let mut next = || {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state % p
            };
            let mut a = vec![next(),next(),next(),next(),next()];
            let mut b = vec![next(),next(),next(),next()];
            if a.last() == Some(&0) { a[4] = 1; }
            if b.last() == Some(&0) { b[3] = 1; }
            // Independent field-Euclidean algorithm using true remainders.
            let mut x = a.clone();
            let mut y = b.clone();
            Fp::trim(&mut x);
            Fp::trim(&mut y);
            while !y.is_empty() {
                let r = f.rem(&x,&y);
                x=y;
                y=r;
            }
            let mut got = f.gcd(&a,&b);
            if x.len()>1 { f.make_monic(&mut x); }
            else if !x.is_empty() { x[0]=1; }
            if got.len()>1 { f.make_monic(&mut got); }
            else if !got.is_empty() { got[0]=1; }
            assert_eq!(got,x,"independent Euclid p={p}");
        }
        for i in 0..64u64 {
            let roots=[(3+i*7)%p,(13+i*11)%p,(23+i*17)%p,(37+i*19)%p];
            let a=round129_linear_factors(&mut f,&roots[..3]);
            let b=round129_linear_factors(&mut f,&roots[1..]);
            let mut common=f.gcd(&a,&b);
            if common.len()>1 {f.make_monic(&mut common);}
            let expected=round129_linear_factors(&mut f,&roots[1..3]);
            // Degeneracies from repeated roots in very small fields are
            // allowed; the independent Euclid comparison above covers them.
            if p>101 {assert_eq!(common,expected,"shared factors p={p}");}
        }
        for kind in ["cubic","quartic","random_cubic","random_quartic","psi3"] {
            for poly in round137_cases(p,kind).iter().take(16) {
                for &root in &f.roots(poly) {
                    let value=poly.iter().rev().fold(0u64,|acc,&c|{
                        let t=f.mul(acc,root);f.add(t,c)
                    });
                    assert_eq!(value,0,"root evaluation p={p} kind={kind}");
                }
            }
        }
    }
    println!("round137 independent polynomial Euclid and root oracle: PASS");
}
