//! `rootfind-mitm` and `rootfind-pairs`: the root-finding decomposition
//! oracles of `research/pdp_root_finder_20261012` as `ic_framework`
//! plug-ins, so that `ic.pipeline` can run them on any counted group
//! and charge them in the harness's unit.
//!
//! Both decide a target `R` by the group law alone, never by an
//! algebraic system.  For a base with abscissae `x_1 < … < x_A` (two
//! signed points above each, one above `x = 0`):
//!
//! - **`rootfind-mitm`** (`m = 3`): a pair table of `P_i + P_j`
//!   (negation folded, the harness's [`PairTable`]) built once per run;
//!   per target the `2A` points `R ∓ P_k`, one representative `P_k` an
//!   abscissa, computed in blocks of 32 abscissae and probed in the table,
//!   stopping at the first hit.  `m = 2` probes `R` itself.
//! - **`rootfind-pairs`** (`m = 3`): no table; the row points `U = R ∓ P_i`,
//!   and for each row the candidates `U ∓ P_j` over the abscissae
//!   `x_j ≥ x_i`, tested for membership in the base.  The enumeration
//!   floor, kept as the memoryless control.
//!
//! **Charged:** every point addition, one each, through the ledger: the
//! table build, the additions a target of `rootfind-mitm` (`2A` for a
//! refutation, the count `mitm:m=3` charges over the `2A` signed points;
//! at most a block more than `mitm` on a hit), the probe's confirming
//! addition on a key hit, and every candidate of `rootfind-pairs`.
//! **Counted, not charged:** table lookups and membership tests
//! (`lookups`).  Where the group batches ([`PairedAdd`]: the binary
//! group, through Montgomery's trick over a block) the additions are
//! still charged one each: the shared inversion is how they are done,
//! not work beside them, so the recorded cost is complete.  The witness
//! indices returned are exact: a probe re-adds the pair it found and a
//! membership test compares full point keys.
//!
//! The binary group batches through [`FastCurve::add_many`]; the prime
//! group uses the default one-inversion-an-addition path, so on a prime
//! curve the two ids measure the same additions as `mitm` and `subtract`
//! with the memoryless three-summand arm added.
//!
//! **`rootfind-mitm-prime-batch`** (prime curves): `rootfind-mitm` with each
//! block's additions done by one shared inversion
//! ([`PairedAdd::add_many_batched`], `prime_fast`'s Montgomery-form batch
//! addition).  The same points, probes, relations and charged additions;
//! the field tally is the batched one (docs/perf/20261112-rust-deep-audit.md).
//!
//! [`FastCurve::add_many`]: crate::cryptanalysis::koblitz_fast::FastCurve::add_many

use std::collections::BTreeMap;

use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    BinaryGroup, CountedGroup, FactorBase, GroupOps, OracleCounters, PairTable, PrimeCurve,
    PrimePoint,
};
use crate::cryptanalysis::koblitz_fast::BatchScratch;

/// `R + Q_j` for every `j`, appended to `out`, charged one addition
/// each.  A group that can share one field inversion over the slice
/// overrides the default and counts that inversion.
pub trait PairedAdd: CountedGroup {
    fn add_many(
        &self,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        r: Self::Elt,
        qs: &[Self::Elt],
        out: &mut Vec<Self::Elt>,
    ) {
        for &q in qs {
            out.push(self.add(ops, r, q));
        }
        let _ = counters;
    }

    /// The additions of `rootfind-mitm-prime-batch`: the default is
    /// [`PairedAdd::add_many`], so only a group that overrides this one (the
    /// prime curve) behaves differently, and every existing oracle keeps
    /// calling `add_many`.
    fn add_many_batched(
        &self,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        r: Self::Elt,
        qs: &[Self::Elt],
        out: &mut Vec<Self::Elt>,
    ) {
        self.add_many(ops, counters, r, qs, out);
    }
}

