
// Appended to both exact production modules for the Round 135 Rust A/B.
// The Euler criterion and independent root evaluation check all outcomes.
pub fn round135_sqrt_oracle() {
    let primes = [5u64, 13, 29, 37, 53, 61, 101, 109, 149, 229, 269,
                  65_213, 1_000_000_021, 4_294_967_029, 2_305_843_009_213_684_517];
    for p in primes {
        assert_eq!(p % 8, 5);
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        let tests = if p <= 269 { p } else { 256 };
        for k in 0..tests {
            let a = if p <= 269 { k } else {
                k.wrapping_mul(786_433).wrapping_add(19) % p
            };
            let expected = a == 0 || f.pow(a, (p - 1) / 2) == 1;
            let actual = f.sqrt_tonelli_shanks(a);
            assert_eq!(actual.is_some(), expected, "p={p} a={a}");
            if let Some(r) = actual {
                assert_eq!(f.mul(r, r), a, "p={p} a={a}");
            }
        }
        // The entire public-facing polynomial root path, not just sqrt.
        let bound = p.min(109);
        for b in 0..bound {
            for c in 0..bound {
                let polynomial = [c, b, 1];
                let got = f.roots(&polynomial);
                for &root in &got {
                    let x2 = f.mul(root, root);
                    let bx = f.mul(b, root);
                    assert_eq!(f.add(f.add(x2, bx), c), 0,
                               "p={p} b={b} c={c} root={root}");
                }
                if p <= 109 {
                    let expected: Vec<u64> = (0..p)
                        .filter(|&x| {
                            let xx = f.mul(x, x);
                            let bx = f.mul(b, x);
                            f.add(f.add(xx, bx), c) == 0
                        }).collect();
                    assert_eq!(got, expected, "p={p} b={b} c={c}");
                }
            }
        }
    }
    println!("round135 Atkin/Euler and complete quadratic oracles: PASS");
}
