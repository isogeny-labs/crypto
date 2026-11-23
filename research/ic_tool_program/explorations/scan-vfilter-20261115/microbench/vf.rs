// The aimed scan's filter stage, a block of 1,024 keys at a time: a faithful
// scalar copy (hash, 32-ahead prefetch, test, bucket prefetch, push) against an
// AVX-512 version (vector hash, word and bits; scalar prefetch from precomputed
// words; gather, test, compress).  Same admitted offsets, checked.
use std::arch::x86_64::*;
use std::time::Instant;
#[inline(always)] fn hash(k: u64) -> u64 { let mut h = k.wrapping_mul(0xff51_afd7_ed55_8ccd); h ^= h >> 33; h.wrapping_mul(0xc4ce_b9fe_1a85_ec53) }
#[inline(always)] fn pf<T>(p: *const T) { unsafe { _mm_prefetch::<_MM_HINT_T0>(p as *const i8) } }
struct G { shift: u64, words: u64, s: [u64; 3] }
impl G {
    #[inline(always)] fn word(&self, h: u64) -> usize { ((h >> self.shift) & self.words) as usize }
    #[inline(always)] fn bits(&self, h: u64) -> u64 { (1u64 << ((h >> self.s[0]) & 63)) | (1u64 << ((h >> self.s[1]) & 63)) | (1u64 << ((h >> self.s[2]) & 63)) }
}
static HYBRID: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
const B: usize = 1024; const LA: usize = 32;
fn scalar(keys: &[u64], g: &G, present: &[u64], bstart: &[u32], bshift: u32, hashes: &mut Vec<u64>, adm: &mut Vec<usize>) -> u64 {
    let mut probes = 0u64;
    hashes.clear(); hashes.extend(keys.iter().copied().map(hash)); adm.clear();
    for &h in hashes.iter().take(LA) { pf(&present[g.word(h)]); }
    for i in 0..keys.len() {
        if let Some(&a) = hashes.get(i + LA) { pf(&present[g.word(a)]); }
        let h = hashes[i]; probes += 1;
        let b = g.bits(h);
        if present[g.word(h)] & b != b { continue; }
        pf(&bstart[(h >> bshift) as usize]);
        adm.push(i);
    }
    probes
}
#[target_feature(enable = "avx512f,avx512dq,avx512vl")]
unsafe fn vector(keys: &[u64], g: &G, present: &[u64], bstart: &[u32], bshift: u32, hashes: &mut Vec<u64>, widx: &mut Vec<u64>, bitsv: &mut Vec<u64>, adm: &mut Vec<usize>) -> u64 {
    let n = keys.len(); hashes.resize(n, 0); widx.resize(n, 0); bitsv.resize(n, 0); adm.clear();
    let (c1, c2) = (_mm512_set1_epi64(0xff51_afd7_ed55_8ccdu64 as i64), _mm512_set1_epi64(0xc4ce_b9fe_1a85_ec53u64 as i64));
    let (sh, wm) = (_mm512_set1_epi64(g.shift as i64), _mm512_set1_epi64(g.words as i64));
    let (s0, s1, s2) = (_mm512_set1_epi64(g.s[0] as i64), _mm512_set1_epi64(g.s[1] as i64), _mm512_set1_epi64(g.s[2] as i64));
    let (m63, one) = (_mm512_set1_epi64(63), _mm512_set1_epi64(1));
    let mut i = 0;
    while i + 8 <= n {
        let k = _mm512_loadu_si512(keys.as_ptr().add(i).cast());
        let mut h = _mm512_mullo_epi64(k, c1);
        h = _mm512_xor_si512(h, _mm512_srli_epi64::<33>(h));
        h = _mm512_mullo_epi64(h, c2);
        let w = _mm512_and_si512(_mm512_srlv_epi64(h, sh), wm);
        let b = _mm512_or_si512(_mm512_or_si512(
            _mm512_sllv_epi64(one, _mm512_and_si512(_mm512_srlv_epi64(h, s0), m63)),
            _mm512_sllv_epi64(one, _mm512_and_si512(_mm512_srlv_epi64(h, s1), m63))),
            _mm512_sllv_epi64(one, _mm512_and_si512(_mm512_srlv_epi64(h, s2), m63)));
        _mm512_storeu_si512(hashes.as_mut_ptr().add(i).cast(), h);
        _mm512_storeu_si512(widx.as_mut_ptr().add(i).cast(), w);
        _mm512_storeu_si512(bitsv.as_mut_ptr().add(i).cast(), b);
        i += 8;
    }
    for j in i..n { let h = hash(keys[j]); hashes[j] = h; widx[j] = g.word(h) as u64; bitsv[j] = g.bits(h); }
    for &w in widx.iter().take(LA) { pf(present.as_ptr().add(w as usize)); }
    if HYBRID.load(std::sync::atomic::Ordering::Relaxed) {
        for j in 0..n {
            if j + LA < n { pf(present.as_ptr().add(*widx.get_unchecked(j + LA) as usize)); }
            let b = *bitsv.get_unchecked(j);
            if *present.get_unchecked(*widx.get_unchecked(j) as usize) & b == b {
                pf(bstart.as_ptr().add((*hashes.get_unchecked(j) >> bshift) as usize)); adm.push(j);
            }
        }
        return n as u64;
    }
    let mut out = [0u32; 8];
    let mut i = 0;
    while i + 8 <= n {
        for a in i + LA..(i + LA + 8).min(n) { pf(present.as_ptr().add(*widx.get_unchecked(a) as usize)); }
        let w = _mm512_loadu_si512(widx.as_ptr().add(i).cast());
        let b = _mm512_loadu_si512(bitsv.as_ptr().add(i).cast());
        let got = _mm512_i64gather_epi64::<8>(w, present.as_ptr().cast());
        let m = _mm512_cmpeq_epi64_mask(_mm512_and_si512(got, b), b);
        if m != 0 {
            let mut mm = m;
            while mm != 0 { let l = mm.trailing_zeros() as usize; mm &= mm - 1;
                let h = *hashes.get_unchecked(i + l); pf(bstart.as_ptr().add((h >> bshift) as usize)); adm.push(i + l); }
        }
        i += 8;
    }
    for j in i..n { let b = bitsv[j]; if present[widx[j] as usize] & b == b { pf(&bstart[(hashes[j] >> bshift) as usize]); adm.push(j); } }
    let _ = &mut out;
    n as u64
}
fn main() {
    let stored: usize = std::env::args().nth(1).map_or(6_265_920, |v| v.parse().unwrap());
    let mut x = 0x9e3779b97f4a7c15u64; let mut rnd = || { x ^= x << 13; x ^= x >> 7; x ^= x << 17; x };
    let bits = (stored * 8).next_power_of_two(); let words = bits / 64; let wb = words.trailing_zeros() as u64;
    let g = G { shift: (64 - wb).min(63), words: (words - 1) as u64, s: [16, 22, 28] };
    let mut present = vec![0u64; words];
    let keys_stored: Vec<u64> = (0..stored).map(|_| rnd()).collect();
    for &k in &keys_stored { let h = hash(k); present[g.word(h)] |= g.bits(h); }
    let bbits = (stored / 16).next_power_of_two().trailing_zeros(); let bshift = 64 - bbits; let bstart = vec![0u32; (1 << bbits) + 1];
    let q = 16_000_000usize;
    let queries: Vec<u64> = (0..q).map(|i| if i % 32 == 0 { keys_stored[(rnd() % stored as u64) as usize] } else { rnd() }).collect();
    let (mut hs, mut wi, mut bv, mut a1, mut a2) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    println!("stored {stored}, filter {} KiB", words * 8 / 1024);
    for rep in 0..3 {
        let t = Instant::now(); let mut tot = 0usize;
        for blk in queries.chunks(B) { scalar(blk, &g, &present, &bstart, bshift as u32, &mut hs, &mut a1); tot += a1.len(); }
        let s = t.elapsed().as_secs_f64() * 1e9 / q as f64;
        let t = Instant::now(); let mut tot2 = 0usize;
        for blk in queries.chunks(B) { unsafe { vector(blk, &g, &present, &bstart, bshift as u32, &mut hs, &mut wi, &mut bv, &mut a2); } tot2 += a2.len(); }
        let v = t.elapsed().as_secs_f64() * 1e9 / q as f64;
        assert_eq!(tot, tot2);
        for blk in queries.chunks(B).take(200) { scalar(blk, &g, &present, &bstart, bshift as u32, &mut hs, &mut a1); unsafe { vector(blk, &g, &present, &bstart, bshift as u32, &mut hs, &mut wi, &mut bv, &mut a2); } assert_eq!(a1, a2); }
        HYBRID.store(true, std::sync::atomic::Ordering::Relaxed);
        let t = Instant::now(); let mut tot3 = 0usize;
        for blk in queries.chunks(B) { unsafe { vector(blk, &g, &present, &bstart, bshift as u32, &mut hs, &mut wi, &mut bv, &mut a2); } tot3 += a2.len(); }
        let hy = t.elapsed().as_secs_f64() * 1e9 / q as f64;
        HYBRID.store(false, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(tot, tot3);
        println!("rep {rep}: scalar {s:.2} ns/key  vector+gather {v:.2}  vector+scalar test {hy:.2} ns/key  admitted {:.4}", tot as f64 / q as f64);
    }
}
