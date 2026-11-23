//! Exact recursive Fermat-ring multiplication used by the analytic primitives.
//!
//! This is the classical prerequisite, **not** the new sub-nlogn algorithm.
//! It follows the negacyclic construction of Brent and Zimmermann, *Modern
//! Computer Arithmetic*, §2.3.3, Algorithm 2.4. No external arithmetic callback
//! or floating-point FFT is used; leaves use the crate's exact Karatsuba product
//! below a fixed cutoff.
//!
//! At a K-bit node (K a multiple of 64) choose a power of two L dividing K
//! (by a small cost model, typically near sqrt(K)/4). Split into L radix-2^w chunks, w=K/L, and convolve in
//! Z/(2^K'+1), with K' the smallest multiple of a power of two Q >= max(L, 64)
//! (Q close to sqrt(K')) that is at least 2w+log2(L)+2. The extra two bits allow
//! symmetric recovery of every signed negacyclic coefficient. Multiplication by
//! roots of unity is a signed binary shift. Forward DIF and inverse DIT retain
//! the same bit-reversed ordering; there is no global bit-reversal pass.
//!
//! Ring elements are fixed-width little-endian limb buffers of K/64+1 limbs,
//! canonical in [0, 2^K]; butterflies, shifts and the final carry pass work in
//! place on those buffers, so a node allocates O(1) buffers rather than O(1)
//! integers per butterfly. Squaring reuses one forward transform.
//!
//! The recurrence has L*K' = 2K+O(K/sqrt K) and log K' = (log K)/2+O(1). Its
//! bit cost is O(K log K log log K). These are mathematical operation bounds,
//! not a runtime benchmark or a proof that Rust's allocator is a multitape
//! machine.

use crate::bigint::{mul_limbs, Big};

/// Products of at most this many bits use `Big::mul` (Karatsuba). Fixed; it
/// never grows with input length.
pub const BASE_BITS: usize = 1 << 17;

/// Fermat rings Z/(2^K+1) with K at most this use one Karatsuba product and a
/// fold. Fixed.
pub const LEAF_BITS: usize = 1 << 14;

/// Counts actual recursive operations, useful for distinguishing the FFT path
/// from a small-input base case. Saturating diagnostic counters do not dispatch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub transform_nodes: u64,
    pub schoolbook_leaves: u64,
    pub butterflies: u64,
    pub depth: usize,
}

/// Multiply arbitrary unsigned integers exactly.
pub fn multiply(a: &Big, b: &Big) -> Big {
    multiply_with_work(a, b).0
}

/// The same implementation, exposing which branches actually ran.
pub fn multiply_with_work(a: &Big, b: &Big) -> (Big, Work) {
    let mut work = Work::default();
    if a.is_zero() || b.is_zero() {
        return (Big::zero(), work);
    }
    let bits = a
        .bits()
        .checked_add(b.bits())
        .expect("product length overflow");
    if bits <= BASE_BITS {
        work.schoolbook_leaves = 1;
        return (a.mul(b), work);
    }
    let value = multiply_top(a, b, bits, LEAF_BITS, &mut work);
    (value, work)
}

/// Exact product through one top-level Fermat node whose ring size n >= bits
/// is chosen together with its dimensions, so that the coefficient ring is
/// not rounded up by a whole granule when the product length is a power of
/// two.
fn multiply_top(a: &Big, b: &Big, bits: usize, leaf: usize, work: &mut Work) -> Big {
    let (n, len, width, child) = plan_top(bits, leaf);
    // a*b < 2^bits <= 2^n, so the modular result is the ordinary product.
    let m = n / 64;
    let load = |x: &Big| {
        let mut v = vec![0u64; m + 1];
        v[..x.0.len()].copy_from_slice(&x.0);
        v
    };
    let (x, y) = (load(a), load(b));
    let mut out = vec![0u64; m + 1];
    let square = a == b;
    node(&x, &y, &mut out, square, leaf, work, 0, (len, width, child));
    Big::from_limbs(&out)
}

/// Top-level dimensions (n, len, width, child) with n = len*width >= bits,
/// n a multiple of 64, minimizing `node_cost`.
fn plan_top(bits: usize, leaf: usize) -> (usize, usize, usize, usize) {
    let mut best: Option<(f64, (usize, usize, usize, usize))> = None;
    for i in 2..=30u32 {
        let len = 1usize << i;
        if len * 64 > bits {
            break;
        }
        let unit = (64 / len).max(1);
        let width = bits.div_ceil(len).next_multiple_of(unit);
        let n = len * width;
        let needed = 2 * width + i as usize + 2;
        for t in 0..GRANULE_STEPS {
            let child = needed.next_multiple_of(len.max(64) << t);
            if child >= n {
                continue;
            }
            let cost = node_cost(len, child, leaf);
            if best.is_none_or(|(c, _)| cost < c) {
                best = Some((cost, (n, len, width, child)));
            }
        }
    }
    best.expect("invalid top-level Fermat dimensions").1
}

fn pow2_floor(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        1 << (usize::BITS - 1 - x.leading_zeros())
    }
}

