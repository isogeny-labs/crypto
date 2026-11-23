
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
        let pair = f.add(f.add(f.mul(a, b), f.mul(a, c)), f.mul(b, c));
        let sum = f.add(f.add(a, b), c);
        let poly = vec![f.sub(0, f.mul(f.mul(a, b), c)), pair, f.sub(0, sum), 1];
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
