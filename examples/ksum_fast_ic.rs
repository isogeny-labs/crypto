//! The generic-track incumbent's index calculus (`ic.pipeline` with
//! `ksum:m=5` on a `prime-abscissa` base) re-implemented on batched
//! Montgomery arithmetic, end to end, so its wall time can be compared with
//! the library pipeline on the same curve, targets and base.
//!
//! The method is the same:
//!
//! 1. **Table.** Every signed 5-sum `P_{i1} ± … ± P_{i5}` (`i1 < … < i5`,
//!    first sign `+`, the negation fold) of the base, keyed by its abscissa.
//!    Built level by level with one field inversion per level batch
//!    (Montgomery's trick); the last level is abscissa-only and forms both
//!    signs `x(S ± P_k)` from one denominator.
//! 2. **Relations.** Many r-adding walks `R ← R + W_j` (`W_j = c_j G + d_j Q`)
//!    run in lockstep with one inversion per step across all lanes; every
//!    walk point's abscissa is probed in the table (one-multiply word Bloom
//!    filter, then an exact bucketed lookup). A hit is re-added in the
//!    library's group law and kept only if `±(a G + b Q)` equals the 5-sum,
//!    which fixes the sign of the relation.
//! 3. **Linear algebra.** A left-kernel vector `λ` of the relation matrix
//!    over `F_r` gives `x = −Σλa / Σλb`; the solve is accepted only if
//!    `x G = Q` in the library's arithmetic.
//!
//! Correctness is checked, never assumed: every relation is verified in the
//! library, and every recovered logarithm reproduces its target. Prototype;
//! `[pcc:unverified]`, no isolation.
//!
//!     ksum_fast_ic --targets FILE [--bits 32] [--seed 20261018202]
//!                  [--size 19] [--lanes 256] [--threads 1] [--reps 1]

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    find_prime_order_curve, CountedGroup, GroupOps, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
use crypto_lib::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
use serde_json::json;

/// Montgomery arithmetic for an odd modulus below 2^32, `R = 2^32`.
#[derive(Clone, Copy)]
struct Mont {
    p: u64,
    ninv: u64,
    r2: u64,
    one: u64,
}

impl Mont {
    fn new(p: u64) -> Self {
        assert!(p < (1 << 32) && p & 1 == 1);
        let mut inv = 1u32;
        for _ in 0..5 {
            inv = inv.wrapping_mul(2u32.wrapping_sub((p as u32).wrapping_mul(inv)));
        }
        let r = (1u128 << 32) % p as u128;
        let mut m = Self {
            p,
            ninv: u64::from(inv.wrapping_neg()),
            r2: ((r * r) % p as u128) as u64,
            one: 0,
        };
        m.one = m.to(1);
        m
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        const LO: u64 = 0xFFFF_FFFF;
        let t = (a & LO) * (b & LO);
        let m = ((t & LO) * (self.ninv & LO)) & LO;
        let u = (t >> 32) + ((m * (self.p & LO)) >> 32) + u64::from(t & LO != 0);
        if u >= self.p {
            u - self.p
        } else {
            u
        }
    }
    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        let s = a + self.p - b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline(always)]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn to(&self, a: u64) -> u64 {
        self.mul(a % self.p, self.r2)
    }
    fn from(&self, a: u64) -> u64 {
        self.mul(a, 1)
    }
    fn inv(&self, a: u64) -> u64 {
        let (mut base, mut e, mut acc) = (a, self.p - 2, self.one);
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(acc, base);
            }
            base = self.mul(base, base);
            e >>= 1;
        }
        acc
    }
}

/// One-multiply word-blocked Bloom filter (8 bits a key, 4 bits a word).
/// The word is chosen by the key's top bits (keys are uniform field
/// elements), so a key range owns a word range and the filter can be
/// filled in parallel by key bucket; the four bits come from one product.
struct Bloom {
    words: Vec<u64>,
    mask: usize,
    /// `key >> wshift` is the word index.
    wshift: u32,
}

impl Bloom {
    fn new(keys: usize, key_bits: u32) -> Self {
        let words = (keys / 8).next_power_of_two().max(256);
        let wbits = words.trailing_zeros();
        Self {
            words: vec![0; words],
            mask: words - 1,
            wshift: key_bits.saturating_sub(wbits),
        }
    }
    #[inline(always)]
    fn slot(&self, key: u64) -> (usize, u64) {
        let h = key.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let bits = (1u64 << (h & 63))
            | (1u64 << ((h >> 6) & 63))
            | (1u64 << ((h >> 12) & 63))
            | (1u64 << ((h >> 18) & 63));
        ((key >> self.wshift) as usize & self.mask, bits)
    }
    fn insert(&mut self, key: u64) {
        let (w, b) = self.slot(key);
        self.words[w] |= b;
    }
    #[inline(always)]
    fn maybe(&self, key: u64) -> bool {
        let (w, b) = self.slot(key);
        self.words[w] & b == b
    }
}

/// Where a table key's witness comes from: a dense list of packed tuples,
/// or the last build level's pair arrays (witness rebuilt on a hit).
enum Source {
    Packed(Vec<u64>),
    /// Pair `t` of the last level is entry `e` (`off[e] <= t < off[e+1]`)
    /// plus base point `llast[e] + 1 + (t − off[e])`.
    Implicit {
        off: Vec<u32>,
        llast: Vec<u32>,
        lpk: Vec<u64>,
        depth: u32,
    },
}

/// Keys bucketed by their top 16 bits (one counting pass and one
/// scatter, 8-byte entries: the key and a reference), behind the Bloom
/// filter. Keys inside a bucket keep insert order, so a lookup returns the
/// first witness, as the library table does.
struct Table {
    keys: Vec<u32>,
    refs: Vec<u32>,
    dir: Vec<u32>,
    shift: u32,
    len: usize,
    collisions: usize,
    bloom: Bloom,
    src: Source,
}

impl Table {
    fn from_iter<I: Fn(&mut dyn FnMut(u32, u32))>(n: usize, p: u64, items: I, src: Source) -> Self {
        let bits = 64 - (p - 1).leading_zeros();
        let shift = bits.saturating_sub(16);
        let nb = 1usize << 16;
        let mut dir = vec![0u32; nb + 1];
        let mut bloom = Bloom::new(n, 64 - (p - 1).leading_zeros());
        items(&mut |k, _| {
            dir[((k >> shift) as usize).min(nb - 1) + 1] += 1;
            bloom.insert(u64::from(k));
        });
        for i in 0..nb {
            dir[i + 1] += dir[i];
        }
        let total = dir[nb] as usize;
        let mut fill: Vec<u32> = dir[..nb].to_vec();
        let mut keys = vec![0u32; total];
        let mut refs = vec![0u32; total];
        items(&mut |k, r| {
            let b = ((k >> shift) as usize).min(nb - 1);
            let at = fill[b] as usize;
            keys[at] = k;
            refs[at] = r;
            fill[b] += 1;
        });
        let mut collisions = 0;
        for b in 0..nb {
            let (lo, hi) = (dir[b] as usize, dir[b + 1] as usize);
            for i in lo + 1..hi {
                if keys[lo..i].contains(&keys[i]) {
                    collisions += 1;
                }
            }
        }
        Self {
            keys,
            refs,
            dir,
            shift,
            len: total - collisions,
            collisions,
            bloom,
            src,
        }
    }
    fn build(kv: Vec<(u32, u64)>, p: u64) -> Self {
        let n = kv.len();
        let vals: Vec<u64> = kv.iter().map(|e| e.1).collect();
        let keys: Vec<u32> = kv.iter().map(|e| e.0).collect();
        Self::from_iter(
            n,
            p,
            |emit: &mut dyn FnMut(u32, u32)| {
                for (i, &k) in keys.iter().enumerate() {
                    emit(k, i as u32);
                }
            },
            Source::Packed(vals),
        )
    }
    /// Duplicate keys (the build's collisions), counted on demand.
    fn count_collisions(&self) -> usize {
        let mut c = 0;
        for b in 0..self.dir.len() - 1 {
            let (lo, hi) = (self.dir[b] as usize, self.dir[b + 1] as usize);
            for i in lo + 1..hi {
                if self.keys[lo..i].contains(&self.keys[i]) {
                    c += 1;
                }
            }
        }
        c
    }
    #[inline(always)]
    fn resolve(&self, r: u32) -> u64 {
        match &self.src {
            Source::Packed(v) => v[r as usize],
            Source::Implicit {
                off,
                llast,
                lpk,
                depth,
            } => {
                let t = r / 2;
                let e = off.partition_point(|&o| o <= t) - 1;
                let k = llast[e] + 1 + (t - off[e]);
                pack_push(lpk[e], *depth, k as usize, r & 1 == 1)
            }
        }
    }
    #[inline(always)]
    fn get(&self, key: u64) -> Option<u64> {
        if !self.bloom.maybe(key) {
            return None;
        }
        self.get_exact(key)
    }
    /// The exact lookup alone (for keys that passed the filter).
    #[inline(always)]
    fn get_exact(&self, key: u64) -> Option<u64> {
        let k = key as u32;
        let b = ((k >> self.shift) as usize).min(self.dir.len() - 2);
        let (lo, hi) = (self.dir[b] as usize, self.dir[b + 1] as usize);
        self.keys[lo..hi]
            .iter()
            .position(|&q| q == k)
            .map(|i| self.resolve(self.refs[lo + i]))
    }
}

/// The last build level in SoA form (`n` live-or-flagged pairs).
/// Packed signed index tuple: per summand 10 bits of index and 1 sign bit.
const IDX_BITS: u32 = 11;

fn pack_push(packed: u64, depth: u32, idx: usize, neg: bool) -> u64 {
    packed | ((idx as u64 | (u64::from(neg) << 10)) << (IDX_BITS * depth))
}

fn unpack(packed: u64, m: u32) -> Vec<(usize, bool)> {
    (0..m)
        .map(|d| {
            let f = (packed >> (IDX_BITS * d)) & ((1 << IDX_BITS) - 1);
            ((f & 1023) as usize, f >> 10 == 1)
        })
        .collect()
}

/// One level entry: a signed partial sum in Montgomery affine form.
#[derive(Clone, Copy)]
struct Partial {
    x: u64,
    y: u64,
    last: usize,
    packed: u64,
}

/// The table of every signed m-sum (first sign +), built with batched
/// inversions; returns the key-value list and the number of field
/// inversions and exceptional (equal-abscissa) pairs skipped.
fn build_ksum(f: &Mont, bx: &[u64], by: &[u64], m: u32) -> (Vec<(u32, u64)>, u64, u64) {
    let a = bx.len();
    let mut level: Vec<Partial> = (0..a)
        .map(|i| Partial {
            x: bx[i],
            y: by[i],
            last: i,
            packed: pack_push(0, 0, i, false),
        })
        .collect();
    let mut invs = 0u64;
    let mut skipped = 0u64;
    let trace = std::env::var_os("KSUM_TRACE").is_some();
    for depth in 1..m {
        let t_level = Instant::now();
        let last_level = depth == m - 1;
        // Every (entry, k > last) pair; both signs share the denominator.
        let mut pairs: Vec<(u32, u32)> = Vec::new();
        for (e, p) in level.iter().enumerate() {
            for k in p.last + 1..a {
                pairs.push((e as u32, k as u32));
            }
        }
        let n = pairs.len();
        let mut d = vec![0u64; n];
        let mut pre = vec![0u64; n];
        // Sixteen interleaved product chains (pair t in chain t % 16).
        const CH: usize = 16;
        let mut acc = [f.one; CH];
        for (t, &(e, k)) in pairs.iter().enumerate() {
            let dd = f.sub(bx[k as usize], level[e as usize].x);
            d[t] = dd;
            pre[t] = acc[t % CH];
            if dd != 0 {
                acc[t % CH] = f.mul(acc[t % CH], dd);
            }
        }
        let mut lp = [0u64; CH];
        let mut all = f.one;
        for c in 0..CH {
            lp[c] = all;
            all = f.mul(all, acc[c]);
        }
        let mut ai = f.inv(all);
        let mut invc = [0u64; CH];
        for c in (0..CH).rev() {
            invc[c] = f.mul(ai, lp[c]);
            ai = f.mul(ai, acc[c]);
        }
        invs += 1;
        let mut next_full: Vec<Partial> = Vec::with_capacity(if last_level { 0 } else { 2 * n });
        let mut out: Vec<(u32, u64)> = Vec::with_capacity(if last_level { 2 * n } else { 0 });
        for t in (0..n).rev() {
            let (e, k) = pairs[t];
            let (e, k) = (e as usize, k as usize);
            let dd = d[t];
            if dd == 0 {
                skipped += 1;
                continue;
            }
            let dinv = f.mul(invc[t % CH], pre[t]);
            invc[t % CH] = f.mul(invc[t % CH], dd);
            let s = level[e];
            let base = f.add(s.x, bx[k]);
            for neg in [false, true] {
                let yk = if neg { f.sub(0, by[k]) } else { by[k] };
                let lam = f.mul(f.sub(yk, s.y), dinv);
                let x3 = f.sub(f.mul(lam, lam), base);
                let packed = pack_push(s.packed, depth, k, neg);
                if last_level {
                    out.push((x3 as u32, packed));
                } else {
                    let y3 = f.sub(f.mul(lam, f.sub(s.x, x3)), s.y);
                    next_full.push(Partial {
                        x: x3,
                        y: y3,
                        last: k,
                        packed,
                    });
                }
            }
        }
        if trace {
            eprintln!(
                "level {} pairs {} {:.3} ms",
                depth + 1,
                n,
                t_level.elapsed().as_secs_f64() * 1e3
            );
        }
        if last_level {
            return (out, invs, skipped);
        }
        level = next_full;
    }
    unreachable!("m >= 2")
}