/// `a*b mod 2^n+1` for `a, b <= 2^n`, `n` a multiple of 64.
#[cfg(test)]
fn multiply_mod_with(a: &Big, b: &Big, n: usize, leaf: usize, work: &mut Work) -> Big {
    assert!(n.is_multiple_of(64) && n > 0);
    let m = n / 64;
    let load = |x: &Big| {
        assert!(x.bits() <= n + 1 && (x.bits() <= n || x.0.len() == m + 1));
        let mut v = vec![0u64; m + 1];
        v[..x.0.len()].copy_from_slice(&x.0);
        v
    };
    let (x, y) = (load(a), load(b));
    let mut out = vec![0u64; m + 1];
    let square = a == b;
    mul_mod(&x, &y, &mut out, square, leaf, work, 0);
    Big::from_limbs(&out)
}

/// Copy only the requested bit interval: shifting the entire source for every
/// chunk would accidentally make splitting quadratic in the number of chunks.
pub(crate) fn bit_slice(x: &Big, start: usize, len: usize) -> Big {
    if len == 0 || start >= x.bits() {
        return Big::zero();
    }
    let actual = len.min(x.bits() - start);
    let mut out = vec![0u64; actual.div_ceil(64)];
    extract_bits(&x.0, start, actual, &mut out);
    Big::from_limbs(&out)
}

/// Write bits `[start, start+len)` of `src` into the low limbs of `dst`
/// (which must hold `len.div_ceil(64)` limbs and is otherwise untouched).
fn extract_bits(src: &[u64], start: usize, len: usize, dst: &mut [u64]) {
    let count = len.div_ceil(64);
    let first = start / 64;
    let shift = start % 64;
    for (j, d) in dst[..count].iter_mut().enumerate() {
        let mut limb = src.get(first + j).copied().unwrap_or(0) >> shift;
        if shift != 0 {
            limb |= src.get(first + j + 1).copied().unwrap_or(0) << (64 - shift);
        }
        *d = limb;
    }
    if !len.is_multiple_of(64) {
        dst[count - 1] &= (1u64 << (len % 64)) - 1;
    }
}

// ---------------------------------------------------------------------------
// Limb primitives.

/// `x += v` (v a single limb) with carry propagation; returns the carry out.
fn add_small(x: &mut [u64], mut v: u64) -> bool {
    for limb in x {
        if v == 0 {
            return false;
        }
        let (s, o) = limb.overflowing_add(v);
        *limb = s;
        v = o as u64;
    }
    v != 0
}

/// `x -= v` with borrow propagation; returns the borrow out.
fn sub_small(x: &mut [u64], mut v: u64) -> bool {
    for limb in x {
        if v == 0 {
            return false;
        }
        let (d, o) = limb.overflowing_sub(v);
        *limb = d;
        v = o as u64;
    }
    v != 0
}

/// `a += b` in place over equal lengths; returns the carry out.
#[inline]
fn add_assign_n(a: &mut [u64], b: &[u64]) -> u64 {
    let mut c = 0u64;
    for (x, &y) in a.iter_mut().zip(b) {
        let (s1, o1) = x.overflowing_add(y);
        let (s2, o2) = s1.overflowing_add(c);
        *x = s2;
        c = (o1 | o2) as u64;
    }
    c
}

/// `out = a - b` over equal lengths; returns the borrow out.
#[inline]
fn sub_n(out: &mut [u64], a: &[u64], b: &[u64]) -> u64 {
    let mut c = 0u64;
    for ((o, &x), &y) in out.iter_mut().zip(a).zip(b) {
        let (d1, o1) = x.overflowing_sub(y);
        let (d2, o2) = d1.overflowing_sub(c);
        *o = d2;
        c = (o1 | o2) as u64;
    }
    c
}

// ---------------------------------------------------------------------------
// Z/(2^K+1) on buffers of m+1 limbs, K = 64m, canonical in [0, 2^K].

/// `x[..m]` holds `lo`; replace `x` by the canonical form of `lo - t`.
/// Requires |t| < 2^K.
fn fold(x: &mut [u64], t: i64) {
    let m = x.len() - 1;
    x[m] = 0;
    let lo = &mut x[..m];
    if t > 0 {
        // Wrapped W = lo - t + 2^K, and -2^K = 1, so the value is W + 1 <= 2^K.
        if sub_small(lo, t as u64) && add_small(lo, 1) {
            x[m] = 1;
        }
    } else if t < 0 && add_small(lo, t.unsigned_abs()) {
        // Wrapped W = lo + |t| - 2^K, and 2^K = -1, so the value is W - 1.
        if lo.iter().all(|&v| v == 0) {
            x[m] = 1;
        } else {
            sub_small(lo, 1);
        }
    }
}

fn is_zero(x: &[u64]) -> bool {
    x.iter().all(|&v| v == 0)
}

