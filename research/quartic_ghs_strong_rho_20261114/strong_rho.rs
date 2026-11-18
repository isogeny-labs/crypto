//! Strong Pollard rho for the odd-prime cubic GHS family used by N4.
//!
//! This is a new comparator.  It does not modify the frozen N4/N4-v2
//! sources.  Walkers advance in lockstep, sharing one Montgomery batch
//! inversion per round.  The extension-field multiplication uses six base
//! field products (Karatsuba), with multiplication by the small defining
//! constant implemented as additions.

use crate::common::{inv_mod_u64, Rng, K, KONE, KZERO};
use crate::ghs::{Inst, EP};
use std::cell::Cell;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FieldCounts {
    pub fq_mul: u64,
    pub fq_inv: u64,
    pub fq_add: u64,
    pub f3_mul: u64,
    pub f3_sq: u64,
    pub f3_inv: u64,
    pub batch_inv: u64,
    pub group_add: u64,
}

impl FieldCounts {
    pub fn since(self, before: Self) -> Self {
        Self {
            fq_mul: self.fq_mul - before.fq_mul,
            fq_inv: self.fq_inv - before.fq_inv,
            fq_add: self.fq_add - before.fq_add,
            f3_mul: self.f3_mul - before.f3_mul,
            f3_sq: self.f3_sq - before.f3_sq,
            f3_inv: self.f3_inv - before.f3_inv,
            batch_inv: self.batch_inv - before.batch_inv,
            group_add: self.group_add - before.group_add,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NativeCounts {
    pub rng_draws: u64,
    pub hashes: u64,
    pub canonicalisations: u64,
    pub lookahead_tests: u64,
    pub dp_tests: u64,
    pub table_ops: u64,
    pub cycle_compares: u64,
    pub scalar_adds: u64,
    pub scalar_muls: u64,
    pub scalar_invs: u64,
}

impl NativeCounts {
    pub fn since(self, before: Self) -> Self {
        Self {
            rng_draws: self.rng_draws - before.rng_draws,
            hashes: self.hashes - before.hashes,
            canonicalisations: self.canonicalisations - before.canonicalisations,
            lookahead_tests: self.lookahead_tests - before.lookahead_tests,
            dp_tests: self.dp_tests - before.dp_tests,
            table_ops: self.table_ops - before.table_ops,
            cycle_compares: self.cycle_compares - before.cycle_compares,
            scalar_adds: self.scalar_adds - before.scalar_adds,
            scalar_muls: self.scalar_muls - before.scalar_muls,
            scalar_invs: self.scalar_invs - before.scalar_invs,
        }
    }

    /// Operations admitted to the inherited N4-v2 section-5a band.
    ///
    /// Raw hashes implement partition/DP/table operations and are not charged
    /// again. RNG draws and scalar adds/inverses remain explicit diagnostics,
    /// matching the frozen treatment of `ladd` and collision inversion.
    pub fn band_ops(self) -> u64 {
        self.canonicalisations
            + self.lookahead_tests
            + self.dp_tests
            + self.table_ops
            + self.cycle_compares
    }
}

/// Native arithmetic for `F_q[z]/(z^3-c)`, `q = 7 mod 12`.
fn barrett_reciprocal(q: u64) -> u64 {
    assert!(
        (2..1u64 << 31).contains(&q),
        "strong-rho field modulus must satisfy 1 < q < 2^31"
    );
    (u128::from(u64::MAX) + 1).div_euclid(u128::from(q)) as u64
}

#[inline(always)]
fn barrett_reduce_product(q: u64, reciprocal: u64, x: u64) -> u64 {
    debug_assert!((2..1u64 << 31).contains(&q));
    debug_assert!(x < q * q);
    let quotient = ((u128::from(x) * u128::from(reciprocal)) >> 64) as u64;
    let residual = x - quotient * q;
    if residual >= q {
        residual - q
    } else {
        residual
    }
}

pub struct FastF3 {
    q: u64,
    reciprocal: u64,
    setup_reciprocal_div: u64,
    c: u64,
    w: u64,
    w2: u64,
    counts: Cell<FieldCounts>,
}

impl FastF3 {
    pub fn from_instance(inst: &Inst) -> Self {
        Self {
            q: inst.q,
            reciprocal: barrett_reciprocal(inst.q),
            // Cold, unpriced diagnostic: constructing the fixed-modulus
            // reducer evaluates one u128 division to obtain floor(2^64/q).
            setup_reciprocal_div: 1,
            c: inst.kf.c,
            w: inst.kf.w,
            w2: inst.kf.w2,
            counts: Cell::new(FieldCounts::default()),
        }
    }

    pub fn counts(&self) -> FieldCounts {
        self.counts.get()
    }

    fn bump(&self, f: impl FnOnce(&mut FieldCounts)) {
        let mut c = self.counts.get();
        f(&mut c);
        self.counts.set(c);
    }

    #[inline(always)]
    fn fa(&self, a: u64, b: u64) -> u64 {
        self.bump(|c| c.fq_add += 1);
        let s = a + b;
        if s >= self.q {
            s - self.q
        } else {
            s
        }
    }

    #[inline(always)]
    fn fs(&self, a: u64, b: u64) -> u64 {
        self.bump(|c| c.fq_add += 1);
        if a >= b {
            a - b
        } else {
            a + self.q - b
        }
    }

    #[inline(always)]
    fn fn_(&self, a: u64) -> u64 {
        self.bump(|c| c.fq_add += 1);
        if a == 0 {
            0
        } else {
            self.q - a
        }
    }

    #[inline(always)]
    fn fm(&self, a: u64, b: u64) -> u64 {
        self.bump(|c| c.fq_mul += 1);
        debug_assert!(a < self.q && b < self.q);
        // For R=2^64 and m=floor(R/q), floor(x*m/R) undershoots
        // floor(x/q) by at most one for x<R.  Here x=a*b<q^2<2^62,
        // so the residual is in [0,2q) and one subtraction is enough.
        let x = a * b;
        barrett_reduce_product(self.q, self.reciprocal, x)
    }

    fn fmul_small(&self, a: u64, k: u64) -> u64 {
        if a == 0 || k == 0 {
            return 0;
        }
        // Every N4-v2 instance has a tiny least non-cube.  Keep a correct
        // fallback, and count the operation actually executed.
        if k <= 16 {
            let (mut n, mut base, mut out) = (k, a, None);
            while n != 0 {
                if n & 1 == 1 {
                    out = Some(match out {
                        Some(r) => self.fa(r, base),
                        None => base,
                    });
                }
                n >>= 1;
                if n != 0 {
                    base = self.fa(base, base);
                }
            }
            out.unwrap_or(0)
        } else {
            self.fm(a, k)
        }
    }

    #[inline(always)]
    pub fn add(&self, a: &K, b: &K) -> K {
        [
            self.fa(a[0], b[0]),
            self.fa(a[1], b[1]),
            self.fa(a[2], b[2]),
        ]
    }

    #[inline(always)]
    pub fn sub(&self, a: &K, b: &K) -> K {
        [
            self.fs(a[0], b[0]),
            self.fs(a[1], b[1]),
            self.fs(a[2], b[2]),
        ]
    }

    #[inline(always)]
    pub fn neg(&self, a: &K) -> K {
        [self.fn_(a[0]), self.fn_(a[1]), self.fn_(a[2])]
    }

    fn mul_raw(&self, a: &K, b: &K) -> K {
        // Cubic Karatsuba: three diagonal and three cross products.
        let p0 = self.fm(a[0], b[0]);
        let p1 = self.fm(a[1], b[1]);
        let p2 = self.fm(a[2], b[2]);
        let s01 = self.fs(
            self.fs(self.fm(self.fa(a[0], a[1]), self.fa(b[0], b[1])), p0),
            p1,
        );
        let s12 = self.fs(
            self.fs(self.fm(self.fa(a[1], a[2]), self.fa(b[1], b[2])), p1),
            p2,
        );
        let s02 = self.fs(
            self.fs(self.fm(self.fa(a[0], a[2]), self.fa(b[0], b[2])), p0),
            p2,
        );
        [
            self.fa(p0, self.fmul_small(s12, self.c)),
            self.fa(s01, self.fmul_small(p2, self.c)),
            self.fa(s02, p1),
        ]
    }

    #[inline(always)]
    pub fn mul(&self, a: &K, b: &K) -> K {
        self.bump(|c| c.f3_mul += 1);
        self.mul_raw(a, b)
    }

    #[inline(always)]
    pub fn sq(&self, a: &K) -> K {
        self.bump(|c| c.f3_sq += 1);
        // Five-product cubic squaring for z^3 = c.  With
        //
        //   d1 = 2*a0*a1, d3 = 2*a1*a2,
        //   d2 = a1^2 + 2*a0*a2,
        //
        // compute d2 from (a0+a1+a2)^2, avoiding separate products for
        // a1^2 and a0*a2.  This saves one priced F_q multiplication versus
        // the six-product generic Karatsuba path while preserving the same
        // two small-constant reductions.
        let m0 = self.fm(a[0], a[0]);
        let m2 = self.fm(a[2], a[2]);
        let m01 = self.fm(a[0], a[1]);
        let m12 = self.fm(a[1], a[2]);
        let sum = self.fa(self.fa(a[0], a[1]), a[2]);
        let ms = self.fm(sum, sum);
        let d1 = self.fa(m01, m01);
        let d3 = self.fa(m12, m12);
        let d2 = self.fs(self.fs(self.fs(self.fs(ms, m0), m2), d1), d3);
        [
            self.fa(m0, self.fmul_small(d3, self.c)),
            self.fa(d1, self.fmul_small(m2, self.c)),
            d2,
        ]
    }

    fn frob(&self, a: &K) -> K {
        [a[0], self.fm(a[1], self.w), self.fm(a[2], self.w2)]
    }

    pub fn inv(&self, a: &K) -> K {
        assert_ne!(*a, KZERO, "inverse of zero");
        self.bump(|c| {
            c.f3_inv += 1;
            c.fq_inv += 1;
        });
        // a^-1 = a^q a^(q^2) / Norm(a).
        let a1 = self.frob(a);
        let a2 = self.frob(&a1);
        let p = self.mul(&a1, &a2);
        let n = self.fa(
            self.fm(a[0], p[0]),
            self.fmul_small(self.fa(self.fm(a[1], p[2]), self.fm(a[2], p[1])), self.c),
        );
        let ni = inv_mod_u64(n, self.q);
        [self.fm(p[0], ni), self.fm(p[1], ni), self.fm(p[2], ni)]
    }

    fn batch_inv_into(&self, values: &[K], prefix: &mut Vec<K>, out: &mut Vec<K>) {
        assert!(values.iter().all(|v| *v != KZERO));
        prefix.clear();
        out.clear();
        if values.is_empty() {
            return;
        }
        self.bump(|c| c.batch_inv += 1);
        if values.len() == 1 {
            out.push(self.inv(&values[0]));
            return;
        }
        let mut acc = values[0];
        prefix.push(acc);
        for v in &values[1..] {
            acc = self.mul(&acc, v);
            prefix.push(acc);
        }
        let mut inv_acc = self.inv(&acc);
        out.resize(values.len(), KZERO);
        for i in (1..values.len()).rev() {
            out[i] = self.mul(&inv_acc, &prefix[i - 1]);
            inv_acc = self.mul(&inv_acc, &values[i]);
        }
        out[0] = inv_acc;
    }

    pub fn batch_inv(&self, values: &[K]) -> Vec<K> {
        let mut prefix = Vec::with_capacity(values.len());
        let mut out = Vec::with_capacity(values.len());
        self.batch_inv_into(values, &mut prefix, &mut out);
        out
    }
}

#[derive(Default)]
struct BatchAddScratch {
    active: Vec<usize>,
    den: Vec<K>,
    prefix: Vec<K>,
    inv: Vec<K>,
}

impl BatchAddScratch {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            active: Vec::with_capacity(capacity),
            den: Vec::with_capacity(capacity),
            prefix: Vec::with_capacity(capacity),
            inv: Vec::with_capacity(capacity),
        }
    }
}

pub struct FastEc<'a> {
    f: &'a FastF3,
    a2: K,
    a4: K,
}

