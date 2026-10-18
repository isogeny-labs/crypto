//! PS-6: the pairs-and-solve oracles of the PDP-speedup program as
//! `ic.pipeline` oracles on a random binary curve, so that the baseline,
//! the PS-2 group-law kernel and the repository's meet-in-the-middle table
//! are charged in ecbench's counted unit, on the same one-target workloads,
//! factor bases and seeds.
//!
//! * `semaev-s4-baseline:m=3` is [`ic_boundary::Oracle::SemaevS4`] as the
//!   boundary ledger runs it (`SubspaceOracle::decompose` on the low-order
//!   subspace, then the framework's sign lift), so the existing ledger row
//!   and this oracle are the same code path.  Its pairs are counted in
//!   `s4_pairs` and priced at the pinned `ns_per_s4_pair` of the curve.
//! * `semaev-s4-grouplaw:m=3[,audit=1]` is the PS-2 kernel
//!   ([`crate::ps6_kernel`]) on the same base.  It returns the baseline's
//!   *witness*, not only its verdict, so a pipeline run with either oracle
//!   draws the same targets, finds the same relations and ends at the same
//!   rank.  A degenerate target (PS-1) is decided by the baseline inside the
//!   call and charged as the baseline is.  With `audit=1` every target is
//!   also decided by the baseline and the two witnesses compared; that work
//!   is counted apart (`audit_*` in the solver block) and never charged.
//! * The meet-in-the-middle incumbent is the library's own `mitm` oracle and
//!   needs no code here.
//!
//! ## What is charged
//!
//! Group work (the factor base's and the table's additions, each trial's
//! `[a]G + [b]Q`, the sign lift of a witness, the verification) goes through
//! the `GroupOps` ledger, as for every oracle.  The baseline's pairs are
//! priced by `price_phase` at the pinned `ns_per_s4_pair` of the curve.  The
//! group-law kernel's pairs have **no pinned conversion**
//! (`docs/ic/calibration.json` carries none for them, and a library edit to
//! add one is outside this plug-in), so the kernel reports them through
//! `solver_totals` in its own unit, `group-law kernel pairs`, which the
//! runner counts, records and leaves out of `S` (`ic_report` takes any
//! non-pinned solver term back out): the kernel row is a **lower bound** and
//! says so.  The per-curve construction of the baseline's `SubspaceOracle`
//! (the span and `L_V`) and of the kernel's base list are native setup with
//! no unit; they are counted under names ending `_uncharged`.
//!
//! The pipeline's own ordering decides nothing here: both oracles return the
//! identical witness for the identical target, so the two arms' trials,
//! relations, matrix rank and recovered scalars agree, and any difference in
//! `S` is the oracle's own charge.

use std::collections::BTreeMap;
use std::time::Instant;

use crypto_lib::cryptanalysis::ecbench::methods::BinaryPlugins;
use crypto_lib::cryptanalysis::ic_boundary::{
    lift_abscissae, BinaryGroup, BinaryInstance, FactorBase, GroupOps, OracleCounters,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, InstanceCtx, Params, SolverCost, SolverTotals, SystemShape,
};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use crypto_lib::cryptanalysis::semaev_decomp::SubspaceOracle;

use crate::ps6_kernel::{degenerate_target, sqrt_b, Kernel, Work};

/// Unit string of the kernel's count.  It is not `word XORs`, so the runner
/// leaves it out of `S` and lists it unpriced.
pub const KERNEL_PAIR_UNIT: &str = "group-law kernel pairs";

/// The low-order subspace `⟨1, z, …, z^{l−1}⟩` of the run's base, which both
/// oracles need: the base must be `binary-subspace` over the standard basis.
fn standard_dimension(fb: &FactorBase<FastPoint>) -> Result<u32, String> {
    let l = fb
        .dimension
        .ok_or("the oracle needs the binary-subspace factor base")?;
    let want: Vec<u64> = (0..l).map(|i| 1u64 << i).collect();
    if fb.subspace_basis.as_deref() != Some(want.as_slice()) {
        return Err("the oracle needs the low-order subspace <1, z, ..., z^{l-1}>".into());
    }
    if l > 30 {
        return Err(format!("dimension {l} is beyond the enumerated subspace"));
    }
    Ok(l)
}