/// `x = -x`.
fn neg_in_place(x: &mut [u64]) {
    let m = x.len() - 1;
    if x[m] == 1 {
        x[..m].fill(0);
        x[0] = 1;
        x[m] = 0;
    } else if !is_zero(&x[..m]) {
        // 2^K + 1 - x = (2^K - 1 - x) + 2.
        for v in &mut x[..m] {
            *v = !*v;
        }
        x[m] = add_small(&mut x[..m], 2) as u64;
    }
}

/// `a = a + b`.
#[inline]
fn add_mod(a: &mut [u64], b: &[u64]) {
    let m = a.len() - 1;
    let c = add_assign_n(&mut a[..m], &b[..m]);
    let t = a[m] + b[m] + c;
    fold(a, t as i64);
}

/// `out = a - b`.
#[inline]
fn sub_mod(out: &mut [u64], a: &[u64], b: &[u64]) {
    let m = out.len() - 1;
    let bw = sub_n(&mut out[..m], &a[..m], &b[..m]);
    let t = a[m] as i64 - b[m] as i64 - bw as i64;
    fold(out, t);
}

/// `out = src * 2^e`; `wide` is scratch of 2m limbs. `out` and `src` differ.
fn shift_mod(src: &[u64], e: usize, out: &mut [u64], wide: &mut [u64]) {
    let m = src.len() - 1;
    let k = 64 * m;
    let e = e % (2 * k);
    let (negative, e) = if e >= k { (true, e - k) } else { (false, e) };
    if e == 0 {
        out.copy_from_slice(src);
        if negative {
            neg_in_place(out);
        }
        return;
    }
    // src <= 2^K and e < K, so src*2^e < 2^(2K) fits in 2m limbs.
    let (q, r) = (e / 64, e % 64);
    let wide = &mut wide[..2 * m];
    wide[..q].fill(0);
    if r == 0 {
        wide[q..q + m + 1].copy_from_slice(src);
        wide[q + m + 1..].fill(0);
    } else {
        let mut prev = 0u64;
        for i in 0..=m {
            let v = src[i];
            wide[q + i] = (v << r) | (prev >> (64 - r));
            prev = v;
        }
        let tail = q + m + 1;
        if tail < 2 * m {
            wide[tail] = prev >> (64 - r);
            wide[tail + 1..].fill(0);
        } else {
            debug_assert_eq!(prev >> (64 - r), 0);
        }
    }
    let (low, high) = wide.split_at(m);
    // src*2^e = low + high*2^K = low - high.
    let bw = if negative {
        sub_n(&mut out[..m], high, low)
    } else {
        sub_n(&mut out[..m], low, high)
    };
    fold(out, -(bw as i64));
}

fn greater_than_half(x: &[u64]) -> bool {
    let m = x.len() - 1;
    if x[m] != 0 {
        return true;
    }
    let top = x[m - 1];
    top >> 63 == 1 && (top << 1 != 0 || !is_zero(&x[..m - 1]))
}

/// Per-node scratch buffers.
struct Scratch {
    t: Vec<u64>,
    wide: Vec<u64>,
}

impl Scratch {
    fn new(m: usize) -> Self {
        Self {
            t: vec![0; m + 1],
            wide: vec![0; 2 * m],
        }
    }
}

/// Forward DIF stages with block sizes `hi, hi/2, ..., lo` over all of
/// `data` (stride `s`) in Z/(2^K+1); block `B` uses root 2^(2K/B). Running
/// every stage from the full length down to 2 gives the bit-reversed
/// transform; disjoint sub-blocks of the low stages are independent.
fn forward_stages(
    data: &mut [u64],
    s: usize,
    hi: usize,
    lo: usize,
    sc: &mut Scratch,
    work: &mut Work,
) {
    let k = 64 * (s - 1);
    let count = data.len() / s;
    let mut block = hi;
    while block >= lo.max(2) {
        let half = block / 2;
        let step = 2 * k / block;
        for chunk in data.chunks_exact_mut(block * s) {
            let (low, high) = chunk.split_at_mut(half * s);
            for (j, (a, b)) in low
                .chunks_exact_mut(s)
                .zip(high.chunks_exact_mut(s))
                .enumerate()
            {
                sub_mod(&mut sc.t, a, b);
                add_mod(a, b);
                shift_mod(&sc.t, j * step, b, &mut sc.wide);
            }
        }
        work.butterflies = work.butterflies.saturating_add((count / 2) as u64);
        block /= 2;
    }
}

/// Inverse DIT stages with block sizes `lo, 2lo, ..., hi` (no 1/len scaling).
fn inverse_stages(
    data: &mut [u64],
    s: usize,
    lo: usize,
    hi: usize,
    sc: &mut Scratch,
    work: &mut Work,
) {
    let k = 64 * (s - 1);
    let count = data.len() / s;
    let mut block = lo.max(2);
    while block <= hi {
        let half = block / 2;
        let step = 2 * k / block;
        for chunk in data.chunks_exact_mut(block * s) {
            let (low, high) = chunk.split_at_mut(half * s);
            for (j, (a, b)) in low
                .chunks_exact_mut(s)
                .zip(high.chunks_exact_mut(s))
                .enumerate()
            {
                shift_mod(b, 2 * k - j * step, &mut sc.t, &mut sc.wide);
                sub_mod(b, a, &sc.t);
                add_mod(a, &sc.t);
            }
        }
        work.butterflies = work.butterflies.saturating_add((count / 2) as u64);
        block *= 2;
    }
}