impl PairedAdd for PrimeCurve {
    /// `R + Q_j` through `prime_fast`'s Montgomery-form batch addition: one
    /// field inversion over the slice's ordinary additions (Montgomery's
    /// trick), the same points as the serial group law in the same order.
    /// Charged one addition each, as the default; the field tally books
    /// what the batched path performs, as
    /// [`CountedGroup::native_lane_walk_adder`] does: `2M` per point
    /// converted into Montgomery form (`R` once a call, each finite `Q_j`),
    /// `2M + 1S` per ordinary addition and, for the slice, one inversion and
    /// `3k + 1` multiplications; an exceptional addition (a doubling) pays
    /// its own inversion; `2` Montgomery reductions per finite output
    /// converted back.  A curve whose `p` exceeds `prime_fast::MAX_BITS`
    /// bits takes the serial path.
    fn add_many_batched(
        &self,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        r: Self::Elt,
        qs: &[Self::Elt],
        out: &mut Vec<Self::Elt>,
    ) {
        use crate::cryptanalysis::prime_fast::{BatchAddScratch, FastCurve, Pt, MAX_BITS};
        if qs.is_empty() {
            return;
        }
        if self.p >= (1u64 << MAX_BITS) {
            self.add_many(ops, counters, r, qs, out);
            return;
        }
        struct Native {
            key: (u64, u64, u64),
            fast: FastCurve,
            scratch: BatchAddScratch,
            left: Vec<Option<Pt>>,
            right: Vec<Option<Pt>>,
        }
        thread_local! {
            static NATIVE: std::cell::RefCell<Option<Native>> = const { std::cell::RefCell::new(None) };
        }
        NATIVE.with(|cell| {
            let mut slot = cell.borrow_mut();
            let key = (self.p, self.a, self.b);
            if slot.as_ref().is_none_or(|n| n.key != key) {
                *slot = Some(Native {
                    key,
                    fast: FastCurve::new("rootfind-batch", self.p, self.a, self.b, 1),
                    scratch: BatchAddScratch::default(),
                    left: Vec::new(),
                    right: Vec::new(),
                });
            }
            let n = slot.as_mut().expect("set above");
            let fr = if r.infinity {
                None
            } else {
                ops.field_ops.muls += 2;
                Some(n.fast.point(r.x, r.y))
            };
            n.left.clear();
            n.right.clear();
            for &q in qs {
                let fq = if q.infinity {
                    None
                } else {
                    ops.field_ops.muls += 2;
                    Some(n.fast.point(q.x, q.y))
                };
                ops.adds += 1;
                match (fr, fq) {
                    (Some(a), Some(b)) if a.x != b.x => {
                        ops.field_ops.muls += 2;
                        ops.field_ops.sqrs += 1;
                    }
                    (Some(a), Some(b)) if a.y == b.y && a.y != 0 => {
                        ops.field_ops.muls += 2;
                        ops.field_ops.sqrs += 2;
                        ops.field_ops.invs += 1;
                        ops.field_ops.muls += 1; // Fp64::inv Montgomery correction.
                    }
                    _ => {}
                }
                n.left.push(fr);
                n.right.push(fq);
            }
            let stats = n
                .fast
                .add_many_batched(&mut n.left, &n.right, &mut n.scratch);
            if stats.ordinary > 0 {
                ops.field_ops.invs += 1;
                ops.field_ops.muls += 3 * stats.ordinary as u64 + 1;
            }
            for &s in &n.left {
                out.push(match s {
                    Some(pt) => {
                        ops.montgomery_reductions += 2;
                        let (x, y) = n.fast.canonical(pt);
                        PrimePoint::affine(x, y)
                    }
                    None => PrimePoint::INFINITY,
                });
            }
        });
        let _ = counters;
    }
}

impl PairedAdd for BinaryGroup<'_> {
    fn add_many(
        &self,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        r: Self::Elt,
        qs: &[Self::Elt],
        out: &mut Vec<Self::Elt>,
    ) {
        if qs.is_empty() {
            return;
        }
        ops.adds += qs.len() as u64;
        let _ = counters;
        // One batch-inversion scratch a thread, reused across calls
        // (`FastCurve::add_many` clears what it uses), instead of a fresh
        // one a block (assessment 2026-11-10, R5).
        thread_local! {
            static SCRATCH: std::cell::RefCell<BatchScratch> =
                std::cell::RefCell::new(BatchScratch::default());
        }
        SCRATCH.with(|s| self.0.add_many(r, qs, out, &mut s.borrow_mut()));
    }
}

