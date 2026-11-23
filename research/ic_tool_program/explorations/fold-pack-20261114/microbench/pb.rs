// Micro-benchmark of fold_partitioned's steps 1 (stream push) and 2 (counting
// sort + filter fill), copied from src/cryptanalysis/koblitz_index_calculus.rs
// at a6a2dc9022, on random keys. Variants are compared on identical output.
use std::time::Instant;

#[inline(always)]
fn hash(key: u64) -> u64 {
    let mut h = key.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    h.wrapping_mul(0xc4ce_b9fe_1a85_ec53)
}
#[inline(always)]
fn filter_slot(h: u64, mask: u64) -> (usize, u64) {
    let more = (1u64 << ((h >> 52) & 63)) | (1u64 << (h >> 58));
    (((h & mask) >> 6) as usize, (1u64 << (h & 63)) | more)
}
#[inline(always)]
fn prefetch(p: *const u64) {
    unsafe { std::arch::x86_64::_mm_prefetch::<{ std::arch::x86_64::_MM_HINT_T0 }>(p as *const i8) }
}
fn filter_bits_for(keys: usize) -> u32 {
    (usize::BITS - (keys.max(1) * 8).leading_zeros()).clamp(6, 32)
}

struct Out {
    bucket_start: Vec<u32>,
    rests: Vec<u32>,
    present: Vec<u64>,
}

// rows: slices of keys, as each_row hands them.
fn baseline(rows: &[Vec<u64>], bucket_bits: u32, part_bits_max: u32, t: &mut [f64; 2]) -> Out {
    let t0 = Instant::now();
    let bucket_shift = 64 - bucket_bits;
    let buckets = 1usize << bucket_bits;
    let part_bits = bucket_bits.min(part_bits_max);
    let parts = 1usize << part_bits;
    let part_shift = 64 - part_bits;
    let total_in: usize = rows.iter().map(|r| r.len()).sum();
    let mean = total_in / parts;
    let room = mean + 6 * (mean as f64).sqrt() as usize + 16;
    let mut hs: Vec<Vec<u64>> = (0..parts).map(|_| Vec::with_capacity(room)).collect();
    let mut ws: Vec<Vec<u32>> = (0..parts).map(|_| Vec::with_capacity(room)).collect();
    for (r, keys) in rows.iter().enumerate() {
        let orbit = r as u32;
        for &key in keys {
            let h = hash(key);
            let p = (h >> part_shift) as usize;
            hs[p].push(h);
            ws[p].push((orbit << 16) | (h as u32 & 0xffff));
        }
    }
    t[0] += t0.elapsed().as_secs_f64();
    let t1 = Instant::now();
    let out = place(&hs, &ws, bucket_shift, buckets, parts);
    t[1] += t1.elapsed().as_secs_f64();
    out
}

fn place(hs: &[Vec<u64>], ws: &[Vec<u32>], bucket_shift: u32, buckets: usize, parts: usize) -> Out {
    const FILL_AHEAD: usize = 16;
    let total: usize = hs.iter().map(|v| v.len()).sum();
    let filter_bits = filter_bits_for(total);
    let mask = (1u64 << filter_bits) - 1;
    let mut present = vec![0u64; (1usize << filter_bits) / 64];
    let per_part = buckets / parts;
    let mut bucket_start = vec![0u32; buckets + 1];
    let mut rests = vec![0u32; total];
    let mut next = 0usize;
    for p in 0..parts {
        let first = p * per_part;
        let mut at = vec![0usize; per_part];
        for &h in &hs[p] {
            at[(h >> bucket_shift) as usize - first] += 1;
        }
        for (b, slot) in at.iter_mut().enumerate() {
            bucket_start[first + b] = next as u32;
            let n = *slot;
            *slot = next;
            next += n;
        }
        let (h_, w_) = (&hs[p], &ws[p]);
        for (i, (&h, &w)) in h_.iter().zip(w_).enumerate() {
            if let Some(&ahead) = h_.get(i + FILL_AHEAD) {
                prefetch(&present[filter_slot(ahead, mask).0]);
            }
            let b = (h >> bucket_shift) as usize - first;
            rests[at[b]] = w;
            at[b] += 1;
            let (word, bits) = filter_slot(h, mask);
            present[word] |= bits;
        }
    }
    bucket_start[buckets] = total as u32;
    Out { bucket_start, rests, present }
}

