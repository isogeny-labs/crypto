// Source-exact benchmark entrypoints appended to both production Rust files
// at CI runtime. Input generation occurs outside the timed loop.
fn round129_linear_factors(field: &mut Fp<'_>, roots: &[u64]) -> Vec<u64> {
    let mut polynomial = vec![1];
    for &root in roots {
        let mut next = vec![0u64; polynomial.len() + 1];
        for (j, &coefficient) in polynomial.iter().enumerate() {
            let product = field.mul(coefficient, root);
            next[j] = field.sub(next[j], product);
            next[j + 1] = field.add(next[j + 1], coefficient);
        }
        polynomial = next;
    }
    polynomial
}

fn round129_psi3(field: &mut Fp<'_>, a: u64, b: u64) -> Vec<u64> {
    let a2 = field.mul(a, a);
    let twelve_b = field.mul(12 % field.p, b);
    let six_a = field.mul(6 % field.p, a);
    vec![field.sub(0, a2), twelve_b, six_a, 0, 3 % field.p]
}

pub fn round129_benchmark(op: &str, p: u64, repetitions: usize) -> u64 {
    let mut tally = FieldTally::default();
    let mut field = Fp { p, tally: &mut tally };
    let mut cases = Vec::new();
    let mut state = 0x129_2026u64 ^ p;
    for i in 0..32u64 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let a = state % p;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let b = state % p;
        let roots = [(3 + 7 * i) % p, (13 + 7 * i) % p,
            (29 + 7 * i) % p, (47 + 7 * i) % p];
        let poly = match op {
            "psi3" => round129_psi3(&mut field, a, b),
            "cubic_monic" => round129_linear_factors(&mut field, &roots[..3]),
            "cubic_scaled3" => round129_linear_factors(&mut field, &roots[..3])
                .iter().map(|&x| field.mul(x, 3)).collect(),
            "quartic_monic" => round129_linear_factors(&mut field, &roots),
            "quartic_scaled3" => round129_linear_factors(&mut field, &roots)
                .iter().map(|&x| field.mul(x, 3)).collect(),
            "quartic_random" => vec![a, b, (a + b) % p, (a ^ b) % p, 3],
            _ => panic!("unsupported operation {op}"),
        };
        cases.push(poly);
    }
    let mut checksum = 0u64;
    for _ in 0..repetitions {
        for poly in &cases {
            let roots = field.roots(std::hint::black_box(poly));
            let fingerprint = roots.iter()
                .fold(roots.len() as u64, |s, &r| s.wrapping_mul(131).wrapping_add(r));
            checksum = checksum.wrapping_add(std::hint::black_box(fingerprint));
        }
    }
    checksum
}

pub fn round129_independent_oracle() {
    for p in [101u64, 65_521, 1_000_000_007, 4_294_967_291,
        (1u64 << 61) - 1] {
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        for i in 0..24u64 {
            let roots = [(3 + 7 * i) % p, (13 + 7 * i) % p,
                (29 + 7 * i) % p, (47 + 7 * i) % p];
            for degree in [3usize, 4usize] {
                let monic = round129_linear_factors(&mut f, &roots[..degree]);
                let mut expected = roots[..degree].to_vec();
                expected.sort_unstable();
                expected.dedup();
                for scalar in [1u64, 3, 7] {
                    let poly: Vec<u64> = monic.iter()
                        .map(|&x| f.mul(x, scalar)).collect();
                    assert_eq!(f.roots(&poly), expected,
                        "known roots p={p}, degree={degree}, scalar={scalar}");
                }
            }
        }
        let mut state = p ^ 0x9e37_79b9_7f4a_7c15;
        for _ in 0..48 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let a = state % p;
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let b = state % p;
            let poly = round129_psi3(&mut f, a, b);
            for root in f.roots(&poly) {
                let value = poly.iter().rev().fold(0u64, |acc, &coeff| {
                    let t = f.mul(acc, root);
                    f.add(t, coeff)
                });
                assert_eq!(value, 0, "psi3 root evaluation p={p}, a={a}, b={b}");
            }
        }
        if p == 101 {
            for a in 0..12u64 {
                for b in 0..12u64 {
                    let poly = round129_psi3(&mut f, a, b);
                    let exhaustive: Vec<u64> = (0..p).filter(|&root| {
                        poly.iter().rev().fold(0u64, |acc, &coeff| {
                            let t = f.mul(acc, root);
                            f.add(t, coeff)
                        }) == 0
                    }).collect();
                    assert_eq!(f.roots(&poly), exhaustive,
                        "exhaustive psi3 p={p}, a={a}, b={b}");
                }
            }
        }
    }
}