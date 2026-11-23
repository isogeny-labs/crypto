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
    pub fq_small_div: u64,
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
            fq_small_div: self.fq_small_div - before.fq_small_div,
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

/// Compile-time-selected sink for native diagnostic counters.
///
/// `CountedNative` retains the historical diagnostics used by `strong-*` and
/// `fast-*`. `BareNative` is a zero-sized sink used only by the selectable
/// `bare-*` production arm. Keeping both behind one statically dispatched
/// interface means the mathematical walk is not forked while the optimizer
/// can erase every bare-mode counter mutation.
trait NativeSink {
    fn snapshot(&self) -> NativeCounts;
    fn bump_rng_draws(&mut self, amount: u64);
    fn bump_hashes(&mut self, amount: u64);
    fn bump_canonicalisations(&mut self, amount: u64);
    fn bump_lookahead_tests(&mut self, amount: u64);
    fn bump_dp_tests(&mut self, amount: u64);
    fn bump_table_ops(&mut self, amount: u64);
    fn bump_cycle_compares(&mut self, amount: u64);
    fn bump_scalar_adds(&mut self, amount: u64);
    fn bump_scalar_muls(&mut self, amount: u64);
    fn bump_scalar_invs(&mut self, amount: u64);
}

#[derive(Clone, Copy, Debug, Default)]
struct CountedNative {
    counts: NativeCounts,
}

impl NativeSink for CountedNative {
    #[inline(always)]
    fn snapshot(&self) -> NativeCounts {
        self.counts
    }

    #[inline(always)]
    fn bump_rng_draws(&mut self, amount: u64) {
        self.counts.rng_draws += amount;
    }

    #[inline(always)]
    fn bump_hashes(&mut self, amount: u64) {
        self.counts.hashes += amount;
    }

    #[inline(always)]
    fn bump_canonicalisations(&mut self, amount: u64) {
        self.counts.canonicalisations += amount;
    }

    #[inline(always)]
    fn bump_lookahead_tests(&mut self, amount: u64) {
        self.counts.lookahead_tests += amount;
    }

    #[inline(always)]
    fn bump_dp_tests(&mut self, amount: u64) {
        self.counts.dp_tests += amount;
    }

    #[inline(always)]
    fn bump_table_ops(&mut self, amount: u64) {
        self.counts.table_ops += amount;
    }

    #[inline(always)]
    fn bump_cycle_compares(&mut self, amount: u64) {
        self.counts.cycle_compares += amount;
    }

    #[inline(always)]
    fn bump_scalar_adds(&mut self, amount: u64) {
        self.counts.scalar_adds += amount;
    }

    #[inline(always)]
    fn bump_scalar_muls(&mut self, amount: u64) {
        self.counts.scalar_muls += amount;
    }

