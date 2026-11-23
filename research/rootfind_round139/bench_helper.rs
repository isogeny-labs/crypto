// Source-exact Rust benchmark helpers appended to baseline and optimized modules.
// Deterministic batches avoid input-generation costs inside the timed section.
pub fn round139_cases(p: u64, kind: &str) -> Vec<Vec<u64>> {
    let mut tally = FieldTally::default();
    let mut f = Fp { p, tally: &mut tally };
    let mut state = p ^ 0x139_2026_11_23u64;
    let mut result = Vec::with_capacity(32);
    for i in 0..32u64 {
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % p
        };
        let (a, b, c, d) = (next(), next(), next(), next());
        result.push(match kind {
            "even_four_roots" => {
                let r = f.mul(a, a);
                let s = f.mul(b, b);
                let sum = f.add(r, s);
                vec![f.mul(r, s), 0, f.sub(0, sum), 0, 1]
            }
            "even_random" => vec![a, 0, b, 0, 1],
            "even_scaled" => {
                let leading = if c == 0 { 1 } else { c };
                vec![f.mul(leading, a), 0, f.mul(leading, b), 0, leading]
            }
            "binomial_quartic" => vec![a, 0, 0, 0, 1],
            "psi3_b0" => {
                let aa = if a == 0 { 1 } else { a };
                let aa2 = f.mul(aa, aa);
                vec![f.sub(0, aa2), 0, f.mul(6 % p, aa), 0, 3 % p]
            }
            "psi3_general" => {
                let aa = if a == 0 { 1 } else { a };
                let bb = if b == 0 { 1 } else { b };
                let aa2 = f.mul(aa, aa);
                vec![f.sub(0, aa2), f.mul(12 % p, bb), f.mul(6 % p, aa), 0, 3 % p]
            }
            "random_quartic" => vec![a, b, c, d, 1],
            "iso3_b0" => vec![if a == 0 { 1 } else { a }],
            _ => panic!("unknown round139 kind {kind}, i={i}"),
        });
    }
    result
}

pub fn round139_benchmark(p: u64, kind: &str, repeats: usize) -> u64 {
    let cases = round139_cases(p, kind);
    let mut tally = FieldTally::default();
    let mut sum = 0u64;
    if kind == "iso3_b0" {
        for _ in 0..repeats {
            for pcase in &cases {
                let curve = PrimeCurve { p, a: pcase[0], b: 0 };
                let fingerprint = match find_isogeny(std::hint::black_box(&curve), 3, &mut tally) {
                    Some(phi) => phi.x0.wrapping_mul(131)
                        .wrapping_add(phi.target.a).wrapping_mul(131)
                        .wrapping_add(phi.target.b),
                    None => 0,
                };
                sum = sum.wrapping_add(std::hint::black_box(fingerprint));
            }
        }
    } else {
        let mut f = Fp { p, tally: &mut tally };
        for _ in 0..repeats {
            for polynomial in &cases {
                let roots = f.roots(std::hint::black_box(polynomial));
                let fingerprint = roots.iter().fold(roots.len() as u64,
                    |acc, &root| acc.wrapping_mul(131).wrapping_add(root));
                sum = sum.wrapping_add(std::hint::black_box(fingerprint));
            }
        }
    }
    sum
}

pub fn round139_independent_oracle() {
    for p in [3u64, 5, 7, 11, 13, 17, 19, 23, 31] {
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        for c0 in 0..p {
            for c2 in 0..p {
                let poly = [c0, 0, c2, 0, 1];
                let got = f.roots(&poly);
                let mut expected = Vec::new();
                for x in 0..p {
                    let mut value = 0;
                    for &coefficient in poly.iter().rev() {
                        let product = f.mul(value, x);
                        value = f.add(product, coefficient);
                    }
                    if value == 0 {
                        expected.push(x);
                    }
                }
                assert_eq!(got, expected, "p={p} poly={poly:?}");
            }
        }
        for a in 1..p {
            let curve = PrimeCurve { p, a, b: 0 };
            let mut iso_tally = FieldTally::default();
            let iso = find_isogeny(&curve, 3, &mut iso_tally);
            if matches!(p % 12, 5 | 7) {
                assert!(iso.is_none(), "3-isogeny without square 3 p={p} a={a}");
            }
            if let Some(isogeny) = iso {
                let psi = [
                    { let aa = f.mul(a, a); f.sub(0, aa) }, 0, f.mul(6 % p, a), 0, 3 % p,
                ];
                let mut val = 0;
                for &coeff in psi.iter().rev() {
                    let product = f.mul(val, isogeny.x0);
                    val = f.add(product, coeff);
                }
                assert_eq!(val, 0, "isogeny root does not satisfy psi3");
            }
        }
    }
    println!("round139 independent small-field oracle PASS");
}
