//! # `ic.koblitz_descent`: the ledger §23 one-target Koblitz index calculus.
//!
//! The pipeline `ic price --single-target` assembles in `one_pass_with`
//! (`src/bin/ic/price.rs`), stage for stage, from the library:
//!
//! 1. **factor_base** — [`build_subgroup_orbit_factor_base_with_cost`]
//!    (`subgroup_orbits` base of `factor_base_points` points, seed
//!    `seed`) and the projected signed-orbit column count;
//! 2. **oracle_setup** — the pair-sum table at the tier
//!    [`PairSumTable::build_within_for`] picks for the run's planned
//!    probing ([`ProbeBudget::for_run`], one target);
//! 3. **relations** — [`RelationCollector::with_pair_table`] and
//!    [`RelationCollector::collect_aimed`], aimed by [`ColumnCoverage`],
//!    `units` work units of `unit_trials` walked `m = 3` probes scanning
//!    `collection_window` summands, extended unit by unit (up to
//!    `max_units`) until the logs are determined;
//! 4. **linear_algebra** — [`FactorBaseLogSolver`] (relation re-check,
//!    sparse filter and block Wiedemann, every column log checked as a
//!    point);
//! 5. **descent_setup** — [`IndividualLogSolver::new`] (the walks'
//!    stride multiples), target-independent;
//! 6. **descent** — [`IndividualLogSolver::solve_report`] on the target:
//!    the walked `m = descent_summands` descent, the online interval;
//! 7. **verify** — the pipeline's own final `[d]G = Q` in the general
//!    arithmetic, as `ic price` makes it.
//!
//! **Charged in the counted unit.**  None of these stages keeps a
//! `GroupOps` ledger, so each is charged by the difference of two
//! [`group_op_count::ops_snapshot`]s around it: every point addition and
//! doubling the stage performs, in every arithmetic it reaches
//! ([`crate::cryptanalysis::group_op_count`] lists them and what one
//! count is).  The counters are per thread, so the whole pipeline runs on
//! one thread: inside a one-thread Rayon pool whose single worker also
//! holds the measurement session and takes the snapshots.  Every
//! parallel iterator in the stages runs on that worker.
//!
//! Native work the unit does not price — canonicalisations, pair-table
//! filter probes and bucket reads, Frobenius coordinate maps, the
//! factor-base lifts, linear-algebra row operations — is counted per phase
//! and reported `*_uncharged`.

use std::collections::BTreeMap;
use std::time::Instant;

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::binary_ecc::BinaryPoint;
use crate::cryptanalysis::ecbench::canonical::{sha256_hex, short_id};
use crate::cryptanalysis::ecbench::methods::FactorBaseFacts;
use crate::cryptanalysis::group_op_count::{ops_snapshot, OpCounts};
use crate::cryptanalysis::ic_boundary::{
    koblitz_factor_base, BinaryGroup, BinaryInstance, ColumnFold, GroupOps,
    OracleCounters, Relation,
};
use crate::cryptanalysis::ic_framework::large_prime::{LargePrimeCollector, LargePrimeConfig};
use crate::cryptanalysis::ic_framework::stages::InstanceCtx;
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase, Snapshot};
use crate::cryptanalysis::koblitz_index_calculus::{
    aimed_window_indices, build_subgroup_orbit_factor_base_with_cost, probe_run_stride,
    projected_signed_orbit_count, walked_probe_scalar, CollectedRelation, ColumnCoverage,
    DecompositionStrategy, FactorBaseLogSolver, FactorBaseSelectionCost, FrobeniusFactorBase,
    IndividualLogSolver, KoblitzCurve, KoblitzIcOptions, LinearAlgebra, PairSumTable,
    ProbeBudget, ProjectedFactorBase, ProjectedRelation, RelationCollector, RelationWorkUnit,
    PROBE_RUN,
};
use crate::cryptanalysis::koblitz_large_prime::LargePrimeLimits;
use crate::cryptanalysis::koblitz_sparse_la::{SparseSolveOptions, SparseSolveReport};

/// The value of a parameter that takes the rule v8 parameter file's value
/// at this curve (`research/ic_tool_program/rule/v8/runs/<k?n?>/T01.params.json`).
pub const RULE_V8: &str = "rule_v8";

/// The `ic` binary's cap on a materialised factor base's abscissae
/// (`experiment::MAX_ABSCISSAE`), kept so the method refuses what
/// `ic price` refuses.
const MAX_ABSCISSAE: usize = 1 << 19;

/// One rule v8 parameter file, the frozen §23 recipe at one size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuleV8 {
    pub n: u32,
    pub a: u8,
    pub descent_summands: u8,
    pub collection_window: u32,
    pub factor_base_points: usize,
    pub unit_trials: u64,
    pub units: usize,
    pub max_trials: u32,
    pub seed: u64,
}

/// The six §23 sizes, copied from `research/ic_tool_program/rule/v8/runs/`
/// (every `T*.params.json` of a size differs from `T01` only in its name
/// and target).  Every file has `summands` 3, `collection_aim` true,
/// `solver` `pair_table`, `max_units` 100000 and the factor base seeded
/// with the run seed.
pub const RULE_V8_ROWS: [RuleV8; 6] = [
    RuleV8 {
        n: 41,
        a: 0,
        descent_summands: 2,
        collection_window: 205,
        factor_base_points: 6560,
        unit_trials: 623,
        units: 1,
        max_trials: 1_651_584,
        seed: 201,
    },
    RuleV8 {
        n: 53,
        a: 0,
        descent_summands: 2,
        collection_window: 874,
        factor_base_points: 27984,
        unit_trials: 1014,
        units: 1,
        max_trials: 3_474_240,
        seed: 201,
    },
    RuleV8 {
        n: 57,
        a: 0,
        descent_summands: 2,
        collection_window: 228,
        factor_base_points: 7296,
        unit_trials: 181,
        units: 1,
        max_trials: 668_608,
        seed: 201,
    },
    RuleV8 {
        n: 61,
        a: 0,
        descent_summands: 2,
        collection_window: 1220,
        factor_base_points: 39040,
        unit_trials: 3504,
        units: 1,
        max_trials: 13_816_640,
        seed: 201,
    },
    RuleV8 {
        n: 47,
        a: 1,
        descent_summands: 3,
        collection_window: 164,
        factor_base_points: 5264,
        unit_trials: 164,
        units: 1,
        max_trials: 498_240,
        seed: 201,
    },
    RuleV8 {
        n: 59,
        a: 1,
        descent_summands: 2,
        collection_window: 944,
        factor_base_points: 30208,
        unit_trials: 935,
        units: 1,
        max_trials: 3_567_296,
        seed: 201,
    },
];

/// The rule v8 row for `K_a / GF(2^n)`, if there is one.
pub fn rule_v8(n: u32, a: u8) -> Option<RuleV8> {
    RULE_V8_ROWS.iter().copied().find(|r| r.n == n && r.a == a)
}

/// Everything the pipeline reads, resolved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescentParams {
    /// Collection summands; the §23 recipe is `3`.
    pub summands: u8,
    pub descent_summands: u8,
    pub collection_aim: bool,
    pub collection_window: Option<u32>,
    pub factor_base_points: usize,
    /// Seeds the factor base, the probes and the descent walks alike.
    pub seed: u64,
    pub unit_trials: u64,
    pub units: usize,
    pub max_units: usize,
    pub max_trials: u32,
}

