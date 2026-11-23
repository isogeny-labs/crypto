
// Source-exact A/B harness inserted into a temporary copy of the production
// file at CI runtime. These wrappers do not change the implementations.
pub fn round128_benchmark(operation: &str, p: u64, repetitions: usize) -> u64 {
    let mut tally = FieldTally::default();
    let mut f = Fp { p, tally: &mut tally };
    let mut cases = Vec::new();
    for i in 0..32u64 {
        let a = (3 + i * 7) % p;
        let b = (13 + i * 7) % p;
        let c = (29 + i * 7) % p;
        let ab = f.mul(a, b);
        let ac = f.mul(a, c);
        let bc = f.mul(b, c);
        let pair = f.add(f.add(ab, ac), bc);
        let sum = f.add(f.add(a, b), c);
        let abc = f.mul(ab, c);
        let poly = vec![f.sub(0, abc), pair, f.sub(0, sum), 1];
        cases.push((a, b, poly));
    }
    let mut checksum = 0u64;
    for _ in 0..repetitions {
        for (a, b, poly) in &cases {
            let v = match operation {
                "mul" => f.mul(*a, *b),
                "add" => f.add(*a, *b),
                "sub" => f.sub(*a, *b),
                "pow" => f.pow(*a, 65_533),
                "rem" => f.rem(poly, &[3, 0, 0, 1]).iter().fold(0u64, |v, c| v.wrapping_add(*c)),
                "powmod" => f.powmod(&[1, 1], p, &[3, 2, 4, 1])
                    .iter().fold(0u64, |v, c| v.wrapping_add(*c)),
                "roots" => f.roots(poly).iter().fold(0u64, |v, c| v.wrapping_add(*c)),
                _ => panic!("invalid benchmark operation"),
            };
            checksum = checksum.wrapping_add(std::hint::black_box(v));
        }
    }
    checksum
}

pub fn round128_independent_oracle() {
    for p in [101u64, 65_521, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
        let mut tally = FieldTally::default();
        let mut f = Fp { p, tally: &mut tally };
        let mut state = p ^ 0x9e37_79b9_7f4a_7c15;
        for _ in 0..20_000 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let a = state % p;
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let b = state % p;
            assert_eq!(f.mul(a, b), (a as u128 * b as u128 % p as u128) as u64);
            assert_eq!(f.add(a, b), ((a as u128 + b as u128) % p as u128) as u64);
            assert_eq!(f.sub(a, b), ((a as u128 + p as u128 - b as u128) % p as u128) as u64);
        }
        for i in 0..32u64 {
            let a = (3 + i * 7) % p;
            let b = (13 + i * 7) % p;
            let c = (29 + i * 7) % p;
            let ab = f.mul(a, b);
            let ac = f.mul(a, c);
            let bc = f.mul(b, c);
            let pair = f.add(f.add(ab, ac), bc);
            let sum = f.add(f.add(a, b), c);
            let abc = f.mul(ab, c);
            let polynomial = vec![f.sub(0, abc), pair, f.sub(0, sum), 1];
            let mut expected = vec![a, b, c];
            expected.sort_unstable();
            expected.dedup();
            assert_eq!(f.roots(&polynomial), expected, "known-root cubic p={p}");
        }
    }
}