// Variant: stream push through software write-combining buffers (one
// cache line of hashes and one of words per partition), flushed whole.
fn swwc(rows: &[Vec<u64>], bucket_bits: u32, part_bits_max: u32, t: &mut [f64; 2]) -> Out {
    let t0 = Instant::now();
    let bucket_shift = 64 - bucket_bits;
    let buckets = 1usize << bucket_bits;
    let part_bits = bucket_bits.min(part_bits_max);
    let parts = 1usize << part_bits;
    let part_shift = 64 - part_bits;
    let total_in: usize = rows.iter().map(|r| r.len()).sum();
    let mean = total_in / parts;
    let room = mean + 6 * (mean as f64).sqrt() as usize + 16;
    let mut hs: Vec<Vec<u64>> = (0..parts).map(|_| Vec::with_capacity(room)).collect();
    let mut ws: Vec<Vec<u32>> = (0..parts).map(|_| Vec::with_capacity(room)).collect();
    const L: usize = 8;
    let mut bh = vec![[0u64; L]; parts];
    let mut bw = vec![[0u32; L]; parts];
    let mut fill = vec![0u8; parts];
    for (r, keys) in rows.iter().enumerate() {
        let orbit = r as u32;
        for &key in keys {
            let h = hash(key);
            let p = (h >> part_shift) as usize;
            let f = fill[p] as usize;
            bh[p][f] = h;
            bw[p][f] = (orbit << 16) | (h as u32 & 0xffff);
            if f + 1 == L {
                hs[p].extend_from_slice(&bh[p]);
                ws[p].extend_from_slice(&bw[p]);
                fill[p] = 0;
            } else {
                fill[p] = (f + 1) as u8;
            }
        }
    }
    for p in 0..parts {
        let f = fill[p] as usize;
        hs[p].extend_from_slice(&bh[p][..f]);
        ws[p].extend_from_slice(&bw[p][..f]);
    }
    t[0] += t0.elapsed().as_secs_f64();
    let t1 = Instant::now();
    let out = place(&hs, &ws, bucket_shift, buckets, parts);
    t[1] += t1.elapsed().as_secs_f64();
    out
}

fn prefault(rows: &[Vec<u64>], bucket_bits: u32, part_bits_max: u32, t: &mut [f64; 2]) -> Out {
    let bucket_shift = 64 - bucket_bits;
    let buckets = 1usize << bucket_bits;
    let part_bits = bucket_bits.min(part_bits_max);
    let parts = 1usize << part_bits;
    let part_shift = 64 - part_bits;
    let total_in: usize = rows.iter().map(|r| r.len()).sum();
    let mean = total_in / parts;
    let room = mean + 6 * (mean as f64).sqrt() as usize + 16;
    let mut hs: Vec<Vec<u64>> = (0..parts).map(|_| { let mut v = vec![1u64; room]; v.clear(); v }).collect();
    let mut ws: Vec<Vec<u32>> = (0..parts).map(|_| { let mut v = vec![1u32; room]; v.clear(); v }).collect();
    let t0 = Instant::now();
    for (r, keys) in rows.iter().enumerate() {
        let orbit = r as u32;
        for &key in keys {
            let h = hash(key);
            let p = (h >> part_shift) as usize;
            hs[p].push(h);
            ws[p].push((orbit << 16) | (h as u32 & 0xffff));
        }
    }
    t[0] += t0.elapsed().as_secs_f64();
    // place with pre-faulted outputs
    let total: usize = hs.iter().map(|v| v.len()).sum();
    let filter_bits = filter_bits_for(total);
    let mut present = vec![1u64; (1usize << filter_bits) / 64];
    let mut bucket_start = vec![1u32; buckets + 1];
    let mut rests = vec![1u32; total];
    let t1 = Instant::now();
    present.iter_mut().for_each(|w| *w = 0);
    place_into(&hs, &ws, bucket_shift, buckets, parts, &mut bucket_start, &mut rests, &mut present);
    t[1] += t1.elapsed().as_secs_f64();
    Out { bucket_start, rests, present }
}