/// Abscissae a batch of `rootfind-mitm`: `R ∓ P_k` for a block are
/// computed together, then probed, so a hit stops the scan within its
/// block and a refutation costs the same `2A` additions however it is
/// batched.
const BLOCK: usize = 32;

/// One signed representative an abscissa: the base point's index, the
/// point, its negative, and whether the point is its own negative (the
/// 2-torsion point above `x = 0`, which gets one addition, not two).
struct Reps<E> {
    index: Vec<usize>,
    point: Vec<E>,
    negated: Vec<E>,
    self_negated: Vec<bool>,
}

impl<E: Copy> Reps<E> {
    fn of<G: CountedGroup<Elt = E>>(g: &G, fb: &FactorBase<E>) -> Self {
        let mut reps = Reps {
            index: Vec::with_capacity(fb.abscissa_list.len()),
            point: Vec::with_capacity(fb.abscissa_list.len()),
            negated: Vec::with_capacity(fb.abscissa_list.len()),
            self_negated: Vec::with_capacity(fb.abscissa_list.len()),
        };
        for x in &fb.abscissa_list {
            let above = fb.x_index.get(x).expect("every listed abscissa is indexed");
            let i = above[0];
            reps.index.push(i);
            reps.point.push(fb.points[i]);
            reps.negated.push(g.neg(fb.points[i]));
            reps.self_negated.push(fb.neg_index[i] == i);
        }
        reps
    }
}

/// Reusable buffers of [`row_points`], held by an oracle so that a block
/// allocates nothing (assessment 2026-11-10, R5): the sums are the same
/// points in the same order as the allocating version returned.
struct RowBufs<E> {
    minus: Vec<E>,
    plus_src: Vec<E>,
    plus: Vec<E>,
    out: Vec<(usize, E)>,
}

impl<E> Default for RowBufs<E> {
    fn default() -> Self {
        Self {
            minus: Vec::new(),
            plus_src: Vec::new(),
            plus: Vec::new(),
            out: Vec::new(),
        }
    }
}

/// `R ∓ P_k` for every representative, as `(signed index of P_k, R − P_k)`
/// pairs: `R − P_k` carries `k`'s own index, `R + P_k` the index of `−P_k`,
/// so that `R = P_{index} + (the returned point)` in both cases.  The
/// pairs are left in `bufs.out`.
fn row_points<G: PairedAdd>(
    g: &G,
    reps: &Reps<G::Elt>,
    fb: &FactorBase<G::Elt>,
    ops: &mut GroupOps,
    counters: &mut OracleCounters,
    r: G::Elt,
    from: usize,
    to: usize,
    bufs: &mut RowBufs<G::Elt>,
    batched: bool,
) {
    let add = |ops: &mut GroupOps,
               counters: &mut OracleCounters,
               qs: &[G::Elt],
               out: &mut Vec<G::Elt>| {
        if batched {
            g.add_many_batched(ops, counters, r, qs, out);
        } else {
            g.add_many(ops, counters, r, qs, out);
        }
    };
    bufs.minus.clear();
    add(ops, counters, &reps.negated[from..to], &mut bufs.minus);
    bufs.plus_src.clear();
    bufs.plus_src.extend(
        (from..to)
            .filter(|&k| !reps.self_negated[k])
            .map(|k| reps.point[k]),
    );
    bufs.plus.clear();
    add(ops, counters, &bufs.plus_src, &mut bufs.plus);
    bufs.out.clear();
    for (k, &s) in (from..).zip(&bufs.minus) {
        bufs.out.push((reps.index[k], s));
    }
    let mut it = bufs.plus.iter();
    for k in from..to {
        if !reps.self_negated[k] {
            bufs.out.push((
                fb.neg_index[reps.index[k]],
                *it.next().expect("one sum a point"),
            ));
        }
    }
}

/// The `rootfind-mitm` oracle.
pub struct RootfindMitmOracle<E> {
    m: u32,
    /// `rootfind-mitm-prime-batch`: the row additions through
    /// [`PairedAdd::add_many_batched`].
    batched: bool,
    table: Option<PairTable>,
    reps: Option<Reps<E>>,
    bufs: RowBufs<E>,
}