/// A spin-waiting worker pool: `threads − 1` persistent workers poll an
/// epoch counter, so a parallel phase starts in well under a microsecond
/// (a rayon or futex wake-up costs 75–150 µs on this host). Tasks are
/// claimed with an atomic index; the caller works too and returns only
/// when every task has finished, which is what makes lending the borrowed
/// closure to the workers sound.
struct SpinPool {
    epoch: std::sync::atomic::AtomicU64,
    next: std::sync::atomic::AtomicUsize,
    done: std::sync::atomic::AtomicUsize,
    tasks: std::sync::atomic::AtomicUsize,
    job: std::sync::atomic::AtomicPtr<()>,
    workers: usize,
}

type Job<'a> = &'a (dyn Fn(usize) + Sync);

static POOL: std::sync::OnceLock<&'static SpinPool> = std::sync::OnceLock::new();

impl SpinPool {
    fn start(threads: usize) {
        use std::sync::atomic::*;
        let pool: &'static SpinPool = Box::leak(Box::new(SpinPool {
            epoch: AtomicU64::new(0),
            next: AtomicUsize::new(0),
            done: AtomicUsize::new(0),
            tasks: AtomicUsize::new(0),
            job: AtomicPtr::new(std::ptr::null_mut()),
            workers: threads.saturating_sub(1),
        }));
        for _ in 0..pool.workers {
            std::thread::spawn(move || {
                let mut seen = 0u64;
                loop {
                    let e = pool.epoch.load(Ordering::Acquire);
                    if e == seen {
                        std::hint::spin_loop();
                        continue;
                    }
                    seen = e;
                    pool.work();
                }
            });
        }
        POOL.set(pool).ok().expect("one pool");
    }
    fn work(&self) {
        use std::sync::atomic::Ordering;
        let ptr = self.job.load(Ordering::Acquire) as *const Job<'static>;
        if ptr.is_null() {
            return;
        }
        // SAFETY: `run` keeps the closure alive until `done == tasks`.
        let job: Job<'static> = unsafe { *ptr };
        let n = self.tasks.load(Ordering::Acquire);
        loop {
            let i = self.next.fetch_add(1, Ordering::AcqRel);
            if i >= n {
                break;
            }
            job(i);
            self.done.fetch_add(1, Ordering::AcqRel);
        }
    }
    fn run(&self, tasks: usize, job: Job<'_>) {
        use std::sync::atomic::Ordering;
        // SAFETY: the reference outlives every use: this call does not
        // return before all `tasks` have run to completion.
        let job: Job<'static> = unsafe { std::mem::transmute(job) };
        let holder: Job<'static> = job;
        self.tasks.store(tasks, Ordering::Release);
        self.next.store(0, Ordering::Release);
        self.done.store(0, Ordering::Release);
        self.job
            .store(&holder as *const Job<'static> as *mut (), Ordering::Release);
        self.epoch.fetch_add(1, Ordering::AcqRel);
        self.work();
        while self.done.load(Ordering::Acquire) < tasks {
            std::hint::spin_loop();
        }
        self.job.store(std::ptr::null_mut(), Ordering::Release);
    }
}

/// Run closures on the spin pool and collect their results in order.
fn par_run<T: Send, F: FnOnce() -> T + Send>(jobs: Vec<F>) -> Vec<T> {
    let n = jobs.len();
    let slots: Vec<Mutex<Option<F>>> = jobs.into_iter().map(|j| Mutex::new(Some(j))).collect();
    let out: Vec<Mutex<Option<T>>> = (0..n).map(|_| Mutex::new(None)).collect();
    let job = |i: usize| {
        let f = slots[i].lock().unwrap().take().expect("each task once");
        *out[i].lock().unwrap() = Some(f());
    };
    POOL.get().expect("pool started").run(n, &job);
    out.into_iter()
        .map(|m| m.into_inner().unwrap().expect("ran"))
        .collect()
}

/// A raw pointer shared by threads that write disjoint indices.
#[derive(Clone, Copy)]
struct SyncPtr<T>(*mut T);
unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}

/// The last level and the table, in parallel: (1) each thread forms the
/// keys of its pair range with the AVX-512 kernel and counts them by top-8
/// key bucket; (2) every thread scatters its keys to its own offsets of
/// each bucket; (3) each thread finishes a contiguous range of buckets
/// (second-level scatter, directory, and the Bloom words those keys own).
/// Witnesses are implicit in the pair index. Same table as the scalar
/// builder (checked by `KSUM_CHECKBUILD`).
#[cfg(target_arch = "x86_64")]
#[allow(clippy::too_many_arguments)]
fn last_level_par(
    f: &Mont,
    lx: &[u64],
    ly: &[u64],
    llast: Vec<u32>,
    lpk: Vec<u64>,
    bx: &[u64],
    by: &[u64],
    depth: u32,
    threads: usize,
    p: u64,
) -> (Table, u64, u64) {
    const CH: usize = 4096;
    let tr0 = Instant::now();
    let trace = std::env::var_os("KSUM_TRACE").is_some();
    let a = bx.len();
    let ne = lx.len();
    let mut off = vec![0u32; ne + 1];
    for e in 0..ne {
        off[e + 1] = off[e] + (a - 1 - llast[e] as usize) as u32;
    }
    let n = off[ne] as usize;
    let bits = 64 - (p - 1).leading_zeros();
    assert!(bits >= 16);
    // Directory bits sized to the table: about 4 keys a bucket, 10 to 16 bits.
    let dbits = (((2 * n) as f64 / 4.0).log2().ceil() as u32).clamp(10, 16);
    let shift = bits - dbits;
    let sub = 1usize << (dbits - 8);
    let top = bits - 8;
    let nt = threads.max(1);
    let per = (n.div_ceil(nt)).div_ceil(avx::GROUP) * avx::GROUP;
    let dummy_k = ((llast[0] as usize + 1) % a) as u64;
    // Phase 1.
    type Part = (Vec<u32>, Vec<u32>, [usize; 256], u64, u64, usize);
    let parts: Vec<Part> = {
        let jobs: Vec<_> = (0..nt)
            .map(|th| {
                let (off, llast) = (&off, &llast);
                move || {
                    let t0 = (th * per).min(n);
                    let t1 = ((th + 1) * per).min(n);
                    let len = t1 - t0;
                    let mut kp = vec![u32::MAX; len];
                    let mut km = vec![u32::MAX; len];
                    let mut hist = [0usize; 256];
                    let (mut invs, mut skipped) = (0u64, 0u64);
                    let mut pe = vec![0u64; CH];
                    let mut pk = vec![dummy_k; CH];
                    let (mut dd, mut pre) = (vec![0u64; CH], vec![0u64; CH]);
                    let mut zm = vec![0u8; CH / 8];
                    let (mut xp, mut xm) = (vec![0u64; CH], vec![0u64; CH]);
                    let (mut yp, mut ym) = (Vec::new(), Vec::new());
                    let mut e = if len > 0 {
                        off.partition_point(|&o| o as usize <= t0) - 1
                    } else {
                        0
                    };
                    let mut c0 = t0;
                    while c0 < t1 {
                        let cl = CH.min(t1 - c0);
                        let clp = cl.div_ceil(avx::GROUP) * avx::GROUP;
                        for i in 0..clp {
                            if i < cl {
                                let t = c0 + i;
                                while off[e + 1] as usize <= t {
                                    e += 1;
                                }
                                pe[i] = e as u64;
                                pk[i] = (llast[e] as usize + 1 + (t - off[e] as usize)) as u64;
                            } else {
                                pe[i] = 0;
                                pk[i] = dummy_k;
                            }
                        }
                        // SAFETY: callers use this only when AVX-512 was detected.
                        unsafe {
                            avx::pair_add(
                                f,
                                lx,
                                ly,
                                bx,
                                by,
                                &pe[..clp],
                                &pk[..clp],
                                &mut dd[..clp],
                                &mut pre[..clp],
                                &mut zm[..clp / 8],
                                &mut xp[..clp],
                                &mut xm[..clp],
                                &mut yp,
                                &mut ym,
                                false,
                            )
                        };
                        invs += 1;
                        for i in 0..cl {
                            if zm[i / 8] >> (i % 8) & 1 == 1 {
                                skipped += 1;
                                continue;
                            }
                            let (u, v) = (xp[i] as u32, xm[i] as u32);
                            kp[c0 - t0 + i] = u;
                            km[c0 - t0 + i] = v;
                            hist[(u >> top) as usize] += 1;
                            hist[(v >> top) as usize] += 1;
                        }
                        c0 += cl;
                    }
                    (kp, km, hist, invs, skipped, t0)
                }
            })
            .collect();
        par_run(jobs)
    };
    if trace {
        eprintln!(
            "    phase 1 (keys) {:.3} ms",
            tr0.elapsed().as_secs_f64() * 1e3
        );
    }
    let (mut invs, mut skipped) = (0u64, 0u64);
    // Phase 2: offsets per (bucket, thread), then parallel scatter.
    let mut c8 = vec![0usize; 257];
    let mut start = vec![[0usize; 256]; nt];
    {
        let mut run = 0;
        for b in 0..256 {
            c8[b] = run;
            for (th, part) in parts.iter().enumerate() {
                start[th][b] = run;
                run += part.2[b];
            }
        }
        c8[256] = run;
    }
    for part in &parts {
        invs += part.3;
        skipped += part.4;
    }
    let total = c8[256];
    let mut tk = vec![0u32; total];
    let mut tr = vec![0u32; total];
    let (tkp, trp) = (SyncPtr(tk.as_mut_ptr()), SyncPtr(tr.as_mut_ptr()));
    {
        let mut jobs = Vec::new();
        for (th, part) in parts.iter().enumerate() {
            let mut fill = start[th];
            jobs.push(move || {
                let (tkp, trp) = (tkp, trp);
                let (kp, km, _, _, _, t0) = part;
                for i in 0..kp.len() {
                    if kp[i] == u32::MAX {
                        continue;
                    }
                    let t = (t0 + i) as u32;
                    for (k, r) in [(kp[i], 2 * t), (km[i], 2 * t + 1)] {
                        let b = (k >> top) as usize;
                        // SAFETY: each (bucket, thread) owns a disjoint range.
                        unsafe {
                            *tkp.0.add(fill[b]) = k;
                            *trp.0.add(fill[b]) = r;
                        }
                        fill[b] += 1;
                    }
                }
            });
        }
        par_run(jobs);
    }
    drop(parts);
    if trace {
        eprintln!(
            "    phase 2 (scatter) {:.3} ms",
            tr0.elapsed().as_secs_f64() * 1e3
        );
    }
    // Phase 3: contiguous bucket ranges per thread.
    let nb = 1usize << dbits;
    let mut dir = vec![0u32; nb + 1];
    let mut keys = vec![0u32; total];
    let mut refs = vec![0u32; total];
    let mut bloom = Bloom::new(total, bits);
    let wbits = bloom.words.len().trailing_zeros();
    assert!(wbits >= 8 && bloom.wshift == bits - wbits);
    let wper = 1usize << (wbits - 8);
    let mut bounds = vec![0usize; nt + 1];
    {
        let mut b = 0;
        for th in 1..nt {
            let goal = total * th / nt;
            while b < 256 && c8[b] < goal {
                b += 1;
            }
            bounds[th] = b;
        }
        bounds[nt] = 256;
    }
    {
        let mut jobs = Vec::new();
        let mut dir_rest: &mut [u32] = &mut dir[..nb];
        let mut keys_rest: &mut [u32] = &mut keys;
        let mut refs_rest: &mut [u32] = &mut refs;
        let mut words_rest: &mut [u64] = &mut bloom.words;
        for th in 0..nt {
            let (b0, b1) = (bounds[th], bounds[th + 1]);
            let (lo, hi) = (c8[b0], c8[b1]);
            let (d_now, d_t) = std::mem::take(&mut dir_rest).split_at_mut((b1 - b0) * sub);
            let (k_now, k_t) = std::mem::take(&mut keys_rest).split_at_mut(hi - lo);
            let (r_now, r_t) = std::mem::take(&mut refs_rest).split_at_mut(hi - lo);
            let (w_now, w_t) = std::mem::take(&mut words_rest).split_at_mut((b1 - b0) * wper);
            dir_rest = d_t;
            keys_rest = k_t;
            refs_rest = r_t;
            words_rest = w_t;
            let (tk, tr, c8) = (&tk, &tr, &c8);
            let wshift = bits - wbits;
            jobs.push(move || {
                for b in b0..b1 {
                    let (blo, bhi) = (c8[b], c8[b + 1]);
                    let mut cnt = [0usize; 257];
                    for &k in &tk[blo..bhi] {
                        cnt[((k >> shift) as usize & (sub - 1)) + 1] += 1;
                    }
                    for i in 0..sub {
                        cnt[i + 1] += cnt[i];
                    }
                    for i in 0..sub {
                        d_now[(b - b0) * sub + i] = (blo + cnt[i]) as u32;
                    }
                    let mut fl = cnt;
                    for i in blo..bhi {
                        let k = tk[i];
                        let sb = (k >> shift) as usize & (sub - 1);
                        k_now[blo - lo + fl[sb]] = k;
                        r_now[blo - lo + fl[sb]] = tr[i];
                        fl[sb] += 1;
                        let h = u64::from(k).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                        let bitsv = (1u64 << (h & 63))
                            | (1u64 << ((h >> 6) & 63))
                            | (1u64 << ((h >> 12) & 63))
                            | (1u64 << ((h >> 18) & 63));
                        let w = (k >> wshift) as usize - b0 * wper;
                        w_now[w] |= bitsv;
                    }
                }
            });
        }
        par_run(jobs);
    }
    dir[nb] = total as u32;
    if trace {
        eprintln!(
            "    phase 3 (buckets) {:.3} ms",
            tr0.elapsed().as_secs_f64() * 1e3
        );
    }
    let table = Table {
        keys,
        refs,
        dir,
        shift,
        len: total,
        collisions: 0,
        bloom,
        src: Source::Implicit {
            off,
            llast,
            lpk,
            depth,
        },
    };
    (table, invs, skipped)
}