/// Forward DIF transform of `len` elements; output in bit-reversed order.
fn forward(data: &mut [u64], len: usize, s: usize, sc: &mut Scratch, work: &mut Work) {
    forward_stages(data, s, len, 2, sc, work);
}

/// Inverse DIT transform (without the 1/len scaling), bit-reversed input.
fn inverse(data: &mut [u64], len: usize, s: usize, sc: &mut Scratch, work: &mut Work) {
    inverse_stages(data, s, 2, len, sc, work);
}

// ---------------------------------------------------------------------------
// Parallelism (std scoped threads only; results never depend on it).

static THREADS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Nodes with K at least this many bits use worker threads.
pub const PARALLEL_BITS: usize = 1 << 18;

/// Set the worker-thread cap for large products: 0 (default) means
/// `std::thread::available_parallelism`, 1 disables threading (for isolated
/// single-core measurements). Products are identical for every setting.
pub fn set_threads(threads: usize) {
    THREADS.store(threads, std::sync::atomic::Ordering::Relaxed);
}

fn threads() -> usize {
    match THREADS.load(std::sync::atomic::Ordering::Relaxed) {
        0 => std::thread::available_parallelism().map_or(1, |n| n.get()),
        n => n,
    }
}

impl Work {
    fn absorb(&mut self, other: &Work) {
        self.transform_nodes = self.transform_nodes.saturating_add(other.transform_nodes);
        self.schoolbook_leaves = self
            .schoolbook_leaves
            .saturating_add(other.schoolbook_leaves);
        self.butterflies = self.butterflies.saturating_add(other.butterflies);
        self.depth = self.depth.max(other.depth);
    }
}

