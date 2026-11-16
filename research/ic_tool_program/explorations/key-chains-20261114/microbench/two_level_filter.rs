// Two-level presence filter micro-benchmark: today's one-word, three-bit filter
// (8 bits a key, power-of-two words) against an L2-sized first level probed
// before it. Queries in blocks of 1024 with the scan's 32-ahead prefetch.
use std::time::Instant;
#[inline(always)] fn hash(k: u64) -> u64 { let mut h = k.wrapping_mul(0xff51_afd7_ed55_8ccd); h ^= h >> 33; h.wrapping_mul(0xc4ce_b9fe_1a85_ec53) }
#[inline(always)] fn pf(p: *const u64) { unsafe { std::arch::x86_64::_mm_prefetch::<{ std::arch::x86_64::_MM_HINT_T0 }>(p as *const i8) } }
struct F { w: Vec<u64>, shift: u32, s: [u32; 3] }
impl F {
    fn new(keys: usize, bits_per_key: usize, s: [u32; 3]) -> F {
        let bits = (keys * bits_per_key).next_power_of_two().max(64);
        let words = bits / 64; let wb = words.trailing_zeros();
        F { w: vec![0; words], shift: 64 - wb, s }
    }
    #[inline(always)] fn word(&self, h: u64) -> usize { if self.w.len() == 1 { 0 } else { (h >> self.shift) as usize } }
    #[inline(always)] fn bits(&self, h: u64) -> u64 { (1u64 << ((h >> self.s[0]) & 63)) | (1u64 << ((h >> self.s[1]) & 63)) | (1u64 << ((h >> self.s[2]) & 63)) }
    fn set(&mut self, h: u64) { let w = self.word(h); self.w[w] |= self.bits(h); }
    #[inline(always)] fn test(&self, h: u64) -> bool { let b = self.bits(h); self.w[self.word(h)] & b == b }
}
fn main() {
    let n: usize = std::env::args().nth(1).map_or(6_265_920, |v| v.parse().unwrap());
    let l1_bits: usize = std::env::args().nth(2).map_or(2, |v| v.parse().unwrap());
    let mut x = 0x9e3779b97f4a7c15u64; let mut rnd = || { x ^= x << 13; x ^= x >> 7; x ^= x << 17; x };
    let stored: Vec<u64> = (0..n).map(|_| rnd()).collect();
    // Level 2 = today's: word from top bits, in-word bits 16..33 (top layout).
    let mut f2 = F::new(n, 8, [16, 22, 28]);
    // Level 1: L2-sized; word from a disjoint bit range (bits 34..), in-word bits 0..17.
    let mut f1 = F::new(n, l1_bits, [0, 6, 12]);
    let f1_shift_src = 34u32; // use bits starting at 34 for the L1 word index
    let l1w = |f: &F, h: u64| -> usize { ((h >> f1_shift_src) as usize) & (f.w.len() - 1) };
    for &k in &stored { let h = hash(k); f2.set(h); let w = l1w(&f1, h); let b = f1.bits(h); f1.w[w] |= b; }
    let q = 20_000_000usize;
    let queries: Vec<u64> = (0..q).map(|i| if i % 32 == 0 { stored[(rnd() % n as u64) as usize] } else { rnd() }).map(hash).collect();
    println!("n {n}  L2 filter {} KiB  L1 filter {} KiB ({l1_bits} bits/key)", f2.w.len() * 8 / 1024, f1.w.len() * 8 / 1024);
    const LA: usize = 32; const B: usize = 1024;
    for rep in 0..3 {
        let t = Instant::now(); let mut adm = 0usize;
        for blk in queries.chunks(B) {
            for &h in blk.iter().take(LA) { pf(&f2.w[f2.word(h)]); }
            for i in 0..blk.len() { if let Some(&a) = blk.get(i + LA) { pf(&f2.w[f2.word(a)]); } if f2.test(blk[i]) { adm += 1; } }
        }
        let one = t.elapsed().as_secs_f64() * 1e9 / q as f64;
        let t = Instant::now(); let mut adm2 = 0usize; let mut pass1 = 0usize;
        let mut cand: Vec<u64> = Vec::with_capacity(B);
        for blk in queries.chunks(B) {
            cand.clear();
            unsafe { cand.set_len(B); }
            let mut len = 0usize;
            for &h in blk.iter().take(16) { pf(&f1.w[l1w(&f1, h)]); }
            for i in 0..blk.len() {
                if let Some(&a) = blk.get(i + 16) { pf(&f1.w[l1w(&f1, a)]); }
                let h = blk[i]; let b = f1.bits(h);
                cand[len] = h; len += (f1.w[l1w(&f1, h)] & b == b) as usize;
            }
            unsafe { cand.set_len(len); }
            pass1 += cand.len();
            for &h in cand.iter().take(LA) { pf(&f2.w[f2.word(h)]); }
            for i in 0..cand.len() { if let Some(&a) = cand.get(i + LA) { pf(&f2.w[f2.word(a)]); } if f2.test(cand[i]) { adm2 += 1; } }
        }
        let two = t.elapsed().as_secs_f64() * 1e9 / q as f64;
        assert!(adm2 <= adm);
        println!("rep {rep}: one level {one:.2} ns/query  two level {two:.2} ns/query  (L1 pass {:.3}, admitted {:.4})", pass1 as f64 / q as f64, adm as f64 / q as f64);
    }
}
