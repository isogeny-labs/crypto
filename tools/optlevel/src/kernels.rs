//! Calibration and control kernels (the Rust port of `kernels/optk.c`).
//!
//! One process, one thread, one kernel: `optlevel kernel NAME [--isa I] [--n N]
//! [--seconds S]`.  The kernel repeats a pass over its arrays until S seconds
//! have elapsed and prints one `optlevel.ops/v1` line.  Each pass exists once
//! per instruction-set variant ("isa"), selected at run time, so one portable
//! binary carries the scalar and the SIMD ceilings of whatever host it runs on:
//!
//! * `scalar`  : the host's scalar instructions (BMI2, POPCNT, FMA where present),
//!   vectorization blocked by an empty asm barrier on each element;
//! * `avx2`    : 256-bit autovectorized loops;
//! * `avx512`  : 512-bit (AVX-512 F/DQ/BW/VL/VPOPCNTDQ) loops;
//! * `base`    : the target's baseline features, autovectorized (any architecture).
//!
//! Symbols are exported as `optk_<isa>_pass_<kernel>` so that
//! `optlevel vector-share` can disassemble them by name.
//!
//! Unlike optk.c the timer is not read after every pass: the number of passes
//! per timer read is chosen so that reads are about 50 microseconds apart
//! (`--timer-every-pass` restores the C behaviour for A/B runs).
use std::time::Instant;

pub const P31: u32 = 2013265921; // 15 * 2^27 + 1
pub const P31_NINV: u32 = 2013265919; // -P31^-1 mod 2^32
pub const P62: u64 = 4611686018427387847; // 2^62 - 57

pub const KERNELS: &[&str] = &[
    "muladd_u32", "muladd_u64", "mulwide_u64", "modmuladd_p31", "modmuladd_p62", "horner_p31",
    "xor_u64", "popcnt_u64", "fma_f64", "fma_f64_reg", "triad", "chase", "clock",
];

pub fn p62_ninv() -> u64 {
    let mut x: u64 = 1; // Newton: P62^-1 mod 2^64, then negate
    for _ in 0..6 {
        x = x.wrapping_mul(2u64.wrapping_sub(P62.wrapping_mul(x)));
    }
    0u64.wrapping_sub(x)
}

pub struct Rng(pub u64);
impl Rng {
    pub fn new() -> Self {
        Rng(0x9E3779B97F4A7C15)
    }
    pub fn next(&mut self) -> u64 {
        // splitmix64, the same stream as optk.c
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// A zero-initialised, page-aligned array (optk.c uses posix_memalign(4096)):
/// 64-byte vector loads and stores never straddle a cache line.
pub struct Buf<T> {
    ptr: *mut T,
    len: usize,
    layout: std::alloc::Layout,
}
impl<T: Copy> Buf<T> {
    pub fn zeroed(len: usize) -> Self {
        let layout = std::alloc::Layout::from_size_align((len * std::mem::size_of::<T>()).max(4096), 4096).expect("layout");
        // SAFETY: the layout has non-zero size; all-zero bytes are a valid T for the integer, float and Node types used here.
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) } as *mut T;
        if ptr.is_null() {
            eprintln!("optlevel kernel: out of memory");
            std::process::exit(2);
        }
        Buf { ptr, len, layout }
    }
}
impl<T> std::ops::Deref for Buf<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        // SAFETY: ptr points at len initialised (zeroed) elements owned by self.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}
impl<T> std::ops::DerefMut for Buf<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: as above, and self is borrowed mutably.
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
impl<T> Drop for Buf<T> {
    fn drop(&mut self) {
        // SAFETY: allocated in zeroed() with this layout.
        unsafe { std::alloc::dealloc(self.ptr as *mut u8, self.layout) }
    }
}

#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct Node {
    pub next: *const Node,
    pub pad: [u64; 7],
}

/// The loop bodies, generic over S (block vectorization with an asm barrier)
/// and F (fuse multiply-add; only where the variant enables FMA).
mod body {
    use super::{Node, P31, P31_NINV, P62};

