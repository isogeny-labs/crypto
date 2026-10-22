use std::time::Instant;
use std::hint::black_box;
#[inline(never)]
pub fn invmod(a: u64, m: u64) -> Option<u64> {
    let (mut r0, mut r1) = (m, a % m);
    let (mut t0, mut t1) = (0u64, 1u64);
    let mut neg0 = true;
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 + q * t1);
        neg0 = !neg0;
    }
    if r0 != 1 { return None; }
    Some(if neg0 { (m - t0) % m } else { t0 % m })
}
#[inline(never)]
pub fn inv_bin(a: u64, p: u64) -> u64 {
    let (mut u, mut v) = (a, p);
    let (mut x1, mut x2) = (1u64, 0u64);
    #[inline(always)]
    fn halve(x: u64, p: u64) -> u64 { if x & 1 == 0 { x >> 1 } else { ((x as u128 + p as u128) >> 1) as u64 } }
    while u != 1 && v != 1 {
        while u & 1 == 0 { u >>= 1; x1 = halve(x1, p); }
        while v & 1 == 0 { v >>= 1; x2 = halve(x2, p); }
        if u >= v { u -= v; x1 = if x1 >= x2 { x1 - x2 } else { x1 + p - x2 }; }
        else { v -= u; x2 = if x2 >= x1 { x2 - x1 } else { x2 + p - x1 }; }
    }
    if u == 1 { x1 } else { x2 }
}
fn main() {
    let ps: [u64; 6] = [65521, 16777213, 1073741789, 4294967291, 1099511627689, 4611686018427387847];
    let mut s: u64 = 0x9e3779b97f4a7c15;
    for &p in &ps {
        let xs: Vec<u64> = (0..200_000).map(|_| { s ^= s << 13; s ^= s >> 7; s ^= s << 17; 1 + s % (p - 1) }).collect();
        for &x in &xs[..2000] { assert_eq!(invmod(x, p).unwrap(), inv_bin(x, p)); }
        let mut best = [f64::MAX; 2];
        for _ in 0..7 {
            let t = Instant::now(); let mut acc = 0u64;
            for &x in &xs { acc ^= invmod(black_box(x), p).unwrap(); } black_box(acc);
            best[0] = best[0].min(t.elapsed().as_nanos() as f64 / xs.len() as f64);
            let t = Instant::now(); let mut acc = 0u64;
            for &x in &xs { acc ^= inv_bin(black_box(x), p); } black_box(acc);
            best[1] = best[1].min(t.elapsed().as_nanos() as f64 / xs.len() as f64);
        }
        println!("bits={:2} euclid={:6.1} ns binary={:6.1} ns ratio={:.2}", 64 - p.leading_zeros(), best[0], best[1], best[1] / best[0]);
    }
}
