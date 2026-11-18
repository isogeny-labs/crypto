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
pub struct FastF3 {
    q: u64,
    c: u64,
    w: u64,
    w2: u64,
    counts: Cell<FieldCounts>,
}

impl FastF3 {
    pub fn from_instance(inst: &Inst) -> Self {
        Self {
            q: inst.q,
            c: inst.kf.c,
            w: inst.kf.w,
            w2: inst.kf.w2,
            counts: Cell::new(FieldCounts::default()),
        }
    }

    pub fn counts(&self) -> FieldCounts {
        self.counts.get()
    }

    fn restore_counts(&self, counts: FieldCounts) {
        self.counts.set(counts);
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
        // The inherited family invariant is q < 2^31, so a*b < 2^62.
        // Avoid widening division in the hottest kernel.
        a * b % self.q
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

    pub fn batch_inv(&self, values: &[K]) -> Vec<K> {
        assert!(values.iter().all(|v| *v != KZERO));
        if values.is_empty() {
            return Vec::new();
        }
        self.bump(|c| c.batch_inv += 1);
        if values.len() == 1 {
            return vec![self.inv(&values[0])];
        }
        let mut prefix = Vec::with_capacity(values.len());
        let mut acc = values[0];
        prefix.push(acc);
        for v in &values[1..] {
            acc = self.mul(&acc, v);
            prefix.push(acc);
        }
        let mut inv_acc = self.inv(&acc);
        let mut out = vec![KZERO; values.len()];
        for i in (1..values.len()).rev() {
            out[i] = self.mul(&inv_acc, &prefix[i - 1]);
            inv_acc = self.mul(&inv_acc, &values[i]);
        }
        out[0] = inv_acc;
        out
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

    pub fn batch_add(&self, points: &[EP], jumps: &[EP]) -> Vec<EP> {
        assert_eq!(points.len(), jumps.len());
        let mut out = vec![EP::Inf; points.len()];
        let mut active = Vec::new();
        let mut den = Vec::new();
        for i in 0..points.len() {
            match (&points[i], &jumps[i]) {
                (EP::Inf, _) | (_, EP::Inf) => out[i] = self.add(&points[i], &jumps[i]),
                (EP::A(x1, _), EP::A(x2, _)) if x1 != x2 => {
                    active.push(i);
                    den.push(self.f.sub(x2, x1));
                }
                _ => out[i] = self.add(&points[i], &jumps[i]),
            }
        }
        let inv = self.f.batch_inv(&den);
        for (j, &i) in active.iter().enumerate() {
            out[i] = self.mixed_with_inv(&points[i], &jumps[i], &inv[j]);
        }
        out
    }

    /// Complete affine batch addition for setup work.  Unlike the walk's
    /// mixed-addition helper above, this also batches tangent denominators,
    /// so a layer of independent doublings pays one field inversion.
    fn batch_add_complete(&self, points: &[EP], addends: &[EP]) -> Vec<EP> {
        assert_eq!(points.len(), addends.len());
        let mut out = vec![EP::Inf; points.len()];
        let mut active = Vec::new();
        let mut nums = Vec::new();
        let mut dens = Vec::new();
        for i in 0..points.len() {
            match (&points[i], &addends[i]) {
                (EP::Inf, _) | (_, EP::Inf) => {
                    out[i] = self.add(&points[i], &addends[i]);
                }
                (EP::A(x1, y1), EP::A(x2, y2)) if x1 == x2 => {
                    if self.f.add(y1, y2) == KZERO {
                        self.f.bump(|c| c.group_add += 1);
                        out[i] = EP::Inf;
                        continue;
                    }
                    let x12 = self.f.sq(x1);
                    let three = self.f.add(&self.f.add(&x12, &x12), &x12);
                    let num = self.f.add(
                        &self
                            .f
                            .add(&three, &self.f.mul(&self.f.add(&self.a2, &self.a2), x1)),
                        &self.a4,
                    );
                    active.push(i);
                    nums.push(num);
                    dens.push(self.f.add(y1, y1));
                }
                (EP::A(x1, y1), EP::A(x2, y2)) => {
                    active.push(i);
                    nums.push(self.f.sub(y2, y1));
                    dens.push(self.f.sub(x2, x1));
                }
            }
        }
        let inv = self.f.batch_inv(&dens);
        for (j, &i) in active.iter().enumerate() {
            self.f.bump(|c| c.group_add += 1);
            out[i] = self.add_with_inv_uncounted(&points[i], &addends[i], &inv[j], &nums[j]);
        }
        out
    }

    /// Cold width-2 table for simultaneous `[a]g + [b]q` multiplication.
    /// All thirteen nontrivial construction operations are counted, while
    /// their denominators are shared in three complete-addition batches.
    fn window2_table(&self, g: &EP, q: &EP) -> [EP; 16] {
        let mut table = [EP::Inf; 16];
        table[4] = *g;
        table[1] = *q;

        let doubled = self.batch_add_complete(&[*g, *q], &[*g, *q]);
        table[8] = doubled[0];
        table[2] = doubled[1];
        let tripled = self.batch_add_complete(&[table[8], table[2]], &[*g, *q]);
        table[12] = tripled[0];
        table[3] = tripled[1];

        let mut left = Vec::with_capacity(9);
        let mut right = Vec::with_capacity(9);
        let mut slots = Vec::with_capacity(9);
        for i in 1..4 {
            for j in 1..4 {
                left.push(table[4 * i]);
                right.push(table[j]);
                slots.push(4 * i + j);
            }
        }
        let mixed = self.batch_add_complete(&left, &right);
        for (&slot, point) in slots.iter().zip(mixed) {
            table[slot] = point;
        }
        table
    }

    /// Evaluate many exact simultaneous multiplications with one shared,
    /// cold-charged width-2 table. Each base-4 Horner layer batches all live
    /// affine denominators across outputs.
    fn batch_window2_mul(&self, table: &[EP; 16], coeffs: &[(u64, u64)]) -> Vec<EP> {
        let Some(bits) = coeffs
            .iter()
            .map(|&(a, b)| 64 - (a | b).leading_zeros() as usize)
            .max()
        else {
            return Vec::new();
        };
        if bits == 0 {
            return vec![EP::Inf; coeffs.len()];
        }
        let top_shift = ((bits - 1) / 2) * 2;
        let mut out = vec![EP::Inf; coeffs.len()];
        let mut first = true;
        for window in (0..=top_shift / 2).rev() {
            let shift = 2 * window;
            if !first {
                for _ in 0..2 {
                    let lanes: Vec<_> = out
                        .iter()
                        .enumerate()
                        .filter_map(|(i, p)| (*p != EP::Inf).then_some(i))
                        .collect();
                    if !lanes.is_empty() {
                        let points: Vec<_> = lanes.iter().map(|&i| out[i]).collect();
                        let doubled = self.batch_add_complete(&points, &points);
                        for (&i, point) in lanes.iter().zip(doubled) {
                            out[i] = point;
                        }
                    }
                }
            }

            let mut lanes = Vec::new();
            let mut points = Vec::new();
            let mut addends = Vec::new();
            for (i, &(a, b)) in coeffs.iter().enumerate() {
                let digit = ((((a >> shift) & 3) << 2) | ((b >> shift) & 3)) as usize;
                if digit != 0 {
                    lanes.push(i);
                    points.push(out[i]);
                    addends.push(table[digit]);
                }
            }
            let summed = self.batch_add_complete(&points, &addends);
            for (&i, point) in lanes.iter().zip(summed) {
                out[i] = point;
            }
            first = false;
        }
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
    /// Batch the cold jump/initial-walker simultaneous multiplications.
    /// `false` is retained only as an in-source exact control.
    pub batch_setup: bool,
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
            batch_setup: true,
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
                a,
                b,
                tail: 0,
                local: 0,
                snapshot: None,
            };
        }
    }
}