    #[inline(always)]
    pub fn bar32<const S: bool>(v: u32) -> u32 {
        #[cfg(target_arch = "x86_64")]
        if S {
            let mut x = v;
            unsafe { core::arch::asm!("/* {0:e} */", inout(reg) x, options(pure, nomem, nostack, preserves_flags)) };
            return x;
        }
        #[cfg(not(target_arch = "x86_64"))]
        if S {
            return core::hint::black_box(v);
        }
        v
    }
    #[inline(always)]
    pub fn bar64<const S: bool>(v: u64) -> u64 {
        #[cfg(target_arch = "x86_64")]
        if S {
            let mut x = v;
            unsafe { core::arch::asm!("/* {0} */", inout(reg) x, options(pure, nomem, nostack, preserves_flags)) };
            return x;
        }
        #[cfg(not(target_arch = "x86_64"))]
        if S {
            return core::hint::black_box(v);
        }
        v
    }
    #[inline(always)]
    pub fn barf<const S: bool>(v: f64) -> f64 {
        #[cfg(target_arch = "x86_64")]
        if S {
            let mut x = v;
            unsafe { core::arch::asm!("/* {0} */", inout(xmm_reg) x, options(pure, nomem, nostack, preserves_flags)) };
            return x;
        }
        #[cfg(not(target_arch = "x86_64"))]
        if S {
            return core::hint::black_box(v);
        }
        v
    }
    #[inline(always)]
    pub fn mont31(a: u32, b: u32) -> u32 {
        let t = a as u64 * b as u64;
        let q = (t as u32).wrapping_mul(P31_NINV);
        let r = ((t + q as u64 * P31 as u64) >> 32) as u32;
        reduce32(r)
    }
    /// r mod p for r < 2p: r - p wraps above r exactly when r < p, so the
    /// smaller of the two is the residue (one subtract and one unsigned min;
    /// vpminud in the vector loops, sub + cmovb in the scalar ones).
    #[inline(always)]
    pub fn reduce32(r: u32) -> u32 {
        r.min(r.wrapping_sub(P31))
    }
    #[inline(always)]
    pub fn reduce64(r: u64) -> u64 {
        r.min(r.wrapping_sub(P62))
    }
    #[inline(always)]
    pub fn add31(a: u32, b: u32) -> u32 {
        reduce32(a.wrapping_add(b))
    }
    #[inline(always)]
    pub fn mont62(a: u64, b: u64, ninv: u64) -> u64 {
        let t = a as u128 * b as u128;
        let q = (t as u64).wrapping_mul(ninv);
        let r = ((t + q as u128 * P62 as u128) >> 64) as u64;
        reduce64(r)
    }
    #[inline(always)]
    pub fn fma<const F: bool>(a: f64, s: f64, b: f64) -> f64 {
        if F { a.mul_add(s, b) } else { a * s + b }
    }