#[allow(clippy::too_many_arguments)]
fn place_into(hs: &[Vec<u64>], ws: &[Vec<u32>], bucket_shift: u32, buckets: usize, parts: usize,
              bucket_start: &mut [u32], rests: &mut [u32], present: &mut [u64]) {
    const FILL_AHEAD: usize = 16;
    let total = rests.len();
    let mask = (present.len() as u64 * 64) - 1;
    let per_part = buckets / parts;
    let mut next = 0usize;
    for p in 0..parts {
        let first = p * per_part;
        let mut at = vec![0usize; per_part];
        for &h in &hs[p] {
            at[(h >> bucket_shift) as usize - first] += 1;
        }
        for (b, slot) in at.iter_mut().enumerate() {
            bucket_start[first + b] = next as u32;
            let n = *slot;
            *slot = next;
            next += n;
        }
        let (h_, w_) = (&hs[p], &ws[p]);
        for (i, (&h, &w)) in h_.iter().zip(w_).enumerate() {
            if let Some(&ahead) = h_.get(i + FILL_AHEAD) {
                prefetch(&present[filter_slot(ahead, mask).0]);
            }
            let b = (h >> bucket_shift) as usize - first;
            rests[at[b]] = w;
            at[b] += 1;
            let (word, bits) = filter_slot(h, mask);
            present[word] |= bits;
        }
    }
    bucket_start[buckets] = total as u32;
}

fn flat(rows: &[Vec<u64>], bucket_bits: u32, part_bits_max: u32, t: &mut [f64; 2]) -> Out {
    let t0 = Instant::now();
    let bucket_shift = 64 - bucket_bits;
    let buckets = 1usize << bucket_bits;
    let part_bits = bucket_bits.min(part_bits_max);
    let parts = 1usize << part_bits;
    let part_shift = 64 - part_bits;
    let total_in: usize = rows.iter().map(|r| r.len()).sum();
    let mean = total_in / parts;
    let room = mean + 6 * (mean as f64).sqrt() as usize + 16;
    // Stride: whole cache lines of both kinds, an odd number of lines of
    // hashes, so the write heads spread over the cache's sets.
    let mut stride = room.div_ceil(16) * 16;
    if (stride / 8) % 2 == 0 { stride += 8; }
    if (stride / 16) % 2 == 0 { stride += 16; if (stride / 8) % 2 == 0 { stride += 8; } }
    let mut hs: Vec<u64> = Vec::with_capacity(stride * parts);
    let mut ws: Vec<u32> = Vec::with_capacity(stride * parts);
    unsafe { hs.set_len(stride * parts); ws.set_len(stride * parts); }
    let mut cur: Vec<usize> = (0..parts).map(|p| p * stride).collect();
    for (r, keys) in rows.iter().enumerate() {
        let orbit = r as u32;
        for &key in keys {
            let h = hash(key);
            let p = (h >> part_shift) as usize;
            let c = cur[p];
            assert!(c < (p + 1) * stride);
            unsafe {
                *hs.get_unchecked_mut(c) = h;
                *ws.get_unchecked_mut(c) = (orbit << 16) | (h as u32 & 0xffff);
            }
            cur[p] = c + 1;
        }
    }
    t[0] += t0.elapsed().as_secs_f64();
    let t1 = Instant::now();
    let hv: Vec<&[u64]> = (0..parts).map(|p| &hs[p * stride..cur[p]]).collect();
    let wv: Vec<&[u32]> = (0..parts).map(|p| &ws[p * stride..cur[p]]).collect();
    let out = place_slices(&hv, &wv, bucket_shift, buckets, parts);
    t[1] += t1.elapsed().as_secs_f64();
    out
}