/// A second, independent Bloom filter for the join mode (hash-chosen word,
/// bits from a different product), so the walk keeps only points that pass
/// both filters.
struct Bloom2 {
    words: Vec<u64>,
    mask: usize,
}

impl Bloom2 {
    fn new(keys: usize) -> Self {
        let words = (keys / 8).next_power_of_two().max(256);
        Self {
            words: vec![0; words],
            mask: words - 1,
        }
    }
    #[inline(always)]
    fn slot(&self, key: u64) -> (usize, u64) {
        let h = key.wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
        let bits = (1u64 << (h & 63))
            | (1u64 << ((h >> 6) & 63))
            | (1u64 << ((h >> 12) & 63))
            | (1u64 << ((h >> 18) & 63));
        ((h >> 34) as usize & self.mask, bits)
    }
    #[inline(always)]
    fn insert(&mut self, key: u64) {
        let (w, b) = self.slot(key);
        self.words[w] |= b;
    }
    #[inline(always)]
    fn maybe(&self, key: u64) -> bool {
        let (w, b) = self.slot(key);
        self.words[w] & b == b
    }
}

/// A walk point that passed both filters.
#[derive(Clone, Copy)]
struct Cand {
    x: u64,
    y: u64,
    a: u64,
    b: u64,
}

/// For the join mode: every key of the last level is formed twice, once
/// for the filters and once for the join; `visit(t0, kp, km)` receives the
/// keys of a chunk starting at pair `t0` (`u32::MAX`: absent).
#[cfg(target_arch = "x86_64")]
#[allow(clippy::too_many_arguments)]
fn last_level_chunks<R: Send>(
    f: &Mont,
    lv: &Levels,
    off: &[u32],
    bx: &[u64],
    by: &[u64],
    threads: usize,
    init: impl Fn() -> R + Sync,
    visit: impl Fn(&mut R, usize, &[u64], &[u64], &[u8], usize) + Sync,
) -> Vec<R> {
    const CH: usize = 4096;
    let a = bx.len();
    let n = *off.last().unwrap() as usize;
    let nt = threads.max(1);
    let per = (n.div_ceil(nt)).div_ceil(avx::GROUP) * avx::GROUP;
    let dummy_k = ((lv.llast[0] as usize + 1) % a) as u64;
    let jobs: Vec<_> = (0..nt)
        .map(|th| {
            let (init, visit) = (&init, &visit);
            move || {
                let mut acc = init();
                let t0 = (th * per).min(n);
                let t1 = ((th + 1) * per).min(n);
                let mut pe = vec![0u64; CH];
                let mut pk = vec![dummy_k; CH];
                let (mut dd, mut pre) = (vec![0u64; CH], vec![0u64; CH]);
                let mut zm = vec![0u8; CH / 8];
                let (mut xp, mut xm) = (vec![0u64; CH], vec![0u64; CH]);
                let (mut yp, mut ym) = (Vec::new(), Vec::new());
                let mut e = if t1 > t0 {
                    off.partition_point(|&o| o as usize <= t0) - 1
                } else {
                    0
                };
                let mut c0 = t0;
                while c0 < t1 {
                    let cl = CH.min(t1 - c0);
                    let clp = cl.div_ceil(avx::GROUP) * avx::GROUP;
                    for i in 0..clp {
                        if i < cl {
                            let t = c0 + i;
                            while off[e + 1] as usize <= t {
                                e += 1;
                            }
                            pe[i] = e as u64;
                            pk[i] = (lv.llast[e] as usize + 1 + (t - off[e] as usize)) as u64;
                        } else {
                            pe[i] = 0;
                            pk[i] = dummy_k;
                        }
                    }
                    // SAFETY: callers use this only when AVX-512 was detected.
                    unsafe {
                        avx::pair_add(
                            f,
                            &lv.lx,
                            &lv.ly,
                            bx,
                            by,
                            &pe[..clp],
                            &pk[..clp],
                            &mut dd[..clp],
                            &mut pre[..clp],
                            &mut zm[..clp / 8],
                            &mut xp[..clp],
                            &mut xm[..clp],
                            &mut yp,
                            &mut ym,
                            false,
                        )
                    };
                    visit(&mut acc, c0, &xp[..cl], &xm[..cl], &zm, cl);
                    c0 += cl;
                }
                acc
            }
        })
        .collect();
    par_run(jobs)
}

/// The two filters over every last-level key (per-thread filters, merged
/// by OR). Returns them with the last level's pair offsets.
#[cfg(target_arch = "x86_64")]
fn filters_par(
    f: &Mont,
    lv: &Levels,
    bx: &[u64],
    by: &[u64],
    threads: usize,
    p: u64,
) -> (Bloom, Bloom2, Vec<u32>, usize, u64) {
    let a = bx.len();
    let ne = lv.lx.len();
    let mut off = vec![0u32; ne + 1];
    for e in 0..ne {
        off[e + 1] = off[e] + (a - 1 - lv.llast[e] as usize) as u32;
    }
    let n = off[ne] as usize;
    let bits = 64 - (p - 1).leading_zeros();
    let parts = last_level_chunks(
        f,
        lv,
        &off,
        bx,
        by,
        threads,
        || (Bloom::new(2 * n, bits), Bloom2::new(2 * n), 0u64),
        |acc, _t0, xp, xm, zm, cl| {
            for i in 0..cl {
                if zm[i / 8] >> (i % 8) & 1 == 1 {
                    acc.2 += 1;
                    continue;
                }
                for k in [xp[i], xm[i]] {
                    acc.0.insert(k);
                    acc.1.insert(k);
                }
            }
        },
    );
    let mut it = parts.into_iter();
    let (mut b1, mut b2, mut skipped) = it.next().unwrap();
    for (o1, o2, sk) in it {
        for (w, v) in b1.words.iter_mut().zip(&o1.words) {
            *w |= v;
        }
        for (w, v) in b2.words.iter_mut().zip(&o2.words) {
            *w |= v;
        }
        skipped += sk;
    }
    (b1, b2, off, n, skipped)
}

/// The join: every last-level key against the candidates' abscissae;
/// returns (candidate index, key reference `2t + sign`) for each match.
#[cfg(target_arch = "x86_64")]
fn join_par(
    f: &Mont,
    lv: &Levels,
    off: &[u32],
    bx: &[u64],
    by: &[u64],
    threads: usize,
    cands: &[Cand],
) -> Vec<(usize, u32)> {
    let mut set: Vec<(u32, u32)> = cands
        .iter()
        .enumerate()
        .map(|(i, c)| (c.x as u32, i as u32))
        .collect();
    set.sort_unstable();
    let set = &set;
    // A 2^16-bit prefilter (8 KB, in L1) in front of the binary search.
    let mut bm = vec![0u64; 1 << 10];
    let hb = |k: u32| ((u64::from(k).wrapping_mul(0x9E37_79B9_7F4A_7C15)) >> 48) as usize;
    for &(k, _) in set.iter() {
        bm[hb(k) >> 6] |= 1 << (hb(k) & 63);
    }
    let bm = &bm;
    let parts = last_level_chunks(
        f,
        lv,
        off,
        bx,
        by,
        threads,
        Vec::<(usize, u32)>::new,
        |acc, t0, xp, xm, zm, cl| {
            for i in 0..cl {
                if zm[i / 8] >> (i % 8) & 1 == 1 {
                    continue;
                }
                for (k, sgn) in [(xp[i] as u32, 0u32), (xm[i] as u32, 1u32)] {
                    let hk = hb(k);
                    if bm[hk >> 6] >> (hk & 63) & 1 == 0 {
                        continue;
                    }
                    let lo = set.partition_point(|e| e.0 < k);
                    let mut j = lo;
                    while j < set.len() && set[j].0 == k {
                        acc.push((set[j].1 as usize, 2 * (t0 + i) as u32 + sgn));
                        j += 1;
                    }
                }
            }
        },
    );
    parts.into_iter().flatten().collect()
}

/// The m-sum table built with the AVX-512 pair kernel: level arrays in
/// SoA form, pairs padded to a multiple of the kernel group with a dummy
/// pair whose outputs are dropped. Same table as `build_ksum`.
#[cfg(target_arch = "x86_64")]
fn build_ksum_avx(
    f: &Mont,
    bx: &[u64],
    by: &[u64],
    m: u32,
    threads: usize,
    p: u64,
) -> (Table, u64, u64) {
    let (lv, invs, skipped) = build_levels_avx(f, bx, by, m);
    let t_level = Instant::now();
    let n: usize = lv.llast.iter().map(|&l| bx.len() - 1 - l as usize).sum();
    let (table, i2, s2) = last_level_par(
        f, &lv.lx, &lv.ly, lv.llast, lv.lpk, bx, by, lv.depth, threads, p,
    );
    if std::env::var_os("KSUM_TRACE").is_some() {
        eprintln!(
            "avx level {} pairs {} {:.3} ms (with table)",
            lv.depth + 1,
            n,
            t_level.elapsed().as_secs_f64() * 1e3
        );
    }
    (table, invs + i2, skipped + s2)
}

/// The levels below the last, in SoA form, for the last-level builders.
#[cfg(target_arch = "x86_64")]
struct Levels {
    lx: Vec<u64>,
    ly: Vec<u64>,
    llast: Vec<u32>,
    lpk: Vec<u64>,
    /// The depth of the next (last) level.
    depth: u32,
}

