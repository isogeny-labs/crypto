//! `genroot`: the generalized root-finder as a decomposition oracle.
//!
//! The lab's generalized root-finder (`tools/generalized-rootfinder`)
//! solves polynomial systems whose source variables are confined to a
//! domain `D`, exactly and completely, without Gröbner bases.  Its
//! [`solver::S3Chain`] is the decomposition problem written as a chain of
//! Semaev `S3` equations,
//!
//! `S3(w_i, x_{i+1}, w_{i+1}) = 0` for `1 <= i < m`, `w_1 = x_1`, `w_m = x(R)`,
//!
//! with the sources `x_i` in `D` (the factor base's abscissae) and the
//! auxiliaries `w_i` anywhere in the field.  It is solved by meeting in the
//! middle: a target-independent table of every forward partial chain over
//! `k` sources, then a backward walk from `x(R)` over the other `m - k`.
//! Each step of either side is a *fiber* — the two roots in `Z` of
//! `S3(x, y, Z)`, i.e. `x(P + Q)` and `x(P - Q)` — found by exact
//! root-finding (Cantor–Zassenhaus) and replayed against `S3`.
//!
//! Because it needs only abscissae, a field and the curve's `S3`, one
//! implementation serves prime curves, ordinary binary curves and Koblitz
//! curves ([`GenRootGroup`]).  The solver source is compiled into this
//! crate from `tools/generalized-rootfinder/src/solver.rs`, so the oracle
//! runs the same code the tool's tests check.
//!
//! **Costing.**  Every fiber the solver computes is charged as two group
//! additions (it yields two sums), and every solution's sign resolution
//! is charged by [`lift_abscissae`].  A fiber's root-finding costs more than
//! two additions; its exact field multiplications and inversions (in the
//! solver's arbitrary-precision arithmetic) are counted and reported as
//! `genroot_field_muls` / `genroot_field_invs` but not priced, and the
//! pipeline marks a `genroot` run's total cost as partial for that reason.
//!
//! **Fail closed.**  A solver error (a cap, an exhausted splitting budget,
//! an unrestricted fiber) is counted in `genroot_unknown` and the target is
//! skipped; it is never read as "does not decompose".  Every returned
//! relation is re-added in the group by [`lift_abscissae`].

#[path = "../../../tools/generalized-rootfinder/src/solver.rs"]
#[allow(
    dead_code,
    unused_imports,
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]
pub mod solver;

use std::collections::BTreeMap;

use num_bigint::BigUint;
use num_traits::ToPrimitive;

use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    lift_abscissae, BinaryGroup, CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve,
    PrimePoint,
};
use crate::cryptanalysis::koblitz_fast::FastPoint;
use solver::field::{op_tally, Field};
use solver::{Limits, S3Chain};

/// A curve group the generalized root-finder can describe: its field, the
/// coefficients of its `S3`, and the abscissa of a point.
pub trait GenRootGroup: CountedGroup {
    /// `(field, [a2, a4, a6] or [0, 0, b], binary)`, as [`S3Chain::new`]
    /// takes them.
    fn genroot_model(&self) -> Result<(Field, [BigUint; 3], bool), String>;
    /// The abscissa of a finite point, `None` for the identity.
    fn genroot_abscissa(&self, p: &Self::Elt) -> Option<u64>;
}

impl GenRootGroup for PrimeCurve {
    fn genroot_model(&self) -> Result<(Field, [BigUint; 3], bool), String> {
        let field = Field::prime(BigUint::from(self.p), None)?;
        Ok((
            field,
            [
                BigUint::from(0u8),
                BigUint::from(self.a),
                BigUint::from(self.b),
            ],
            false,
        ))
    }
    fn genroot_abscissa(&self, p: &PrimePoint) -> Option<u64> {
        (!p.infinity).then_some(p.x)
    }
}

