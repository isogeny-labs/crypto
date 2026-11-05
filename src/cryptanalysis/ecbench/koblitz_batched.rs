//! # `ic.koblitz_batched`: the candidate tournament worker's pair-table
//! pipeline, charged in the counted unit.
//!
//! `examples/ic_tournament_worker.rs` is the pipeline the native IC
//! tournament (`icprog tournament`) measures in Valgrind instruction reads:
//! a subgroup-orbit factor base, the signed-Frobenius pair table, relation
//! collection in batches of `batch_trials` queries with a rank check
//! (`FactorBaseLogSolver::try_solve`) after every batch, and the individual
//! logarithm of one public target.  Its `batch_trials` has no analogue in
//! `ic.pipeline` (which eliminates incrementally, one trial at a time) or
//! in `ic.koblitz_descent` (which collects in planned, aimed units), so a
//! batch-size question cannot be asked of either.  This method makes the
//! same library calls the worker makes, in the same order, so that the
//! worker's candidates can be measured by `ecbench` on an isolated host
//! (L2, replay and an A/A control), which a tournament round cannot give.
//!
//! The stages, with the worker's calls:
//!
//! 1. **factor_base** — [`FactorBaseSpec::SubgroupOrbits`] materialised on
//!    the curve (seed `fb_seed`, `factor_base_points` points);
//! 2. **oracle_setup** — [`PairSumTable::build`];
//! 3. **relations** — [`RelationCollector::collect_observed`] on
//!    [`RelationWorkUnit`]s of `batch_trials` queries starting at the
//!    trials already drawn;
//! 4. **linear_algebra** — [`FactorBaseLogSolver::push`] and `try_solve`
//!    after each batch, then the certificate check the worker makes
//!    (`rejected_relations == 0`, [`FactorBaseLogTable::verify`]);
//! 5. **descent_setup** — [`IndividualLogSolver::new`];
//! 6. **descent** — [`IndividualLogSolver::solve_observed`], the online
//!    interval;
//! 7. **verify** — the final `[d]G = Q` in the general arithmetic.
//!
//! The options are the worker's: `KoblitzIcOptions::default()` with `m`,
//! `seed` and `max_trials` set, the pair-table strategy, sparse linear
//! algebra, no direct relations.  **Charged in the counted unit** as
//! [`koblitz_descent`](super::koblitz_descent) is: each stage by the
//! difference of two [`ops_snapshot`]s around it, the whole pipeline on
//! one thread inside a one-thread Rayon pool that also holds the
//! measurement session.

use std::collections::BTreeMap;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::binary_ecc::BinaryPoint;
use crate::cryptanalysis::ecbench::koblitz_descent::{DescentCounts, DescentOutcome, PhaseTally};
use crate::cryptanalysis::group_op_count::{ops_snapshot, OpCounts};
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase};
use crate::cryptanalysis::koblitz_factor_base_search::FactorBaseSpec;
use crate::cryptanalysis::koblitz_index_calculus::{
    projected_signed_orbit_count, CollectedRelation, DecompositionStrategy, FactorBaseLogSolver,
    IndividualLogSolver, KoblitzCurve, KoblitzIcOptions, LinearAlgebra, PairSumTable,
    RelationCollector, RelationWorkUnit,
};
use crate::cryptanalysis::koblitz_sparse_la::SparseSolveOptions;

/// The phase names, in order; the same seven `ic.koblitz_descent` reports.
pub const PHASES: [&str; 7] = [
    "factor_base",
    "oracle_setup",
    "relations",
    "linear_algebra",
    "descent_setup",
    "descent",
    "verify",
];

/// The worker's limits on a collection, kept so the method refuses what the
/// worker refuses.
const MAX_BATCH_TRIALS: u64 = 4096;
const MAX_TRIALS: u64 = 65536;

/// The parameters of one `ic.koblitz_batched` run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchedParams {
    /// Points of the subgroup-orbit factor base.
    pub factor_base_points: usize,
    /// Seeds the factor base (the tournament registers `43`).
    pub fb_seed: u64,
    /// Queries collected before each rank check.
    pub batch_trials: u64,
    /// Cap on the queries drawn in all.
    pub max_trials: u64,
    /// Collection summands (`m`).
    pub summands: usize,
    /// Seeds the probes and the descent; `run` takes the run's algorithm seed.
    pub seed: u64,
}