impl DescentParams {
    /// From an `ic.koblitz_descent` method's parameters on `K_a/GF(2^n)`.
    /// `rule_v8` takes the rule v8 file's value at this size and is an
    /// error at any other; `seed = run` takes the run's algorithm seed.
    pub fn from_method(
        params: &BTreeMap<String, String>,
        n: u32,
        a: u8,
        run_seed: u64,
    ) -> Result<Self, String> {
        let row = rule_v8(n, a);
        let get = |name: &str| -> Result<&str, String> {
            params
                .get(name)
                .map(String::as_str)
                .ok_or_else(|| format!("ic.koblitz_descent needs `{name}`"))
        };
        let need_row = |name: &str| {
            row.ok_or_else(|| {
                format!(
                    "`{name}` = rule_v8 has no value on K_{a}/GF(2^{n}): the rule v8 files cover \
                     k0n41, k0n53, k0n57, k0n61, k1n47 and k1n59; give it explicitly"
                )
            })
        };
        let int = |name: &str, pick: fn(&RuleV8) -> u64| -> Result<u64, String> {
            let v = get(name)?;
            if v == RULE_V8 {
                return Ok(pick(&need_row(name)?));
            }
            v.parse()
                .map_err(|_| format!("parameter `{name}` is not an integer: `{v}`"))
        };
        let summands: u8 = get("summands")?
            .parse()
            .map_err(|_| "parameter `summands` is not an integer")?;
        if summands != 3 {
            return Err(
                "ic.koblitz_descent collects with summands = 3 (the aimed m = 3 window)".into(),
            );
        }
        let descent_summands = int("descent_summands", |r| u64::from(r.descent_summands))?;
        if !(2..=4).contains(&descent_summands) {
            return Err("descent_summands must be 2, 3 or 4".into());
        }
        let collection_aim = match get("collection_aim")? {
            "true" => true,
            "false" => false,
            other => {
                return Err(format!(
                    "collection_aim must be true or false, not `{other}`"
                ))
            }
        };
        let window = int("collection_window", |r| u64::from(r.collection_window))?;
        if window == 0 || window > u64::from(u32::MAX) {
            return Err("collection_window must be 1..2^32".into());
        }
        let seed = match get("seed")? {
            "run" => run_seed,
            _ => int("seed", |r| r.seed)?,
        };
        let max_trials = int("max_trials", |r| u64::from(r.max_trials))?;
        if max_trials == 0 || max_trials > 100_000_000 {
            return Err("max_trials must be 1..=100000000".into());
        }
        let p = Self {
            summands,
            descent_summands: descent_summands as u8,
            collection_aim,
            collection_window: Some(window as u32),
            factor_base_points: usize::try_from(int("factor_base_points", |r| {
                r.factor_base_points as u64
            })?)
            .map_err(|_| "factor_base_points does not fit usize")?,
            seed,
            unit_trials: int("unit_trials", |r| r.unit_trials)?,
            units: usize::try_from(int("units", |r| r.units as u64)?)
                .map_err(|_| "units does not fit usize")?,
            max_units: usize::try_from(int("max_units", |_| 100_000)?)
                .map_err(|_| "max_units does not fit usize")?,
            max_trials: max_trials as u32,
        };
        if p.factor_base_points == 0 || p.unit_trials == 0 || p.units == 0 {
            return Err("factor_base_points, unit_trials and units must be positive".into());
        }
        Ok(p)
    }

    /// The options `ic price` builds (`experiment::ic_options_with_descent`
    /// with the pair-table solver and the default sparse linear algebra).
    pub fn options(&self) -> KoblitzIcOptions {
        KoblitzIcOptions {
            m: self.summands as usize,
            descent_m: Some(self.descent_summands as usize),
            collection_window: self.collection_window.map(|w| w as usize),
            strategy: DecompositionStrategy::PairTable,
            collapse_negation: true,
            collapse_projected_orbits: true,
            crossbred: None,
            allow_direct_relation: false,
            max_trials: self.max_trials as usize,
            seed: self.seed,
            linear_algebra: LinearAlgebra::Sparse(SparseSolveOptions::default()),
            ..KoblitzIcOptions::default()
        }
    }
}

/// What one stage spent: its counted operations and its wall time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseTally {
    pub name: String,
    pub ops: OpCounts,
    pub wall_ns: u64,
}

/// The pipeline's own counts, the ones `ic price` reports beside them.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DescentCounts {
    pub fb_points: usize,
    pub fb_signed_orbits: usize,
    pub fb_abscissae: usize,
    pub columns: usize,
    pub selection: FactorBaseSelectionCost,
    pub pair_table_tier: String,
    pub pair_table_entries: usize,
    pub units: usize,
    pub units_planned: usize,
    pub units_extended: usize,
    pub trials: u64,
    pub summands_scanned: u64,
    pub relations_collected: usize,
    /// `[unit, relations, trials, summands_scanned]`, as `ic price`'s
    /// `per_unit_index_relations_trials_summands`.
    pub per_unit: Vec<[u64; 4]>,
    pub log_columns: usize,
    pub relations_accepted: usize,
    pub relations_rejected: usize,
    pub relations_duplicate: usize,
    pub solve_attempts: usize,
    pub sparse: Option<SparseSolveReport>,
    pub descent_trials: usize,
}

/// One run of the pipeline.
#[derive(Debug)]
pub struct DescentOutcome {
    /// The descent's logarithm when the pipeline's final check holds.
    pub recovered: Option<BigUint>,
    /// The descent returned a logarithm (checked inside the descent).
    pub descended: bool,
    /// The final `[d]G = Q` in the general arithmetic held.
    pub verified: bool,
    pub phases: Vec<PhaseTally>,
    pub counts: DescentCounts,
    /// The measurement session's snapshot, the online interval included.
    pub snapshot: Result<Snapshot, String>,
    pub factor_base: FrobeniusFactorBase,
    /// The pool's worker ran everything: one thread, the counting thread.
    pub single_thread: bool,
    /// The probe scalar `a` of every relation the log solver re-checked,
    /// in the order it saw them, and the solved column logarithms: what an
    /// independent reconstruction of the `linear_algebra` charge needs.
    pub relation_scalars: Vec<u64>,
    pub column_logs: Vec<BigUint>,
}

/// Parameters of the versioned aimed large-prime collector.  A zero arm is
/// an exact plumbing control that delegates to [`run`] without constructing
/// any large-prime state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AimedLargePrimeParams {
    pub large_primes: u32,
    pub domain_factor: f64,
    pub raw_x_cap: u64,
}

impl AimedLargePrimeParams {
    pub fn from_method(params: &BTreeMap<String, String>) -> Result<Self, String> {
        let large_primes = params
            .get("large_primes")
            .ok_or("ic.koblitz_descent_lp_v1 needs `large_primes`")?
            .parse::<u32>()
            .map_err(|_| "large_primes is not an integer")?;
        if large_primes > 2 {
            return Err("large_primes must be 0, 1 or 2".into());
        }
        let domain_factor = params
            .get("lp_domain_factor")
            .ok_or("ic.koblitz_descent_lp_v1 needs `lp_domain_factor`")?
            .parse::<f64>()
            .map_err(|_| "lp_domain_factor is not a number")?;
        if !domain_factor.is_finite() || domain_factor <= 0.0 {
            return Err("lp_domain_factor must be a positive finite number".into());
        }
        let raw_x_cap = params
            .get("lp_raw_x_cap")
            .ok_or("ic.koblitz_descent_lp_v1 needs `lp_raw_x_cap`")?
            .parse::<u64>()
            .map_err(|_| "lp_raw_x_cap is not an integer")?;
        if raw_x_cap == 0 {
            return Err("lp_raw_x_cap must be positive".into());
        }
        Ok(Self {
            large_primes,
            domain_factor,
            raw_x_cap,
        })
    }

    fn config(self) -> LargePrimeConfig {
        LargePrimeConfig {
            large_primes: self.large_primes,
            domain_factor: self.domain_factor,
            raw_x_cap: self.raw_x_cap,
            limits: LargePrimeLimits {
                max_merge_depth: 4096,
                max_row_terms: 1 << 16,
                max_pivots: 1 << 24,
            },
        }
    }
}

/// Additional, replay-stable observations of the aimed large-prime path.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AimedLargePrimeCounts {
    pub setup: BTreeMap<String, u64>,
    pub relations: BTreeMap<String, u64>,
    pub ordinary_relations: usize,
    pub projected_relations: usize,
    pub oracle_lookups: u64,
    pub oracle_canonicalisations: u64,
    pub oracle_frobenius_maps: u64,
    /// Scalar-multiplication calls made through the framework wrapper.
    /// Their underlying additions/doublings are already in the thread-local
    /// phase snapshots; only this metadata is merged into the phase ledger.
    pub wrapper_scalar_mults_setup: u64,
    pub wrapper_scalar_mults_relations: u64,
}

/// One versioned aimed-LP run.  `base` has the same whole-pipeline shape as
/// the frozen method; `large_prime` is empty for the exact LP=0 control.
#[derive(Debug)]
pub struct AimedLargePrimeOutcome {
    pub base: DescentOutcome,
    pub large_prime: AimedLargePrimeCounts,
}

#[derive(Clone, Debug)]
enum AimedRelation {
    Ordinary(CollectedRelation),
    Projected(ProjectedRelation),
}

#[derive(Clone, Debug, Default)]
struct AimedUnit {
    relations: Vec<AimedRelation>,
    trials: u64,
    summands_scanned: u64,
    /// See [`AimedLargePrimeCounts::wrapper_scalar_mults_relations`].
    wrapper_scalar_mults: u64,
}

/// Exclusive phase clocks and counter snapshots: everything between two
/// laps belongs to the phase named at the second.
struct Ledger {
    ops: OpCounts,
    at: Instant,
    phases: Vec<PhaseTally>,
}

impl Ledger {
    fn start() -> Self {
        Self {
            ops: ops_snapshot(),
            at: Instant::now(),
            phases: Vec::new(),
        }
    }
    fn lap(&mut self, name: &str) {
        self.lap_with_wrapper_scalar_mults(name, 0);
    }