    #[inline(always)]
    pub fn muladd_u32<const S: bool>(a: &mut [u32], c: &[u32], m: u32) {
        for (x, &y) in a.iter_mut().zip(c) {
            *x = bar32::<S>(x.wrapping_mul(m).wrapping_add(y));
        }
    }
    #[inline(always)]
    pub fn muladd_u64<const S: bool>(a: &mut [u64], c: &[u64], m: u64) {
        for (x, &y) in a.iter_mut().zip(c) {
            *x = bar64::<S>(x.wrapping_mul(m).wrapping_add(y));
        }
    }
    #[inline(always)]
    pub fn mulwide_u64<const S: bool>(a: &mut [u64], c: &[u64], m: u64) {
        for (x, &y) in a.iter_mut().zip(c) {
            *x = bar64::<S>((((*x as u128) * (m as u128)) >> 64) as u64).wrapping_add(y);
        }
    }
    #[inline(always)]
    pub fn modmuladd_p31<const S: bool>(a: &mut [u32], b: &[u32], m: u32) {
        for (x, &y) in a.iter_mut().zip(b) {
            *x = bar32::<S>(add31(*x, mont31(y, m)));
        }
    }
    #[inline(always)]
    pub fn modmuladd_p62<const S: bool>(a: &mut [u64], b: &[u64], m: u64, ninv: u64) {
        for (x, &y) in a.iter_mut().zip(b) {
            *x = bar64::<S>(reduce64(*x + mont62(y, m, ninv)));
        }
    }
    #[inline(always)]
    pub fn horner_p31(b: &[u32], m: u32, mut acc: u32) -> u32 {
        for &y in b {
            acc = add31(mont31(acc, m), y);
        }
        acc
    }
    #[inline(always)]
    pub fn xor_u64<const S: bool>(a: &mut [u64], b: &[u64]) {
        for (x, &y) in a.iter_mut().zip(b) {
            *x ^= bar64::<S>(y);
        }
    }
    #[inline(always)]
    pub fn popcnt_u64<const S: bool>(a: &[u64], b: &[u64]) -> u64 {
        let mut s = 0u64;
        for (&x, &y) in a.iter().zip(b) {
            s += bar64::<S>((x & y).count_ones() as u64);
        }
        s
    }
    #[inline(always)]
    pub fn fma_f64<const S: bool, const F: bool>(a: &mut [f64], b: &[f64], s: f64) {
        for (x, &y) in a.iter_mut().zip(b) {
            *x = barf::<S>(fma::<F>(*x, s, y));
        }
    }
    #[inline(always)]
    pub fn triad<const S: bool, const F: bool>(a: &mut [f64], b: &[f64], c: &[f64], s: f64) {
        for ((x, &y), &z) in a.iter_mut().zip(b).zip(c) {
            *x = barf::<S>(fma::<F>(s, z, y));
        }
    }
    /// Register-resident multiply-add, portable form: A accumulators of L lanes.
    #[inline(always)]
    pub fn fma_reg<const F: bool, const A: usize, const L: usize>(n: usize, s0: f64) -> f64 {
        let mut acc = [[0f64; L]; A];
        let mut c = [0f64; L];
        for j in 0..L {
            c[j] = 1.0 + j as f64;
        }
        for i in 0..A {
            for j in 0..L {
                acc[i][j] = (i + j) as f64;
            }
        }
        let mut it = 0u64;
        let iters = (n / (A * L)) as u64;
        while it < iters {
            for i in 0..A {
                for j in 0..L {
                    acc[i][j] = fma::<F>(acc[i][j], s0, c[j]);
                }
            }
            it = bar64::<true>(it) + 1;
        }
        let mut r = 0.0;
        for i in 0..A {
            for j in 0..L {
                r += acc[i][j];
            }
        }
        r
    }
    #[inline(always)]
    pub fn chase(mut p: *const Node, n: usize) -> *const Node {
        for _ in 0..n {
            p = unsafe { (*p).next };
        }
        p
    }
    #[inline(always)]
    pub fn clock(mut x: u64, n: usize) -> u64 {
        for _ in 0..n {
            x = bar64::<true>(x + 1);
        }
        x
    }
}

/// One table of passes per isa.
#[derive(Clone, Copy)]
pub struct Passes {
    pub muladd_u32: unsafe fn(&mut [u32], &[u32], u32),
    pub muladd_u64: unsafe fn(&mut [u64], &[u64], u64),
    pub mulwide_u64: unsafe fn(&mut [u64], &[u64], u64),
    pub modmuladd_p31: unsafe fn(&mut [u32], &[u32], u32),
    pub modmuladd_p62: unsafe fn(&mut [u64], &[u64], u64, u64),
    pub horner_p31: unsafe fn(&[u32], u32, u32) -> u32,
    pub xor_u64: unsafe fn(&mut [u64], &[u64]),
    pub popcnt_u64: unsafe fn(&[u64], &[u64]) -> u64,
    pub fma_f64: unsafe fn(&mut [f64], &[f64], f64),
    pub triad: unsafe fn(&mut [f64], &[f64], &[f64], f64),
    pub fma_reg: unsafe fn(usize, f64) -> f64,
    /// scalar multiply-adds per iteration of fma_reg (n is rounded down to a multiple)
    pub fma_reg_unit: usize,
    pub chase: unsafe fn(*const Node, usize) -> *const Node,
    pub clock: unsafe fn(u64, usize) -> u64,
}

