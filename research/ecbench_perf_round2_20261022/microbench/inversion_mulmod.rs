use std::time::Instant;
use std::hint::black_box;
fn invmod(a: u64, m: u64) -> Option<u64> {
    let (mut r0, mut r1) = (m, a % m);
    let (mut t0, mut t1) = (0u64, 1u64);
    let mut neg0 = true;
    while r1 != 0 { let q = r0 / r1; (r0, r1) = (r1, r0 - q * r1); (t0, t1) = (t1, t0 + q * t1); neg0 = !neg0; }
    if r0 != 1 { return None; }
    Some(if neg0 { (m - t0) % m } else { t0 % m })
}
fn invmod32(a: u32, m: u32) -> Option<u32> {
    let (mut r0, mut r1) = (m, a % m);
    let (mut t0, mut t1) = (0u32, 1u32);
    let mut neg0 = true;
    while r1 != 0 { let q = r0 / r1; (r0, r1) = (r1, r0 - q * r1); (t0, t1) = (t1, t0 + q * t1); neg0 = !neg0; }
    if r0 != 1 { return None; }
    Some(if neg0 { (m - t0) % m } else { t0 % m })
}
// quotient-1 shortcut: most quotients are small; avoid div when r0 < 2*r1
fn invmod32s(a: u32, m: u32) -> Option<u32> {
    let (mut r0, mut r1) = (m, a % m);
    let (mut t0, mut t1) = (0u32, 1u32);
    let mut neg0 = true;
    while r1 != 0 {
        let d = r0 - r1;
        let (q, r) = if d < r1 { (1, d) } else { let q = r0 / r1; (q, r0 - q*r1) };
        (r0, r1) = (r1, r); (t0, t1) = (t1, t0 + q * t1); neg0 = !neg0; }
    if r0 != 1 { return None; }
    Some(if neg0 { (m - t0) % m } else { t0 % m })
}
fn mulmod(a: u64, b: u64, m: u64) -> u64 { (a*b)%m }
fn mulb(a: u64, b: u64, m: u64, mi: u64) -> u64 { let x=a*b; let q=((x as u128*mi as u128)>>64) as u64; let mut r=x-q*m; if r>=m {r-=m} r }
fn main(){
    let m: u64 = 4294967291; let mi = u64::MAX / m; // floor((2^64-1)/m)
    let n = 5_000_000u64;
    let mut xs = vec![0u64; n as usize]; let mut s=12345u64;
    for x in xs.iter_mut(){ s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *x = 1 + (s>>33) % (m-1); }
    for _ in 0..3 {
    let t=Instant::now(); let mut acc=0u64; for &x in &xs { acc ^= invmod(black_box(x)^ (acc&1), m).unwrap(); } println!("inv64 {:.1} ns {}", t.elapsed().as_nanos() as f64/n as f64, acc);
    let t=Instant::now(); let mut acc=0u32; for &x in &xs { acc ^= invmod32(black_box(x as u32)^(acc&1), m as u32).unwrap(); } println!("inv32 {:.1} ns {}", t.elapsed().as_nanos() as f64/n as f64, acc);
    let t=Instant::now(); let mut acc=0u32; for &x in &xs { acc ^= invmod32s(black_box(x as u32)^(acc&1), m as u32).unwrap(); } println!("inv32s {:.1} ns {}", t.elapsed().as_nanos() as f64/n as f64, acc);
    let t=Instant::now(); let mut acc=1u64; for &x in &xs { acc = mulmod(acc, black_box(x), m); } println!("mul div {:.2} ns {}", t.elapsed().as_nanos() as f64/n as f64, acc);
    let t=Instant::now(); let mut acc=1u64; for &x in &xs { acc = mulb(acc, black_box(x), m, mi); } println!("mul barrett {:.2} ns {}", t.elapsed().as_nanos() as f64/n as f64, acc);
    }
    // exhaustive-ish check barrett
    for &x in &xs { for &y in xs.iter().take(50) { assert_eq!(mulmod(x,y,m), mulb(x,y,m,mi)); } assert_eq!(invmod(x,m).unwrap() as u32, invmod32s(x as u32, m as u32).unwrap()); }
    for mm in [3u64,5,7,65521,4294967291] { let mi=u64::MAX/mm; for a in 0..mm.min(3000) { for b in [0,1,mm-1,mm/2] { assert_eq!(mulmod(a,b,mm), mulb(a,b,mm,mi)); } } }
    println!("ok");
}