impl<'a> FastEc<'a> {
    pub fn from_instance(f: &'a FastF3, inst: &Inst) -> Self {
        Self {
            f,
            a2: inst.ec.a2,
            a4: inst.ec.a4,
        }
    }

    pub fn add(&self, p: &EP, q: &EP) -> EP {
        self.f.bump(|c| c.group_add += 1);
        match (p, q) {
            (EP::Inf, _) => *q,
            (_, EP::Inf) => *p,
            (EP::A(x1, y1), EP::A(x2, y2)) => {
                let (num, den) = if x1 == x2 {
                    if self.f.add(y1, y2) == KZERO {
                        return EP::Inf;
                    }
                    let x12 = self.f.sq(x1);
                    let three = self.f.add(&self.f.add(&x12, &x12), &x12);
                    (
                        self.f.add(
                            &self
                                .f
                                .add(&three, &self.f.mul(&self.f.add(&self.a2, &self.a2), x1)),
                            &self.a4,
                        ),
                        self.f.add(y1, y1),
                    )
                } else {
                    (self.f.sub(y2, y1), self.f.sub(x2, x1))
                };
                let inv = self.f.inv(&den);
                self.add_with_inv_uncounted(p, q, &inv, &num)
            }
        }
    }

    fn add_with_inv_uncounted(&self, p: &EP, q: &EP, inv: &K, num: &K) -> EP {
        let (EP::A(x1, y1), EP::A(x2, _)) = (p, q) else {
            unreachable!()
        };
        let lam = self.f.mul(num, inv);
        let x3 = self
            .f
            .sub(&self.f.sub(&self.f.sub(&self.f.sq(&lam), &self.a2), x1), x2);
        let y3 = self.f.sub(&self.f.mul(&lam, &self.f.sub(x1, &x3)), y1);
        EP::A(x3, y3)
    }