fn place_slices(hs: &[&[u64]], ws: &[&[u32]], bucket_shift: u32, buckets: usize, parts: usize) -> Out {
    const FILL_AHEAD: usize = 16;
    let total: usize = hs.iter().map(|v| v.len()).sum();
    let filter_bits = filter_bits_for(total);
    let mask = (1u64 << filter_bits) - 1;
    let mut present = vec![0u64; (1usize << filter_bits) / 64];
    let per_part = buckets / parts;
    let mut bucket_start = vec![0u32; buckets + 1];
    let mut rests = vec![0u32; total];
    let mut next = 0usize;
    let mut at = vec![0usize; per_part];
    for p in 0..parts {
        let first = p * per_part;
        at.iter_mut().for_each(|a| *a = 0);
        for &h in hs[p] {
            at[(h >> bucket_shift) as usize - first] += 1;
        }
        for (b, slot) in at.iter_mut().enumerate() {
            bucket_start[first + b] = next as u32;
            let n = *slot;
            *slot = next;
            next += n;
        }
        let (h_, w_) = (hs[p], ws[p]);
        for (i, (&h, &w)) in h_.iter().zip(w_).enumerate() {
            if let Some(&ahead) = h_.get(i + FILL_AHEAD) {
                prefetch(&present[filter_slot(ahead, mask).0]);
            }
            let b = (h >> bucket_shift) as usize - first;
            rests[at[b]] = w;
            at[b] += 1;
            let (word, bits) = filter_slot(h, mask);
            present[word] |= bits;
        }
    }
    bucket_start[buckets] = total as u32;
    Out { bucket_start, rests, present }
}

#[repr(C, align(64))]
struct Buf { h: [u64; 16], w: [u32; 16] }

fn ntswwc(rows: &[Vec<u64>], bucket_bits: u32, part_bits_max: u32, t: &mut [f64; 2]) -> Out {
    use std::arch::x86_64::*;
    let t0 = Instant::now();
    let bucket_shift = 64 - bucket_bits;
    let buckets = 1usize << bucket_bits;
    let part_bits = bucket_bits.min(part_bits_max);
    let parts = 1usize << part_bits;
    let part_shift = 64 - part_bits;
    let total_in: usize = rows.iter().map(|r| r.len()).sum();
    let mean = total_in / parts;
    let room = mean + 6 * (mean as f64).sqrt() as usize + 16;
    let mut stride = room.div_ceil(16) * 16 + 16;
    if (stride / 16) % 2 == 0 { stride += 16; }
    // 64-byte aligned backing for both kinds.
    let lay_h = std::alloc::Layout::from_size_align(stride * parts * 8, 64).unwrap();
    let lay_w = std::alloc::Layout::from_size_align(stride * parts * 4, 64).unwrap();
    let hp = unsafe { std::alloc::alloc(lay_h) as *mut u64 };
    let wp = unsafe { std::alloc::alloc(lay_w) as *mut u32 };
    let mut bufs: Vec<Buf> = (0..parts).map(|_| Buf { h: [0; 16], w: [0; 16] }).collect();
    let mut fill = vec![0u32; parts];
    let mut cur: Vec<usize> = (0..parts).map(|p| p * stride).collect();
    for (r, keys) in rows.iter().enumerate() {
        let orbit = r as u32;
        for &key in keys {
            let h = hash(key);
            let p = (h >> part_shift) as usize;
            let f = fill[p] as usize;
            let b = unsafe { bufs.get_unchecked_mut(p) };
            b.h[f] = h;
            b.w[f] = (orbit << 16) | (h as u32 & 0xffff);
            if f == 15 {
                let c = cur[p];
                assert!(c + 16 <= (p + 1) * stride);
                unsafe {
                    let src = b.h.as_ptr() as *const __m512i;
                    _mm512_stream_si512(hp.add(c) as *mut __m512i, _mm512_load_si512(src));
                    _mm512_stream_si512(hp.add(c + 8) as *mut __m512i, _mm512_load_si512(src.add(1)));
                    _mm512_stream_si512(wp.add(c) as *mut __m512i, _mm512_load_si512(b.w.as_ptr() as *const __m512i));
                }
                cur[p] = c + 16;
                fill[p] = 0;
            } else {
                fill[p] = (f + 1) as u32;
            }
        }
    }
    unsafe { _mm_sfence(); }
    for p in 0..parts {
        let f = fill[p] as usize;
        let c = cur[p];
        unsafe {
            std::ptr::copy_nonoverlapping(bufs[p].h.as_ptr(), hp.add(c), f);
            std::ptr::copy_nonoverlapping(bufs[p].w.as_ptr(), wp.add(c), f);
        }
        cur[p] = c + f;
    }
    t[0] += t0.elapsed().as_secs_f64();
    let t1 = Instant::now();
    let hv: Vec<&[u64]> = (0..parts).map(|p| unsafe { std::slice::from_raw_parts(hp.add(p * stride), cur[p] - p * stride) }).collect();
    let wv: Vec<&[u32]> = (0..parts).map(|p| unsafe { std::slice::from_raw_parts(wp.add(p * stride), cur[p] - p * stride) }).collect();
    let out = place_slices(&hv, &wv, bucket_shift, buckets, parts);
    t[1] += t1.elapsed().as_secs_f64();
    unsafe { std::alloc::dealloc(hp as *mut u8, lay_h); std::alloc::dealloc(wp as *mut u8, lay_w); }
    out
}