impl GenRootGroup for BinaryGroup<'_> {
    fn genroot_model(&self) -> Result<(Field, [BigUint; 3], bool), String> {
        let curve = self.0;
        let field = Field::binary(curve.n as usize, BigUint::from(curve.field.irr))?;
        Ok((
            field,
            [
                BigUint::from(0u8),
                BigUint::from(0u8),
                BigUint::from(curve.b),
            ],
            true,
        ))
    }
    fn genroot_abscissa(&self, p: &FastPoint) -> Option<u64> {
        (!p.infinity).then_some(p.x)
    }
}

/// The `genroot` decomposition oracle.  See the module documentation.
pub struct GenRootOracle {
    m: u32,
    chain: Option<S3Chain>,
    setup_fibers: u64,
    setup_field_muls: u64,
    setup_field_invs: u64,
    /// Fibers computed while solving targets.
    target_fibers: u64,
    cache_hits: u64,
    probes: u64,
    field_muls: u64,
    field_invs: u64,
    /// Targets the solver could not decide.  Must stay zero for a run's
    /// relation yield to mean anything.
    pub unknown: u64,
    /// Chain solutions found (abscissa tuples), and how many of them were
    /// lifted before a relation was returned.
    solutions: u64,
    truncated: u64,
    lift_failures: u64,
    last_error: Option<String>,
}

impl GenRootOracle {
    pub fn new(m: u32) -> Self {
        Self {
            m,
            chain: None,
            setup_fibers: 0,
            setup_field_muls: 0,
            setup_field_invs: 0,
            target_fibers: 0,
            cache_hits: 0,
            probes: 0,
            field_muls: 0,
            field_invs: 0,
            unknown: 0,
            solutions: 0,
            truncated: 0,
            lift_failures: 0,
            last_error: None,
        }
    }

    /// The left split `k`: by default `ceil(m / 2)`, which keeps the
    /// per-target walk over `floor(m / 2)` sources.
    fn default_left(m: u32) -> u32 {
        m.div_ceil(2)
    }

    /// The last solver error, if any target went undecided.
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}