struct InitialState {
    ma: Vec<u64>,
    mb: Vec<u64>,
    mp: Vec<EP>,
    walkers: Vec<Walker>,
}

/// Build the jump table and initial walkers. The candidate keeps the exact
/// baseline RNG stream and coefficients; only point construction changes.
fn initial_state(
    ec: &FastEc<'_>,
    f: &FastF3,
    g: &EP,
    q: &EP,
    n: u64,
    r: usize,
    batch: usize,
    rng: &mut Rng,
    native: &mut NativeCounts,
    batch_setup: bool,
) -> InitialState {
    let mut ma = Vec::with_capacity(r);
    let mut mb = Vec::with_capacity(r);
    for _ in 0..r {
        ma.push(rng.below(n - 1) + 1);
        mb.push(rng.below(n - 1) + 1);
        native.rng_draws += 2;
    }

    if !batch_setup {
        let mp = ma
            .iter()
            .zip(&mb)
            .map(|(&a, &b)| ec.add(&ec.mul(g, a as u128), &ec.mul(q, b as u128)))
            .collect();
        let walkers = (0..batch)
            .map(|_| new_walker(ec, f, g, q, n, rng, native))
            .collect();
        return InitialState {
            ma,
            mb,
            mp,
            walkers,
        };
    }

    let table = ec.window2_table(g, q);
    let jump_coeffs: Vec<_> = ma.iter().copied().zip(mb.iter().copied()).collect();
    let mp = ec.batch_window2_mul(&table, &jump_coeffs);

    // The baseline retries one lane to completion before drawing the next.
    // Tentatively batch the overwhelmingly common no-identity case; on an
    // identity, restore every observable counter and RNG bit before taking
    // the exact sequential path.
    let rng_before = rng.0;
    let native_before = *native;
    let field_before = f.counts();
    let walker_coeffs: Vec<_> = (0..batch)
        .map(|_| {
            native.rng_draws += 2;
            (rng.below(n), rng.below(n - 1) + 1)
        })
        .collect();
    let walker_points = ec.batch_window2_mul(&table, &walker_coeffs);
    let walkers = if walker_points.iter().any(|p| *p == EP::Inf) {
        rng.0 = rng_before;
        *native = native_before;
        f.restore_counts(field_before);
        (0..batch)
            .map(|_| new_walker(ec, f, g, q, n, rng, native))
            .collect()
    } else {
        walker_points
            .into_iter()
            .zip(walker_coeffs)
            .map(|(p, (a, b))| {
                let (p, a, b) = canon(f, p, a, b, n, native);
                Walker {
                    p,
                    a,
                    b,
                    tail: 0,
                    local: 0,
                    snapshot: None,
                }
            })
            .collect()
    };
    InitialState {
        ma,
        mb,
        mp,
        walkers,
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
    first: Vec<usize>,
    native: &mut NativeCounts,
) -> (Vec<EP>, Vec<usize>, u64) {
    let r = jumps.len();
    let mut chosen = first;
    let mut next = vec![EP::Inf; walkers.len()];
    let mut pending: Vec<usize> = (0..walkers.len()).collect();
    let mut lookahead = 0u64;
    for attempt in 0..r {
        let points: Vec<_> = pending.iter().map(|&i| walkers[i].p).collect();
        let addends: Vec<_> = pending.iter().map(|&i| jumps[chosen[i]]).collect();
        let candidates = ec.batch_add(&points, &addends);
        let mut retry = Vec::new();
        for (&i, candidate) in pending.iter().zip(candidates) {
            native.lookahead_tests += 1;
            let same_partition = match candidate {
                EP::Inf => false,
                EP::A(x, _) => {
                    native.hashes += 1;
                    (hx(&x) % r as u64) as usize == chosen[i]
                }
            };
            if same_partition && attempt + 1 < r {
                chosen[i] = (chosen[i] + 1) % r;
                lookahead += 1;
                retry.push(i);
            } else {
                next[i] = candidate;
            }
        }
        pending = retry;
        if pending.is_empty() {
            return (next, chosen, lookahead);
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
    let mut j = (hx(x) % r as u64) as usize;
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
                (hx(&nx) % r as u64) as usize == j
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
    let InitialState {
        ma,
        mb,
        mp,
        mut walkers,
    } = initial_state(
        &ec,
        &f,
        &inst.g,
        &target,
        n,
        opts.r,
        opts.batch,
        &mut rng,
        &mut native,
        opts.batch_setup,
    );
    let setup_field = f.counts();
    let setup_native = native;
    let mask = (1u64 << opts.dp_bits) - 1;
    let tail_cap = 20u64 << opts.dp_bits;
    let mut table: HashMap<K, (K, u64, u64)> = HashMap::new();
    let (mut steps, mut restarts, mut lookahead, mut cycles, mut cycle_adds) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut found = None;

    'main: while steps <= opts.max_steps {
        let first: Vec<_> = walkers
            .iter()
            .map(|w| {
                let EP::A(x, _) = w.p else { unreachable!() };
                native.hashes += 1;
                (hx(&x) % opts.r as u64) as usize
            })
            .collect();
        let (next, js, extra) = batched_lookahead_step(&ec, &mp, &walkers, first, &mut native);
        lookahead += extra;

        for i in 0..walkers.len() {
            steps += 1;
            let j = js[i];
            let a = sadd(walkers[i].a, ma[j], n, &mut native);
            let b = sadd(walkers[i].b, mb[j], n, &mut native);
            if next[i] == EP::Inf {
                restarts += 1;
                walkers[i] = new_walker(&ec, &f, &inst.g, &target, n, &mut rng, &mut native);
                continue;
            }
            let (mut p, mut a, mut b) = canon(&f, next[i], a, b, n, &mut native);

            let EP::A(mut x, _) = p else { unreachable!() };
            let mut h = hx(&x);
            native.hashes += 1;
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
    let instance = || Inst::search(223, 1, 200_000).expect("q=223 instance");
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
        assert_eq!(
            ec.batch_add(&points, &jumps),
            points
                .iter()
                .zip(jumps.iter())
                .map(|(p, q)| inst.ec.add(p, q))
                .collect::<Vec<_>>()
        );
        let neg_g = match inst.g {
            EP::Inf => unreachable!(),
            EP::A(x, y) => EP::A(x, f.neg(&y)),
        };
        let complete_left = [EP::Inf, inst.g, inst.g, inst.g, points[1], points[2]];
        let complete_right = [inst.g, EP::Inf, inst.g, neg_g, points[1], points[7]];
        assert_eq!(
            ec.batch_add_complete(&complete_left, &complete_right),
            complete_left
                .iter()
                .zip(&complete_right)
                .map(|(p, q)| inst.ec.add(p, q))
                .collect::<Vec<_>>()
        );
        for k in [1u128, 2, 3, 17, 255, 65_537] {
            assert_eq!(ec.mul(&inst.g, k), inst.ec.mul(&inst.g, k));
        }
        let before_table = f.counts();
        let table = ec.window2_table(&inst.g, &points[16]);
        let table_counts = f.counts().since(before_table);
        assert_eq!(table_counts.group_add, 13);
        assert_eq!(table_counts.batch_inv, 3);
        assert_eq!(table_counts.fq_inv, 3);
        let mut coeff_rng = Rng::new(0xB47C_2002);
        let mut coeffs = vec![(0, 0), (1, 0), (0, 1), (3, 3), (inst.ell - 1, 1)];
        coeffs.extend((0..64).map(|_| (coeff_rng.below(inst.ell), coeff_rng.below(inst.ell))));
        let got = ec.batch_window2_mul(&table, &coeffs);
        let expected: Vec<_> = coeffs
            .iter()
            .map(|&(a, b)| {
                inst.ec.add(
                    &inst.ec.mul(&inst.g, a as u128),
                    &inst.ec.mul(&points[16], b as u128),
                )
            })
            .collect();
        assert_eq!(got, expected);
        eprintln!(
            "selftest: complete batches and width-2 simultaneous multiplication match frozen curve; cold table={table_counts:?}"
        );
    }
    {
        let inst = instance();
        let target = inst.ec.mul(&inst.g, 119);
        let f_legacy = FastF3::from_instance(&inst);
        let ec_legacy = FastEc::from_instance(&f_legacy, &inst);
        let f_batch = FastF3::from_instance(&inst);
        let ec_batch = FastEc::from_instance(&f_batch, &inst);
        let mut rng_legacy = Rng::new(0x51A7_E001);
        let mut rng_batch = Rng::new(0x51A7_E001);
        let mut native_legacy = NativeCounts::default();
        let mut native_batch = NativeCounts::default();
        let legacy = initial_state(
            &ec_legacy,
            &f_legacy,
            &inst.g,
            &target,
            inst.ell,
            32,
            8,
            &mut rng_legacy,
            &mut native_legacy,
            false,
        );
        let batched = initial_state(
            &ec_batch,
            &f_batch,
            &inst.g,
            &target,
            inst.ell,
            32,
            8,
            &mut rng_batch,
            &mut native_batch,
            true,
        );
        assert_eq!(legacy.ma, batched.ma);
        assert_eq!(legacy.mb, batched.mb);
        assert_eq!(legacy.mp, batched.mp);
        assert_eq!(legacy.walkers.len(), batched.walkers.len());
        for (a, b) in legacy.walkers.iter().zip(&batched.walkers) {
            assert_eq!(a.p, b.p);
            assert_eq!((a.a, a.b, a.tail, a.local), (b.a, b.b, b.tail, b.local));
            assert_eq!(a.snapshot, b.snapshot);
        }
        assert_eq!(rng_legacy.0, rng_batch.0);
        assert_eq!(native_legacy, native_batch);
        let lc = f_legacy.counts();
        let bc = f_batch.counts();
        assert!(
            bc.fq_mul <= lc.fq_mul,
            "batch fq_mul {} > legacy {}",
            bc.fq_mul,
            lc.fq_mul
        );
        assert!(
            bc.fq_inv <= lc.fq_inv,
            "batch fq_inv {} > legacy {}",
            bc.fq_inv,
            lc.fq_inv
        );
        assert!(bc.fq_mul < lc.fq_mul || bc.fq_inv < lc.fq_inv);
        eprintln!("selftest: setup state exact; field legacy={lc:?} batch={bc:?}");
    }
    {
        let inst = instance();
        for (i, width) in [1usize, 8, 32].into_iter().enumerate() {
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
            if width > 1 {
                assert!(out.walk_field.fq_inv < out.steps);
            }
        }
        eprintln!("selftest: planted logs recovered at widths 1, 8, and 32");
    }
    {
        let inst = instance();
        let mut legacy_opts = StrongOpts::for_instance(&inst, 8);
        legacy_opts.batch_setup = false;
        let legacy = strong_rho(&inst, 119, 0xC01D, legacy_opts);
        let batched = strong_rho(&inst, 119, 0xC01D, StrongOpts::for_instance(&inst, 8));
        assert!(legacy.ok && batched.ok);
        assert_eq!(legacy.found, batched.found);
        assert_eq!(legacy.steps, batched.steps);
        assert_eq!(legacy.dps, batched.dps);
        assert_eq!(legacy.restarts, batched.restarts);
        assert_eq!(legacy.lookahead, batched.lookahead);
        assert_eq!(legacy.cycles, batched.cycles);
        assert_eq!(legacy.cycle_adds, batched.cycle_adds);
        assert_eq!(legacy.setup_native, batched.setup_native);
        assert_eq!(legacy.walk_field, batched.walk_field);
        assert_eq!(legacy.verify_field, batched.verify_field);
        assert_eq!(legacy.walk_native, batched.walk_native);
        assert!(batched.setup_field.fq_mul <= legacy.setup_field.fq_mul);
        assert!(batched.setup_field.fq_inv <= legacy.setup_field.fq_inv);
        assert!(
            batched.setup_field.fq_mul < legacy.setup_field.fq_mul
                || batched.setup_field.fq_inv < legacy.setup_field.fq_inv
        );
        eprintln!(
            "selftest: full trajectory exact; setup legacy={:?} batch={:?}",
            legacy.setup_field, batched.setup_field
        );
    }
    println!("SELFTEST OK");
}