macro_rules! isa_mod {
    ($m:ident, $s:literal, $f:literal, $acc:literal, $lanes:literal, $reg:path $(, $feat:literal)?) => {
        pub mod $m {
            use super::{body, Node, Passes};
            macro_rules! sym { ($k:literal) => { concat!("optk_", stringify!($m), "_pass_", $k) }; }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("muladd_u32")]
            pub unsafe fn muladd_u32(a: &mut [u32], c: &[u32], m: u32) { body::muladd_u32::<$s>(a, c, m) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("muladd_u64")]
            pub unsafe fn muladd_u64(a: &mut [u64], c: &[u64], m: u64) { body::muladd_u64::<$s>(a, c, m) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("mulwide_u64")]
            pub unsafe fn mulwide_u64(a: &mut [u64], c: &[u64], m: u64) { body::mulwide_u64::<$s>(a, c, m) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("modmuladd_p31")]
            pub unsafe fn modmuladd_p31(a: &mut [u32], b: &[u32], m: u32) { body::modmuladd_p31::<$s>(a, b, m) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("modmuladd_p62")]
            pub unsafe fn modmuladd_p62(a: &mut [u64], b: &[u64], m: u64, ninv: u64) { body::modmuladd_p62::<$s>(a, b, m, ninv) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("horner_p31")]
            pub unsafe fn horner_p31(b: &[u32], m: u32, acc: u32) -> u32 { body::horner_p31(b, m, acc) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("xor_u64")]
            pub unsafe fn xor_u64(a: &mut [u64], b: &[u64]) { body::xor_u64::<$s>(a, b) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("popcnt_u64")]
            pub unsafe fn popcnt_u64(a: &[u64], b: &[u64]) -> u64 { body::popcnt_u64::<$s>(a, b) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("fma_f64")]
            pub unsafe fn fma_f64(a: &mut [f64], b: &[f64], s: f64) { body::fma_f64::<$s, $f>(a, b, s) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("triad")]
            pub unsafe fn triad(a: &mut [f64], b: &[f64], c: &[f64], s: f64) { body::triad::<$s, $f>(a, b, c, s) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("fma_reg_generic")]
            pub unsafe fn fma_reg(n: usize, s0: f64) -> f64 { body::fma_reg::<$f, $acc, $lanes>(n, s0) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("chase")]
            pub unsafe fn chase(p: *const Node, n: usize) -> *const Node { body::chase(p, n) }
            #[inline(never)] $(#[target_feature(enable = $feat)])? #[export_name = sym!("clock")]
            pub unsafe fn clock(x: u64, n: usize) -> u64 { body::clock(x, n) }
            pub const PASSES: Passes = Passes {
                muladd_u32, muladd_u64, mulwide_u64, modmuladd_p31, modmuladd_p62, horner_p31, xor_u64,
                popcnt_u64, fma_f64, triad, fma_reg: $reg, fma_reg_unit: $acc * $lanes, chase, clock,
            };
        }
    };
}

isa_mod!(base, false, false, 8, 2, self::fma_reg);
isa_mod!(base_scalar, true, false, 8, 1, self::fma_reg);
#[cfg(target_arch = "x86_64")]
isa_mod!(scalar, true, true, 8, 1, self::fma_reg, "avx,avx2,fma,bmi1,bmi2,popcnt,lzcnt");
#[cfg(target_arch = "x86_64")]
isa_mod!(avx2, false, true, 12, 4, super::fma_reg_ymm, "avx,avx2,fma,bmi1,bmi2,popcnt,lzcnt");
#[cfg(target_arch = "x86_64")]
isa_mod!(avx512, false, true, 16, 8, super::fma_reg_zmm, "avx,avx2,fma,bmi1,bmi2,popcnt,lzcnt,avx512f,avx512dq,avx512bw,avx512vl,avx512cd,avx512vpopcntdq");