impl<E> RootfindMitmOracle<E> {
    pub fn new(m: u32) -> Self {
        Self {
            m,
            batched: false,
            table: None,
            reps: None,
            bufs: RowBufs::default(),
        }
    }

    /// `rootfind-mitm-prime-batch`: `rootfind-mitm`'s table, blocks, probes
    /// and charge, with each block's `R ∓ P_k` computed by one batched
    /// inversion ([`PairedAdd::add_many_batched`]).  On a prime curve the
    /// field tally changes (an inversion a block instead of an addition);
    /// the points, their order, the relations and the charged additions are
    /// `rootfind-mitm`'s.
    pub fn new_prime_batch(m: u32) -> Self {
        Self {
            batched: true,
            ..Self::new(m)
        }
    }
}

impl<G: PairedAdd> DecompositionOracle<G> for RootfindMitmOracle<G::Elt> {
    fn name(&self) -> &str {
        if self.batched {
            "rootfind-mitm-prime-batch"
        } else {
            "rootfind-mitm"
        }
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "root-finding by the group law over a pair table ({}): the 2A points R ∓ P_k, one \
             representative an abscissa, computed in one batch and probed; m = {}",
            if params.flag("full_table") {
                "built in full"
            } else {
                "negation folded"
            },
            self.m
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("m", "summands a relation: 2 (probe R) or 3 (probe R ∓ P_k)"),
            (
                "full_table",
                "1 to build every i ≤ j pair instead of one pair per orbit under negation",
            ),
        ]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<G>,
        fb: &FactorBase<G::Elt>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        if !matches!(self.m, 2 | 3) {
            return Err(format!("rootfind-mitm runs at m = 2 or 3, not {}", self.m));
        }
        let table = if params.flag("full_table") {
            PairTable::build(ctx.group, fb)
        } else {
            PairTable::build_negation_folded(ctx.group, fb)
        };
        ops.merge(table.build_ops);
        self.table = Some(table);
        self.reps = Some(Reps::of(ctx.group, fb));
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        let mut counts = BTreeMap::new();
        if let Some(table) = &self.table {
            counts.insert("pair_table_entries".into(), table.entries);
        }
        if let Some(reps) = &self.reps {
            counts.insert("abscissae".into(), reps.index.len() as u64);
        }
        counts
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = self
            .table
            .as_ref()
            .map(PairTable::work_counters)
            .unwrap_or_default();
        // `Reps::of` reads `x_index` once an abscissa.
        if let Some(reps) = &self.reps {
            *out.entry("lookups".to_string()).or_insert(0) += reps.index.len() as u64;
        }
        out.insert("counted:table_insert_attempts".to_string(), 1);
        out
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<G>,
        fb: &FactorBase<G::Elt>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: G::Elt,
    ) -> Option<Vec<usize>> {
        let g = ctx.group;
        let table = self.table.as_ref()?;
        let reps = self.reps.as_ref()?;
        if self.m == 2 {
            return table
                .probe(g, fb, ops, &mut counters.lookups, point)
                .map(|(j, l)| vec![j, l]);
        }
        let a = reps.index.len();
        let mut from = 0;
        while from < a {
            let to = (from + BLOCK).min(a);
            row_points(
                g,
                reps,
                fb,
                ops,
                counters,
                point,
                from,
                to,
                &mut self.bufs,
                self.batched,
            );
            for &(k, s) in &self.bufs.out {
                if g.is_identity(&s) {
                    continue;
                }
                if let Some((j, l)) = table.probe(g, fb, ops, &mut counters.lookups, s) {
                    return Some(vec![k, j, l]);
                }
            }
            from = to;
        }
        None
    }
}

/// The `rootfind-pairs` oracle: the memoryless three-summand search.
pub struct RootfindPairsOracle<E> {
    m: u32,
    reps: Option<Reps<E>>,
    /// Representative position of each signed index, built in `prepare`.
    position: BTreeMap<usize, usize>,
    rows: RowBufs<E>,
    cands: RowBufs<E>,
}

impl<E> RootfindPairsOracle<E> {
    pub fn new(m: u32) -> Self {
        Self {
            m,
            reps: None,
            position: BTreeMap::new(),
            rows: RowBufs::default(),
            cands: RowBufs::default(),
        }
    }
}