impl BatchedParams {
    /// From an `ic.koblitz_batched` method's parameters.
    pub fn from_method(params: &BTreeMap<String, String>, run_seed: u64) -> Result<Self, String> {
        let get = |name: &str| -> Result<&str, String> {
            params
                .get(name)
                .map(String::as_str)
                .ok_or_else(|| format!("ic.koblitz_batched needs `{name}`"))
        };
        let int = |name: &str| -> Result<u64, String> {
            get(name)?
                .parse()
                .map_err(|_| format!("parameter `{name}` is not an integer: `{}`", params[name]))
        };
        let seed = match get("seed")? {
            "run" => run_seed,
            _ => int("seed")?,
        };
        let p = Self {
            factor_base_points: usize::try_from(int("factor_base_points")?)
                .map_err(|_| "factor_base_points does not fit usize")?,
            fb_seed: int("fb_seed")?,
            batch_trials: int("batch_trials")?,
            max_trials: int("max_trials")?,
            summands: usize::try_from(int("summands")?)
                .map_err(|_| "summands does not fit usize")?,
            seed,
        };
        if p.factor_base_points == 0 {
            return Err("factor_base_points must be positive".into());
        }
        if p.batch_trials == 0
            || p.batch_trials > MAX_BATCH_TRIALS
            || p.max_trials > MAX_TRIALS
            || p.max_trials < p.batch_trials
            || !(2..=4).contains(&p.summands)
        {
            return Err(format!(
                "invalid collection limits: batch_trials 1..={MAX_BATCH_TRIALS}, \
                 batch_trials <= max_trials <= {MAX_TRIALS}, summands 2..=4"
            ));
        }
        Ok(p)
    }

    /// The options the worker builds for its `pair_table` / `sparse` jobs.
    pub fn options(&self) -> KoblitzIcOptions {
        KoblitzIcOptions {
            m: self.summands,
            seed: self.seed,
            max_trials: self.max_trials as usize,
            strategy: DecompositionStrategy::PairTable,
            linear_algebra: LinearAlgebra::Sparse(SparseSolveOptions::default()),
            allow_direct_relation: false,
            ..KoblitzIcOptions::default()
        }
    }

    /// The factor base the tournament's jobs name.
    pub fn factor_base_spec(&self) -> FactorBaseSpec {
        FactorBaseSpec::SubgroupOrbits {
            seed: self.fb_seed,
            points: self.factor_base_points,
        }
    }
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

/// Run the pipeline on `q` in a one-thread Rayon pool, the measurement
/// session and the counters on its worker.
pub fn run(
    kc: &KoblitzCurve,
    q: &BinaryPoint,
    p: &BatchedParams,
) -> Result<DescentOutcome, String> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .thread_name(|_| "ic-koblitz-batched".into())
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
    p: &BatchedParams,
) -> Result<DescentOutcome, String> {
    if kc.n >= 64 || kc.subgroup_order.bits() > 64 || kc.n.is_multiple_of(2) {
        return Err("ic.koblitz_batched needs odd n < 64 and r <= 2^64".into());
    }
    let mut ledger = Ledger::start();
    let opts = p.options();

    // 1. The factor base, as the tournament jobs name it.
    let fb = p.factor_base_spec().materialize(kc)?;
    let columns = projected_signed_orbit_count(kc, &fb);
    ledger.lap("factor_base");

    // 2. The pair table.
    let pair = PairSumTable::build(kc, &fb).ok_or("pair table unavailable")?;
    ledger.lap("oracle_setup");

    // 3-4. Collect `batch_trials` queries, push them, try to solve: the
    // worker's loop.
    let collector = RelationCollector::with_pair_table(kc, &fb, &opts, Some(&pair))
        .ok_or("factor base cannot decompose with this summand count")?;
    let mut system =
        FactorBaseLogSolver::new(kc, &fb, &opts).ok_or("factor base has no projected columns")?;
    let mut trials = 0u64;
    let mut relations: Vec<CollectedRelation> = Vec::new();
    let mut per_batch: Vec<[u64; 4]> = Vec::new();
    let mut summands_scanned = 0u64;
    let mut outcome = None;
    while trials < p.max_trials && outcome.is_none() {
        let count = p.batch_trials.min(p.max_trials - trials);
        let (rows, report) = collector.collect_observed(RelationWorkUnit {
            seed: p.seed,
            start: trials,
            count,
        });
        ledger.lap("relations");
        per_batch.push([
            per_batch.len() as u64,
            rows.len() as u64,
            report.trials as u64,
            report.summands_scanned,
        ]);
        summands_scanned += report.summands_scanned;
        trials += count;
        system.push(&rows);
        outcome = system.try_solve();
        relations.extend(rows);
        ledger.lap("linear_algebra");
    }
    let final_report = system.report();
    let Some((table, solved)) = outcome else {
        return Err(format!(
            "{trials} queries ({} relations) did not determine every one of {} columns",
            relations.len(),
            final_report.columns
        ));
    };
    if solved.rejected_relations != 0 || !table.verify(kc) {
        return Err("invalid relation or factor-base log certificate".into());
    }
    ledger.lap("linear_algebra");

    // 5. The descent's target-independent set-up.
    let descent = IndividualLogSolver::new(kc, &fb, &table, &opts, Some(&pair))
        .ok_or("the log table does not match the base")?;
    ledger.lap("descent_setup");

    // 6. The online interval.
    measurement::begin_online(Phase::TargetQuery);
    let report = descent.solve_observed(q);
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
        pair_table_tier: pair.tier().into(),
        pair_table_entries: pair.len(),
        units: per_batch.len(),
        units_planned: p.max_trials.div_ceil(p.batch_trials) as usize,
        units_extended: 0,
        trials,
        summands_scanned,
        relations_collected: relations.len(),
        per_unit: per_batch,
        log_columns: final_report.columns,
        relations_accepted: final_report.relations,
        relations_rejected: final_report.rejected_relations,
        relations_duplicate: final_report.duplicate_relations,
        solve_attempts: final_report.solve_attempts,
        sparse: final_report.sparse_report.clone(),
        descent_trials: report.trials,
        ..DescentCounts::default()
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
        relation_scalars: relations.iter().map(|r| r.a).collect(),
    })
}

