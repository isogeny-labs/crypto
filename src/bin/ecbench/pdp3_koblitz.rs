//! The PDP3 decomposition of `koblitz_index_calculus` as an `ic.pipeline`
//! oracle on a Koblitz curve: the Groebner engines (inherited F4, F6-IC,
//! matrix F4, matrix F5) and the root-finding engine
//! (`engine=rootfind`, [`PairSumTable`]) on one base.
//!
//! PR #1333 measured F6-IC, which closes branches of an inherited Boolean
//! F4 search with exact factor-base geometry, in its own worker
//! (`examples/ic_tournament_worker.rs`), on its own wall clock and on one
//! curve.  This module runs the same two functions,
//! [`groebner_decompose`] and [`groebner_decompose_f6_ic`], unmodified,
//! inside the native `ic.pipeline`, so both arms are charged in ecbench's
//! counted unit beside strong rho on the same public point.
//!
//! ## The root-finding engine
//!
//! `pdp3-koblitz:m=2|3,engine=rootfind` is the decomposition
//! `koblitz_index_calculus_dlp` runs under
//! `DecompositionStrategy::PairTable` (`RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`):
//! the pair sums `P_i + P_j` of the base are tabulated once
//! ([`PairSumTable::build_full_within`], the summand-storing tier, so a
//! hit is read and never recovered by a scan), and a target is decomposed
//! by *finding* its partner in the table rather than by row-reducing a
//! Weil-descended Semaev system.  It takes no `degree` and no
//! `node_budget` and refuses them: there is no Groebner search to bound.
//!
//! - **Charged:** the table's `|F|(|F|+1)/2` pair additions
//!   (`|F|` of them doublings), counted from the base's size since the
//!   batched builder carries no ledger; for `m = 3` every `R − P_k` on the
//!   ledger until the first hit; and the re-addition of every hit in the
//!   group before it is returned (fail-closed, as the strategy's own
//!   `decompose` re-checks).
//! - **Counted, not charged:** one `lookup` per table probe, under the
//!   `lookups` counter the other table oracles use (priced only where the
//!   curve has a pinned ratio); the table's entry count in `oracle_setup`.
//!
//! It is exact and complete for `m ∈ {2, 3}`, so against a Groebner
//! engine with an unexhausted node budget it issues the same queries and
//! finds the same relations; the comparison is one of engines on one
//! query stream, as the F4/F6 arms are.  Its `S` is a complete count of
//! group additions, while a Groebner arm's `S` is a lower bound (the
//! solver's word XORs are counted, not charged), so the two are compared
//! on the resource vector and on wall time, not by dividing one `S` by the
//! other.
//!
//! `pdp3-koblitz:m=3,engine=rootfind-window` is the same table with the
//! cyclic third-summand window of [`PairSumTable::decompose_fast_window`]:
//! `⌈|F|/3⌉` probes a target instead of `|F|`, started at `x(R) mod |F|`
//! so the choice is deterministic in the target.  It is a different
//! search (some triples the full scan would catch on a later `k` can be
//! missed here), so it is a separate engine id and is not compared on
//! query-stream identity with `rootfind` or with inherited-F4.  The
//! table build charge is unchanged; only the per-target scan shortens.
//!
//! `pdp3-koblitz:m=2|3,engine=rootfind-negfold` builds the ledgered
//! [`PairTable::build_negation_folded`] instead of the full
//! [`PairSumTable`]: about `|F|²/4` group additions rather than
//! `|F|(|F|+1)/2`, with the same sign-recovering probe as `mitm`'s
//! `negation_folded=1`.  Completeness is unchanged; cold `S` drops by the
//! saved set-up.  `rootfind-window-negfold` is the windowed `m = 3` scan
//! on that half table.
//! `rootfind-negfold-batch` keeps the same ordered half table and probe,
//! but shares an inversion over each binary table row's ordinary additions.
//! For `m = 3`, consecutive scan additions share an inversion in blocks of
//! eight. The scan computes an ordinate only when a table key is present.
//! Completed blocks charge every addition, including any candidates after
//! the first hit. Any wall benefit needs a
//! separate controlled measurement. It has a separate method identity so
//! the scalar arm remains reproducible.
//! `rootfind-negfold-batch-harvest` keeps that table and query order while
//! converting repeated sum keys into checked homogeneous setup rows. The
//! extra point additions enter oracle setup; row elimination enters the
//! linear-algebra phase. It is a separate engine id from the batch control.
//!
//! ## The base
//!
//! `koblitz-standard-subspace:dimension=d` is
//! [`build_standard_subspace_factor_base`]: every point whose abscissa lies
//! in the span of `1, z, …, z^{d−1}` in the curve's polynomial basis.  It is
//! the base #1333's F4/F5/F6 workers used.  The span is not
//! Frobenius-invariant, so its orbits are singletons and the column fold
//! is by negation only.
//!
//! ## What the oracle charges
//!
//! - Every affine point addition the decomposition performs is charged on
//!   the `GroupOps` ledger.  For F6-IC that includes the gate's
//!   `geometric_group_additions` (support closure, residual arithmetic and
//!   independent witness replay), which the solver counts but does not put
//!   on a ledger.  Lifting a witness to signed base points goes through the
//!   framework's `lift_abscissae`, as for every other algebraic oracle.
//! - The Boolean solver's own work is reported through `solver_totals` in
//!   word XORs, read from [`f4_word_ops_thread`] around each call, under the
//!   unit string the other inherited-F4 adapters use.  The runner leaves
//!   that unit unpriced, so `S` is a lower bound and the record says so.
//!   Reductions, splits, propagations and every F6 gate counter ride in the
//!   totals' `extra` map.
//! - The whole of each call — the system's instantiation (and, on the
//!   first call, its template's symbolic `F_{2^n}` build), the solver's
//!   Macaulay builds, sorts, indexes, packing, pivot searches, read-back
//!   and tree scans, the lift of each root and the gate's lookups — is
//!   also reported in `bitop/v1` primitives through the `bitop_*` keys of
//!   `extra` with `bitop_complete = 1`
//!   ([`crypto_lib::cryptanalysis::bitop_tally`]), which is what the
//!   `bitop/v1` work view prices.
//!
//! The word-XOR counter is per thread.  `ic.pipeline` decomposes on its
//! calling thread, and the elimination runs there.
//!
//! ## Determinism
//!
//! The solver reads several tuning variables from the environment
//! (`SOLVER_SPLIT_RULE`, `KIC_*`, `F4_*`).  An override would change the
//! search without changing the method's identity, so `prepare` refuses to
//! run while any of them is set, as #1333's exclusive worker does.
//!
//! ## Why it lives in the binary
//!
//! The frozen-source replay workflows rebuild the library against pre-F6
//! snapshots of `koblitz_index_calculus.rs`
//! (`research/notes/ecc2k130/compact_frozen_source_replay_20260929`), which
//! lack the function this module calls. Other frozen evaluations pin
//! `Cargo.toml` by hash, and the tournament admits no root build script, so
//! neither a feature nor a `cfg` can gate it. The module therefore lives in
//! the `ecbench` binary, which those replays never build, and reaches
//! `ic.pipeline` through `methods::register_binary_plugins`.

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use num_bigint::BigUint;
use num_traits::One;

use crypto_lib::binary_ecc::{BinaryCurve, F2mElement};

use crypto_lib::cryptanalysis::bitop_tally;
use crypto_lib::cryptanalysis::ecbench::methods::BinaryPlugins;
use crypto_lib::cryptanalysis::ic_boundary::{
    decompose_mitm, koblitz_factor_base, lift_abscissae, BinaryGroup, BinaryInstance, ColumnFold,
    CountedGroup, FactorBase, GroupOps, OracleCounters, PairTable,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, SolverCost, SolverTotals,
    SystemShape,
};
use crypto_lib::cryptanalysis::koblitz_fast::{BatchScratch, FastPoint};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    f4_word_ops_thread, parse_linear_harvest, set_linear_harvest, FieldStructure, LinearHarvest,
    SolveStats, SolverEngine,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, groebner_decompose, groebner_decompose_f6_ic,
    groebner_decompose_f6_ic_pair_index, point_key, reset_f6_ic_pair_index_cache,
    FrobeniusFactorBase, KoblitzCurve, PairSumTable,
};

/// The unit string of the solver's count.  It matches the symmetrised and
/// inherited-F4 adapters, so the runner treats all three alike: counted,
/// reported, and left out of `S`.
pub const WORD_XOR_UNIT: &str =
    "word XORs (elimination, specialisation and linear elimination only)";

/// Environment prefixes the Groebner code reads for tuning overrides.
const TUNING_PREFIXES: [&str; 3] = ["KIC_", "F4_", "SOLVER_"];

/// A generic binary curve `y² + xy = x³ + ax² + b` (`a ∈ {0, 1}`, `b`
/// anywhere in `F_{2^n}`) seen through the [`KoblitzCurve`] type the
/// decomposers take, with the 2-power Frobenius switched off: the same
/// shape [`KoblitzCurve::isogenous_model`] gives a floor curve
/// (`frobenius_is_endomorphism = false`, `λ = 1`, `k = 1`), but with the
/// curve's own counted order, subgroup and generator rather than a Koblitz
/// class's.  Nothing the decomposition reads depends on Frobenius: the
/// Semaev system takes only `b`, the base is folded by negation alone, and
/// F6-IC's gate needs the subspace, base membership and group addition.
fn generic_view(inst: &BinaryInstance) -> Result<KoblitzCurve, String> {
    if inst.a > 1 {
        return Err(format!(
            "the generic view takes a ∈ {{0, 1}}, this curve has a = {:#x}",
            inst.a
        ));
    }
    let n = inst.n;
    let curve = BinaryCurve {
        m: n,
        irreducible: inst.irreducible.clone(),
        a: inst.gf.to_element(inst.a),
        b: inst.gf.to_element(inst.b),
        generator: inst.fast.lower(inst.generator),
        order: BigUint::from(inst.r),
        cofactor: BigUint::from(inst.cofactor),
    };
    let trace = (1i128 << n) + 1 - i128::from(inst.group_order);
    Ok(KoblitzCurve {
        a: inst.a as u8,
        n,
        curve,
        trace: trace as i64,
        group_order: BigUint::from(inst.group_order),
        subgroup_order: BigUint::from(inst.r),
        cofactor: BigUint::from(inst.cofactor),
        lambda: BigUint::one(),
        frobenius_is_endomorphism: false,
        k: 1,
        q: 2,
        a_index: inst.a,
        b_index: 0,
        subfield_basis: vec![F2mElement::one(n)],
    })
}

/// The instance the decomposers read: the run's own when it is a Koblitz
/// curve, otherwise a copy that carries [`generic_view`].  The copy differs
/// from the run's instance only in that field; the run's group, points and
/// identity are untouched.
enum View<'i> {
    Koblitz(&'i BinaryInstance),
    Generic(Box<BinaryInstance>),
}

