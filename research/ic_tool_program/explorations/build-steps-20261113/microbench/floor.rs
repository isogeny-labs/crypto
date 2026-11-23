use std::time::Instant;
#[inline(always)]
fn hash(key: u64) -> u64 { let mut h = key.wrapping_mul(0xff51_afd7_ed55_8ccd); h ^= h >> 33; h.wrapping_mul(0xc4ce_b9fe_1a85_ec53) }
fn main() {
    let total = 3_707_880usize;
    let mut s = 0x9e3779b97f4a7c15u64;
    let mut rows = Vec::new(); let mut left = total;
    while left > 0 { let n = left.min(3000); rows.push((0..n).map(|_| { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s >> 10 }).collect::<Vec<u64>>()); left -= n; }
    let mut parts = vec![0u64; 256];
    for _ in 0..3 {
        let t = Instant::now(); let mut acc = 0u64;
        for r in &rows { for &k in r { acc ^= hash(k); } }
        let a = t.elapsed().as_secs_f64() * 1e9 / total as f64;
        let t = Instant::now();
        for r in &rows { for &k in r { let h = hash(k); parts[(h >> 56) as usize] += h; } }
        let b = t.elapsed().as_secs_f64() * 1e9 / total as f64;
        let t = Instant::now(); let mut big = vec![0u64; total];
        let mut i = 0; for r in &rows { for &k in r { big[i] = hash(k); i += 1; } }
        let c = t.elapsed().as_secs_f64() * 1e9 / total as f64;
        println!("hash only {a:.2}  hash+counter {b:.2}  hash+seq store(fresh) {c:.2} ns/key  {}", acc ^ parts[3] ^ big[5]);
    }
}