/// Levels 1 .. m − 1 of the m-sum build with the AVX-512 pair kernel.
#[cfg(target_arch = "x86_64")]
fn build_levels_avx(f: &Mont, bx: &[u64], by: &[u64], m: u32) -> (Levels, u64, u64) {
    let a = bx.len();
    let mut lx: Vec<u64> = bx.to_vec();
    let mut ly: Vec<u64> = by.to_vec();
    let mut llast: Vec<u32> = (0..a as u32).collect();
    let mut lpk: Vec<u64> = (0..a).map(|i| pack_push(0, 0, i, false)).collect();
    let mut invs = 0u64;
    let mut skipped = 0u64;
    let trace = std::env::var_os("KSUM_TRACE").is_some();
    for depth in 1..m {
        let t_level = Instant::now();
        let last_level = depth == m - 1;
        let n: usize = llast.iter().map(|&l| a - 1 - l as usize).sum();
        if last_level {
            let _ = (n, t_level);
            return (
                Levels {
                    lx,
                    ly,
                    llast,
                    lpk,
                    depth,
                },
                invs,
                skipped,
            );
        }
        let padded = n.div_ceil(avx::GROUP) * avx::GROUP;
        let mut pe = vec![0u64; padded];
        let mut pk = vec![((llast[0] as usize + 1) % a) as u64; padded];
        let mut t = 0;
        for e in 0..lx.len() {
            for k in llast[e] as usize + 1..a {
                pe[t] = e as u64;
                pk[t] = k as u64;
                t += 1;
            }
        }
        // Pairs past `n` are padding (entry 0 again); their outputs are dropped.
        if trace {
            eprintln!(
                "  level {} pair lists {:.3} ms",
                depth + 1,
                t_level.elapsed().as_secs_f64() * 1e3
            );
        }
        let mut dd = vec![0u64; padded];
        let mut pre = vec![0u64; padded];
        let mut zm = vec![0u8; padded / 8];
        let mut xp = vec![0u64; padded];
        let mut xm = vec![0u64; padded];
        let (mut yp, mut ym) = (vec![0u64; padded], vec![0u64; padded]);
        // Split across the pool's threads above a few thousand pairs: each
        // thread runs the kernel on a contiguous, group-aligned range and
        // writes its own slices (one inversion a range).
        let nt = POOL.get().map(|p| p.workers + 1).unwrap_or(1);
        if nt > 1 && padded >= 4096 {
            let per = padded.div_ceil(nt).div_ceil(avx::GROUP) * avx::GROUP;
            let (lxr, lyr, per_r) = (&lx, &ly, per);
            let mut jobs = Vec::new();
            let (mut ddr, mut prer, mut zmr): (&mut [u64], &mut [u64], &mut [u8]) =
                (&mut dd, &mut pre, &mut zm);
            let (mut xpr, mut xmr, mut ypr, mut ymr): (
                &mut [u64],
                &mut [u64],
                &mut [u64],
                &mut [u64],
            ) = (&mut xp, &mut xm, &mut yp, &mut ym);
            let mut start = 0;
            while start < padded {
                let len = per_r.min(padded - start);
                let (d0, d1) = std::mem::take(&mut ddr).split_at_mut(len);
                let (p0, p1) = std::mem::take(&mut prer).split_at_mut(len);
                let (z0, z1) = std::mem::take(&mut zmr).split_at_mut(len / 8);
                let (a0, a1) = std::mem::take(&mut xpr).split_at_mut(len);
                let (b0, b1) = std::mem::take(&mut xmr).split_at_mut(len);
                let (c0, c1) = std::mem::take(&mut ypr).split_at_mut(len);
                let (e0, e1) = std::mem::take(&mut ymr).split_at_mut(len);
                (ddr, prer, zmr, xpr, xmr, ypr, ymr) = (d1, p1, z1, a1, b1, c1, e1);
                let (pe_s, pk_s) = (&pe[start..start + len], &pk[start..start + len]);
                jobs.push(move || {
                    // SAFETY: callers use this only when AVX-512 was detected.
                    unsafe {
                        avx::pair_add(
                            f, lxr, lyr, bx, by, pe_s, pk_s, d0, p0, z0, a0, b0, c0, e0, true,
                        )
                    };
                });
                start += len;
            }
            invs += jobs.len() as u64;
            par_run(jobs);
        } else {
            // SAFETY: callers use this only when AVX-512 was detected.
            unsafe {
                avx::pair_add(
                    f, &lx, &ly, bx, by, &pe, &pk, &mut dd, &mut pre, &mut zm, &mut xp, &mut xm,
                    &mut yp, &mut ym, true,
                )
            };
            invs += 1;
        }
        let zero_at = |t: usize| zm[t / 8] >> (t % 8) & 1 == 1;

        let live = (0..n).filter(|&t| !zero_at(t)).count();
        skipped += (n - live) as u64;
        let mut nx = vec![0u64; 2 * live];
        let mut ny = vec![0u64; 2 * live];
        let mut nl = vec![0u32; 2 * live];
        let mut np = vec![0u64; 2 * live];
        let mut o = 0;
        for t in 0..n {
            if zero_at(t) {
                continue;
            }
            let (e, k) = (pe[t] as usize, pk[t] as usize);
            nx[o] = xp[t];
            ny[o] = yp[t];
            nl[o] = k as u32;
            np[o] = pack_push(lpk[e], depth, k, false);
            nx[o + 1] = xm[t];
            ny[o + 1] = ym[t];
            nl[o + 1] = k as u32;
            np[o + 1] = pack_push(lpk[e], depth, k, true);
            o += 2;
        }
        if trace {
            eprintln!(
                "avx level {} pairs {} {:.3} ms",
                depth + 1,
                n,
                t_level.elapsed().as_secs_f64() * 1e3
            );
        }
        lx = nx;
        ly = ny;
        llast = nl;
        lpk = np;
    }
    unreachable!("m >= 2")
}

/// A verified relation: Σ signs·P_idx = a G + b Q.
#[derive(Clone)]
struct Rel {
    cols: Vec<(usize, bool)>,
    a: u64,
    b: u64,
}

fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}
fn addm(a: u64, b: u64, r: u64) -> u64 {
    let s = a + b;
    if s >= r {
        s - r
    } else {
        s
    }
}
fn subm(a: u64, b: u64, r: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + r - b
    }
}
fn powm(mut b: u64, mut e: u64, r: u64) -> u64 {
    let mut acc = 1;
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, b, r);
        }
        b = mulmod(b, b, r);
        e >>= 1;
    }
    acc
}

/// `x = −Σλa/Σλb` for a left-kernel vector λ of the relation matrix.
fn solve(rels: &[Rel], a: usize, r: u64) -> Option<u64> {
    let m = rels.len();
    let mut mat: Vec<Vec<u64>> = rels
        .iter()
        .map(|rel| {
            let mut row = vec![0u64; a];
            for &(i, neg) in &rel.cols {
                row[i] = if neg {
                    subm(row[i], 1, r)
                } else {
                    addm(row[i], 1, r)
                };
            }
            row
        })
        .collect();
    let mut track: Vec<Vec<u64>> = (0..m)
        .map(|i| (0..m).map(|j| u64::from(i == j)).collect())
        .collect();
    let mut prow = 0;
    for col in 0..a {
        let Some(piv) = (prow..m).find(|&i| mat[i][col] != 0) else {
            continue;
        };
        mat.swap(prow, piv);
        track.swap(prow, piv);
        let inv = powm(mat[prow][col], r - 2, r);
        for i in 0..m {
            if i != prow && mat[i][col] != 0 {
                let fct = mulmod(mat[i][col], inv, r);
                for j in 0..a {
                    let t = mulmod(fct, mat[prow][j], r);
                    mat[i][j] = subm(mat[i][j], t, r);
                }
                for j in 0..m {
                    let t = mulmod(fct, track[prow][j], r);
                    track[i][j] = subm(track[i][j], t, r);
                }
            }
        }
        prow += 1;
    }
    for i in prow..m {
        let lam = &track[i];
        let (mut sa, mut sb) = (0u64, 0u64);
        for (j, rel) in rels.iter().enumerate() {
            sa = addm(sa, mulmod(lam[j], rel.a, r), r);
            sb = addm(sb, mulmod(lam[j], rel.b, r), r);
        }
        if sb != 0 {
            return Some(mulmod(subm(0, sa, r), powm(sb, r - 2, r), r));
        }
    }
    None
}

struct Instance<'g, G: CountedGroup<Elt = PrimePoint>> {
    g: &'g G,
    f: Mont,
    r: u64,
    gen: PrimePoint,
    reps: Vec<PrimePoint>,
    bx: Vec<u64>,
    by: Vec<u64>,
}

/// Walk statistics of one solve.
#[derive(Default, Clone, Copy)]
struct WalkStats {
    steps: u64,
    filter_pass: u64,
    hits: u64,
    verified: u64,
    rejected: u64,
    exceptional: u64,
}

/// Interleaved product chains for the lockstep walk.
const LANE_K: usize = 16;
const NW: usize = 32;