impl<'i> View<'i> {
    fn of(inst: &'i BinaryInstance) -> Result<Self, String> {
        if inst.koblitz.is_some() {
            return Ok(Self::Koblitz(inst));
        }
        Ok(Self::Generic(Box::new(BinaryInstance {
            name: inst.name.clone(),
            n: inst.n,
            irreducible: inst.irreducible.clone(),
            fast: inst.fast.clone(),
            gf: inst.gf.clone(),
            artin_schreier: inst.artin_schreier.clone(),
            a: inst.a,
            b: inst.b,
            group_order: inst.group_order,
            r: inst.r,
            cofactor: inst.cofactor,
            generator: inst.generator,
            koblitz: Some(generic_view(inst)?),
        })))
    }

    fn instance(&self) -> &BinaryInstance {
        match self {
            Self::Koblitz(i) => i,
            Self::Generic(i) => i,
        }
    }

    fn curve(&self) -> &KoblitzCurve {
        self.instance()
            .koblitz
            .as_ref()
            .expect("a view always carries a curve")
    }
}

fn standard_subspace(
    inst: &BinaryInstance,
    params: &Params,
) -> Result<FrobeniusFactorBase, String> {
    let kc = inst
        .koblitz
        .as_ref()
        .ok_or("this instance has no curve view")?;
    let d = params
        .get("dimension")
        .ok_or("missing parameter `dimension`; #1333 used 4 at n9 and 6 at n17")?;
    let d: u32 = d
        .parse()
        .map_err(|_| format!("dimension `{d}` is not a number"))?;
    build_standard_subspace_factor_base(kc, d)
}

/// `binary-standard-subspace:dimension=d`, also registered under its first
/// name, `koblitz-standard-subspace`.  On a Koblitz curve the two are the
/// same base; on any other binary curve it is built through
/// [`generic_view`].
pub struct KoblitzStandardSubspaceBase<'i> {
    name: &'static str,
    view: Result<View<'i>, String>,
}

impl<'i> KoblitzStandardSubspaceBase<'i> {
    pub fn new(name: &'static str, instance: &'i BinaryInstance) -> Self {
        Self {
            name,
            view: View::of(instance),
        }
    }
}

impl<'a> FactorBaseBuilder<BinaryGroup<'a>> for KoblitzStandardSubspaceBase<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "every point with abscissa in span(1, z, …, z^{{d−1}}), d = {}, folded by negation \
             (the base of #1333's F4/F5/F6 workers)",
            params.get("dimension").unwrap_or("?")
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[(
            "dimension",
            "d, the subspace dimension: 1..=20 and below the field degree",
        )]
    }

    fn build(
        &self,
        _ctx: &InstanceCtx<BinaryGroup<'a>>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<FactorBase<FastPoint>, String> {
        let view = self.view.as_ref().map_err(Clone::clone)?;
        let frob = standard_subspace(view.instance(), params)?;
        let mut fb = koblitz_factor_base(
            view.instance(),
            &frob,
            ColumnFold::SignedFrobeniusOrbit,
            format!(
                "standard subspace, dimension {}, {} points",
                frob.ell,
                frob.points.len()
            ),
        )
        .ok_or("the standard subspace produced no usable factor base")?;
        fb.dimension = Some(frob.ell);
        fb.subspace_basis = Some((0..frob.ell).map(|i| 1u64 << i).collect());
        Ok(fb)
    }
}

/// Which decomposer, and which reduction engine, the oracle calls.
///
/// Each engine runs under its own default splitting rule
/// (`SplitRule::Auto`): the inherited engines split on the highest free
/// variable, the matrix engines on the lowest.  So an engine arm is the
/// whole solver choice, as in #1333's three-way comparison, not one
/// reduction kernel on a shared tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdp3Engine {
    /// `groebner_decompose` with inherited F4: the control.
    InheritedF4,
    /// `groebner_decompose_f6_ic`: inherited F4 with the geometric gate.
    F6Ic,
    /// `groebner_decompose_f6_ic_pair_index`: F6-IC whose one-fixed-summand
    /// closure answers from a pair-sum index of the base (built once per
    /// run, its |F|(|F|+1)/2 additions charged) with one charged addition
    /// per lift, instead of scanning the base at every closed node.
    F6IcPair,
    /// `groebner_decompose` with Boolean matrix-F4, every node from scratch.
    MatrixF4,
    /// `groebner_decompose` with Boolean matrix-F5: matrix-F4's matrices
    /// less the rows the F5 criterion predicts reduce to zero.
    MatrixF5,
    /// The root-finding engine of `DecompositionStrategy::PairTable`: a
    /// [`PairSumTable`] of the base's pair sums, probed once per target
    /// (`m = 2`) or once per `R − P_k` (`m = 3`).  No Groebner search, so
    /// no `degree` and no `node_budget`.
    Rootfind,
    /// As [`Self::Rootfind`], but the `m = 3` third-summand scan is the
    /// cyclic window of length `⌈|F|/3⌉` (see
    /// [`PairSumTable::decompose_fast_window`]).  Refuses `m ≠ 3`.
    RootfindWindow,
    /// Root finding on [`PairTable::build_negation_folded`]: about half
    /// the set-up additions of [`Self::Rootfind`], sign-recovering probe.
    RootfindNegfold,
    /// The same negation-folded table and probe, with batched binary
    /// additions during table construction.
    RootfindNegfoldBatch,
    /// The batch table plus verified homogeneous rows from duplicate keys.
    RootfindNegfoldBatchHarvest,
    /// Harvest non-torsion collisions and seed the order-two row directly.
    RootfindNegfoldBatchHarvestTorsion,
    /// Windowed `m = 3` scan on the negation-folded [`PairTable`].
    RootfindWindowNegfold,
}

impl Pdp3Engine {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "inherited-f4" => Ok(Self::InheritedF4),
            "f6-ic" => Ok(Self::F6Ic),
            "f6-ic-pair" => Ok(Self::F6IcPair),
            "matrix-f4" => Ok(Self::MatrixF4),
            "matrix-f5" => Ok(Self::MatrixF5),
            "rootfind" => Ok(Self::Rootfind),
            "rootfind-window" => Ok(Self::RootfindWindow),
            "rootfind-negfold" => Ok(Self::RootfindNegfold),
            "rootfind-negfold-batch" => Ok(Self::RootfindNegfoldBatch),
            "rootfind-negfold-batch-harvest" => Ok(Self::RootfindNegfoldBatchHarvest),
            "rootfind-negfold-batch-harvest-torsion" => {
                Ok(Self::RootfindNegfoldBatchHarvestTorsion)
            }
            "rootfind-window-negfold" => Ok(Self::RootfindWindowNegfold),
            other => Err(format!(
                "unknown engine `{other}`; try inherited-f4, f6-ic, f6-ic-pair, matrix-f4, \
                 matrix-f5, \
                 rootfind, rootfind-window, rootfind-negfold, rootfind-negfold-batch, \
                 rootfind-negfold-batch-harvest, rootfind-negfold-batch-harvest-torsion \
                 or rootfind-window-negfold"
            )),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::InheritedF4 => "inherited-f4",
            Self::F6Ic => "f6-ic",
            Self::F6IcPair => "f6-ic-pair",
            Self::MatrixF4 => "matrix-f4",
            Self::MatrixF5 => "matrix-f5",
            Self::Rootfind => "rootfind",
            Self::RootfindWindow => "rootfind-window",
            Self::RootfindNegfold => "rootfind-negfold",
            Self::RootfindNegfoldBatch => "rootfind-negfold-batch",
            Self::RootfindNegfoldBatchHarvest => "rootfind-negfold-batch-harvest",
            Self::RootfindNegfoldBatchHarvestTorsion => "rootfind-negfold-batch-harvest-torsion",
            Self::RootfindWindowNegfold => "rootfind-window-negfold",
        }
    }

    /// Whether the engine solves an algebraic system at all.
    pub fn is_groebner(self) -> bool {
        !self.is_rootfind()
    }

    /// Whether the engine builds and probes a pair-sum table (full or
    /// negation-folded).
    pub fn is_rootfind(self) -> bool {
        matches!(
            self,
            Self::Rootfind
                | Self::RootfindWindow
                | Self::RootfindNegfold
                | Self::RootfindNegfoldBatch
                | Self::RootfindNegfoldBatchHarvest
                | Self::RootfindNegfoldBatchHarvestTorsion
                | Self::RootfindWindowNegfold
        )
    }

    /// Whether set-up uses [`PairTable::build_negation_folded`].
    pub fn is_negfold(self) -> bool {
        matches!(
            self,
            Self::RootfindNegfold
                | Self::RootfindNegfoldBatch
                | Self::RootfindNegfoldBatchHarvest
                | Self::RootfindNegfoldBatchHarvestTorsion
                | Self::RootfindWindowNegfold
        )
    }

    /// Whether the `m = 3` third-summand scan is windowed.
    pub fn is_window(self) -> bool {
        matches!(self, Self::RootfindWindow | Self::RootfindWindowNegfold)
    }

    /// The reduction engine at each node of the splitting tree, or
    /// `None` for a root-finding engine, which has no tree.
    fn solver(self, max_degree: u32) -> Option<SolverEngine> {
        match self {
            Self::InheritedF4 | Self::F6Ic | Self::F6IcPair => {
                Some(SolverEngine::InheritedF4 { max_degree })
            }
            Self::MatrixF4 => Some(SolverEngine::MatrixF4 { max_degree }),
            Self::MatrixF5 => Some(SolverEngine::MatrixF5 { max_degree }),
            Self::Rootfind
            | Self::RootfindWindow
            | Self::RootfindNegfold
            | Self::RootfindNegfoldBatch
            | Self::RootfindNegfoldBatchHarvest
            | Self::RootfindNegfoldBatchHarvestTorsion
            | Self::RootfindWindowNegfold => None,
        }
    }
}

/// Third-summand window length for `engine=rootfind-window`: `⌈n/3⌉`,
/// at least one when the base is non-empty.  Matches the yield/cost
/// trade recorded on [`PairSumTable::decompose_fast_window`].
pub fn rootfind_window_len(points: usize) -> usize {
    rootfind_window_len_div(points, 3)
}