    /// Merge only framework scalar-multiplication metadata. `BinaryGroup`
    /// calls the fast curve underneath, so its additions and doublings are
    /// already present in `ops_snapshot()` and must not be merged again.
    fn lap_with_wrapper_scalar_mults(&mut self, name: &str, wrapper_scalar_mults: u64) {
        let (now_ops, now) = (ops_snapshot(), Instant::now());
        let mut delta = now_ops.since(&self.ops);
        delta.scalar_mults = delta.scalar_mults.saturating_add(wrapper_scalar_mults);
        let ns = now.duration_since(self.at).as_nanos() as u64;
        match self.phases.iter_mut().find(|p| p.name == name) {
            Some(p) => {
                p.ops.merge(&delta);
                p.wall_ns += ns;
            }
            None => self.phases.push(PhaseTally {
                name: name.into(),
                ops: delta,
                wall_ns: ns,
            }),
        }
        self.ops = now_ops;
        self.at = now;
    }
}

/// The phase names, in order.
pub const PHASES: [&str; 7] = [
    "factor_base",
    "oracle_setup",
    "relations",
    "linear_algebra",
    "descent_setup",
    "descent",
    "verify",
];

/// Run the pipeline on `q` in a one-thread Rayon pool, the measurement
/// session and the counters on its worker.
pub fn run(
    kc: &KoblitzCurve,
    q: &BinaryPoint,
    p: &DescentParams,
) -> Result<DescentOutcome, String> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .thread_name(|_| "ic-koblitz-descent".into())
        .build()
        .map_err(|e| format!("one-thread pool: {e}"))?;
    pool.install(|| {
        let single = rayon::current_num_threads() == 1 && rayon::current_thread_index() == Some(0);
        let session = measurement::Session::begin().map_err(|e| e.to_string())?;
        let out = pipeline(kc, q, p);
        let snapshot = session.finish().map_err(|e| e.to_string());
        let mut out = out?;
        out.snapshot = snapshot;
        out.single_thread = single && rayon::current_thread_index() == Some(0);
        Ok(out)
    })
}