fn require_m3(summands: u32, name: &str) -> Result<(), String> {
    if summands != 3 {
        return Err(format!(
            "{name} is the three-summand (S4) oracle: pass m=3, got m={summands}"
        ));
    }
    Ok(())
}

/// Sign-resolve a witness of abscissae into base-point indices, exactly as
/// `Oracle::SemaevS4` does after `SubspaceOracle::decompose`.
fn lift_witness(
    ctx: &InstanceCtx<BinaryGroup<'_>>,
    fb: &FactorBase<FastPoint>,
    ops: &mut GroupOps,
    counters: &mut OracleCounters,
    witness: Option<[u64; 3]>,
    point: FastPoint,
) -> Option<Vec<usize>> {
    let xs = witness?;
    counters.s4_hits += 1;
    match lift_abscissae(ctx.group, fb, ops, &xs, point) {
        Some(w) => Some(w),
        None => {
            counters.lift_failures += 1;
            None
        }
    }
}

// ── (a) the baseline ──────────────────────────────────────────────

/// `semaev-s4-baseline:m=3`.
pub struct S4Baseline<'i> {
    summands: u32,
    inst: &'i BinaryInstance,
    oracle: Option<SubspaceOracle>,
    setup: BTreeMap<String, u64>,
}

impl<'i> S4Baseline<'i> {
    pub fn new(summands: u32, inst: &'i BinaryInstance) -> Self {
        Self {
            summands,
            inst,
            oracle: None,
            setup: BTreeMap::new(),
        }
    }
}

impl<'a> DecompositionOracle<BinaryGroup<'a>> for S4Baseline<'_> {
    fn name(&self) -> &str {
        "semaev-s4-baseline"
    }

    fn summands(&self) -> u32 {
        self.summands
    }

    fn describe(&self, _params: &Params) -> String {
        "Semaev S4 pairs-and-solve over the low-order subspace (semaev_decomp::SubspaceOracle, \
         the oracle the boundary ledger runs as Oracle::SemaevS4); pairs priced at the pinned \
         ns_per_s4_pair"
            .into()
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[("m", "summands; this oracle is the three-summand S4 loop, so 3")]
    }

    fn prepare(
        &mut self,
        _ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        _params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<(), String> {
        require_m3(self.summands, "semaev-s4-baseline")?;
        let l = standard_dimension(fb)?;
        let basis = fb.subspace_basis.clone().expect("checked above");
        let o = SubspaceOracle::new(&basis, self.inst.b, &self.inst.gf);
        self.setup
            .insert("s4_oracle_span_words_uncharged".into(), o.span.len() as u64);
        self.setup
            .insert("s4_oracle_lv_coefficients_uncharged".into(), o.lv.len() as u64);
        debug_assert_eq!(o.l, l);
        self.oracle = Some(o);
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        self.setup.clone()
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: FastPoint,
    ) -> Option<Vec<usize>> {
        let (witness, pairs) = self.oracle.as_ref()?.decompose(point.x, &self.inst.gf);
        counters.s4_pairs += pairs;
        lift_witness(ctx, fb, ops, counters, witness, point)
    }
}

// ── (b) the PS-2 group-law kernel ─────────────────────────────────

/// `semaev-s4-grouplaw:m=3[,audit=1]`.
pub struct S4GroupLaw<'i> {
    summands: u32,
    inst: &'i BinaryInstance,
    audit: bool,
    l: u32,
    /// The baseline, for degenerate targets (and, with `audit`, every target).
    baseline: Option<SubspaceOracle>,
    kernel: Option<Kernel<'i>>,
    totals: SolverTotals,
    shape: SystemShape,
    setup: BTreeMap<String, u64>,
    /// `audit_checks`, `audit_mismatches`, `audit_baseline_pairs`,
    /// `degenerate_targets`, `kernel_calls`, `kernel_hits`.
    tally: BTreeMap<String, u64>,
    /// Targets decided by the kernel, kernel work summed.
    work: Work,
    /// Witnesses that differed from the baseline's (audit only); must be zero.
    pub mismatches: u64,
}