/// Resolve a batch table's x-only collisions into checked group equations.
/// The table itself keeps the same first witness and query behavior.
fn verified_collision_rows(
    g: &BinaryGroup<'_>,
    fb: &FactorBase<FastPoint>,
    r: u64,
    collisions: &[[usize; 4]],
    ops: &mut GroupOps,
) -> Result<(Vec<Vec<u64>>, u64), String> {
    let mut rows = Vec::with_capacity(collisions.len());
    let mut zero_rows = 0u64;
    for &[i, j, fi, fj] in collisions {
        if [i, j, fi, fj].iter().any(|&index| index >= fb.points.len()) {
            return Err("pair-table collision has an out-of-base witness".into());
        }
        let current = g.add(ops, fb.points[i], fb.points[j]);
        let first = g.add(ops, fb.points[fi], fb.points[fj]);
        if g.is_identity(&current) || g.is_identity(&first) || current.x != first.x {
            return Err("pair-table collision has unequal or infinite sums".into());
        }
        let (indices, check_rhs) = if current == first {
            let nfi = *fb
                .neg_index
                .get(fi)
                .ok_or("pair-table collision has a missing negative index")?;
            let nfj = *fb
                .neg_index
                .get(fj)
                .ok_or("pair-table collision has a missing negative index")?;
            if fb.points.get(nfi) != Some(&g.neg(fb.points[fi]))
                || fb.points.get(nfj) != Some(&g.neg(fb.points[fj]))
            {
                return Err("pair-table collision has an incorrect negative index".into());
            }
            ([i, j, nfi, nfj], g.neg(first))
        } else if current == g.neg(first) {
            ([i, j, fi, fj], first)
        } else {
            return Err("pair-table collision has unresolved sum sign".into());
        };
        // Both pair sums were independently recomputed above. Reuse them for
        // the complete four-point identity rather than re-adding the same
        // four base points from scratch.
        let check = g.add(ops, current, check_rhs);
        if !g.is_identity(&check) {
            return Err("pair-table collision failed full group verification".into());
        }
        let mut row = vec![0u64; fb.columns];
        for &index in &indices {
            let col = fb.col_of[index];
            row[col] = (row[col] + fb.coef_of[index]) % r;
        }
        if row.iter().all(|&coefficient| coefficient == 0) {
            zero_rows += 1;
        } else {
            rows.push(row);
        }
    }
    Ok((rows, zero_rows))
}

/// Window length `⌈n/div⌉` for a configurable divisor (`div ≥ 1`).
pub fn rootfind_window_len_div(points: usize, div: usize) -> usize {
    if points == 0 || div == 0 {
        0
    } else {
        points.div_ceil(div).max(1)
    }
}

/// The group operations [`PairSumTable::build_full_within`] performs for
/// a base of `points` signed points: every `P_i + P_j` with `i ≤ j`, the
/// `i = j` terms being doublings.  Counted from the base's size because
/// the batched builder (one Montgomery inversion per row) carries no
/// ledger, the way `koblitz_factor_base` counts its constructor.
pub fn pair_table_build_ops(points: usize) -> (u64, u64) {
    let n = points as u64;
    (n * n.saturating_sub(1) / 2, n)
}

/// The per-call counters `extra` carries, by name.  `*_max` keys are
/// maxima over calls; the rest are totals (`SolverTotals::absorb`).
fn stats_extra(s: &SolveStats) -> BTreeMap<String, u64> {
    let mut e = BTreeMap::new();
    let mut put = |k: &str, v: u64| {
        e.insert(k.to_string(), v);
    };
    put("reductions", s.reductions as u64);
    put("splits", s.splits as u64);
    put("propagations", s.propagations as u64);
    put("infeasible_branches", s.infeasible_branches as u64);
    put("eliminated", s.eliminated as u64);
    put("oversize", s.oversize as u64);
    put("exhausted", u64::from(s.exhausted));
    put("unsupported", u64::from(s.unsupported));
    put("max_degree_built_max", u64::from(s.max_degree_built));
    put("geometric_refutations", s.geometric_refutations as u64);
    put("geometric_witnesses", s.geometric_witnesses as u64);
    put("geometric_support_checks", s.geometric_support_checks);
    put("geometric_residual_lookups", s.geometric_residual_lookups);
    put(
        "geometric_fast_residual_lookups",
        s.geometric_fast_residual_lookups,
    );
    put("geometric_batch_groups", s.geometric_batch_groups);
    put("geometric_group_additions", s.geometric_group_additions);
    put("geometric_fallbacks", s.geometric_fallbacks as u64);
    e
}

/// `pdp3-koblitz:m=3,engine=inherited-f4|f6-ic|f6-ic-pair,degree=D,node_budget=N[,linear_harvest=H]`,
/// `pdp3-koblitz:m=2|3,engine=rootfind`,
/// `pdp3-koblitz:m=3,engine=rootfind-window`,
/// `pdp3-koblitz:m=2|3,engine=rootfind-negfold`, or
/// `pdp3-koblitz:m=2|3,engine=rootfind-negfold-batch`, or
/// `pdp3-koblitz:m=2|3,engine=rootfind-negfold-batch-harvest`,
/// `pdp3-koblitz:m=2|3,engine=rootfind-negfold-batch-harvest-torsion`, or
/// `pdp3-koblitz:m=3,engine=rootfind-window-negfold`.
pub struct Pdp3KoblitzOracle<'i> {
    name: &'static str,
    summands: u32,
    view: Result<View<'i>, String>,
    engine: Pdp3Engine,
    solver: SolverEngine,
    node_budget: usize,
    /// The root linear-row harvest of `koblitz_groebner` (PR #1862) for
    /// this arm: `None` leaves the library's process default in force (the
    /// parameter was not given, so the method's identity and counters are
    /// those of every earlier session); `Some(None)` switches it off;
    /// `Some(Some(h))` forces configuration `h`.
    linear_harvest: Option<Option<LinearHarvest>>,
    frob: Option<FrobeniusFactorBase>,
    index_of: HashMap<(BigUint, BigUint), usize>,
    st: Option<FieldStructure>,
    shape: Option<SystemShape>,
    totals: SolverTotals,
    /// Full [`PairSumTable`] under `rootfind` / `rootfind-window`.
    pair: Option<PairSumTable>,
    /// Negation-folded [`PairTable`] under `*-negfold` engines.
    pair_neg: Option<PairTable>,
    /// Verified homogeneous rows harvested from duplicate batch-table keys.
    setup_rows: Vec<Vec<u64>>,
    collision_candidates: u64,
    collision_verified: u64,
    collision_zero_rows: u64,
    collision_columns: usize,
    torsion_skipped: u64,
    torsion_inversion_lanes: u64,
    torsion_inversion_batches: u64,
    torsion_seed_rows: u64,
    /// Opt-in direct lookup after the torsion table's first witnesses exist.
    dense_probe: bool,
    /// Reused by the batched `m = 3` scan; all completed additions are charged.
    scan_scratch: BatchScratch,
    scan_addends: Vec<FastPoint>,
    scan_sums: Vec<FastPoint>,
    scan_lambdas: Vec<u64>,
    #[cfg(target_arch = "x86_64")]
    scan_x3: Vec<u64>,
    /// The signed base starts with one self-negative point, then adjacent
    /// pairs. Checked once so every later relation scan can align its blocks.
    scan_pair_layout: Option<bool>,
    /// Base size the table was built on; `0` under a Groebner engine.
    pair_table_points: usize,
    /// Third-summand window for windowed engines; `0` otherwise.
    scan_window_len: usize,
    /// Divisor for `⌈|F|/div⌉` under `rootfind-window-negfold`; `3` for
    /// the fixed `rootfind-window` engine; `0` otherwise.
    window_div: usize,
    /// Witnesses the solver returned that did not re-lift through the
    /// framework's sign resolution.  Must stay zero.
    pub unliftable: u64,
    /// Pair-table hits whose summands did not re-add to the probed point
    /// in the group.  Must stay zero; a nonzero count would mean the
    /// table and the run's base disagree on a point.
    pub table_mismatches: u64,
}

impl<'i> Pdp3KoblitzOracle<'i> {
    pub fn new(name: &'static str, summands: u32, instance: &'i BinaryInstance) -> Self {
        Self {
            name,
            summands,
            view: View::of(instance),
            engine: Pdp3Engine::InheritedF4,
            solver: SolverEngine::InheritedF4 { max_degree: 3 },
            node_budget: 0,
            linear_harvest: None,
            frob: None,
            index_of: HashMap::new(),
            st: None,
            shape: None,
            totals: SolverTotals::default(),
            pair: None,
            pair_neg: None,
            setup_rows: Vec::new(),
            collision_candidates: 0,
            collision_verified: 0,
            collision_zero_rows: 0,
            collision_columns: 0,
            torsion_skipped: 0,
            torsion_inversion_lanes: 0,
            torsion_inversion_batches: 0,
            torsion_seed_rows: 0,
            dense_probe: false,
            scan_scratch: BatchScratch::default(),
            scan_addends: Vec::new(),
            scan_sums: Vec::new(),
            scan_lambdas: Vec::new(),
            #[cfg(target_arch = "x86_64")]
            scan_x3: Vec::new(),
            scan_pair_layout: None,
            pair_table_points: 0,
            scan_window_len: 0,
            window_div: 0,
            unliftable: 0,
            table_mismatches: 0,
        }
    }

    /// Which engine `prepare` selected.
    #[allow(dead_code)] // used by binary-side unit checks and future adapters
    pub fn engine(&self) -> Pdp3Engine {
        self.engine
    }