    fn mixed_with_inv(&self, p: &EP, q: &EP, inv: &K) -> EP {
        self.f.bump(|c| c.group_add += 1);
        let (EP::A(_, y1), EP::A(_, y2)) = (p, q) else {
            unreachable!()
        };
        let num = self.f.sub(y2, y1);
        self.add_with_inv_uncounted(p, q, inv, &num)
    }

    pub fn mul(&self, p: &EP, mut k: u128) -> EP {
        let mut r = EP::Inf;
        let mut b = *p;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            k >>= 1;
            if k > 0 {
                b = self.add(&b, &b);
            }
        }
        r
    }

    fn batch_add_into(
        &self,
        points: &[EP],
        jumps: &[EP],
        out: &mut Vec<EP>,
        scratch: &mut BatchAddScratch,
    ) {
        assert_eq!(points.len(), jumps.len());
        out.clear();
        out.resize(points.len(), EP::Inf);
        scratch.active.clear();
        scratch.den.clear();
        for i in 0..points.len() {
            match (&points[i], &jumps[i]) {
                (EP::Inf, _) | (_, EP::Inf) => out[i] = self.add(&points[i], &jumps[i]),
                (EP::A(x1, _), EP::A(x2, _)) if x1 != x2 => {
                    scratch.active.push(i);
                    scratch.den.push(self.f.sub(x2, x1));
                }
                _ => out[i] = self.add(&points[i], &jumps[i]),
            }
        }
        self.f
            .batch_inv_into(&scratch.den, &mut scratch.prefix, &mut scratch.inv);
        for (j, &i) in scratch.active.iter().enumerate() {
            out[i] = self.mixed_with_inv(&points[i], &jumps[i], &scratch.inv[j]);
        }
    }

    pub fn batch_add(&self, points: &[EP], jumps: &[EP]) -> Vec<EP> {
        let mut out = Vec::with_capacity(points.len());
        let mut scratch = BatchAddScratch::with_capacity(points.len());
        self.batch_add_into(points, jumps, &mut out, &mut scratch);
        out
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StrongOpts {
    pub batch: usize,
    pub r: usize,
    pub dp_bits: u32,
    pub snapshot_period: u64,
    pub max_steps: u64,
}

impl StrongOpts {
    pub fn for_instance(inst: &Inst, batch: usize) -> Self {
        let s = (inst.ell as f64).sqrt() / 64.0;
        let dp_bits = (s.max(4.0).log2().floor() as u32).max(1);
        Self {
            batch,
            r: 32,
            dp_bits,
            snapshot_period: 128,
            max_steps: 60 * (inst.ell as f64).sqrt() as u64 + 200_000,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StrongOut {
    pub ok: bool,
    pub found: Option<u64>,
    pub steps: u64,
    pub dps: u64,
    pub restarts: u64,
    pub lookahead: u64,
    pub cycles: u64,
    pub cycle_adds: u64,
    pub setup_reciprocal_div: u64,
    pub setup_field: FieldCounts,
    pub walk_field: FieldCounts,
    pub verify_field: FieldCounts,
    pub setup_native: NativeCounts,
    pub walk_native: NativeCounts,
    pub wall_s: f64,
}

#[derive(Clone)]
struct Walker {
    p: EP,
    h: Option<u64>,
    a: u64,
    b: u64,
    tail: u64,
    local: u64,
    snapshot: Option<(EP, u64)>,
}

#[inline(always)]
fn sadd(a: u64, b: u64, n: u64, c: &mut NativeCounts) -> u64 {
    c.scalar_adds += 1;
    let (s, overflow) = a.overflowing_add(b);
    if overflow || s >= n {
        s.wrapping_sub(n)
    } else {
        s
    }
}

#[inline(always)]
fn ssub(a: u64, b: u64, n: u64, c: &mut NativeCounts) -> u64 {
    c.scalar_adds += 1;
    if a >= b {
        a - b
    } else {
        a + (n - b)
    }
}

#[inline(always)]
fn smul(a: u64, b: u64, n: u64, c: &mut NativeCounts) -> u64 {
    c.scalar_muls += 1;
    ((a as u128 * b as u128) % n as u128) as u64
}

#[inline(always)]
fn hx(x: &K) -> u64 {
    x[0].wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ x[1].wrapping_mul(0xC2B2_AE3D_27D4_EB4F) ^ x[2]
}

#[inline(always)]
fn partition_index(h: u64, r: usize) -> usize {
    debug_assert!(r > 0);
    if r.is_power_of_two() {
        (h & (r as u64 - 1)) as usize
    } else {
        (h % r as u64) as usize
    }
}

fn canon(f: &FastF3, p: EP, a: u64, b: u64, n: u64, c: &mut NativeCounts) -> (EP, u64, u64) {
    c.canonicalisations += 1;
    let EP::A(x, y) = p else { return (p, a, b) };
    let first = if y[0] != 0 {
        y[0]
    } else if y[1] != 0 {
        y[1]
    } else {
        y[2]
    };
    if first <= f.q / 2 {
        (EP::A(x, y), a, b)
    } else {
        let ny = f.neg(&y);
        let na = if a == 0 { 0 } else { n - a };
        let nb = if b == 0 { 0 } else { n - b };
        c.scalar_adds += 2;
        (EP::A(x, ny), na, nb)
    }
}

fn new_walker(
    ec: &FastEc<'_>,
    f: &FastF3,
    g: &EP,
    q: &EP,
    n: u64,
    rng: &mut Rng,
    c: &mut NativeCounts,
) -> Walker {
    loop {
        let (a, b) = (rng.below(n), rng.below(n - 1) + 1);
        c.rng_draws += 2;
        let p = ec.add(&ec.mul(g, a as u128), &ec.mul(q, b as u128));
        if p != EP::Inf {
            let (p, a, b) = canon(f, p, a, b, n, c);
            return Walker {
                p,
                h: None,
                a,
                b,
                tail: 0,
                local: 0,
                snapshot: None,
            };
        }
    }
}

#[derive(Default)]
struct StepScratch {
    first: Vec<usize>,
    chosen: Vec<usize>,
    next: Vec<EP>,
    next_hash: Vec<Option<u64>>,
    pending: Vec<usize>,
    retry: Vec<usize>,
    points: Vec<EP>,
    addends: Vec<EP>,
    candidates: Vec<EP>,
    batch_add: BatchAddScratch,
}

impl StepScratch {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            first: Vec::with_capacity(capacity),
            chosen: Vec::with_capacity(capacity),
            next: Vec::with_capacity(capacity),
            next_hash: Vec::with_capacity(capacity),
            pending: Vec::with_capacity(capacity),
            retry: Vec::with_capacity(capacity),
            points: Vec::with_capacity(capacity),
            addends: Vec::with_capacity(capacity),
            candidates: Vec::with_capacity(capacity),
            batch_add: BatchAddScratch::with_capacity(capacity),
        }
    }
}

/// One quotient-walk step for every lane, with the frozen rho-neg look-ahead
/// rule applied in batches. If jump j would land in partition j again, retry
/// from the original point with j+1. This removes the dominant fruitless
/// two-cycle without giving up a shared inversion on the common first try.
fn batched_lookahead_step(
    ec: &FastEc<'_>,
    jumps: &[EP],
    walkers: &[Walker],
    native: &mut NativeCounts,
    scratch: &mut StepScratch,
) -> u64 {
    let r = jumps.len();
    let StepScratch {
        first,
        chosen,
        next,
        next_hash,
        pending,
        retry,
        points,
        addends,
        candidates,
        batch_add,
    } = scratch;
    chosen.clear();
    chosen.extend(first.iter().copied());
    next.clear();
    next.resize(walkers.len(), EP::Inf);
    next_hash.clear();
    next_hash.resize(walkers.len(), None);
    pending.clear();
    pending.extend(0..walkers.len());
    let mut lookahead = 0u64;
    for attempt in 0..r {
        points.clear();
        points.extend(pending.iter().map(|&i| walkers[i].p));
        addends.clear();
        addends.extend(pending.iter().map(|&i| jumps[chosen[i]]));
        ec.batch_add_into(points, addends, candidates, batch_add);
        retry.clear();
        for (slot, &i) in pending.iter().enumerate() {
            let candidate = candidates[slot];
            native.lookahead_tests += 1;
            let candidate_hash = match candidate {
                EP::Inf => None,
                EP::A(x, _) => {
                    native.hashes += 1;
                    Some(hx(&x))
                }
            };
            let same_partition = candidate_hash
                .map(|h| partition_index(h, r) == chosen[i])
                .unwrap_or(false);
            if same_partition && attempt + 1 < r {
                chosen[i] = (chosen[i] + 1) % r;
                lookahead += 1;
                retry.push(i);
            } else {
                next[i] = candidate;
                next_hash[i] = candidate_hash;
            }
        }
        std::mem::swap(pending, retry);
        if pending.is_empty() {
            return lookahead;
        }
    }
    unreachable!("look-ahead loop accepts its final candidate")
}

fn point_key(p: &EP) -> [u64; 6] {
    match p {
        EP::Inf => [u64::MAX; 6],
        EP::A(x, y) => [x[2], x[1], x[0], y[2], y[1], y[0]],
    }
}

/// Scalar version used only to traverse a detected fruitless cycle. Returning
/// to the cycle's canonical minimum before doubling makes escape independent
/// of a lane's snapshot phase, so merged walks remain coalesced.
fn single_lookahead_step(
    ec: &FastEc<'_>,
    f: &FastF3,
    jumps: &[EP],
    ma: &[u64],
    mb: &[u64],
    p: &EP,
    a: u64,
    b: u64,
    n: u64,
    native: &mut NativeCounts,
) -> Option<(EP, u64, u64, u64, u64)> {
    let r = jumps.len();
    let EP::A(x, _) = p else { return None };
    native.hashes += 1;
    let mut j = partition_index(hx(x), r);
    let mut adds = 0u64;
    let mut extra = 0u64;
    for attempt in 0..r {
        let candidate = ec.add(p, &jumps[j]);
        adds += 1;
        native.lookahead_tests += 1;
        let same_partition = match candidate {
            EP::Inf => false,
            EP::A(nx, _) => {
                native.hashes += 1;
                partition_index(hx(&nx), r) == j
            }
        };
        if same_partition && attempt + 1 < r {
            j = (j + 1) % r;
            extra += 1;
            continue;
        }
        if candidate == EP::Inf {
            return None;
        }
        let na = sadd(a, ma[j], n, native);
        let nb = sadd(b, mb[j], n, native);
        let (candidate, na, nb) = canon(f, candidate, na, nb, n, native);
        return Some((candidate, na, nb, adds, extra));
    }
    unreachable!("look-ahead loop accepts its final candidate")
}

fn collision_scalar(
    n: u64,
    y: &K,
    a: u64,
    b: u64,
    y2: &K,
    a2: u64,
    b2: u64,
    c: &mut NativeCounts,
) -> Option<u64> {
    let (num, den) = if y == y2 {
        (ssub(a, a2, n, c), ssub(b2, b, n, c))
    } else {
        let sum_a = sadd(a, a2, n, c);
        let sum_b = sadd(b, b2, n, c);
        (if sum_a == 0 { 0 } else { n - sum_a }, sum_b)
    };
    if den == 0 {
        return None;
    }
    c.scalar_invs += 1;
    let di = inv_mod_u64(den, n);
    Some(smul(num, di, n, c))
}

pub fn strong_rho(inst: &Inst, k_true: u64, seed: u64, opts: StrongOpts) -> StrongOut {
    assert!(opts.batch > 0 && opts.r > 0 && opts.dp_bits < 63 && opts.snapshot_period > 0);
    let start = Instant::now();
    let n = inst.ell;
    let f = FastF3::from_instance(inst);
    let ec = FastEc::from_instance(&f, inst);
    let mut native = NativeCounts::default();
    let mut rng = Rng::new(seed ^ 0x4E45_47);
    let target = ec.mul(&inst.g, k_true as u128);

    let mut ma = Vec::with_capacity(opts.r);
    let mut mb = Vec::with_capacity(opts.r);
    let mut mp = Vec::with_capacity(opts.r);
    for _ in 0..opts.r {
        let a = rng.below(n - 1) + 1;
        let b = rng.below(n - 1) + 1;
        native.rng_draws += 2;
        ma.push(a);
        mb.push(b);
        mp.push(ec.add(&ec.mul(&inst.g, a as u128), &ec.mul(&target, b as u128)));
    }
    let mut walkers: Vec<_> = (0..opts.batch)
        .map(|_| new_walker(&ec, &f, &inst.g, &target, n, &mut rng, &mut native))
        .collect();
    let setup_field = f.counts();
    let setup_native = native;
    let mask = (1u64 << opts.dp_bits) - 1;
    let tail_cap = 20u64 << opts.dp_bits;
    let mut table: HashMap<K, (K, u64, u64)> = HashMap::with_capacity(128);
    let mut scratch = StepScratch::with_capacity(opts.batch);
    let (mut steps, mut restarts, mut lookahead, mut cycles, mut cycle_adds) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut found = None;

    'main: while steps <= opts.max_steps {
        scratch.first.clear();
        for walker in &mut walkers {
            let h = match walker.h {
                Some(h) => h,
                None => {
                    let EP::A(x, _) = walker.p else {
                        unreachable!()
                    };
                    let h = hx(&x);
                    walker.h = Some(h);
                    native.hashes += 1;
                    h
                }
            };
            scratch.first.push(partition_index(h, opts.r));
        }
        let extra = batched_lookahead_step(&ec, &mp, &walkers, &mut native, &mut scratch);
        lookahead += extra;

        for i in 0..walkers.len() {
            steps += 1;
            let j = scratch.chosen[i];
            let a = sadd(walkers[i].a, ma[j], n, &mut native);
            let b = sadd(walkers[i].b, mb[j], n, &mut native);
            if scratch.next[i] == EP::Inf {
                restarts += 1;
                walkers[i] = new_walker(&ec, &f, &inst.g, &target, n, &mut rng, &mut native);
                continue;
            }
            let (mut p, mut a, mut b) = canon(&f, scratch.next[i], a, b, n, &mut native);

            let EP::A(mut x, _) = p else { unreachable!() };
            let mut h = scratch.next_hash[i].expect("affine candidate hash");
            walkers[i].local += 1;
            let mut cycle_len = None;
            if let Some((snapshot, snapshot_step)) = walkers[i].snapshot {
                native.cycle_compares += 1;
                if snapshot == p {
                    cycle_len = Some(walkers[i].local - snapshot_step);
                }
            }
            if let Some(cycle_len) = cycle_len {
                cycles += 1;
                let (mut cp, mut ca, mut cb) = (p, a, b);
                let (mut best, mut best_a, mut best_b) = (p, a, b);
                let mut failed = false;
                for _ in 0..cycle_len {
                    let Some((np, na, nb, adds, extra)) =
                        single_lookahead_step(&ec, &f, &mp, &ma, &mb, &cp, ca, cb, n, &mut native)
                    else {
                        failed = true;
                        break;
                    };
                    cycle_adds += adds;
                    lookahead += extra;
                    (cp, ca, cb) = (np, na, nb);
                    if point_key(&cp) < point_key(&best) {
                        (best, best_a, best_b) = (cp, ca, cb);
                    }
                }
                if failed {
                    restarts += 1;
                    walkers[i] = new_walker(&ec, &f, &inst.g, &target, n, &mut rng, &mut native);
                    continue;
                }
                let doubled = ec.add(&best, &best);
                cycle_adds += 1;
                if doubled == EP::Inf {
                    restarts += 1;
                    walkers[i] = new_walker(&ec, &f, &inst.g, &target, n, &mut rng, &mut native);
                    continue;
                }
                let da = sadd(best_a, best_a, n, &mut native);
                let db = sadd(best_b, best_b, n, &mut native);
                (p, a, b) = canon(&f, doubled, da, db, n, &mut native);
                walkers[i].snapshot = None;
                let EP::A(nx, _) = p else { unreachable!() };
                x = nx;
                h = hx(&x);
                native.hashes += 1;
            }
            if walkers[i].local % opts.snapshot_period == 0 {
                walkers[i].snapshot = Some((p, walkers[i].local));
            }

            walkers[i].p = p;
            walkers[i].h = Some(h);
            walkers[i].a = a;
            walkers[i].b = b;
            walkers[i].tail += 1;

            native.dp_tests += 1;
            if (h >> 20) & mask == 0 {
                native.table_ops += 1;
                let EP::A(_, y) = p else { unreachable!() };
                if let Some((y2, a2, b2)) = table.get(&x).copied() {
                    if let Some(k) = collision_scalar(n, &y, a, b, &y2, a2, b2, &mut native) {
                        found = Some(k);
                        break 'main;
                    }
                    restarts += 1;
                    walkers[i] = new_walker(&ec, &f, &inst.g, &target, n, &mut rng, &mut native);
                } else {
                    table.insert(x, (y, a, b));
                    native.table_ops += 1;
                    walkers[i].tail = 0;
                }
            } else if walkers[i].tail > tail_cap {
                restarts += 1;
                walkers[i] = new_walker(&ec, &f, &inst.g, &target, n, &mut rng, &mut native);
            }
            if steps > opts.max_steps {
                break 'main;
            }
        }
    }

    let walk_field_end = f.counts();
    let walk_native_end = native;
    let ok = found
        .map(|k| ec.mul(&inst.g, k as u128) == target)
        .unwrap_or(false);
    let verify_end = f.counts();
    StrongOut {
        ok: ok && found == Some(k_true),
        found,
        steps,
        dps: table.len() as u64,
        restarts,
        lookahead,
        cycles,
        cycle_adds,
        setup_reciprocal_div: f.setup_reciprocal_div,
        setup_field,
        walk_field: walk_field_end.since(setup_field),
        verify_field: verify_end.since(walk_field_end),
        setup_native,
        walk_native: walk_native_end.since(setup_native),
        wall_s: start.elapsed().as_secs_f64(),
    }
}

/// Deterministic development controls.  Kept outside Rust's test harness so
/// the frozen standalone N4 module is compiled exactly as it is in its driver
/// (its own `cfg(test)` suite depends on Cargo-only test dependencies).
pub fn selftest() {
    {
        let reduce = |q: u64, a: u64, b: u64| {
            assert!(a < q && b < q);
            barrett_reduce_product(q, barrett_reciprocal(q), a * b)
        };
        let reference =
            |q: u64, a: u64, b: u64| (u128::from(a) * u128::from(b) % u128::from(q)) as u64;
        for q in 2..=257 {
            for a in 0..q {
                for b in 0..q {
                    assert_eq!(reduce(q, a, b), reference(q, a, b), "q={q} a={a} b={b}");
                }
            }
        }
        let mut rng = Rng::new(0x5245_4455_4345_5233);
        for q in [
            2u64,
            3,
            7,
            223,
            4051,
            65_535,
            65_536,
            65_537,
            (1u64 << 30) - 35,
            (1u64 << 31) - 1,
        ] {
            for a in [0, 1, q / 2, q - 2, q - 1] {
                for b in [0, 1, q / 2, q - 2, q - 1] {
                    assert_eq!(reduce(q, a, b), reference(q, a, b), "q={q} a={a} b={b}");
                }
            }
            for _ in 0..10_000 {
                let (a, b) = (rng.below(q), rng.below(q));
                assert_eq!(reduce(q, a, b), reference(q, a, b), "q={q} a={a} b={b}");
            }
        }
        eprintln!("selftest: reciprocal reduction matches widened remainder");
    }
    let instance = || Inst::search(223, 1, 200_000).expect("q=223 instance");
    for r in [1usize, 2, 4, 8, 16, 32, 3, 5, 31, 33] {
        for h in [0, 1, 31, 32, 33, u32::MAX as u64, u64::MAX] {
            assert_eq!(partition_index(h, r), (h % r as u64) as usize);
        }
    }
    eprintln!("selftest: masked and fallback partitions match modulo");
    {
        let inst = instance();
        let f = FastF3::from_instance(&inst);
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let a = inst.kf.rand(&mut rng);
            let b = inst.kf.rand(&mut rng);
            assert_eq!(f.add(&a, &b), inst.kf.add(&a, &b));
            assert_eq!(f.sub(&a, &b), inst.kf.sub(&a, &b));
            assert_eq!(f.mul(&a, &b), inst.kf.mul(&a, &b));
            let before_sq = f.counts();
            let sq = f.sq(&a);
            let sq_counts = f.counts().since(before_sq);
            let before_mul = f.counts();
            let generic_sq = f.mul(&a, &a);
            let mul_counts = f.counts().since(before_mul);
            assert_eq!(sq, generic_sq);
            assert_eq!(sq, inst.kf.mul(&a, &a));
            assert_eq!(sq_counts.fq_mul + 1, mul_counts.fq_mul);
            assert_eq!(sq_counts.f3_sq, 1);
            assert_eq!(sq_counts.f3_mul, 0);
            assert_eq!(mul_counts.f3_sq, 0);
            assert_eq!(mul_counts.f3_mul, 1);
            if a != KZERO {
                assert_eq!(f.inv(&a), inst.kf.inv(&a));
                assert_eq!(f.mul(&a, &f.inv(&a)), KONE);
            }
        }
        eprintln!("selftest: fast field matches frozen field");
    }
    {
        let inst = instance();
        let f = FastF3::from_instance(&inst);
        let mut rng = Rng::new(9);
        let values: Vec<_> = (0..37)
            .map(|_| loop {
                let x = inst.kf.rand(&mut rng);
                if x != KZERO {
                    break x;
                }
            })
            .collect();
        let got = f.batch_inv(&values);
        for (x, xi) in values.iter().zip(got.iter()) {
            assert_eq!(f.mul(x, xi), KONE);
            assert_eq!(*xi, inst.kf.inv(x));
        }
        eprintln!("selftest: batch inverse matches pointwise inversion");
    }
    {
        let inst = instance();
        let f = FastF3::from_instance(&inst);
        let ec = FastEc::from_instance(&f, &inst);
        let points: Vec<_> = (1..=40).map(|k| inst.ec.mul(&inst.g, k)).collect();
        let jumps: Vec<_> = (0..40)
            .map(|k| inst.ec.mul(&inst.g, (3 * k + 17) as u128))
            .collect();
        for (p, q) in points.iter().zip(jumps.iter()) {
            assert_eq!(ec.add(p, q), inst.ec.add(p, q));
        }
        let expected: Vec<_> = points
            .iter()
            .zip(jumps.iter())
            .map(|(p, q)| inst.ec.add(p, q))
            .collect();
        assert_eq!(ec.batch_add(&points, &jumps), expected);
        let mut scratch = BatchAddScratch::default();
        let mut reused = Vec::new();
        ec.batch_add_into(&points, &jumps, &mut reused, &mut scratch);
        assert_eq!(reused, expected);
        let capacities = (
            reused.capacity(),
            scratch.active.capacity(),
            scratch.den.capacity(),
            scratch.prefix.capacity(),
            scratch.inv.capacity(),
        );
        ec.batch_add_into(&points, &jumps, &mut reused, &mut scratch);
        assert_eq!(reused, expected);
        assert_eq!(
            capacities,
            (
                reused.capacity(),
                scratch.active.capacity(),
                scratch.den.capacity(),
                scratch.prefix.capacity(),
                scratch.inv.capacity(),
            )
        );
        for k in [1u128, 2, 3, 17, 255, 65_537] {
            assert_eq!(ec.mul(&inst.g, k), inst.ec.mul(&inst.g, k));
        }
        eprintln!("selftest: fast and batched curve addition match frozen curve");
    }
    {
        let inst = instance();
        for (i, width) in [1usize, 4, 8, 16, 32].into_iter().enumerate() {
            let k = 17 + i as u64 * 101;
            let out = strong_rho(
                &inst,
                k,
                100 + i as u64,
                StrongOpts::for_instance(&inst, width),
            );
            assert!(
                out.ok,
                "width={width} found={:?} steps={}",
                out.found, out.steps
            );
            assert_eq!(out.found, Some(k));
            assert_selftest_oracle(width, &out);
            assert_eq!(out.setup_reciprocal_div, 1);
            if width > 1 {
                assert!(out.walk_field.fq_inv < out.steps);
            }
        }
        eprintln!("selftest: planted logs recovered at widths 1, 4, 8, 16, and 32");
    }
    println!("SELFTEST OK");
}