impl<'i> S4GroupLaw<'i> {
    pub fn new(summands: u32, inst: &'i BinaryInstance) -> Self {
        Self {
            summands,
            inst,
            audit: false,
            l: 0,
            baseline: None,
            kernel: None,
            totals: SolverTotals::default(),
            shape: SystemShape::default(),
            setup: BTreeMap::new(),
            tally: BTreeMap::new(),
            work: Work::default(),
            mismatches: 0,
        }
    }

    fn bump(&mut self, key: &str, by: u64) {
        *self.tally.entry(key.into()).or_insert(0) += by;
    }
}

impl<'a> DecompositionOracle<BinaryGroup<'a>> for S4GroupLaw<'_> {
    fn name(&self) -> &str {
        "semaev-s4-grouplaw"
    }

    fn summands(&self) -> u32 {
        self.summands
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "Semaev S4 pairs-and-solve by the group law (the PS-2 scalar kernel, generalised to \
             any a, b): the four candidates x(P1 +- P2 +- R) per pair of rational base points; \
             pairs reported in their own unit, not priced{}",
            if params.flag("audit") {
                "; every target is also decided by the baseline and the witnesses compared"
            } else {
                ""
            }
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("m", "summands; this oracle is the three-summand S4 loop, so 3"),
            (
                "audit",
                "1 to decide every target with the baseline as well and compare witnesses; the \
                 baseline's work is counted apart and not charged",
            ),
        ]
    }

    fn prepare(
        &mut self,
        _ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<(), String> {
        require_m3(self.summands, "semaev-s4-grouplaw")?;
        let l = standard_dimension(fb)?;
        self.l = l;
        self.audit = params.flag("audit");
        let inst = self.inst;
        let basis = fb.subspace_basis.clone().expect("checked above");
        let o = SubspaceOracle::new(&basis, inst.b, &inst.gf);
        self.setup
            .insert("s4_oracle_span_words_uncharged".into(), o.span.len() as u64);
        self.setup
            .insert("s4_oracle_lv_coefficients_uncharged".into(), o.lv.len() as u64);
        self.baseline = Some(o);
        // Rational base points of V in increasing abscissa: the run's own
        // base supplies every abscissa of V that lifts (x = 0 is not in it,
        // being the 2-torsion point T = (0, sqrt b), which the baseline's
        // span does contain and the kernel's base must too).
        let mut base: Vec<(u64, u64)> = Vec::with_capacity(fb.abscissa_list.len() + 1);
        base.push((0, sqrt_b(&inst.gf, inst.b)));
        for &x in &fb.abscissa_list {
            let idx = fb.x_index.get(&x).ok_or("abscissa without a point")?;
            let p = fb.points[idx[0]];
            base.push((p.x, p.y));
        }
        base.sort_unstable_by_key(|&(x, _)| x);
        if base.windows(2).any(|w| w[0].0 == w[1].0) || base.iter().any(|&(x, _)| x >> l != 0) {
            return Err("the factor base is not the abscissae of the subspace".into());
        }
        self.setup
            .insert("grouplaw_base_points_uncharged".into(), base.len() as u64);
        self.kernel = Some(Kernel::new(&inst.gf, inst.a, l, base));
        self.totals = SolverTotals {
            solver: "ps6 group-law kernel (PS-2 generalised)".into(),
            ..Default::default()
        };
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        self.setup.clone()
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        fb: &FactorBase<FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: FastPoint,
    ) -> Option<Vec<usize>> {
        let inst = self.inst;
        let gf = &inst.gf;
        // One field inversion per target (the degeneracy test); no unit, so it
        // is counted in `inversions`, which the runner lists unpriced.
        counters.inversions += 1;
        let witness = if degenerate_target(gf, inst.b, self.l, point.x) {
            // PS-1: the baseline decides these, and is charged as the
            // baseline is, in the pinned s4_pairs.
            let (w, pairs) = self.baseline.as_ref()?.decompose(point.x, gf);
            counters.s4_pairs += pairs;
            self.bump("degenerate_targets", 1);
            w
        } else {
            let started = Instant::now();
            let (w, work) = self.kernel.as_mut()?.first((point.x, point.y));
            let wall_ns = started.elapsed().as_nanos() as u64;
            self.work.absorb(work);
            self.bump("kernel_calls", 1);
            self.bump("kernel_hits", u64::from(w.is_some()));
            let mut extra = BTreeMap::new();
            extra.insert("rows".to_string(), work.rows);
            extra.insert("batch_elements".to_string(), work.batch_elems);
            extra.insert("exact_pairs".to_string(), work.exact_pairs);
            let cost = SolverCost {
                ops: work.pairs,
                op_unit: KERNEL_PAIR_UNIT.into(),
                wall_ns,
                peak_bytes: 0,
                degree_reached: None,
                solving_degree: None,
                timed_out: false,
                extra,
            };
            self.totals.absorb(&self.shape, Some(&cost), false);
            if self.audit {
                let (bw, bpairs) = self.baseline.as_ref()?.decompose(point.x, gf);
                self.bump("audit_checks", 1);
                self.bump("audit_baseline_pairs", bpairs);
                if bw != w {
                    self.bump("audit_mismatches", 1);
                    self.mismatches += 1;
                }
            }
            w
        };
        // The tally rides in the solver block's `extra`, a total over calls.
        for (k, v) in std::mem::take(&mut self.tally) {
            *self.totals.extra.entry(k).or_insert(0) += v;
        }
        lift_witness(ctx, fb, ops, counters, witness, point)
    }

    fn last_system(&self) -> Option<SystemShape> {
        None
    }

    fn solver_totals(&self) -> Option<SolverTotals> {
        Some(self.totals.clone())
    }
}