/// Run `f` on `parts` disjoint equal pieces of `data` (each a whole number of
/// stride-`s` elements) on scoped threads, merging their work counters.
fn par_pieces<F>(data: &mut [u64], parts: usize, work: &mut Work, f: F)
where
    F: Fn(usize, &mut [u64], &mut Work) + Sync,
{
    let piece = data.len() / parts;
    let results: Vec<Work> = std::thread::scope(|scope| {
        let handles: Vec<_> = data
            .chunks_mut(piece)
            .enumerate()
            .map(|(index, chunk)| {
                let f = &f;
                scope.spawn(move || {
                    let mut local = Work::default();
                    f(index, chunk, &mut local);
                    local
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("fastint worker panicked"))
            .collect()
    });
    for w in &results {
        work.absorb(w);
    }
}

/// Full forward transform, splitting the low stages over `parts` threads.
fn forward_par(
    data: &mut [u64],
    len: usize,
    s: usize,
    parts: usize,
    sc: &mut Scratch,
    work: &mut Work,
) {
    if parts <= 1 {
        forward(data, len, s, sc, work);
        return;
    }
    let sub = len / parts;
    forward_stages(data, s, len, 2 * sub, sc, work);
    let mc = s - 1;
    par_pieces(data, parts, work, |_, piece, w| {
        forward_stages(piece, s, sub, 2, &mut Scratch::new(mc), w);
    });
}

/// Full inverse transform, splitting the low stages over `parts` threads.
fn inverse_par(
    data: &mut [u64],
    len: usize,
    s: usize,
    parts: usize,
    sc: &mut Scratch,
    work: &mut Work,
) {
    if parts <= 1 {
        inverse(data, len, s, sc, work);
        return;
    }
    let sub = len / parts;
    let mc = s - 1;
    par_pieces(data, parts, work, |_, piece, w| {
        inverse_stages(piece, s, 2, sub, &mut Scratch::new(mc), w);
    });
    inverse_stages(data, s, 2 * sub, len, sc, work);
}

/// Granule choices: the child ring size is a multiple of max(len, 64) << t.
const GRANULE_STEPS: usize = 7;

/// Valid recursive dimensions for a K-bit node: (len, chunk width, child K').
fn candidates(k: usize) -> Vec<(usize, usize, usize)> {
    let mut out = Vec::new();
    for i in 2..=k.trailing_zeros().min(40) {
        let len = 1usize << i;
        let width = k / len;
        let needed = 2 * width + i as usize + 2;
        let mut last = 0;
        for t in 0..GRANULE_STEPS {
            let child = needed.next_multiple_of(len.max(64) << t);
            if child < k && child != last {
                out.push((len, width, child));
            }
            last = child;
        }
    }
    out
}

/// Relative cost model (seconds-like units) used only to choose dimensions:
/// a Karatsuba leaf costs about KARATSUBA_COST*m^log2(3) for m limbs, and a
/// butterfly about BUTTERFLY_COST per limb. Correctness never depends on it.
const KARATSUBA_COST: f64 = 4.2e-9;
const BUTTERFLY_COST: f64 = 3.0e-9;

type PlanCache = std::collections::HashMap<(usize, usize), ((usize, usize, usize), f64)>;

thread_local! {
    static PLANS: std::cell::RefCell<PlanCache> = Default::default();
}

/// Best (dimensions, cost) for a K-bit node above the leaf, memoized.
fn best_plan(k: usize, leaf_bits: usize) -> ((usize, usize, usize), f64) {
    if let Some(hit) = PLANS.with(|c| c.borrow().get(&(k, leaf_bits)).copied()) {
        return hit;
    }
    let best = candidates(k)
        .into_iter()
        .map(|d| (d, node_cost(d.0, d.2, leaf_bits)))
        .min_by(|x, y| x.1.total_cmp(&y.1))
        .expect("invalid recursive Fermat dimensions");
    PLANS.with(|c| c.borrow_mut().insert((k, leaf_bits), best));
    best
}

fn model_cost(k: usize, leaf_bits: usize) -> f64 {
    if k <= leaf_bits {
        return KARATSUBA_COST * ((k / 64) as f64).powf(1.585);
    }
    best_plan(k, leaf_bits).1
}

fn node_cost(len: usize, child: usize, leaf_bits: usize) -> f64 {
    let limbs = (child / 64 + 1) as f64;
    let log_len = len.trailing_zeros() as f64;
    // Three transforms (two forward, one inverse) plus spread/untwist passes.
    let transforms = (3.0 * log_len + 4.0) * (len / 2) as f64 * limbs * BUTTERFLY_COST;
    transforms + len as f64 * model_cost(child, leaf_bits)
}

/// Recursive dimensions for a K-bit node, chosen by `node_cost`.
fn plan(k: usize, leaf_bits: usize) -> (usize, usize, usize) {
    debug_assert!(k.is_multiple_of(64));
    best_plan(k, leaf_bits).0
}

/// Split the K-bit value `src` (< 2^K) into `len` chunks of `width` bits and
/// write chunk j times 2^(j*twist) into element j of `dst` (stride s').
fn spread(src: &[u64], width: usize, len: usize, twist: usize, dst: &mut [u64], sc: &mut Scratch) {
    let s = sc.t.len();
    for (j, d) in dst.chunks_exact_mut(s).take(len).enumerate() {
        sc.t.fill(0);
        extract_bits(src, j * width, width, &mut sc.t);
        shift_mod(&sc.t, j * twist, d, &mut sc.wide);
    }
}

/// `acc += value * 2^offset` with carry propagation.
fn add_shifted(acc: &mut [u64], value: &[u64], offset: usize) {
    let (q, r) = (offset / 64, offset % 64);
    let mut carry = 0u64;
    let mut prev = 0u64;
    let mut i = 0;
    while i <= value.len() {
        let v = value.get(i).copied().unwrap_or(0);
        let limb = if r == 0 {
            v
        } else {
            (v << r) | (prev >> (64 - r))
        };
        prev = v;
        let (s1, o1) = acc[q + i].overflowing_add(limb);
        let (s2, o2) = s1.overflowing_add(carry);
        acc[q + i] = s2;
        carry = (o1 | o2) as u64;
        i += 1;
    }
    let overflow = add_small(&mut acc[q + i..], carry);
    debug_assert!(!overflow);
}

/// Reduce a nonnegative integer of fewer than 2K+64 bits modulo 2^K+1.
fn reduce_wide(x: &[u64], out: &mut [u64]) {
    let m = out.len() - 1;
    debug_assert!(is_zero(&x[2 * m..]));
    let bw = sub_n(&mut out[..m], &x[..m], &x[m..2 * m]);
    fold(out, -(bw as i64));
}

/// Leaf: one exact product, then fold the high half.
fn leaf(a: &[u64], b: &[u64], out: &mut [u64], square: bool) {
    let m = out.len() - 1;
    let product = if square {
        mul_limbs(&a[..m], &a[..m])
    } else {
        mul_limbs(&a[..m], &b[..m])
    };
    let bw = sub_n(&mut out[..m], &product[..m], &product[m..]);
    fold(out, -(bw as i64));
}

/// `out = a*b mod 2^K+1` with K = 64*(len-1); `square` means a == b.
fn mul_mod(
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
    square: bool,
    leaf_bits: usize,
    work: &mut Work,
    depth: usize,
) {
    work.depth = work.depth.max(depth);
    let m = out.len() - 1;
    let k = 64 * m;
    // Canonical endpoint 2^K represents -1 and is not an ordinary K-bit value.
    if a[m] != 0 {
        out.copy_from_slice(b);
        neg_in_place(out);
        return;
    }
    if b[m] != 0 {
        out.copy_from_slice(a);
        neg_in_place(out);
        return;
    }
    if is_zero(a) || is_zero(b) {
        out.fill(0);
        return;
    }
    if k <= leaf_bits {
        work.schoolbook_leaves = work.schoolbook_leaves.saturating_add(1);
        leaf(a, b, out, square);
        return;
    }
    let dims = plan(k, leaf_bits);
    node(a, b, out, square, leaf_bits, work, depth, dims);
}

/// One Fermat node with explicit dimensions; `a, b < 2^K` nonzero.
#[allow(clippy::too_many_arguments)]
fn node(
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
    square: bool,
    leaf_bits: usize,
    work: &mut Work,
    depth: usize,
    (len, width, child): (usize, usize, usize),
) {
    work.depth = work.depth.max(depth);
    work.transform_nodes = work.transform_nodes.saturating_add(1);
    let m = out.len() - 1;
    let k = 64 * m;
    let mc = child / 64;
    let s = mc + 1;
    let twist = child / len;
    let log_len = len.trailing_zeros() as usize;
    let mut sc = Scratch::new(mc);
    let parts = if k >= PARALLEL_BITS {
        threads().min(len / 4).max(1)
    } else {
        1
    };
    // Equal power-of-two pieces keep transform sub-blocks aligned.
    let parts = if parts > 1 { pow2_floor(parts) } else { 1 };

    let load = |x: &[u64], sc: &mut Scratch, w: &mut Work| {
        let mut f = vec![0u64; len * s];
        spread(&x[..m], width, len, twist, &mut f, sc);
        forward_par(&mut f, len, s, parts, sc, w);
        f
    };
    let (mut fa, fb) = if square {
        (load(a, &mut sc, work), None)
    } else if parts > 1 {
        let (fa, (fb, wb)) = std::thread::scope(|scope| {
            let hb = scope.spawn(|| {
                let mut w = Work::default();
                let f = load(b, &mut Scratch::new(mc), &mut w);
                (f, w)
            });
            let fa = load(a, &mut sc, work);
            (fa, hb.join().expect("fastint worker panicked"))
        });
        work.absorb(&wb);
        (fa, Some(fb))
    } else {
        let fa = load(a, &mut sc, work);
        let fb = load(b, &mut sc, work);
        (fa, Some(fb))
    };
    let piece = len / parts;
    let pointwise = |index: usize, chunk: &mut [u64], w: &mut Work| {
        let mut product = vec![0u64; s];
        for (j, x) in chunk.chunks_exact_mut(s).enumerate() {
            let global = index * piece + j;
            match &fb {
                Some(f) => mul_mod(
                    x,
                    &f[global * s..(global + 1) * s],
                    &mut product,
                    false,
                    leaf_bits,
                    w,
                    depth + 1,
                ),
                None => mul_mod(x, x, &mut product, true, leaf_bits, w, depth + 1),
            }
            x.copy_from_slice(&product);
        }
    };
    if parts > 1 {
        par_pieces(&mut fa, parts, work, pointwise);
    } else {
        pointwise(0, &mut fa, work);
    }
    drop(fb);
    inverse_par(&mut fa, len, s, parts, &mut sc, work);

    // Untwist, divide by len, read each coefficient as a signed integer in
    // (-2^(K'-1), 2^(K'-1)], and accumulate sum c_j 2^(j*width).
    let mut positive = vec![0u64; 2 * m + 2];
    let mut negative = vec![0u64; 2 * m + 2];
    for (j, c) in fa.chunks_exact(s).enumerate() {
        let e = (j * twist + log_len) % (2 * child);
        shift_mod(c, (2 * child - e) % (2 * child), &mut sc.t, &mut sc.wide);
        if greater_than_half(&sc.t) {
            neg_in_place(&mut sc.t);
            add_shifted(&mut negative, &sc.t, j * width);
        } else {
            add_shifted(&mut positive, &sc.t, j * width);
        }
    }
    let mut p = vec![0u64; m + 1];
    let mut q = vec![0u64; m + 1];
    reduce_wide(&positive, &mut p);
    reduce_wide(&negative, &mut q);
    sub_mod(out, &p, &q);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(limbs: usize, seed: u64) -> Big {
        let mut state = seed;
        let values = (0..limbs)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state
            })
            .collect::<Vec<_>>();
        Big::from_limbs(&values)
    }

    fn pow2(n: usize) -> Big {
        Big::from_u64(1).shl(n)
    }

    /// A ring size >= bits usable with `multiply_mod_with`.
    fn top_ring_bits(bits: usize) -> usize {
        bits.next_multiple_of(pow2_floor(bits.isqrt()).max(64))
    }

    fn modulus(n: usize) -> Big {
        pow2(n).add_small(1)
    }

    /// Ring element buffer of m+1 limbs from a canonical Big.
    fn element(x: &Big, m: usize) -> Vec<u64> {
        let mut v = vec![0u64; m + 1];
        v[..x.0.len()].copy_from_slice(&x.0);
        v
    }

    #[test]
    fn small_native_oracle_and_zero() {
        for a in [0, 1, 7, u32::MAX as u64, u64::MAX] {
            for b in [0, 1, 13, u64::MAX] {
                assert_eq!(
                    multiply(&Big::from_u64(a), &Big::from_u64(b)),
                    Big::from_u128(a as u128 * b as u128)
                );
            }
        }
    }

    #[test]
    fn recursive_products_against_independent_schoolbook() {
        for (a_len, b_len) in [
            (33, 33),
            (65, 1),
            (67, 70),
            (180, 193),
            (511, 257),
            (600, 600),
            (1100, 300),
            (2048, 2048),
        ] {
            let a = fixture(a_len, 0x9e3779b97f4a7c15);
            let b = fixture(b_len, 0x2545f4914f6cdd1d);
            let (got, work) = multiply_with_work(&a, &b);
            assert_eq!(got, a.mul_schoolbook(&b), "lengths {a_len},{b_len}");
            assert!(work.schoolbook_leaves > 0);
            if a.bits() + b.bits() > BASE_BITS {
                assert!(work.transform_nodes > 0, "lengths {a_len},{b_len}");
                assert!(work.butterflies > 0);
            } else {
                assert_eq!(work.transform_nodes, 0);
            }
            // Squaring shares one forward transform.
            assert_eq!(multiply(&a, &a), a.mul_schoolbook(&a), "square {a_len}");
        }
    }

    #[test]
    fn deep_recursion_with_small_leaf() {
        let mut max_depth = 0;
        for (limbs, leaf) in [(64, 256), (150, 512), (300, 256), (513, 1024), (2048, 256)] {
            let a = fixture(limbs, 11 + limbs as u64);
            let b = fixture(limbs - 3, 29 + limbs as u64);
            let n = top_ring_bits(a.bits() + b.bits());
            let mut work = Work::default();
            assert_eq!(
                multiply_mod_with(&a, &b, n, leaf, &mut work),
                a.mul_schoolbook(&b),
                "limbs {limbs} leaf {leaf}"
            );
            max_depth = max_depth.max(work.depth);
            let n = top_ring_bits(2 * a.bits());
            let mut work = Work::default();
            assert_eq!(
                multiply_mod_with(&a, &a, n, leaf, &mut work),
                a.mul_schoolbook(&a)
            );
        }
        // Recursive Fermat nodes below the top (depth counts from 0).
        assert!(max_depth >= 2, "max depth {max_depth}");
    }

    #[test]
    fn negative_coefficients_endpoint_and_long_carries() {
        for n in [8192, 16384, 32768] {
            for leaf in [256, 1024, LEAF_BITS] {
                let near = pow2(n).sub_small(1);
                let mut work = Work::default();
                let mut run = |a: &Big, b: &Big| multiply_mod_with(a, b, n, leaf, &mut work);
                assert_eq!(run(&near, &near), Big::from_u64(4));
                assert_eq!(run(&pow2(n), &near), Big::from_u64(2));
                assert_eq!(run(&pow2(n), &pow2(n)), Big::from_u64(1));
                assert_eq!(run(&Big::zero(), &near), Big::zero());
                let a = fixture(n / 64, 123);
                let b = fixture(n / 64, 456);
                let expected = a.mul_schoolbook(&b).divrem(&modulus(n)).1;
                assert_eq!(run(&a, &b), expected, "n {n} leaf {leaf}");
                let expected = a.mul_schoolbook(&a).divrem(&modulus(n)).1;
                assert_eq!(run(&a, &a), expected, "square n {n} leaf {leaf}");
            }
        }
    }

    #[test]
    fn ring_primitives_against_big_oracle() {
        for m in [1usize, 2, 3, 5] {
            let n = 64 * m;
            let p = modulus(n);
            let inputs = [
                Big::zero(),
                Big::from_u64(1),
                Big::from_u64(2),
                pow2(n - 1),
                pow2(n - 1).add_small(1),
                pow2(n).sub_small(1),
                pow2(n),
                fixture(m, 99).divrem(&p).1,
                fixture(m, 7).divrem(&p).1,
            ];
            let mut wide = vec![0u64; 2 * m];
            for x in &inputs {
                let ex = element(x, m);
                for exponent in [
                    0,
                    1,
                    63,
                    64,
                    65,
                    n - 1,
                    n,
                    n + 1,
                    2 * n - 1,
                    2 * n,
                    3 * n + 5,
                ] {
                    let mut out = vec![0u64; m + 1];
                    shift_mod(&ex, exponent, &mut out, &mut wide);
                    assert_eq!(
                        Big::from_limbs(&out),
                        x.shl(exponent).divrem(&p).1,
                        "m {m} shift {exponent}"
                    );
                }
                let mut negated = ex.clone();
                neg_in_place(&mut negated);
                assert_eq!(Big::from_limbs(&negated), p.sub(x).divrem(&p).1);
                assert_eq!(greater_than_half(&ex), x > &pow2(n - 1));
                for y in &inputs {
                    let ey = element(y, m);
                    let mut sum = ex.clone();
                    add_mod(&mut sum, &ey);
                    assert_eq!(Big::from_limbs(&sum), x.add(y).divrem(&p).1);
                    let mut difference = vec![0u64; m + 1];
                    sub_mod(&mut difference, &ex, &ey);
                    assert_eq!(Big::from_limbs(&difference), x.add(&p).sub(y).divrem(&p).1);
                }
            }
            for t in [-3i64, -1, 0, 1, 3] {
                for lo in [Big::zero(), Big::from_u64(1), pow2(n).sub_small(1)] {
                    let mut x = element(&lo, m);
                    fold(&mut x, t);
                    let expected = if t >= 0 {
                        lo.add(&p).add(&p).sub_small(t as u64).divrem(&p).1
                    } else {
                        lo.add_small(t.unsigned_abs()).divrem(&p).1
                    };
                    assert_eq!(Big::from_limbs(&x), expected, "fold m {m} t {t}");
                }
            }
        }
    }

    #[test]
    fn retained_transform_layout_round_trip() {
        for (m, len) in [(1usize, 16usize), (2, 8), (3, 64)] {
            let s = m + 1;
            let p = modulus(64 * m);
            let mut sc = Scratch::new(m);
            let mut work = Work::default();
            let mut data = vec![0u64; len * s];
            for (j, c) in data.chunks_exact_mut(s).enumerate() {
                let value = fixture(m, j as u64 + 1).divrem(&p).1;
                c.copy_from_slice(&element(&value, m));
            }
            if len > 4 {
                data[s..2 * s].fill(0);
                data[2 * s + m] = 1; // the endpoint 2^K
                data[2 * s..2 * s + m].fill(0);
            }
            let original = data.clone();
            forward(&mut data, len, s, &mut sc, &mut work);
            inverse(&mut data, len, s, &mut sc, &mut work);
            // The inverse omits 1/len: compare against len * original.
            for (got, want) in data.chunks_exact(s).zip(original.chunks_exact(s)) {
                let want = Big::from_limbs(want).mul_small(len as u64).divrem(&p).1;
                assert_eq!(Big::from_limbs(got), want);
            }
        }
    }

    #[test]
    fn plan_dimensions_are_valid() {
        let mut k = 256 * 64;
        while k <= 1 << 26 {
            for extra in [0, 64, 4096] {
                let kk = top_ring_bits(k + extra);
                let (len, width, child) = plan(kk, LEAF_BITS);
                assert!(len.is_power_of_two() && len >= 4);
                assert_eq!(len * width, kk);
                assert!(child.is_multiple_of(64) && child.is_multiple_of(len));
                assert!(child >= 2 * width + len.trailing_zeros() as usize + 2);
                assert!(child < kk);
            }
            k *= 2;
        }
    }

    #[test]
    fn top_level_plan_is_valid_and_tight() {
        for base in [BASE_BITS, 1 << 18, 1 << 20, 1 << 23, 1 << 26] {
            for bits in [
                base + 1,
                base + 64,
                base * 5 / 4,
                base * 3 / 2,
                base * 2 - 7,
                base * 2,
            ] {
                let (n, len, width, child) = plan_top(bits, LEAF_BITS);
                assert!(n >= bits && n.is_multiple_of(64));
                assert_eq!(len * width, n);
                assert!(len.is_power_of_two() && len >= 4);
                assert!(child.is_multiple_of(64) && child.is_multiple_of(len));
                assert!(child >= 2 * width + len.trailing_zeros() as usize + 2);
                assert!(child < n);
                // The ring is never more than one chunk row per element above bits.
                assert!(
                    n - bits < len * (64 / len).max(1) + len,
                    "bits {bits} n {n}"
                );
            }
        }
    }

    #[test]
    fn top_level_path_near_powers_of_two() {
        for limbs in [64usize, 127, 128, 129, 255, 256, 300] {
            for leaf in [256, 1024] {
                let a = fixture(limbs, 5 + limbs as u64);
                let b = fixture(limbs + 1, 17 + limbs as u64);
                let mut work = Work::default();
                let bits = a.bits() + b.bits();
                assert_eq!(
                    multiply_top(&a, &b, bits, leaf, &mut work),
                    a.mul_schoolbook(&b)
                );
                assert!(work.transform_nodes > 0);
                let mut work = Work::default();
                assert_eq!(
                    multiply_top(&a, &a, 2 * a.bits(), leaf, &mut work),
                    a.mul_schoolbook(&a)
                );
            }
        }
    }

    #[test]
    fn threaded_products_are_identical() {
        // Above PARALLEL_BITS so worker threads are used when allowed.
        let a = fixture(9000, 3);
        let b = fixture(8100, 4);
        let expected = a.mul(&b);
        let expected_square = a.mul(&a);
        for threads in [1, 2, 4, 0] {
            set_threads(threads);
            let (got, work) = multiply_with_work(&a, &b);
            assert_eq!(got, expected, "threads {threads}");
            assert!(work.transform_nodes > 0);
            assert_eq!(
                multiply(&a, &a),
                expected_square,
                "square threads {threads}"
            );
        }
        set_threads(0);
    }

    #[test]
    fn non_aligned_bit_chunks() {
        let x = fixture(8, 777);
        for offset in [0, 1, 63, 64, 65, 127, 333, 512] {
            for width in [0, 1, 33, 64, 65, 135] {
                let expected = if width == 0 {
                    Big::zero()
                } else {
                    x.shr(offset).divrem(&pow2(width)).1
                };
                assert_eq!(bit_slice(&x, offset, width), expected);
            }
        }
    }
}
