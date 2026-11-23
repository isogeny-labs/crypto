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


fn round129_find_isogeny_benchmark(p: u64, ell: u64, repetitions: usize) -> u64 {
    let mut state = p ^ 0x129_1511_2026;
    let mut curves = Vec::new();
    for _ in 0..32 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let a = state % p;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let b = state % p;
        curves.push(PrimeCurve { p, a, b });
    }
    let mut tally = FieldTally::default();
    let mut checksum = 0u64;
    for _ in 0..repetitions {
        for curve in &curves {
            let fingerprint = match find_isogeny(std::hint::black_box(curve), ell, &mut tally) {
                Some(iso) => iso.x0.wrapping_mul(131)
                    .wrapping_add(iso.target.a).wrapping_add(iso.target.b),
                None => 0,
            };
            checksum = checksum.wrapping_add(std::hint::black_box(fingerprint));
        }
    }
    checksum
}

pub fn round129_benchmark(op: &str, p: u64, repetitions: usize) -> u64 {
    if op == "isogeny2" || op == "isogeny3" {
        let ell = if op == "isogeny2" { 2 } else { 3 };
        return round129_find_isogeny_benchmark(p, ell, repetitions);
    }
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
            "linear_monic" => round129_linear_factors(&mut field, &roots[..1]),
            "linear_scaled3" => round129_linear_factors(&mut field, &roots[..1])
                .iter().map(|&x| field.mul(x, 3)).collect(),
            "linear_padded" => {
                let mut v = round129_linear_factors(&mut field, &roots[..1]);
                v.extend_from_slice(&[0, 0]);
                v
            },
            "quadratic_double_root" => round129_linear_factors(&mut field, &[roots[0], roots[0]]),
            "quadratic_x2_plus_1" => vec![1, 0, 1],
            "quadratic_monic" => round129_linear_factors(&mut field, &roots[..2]),
            "quadratic_scaled3" => round129_linear_factors(&mut field, &roots[..2])
                .iter().map(|&x| field.mul(x, 3)).collect(),
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
pub fn round131_quadratic_oracle() {
    // Exhaustive small-prime enumeration includes irreducible quadratics
    // and repeated roots, which the x^p-x GCD must handle correctly.
    for p in [3u64, 5, 7, 11, 13, 19, 23, 31, 101] {
        let mut tally = FieldTally::default();
        let mut field = Fp { p, tally: &mut tally };
        for b in 0..p {
            for c in 0..p {
                let expected: Vec<u64> = (0..p).filter(|&x| {
                    let square = field.mul(x, x);
                    let bx = field.mul(b, x);
                    field.add(field.add(square, bx), c) == 0
                }).collect();
                assert_eq!(field.roots(&[c, b, 1]), expected,
                    "p={p}, b={b}, c={c}");
            }
        }
    }
    // Compare known split quadratics against independently computed factors
    // across small, 32-bit, and Mersenne fields.
    for p in [65_521u64, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
        let mut tally = FieldTally::default();
        let mut field = Fp { p, tally: &mut tally };
        for i in 0..64u64 {
            let first = (37 + 139 * i) % p;
            let second = (411 + 127 * i) % p;
            let product = field.mul(first, second);
            let sum = field.add(first, second);
            let polynomial = [product, field.sub(0, sum), 1];
            let mut expected = vec![first, second];
            expected.sort_unstable();
            expected.dedup();
            assert_eq!(field.roots(&polynomial), expected, "p={p}, i={i}");
        }
    }
}

pub fn round132_low_degree_oracle() {
    for p in [3u64, 5, 7, 11, 13, 19, 23, 31, 101] {
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        for b in 0..p {
            for c in 0..p {
                let monic = [c, b, 1];
                let brute: Vec<u64> = (0..p).filter(|&x| {
                    let sq = f.mul(x, x);
                    let linear = f.mul(b, x);
                    f.add(f.add(sq, linear), c) == 0
                }).collect();
                assert_eq!(f.roots(&monic), brute, "quad p={p} b={b} c={c}");
            }
            for a in 1..p {
                let expected: Vec<u64> = (0..p).filter(|&x| {
                    let ax = f.mul(a, x);
                    f.add(ax, b) == 0
                }).collect();
                assert_eq!(f.roots(&[b, a, 0, 0]), expected, "linear p={p} a={a} b={b}");
            }
        }
    }
    for p in [65_521u64, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        for i in 0..64u64 {
            let root = (11 + 199 * i) % p;
            let leading = [1u64, 3, 7, p - 1][(i as usize) % 4];
            let prod = f.mul(leading, root);
            assert_eq!(f.roots(&[f.sub(0, prod), leading]), vec![root]);
            let second = (root + 271) % p;
            let product = f.mul(root, second);
            let sum = f.add(root, second);
            let mut expected = vec![root, second];
            expected.sort_unstable();
            expected.dedup();
            assert_eq!(f.roots(&[product, f.sub(0, sum), 1]), expected);
            assert_eq!(f.roots(&[f.mul(root, root), f.sub(0, f.add(root, root)), 1]),
                vec![root]);
        }
    }
}