/// Register-resident multiply-add: 16 independent zmm accumulators, no loads
/// or stores in the loop; the port-limited peak a blocked matrix product can
/// approach.  n counts scalar multiply-adds (a multiple of 128).
#[cfg(target_arch = "x86_64")]
#[inline(never)]
#[target_feature(enable = "avx512f,fma")]
#[export_name = "optk_avx512_pass_fma_reg"]
pub unsafe fn fma_reg_zmm(n: usize, s0: f64) -> f64 {
    use core::arch::x86_64::*;
    let s = _mm512_set1_pd(s0);
    let c = _mm512_setr_pd(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    let lane = _mm512_setr_pd(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
    let mut acc = [_mm512_setzero_pd(); 16];
    for (i, a) in acc.iter_mut().enumerate() {
        *a = _mm512_add_pd(_mm512_set1_pd(i as f64), lane);
    }
    let (mut it, iters) = (0u64, (n / 128) as u64);
    while it < iters {
        for a in acc.iter_mut() {
            *a = _mm512_fmadd_pd(*a, s, c);
        }
        it = body::bar64::<true>(it) + 1;
    }
    let mut r = 0.0;
    for a in acc {
        let mut out = [0f64; 8];
        _mm512_storeu_pd(out.as_mut_ptr(), a);
        for x in out {
            r += x;
        }
    }
    r
}
/// The same with 12 ymm accumulators (n a multiple of 48).
#[cfg(target_arch = "x86_64")]
#[inline(never)]
#[target_feature(enable = "avx2,fma")]
#[export_name = "optk_avx2_pass_fma_reg"]
pub unsafe fn fma_reg_ymm(n: usize, s0: f64) -> f64 {
    use core::arch::x86_64::*;
    let s = _mm256_set1_pd(s0);
    let c = _mm256_setr_pd(1.0, 2.0, 3.0, 4.0);
    let lane = _mm256_setr_pd(0.0, 1.0, 2.0, 3.0);
    let mut acc = [_mm256_setzero_pd(); 12];
    for (i, a) in acc.iter_mut().enumerate() {
        *a = _mm256_add_pd(_mm256_set1_pd(i as f64), lane);
    }
    let (mut it, iters) = (0u64, (n / 48) as u64);
    while it < iters {
        for a in acc.iter_mut() {
            *a = _mm256_fmadd_pd(*a, s, c);
        }
        it = body::bar64::<true>(it) + 1;
    }
    let mut r = 0.0;
    for a in acc {
        let mut out = [0f64; 4];
        _mm256_storeu_pd(out.as_mut_ptr(), a);
        for x in out {
            r += x;
        }
    }
    r
}

/// Variants this host can run, in the order they are tried.  `scalar` is the
/// vectorization-free build of the widest scalar instruction set present.
pub fn available_isas() -> Vec<&'static str> {
    #[allow(unused_mut)]
    let mut v = Vec::new();
    #[cfg(target_arch = "x86_64")]
    {
        let avx2 = is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") && is_x86_feature_detected!("bmi1")
            && is_x86_feature_detected!("bmi2") && is_x86_feature_detected!("popcnt") && is_x86_feature_detected!("lzcnt");
        let avx512 = avx2 && is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512dq")
            && is_x86_feature_detected!("avx512bw") && is_x86_feature_detected!("avx512vl")
            && is_x86_feature_detected!("avx512cd") && is_x86_feature_detected!("avx512vpopcntdq");
        if avx512 {
            v.push("avx512");
        }
        if avx2 {
            v.push("avx2");
            v.push("scalar");
            return v;
        }
    }
    v.push("base");
    v.push("scalar");
    v
}

/// The pass table of one isa, or None when this CPU lacks its features.
///
/// Safety of every function in the table: it may be called only on a CPU with
/// the features of its isa, which is what a `Some` from here certifies.
pub fn passes(isa: &str) -> Option<Passes> {
    let avail = available_isas();
    if !avail.contains(&isa) {
        return None;
    }
    #[cfg(target_arch = "x86_64")]
    {
        let wide_scalar = avail.contains(&"avx2");
        match isa {
            "avx512" => return Some(avx512::PASSES),
            "avx2" => return Some(avx2::PASSES),
            "scalar" if wide_scalar => return Some(scalar::PASSES),
            _ => {}
        }
    }
    match isa {
        "base" => Some(base::PASSES),
        "scalar" => Some(base_scalar::PASSES),
        _ => None,
    }
}

/// The exported symbol of a kernel's pass in one isa (for `vector-share`).
pub fn symbol(isa: &str, kernel: &str) -> String {
    let m = if isa == "scalar" && !available_isas().contains(&"avx2") { "base_scalar" } else { isa };
    let k = if kernel == "fma_f64_reg" { "fma_reg" } else { kernel };
    format!("optk_{m}_pass_{k}")
}

pub struct Outcome {
    pub kernel: String,
    pub isa: String,
    pub op_class: String,
    pub ops: u64,
    pub seconds: f64,
    pub bytes_per_op: f64,
    pub working_set: usize,
    pub elements: usize,
    pub checksum: u64,
    pub passes_per_timer_read: u64,
}

impl Outcome {
    pub fn json_line(&self) -> String {
        format!(
            "{{\"schema\":\"optlevel.ops/v1\",\"kernel\":\"{}\",\"isa\":\"{}\",\"op_class\":\"{}\",\"ops\":{},\"kernel_seconds\":{:.9},\"bytes_declared\":{:.0},\"working_set_bytes\":{},\"elements\":{},\"passes_per_timer_read\":{},\"checksum\":\"{:016x}\"}}",
            self.kernel, self.isa, self.op_class, self.ops, self.seconds, self.bytes_per_op * self.ops as f64,
            self.working_set, self.elements, self.passes_per_timer_read, self.checksum
        )
    }
}

/// How the timed region ends: after `seconds`, or after exactly `passes`
/// passes (tests; makes the checksum a function of the inputs alone).
#[derive(Clone, Copy)]
pub struct Limit {
    pub seconds: f64,
    pub passes: Option<u64>,
    pub timer_every_pass: bool,
}

struct Timed {
    ops: u64,
    seconds: f64,
    batch: u64,
}

/// Warm-up pass (untimed: page faults, caches), then passes until the limit.
fn timed(limit: Limit, n: usize, mut pass: impl FnMut()) -> Timed {
    pass();
    if let Some(k) = limit.passes {
        let t0 = Instant::now();
        for _ in 0..k {
            pass();
        }
        return Timed { ops: k * n as u64, seconds: t0.elapsed().as_secs_f64(), batch: k.max(1) };
    }
    let mut batch = 1u64;
    if !limit.timer_every_pass {
        // passes per timer read: about 50 us between reads, from the faster of two probe passes
        let mut best = f64::INFINITY;
        for _ in 0..2 {
            let t = Instant::now();
            pass();
            best = best.min(t.elapsed().as_secs_f64());
        }
        if best < 50e-6 {
            batch = ((50e-6 / best.max(1e-9)).ceil() as u64).clamp(1, 1 << 20);
        }
    }
    let mut ops = 0u64;
    let t0 = Instant::now();
    loop {
        for _ in 0..batch {
            pass();
        }
        ops += batch * n as u64;
        let el = t0.elapsed().as_secs_f64();
        if el >= limit.seconds {
            return Timed { ops, seconds: el, batch };
        }
    }
}

pub fn run(kernel: &str, isa: &str, n_arg: usize, limit: Limit) -> Result<Outcome, String> {
    let p = passes(isa).ok_or_else(|| format!("isa {isa} is not available on this host (available: {})", available_isas().join(" ")))?;
    let mut rng = Rng::new();
    let mut n = n_arg;
    let mut cls = kernel.to_string();
    let (t, bytes_per_op, ws, sum): (Timed, f64, usize, u64);
    // SAFETY (every unsafe call below): `passes` returned this table only after
    // run-time detection of the CPU features its functions were compiled for.
    match kernel {
        "muladd_u32" | "modmuladd_p31" => {
            if n == 0 {
                n = 2048;
            }
            let modp = kernel == "modmuladd_p31";
            let mut a = Buf::<u32>::zeroed(n);
            let mut c = Buf::<u32>::zeroed(n);
            for i in 0..n {
                if modp {
                    a[i] = (rng.next() % P31 as u64) as u32;
                    c[i] = (rng.next() % P31 as u64) as u32;
                } else {
                    a[i] = rng.next() as u32;
                    c[i] = rng.next() as u32;
                }
            }
            t = if modp {
                timed(limit, n, || unsafe { (p.modmuladd_p31)(&mut a, &c, 1234567891) })
            } else {
                timed(limit, n, || unsafe { (p.muladd_u32)(&mut a, &c, 2654435761) })
            };
            ws = 8 * n;
            bytes_per_op = 12.0;
            sum = a.iter().fold(0u64, |s, &x| s.wrapping_add(x as u64));
        }
        "muladd_u64" | "mulwide_u64" | "modmuladd_p62" | "xor_u64" | "popcnt_u64" => {
            if n == 0 {
                n = 1024;
            }
            let mut a = Buf::<u64>::zeroed(n);
            let mut b = Buf::<u64>::zeroed(n);
            for i in 0..n {
                a[i] = rng.next();
                b[i] = rng.next();
                if kernel == "modmuladd_p62" {
                    a[i] %= P62;
                    b[i] %= P62;
                }
            }
            let ninv = p62_ninv();
            let mut pop = 0u64;
            t = match kernel {
                "muladd_u64" => timed(limit, n, || unsafe { (p.muladd_u64)(&mut a, &b, 0x9E3779B97F4A7C15) }),
                "mulwide_u64" => timed(limit, n, || unsafe { (p.mulwide_u64)(&mut a, &b, 0x9E3779B97F4A7C15) }),
                "modmuladd_p62" => timed(limit, n, || unsafe { (p.modmuladd_p62)(&mut a, &b, 0x123456789ABCDEF, ninv) }),
                "xor_u64" => timed(limit, n, || unsafe { (p.xor_u64)(&mut a, &b) }),
                _ => timed(limit, n, || pop = pop.wrapping_add(unsafe { (p.popcnt_u64)(&a, &b) })),
            };
            ws = 16 * n;
            bytes_per_op = if kernel == "popcnt_u64" { 16.0 } else { 24.0 };
            sum = if kernel == "popcnt_u64" { pop } else { a.iter().fold(0u64, |s, &x| s.wrapping_add(x)) };
        }
        "horner_p31" => {
            cls = "modmuladd_p31".into();
            if n == 0 {
                n = 2048;
            }
            let mut b = Buf::<u32>::zeroed(n);
            for x in b.iter_mut() {
                *x = (rng.next() % P31 as u64) as u32;
            }
            let mut acc = 1u32;
            t = timed(limit, n, || acc = unsafe { (p.horner_p31)(&b, 1234567891, acc) });
            ws = 4 * n;
            bytes_per_op = 4.0;
            sum = acc as u64;
        }
        "fma_f64" => {
            if n == 0 {
                n = 1024;
            }
            let mut a = Buf::<f64>::zeroed(n);
            let mut b = Buf::<f64>::zeroed(n);
            for i in 0..n {
                a[i] = (rng.next() >> 40) as f64;
                b[i] = (rng.next() >> 40) as f64;
            }
            t = timed(limit, n, || unsafe { (p.fma_f64)(&mut a, &b, 0.5) });
            ws = 16 * n;
            bytes_per_op = 24.0;
            sum = a.iter().fold(0u64, |s, &x| s.wrapping_add(x as u64));
        }
        "fma_f64_reg" => {
            cls = "fma_f64".into();
            if n == 0 {
                n = 128 << 13;
            }
            n -= n % p.fma_reg_unit;
            let mut r = 0f64;
            t = timed(limit, n, || r += unsafe { (p.fma_reg)(n, 0.5) });
            ws = 0;
            bytes_per_op = 0.0;
            sum = r as u64;
        }
        "triad" => {
            // STREAM triad; 32 bytes per element: two reads, one write and the
            // write-allocate read of the destination line.
            cls = "fma_f64".into();
            if n == 0 {
                n = 48 << 20;
            }
            let (mut a, mut b, mut c) = (Buf::<f64>::zeroed(n), Buf::<f64>::zeroed(n), Buf::<f64>::zeroed(n));
            for i in 0..n {
                a[i] = 0.0; // written, so the pages exist before the warm-up pass
                b[i] = (i & 1023) as f64;
                c[i] = 1.0;
            }
            t = timed(limit, n, || unsafe { (p.triad)(&mut a, &b, &c, 3.0) });
            ws = 24 * n;
            bytes_per_op = 32.0;
            sum = a.iter().step_by(4097).fold(0u64, |s, &x| s.wrapping_add(x as u64));
        }
        "chase" => {
            // one dependent load per operation over a random single cycle of
            // 64-byte nodes (Sattolo's algorithm)
            cls = "ptr_hop".into();
            if n == 0 {
                n = 256;
            }
            let mut v = Buf::<Node>::zeroed(n);
            let mut perm: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                let j = (rng.next() % i as u64) as usize;
                perm.swap(i, j);
            }
            let base = v.as_ptr();
            for i in 0..n {
                v[i].next = unsafe { base.add(perm[i]) };
                v[i].pad[0] = i as u64;
            }
            drop(perm);
            let mut ptr = base;
            t = timed(limit, n, || ptr = unsafe { (p.chase)(ptr, n) });
            ws = 64 * n;
            bytes_per_op = 64.0;
            sum = unsafe { (*ptr).pad[0] };
        }
        "clock" => {
            // dependent 1-cycle additions: operations per second = effective core clock
            if n == 0 {
                n = 1 << 20;
            }
            let mut x = 0u64;
            t = timed(limit, n, || x = unsafe { (p.clock)(x, n) });
            ws = 0;
            bytes_per_op = 0.0;
            sum = x;
        }
        _ => return Err(format!("unknown kernel {kernel}; kernels: {}", KERNELS.join(" "))),
    }
    Ok(Outcome { kernel: kernel.into(), isa: isa.into(), op_class: cls, ops: t.ops, seconds: t.seconds, bytes_per_op,
                 working_set: ws, elements: n, checksum: sum, passes_per_timer_read: t.batch })
}

