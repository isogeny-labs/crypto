use std::collections::BTreeSet;
fn main() {
    let mut bases = 0u64;
    for p in [5u64, 7, 11, 13, 17, 19] {
        for mask in 1u64..(1 << ((p - 1) / 2)) {
            let base: Vec<u64> = (1..=(p - 1) / 2).filter(|i| mask & (1 << (i - 1)) != 0).collect();
            let b = base.len() as u64;
            if b > 3 { continue; }
            let signed: Vec<u64> = base.iter().flat_map(|&x| [x, p-x]).collect();
            let mut sums = BTreeSet::new();
            for &x in &signed { for &y in &signed { sums.insert((x+y)%p); } }
            assert!(sums.contains(&0));
            assert!(sums.len() as u64 <= 2*b*b+1);
            assert!(sums.iter().filter(|&&x| x != 0).count() as u64 <= 2*b*b);
            bases += 1;
        }
        for q in 1..p { for b in 1..p {
            let images: BTreeSet<u64> = (0..p).map(|a| (a+b*q)%p).collect();
            assert_eq!(images.len() as u64, p);
        }}
    }
    let b = 128u128;
    let trials = 1_000_000u128;
    let numerator = trials*(2*b*b+1);
    assert_eq!(numerator, 32_769_000_000);
    assert!(numerator < (1u128 << 35));
    println!("finite signed-base checks: {bases}; translation maps: passed");
    println!("exact numerator: {numerator} < 2^35; order-1 > 2^252; hit bound < 2^-217");
}