/// Run the prospective aimed large-prime method.  `large_primes = 0` is
/// intentionally the frozen implementation itself: the control does not even
/// construct the adapter, so every phase count remains byte-for-byte equal.
pub fn run_aimed_large_prime(
    instance: &BinaryInstance,
    q: &BinaryPoint,
    p: &DescentParams,
    lp: &AimedLargePrimeParams,
) -> Result<AimedLargePrimeOutcome, String> {
    let kc = instance
        .koblitz
        .as_ref()
        .ok_or("ic.koblitz_descent_lp_v1 needs a Koblitz instance")?;
    if lp.large_primes == 0 {
        return Ok(AimedLargePrimeOutcome {
            base: run(kc, q, p)?,
            large_prime: AimedLargePrimeCounts::default(),
        });
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .thread_name(|_| "ic-koblitz-descent-lp-v1".into())
        .build()
        .map_err(|e| format!("one-thread pool: {e}"))?;
    pool.install(|| {
        let single = rayon::current_num_threads() == 1 && rayon::current_thread_index() == Some(0);
        let session = measurement::Session::begin().map_err(|e| e.to_string())?;
        let out = pipeline_aimed_large_prime(instance, q, p, lp);
        let snapshot = session.finish().map_err(|e| e.to_string());
        let mut out = out?;
        out.base.snapshot = snapshot;
        out.base.single_thread = single && rayon::current_thread_index() == Some(0);
        Ok(out)
    })
}

fn raw_to_projected_columns(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
) -> Result<Vec<(usize, u64)>, String> {
    let projected = ProjectedFactorBase::new(kc, fb);
    let r = kc
        .subgroup_order
        .to_u64()
        .ok_or("projected column map needs a word-sized subgroup")?;
    let mut out = Vec::with_capacity(fb.signed_orbits.len());
    for (raw_column, orbit) in fb.signed_orbits.iter().enumerate() {
        let representative = *orbit
            .first()
            .ok_or_else(|| format!("raw signed orbit {raw_column} is empty"))?;
        let (column, shift, negated) = projected
            .location_of(representative)
            .ok_or_else(|| format!("raw signed orbit {raw_column} projects to the identity"))?;
        let coefficient = kc
            .lambda
            .modpow(&BigUint::from(shift), &kc.subgroup_order)
            .to_u64()
            .ok_or("projected coefficient does not fit u64")?;
        out.push((column, if negated { (r - coefficient) % r } else { coefficient }));
    }
    if out.len() != projected.columns()
        || out.iter().map(|(column, _)| *column).collect::<std::collections::BTreeSet<_>>().len()
            != projected.columns()
    {
        return Err("subgroup-orbit columns did not map bijectively after cofactor projection".into());
    }
    Ok(out)
}

fn projected_relation_from_raw(
    trial: u64,
    a: u64,
    raw_columns: impl IntoIterator<Item = (usize, u64)>,
    raw_to_projected: &[(usize, u64)],
    r: u64,
) -> Result<ProjectedRelation, String> {
    let mut columns: BTreeMap<usize, u64> = BTreeMap::new();
    for (raw_column, coefficient) in raw_columns {
        let &(column, scale) = raw_to_projected
            .get(raw_column)
            .ok_or_else(|| format!("large-prime row names raw column {raw_column} out of range"))?;
        let weighted = ((coefficient as u128 * scale as u128) % r as u128) as u64;
        let slot = columns.entry(column).or_insert(0);
        *slot = ((*slot as u128 + weighted as u128) % r as u128) as u64;
    }
    columns.retain(|_, coefficient| *coefficient != 0);
    if columns.is_empty() {
        return Err("large-prime elimination produced an empty projected row".into());
    }
    Ok(ProjectedRelation {
        trial,
        a,
        columns: columns.into_iter().collect(),
    })
}

fn adapt_large_prime_relation(
    trial: u64,
    probe_a: u64,
    relation: Relation,
    raw_to_projected: &[(usize, u64)],
    r: u64,
) -> Result<AimedRelation, String> {
    match relation {
        Relation::Summands(points) => {
            if points.len() != 3 {
                return Err(format!(
                    "aimed m=3 large-prime collection refuses a direct {}-summand relation",
                    points.len()
                ));
            }
            Ok(AimedRelation::Ordinary(CollectedRelation {
                trial,
                a: probe_a,
                points,
            }))
        }
        Relation::Row { columns, a, b } => {
            if b != 0 {
                return Err("factor-base precomputation received a target-bearing LP row".into());
            }
            Ok(AimedRelation::Projected(projected_relation_from_raw(
                trial,
                a,
                columns,
                raw_to_projected,
                r,
            )?))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_aimed_large_prime_unit(
    instance: &BinaryInstance,
    ctx: &InstanceCtx<BinaryGroup<'_>>,
    framework_fb: &crate::cryptanalysis::ic_boundary::FactorBase<
        crate::cryptanalysis::koblitz_fast::FastPoint,
    >,
    collector: &mut LargePrimeCollector<'_>,
    counters: &mut OracleCounters,
    unit: RelationWorkUnit,
    aimed: Option<&[u32]>,
    window: usize,
    raw_to_projected: &[(usize, u64)],
) -> Result<AimedUnit, String> {
    let mut pool: Vec<u32> = aimed.map_or_else(
        || (0..framework_fb.points.len() as u32).collect(),
        |points| {
            let mut points: Vec<u32> = points
                .iter()
                .copied()
                .filter(|&index| (index as usize) < framework_fb.points.len())
                .collect();
            points.sort_unstable();
            points.dedup();
            points
        },
    );
    if pool.is_empty() {
        pool.extend(0..framework_fb.points.len() as u32);
    }
    let scan_len = window.min(pool.len());
    if scan_len == 0 {
        return Err("aimed large-prime scan has an empty window".into());
    }
    let end = unit.start.saturating_add(unit.count);
    if end == unit.start {
        return Ok(AimedUnit::default());
    }
    let stride = probe_run_stride(unit.seed, instance.r);
    let stride_point = instance.fast.mul_u64(instance.generator, stride);
    let first_run = unit.start / PROBE_RUN;
    let last_run = end.saturating_sub(1) / PROBE_RUN;
    let mut out = AimedUnit {
        trials: end - unit.start,
        summands_scanned: (end - unit.start) * scan_len as u64,
        ..AimedUnit::default()
    };
    let mut ops = GroupOps::default();
    for run in first_run..=last_run {
        let run_start = (run * PROBE_RUN).max(unit.start);
        let run_end = ((run + 1) * PROBE_RUN).min(end);
        let mut a = walked_probe_scalar(unit.seed, run_start, instance.r);
        let mut point = instance.fast.mul_u64(instance.generator, a);
        for trial in run_start..run_end {
            if !point.infinity && a != 0 {
                let indices = aimed_window_indices(
                    &pool,
                    framework_fb.points.len(),
                    window,
                    unit.seed,
                    trial,
                )
                .ok_or("aimed large-prime scan has no valid third summands")?;
                let relations = collector.decompose_indices_lp_only(
                    ctx,
                    framework_fb,
                    &mut ops,
                    counters,
                    point,
                    a,
                    0,
                    &indices,
                );
                for relation in relations {
                    let adapted = adapt_large_prime_relation(
                        trial,
                        a,
                        relation,
                        raw_to_projected,
                        instance.r,
                    )?;
                    out.relations.push(adapted);
                }
            }
            a = ((a as u128 + stride as u128) % instance.r as u128) as u64;
            point = instance.fast.add(point, stride_point);
        }
    }
    out.wrapper_scalar_mults = ops.scalar_mults;
    Ok(out)
}

fn add_aimed_coverage(coverage: &mut Option<ColumnCoverage>, relations: &[AimedRelation]) {
    let Some(coverage) = coverage.as_mut() else {
        return;
    };
    for relation in relations {
        match relation {
            AimedRelation::Ordinary(relation) => coverage.add(std::slice::from_ref(relation)),
            AimedRelation::Projected(relation) => {
                coverage.add_projected(std::slice::from_ref(relation))
            }
        }
    }
}

fn push_aimed_solver(solver: &mut FactorBaseLogSolver<'_>, relations: &[AimedRelation]) {
    for relation in relations {
        match relation {
            AimedRelation::Ordinary(relation) => solver.push(std::slice::from_ref(relation)),
            AimedRelation::Projected(relation) => {
                solver.push_projected(std::slice::from_ref(relation))
            }
        }
    }
}

fn pipeline_aimed_large_prime(
    instance: &BinaryInstance,
    q: &BinaryPoint,
    p: &DescentParams,
    lp: &AimedLargePrimeParams,
) -> Result<AimedLargePrimeOutcome, String> {
    let kc = instance.koblitz.as_ref().ok_or("not a Koblitz instance")?;
    if kc.n >= 64 || kc.subgroup_order.bits() > 64 || kc.n.is_multiple_of(2) {
        return Err("ic.koblitz_descent_lp_v1 needs odd n < 64 and r <= 2^64".into());
    }
    let mut ledger = Ledger::start();
    let ic = p.options();
    let (fb, selection) =
        build_subgroup_orbit_factor_base_with_cost(kc, p.seed, p.factor_base_points)?;
    if fb.subspace.len() > MAX_ABSCISSAE {
        return Err(format!(
            "factor base has {} abscissae, above the materialization limit {MAX_ABSCISSAE}",
            fb.subspace.len()
        ));
    }
    let columns = projected_signed_orbit_count(kc, &fb);
    let window = p.collection_window.ok_or("aimed LP collection needs a window")? as usize;
    if window >= fb.points.len() {
        return Err(format!(
            "collection_window must be smaller than the {}-point factor base",
            fb.points.len()
        ));
    }
    ledger.lap("factor_base");

    let probes = ProbeBudget::for_run(
        kc,
        fb.points.len(),
        true,
        p.collection_window,
        p.unit_trials,
        p.units,
        p.descent_summands,
        1,
    );
    let pair = PairSumTable::build_within_for(kc, &fb, PairSumTable::DEFAULT_BYTE_BUDGET, probes)
        .ok_or("field too wide for the pair table")?;
    let framework_fb = koblitz_factor_base(
        instance,
        &fb,
        ColumnFold::SignedFrobeniusOrbit,
        "the rule-v8 subgroup-orbit base, raw signed-Frobenius columns for exact LP elimination"
            .into(),
    )
    .ok_or("could not adapt the subgroup-orbit base to the LP collector")?;
    if framework_fb.points.len() != fb.points.len()
        || framework_fb.columns != fb.signed_orbits.len()
        || framework_fb
            .points
            .iter()
            .zip(&fb.points)
            .any(|(fast, general)| *fast != instance.fast.lift(general))
    {
        return Err("LP factor-base adapter changed the Table A point order or columns".into());
    }
    let raw_to_projected = raw_to_projected_columns(kc, &fb)?;
    let group = BinaryGroup(&instance.fast);
    let target = instance.fast.lift(q);
    let ctx = InstanceCtx {
        group: &group,
        generator: instance.generator,
        target,
        r: instance.r,
        cofactor: instance.cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: Some(instance.n),
    };
    let mut setup_ops = GroupOps::default();
    let mut collector =
        LargePrimeCollector::new(instance, &ctx, &framework_fb, lp.config(), &mut setup_ops)?;
    let lp_setup = collector.setup_counters();
    let wrapper_scalar_mults_setup = setup_ops.scalar_mults;
    ledger.lap_with_wrapper_scalar_mults("oracle_setup", wrapper_scalar_mults_setup);

    let work = |unit: usize| RelationWorkUnit {
        seed: p.seed,
        start: unit as u64 * p.unit_trials,
        count: p.unit_trials,
    };
    let mut units: BTreeMap<usize, AimedUnit> = BTreeMap::new();
    let mut coverage = if p.collection_aim {
        Some(ColumnCoverage::new(kc, &fb).ok_or("factor base has no projected columns")?)
    } else {
        None
    };
    let mut oracle_counters = OracleCounters::default();
    let mut pending_wrapper_scalar_mults = 0u64;
    for unit in 0..p.units {
        let aim = coverage
            .as_ref()
            .map(ColumnCoverage::missing_points)
            .filter(|points| !points.is_empty());
        let collected = collect_aimed_large_prime_unit(
            instance,
            &ctx,
            &framework_fb,
            &mut collector,
            &mut oracle_counters,
            work(unit),
            aim.as_deref(),
            window,
            &raw_to_projected,
        )?;
        pending_wrapper_scalar_mults = pending_wrapper_scalar_mults
            .saturating_add(collected.wrapper_scalar_mults);
        add_aimed_coverage(&mut coverage, &collected.relations);
        units.insert(unit, collected);
    }
    if let Some(error) = collector.last_error() {
        return Err(format!("large-prime collector rejected an exact partial: {error}"));
    }
    ledger.lap_with_wrapper_scalar_mults("relations", pending_wrapper_scalar_mults);
    let mut solver =
        FactorBaseLogSolver::new(kc, &fb, &ic).ok_or("factor base has no projected columns")?;
    for unit in units.values() {
        push_aimed_solver(&mut solver, &unit.relations);
    }
    let mut outcome = solver.try_solve();
    ledger.lap("linear_algebra");
    let mut extended = 0usize;
    while outcome.is_none() {
        let next = units.keys().max().map_or(0, |unit| unit + 1);
        if next >= p.max_units {
            break;
        }
        let aim = coverage
            .as_ref()
            .map(ColumnCoverage::missing_points)
            .filter(|points| !points.is_empty());
        let collected = collect_aimed_large_prime_unit(
            instance,
            &ctx,
            &framework_fb,
            &mut collector,
            &mut oracle_counters,
            work(next),
            aim.as_deref(),
            window,
            &raw_to_projected,
        )?;
        ledger.lap_with_wrapper_scalar_mults("relations", collected.wrapper_scalar_mults);
        add_aimed_coverage(&mut coverage, &collected.relations);
        push_aimed_solver(&mut solver, &collected.relations);
        units.insert(next, collected);
        extended += 1;
        outcome = solver.try_solve();
        ledger.lap("linear_algebra");
        if let Some(error) = collector.last_error() {
            return Err(format!("large-prime collector rejected an exact partial: {error}"));
        }
    }
    let final_report = solver.report();
    let table = match outcome {
        Some((table, report)) if report.verified => table,
        _ => {
            return Err(format!(
                "aimed LP relations from {} units did not determine every one of {} columns",
                units.len(),
                final_report.columns
            ))
        }
    };
    ledger.lap("linear_algebra");

    let descent = IndividualLogSolver::new(kc, &fb, &table, &ic, Some(&pair))
        .ok_or("the log table does not match the base")?;
    ledger.lap("descent_setup");
    measurement::begin_online(Phase::TargetQuery);
    let report = descent.solve_report(q);
    measurement::end_online();
    ledger.lap("descent");
    let verified = report
        .log
        .as_ref()
        .is_some_and(|d| d < &kc.subgroup_order && kc.mul(kc.generator(), d) == *q);
    ledger.lap("verify");

    let relation_scalars: Vec<u64> = units
        .values()
        .flat_map(|unit| unit.relations.iter())
        .map(|relation| match relation {
            AimedRelation::Ordinary(relation) => relation.a,
            AimedRelation::Projected(relation) => relation.a,
        })
        .collect();
    let ordinary_relations = units
        .values()
        .flat_map(|unit| &unit.relations)
        .filter(|relation| matches!(relation, AimedRelation::Ordinary(_)))
        .count();
    let projected_relations = units
        .values()
        .flat_map(|unit| &unit.relations)
        .filter(|relation| matches!(relation, AimedRelation::Projected(_)))
        .count();
    let lp_relations = collector.relation_counters();
    let wrapper_scalar_mults_relations = units
        .values()
        .map(|unit| unit.wrapper_scalar_mults)
        .sum();
    let counts = DescentCounts {
        fb_points: fb.points.len(),
        fb_signed_orbits: fb.signed_orbits.len(),
        fb_abscissae: fb.subspace.len(),
        columns,
        selection,
        pair_table_tier: pair.tier().into(),
        pair_table_entries: pair.len(),
        units: units.len(),
        units_planned: p.units,
        units_extended: extended,
        trials: units.values().map(|unit| unit.trials).sum(),
        summands_scanned: units.values().map(|unit| unit.summands_scanned).sum(),
        relations_collected: ordinary_relations + projected_relations,
        per_unit: units
            .iter()
            .map(|(index, unit)| {
                [
                    *index as u64,
                    unit.relations.len() as u64,
                    unit.trials,
                    unit.summands_scanned,
                ]
            })
            .collect(),
        log_columns: final_report.columns,
        relations_accepted: final_report.relations,
        relations_rejected: final_report.rejected_relations,
        relations_duplicate: final_report.duplicate_relations,
        solve_attempts: final_report.solve_attempts,
        sparse: final_report.sparse_report.clone(),
        descent_trials: report.trials,
    };
    Ok(AimedLargePrimeOutcome {
        base: DescentOutcome {
            recovered: if verified { report.log.clone() } else { None },
            descended: report.log.is_some(),
            verified,
            phases: ledger.phases,
            counts,
            snapshot: Err("not yet finished".into()),
            factor_base: fb,
            single_thread: false,
            relation_scalars,
            column_logs: table.columns.iter().map(|(_, log)| log.clone()).collect(),
        },
        large_prime: AimedLargePrimeCounts {
            setup: lp_setup,
            relations: lp_relations,
            ordinary_relations,
            projected_relations,
            oracle_lookups: oracle_counters.lookups,
            oracle_canonicalisations: oracle_counters.canonicalisations,
            oracle_frobenius_maps: oracle_counters.frobenius_maps,
            wrapper_scalar_mults_setup,
            wrapper_scalar_mults_relations,
        },
    })
}

fn pipeline(
    kc: &KoblitzCurve,
    q: &BinaryPoint,
    p: &DescentParams,
) -> Result<DescentOutcome, String> {
    if kc.n >= 64 || kc.subgroup_order.bits() > 64 || kc.n.is_multiple_of(2) {
        return Err("ic.koblitz_descent needs odd n < 64 and r ≤ 2^64".into());
    }
    let mut ledger = Ledger::start();
    let ic = p.options();

    // 1. The factor base and the projection `ic price` charges to it.
    let (fb, selection) =
        build_subgroup_orbit_factor_base_with_cost(kc, p.seed, p.factor_base_points)?;
    if fb.subspace.len() > MAX_ABSCISSAE {
        return Err(format!(
            "factor base has {} abscissae, above the materialization limit {MAX_ABSCISSAE}",
            fb.subspace.len()
        ));
    }
    let columns = projected_signed_orbit_count(kc, &fb);
    if let Some(window) = p.collection_window {
        if window as usize >= fb.points.len() {
            return Err(format!(
                "collection_window must be smaller than the {}-point factor base",
                fb.points.len()
            ));
        }
    }
    ledger.lap("factor_base");

    // 2. The pair table at the tier the run's probing selects.
    let probes = ProbeBudget::for_run(
        kc,
        fb.points.len(),
        true,
        p.collection_window,
        p.unit_trials,
        p.units,
        p.descent_summands,
        1,
    );
    let pair = PairSumTable::build_within_for(kc, &fb, PairSumTable::DEFAULT_BYTE_BUDGET, probes)
        .ok_or("field too wide for the pair table")?;
    ledger.lap("oracle_setup");

    // 3. The planned units, aimed.
    let work = |unit: usize| RelationWorkUnit {
        seed: p.seed,
        start: unit as u64 * p.unit_trials,
        count: p.unit_trials,
    };
    let mut units: BTreeMap<usize, (Vec<CollectedRelation>, u64, u64)> = BTreeMap::new();
    {
        let collector = RelationCollector::with_pair_table(kc, &fb, &ic, Some(&pair))
            .ok_or("factor base cannot decompose with this summand count")?;
        let mut coverage = if p.collection_aim {
            Some(ColumnCoverage::new(kc, &fb).ok_or("factor base has no projected columns")?)
        } else {
            None
        };
        for u in 0..p.units {
            let aim = coverage
                .as_ref()
                .map(|cov| cov.missing_points())
                .filter(|pts| !pts.is_empty());
            let (relations, report) = collector.collect_aimed(work(u), aim.as_deref());
            if let Some(cov) = coverage.as_mut() {
                cov.add(&relations);
            }
            units.insert(
                u,
                (relations, report.trials as u64, report.summands_scanned),
            );
        }
    }
    ledger.lap("relations");

    // 4. Logs: verify, solve, and extend until determined.
    let merged: Vec<CollectedRelation> = units
        .values()
        .flat_map(|(rels, _, _)| rels.iter().cloned())
        .collect();
    let mut relation_scalars: Vec<u64> = merged.iter().map(|r| r.a).collect();
    let mut solver =
        FactorBaseLogSolver::new(kc, &fb, &ic).ok_or("factor base has no projected columns")?;
    solver.push(&merged);
    let mut outcome = solver.try_solve();
    ledger.lap("linear_algebra");
    let mut extended = 0usize;
    if outcome.is_none() {
        let collector = RelationCollector::with_pair_table(kc, &fb, &ic, Some(&pair))
            .ok_or("factor base cannot decompose with this summand count")?;
        let mut extend_coverage = if p.collection_aim {
            let mut cov =
                ColumnCoverage::new(kc, &fb).ok_or("factor base has no projected columns")?;
            cov.add(&merged);
            Some(cov)
        } else {
            None
        };
        ledger.lap("relations");
        while outcome.is_none() {
            let next = units.keys().max().map_or(0, |m| m + 1);
            if next >= p.max_units {
                break;
            }
            let aim = extend_coverage
                .as_ref()
                .map(|cov| cov.missing_points())
                .filter(|pts| !pts.is_empty());
            let (relations, report) = collector.collect_aimed(work(next), aim.as_deref());
            if let Some(cov) = extend_coverage.as_mut() {
                cov.add(&relations);
            }
            ledger.lap("relations");
            relation_scalars.extend(relations.iter().map(|r| r.a));
            solver.push(&relations);
            units.insert(
                next,
                (relations, report.trials as u64, report.summands_scanned),
            );
            extended += 1;
            outcome = solver.try_solve();
            ledger.lap("linear_algebra");
        }
    }
    let final_report = solver.report();
    let table = match outcome {
        Some((table, report)) if report.verified => table,
        _ => {
            return Err(format!(
                "relations from {} units did not determine every one of {} columns",
                units.len(),
                final_report.columns
            ))
        }
    };
    ledger.lap("linear_algebra");

    // 5. The descent's target-independent set-up.
    let descent = IndividualLogSolver::new(kc, &fb, &table, &ic, Some(&pair))
        .ok_or("the log table does not match the base")?;
    ledger.lap("descent_setup");

    // 6. The online interval: the first target-dependent operation to
    // the logarithm the descent has checked.
    measurement::begin_online(Phase::TargetQuery);
    let report = descent.solve_report(q);
    measurement::end_online();
    ledger.lap("descent");

    // 7. The pipeline's own check, in the general arithmetic.
    let verified = report
        .log
        .as_ref()
        .is_some_and(|d| d < &kc.subgroup_order && kc.mul(kc.generator(), d) == *q);
    ledger.lap("verify");

    let counts = DescentCounts {
        fb_points: fb.points.len(),
        fb_signed_orbits: fb.signed_orbits.len(),
        fb_abscissae: fb.subspace.len(),
        columns,
        selection,
        pair_table_tier: pair.tier().into(),
        pair_table_entries: pair.len(),
        units: units.len(),
        units_planned: p.units,
        units_extended: extended,
        trials: units.values().map(|u| u.1).sum(),
        summands_scanned: units.values().map(|u| u.2).sum(),
        relations_collected: units.values().map(|u| u.0.len()).sum(),
        per_unit: units
            .iter()
            .map(|(u, (rels, tr, sc))| [*u as u64, rels.len() as u64, *tr, *sc])
            .collect(),
        log_columns: final_report.columns,
        relations_accepted: final_report.relations,
        relations_rejected: final_report.rejected_relations,
        relations_duplicate: final_report.duplicate_relations,
        solve_attempts: final_report.solve_attempts,
        sparse: final_report.sparse_report.clone(),
        descent_trials: report.trials,
    };
    Ok(DescentOutcome {
        recovered: if verified { report.log.clone() } else { None },
        descended: report.log.is_some(),
        verified,
        phases: ledger.phases,
        counts,
        snapshot: Err("not yet finished".into()),
        column_logs: table.columns.iter().map(|(_, log)| log.clone()).collect(),
        factor_base: fb,
        single_thread: false,
        relation_scalars,
    })
}

/// The factor base's identity, computed as `ic.pipeline`'s is
/// (`methods::factor_base_facts`): curve, family, parameters, columns,
/// signed points and the SHA-256 of the sorted packed point keys.
pub fn factor_base_facts(
    slug: &str,
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    columns: usize,
    points: usize,
    seed: u64,
) -> Result<FactorBaseFacts, String> {
    let fast = crate::cryptanalysis::koblitz_fast::FastCurve::new(&kc.curve)
        .ok_or("field too wide for single-word arithmetic")?;
    let mut keys: Vec<u64> = fb.points.iter().map(|p| fast.lift(p).pack()).collect();
    keys.sort_unstable();
    let mut bytes = Vec::with_capacity(keys.len() * 8);
    for k in &keys {
        bytes.extend_from_slice(&k.to_be_bytes());
    }
    let points_sha256 = sha256_hex(&bytes);
    let params: BTreeMap<String, String> = [
        ("points".to_string(), points.to_string()),
        ("seed".to_string(), seed.to_string()),
    ]
    .into_iter()
    .collect();
    let family = "subgroup-orbits".to_string();
    let (fb_id, fb_sha256) = short_id(
        "FB1",
        &json!({
            "schema": "ecbench.factor_base/v1",
            "curve": slug,
            "family": family,
            "params": params,
            "columns": columns,
            "signed_points": fb.points.len(),
            "points_sha256": points_sha256,
        }),
    )?;
    Ok(FactorBaseFacts {
        fb_id,
        fb_sha256,
        family,
        params,
        description: format!(
            "subgroup_orbits: {} points in {} signed Frobenius orbits, seed {seed}",
            fb.points.len(),
            fb.signed_orbits.len()
        ),
        signed_points: fb.points.len() as u64,
        abscissae: fb.subspace.len() as u64,
        columns: columns as u64,
        dimension: None,
        points_sha256,
    })
}

/// Rebuild the factor base an `ic.koblitz_descent` run used, for a claim:
/// the facts and the points as `(x, y)` words.
pub fn rebuild_factor_base(
    slug: &str,
    kc: &KoblitzCurve,
    params: &BTreeMap<String, String>,
    run_seed: u64,
) -> Result<(FactorBaseFacts, Vec<(u64, u64)>), String> {
    let p = DescentParams::from_method(params, kc.n, kc.a, run_seed)?;
    let (fb, _) = build_subgroup_orbit_factor_base_with_cost(kc, p.seed, p.factor_base_points)?;
    let columns = fb.signed_orbits.len();
    let facts = factor_base_facts(slug, kc, &fb, columns, p.factor_base_points, p.seed)?;
    let fast = crate::cryptanalysis::koblitz_fast::FastCurve::new(&kc.curve)
        .ok_or("field too wide for single-word arithmetic")?;
    let points = fb
        .points
        .iter()
        .map(|pt| {
            let f = fast.lift(pt);
            (f.x, f.y)
        })
        .collect();
    Ok((facts, points))
}

/// The native counters of a tally under the names a report lists.
pub fn native_of(ops: &OpCounts) -> BTreeMap<String, u64> {
    [
        ("canonicalisations", ops.canonicalisations),
        ("pair_filter_probes", ops.filter_probes),
        ("pair_bucket_reads", ops.bucket_reads),
        ("frobenius_maps", ops.frobenius_maps),
    ]
    .into_iter()
    .filter(|(_, v)| *v > 0)
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

/// The descent's counts as JSON, for a report's `detail`.
pub fn detail_of(out: &DescentOutcome, p: &DescentParams) -> Value {
    let declined: u64 = out.phases.iter().map(|ph| ph.ops.declined_lanes).sum();
    json!({
        "schema": "ecbench.ic_koblitz_descent/v1",
        "recipe": "ledger §23: aimed m = 3 collection, m = descent_summands walked descent, one target",
        "params": p,
        "counts": out.counts,
        "descended": out.descended,
        "verified": out.verified,
        "single_thread": out.single_thread,
        "phase_ops": out.phases,
        // Host-dependent (AVX-512 kernels decline a degenerate block after
        // their forward pass), so kept here, out of the replayed counters.
        "vector_declined_lanes": declined,
        "recovered": out.recovered.as_ref().map(|d| d.to_u64().map_or_else(|| d.to_string(), |v| v.to_string())),
    })
}

/// The versioned aimed-LP detail block.  It keeps the frozen pipeline's
/// phase/count vocabulary and adds the exact graph and table observations.
pub fn detail_of_aimed(
    out: &AimedLargePrimeOutcome,
    p: &DescentParams,
    lp: &AimedLargePrimeParams,
) -> Value {
    let base = &out.base;
    let declined: u64 = base.phases.iter().map(|phase| phase.ops.declined_lanes).sum();
    json!({
        "schema": "ecbench.ic_koblitz_descent_lp/v1",
        "recipe": "rule-v8 aimed m=3 collection plus exact bounded large-prime elimination, walked m=descent_summands descent, one target",
        "params": p,
        "large_prime_params": lp,
        "counts": base.counts,
        "large_prime": out.large_prime,
        "descended": base.descended,
        "verified": base.verified,
        "single_thread": base.single_thread,
        "phase_ops": base.phases,
        "vector_declined_lanes": declined,
        "recovered": base.recovered.as_ref().map(|d| d.to_u64().map_or_else(|| d.to_string(), |v| v.to_string())),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::koblitz_instance;
    use crate::cryptanalysis::koblitz_index_calculus::{
        build_subgroup_orbit_factor_base, probe_run_stride, walked_probe_scalar, DESCENT_WALKS,
        PROBE_RUN,
    };
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    fn params(seed: u64, points: usize, window: u32, unit_trials: u64) -> DescentParams {
        DescentParams {
            summands: 3,
            descent_summands: 2,
            collection_aim: true,
            collection_window: Some(window),
            factor_base_points: points,
            seed,
            unit_trials,
            units: 1,
            max_units: 10_000,
            max_trials: 10_000_000,
        }
    }

    /// `(doublings, additions)` of a double-and-add ladder for `k` with no
    /// degenerate step; `(0, 0)` for `k = 0`, which returns `O` at once.
    fn ladder(k: &BigUint) -> (u64, u64) {
        if k.bits() == 0 {
            return (0, 0);
        }
        (k.bits() - 1, k.count_ones() - 1)
    }

    fn ladder_gae(k: &BigUint) -> u64 {
        let (d, a) = ladder(k);
        d + a
    }

    fn phase<'a>(out: &'a DescentOutcome, name: &str) -> &'a OpCounts {
        &out.phases
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("no phase {name}"))
            .ops
    }

    /// **Completeness of the count.**  The whole §23 pipeline at small
    /// degree, each phase's counted operations against what can be
    /// reconstructed from outside the counters:
    ///
    /// - `factor_base`: every scalar multiplication is a cofactor
    ///   multiplication, so the doublings and additions are its ladder
    ///   times the multiplications, at least one per lift;
    /// - `oracle_setup`: exactly one operation per stored folded pair
    ///   (the rows' sums), of which exactly one doubling per row (a row's
    ///   representative plus itself);
    /// - `relations`: exactly one stride and one anchor multiplication per
    ///   unit and run, and at least the anchors' and strides' ladders, one
    ///   addition per summand scanned and one per walk step; the hits'
    ///   recoveries and witness re-checks are data-dependent and are the
    ///   uncross-checked remainder;
    /// - `linear_algebra`: exactly, the re-check of every relation (two
    ///   additions and the ladder of `[a]G`) and the check of every column
    ///   logarithm (its ladder);
    /// - `descent_setup`: exactly the stride's ladder and 63 additions;
    /// - `descent`: at least the walks' start (two ladders, one addition,
    ///   63 batched additions), 64 additions per completed round, the
    ///   witness check and the recovery ladder; the hit's pair recovery is
    ///   the uncross-checked remainder;
    /// - `verify`: exactly the ladder of `[d]G`.
    #[test]
    fn every_phase_counts_at_least_its_reconstruction_and_the_log_is_right() {
        let mut sizes = Vec::new();
        for n in [23u32, 29, 31, 37] {
            for a in [0u8, 1] {
                if let Some(kc) = KoblitzCurve::new(a, n) {
                    sizes.push(kc);
                    break;
                }
            }
        }
        assert!(sizes.len() >= 2, "two small Koblitz curves");
        let mut extended_somewhere = false;
        for (i, kc) in sizes.iter().take(2).enumerate() {
            let r = kc.subgroup_order.to_u64().unwrap();
            // A unit of 8 probes cannot cover the base at once, so the
            // extension loop runs too.
            for (points, window, unit_trials, seed) in
                [(256usize, 64u32, 64u64, 7u64), (256, 48, 8, 11)]
            {
                let points = points << i;
                let d = BigUint::from(123_457u64 + 1000 * seed) % &kc.subgroup_order;
                let q = kc.mul(kc.generator(), &d);
                let p = params(seed, points, window, unit_trials);
                let out = run(kc, &q, &p).unwrap();
                let c = &out.counts;
                let what = format!("K_{}/GF(2^{}), seed {seed}", kc.a, kc.n);
                eprintln!(
                    "{what}: {} points, {} units ({} extended), {} trials, {} summands, {} pairs, descent {} trials; phases {:?}",
                    c.fb_points, c.units, c.units_extended, c.trials, c.summands_scanned,
                    c.pair_table_entries, c.descent_trials,
                    out.phases.iter().map(|p| (p.name.as_str(), p.ops.adds, p.ops.doubles, p.ops.scalar_mults)).collect::<Vec<_>>()
                );
                assert!(out.single_thread, "{what}: counted on one thread");
                assert_eq!(out.recovered.as_ref(), Some(&d), "{what}: recovered log");
                assert!(out.verified, "{what}");
                extended_somewhere |= c.units_extended > 0;

                // factor_base
                let fbo = phase(&out, "factor_base");
                let (hd, ha) = ladder(&kc.cofactor);
                assert_eq!(
                    fbo.doubles,
                    hd * fbo.scalar_mults,
                    "{what}: factor_base doublings"
                );
                assert_eq!(
                    fbo.adds,
                    ha * fbo.scalar_mults,
                    "{what}: factor_base additions"
                );
                assert!(
                    fbo.scalar_mults >= c.selection.cofactor_multiplications,
                    "{what}"
                );

                // oracle_setup
                let os = phase(&out, "oracle_setup");
                assert_eq!(c.pair_table_tier, "folded", "{what}");
                assert_eq!(
                    os.adds + os.doubles,
                    c.pair_table_entries as u64,
                    "{what}: one op per pair"
                );
                assert_eq!(
                    os.doubles, c.fb_signed_orbits as u64,
                    "{what}: one doubling per row"
                );

                // relations
                let rel = phase(&out, "relations");
                let mut mults = 0u64;
                let (mut lad_d, mut lad_a) = (0u64, 0u64);
                for &[u, _, trials, _] in &c.per_unit {
                    let start = u * p.unit_trials;
                    let end = start + trials;
                    let (d1, a1) = ladder(&BigUint::from(probe_run_stride(p.seed, r)));
                    mults += 1;
                    lad_d += d1;
                    lad_a += a1;
                    for run in start / PROBE_RUN..=(end - 1) / PROBE_RUN {
                        let run_start = (run * PROBE_RUN).max(start);
                        let a = walked_probe_scalar(p.seed, run_start, r);
                        if a != 0 {
                            mults += 1;
                        }
                        let (d2, a2) = ladder(&BigUint::from(a));
                        lad_d += d2;
                        lad_a += a2;
                    }
                }
                assert_eq!(
                    rel.scalar_mults, mults,
                    "{what}: relations scalar multiplications"
                );
                assert!(rel.doubles >= lad_d, "{what}: relations doublings");
                assert!(
                    rel.adds + rel.doubles >= lad_a + lad_d + c.summands_scanned + c.trials,
                    "{what}: relations at least scan + walk + ladders"
                );

                // linear_algebra
                let la = phase(&out, "linear_algebra");
                assert_eq!(c.relations_rejected, 0, "{what}");
                let checks: u64 = out
                    .relation_scalars
                    .iter()
                    .map(|&a| 2 + ladder_gae(&BigUint::from(a)))
                    .sum::<u64>()
                    + out.column_logs.iter().map(ladder_gae).sum::<u64>();
                assert_eq!(
                    la.adds + la.doubles,
                    checks,
                    "{what}: linear_algebra re-checks"
                );
                assert_eq!(
                    la.scalar_mults,
                    (out.relation_scalars.len() + out.column_logs.len()) as u64,
                    "{what}"
                );

                // descent_setup
                let ds = phase(&out, "descent_setup");
                let stride = (r / DESCENT_WALKS as u64).max(1 << 20);
                assert_eq!(
                    ds.adds + ds.doubles,
                    ladder_gae(&BigUint::from(stride)) + (DESCENT_WALKS as u64 - 1),
                    "{what}: descent set-up"
                );
                assert_eq!(ds.scalar_mults, 1, "{what}");

                // descent
                let de = phase(&out, "descent");
                let mut rng = StdRng::seed_from_u64(p.seed ^ 0x57_41_4c_4b_44_45_53_00);
                let a0 = rng.gen_range(1..r);
                let b = rng.gen_range(1..r);
                let rounds = (c.descent_trials as u64 - 1) / DESCENT_WALKS as u64;
                let floor = ladder_gae(&BigUint::from(a0))
                    + ladder_gae(&BigUint::from(b))
                    + 1
                    + (DESCENT_WALKS as u64 - 1)
                    + DESCENT_WALKS as u64 * rounds
                    + 1
                    + ladder_gae(&d);
                assert!(
                    de.adds + de.doubles >= floor,
                    "{what}: descent {de:?} < {floor}"
                );
                assert!(de.scalar_mults >= 3, "{what}");

                // verify
                let v = phase(&out, "verify");
                assert_eq!((v.doubles, v.adds), ladder(&d), "{what}: verify ladder");

                // The whole: at least every summand scanned and every pair stored.
                let total: u64 = out.phases.iter().map(|p| p.ops.adds + p.ops.doubles).sum();
                assert!(
                    total >= c.summands_scanned + c.pair_table_entries as u64,
                    "{what}"
                );
                // Native work is counted, not dropped.
                assert!(
                    rel.filter_probes >= c.summands_scanned,
                    "{what}: one filter probe per summand"
                );
                assert!(
                    de.filter_probes as usize >= c.descent_trials,
                    "{what}: one probe per descent trial"
                );
                assert!(
                    os.canonicalisations >= c.pair_table_entries as u64,
                    "{what}"
                );
            }
        }
        assert!(
            extended_somewhere,
            "the extension loop ran in at least one case"
        );
    }

    #[test]
    fn the_rule_v8_defaults_are_the_parameter_files() {
        let defaults: BTreeMap<String, String> = [
            ("summands", "3"),
            ("descent_summands", RULE_V8),
            ("collection_aim", "true"),
            ("collection_window", RULE_V8),
            ("factor_base_points", RULE_V8),
            ("unit_trials", RULE_V8),
            ("units", RULE_V8),
            ("max_units", "100000"),
            ("max_trials", RULE_V8),
            ("seed", RULE_V8),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
        let root = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/research/ic_tool_program/rule/v8/runs"
        );
        for row in RULE_V8_ROWS {
            let path = format!("{root}/k{}n{}/T01.params.json", row.a, row.n);
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
            let f: Value = serde_json::from_str(&text).unwrap();
            let p = DescentParams::from_method(&defaults, row.n, row.a, 0).unwrap();
            assert_eq!(f["curve"]["degree"], row.n);
            assert_eq!(f["curve"]["curve_a"], row.a);
            assert_eq!(f["summands"], u64::from(p.summands));
            assert_eq!(f["descent_summands"], u64::from(p.descent_summands));
            assert_eq!(f["collection_aim"], p.collection_aim);
            assert_eq!(
                f["collection_window"],
                u64::from(p.collection_window.unwrap())
            );
            assert_eq!(f["solver"], "pair_table");
            assert_eq!(f["seed"], p.seed);
            assert_eq!(f["max_trials"], u64::from(p.max_trials));
            assert_eq!(f["collection"]["unit_trials"], p.unit_trials);
            assert_eq!(f["collection"]["units"], p.units as u64);
            assert_eq!(f["collection"]["max_units"], p.max_units as u64);
            assert_eq!(f["factor_base"]["spec"]["kind"], "subgroup_orbits");
            assert_eq!(
                f["factor_base"]["spec"]["points"],
                p.factor_base_points as u64
            );
            assert_eq!(f["factor_base"]["spec"]["seed"], p.seed);
        }
        // Elsewhere a rule_v8 value is refused, and explicit values work.
        assert!(DescentParams::from_method(&defaults, 23, 0, 0).is_err());
        let mut explicit = defaults.clone();
        for (k, v) in [
            ("descent_summands", "2"),
            ("collection_window", "64"),
            ("factor_base_points", "256"),
            ("unit_trials", "64"),
            ("units", "1"),
            ("max_trials", "100000"),
            ("seed", "run"),
        ] {
            explicit.insert(k.into(), v.into());
        }
        assert_eq!(
            DescentParams::from_method(&explicit, 23, 0, 99)
                .unwrap()
                .seed,
            99
        );
    }

    #[test]
    fn projected_large_prime_rows_map_to_the_frozen_log_columns_and_fail_closed() {
        let kc = KoblitzCurve::new(0, 13).expect("K_0/GF(2^13)");
        let fb = build_subgroup_orbit_factor_base(&kc, 7, 80).expect("small subgroup base");
        let map = raw_to_projected_columns(&kc, &fb).expect("bijective projected map");
        let r = kc.subgroup_order.to_u64().unwrap();
        let mut rows = Vec::new();
        for (raw_column, orbit) in fb.signed_orbits.iter().enumerate() {
            let point = &fb.points[orbit[0]];
            let a = (1..r)
                .find(|a| kc.mul(kc.generator(), &BigUint::from(*a)) == *point)
                .expect("toy subgroup logarithm for a mapping test");
            rows.push(
                projected_relation_from_raw(0, a, [(raw_column, 1)], &map, r)
                    .expect("mapped sparse row"),
            );
        }
        let mut opts = params(7, 80, 16, 32).options();
        opts.linear_algebra = LinearAlgebra::Sparse(SparseSolveOptions::default());
        let mut solver = FactorBaseLogSolver::new(&kc, &fb, &opts).unwrap();
        solver.push_projected(&rows);
        assert_eq!(solver.report().rejected_relations, 0);
        assert_eq!(solver.relations(), solver.columns());
        let (table, report) = solver.try_solve().expect("full-rank diagonal rows");
        assert!(report.verified && report.sparse);
        assert!(report.sparse_report.is_some());
        assert!(table.verify(&kc));

        let mut forged = rows[0].clone();
        forged.columns[0].1 = (forged.columns[0].1 + 1) % r;
        let mut reject = FactorBaseLogSolver::new(&kc, &fb, &opts).unwrap();
        reject.push_projected(&[forged]);
        assert_eq!(reject.relations(), 0);
        assert_eq!(reject.report().rejected_relations, 1);
    }

    /// The nonzero LP adapter deliberately has its own Frobenius-folded pair
    /// table, but it must not change which walked probes have an ordinary
    /// aimed m=3 decomposition.  A different exact witness for the same probe
    /// is harmless (and is independently replayed); gaining or losing an
    /// ordinary hit would confound graph closure with a changed decomposition
    /// oracle.  This fixture therefore compares the complete ordinary-hit
    /// census and rechecks every witness equation before considering any
    /// graph-produced projected row.
    #[test]
    fn aimed_lp_adapter_preserves_the_frozen_ordinary_three_sum_opportunities() {
        let inst = koblitz_instance(0, 13).expect("K_0/GF(2^13)");
        let kc = inst.koblitz.as_ref().unwrap();
        let p = params(11, 80, 16, 256);
        let (fb, _) = build_subgroup_orbit_factor_base_with_cost(
            kc,
            p.seed,
            p.factor_base_points,
        )
        .expect("toy subgroup base");
        let probes = ProbeBudget::for_run(
            kc,
            fb.points.len(),
            true,
            p.collection_window,
            p.unit_trials,
            p.units,
            p.descent_summands,
            1,
        );
        let pair = PairSumTable::build_within_for(
            kc,
            &fb,
            PairSumTable::DEFAULT_BYTE_BUDGET,
            probes,
        )
        .expect("frozen aimed pair table");
        let options = p.options();
        let ordinary = RelationCollector::with_pair_table(kc, &fb, &options, Some(&pair))
            .expect("frozen aimed collector");
        let aimed: Vec<u32> = (0..16).collect();
        let unit = RelationWorkUnit {
            seed: p.seed,
            start: 0,
            count: p.unit_trials,
        };
        let frozen = ordinary.collect_aimed(unit, Some(&aimed)).0;

        let framework_fb = koblitz_factor_base(
            &inst,
            &fb,
            ColumnFold::SignedFrobeniusOrbit,
            "ordinary-stream parity fixture".into(),
        )
        .expect("framework view of the same base");
        let raw_to_projected = raw_to_projected_columns(kc, &fb).unwrap();
        let group = BinaryGroup(&inst.fast);
        let q = inst.fast.mul_u64(inst.generator, 777 % inst.r);
        let ctx = InstanceCtx {
            group: &group,
            generator: inst.generator,
            target: q,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(inst.n),
        };
        let mut setup = GroupOps::default();
        let mut lp = LargePrimeCollector::new(
            &inst,
            &ctx,
            &framework_fb,
            AimedLargePrimeParams {
                large_primes: 2,
                domain_factor: 3.0,
                raw_x_cap: 1 << 13,
            }
            .config(),
            &mut setup,
        )
        .expect("LP adapter");
        let mut oracle_counters = OracleCounters::default();
        let adapted = collect_aimed_large_prime_unit(
            &inst,
            &ctx,
            &framework_fb,
            &mut lp,
            &mut oracle_counters,
            unit,
            Some(&aimed),
            p.collection_window.unwrap() as usize,
            &raw_to_projected,
        )
        .expect("adapted aimed unit");

        let frozen_by_trial: BTreeMap<u64, &CollectedRelation> =
            frozen.iter().map(|relation| (relation.trial, relation)).collect();
        let adapted_by_trial: BTreeMap<u64, &CollectedRelation> = adapted
            .relations
            .iter()
            .filter_map(|relation| match relation {
                AimedRelation::Ordinary(relation) => Some((relation.trial, relation)),
                AimedRelation::Projected(_) => None,
            })
            .collect();
        assert!(!frozen_by_trial.is_empty(), "fixture must exercise ordinary hits");
        assert_eq!(
            adapted_by_trial.keys().collect::<Vec<_>>(),
            frozen_by_trial.keys().collect::<Vec<_>>(),
            "LP graph support must add projected rows without changing ordinary m=3 hit opportunities"
        );
        for (trial, relation) in adapted_by_trial {
            let baseline = frozen_by_trial[&trial];
            assert_eq!(relation.a, baseline.a, "trial {trial} probe scalar");
            assert_eq!(relation.points.len(), 3, "trial {trial} remains m=3");
            let sum = relation
                .points
                .iter()
                .fold(BinaryPoint::Infinity, |sum, &index| {
                    kc.add(&sum, &fb.points[index])
                });
            assert_eq!(
                sum,
                kc.mul(kc.generator(), &BigUint::from(relation.a)),
                "trial {trial} adapted ordinary witness equation"
            );
        }
    }

    #[test]
    fn lp_zero_is_the_frozen_pipeline_and_two_lp_recovers_a_toy_target() {
        let inst = koblitz_instance(0, 13).expect("K_0/GF(2^13)");
        let kc = inst.koblitz.as_ref().unwrap();
        let p = params(11, 80, 16, 32);
        let d = BigUint::from(777u64) % &kc.subgroup_order;
        let q = kc.mul(kc.generator(), &d);
        let plain = run(kc, &q, &p).expect("frozen run");
        let zero = run_aimed_large_prime(
            &inst,
            &q,
            &p,
            &AimedLargePrimeParams {
                large_primes: 0,
                domain_factor: 1.0,
                raw_x_cap: 1 << 13,
            },
        )
        .expect("LP=0 control");
        assert_eq!(zero.base.recovered, plain.recovered);
        assert_eq!(zero.base.counts, plain.counts);
        assert_eq!(zero.base.relation_scalars, plain.relation_scalars);
        assert_eq!(zero.base.column_logs, plain.column_logs);
        assert_eq!(zero.base.phases.len(), plain.phases.len());
        for (control, baseline) in zero.base.phases.iter().zip(&plain.phases) {
            assert_eq!(control.name, baseline.name);
            assert_eq!(control.ops, baseline.ops, "{}", control.name);
        }
        assert_eq!(zero.large_prime, AimedLargePrimeCounts::default());

        let two = run_aimed_large_prime(
            &inst,
            &q,
            &p,
            &AimedLargePrimeParams {
                large_primes: 2,
                domain_factor: 3.0,
                raw_x_cap: 1 << 13,
            },
        )
        .expect("2LP toy run");
        assert_eq!(two.base.recovered.as_ref(), Some(&d));
        assert!(two.base.verified);
        assert!(two.large_prime.projected_relations > 0);
        assert!(two.large_prime.ordinary_relations < two.base.counts.log_columns);
        assert_eq!(
            two.large_prime.projected_relations as u64,
            two.large_prime
                .relations
                .get("lp_merged_relations")
                .copied()
                .unwrap_or(0),
            "every admitted projected row is produced by graph elimination"
        );
        assert_eq!(
            two.base.counts.relations_collected,
            two.large_prime.ordinary_relations + two.large_prime.projected_relations
        );
        assert!(
            two.large_prime
                .relations
                .get("lp_partials_fed")
                .copied()
                .unwrap_or(0)
                > 0
        );
        assert!(
            two.large_prime
                .relations
                .get("lp_merged_relations")
                .copied()
                .unwrap_or(0)
                > 0
        );
        assert_eq!(
            two.large_prime
                .relations
                .get("lp_direct_two_sum")
                .copied(),
            Some(0)
        );
        assert_eq!(
            two.large_prime
                .relations
                .get("lp_verification_failures")
                .copied(),
            Some(0)
        );
    }
}