/// The run's counts as JSON, for a report's `detail`.
pub fn detail_of(out: &DescentOutcome, p: &BatchedParams) -> Value {
    let declined: u64 = out.phases.iter().map(|ph| ph.ops.declined_lanes).sum();
    json!({
        "schema": "ecbench.ic_koblitz_batched/v1",
        "recipe": "tournament worker: subgroup-orbit base, pair table, batch_trials collection with a rank check per batch, one-target descent",
        "params": p,
        "counts": out.counts,
        "descended": out.descended,
        "verified": out.verified,
        "single_thread": out.single_thread,
        "phase_ops": out.phases,
        "vector_declined_lanes": declined,
        "recovered": out.recovered.as_ref().map(|d| d.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use num_bigint::BigUint;

    /// One public job of round 0026 (`research/ic_candidate_tournament_20260915/
    /// runs/round-0026/fixtures.json`), with what `examples/ic_tournament_worker.rs`
    /// itself reports for it, run from the worker's own binary.
    struct Cell {
        name: &'static str,
        a: u8,
        n: u32,
        /// The public target.
        target: (&'static str, &'static str),
        /// The job's algorithm seed.
        seed: u64,
        /// The job's subgroup-orbit factor base (seed 43 in every cell).
        points: usize,
        /// The scalar the worker recovers, and the projected columns.
        scalar: &'static str,
        columns: usize,
        /// `(batch_trials, queries drawn, relations accepted)`.  Every
        /// cell is solved on its first solve attempt, with no rejected or
        /// duplicate relation.
        runs: &'static [(u64, u64, usize)],
    }

    const CELLS: &[Cell] = &[
        Cell {
            name: "n13a0-000",
            a: 0,
            n: 13,
            target: ("2388", "192"),
            seed: 4_753_082_171_431_670_784,
            points: 78,
            scalar: "33",
            columns: 7,
            runs: &[(64, 64, 64), (16, 16, 16), (4, 8, 8), (2, 8, 8)],
        },
        Cell {
            name: "n17a1-000",
            a: 1,
            n: 17,
            target: ("72229", "22981"),
            seed: 1_904_932_568_182_942_595,
            points: 102,
            scalar: "26461",
            columns: 8,
            runs: &[(64, 64, 64), (16, 16, 16), (4, 8, 8), (2, 8, 8)],
        },
        Cell {
            name: "n19a0-000",
            a: 0,
            n: 19,
            target: ("257541", "143934"),
            seed: 13_349_067_008_857_821_907,
            points: 114,
            scalar: "124122",
            columns: 8,
            runs: &[(64, 64, 64), (16, 16, 16), (4, 8, 8), (2, 8, 8)],
        },
        // The registered round's holdout cell (confirmation stage).
        Cell {
            name: "n19a1-000",
            a: 1,
            n: 19,
            target: ("514088", "475569"),
            seed: 17_260_057_922_115_205_155,
            points: 114,
            scalar: "218833",
            columns: 8,
            runs: &[(64, 64, 64), (16, 16, 16)],
        },
    ];

    fn params(cell: &Cell, batch_trials: u64) -> BatchedParams {
        BatchedParams {
            factor_base_points: cell.points,
            fb_seed: 43,
            batch_trials,
            max_trials: 4096,
            summands: 3,
            seed: cell.seed,
        }
    }

    fn run_cell(cell: &Cell, batch_trials: u64) -> DescentOutcome {
        let kc = KoblitzCurve::new(cell.a, cell.n).expect("Koblitz curve");
        let coord = |s: &str| F2mElement::from_biguint(&s.parse::<BigUint>().unwrap(), cell.n);
        let q = BinaryPoint::Affine {
            x: coord(cell.target.0),
            y: coord(cell.target.1),
        };
        assert!(kc.curve.is_on_curve(&q), "{}", cell.name);
        run(&kc, &q, &params(cell, batch_trials)).expect("the pipeline runs")
    }

    /// **Same algorithm as the tournament worker.**  Each row is what the
    /// worker's own binary reports for the cell's job (`mode` `ic`,
    /// `pair_table`, `sparse`, `max_trials` 4096) at that `batch_trials`:
    /// the queries drawn, the relations accepted, the columns, one solve
    /// attempt, and the recovered scalar.  A drift here means this method
    /// no longer measures what the tournament measured.
    ///
    /// Batches 4 and 2 draw eight queries, the fewest that cover the
    /// columns, in several batches: the rank check runs every batch but a
    /// solve is attempted only once the relations can cover the columns.
    /// Batches 64 and 16 draw one whole batch, so `batch_trials` moves the
    /// surplus collected after the columns are covered (round 0026's
    /// `batch16`).
    #[test]
    fn it_reproduces_the_tournament_workers_counts_on_four_round_0026_cells() {
        for cell in CELLS {
            for &(batch, trials, relations) in cell.runs {
                let at = format!("{} batch_trials {batch}", cell.name);
                let out = run_cell(cell, batch);
                assert!(out.single_thread, "{at}");
                assert!(out.verified, "{at}");
                assert_eq!(
                    out.recovered.as_ref().map(ToString::to_string).as_deref(),
                    Some(cell.scalar),
                    "{at}"
                );
                assert_eq!(out.counts.trials, trials, "{at}");
                assert_eq!(out.counts.relations_collected, relations, "{at}");
                assert_eq!(out.counts.relations_accepted, relations, "{at}");
                assert_eq!(out.counts.relations_rejected, 0, "{at}");
                assert_eq!(out.counts.relations_duplicate, 0, "{at}");
                assert_eq!(out.counts.solve_attempts, 1, "{at}");
                assert_eq!(out.counts.log_columns, cell.columns, "{at}");
                let names: Vec<&str> = out.phases.iter().map(|p| p.name.as_str()).collect();
                assert_eq!(names, PHASES, "{at}");
            }
        }
    }

    /// The count is a function of the inputs: a second run counts the same
    /// group operations in every phase.
    #[test]
    fn it_counts_deterministically() {
        let cell = &CELLS[0];
        let a = run_cell(cell, 16);
        let b = run_cell(cell, 16);
        for (x, y) in a.phases.iter().zip(&b.phases) {
            assert_eq!(x.name, y.name);
            assert_eq!(x.ops.adds, y.ops.adds, "{}", x.name);
            assert_eq!(x.ops.doubles, y.ops.doubles, "{}", x.name);
            assert_eq!(x.ops.scalar_mults, y.ops.scalar_mults, "{}", x.name);
        }
    }
}