impl<G: PairedAdd> DecompositionOracle<G> for RootfindPairsOracle<G::Elt> {
    fn name(&self) -> &str {
        "rootfind-pairs"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, _params: &Params) -> String {
        format!(
            "root-finding by the group law with no table: row points U = R ∓ P_i, candidates \
             U ∓ P_j over x_j ≥ x_i tested in the base; the enumeration floor; m = {}",
            self.m
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[("m", "summands a relation: 3")]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<G>,
        fb: &FactorBase<G::Elt>,
        _params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<(), String> {
        if self.m != 3 {
            return Err(format!("rootfind-pairs runs at m = 3, not {}", self.m));
        }
        let reps = Reps::of(ctx.group, fb);
        // Row `i` pairs with the abscissae `x_j ≥ x_i`; the position of a
        // row point's abscissa among the representatives is recovered
        // from its index.  Built once here, not once a target (it depends
        // only on the base, as `reps` does).
        self.position = reps
            .index
            .iter()
            .enumerate()
            .flat_map(|(pos, &i)| [(i, pos), (fb.neg_index[i], pos)])
            .collect();
        self.reps = Some(reps);
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        let mut counts = BTreeMap::new();
        if let Some(reps) = &self.reps {
            counts.insert("abscissae".into(), reps.index.len() as u64);
        }
        counts
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        // No table; `Reps::of` reads `x_index` once an abscissa.
        let mut out = BTreeMap::new();
        if let Some(reps) = &self.reps {
            out.insert("lookups".to_string(), reps.index.len() as u64);
        }
        out.insert("counted:table_insert_attempts".to_string(), 1);
        out
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<G>,
        fb: &FactorBase<G::Elt>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: G::Elt,
    ) -> Option<Vec<usize>> {
        let g = ctx.group;
        let reps = self.reps.as_ref()?;
        let a = reps.index.len();
        row_points(
            g,
            reps,
            fb,
            ops,
            counters,
            point,
            0,
            a,
            &mut self.rows,
            false,
        );
        for &(si, u) in &self.rows.out {
            if g.is_identity(&u) {
                continue;
            }
            let from = self.position[&si];
            row_points(
                g,
                reps,
                fb,
                ops,
                counters,
                u,
                from,
                a,
                &mut self.cands,
                false,
            );
            for &(sj, c) in &self.cands.out {
                counters.lookups += 1;
                if g.is_identity(&c) {
                    continue;
                }
                if let Some(l) = fb.index_of_key(g.key(&c)) {
                    return Some(vec![si, sj, l]);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::plugins::{BinarySubspaceBase, MitmOracle, PrimeAbscissaBase};
    use super::super::stages::FactorBaseBuilder;
    use super::*;
    use crate::cryptanalysis::ic_boundary::{koblitz_instance, prime_instance_for};

    fn check<G: PairedAdd>(ctx: &InstanceCtx<G>, fb: &FactorBase<G::Elt>, targets: &[G::Elt]) {
        let g = ctx.group;
        let params = Params::default();
        let mut ops = GroupOps::default();
        let mut mitm = MitmOracle::new(3);
        let mut rf_mitm = RootfindMitmOracle::new(3);
        let mut rf_pairs = RootfindPairsOracle::new(3);
        let (_, folded) = Params::parse_spec("mitm:negation_folded=1").unwrap();
        mitm.prepare(ctx, fb, &folded, &mut ops).unwrap();
        rf_mitm.prepare(ctx, fb, &params, &mut ops).unwrap();
        rf_pairs.prepare(ctx, fb, &params, &mut ops).unwrap();
        let (mut found, mut refuted) = (0, 0);
        for &t in targets {
            let (mut o1, mut o2, mut o3) = (
                GroupOps::default(),
                GroupOps::default(),
                GroupOps::default(),
            );
            let (mut c1, mut c2, mut c3) = (
                OracleCounters::default(),
                OracleCounters::default(),
                OracleCounters::default(),
            );
            let a = mitm.decompose(ctx, fb, &mut o1, &mut c1, t);
            let b = rf_mitm.decompose(ctx, fb, &mut o2, &mut c2, t);
            let c = rf_pairs.decompose(ctx, fb, &mut o3, &mut c3, t);
            assert_eq!(a.is_some(), b.is_some(), "verdicts differ on a target");
            assert_eq!(a.is_some(), c.is_some(), "the memoryless verdict differs");
            for w in [a.as_ref(), b.as_ref(), c.as_ref()].into_iter().flatten() {
                let mut scratch = GroupOps::default();
                let mut sum = g.identity();
                for &i in w {
                    sum = g.add(&mut scratch, sum, fb.points[i]);
                }
                assert_eq!(sum, t, "a witness does not sum to its target");
            }
            if a.is_none() {
                // A refutation visits every signed point in both table
                // oracles: the same additions, however they are batched.
                assert_eq!(o1.adds, o2.adds, "refutation charge differs");
                refuted += 1;
            } else {
                found += 1;
            }
        }
        assert!(found > 0 && refuted > 0, "found {found}, refuted {refuted}");
    }

    #[test]
    fn binary_rootfind_oracles_agree_with_mitm_on_a_koblitz_subspace_base() {
        let inst = koblitz_instance(1, 17).expect("registered n17 Koblitz instance");
        let g = BinaryGroup(&inst.fast);
        let ctx = InstanceCtx {
            group: &g,
            generator: inst.generator,
            target: inst.generator,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(inst.n),
        };
        let (_, params) = Params::parse_spec("binary-subspace:dimension=6").unwrap();
        let mut ops = GroupOps::default();
        let fb = BinarySubspaceBase { instance: &inst }
            .build(&ctx, &params, &mut ops)
            .unwrap();
        let mut scratch = GroupOps::default();
        let targets: Vec<_> = (1..=96u64)
            .map(|k| {
                g.mul(
                    &mut scratch,
                    inst.generator,
                    k.wrapping_mul(0x9E37_79B9) % inst.r,
                )
            })
            .collect();
        check(&ctx, &fb, &targets);
    }

    #[test]
    fn prime_rootfind_oracles_agree_with_mitm_on_an_abscissa_base() {
        let inst = prime_instance_for(18, 7);
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target: inst.generator_point(),
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        };
        let (_, params) = Params::parse_spec("prime-abscissa:size=24").unwrap();
        let mut ops = GroupOps::default();
        let fb = PrimeAbscissaBase { instance: &inst }
            .build(&ctx, &params, &mut ops)
            .unwrap();
        let mut scratch = GroupOps::default();
        let targets: Vec<_> = (1..=96u64)
            .map(|k| {
                inst.curve.mul(
                    &mut scratch,
                    inst.generator_point(),
                    k.wrapping_mul(0x9E37_79B9) % inst.r,
                )
            })
            .collect();
        check(&ctx, &fb, &targets);

        // rootfind-mitm-prime-batch: the same witnesses, the same charged
        // additions and lookups on every target, and fewer inversions.
        let mut plain = RootfindMitmOracle::new(3);
        let mut batch = RootfindMitmOracle::new_prime_batch(3);
        assert_eq!(
            DecompositionOracle::<PrimeCurve>::name(&batch),
            "rootfind-mitm-prime-batch"
        );
        let (mut p_ops, mut b_ops) = (GroupOps::default(), GroupOps::default());
        plain.prepare(&ctx, &fb, &params, &mut p_ops).unwrap();
        batch.prepare(&ctx, &fb, &params, &mut b_ops).unwrap();
        assert_eq!(p_ops, b_ops, "the table build is shared");
        let (mut p_inv, mut b_inv) = (0u64, 0u64);
        for &t in &targets {
            let (mut o1, mut o2) = (GroupOps::default(), GroupOps::default());
            let (mut c1, mut c2) = (OracleCounters::default(), OracleCounters::default());
            let a = plain.decompose(&ctx, &fb, &mut o1, &mut c1, t);
            let b = batch.decompose(&ctx, &fb, &mut o2, &mut c2, t);
            assert_eq!(a, b, "witnesses differ");
            assert_eq!((o1.adds, o1.doubles), (o2.adds, o2.doubles));
            assert_eq!(c1.lookups, c2.lookups);
            p_inv += o1.field_ops.invs;
            b_inv += o2.field_ops.invs;
        }
        assert!(
            b_inv * 4 < p_inv,
            "batched {b_inv} vs serial {p_inv} inversions"
        );
    }
}