// ── registration ──────────────────────────────────────────────────

const ORACLE_NAMES: [&str; 2] = ["semaev-s4-baseline", "semaev-s4-grouplaw"];

/// This module's oracles, for the ecbench binary's plug-in table.  It adds
/// no factor base: the arms use the library's `binary-subspace`.
pub fn oracle<'i>(
    name: &str,
    summands: u32,
    instance: &'i BinaryInstance,
) -> Option<Box<dyn DecompositionOracle<BinaryGroup<'i>> + 'i>> {
    let name = ORACLE_NAMES.iter().find(|n| **n == name)?;
    Some(match *name {
        "semaev-s4-baseline" => Box::new(S4Baseline::new(summands, instance)),
        _ => Box::new(S4GroupLaw::new(summands, instance)),
    })
}

/// The binary's whole plug-in table: PDP3's (factor bases and oracle) and
/// this module's oracles.  `methods::register_binary_plugins` keeps the
/// first registration only, so the two tables are merged here.
pub fn plugins(pdp3: BinaryPlugins) -> BinaryPlugins {
    // `BinaryPlugins` holds plain function pointers, so the merge needs the
    // PDP3 table in a static; `OnceLock` keeps it for the two closures-free
    // functions below.
    let _ = PDP3.set(pdp3);
    BinaryPlugins {
        base: |name, instance| {
            let pdp3 = PDP3.get()?;
            (pdp3.base)(name, instance)
        },
        oracle: |name, summands, instance| {
            oracle(name, summands, instance).or_else(|| {
                let pdp3 = PDP3.get()?;
                (pdp3.oracle)(name, summands, instance)
            })
        },
    }
}

static PDP3: std::sync::OnceLock<BinaryPlugins> = std::sync::OnceLock::new();