    #[inline(always)]
    fn bump_scalar_invs(&mut self, amount: u64) {
        self.counts.scalar_invs += amount;
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct BareNative;

// A type-level guard: bare mode owns no `NativeCounts` storage that the hot
// path could mutate. Optimized-IR validation below the source level remains a
// separate release gate.
const _: [(); 0] = [(); std::mem::size_of::<BareNative>()];

impl NativeSink for BareNative {
    #[inline(always)]
    fn snapshot(&self) -> NativeCounts {
        NativeCounts::default()
    }

    #[inline(always)]
    fn bump_rng_draws(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_hashes(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_canonicalisations(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_lookahead_tests(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_dp_tests(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_table_ops(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_cycle_compares(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_scalar_adds(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_scalar_muls(&mut self, _amount: u64) {}
    #[inline(always)]
    fn bump_scalar_invs(&mut self, _amount: u64) {}
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

pub struct FastF3<const COUNT: bool = true> {
    q: u64,
    reciprocal: u64,
    setup_reciprocal_div: u64,
    c: u64,
    w: u64,
    w2: u64,
    use_mul5: bool,
    counts: Cell<FieldCounts>,
}

impl<const COUNT: bool> FastF3<COUNT> {
    pub fn from_instance(inst: &Inst) -> Self {
        Self::from_instance_with_mul5(inst, false)
    }

    pub fn from_instance_with_mul5(inst: &Inst, use_mul5: bool) -> Self {
        Self {
            q: inst.q,
            reciprocal: barrett_reciprocal(inst.q),
            // Cold, unpriced diagnostic: constructing the fixed-modulus
            // reducer evaluates one u128 division to obtain floor(2^64/q).
            setup_reciprocal_div: 1,
            c: inst.kf.c,
            w: inst.kf.w,
            w2: inst.kf.w2,
            use_mul5,
            counts: Cell::new(FieldCounts::default()),
        }
    }

    pub fn counts(&self) -> FieldCounts {
        self.counts.get()
    }

    #[inline(always)]
    fn bump(&self, f: impl FnOnce(&mut FieldCounts)) {
        if COUNT {
            let mut c = self.counts.get();
            f(&mut c);
            self.counts.set(c);
        }
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
    fn fdiv2(&self, x: u64) -> u64 {
        debug_assert!(x < self.q && self.q & 1 == 1);
        self.bump(|c| c.fq_small_div += 1);
        if x & 1 == 0 {
            x / 2
        } else {
            (x + self.q) / 2
        }
    }

    #[inline(always)]
    fn fdiv3(&self, x: u64) -> u64 {
        debug_assert!(x < self.q && self.q % 3 != 0);
        self.bump(|c| c.fq_small_div += 1);
        for k in 0..3 {
            let lifted = x + k * self.q;
            if lifted % 3 == 0 {
                return lifted / 3;
            }
        }
        unreachable!("one of x, x+q, x+2q is divisible by three")
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

    fn mul6_raw(&self, a: &K, b: &K) -> K {
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

    fn mul5_raw(&self, a: &K, b: &K) -> K {
        debug_assert!(self.q % 2 != 0 && self.q % 3 != 0);
        let v0 = self.fm(a[0], b[0]);
        let v1 = self.fm(
            self.fa(self.fa(a[0], a[1]), a[2]),
            self.fa(self.fa(b[0], b[1]), b[2]),
        );
        let vm1 = self.fm(
            self.fa(self.fs(a[0], a[1]), a[2]),
            self.fa(self.fs(b[0], b[1]), b[2]),
        );
        let two_a1 = self.fa(a[1], a[1]);
        let two_a2 = self.fa(a[2], a[2]);
        let a_at_2 = self.fa(self.fa(a[0], two_a1), self.fa(two_a2, two_a2));
        let two_b1 = self.fa(b[1], b[1]);
        let two_b2 = self.fa(b[2], b[2]);
        let b_at_2 = self.fa(self.fa(b[0], two_b1), self.fa(two_b2, two_b2));
        let v2 = self.fm(a_at_2, b_at_2);
        let vinf = self.fm(a[2], b[2]);

        let even = self.fdiv2(self.fa(v1, vm1));
        let d2 = self.fs(self.fs(even, v0), vinf);
        let odd = self.fdiv2(self.fs(v1, vm1));
        let at2 = self.fdiv2(self.fs(
            self.fs(self.fs(v2, v0), self.fmul_small(d2, 4)),
            self.fmul_small(vinf, 16),
        ));
        let d3 = self.fdiv3(self.fs(at2, odd));
        let d1 = self.fs(odd, d3);
        [
            self.fa(v0, self.fmul_small(d3, self.c)),
            self.fa(d1, self.fmul_small(vinf, self.c)),
            d2,
        ]
    }

    fn mul_raw(&self, a: &K, b: &K) -> K {
        if self.use_mul5 && self.q % 2 != 0 && self.q % 3 != 0 {
            self.mul5_raw(a, b)
        } else {
            self.mul6_raw(a, b)
        }
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

    /// Frozen Round B inverse, retained as a deterministic comparison oracle.
    fn inv_frobenius_reference(&self, a: &K) -> K {
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

    pub fn inv(&self, a: &K) -> K {
        assert_ne!(*a, KZERO, "inverse of zero");
        self.bump(|c| {
            c.f3_inv += 1;
            c.fq_inv += 1;
        });
        // Direct adjugate in F_q[z]/(z^3-c).  This computes the same
        // a^q*a^(q^2) coefficients as the frozen path without two Frobenius
        // maps or a general cubic multiplication.
        let adj0 = self.fs(
            self.fm(a[0], a[0]),
            self.fmul_small(self.fm(a[1], a[2]), self.c),
        );
        let adj1 = self.fs(
            self.fmul_small(self.fm(a[2], a[2]), self.c),
            self.fm(a[0], a[1]),
        );
        let adj2 = self.fs(self.fm(a[1], a[1]), self.fm(a[0], a[2]));
        let n = self.fa(
            self.fm(a[0], adj0),
            self.fmul_small(self.fa(self.fm(a[1], adj2), self.fm(a[2], adj1)), self.c),
        );
        let ni = inv_mod_u64(n, self.q);
        [self.fm(adj0, ni), self.fm(adj1, ni), self.fm(adj2, ni)]
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

pub struct FastEc<'a, const COUNT: bool = true> {
    f: &'a FastF3<COUNT>,
    a2: K,
    a4: K,
}

impl<'a, const COUNT: bool> FastEc<'a, COUNT> {
    pub fn from_instance(f: &'a FastF3<COUNT>, inst: &Inst) -> Self {
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
    /// Batch the cold jump simultaneous multiplications. Initial walkers stay
    /// on the lane-ordered retry path in `new_walker`.
    /// `false` is retained only as an in-source exact control.
    pub batch_setup: bool,
    /// Reuse the cold width-two joint table for each initial walker while
    /// preserving the baseline's lane-ordered coefficient and retry stream.
    /// This remains a selectable candidate until its complete setup cost and
    /// isolated wall time pass the promotion gates.
    pub reuse_setup_table_for_walkers: bool,
    /// Select the five-evaluation cubic multiplication diagnostic. The
    /// six-product Karatsuba kernel remains the default and differential
    /// control until isolated end-to-end promotion gates pass.
    pub use_mul5: bool,
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
            reuse_setup_table_for_walkers: false,
            use_mul5: false,
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
fn sadd<N: NativeSink>(a: u64, b: u64, n: u64, c: &mut N) -> u64 {
    c.bump_scalar_adds(1);
    let (s, overflow) = a.overflowing_add(b);
    if overflow || s >= n {
        s.wrapping_sub(n)
    } else {
        s
    }
}

#[inline(always)]
fn ssub<N: NativeSink>(a: u64, b: u64, n: u64, c: &mut N) -> u64 {
    c.bump_scalar_adds(1);
    if a >= b {
        a - b
    } else {
        a + (n - b)
    }
}

#[inline(always)]
fn smul<N: NativeSink>(a: u64, b: u64, n: u64, c: &mut N) -> u64 {
    c.bump_scalar_muls(1);
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

fn canon<const COUNT: bool, N: NativeSink>(
    f: &FastF3<COUNT>,
    p: EP,
    a: u64,
    b: u64,
    n: u64,
    c: &mut N,
) -> (EP, u64, u64) {
    c.bump_canonicalisations(1);
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
        c.bump_scalar_adds(2);
        (EP::A(x, ny), na, nb)
    }
}

fn new_walker<const COUNT: bool, N: NativeSink>(
    ec: &FastEc<'_, COUNT>,
    f: &FastF3<COUNT>,
    g: &EP,
    q: &EP,
    n: u64,
    rng: &mut Rng,
    c: &mut N,
) -> Walker {
    loop {
        let (a, b) = (rng.below(n), rng.below(n - 1) + 1);
        c.bump_rng_draws(2);
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

/// Construct one walker from the already cold-charged width-two joint table.
///
/// Coefficients are still sampled and rejected one walker at a time. This is
/// intentionally not a cross-walker batch: an infinity retry must consume the
/// same next coefficient pair as the baseline and must retain the field work
/// spent on the rejected attempt.
fn new_walker_with_table<const COUNT: bool, N: NativeSink>(
    ec: &FastEc<'_, COUNT>,
    f: &FastF3<COUNT>,
    table: &[EP; 16],
    n: u64,
    rng: &mut Rng,
    c: &mut N,
) -> Walker {
    loop {
        let (a, b) = (rng.below(n), rng.below(n - 1) + 1);
        c.bump_rng_draws(2);
        let mut points = ec.batch_window2_mul(table, &[(a, b)]);
        let p = points.pop().expect("one coefficient pair yields one point");
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

struct InitialState {
    ma: Vec<u64>,
    mb: Vec<u64>,
    mp: Vec<EP>,
    walkers: Vec<Walker>,
}

/// Build the jump table and initial walkers. The candidate keeps the exact
/// baseline RNG stream and coefficients; only point construction changes.
fn initial_state<const COUNT: bool, N: NativeSink>(
    ec: &FastEc<'_, COUNT>,
    f: &FastF3<COUNT>,
    g: &EP,
    q: &EP,
    n: u64,
    r: usize,
    batch: usize,
    rng: &mut Rng,
    native: &mut N,
    batch_setup: bool,
    reuse_setup_table_for_walkers: bool,
) -> InitialState {
    assert!(
        batch_setup || !reuse_setup_table_for_walkers,
        "setup-table walker reuse requires batched setup"
    );
    let mut ma = Vec::with_capacity(r);
    let mut mb = Vec::with_capacity(r);
    for _ in 0..r {
        ma.push(rng.below(n - 1) + 1);
        mb.push(rng.below(n - 1) + 1);
        native.bump_rng_draws(2);
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
    // Keep walker construction on the exact sequential retry path. Apart
    // from charging every attempted point, this centralizes all Walker state
    // initialization for later hot-loop cache fields.
    let walkers = if reuse_setup_table_for_walkers {
        (0..batch)
            .map(|_| new_walker_with_table(ec, f, &table, n, rng, native))
            .collect()
    } else {
        (0..batch)
            .map(|_| new_walker(ec, f, g, q, n, rng, native))
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
fn batched_lookahead_step<const COUNT: bool, N: NativeSink>(
    ec: &FastEc<'_, COUNT>,
    jumps: &[EP],
    walkers: &[Walker],
    native: &mut N,
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
            native.bump_lookahead_tests(1);
            let candidate_hash = match candidate {
                EP::Inf => None,
                EP::A(x, _) => {
                    native.bump_hashes(1);
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
fn single_lookahead_step<const COUNT: bool, N: NativeSink>(
    ec: &FastEc<'_, COUNT>,
    f: &FastF3<COUNT>,
    jumps: &[EP],
    ma: &[u64],
    mb: &[u64],
    p: &EP,
    a: u64,
    b: u64,
    n: u64,
    native: &mut N,
) -> Option<(EP, u64, u64, u64, u64)> {
    let r = jumps.len();
    let EP::A(x, _) = p else { return None };
    native.bump_hashes(1);
    let mut j = partition_index(hx(x), r);
    let mut adds = 0u64;
    let mut extra = 0u64;
    for attempt in 0..r {
        let candidate = ec.add(p, &jumps[j]);
        adds += 1;
        native.bump_lookahead_tests(1);
        let same_partition = match candidate {
            EP::Inf => false,
            EP::A(nx, _) => {
                native.bump_hashes(1);
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

fn collision_scalar<N: NativeSink>(
    n: u64,
    y: &K,
    a: u64,
    b: u64,
    y2: &K,
    a2: u64,
    b2: u64,
    c: &mut N,
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
    c.bump_scalar_invs(1);
    let di = inv_mod_u64(den, n);
    Some(smul(num, di, n, c))
}

fn strong_rho_impl<const FIELD_COUNT: bool, N: NativeSink + Default>(
    inst: &Inst,
    k_true: u64,
    seed: u64,
    opts: StrongOpts,
) -> StrongOut {
    assert!(opts.batch > 0 && opts.r > 0 && opts.dp_bits < 63 && opts.snapshot_period > 0);
    let start = Instant::now();
    let n = inst.ell;
    let f = FastF3::<FIELD_COUNT>::from_instance_with_mul5(inst, opts.use_mul5);
    let ec = FastEc::from_instance(&f, inst);
    let mut native = N::default();
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
        opts.reuse_setup_table_for_walkers,
    );
    let setup_field = f.counts();
    let setup_native = native.snapshot();
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
                    native.bump_hashes(1);
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
                native.bump_cycle_compares(1);
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
                native.bump_hashes(1);
            }
            if walkers[i].local % opts.snapshot_period == 0 {
                walkers[i].snapshot = Some((p, walkers[i].local));
            }

            walkers[i].p = p;
            walkers[i].h = Some(h);
            walkers[i].a = a;
            walkers[i].b = b;
            walkers[i].tail += 1;

            native.bump_dp_tests(1);
            if (h >> 20) & mask == 0 {
                native.bump_table_ops(1);
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
                    native.bump_table_ops(1);
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
    let walk_native_end = native.snapshot();
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

/// Exactly counted reference path. Its field-operation totals remain the
/// authoritative input to modeled-cost comparisons.
pub fn strong_rho(inst: &Inst, k_true: u64, seed: u64, opts: StrongOpts) -> StrongOut {
    strong_rho_impl::<true, CountedNative>(inst, k_true, seed, opts)
}

/// Production arithmetic path with field-operation counter updates removed at
/// monomorphization time. Its zero field totals are intentionally non-authoritative.
pub fn strong_rho_uncounted(inst: &Inst, k_true: u64, seed: u64, opts: StrongOpts) -> StrongOut {
    strong_rho_impl::<false, CountedNative>(inst, k_true, seed, opts)
}

/// Selectable production path with both field and native counter mutation
/// removed at monomorphization time. Its zero count families are deliberately
/// non-authoritative; it shares the exact mathematical walk with the counted
/// and field-counter-free arms.
#[inline(never)]
pub fn strong_rho_bare(inst: &Inst, k_true: u64, seed: u64, opts: StrongOpts) -> StrongOut {
    strong_rho_impl::<false, BareNative>(inst, k_true, seed, opts)
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
    let check_inverses = |f: &FastF3, kf: &crate::common::Kf, seed: u64, label: &str| {
        let mut rng = Rng::new(seed);
        let mut direct_range = (u64::MAX, 0u64);
        let mut reference_range = (u64::MAX, 0u64);
        for _ in 0..200 {
            let a = loop {
                let a = kf.rand(&mut rng);
                if a != KZERO {
                    break a;
                }
            };
            let before = f.counts();
            let direct = f.inv(&a);
            let direct_counts = f.counts().since(before);
            let before = f.counts();
            let reference = f.inv_frobenius_reference(&a);
            let reference_counts = f.counts().since(before);

            assert_eq!(direct, reference, "{label}: direct/reference for {a:?}");
            assert_eq!(direct, kf.inv(&a), "{label}: frozen inverse for {a:?}");
            assert_eq!(f.mul(&a, &direct), KONE, "{label}: identity for {a:?}");
            assert_eq!(
                direct_counts.fq_mul + 4,
                reference_counts.fq_mul,
                "{label}: fq_mul delta for {a:?}"
            );
            assert_eq!(direct_counts.fq_inv, 1, "{label}: direct fq_inv");
            assert_eq!(reference_counts.fq_inv, 1, "{label}: reference fq_inv");
            assert_eq!(direct_counts.f3_inv, 1, "{label}: direct f3_inv");
            assert_eq!(reference_counts.f3_inv, 1, "{label}: reference f3_inv");
            assert_eq!(direct_counts.f3_mul, 0, "{label}: direct f3_mul");
            assert_eq!(reference_counts.f3_mul, 1, "{label}: reference f3_mul");

            direct_range.0 = direct_range.0.min(direct_counts.fq_mul);
            direct_range.1 = direct_range.1.max(direct_counts.fq_mul);
            reference_range.0 = reference_range.0.min(reference_counts.fq_mul);
            reference_range.1 = reference_range.1.max(reference_counts.fq_mul);
        }
        eprintln!(
            "selftest: {label}: 200 inverses match; fq_mul direct={}..{}, reference={}..{}, delta=4 each",
            direct_range.0, direct_range.1, reference_range.0, reference_range.1,
        );
    };
    {
        let inst = instance();
        let f = FastF3::<true>::from_instance(&inst);
        let uncounted = FastF3::<false>::from_instance(&inst);
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let a = inst.kf.rand(&mut rng);
            let b = inst.kf.rand(&mut rng);
            assert_eq!(f.add(&a, &b), inst.kf.add(&a, &b));
            assert_eq!(uncounted.add(&a, &b), inst.kf.add(&a, &b));
            assert_eq!(f.sub(&a, &b), inst.kf.sub(&a, &b));
            assert_eq!(uncounted.sub(&a, &b), inst.kf.sub(&a, &b));
            assert_eq!(f.mul(&a, &b), inst.kf.mul(&a, &b));
            assert_eq!(uncounted.mul(&a, &b), inst.kf.mul(&a, &b));
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
            assert_eq!(uncounted.sq(&a), sq);
            if a != KZERO {
                assert_eq!(f.inv(&a), inst.kf.inv(&a));
                assert_eq!(f.mul(&a, &f.inv(&a)), KONE);
                assert_eq!(uncounted.inv(&a), inst.kf.inv(&a));
                assert_eq!(uncounted.mul(&a, &uncounted.inv(&a)), KONE);
            }
        }
        assert_eq!(uncounted.counts(), FieldCounts::default());
        check_inverses(&f, &inst.kf, 8, "small-c");
        eprintln!("selftest: fast field matches frozen field");
    }
    {
        let inst = instance();
        let baseline = FastF3::<true>::from_instance(&inst);
        let mul5 = FastF3::<true>::from_instance_with_mul5(&inst, true);
        let mul5_uncounted = FastF3::<false>::from_instance_with_mul5(&inst, true);
        let mut rng = Rng::new(0x17_33_2026);
        for _ in 0..20_000 {
            let a = [rng.below(inst.q), rng.below(inst.q), rng.below(inst.q)];
            let b = [rng.below(inst.q), rng.below(inst.q), rng.below(inst.q)];
            let before = mul5.counts();
            let got = mul5.mul(&a, &b);
            let delta = mul5.counts().since(before);
            assert_eq!(got, baseline.mul(&a, &b));
            assert_eq!(got, inst.kf.mul(&a, &b));
            assert_eq!(mul5_uncounted.mul(&a, &b), got);
            assert_eq!(delta.fq_mul, 5);
            assert_eq!(delta.fq_small_div, 4);
            assert_eq!(delta.f3_mul, 1);
            assert_eq!(delta.fq_inv, 0);
            assert_eq!(delta.f3_sq, 0);
            assert_eq!(delta.f3_inv, 0);
        }
        assert_eq!(mul5_uncounted.counts(), FieldCounts::default());
        eprintln!(
            "selftest: production mul5 matches Karatsuba/frozen field on 20000 pairs; per call 5 fq_mul + 4 fq_small_div"
        );
    }
    {
        let q = 223;
        let kf = crate::common::Kf {
            fp: crate::common::Fp { q },
            c: 18,
            w: 39,
            w2: 183,
            size: (q as u128).pow(3),
        };
        let f = FastF3 {
            q,
            reciprocal: barrett_reciprocal(q),
            setup_reciprocal_div: 1,
            c: kf.c,
            w: kf.w,
            w2: kf.w2,
            use_mul5: false,
            counts: Cell::new(FieldCounts::default()),
        };
        check_inverses(&f, &kf, 10, "fallback-c=18");
    }
    {
        let inst = instance();
        let f = FastF3::<true>::from_instance(&inst);
        let uncounted = FastF3::<false>::from_instance(&inst);
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
        let got_uncounted = uncounted.batch_inv(&values);
        assert_eq!(got_uncounted, got);
        for (x, xi) in values.iter().zip(got.iter()) {
            assert_eq!(f.mul(x, xi), KONE);
            assert_eq!(*xi, inst.kf.inv(x));
        }
        assert_eq!(uncounted.counts(), FieldCounts::default());
        eprintln!("selftest: batch inverse matches pointwise inversion");
    }
    {
        let inst = instance();
        let f = FastF3::<true>::from_instance(&inst);
        let ec = FastEc::from_instance(&f, &inst);
        let uncounted = FastF3::<false>::from_instance(&inst);
        let ec_uncounted = FastEc::from_instance(&uncounted, &inst);
        let points: Vec<_> = (1..=40).map(|k| inst.ec.mul(&inst.g, k)).collect();
        let jumps: Vec<_> = (0..40)
            .map(|k| inst.ec.mul(&inst.g, (3 * k + 17) as u128))
            .collect();
        for (p, q) in points.iter().zip(jumps.iter()) {
            assert_eq!(ec.add(p, q), inst.ec.add(p, q));
            assert_eq!(ec_uncounted.add(p, q), inst.ec.add(p, q));
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
        assert_eq!(
            ec_uncounted.batch_add(&points, &jumps),
            ec.batch_add(&points, &jumps)
        );
        for k in [1u128, 2, 3, 17, 255, 65_537] {
            assert_eq!(ec.mul(&inst.g, k), inst.ec.mul(&inst.g, k));
            assert_eq!(ec_uncounted.mul(&inst.g, k), inst.ec.mul(&inst.g, k));
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
        let table_uncounted = ec_uncounted.window2_table(&inst.g, &points[16]);
        assert_eq!(table_uncounted, table);
        assert_eq!(
            ec_uncounted.batch_window2_mul(&table_uncounted, &coeffs),
            expected
        );
        assert_eq!(uncounted.counts(), FieldCounts::default());
        eprintln!(
            "selftest: complete batches and width-2 simultaneous multiplication match frozen curve; cold table={table_counts:?}"
        );
    }
    {
        let inst = instance();
        let target = inst.ec.mul(&inst.g, 119);
        let f_legacy = FastF3::<true>::from_instance(&inst);
        let ec_legacy = FastEc::from_instance(&f_legacy, &inst);
        let f_batch = FastF3::<true>::from_instance(&inst);
        let ec_batch = FastEc::from_instance(&f_batch, &inst);
        let f_table = FastF3::<true>::from_instance(&inst);
        let ec_table = FastEc::from_instance(&f_table, &inst);
        let mut rng_legacy = Rng::new(0x51A7_E001);
        let mut rng_batch = Rng::new(0x51A7_E001);
        let mut rng_table = Rng::new(0x51A7_E001);
        let mut native_legacy = CountedNative::default();
        let mut native_batch = CountedNative::default();
        let mut native_table = CountedNative::default();
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
            false,
        );
        let tabled = initial_state(
            &ec_table,
            &f_table,
            &inst.g,
            &target,
            inst.ell,
            32,
            8,
            &mut rng_table,
            &mut native_table,
            true,
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
        assert_eq!(native_legacy.snapshot(), native_batch.snapshot());
        assert_eq!(batched.ma, tabled.ma);
        assert_eq!(batched.mb, tabled.mb);
        assert_eq!(batched.mp, tabled.mp);
        assert_eq!(batched.walkers.len(), tabled.walkers.len());
        for (a, b) in batched.walkers.iter().zip(&tabled.walkers) {
            assert_eq!(a.p, b.p);
            assert_eq!((a.a, a.b, a.tail, a.local), (b.a, b.b, b.tail, b.local));
            assert_eq!(a.snapshot, b.snapshot);
        }
        assert_eq!(rng_batch.0, rng_table.0);
        assert_eq!(native_batch.snapshot(), native_table.snapshot());
        let lc = f_legacy.counts();
        let bc = f_batch.counts();
        let tc = f_table.counts();
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
        eprintln!(
            "selftest: setup state exact; field legacy={lc:?} batch={bc:?} walker_table={tc:?}"
        );
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
            let uncounted = strong_rho_uncounted(
                &inst,
                k,
                100 + i as u64,
                StrongOpts::for_instance(&inst, width),
            );
            let bare = strong_rho_bare(
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
            assert_eq!(uncounted.setup_reciprocal_div, 1);
            assert_eq!(bare.setup_reciprocal_div, 1);
            assert_eq!(uncounted.ok, out.ok);
            assert_eq!(uncounted.found, out.found);
            assert_eq!(uncounted.steps, out.steps);
            assert_eq!(uncounted.dps, out.dps);
            assert_eq!(uncounted.restarts, out.restarts);
            assert_eq!(uncounted.lookahead, out.lookahead);
            assert_eq!(uncounted.cycles, out.cycles);
            assert_eq!(uncounted.cycle_adds, out.cycle_adds);
            assert_eq!(uncounted.setup_native, out.setup_native);
            assert_eq!(uncounted.walk_native, out.walk_native);
            assert_eq!(uncounted.setup_field, FieldCounts::default());
            assert_eq!(uncounted.walk_field, FieldCounts::default());
            assert_eq!(uncounted.verify_field, FieldCounts::default());
            assert_eq!(bare.ok, out.ok);
            assert_eq!(bare.found, out.found);
            assert_eq!(bare.steps, out.steps);
            assert_eq!(bare.dps, out.dps);
            assert_eq!(bare.restarts, out.restarts);
            assert_eq!(bare.lookahead, out.lookahead);
            assert_eq!(bare.cycles, out.cycles);
            assert_eq!(bare.cycle_adds, out.cycle_adds);
            assert_eq!(bare.setup_field, FieldCounts::default());
            assert_eq!(bare.walk_field, FieldCounts::default());
            assert_eq!(bare.verify_field, FieldCounts::default());
            assert_eq!(bare.setup_native, NativeCounts::default());
            assert_eq!(bare.walk_native, NativeCounts::default());
            if width > 1 {
                assert!(out.walk_field.fq_inv < out.steps);
            }
        }
        eprintln!(
            "selftest: counted, field-counter-free, and bare planted logs match at widths 1, 4, 8, 16, and 32"
        );
    }
    {
        let inst = instance();
        let mut legacy_opts = StrongOpts::for_instance(&inst, 8);
        legacy_opts.batch_setup = false;
        let legacy = strong_rho(&inst, 119, 0xC01D, legacy_opts);
        let batched = strong_rho(&inst, 119, 0xC01D, StrongOpts::for_instance(&inst, 8));
        let mut table_opts = StrongOpts::for_instance(&inst, 8);
        table_opts.reuse_setup_table_for_walkers = true;
        let tabled = strong_rho(&inst, 119, 0xC01D, table_opts);
        let mut mul5_opts = StrongOpts::for_instance(&inst, 8);
        mul5_opts.use_mul5 = true;
        let mul5 = strong_rho(&inst, 119, 0xC01D, mul5_opts);
        let mut combined_opts = StrongOpts::for_instance(&inst, 8);
        combined_opts.use_mul5 = true;
        combined_opts.reuse_setup_table_for_walkers = true;
        let combined = strong_rho(&inst, 119, 0xC01D, combined_opts);
        let bare_combined = strong_rho_bare(&inst, 119, 0xC01D, combined_opts);
        assert!(legacy.ok && batched.ok && tabled.ok && mul5.ok && combined.ok);
        assert!(bare_combined.ok);
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
        assert_eq!(batched.found, tabled.found);
        assert_eq!(batched.steps, tabled.steps);
        assert_eq!(batched.dps, tabled.dps);
        assert_eq!(batched.restarts, tabled.restarts);
        assert_eq!(batched.lookahead, tabled.lookahead);
        assert_eq!(batched.cycles, tabled.cycles);
        assert_eq!(batched.cycle_adds, tabled.cycle_adds);
        assert_eq!(batched.setup_native, tabled.setup_native);
        assert_eq!(batched.walk_field, tabled.walk_field);
        assert_eq!(batched.verify_field, tabled.verify_field);
        assert_eq!(batched.walk_native, tabled.walk_native);
        for (kernel_baseline, candidate) in [(&batched, &mul5), (&tabled, &combined)] {
            assert_eq!(batched.found, candidate.found);
            assert_eq!(batched.steps, candidate.steps);
            assert_eq!(batched.dps, candidate.dps);
            assert_eq!(batched.restarts, candidate.restarts);
            assert_eq!(batched.lookahead, candidate.lookahead);
            assert_eq!(batched.cycles, candidate.cycles);
            assert_eq!(batched.cycle_adds, candidate.cycle_adds);
            assert_eq!(batched.setup_native, candidate.setup_native);
            assert_eq!(batched.walk_native, candidate.walk_native);
            for (baseline_phase, candidate_phase) in [
                (kernel_baseline.setup_field, candidate.setup_field),
                (kernel_baseline.walk_field, candidate.walk_field),
                (kernel_baseline.verify_field, candidate.verify_field),
            ] {
                assert_eq!(baseline_phase.f3_mul, candidate_phase.f3_mul);
                assert_eq!(
                    baseline_phase.fq_mul - candidate_phase.fq_mul,
                    baseline_phase.f3_mul
                );
                assert_eq!(candidate_phase.fq_small_div, 4 * candidate_phase.f3_mul);
            }
        }
        assert_eq!(bare_combined.found, combined.found);
        assert_eq!(bare_combined.steps, combined.steps);
        assert_eq!(bare_combined.dps, combined.dps);
        assert_eq!(bare_combined.restarts, combined.restarts);
        assert_eq!(bare_combined.lookahead, combined.lookahead);
        assert_eq!(bare_combined.cycles, combined.cycles);
        assert_eq!(bare_combined.cycle_adds, combined.cycle_adds);
        assert_eq!(bare_combined.setup_field, FieldCounts::default());
        assert_eq!(bare_combined.walk_field, FieldCounts::default());
        assert_eq!(bare_combined.verify_field, FieldCounts::default());
        assert_eq!(bare_combined.setup_native, NativeCounts::default());
        assert_eq!(bare_combined.walk_native, NativeCounts::default());
        assert!(batched.setup_field.fq_mul <= legacy.setup_field.fq_mul);
        assert!(batched.setup_field.fq_inv <= legacy.setup_field.fq_inv);
        assert!(
            batched.setup_field.fq_mul < legacy.setup_field.fq_mul
                || batched.setup_field.fq_inv < legacy.setup_field.fq_inv
        );
        eprintln!(
            "selftest: full trajectory exact; setup legacy={:?} batch={:?} walker_table={:?} mul5={:?} combined={:?}",
            legacy.setup_field,
            batched.setup_field,
            tabled.setup_field,
            mul5.setup_field,
            combined.setup_field
        );
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
                fq_small_div: 0,
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
            fc!(41497, 97, 156454, 5338, 1661, 97, 33, 1027),
            fc!(35246, 1215, 103771, 2431, 1216, 1215, 1210, 1215),
            fc!(189, 5, 610, 14, 9, 5, 0, 6),
            nc!(66, 0, 1, 0, 0, 0, 0, 0, 0, 0),
            nc!(0, 1220, 1182, 1214, 1177, 127, 925, 3556, 1, 1),
        ),
        4 => (
            (Some(118), 1263, 81, 0, 39, 1, 5),
            fc!(48609, 286, 179254, 5877, 1983, 286, 33, 1229),
            fc!(43631, 360, 155202, 5461, 1309, 360, 355, 1308),
            fc!(356, 10, 1126, 26, 16, 10, 0, 11),
            nc!(72, 0, 4, 0, 0, 0, 0, 4, 0, 0),
            nc!(0, 1316, 1268, 1307, 1263, 163, 627, 3782, 1, 1),
        ),
        8 => (
            (Some(219), 2310, 139, 0, 71, 0, 0),
            fc!(57703, 537, 208171, 6544, 2399, 537, 33, 1488),
            fc!(81263, 357, 296497, 10844, 2383, 357, 357, 2383),
            fc!(425, 12, 1342, 31, 19, 12, 0, 13),
            nc!(80, 0, 8, 0, 0, 0, 0, 14, 0, 0),
            nc!(0, 2391, 2310, 2383, 2310, 279, 1286, 6940, 1, 1),
        ),
        16 => (
            (Some(320), 2269, 131, 1, 57, 1, 5),
            fc!(74182, 990, 260785, 7752, 3158, 990, 33, 1956),
            fc!(82592, 250, 304118, 11242, 2428, 250, 188, 2393),
            fc!(349, 9, 1138, 26, 17, 9, 0, 10),
            nc!(96, 0, 16, 0, 0, 0, 0, 10, 0, 0),
            nc!(2, 2355, 2275, 2333, 2269, 264, 198, 6768, 1, 1),
        ),
        32 => (
            (Some(421), 2623, 150, 0, 78, 0, 0),
            fc!(109508, 1967, 373089, 10349, 4762, 1967, 33, 2969),
            fc!(93772, 133, 348582, 13111, 2702, 133, 133, 2702),
            fc!(436, 12, 1390, 32, 20, 12, 0, 13),
            nc!(128, 0, 32, 0, 0, 0, 0, 34, 0, 0),
            nc!(0, 2734, 2623, 2702, 2623, 301, 0, 7800, 1, 1),
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
