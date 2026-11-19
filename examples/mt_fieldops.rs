//! Multi-target ECDLP on a small prime-order curve, counted in FIELD operations (experimental).
//!
//! Four arms share one counted u64 field layer (p < 2^31): every inversion (I), multiplication (M)
//! and squaring (S) is counted, plus hash inserts/lookups (H) and solver edge operations (L).
//! Additions/subtractions in F_p are not counted.  Costs are converted to group-addition
//! equivalents (GAE = one affine addition = I + 2M + S) in the analysis, at several I/M ratios.
//!
//! The lever under test is "P +- Q with one inversion": x(P+Q) and x(P-Q) share the denominator
//! x_Q - x_P, so both x-coordinates cost I + 2M + 2S (one affine addition costs I + 2M + S).
//! Arms:
//!   rho2xksp, rhodpup  rho2xks / rhodpu with a flat table and pipelined or prefetched lookups.
//!   ic5b  ic5 with a std or flat target table and lock-step batched, prefetched probes (same counted ops).
//!   ic4   index calculus: factor base = m smallest-x points (logs unknown), x-only pair-sum table
//!         {x(F_i +- F_j)} built with shared inversions, factor-base logs from relations
//!         V_j +- W_k = +-(F_i +- F_j), then per target paired trials x(Q_t +- W_k) against the table.
//!         W_k (a shared arithmetic progression of known multiples) is computed once for all targets.
//!   rho   anchored batch rho as in `ic_batch_rho` (r-adding walks, database of visited x, x-only lookups).
//!   rho2  rho with the same lever: every walk step R -> R + A also yields R - A in full (+2M+S, y kept
//!         so a hit needs no recomputation), stored in phase 1 and looked up in phase 2.
//!   rho2x rho2 with an x-only sibling (I + 3M + 2S per step), the sibling recomputed in full on a hit (counted).
//!   rhoseq anchored rho whose phase-1 database is the sequential set {G, 2G, ..., NG} (no walk waste).
//!   bsgs2 multi-target BSGS with a sum-set baby table {x(jU +- iG)} (shared inversions) and paired giant
//!         trials x(Q_t +- kS G), giants shared by all targets.
//! On a hit, the sign is resolved by recomputing the two points in full (counted); every recovered log
//! is verified by a counted double-and-add scalar multiplication.
//!
//! usage: mt_fieldops <arm> <instance.json> <targets> <seed> <param>
//!   ic4: param = mu (m = ceil(mu (nT/2)^(1/4))), optional 6th arg g (relations per fb element, default 1.5)
//!   rho, rho2, rho2x, rhoseq: param = kappa (database = kappa sqrt(nT) x-classes)
//!   bsgs2: param = mf (baby x-classes = mf sqrt(nT)/2)
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cell::Cell;
// The hash tables of every arm. Default build: std HashMap (SipHash, RandomState). With the cargo feature
// `mt-fasthash`: the same tables with a fast multiplicative hasher (MtFastH). Counted operations are identical in
// both builds (a hash-table operation is counted per call, whatever the hasher); only wall time differs.
#[cfg(not(feature = "mt-fasthash"))]
type HState = std::collections::hash_map::RandomState;
#[cfg(feature = "mt-fasthash")]
type HState = std::hash::BuildHasherDefault<MtFastH>;
type HashMap<K, V> = std::collections::HashMap<K, V, HState>;
#[derive(Default)]
#[allow(dead_code)]
struct MtFastH(u64);
impl std::hash::Hasher for MtFastH {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, b: &[u8]) {
        for &c in b {
            self.0 = (self.0 ^ c as u64).wrapping_mul(0x100_0000_01b3);
        }
    }
    fn write_u64(&mut self, v: u64) {
        self.0 = (self.0 ^ v ^ (v >> 29)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    }
}
use std::time::Instant;

type Pt = Option<(u64, u64)>;

struct F {
    p: u64,
    a: u64,
    b: u64,
    ci: Cell<u64>,
    cm: Cell<u64>,
    cs: Cell<u64>,
    ch: Cell<u64>,
    cl: Cell<u64>,
    cn: Cell<u64>,
}

impl F {
    fn add(&self, x: u64, y: u64) -> u64 {
        let s = x + y;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn sub(&self, x: u64, y: u64) -> u64 {
        if x >= y {
            x - y
        } else {
            x + self.p - y
        }
    }
    fn neg(&self, x: u64) -> u64 {
        if x == 0 {
            0
        } else {
            self.p - x
        }
    }
    fn mul(&self, x: u64, y: u64) -> u64 {
        self.cm.set(self.cm.get() + 1);
        ((x as u128 * y as u128) % self.p as u128) as u64
    }
    fn sqr(&self, x: u64) -> u64 {
        self.cs.set(self.cs.get() + 1);
        ((x as u128 * x as u128) % self.p as u128) as u64
    }
    fn inv(&self, x: u64) -> u64 {
        self.ci.set(self.ci.get() + 1);
        let (mut r0, mut r1) = (self.p as i128, x as i128);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        assert_eq!(r0, 1, "non-invertible");
        t0.rem_euclid(self.p as i128) as u64
    }
    fn hash(&self) {
        self.ch.set(self.ch.get() + 1);
    }
    // unpriced native work (step-index hashes, canonicalisations, distinguished-point tests, W-step indices),
    // counted so that the section 5a band can price it at h
    fn native(&self) {
        self.cn.set(self.cn.get() + 1);
    }
    fn lin(&self, k: u64) {
        self.cl.set(self.cl.get() + k);
    }
    fn small(&self, k: u64) -> u64 {
        k % self.p
    }
    /// Full affine addition (handles doubling and inverses).
    fn padd(&self, p: Pt, q: Pt) -> Pt {
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        let lam = if x1 == x2 {
            if self.add(y1, y2) == 0 {
                return None;
            }
            // doubling: (3x^2 + a) / (2y)
            let xx = self.sqr(x1);
            let num = self.add(self.add(self.add(xx, xx), xx), self.a);
            let d = self.inv(self.add(y1, y1));
            self.mul(num, d)
        } else {
            let d = self.inv(self.sub(x2, x1));
            self.mul(self.sub(y2, y1), d)
        };
        let x3 = self.sub(self.sub(self.sqr(lam), x1), x2);
        let y3 = self.sub(self.mul(lam, self.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    fn pneg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, self.neg(y)))
    }
    /// x(P+Q), x(P-Q) with one shared inversion (I + 2M + 2S).  None if x_P == x_Q or a point is O.
    fn xpair(&self, p: Pt, q: Pt) -> Option<(u64, u64)> {
        let ((x1, y1), (x2, y2)) = (p?, q?);
        if x1 == x2 {
            return None;
        }
        let d = self.inv(self.sub(x2, x1));
        let s = self.add(x1, x2);
        let lp = self.mul(self.sub(y2, y1), d);
        let lm = self.mul(self.neg(self.add(y2, y1)), d);
        Some((self.sub(self.sqr(lp), s), self.sub(self.sqr(lm), s)))
    }
    /// Montgomery's simultaneous inversion of nonzero values in place: 1 I + 3 (L - 1) M
    /// (research/mt_ic_scale_20261115, arms ic5b "bi" and rhodpb).
    fn batch_inv(&self, v: &mut [u64], scratch: &mut Vec<u64>) {
        if v.is_empty() {
            return;
        }
        scratch.clear();
        let mut acc = v[0];
        scratch.push(acc);
        for &x in &v[1..] {
            acc = self.mul(acc, x);
            scratch.push(acc);
        }
        let mut inv = self.inv(acc);
        for i in (1..v.len()).rev() {
            let vi = v[i];
            v[i] = self.mul(inv, scratch[i - 1]);
            inv = self.mul(inv, vi);
        }
        v[0] = inv;
    }
    /// `xpair` with d = 1 / (x2 - x1) given (x1 != x2): 2M + 2S.
    fn xpair_d(&self, p: (u64, u64), q: (u64, u64), d: u64) -> (u64, u64) {
        let ((x1, y1), (x2, y2)) = (p, q);
        let s = self.add(x1, x2);
        let lp = self.mul(self.sub(y2, y1), d);
        let lm = self.mul(self.neg(self.add(y2, y1)), d);
        (self.sub(self.sqr(lp), s), self.sub(self.sqr(lm), s))
    }
    /// P + Q with d = 1 / (x2 - x1) given (x1 != x2): 2M + S.
    fn padd_d(&self, p: (u64, u64), q: (u64, u64), d: u64) -> Pt {
        let ((x1, y1), (x2, y2)) = (p, q);
        let lam = self.mul(self.sub(y2, y1), d);
        let x3 = self.sub(self.sub(self.sqr(lam), x1), x2);
        let y3 = self.sub(self.mul(lam, self.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    /// P + Q in full and the sibling P - Q in full, sharing the inversion: (I + 2M + S) + (2M + S).
    fn padd_with_sibling(&self, p: Pt, q: Pt) -> (Pt, Pt) {
        if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
            if x1 != x2 {
                let d = self.inv(self.sub(x2, x1));
                let lam = self.mul(self.sub(y2, y1), d);
                let x3 = self.sub(self.sub(self.sqr(lam), x1), x2);
                let y3 = self.sub(self.mul(lam, self.sub(x1, x3)), y1);
                // P - Q = P + (-Q): slope (-y2 - y1)/(x2 - x1)
                let lm = self.mul(self.neg(self.add(y2, y1)), d);
                let xm = self.sub(self.sub(self.sqr(lm), x1), x2);
                let ym = self.sub(self.mul(lm, self.sub(x1, xm)), y1);
                return (Some((x3, y3)), Some((xm, ym)));
            }
        }
        (self.padd(p, q), None)
    }
    /// `padd_with_sibling` with d = 1 / (x2 - x1) given (x1 != x2): (2M + S) + (2M + S)
    /// (research/mt_ic_rel_20261115, ic5b option "cb").
    fn padd_sib_d(&self, p: (u64, u64), q: (u64, u64), d: u64) -> (Pt, Pt) {
        let ((x1, y1), (x2, y2)) = (p, q);
        let lam = self.mul(self.sub(y2, y1), d);
        let x3 = self.sub(self.sub(self.sqr(lam), x1), x2);
        let y3 = self.sub(self.mul(lam, self.sub(x1, x3)), y1);
        let lm = self.mul(self.neg(self.add(y2, y1)), d);
        let xm = self.sub(self.sub(self.sqr(lm), x1), x2);
        let ym = self.sub(self.mul(lm, self.sub(x1, xm)), y1);
        (Some((x3, y3)), Some((xm, ym)))
    }
    fn smul(&self, p: Pt, k: u64) -> Pt {
        let mut r: Pt = None;
        let mut base = p;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.padd(r, base);
            }
            k >>= 1;
            if k > 0 {
                base = self.padd(base, base);
            }
        }
        r
    }
    fn pow(&self, mut b: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            e >>= 1;
            if e > 0 {
                b = self.sqr(b);
            }
        }
        r
    }
    /// Counted Tonelli-Shanks; None for a non-residue.
    fn sqrt(&self, v: u64) -> Option<u64> {
        if v == 0 {
            return Some(0);
        }
        let p = self.p;
        if self.pow(v, (p - 1) / 2) != 1 {
            return None;
        }
        let (mut q, mut s) = (p - 1, 0u32);
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut z = 2u64;
        while self.pow(z, (p - 1) / 2) != p - 1 {
            z += 1;
        }
        let mut c = self.pow(z, q);
        let mut x = self.pow(v, q.div_ceil(2));
        let mut t = self.pow(v, q);
        let mut m = s;
        while t != 1 {
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 {
                tt = self.sqr(tt);
                i += 1;
            }
            let mut bb = c;
            for _ in 0..(m - i - 1) {
                bb = self.sqr(bb);
            }
            x = self.mul(x, bb);
            c = self.sqr(bb);
            t = self.mul(t, c);
            m = i;
        }
        Some(x)
    }
}

fn md(a: i128, n: u64) -> u64 {
    a.rem_euclid(n as i128) as u64
}

/// x_t from a hit "Q + s_w * W = eps * D" where log W = w and log D = dl, eps unknown; recompute
/// both points to resolve eps (counted), then return the log.
fn resolve(f: &F, q: Pt, w_pt: Pt, s_w: i64, w: u64, d_pt: Pt, dl: u64, n: u64) -> u64 {
    let wp = if s_w > 0 { w_pt } else { f.pneg(w_pt) };
    let r = f.padd(q, wp);
    let eps: i128 = if r == d_pt { 1 } else { -1 };
    md(eps * dl as i128 - s_w as i128 * w as i128, n)
}

struct Out {
    ok: usize,
    extra: String,
}