/// One r-adding step for every lane: `LANE_K` independent product chains
/// (lane `l` in chain `l % LANE_K`) so the Montgomery trick is not one
/// serial chain, one inversion for all lanes. Lanes with `x_R = x_W`
/// keep their point and are listed in `exc` for a library step.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn step_lanes(
    f: &Mont,
    x: &mut [u64],
    y: &mut [u64],
    la: &mut [u64],
    lb: &mut [u64],
    wx: &[u64],
    wy: &[u64],
    wc: &[u64],
    wd: &[u64],
    r: u64,
    j: &mut [usize],
    dd: &mut [u64],
    pre: &mut [u64],
    zero: &mut [bool],
    exc: &mut Vec<usize>,
) {
    const K: usize = LANE_K;
    let groups = x.len() / K;
    let one = f.one;
    let mut acc = [one; K];
    for gi in 0..groups {
        for i in 0..K {
            let l = gi * K + i;
            let jj = ((x[l].wrapping_mul(0xD6E8_FEB8_6659_FD93)) >> 59) as usize & (NW - 1);
            j[l] = jj;
            let d = f.sub(wx[jj], x[l]);
            let z = d == 0;
            zero[l] = z;
            let d = if z { one } else { d };
            dd[l] = d;
            pre[l] = acc[i];
            acc[i] = f.mul(acc[i], d);
        }
    }
    let mut lpre = [0u64; K];
    let mut all = one;
    for i in 0..K {
        lpre[i] = all;
        all = f.mul(all, acc[i]);
    }
    let mut ai = f.inv(all);
    let mut inv = [0u64; K];
    for i in (0..K).rev() {
        inv[i] = f.mul(ai, lpre[i]);
        ai = f.mul(ai, acc[i]);
    }
    for gi in (0..groups).rev() {
        for i in 0..K {
            let l = gi * K + i;
            let jj = j[l];
            let dinv = f.mul(inv[i], pre[l]);
            inv[i] = f.mul(inv[i], dd[l]);
            let (xw, yw) = (wx[jj], wy[jj]);
            let lam = f.mul(f.sub(yw, y[l]), dinv);
            let x3 = f.sub(f.sub(f.mul(lam, lam), x[l]), xw);
            let y3 = f.sub(f.mul(lam, f.sub(x[l], x3)), y[l]);
            let z = zero[l];
            x[l] = if z { x[l] } else { x3 };
            y[l] = if z { y[l] } else { y3 };
            la[l] = if z { la[l] } else { addm(la[l], wc[jj], r) };
            lb[l] = if z { lb[l] } else { addm(lb[l], wd[jj], r) };
        }
    }
    for (l, &z) in zero.iter().enumerate() {
        if z {
            exc.push(l);
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,avx512f,avx512dq,avx512vl,bmi2")]
#[allow(clippy::too_many_arguments)]
unsafe fn step_lanes_v(
    f: &Mont,
    x: &mut [u64],
    y: &mut [u64],
    la: &mut [u64],
    lb: &mut [u64],
    wx: &[u64],
    wy: &[u64],
    wc: &[u64],
    wd: &[u64],
    r: u64,
    j: &mut [usize],
    dd: &mut [u64],
    pre: &mut [u64],
    zero: &mut [bool],
    exc: &mut Vec<usize>,
) {
    step_lanes(f, x, y, la, lb, wx, wy, wc, wd, r, j, dd, pre, zero, exc)
}

#[allow(clippy::too_many_arguments)]
/// AVX-512 present and not disabled with `KSUM_SCALAR` (decided once).
fn use_simd() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        std::env::var_os("KSUM_SCALAR").is_none()
            && is_x86_feature_detected!("avx512f")
            && is_x86_feature_detected!("avx512dq")
            && is_x86_feature_detected!("avx512vl")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

fn step_any(
    simd: bool,
    f: &Mont,
    x: &mut [u64],
    y: &mut [u64],
    la: &mut [u64],
    lb: &mut [u64],
    wx: &[u64],
    wy: &[u64],
    wc: &[u64],
    wd: &[u64],
    r: u64,
    j: &mut [usize],
    dd: &mut [u64],
    pre: &mut [u64],
    zero: &mut [bool],
    exc: &mut Vec<usize>,
) {
    #[cfg(target_arch = "x86_64")]
    if simd {
        // SAFETY: `simd` is set only when the features were detected.
        return unsafe { step_lanes_v(f, x, y, la, lb, wx, wy, wc, wd, r, j, dd, pre, zero, exc) };
    }
    step_lanes(f, x, y, la, lb, wx, wy, wc, wd, r, j, dd, pre, zero, exc)
}

/// The walk step with explicit AVX-512 intrinsics: 8 lanes a vector, `V`
/// vectors interleaved (`8 V` independent product chains), Montgomery
/// products by `vpmuludq` (every residue is below 2^32) and conditional
/// subtraction as `min(u, u − p)`; step points and coefficients gathered.
#[cfg(target_arch = "x86_64")]
mod avx {
    use super::{Mont, NW};
    use std::arch::x86_64::*;

    pub const V: usize = 4;
    pub const GROUP: usize = 8 * V;

    #[derive(Clone, Copy)]
    struct C {
        p: __m512i,
        ninv: __m512i,
        lo: __m512i,
        one64: __m512i,
        onem: __m512i,
        r: __m512i,
        hashc: __m512i,
    }

    #[inline(always)]
    unsafe fn mmul(c: &C, a: __m512i, b: __m512i) -> __m512i {
        let t = _mm512_mul_epu32(a, b);
        let m = _mm512_mul_epu32(t, c.ninv);
        let mp = _mm512_mul_epu32(m, c.p);
        let u = _mm512_add_epi64(_mm512_srli_epi64::<32>(t), _mm512_srli_epi64::<32>(mp));
        let carry = _mm512_test_epi64_mask(t, c.lo);
        let u = _mm512_mask_add_epi64(u, carry, u, c.one64);
        _mm512_min_epu64(u, _mm512_sub_epi64(u, c.p))
    }
    #[inline(always)]
    unsafe fn msub(c: &C, a: __m512i, b: __m512i) -> __m512i {
        let s = _mm512_add_epi64(a, _mm512_sub_epi64(c.p, b));
        _mm512_min_epu64(s, _mm512_sub_epi64(s, c.p))
    }
    #[inline(always)]
    unsafe fn addr(c: &C, a: __m512i, b: __m512i) -> __m512i {
        let s = _mm512_add_epi64(a, b);
        _mm512_min_epu64(s, _mm512_sub_epi64(s, c.r))
    }

    /// One step for every lane (`x.len()` a multiple of `GROUP`).
    #[target_feature(enable = "avx2,avx512f,avx512dq,avx512vl,bmi2")]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn step(
        f: &Mont,
        x: &mut [u64],
        y: &mut [u64],
        la: &mut [u64],
        lb: &mut [u64],
        wx: &[u64],
        wy: &[u64],
        wc: &[u64],
        wd: &[u64],
        r: u64,
        j: &mut [u64],
        dd: &mut [u64],
        pre: &mut [u64],
        zmask: &mut [u8],
        exc: &mut Vec<usize>,
        xs: &mut [u64],
    ) {
        debug_assert!(wx.len() == NW && x.len() % GROUP == 0);
        debug_assert!(wd.iter().all(|&d| d == 1), "steps W_j = Q + c_j G");
        let c = C {
            p: _mm512_set1_epi64(f.p as i64),
            ninv: _mm512_set1_epi64(f.ninv as i64),
            lo: _mm512_set1_epi64(0xFFFF_FFFF),
            one64: _mm512_set1_epi64(1),
            onem: _mm512_set1_epi64(f.one as i64),
            r: _mm512_set1_epi64(r as i64),
            hashc: _mm512_set1_epi64(0xD6E8_FEB8_6659_FD93u64 as i64),
        };
        let lanes = x.len();
        let groups = lanes / GROUP;
        let mut acc = [c.onem; V];
        for g in 0..groups {
            for v in 0..V {
                let o = g * GROUP + v * 8;
                let xv = _mm512_loadu_si512(x.as_ptr().add(o) as *const _);
                let jj = _mm512_srli_epi64::<59>(_mm512_mullo_epi64(xv, c.hashc));
                let xw = _mm512_i64gather_epi64::<8>(jj, wx.as_ptr() as *const _);
                let d = msub(&c, xw, xv);
                let z = _mm512_cmpeq_epi64_mask(d, _mm512_setzero_si512());
                let d = _mm512_mask_mov_epi64(d, z, c.onem);
                _mm512_storeu_si512(j.as_mut_ptr().add(o) as *mut _, jj);
                _mm512_storeu_si512(dd.as_mut_ptr().add(o) as *mut _, d);
                _mm512_storeu_si512(pre.as_mut_ptr().add(o) as *mut _, acc[v]);
                zmask[o / 8] = z;
                acc[v] = mmul(&c, acc[v], d);
            }
        }
        // Invert the 8 V chain products with one scalar Fermat inversion.
        let mut a = [0u64; 8 * V];
        for v in 0..V {
            _mm512_storeu_si512(a.as_mut_ptr().add(v * 8) as *mut _, acc[v]);
        }
        let mut lp = [0u64; 8 * V];
        let mut all = f.one;
        for i in 0..8 * V {
            lp[i] = all;
            all = f.mul(all, a[i]);
        }
        let mut ai = f.inv(all);
        let mut iv = [0u64; 8 * V];
        for i in (0..8 * V).rev() {
            iv[i] = f.mul(ai, lp[i]);
            ai = f.mul(ai, a[i]);
        }
        let mut inv = [c.onem; V];
        for v in 0..V {
            inv[v] = _mm512_loadu_si512(iv.as_ptr().add(v * 8) as *const _);
        }
        for g in (0..groups).rev() {
            for v in 0..V {
                let o = g * GROUP + v * 8;
                let jj = _mm512_loadu_si512(j.as_ptr().add(o) as *const _);
                let pv = _mm512_loadu_si512(pre.as_ptr().add(o) as *const _);
                let dv = _mm512_loadu_si512(dd.as_ptr().add(o) as *const _);
                let dinv = mmul(&c, inv[v], pv);
                inv[v] = mmul(&c, inv[v], dv);
                let xv = _mm512_loadu_si512(x.as_ptr().add(o) as *const _);
                let yv = _mm512_loadu_si512(y.as_ptr().add(o) as *const _);
                // x_W = d + x_R (d was stored by the forward pass; wrong
                // only on masked lanes).
                let xw = madd(&c, dv, xv);
                let yw = _mm512_i64gather_epi64::<8>(jj, wy.as_ptr() as *const _);
                let lam = mmul(&c, msub(&c, yw, yv), dinv);
                let x3 = msub(&c, msub(&c, mmul(&c, lam, lam), xv), xw);
                let y3 = msub(&c, mmul(&c, lam, msub(&c, xv, x3)), yv);
                let keep = !zmask[o / 8];
                // The sibling R − W_j from the same denominator: a second
                // probe point for one more product and one square.
                let lmi = mmul(&c, msub(&c, msub(&c, _mm512_setzero_si512(), yw), yv), dinv);
                let x3m = msub(&c, msub(&c, mmul(&c, lmi, lmi), xv), xw);
                _mm512_storeu_si512(
                    xs.as_mut_ptr().add(o) as *mut _,
                    _mm512_mask_mov_epi64(_mm512_set1_epi64(-1), keep, x3m),
                );
                _mm512_storeu_si512(
                    x.as_mut_ptr().add(o) as *mut _,
                    _mm512_mask_mov_epi64(xv, keep, x3),
                );
                _mm512_storeu_si512(
                    y.as_mut_ptr().add(o) as *mut _,
                    _mm512_mask_mov_epi64(yv, keep, y3),
                );
                let lav = _mm512_loadu_si512(la.as_ptr().add(o) as *const _);
                let lbv = _mm512_loadu_si512(lb.as_ptr().add(o) as *const _);
                let cw = _mm512_i64gather_epi64::<8>(jj, wc.as_ptr() as *const _);
                _mm512_storeu_si512(
                    la.as_mut_ptr().add(o) as *mut _,
                    _mm512_mask_mov_epi64(lav, keep, addr(&c, lav, cw)),
                );
                _mm512_storeu_si512(
                    lb.as_mut_ptr().add(o) as *mut _,
                    _mm512_mask_mov_epi64(lbv, keep, addr(&c, lbv, c.one64)),
                );
            }
        }
        for (vi, &z) in zmask.iter().enumerate() {
            let mut z = z;
            while z != 0 {
                exc.push(vi * 8 + z.trailing_zeros() as usize);
                z &= z - 1;
            }
        }
    }

    #[inline(always)]
    unsafe fn madd(c: &C, a: __m512i, b: __m512i) -> __m512i {
        let s = _mm512_add_epi64(a, b);
        _mm512_min_epu64(s, _mm512_sub_epi64(s, c.p))
    }

    /// Batched affine additions `S_e ± P_k` for a list of pairs (both signs
    /// from one denominator; `n` a multiple of `GROUP`, the caller pads).
    /// Writes `x(S + P)`, `x(S − P)` and, with `want_y`, the ordinates;
    /// pairs with `x_S = x_P` are flagged in `zmask` and left as 0.
    #[target_feature(enable = "avx2,avx512f,avx512dq,avx512vl,bmi2")]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn pair_add(
        f: &Mont,
        sx: &[u64],
        sy: &[u64],
        bx: &[u64],
        by: &[u64],
        pe: &[u64],
        pk: &[u64],
        dd: &mut [u64],
        pre: &mut [u64],
        zmask: &mut [u8],
        xp: &mut [u64],
        xm: &mut [u64],
        yp: &mut [u64],
        ym: &mut [u64],
        want_y: bool,
    ) {
        let c = C {
            p: _mm512_set1_epi64(f.p as i64),
            ninv: _mm512_set1_epi64(f.ninv as i64),
            lo: _mm512_set1_epi64(0xFFFF_FFFF),
            one64: _mm512_set1_epi64(1),
            onem: _mm512_set1_epi64(f.one as i64),
            r: _mm512_set1_epi64(0),
            hashc: _mm512_set1_epi64(0),
        };
        let zero = _mm512_setzero_si512();
        let n = pe.len();
        let groups = n / GROUP;
        let mut acc = [c.onem; V];
        for g in 0..groups {
            for v in 0..V {
                let o = g * GROUP + v * 8;
                let e = _mm512_loadu_si512(pe.as_ptr().add(o) as *const _);
                let k = _mm512_loadu_si512(pk.as_ptr().add(o) as *const _);
                let x1 = _mm512_i64gather_epi64::<8>(e, sx.as_ptr() as *const _);
                let x2 = _mm512_i64gather_epi64::<8>(k, bx.as_ptr() as *const _);
                let d = msub(&c, x2, x1);
                let z = _mm512_cmpeq_epi64_mask(d, zero);
                let d = _mm512_mask_mov_epi64(d, z, c.onem);
                _mm512_storeu_si512(dd.as_mut_ptr().add(o) as *mut _, d);
                _mm512_storeu_si512(pre.as_mut_ptr().add(o) as *mut _, acc[v]);
                zmask[o / 8] = z;
                acc[v] = mmul(&c, acc[v], d);
            }
        }
        let mut a = [0u64; 8 * V];
        for v in 0..V {
            _mm512_storeu_si512(a.as_mut_ptr().add(v * 8) as *mut _, acc[v]);
        }
        let mut lp = [0u64; 8 * V];
        let mut all = f.one;
        for i in 0..8 * V {
            lp[i] = all;
            all = f.mul(all, a[i]);
        }
        let mut ai = f.inv(all);
        let mut iv = [0u64; 8 * V];
        for i in (0..8 * V).rev() {
            iv[i] = f.mul(ai, lp[i]);
            ai = f.mul(ai, a[i]);
        }
        let mut inv = [c.onem; V];
        for v in 0..V {
            inv[v] = _mm512_loadu_si512(iv.as_ptr().add(v * 8) as *const _);
        }
        for g in (0..groups).rev() {
            for v in 0..V {
                let o = g * GROUP + v * 8;
                let e = _mm512_loadu_si512(pe.as_ptr().add(o) as *const _);
                let k = _mm512_loadu_si512(pk.as_ptr().add(o) as *const _);
                let pv = _mm512_loadu_si512(pre.as_ptr().add(o) as *const _);
                let dv = _mm512_loadu_si512(dd.as_ptr().add(o) as *const _);
                let dinv = mmul(&c, inv[v], pv);
                inv[v] = mmul(&c, inv[v], dv);
                let x1 = _mm512_i64gather_epi64::<8>(e, sx.as_ptr() as *const _);
                let y1 = _mm512_i64gather_epi64::<8>(e, sy.as_ptr() as *const _);
                let x2 = _mm512_i64gather_epi64::<8>(k, bx.as_ptr() as *const _);
                let y2 = _mm512_i64gather_epi64::<8>(k, by.as_ptr() as *const _);
                let base = madd(&c, x1, x2);
                let lpl = mmul(&c, msub(&c, y2, y1), dinv);
                let lmi = mmul(&c, msub(&c, msub(&c, zero, y2), y1), dinv);
                let x3p = msub(&c, mmul(&c, lpl, lpl), base);
                let x3m = msub(&c, mmul(&c, lmi, lmi), base);
                let keep = !zmask[o / 8];
                _mm512_storeu_si512(
                    xp.as_mut_ptr().add(o) as *mut _,
                    _mm512_maskz_mov_epi64(keep, x3p),
                );
                _mm512_storeu_si512(
                    xm.as_mut_ptr().add(o) as *mut _,
                    _mm512_maskz_mov_epi64(keep, x3m),
                );
                if want_y {
                    let y3p = msub(&c, mmul(&c, lpl, msub(&c, x1, x3p)), y1);
                    let y3m = msub(&c, mmul(&c, lmi, msub(&c, x1, x3m)), y1);
                    _mm512_storeu_si512(yp.as_mut_ptr().add(o) as *mut _, y3p);
                    _mm512_storeu_si512(ym.as_mut_ptr().add(o) as *mut _, y3m);
                }
            }
        }
    }

    /// Lanes whose abscissa passes the Bloom filter (the same slot function
    /// as `Bloom::slot`), 8 keys a step; `x.len()` a multiple of 8.
    #[target_feature(enable = "avx2,avx512f,avx512dq,avx512vl,bmi2")]
    pub unsafe fn bloom_pass(
        words: &[u64],
        mask: usize,
        wshift: u32,
        x: &[u64],
        out: &mut Vec<usize>,
    ) {
        let c = _mm512_set1_epi64(0x9E37_79B9_7F4A_7C15u64 as i64);
        let m63 = _mm512_set1_epi64(63);
        let wm = _mm512_set1_epi64(mask as i64);
        let ws = _mm512_set1_epi64(i64::from(wshift));
        let one = _mm512_set1_epi64(1);
        for (ci, chunk) in x.chunks_exact(8).enumerate() {
            let xv = _mm512_loadu_si512(chunk.as_ptr() as *const _);
            let h = _mm512_mullo_epi64(xv, c);
            let idx = _mm512_and_si512(_mm512_srlv_epi64(xv, ws), wm);
            let w = _mm512_i64gather_epi64::<8>(idx, words.as_ptr() as *const _);
            let b0 = _mm512_sllv_epi64(one, _mm512_and_si512(h, m63));
            let b1 = _mm512_sllv_epi64(one, _mm512_and_si512(_mm512_srli_epi64::<6>(h), m63));
            let b2 = _mm512_sllv_epi64(one, _mm512_and_si512(_mm512_srli_epi64::<12>(h), m63));
            let b3 = _mm512_sllv_epi64(one, _mm512_and_si512(_mm512_srli_epi64::<18>(h), m63));
            let bits = _mm512_or_si512(_mm512_or_si512(b0, b1), _mm512_or_si512(b2, b3));
            let mut pass: u8 = _mm512_cmpeq_epi64_mask(_mm512_and_si512(w, bits), bits);
            while pass != 0 {
                out.push(ci * 8 + pass.trailing_zeros() as usize);
                pass &= pass - 1;
            }
        }
    }
}

/// `R_l ← R_l + W_{idx[l]}` for every lane, one inversion (scalar;
/// used once per thread for the lane starts). A lane with `x_R = x_W`
/// would need a doubling; it cannot occur for these starts with
/// overwhelming probability and is asserted against.
fn batched_add(f: &Mont, x: &mut [u64], y: &mut [u64], idx: &[usize], wx: &[u64], wy: &[u64]) {
    let n = x.len();
    let mut pre = vec![0u64; n];
    let mut acc = f.one;
    for l in 0..n {
        let d = f.sub(wx[idx[l]], x[l]);
        assert!(d != 0, "a lane start met a step point");
        pre[l] = acc;
        acc = f.mul(acc, d);
    }
    let mut inv = f.inv(acc);
    for l in (0..n).rev() {
        let d = f.sub(wx[idx[l]], x[l]);
        let dinv = f.mul(inv, pre[l]);
        inv = f.mul(inv, d);
        let lam = f.mul(f.sub(wy[idx[l]], y[l]), dinv);
        let x3 = f.sub(f.sub(f.mul(lam, lam), x[l]), wx[idx[l]]);
        y[l] = f.sub(f.mul(lam, f.sub(x[l], x3)), y[l]);
        x[l] = x3;
    }
}

/// One thread's lockstep walks until `done` is set; verified relations go
/// to `sink`.
#[allow(clippy::too_many_arguments)]
fn walk_thread<G: CountedGroup<Elt = PrimePoint>>(
    inst: &Instance<G>,
    table: Option<&Table>,
    m: u32,
    q: PrimePoint,
    steps_w: &[(PrimePoint, u64, u64)],
    lanes: usize,
    seed: u64,
    done: &AtomicBool,
    sink: &Mutex<(Vec<Rel>, HashSet<u64>)>,
    need: usize,
    filt: Option<(&Bloom, &Bloom2, &Mutex<Vec<Cand>>, usize)>,
) -> WalkStats {
    let (g, f, r) = (inst.g, &inst.f, inst.r);
    let mut local: Vec<Cand> = Vec::new();
    let mut ops = GroupOps::default();
    let mut st = WalkStats::default();
    let wx: Vec<u64> = steps_w.iter().map(|(p, _, _)| f.to(p.x)).collect();
    let wy: Vec<u64> = steps_w.iter().map(|(p, _, _)| f.to(p.y)).collect();
    assert_eq!(steps_w.len(), NW);
    let wc: Vec<u64> = steps_w.iter().map(|w| w.1).collect();
    let wd: Vec<u64> = steps_w.iter().map(|w| w.2).collect();
    // Lane starts: R_0 = a0 G + Q (library), then lane l = R_0 + W_u + W_v + W_w
    // for distinct triples u < v < w (4960 of them), by three batched additions.
    let a0 = 1 + seed % (r - 1);
    let a0g = g.mul(&mut ops, inst.gen, a0);
    let r0 = g.add(&mut ops, a0g, q);
    let triples: Vec<[usize; 3]> = (0..NW)
        .flat_map(|u| (u + 1..NW).flat_map(move |v| (v + 1..NW).map(move |w| [u, v, w])))
        .collect();
    assert!(lanes <= triples.len(), "at most {} lanes", triples.len());
    let off = (seed as usize) % triples.len();
    let lane_triple = |l: usize| triples[(off + l * 7) % triples.len()];
    let mut x = vec![f.to(r0.x); lanes];
    let mut y = vec![f.to(r0.y); lanes];
    let mut la = vec![a0; lanes];
    let mut lb = vec![1u64; lanes];
    for pass in 0..3 {
        let idx: Vec<usize> = (0..lanes).map(|l| lane_triple(l)[pass]).collect();
        batched_add(f, &mut x, &mut y, &idx, &wx, &wy);
        for l in 0..lanes {
            la[l] = addm(la[l], wc[idx[l]], r);
            lb[l] = addm(lb[l], wd[idx[l]], r);
        }
    }
    let mut j = vec![0usize; lanes];
    let mut dd = vec![0u64; lanes];
    let mut pre = vec![0u64; lanes];
    let mut found: Vec<(usize, u64)> = Vec::new();
    // KSUM_NOPROBE: time the walk arithmetic alone (stops at a step cap).
    let probe = std::env::var_os("KSUM_NOPROBE").is_none();
    let simd = use_simd();
    // KSUM_NOINTR: the compiler-vectorized step instead of the intrinsics.
    let intr = simd && std::env::var_os("KSUM_NOINTR").is_none();
    let mut j64 = vec![0u64; lanes];
    let mut zmask = vec![0u8; lanes / 8];
    let mut cand: Vec<usize> = Vec::with_capacity(lanes);
    // Sibling probes: x(R − W_j) of every step, with the pre-step points
    // kept for verifying a hit (KSUM_NOSIB disables them).
    let sib = intr && std::env::var_os("KSUM_NOSIB").is_none();
    let mut xs = vec![u64::MAX; lanes];
    let (mut xprev, mut yprev) = (vec![0u64; lanes], vec![0u64; lanes]);
    let mut found2: Vec<(usize, u64)> = Vec::new();
    // Fixed offsets T_i = t_i G (KSUM_OFFS, default 0): every walk point R
    // is also probed as R ± T_i, two points per offset from one shared
    // denominator, through the table builder's pair kernel.
    let n_offs: usize = std::env::var("KSUM_OFFS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let offs: Vec<(PrimePoint, u64)> = (0..n_offs)
        .map(|i| {
            let t = (0x9E37_79B9u64.wrapping_mul(i as u64 + 1) ^ 0x5bd1_e995) % r;
            (g.mul(&mut ops, inst.gen, t), t)
        })
        .collect();
    let otx: Vec<u64> = offs.iter().map(|o| f.to(o.0.x)).collect();
    let oty: Vec<u64> = offs.iter().map(|o| f.to(o.0.y)).collect();
    let lane_ids: Vec<u64> = (0..lanes as u64).collect();
    let mut opk = vec![0u64; lanes];
    let (mut odd, mut opre) = (vec![0u64; lanes], vec![0u64; lanes]);
    let mut ozm = vec![0u8; lanes / 8];
    let (mut oxp, mut oxm) = (vec![0u64; lanes], vec![0u64; lanes]);
    let (mut oyp, mut oym) = (Vec::<u64>::new(), Vec::<u64>::new());
    let mut found3: Vec<(usize, u64, usize, bool)> = Vec::new();
    let cap: u64 = if probe { u64::MAX } else { 300_000 };
    let mut zero = vec![false; lanes];
    let mut exc: Vec<usize> = Vec::new();
    assert!(lanes % 32 == 0, "lanes must be a multiple of 32");
    while !done.load(Ordering::Relaxed) && st.steps < cap {
        for _ in 0..64 {
            // Probe the current points, then step every lane once.
            if let Some((b1, b2, shared, cap)) = filt {
                cand.clear();
                #[cfg(target_arch = "x86_64")]
                // SAFETY: join mode runs only when AVX-512 was detected.
                unsafe {
                    avx::bloom_pass(&b1.words, b1.mask, b1.wshift, &x, &mut cand)
                };
                for &l in &cand {
                    if b2.maybe(x[l]) {
                        local.push(Cand {
                            x: x[l],
                            y: y[l],
                            a: la[l],
                            b: lb[l],
                        });
                    }
                }
                st.steps += lanes as u64;
                st.filter_pass += cand.len() as u64;
                if !local.is_empty() {
                    let mut sh = shared.lock().unwrap();
                    sh.extend(local.drain(..));
                    if sh.len() >= cap {
                        done.store(true, Ordering::Relaxed);
                    }
                }
            } else if probe {
                #[cfg(target_arch = "x86_64")]
                if intr {
                    cand.clear();
                    // SAFETY: `intr` implies AVX-512 was detected.
                    unsafe {
                        avx::bloom_pass(
                            &table.unwrap().bloom.words,
                            table.unwrap().bloom.mask,
                            table.unwrap().bloom.wshift,
                            &x,
                            &mut cand,
                        )
                    };
                    for &l in &cand {
                        if let Some(v) = table.unwrap().get_exact(x[l]) {
                            found.push((l, v));
                        }
                    }
                    let tbl = table.unwrap();
                    for oi in 0..n_offs {
                        opk.iter_mut().for_each(|v| *v = oi as u64);
                        // SAFETY: `intr` implies AVX-512 was detected.
                        unsafe {
                            avx::pair_add(
                                f, &x, &y, &otx, &oty, &lane_ids, &opk, &mut odd, &mut opre,
                                &mut ozm, &mut oxp, &mut oxm, &mut oyp, &mut oym, false,
                            )
                        };
                        for (out, neg) in [(&oxp, false), (&oxm, true)] {
                            cand.clear();
                            // SAFETY: as above.
                            unsafe {
                                avx::bloom_pass(
                                    &tbl.bloom.words,
                                    tbl.bloom.mask,
                                    tbl.bloom.wshift,
                                    out,
                                    &mut cand,
                                )
                            };
                            for &l in &cand {
                                if ozm[l / 8] >> (l % 8) & 1 == 1 {
                                    continue;
                                }
                                if let Some(v) = tbl.get_exact(out[l]) {
                                    found3.push((l, v, oi, neg));
                                }
                            }
                        }
                        st.steps += 2 * lanes as u64;
                    }
                } else {
                    for l in 0..lanes {
                        if let Some(v) = table.unwrap().get(x[l]) {
                            found.push((l, v));
                        }
                    }
                }
                #[cfg(not(target_arch = "x86_64"))]
                for l in 0..lanes {
                    if let Some(v) = table.unwrap().get(x[l]) {
                        found.push((l, v));
                    }
                }
            }
            if filt.is_none() {
                st.steps += lanes as u64;
            }
            for (l, v) in found.drain(..) {
                st.hits += 1;
                let rp = PrimePoint::affine(f.from(x[l]), f.from(y[l]));
                let cols = unpack(v, m);
                let mut s = PrimePoint {
                    x: 0,
                    y: 0,
                    infinity: true,
                };
                for &(i, neg) in &cols {
                    let pt = if neg {
                        g.neg(inst.reps[i])
                    } else {
                        inst.reps[i]
                    };
                    s = g.add(&mut ops, s, pt);
                }
                let (a, b) = if s == rp {
                    (la[l], lb[l])
                } else if s == g.neg(rp) {
                    (subm(0, la[l], r), subm(0, lb[l], r))
                } else {
                    st.rejected += 1;
                    continue;
                };
                st.verified += 1;
                let mut guard = sink.lock().unwrap();
                if guard.0.len() < need && guard.1.insert(v ^ a.rotate_left(17) ^ b.rotate_left(41))
                {
                    guard.0.push(Rel { cols, a, b });
                    if guard.0.len() >= need {
                        done.store(true, Ordering::Relaxed);
                    }
                }
            }
            for (l, v, oi, neg) in found3.drain(..) {
                st.hits += 1;
                // R ± T_i: (a, b) = (la ± t_i, lb).
                let rcur = PrimePoint::affine(f.from(x[l]), f.from(y[l]));
                let tp = if neg { g.neg(offs[oi].0) } else { offs[oi].0 };
                let rp = g.add(&mut ops, rcur, tp);
                let pa = if neg {
                    subm(la[l], offs[oi].1, r)
                } else {
                    addm(la[l], offs[oi].1, r)
                };
                let pb = lb[l];
                let cols = unpack(v, m);
                let mut s5 = PrimePoint {
                    x: 0,
                    y: 0,
                    infinity: true,
                };
                for &(i, ng) in &cols {
                    let pt = if ng {
                        g.neg(inst.reps[i])
                    } else {
                        inst.reps[i]
                    };
                    s5 = g.add(&mut ops, s5, pt);
                }
                let (a, b) = if s5 == rp {
                    (pa, pb)
                } else if s5 == g.neg(rp) {
                    (subm(0, pa, r), subm(0, pb, r))
                } else {
                    st.rejected += 1;
                    continue;
                };
                st.verified += 1;
                let mut guard = sink.lock().unwrap();
                if guard.0.len() < need && guard.1.insert(v ^ a.rotate_left(17) ^ b.rotate_left(41))
                {
                    guard.0.push(Rel { cols, a, b });
                    if guard.0.len() >= need {
                        done.store(true, Ordering::Relaxed);
                    }
                }
            }
            exc.clear();
            if sib {
                xprev.copy_from_slice(&x);
                yprev.copy_from_slice(&y);
            }
            #[cfg(target_arch = "x86_64")]
            if intr {
                // SAFETY: `intr` is set only when AVX-512 was detected.
                unsafe {
                    avx::step(
                        f, &mut x, &mut y, &mut la, &mut lb, &wx, &wy, &wc, &wd, r, &mut j64,
                        &mut dd, &mut pre, &mut zmask, &mut exc, &mut xs,
                    )
                };
                for &l in &exc {
                    j[l] = j64[l] as usize;
                }
            } else {
                step_any(
                    simd, f, &mut x, &mut y, &mut la, &mut lb, &wx, &wy, &wc, &wd, r, &mut j,
                    &mut dd, &mut pre, &mut zero, &mut exc,
                );
            }
            #[cfg(not(target_arch = "x86_64"))]
            step_any(
                simd, f, &mut x, &mut y, &mut la, &mut lb, &wx, &wy, &wc, &wd, r, &mut j, &mut dd,
                &mut pre, &mut zero, &mut exc,
            );
            for &l in &exc {
                // R = ±W_j: step with the library instead.
                st.exceptional += 1;
                let jj = j[l];
                let rp = PrimePoint::affine(f.from(x[l]), f.from(y[l]));
                let (w, c, dco) = steps_w[(jj + 1) % NW];
                let np = g.add(&mut ops, rp, w);
                x[l] = f.to(np.x);
                y[l] = f.to(np.y);
                la[l] = addm(la[l], c, r);
                lb[l] = addm(lb[l], dco, r);
            }
            #[cfg(target_arch = "x86_64")]
            if sib && filt.is_none() && probe {
                let tbl = table.unwrap();
                cand.clear();
                // SAFETY: `sib` implies AVX-512 was detected.
                unsafe {
                    avx::bloom_pass(
                        &tbl.bloom.words,
                        tbl.bloom.mask,
                        tbl.bloom.wshift,
                        &xs,
                        &mut cand,
                    )
                };
                for &l in &cand {
                    if xs[l] != u64::MAX {
                        if let Some(v) = tbl.get_exact(xs[l]) {
                            found2.push((l, v));
                        }
                    }
                }
                st.steps += lanes as u64;
                for (l, v) in found2.drain(..) {
                    st.hits += 1;
                    // R_prev − W_j, with (a, b) = (la − 2 c_j, lb − 2).
                    let jj = j64[l] as usize;
                    let rprev = PrimePoint::affine(f.from(xprev[l]), f.from(yprev[l]));
                    let rp = g.add(&mut ops, rprev, g.neg(steps_w[jj].0));
                    let (pa, pb) = (subm(la[l], addm(wc[jj], wc[jj], r), r), subm(lb[l], 2, r));
                    let cols = unpack(v, m);
                    let mut s5 = PrimePoint {
                        x: 0,
                        y: 0,
                        infinity: true,
                    };
                    for &(i, neg) in &cols {
                        let pt = if neg {
                            g.neg(inst.reps[i])
                        } else {
                            inst.reps[i]
                        };
                        s5 = g.add(&mut ops, s5, pt);
                    }
                    let (a, b) = if s5 == rp {
                        (pa, pb)
                    } else if s5 == g.neg(rp) {
                        (subm(0, pa, r), subm(0, pb, r))
                    } else {
                        st.rejected += 1;
                        continue;
                    };
                    st.verified += 1;
                    let mut guard = sink.lock().unwrap();
                    if guard.0.len() < need
                        && guard.1.insert(v ^ a.rotate_left(17) ^ b.rotate_left(41))
                    {
                        guard.0.push(Rel { cols, a, b });
                        if guard.0.len() >= need {
                            done.store(true, Ordering::Relaxed);
                        }
                    }
                }
            }
        }
    }
    st
}

/// One complete solve in join mode: levels, the two filters, the walk to
/// a candidate budget, the join, verification, linear algebra; more walking
/// rounds if the verified relations or the kernel fall short.
#[cfg(target_arch = "x86_64")]
#[allow(clippy::too_many_arguments)]
fn solve_join<G: CountedGroup<Elt = PrimePoint> + Sync>(
    inst: &Instance<G>,
    g: &G,
    gen: PrimePoint,
    q: PrimePoint,
    known: Option<u64>,
    m: u32,
    lanes: usize,
    threads: usize,
    extra: usize,
    ti: usize,
    rep: usize,
    bits: u32,
    name: &str,
) -> serde_json::Value {
    let (f, r, a) = (&inst.f, inst.r, inst.bx.len());
    let t0 = Instant::now();
    let (lv, invs, skipped) = build_levels_avx(f, &inst.bx, &inst.by, m);
    let t_levels = t0.elapsed().as_secs_f64();
    let (b1, b2, off, n, sk2) = filters_par(f, &lv, &inst.bx, &inst.by, threads, g_p(g));
    let t_build = t0.elapsed().as_secs_f64();
    let mut wops = GroupOps::default();
    let mut sstate = 0x2545_F491_4F6C_DD1Du64 ^ (ti as u64) << 20 ^ rep as u64;
    let mut rnd = || {
        sstate ^= sstate << 13;
        sstate ^= sstate >> 7;
        sstate ^= sstate << 17;
        sstate
    };
    let steps_w = make_steps(g, &mut wops, gen, q, r);
    // Candidate budget from the filters' fill: a walk point is a table
    // hit with probability about E / (r / 2) and passes both filters
    // falsely with about (fill1 · fill2)^4.
    let fill =
        |w: &[u64]| w.iter().map(|x| x.count_ones() as f64).sum::<f64>() / (64 * w.len()) as f64;
    let (f1, f2) = (fill(&b1.words), fill(&b2.words));
    let entries = 2 * n as u64 - 2 * sk2;
    let p_true = entries as f64 / (r as f64 / 2.0);
    let fpr = (f1 * f2).powi(4);
    let mut q_est = p_true / (p_true + fpr);
    let mut need = a + extra;
    let sink = Mutex::new((Vec::<Rel>::new(), HashSet::<u64>::new()));
    let cands = Mutex::new(Vec::<Cand>::new());
    let mut joined = 0usize;
    let (mut t_walk, mut t_join, mut t_la) = (0f64, 0f64, 0f64);
    let (mut rounds, mut matches_total, mut rejected) = (0u32, 0usize, 0u64);
    let mut stats = WalkStats::default();
    let mut ops = GroupOps::default();
    let x = loop {
        rounds += 1;
        let have = sink.lock().unwrap().0.len();
        let want = need.saturating_sub(have) + 2;
        let cap = joined + ((want as f64 / q_est) * 1.25).ceil() as usize;
        let done = AtomicBool::new(false);
        let tw = Instant::now();
        let round: Vec<WalkStats> = {
            let jobs: Vec<_> = (0..threads)
                .map(|th| {
                    let (steps_w, done, sink, cands, b1, b2) =
                        (&steps_w, &done, &sink, &cands, &b1, &b2);
                    let sd = rnd() ^ (th as u64) << 32;
                    move || {
                        walk_thread(
                            inst,
                            None,
                            m,
                            q,
                            steps_w,
                            lanes,
                            sd,
                            done,
                            sink,
                            usize::MAX,
                            Some((b1, b2, cands, cap)),
                        )
                    }
                })
                .collect();
            par_run(jobs)
        };
        t_walk += tw.elapsed().as_secs_f64();
        for s in round {
            stats.steps += s.steps;
            stats.filter_pass += s.filter_pass;
        }
        // Join the new candidates.
        let tj = Instant::now();
        let all = cands.lock().unwrap().clone();
        let fresh = &all[joined..];
        let found = join_par(f, &lv, &off, &inst.bx, &inst.by, threads, fresh);
        matches_total += found.len();
        let mut true_hits = 0;
        for (ci, rf) in found {
            let c = fresh[ci];
            let t = (rf / 2) as usize;
            let e = off.partition_point(|&o| o as usize <= t) - 1;
            let k = lv.llast[e] as usize + 1 + (t - off[e] as usize);
            let v = pack_push(lv.lpk[e], lv.depth, k, rf & 1 == 1);
            let rp = PrimePoint::affine(f.from(c.x), f.from(c.y));
            let cols = unpack(v, m);
            let mut sum = PrimePoint {
                x: 0,
                y: 0,
                infinity: true,
            };
            for &(i, neg) in &cols {
                let pt = if neg {
                    g.neg(inst.reps[i])
                } else {
                    inst.reps[i]
                };
                sum = g.add(&mut ops, sum, pt);
            }
            let (ra, rb) = if sum == rp {
                (c.a, c.b)
            } else if sum == g.neg(rp) {
                (subm(0, c.a, r), subm(0, c.b, r))
            } else {
                rejected += 1;
                continue;
            };
            true_hits += 1;
            let mut guard = sink.lock().unwrap();
            if guard.1.insert(v ^ ra.rotate_left(17) ^ rb.rotate_left(41)) {
                guard.0.push(Rel { cols, a: ra, b: rb });
            }
        }
        if !fresh.is_empty() {
            // Re-estimate the true fraction from what this round saw.
            q_est = ((true_hits as f64 + 1.0) / (fresh.len() as f64 + 1.0)).max(1e-4);
        }
        joined = all.len();
        t_join += tj.elapsed().as_secs_f64();
        let tl = Instant::now();
        let have = sink.lock().unwrap().0.len();
        if have >= need {
            let x = solve(&sink.lock().unwrap().0, a, r);
            t_la += tl.elapsed().as_secs_f64();
            if x.is_some() || rounds >= 16 {
                break x;
            }
            need += 2;
        } else if rounds >= 16 {
            break None;
        }
    };
    let total = t0.elapsed().as_secs_f64();
    let check = x.map(|x| g.mul(&mut wops, gen, x) == q).unwrap_or(false);
    let rels = sink.into_inner().unwrap().0.len();
    json!({
        "schema": "rootfind.ksum-fast/v1",
        "mode": "join",
        "isolation": "none",
        "curve": name,
        "bits": bits,
        "m": m,
        "abscissae": a,
        "lanes": lanes,
        "threads": threads,
        "rep": rep,
        "target_index": ti,
        "table_entries": entries,
        "table_inversions": invs,
        "table_exceptional_skipped": skipped + sk2,
        "filter_fill": [f1, f2],
        "candidates": joined,
        "join_matches": matches_total,
        "hits_rejected": rejected,
        "relations": rels,
        "rounds": rounds,
        "walk_steps": stats.steps,
        "solved": x.is_some(),
        "log": x,
        "log_matches_known": known.map(|k| Some(k) == x),
        "x_g_equals_q": check,
        "levels_ms": t_levels * 1e3,
        "build_ms": t_build * 1e3,
        "walk_ms": t_walk * 1e3,
        "join_ms": t_join * 1e3,
        "la_ms": t_la * 1e3,
        "solve_ms": total * 1e3,
    })
}

/// The field prime of a prime-curve group (for the filters' key width).
fn g_p<G: CountedGroup<Elt = PrimePoint>>(_g: &G) -> u64 {
    G_P.with(|p| p.get())
}

thread_local! {
    static G_P: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// The r-adding walk's steps `W_j = Q + 2^j G` (`c_j = 2^j mod r`,
/// `d_j = 1`): one doubling chain and 32 additions instead of 64 scalar
/// multiplications; every step still has a known decomposition.
fn make_steps<G: CountedGroup<Elt = PrimePoint>>(
    g: &G,
    ops: &mut GroupOps,
    gen: PrimePoint,
    q: PrimePoint,
    r: u64,
) -> Vec<(PrimePoint, u64, u64)> {
    let mut p = gen;
    let mut c = 1u64 % r;
    (0..NW)
        .map(|_| {
            let w = g.add(ops, q, p);
            let out = (w, c, 1u64);
            p = g.add(ops, p, p);
            c = addm(c, c, r);
            out
        })
        .collect()
}

fn hexval(s: &str) -> u64 {
    u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).expect("hex")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let bits: u32 = get("--bits", "32").parse().unwrap();
    let seed: u64 = get("--seed", "20261018202").parse().unwrap();
    let size: usize = get("--size", "19").parse().unwrap();
    let m: u32 = get("--m", "5").parse().unwrap();
    let lanes: usize = get("--lanes", "256").parse().unwrap();
    let threads: usize = get("--threads", "1").parse().unwrap();
    let reps: usize = get("--reps", "1").parse().unwrap();
    let extra: usize = get("--extra", "1").parse().unwrap();
    let mode = get("--mode", "table");
    let amortize = get("--amortize", "0") == "1";
    let targets_file = get("--targets", "");
    SpinPool::start(threads);
    let targets: Vec<(PrimePoint, Option<u64>)> = std::fs::read_to_string(&targets_file)
        .expect("--targets FILE: lines 'x y [log]' in hex/hex/decimal")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let w: Vec<&str> = l.split_whitespace().collect();
            (
                PrimePoint::affine(hexval(w[0]), hexval(w[1])),
                w.get(2).map(|s| s.parse().unwrap()),
            )
        })
        .collect();

    let t_setup = Instant::now();
    let inst0 = find_prime_order_curve(bits, seed);
    let g = &inst0.curve;
    let f = Mont::new(g.p);
    let gen = inst0.generator_point();
    let ctx = InstanceCtx {
        group: g,
        generator: gen,
        target: gen,
        r: inst0.r,
        cofactor: inst0.cofactor,
        group_order: inst0.group_order,
        name: inst0.name.clone(),
        field_degree: None,
    };
    let mut ops = GroupOps::default();
    let (_, bparams) = Params::parse_spec(&format!("prime-abscissa:size={size}")).unwrap();
    let fb = (PrimeAbscissaBase { instance: &inst0 })
        .build(&ctx, &bparams, &mut ops)
        .expect("the base builds");
    let reps_pts: Vec<PrimePoint> = fb
        .abscissa_list
        .iter()
        .map(|x| fb.points[fb.x_index[x][0]])
        .filter(|p| !p.infinity && p.y != 0)
        .collect();
    let a = reps_pts.len();
    assert!(a <= 1023);
    let inst = Instance {
        g,
        f,
        r: inst0.r,
        gen,
        bx: reps_pts.iter().map(|p| f.to(p.x)).collect(),
        by: reps_pts.iter().map(|p| f.to(p.y)).collect(),
        reps: reps_pts,
    };
    let setup_s = t_setup.elapsed().as_secs_f64();
    G_P.with(|c| c.set(g.p));
    #[cfg(target_arch = "x86_64")]
    if std::env::var_os("KSUM_CHECKBUILD").is_some() && use_simd() {
        // The AVX-512 builder must produce exactly the scalar builder's
        // (key, witness) multiset.
        let (mut s1, i1, k1) = build_ksum(&f, &inst.bx, &inst.by, m);
        let (t2, i2, k2) = build_ksum_avx(&f, &inst.bx, &inst.by, m, threads, g.p);
        let mut s2: Vec<(u32, u64)> = t2
            .keys
            .iter()
            .zip(&t2.refs)
            .map(|(&k, &r)| (k, t2.resolve(r)))
            .collect();
        s1.sort_unstable();
        s2.sort_unstable();
        eprintln!(
            "checkbuild: scalar {} entries ({} skipped, {} inversions), avx {} ({} skipped, {} inversions), identical: {}",
            s1.len(), k1, i1, s2.len(), k2, i2, s1 == s2
        );
        assert_eq!(s1, s2, "the two builders disagree");
        return;
    }
    if std::env::var_os("KSUM_PARBENCH").is_some() {
        // Round-trip cost of one par_run of `threads` trivial jobs, back to
        // back and after a 1 ms pause (worker threads asleep).
        for pause in [false, true] {
            let mut tot = 0f64;
            let iters = 200;
            for _ in 0..iters {
                if pause {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                let t = Instant::now();
                let jobs: Vec<_> = (0..threads)
                    .map(|i| move || std::hint::black_box(i * 2))
                    .collect();
                let v = par_run(jobs);
                std::hint::black_box(v);
                tot += t.elapsed().as_secs_f64();
            }
            eprintln!(
                "par_run x{threads}, pause {pause}: {:.1} us",
                tot * 1e6 / iters as f64
            );
        }
        return;
    }
    if std::env::var_os("KSUM_BENCH").is_some() {
        // The step kernel alone on synthetic lanes (base points as walk
        // steps, base points as starts), for profiling.
        let lanes = 256;
        let wx: Vec<u64> = (0..NW).map(|i| inst.bx[i % a]).collect();
        let wy: Vec<u64> = (0..NW).map(|i| inst.by[i % a]).collect();
        let wc = vec![1u64; NW];
        let wd = vec![1u64; NW];
        let start = g.add(&mut ops, inst.reps[0], inst.reps[1]);
        let mut cur = start;
        let mut x = vec![0u64; lanes];
        let mut y = vec![0u64; lanes];
        for l in 0..lanes {
            cur = g.add(&mut ops, cur, inst.reps[(l + 2) % a]);
            x[l] = f.to(cur.x);
            y[l] = f.to(cur.y);
        }
        let (mut la, mut lb) = (vec![0u64; lanes], vec![0u64; lanes]);
        let (mut j, mut dd, mut pre, mut zero, mut exc) = (
            vec![0usize; lanes],
            vec![0u64; lanes],
            vec![0u64; lanes],
            vec![false; lanes],
            Vec::new(),
        );
        for simd in [false, use_simd()] {
            let t = Instant::now();
            let iters = 2000;
            for _ in 0..iters {
                exc.clear();
                step_any(
                    simd, &f, &mut x, &mut y, &mut la, &mut lb, &wx, &wy, &wc, &wd, inst.r, &mut j,
                    &mut dd, &mut pre, &mut zero, &mut exc,
                );
            }
            let ns = t.elapsed().as_secs_f64() * 1e9 / (iters * lanes) as f64;
            eprintln!(
                "step kernel simd={simd}: {ns:.2} ns per lane-step (exc {})",
                exc.len()
            );
        }
        #[cfg(target_arch = "x86_64")]
        if use_simd() {
            let mut j64 = vec![0u64; lanes];
            let mut zm = vec![0u8; lanes / 8];
            let t = Instant::now();
            let iters = 2000;
            for _ in 0..iters {
                exc.clear();
                // SAFETY: AVX-512 detected.
                unsafe {
                    avx::step(
                        &f,
                        &mut x,
                        &mut y,
                        &mut la,
                        &mut lb,
                        &wx,
                        &wy,
                        &wc,
                        &wd,
                        inst.r,
                        &mut j64,
                        &mut dd,
                        &mut pre,
                        &mut zm,
                        &mut exc,
                        &mut vec![0u64; lanes],
                    )
                };
            }
            let ns = t.elapsed().as_secs_f64() * 1e9 / (iters * lanes) as f64;
            eprintln!(
                "step kernel intrinsics: {ns:.2} ns per lane-step (exc {})",
                exc.len()
            );
        }
        return;
    }
    eprintln!(
        "curve {} p={} r={} base {} abscissae",
        inst0.name, g.p, inst.r, a
    );

    #[cfg(target_arch = "x86_64")]
    if mode == "join" {
        assert!(use_simd(), "join mode needs AVX-512");
        for rep in 0..reps {
            for (ti, &(q, known)) in targets.iter().enumerate() {
                let row = solve_join(
                    &inst,
                    g,
                    gen,
                    q,
                    known,
                    m,
                    lanes,
                    threads,
                    extra,
                    ti,
                    rep,
                    bits,
                    &inst0.name,
                );
                println!("{row}");
            }
        }
        return;
    }
    for rep in 0..reps {
        // `--amortize 1`: one table per curve, reused by every target (a
        // multi-target run); reported separately, never as the per-solve
        // number, which charges the build to every solve as the pipeline does.
        let avx_build =
            cfg!(target_arch = "x86_64") && use_simd() && std::env::var_os("KSUM_NOINTR").is_none();
        let shared: Option<(Table, f64)> = if amortize {
            let tb = Instant::now();
            #[cfg(target_arch = "x86_64")]
            let t = build_ksum_avx(&inst.f, &inst.bx, &inst.by, m, threads, g.p).0;
            #[cfg(not(target_arch = "x86_64"))]
            let t = Table::build(build_ksum(&inst.f, &inst.bx, &inst.by, m).0, g.p);
            let ms = tb.elapsed().as_secs_f64() * 1e3;
            eprintln!(
                "amortized table: {} entries in {:.3} ms (once per curve)",
                t.len, ms
            );
            Some((t, ms))
        } else {
            None
        };
        for (ti, &(q, known)) in targets.iter().enumerate() {
            let t0 = Instant::now();
            // Table build (charged to every solve, as the pipeline does).
            let owned;
            let (table, invs, skipped, sort_ms) = if let Some((t, _)) = &shared {
                (t, 0, 0, 0.0)
            } else if avx_build {
                #[cfg(target_arch = "x86_64")]
                {
                    let (table, invs, skipped) =
                        build_ksum_avx(&inst.f, &inst.bx, &inst.by, m, threads, g.p);
                    owned = table;
                    (&owned, invs, skipped, 0.0)
                }
                #[cfg(not(target_arch = "x86_64"))]
                unreachable!()
            } else {
                let (kv, invs, skipped) = build_ksum(&inst.f, &inst.bx, &inst.by, m);
                let t_sort = Instant::now();
                owned = Table::build(kv, g.p);
                (&owned, invs, skipped, t_sort.elapsed().as_secs_f64() * 1e3)
            };
            let raw = table.keys.len();
            let collisions =
                table
                    .collisions
                    .max(if std::env::var_os("KSUM_COLLISIONS").is_some() {
                        table.count_collisions()
                    } else {
                        0
                    });
            let t_build = t0.elapsed().as_secs_f64();
            // Walk steps W_j = c_j G + d_j Q (charged too).
            let mut wops = GroupOps::default();
            let mut sstate = 0x2545_F491_4F6C_DD1Du64 ^ (ti as u64) << 20 ^ rep as u64;
            let mut rnd = || {
                sstate ^= sstate << 13;
                sstate ^= sstate >> 7;
                sstate ^= sstate << 17;
                sstate
            };
            let steps_w = make_steps(g, &mut wops, gen, q, inst.r);
            // Collect A + extra relations (the library stops at its first
            // usable kernel vector, about A + 1); if the kernel gives no
            // usable vector, walk on for two more and try again.
            let mut need = a + extra;
            let sink = Mutex::new((Vec::<Rel>::new(), HashSet::<u64>::new()));
            let mut stats: Vec<WalkStats> = Vec::new();
            let (mut t_walk, mut t_la, mut la_rounds) = (0f64, 0f64, 0u32);
            let x = loop {
                let done = AtomicBool::new(false);
                let t_walk0 = Instant::now();
                let round: Vec<WalkStats> = {
                    let jobs: Vec<_> = (0..threads)
                        .map(|th| {
                            let (inst, table, steps_w, done, sink) =
                                (&inst, table, &steps_w, &done, &sink);
                            let sd = rnd() ^ (th as u64) << 32;
                            move || {
                                walk_thread(
                                    inst,
                                    Some(table),
                                    m,
                                    q,
                                    steps_w,
                                    lanes,
                                    sd,
                                    done,
                                    sink,
                                    need,
                                    None,
                                )
                            }
                        })
                        .collect();
                    par_run(jobs)
                };
                t_walk += t_walk0.elapsed().as_secs_f64();
                stats.extend(round);
                let t_la0 = Instant::now();
                let x = solve(&sink.lock().unwrap().0, a, inst.r);
                t_la += t_la0.elapsed().as_secs_f64();
                la_rounds += 1;
                if x.is_some() || la_rounds >= 8 {
                    break x;
                }
                need += 2;
            };
            let rels = sink.into_inner().unwrap().0;
            let total = t0.elapsed().as_secs_f64();
            let check = x.map(|x| g.mul(&mut wops, gen, x) == q).unwrap_or(false);
            let sum = stats.iter().fold(WalkStats::default(), |acc, s| WalkStats {
                steps: acc.steps + s.steps,
                filter_pass: acc.filter_pass + s.filter_pass,
                hits: acc.hits + s.hits,
                verified: acc.verified + s.verified,
                rejected: acc.rejected + s.rejected,
                exceptional: acc.exceptional + s.exceptional,
            });
            println!(
                "{}",
                json!({
                    "schema": "rootfind.ksum-fast/v1",
                    "isolation": "none",
                    "curve": inst0.name,
                    "bits": bits,
                    "m": m,
                    "abscissae": a,
                    "lanes": lanes,
                    "threads": threads,
                    "rep": rep,
                    "target_index": ti,
                    "table_raw": raw,
                    "table_entries": table.len,
                    "table_collisions_counted": collisions,
                    "table_inversions": invs,
                    "table_exceptional_skipped": skipped,
                    "relations": rels.len(),
                    "la_rounds": la_rounds,
                    "walk_steps": sum.steps,
                    "hits": sum.hits,
                    "hits_verified": sum.verified,
                    "hits_rejected": sum.rejected,
                    "walk_exceptional": sum.exceptional,
                    "solved": x.is_some(),
                    "log": x,
                    "log_matches_known": known.map(|k| Some(k) == x),
                    "x_g_equals_q": check,
                    "build_ms": t_build * 1e3,
                    "build_sort_ms": sort_ms,
                    "amortized_table": amortize,
                    "amortized_build_ms_once": shared.as_ref().map(|s| s.1),
                    "walk_ms": t_walk * 1e3,
                    "la_ms": t_la * 1e3,
                    "solve_ms": total * 1e3,
                    "ns_per_step": t_walk * 1e9 / sum.steps.max(1) as f64 * threads as f64,
                    "setup_once_ms": setup_s * 1e3,
                })
            );
        }
    }
}
