//! # Thread-local counts of the curve group operations a computation spent.
//!
//! The binary-curve arithmetic the Koblitz index-calculus pipeline runs
//! on — [`crate::binary_ecc::curve`]'s `point_add` / `point_double`, the
//! single-word [`crate::cryptanalysis::koblitz_fast::FastCurve`] (its
//! affine, batched, lazy, AVX-512 and López–Dahab ladder paths) and
//! [`crate::cryptanalysis::koblitz_fast_arith`]'s 64- and 128-bit curves
//! — increments the counters here, so that a caller can charge a stage
//! that has no `GroupOps` ledger of its own by the difference of two
//! [`ops_snapshot`]s.
//!
//! **What one count is.**  An addition is counted once for every sum of
//! two finite points the code forms, `P + (−P) = O` included; the sum
//! `P + P` is counted as the doubling it delegates to, and a doubling once
//! for every finite point doubled.  A sum with `O` as an operand does no
//! field work and is not counted, nor is the first step of a
//! double-and-add from `O`, which only copies its operand.  A batched
//! call over `k` pairs counts `k`, however many inversions it shares.  A
//! scalar multiplication counts the doublings and additions its ladder
//! actually performs (`bits − 1` and `popcount − 1` for a scalar of
//! `bits` bits with no degenerate step).  Completing a lazy sum's
//! ordinate (`FastCurve::finish_lazy`) is not a second addition.
//!
//! **Native work beside it.**  The same module counts work the unit does
//! not price — Frobenius canonicalisations, pair-table filter probes and
//! bucket reads, Frobenius coordinate maps, and the lanes of a vector
//! batch that started and was declined (its forward pass ran, a fraction
//! of an addition's work per lane, before the scalar path redid the
//! sums) — so a report can list it under `*_uncharged` instead of
//! dropping it.
//!
//! **Threads.**  The counters are per thread: work a Rayon pool or any
//! other thread does is counted on that thread.  A caller that charges
//! by snapshot differences must therefore run the whole computation on
//! one thread (`ecbench`'s `ic.koblitz_descent` runs it inside a
//! one-thread pool and snapshots inside it).
//!
//! The counters never feed back into the arithmetic: no result, random
//! stream or branch depends on them.

use std::cell::Cell;

use serde::{Deserialize, Serialize};

thread_local! {
    static ADDS: Cell<u64> = const { Cell::new(0) };
    static DOUBLES: Cell<u64> = const { Cell::new(0) };
    static SCALAR_MULTS: Cell<u64> = const { Cell::new(0) };
    static CANONICALISATIONS: Cell<u64> = const { Cell::new(0) };
    static FILTER_PROBES: Cell<u64> = const { Cell::new(0) };
    static BUCKET_READS: Cell<u64> = const { Cell::new(0) };
    static FROBENIUS_MAPS: Cell<u64> = const { Cell::new(0) };
    static DECLINED_LANES: Cell<u64> = const { Cell::new(0) };
}

#[inline(always)]
fn bump(c: &'static std::thread::LocalKey<Cell<u64>>, k: u64) {
    c.with(|v| v.set(v.get().wrapping_add(k)));
}

/// `k` point additions.
#[inline(always)]
pub fn count_adds(k: u64) {
    bump(&ADDS, k);
}

/// `k` point doublings.
#[inline(always)]
pub fn count_doubles(k: u64) {
    bump(&DOUBLES, k);
}

/// `k` scalar multiplications (informational: their additions and
/// doublings are counted as such).
#[inline(always)]
pub fn count_scalar_mults(k: u64) {
    bump(&SCALAR_MULTS, k);
}

/// `k` Frobenius-orbit canonicalisations of an abscissa.
#[inline(always)]
pub fn count_canonicalisations(k: u64) {
    bump(&CANONICALISATIONS, k);
}

/// `k` pair-table presence-filter probes.
#[inline(always)]
pub fn count_filter_probes(k: u64) {
    bump(&FILTER_PROBES, k);
}

/// `k` pair-table bucket reads after the filter admitted a key, or
/// point-index reads while recovering the summands of a hit.
#[inline(always)]
pub fn count_bucket_reads(k: u64) {
    bump(&BUCKET_READS, k);
}

/// `k` Frobenius coordinate maps `x ↦ x^q`.
#[inline(always)]
pub fn count_frobenius_maps(k: u64) {
    bump(&FROBENIUS_MAPS, k);
}