    /// The root-finding decomposition: one table probe for `m = 2`, one
    /// per `R − P_k` for `m = 3` (full base, or the cyclic window under
    /// a windowed engine), every hit re-added in the group on the ledger
    /// before it is returned.
    fn decompose_rootfind<'a>(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: FastPoint,
    ) -> Option<Vec<usize>> {
        if self.engine.is_negfold() {
            return self.decompose_rootfind_negfold(ctx, fb, ops, counters, point);
        }
        let table = self.pair.as_ref()?;
        let g = ctx.group;
        let mut pairs: Vec<(u32, u32)> = Vec::new();
        match self.summands {
            2 => {
                counters.lookups += 1;
                table.pairs_for(point, &mut pairs);
                for &(i, j) in &pairs {
                    let (i, j) = (i as usize, j as usize);
                    if g.add(ops, fb.points[i], fb.points[j]) == point {
                        return Some(vec![i, j]);
                    }
                    self.table_mismatches += 1;
                    counters.frobfold_mismatches += 1;
                }
                None
            }
            3 => {
                let n = fb.points.len();
                if n == 0 {
                    return None;
                }
                let (start, len) = if self.engine.is_window() {
                    let len = self.scan_window_len.max(1).min(n);
                    // Deterministic in the target: the same public
                    // point always opens the window at the same index.
                    ((point.x as usize) % n, len)
                } else {
                    (0, n)
                };
                for t in 0..len {
                    let k = (start + t) % n;
                    let rest = g.add(ops, point, g.neg(fb.points[k]));
                    if g.is_identity(&rest) {
                        continue;
                    }
                    counters.lookups += 1;
                    table.pairs_for(rest, &mut pairs);
                    for &(i, j) in &pairs {
                        let (i, j) = (i as usize, j as usize);
                        if g.add(ops, fb.points[i], fb.points[j]) == rest {
                            let mut found = vec![i, j, k];
                            found.sort_unstable();
                            return Some(found);
                        }
                        self.table_mismatches += 1;
                        counters.frobfold_mismatches += 1;
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Negation-folded [`PairTable`] path: same probe as `mitm` with
    /// `negation_folded=1`, optionally windowed for `m = 3`.
    fn decompose_rootfind_negfold<'a>(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: FastPoint,
    ) -> Option<Vec<usize>> {
        let table = self.pair_neg.as_ref()?;
        let g = ctx.group;
        if matches!(
            self.engine,
            Pdp3Engine::RootfindNegfoldBatch
                | Pdp3Engine::RootfindNegfoldBatchHarvest
                | Pdp3Engine::RootfindNegfoldBatchHarvestTorsion
        ) && self.summands == 3
        {
            const SCAN_BLOCK: usize = 8;
            let paired_layout = *self.scan_pair_layout.get_or_insert_with(|| {
                fb.points.len() % 2 == 1
                    && fb.neg_index.first() == Some(&0)
                    && fb.points[1..].chunks_exact(2).enumerate().all(|(j, pair)| {
                        let k = 1 + 2 * j;
                        fb.neg_index[k] == k + 1
                            && fb.neg_index[k + 1] == k
                            && !pair[0].infinity
                            && !pair[1].infinity
                            && pair[0].x == pair[1].x
                    })
            });
            let mut from = 0;
            while from < fb.points.len() {
                // The first point has no signed twin. Keeping it in its
                // own block makes every following eight-point block four
                // complete pairs, ready for the existing paired SIMD kernel.
                let to = if paired_layout && from == 0 {
                    1
                } else {
                    (from + SCAN_BLOCK).min(fb.points.len())
                };
                self.scan_addends.clear();
                self.scan_addends
                    .extend(fb.points[from..to].iter().map(|&p| g.neg(p)));
                self.scan_sums.clear();
                self.scan_lambdas.clear();
                #[cfg(target_arch = "x86_64")]
                let simd_paired = if paired_layout && from > 0 {
                    let pairs = self.scan_addends.len() / 2;
                    debug_assert_eq!(2 * pairs, self.scan_addends.len());
                    debug_assert!(pairs <= SCAN_BLOCK / 2);
                    let (mut px, mut pya, mut pyb) = ([0u64; 4], [0u64; 4], [0u64; 4]);
                    for (j, pair) in self.scan_addends.chunks_exact(2).enumerate() {
                        debug_assert_eq!(pair[0].x, pair[1].x);
                        px[j] = pair[0].x;
                        pya[j] = pair[0].y;
                        pyb[j] = pair[1].y;
                    }
                    self.scan_x3.clear();
                    g.0.add_many_lazy_soa_paired(
                        point,
                        &px[..pairs],
                        &pya[..pairs],
                        &pyb[..pairs],
                        &mut self.scan_x3,
                        &mut self.scan_lambdas,
                        &mut self.scan_scratch,
                        self.scan_addends.len() as u64,
                    )
                } else {
                    false
                };
                #[cfg(not(target_arch = "x86_64"))]
                let simd_paired = false;
                #[cfg(target_arch = "x86_64")]
                if simd_paired {
                    self.scan_sums.extend(
                        self.scan_x3
                            .iter()
                            .copied()
                            .map(|x| FastPoint::affine(x, 0)),
                    );
                }
                if !simd_paired {
                    g.0.add_many_lazy_paired_x(
                        point,
                        &self.scan_addends,
                        &mut self.scan_sums,
                        &mut self.scan_lambdas,
                        &mut self.scan_scratch,
                    );
                }
                assert_eq!(self.scan_sums.len(), self.scan_addends.len());
                assert_eq!(self.scan_lambdas.len(), self.scan_addends.len());
                // The batch has already formed all sums, even if an early
                // probe hits. The ledger therefore charges the entire block.
                ops.adds += self.scan_addends.len() as u64;
                for (offset, (&rest, &lambda)) in
                    self.scan_sums.iter().zip(&self.scan_lambdas).enumerate()
                {
                    if g.is_identity(&rest) {
                        continue;
                    }
                    if let Some((i, j)) = table.probe_binary_lazy(
                        g,
                        fb,
                        ops,
                        &mut counters.lookups,
                        point,
                        rest,
                        lambda,
                    ) {
                        return Some(vec![from + offset, i, j]);
                    }
                }
                from = to;
            }
            return None;
        }
        if !self.engine.is_window() {
            return decompose_mitm(g, fb, table, self.summands, ops, counters, point);
        }
        // Windowed m = 3 on the half table.
        let n = fb.points.len();
        if n == 0 || self.summands != 3 {
            return None;
        }
        let len = self.scan_window_len.max(1).min(n);
        let start = (point.x as usize) % n;
        for t in 0..len {
            let k = (start + t) % n;
            let rest = g.add(ops, point, g.neg(fb.points[k]));
            if g.is_identity(&rest) {
                continue;
            }
            if let Some((i, j)) = table.probe(g, fb, ops, &mut counters.lookups, rest) {
                let mut found = vec![i, j, k];
                found.sort_unstable();
                return Some(found);
            }
        }
        None
    }
}

impl<'a> DecompositionOracle<BinaryGroup<'a>> for Pdp3KoblitzOracle<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn summands(&self) -> u32 {
        self.summands
    }

    fn describe(&self, params: &Params) -> String {
        match params.get("engine") {
            Some("rootfind") => format!(
                "root finding over the standard subspace: the pair-sum table of \
                 DecompositionStrategy::PairTable, probed once per target (m = 2) or once \
                 per R − P_k (m = {}); no Groebner search",
                self.summands
            ),
            Some("rootfind-window") => format!(
                "root finding over the standard subspace with a cyclic third-summand window \
                 of length ⌈|F|/3⌉ (PairSumTable::decompose_fast_window); m = {}; no Groebner \
                 search",
                self.summands
            ),
            Some("rootfind-negfold") => format!(
                "root finding over the standard subspace on PairTable::build_negation_folded \
                 (~|F|²/4 set-up adds); probed once per target (m = 2) or once per R − P_k \
                 (m = {}); no Groebner search",
                self.summands
            ),
            Some("rootfind-negfold-batch") => format!(
                "root finding over the standard subspace on a negation-folded PairTable \
                 built with batched binary additions; the same probe as rootfind-negfold \
                 (m = {}); no Groebner search",
                self.summands
            ),
            Some("rootfind-negfold-batch-harvest") => format!(
                "rootfind-negfold-batch with checked homogeneous seed rows from duplicate \
                 pair sums (m = {}); no Groebner search",
                self.summands
            ),
            Some("rootfind-negfold-batch-harvest-torsion") => format!(
                "torsion-row harvest with {} pair-table lookup (m = {}); no Groebner search",
                if params.get("layout") == Some("dense") {
                    "direct-index"
                } else {
                    "hash"
                },
                self.summands
            ),
            Some("rootfind-window-negfold") => format!(
                "root finding with negation-folded PairTable and cyclic third-summand window \
                 of length ⌈|F|/window_div⌉ (div={}); m = {}; no Groebner search",
                params.get("window_div").unwrap_or("?"),
                self.summands
            ),
            _ => format!(
                "Weil-descend S_{} over the standard subspace and solve with #1333's {} \
                 (Macaulay cap {}, {} reduction calls)",
                self.summands + 1,
                params.get("engine").unwrap_or("?"),
                params.get("degree").unwrap_or("?"),
                params.get("node_budget").unwrap_or("?"),
            ),
        }
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            (
                "m",
                "summands; #1333 ran 3 (Semaev S4); rootfind, rootfind-negfold and \
                 rootfind-negfold-batch take 2 or 3; \
                 window engines take 3 only",
            ),
            (
                "engine",
                "inherited-f4 (the control), f6-ic, matrix-f4, matrix-f5, rootfind (full \
                 PairSumTable), rootfind-window (m = 3 cyclic ⌈|F|/3⌉ window), \
                 rootfind-negfold (PairTable negation fold, ~half set-up), \
                 rootfind-negfold-batch (same table with batched binary additions), \
                 rootfind-negfold-batch-harvest (batch table plus verified seed rows), or \
                 rootfind-window-negfold (window + negation fold)",
            ),
            (
                "degree",
                "highest Macaulay degree built before splitting; refused under every \
                 rootfind* engine",
            ),
            (
                "node_budget",
                "reduction calls before a solve gives up; exhaustion is a failed query; \
                 refused under every rootfind* engine",
            ),
            (
                "linear_harvest",
                "optional, inherited-f4 and f6-ic only: off (the engine before PR #1862), \
                 default (the library default now in force), gpv (PR #1862's gate alone) or \
                 every (every node, no gate). Omitted: the library default, with the \
                 method's identity unchanged. Refused under every other engine",
            ),
            (
                "dimension",
                "the base's; copied from koblitz-standard-subspace when omitted",
            ),
            (
                "window_div",
                "under rootfind-window-negfold (default 3): window length ⌈|F|/div⌉; \
                 refused under other engines (rootfind-window stays fixed at div=3)",
            ),
            (
                "layout",
                "optional dense under rootfind-negfold-batch-harvest-torsion only: direct \
                 binary-key index for n<=23; omitted means the incumbent hash map",
            ),
        ]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        if let Some((k, _)) =
            std::env::vars().find(|(k, _)| TUNING_PREFIXES.iter().any(|p| k.starts_with(p)))
        {
            return Err(format!(
                "pdp3-koblitz refuses to run with the solver override `{k}` set: it would \
                 change the search without changing the method's identity"
            ));
        }
        if !(2..=3).contains(&self.summands) {
            return Err(format!(
                "pdp3-koblitz takes m = 2 or 3, got {}",
                self.summands
            ));
        }
        let view = self.view.as_ref().map_err(Clone::clone)?;
        let kc = view.curve();
        // Parameters that change the search have no default.
        let engine = params.get("engine").ok_or(
            "missing parameter `engine`; try inherited-f4, f6-ic, matrix-f4, matrix-f5, \
             rootfind, rootfind-window, rootfind-negfold, rootfind-negfold-batch, \
             rootfind-negfold-batch-harvest, rootfind-negfold-batch-harvest-torsion \
             or rootfind-window-negfold",
        )?;
        self.engine = Pdp3Engine::parse(engine)?;
        self.scan_pair_layout = None;
        self.setup_rows.clear();
        self.collision_candidates = 0;
        self.collision_verified = 0;
        self.collision_zero_rows = 0;
        self.collision_columns = 0;
        self.torsion_skipped = 0;
        self.torsion_inversion_lanes = 0;
        self.torsion_inversion_batches = 0;
        self.torsion_seed_rows = 0;
        self.dense_probe = match params.get("layout") {
            None => false,
            Some("dense") if self.engine == Pdp3Engine::RootfindNegfoldBatchHarvestTorsion => true,
            Some(_) if self.engine == Pdp3Engine::RootfindNegfoldBatchHarvestTorsion => {
                return Err("torsion harvest layout must be dense when specified".into());
            }
            Some(_) => {
                return Err("layout is only supported by torsion harvest".into());
            }
        };
        if self.engine == Pdp3Engine::F6IcPair {
            // Reuse within this pipeline solve, never across independent
            // records or replayed records in the auditor process.
            reset_f6_ic_pair_index_cache();
        }
        let mut max_degree = 0u32;
        if self.engine.is_groebner() {
            let degree = params
                .get("degree")
                .ok_or("missing parameter `degree`; #1333 ran 3")?;
            max_degree = degree
                .parse()
                .map_err(|_| format!("degree `{degree}` is not a number"))?;
            if !(2..=4).contains(&max_degree) {
                return Err(format!("degree must be 2..=4, got {max_degree}"));
            }
            self.solver = self
                .engine
                .solver(max_degree)
                .expect("every Groebner engine has a reduction engine");
            let budget = params
                .get("node_budget")
                .ok_or("missing parameter `node_budget`; #1333 ran 8192 cold")?;
            self.node_budget = budget
                .parse()
                .map_err(|_| format!("node_budget `{budget}` is not a number"))?;
            self.linear_harvest = match params.get("linear_harvest") {
                None => None,
                Some(raw) => {
                    if !matches!(
                        self.engine,
                        Pdp3Engine::InheritedF4 | Pdp3Engine::F6Ic | Pdp3Engine::F6IcPair
                    ) {
                        return Err(format!(
                            "engine={} does not take linear_harvest (only inherited-f4 and \
                             f6-ic harvest the root's linear rows)",
                            self.engine.name()
                        ));
                    }
                    Some(match raw {
                        "off" => None,
                        "default" => parse_linear_harvest("1"),
                        "gpv" => parse_linear_harvest("gpv"),
                        "every" => parse_linear_harvest("every"),
                        other => {
                            return Err(format!(
                                "linear_harvest `{other}` is not off, default, gpv or every"
                            ))
                        }
                    })
                }
            };
            self.scan_window_len = 0;
            self.window_div = 0;
            self.pair_table_points = 0;
            self.pair = None;
            self.pair_neg = None;
        } else {
            // A root-finding engine has no Groebner search to bound.  A
            // bound it ignored would be a parameter in the method's
            // identity that changes nothing, so it is refused.
            for key in ["degree", "node_budget", "linear_harvest"] {
                if params.get(key).is_some() {
                    return Err(format!(
                        "engine={} performs no Groebner search and takes no `{key}`",
                        self.engine.name()
                    ));
                }
            }
            if self.engine.is_window() && self.summands != 3 {
                return Err(format!(
                    "engine={} takes m = 3 (a third-summand window), got {}",
                    self.engine.name(),
                    self.summands
                ));
            }
            // window_div changes the search; optional (default 3) under
            // rootfind-window-negfold, refused elsewhere (the fixed
            // rootfind-window engine keeps div=3 with no parameter).
            match self.engine {
                Pdp3Engine::RootfindWindowNegfold => {
                    // Absent means the window `rootfind-window-negfold` had
                    // before the divisor became a parameter (`⌈|F|/3⌉`): the
                    // committed parity cycle-1 session names the engine without
                    // one and must keep replaying.  New specs should state it.
                    let div: usize = match params.get("window_div") {
                        None => 3,
                        Some(raw) => raw
                            .parse()
                            .map_err(|_| format!("window_div `{raw}` is not a number"))?,
                    };
                    if !(2..=8).contains(&div) {
                        return Err(format!("window_div must be 2..=8, got {div}"));
                    }
                    self.window_div = div;
                }
                _ => {
                    if params.get("window_div").is_some() {
                        return Err(format!(
                            "engine={} does not take window_div (only rootfind-window-negfold)",
                            self.engine.name()
                        ));
                    }
                    self.window_div = if self.engine == Pdp3Engine::RootfindWindow {
                        3
                    } else {
                        0
                    };
                }
            }
        }
        let dimension = fb
            .dimension
            .ok_or("pdp3-koblitz needs the koblitz-standard-subspace base")?;
        let mut base_params = params.clone();
        if base_params.get("dimension").is_none() {
            base_params.set("dimension", dimension.to_string());
        }
        let frob = standard_subspace(view.instance(), &base_params)?;
        // The base the runner built must be this one, point for point and
        // in the same order: the decomposer returns indices into `frob`,
        // and the relation is written against the runner's base.
        if frob.points.len() != fb.points.len() {
            return Err(format!(
                "pdp3-koblitz needs koblitz-standard-subspace with the same dimension: its \
                 base has {} points, the run's has {}",
                frob.points.len(),
                fb.points.len()
            ));
        }
        for (i, p) in frob.points.iter().enumerate() {
            if fb.index_of_key(view.instance().fast.lift(p).pack()) != Some(i) {
                return Err(
                    "pdp3-koblitz needs koblitz-standard-subspace with the same dimension: \
                     the bases differ"
                        .into(),
                );
            }
        }
        if (self.summands as u64) * u64::from(frob.ell) > 64 {
            return Err(format!(
                "the descent has m·d = {}·{} > 64 Boolean unknowns",
                self.summands, frob.ell
            ));
        }
        if self.engine.is_rootfind() {
            self.pair_table_points = frob.points.len();
            self.scan_window_len = match (self.engine.is_window(), self.window_div) {
                (true, 3) => rootfind_window_len(frob.points.len()),
                (true, div) if div > 0 => rootfind_window_len_div(frob.points.len(), div),
                _ => 0,
            };
            if self.engine.is_negfold() {
                // Ledgered half-table: same construction as mitm with
                // negation_folded=1.  Actual GroupOps from the build are
                // merged (not estimated from |F|).
                if fb.abscissa_list.is_empty() {
                    return Err(
                        "engine=rootfind-negfold needs a factor base with abscissa grouping".into(),
                    );
                }
                let table = match self.engine {
                    Pdp3Engine::RootfindNegfoldBatch => {
                        PairTable::build_negation_folded_binary_batch(ctx.group, fb)
                    }
                    Pdp3Engine::RootfindNegfoldBatchHarvest => {
                        let (table, collisions) =
                            PairTable::build_negation_folded_binary_batch_with_collisions(
                                ctx.group, fb,
                            );
                        self.collision_candidates = collisions.len() as u64;
                        self.collision_verified = collisions.len() as u64;
                        self.collision_columns = fb.columns;
                        let (rows, zero) =
                            verified_collision_rows(ctx.group, fb, ctx.r, &collisions, ops)?;
                        self.setup_rows = rows;
                        self.collision_zero_rows = zero;
                        table
                    }
                    Pdp3Engine::RootfindNegfoldBatchHarvestTorsion => {
                        if ctx.r.is_multiple_of(2) || !ctx.cofactor.is_multiple_of(2) {
                            return Err(
                                "torsion harvest requires odd subgroup order and even cofactor"
                                    .into(),
                            );
                        }
                        let (table, collisions, stats) =
                            PairTable::build_negation_folded_binary_batch_torsion_with_collisions(
                                ctx.group, fb,
                            )?;
                        let table = if self.dense_probe {
                            table.into_binary_dense(ctx.group.0.n)?
                        } else {
                            table
                        };
                        self.collision_candidates =
                            collisions.len() as u64 + stats.skipped_twin_collisions;
                        self.collision_verified = collisions.len() as u64;
                        self.collision_columns = fb.columns;
                        self.torsion_skipped = stats.skipped_twin_collisions;
                        self.torsion_inversion_lanes = stats.inversion_lanes;
                        self.torsion_inversion_batches = stats.inversion_batches;
                        let (mut rows, zero) =
                            verified_collision_rows(ctx.group, fb, ctx.r, &collisions, ops)?;
                        let mut seed = vec![0; fb.columns];
                        seed[0] = 1;
                        rows.push(seed);
                        self.torsion_seed_rows = 1;
                        self.setup_rows = rows;
                        self.collision_zero_rows = zero;
                        table
                    }
                    _ => PairTable::build_negation_folded(ctx.group, fb),
                };
                ops.merge(table.build_ops);
                self.pair_neg = Some(table);
                self.pair = None;
            } else {
                // The table of every `P_i + P_j`, in the summand-storing tier
                // so a hit is read, never recovered by a scan.  Its additions
                // are charged to the oracle set-up phase from the base's size.
                let table = PairSumTable::build_full_within(
                    kc,
                    &frob,
                    PairSumTable::DEFAULT_BYTE_BUDGET,
                )
                .ok_or_else(|| {
                    format!(
                        "the pair-sum table of {} points does not fit the {} GiB budget or the field \
                         is too wide to pack a point into a word",
                        frob.points.len(),
                        PairSumTable::DEFAULT_BYTE_BUDGET >> 30
                    )
                })?;
                // The table's own lift must agree with the run's, point for
                // point, or a probe key would name the wrong entry.  The
                // group re-addition of every hit would catch a disagreement
                // too; this catches it before any target is spent.
                for (i, p) in frob.points.iter().enumerate() {
                    if table.curve().lift(p) != fb.points[i] {
                        return Err(
                            "the pair-sum table lifts a base point differently from the run".into(),
                        );
                    }
                }
                let (adds, doubles) = pair_table_build_ops(frob.points.len());
                ops.adds += adds;
                ops.doubles += doubles;
                self.pair = Some(table);
                self.pair_neg = None;
            }
            self.frob = Some(frob);
            self.shape = None;
            self.st = None;
            self.index_of.clear();
            return Ok(());
        }
        self.index_of = frob
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| (point_key(p), i))
            .collect();
        self.st = Some(FieldStructure::new(kc.n, &kc.curve.irreducible));
        // Preserve the historical summand-only description for old IDs.
        // v2 includes the actual chain's intermediate field coordinates
        // and all coordinate-equation blocks. Solver calls are unchanged.
        let versioned_shape = self.name.ends_with("-v2");
        self.shape = Some(SystemShape {
            n_vars: (self.summands * frob.ell) as usize
                + if versioned_shape {
                    (self.summands as usize - 2) * kc.n as usize
                } else {
                    0
                },
            n_equations: if versioned_shape {
                (self.summands as usize - 1) * kc.n as usize
            } else {
                kc.n as usize
            },
            degrees: Vec::new(),
            semi_regular_degree: None,
        });
        self.totals = SolverTotals {
            solver: format!(
                "pdp3-koblitz {} (cap {}, budget {})",
                self.engine.name(),
                max_degree,
                self.node_budget
            ),
            ..Default::default()
        };
        self.frob = Some(frob);
        Ok(())
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: FastPoint,
    ) -> Option<Vec<usize>> {
        if self.engine.is_rootfind() {
            return self.decompose_rootfind(ctx, fb, ops, counters, point);
        }
        let (Some(frob), Some(st), Some(shape)) = (&self.frob, &self.st, &self.shape) else {
            return None;
        };
        let view = self.view.as_ref().ok()?;
        let kc = view.curve();
        let target = view.instance().fast.lower(point);
        let decompose = match self.engine {
            Pdp3Engine::F6Ic => groebner_decompose_f6_ic,
            Pdp3Engine::F6IcPair => groebner_decompose_f6_ic_pair_index,
            _ => groebner_decompose,
        };
        let before = f4_word_ops_thread();
        let tally_before = bitop_tally::snapshot();
        let started = Instant::now();
        // The harvest is a thread-local override, read on the solving
        // thread; set for this solve and always cleared after it.
        set_linear_harvest(self.linear_harvest);
        let (indices, stats) = decompose(
            kc,
            frob,
            &self.index_of,
            st,
            &target,
            self.summands as usize,
            self.solver,
            self.node_budget,
        );
        set_linear_harvest(None);
        let word_xors = f4_word_ops_thread().wrapping_sub(before);
        let wall_ns = started.elapsed().as_nanos() as u64;
        // The gate's point additions are group work: charge them.
        ops.adds += stats.geometric_group_additions;
        // The whole call in `bitop/v1` primitives: the engine's word XORs
        // (`ops`), everything else `bitop_tally` saw on this thread — the
        // system's instantiation from its template (and the template's
        // symbolic `F_{2^n}` build on the first call), the solver's
        // Macaulay builds, column sorts and indexes, packing, pivot
        // searches, read-back and splitting-tree scans, and the lift of
        // each root (`F_{2^n}` operations and base lookups) — and the
        // F6 gate's lookups, a hash probe each, and support checks, a
        // word operation a coordinate.  The gate's point additions are
        // group work, charged to `ops.adds` above, not here.
        let mut tally = bitop_tally::snapshot().since(tally_before);
        tally.hash_probes += stats.geometric_residual_lookups
            + stats.geometric_fast_residual_lookups
            + stats.geometric_pair_index_lookups;
        tally.word_ops += (stats.geometric_support_checks + stats.geometric_partial_support_checks)
            * u64::from(frob.ell);
        let mut extra = stats_extra(&stats);
        tally.put_complete(&mut extra, word_xors);
        let cost = SolverCost {
            ops: word_xors,
            op_unit: WORD_XOR_UNIT.into(),
            wall_ns,
            peak_bytes: 0,
            degree_reached: Some(stats.max_degree_built),
            solving_degree: None,
            first_fall_degree: None,
            last_fall_degree: None,
            degree_of_regularity_observed: None,
            degree_trace: Vec::new(),
            timed_out: stats.exhausted,
            extra,
        };
        self.totals.absorb(shape, Some(&cost), stats.exhausted);
        let indices = indices?;
        let xs: Vec<u64> = indices.iter().map(|&i| fb.points[i].x).collect();
        match lift_abscissae(ctx.group, fb, ops, &xs, point) {
            Some(found) => Some(found),
            None => {
                counters.lift_failures += 1;
                self.unliftable += 1;
                None
            }
        }
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        // The full pair-sum table inserts every unordered pair once; the
        // negation-folded PairTable counts its own attempts; the algebraic
        // engines build no table.
        let mut c = match (&self.pair, &self.pair_neg) {
            (Some(_), _) => {
                let f = self.pair_table_points as u64;
                BTreeMap::from([("table_insert_attempts".to_string(), f * (f + 1) / 2)])
            }
            (None, Some(t)) => t.work_counters(),
            (None, None) => BTreeMap::new(),
        };
        c.insert("counted:table_insert_attempts".to_string(), 1);
        if self.collision_verified > 0 {
            c.insert(
                "row_build_words".to_string(),
                self.collision_verified * (self.collision_columns as u64 + 4)
                    + self.torsion_seed_rows * self.collision_columns as u64,
            );
            c.insert(
                "row_build_modr_adds".to_string(),
                4 * self.collision_verified,
            );
        }
        if self.torsion_inversion_lanes > 0 {
            c.insert(
                "field_batch_inverse_lanes".into(),
                self.torsion_inversion_lanes,
            );
            c.insert(
                "field_batch_inverse_batches".into(),
                self.torsion_inversion_batches,
            );
        }
        c
    }

    fn setup_rows(&self) -> &[Vec<u64>] {
        &self.setup_rows
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        let mut counts = BTreeMap::new();
        if let Some(table) = &self.pair {
            counts.insert("pair_table_entries".into(), table.len() as u64);
            counts.insert("pair_table_kind".into(), 0); // full PairSumTable
        }
        if let Some(table) = &self.pair_neg {
            counts.insert("pair_table_entries".into(), table.entries);
            counts.insert("pair_table_kind".into(), 1); // negation_folded PairTable
            counts.insert(
                "pair_table_build_adds".into(),
                table.build_ops.adds + table.build_ops.doubles,
            );
            if let Some(bytes) = table.dense_bytes() {
                counts.insert("pair_table_dense_bytes".into(), bytes);
            }
        }
        if matches!(
            self.engine,
            Pdp3Engine::RootfindNegfoldBatchHarvest
                | Pdp3Engine::RootfindNegfoldBatchHarvestTorsion
        ) {
            counts.insert("collision_candidates".into(), self.collision_candidates);
            counts.insert(
                "collision_nonzero_rows".into(),
                self.setup_rows.len() as u64 - self.torsion_seed_rows,
            );
            counts.insert("collision_zero_rows".into(), self.collision_zero_rows);
        }
        if self.engine == Pdp3Engine::RootfindNegfoldBatchHarvestTorsion {
            counts.insert("torsion_skipped_collisions".into(), self.torsion_skipped);
            counts.insert("torsion_seed_rows".into(), self.torsion_seed_rows);
            counts.insert("torsion_inverse_lanes".into(), self.torsion_inversion_lanes);
            counts.insert(
                "torsion_inverse_batches".into(),
                self.torsion_inversion_batches,
            );
            counts.insert("collision_verified".into(), self.collision_verified);
        }
        if self.pair_table_points > 0 {
            counts.insert("pair_table_points".into(), self.pair_table_points as u64);
        }
        if self.scan_window_len > 0 {
            counts.insert("scan_window_len".into(), self.scan_window_len as u64);
        }
        if self.window_div > 0 {
            counts.insert("window_div".into(), self.window_div as u64);
        }
        counts
    }

    /// A table oracle solves no system: under a root-finding engine both
    /// are `None`, as for `mitm`, and the record carries no `solver` block.
    fn last_system(&self) -> Option<SystemShape> {
        if self.engine.is_rootfind() {
            return None;
        }
        self.shape.clone()
    }

    fn solver_totals(&self) -> Option<SolverTotals> {
        if self.engine.is_rootfind() {
            return None;
        }
        Some(self.totals.clone())
    }
}