// Filter word from the hash's top bits (partition-local), bits in the word
// from bits independent of the index: the layout change under test.
#[inline(always)]
fn filter_slot_top(h: u64, word_bits: u32) -> (usize, u64) {
    let more = (1u64 << ((h >> 6) & 63)) | (1u64 << ((h >> 12) & 63));
    ((h >> (64 - word_bits)) as usize, (1u64 << (h & 63)) | more)
}

fn topfilter(rows: &[Vec<u64>], bucket_bits: u32, part_bits_max: u32, t: &mut [f64; 2]) -> Out {
    let t0 = Instant::now();
    let bucket_shift = 64 - bucket_bits;
    let buckets = 1usize << bucket_bits;
    let part_bits = bucket_bits.min(part_bits_max);
    let parts = 1usize << part_bits;
    let part_shift = 64 - part_bits;
    let total_in: usize = rows.iter().map(|r| r.len()).sum();
    let mean = total_in / parts;
    let room = mean + 6 * (mean as f64).sqrt() as usize + 16;
    let mut hs: Vec<Vec<u64>> = (0..parts).map(|_| Vec::with_capacity(room)).collect();
    let mut ws: Vec<Vec<u32>> = (0..parts).map(|_| Vec::with_capacity(room)).collect();
    for (r, keys) in rows.iter().enumerate() {
        let orbit = r as u32;
        for &key in keys {
            let h = hash(key);
            let p = (h >> part_shift) as usize;
            hs[p].push(h);
            ws[p].push((orbit << 16) | (h as u32 & 0xffff));
        }
    }
    t[0] += t0.elapsed().as_secs_f64();
    let t1 = Instant::now();
    let total: usize = hs.iter().map(|v| v.len()).sum();
    let filter_bits = filter_bits_for(total);
    let word_bits = filter_bits - 6;
    let mut present = vec![0u64; 1usize << word_bits];
    let per_part = buckets / parts;
    let mut bucket_start = vec![0u32; buckets + 1];
    let mut rests = vec![0u32; total];
    let mut next = 0usize;
    for p in 0..parts {
        let first = p * per_part;
        let mut at = vec![0usize; per_part];
        for &h in &hs[p] {
            at[(h >> bucket_shift) as usize - first] += 1;
        }
        for (b, slot) in at.iter_mut().enumerate() {
            bucket_start[first + b] = next as u32;
            let n = *slot;
            *slot = next;
            next += n;
        }
        for (&h, &w) in hs[p].iter().zip(&ws[p]) {
            let b = (h >> bucket_shift) as usize - first;
            rests[at[b]] = w;
            at[b] += 1;
            let (word, bits) = filter_slot_top(h, word_bits);
            present[word] |= bits;
        }
    }
    bucket_start[buckets] = total as u32;
    t[1] += t1.elapsed().as_secs_f64();
    // A different filter by design: return the reference's so the harness
    // compares rests and buckets only.
    Out { bucket_start, rests, present: Vec::new() }
}

