// Exact finite permutation enumeration; no timing or probabilistic claims.
use std::collections::BTreeMap;
fn canonical(v: (u32,u32)) -> (u32,u32) {
    [1,2,4].iter().map(|s| ((s*v.0)%7,(s*v.1)%7)).min().unwrap()
}
fn main() {
    let mut points = Vec::new();
    for a in 0..7 { for b in 0..7 {
        if (a,b)!=(0,0) && canonical((a,b))==(a,b) { points.push((a,b)); }
    }}
    assert_eq!(points.len(),16);
    let mut histogram = BTreeMap::<Vec<usize>,usize>::new();
    let mut sl_count=0; let mut qr_count=0; let mut longest=0;
    for a in 0..7 { for b in 0..7 { for c in 0..7 { for d in 0..7 {
        let det=(a*d+49-b*c)%7;
        if ![1,2,4].contains(&det) { continue; }
        qr_count+=1;
        // Squaring permutes QR scalars; choose the unique trivial-on-points
        // scalar that turns this matrix into determinant one.
        let scalar=[1,2,4].into_iter().find(|s| (s*s*det)%7==1).unwrap();
        let mut perm=Vec::new(); let mut scaled=Vec::new();
        for &(x,y) in &points {
            let w=((a*x+b*y)%7,(c*x+d*y)%7);
            let p=canonical(w);
            perm.push(points.iter().position(|q| *q==p).unwrap());
            scaled.push(points.iter().position(|q| *q==canonical(((scalar*w.0)%7,(scalar*w.1)%7))).unwrap());
        }
        assert_eq!(perm,scaled);
        let mut sorted=perm.clone(); sorted.sort(); assert_eq!(sorted,(0..16).collect::<Vec<_>>());
        let mut seen=[false;16]; let mut cycles=Vec::new();
        for i in 0..16 {
            if seen[i] { continue; }
            let mut j=i; let mut length=0;
            while !seen[j] { seen[j]=true; length+=1; j=perm[j]; }
            assert_eq!(j,i); cycles.push(length); longest=longest.max(length);
        }
        cycles.sort(); assert!(!cycles.contains(&16));
        if det==1 { sl_count+=1; *histogram.entry(cycles).or_default()+=1; }
    }}}}
    assert_eq!(sl_count,336); assert_eq!(qr_count,1008);
    for (cycles,count) in histogram { println!("FIBER51_CYCLE_TYPE {:?} count={}",cycles,count); }
    println!("FIBER51_RESULT points=16 SL2_matrices={} QR_determinant_matrices={} longest_cycle={} degree16_single_cycles=0 scalar_normalization_checked=true",sl_count,qr_count,longest);
    let mut tested_primes=0;
    let mut first77=None; let mut first161=None;
    for p in (3u32..=29).step_by(2) {
        if (2..p).any(|d| p%d==0) { continue; }
        tested_primes+=1;
        let split=[1,2,4].contains(&(p%7));
        let good77=split && 77%p!=0; let good161=split && 161%p!=0;
        if good77 && first77.is_none() { first77=Some(p); }
        if good161 && first161.is_none() { first161=Some(p); }
        println!("FIBER51_PRIME_FILTER prime={} QR_mod7={} good_split_level77={} good_split_level161={}",p,split,good77,good161);
    }
    assert_eq!(tested_primes,9);
    assert_eq!(first77,Some(23)); assert_eq!(first161,Some(11));
    println!("FIBER51_PRIME_RESULT first_good_split_odd_prime_level77=23 first_good_split_odd_prime_level161=11 target_residual_eigenvalue_not_inferred=true");
    println!("FIBER51_ALL_CHECKS_PASS");
}
