use num_bigint::BigUint;
fn counts(b: usize, max_m: usize) -> Vec<BigUint> {
    let mut exact = vec![BigUint::from(0u8); max_m+1]; exact[0]=BigUint::from(1u8);
    for _ in 0..b {
        let mut prefix=BigUint::from(0u8);
        let mut next=Vec::new();
        for value in &exact { next.push(value+(&prefix<<1usize)); prefix+=value; }
        exact=next;
    }
    let mut parity=exact;
    for m in 2..=max_m { let earlier=parity[m-2].clone();parity[m]+=earlier; }
    parity
}
fn main() {
    for b in [1usize,2] {
        for (m,v) in counts(b,128).iter().enumerate() {
            let expected=BigUint::from(m+1).pow(b as u32);assert_eq!(*v,expected);
        }
    }
    for b in 1..=128 { assert_eq!(counts(b,2)[2],BigUint::from(2*b*b+1)); }
    let r:BigUint="7237005577332262213973186563042994240857116359379907606001950938285454250989".parse().unwrap();
    let n=counts(128,128);let trials=BigUint::from(1_000_000u64);
    let first=(1..=128).find(|&m| &n[m]*&trials>=&r-1u8).unwrap();
    println!("checks: B=1,2 closed forms through m=128; pair identity B=1..128: passed");
    println!("B,m,coefficient_vectors,bits,hit_union_numerator_for_1M_probes");
    for m in [2,4,8,16,32,first-1,first,64] {
        println!("128,{m},{},{},{}",n[m],n[m].bits(),&n[m]*&trials);
    }
    println!("first m whose coverage upper bound stops excluding unit expected hit count at 1M probes: {first}");
    for h in [first/2,(first+1)/2] { println!("split side m={h}: {} coefficient vectors ({} bits)",n[h],n[h].bits()); }
}