// One packed stream: the hash with bits 34..37 and 56..63 replaced by the
// row's orbit (< 4096); the partition restores bits 56..63.
fn packed(rows: &[Vec<u64>], bucket_bits: u32, _pb: u32, t: &mut [f64; 2]) -> Out {
    let t0 = Instant::now();
    let bucket_shift = 64 - bucket_bits;
    let buckets = 1usize << bucket_bits;
    let parts = 256usize;
    let total_in: usize = rows.iter().map(|r| r.len()).sum();
    let mean = total_in / parts;
    let room = mean + 6 * (mean as f64).sqrt() as usize + 16;
    let mut vs: Vec<Vec<u64>> = (0..parts).map(|_| Vec::with_capacity(room)).collect();
    const KEEP: u64 = !((0xF << 34) | (0xFF << 56));
    for (r, keys) in rows.iter().enumerate() {
        let orbit = (r as u64) & 0xFFF;
        let tag = ((orbit & 0xF) << 34) | ((orbit >> 4) << 56);
        for &key in keys {
            let h = hash(key);
            vs[(h >> 56) as usize].push((h & KEEP) | tag);
        }
    }
    t[0] += t0.elapsed().as_secs_f64();
    let t1 = Instant::now();
    let total: usize = vs.iter().map(|v| v.len()).sum();
    let filter_bits = filter_bits_for(total);
    let word_bits = filter_bits - 6;
    let mut present = vec![0u64; 1usize << word_bits];
    let per_part = buckets / parts;
    let mut bucket_start = vec![0u32; buckets + 1];
    let mut rests = vec![0u32; total];
    let mut next = 0usize;
    let mut at = vec![0usize; per_part];
    for p in 0..parts {
        let first = p * per_part;
        let top = (p as u64) << 56;
        at.iter_mut().for_each(|a| *a = 0);
        for &v in &vs[p] {
            let h = (v & KEEP) | top;
            at[(h >> bucket_shift) as usize - first] += 1;
        }
        for (b, slot) in at.iter_mut().enumerate() {
            bucket_start[first + b] = next as u32;
            let n = *slot;
            *slot = next;
            next += n;
        }
        for &v in &vs[p] {
            let h = (v & KEEP) | top;
            let orbit = ((v >> 34) & 0xF) | ((v >> 56) << 4);
            let b = (h >> bucket_shift) as usize - first;
            rests[at[b]] = ((orbit as u32) << 16) | (h as u32 & 0xffff);
            at[b] += 1;
            let (word, bits) = filter_slot_top(h, word_bits);
            present[word] |= bits;
        }
    }
    bucket_start[buckets] = total as u32;
    t[1] += t1.elapsed().as_secs_f64();
    Out { bucket_start, rests, present: Vec::new() }
}

fn main() {
    let total: usize = std::env::args().nth(1).map_or(3_707_880, |v| v.parse().unwrap());
    let row_len = 3000usize;
    let mut s = 0x9e3779b97f4a7c15u64;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut rows = Vec::new();
    let mut left = total;
    while left > 0 {
        let n = left.min(row_len);
        rows.push((0..n).map(|_| rnd() >> 10).collect::<Vec<u64>>());
        left -= n;
    }
    let pairs = total as f64;
    let bucket_bits = (usize::BITS - (total / 16).leading_zeros()).min(40);
    println!("pairs {total} buckets 2^{bucket_bits}");
    let reference = baseline(&rows, bucket_bits, 8, &mut [0.0; 2]);
    for (name, f, pb) in [
        ("baseline256", baseline as fn(&[Vec<u64>], u32, u32, &mut [f64; 2]) -> Out, 8u32),
        ("baseline128", baseline, 7),
        ("baseline64", baseline, 6),
        ("swwc256", swwc, 8),
        ("swwc128", swwc, 7),
        ("prefault256", prefault, 8),
        ("flat256", flat, 8),
        ("flat128", flat, 7),
        ("ntswwc256", ntswwc, 8),
        ("ntswwc128", ntswwc, 7),
        ("topfilter256", topfilter, 8),
        ("packed256", packed, 8),
    ] {
        let mut t = [0.0; 2];
        let reps = 7;
        let mut best = [f64::MAX; 2];
        for _ in 0..reps {
            let mut tt = [0.0; 2];
            let o = f(&rows, bucket_bits, pb, &mut tt);
            if pb == 8 {
                assert_eq!(o.rests, reference.rests);
                assert_eq!(o.bucket_start, reference.bucket_start);
            }
            if !o.present.is_empty() { assert_eq!(o.present, reference.present); }
            for k in 0..2 {
                best[k] = best[k].min(tt[k]);
                t[k] += tt[k];
            }
        }
        println!(
            "{name:12} push {:6.2} ns/pair  place {:6.2} ns/pair  (best of {reps})",
            best[0] * 1e9 / pairs,
            best[1] * 1e9 / pairs
        );
    }
}