impl<G: GenRootGroup> DecompositionOracle<G> for GenRootOracle {
    fn name(&self) -> &str {
        "genroot"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, params: &Params) -> String {
        let left = params
            .get("left")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or_else(|| Self::default_left(self.m));
        format!(
            "generalized root-finder: the exact all-roots S3 chain over the base's abscissae, \
             m = {}, meeting in the middle after {} source(s); each fiber is found by exact \
             root-finding and charged as two group additions (its field work is reported, \
             not priced)",
            self.m, left
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("m", "summands, 2 to 6"),
            (
                "left",
                "sources in the target-independent table, 1..m-1; default ceil(m/2)",
            ),
            (
                "max_table",
                "cap on left-table records (default 4000000); exceeding it refuses the run",
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
        if !(2..=6).contains(&self.m) {
            return Err(format!("genroot takes m = 2..6, got {}", self.m));
        }
        for key in params.0.keys() {
            if !matches!(key.as_str(), "m" | "left" | "max_table") {
                return Err(format!("genroot does not read `{key}`"));
            }
        }
        let left = match params.get("left") {
            Some(v) => v
                .parse::<u32>()
                .map_err(|_| format!("genroot: left={v} is not a number"))?,
            None => Self::default_left(self.m),
        };
        if left == 0 || left >= self.m {
            return Err(format!(
                "genroot: left must be in 1..{}, got {left}",
                self.m
            ));
        }
        let max_table = match params.get("max_table") {
            Some(v) => v
                .parse::<usize>()
                .map_err(|_| format!("genroot: max_table={v} is not a number"))?,
            None => 4_000_000,
        };
        if fb.abscissa_list.is_empty() {
            return Err("genroot needs a non-empty factor base".into());
        }
        let (field, curve, binary) = ctx.group.genroot_model()?;
        let domain: Vec<BigUint> = fb.abscissa_list.iter().map(|&x| BigUint::from(x)).collect();
        let limits = Limits {
            // Work is bounded per target by the split, not by a running
            // budget that would end a long run part-way.
            max_work: u64::MAX,
            max_table,
            max_solutions: 1 << 16,
            max_domain: domain.len(),
            max_degree: 2,
            max_trials: u64::MAX,
            seed: 1,
        };
        let (m0, i0) = op_tally();
        let chain = S3Chain::new(
            field,
            domain,
            curve,
            binary,
            self.m as usize,
            left as usize,
            &limits,
        )
        .map_err(|e| format!("genroot: {e}"))?;
        let (m1, i1) = op_tally();
        self.setup_fibers = chain.setup_calls;
        self.setup_field_muls = m1 - m0;
        self.setup_field_invs = i1 - i0;
        ops.adds += 2 * chain.setup_calls;
        ops.field_ops.muls += m1 - m0;
        ops.field_ops.invs += i1 - i0;
        self.chain = Some(chain);
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        let mut out = BTreeMap::new();
        if let Some(chain) = &self.chain {
            out.insert("genroot_setup_fibers".into(), self.setup_fibers);
            out.insert("genroot_left_records".into(), chain.left_records as u64);
            out.insert("genroot_setup_field_muls".into(), self.setup_field_muls);
            out.insert("genroot_setup_field_invs".into(), self.setup_field_invs);
        }
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
        let x = ctx.group.genroot_abscissa(&point)?;
        let chain = self.chain.as_mut()?;
        let (calls0, hits0, probes0) = (chain.calls, chain.cache_hits, chain.probes);
        let (m0, i0) = op_tally();
        let result = chain.solve(&BigUint::from(x));
        let (m1, i1) = op_tally();
        let fibers = chain.calls - calls0;
        let hits = chain.cache_hits - hits0;
        let probes = chain.probes - probes0;
        self.target_fibers += fibers;
        self.cache_hits += hits;
        self.probes += probes;
        self.field_muls += m1 - m0;
        self.field_invs += i1 - i0;
        ops.adds += 2 * fibers;
        ops.field_ops.muls += m1 - m0;
        ops.field_ops.invs += i1 - i0;
        counters.lookups += hits + probes;
        let search = match result {
            Ok(s) => s,
            Err(e) => {
                self.unknown += 1;
                self.last_error = Some(e);
                return None;
            }
        };
        if search.lower_bound > search.solutions.len() as u64 {
            self.truncated += 1;
        }
        let m = self.m as usize;
        for row in &search.solutions {
            self.solutions += 1;
            let xs: Option<Vec<u64>> = row[..m].iter().map(ToPrimitive::to_u64).collect();
            let Some(xs) = xs else {
                self.lift_failures += 1;
                counters.lift_failures += 1;
                continue;
            };
            match lift_abscissae(ctx.group, fb, ops, &xs, point) {
                Some(indices) => return Some(indices),
                None => {
                    self.lift_failures += 1;
                    counters.lift_failures += 1;
                }
            }
        }
        if !search.solutions.is_empty() {
            counters.unliftable_systems += 1;
        }
        None
    }

    fn relation_counters(&self) -> BTreeMap<String, u64> {
        BTreeMap::from([
            ("genroot_fibers".to_string(), self.target_fibers),
            ("genroot_cache_hits".to_string(), self.cache_hits),
            ("genroot_probes".to_string(), self.probes),
            ("genroot_solutions".to_string(), self.solutions),
            ("genroot_truncated".to_string(), self.truncated),
            ("genroot_lift_failures".to_string(), self.lift_failures),
            ("genroot_unknown".to_string(), self.unknown),
        ])
    }

    fn relation_work_native(&self) -> BTreeMap<String, u64> {
        BTreeMap::from([
            ("genroot_field_muls".to_string(), self.field_muls),
            ("genroot_field_invs".to_string(), self.field_invs),
        ])
    }
}
