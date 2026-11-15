use std::collections::{BTreeMap, BTreeSet};

const MODULUS: u16 = 0x83; // z^7 + z + 1

fn mul(mut x: u16, mut y: u16) -> u16 {
    let mut z = 0;
    while y != 0 {
        if y & 1 != 0 { z ^= x; }
        y >>= 1;
        x <<= 1;
        if x & 128 != 0 { x ^= MODULUS; }
    }
    z
}

fn pow(mut x: u16, mut n: u16) -> u16 {
    let mut z = 1;
    while n != 0 {
        if n & 1 != 0 { z = mul(z, x); }
        x = mul(x, x);
        n >>= 1;
    }
    z
}

fn trace(mut x: u16) -> u16 {
    let mut z = 0;
    for _ in 0..7 {
        z ^= x;
        x = mul(x, x);
    }
    assert!(z <= 1);
    z
}

fn kernel(arm: usize) -> BTreeSet<u16> {
    (0..128).filter(|&x| {
        let x2 = mul(x, x);
        let x4 = mul(x2, x2);
        let x8 = mul(x4, x4);
        (if arm == 0 { x8 ^ x2 ^ x } else { x8 ^ x4 ^ x }) == 0
    }).collect()
}

fn order(a: u16, b: u16) -> u16 {
    let mut count = 2; // infinity and the unique x=0 point
    for x in 1..128 {
        let x2 = mul(x, x);
        let rhs = x ^ a ^ mul(b, pow(x2, 126));
        if trace(rhs) == 0 { count += 2; }
    }
    count
}

fn main() {
    for x in 1..128 { assert_eq!(mul(x, pow(x, 126)), 1); }
    assert_eq!(trace(1), 1);
    let kernels = [kernel(0), kernel(1)];
    assert_eq!(kernels[0].len(), 8);
    assert_eq!(kernels[1].len(), 8);
    assert_eq!(kernels[0].intersection(&kernels[1]).count(), 1);
    let mut w1 = BTreeSet::new();
    for (arm, v) in kernels.iter().enumerate() {
        let mut one = BTreeSet::new();
        for &r in v.iter().filter(|&&x| x != 0) {
            for &s in v.iter().filter(|&&x| x != 0) {
                let z = mul(r, s ^ 1);
                one.insert(mul(z, z));
            }
        }
        assert_eq!(one.len(), 42, "arm {}", arm + 1);
        w1.extend(one);
    }
    assert_eq!(w1.len(), 56);
    let classical: BTreeSet<_> = kernels[0].union(&kernels[1]).copied().filter(|&x| x != 0).collect();
    assert_eq!(classical.len(), 14);
    assert!(classical.is_subset(&w1));

    println!("b,a,order,t,classical,place7,exceptional_subfield_trace");
    let mut classical_traces = BTreeSet::new();
    let mut nonclassical_traces = BTreeSet::new();
    let mut counts: BTreeMap<(i16, bool), usize> = BTreeMap::new();
    for &b in &w1 {
        for a in [0, 1] {
            let n = order(a, b);
            let t = 129_i16 - n as i16;
            assert!((107..=151).contains(&n));
            assert_eq!(n % 2, 0);
            assert_eq!(t & 1, 1);
            let old = classical.contains(&b);
            let place7 = 18 - t;
            if old { classical_traces.insert(t); } else { nonclassical_traces.insert(t); }
            *counts.entry((t, old)).or_default() += 1;
            println!("{b},{a},{n},{t},{old},{place7},{}", t == 13 || t == -13);
        }
    }
    eprintln!("w1_b={} classical_b={} nonclassical_b={} rows={}", w1.len(), classical.len(), w1.len()-classical.len(), 2*w1.len());
    eprintln!("classical_traces={classical_traces:?}");
    eprintln!("nonclassical_traces={nonclassical_traces:?}");
    eprintln!("nonclassical_only_traces={:?}", nonclassical_traces.difference(&classical_traces).collect::<Vec<_>>());
    eprintln!("counts_by_trace_and_class={counts:?}");
}