/// The base's names: the first one it had, and the one that says it takes
/// any binary curve.  Both build the same base.
const BASE_NAMES: [&str; 2] = ["koblitz-standard-subspace", "binary-standard-subspace"];

/// The oracle's names, likewise.
const ORACLE_NAMES: [&str; 4] = [
    "pdp3-koblitz",
    "pdp3-binary",
    "pdp3-koblitz-v2",
    "pdp3-binary-v2",
];

/// This module's plug-ins, for `methods::register_binary_plugins`.
pub fn plugins() -> BinaryPlugins {
    BinaryPlugins {
        base: |name, instance| {
            let name = BASE_NAMES.iter().find(|n| **n == name)?;
            Some(Box::new(KoblitzStandardSubspaceBase::new(name, instance))
                as Box<dyn FactorBaseBuilder<BinaryGroup<'_>> + '_>)
        },
        oracle: |name, summands, instance| {
            let name = ORACLE_NAMES.iter().find(|n| **n == name)?;
            Some(Box::new(Pdp3KoblitzOracle::new(name, summands, instance))
                as Box<dyn DecompositionOracle<BinaryGroup<'_>> + '_>)
        },
    }
}

/// Committed sessions recorded before later changes altered what a
/// registered method counts under the same id (`ecbench::legacy`): they
/// must still replay bit for bit, each under the accounting that produced
/// it.  The oracle lives in this binary, so the replays do too.
#[cfg(test)]
mod engine_names {
    use super::{verified_collision_rows, Pdp3Engine, Pdp3KoblitzOracle};