/// The table of ic5b, rho2xksp and rhodpup: std HashMap (exactly ic5's) or a flat open-addressed array of slots
/// (x key, u32 index into `ents`), load factor at most 1/2, multiplicative hash, linear probing.
/// For p < 2^32 the slots are 8 bytes (u32 key, `Flat`); for larger p they are 16 bytes (u64 key, `FlatW`,
/// added for research/mt_ic_scale_20261115). First insert of an x wins, as `entry().or_insert`.
enum Tab<V: Copy> {
    Std(HashMap<u64, V>),
    Flat {
        slots: Vec<(u32, u32)>,
        ents: Vec<V>,
        mask: usize,
        shift: u32,
    },
    FlatW {
        slots: Vec<(u64, u32)>,
        ents: Vec<V>,
        mask: usize,
        shift: u32,
    },
    /// 8-byte slots (low 32 bits of x as a fingerprint, u32 index; index FLAT_EMPTY = empty) with the full x of
    /// every entry in `xs`, read only on a fingerprint match, so the table is exactly FlatW's
    /// (research/mt_ic_mem_20261115, ic5b table mode "flatf" when p >= 2^32).
    FlatF {
        slots: Vec<(u32, u32)>,
        xs: Vec<u64>,
        ents: Vec<V>,
        mask: usize,
        shift: u32,
    },
}
const FLAT_EMPTY: u32 = u32::MAX;
extern "C" {
    fn madvise(addr: *mut std::ffi::c_void, len: usize, advice: i32) -> i32;
}
/// Environment `MTF_HP=1` (research/mt_ic_hp_20261118): every flat table's arrays, for every arm (IC and rho
/// alike), ask for transparent huge pages with madvise(MADV_HUGEPAGE) before first touch. On a host whose THP mode
/// is "madvise" this replaces 4 KB pages by 2 MB pages; nothing else changes.
fn hp_on() -> bool {
    static HP: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *HP.get_or_init(|| std::env::var("MTF_HP").map(|v| v == "1").unwrap_or(false))
}
fn hp_advise<T>(v: &Vec<T>) {
    let bytes = v.capacity() * std::mem::size_of::<T>();
    if !hp_on() || bytes < (4 << 20) {
        return;
    }
    let p = v.as_ptr() as usize;
    let a = (p + (2 << 20) - 1) & !((2 << 20) - 1);
    let end = (p + bytes) & !((2 << 20) - 1);
    if end > a {
        // SAFETY: [a, end) lies inside the vector's own allocation; MADV_HUGEPAGE (14) only changes paging policy.
        unsafe {
            madvise(a as *mut std::ffi::c_void, end - a, 14);
        }
    }
}
/// `vec![x; n]` whose pages are advised before they are first written.
fn hp_vec<T: Clone>(n: usize, x: T) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    hp_advise(&v);
    v.resize(n, x);
    v
}
/// Environment `MTF_Z=1` (research/mt_ic_tb_20261119): every flat table, for every arm, marks an empty slot by
/// a stored index of 0 (entries store index + 1), so its slot array is allocated already zeroed (lazily, by the
/// kernel) instead of being filled with a non-zero "empty" pattern first. Contents and lookups are unchanged.
static ZMODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[inline]
fn z_on() -> bool {
    ZMODE.load(std::sync::atomic::Ordering::Relaxed)
}
/// The slot array of a flat table: zeroed (zero mode) or filled with `empty`, advised for huge pages either way.
fn slot_vec<T: Clone>(n: usize, zero: T, empty: T) -> Vec<T> {
    if z_on() {
        let v = vec![zero; n];
        hp_advise(&v);
        v
    } else {
        hp_vec(n, empty)
    }
}
/// Slot value -> (empty?, entry index), and entry index -> slot value, in the current mode.
#[inline]
fn slot_free(v: u32) -> bool {
    if z_on() {
        v == 0
    } else {
        v == FLAT_EMPTY
    }
}
#[inline]
fn slot_idx(v: u32) -> usize {
    if z_on() {
        v as usize - 1
    } else {
        v as usize
    }
}
#[inline]
fn slot_val(i: usize) -> u32 {
    if z_on() {
        i as u32 + 1
    } else {
        i as u32
    }
}
/// `Vec::with_capacity(n)`, advised before first touch.
fn hp_cap<T>(n: usize) -> Vec<T> {
    let v = Vec::with_capacity(n);
    hp_advise(&v);
    v
}
/// A flat-table key: the x coordinate, narrowed to u32 when p < 2^32. `EMPTY` is never a valid x.
trait FKey: Copy + Eq {
    const EMPTY: Self;
    const ZERO: Self;
    fn of(x: u64) -> Self;
    fn wide(self) -> u64;
}
impl FKey for u32 {
    const EMPTY: u32 = u32::MAX;
    const ZERO: u32 = 0;
    #[inline]
    fn of(x: u64) -> u32 {
        x as u32
    }
    #[inline]
    fn wide(self) -> u64 {
        self as u64
    }
}
impl FKey for u64 {
    const EMPTY: u64 = u64::MAX;
    const ZERO: u64 = 0;
    #[inline]
    fn of(x: u64) -> u64 {
        x
    }
    #[inline]
    fn wide(self) -> u64 {
        self
    }
}
/// A `Flat`/`FlatW` slot is free when its key is the EMPTY sentinel (classic) or its value is 0 (zero mode).
#[inline]
fn kslot_free<K: FKey>(k: K, v: u32) -> bool {
    if z_on() {
        v == 0
    } else {
        k == K::EMPTY
    }
}
#[inline]
fn flat_home(x: u64, shift: u32) -> usize {
    (x.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> shift) as usize
}
#[inline(always)]
fn flat_insert<K: FKey, V>(
    slots: &mut Vec<(K, u32)>,
    ents: &mut Vec<V>,
    mask: &mut usize,
    shift: &mut u32,
    x: u64,
    e: V,
) {
    // keep the load factor at most 1/2: double and reinsert (never reached by ic5b, whose capacity bound is
    // exact; rho databases grow with retained walks)
    if 2 * ents.len() >= slots.len() {
        let size = 2 * slots.len();
        let nshift = 64 - size.trailing_zeros();
        let mut ns = slot_vec(size, (K::ZERO, 0u32), (K::EMPTY, 0u32));
        for &(k, v) in slots.iter() {
            if !kslot_free(k, v) {
                let mut j = flat_home(k.wide(), nshift);
                while !kslot_free(ns[j].0, ns[j].1) {
                    j = (j + 1) & (size - 1);
                }
                ns[j] = (k, v);
            }
        }
        *slots = ns;
        *mask = size - 1;
        *shift = nshift;
    }
    let key = K::of(x);
    let mut i = flat_home(x, *shift);
    loop {
        let (k, v) = slots[i];
        if kslot_free(k, v) {
            slots[i] = (key, slot_val(ents.len()));
            ents.push(e);
            return;
        }
        if k == key {
            return;
        }
        i = (i + 1) & *mask;
    }
}
#[inline]
fn flat_get<K: FKey, V: Copy>(
    slots: &[(K, u32)],
    ents: &[V],
    mask: usize,
    shift: u32,
    x: u64,
) -> Option<V> {
    if x >= K::EMPTY.wide() {
        return None;
    }
    let key = K::of(x);
    let mut i = flat_home(x, shift);
    loop {
        let (k, v) = slots[i];
        if kslot_free(k, v) {
            return None;
        }
        if k == key {
            return Some(ents[slot_idx(v)]);
        }
        i = (i + 1) & mask;
    }
}
#[inline(always)]
fn flatf_insert<V>(
    slots: &mut [(u32, u32)],
    xs: &mut Vec<u64>,
    ents: &mut Vec<V>,
    mask: usize,
    shift: u32,
    x: u64,
    e: V,
) {
    // ic5b sizes the table exactly (load factor at most 1/2, or 0.6 for "flatfd"), so no resize is needed
    assert!(4 * ents.len() < 3 * slots.len(), "flatf table full");
    let fp = x as u32;
    let mut i = flat_home(x, shift);
    loop {
        let (k, v) = slots[i];
        if slot_free(v) {
            slots[i] = (fp, slot_val(ents.len()));
            ents.push(e);
            xs.push(x);
            return;
        }
        if k == fp && xs[slot_idx(v)] == x {
            return;
        }
        i = (i + 1) & mask;
    }
}
#[inline]
fn flatf_get<V: Copy>(
    slots: &[(u32, u32)],
    xs: &[u64],
    ents: &[V],
    mask: usize,
    shift: u32,
    x: u64,
) -> Option<V> {
    let fp = x as u32;
    let mut i = flat_home(x, shift);
    loop {
        let (k, v) = slots[i];
        if slot_free(v) {
            return None;
        }
        if k == fp && xs[slot_idx(v)] == x {
            return Some(ents[slot_idx(v)]);
        }
        i = (i + 1) & mask;
    }
}
#[inline]
fn flat_prefetch<T>(slots: &[T], i: usize) {
    #[cfg(target_arch = "x86_64")]
    // SAFETY: `i` is masked into range by `flat_home` (the shift leaves log2(len) bits); a prefetch never faults.
    unsafe {
        std::arch::x86_64::_mm_prefetch(
            slots.as_ptr().add(i) as *const i8,
            std::arch::x86_64::_MM_HINT_T0,
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = (slots, i);
}
impl<V: Copy> Tab<V> {
    fn new(flat: bool, cap: usize, p: u64) -> Tab<V> {
        if flat {
            let size = (2 * cap.max(1)).next_power_of_two();
            let (mask, shift) = (size - 1, 64 - size.trailing_zeros());
            if p <= FLAT_EMPTY as u64 {
                Tab::Flat {
                    slots: slot_vec(size, (0, 0), (FLAT_EMPTY, 0)),
                    ents: hp_cap(cap),
                    mask,
                    shift,
                }
            } else {
                Tab::FlatW {
                    slots: slot_vec(size, (0, 0), (u64::MAX, 0)),
                    ents: hp_cap(cap),
                    mask,
                    shift,
                }
            }
        } else {
            Tab::Std(HashMap::with_capacity_and_hasher(cap, Default::default()))
        }
    }
    /// ic5b table mode "flatf": `Flat` for p < 2^32, `FlatF` (8-byte fingerprint slots) above.
    fn new_fp(cap: usize, p: u64) -> Tab<V> {
        if p <= FLAT_EMPTY as u64 {
            return Tab::new(true, cap, p);
        }
        let size = (2 * cap.max(1)).next_power_of_two();
        Tab::FlatF {
            slots: slot_vec(size, (0, 0), (0, FLAT_EMPTY)),
            xs: hp_cap(cap),
            ents: hp_cap(cap),
            mask: size - 1,
            shift: 64 - size.trailing_zeros(),
        }
    }
    /// ic5b table mode "flatfd" (research/mt_ic_rel_20261115): `FlatF` for every p, sized to load at most 0.6
    /// (slots = next power of two >= 5 cap / 3) instead of 1/2, which halves the slot array whenever 2 cap is
    /// just above a power of two. Same x -> entry map as "flat"/"flatf"; probes stay mostly within a cache line.
    fn new_fp_dense(cap: usize, _p: u64) -> Tab<V> {
        let size = (cap.max(1) * 5).div_ceil(3).next_power_of_two();
        Tab::FlatF {
            slots: slot_vec(size, (0, 0), (0, FLAT_EMPTY)),
            xs: hp_cap(cap),
            ents: hp_cap(cap),
            mask: size - 1,
            shift: 64 - size.trailing_zeros(),
        }
    }
    /// Insert a chunk in order (research/mt_ic_tb_20261119, ic5b option "ib"): the variant is matched once per
    /// chunk and each slot is prefetched 32 inserts ahead, so the table equals in-order `insert_if_absent`.
    fn insert_batch(&mut self, items: &[(u64, V)]) {
        const D: usize = 32;
        match self {
            Tab::Std(m) => {
                for &(x, e) in items {
                    m.entry(x).or_insert(e);
                }
            }
            Tab::Flat {
                slots,
                ents,
                mask,
                shift,
            } => {
                for i in 0..items.len() {
                    if i + D < items.len() {
                        flat_prefetch(slots, flat_home(items[i + D].0, *shift));
                    }
                    flat_insert(slots, ents, mask, shift, items[i].0, items[i].1);
                }
            }
            Tab::FlatW {
                slots,
                ents,
                mask,
                shift,
            } => {
                for i in 0..items.len() {
                    if i + D < items.len() {
                        flat_prefetch(slots, flat_home(items[i + D].0, *shift));
                    }
                    flat_insert(slots, ents, mask, shift, items[i].0, items[i].1);
                }
            }
            Tab::FlatF {
                slots,
                xs,
                ents,
                mask,
                shift,
            } => {
                for i in 0..items.len() {
                    if i + D < items.len() {
                        flat_prefetch(slots, flat_home(items[i + D].0, *shift));
                    }
                    flatf_insert(slots, xs, ents, *mask, *shift, items[i].0, items[i].1);
                }
            }
        }
    }
    fn insert_if_absent(&mut self, x: u64, e: V) {
        match self {
            Tab::Std(m) => {
                m.entry(x).or_insert(e);
            }
            Tab::Flat {
                slots,
                ents,
                mask,
                shift,
            } => flat_insert(slots, ents, mask, shift, x, e),
            Tab::FlatW {
                slots,
                ents,
                mask,
                shift,
            } => flat_insert(slots, ents, mask, shift, x, e),
            Tab::FlatF {
                slots,
                xs,
                ents,
                mask,
                shift,
            } => flatf_insert(slots, xs, ents, *mask, *shift, x, e),
        }
    }
    #[inline]
    fn get(&self, x: u64) -> Option<V> {
        match self {
            Tab::Std(m) => m.get(&x).copied(),
            Tab::Flat {
                slots,
                ents,
                mask,
                shift,
            } => flat_get(slots, ents, *mask, *shift, x),
            Tab::FlatW {
                slots,
                ents,
                mask,
                shift,
            } => flat_get(slots, ents, *mask, *shift, x),
            Tab::FlatF {
                slots,
                xs,
                ents,
                mask,
                shift,
            } => flatf_get(slots, xs, ents, *mask, *shift, x),
        }
    }
    #[inline]
    fn prefetch(&self, x: u64) {
        match self {
            Tab::Std(_) => {}
            Tab::Flat { slots, shift, .. } => flat_prefetch(slots, flat_home(x, *shift)),
            Tab::FlatW { slots, shift, .. } => flat_prefetch(slots, flat_home(x, *shift)),
            Tab::FlatF { slots, shift, .. } => flat_prefetch(slots, flat_home(x, *shift)),
        }
    }
    /// The flat tables' hash shift (home = (x * C) >> shift); None for `Std`.
    fn shift(&self) -> Option<u32> {
        match self {
            Tab::Std(_) => None,
            Tab::Flat { shift, .. } | Tab::FlatW { shift, .. } | Tab::FlatF { shift, .. } => {
                Some(*shift)
            }
        }
    }
    fn len(&self) -> usize {
        match self {
            Tab::Std(m) => m.len(),
            Tab::Flat { ents, .. } | Tab::FlatW { ents, .. } | Tab::FlatF { ents, .. } => {
                ents.len()
            }
        }
    }
}

/// Insert a chunk of table entries in order of their home slot (stable counting sort on the top bits of the home,
/// at most 4096 buckets), so consecutive inserts touch nearby slots (research/mt_ic_rel_20261115, ic5b option
/// "rad"). Equal x share a home, so their relative order, and with it "first insert wins", is kept: the table maps
/// every x to the same entry as in-order insertion; only slot positions and entry indices may differ.
fn rad_flush<V: Copy>(table: &mut Tab<V>, buf: &mut Vec<(u64, V)>, tmp: &mut Vec<(u64, V)>) {
    if buf.is_empty() {
        return;
    }
    let Some(shift) = table.shift() else {
        for &(x, e) in buf.iter() {
            table.insert_if_absent(x, e);
        }
        buf.clear();
        return;
    };
    let bb = (64 - shift).min(12);
    let bucket = |x: u64| (x.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> (64 - bb)) as usize;
    let mut pos = vec![0usize; (1usize << bb) + 1];
    for &(x, _) in buf.iter() {
        pos[bucket(x) + 1] += 1;
    }
    for i in 1..pos.len() {
        pos[i] += pos[i - 1];
    }
    tmp.clear();
    tmp.resize(buf.len(), buf[0]);
    for &(x, e) in buf.iter() {
        let b = bucket(x);
        tmp[pos[b]] = (x, e);
        pos[b] += 1;
    }
    // home-ordered inserts, each slot prefetched 32 inserts ahead
    for i in 0..tmp.len() {
        if i + 32 < tmp.len() {
            table.prefetch(tmp[i + 32].0);
        }
        table.insert_if_absent(tmp[i].0, tmp[i].1);
    }
    buf.clear();
}

fn main() {
    ZMODE.store(
        std::env::var("MTF_Z").map(|v| v == "1").unwrap_or(false),
        std::sync::atomic::Ordering::Relaxed,
    );
    let args: Vec<String> = std::env::args().collect();
    let arm = args[1].as_str();
    let d: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&args[2]).unwrap()).unwrap();
    let targets: usize = args[3].parse().unwrap();
    let seed: u64 = args[4].parse().unwrap();
    let param: f64 = args[5].parse().unwrap();
    let num = |v: &serde_json::Value| -> u64 { v.to_string().trim_matches('"').parse().unwrap() };
    let p = num(&d["field"]["modulus"]);
    assert!(p < (1u64 << 60)); // bank sums of three logs must fit in u64
    let f = F {
        p,
        a: num(&d["a"]),
        b: num(&d["b"]),
        ci: Cell::new(0),
        cm: Cell::new(0),
        cs: Cell::new(0),
        ch: Cell::new(0),
        cl: Cell::new(0),
        cn: Cell::new(0),
    };
    let n = num(&d["subgroup_order"]);
    let g: Pt = Some((num(&d["generator"]["x"]), num(&d["generator"]["y"])));
    // Planted targets (uncounted): same planting stream for every arm.
    let mut plant = StdRng::seed_from_u64(seed ^ 0x9e37_79b9_7f4a_7c15);
    let xs: Vec<u64> = (0..targets).map(|_| plant.gen_range(1..n)).collect();
    let qs: Vec<Pt> = {
        // computed with an uncounted copy of the field
        let f0 = F {
            p,
            a: f.a,
            b: f.b,
            ci: Cell::new(0),
            cm: Cell::new(0),
            cs: Cell::new(0),
            ch: Cell::new(0),
            cl: Cell::new(0),
            cn: Cell::new(0),
        };
        xs.iter().map(|&x| f0.smul(g, x)).collect()
    };
    let mut rng = StdRng::seed_from_u64(seed.wrapping_mul(0x2545_f491_4f6c_dd1d) ^ 0xabcdef);
    let t0 = Instant::now();
    let mut found: Vec<u64> = vec![0; targets];
    let mut snap: Vec<(String, [u64; 5])> = Vec::new();
    let take = |f: &F| [f.ci.get(), f.cm.get(), f.cs.get(), f.ch.get(), f.cl.get()];

    let out: Out = match arm {
        "ic4" => {
            let mu = param;
            let gfac: f64 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(1.5);
            let m = ((mu * ((n as f64) * targets as f64 / 2.0).powf(0.25)).ceil() as usize).max(4);
            // Factor base: smallest x with x^3 + ax + b a nonzero square (counted).
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // x-only pair-sum table: key x -> (i, j, kind) kind 0: F_i+F_j, 1: F_i-F_j, 2: 2F_i
            let mut table: HashMap<u64, (u32, u32, u8)> =
                HashMap::with_capacity_and_hasher(m * m + 8, Default::default());
            for i in 0..m {
                let pi = Some(fb[i]);
                if let Some((x2, _)) = f.padd(pi, pi) {
                    f.hash();
                    table.entry(x2).or_insert((i as u32, i as u32, 2));
                }
                for j in (i + 1)..m {
                    if let Some((xp, xm)) = f.xpair(pi, Some(fb[j])) {
                        f.hash();
                        table.entry(xp).or_insert((i as u32, j as u32, 0));
                        f.hash();
                        table.entry(xm).or_insert((i as u32, j as u32, 1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u32, u8)| -> Pt {
                let (i, j, k) = e;
                let pi = Some(fb[i as usize]);
                match k {
                    2 => f.padd(pi, pi),
                    0 => f.padd(pi, Some(fb[j as usize])),
                    _ => f.padd(pi, f.pneg(Some(fb[j as usize]))),
                }
            };
            // Shared progression W_k = W_0 + k D (known logs), extended on demand.
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, k: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= k {
                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);
                }
            };
            // Phase 1: relations V_j +- W_k = eps (F_a +- F_b) on known points.
            // Relation: eps*(s_a y_a + s_b y_b) = v_j +- w_k.  Stored as (a, s_a, b, s_b, rhs) after eps fix.
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            let need = (gfac * m as f64).ceil() as usize;
            let mut rels: Vec<(u32, i8, u32, i8, u64)> = Vec::with_capacity(need);
            let mut seen_pairs: HashMap<(u32, u32, u8), ()> = HashMap::default();
            let mut p1_trials = 0u64;
            'outer: loop {
                for k in 0..kw {
                    p1_trials += 2;
                    if let Some((xp, xm)) = f.xpair(v_pt, w_pts[k]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                f.hash();
                                if seen_pairs.insert(e, ()).is_some() {
                                    continue;
                                }
                                // V + sw W = eps * D
                                let wp = if sw > 0 { w_pts[k] } else { f.pneg(w_pts[k]) };
                                let r = f.padd(v_pt, wp);
                                let dpt2 = entry_pt(&f, e);
                                let eps: i8 = if r == dpt2 { 1 } else { -1 };
                                let rhs = md(v_log as i128 + sw as i128 * w_log[k] as i128, n);
                                let (i, j, kind) = e;
                                let (sa, sb): (i8, i8) = match kind {
                                    2 => (2 * eps, 0),
                                    0 => (eps, eps),
                                    _ => (eps, -eps),
                                };
                                rels.push((i, sa, j, sb, rhs));
                                if rels.len() >= need {
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                v_pt = f.padd(v_pt, e_pt);
                v_log = (v_log + e_log) % n;
            }
            snap.push(("relations".into(), take(&f)));
            // Solve the signed graph: y_i = c_i * r + d_i on each component (c_i = +-1), fix r by an
            // odd cycle / doubled-edge / self relation.
            let inv_mod = |a: u64| -> u64 {
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let mut adj: Vec<Vec<usize>> = vec![Vec::new(); m];
            for (ri, r) in rels.iter().enumerate() {
                adj[r.0 as usize].push(ri);
                if r.3 != 0 && r.2 != r.0 {
                    adj[r.2 as usize].push(ri);
                }
            }
            let mut ylog: Vec<Option<u64>> = vec![None; m];
            let mut comp: Vec<i64> = vec![-1; m];
            let mut cd: Vec<(i64, u64)> = vec![(0, 0); m];
            for root in 0..m {
                if comp[root] >= 0 || adj[root].is_empty() {
                    continue;
                }
                comp[root] = root as i64;
                cd[root] = (1, 0);
                let mut order = vec![root];
                let mut qi = 0;
                let mut fix: Option<u64> = None;
                while qi < order.len() {
                    let u = order[qi];
                    qi += 1;
                    for &ri in &adj[u] {
                        f.lin(1);
                        let (a, sa, b, sb, rhs) = rels[ri];
                        let (a, b) = (a as usize, b as usize);
                        // sa*y_a + sb*y_b = rhs
                        if sb == 0 || a == b {
                            // (sa+sb) y_a = rhs
                            let coef = sa as i64 + if a == b { sb as i64 } else { 0 };
                            let (c, dv) = cd[a];
                            let k = coef * c;
                            if fix.is_none() && k != 0 {
                                let rest = md(rhs as i128 - coef as i128 * dv as i128, n);
                                fix = Some(
                                    (rest as u128 * inv_mod(md(k as i128, n)) as u128 % n as u128)
                                        as u64,
                                );
                            }
                            continue;
                        }
                        let (o, so, su) = if a == u { (b, sb, sa) } else { (a, sa, sb) };
                        let (cu, du) = cd[u];
                        // su*y_u + so*y_o = rhs -> y_o = (rhs - su*y_u)/so, so = +-1
                        let co = -(su as i64) * cu * so as i64;
                        let dov = md((rhs as i128 - su as i128 * du as i128) * so as i128, n);
                        if comp[o] < 0 {
                            comp[o] = root as i64;
                            cd[o] = (co, dov);
                            order.push(o);
                        } else if fix.is_none() {
                            let (c2, d2) = cd[o];
                            let k = c2 - co;
                            if k != 0 {
                                let rest = md(dov as i128 - d2 as i128, n);
                                fix = Some(
                                    (rest as u128 * inv_mod(md(k as i128, n)) as u128 % n as u128)
                                        as u64,
                                );
                            }
                        }
                    }
                }
                if let Some(r) = fix {
                    for &u in &order {
                        let (c, dv) = cd[u];
                        ylog[u] = Some(md(c as i128 * r as i128 + dv as i128, n));
                    }
                }
            }
            let determined = ylog.iter().filter(|v| v.is_some()).count();
            snap.push(("solve".into(), take(&f)));
            // Phase 2: paired trials x(Q_t +- W_k).
            let mut trials = 0u64;
            let mut wasted_hits = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut k = 0usize;
                let v = 'tgt: loop {
                    ensure_w(&f, k, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[k].unwrap().0 {
                        break 'tgt if q == w_pts[k] {
                            w_log[k]
                        } else {
                            (n - w_log[k]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[k]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                let (i, j, kind) = e;
                                let yl = match kind {
                                    2 => ylog[i as usize].map(|y| md(2 * y as i128, n)),
                                    0 => ylog[i as usize]
                                        .zip(ylog[j as usize])
                                        .map(|(a, b)| md(a as i128 + b as i128, n)),
                                    _ => ylog[i as usize]
                                        .zip(ylog[j as usize])
                                        .map(|(a, b)| md(a as i128 - b as i128, n)),
                                };
                                match yl {
                                    Some(dl) => {
                                        let dpt2 = entry_pt(&f, e);
                                        break 'tgt resolve(
                                            &f, q, w_pts[k], sw, w_log[k], dpt2, dl, n,
                                        );
                                    }
                                    None => wasted_hits += 1,
                                }
                            }
                        }
                    }
                    k += 1;
                    if k > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"m\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"determined\":{},\"trials\":{},\"wasted_hits\":{},\"w_len\":{}",
                    m, table.len(), rels.len(), p1_trials, determined, trials, wasted_hits, w_pts.len()
                ),
            }
        }
        "ic5" => {
            // k-term index calculus: table of x(F_{i1} + s2 F_{i2} + ... + s_k F_{ik}), i1 < ... < ik,
            // built as x(P +- F_l) from full (k-1)-term points P (shared inversion per pair);
            // factor-base logs by incremental Gaussian elimination mod n (every mod-n product counted
            // as M, every mod-n inverse as I); targets by paired trials x(Q +- W_k) as in ic4.
            let mu = param;
            let k: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            let gfac: f64 = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(1.2);
            assert!((2..=4).contains(&k));
            let kstar = mu * mu * ((n as f64) * targets as f64 / 2.0).sqrt();
            let classes = |m: usize| -> f64 {
                let mut c = 1.0;
                for i in 0..k {
                    c *= (m - i) as f64 / (i + 1) as f64;
                }
                c * (1u64 << (k - 1)) as f64
            };
            let mut m = k + 1;
            while classes(m) < kstar {
                m += 1;
            }
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // (k-1)-term full points: (point, indices, signs); first sign +1.
            let mut combos: Vec<(Pt, Vec<u16>, Vec<i8>)> = (0..m)
                .map(|i| (Some(fb[i]), vec![i as u16], vec![1i8]))
                .collect();
            for _ in 2..k {
                let mut next = Vec::new();
                for (p, idx, sg) in &combos {
                    let last = *idx.last().unwrap() as usize;
                    for l in (last + 1)..m {
                        let (sum, dif) = f.padd_with_sibling(*p, Some(fb[l]));
                        for (pt, s) in [(sum, 1i8), (dif, -1i8)] {
                            if pt.is_some() {
                                let mut i2 = idx.clone();
                                i2.push(l as u16);
                                let mut s2 = sg.clone();
                                s2.push(s);
                                next.push((pt, i2, s2));
                            }
                        }
                    }
                }
                combos = next;
            }
            snap.push(("combos".into(), take(&f)));
            // table: x -> (combo id, l, sign of F_l)
            let mut table: HashMap<u64, (u32, u16, i8)> =
                HashMap::with_capacity_and_hasher(kstar as usize + 16, Default::default());
            for (ci_, (p, idx, _)) in combos.iter().enumerate() {
                let last = *idx.last().unwrap() as usize;
                for l in (last + 1)..m {
                    if let Some((xp, xm)) = f.xpair(*p, Some(fb[l])) {
                        f.hash();
                        table.entry(xp).or_insert((ci_ as u32, l as u16, 1));
                        f.hash();
                        table.entry(xm).or_insert((ci_ as u32, l as u16, -1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u16, i8)| -> Pt {
                let fl = Some(fb[e.1 as usize]);
                f.padd(
                    combos[e.0 as usize].0,
                    if e.2 > 0 { fl } else { f.pneg(fl) },
                )
            };
            // coefficient vector of an entry: (index, sign) pairs
            let entry_terms = |e: (u32, u16, i8)| -> Vec<(usize, i64)> {
                let (_, idx, sg) = &combos[e.0 as usize];
                let mut v: Vec<(usize, i64)> = idx
                    .iter()
                    .zip(sg)
                    .map(|(&i, &s)| (i as usize, s as i64))
                    .collect();
                v.push((e.1 as usize, e.2 as i64));
                v
            };
            // counted arithmetic mod n
            let mulm = |a: u64, b: u64| -> u64 {
                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64
            };
            let invm = |a: u64| -> u64 {
                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, kk: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= kk {
                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);
                }
            };
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            // incremental echelon form: pivots[col] = Some(row) with row[col] = 1 (row has m+1 entries, last = rhs)
            let mut pivots: Vec<Option<Vec<u64>>> = vec![None; m];
            let mut rank = 0usize;
            let mut seen: HashMap<(u32, u16, i8), ()> = HashMap::default();
            let mut nrel = 0usize;
            let budget_rel = (gfac * m as f64).ceil() as usize;
            let mut p1_trials = 0u64;
            let mut kk = 0usize;
            while rank < m {
                if kk == kw {
                    kk = 0;
                    v_pt = f.padd(v_pt, e_pt);
                    v_log = (v_log + e_log) % n;
                }
                p1_trials += 2;
                if let Some((xp, xm)) = f.xpair(v_pt, w_pts[kk]) {
                    for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                        f.hash();
                        if let Some(&e) = table.get(&xx) {
                            f.hash();
                            if seen.insert(e, ()).is_some() {
                                continue;
                            }
                            let wp = if sw > 0 { w_pts[kk] } else { f.pneg(w_pts[kk]) };
                            let r = f.padd(v_pt, wp);
                            let eps: i64 = if r == entry_pt(&f, e) { 1 } else { -1 };
                            let mut row = vec![0u64; m + 1];
                            for (i, s) in entry_terms(e) {
                                row[i] = md((row[i] as i128) + (eps * s) as i128, n);
                            }
                            row[m] = md(v_log as i128 + sw as i128 * w_log[kk] as i128, n);
                            nrel += 1;
                            // reduce against pivots
                            for c in 0..m {
                                if row[c] == 0 {
                                    continue;
                                }
                                if let Some(pr) = &pivots[c] {
                                    let fct = row[c];
                                    for j in c..=m {
                                        if pr[j] != 0 {
                                            row[j] =
                                                md(row[j] as i128 - mulm(fct, pr[j]) as i128, n);
                                        }
                                    }
                                } else {
                                    let iv = invm(row[c]);
                                    for j in c..=m {
                                        if row[j] != 0 {
                                            row[j] = mulm(row[j], iv);
                                        }
                                    }
                                    pivots[c] = Some(row.clone());
                                    rank += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                kk += 1;
                if nrel > 50 * budget_rel + 1000 {
                    panic!("relation budget");
                }
            }
            // back substitution
            let mut ylog: Vec<u64> = vec![0; m];
            for c in (0..m).rev() {
                let pr = pivots[c].as_ref().unwrap();
                let mut v = pr[m] as i128;
                for j in (c + 1)..m {
                    if pr[j] != 0 {
                        v -= mulm(pr[j], ylog[j]) as i128;
                    }
                }
                ylog[c] = md(v, n);
            }
            snap.push(("relations".into(), take(&f)));
            let mut trials = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let v = 'tgt: loop {
                    ensure_w(&f, kq, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[kq].unwrap().0 {
                        break 'tgt if q == w_pts[kq] {
                            w_log[kq]
                        } else {
                            (n - w_log[kq]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                let dl = md(
                                    entry_terms(e)
                                        .iter()
                                        .map(|&(i, s)| s as i128 * ylog[i] as i128)
                                        .sum(),
                                    n,
                                );
                                let dpt2 = entry_pt(&f, e);
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                        }
                    }
                    kq += 1;
                    if kq > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            let _ = budget_rel;
            Out {
                ok: 0,
                extra: format!(
                    "\"k\":{},\"m\":{},\"combos\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"trials\":{},\"w_len\":{}",
                    k, m, combos.len(), table.len(), nrel, p1_trials, trials, w_pts.len()
                ),
            }
        }
        "ic5b" => {
            // ic5 with a choice of target-phase table and probe order (research/mt_table_layout_20261114):
            //   args[8] = "std" (std HashMap, as ic5) | "flat" (open-addressed array of 8-byte slots, x < 2^32);
            //   args[9] = B, targets processed in lock-step batches of B with each batch's probe slots prefetched.
            // Every target's trial sequence is ic5's, so the recovered logs and every counted operation equal ic5's;
            // only the order of memory accesses differs. "std", B = 1 is ic5 itself.
            // k-term index calculus: table of x(F_{i1} + s2 F_{i2} + ... + s_k F_{ik}), i1 < ... < ik,
            // built as x(P +- F_l) from full (k-1)-term points P (shared inversion per pair);
            // factor-base logs by incremental Gaussian elimination mod n (every mod-n product counted
            // as M, every mod-n inverse as I); targets by paired trials x(Q +- W_k) as in ic4.
            let mu = param;
            let k: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            let gfac: f64 = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(1.2);
            // "flatf" (research/mt_ic_mem_20261115): "flat" with 8-byte fingerprint slots when p >= 2^32
            let tmode = args.get(8).map(|s| s.as_str()).unwrap_or("std");
            let flat = match tmode {
                "std" => false,
                "flat" | "flatf" | "flatfd" => true,
                o => panic!("ic5b table mode {o}"),
            };
            let bsz: usize = args.get(9).map(|s| s.parse().unwrap()).unwrap_or(1).max(1);
            // args[10] = "bi": Montgomery simultaneous inversion in the table build (per combo) and in the target
            // phase (per lock-step round); the table, every trial and the recovered logs are unchanged, only I and
            // M counts move (research/mt_ic_scale_20261115). Default "plain" is ic5b as before.
            let bi = args.get(10).map(|s| s.as_str()) == Some("bi");
            // args[11] = "pf" (research/mt_ic_mem_20261115): table-build inserts go through a FIFO of PF_DIST
            // entries whose slots are prefetched; insert order, the table and every count are unchanged.
            // args[11] is a comma-separated option set (research/mt_ic_rel_20261115): "pf" as before; "rel" batches
            // the relation phase's inversions per block of kw trials and prefetches its probes; "rad" inserts the
            // table in home-slot order per chunk (rad_flush); "cb" batches the (k-1)-term combination step. None of
            // them changes the table's x -> entry map, a trial, a relation or a recovered log; I and M move.
            let opts: Vec<&str> = args
                .get(11)
                .map(|s| s.split(',').collect())
                .unwrap_or_default();
            for o in &opts {
                assert!(
                    ["pf", "rel", "rad", "cb", "xb", "ib", "blk"].contains(o),
                    "ic5b option {o}"
                );
            }
            let pf = opts.contains(&"pf");
            let rel_b = opts.contains(&"rel");
            let rad = opts.contains(&"rad");
            let cb = opts.contains(&"cb");
            // "xb" (research/mt_ic_tb_20261119): the table build's simultaneous inversions span consecutive combos
            // until at least XB_MIN denominators are pending, instead of one per combo; insert order is unchanged.
            let xb = opts.contains(&"xb");
            const XB_MIN: usize = 64;
            // "ib" (research/mt_ic_tb_20261119): table-build inserts are buffered IB_CHUNK at a time and inserted
            // in order by Tab::insert_batch; the table equals in-order insertion.
            let ib = opts.contains(&"ib");
            const IB_CHUNK: usize = 1024;
            // "blk" (research/mt_st_20261119): each active target computes its next BLK_K trial pairs in one round
            // (one simultaneous inversion, all probes prefetched) and checks them in order, stopping at the first hit;
            // the trials checked, the hits and the logs are unchanged, only I and M move (single-target batching).
            let blk_k: usize = if opts.contains(&"blk") { 64 } else { 1 };
            let mut ibuf: Vec<(u64, (u32, u16, i8))> =
                Vec::with_capacity(if ib { IB_CHUNK } else { 0 });
            const RAD_CHUNK: usize = 1 << 22;
            let mut rbuf: Vec<(u64, (u32, u16, i8))> = Vec::new();
            let mut rtmp: Vec<(u64, (u32, u16, i8))> = Vec::new();
            const PF_DIST: usize = 32;
            let mut ring: std::collections::VecDeque<(u64, (u32, u16, i8))> =
                std::collections::VecDeque::with_capacity(PF_DIST + 1);
            let mut bscr: Vec<u64> = Vec::new();
            assert!((2..=5).contains(&k)); // k = 5 added for research/mt_ic_retune_20261115
            let kstar = mu * mu * ((n as f64) * targets as f64 / 2.0).sqrt();
            let classes = |m: usize| -> f64 {
                let mut c = 1.0;
                for i in 0..k {
                    c *= (m - i) as f64 / (i + 1) as f64;
                }
                c * (1u64 << (k - 1)) as f64
            };
            let mut m = k + 1;
            while classes(m) < kstar {
                m += 1;
            }
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            let ms_at_fb = t0.elapsed().as_secs_f64() * 1e3;
            // (k-1)-term full points: (point, indices, signs); first sign +1.
            let mut combos: Vec<(Pt, Vec<u16>, Vec<i8>)> = (0..m)
                .map(|i| (Some(fb[i]), vec![i as u16], vec![1i8]))
                .collect();
            let mut cdens: Vec<u64> = Vec::with_capacity(m);
            for _ in 2..k {
                let mut next = Vec::new();
                for (p, idx, sg) in &combos {
                    let last = *idx.last().unwrap() as usize;
                    // "cb": one simultaneous inversion per P over its l with x(F_l) != x(P)
                    let batched = match (*p, cb) {
                        (Some(pp), true) => {
                            cdens.clear();
                            for l in (last + 1)..m {
                                if fb[l].0 != pp.0 {
                                    cdens.push(f.sub(fb[l].0, pp.0));
                                }
                            }
                            f.batch_inv(&mut cdens, &mut bscr);
                            true
                        }
                        _ => false,
                    };
                    let mut j = 0usize;
                    for l in (last + 1)..m {
                        let (sum, dif) = if batched && fb[l].0 != p.unwrap().0 {
                            j += 1;
                            f.padd_sib_d(p.unwrap(), fb[l], cdens[j - 1])
                        } else {
                            f.padd_with_sibling(*p, Some(fb[l]))
                        };
                        for (pt, s) in [(sum, 1i8), (dif, -1i8)] {
                            if pt.is_some() {
                                let mut i2 = idx.clone();
                                i2.push(l as u16);
                                let mut s2 = sg.clone();
                                s2.push(s);
                                next.push((pt, i2, s2));
                            }
                        }
                    }
                }
                combos = next;
            }
            snap.push(("combos".into(), take(&f)));
            let ms_at_combos = t0.elapsed().as_secs_f64() * 1e3;
            // table: x -> (combo id, l, sign of F_l)
            let cap: usize = combos
                .iter()
                .map(|(_, idx, _)| 2 * (m - 1 - *idx.last().unwrap() as usize))
                .sum::<usize>()
                .max(kstar as usize + 16);
            let mut table = if tmode == "flatf" {
                Tab::new_fp(cap, p)
            } else if tmode == "flatfd" {
                Tab::new_fp_dense(cap, p)
            } else {
                Tab::new(flat, cap, p)
            };
            let mut put = |table: &mut Tab<(u32, u16, i8)>, x: u64, e: (u32, u16, i8)| {
                if ib {
                    ibuf.push((x, e));
                    if ibuf.len() >= IB_CHUNK {
                        table.insert_batch(&ibuf);
                        ibuf.clear();
                    }
                } else if rad {
                    rbuf.push((x, e));
                    if rbuf.len() >= RAD_CHUNK {
                        rad_flush(table, &mut rbuf, &mut rtmp);
                    }
                } else if pf {
                    table.prefetch(x);
                    ring.push_back((x, e));
                    if ring.len() > PF_DIST {
                        let (x0, e0) = ring.pop_front().unwrap();
                        table.insert_if_absent(x0, e0);
                    }
                } else {
                    table.insert_if_absent(x, e);
                }
            };
            let mut dens: Vec<u64> = Vec::with_capacity(m);
            let mut xq: Vec<(u32, u16, (u64, u64))> = Vec::with_capacity(2 * XB_MIN + m);
            for (ci_, (p, idx, _)) in combos.iter().enumerate() {
                let last = *idx.last().unwrap() as usize;
                if bi && xb {
                    // queue this combo's pairs; flush when enough denominators are pending (or at the last combo)
                    if let Some(pp) = *p {
                        for (l, fl) in fb.iter().enumerate().take(m).skip(last + 1) {
                            if fl.0 != pp.0 {
                                dens.push(f.sub(fl.0, pp.0));
                                xq.push((ci_ as u32, l as u16, pp));
                            }
                        }
                    }
                    if dens.len() >= XB_MIN || ci_ + 1 == combos.len() {
                        f.batch_inv(&mut dens, &mut bscr);
                        for (j, &(c, l, pp)) in xq.iter().enumerate() {
                            let (xp, xm) = f.xpair_d(pp, fb[l as usize], dens[j]);
                            f.hash();
                            put(&mut table, xp, (c, l, 1));
                            f.hash();
                            put(&mut table, xm, (c, l, -1));
                        }
                        dens.clear();
                        xq.clear();
                    }
                    continue;
                }
                if bi {
                    // one simultaneous inversion over this combo's l > last (xpair is None when P is None or x equal)
                    let pp = match *p {
                        Some(v) => v,
                        None => continue,
                    };
                    dens.clear();
                    for l in (last + 1)..m {
                        if fb[l].0 != pp.0 {
                            dens.push(f.sub(fb[l].0, pp.0));
                        }
                    }
                    f.batch_inv(&mut dens, &mut bscr);
                    let mut j = 0usize;
                    for l in (last + 1)..m {
                        if fb[l].0 == pp.0 {
                            continue;
                        }
                        let (xp, xm) = f.xpair_d(pp, fb[l], dens[j]);
                        j += 1;
                        f.hash();
                        put(&mut table, xp, (ci_ as u32, l as u16, 1));
                        f.hash();
                        put(&mut table, xm, (ci_ as u32, l as u16, -1));
                    }
                    continue;
                }
                for l in (last + 1)..m {
                    if let Some((xp, xm)) = f.xpair(*p, Some(fb[l])) {
                        f.hash();
                        put(&mut table, xp, (ci_ as u32, l as u16, 1));
                        f.hash();
                        put(&mut table, xm, (ci_ as u32, l as u16, -1));
                    }
                }
            }
            let _ = put; // end the closure's borrow of `ring`, `rbuf` and `ibuf`
            table.insert_batch(&ibuf);
            drop(ibuf);
            while let Some((x0, e0)) = ring.pop_front() {
                table.insert_if_absent(x0, e0);
            }
            rad_flush(&mut table, &mut rbuf, &mut rtmp);
            drop((rbuf, rtmp));
            snap.push(("table".into(), take(&f)));
            let ms_at_table = t0.elapsed().as_secs_f64() * 1e3;
            let entry_pt = |f: &F, e: (u32, u16, i8)| -> Pt {
                let fl = Some(fb[e.1 as usize]);
                f.padd(
                    combos[e.0 as usize].0,
                    if e.2 > 0 { fl } else { f.pneg(fl) },
                )
            };
            // coefficient vector of an entry: (index, sign) pairs
            let entry_terms = |e: (u32, u16, i8)| -> Vec<(usize, i64)> {
                let (_, idx, sg) = &combos[e.0 as usize];
                let mut v: Vec<(usize, i64)> = idx
                    .iter()
                    .zip(sg)
                    .map(|(&i, &s)| (i as usize, s as i64))
                    .collect();
                v.push((e.1 as usize, e.2 as i64));
                v
            };
            // counted arithmetic mod n
            let mulm = |a: u64, b: u64| -> u64 {
                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64
            };
            let invm = |a: u64| -> u64 {
                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, kk: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= kk {
                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);
                }
            };
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            // incremental echelon form: pivots[col] = Some(row) with row[col] = 1 (row has m+1 entries, last = rhs)
            let mut pivots: Vec<Option<Vec<u64>>> = vec![None; m];
            let mut rank = 0usize;
            let mut seen: HashMap<(u32, u16, i8), ()> = HashMap::default();
            let mut nrel = 0usize;
            let budget_rel = (gfac * m as f64).ceil() as usize;
            let mut p1_trials = 0u64;
            let mut kk = 0usize;
            let mut rblk: Vec<Option<(u64, u64)>> = Vec::with_capacity(kw);
            while rank < m {
                if kk == kw {
                    kk = 0;
                    v_pt = f.padd(v_pt, e_pt);
                    v_log = (v_log + e_log) % n;
                }
                p1_trials += 2;
                if rel_b && kk == 0 {
                    // "rel": the block's kw pairs share one simultaneous inversion; all probes prefetched
                    rblk.clear();
                    if let Some(vp) = v_pt {
                        dens.clear();
                        for w in &w_pts[..kw] {
                            if w.unwrap().0 != vp.0 {
                                dens.push(f.sub(w.unwrap().0, vp.0));
                            }
                        }
                        f.batch_inv(&mut dens, &mut bscr);
                        let mut j = 0usize;
                        for w in &w_pts[..kw] {
                            if w.unwrap().0 == vp.0 {
                                rblk.push(None);
                                continue;
                            }
                            let pr = f.xpair_d(vp, w.unwrap(), dens[j]);
                            j += 1;
                            table.prefetch(pr.0);
                            table.prefetch(pr.1);
                            rblk.push(Some(pr));
                        }
                    } else {
                        rblk.resize(kw, None);
                    }
                }
                let pair = if rel_b {
                    rblk[kk]
                } else {
                    f.xpair(v_pt, w_pts[kk])
                };
                if let Some((xp, xm)) = pair {
                    for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                        f.hash();
                        if let Some(e) = table.get(xx) {
                            f.hash();
                            if seen.insert(e, ()).is_some() {
                                continue;
                            }
                            let wp = if sw > 0 { w_pts[kk] } else { f.pneg(w_pts[kk]) };
                            let r = f.padd(v_pt, wp);
                            let eps: i64 = if r == entry_pt(&f, e) { 1 } else { -1 };
                            let mut row = vec![0u64; m + 1];
                            for (i, s) in entry_terms(e) {
                                row[i] = md((row[i] as i128) + (eps * s) as i128, n);
                            }
                            row[m] = md(v_log as i128 + sw as i128 * w_log[kk] as i128, n);
                            nrel += 1;
                            // reduce against pivots
                            for c in 0..m {
                                if row[c] == 0 {
                                    continue;
                                }
                                if let Some(pr) = &pivots[c] {
                                    let fct = row[c];
                                    for j in c..=m {
                                        if pr[j] != 0 {
                                            row[j] =
                                                md(row[j] as i128 - mulm(fct, pr[j]) as i128, n);
                                        }
                                    }
                                } else {
                                    let iv = invm(row[c]);
                                    for j in c..=m {
                                        if row[j] != 0 {
                                            row[j] = mulm(row[j], iv);
                                        }
                                    }
                                    pivots[c] = Some(row.clone());
                                    rank += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                kk += 1;
                if nrel > 50 * budget_rel + 1000 {
                    panic!("relation budget");
                }
            }
            // back substitution
            let mut ylog: Vec<u64> = vec![0; m];
            for c in (0..m).rev() {
                let pr = pivots[c].as_ref().unwrap();
                let mut v = pr[m] as i128;
                for j in (c + 1)..m {
                    if pr[j] != 0 {
                        v -= mulm(pr[j], ylog[j]) as i128;
                    }
                }
                ylog[c] = md(v, n);
            }
            snap.push(("relations".into(), take(&f)));
            let ms_at_relations = t0.elapsed().as_secs_f64() * 1e3;
            let mut trials = 0u64;
            let t_tgt = Instant::now();
            // lock-step batches: act[j] = (target, kq); a finished slot takes the next target
            let mut act: Vec<(usize, usize)> = Vec::with_capacity(bsz);
            let mut next_t = 0usize;
            while act.len() < bsz && next_t < targets {
                act.push((next_t, 0));
                next_t += 1;
            }
            // per slot: Err(log) = solved by the x(Q) = x(W_k) shortcut; Ok(None) = degenerate pair; Ok(Some(x pair))
            let mut prob: Vec<Result<Option<(u64, u64)>, u64>> = Vec::with_capacity(bsz);
            let mut vact: Vec<(usize, usize)> = Vec::with_capacity(bsz * blk_k);
            while !act.is_empty() {
                let kmax = act.iter().map(|a| a.1).max().unwrap() + blk_k - 1;
                ensure_w(&f, kmax, &mut w_pts, &mut w_log);
                prob.clear();
                vact.clear();
                for &(t, kq) in &act {
                    for j in 0..blk_k {
                        vact.push((t, kq + j));
                    }
                }
                if bi {
                    // q and w_pts[kq] are never None here (as in the plain path, which unwraps both)
                    dens.clear();
                    for &(t, kq) in &vact {
                        let (qx, wx) = (qs[t].unwrap().0, w_pts[kq].unwrap().0);
                        if qx != wx {
                            dens.push(f.sub(wx, qx));
                        }
                    }
                    f.batch_inv(&mut dens, &mut bscr);
                }
                let mut jd = 0usize;
                for &(t, kq) in &vact {
                    let q = qs[t];
                    if q.unwrap().0 == w_pts[kq].unwrap().0 {
                        prob.push(Err(if q == w_pts[kq] {
                            w_log[kq]
                        } else {
                            (n - w_log[kq]) % n
                        }));
                        continue;
                    }
                    let pr = if bi {
                        jd += 1;
                        Some(f.xpair_d(q.unwrap(), w_pts[kq].unwrap(), dens[jd - 1]))
                    } else {
                        f.xpair(q, w_pts[kq])
                    };
                    if let Some((xp, xm)) = pr {
                        table.prefetch(xp);
                        table.prefetch(xm);
                    }
                    prob.push(Ok(pr));
                }
                let mut nact: Vec<(usize, usize)> = Vec::with_capacity(bsz);
                for (ja, &(t, kq0)) in act.iter().enumerate() {
                    let mut solved: Option<u64> = None;
                    let mut kq = kq0;
                    for jb in 0..blk_k {
                        kq = kq0 + jb;
                        let j = ja * blk_k + jb;
                        trials += 2;
                        match prob[j] {
                            Err(v) => solved = Some(v),
                            Ok(Some((xp, xm))) => {
                                for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                                    f.hash();
                                    if let Some(e) = table.get(xx) {
                                        let dl = md(
                                            entry_terms(e)
                                                .iter()
                                                .map(|&(i, s)| s as i128 * ylog[i] as i128)
                                                .sum(),
                                            n,
                                        );
                                        let dpt2 = entry_pt(&f, e);
                                        solved = Some(resolve(
                                            &f, qs[t], w_pts[kq], sw, w_log[kq], dpt2, dl, n,
                                        ));
                                        break;
                                    }
                                }
                            }
                            Ok(None) => {}
                        }
                        if solved.is_some() {
                            break;
                        }
                    }
                    match solved {
                        Some(v) => {
                            found[t] = v;
                            if next_t < targets {
                                nact.push((next_t, 0));
                                next_t += 1;
                            }
                        }
                        None => {
                            if kq + 1 > 50_000_000 {
                                panic!("budget");
                            }
                            nact.push((t, kq + 1));
                        }
                    }
                }
                act = nact;
            }
            let ms_targets = t_tgt.elapsed().as_secs_f64() * 1e3;
            snap.push(("targets".into(), take(&f)));
            let _ = budget_rel;
            Out {
                ok: 0,
                extra: format!(
                    "\"k\":{},\"m\":{},\"combos\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"trials\":{},\"w_len\":{},\"table_mode\":\"{}\",\"inv\":\"{}\",\"pf\":{},\"opts\":\"{}\",\"batch\":{},\"ms_targets\":{:.3},\"ms_at_fb\":{:.3},\"ms_at_combos\":{:.3},\"ms_at_table\":{:.3},\"ms_at_relations\":{:.3}",
                    k, m, combos.len(), table.len(), nrel, p1_trials, trials, w_pts.len(),
                    tmode, if bi { "bi" } else { "plain" }, pf, opts.join(","), bsz, ms_targets, ms_at_fb, ms_at_combos, ms_at_table, ms_at_relations
                ),
            }
        }
        "ic5w" => {
            // k-term index calculus: table of x(F_{i1} + s2 F_{i2} + ... + s_k F_{ik}), i1 < ... < ik,
            // built as x(P +- F_l) from full (k-1)-term points P (shared inversion per pair);
            // factor-base logs by incremental Gaussian elimination mod n (every mod-n product counted
            // as M, every mod-n inverse as I); targets by paired trials x(Q +- W_k) as in ic4.
            let mu = param;
            let k: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            let gfac: f64 = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(1.2);
            assert!((2..=4).contains(&k));
            let kstar = mu * mu * ((n as f64) * targets as f64 / 2.0).sqrt();
            let classes = |m: usize| -> f64 {
                let mut c = 1.0;
                for i in 0..k {
                    c *= (m - i) as f64 / (i + 1) as f64;
                }
                c * (1u64 << (k - 1)) as f64
            };
            let mut m = k + 1;
            while classes(m) < kstar {
                m += 1;
            }
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // (k-1)-term full points: (point, indices, signs); first sign +1.
            let mut combos: Vec<(Pt, Vec<u16>, Vec<i8>)> = (0..m)
                .map(|i| (Some(fb[i]), vec![i as u16], vec![1i8]))
                .collect();
            for _ in 2..k {
                let mut next = Vec::new();
                for (p, idx, sg) in &combos {
                    let last = *idx.last().unwrap() as usize;
                    for l in (last + 1)..m {
                        let (sum, dif) = f.padd_with_sibling(*p, Some(fb[l]));
                        for (pt, s) in [(sum, 1i8), (dif, -1i8)] {
                            if pt.is_some() {
                                let mut i2 = idx.clone();
                                i2.push(l as u16);
                                let mut s2 = sg.clone();
                                s2.push(s);
                                next.push((pt, i2, s2));
                            }
                        }
                    }
                }
                combos = next;
            }
            snap.push(("combos".into(), take(&f)));
            // table: x -> (combo id, l, sign of F_l)
            let mut table: HashMap<u64, (u32, u16, i8)> =
                HashMap::with_capacity_and_hasher(kstar as usize + 16, Default::default());
            for (ci_, (p, idx, _)) in combos.iter().enumerate() {
                let last = *idx.last().unwrap() as usize;
                for l in (last + 1)..m {
                    if let Some((xp, xm)) = f.xpair(*p, Some(fb[l])) {
                        f.hash();
                        table.entry(xp).or_insert((ci_ as u32, l as u16, 1));
                        f.hash();
                        table.entry(xm).or_insert((ci_ as u32, l as u16, -1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u16, i8)| -> Pt {
                let fl = Some(fb[e.1 as usize]);
                f.padd(
                    combos[e.0 as usize].0,
                    if e.2 > 0 { fl } else { f.pneg(fl) },
                )
            };
            // coefficient vector of an entry: (index, sign) pairs
            let entry_terms = |e: (u32, u16, i8)| -> Vec<(usize, i64)> {
                let (_, idx, sg) = &combos[e.0 as usize];
                let mut v: Vec<(usize, i64)> = idx
                    .iter()
                    .zip(sg)
                    .map(|(&i, &s)| (i as usize, s as i64))
                    .collect();
                v.push((e.1 as usize, e.2 as i64));
                v
            };
            // counted arithmetic mod n
            let mulm = |a: u64, b: u64| -> u64 {
                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64
            };
            let invm = |a: u64| -> u64 {
                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let _ = dpt;
            const RWN: usize = 32;
            let rwd: Vec<(Pt, u64)> = (0..RWN)
                .map(|_| {
                    let c = rng.gen_range(1..n);
                    (f.smul(g, c), c)
                })
                .collect();
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, kk: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= kk {
                    let last = *w_pts.last().unwrap();
                    // step index from the position k (splitmix64), not from the point: a point-driven
                    // r-adding W walk is a rho walk and cycles after ~sqrt(n) terms (first attempt, 9 runs hit
                    // the trial budget); a position-driven sum never cycles
                    f.native();
                    let j = {
                        let mut z = (w_pts.len() as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
                        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                        ((z ^ (z >> 31)) >> 59) as usize
                    };
                    w_pts.push(f.padd(last, rwd[j].0));
                    w_log.push((w_log.last().unwrap() + rwd[j].1) % n);
                    let _ = dd;
                }
            };
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            // incremental echelon form: pivots[col] = Some(row) with row[col] = 1 (row has m+1 entries, last = rhs)
            let mut pivots: Vec<Option<Vec<u64>>> = vec![None; m];
            let mut rank = 0usize;
            let mut seen: HashMap<(u32, u16, i8), ()> = HashMap::default();
            let mut nrel = 0usize;
            let budget_rel = (gfac * m as f64).ceil() as usize;
            let mut p1_trials = 0u64;
            let mut kk = 0usize;
            while rank < m {
                if kk == kw {
                    kk = 0;
                    v_pt = f.padd(v_pt, e_pt);
                    v_log = (v_log + e_log) % n;
                }
                p1_trials += 2;
                if let Some((xp, xm)) = f.xpair(v_pt, w_pts[kk]) {
                    for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                        f.hash();
                        if let Some(&e) = table.get(&xx) {
                            f.hash();
                            if seen.insert(e, ()).is_some() {
                                continue;
                            }
                            let wp = if sw > 0 { w_pts[kk] } else { f.pneg(w_pts[kk]) };
                            let r = f.padd(v_pt, wp);
                            let eps: i64 = if r == entry_pt(&f, e) { 1 } else { -1 };
                            let mut row = vec![0u64; m + 1];
                            for (i, s) in entry_terms(e) {
                                row[i] = md((row[i] as i128) + (eps * s) as i128, n);
                            }
                            row[m] = md(v_log as i128 + sw as i128 * w_log[kk] as i128, n);
                            nrel += 1;
                            // reduce against pivots
                            for c in 0..m {
                                if row[c] == 0 {
                                    continue;
                                }
                                if let Some(pr) = &pivots[c] {
                                    let fct = row[c];
                                    for j in c..=m {
                                        if pr[j] != 0 {
                                            row[j] =
                                                md(row[j] as i128 - mulm(fct, pr[j]) as i128, n);
                                        }
                                    }
                                } else {
                                    let iv = invm(row[c]);
                                    for j in c..=m {
                                        if row[j] != 0 {
                                            row[j] = mulm(row[j], iv);
                                        }
                                    }
                                    pivots[c] = Some(row.clone());
                                    rank += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                kk += 1;
                if nrel > 50 * budget_rel + 1000 {
                    panic!("relation budget");
                }
            }
            // back substitution
            let mut ylog: Vec<u64> = vec![0; m];
            for c in (0..m).rev() {
                let pr = pivots[c].as_ref().unwrap();
                let mut v = pr[m] as i128;
                for j in (c + 1)..m {
                    if pr[j] != 0 {
                        v -= mulm(pr[j], ylog[j]) as i128;
                    }
                }
                ylog[c] = md(v, n);
            }
            snap.push(("relations".into(), take(&f)));
            let mut trials = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let v = 'tgt: loop {
                    ensure_w(&f, kq, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[kq].unwrap().0 {
                        break 'tgt if q == w_pts[kq] {
                            w_log[kq]
                        } else {
                            (n - w_log[kq]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                let dl = md(
                                    entry_terms(e)
                                        .iter()
                                        .map(|&(i, s)| s as i128 * ylog[i] as i128)
                                        .sum(),
                                    n,
                                );
                                let dpt2 = entry_pt(&f, e);
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                        }
                    }
                    kq += 1;
                    if kq > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            let _ = budget_rel;
            Out {
                ok: 0,
                extra: format!(
                    "\"k\":{},\"m\":{},\"combos\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"trials\":{},\"w_len\":{}",
                    k, m, combos.len(), table.len(), nrel, p1_trials, trials, w_pts.len()
                ),
            }
        }
        "ic5ks" => {
            // k-term index calculus: table of x(F_{i1} + s2 F_{i2} + ... + s_k F_{ik}), i1 < ... < ik,
            // built as x(P +- F_l) from full (k-1)-term points P (shared inversion per pair);
            // factor-base logs by incremental Gaussian elimination mod n (every mod-n product counted
            // as M, every mod-n inverse as I); targets by paired trials x(Q +- W_k) as in ic4.
            let mu = param;
            let k: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            let gfac: f64 = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(1.2);
            assert!((2..=4).contains(&k));
            let kstar = mu * mu * ((n as f64) * targets as f64 / 2.0).sqrt();
            let classes = |m: usize| -> f64 {
                let mut c = 1.0;
                for i in 0..k {
                    c *= (m - i) as f64 / (i + 1) as f64;
                }
                c * (1u64 << (k - 1)) as f64
            };
            let mut m = k + 1;
            while classes(m) < kstar {
                m += 1;
            }
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // (k-1)-term full points: (point, indices, signs); first sign +1.
            let mut combos: Vec<(Pt, Vec<u16>, Vec<i8>)> = (0..m)
                .map(|i| (Some(fb[i]), vec![i as u16], vec![1i8]))
                .collect();
            for _ in 2..k {
                let mut next = Vec::new();
                for (p, idx, sg) in &combos {
                    let last = *idx.last().unwrap() as usize;
                    for l in (last + 1)..m {
                        let (sum, dif) = f.padd_with_sibling(*p, Some(fb[l]));
                        for (pt, s) in [(sum, 1i8), (dif, -1i8)] {
                            if pt.is_some() {
                                let mut i2 = idx.clone();
                                i2.push(l as u16);
                                let mut s2 = sg.clone();
                                s2.push(s);
                                next.push((pt, i2, s2));
                            }
                        }
                    }
                }
                combos = next;
            }
            snap.push(("combos".into(), take(&f)));
            // table: x -> (combo id, l, sign of F_l)
            let mut table: HashMap<u64, (u32, u16, i8)> =
                HashMap::with_capacity_and_hasher(kstar as usize + 16, Default::default());
            for (ci_, (p, idx, _)) in combos.iter().enumerate() {
                let last = *idx.last().unwrap() as usize;
                for l in (last + 1)..m {
                    if let Some((xp, xm)) = f.xpair(*p, Some(fb[l])) {
                        f.hash();
                        table.entry(xp).or_insert((ci_ as u32, l as u16, 1));
                        f.hash();
                        table.entry(xm).or_insert((ci_ as u32, l as u16, -1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u16, i8)| -> Pt {
                let fl = Some(fb[e.1 as usize]);
                f.padd(
                    combos[e.0 as usize].0,
                    if e.2 > 0 { fl } else { f.pneg(fl) },
                )
            };
            // coefficient vector of an entry: (index, sign) pairs
            let entry_terms = |e: (u32, u16, i8)| -> Vec<(usize, i64)> {
                let (_, idx, sg) = &combos[e.0 as usize];
                let mut v: Vec<(usize, i64)> = idx
                    .iter()
                    .zip(sg)
                    .map(|(&i, &s)| (i as usize, s as i64))
                    .collect();
                v.push((e.1 as usize, e.2 as i64));
                v
            };
            // counted arithmetic mod n
            let mulm = |a: u64, b: u64| -> u64 {
                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64
            };
            let invm = |a: u64| -> u64 {
                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, kk: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= kk {
                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);
                }
            };
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            // incremental echelon form: pivots[col] = Some(row) with row[col] = 1 (row has m+1 entries, last = rhs)
            let mut pivots: Vec<Option<Vec<u64>>> = vec![None; m];
            let mut rank = 0usize;
            let mut seen: HashMap<(u32, u16, i8), ()> = HashMap::default();
            let mut nrel = 0usize;
            let budget_rel = (gfac * m as f64).ceil() as usize;
            let mut p1_trials = 0u64;
            let mut kk = 0usize;
            while rank < m {
                if kk == kw {
                    kk = 0;
                    v_pt = f.padd(v_pt, e_pt);
                    v_log = (v_log + e_log) % n;
                }
                p1_trials += 2;
                if let Some((xp, xm)) = f.xpair(v_pt, w_pts[kk]) {
                    for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                        f.hash();
                        if let Some(&e) = table.get(&xx) {
                            f.hash();
                            if seen.insert(e, ()).is_some() {
                                continue;
                            }
                            let wp = if sw > 0 { w_pts[kk] } else { f.pneg(w_pts[kk]) };
                            let r = f.padd(v_pt, wp);
                            let eps: i64 = if r == entry_pt(&f, e) { 1 } else { -1 };
                            let mut row = vec![0u64; m + 1];
                            for (i, s) in entry_terms(e) {
                                row[i] = md((row[i] as i128) + (eps * s) as i128, n);
                            }
                            row[m] = md(v_log as i128 + sw as i128 * w_log[kk] as i128, n);
                            nrel += 1;
                            // reduce against pivots
                            for c in 0..m {
                                if row[c] == 0 {
                                    continue;
                                }
                                if let Some(pr) = &pivots[c] {
                                    let fct = row[c];
                                    for j in c..=m {
                                        if pr[j] != 0 {
                                            row[j] =
                                                md(row[j] as i128 - mulm(fct, pr[j]) as i128, n);
                                        }
                                    }
                                } else {
                                    let iv = invm(row[c]);
                                    for j in c..=m {
                                        if row[j] != 0 {
                                            row[j] = mulm(row[j], iv);
                                        }
                                    }
                                    pivots[c] = Some(row.clone());
                                    rank += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                kk += 1;
                if nrel > 50 * budget_rel + 1000 {
                    panic!("relation budget");
                }
            }
            // back substitution
            let mut ylog: Vec<u64> = vec![0; m];
            for c in (0..m).rev() {
                let pr = pivots[c].as_ref().unwrap();
                let mut v = pr[m] as i128;
                for j in (c + 1)..m {
                    if pr[j] != 0 {
                        v -= mulm(pr[j], ylog[j]) as i128;
                    }
                }
                ylog[c] = md(v, n);
            }
            snap.push(("relations".into(), take(&f)));
            let mut trials = 0u64;
            let mut ret: HashMap<u64, (u32, u32, i8)> = HashMap::default();
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let mut trail: Vec<(u64, u32, i8)> = Vec::new();
                let v = 'tgt: loop {
                    ensure_w(&f, kq, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[kq].unwrap().0 {
                        break 'tgt if q == w_pts[kq] {
                            w_log[kq]
                        } else {
                            (n - w_log[kq]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            trail.push((xx, kq as u32, sw as i8));
                            if let Some(&(t2, k2, s2)) = ret.get(&xx) {
                                // Q + sw W_kq = eps (Q_t2 + s2 W_k2), Q_t2 solved: recompute the stored point (counted)
                                let wp2 = if s2 > 0 {
                                    w_pts[k2 as usize]
                                } else {
                                    f.pneg(w_pts[k2 as usize])
                                };
                                let dpt2 = f.padd(qs[t2 as usize], wp2);
                                let dl = md(
                                    found[t2 as usize] as i128
                                        + s2 as i128 * w_log[k2 as usize] as i128,
                                    n,
                                );
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                            if let Some(&e) = table.get(&xx) {
                                let dl = md(
                                    entry_terms(e)
                                        .iter()
                                        .map(|&(i, s)| s as i128 * ylog[i] as i128)
                                        .sum(),
                                    n,
                                );
                                let dpt2 = entry_pt(&f, e);
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                        }
                    }
                    kq += 1;
                    if kq > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (xx, kk2, s2) in trail {
                    f.hash();
                    ret.entry(xx).or_insert((t as u32, kk2, s2));
                }
            }
            snap.push(("targets".into(), take(&f)));
            let _ = budget_rel;
            Out {
                ok: 0,
                extra: format!(
                    "\"k\":{},\"m\":{},\"combos\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"trials\":{},\"w_len\":{}",
                    k, m, combos.len(), table.len(), nrel, p1_trials, trials, w_pts.len()
                ),
            }
        }
        "ic5rw" => {
            // k-term index calculus: table of x(F_{i1} + s2 F_{i2} + ... + s_k F_{ik}), i1 < ... < ik,
            // built as x(P +- F_l) from full (k-1)-term points P (shared inversion per pair);
            // factor-base logs by incremental Gaussian elimination mod n (every mod-n product counted
            // as M, every mod-n inverse as I); targets by paired trials x(Q +- W_k) as in ic4.
            let mu = param;
            let k: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            let gfac: f64 = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(1.2);
            assert!((2..=4).contains(&k));
            let kstar = mu * mu * ((n as f64) * targets as f64 / 2.0).sqrt();
            let classes = |m: usize| -> f64 {
                let mut c = 1.0;
                for i in 0..k {
                    c *= (m - i) as f64 / (i + 1) as f64;
                }
                c * (1u64 << (k - 1)) as f64
            };
            let mut m = k + 1;
            while classes(m) < kstar {
                m += 1;
            }
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // (k-1)-term full points: (point, indices, signs); first sign +1.
            let mut combos: Vec<(Pt, Vec<u16>, Vec<i8>)> = (0..m)
                .map(|i| (Some(fb[i]), vec![i as u16], vec![1i8]))
                .collect();
            for _ in 2..k {
                let mut next = Vec::new();
                for (p, idx, sg) in &combos {
                    let last = *idx.last().unwrap() as usize;
                    for l in (last + 1)..m {
                        let (sum, dif) = f.padd_with_sibling(*p, Some(fb[l]));
                        for (pt, s) in [(sum, 1i8), (dif, -1i8)] {
                            if pt.is_some() {
                                let mut i2 = idx.clone();
                                i2.push(l as u16);
                                let mut s2 = sg.clone();
                                s2.push(s);
                                next.push((pt, i2, s2));
                            }
                        }
                    }
                }
                combos = next;
            }
            snap.push(("combos".into(), take(&f)));
            // table: x -> (combo id, l, sign of F_l)
            let mut table: HashMap<u64, (u32, u16, i8)> =
                HashMap::with_capacity_and_hasher(kstar as usize + 16, Default::default());
            for (ci_, (p, idx, _)) in combos.iter().enumerate() {
                let last = *idx.last().unwrap() as usize;
                for l in (last + 1)..m {
                    if let Some((xp, xm)) = f.xpair(*p, Some(fb[l])) {
                        f.hash();
                        table.entry(xp).or_insert((ci_ as u32, l as u16, 1));
                        f.hash();
                        table.entry(xm).or_insert((ci_ as u32, l as u16, -1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u16, i8)| -> Pt {
                let fl = Some(fb[e.1 as usize]);
                f.padd(
                    combos[e.0 as usize].0,
                    if e.2 > 0 { fl } else { f.pneg(fl) },
                )
            };
            // coefficient vector of an entry: (index, sign) pairs
            let entry_terms = |e: (u32, u16, i8)| -> Vec<(usize, i64)> {
                let (_, idx, sg) = &combos[e.0 as usize];
                let mut v: Vec<(usize, i64)> = idx
                    .iter()
                    .zip(sg)
                    .map(|(&i, &s)| (i as usize, s as i64))
                    .collect();
                v.push((e.1 as usize, e.2 as i64));
                v
            };
            // counted arithmetic mod n
            let mulm = |a: u64, b: u64| -> u64 {
                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64
            };
            let invm = |a: u64| -> u64 {
                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let _ = dpt;
            const RWN: usize = 32;
            let rwd: Vec<(Pt, u64)> = (0..RWN)
                .map(|_| {
                    let c = rng.gen_range(1..n);
                    (f.smul(g, c), c)
                })
                .collect();
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, kk: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= kk {
                    let last = *w_pts.last().unwrap();
                    // step index from the position k (splitmix64), not from the point: a point-driven
                    // r-adding W walk is a rho walk and cycles after ~sqrt(n) terms (first attempt, 9 runs hit
                    // the trial budget); a position-driven sum never cycles
                    f.native();
                    let j = {
                        let mut z = (w_pts.len() as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
                        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                        ((z ^ (z >> 31)) >> 59) as usize
                    };
                    w_pts.push(f.padd(last, rwd[j].0));
                    w_log.push((w_log.last().unwrap() + rwd[j].1) % n);
                    let _ = dd;
                }
            };
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            // incremental echelon form: pivots[col] = Some(row) with row[col] = 1 (row has m+1 entries, last = rhs)
            let mut pivots: Vec<Option<Vec<u64>>> = vec![None; m];
            let mut rank = 0usize;
            let mut seen: HashMap<(u32, u16, i8), ()> = HashMap::default();
            let mut nrel = 0usize;
            let budget_rel = (gfac * m as f64).ceil() as usize;
            let mut p1_trials = 0u64;
            let mut kk = 0usize;
            while rank < m {
                if kk == kw {
                    kk = 0;
                    v_pt = f.padd(v_pt, e_pt);
                    v_log = (v_log + e_log) % n;
                }
                p1_trials += 2;
                if let Some((xp, xm)) = f.xpair(v_pt, w_pts[kk]) {
                    for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                        f.hash();
                        if let Some(&e) = table.get(&xx) {
                            f.hash();
                            if seen.insert(e, ()).is_some() {
                                continue;
                            }
                            let wp = if sw > 0 { w_pts[kk] } else { f.pneg(w_pts[kk]) };
                            let r = f.padd(v_pt, wp);
                            let eps: i64 = if r == entry_pt(&f, e) { 1 } else { -1 };
                            let mut row = vec![0u64; m + 1];
                            for (i, s) in entry_terms(e) {
                                row[i] = md((row[i] as i128) + (eps * s) as i128, n);
                            }
                            row[m] = md(v_log as i128 + sw as i128 * w_log[kk] as i128, n);
                            nrel += 1;
                            // reduce against pivots
                            for c in 0..m {
                                if row[c] == 0 {
                                    continue;
                                }
                                if let Some(pr) = &pivots[c] {
                                    let fct = row[c];
                                    for j in c..=m {
                                        if pr[j] != 0 {
                                            row[j] =
                                                md(row[j] as i128 - mulm(fct, pr[j]) as i128, n);
                                        }
                                    }
                                } else {
                                    let iv = invm(row[c]);
                                    for j in c..=m {
                                        if row[j] != 0 {
                                            row[j] = mulm(row[j], iv);
                                        }
                                    }
                                    pivots[c] = Some(row.clone());
                                    rank += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                kk += 1;
                if nrel > 50 * budget_rel + 1000 {
                    panic!("relation budget");
                }
            }
            // back substitution
            let mut ylog: Vec<u64> = vec![0; m];
            for c in (0..m).rev() {
                let pr = pivots[c].as_ref().unwrap();
                let mut v = pr[m] as i128;
                for j in (c + 1)..m {
                    if pr[j] != 0 {
                        v -= mulm(pr[j], ylog[j]) as i128;
                    }
                }
                ylog[c] = md(v, n);
            }
            snap.push(("relations".into(), take(&f)));
            let mut trials = 0u64;
            let mut ret: HashMap<u64, (u32, u32, i8)> = HashMap::default();
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let mut trail: Vec<(u64, u32, i8)> = Vec::new();
                let v = 'tgt: loop {
                    ensure_w(&f, kq, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[kq].unwrap().0 {
                        break 'tgt if q == w_pts[kq] {
                            w_log[kq]
                        } else {
                            (n - w_log[kq]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            trail.push((xx, kq as u32, sw as i8));
                            if let Some(&(t2, k2, s2)) = ret.get(&xx) {
                                // Q + sw W_kq = eps (Q_t2 + s2 W_k2), Q_t2 solved: recompute the stored point (counted)
                                let wp2 = if s2 > 0 {
                                    w_pts[k2 as usize]
                                } else {
                                    f.pneg(w_pts[k2 as usize])
                                };
                                let dpt2 = f.padd(qs[t2 as usize], wp2);
                                let dl = md(
                                    found[t2 as usize] as i128
                                        + s2 as i128 * w_log[k2 as usize] as i128,
                                    n,
                                );
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                            if let Some(&e) = table.get(&xx) {
                                let dl = md(
                                    entry_terms(e)
                                        .iter()
                                        .map(|&(i, s)| s as i128 * ylog[i] as i128)
                                        .sum(),
                                    n,
                                );
                                let dpt2 = entry_pt(&f, e);
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                        }
                    }
                    kq += 1;
                    if kq > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (xx, kk2, s2) in trail {
                    f.hash();
                    ret.entry(xx).or_insert((t as u32, kk2, s2));
                }
            }
            snap.push(("targets".into(), take(&f)));
            let _ = budget_rel;
            Out {
                ok: 0,
                extra: format!(
                    "\"k\":{},\"m\":{},\"combos\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"trials\":{},\"w_len\":{}",
                    k, m, combos.len(), table.len(), nrel, p1_trials, trials, w_pts.len()
                ),
            }
        }
        "rho" | "rho2" => {
            let sib = arm == "rho2";
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize {
                f.native();
                pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0)
            };
            // db: x -> (log, y or u64::MAX for a sibling without y)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64)> =
                HashMap::with_capacity_and_hasher(db_target * 2, Default::default());
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    // chained starts s <- s + B_k: one addition per restart and unboundedly many distinct
                    // starts (a fixed-bank triple sum capped the walks at C(66, 3) and could not fill large dbs)
                    let k1 = rng.gen_range(0..BANK);
                    sp = f.padd(sp, bank[k1].0);
                    sa = (sa + bank[k1].1) % n;
                    cur = sp;
                    ca = sa;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = if sib {
                    f.padd_with_sibling(prev, arms[h].0)
                } else {
                    (f.padd(prev, arms[h].0), None)
                };
                cur = nx;
                if let Some((xm, ym)) = sx {
                    f.hash();
                    db.entry(xm).or_insert(((ca + n - arms[h].1) % n, ym));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.entry(x).or_insert((ca, y));
                }
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let _ = prev;
                    let (nx, sx) = if sib {
                        f.padd_with_sibling(prev, arms[h].0)
                    } else {
                        (f.padd(prev, arms[h].0), None)
                    };
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    // main point: cur = ca G + Q
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2)) = db.get(&x) {
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if let Some((xm, ym)) = sx {
                        f.hash();
                        if let Some(&(a2, y2)) = db.get(&xm) {
                            // sibling: prev - A = (pa - a_A) G + Q, known in full
                            let eps: i128 = if ym == y2 { 1 } else { -1 };
                            break md(
                                eps * a2 as i128 - md(pa as i128 - arms[h].1 as i128, n) as i128,
                                n,
                            );
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }

        "rho2x" => {
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            const NOY: u64 = u64::MAX;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize {
                f.native();
                pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0)
            };
            // full P + A and x(P - A) with one inversion: I + 3M + 2S
            let step_x = |f: &F, p: Pt, q: Pt| -> (Pt, Option<u64>) {
                if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
                    if x1 != x2 {
                        let d = f.inv(f.sub(x2, x1));
                        let lam = f.mul(f.sub(y2, y1), d);
                        let x3 = f.sub(f.sub(f.sqr(lam), x1), x2);
                        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                        let lm = f.mul(f.neg(f.add(y2, y1)), d);
                        let xm = f.sub(f.sub(f.sqr(lm), x1), x2);
                        return (Some((x3, y3)), Some(xm));
                    }
                }
                (f.padd(p, q), None)
            };
            // db: x -> (log, y or NOY, (prev_x, prev_y, arm)) ; a NOY entry is prev - arms[arm], recomputed on a hit (counted)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64, (u64, u64, usize))> =
                HashMap::with_capacity_and_hasher(db_target * 2, Default::default());
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    // chained starts s <- s + B_k: one addition per restart and unboundedly many distinct
                    // starts (a fixed-bank triple sum capped the walks at C(66, 3) and could not fill large dbs)
                    let k1 = rng.gen_range(0..BANK);
                    sp = f.padd(sp, bank[k1].0);
                    sa = (sa + bank[k1].1) % n;
                    cur = sp;
                    ca = sa;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = step_x(&f, prev, arms[h].0);
                cur = nx;
                if let Some(xm) = sx {
                    let (px, py) = prev.unwrap();
                    f.hash();
                    db.entry(xm)
                        .or_insert(((ca + n - arms[h].1) % n, NOY, (px, py, h)));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.entry(x).or_insert((ca, y, (0, 0, 0)));
                }
            }
            snap.push(("db".into(), take(&f)));
            let full_y = |f: &F, y2: u64, aux: (u64, u64, usize)| -> u64 {
                if y2 != NOY {
                    return y2;
                }
                f.padd(Some((aux.0, aux.1)), f.pneg(arms[aux.2].0))
                    .unwrap()
                    .1
            };
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&x) {
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if let Some(xm) = sx {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&xm) {
                            let ym = f.padd(prev, f.pneg(arms[h].0)).unwrap().1; // target sibling in full (counted)
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if ym == y2 { 1 } else { -1 };
                            break md(
                                eps * a2 as i128 - md(pa as i128 - arms[h].1 as i128, n) as i128,
                                n,
                            );
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }

        "rho2xks" => {
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            const NOY: u64 = u64::MAX;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize {
                f.native();
                pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0)
            };
            // full P + A and x(P - A) with one inversion: I + 3M + 2S
            let step_x = |f: &F, p: Pt, q: Pt| -> (Pt, Option<u64>) {
                if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
                    if x1 != x2 {
                        let d = f.inv(f.sub(x2, x1));
                        let lam = f.mul(f.sub(y2, y1), d);
                        let x3 = f.sub(f.sub(f.sqr(lam), x1), x2);
                        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                        let lm = f.mul(f.neg(f.add(y2, y1)), d);
                        let xm = f.sub(f.sub(f.sqr(lm), x1), x2);
                        return (Some((x3, y3)), Some(xm));
                    }
                }
                (f.padd(p, q), None)
            };
            // db: x -> (log, y or NOY, (prev_x, prev_y, arm)) ; a NOY entry is prev - arms[arm], recomputed on a hit (counted)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64, (u64, u64, usize))> =
                HashMap::with_capacity_and_hasher(db_target * 2, Default::default());
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    // chained starts s <- s + B_k: one addition per restart and unboundedly many distinct
                    // starts (a fixed-bank triple sum capped the walks at C(66, 3) and could not fill large dbs)
                    let k1 = rng.gen_range(0..BANK);
                    sp = f.padd(sp, bank[k1].0);
                    sa = (sa + bank[k1].1) % n;
                    cur = sp;
                    ca = sa;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = step_x(&f, prev, arms[h].0);
                cur = nx;
                if let Some(xm) = sx {
                    let (px, py) = prev.unwrap();
                    f.hash();
                    db.entry(xm)
                        .or_insert(((ca + n - arms[h].1) % n, NOY, (px, py, h)));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.entry(x).or_insert((ca, y, (0, 0, 0)));
                }
            }
            snap.push(("db".into(), take(&f)));
            let full_y = |f: &F, y2: u64, aux: (u64, u64, usize)| -> u64 {
                if y2 != NOY {
                    return y2;
                }
                f.padd(Some((aux.0, aux.1)), f.pneg(arms[aux.2].0))
                    .unwrap()
                    .1
            };
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut trail: Vec<(u64, u64, u64, (u64, u64, usize))> = Vec::new();
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        trail.push((x, ca, y, (0, 0, 0)));
                    }
                    if let (Some(xm), Some((px, py))) = (sx, prev) {
                        trail.push((xm, (pa + n - arms[h].1) % n, NOY, (px, py, h)));
                    }
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&x) {
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if let Some(xm) = sx {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&xm) {
                            let ym = f.padd(prev, f.pneg(arms[h].0)).unwrap().1; // target sibling in full (counted)
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if ym == y2 { 1 } else { -1 };
                            break md(
                                eps * a2 as i128 - md(pa as i128 - arms[h].1 as i128, n) as i128,
                                n,
                            );
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (x, off, y, aux) in trail {
                    f.hash();
                    db.entry(x).or_insert(((off + v) % n, y, aux));
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }

        "rho2xksp" => {
            // rho2xks with the treatment ic5b's arm B got (research/mt_rho_rematch_20261115): the database is
            // a flat open-addressed table (Tab, as ic5b "flat"), and the target phase is pipelined by one step:
            // after a step its two probe slots are prefetched, the next step's arithmetic is done, and only then
            // are the previous step's lookups made. Targets stay sequential, so the retention of solved walks is
            // rho2xks's. A hit is therefore seen one step late: at most one extra step per target (counted).
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            const NOY: u64 = u64::MAX;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize {
                f.native();
                pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0)
            };
            // full P + A and x(P - A) with one inversion: I + 3M + 2S
            let step_x = |f: &F, p: Pt, q: Pt| -> (Pt, Option<u64>) {
                if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
                    if x1 != x2 {
                        let d = f.inv(f.sub(x2, x1));
                        let lam = f.mul(f.sub(y2, y1), d);
                        let x3 = f.sub(f.sub(f.sqr(lam), x1), x2);
                        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                        let lm = f.mul(f.neg(f.add(y2, y1)), d);
                        let xm = f.sub(f.sub(f.sqr(lm), x1), x2);
                        return (Some((x3, y3)), Some(xm));
                    }
                }
                (f.padd(p, q), None)
            };
            // db: x -> (log, y or NOY, (prev_x, prev_y, arm)) ; a NOY entry is prev - arms[arm], recomputed on a hit (counted)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            // capacity: the build fills db_target entries; retention adds up to the target trails
            let mut db: Tab<(u64, u64, (u64, u64, usize))> = Tab::new(true, db_target * 2, f.p);
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    // chained starts s <- s + B_k: one addition per restart and unboundedly many distinct
                    // starts (a fixed-bank triple sum capped the walks at C(66, 3) and could not fill large dbs)
                    let k1 = rng.gen_range(0..BANK);
                    sp = f.padd(sp, bank[k1].0);
                    sa = (sa + bank[k1].1) % n;
                    cur = sp;
                    ca = sa;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = step_x(&f, prev, arms[h].0);
                cur = nx;
                if let Some(xm) = sx {
                    let (px, py) = prev.unwrap();
                    f.hash();
                    db.insert_if_absent(xm, ((ca + n - arms[h].1) % n, NOY, (px, py, h)));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.insert_if_absent(x, (ca, y, (0, 0, 0)));
                }
            }
            snap.push(("db".into(), take(&f)));
            let full_y = |f: &F, y2: u64, aux: (u64, u64, usize)| -> u64 {
                if y2 != NOY {
                    return y2;
                }
                f.padd(Some((aux.0, aux.1)), f.pneg(arms[aux.2].0))
                    .unwrap()
                    .1
            };
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut trail: Vec<(u64, u64, u64, (u64, u64, usize))> = Vec::new();
                // the previous step's lookups, made one step late: (x, y, ca) of the new point and
                // (xm, prev, pa, h) of its x-only sibling
                let mut prev_look: Option<(
                    Option<(u64, u64, u64)>,
                    Option<(u64, Pt, u64, usize)>,
                )> = None;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        trail.push((x, ca, y, (0, 0, 0)));
                    }
                    if let (Some(xm), Some((px, py))) = (sx, prev) {
                        trail.push((xm, (pa + n - arms[h].1) % n, NOY, (px, py, h)));
                    }
                    if let Some((x, _)) = cur {
                        db.prefetch(x);
                    }
                    if let Some(xm) = sx {
                        db.prefetch(xm);
                    }
                    let this_look = (cur.map(|(x, y)| (x, y, ca)), sx.map(|xm| (xm, prev, pa, h)));
                    if let Some((cl, sl)) = prev_look.take() {
                        if let Some((x, y, cca)) = cl {
                            f.hash();
                            if let Some((a2, y2, aux)) = db.get(x) {
                                let y2 = full_y(&f, y2, aux);
                                let eps: i128 = if y == y2 { 1 } else { -1 };
                                break md(eps * a2 as i128 - cca as i128, n);
                            }
                        }
                        if let Some((xm, sprev, spa, sh)) = sl {
                            f.hash();
                            if let Some((a2, y2, aux)) = db.get(xm) {
                                let ym = f.padd(sprev, f.pneg(arms[sh].0)).unwrap().1;
                                let y2 = full_y(&f, y2, aux);
                                let eps: i128 = if ym == y2 { 1 } else { -1 };
                                break md(
                                    eps * a2 as i128
                                        - md(spa as i128 - arms[sh].1 as i128, n) as i128,
                                    n,
                                );
                            }
                        }
                    }
                    prev_look = Some(this_look);
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (x, off, y, aux) in trail {
                    f.hash();
                    db.insert_if_absent(x, ((off + v) % n, y, aux));
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }

        "rhoseq" => {
            // validator control: anchored rho with a sequential phase-1 database {G, 2G, ..., NG} (no walk waste, no restarts);
            // phase 2 identical to "rho".
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize {
                f.native();
                pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0)
            };
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64)> =
                HashMap::with_capacity_and_hasher(db_target * 2, Default::default());
            let mut cur: Pt = g;
            let mut ca = 1u64;
            let mut db_steps = 0u64;
            while db.len() < db_target {
                let (x, y) = cur.unwrap();
                f.hash();
                db.entry(x).or_insert((ca, y));
                cur = f.padd(cur, g);
                ca += 1;
                db_steps += 1;
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    cur = f.padd(cur, arms[h].0);
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2)) = db.get(&x) {
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }
        "rhodp" => {
            // Multi-target distinguished-point rho on {P, -P} classes (Kuhn-Struik with retention of solved
            // walks, optional precomputation). Walk: P -> canon(P + A_h(P)), R = 1024 steps A_i = B_j + B_k from
            // the 64-point bank; if the next point would use the same step index (a fruitless 2-cycle risk),
            // the step A_(h+1) is used instead (the extra addition is counted). A point is distinguished when
            // d hash bits are zero (theta = 2^d steps per walk on average); walks longer than 20 theta are
            // abandoned (their steps stay counted). param = kappa: precomputation walks from known-log starts
            // until kappa sqrt(nT) steps are spent; args[6] = offset of d from d0 = round(log2(n/T)/4).
            // Each target walks from chained starts Q + s G until a distinguished point is found in the
            // database (solved) or in its own earlier walks with the opposite Q sign (solved); otherwise it is
            // kept pending. After a target is solved, its pending distinguished points are inserted with known
            // logs. Every addition is counted; every database or pending lookup and insert is one hash op.
            const BANK: usize = 64;
            const R: usize = 1024;
            let kappa = param;
            let doff: i32 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(0);
            let d0 = (0.25 * ((n as f64) / targets as f64).log2()).round() as i32;
            let d = (d0 + doff).max(1) as u32;
            let mask = (1u64 << d) - 1;
            let theta = 1u64 << d;
            let maxwalk = 20 * theta;
            let half = f.p / 2;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let mut arms: Vec<(Pt, u64)> = Vec::with_capacity(R);
            for _ in 0..R {
                let (i, j) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                arms.push((f.padd(bank[i].0, bank[j].0), (bank[i].1 + bank[j].1) % n));
            }
            let mix = |x: u64| -> u64 { x.wrapping_mul(0x9E37_79B9_7F4A_7C15) };
            let idx = |p: Pt| -> usize { p.map(|(x, _)| (mix(x) >> 40) as usize % R).unwrap_or(0) };
            let is_dp =
                |p: Pt| -> bool { p.map(|(x, _)| (mix(x) >> 8) & mask == 0).unwrap_or(false) };
            // canonical class representative (y <= p/2); returns the sign applied
            // native work per generated point: one canonicalisation and one hash of x (the step index and the
            // distinguished-point test are bits of that one hash, so they are not counted again)
            let canon = |p: Pt| -> (Pt, i64) {
                f.native();
                f.native();
                match p {
                    Some((x, y)) if y > half => (Some((x, f.p - y)), -1),
                    _ => (p, 1),
                }
            };
            // one walk step on (point, a, b) with point = a G + b Q canonical
            let step = |f: &F, p: Pt, a: u64, b: i64| -> (Pt, u64, i64) {
                let h = idx(p);
                let (mut q1, mut s1) = canon(f.padd(p, arms[h].0));
                let mut hh = h;
                if q1.is_some() && idx(q1) == h {
                    hh = (h + 1) % R;
                    let r = canon(f.padd(p, arms[hh].0));
                    q1 = r.0;
                    s1 = r.1;
                }
                let a1 = md(s1 as i128 * ((a + arms[hh].1) % n) as i128, n);
                (q1, a1, s1 * b)
            };
            let mut db: HashMap<u64, u64> = HashMap::default();
            let pre_budget = (kappa * ((n as f64) * targets as f64).sqrt()) as u64;
            let mut pre_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while pre_steps < pre_budget {
                let k1 = rng.gen_range(0..BANK);
                sp = f.padd(sp, bank[k1].0);
                sa = (sa + bank[k1].1) % n;
                let (mut p, s0) = canon(sp);
                let mut a = md(s0 as i128 * sa as i128, n);
                let mut len = 0u64;
                while p.is_some() && len < maxwalk {
                    let r = step(&f, p, a, 0);
                    p = r.0;
                    a = r.1;
                    len += 1;
                    if is_dp(p) {
                        f.hash();
                        db.entry(p.unwrap().0).or_insert(a);
                        break;
                    }
                }
                pre_steps += len;
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            let mut walks = 0u64;
            let mut abandoned = 0u64;
            let inv2 = n.div_ceil(2);
            for t in 0..targets {
                let q = qs[t];
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut pending: HashMap<u64, (u64, i64)> = HashMap::default();
                let v = 'tgt: loop {
                    let k1 = rng.gen_range(0..BANK);
                    tp = f.padd(tp, bank[k1].0);
                    ta = (ta + bank[k1].1) % n;
                    let (mut p, s0) = canon(tp);
                    let mut a = md(s0 as i128 * ta as i128, n);
                    let mut b = s0;
                    let mut len = 0u64;
                    walks += 1;
                    while p.is_some() && len < maxwalk {
                        let r = step(&f, p, a, b);
                        p = r.0;
                        a = r.1;
                        b = r.2;
                        len += 1;
                        if is_dp(p) {
                            let x = p.unwrap().0;
                            f.hash();
                            if let Some(&a2) = db.get(&x) {
                                // a G + b Q = a2 G
                                break 'tgt md(b as i128 * (a2 as i128 - a as i128), n);
                            }
                            f.hash();
                            if let Some(&(a2, b2)) = pending.get(&x) {
                                if b2 != b {
                                    // a + b q = a2 + b2 q, b2 = -b: q = (a2 - a) / (2 b)
                                    let num = md(a2 as i128 - a as i128, n);
                                    let qv = md(
                                        b as i128
                                            * ((num as u128 * inv2 as u128) % n as u128) as i128,
                                        n,
                                    );
                                    f.cm.set(f.cm.get() + 1);
                                    break 'tgt qv;
                                }
                            } else {
                                f.hash();
                                pending.insert(x, (a, b));
                            }
                            break;
                        }
                    }
                    steps_total += len;
                    if len >= maxwalk {
                        abandoned += 1;
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (x, (a, b)) in pending {
                    f.hash();
                    db.entry(x)
                        .or_insert(md(a as i128 + b as i128 * v as i128, n));
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"d\":{},\"pre_steps\":{},\"target_steps\":{},\"walks\":{},\"abandoned\":{}",
                    db.len(),
                    d,
                    pre_steps,
                    steps_total,
                    walks,
                    abandoned
                ),
            }
        }
        "rhodpu" => {
            // Multi-target distinguished-point rho on {P, -P} classes (Kuhn-Struik with retention of solved
            // walks, optional precomputation). Walk: P -> canon(P + A_h(P)), R = 1024 steps A_i = B_j + B_k from
            // the 64-point bank; if the next point would use the same step index (a fruitless 2-cycle risk),
            // the step A_(h+1) is used instead (the extra addition is counted). A point is distinguished when
            // d hash bits are zero (theta = 2^d steps per walk on average); walks longer than 20 theta are
            // abandoned (their steps stay counted). param = kappa: precomputation walks from known-log starts
            // until kappa sqrt(nT) steps are spent; args[6] = offset of d from d0 = round(log2(n/T)/4).
            // Each target walks from chained starts Q + s G until a distinguished point is found in the
            // database (solved) or in its own earlier walks with the opposite Q sign (solved); otherwise it is
            // kept pending. After a target is solved, its pending distinguished points are inserted with known
            // logs. Every addition is counted; every database or pending lookup and insert is one hash op.
            const BANK: usize = 64;
            const R: usize = 1024;
            let kappa = param;
            let doff: i32 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(0);
            let d0 = (0.25 * ((n as f64) / targets as f64).log2()).round() as i32;
            let d = (d0 + doff).max(1) as u32;
            let mask = (1u64 << d) - 1;
            let theta = 1u64 << d;
            let maxwalk = 20 * theta;
            let half = f.p / 2;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let mut arms: Vec<(Pt, u64)> = Vec::with_capacity(R);
            let mut used = std::collections::HashSet::new();
            while arms.len() < R {
                let (i, j) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                if i == j || !used.insert((i.min(j), i.max(j))) {
                    continue;
                }
                arms.push((f.padd(bank[i].0, bank[j].0), (bank[i].1 + bank[j].1) % n));
            }
            let mix = |x: u64| -> u64 { x.wrapping_mul(0x9E37_79B9_7F4A_7C15) };
            let idx = |p: Pt| -> usize { p.map(|(x, _)| (mix(x) >> 40) as usize % R).unwrap_or(0) };
            let is_dp =
                |p: Pt| -> bool { p.map(|(x, _)| (mix(x) >> 8) & mask == 0).unwrap_or(false) };
            // canonical class representative (y <= p/2); returns the sign applied
            // native work per generated point: one canonicalisation and one hash of x (the step index and the
            // distinguished-point test are bits of that one hash, so they are not counted again)
            let canon = |p: Pt| -> (Pt, i64) {
                f.native();
                f.native();
                match p {
                    Some((x, y)) if y > half => (Some((x, f.p - y)), -1),
                    _ => (p, 1),
                }
            };
            // one walk step on (point, a, b) with point = a G + b Q canonical
            let step = |f: &F, p: Pt, a: u64, b: i64| -> (Pt, u64, i64) {
                let h = idx(p);
                let (mut q1, mut s1) = canon(f.padd(p, arms[h].0));
                let mut hh = h;
                if q1.is_some() && idx(q1) == h {
                    hh = (h + 1) % R;
                    let r = canon(f.padd(p, arms[hh].0));
                    q1 = r.0;
                    s1 = r.1;
                }
                let a1 = md(s1 as i128 * ((a + arms[hh].1) % n) as i128, n);
                (q1, a1, s1 * b)
            };
            let mut db: HashMap<u64, u64> = HashMap::default();
            let pre_budget = (kappa * ((n as f64) * targets as f64).sqrt()) as u64;
            let mut pre_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while pre_steps < pre_budget {
                let k1 = rng.gen_range(0..BANK);
                sp = f.padd(sp, bank[k1].0);
                sa = (sa + bank[k1].1) % n;
                let (mut p, s0) = canon(sp);
                let mut a = md(s0 as i128 * sa as i128, n);
                let mut len = 0u64;
                while p.is_some() && len < maxwalk {
                    let r = step(&f, p, a, 0);
                    p = r.0;
                    a = r.1;
                    len += 1;
                    if is_dp(p) {
                        f.hash();
                        db.entry(p.unwrap().0).or_insert(a);
                        break;
                    }
                }
                pre_steps += len;
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            let mut walks = 0u64;
            let mut abandoned = 0u64;
            let inv2 = n.div_ceil(2);
            for t in 0..targets {
                let q = qs[t];
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut pending: HashMap<u64, (u64, i64)> = HashMap::default();
                let v = 'tgt: loop {
                    let k1 = rng.gen_range(0..BANK);
                    tp = f.padd(tp, bank[k1].0);
                    ta = (ta + bank[k1].1) % n;
                    let (mut p, s0) = canon(tp);
                    let mut a = md(s0 as i128 * ta as i128, n);
                    let mut b = s0;
                    let mut len = 0u64;
                    walks += 1;
                    while p.is_some() && len < maxwalk {
                        let r = step(&f, p, a, b);
                        p = r.0;
                        a = r.1;
                        b = r.2;
                        len += 1;
                        if is_dp(p) {
                            let x = p.unwrap().0;
                            f.hash();
                            if let Some(&a2) = db.get(&x) {
                                // a G + b Q = a2 G
                                break 'tgt md(b as i128 * (a2 as i128 - a as i128), n);
                            }
                            f.hash();
                            if let Some(&(a2, b2)) = pending.get(&x) {
                                if b2 != b {
                                    // a + b q = a2 + b2 q, b2 = -b: q = (a2 - a) / (2 b)
                                    let num = md(a2 as i128 - a as i128, n);
                                    let qv = md(
                                        b as i128
                                            * ((num as u128 * inv2 as u128) % n as u128) as i128,
                                        n,
                                    );
                                    f.cm.set(f.cm.get() + 1);
                                    break 'tgt qv;
                                }
                            } else {
                                f.hash();
                                pending.insert(x, (a, b));
                            }
                            break;
                        }
                    }
                    steps_total += len;
                    if len >= maxwalk {
                        abandoned += 1;
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (x, (a, b)) in pending {
                    f.hash();
                    db.entry(x)
                        .or_insert(md(a as i128 + b as i128 * v as i128, n));
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"d\":{},\"pre_steps\":{},\"target_steps\":{},\"walks\":{},\"abandoned\":{}",
                    db.len(),
                    d,
                    pre_steps,
                    steps_total,
                    walks,
                    abandoned
                ),
            }
        }
        "rhodpup" => {
            // rhodpu with the treatment ic5b's arm B got (research/mt_rho_rematch_20261115): the database and
            // each target's pending table are flat open-addressed tables (Tab, as ic5b "flat"), and each lookup
            // slot is prefetched as soon as the distinguished point is known. Everything else is rhodpu's: the
            // walks, the counted operations and the recovered logs are identical.
            // Multi-target distinguished-point rho on {P, -P} classes (Kuhn-Struik with retention of solved
            // walks, optional precomputation). Walk: P -> canon(P + A_h(P)), R = 1024 steps A_i = B_j + B_k from
            // the 64-point bank; if the next point would use the same step index (a fruitless 2-cycle risk),
            // the step A_(h+1) is used instead (the extra addition is counted). A point is distinguished when
            // d hash bits are zero (theta = 2^d steps per walk on average); walks longer than 20 theta are
            // abandoned (their steps stay counted). param = kappa: precomputation walks from known-log starts
            // until kappa sqrt(nT) steps are spent; args[6] = offset of d from d0 = round(log2(n/T)/4).
            // Each target walks from chained starts Q + s G until a distinguished point is found in the
            // database (solved) or in its own earlier walks with the opposite Q sign (solved); otherwise it is
            // kept pending. After a target is solved, its pending distinguished points are inserted with known
            // logs. Every addition is counted; every database or pending lookup and insert is one hash op.
            const BANK: usize = 64;
            const R: usize = 1024;
            let kappa = param;
            let doff: i32 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(0);
            let d0 = (0.25 * ((n as f64) / targets as f64).log2()).round() as i32;
            let d = (d0 + doff).max(1) as u32;
            let mask = (1u64 << d) - 1;
            let theta = 1u64 << d;
            let maxwalk = 20 * theta;
            let half = f.p / 2;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let mut arms: Vec<(Pt, u64)> = Vec::with_capacity(R);
            let mut used = std::collections::HashSet::new();
            while arms.len() < R {
                let (i, j) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                if i == j || !used.insert((i.min(j), i.max(j))) {
                    continue;
                }
                arms.push((f.padd(bank[i].0, bank[j].0), (bank[i].1 + bank[j].1) % n));
            }
            let mix = |x: u64| -> u64 { x.wrapping_mul(0x9E37_79B9_7F4A_7C15) };
            let idx = |p: Pt| -> usize { p.map(|(x, _)| (mix(x) >> 40) as usize % R).unwrap_or(0) };
            let is_dp =
                |p: Pt| -> bool { p.map(|(x, _)| (mix(x) >> 8) & mask == 0).unwrap_or(false) };
            // canonical class representative (y <= p/2); returns the sign applied
            // native work per generated point: one canonicalisation and one hash of x (the step index and the
            // distinguished-point test are bits of that one hash, so they are not counted again)
            let canon = |p: Pt| -> (Pt, i64) {
                f.native();
                f.native();
                match p {
                    Some((x, y)) if y > half => (Some((x, f.p - y)), -1),
                    _ => (p, 1),
                }
            };
            // one walk step on (point, a, b) with point = a G + b Q canonical
            let step = |f: &F, p: Pt, a: u64, b: i64| -> (Pt, u64, i64) {
                let h = idx(p);
                let (mut q1, mut s1) = canon(f.padd(p, arms[h].0));
                let mut hh = h;
                if q1.is_some() && idx(q1) == h {
                    hh = (h + 1) % R;
                    let r = canon(f.padd(p, arms[hh].0));
                    q1 = r.0;
                    s1 = r.1;
                }
                let a1 = md(s1 as i128 * ((a + arms[hh].1) % n) as i128, n);
                (q1, a1, s1 * b)
            };
            let db_cap = ((kappa * ((n as f64) * targets as f64).sqrt()) as usize >> d).max(1024)
                + 4 * targets;
            let mut db: Tab<u64> = Tab::new(true, db_cap, f.p);
            let pre_budget = (kappa * ((n as f64) * targets as f64).sqrt()) as u64;
            let mut pre_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while pre_steps < pre_budget {
                let k1 = rng.gen_range(0..BANK);
                sp = f.padd(sp, bank[k1].0);
                sa = (sa + bank[k1].1) % n;
                let (mut p, s0) = canon(sp);
                let mut a = md(s0 as i128 * sa as i128, n);
                let mut len = 0u64;
                while p.is_some() && len < maxwalk {
                    let r = step(&f, p, a, 0);
                    p = r.0;
                    a = r.1;
                    len += 1;
                    if is_dp(p) {
                        f.hash();
                        db.insert_if_absent(p.unwrap().0, a);
                        break;
                    }
                }
                pre_steps += len;
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            let mut walks = 0u64;
            let mut abandoned = 0u64;
            let inv2 = n.div_ceil(2);
            for t in 0..targets {
                let q = qs[t];
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut pending: Vec<(u64, (u64, i64))> = Vec::new();
                let mut ptab: Tab<(u64, i64)> = Tab::new(true, 64, f.p);
                let v = 'tgt: loop {
                    let k1 = rng.gen_range(0..BANK);
                    tp = f.padd(tp, bank[k1].0);
                    ta = (ta + bank[k1].1) % n;
                    let (mut p, s0) = canon(tp);
                    let mut a = md(s0 as i128 * ta as i128, n);
                    let mut b = s0;
                    let mut len = 0u64;
                    walks += 1;
                    while p.is_some() && len < maxwalk {
                        let r = step(&f, p, a, b);
                        p = r.0;
                        a = r.1;
                        b = r.2;
                        len += 1;
                        if is_dp(p) {
                            let x = p.unwrap().0;
                            db.prefetch(x);
                            ptab.prefetch(x);
                            f.hash();
                            if let Some(a2) = db.get(x) {
                                // a G + b Q = a2 G
                                break 'tgt md(b as i128 * (a2 as i128 - a as i128), n);
                            }
                            f.hash();
                            if let Some((a2, b2)) = ptab.get(x) {
                                if b2 != b {
                                    // a + b q = a2 + b2 q, b2 = -b: q = (a2 - a) / (2 b)
                                    let num = md(a2 as i128 - a as i128, n);
                                    let qv = md(
                                        b as i128
                                            * ((num as u128 * inv2 as u128) % n as u128) as i128,
                                        n,
                                    );
                                    f.cm.set(f.cm.get() + 1);
                                    break 'tgt qv;
                                }
                            } else {
                                f.hash();
                                ptab.insert_if_absent(x, (a, b));
                                pending.push((x, (a, b)));
                            }
                            break;
                        }
                    }
                    steps_total += len;
                    if len >= maxwalk {
                        abandoned += 1;
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (x, (a, b)) in pending {
                    f.hash();
                    db.insert_if_absent(x, md(a as i128 + b as i128 * v as i128, n));
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"d\":{},\"pre_steps\":{},\"target_steps\":{},\"walks\":{},\"abandoned\":{}",
                    db.len(),
                    d,
                    pre_steps,
                    steps_total,
                    walks,
                    abandoned
                ),
            }
        }
        "rhodpb" => {
            // rhodpup run as B lock-step lanes (args[7] = B, default 32) so that every round's point additions share
            // one Montgomery simultaneous inversion (research/mt_ic_scale_20261115): the standard parallel form of
            // multi-target distinguished-point rho. Each lane holds one target at a time with its own walk, pending
            // set and chained starts; a solved lane inserts its pending points with known logs (retention) and takes
            // the next target. Walk, step rule, distinguished-point rule, abandon rule and counted native work are
            // rhodpu's; the order in which targets meet the database differs, so this is a different sample of the
            // same algorithm, not a replay. Precomputation (kappa > 0) runs as B lock-step walks from known-log
            // chained starts until kappa sqrt(nT) steps are spent.
            const BANK: usize = 64;
            const R: usize = 1024;
            let kappa = param;
            let doff: i32 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(0);
            let lanes: usize = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(32).max(1);
            // args[8] = "st" (research/mt_st_20261119): single-target lanes. All lanes walk the current target, each
            // from its own random chained start, sharing one pending table; a collision of two lanes' walks with
            // opposite Q signs (or a database hit) solves it, and then every lane moves to the next target. Walk,
            // step, distinguished-point and abandon rules are rhodpb's.
            let st = args.get(8).map(|s| s.as_str()) == Some("st");
            let mut spt: Tab<(u64, i64)> = Tab::new(true, 64, f.p);
            let mut spending: Vec<(u64, (u64, i64))> = Vec::new();
            let d0 = (0.25 * ((n as f64) / targets as f64).log2()).round() as i32;
            let d = (d0 + doff).max(1) as u32;
            let mask = (1u64 << d) - 1;
            let theta = 1u64 << d;
            let maxwalk = 20 * theta;
            let half = f.p / 2;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let mut arms: Vec<(Pt, u64)> = Vec::with_capacity(R);
            let mut used = std::collections::HashSet::new();
            while arms.len() < R {
                let (i, j) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                if i == j || !used.insert((i.min(j), i.max(j))) {
                    continue;
                }
                arms.push((f.padd(bank[i].0, bank[j].0), (bank[i].1 + bank[j].1) % n));
            }
            let mix = |x: u64| -> u64 { x.wrapping_mul(0x9E37_79B9_7F4A_7C15) };
            let idx = |p: Pt| -> usize { p.map(|(x, _)| (mix(x) >> 40) as usize % R).unwrap_or(0) };
            let is_dp =
                |p: Pt| -> bool { p.map(|(x, _)| (mix(x) >> 8) & mask == 0).unwrap_or(false) };
            let canon = |p: Pt| -> (Pt, i64) {
                f.native();
                f.native();
                match p {
                    Some((x, y)) if y > half => (Some((x, f.p - y)), -1),
                    _ => (p, 1),
                }
            };
            // one batched round of additions lhs[j] + rhs[j]; pairs with equal x (or a None) use f.padd alone
            let mut dens: Vec<u64> = Vec::new();
            let mut scr: Vec<u64> = Vec::new();
            let mut add_batch = |f: &F, pairs: &[(Pt, Pt)], out: &mut Vec<Pt>| {
                dens.clear();
                for &(a, b) in pairs {
                    if let (Some(pa), Some(pb)) = (a, b) {
                        if pa.0 != pb.0 {
                            dens.push(f.sub(pb.0, pa.0));
                        }
                    }
                }
                f.batch_inv(&mut dens, &mut scr);
                out.clear();
                let mut j = 0usize;
                for &(a, b) in pairs {
                    match (a, b) {
                        (Some(pa), Some(pb)) if pa.0 != pb.0 => {
                            out.push(f.padd_d(pa, pb, dens[j]));
                            j += 1;
                        }
                        _ => out.push(f.padd(a, b)),
                    }
                }
            };
            let db_cap = ((kappa * ((n as f64) * targets as f64).sqrt()) as usize >> d).max(1024)
                + 4 * targets;
            let mut db: Tab<u64> = Tab::new(true, db_cap, f.p);
            let pre_budget = (kappa * ((n as f64) * targets as f64).sqrt()) as u64;
            let mut pre_steps = 0u64;
            let mut pairs: Vec<(Pt, Pt)> = Vec::with_capacity(lanes);
            let mut outs: Vec<Pt> = Vec::with_capacity(lanes);
            // lane state for walks: start chain (sp, sa); walk (p, a, b, len); mode 0 = needs a start, 1 = walking
            struct Lane {
                t: usize,
                sp: Pt,
                sa: u64,
                p: Pt,
                a: u64,
                b: i64,
                len: u64,
                walking: bool,
                h: usize,
                pending: Vec<(u64, (u64, i64))>,
                ptab: Option<Tab<(u64, i64)>>,
            }
            if pre_budget > 0 {
                let mut pl: Vec<Lane> = (0..lanes)
                    .map(|j| Lane {
                        t: 0,
                        sp: bank[j % BANK].0,
                        sa: bank[j % BANK].1,
                        p: None,
                        a: 0,
                        b: 0,
                        len: 0,
                        walking: false,
                        h: 0,
                        pending: Vec::new(),
                        ptab: None,
                    })
                    .collect();
                let mut active: Vec<usize> = (0..lanes).collect();
                let mut starting = true;
                while !active.is_empty() {
                    pairs.clear();
                    let mut kk: Vec<usize> = Vec::with_capacity(active.len());
                    for &j in &active {
                        let ln = &mut pl[j];
                        if !ln.walking {
                            let k1 = rng.gen_range(0..BANK);
                            kk.push(k1);
                            pairs.push((ln.sp, bank[k1].0));
                        } else {
                            kk.push(usize::MAX);
                            ln.h = idx(ln.p);
                            pairs.push((ln.p, arms[ln.h].0));
                        }
                    }
                    add_batch(&f, &pairs, &mut outs);
                    // second additions for the fruitless-cycle rule (rare)
                    let mut redo: Vec<usize> = Vec::new();
                    let mut res: Vec<(Pt, i64, usize)> = vec![(None, 0, 0); active.len()];
                    for (q, &j) in active.iter().enumerate() {
                        let ln = &mut pl[j];
                        if kk[q] != usize::MAX {
                            ln.sp = outs[q];
                            ln.sa = (ln.sa + bank[kk[q]].1) % n;
                            let (pp, s0) = canon(ln.sp);
                            ln.p = pp;
                            ln.a = md(s0 as i128 * ln.sa as i128, n);
                            ln.len = 0;
                            ln.walking = pp.is_some();
                            continue;
                        }
                        let (q1, s1) = canon(outs[q]);
                        if q1.is_some() && idx(q1) == ln.h {
                            redo.push(q);
                        } else {
                            res[q] = (q1, s1, ln.h);
                        }
                    }
                    if !redo.is_empty() {
                        pairs.clear();
                        for &q in &redo {
                            let ln = &pl[active[q]];
                            pairs.push((ln.p, arms[(ln.h + 1) % R].0));
                        }
                        add_batch(&f, &pairs, &mut outs);
                        for (r, &q) in redo.iter().enumerate() {
                            let (q1, s1) = canon(outs[r]);
                            res[q] = (q1, s1, (pl[active[q]].h + 1) % R);
                        }
                    }
                    let mut nactive = Vec::with_capacity(active.len());
                    for (q, &j) in active.iter().enumerate() {
                        let ln = &mut pl[j];
                        if kk[q] != usize::MAX {
                            if ln.walking || starting {
                                nactive.push(j);
                            }
                            continue;
                        }
                        let (q1, s1, hh) = res[q];
                        ln.a = md(s1 as i128 * ((ln.a + arms[hh].1) % n) as i128, n);
                        ln.p = q1;
                        ln.len += 1;
                        let mut end = false;
                        if is_dp(ln.p) {
                            f.hash();
                            db.insert_if_absent(ln.p.unwrap().0, ln.a);
                            end = true;
                        } else if ln.p.is_none() || ln.len >= maxwalk {
                            end = true;
                        }
                        if end {
                            pre_steps += ln.len;
                            ln.walking = false;
                            if pre_steps >= pre_budget {
                                starting = false;
                            }
                            if starting {
                                nactive.push(j);
                            }
                        } else {
                            nactive.push(j);
                        }
                    }
                    active = nactive;
                }
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            let mut walks = 0u64;
            let mut abandoned = 0u64;
            let inv2 = n.div_ceil(2);
            let mut next_t = 0usize;
            let mut tl: Vec<Lane> = Vec::with_capacity(lanes);
            if st {
                for _ in 0..lanes {
                    tl.push(Lane {
                        t: 0,
                        sp: qs[0],
                        sa: 0,
                        p: None,
                        a: 0,
                        b: 0,
                        len: 0,
                        walking: false,
                        h: 0,
                        pending: Vec::new(),
                        ptab: None,
                    });
                }
                next_t = 1;
            }
            while !st && tl.len() < lanes && next_t < targets {
                tl.push(Lane {
                    t: next_t,
                    sp: qs[next_t],
                    sa: 0,
                    p: None,
                    a: 0,
                    b: 0,
                    len: 0,
                    walking: false,
                    h: 0,
                    pending: Vec::new(),
                    ptab: Some(Tab::new(true, 64, f.p)),
                });
                next_t += 1;
            }
            let mut kk: Vec<usize> = Vec::with_capacity(lanes);
            let mut res: Vec<(Pt, i64, usize)> = Vec::with_capacity(lanes);
            let mut redo: Vec<usize> = Vec::new();
            while !tl.is_empty() {
                pairs.clear();
                kk.clear();
                for ln in tl.iter_mut() {
                    if !ln.walking {
                        let k1 = rng.gen_range(0..BANK);
                        kk.push(k1);
                        pairs.push((ln.sp, bank[k1].0));
                    } else {
                        kk.push(usize::MAX);
                        ln.h = idx(ln.p);
                        pairs.push((ln.p, arms[ln.h].0));
                    }
                }
                add_batch(&f, &pairs, &mut outs);
                res.clear();
                res.resize(tl.len(), (None, 0, 0));
                redo.clear();
                for (q, ln) in tl.iter_mut().enumerate() {
                    if kk[q] != usize::MAX {
                        ln.sp = outs[q];
                        ln.sa = (ln.sa + bank[kk[q]].1) % n;
                        let (pp, s0) = canon(ln.sp);
                        ln.p = pp;
                        ln.a = md(s0 as i128 * ln.sa as i128, n);
                        ln.b = s0;
                        ln.len = 0;
                        ln.walking = pp.is_some();
                        walks += 1;
                        continue;
                    }
                    let (q1, s1) = canon(outs[q]);
                    if q1.is_some() && idx(q1) == ln.h {
                        redo.push(q);
                    } else {
                        res[q] = (q1, s1, ln.h);
                    }
                }
                if !redo.is_empty() {
                    pairs.clear();
                    for &q in &redo {
                        let ln = &tl[q];
                        pairs.push((ln.p, arms[(ln.h + 1) % R].0));
                    }
                    add_batch(&f, &pairs, &mut outs);
                    for (r, &q) in redo.iter().enumerate() {
                        let (q1, s1) = canon(outs[r]);
                        res[q] = (q1, s1, (tl[q].h + 1) % R);
                    }
                }
                // prefetch the lookup slots of every lane that reached a distinguished point
                for (q, ln) in tl.iter().enumerate() {
                    if kk[q] == usize::MAX && is_dp(res[q].0) {
                        let x = res[q].0.unwrap().0;
                        db.prefetch(x);
                        if st {
                            spt.prefetch(x);
                        } else {
                            ln.ptab.as_ref().unwrap().prefetch(x);
                        }
                    }
                }
                let mut q = 0usize;
                let mut keep: Vec<bool> = vec![true; tl.len()];
                let mut refill: Vec<usize> = Vec::new();
                let mut st_solved = false;
                for ln in tl.iter_mut() {
                    if kk[q] != usize::MAX {
                        q += 1;
                        continue;
                    }
                    let (q1, s1, hh) = res[q];
                    ln.a = md(s1 as i128 * ((ln.a + arms[hh].1) % n) as i128, n);
                    ln.b *= s1;
                    ln.p = q1;
                    ln.len += 1;
                    let mut solved: Option<u64> = None;
                    let mut end = false;
                    if st && st_solved {
                        // the target was solved earlier in this round: this lane's step is spent and counted
                        // with its walk in the tally below
                        ln.len += 1;
                        q += 1;
                        continue;
                    }
                    if is_dp(ln.p) {
                        let x = ln.p.unwrap().0;
                        f.hash();
                        if let Some(a2) = db.get(x) {
                            solved = Some(md(ln.b as i128 * (a2 as i128 - ln.a as i128), n));
                        } else {
                            f.hash();
                            let pt = if st {
                                &mut spt
                            } else {
                                ln.ptab.as_mut().unwrap()
                            };
                            if let Some((a2, b2)) = pt.get(x) {
                                if b2 != ln.b {
                                    let num = md(a2 as i128 - ln.a as i128, n);
                                    let qv = md(
                                        ln.b as i128
                                            * ((num as u128 * inv2 as u128) % n as u128) as i128,
                                        n,
                                    );
                                    f.cm.set(f.cm.get() + 1);
                                    solved = Some(qv);
                                }
                            } else {
                                f.hash();
                                pt.insert_if_absent(x, (ln.a, ln.b));
                                if st {
                                    spending.push((x, (ln.a, ln.b)));
                                } else {
                                    ln.pending.push((x, (ln.a, ln.b)));
                                }
                            }
                        }
                        end = true;
                    } else if ln.p.is_none() || ln.len >= maxwalk {
                        end = true;
                    }
                    if end {
                        steps_total += ln.len;
                        if ln.len >= maxwalk {
                            abandoned += 1;
                        }
                        ln.walking = false;
                    }
                    if let Some(v) = solved {
                        found[ln.t] = v;
                        let pend = if st { &spending } else { &ln.pending };
                        for &(x, (a, b)) in pend {
                            f.hash();
                            db.insert_if_absent(x, md(a as i128 + b as i128 * v as i128, n));
                        }
                        if st {
                            st_solved = true;
                        } else {
                            refill.push(q);
                        }
                    }
                    q += 1;
                }
                if st && st_solved {
                    // count the unfinished walks' steps, then move every lane to the next target
                    for ln in tl.iter_mut() {
                        if ln.walking {
                            steps_total += ln.len;
                            ln.walking = false;
                        }
                    }
                    if next_t < targets {
                        for ln in tl.iter_mut() {
                            ln.t = next_t;
                            ln.sp = qs[next_t];
                            ln.sa = 0;
                        }
                        next_t += 1;
                        spt = Tab::new(true, 64, f.p);
                        spending.clear();
                    } else {
                        tl.clear();
                    }
                    continue;
                }
                for q in refill {
                    if next_t < targets {
                        let ln = &mut tl[q];
                        ln.t = next_t;
                        ln.sp = qs[next_t];
                        ln.sa = 0;
                        ln.walking = false;
                        ln.pending.clear();
                        ln.ptab = Some(Tab::new(true, 64, f.p));
                        next_t += 1;
                    } else {
                        keep[q] = false;
                    }
                }
                let mut it = keep.iter();
                tl.retain(|_| *it.next().unwrap());
                if steps_total > 4_000_000_000 {
                    panic!("budget");
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"d\":{},\"pre_steps\":{},\"target_steps\":{},\"walks\":{},\"abandoned\":{},\"lanes\":{},\"st\":{}",
                    db.len(),
                    d,
                    pre_steps,
                    steps_total,
                    walks,
                    abandoned,
                    lanes,
                    st
                ),
            }
        }
        "bsgs2" => {
            let mf = param;
            let mx_goal = (mf * ((n as f64) * targets as f64).sqrt() / 2.0).ceil() as u64;
            // baby classes |k| <= Mx, Mx = (2u+1)(v-1) + u
            let u = ((mx_goal as f64 / 2.0).sqrt().round() as u64).max(1);
            let v = ((mx_goal + 2 * u + 1) / (2 * u + 1)).max(1) + 1;
            let mx = (2 * u + 1) * (v - 1) + u;
            assert!(2 * mx + 1 < n, "baby set exceeds group");
            let mut a_pts: Vec<Pt> = vec![g];
            for _ in 1..u {
                let l = *a_pts.last().unwrap();
                a_pts.push(f.padd(l, g));
            }
            let ugen = f.padd(f.padd(a_pts[u as usize - 1], a_pts[u as usize - 1]), g); // (2u+1) G
            let mut b_pts: Vec<Pt> = vec![None];
            for _ in 1..v {
                let l = *b_pts.last().unwrap();
                b_pts.push(f.padd(l, ugen));
            }
            let mut baby: HashMap<u64, u64> =
                HashMap::with_capacity_and_hasher((2 * mx + 4) as usize, Default::default());
            for i in 0..u as usize {
                f.hash();
                baby.entry(a_pts[i].unwrap().0).or_insert(i as u64 + 1);
            }
            for j in 1..v as usize {
                let c = (2 * u + 1) * j as u64;
                f.hash();
                baby.entry(b_pts[j].unwrap().0).or_insert(c);
                for i in 0..u as usize {
                    if let Some((xp, xm)) = f.xpair(b_pts[j], a_pts[i]) {
                        f.hash();
                        baby.entry(xp).or_insert(c + i as u64 + 1);
                        f.hash();
                        baby.entry(xm).or_insert(c - i as u64 - 1);
                    }
                }
            }
            snap.push(("baby".into(), take(&f)));
            // recompute the baby point k G (|k| <= Mx) from the lists: one counted addition at most
            let baby_pt = |f: &F, k: u64| -> Pt {
                let w = 2 * u + 1;
                let j = (k + u) / w;
                let r = k as i64 - (j * w) as i64;
                if j == 0 {
                    a_pts[r as usize - 1]
                } else if r == 0 {
                    b_pts[j as usize]
                } else if r > 0 {
                    f.padd(b_pts[j as usize], a_pts[r as usize - 1])
                } else {
                    f.padd(b_pts[j as usize], f.pneg(a_pts[(-r) as usize - 1]))
                }
            };
            let stride = 2 * mx + 1;
            let spt = f.smul(g, stride);
            let mut h_pts: Vec<Pt> = vec![None, spt];
            let mut trials = 0u64;
            for t in 0..targets {
                let q = qs[t];
                trials += 1;
                f.hash();
                let mut v_found: Option<u64> = None;
                if let Some(&k) = baby.get(&q.unwrap().0) {
                    let kp = baby_pt(&f, k);
                    v_found = Some(if kp == q { k % n } else { (n - k % n) % n });
                }
                let mut kk = 1usize;
                while v_found.is_none() {
                    while h_pts.len() <= kk {
                        let l = *h_pts.last().unwrap();
                        h_pts.push(f.padd(l, spt));
                    }
                    trials += 2;
                    let hl = md(kk as i128 * stride as i128, n);
                    if q.unwrap().0 == h_pts[kk].unwrap().0 {
                        v_found = Some(if q == h_pts[kk] { hl } else { (n - hl) % n });
                    } else if let Some((xp, xm)) = f.xpair(q, h_pts[kk]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&k) = baby.get(&xx) {
                                let dpt = baby_pt(&f, k);
                                v_found = Some(resolve(&f, q, h_pts[kk], sw, hl, dpt, k % n, n));
                                break;
                            }
                        }
                    }
                    kk += 1;
                    if kk as u64 > n {
                        panic!("budget");
                    }
                }
                found[t] = v_found.unwrap();
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"u\":{},\"v\":{},\"mx\":{},\"baby\":{},\"giants\":{},\"trials\":{}",
                    u,
                    v,
                    mx,
                    baby.len(),
                    h_pts.len(),
                    trials
                ),
            }
        }
        "opcost" => {
            // Timed microbenchmark (no counted result): wall-clock ns per operation of this harness's own
            // implementations on this host, to price the section 5a band: field mul/sqr/inv, one affine
            // addition (= 1 GAE by definition: I + 2M + S), x-pair (shared inversion), std HashMap lookup and
            // insert at table sizes 2^10..2^24 (the harness's key/value shape), and the native operations
            // (step index x mod 32, canonicalisation + one 64-bit hash of x). A fast multiplicative hasher is
            // timed as a sensitivity (the harness itself uses the std hasher). param = repetition scale.
            use std::hint::black_box;
            let scale = param.max(0.01);
            let reps = |base: f64| -> u64 { ((base * scale) as u64).max(1000) };
            let time_ns = |n: u64, t: Instant| -> f64 { t.elapsed().as_nanos() as f64 / n as f64 };
            let p = f.p;
            let mut res: Vec<String> = Vec::new();
            // field ops
            let n1 = reps(2e7);
            let mut x = 12345u64 % p;
            let y = 987654321u64 % p;
            let t = Instant::now();
            for _ in 0..n1 {
                x = f.mul(black_box(x), y);
            }
            res.push(format!("\"mul_ns\":{:.3}", time_ns(n1, t)));
            black_box(x);
            let t = Instant::now();
            for _ in 0..n1 {
                x = f.sqr(black_box(x));
            }
            res.push(format!("\"sqr_ns\":{:.3}", time_ns(n1, t)));
            black_box(x);
            let n2 = reps(1e6);
            let t = Instant::now();
            for _ in 0..n2 {
                x = f.inv(black_box(x) | 1);
            }
            res.push(format!("\"inv_ns\":{:.3}", time_ns(n2, t)));
            black_box(x);
            // affine addition chain P <- P + Q (one GAE each)
            let q = f.smul(g, 7);
            let mut pt = f.smul(g, 3);
            let t = Instant::now();
            for _ in 0..n2 {
                pt = f.padd(black_box(pt), q);
                if pt.is_none() {
                    pt = g;
                }
            }
            let gae_ns = time_ns(n2, t);
            res.push(format!("\"padd_ns\":{:.3}", gae_ns));
            black_box(pt);
            let t = Instant::now();
            let mut acc = 0u64;
            for _ in 0..n2 {
                if let Some((a, b)) = f.xpair(black_box(pt), q) {
                    acc ^= a ^ b;
                }
                pt = f.padd(pt, q);
            }
            res.push(format!("\"xpair_plus_padd_ns\":{:.3}", time_ns(n2, t)));
            black_box(acc);
            // native ops
            let half = p / 2;
            let mix = |x: u64| -> u64 { x.wrapping_mul(0x9E37_79B9_7F4A_7C15) };
            let n3 = reps(5e7);
            let mut xs: Vec<(u64, u64)> = Vec::with_capacity(1 << 16);
            for _ in 0..(1 << 16) {
                xs.push((rng.gen_range(0..p), rng.gen_range(0..p)));
            }
            let t = Instant::now();
            let mut acc = 0u64;
            for i in 0..n3 {
                let (xx, yy) = xs[(i & 0xffff) as usize];
                let (cx, cy) = if black_box(yy) > half {
                    (xx, p - yy)
                } else {
                    (xx, yy)
                };
                acc = acc.wrapping_add(cx ^ cy ^ (mix(cx) >> 40));
            }
            res.push(format!("\"canon_hash_ns\":{:.3}", time_ns(n3, t)));
            let t = Instant::now();
            for i in 0..n3 {
                let (xx, _) = xs[(i & 0xffff) as usize];
                acc = acc.wrapping_add((black_box(xx) as usize % 32) as u64);
            }
            res.push(format!("\"step_index_ns\":{:.3}", time_ns(n3, t)));
            let t = Instant::now();
            for i in 0..n3 {
                let (xx, _) = xs[(i & 0xffff) as usize];
                acc = acc.wrapping_add(black_box(xx));
            }
            res.push(format!("\"loop_baseline_ns\":{:.3}", time_ns(n3, t)));
            black_box(acc);
            // hash tables: std (as in the harness) and a fast multiplicative hasher (sensitivity)
            #[derive(Default)]
            struct FastH(u64);
            impl std::hash::Hasher for FastH {
                fn finish(&self) -> u64 {
                    self.0
                }
                fn write(&mut self, b: &[u8]) {
                    for &c in b {
                        self.0 = (self.0 ^ c as u64).wrapping_mul(0x100_0000_01b3);
                    }
                }
                fn write_u64(&mut self, v: u64) {
                    self.0 = (v ^ (v >> 29)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                }
            }
            type Fast = std::hash::BuildHasherDefault<FastH>;
            let mut tabs: Vec<String> = Vec::new();
            for lg in [10u32, 12, 14, 16, 18, 20, 22, 24] {
                let size = 1usize << lg;
                let keys: Vec<u64> = (0..size).map(|_| rng.gen_range(0..p)).collect();
                let probes: Vec<u64> = (0..(1 << 20))
                    .map(|i| {
                        if i % 2 == 0 {
                            keys[rng.gen_range(0..size)]
                        } else {
                            rng.gen_range(0..p)
                        }
                    })
                    .collect();
                let nl = reps(4e6);
                // std hasher, value shape (u64, u64) as the rho databases
                let mut m: std::collections::HashMap<u64, (u64, u64)> =
                    std::collections::HashMap::with_capacity(size * 2);
                let t = Instant::now();
                for &k in &keys {
                    m.entry(k).or_insert((k, k));
                }
                let ins = time_ns(size as u64, t);
                let t = Instant::now();
                let mut hits = 0u64;
                for i in 0..nl {
                    if m.contains_key(&probes[(i as usize) & ((1 << 20) - 1)]) {
                        hits += 1;
                    }
                }
                let look = time_ns(nl, t);
                black_box(hits);
                drop(m);
                let mut m2: std::collections::HashMap<u64, (u64, u64), Fast> =
                    std::collections::HashMap::with_capacity_and_hasher(size * 2, Fast::default());
                let t = Instant::now();
                for &k in &keys {
                    m2.entry(k).or_insert((k, k));
                }
                let ins2 = time_ns(size as u64, t);
                let t = Instant::now();
                let mut hits2 = 0u64;
                for i in 0..nl {
                    if m2.contains_key(&probes[(i as usize) & ((1 << 20) - 1)]) {
                        hits2 += 1;
                    }
                }
                let look2 = time_ns(nl, t);
                black_box(hits2);
                tabs.push(format!(
                    "{{\"log2_size\":{},\"std_insert_ns\":{:.3},\"std_lookup_ns\":{:.3},\"fast_insert_ns\":{:.3},\"fast_lookup_ns\":{:.3}}}",
                    lg, ins, look, ins2, look2
                ));
            }
            res.push(format!("\"tables\":[{}]", tabs.join(",")));
            Out {
                ok: 0,
                extra: res.join(","),
            }
        }
        _ => panic!("unknown arm"),
    };
    // wall time of the method itself (everything before the shared verification pass)
    let ms_arm = t0.elapsed().as_secs_f64() * 1e3;
    // Verification (counted): every recovered log by double-and-add.
    let mut ok = out.ok;
    for t in 0..targets {
        if f.smul(g, found[t]) == qs[t] && found[t] == xs[t] % n {
            ok += 1;
        }
    }
    snap.push(("verify".into(), take(&f)));
    let _ = f.small(0);
    let phases: Vec<String> = snap
        .iter()
        .map(|(k, v)| format!("\"{}\":[{},{},{},{},{}]", k, v[0], v[1], v[2], v[3], v[4]))
        .collect();
    println!(
        "{{\"arm\":\"{}\",\"n\":{},\"p\":{},\"targets\":{},\"seed\":{},\"param\":{},\"ok\":{},\"I\":{},\"M\":{},\"S\":{},\"H\":{},\"L\":{},\"N\":{},\"phases\":{{{}}},{},\"ms_arm\":{:.1},\"ms\":{:.1}}}",
        arm, n, p, targets, seed, param, ok, f.ci.get(), f.cm.get(), f.cs.get(), f.ch.get(), f.cl.get(), f.cn.get(),
        phases.join(","), out.extra, ms_arm, t0.elapsed().as_secs_f64() * 1e3
    );
}