fn assert_selftest_oracle(width: usize, out: &StrongOut) {
    macro_rules! fc {
        ($qm:expr, $qi:expr, $qa:expr, $m:expr, $s:expr, $i:expr, $bi:expr, $ga:expr) => {
            FieldCounts {
                fq_mul: $qm,
                fq_inv: $qi,
                fq_add: $qa,
                f3_mul: $m,
                f3_sq: $s,
                f3_inv: $i,
                batch_inv: $bi,
                group_add: $ga,
            }
        };
    }
    macro_rules! nc {
        ($rng:expr, $hash:expr, $canon:expr, $look:expr, $dp:expr, $table:expr,
         $cycle:expr, $sa:expr, $sm:expr, $si:expr) => {
            NativeCounts {
                rng_draws: $rng,
                hashes: $hash,
                canonicalisations: $canon,
                lookahead_tests: $look,
                dp_tests: $dp,
                table_ops: $table,
                cycle_compares: $cycle,
                scalar_adds: $sa,
                scalar_muls: $sm,
                scalar_invs: $si,
            }
        };
    }
    let expected = match width {
        1 => (
            (Some(17), 1177, 63, 0, 33, 1, 5),
            fc!(77627, 1928, 245776, 7057, 3201, 1928, 0, 1995),
            fc!(40106, 1215, 118351, 3646, 1216, 1215, 1210, 1215),
            fc!(209, 5, 670, 19, 9, 5, 0, 6),
            nc!(66, 0, 1, 0, 0, 0, 0, 0, 0, 0),
            nc!(0, 1220, 1182, 1214, 1177, 127, 925, 3556, 1, 1),
        ),
        8 => (
            (Some(118), 1917, 119, 0, 65, 1, 5),
            fc!(96261, 2394, 304547, 8751, 3963, 2394, 0, 2475),
            fc!(69055, 303, 250997, 9345, 1991, 303, 298, 1990),
            fc!(396, 10, 1246, 36, 16, 10, 0, 11),
            nc!(80, 0, 8, 0, 0, 0, 0, 6, 0, 0),
            nc!(0, 2002, 1922, 1989, 1917, 239, 785, 5708, 1, 1),
        ),
        32 => (
            (Some(219), 1421, 76, 0, 45, 0, 0),
            fc!(153758, 3819, 487027, 13978, 6340, 3819, 0, 3948),
            fc!(51827, 74, 192424, 7277, 1485, 74, 74, 1485),
            fc!(473, 12, 1486, 43, 19, 12, 0, 13),
            nc!(128, 0, 32, 0, 0, 0, 0, 46, 0, 0),
            nc!(0, 1517, 1421, 1485, 1421, 153, 0, 4220, 1, 1),
        ),
        _ => unreachable!("self-test width"),
    };
    assert_eq!(
        (
            out.found,
            out.steps,
            out.dps,
            out.restarts,
            out.lookahead,
            out.cycles,
            out.cycle_adds,
        ),
        expected.0
    );
    assert_eq!(out.setup_field, expected.1);
    assert_eq!(out.walk_field, expected.2);
    assert_eq!(out.verify_field, expected.3);
    assert_eq!(out.setup_native, expected.4);
    assert_eq!(out.walk_native, expected.5);
}