/// `k` lanes of a vector batch whose forward pass ran before the batch
/// was declined.
#[inline(always)]
pub fn count_declined_lanes(k: u64) {
    bump(&DECLINED_LANES, k);
}

/// The counters of this thread at one moment, or the difference of two.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpCounts {
    pub adds: u64,
    pub doubles: u64,
    pub scalar_mults: u64,
    pub canonicalisations: u64,
    pub filter_probes: u64,
    pub bucket_reads: u64,
    pub frobenius_maps: u64,
    pub declined_lanes: u64,
}

impl OpCounts {
    /// What was counted between `earlier` and `self`.
    pub fn since(&self, earlier: &OpCounts) -> OpCounts {
        OpCounts {
            adds: self.adds.wrapping_sub(earlier.adds),
            doubles: self.doubles.wrapping_sub(earlier.doubles),
            scalar_mults: self.scalar_mults.wrapping_sub(earlier.scalar_mults),
            canonicalisations: self
                .canonicalisations
                .wrapping_sub(earlier.canonicalisations),
            filter_probes: self.filter_probes.wrapping_sub(earlier.filter_probes),
            bucket_reads: self.bucket_reads.wrapping_sub(earlier.bucket_reads),
            frobenius_maps: self.frobenius_maps.wrapping_sub(earlier.frobenius_maps),
            declined_lanes: self.declined_lanes.wrapping_sub(earlier.declined_lanes),
        }
    }

    /// Add another tally to this one.
    pub fn merge(&mut self, other: &OpCounts) {
        self.adds += other.adds;
        self.doubles += other.doubles;
        self.scalar_mults += other.scalar_mults;
        self.canonicalisations += other.canonicalisations;
        self.filter_probes += other.filter_probes;
        self.bucket_reads += other.bucket_reads;
        self.frobenius_maps += other.frobenius_maps;
        self.declined_lanes += other.declined_lanes;
    }

    /// Group-addition equivalents: additions plus doublings.
    pub fn gae(&self) -> f64 {
        (self.adds + self.doubles) as f64
    }
}

/// This thread's counters now.
pub fn ops_snapshot() -> OpCounts {
    OpCounts {
        adds: ADDS.with(Cell::get),
        doubles: DOUBLES.with(Cell::get),
        scalar_mults: SCALAR_MULTS.with(Cell::get),
        canonicalisations: CANONICALISATIONS.with(Cell::get),
        filter_probes: FILTER_PROBES.with(Cell::get),
        bucket_reads: BUCKET_READS.with(Cell::get),
        frobenius_maps: FROBENIUS_MAPS.with(Cell::get),
        declined_lanes: DECLINED_LANES.with(Cell::get),
    }
}