    #[test]
    fn f6_ic_pair_parses_and_shares_the_f6_solver() {
        let engine = Pdp3Engine::parse("f6-ic-pair").unwrap();
        assert_eq!(engine, Pdp3Engine::F6IcPair);
        assert_eq!(engine.name(), "f6-ic-pair");
        assert!(engine.is_groebner());
        assert_eq!(engine.solver(3), Pdp3Engine::F6Ic.solver(3));
    }

    #[test]
    fn binary_batch_table_engine_keeps_the_rootfind_contract() {
        let engine = Pdp3Engine::parse("rootfind-negfold-batch").unwrap();
        assert_eq!(engine.name(), "rootfind-negfold-batch");
        assert!(engine.is_rootfind());
        assert!(engine.is_negfold());
        assert!(!engine.is_window());
        assert_eq!(engine.solver(3), None);
        let harvest = Pdp3Engine::parse("rootfind-negfold-batch-harvest").unwrap();
        assert!(harvest.is_rootfind() && harvest.is_negfold());
        assert_eq!(harvest.solver(3), None);
    }

    #[test]
    fn batch_collision_rows_match_fixed_census_and_reject_tampering() {
        use super::{BinaryGroup, ColumnFold, GroupOps, PairTable};
        use crypto_lib::cryptanalysis::ic_boundary::{
            koblitz_factor_base, koblitz_instance, CountedGroup, IncrementalGauss, RowStatus,
        };
        use crypto_lib::cryptanalysis::koblitz_index_calculus::build_standard_subspace_factor_base;

        let mut checked_negative_index = false;
        for (n, d, expected_candidates, expected_skipped, expected_rank) in [
            (13, 5, 27, 16, 5),
            (19, 7, 99, 69, 11),
            (23, 8, 201, 137, 23),
        ] {
            let inst = koblitz_instance(0, n).unwrap();
            let frob =
                build_standard_subspace_factor_base(inst.koblitz.as_ref().unwrap(), d).unwrap();
            let fb = koblitz_factor_base(
                &inst,
                &frob,
                ColumnFold::SignedFrobeniusOrbit,
                format!("d={d}"),
            )
            .unwrap();
            let group = BinaryGroup(&inst.fast);
            let plain = PairTable::build_negation_folded_binary_batch(&group, &fb);
            let (harvested, collisions) =
                PairTable::build_negation_folded_binary_batch_with_collisions(&group, &fb);
            assert_eq!(harvested.entries, plain.entries);
            assert_eq!(harvested.insert_attempts, plain.insert_attempts);
            assert_eq!(harvested.build_ops.adds, plain.build_ops.adds);
            assert_eq!(collisions.len(), expected_candidates);
            let mut ops = GroupOps::default();
            let (rows, zero) =
                verified_collision_rows(&group, &fb, inst.r, &collisions, &mut ops).unwrap();
            assert_eq!(ops.adds, 3 * collisions.len() as u64);
            assert_eq!(zero, 0);
            assert_eq!(rows.len(), expected_candidates);
            assert_eq!(ops.adds, 3 * expected_candidates as u64);
            let mut matrix = IncrementalGauss::new(fb.columns + 1, inst.r);
            for mut row in rows {
                row.push(0);
                assert_ne!(matrix.add_row(row, 0), RowStatus::Inconsistent);
            }
            assert_eq!(matrix.rank(), expected_rank);

            let (torsion, retained, stats) =
                PairTable::build_negation_folded_binary_batch_torsion_with_collisions(&group, &fb)
                    .unwrap();
            assert_eq!(torsion.entries, harvested.entries);
            assert_eq!(stats.skipped_twin_collisions, expected_skipped);
            assert_eq!(
                retained.len() + expected_skipped as usize,
                expected_candidates
            );
            let old_retained: Vec<_> = collisions
                .iter()
                .copied()
                .filter(|collision| {
                    if collision[0] != 0 {
                        return true;
                    }
                    assert_eq!(collision[2], 0);
                    assert_eq!(fb.neg_index[collision[1]], collision[3]);
                    assert_eq!(group.neg(fb.points[collision[1]]), fb.points[collision[3]]);
                    false
                })
                .collect();
            assert_eq!(retained, old_retained);
            let (retained_rows, retained_zero) =
                verified_collision_rows(&group, &fb, inst.r, &retained, &mut GroupOps::default())
                    .unwrap();
            assert_eq!(retained_zero, 0);
            let mut seeded = IncrementalGauss::new(fb.columns + 1, inst.r);
            let mut seed = vec![0; fb.columns + 1];
            seed[0] = 1;
            assert_ne!(seeded.add_row(seed, 0), RowStatus::Inconsistent);
            for mut row in retained_rows {
                row.push(0);
                assert_ne!(seeded.add_row(row, 0), RowStatus::Inconsistent);
            }
            assert_eq!(seeded.rank(), expected_rank);

            let mut corrupt = collisions[0];
            let mut rejected = false;
            for replacement in 0..fb.points.len() {
                corrupt[0] = replacement;
                if verified_collision_rows(&group, &fb, inst.r, &[corrupt], &mut ops).is_err() {
                    rejected = true;
                    break;
                }
            }
            assert!(rejected, "a modified collision witness must be rejected");
            for &collision in &collisions {
                let mut check_ops = GroupOps::default();
                let current = group.add(
                    &mut check_ops,
                    fb.points[collision[0]],
                    fb.points[collision[1]],
                );
                let first = group.add(
                    &mut check_ops,
                    fb.points[collision[2]],
                    fb.points[collision[3]],
                );
                if current != first {
                    continue;
                }
                let mut corrupt_fb = fb.clone();
                let fi = collision[2];
                corrupt_fb.neg_index[fi] = (0..fb.points.len())
                    .find(|&index| fb.points[index] != group.neg(fb.points[fi]))
                    .unwrap();
                let error = verified_collision_rows(
                    &group,
                    &corrupt_fb,
                    inst.r,
                    &[collision],
                    &mut GroupOps::default(),
                )
                .unwrap_err();
                assert!(error.contains("incorrect negative index"));
                checked_negative_index = true;
                break;
            }
        }
        assert!(
            checked_negative_index,
            "an equal-sum collision must test the negative index"
        );
    }