/// `optlevel kernel NAME [--isa I] [--n N] [--seconds S] [--passes K] [--timer-every-pass]`
pub fn main(args: &[String]) -> i32 {
    let usage = || {
        eprintln!("usage: optlevel kernel KERNEL [--isa {}] [--n ELEMENTS] [--seconds S] [--passes K] [--timer-every-pass]\nkernels: {}",
                  available_isas().join("|"), KERNELS.join(" "));
        2
    };
    if args.is_empty() {
        return usage();
    }
    let (mut n, mut isa) = (0usize, available_isas()[0].to_string());
    let mut limit = Limit { seconds: 1.0, passes: None, timer_every_pass: false };
    let mut i = 1;
    while i < args.len() {
        let val = args.get(i + 1);
        match (args[i].as_str(), val) {
            ("--n", Some(v)) => n = match v.parse() { Ok(x) => x, Err(_) => return usage() },
            ("--seconds", Some(v)) => limit.seconds = match v.parse() { Ok(x) => x, Err(_) => return usage() },
            ("--passes", Some(v)) => limit.passes = match v.parse() { Ok(x) => Some(x), Err(_) => return usage() },
            ("--isa", Some(v)) => isa = v.clone(),
            ("--timer-every-pass", _) => {
                limit.timer_every_pass = true;
                i += 1;
                continue;
            }
            _ => return usage(),
        }
        i += 2;
    }
    match run(&args[0], &isa, n, limit) {
        Ok(o) => {
            println!("{}", o.json_line());
            0
        }
        Err(e) => {
            eprintln!("optlevel kernel: {e}");
            2
        }
    }
}