/// Run `f` and return what it counted on this thread.
pub fn counted<T>(f: impl FnOnce() -> T) -> (T, OpCounts) {
    let before = ops_snapshot();
    let out = f();
    (out, ops_snapshot().since(&before))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::curve::{point_add, point_double, scalar_mul};
    use crate::cryptanalysis::koblitz_fast::{BatchScratch, FastCurve, FastPoint};
    use crate::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
    use num_bigint::BigUint;

    fn setup() -> (KoblitzCurve, FastCurve, FastPoint) {
        let kc = KoblitzCurve::new(0, 23).expect("K_0 over GF(2^23)");
        let fc = FastCurve::new(&kc.curve).expect("one word");
        let g = fc.lift(kc.generator());
        (kc, fc, g)
    }

    #[test]
    fn single_operations_count_once_and_identity_operands_not_at_all() {
        let (kc, fc, g) = setup();
        let g2 = fc.mul_u64(g, 2);
        let ((), c) = counted(|| {
            let _ = fc.add(g, g2);
        });
        assert_eq!((c.adds, c.doubles), (1, 0));
        let ((), c) = counted(|| {
            let _ = fc.add(g, g);
        });
        assert_eq!((c.adds, c.doubles), (0, 1), "P + P is the doubling");
        let ((), c) = counted(|| {
            let _ = fc.add(g, fc.neg(g));
        });
        assert_eq!((c.adds, c.doubles), (1, 0), "P + (−P) is an addition");
        let ((), c) = counted(|| {
            let _ = fc.add(FastPoint::INFINITY, g);
        });
        assert_eq!((c.adds, c.doubles), (0, 0));
        let gb = kc.generator().clone();
        let ((), c) = counted(|| {
            let _ = point_add(&kc.curve, &gb, &point_double(&kc.curve, &gb));
        });
        assert_eq!((c.adds, c.doubles), (1, 1));
    }

    #[test]
    fn a_ladder_counts_bits_minus_one_doublings_and_popcount_minus_one_additions() {
        let (kc, fc, g) = setup();
        for k in [1u64, 2, 3, 0b1011_0110_1, 1_234_567] {
            let bits = 64 - u64::from(k.leading_zeros());
            let ones = u64::from(k.count_ones());
            let ((), c) = counted(|| {
                let _ = fc.mul_u64(g, k);
            });
            assert_eq!(
                (c.doubles, c.adds),
                (bits - 1, ones - 1),
                "fast ladder, k = {k}"
            );
            let gb = kc.generator().clone();
            let ((), c) = counted(|| {
                let _ = scalar_mul(&kc.curve, &gb, &BigUint::from(k));
            });
            assert_eq!(
                (c.doubles, c.adds),
                (bits - 1, ones - 1),
                "general ladder, k = {k}"
            );
            let arith = kc.fast_binary_curve().unwrap();
            let p = Some((g.x, g.y));
            let ((), c) = counted(|| {
                let _ = arith.scalar_mul(p, &BigUint::from(k));
            });
            assert_eq!(
                (c.doubles, c.adds),
                (bits - 1, ones - 1),
                "word ladder, k = {k}"
            );
        }
    }

    #[test]
    fn a_batch_of_k_counts_k_on_every_path() {
        let (kc, fc, g) = setup();
        let qs: Vec<FastPoint> = (2..42u64).map(|k| fc.mul_u64(g, k)).collect();
        let (qx, qy): (Vec<u64>, Vec<u64>) = qs.iter().map(|q| (q.x, q.y)).unzip();
        let mut scratch = BatchScratch::default();
        let (mut out, mut lambdas, mut x3) = (Vec::new(), Vec::new(), Vec::new());
        let ((), c) = counted(|| fc.add_many(g, &qs, &mut out, &mut scratch));
        assert_eq!((c.adds, c.doubles), (40, 0));
        let ((), c) = counted(|| fc.add_many_lazy(g, &qs, &mut out, &mut lambdas, &mut scratch));
        assert_eq!((c.adds, c.doubles), (40, 0));
        let (done, c) =
            counted(|| fc.add_many_lazy_soa(g, &qx, &qy, &mut x3, &mut lambdas, &mut scratch));
        assert_eq!(c.adds, if done { 40 } else { 0 });
        let ps = vec![g; qs.len()];
        let ((), c) = counted(|| fc.add_pairwise(&ps, &qs, &mut out, &mut scratch));
        assert_eq!((c.adds, c.doubles), (40, 0));
        // A batch holding `P`, `−P` and `O`: one doubling, one addition to
        // `O`, nothing for `O`, and the ordinary rest.
        let mut odd = qs.clone();
        odd.push(g);
        odd.push(fc.neg(g));
        odd.push(FastPoint::INFINITY);
        let ((), c) = counted(|| fc.add_many(g, &odd, &mut out, &mut scratch));
        assert_eq!((c.adds, c.doubles), (41, 1));
        let ((), c) = counted(|| fc.add_many_lazy(g, &odd, &mut out, &mut lambdas, &mut scratch));
        assert_eq!((c.adds, c.doubles), (41, 1));
        let arith = kc.fast_binary_curve().unwrap();
        let pairs: Vec<_> = qs
            .iter()
            .map(|q| (g.x, g.y, q.x, q.y, false, false))
            .collect();
        let ((), c) = counted(|| {
            let _ = arith.batch_add(&pairs);
        });
        assert_eq!((c.adds, c.doubles), (40, 0));
        let pts: Vec<_> = qs.iter().map(|q| Some((q.x, q.y))).collect();
        let ((), c) = counted(|| {
            let _ = arith.batch_double(&pts);
        });
        assert_eq!((c.adds, c.doubles), (0, 40));
    }

    #[test]
    fn counting_changes_no_result() {
        // The instrumented paths agree with each other exactly as before.
        let (kc, fc, g) = setup();
        for k in [3u64, 77, 1 << 20, 4_194_301] {
            let fast = fc.lower(fc.mul_u64(g, k));
            let slow = scalar_mul(&kc.curve, kc.generator(), &BigUint::from(k));
            assert_eq!(fast, slow);
        }
    }
}