    #[test]
    fn harvested_rows_seed_a_same_target_k13_pipeline() {
        use super::{BinaryGroup, GroupOps, KoblitzStandardSubspaceBase};
        use crypto_lib::cryptanalysis::ic_boundary::{koblitz_instance, Calibration, CountedGroup};
        use crypto_lib::cryptanalysis::ic_framework::stages::{InstanceCtx, Targets};
        use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec};

        let inst = koblitz_instance(0, 13).unwrap();
        let group = BinaryGroup(&inst.fast);
        let planted = 641u64;
        let target = group.mul(&mut GroupOps::default(), inst.generator, planted);
        let ctx = InstanceCtx {
            group: &group,
            generator: inst.generator,
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(inst.n),
        };
        let base = KoblitzStandardSubspaceBase::new("koblitz-standard-subspace", &inst);
        let mut spec = PipelineSpec {
            factor_base: "koblitz-standard-subspace".into(),
            oracle: "pdp3-koblitz".into(),
            targets: Targets::Walk,
            max_trials: 20_000,
            seed: 811,
            linalg: "incremental-gauss".into(),
            ..Default::default()
        };
        spec.factor_base_params.set("dimension", "5");
        spec.oracle_params.set("engine", "rootfind-negfold-batch");
        let mut old_oracle = Pdp3KoblitzOracle::new("pdp3-koblitz", 3, &inst);
        let old = run_pipeline(
            &ctx,
            &spec,
            &base,
            &mut old_oracle,
            planted,
            &Calibration::default(),
            None,
        )
        .unwrap();
        spec.oracle_params
            .set("engine", "rootfind-negfold-batch-harvest");
        let mut new_oracle = Pdp3KoblitzOracle::new("pdp3-koblitz", 3, &inst);
        let new = run_pipeline(
            &ctx,
            &spec,
            &base,
            &mut new_oracle,
            planted,
            &Calibration::default(),
            None,
        )
        .unwrap();
        assert!(old.verified && new.verified);
        assert_eq!(old.recovered, Some(planted));
        assert_eq!(new.recovered, Some(planted));
        assert_eq!(
            new.decomposition.setup.get("pair_table_entries"),
            old.decomposition.setup.get("pair_table_entries")
        );
        assert_eq!(new.decomposition.setup.get("collision_candidates"), 27);
        assert_eq!(new.linear_algebra.cost.get("setup_seed_independent"), 5);
        assert_eq!(
            new.decomposition.setup.group_ops.adds,
            old.decomposition.setup.group_ops.adds + 3 * 27
        );
        assert!(new.decomposition.targets_tried <= old.decomposition.targets_tried);
        spec.oracle_params
            .set("engine", "rootfind-negfold-batch-harvest-torsion");
        let mut torsion_oracle = Pdp3KoblitzOracle::new("pdp3-koblitz", 3, &inst);
        let torsion = run_pipeline(
            &ctx,
            &spec,
            &base,
            &mut torsion_oracle,
            planted,
            &Calibration::default(),
            None,
        )
        .unwrap();
        spec.oracle_params.set("layout", "dense");
        let mut dense_oracle = Pdp3KoblitzOracle::new("pdp3-koblitz", 3, &inst);
        let dense = run_pipeline(
            &ctx,
            &spec,
            &base,
            &mut dense_oracle,
            planted,
            &Calibration::default(),
            None,
        )
        .unwrap();
        assert!(dense.verified);
        assert_eq!(dense.recovered, torsion.recovered);
        assert_eq!(
            dense.decomposition.targets_tried,
            torsion.decomposition.targets_tried
        );
        assert_eq!(
            dense.decomposition.relations_found,
            torsion.decomposition.relations_found
        );
        assert_eq!(dense.linear_algebra.rank, torsion.linear_algebra.rank);
        assert_eq!(dense.total_gae, torsion.total_gae);
        assert_eq!(
            dense.decomposition.setup.get("pair_table_dense_bytes"),
            65_544
        );
        spec.oracle_params.set("engine", "rootfind-negfold-batch");
        let mut unsupported = Pdp3KoblitzOracle::new("pdp3-koblitz", 3, &inst);
        assert!(run_pipeline(
            &ctx,
            &spec,
            &base,
            &mut unsupported,
            planted,
            &Calibration::default(),
            None,
        )
        .is_err());
        assert!(torsion.verified);
        assert_eq!(torsion.recovered, Some(planted));
        assert_eq!(torsion.linear_algebra.cost.get("setup_seed_independent"), 5);
        assert_eq!(torsion.decomposition.setup.get("collision_candidates"), 27);
        assert_eq!(
            torsion
                .decomposition
                .setup
                .get("torsion_skipped_collisions"),
            16
        );
        assert_eq!(torsion.decomposition.setup.get("collision_verified"), 11);
        assert_eq!(torsion.decomposition.setup.get("torsion_seed_rows"), 1);
        assert_eq!(
            torsion.decomposition.setup.get("pair_table_entries"),
            new.decomposition.setup.get("pair_table_entries")
        );
        eprintln!(
            "torsion trials={} target_rows={} rank={} setup_adds={} total_gae={} seed_rank={} collision_verified={} skipped={}",
            torsion.decomposition.targets_tried,
            torsion.decomposition.relations_found,
            torsion.linear_algebra.rank,
            torsion.decomposition.setup.group_ops.adds,
            torsion.total_gae,
            torsion.linear_algebra.cost.get("setup_seed_independent"),
            torsion.decomposition.setup.get("collision_verified"),
            torsion.decomposition.setup.get("torsion_skipped_collisions"),
        );
        eprintln!(
            "old trials={} target_rows={} rank={} setup_adds={} total_gae={}; harvest trials={} target_rows={} rank={} setup_adds={} total_gae={} seed_rank={}",
            old.decomposition.targets_tried,
            old.decomposition.relations_found,
            old.linear_algebra.rank,
            old.decomposition.setup.group_ops.adds,
            old.total_gae,
            new.decomposition.targets_tried,
            new.decomposition.relations_found,
            new.linear_algebra.rank,
            new.decomposition.setup.group_ops.adds,
            new.total_gae,
            new.linear_algebra.cost.get("setup_seed_independent"),
        );
    }

    #[test]
    fn binary_batch_scan_keeps_scalar_first_witness_and_lookup_order() {
        use super::{decompose_mitm, BinaryGroup, ColumnFold, FastPoint, GroupOps};
        use super::{OracleCounters, PairTable};
        use crypto_lib::cryptanalysis::ic_boundary::{koblitz_factor_base, koblitz_instance};
        use crypto_lib::cryptanalysis::ic_framework::stages::InstanceCtx;
        use crypto_lib::cryptanalysis::koblitz_index_calculus::build_standard_subspace_factor_base;
        use num_bigint::BigUint;

        let inst = koblitz_instance(0, 13).expect("Koblitz E0 n13");
        let frob = build_standard_subspace_factor_base(inst.koblitz.as_ref().unwrap(), 5)
            .expect("standard subspace");
        let fb = koblitz_factor_base(&inst, &frob, ColumnFold::Abscissa, "d5".into())
            .expect("signed base");
        assert_eq!(fb.points.len(), 33);
        assert_eq!(fb.neg_index[0], 0, "the first K13 point is self-negative");
        for k in (1..fb.points.len()).step_by(2) {
            assert_eq!(fb.neg_index[k], k + 1, "K13 signed pair at {k}");
            assert_eq!(fb.points[k].x, fb.points[k + 1].x);
        }
        let group = BinaryGroup(&inst.fast);
        let table = PairTable::build_negation_folded_binary_batch(&group, &fb);
        let ctx = InstanceCtx {
            group: &group,
            generator: inst.generator,
            target: inst.generator,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(inst.n),
        };
        let mut batch = Pdp3KoblitzOracle::new("pdp3-binary", 3, &inst);
        batch.engine = Pdp3Engine::RootfindNegfoldBatch;
        batch.pair_neg = Some(table);
        let mut points = Vec::new();
        for k in 1..=256u64 {
            points.push(
                group
                    .0
                    .mul(inst.generator, &BigUint::from((53 * k % (inst.r - 1)) + 1)),
            );
        }
        for k in [0, 1, fb.points.len() / 2, fb.points.len() - 1] {
            points.push(
                group
                    .0
                    .add(fb.points[k], group.0.add(fb.points[3], fb.points[7])),
            );
        }
        points.push(fb.points[0]);
        points.push(group.0.neg(fb.points[0]));
        points.push(FastPoint::INFINITY);
        for point in points {
            let mut scalar_ops = GroupOps::default();
            let mut scalar_ctr = OracleCounters::default();
            let expected = decompose_mitm(
                &group,
                &fb,
                batch.pair_neg.as_ref().unwrap(),
                3,
                &mut scalar_ops,
                &mut scalar_ctr,
                point,
            );
            let mut batch_ops = GroupOps::default();
            let mut batch_ctr = OracleCounters::default();
            let got =
                batch.decompose_rootfind_negfold(&ctx, &fb, &mut batch_ops, &mut batch_ctr, point);
            assert_eq!(got, expected, "first witness at {point:?}");
            assert_eq!(batch_ctr.lookups, scalar_ctr.lookups);
            // A completed eight-point block may do up to seven additions
            // beyond the scalar first-hit point; misses do identical work.
            assert!(batch_ops.adds >= scalar_ops.adds);
            assert!(batch_ops.adds <= scalar_ops.adds + 7);
        }
        assert_eq!(batch.scan_pair_layout, Some(true));
    }
}

