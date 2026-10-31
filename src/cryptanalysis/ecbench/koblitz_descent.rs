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
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase, Snapshot};
use crate::cryptanalysis::koblitz_index_calculus::{
    build_subgroup_orbit_factor_base_with_cost, projected_signed_orbit_count, CollectedRelation,
    ColumnCoverage, DecompositionStrategy, FactorBaseLogSolver, FactorBaseSelectionCost,
    FrobeniusFactorBase, IndividualLogSolver, KoblitzCurve, KoblitzIcOptions, LinearAlgebra,
    PairSumTable, ProbeBudget, RelationCollector, RelationWorkUnit,
};
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
        let (now_ops, now) = (ops_snapshot(), Instant::now());
        let delta = now_ops.since(&self.ops);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::koblitz_index_calculus::{
        probe_run_stride, walked_probe_scalar, DESCENT_WALKS, PROBE_RUN,
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
}