#[cfg(test)]
mod committed_replays {
    use crypto_lib::cryptanalysis::ecbench::audit::{
        replay_input_json, replay_record, replay_with_accounting,
    };
    use crypto_lib::cryptanalysis::ecbench::methods::register_binary_plugins;
    use crypto_lib::cryptanalysis::ecbench::runner::read_records;
    use crypto_lib::cryptanalysis::ecbench::spec::{plan, Spec};
    use std::path::PathBuf;

    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// Replay `seq` of `rel`; `None` when the checkout does not carry the session.
    fn replay(rel: &str, seq: u64) -> Option<(bool, String)> {
        register_binary_plugins(crate::binary_plugins());
        let dir = root().join(rel);
        dir.join("session.json")
            .exists()
            .then(|| replay_record(&dir, seq).unwrap())
    }

    #[test]
    fn rootfind_records_predating_the_set_up_descriptors_replay() {
        // oracle_setup then carried `pair_table_entries` alone.
        let rel = "research/ecbench_rootfind_engine_20261012/abandoned/claim-n13-interrupted";
        if let Some((ok, detail)) = replay(rel, 59) {
            assert!(ok && detail == "identical", "{detail}");
        }
    }

    #[test]
    fn inherited_f4_record_from_apple_silicon_replays_under_the_pre_2043_accounting() {
        // Recorded on aarch64 before the eager rewrite and before the F4
        // word XORs were priced: lazy rewrite, wall-priced solver term
        // taken back out, 300,531 word XORs (current accounting: 301,961).
        let rel = "research/ecbench_rootfind_engine_20261012/abandoned/claim-n13-interrupted";
        if let Some((ok, detail)) = replay(rel, 58) {
            assert!(ok && detail.contains("pre-2043"), "{detail}");
        }
    }

    #[test]
    fn priced_inherited_f4_record_replays_under_the_pre_2043_priced_accounting() {
        let rel = "research/ecbench_rootfind_faster_20261014/sessions/cycle1";
        if let Some((ok, detail)) = replay(rel, 10) {
            assert!(ok && detail.contains("pre-2043-priced"), "{detail}");
        }
    }

    #[test]
    fn f6_ic_ladder_record_replays_under_the_pre_1646_accounting() {
        // Recorded 2026-10-05, before the early node oracle and the Four
        // Russians echelon of #1646, the harvest and gate-first defaults.
        let rel = "research/f6_ic_ecbench_ladder_20261005/sessions/n13";
        for seq in [24, 25] {
            if let Some((ok, detail)) = replay(rel, seq) {
                assert!(ok && detail.contains("pre-1646"), "seq {seq}: {detail}");
            }
        }
    }

    #[test]
    fn exhausted_inherited_f4_record_replays_under_the_pre_2043_accounting() {
        let rel = "research/f6_ic_ecbench_ladder_20261005/sessions/n7";
        if let Some((ok, detail)) = replay(rel, 1) {
            assert!(ok, "{detail}");
        }
    }

    #[test]
    fn calibration_row_added_in_1640_is_not_applied_to_older_records() {
        let rel = "research/ecbench_all_candidates_20261003/sessions/koblitz";
        if let Some((ok, detail)) = replay(rel, 625) {
            assert!(ok && detail.contains("pre-1646"), "{detail}");
        }
    }

    /// Recorded 2026-10-24, before repair R8 (`59fcca75d4`, 2026-10-26)
    /// narrowed the default linear harvest from `gpv OR rank >= 331` to the
    /// rank gate: the `f4-m3-default` arm (every sixth seq from 2) harvests
    /// fewer root rows today and counts fewer solver operations; the other
    /// arms (`off`, `gpv`, m = 2) never differed.
    #[test]
    fn f4_default_harvest_record_replays_under_the_pre_r8_harvest_accounting() {
        let rel = "research/f4_harvest_remeasure_20261024/sessions/K1n11";
        for seq in [2, 8, 140] {
            if let Some((ok, detail)) = replay(rel, seq) {
                assert!(
                    ok && detail.contains("pre-r8-harvest"),
                    "seq {seq}: {detail}"
                );
            }
        }
        for seq in [1, 3, 4, 5] {
            if let Some((ok, detail)) = replay(rel, seq) {
                assert!(ok && detail == "identical", "seq {seq}: {detail}");
            }
        }
    }

    /// The profile excuses nothing the record does not reproduce, and is
    /// offered to its producer alone.
    #[test]
    fn the_pre_r8_harvest_accounting_does_not_excuse_a_changed_count_or_binary() {
        let rel = "research/f4_harvest_remeasure_20261024/sessions/K1n11";
        if let Some((ok, detail)) = replay_edited(rel, 2, |r| {
            *r.phases[2].native.get_mut("solver_ops").unwrap() += 1;
        }) {
            assert!(!ok, "{detail}");
        }
        if let Some((ok, detail)) = replay_edited(rel, 2, |r| {
            r.binary_sha256 = Some("0".repeat(64));
        }) {
            assert!(!ok, "{detail}");
        }
    }

    #[test]
    fn window_negfold_without_a_divisor_is_the_original_third_window() {
        let rel = "research/ecbench_rootfind_parity_20261013/sessions/cycle1";
        if let Some((ok, detail)) = replay(rel, 17) {
            assert!(ok, "{detail}");
        }
    }

    /// Replay record `seq` of `rel` after `edit` has changed the record
    /// (not its session): `None` when the checkout does not carry it.
    fn replay_edited(
        rel: &str,
        seq: u64,
        edit: impl FnOnce(&mut crypto_lib::cryptanalysis::ecbench::record::Record),
    ) -> Option<(bool, String)> {
        register_binary_plugins(crate::binary_plugins());
        let dir = root().join(rel);
        if !dir.join("session.json").exists() {
            return None;
        }
        let spec_text = std::fs::read_to_string(dir.join("spec.json")).unwrap();
        let p = plan(Spec::from_json(&spec_text).unwrap()).unwrap();
        let mut records = read_records(&dir).unwrap();
        let r = records.iter_mut().find(|r| r.seq == seq).unwrap();
        edit(r);
        let session = crypto_lib::cryptanalysis::ecbench::runner::read_session(&dir).unwrap();
        let host: crypto_lib::cryptanalysis::ecbench::host::HostCapsule =
            serde_json::from_str(&std::fs::read_to_string(dir.join("host.json")).unwrap()).unwrap();
        Some(replay_with_accounting(
            &replay_input_json(&p, r).unwrap(),
            r,
            &host,
            &session,
        ))
    }

    /// A legacy accounting is accepted only when the whole record
    /// reproduces: one word XOR more in the record, and it does not.
    #[test]
    fn a_legacy_accounting_does_not_excuse_a_changed_count() {
        let rel = "research/ecbench_rootfind_engine_20261012/abandoned/claim-n13-interrupted";
        if let Some((ok, detail)) = replay_edited(rel, 58, |r| {
            *r.phases[2].native.get_mut("solver_ops").unwrap() += 1;
        }) {
            assert!(!ok, "{detail}");
            assert!(detail.contains("phase counts"), "{detail}");
        }
    }

    /// A record that names a binary no profile lists is replayed exactly:
    /// the same committed record, one digit of its binary hash changed,
    /// is not excused by any legacy accounting.  One case per profile, and
    /// one for the set-up descriptors and the missing `window_div`.
    #[test]
    fn a_record_from_an_unlisted_binary_is_not_excused() {
        let unlisted = "0".repeat(64);
        let cases = [
            // pre-2043: lazy rewrite, F4 solver term taken out
            (
                "research/ecbench_rootfind_engine_20261012/abandoned/claim-n13-interrupted",
                58,
            ),
            // pre-2043-priced
            (
                "research/ecbench_rootfind_faster_20261014/sessions/cycle1",
                10,
            ),
            // pre-1646
            ("research/f6_ic_ecbench_ladder_20261005/sessions/n13", 25),
            // the calibration row of #1640, under pre-1646
            (
                "research/ecbench_all_candidates_20261003/sessions/koblitz",
                625,
            ),
            // set-up descriptors the record predates
            (
                "research/ecbench_rootfind_engine_20261012/abandoned/claim-n13-interrupted",
                59,
            ),
            // rootfind-window-negfold without window_div
            (
                "research/ecbench_rootfind_parity_20261013/sessions/cycle1",
                17,
            ),
        ];
        for (rel, seq) in cases {
            let listed = replay_edited(rel, seq, |_| {});
            let Some((ok, detail)) = listed else { continue };
            assert!(ok, "{rel} seq {seq} replays as committed: {detail}");
            let (ok, detail) = replay_edited(rel, seq, |r| {
                r.binary_sha256 = Some(unlisted.clone());
            })
            .unwrap();
            assert!(
                !ok,
                "{rel} seq {seq} excused from an unlisted binary: {detail}"
            );
            assert!(!detail.contains("accounting of the code"), "{detail}");
        }
    }
}
