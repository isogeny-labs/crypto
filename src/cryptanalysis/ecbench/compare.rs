//! Comparing two arms.
//!
//! **Operations** are the metric (AGENTS.md §2, §6).  Runs are matched by
//! `(workload, round)`: within one session the two arms of a round ran
//! the same workload under the same algorithm seed, and across two
//! sessions of one spec they did too.  The ratio is
//!
//! ```text
//!   B / A  =  Σ S_B / Σ S_A        over the matched pairs
//! ```
//!
//! which is AGENTS.md §8's `baseline_total / candidate_total` in `S`: a
//! ratio of totals, never a mean of ratios, which is biased whenever the
//! arms spread differently (rho's cost varies with its seed, BSGS's does
//! not).
//!
//! *Interval (schema v2).*  The primary `ci95` is the linearised
//! (delta-method) interval of the ratio of totals: workloads are the
//! clusters, `z_k = (B_k − R·A_k) / mean(A)` their influence values, the
//! standard error `sd(z)/√K` and the quantile Student `t(K−1)`, the interval
//! read on the log scale (`R·exp(±t·se/R)`, positive by construction).  A
//! percentile two-stage bootstrap of the same statistic covers 85 % at two
//! workloads and about 93 % at three or four (DEEP_DIVE §2.1); the
//! linearised interval covers 94–96 % at every `K` (a Rust Monte-Carlo test
//! in `stats.rs` pins that).  The bootstrap interval is kept beside it as
//! `ci95_bootstrap`.  Below five workloads the interval is `indicative`;
//! with one workload the clusters are the pairs and the interval says
//! nothing about other targets (`interval_scope = within_workload`).
//!
//! *Survivorship.*  A run that timed out or exhausted its budget is
//! right-censored, not dropped: an exhausted run enters at the cost it
//! spent, a timed-out one at the cost its wall time implies (a lower
//! bound, `0` when the arm has no verified run to convert wall time
//! with).  With any censored pair the ratio is a *bound* (`bound = lower`
//! when only B is censored, `upper` when only A is, `indeterminate` when
//! both are) and `censored_pairs` counts them.
//!
//! *Pricing band.*  AGENTS.md §5a: the unpriced native work of every arm
//! is priced at 0, 0.1 and 1 GAE per operation and the order is reported
//! only where it holds at every weight (`ops.band`).
//!
//! **Wall time** is a practicality note.  It is compared only inside one
//! session (same host class, interleaved), only over pairs where both
//! runs earned the spec's `isolation_required`, and only with at least five
//! such pairs.  The statistic is the cost per operation: each arm's
//! `Σ solve_wall / Σ total_gae` over the admitted pairs and its ratio B/A,
//! with the same linearised `t` interval (pairs are the timing replicates).
//! The per-pair median ratio of v1 carries rho's work variance and is a
//! different estimand from the ops ratio; it stays as `median_ratio` for
//! compatibility and is not the headline.  The ratio of total wall times
//! is the secondary "end-to-end" figure.  The A/A arm (preferably the pair
//! that includes arm A) estimates the position bias; the admitted wall
//! result divides it out and `outside_noise` asks whether the corrected
//! interval excludes 1.  The v1 criterion (non-overlap of two median
//! intervals) is kept as `legacy_outside_noise`.
//!
//! A comparison is refused, not qualified, when the arms ran different
//! workloads or units, or when a session is not `complete` (unless
//! `--allow-interrupted`, which is recorded).  It is `incomplete` when a
//! measured run of either arm did not verify, `partial` when some runs have
//! no partner, and `bounded` when either arm left work unpriced.
//! Classifying a change as advance, engineering, relabelling or accounting
//! (AGENTS.md §3) stays with the author.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::cryptanalysis::ecbench::band::{priced_s, unpriced_counts, DEFAULT_WEIGHTS};
use crate::cryptanalysis::ecbench::canonical::short_id;
use crate::cryptanalysis::ecbench::record::Record;
use crate::cryptanalysis::ecbench::runner::{read_records, read_session, Session};
use crate::cryptanalysis::ecbench::spec::{Level, Spec};
use crate::cryptanalysis::ecbench::stats::{
    bootstrap_ci, cluster_bootstrap_ci, divide_independent, influence_se, linearised_ratio, mean,
    median, Linearised,
};

/// The v1 schema name.  Kept under this name for the readers that match on
/// it (`db`, `calibration`); new comparisons are [`COMPARISON_SCHEMA_V2`].
pub const COMPARISON_SCHEMA: &str = "ecbench.comparison/v1";
/// Linearised primary interval, per-operation wall statistic, bias-corrected
/// noise test, censoring bounds, pricing band, tail diagnostics.
pub const COMPARISON_SCHEMA_V2: &str = "ecbench.comparison/v2";

/// Either comparison schema.
pub fn is_comparison_schema(s: &str) -> bool {
    s == COMPARISON_SCHEMA || s == COMPARISON_SCHEMA_V2
}

/// Wall-time and A/A figures need at least this many admitted pairs.
pub const MIN_WALL_PAIRS: usize = 5;

/// An interval over fewer workloads than this is `indicative`.
pub const INDICATIVE_BELOW_WORKLOADS: usize = 5;

/// An arm whose largest run is more than this share of its total S has a
/// heavy tail: the mean is carried by one run.
pub const HEAVY_TAIL_SHARE: f64 = 0.25;

/// What the measured arm's tail looks like (verified runs only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TailDiagnostics {
    pub runs: u64,
    pub median_s: f64,
    pub max_s: f64,
    pub max_over_median: Option<f64>,
    /// The largest run's share of `Σ S` over the arm.
    pub top_run_share: f64,
    pub top_run_id: String,
    pub heavy_tail: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmSummary {
    pub session_id: String,
    pub arm: String,
    pub method_id: String,
    pub method: String,
    pub measured: u64,
    pub verified: u64,
    pub statuses: BTreeMap<String, u64>,
    pub mean_s: Option<f64>,
    pub mean_ratio_to_floor: Option<f64>,
    pub lower_bound: bool,
    pub deterministic: bool,
    /// Measured runs that were right-censored (timeout or exhausted).
    #[serde(default)]
    pub censored: u64,
    /// Max/median S and the top run's share (AGENTS.md §5 heavy tails).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<TailDiagnostics>,
    /// Every unpriced counter of the arm with its count summed over the
    /// verified runs; `None` where some run names the counter and carries
    /// no count (the arm cannot be ranked above weight 0).
    #[serde(default)]
    pub unpriced_counts: BTreeMap<String, Option<u64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadRow {
    pub workload_id: String,
    pub slug: String,
    #[serde(with = "crate::cryptanalysis::ecbench::canonical::compat_u128")]
    pub r: u128,
    pub floor_s: f64,
    /// Matched pairs on this workload (censored ones included).
    pub pairs: u64,
    pub a_mean_s: Option<f64>,
    pub b_mean_s: Option<f64>,
    /// `Σ S_B / Σ S_A` over the pairs.
    pub ratio_b_over_a: Option<f64>,
    /// Pairs with a censored side.
    #[serde(default)]
    pub censored_pairs: u64,
}

/// One curve's ratio: the per-size figure a claim about scaling reads.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurveRatio {
    pub slug: String,
    pub log2_r: f64,
    pub workloads: u64,
    pub pairs: u64,
    pub ratio_b_over_a: Option<f64>,
    pub ci95: Option<(f64, f64)>,
    pub ci_method: String,
    #[serde(default)]
    pub indicative: bool,
    #[serde(default)]
    pub censored_pairs: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ci95_bootstrap: Option<(f64, f64)>,
}

/// AGENTS.md §5a for one comparison: the ratio with every arm's unpriced
/// native work charged at each weight.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpsBand {
    pub weights: Vec<f64>,
    /// `Σ priced S_B / Σ priced S_A` at each weight; `None` where a run has
    /// an unpriced counter with no count.
    pub ratio_b_over_a: Vec<Option<f64>>,
    /// `B < A at every weight`, `B > A at every weight`, `unresolved inside
    /// the band` or `unrankable (uncounted unpriced work)`.
    pub order: String,
    /// Verified pairs the band is computed over.
    pub pairs: u64,
    pub note: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpsResult {
    /// `ok`; `incomplete` (a measured run did not verify); `partial`
    /// (some runs have no partner); or `empty`.
    pub status: String,
    pub unit: String,
    /// `Σ S_B / Σ S_A` over every matched pair, all curves pooled.  When
    /// `censored_pairs > 0` it is a bound (see `bound`).
    pub ratio_b_over_a: Option<f64>,
    /// v2: the linearised `t(K−1)` interval.  v1: the percentile two-stage
    /// bootstrap.
    pub ci95: Option<(f64, f64)>,
    /// v2: `linearised_t` (two or more workloads) or `within_t` (one
    /// workload: pairs are the clusters, so the interval says nothing about
    /// other targets).  v1: `cluster` or `within` (bootstrap).
    pub ci_method: String,
    pub pairs: u64,
    /// Measured runs of either arm with a cost and no partner.
    pub unpaired_runs: u64,
    /// The pairs ran under the same algorithm seeds.
    pub same_seeds: bool,
    pub same_session: bool,
    /// Pooled over curves of different sizes; read `curves` for scaling.
    pub pooled_curves: u64,
    /// Either arm left work unpriced. The displayed ratio is diagnostic:
    /// dividing two lower bounds does not bound the full-cost ratio.
    pub bounded: bool,
    pub resamples: usize,
    /// Workloads (clusters) behind the interval.
    #[serde(default)]
    pub workloads: u64,
    /// Student-t degrees of freedom of the linearised interval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dof: Option<f64>,
    /// Fewer than [`INDICATIVE_BELOW_WORKLOADS`] workloads (or one
    /// workload): the interval is indicative, not a result.
    #[serde(default)]
    pub indicative: bool,
    /// `workloads` or `within_workload`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_scope: Option<String>,
    /// The percentile two-stage (or, with one workload, within-workload)
    /// bootstrap interval of the same statistic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ci95_bootstrap: Option<(f64, f64)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ci_bootstrap_method: Option<String>,
    /// Pairs with a timed-out or exhausted side, entered at their
    /// (lower-bound) spent cost.
    #[serde(default)]
    pub censored_pairs: u64,
    /// `lower` (only B censored), `upper` (only A) or `indeterminate`;
    /// absent when nothing is censored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<String>,
    /// The ratio over the pairs where both sides verified: the v1 figure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_only_ratio: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub band: Option<OpsBand>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WallResult {
    /// `admitted`, `descriptive` or `refused`.
    pub status: String,
    pub reasons: Vec<String>,
    pub required_level: Level,
    pub pairs: u64,
    pub pairs_below_level: u64,
    /// Median over pairs of `solve_wall(B) / solve_wall(A)`.  The v1
    /// headline: a different estimand from the ops ratio, kept for
    /// compatibility.
    pub median_ratio: Option<f64>,
    /// v2: the interval of `ratio_b_over_a` (linearised `t`, pairs as
    /// clusters).  v1: the bootstrap interval of the median.
    pub ci95: Option<(f64, f64)>,
    /// The two arms of the noise floor (denominator, numerator).
    pub aa_arms: Option<(String, String)>,
    pub aa_pairs: u64,
    /// v1: the A/A median ratio.
    pub aa_median_ratio: Option<f64>,
    /// v2: the interval of the A/A ns-per-GAE ratio.  v1: the median's.
    pub aa_ci95: Option<(f64, f64)>,
    /// v2: the corrected B/A interval excludes 1.  v1: the B/A interval
    /// excludes 1 and does not overlap the A/A interval.
    pub outside_noise: Option<bool>,

    /// v2 primary: `(Σ solve_wall_B / Σ GAE_B) / (Σ solve_wall_A / Σ GAE_A)`
    /// over the admitted pairs (all verified pairs when not admitted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratio_b_over_a: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ns_per_gae_a: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ns_per_gae_b: Option<f64>,
    /// Arms whose GAE is a lower bound (unpriced work): their ns per GAE is
    /// an upper bound on the cost per operation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ns_per_gae_upper_bound_arms: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ci_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dof: Option<f64>,
    /// End to end: `Σ solve_wall_B / Σ solve_wall_A` and its linearised
    /// interval.  Carries the work variance; secondary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_wall_ratio: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_wall_ci95: Option<(f64, f64)>,
    /// The two compared arms run one method: this comparison is itself an
    /// A/A control and has nothing to correct.
    #[serde(default)]
    pub self_aa: bool,
    /// Why this A/A pair: `includes arm A`, `includes the reference arm` or
    /// `first alphabetical`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aa_selection: Option<String>,
    /// The A/A ns-per-GAE ratio: the position bias estimate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aa_ratio: Option<f64>,
    /// `ratio_b_over_a / aa_ratio` and its interval (independence on the
    /// log scale; conservative when the A/A shares an arm).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corrected_ratio: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corrected_ci95: Option<(f64, f64)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corrected_dof: Option<f64>,
    /// The v1 criterion, computed as v1 did (median ratios, bootstrap
    /// intervals, first alphabetical same-method pair).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_outside_noise: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_ci95: Option<(f64, f64)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_aa_ci95: Option<(f64, f64)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_aa_arms: Option<(String, String)>,
}

/// Operational measurement quality. The score is a fixed rubric, not a
/// probability or a substitute for the bootstrap interval or wall gate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeasurementQuality {
    pub score: Option<u8>,
    pub rubric: String,
    pub paired_attempts: u64,
    pub verified_pairs: u64,
    pub admitted_pairs: u64,
    pub levels: BTreeMap<String, u64>,
    /// The 95th percentile over verified runs with observable contention.
    pub p95_foreign_busy_percent: Option<f64>,
    pub p95_run_queue_percent: Option<f64>,
    pub contention_observed_runs: u64,
    pub notes: Vec<String>,
}

fn yes() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comparison {
    pub schema: String,
    pub comparison_id: String,
    /// The deterministic bootstrap seed. Historical v1 files did not store
    /// it and remain readable; calibration requires a recorded seed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resample_seed: Option<u64>,
    pub a: ArmSummary,
    pub b: ArmSummary,
    pub workloads: Vec<WorkloadRow>,
    pub curves: Vec<CurveRatio>,
    pub ops: OpsResult,
    pub wall: WallResult,
    /// Added after the original v1 comparison files; historical files
    /// deserialize without it and keep their original meaning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<MeasurementQuality>,
    pub verdict: String,
    /// The id a v1 comparison of the same inputs carries; calibration
    /// replays v1 files against it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_comparison_id: Option<String>,
    /// Status of each side's session (`complete`, `interrupted`, `running`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a_session_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b_session_status: Option<String>,
    /// `--allow-interrupted` was given.
    #[serde(default)]
    pub allow_interrupted: bool,
    /// Both sessions are `complete`.  False means the comparison is
    /// descriptive evidence of an interrupted run, not a result.
    #[serde(default = "yes")]
    pub admissible: bool,
}

/// How a comparison is run beyond which arms.
#[derive(Clone, Debug)]
pub struct CompareOptions {
    pub resamples: usize,
    pub seed: u64,
    /// Compare sessions whose status is not `complete`; recorded and the
    /// comparison is marked not admissible.
    pub allow_interrupted: bool,
}

// ── Tail diagnostics ───────────────────────────────────────────────

/// Max/median and the top run's share of the total over `(run id, S)`.
pub fn tail_diagnostics(runs: &[(String, f64)]) -> Option<TailDiagnostics> {
    if runs.is_empty() {
        return None;
    }
    let s: Vec<f64> = runs.iter().map(|r| r.1).collect();
    let total: f64 = s.iter().sum();
    let (top_id, top) = runs
        .iter()
        .max_by(|x, y| x.1.total_cmp(&y.1))
        .map(|r| (r.0.clone(), r.1))?;
    let med = median(&s)?;
    let share = if total > 0.0 { top / total } else { 0.0 };
    Some(TailDiagnostics {
        runs: runs.len() as u64,
        median_s: med,
        max_s: top,
        max_over_median: (med > 0.0).then(|| top / med),
        top_run_share: share,
        top_run_id: top_id,
        heavy_tail: share > HEAVY_TAIL_SHARE,
    })
}

fn summarize(session_id: &str, arm: &str, recs: &[&Record]) -> Result<ArmSummary, String> {
    let first = recs
        .first()
        .ok_or_else(|| format!("arm `{arm}` has no measured runs"))?;
    let mut statuses = BTreeMap::new();
    for r in recs {
        *statuses.entry(r.outcome.status.clone()).or_insert(0) += 1;
    }
    let ok: Vec<&&Record> = recs.iter().filter(|r| r.counts()).collect();
    let s: Vec<f64> = ok.iter().filter_map(|r| r.cost.s).collect();
    let f: Vec<f64> = ok
        .iter()
        .filter_map(|r| r.boundaries.ratio_to_floor)
        .collect();
    let tail_runs: Vec<(String, f64)> = ok
        .iter()
        .filter_map(|r| Some((r.run_id.clone(), r.cost.s?)))
        .collect();
    let mut unpriced: BTreeMap<String, Option<u64>> = BTreeMap::new();
    for r in &ok {
        for (name, n) in unpriced_counts(r) {
            let e = unpriced.entry(name).or_insert(Some(0));
            *e = match (*e, n) {
                (Some(t), Some(n)) => Some(t + n),
                _ => None,
            };
        }
    }
    let censored = recs
        .iter()
        .filter(|r| matches!(r.outcome.status.as_str(), "timeout" | "exhausted"))
        .count() as u64;
    Ok(ArmSummary {
        session_id: session_id.into(),
        arm: arm.into(),
        method_id: first.method.method_id.clone(),
        method: first.method.id.clone(),
        measured: recs.len() as u64,
        verified: ok.len() as u64,
        statuses,
        mean_s: mean(&s),
        mean_ratio_to_floor: mean(&f),
        lower_bound: recs.iter().any(|r| r.cost.lower_bound),
        deterministic: recs.iter().all(|r| r.cost.deterministic),
        censored,
        tail: tail_diagnostics(&tail_runs),
        unpriced_counts: unpriced,
    })
}

fn measured<'a>(recs: &'a [Record], arm: &str) -> Vec<&'a Record> {
    recs.iter().filter(|r| r.arm == arm && !r.warmup).collect()
}

// ── Pairs, censoring ───────────────────────────────────────────────

/// One matched `(workload, round)`: `S` of each arm, whether that value is
/// a lower bound on the run's cost (the run was cut off), and the pair's
/// priced `S` at each [`DEFAULT_WEIGHTS`] entry.
#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    pub a: f64,
    pub b: f64,
    pub a_censored: bool,
    pub b_censored: bool,
    pub band: Vec<(Option<f64>, Option<f64>)>,
}

impl Pair {
    pub fn exact(a: f64, b: f64) -> Self {
        Self {
            a,
            b,
            a_censored: false,
            b_censored: false,
            band: Vec::new(),
        }
    }
    fn censored(&self) -> bool {
        self.a_censored || self.b_censored
    }
}

/// `Σ b / Σ a` over pairs, `None` when undefined.
fn ratio_of_sums(pairs: impl Iterator<Item = (f64, f64)>) -> Option<f64> {
    let (mut sa, mut sb, mut n) = (0.0, 0.0, 0usize);
    for (a, b) in pairs {
        sa += a;
        sb += b;
        n += 1;
    }
    (n > 0 && sa > 0.0).then(|| sb / sa)
}

fn pair_ratio(pairs: impl Iterator<Item = Pair>) -> Option<f64> {
    ratio_of_sums(pairs.map(|p| (p.a, p.b)))
}

/// What converts a timed-out run's wall time into a spent-cost lower bound:
/// the arm's largest ns per GAE and largest process overhead (fork to reap
/// minus the in-process solve) over its verified runs.  Conservative on
/// both: the bound is as low as the arm's own data allow.
struct ArmCensor {
    max_ns_per_gae: Option<f64>,
    max_overhead_ns: Option<u64>,
}

fn arm_censor(recs: &[&Record]) -> ArmCensor {
    let mut k: Option<f64> = None;
    let mut o: Option<u64> = None;
    for r in recs.iter().filter(|r| r.counts()) {
        if let (Some(w), Some(g)) = (r.time.solve_wall_ns, r.cost.total_gae) {
            if w > 0 && g > 0.0 {
                let ns = w as f64 / g;
                k = Some(k.map_or(ns, |x: f64| x.max(ns)));
                let over = r.time.process_wall_ns.saturating_sub(w);
                o = Some(o.map_or(over, |x: u64| x.max(over)));
            }
        }
    }
    ArmCensor {
        max_ns_per_gae: k,
        max_overhead_ns: o,
    }
}

/// `(S, censored)` of a run, `None` for a run with no usable cost (error,
/// crash, wrong answer, or no partner).  An exhausted run is censored at
/// the cost it spent; a timed-out run at the cost its wall time implies.
fn run_cost(r: &Record, c: &ArmCensor) -> Option<(f64, bool)> {
    if r.counts() {
        return r.cost.s.map(|s| (s, false));
    }
    match r.outcome.status.as_str() {
        "exhausted" => r.cost.s.map(|s| (s, true)),
        "timeout" => {
            if let Some(s) = r.cost.s {
                return Some((s, true));
            }
            let sqrt_r = (r.workload.curve.r as f64).sqrt();
            let lb = match (c.max_ns_per_gae, c.max_overhead_ns) {
                (Some(k), Some(o)) if sqrt_r > 0.0 => {
                    (r.time.process_wall_ns.saturating_sub(o) as f64 / k) / sqrt_r
                }
                _ => 0.0,
            };
            Some((lb, true))
        }
        _ => None,
    }
}

/// Matched pairs per workload, with the seeds' agreement.
struct Matched {
    by_workload: BTreeMap<String, Vec<Pair>>,
    unpaired: u64,
    same_seeds: bool,
}

fn priced_pair(ra: &Record, rb: &Record) -> Vec<(Option<f64>, Option<f64>)> {
    DEFAULT_WEIGHTS
        .iter()
        .map(|&w| {
            (
                priced_s(ra, w).ok().flatten(),
                priced_s(rb, w).ok().flatten(),
            )
        })
        .collect()
}

fn match_pairs(ma: &[&Record], mb: &[&Record]) -> Matched {
    let (ca, cb) = (arm_censor(ma), arm_censor(mb));
    let key = |r: &Record| (r.workload.workload_id.clone(), r.round);
    let a: BTreeMap<(String, u32), (&Record, f64, bool)> = ma
        .iter()
        .filter_map(|r| run_cost(r, &ca).map(|(s, c)| (key(r), (*r, s, c))))
        .collect();
    let b: BTreeMap<(String, u32), (&Record, f64, bool)> = mb
        .iter()
        .filter_map(|r| run_cost(r, &cb).map(|(s, c)| (key(r), (*r, s, c))))
        .collect();
    let mut by_workload: BTreeMap<String, Vec<Pair>> = BTreeMap::new();
    let mut same_seeds = true;
    for (k, (ra, sa, ca)) in &a {
        if let Some((rb, sb, cb)) = b.get(k) {
            same_seeds &= ra.algorithm_seed == rb.algorithm_seed;
            let exact = !*ca && !*cb;
            by_workload.entry(k.0.clone()).or_default().push(Pair {
                a: *sa,
                b: *sb,
                a_censored: *ca,
                b_censored: *cb,
                band: if exact {
                    priced_pair(ra, rb)
                } else {
                    Vec::new()
                },
            });
        }
    }
    let paired: usize = by_workload.values().map(|v| v.len()).sum();
    let unpaired = (a.len() - paired) as u64 + (b.len() - paired) as u64;
    Matched {
        by_workload,
        unpaired,
        same_seeds,
    }
}

/// The direction in which a censored comparison bounds the true ratio: a
/// censored B lower-bounds `Σ B` (so the ratio from below), a censored A
/// lower-bounds `Σ A` (so the ratio from above), both leave it open.
pub fn bound_direction(pairs: &[Pair]) -> Option<String> {
    let a = pairs.iter().any(|p| p.a_censored);
    let b = pairs.iter().any(|p| p.b_censored);
    match (a, b) {
        (false, false) => None,
        (false, true) => Some("lower".into()),
        (true, false) => Some("upper".into()),
        (true, true) => Some("indeterminate".into()),
    }
}

/// The ratio over `strata` with every interval and flag.
#[derive(Clone, Debug)]
pub struct RatioReport {
    pub ratio: Option<f64>,
    pub ci95: Option<(f64, f64)>,
    pub ci_method: String,
    pub dof: Option<f64>,
    pub workloads: usize,
    pub pairs: usize,
    pub indicative: bool,
    pub scope: Option<String>,
    pub ci95_bootstrap: Option<(f64, f64)>,
    pub bootstrap_method: String,
    pub censored_pairs: usize,
    pub bound: Option<String>,
    pub verified_only_ratio: Option<f64>,
}

/// The ratio of totals over `strata` (workloads of pairs) with its
/// linearised `t` interval (workloads as clusters; one workload: pairs as
/// clusters), the bootstrap interval beside it, and the censoring bound.
pub fn ratio_report(strata: &[Vec<Pair>], resamples: usize, seed: u64) -> RatioReport {
    let strata: Vec<Vec<Pair>> = strata.iter().filter(|s| !s.is_empty()).cloned().collect();
    let stat = |s: &[Vec<Pair>]| pair_ratio(s.iter().flatten().cloned());
    let all: Vec<Pair> = strata.iter().flatten().cloned().collect();
    let ratio = stat(&strata);
    let k = strata.len();
    let (lin, ci_method, scope) = if k >= 2 {
        let clusters: Vec<(f64, f64)> = strata
            .iter()
            .map(|s| (s.iter().map(|p| p.a).sum(), s.iter().map(|p| p.b).sum()))
            .collect();
        (
            linearised_ratio(&clusters),
            "linearised_t",
            Some("workloads"),
        )
    } else if k == 1 {
        let clusters: Vec<(f64, f64)> = strata[0].iter().map(|p| (p.a, p.b)).collect();
        (
            linearised_ratio(&clusters),
            "within_t",
            Some("within_workload"),
        )
    } else {
        (None, "none", None)
    };
    let (boot, bootstrap_method) = if k >= 2 {
        (
            cluster_bootstrap_ci(&strata, resamples, seed, stat),
            "cluster",
        )
    } else {
        (bootstrap_ci(&strata, resamples, seed, stat), "within")
    };
    let exact: Vec<Pair> = all.iter().filter(|p| !p.censored()).cloned().collect();
    RatioReport {
        ratio,
        ci95: lin.and_then(|l| l.ci95_log().or_else(|| l.ci95())),
        ci_method: ci_method.into(),
        dof: lin.map(|l| l.df),
        workloads: k,
        pairs: all.len(),
        indicative: k < INDICATIVE_BELOW_WORKLOADS,
        scope: scope.map(String::from),
        ci95_bootstrap: boot,
        bootstrap_method: bootstrap_method.into(),
        censored_pairs: all.iter().filter(|p| p.censored()).count(),
        bound: bound_direction(&all),
        verified_only_ratio: pair_ratio(exact.into_iter()),
    }
}

/// AGENTS.md §5a over the exact pairs: the ratio at each weight and whether
/// the order holds at all of them.
pub fn band_report(
    strata: &[Vec<Pair>],
    a_arm: &str,
    b_arm: &str,
    censored_pairs: usize,
) -> Option<OpsBand> {
    let exact: Vec<&Pair> = strata
        .iter()
        .flatten()
        .filter(|p| !p.censored() && p.band.len() == DEFAULT_WEIGHTS.len())
        .collect();
    if exact.is_empty() {
        return None;
    }
    let ratios: Vec<Option<f64>> = (0..DEFAULT_WEIGHTS.len())
        .map(|i| {
            let (mut sa, mut sb) = (0.0, 0.0);
            for p in &exact {
                let (Some(x), Some(y)) = p.band[i] else {
                    return None;
                };
                sa += x;
                sb += y;
            }
            (sa > 0.0).then(|| sb / sa)
        })
        .collect();
    let order = if ratios.iter().any(Option::is_none) {
        "unrankable (uncounted unpriced work)".to_string()
    } else if ratios.iter().flatten().all(|r| *r < 1.0) {
        format!("{b_arm} < {a_arm} at every weight")
    } else if ratios.iter().flatten().all(|r| *r > 1.0) {
        format!("{b_arm} > {a_arm} at every weight")
    } else {
        "unresolved inside the band".to_string()
    };
    let mut note =
        String::from("point ratios of totals; rank only where the order holds at every weight");
    if censored_pairs > 0 {
        note.push_str(&format!(
            "; over the {} verified pair(s), {censored_pairs} censored pair(s) excluded",
            exact.len()
        ));
    }
    Some(OpsBand {
        weights: DEFAULT_WEIGHTS.to_vec(),
        ratio_b_over_a: ratios,
        order,
        pairs: exact.len() as u64,
        note,
    })
}

// ── Comparison ─────────────────────────────────────────────────────

/// Compare arm `b` against arm `a`, refusing a session that is not
/// `complete`.  `b_dir` names a second session (a candidate binary's run of
/// the same spec); wall time is then refused.
pub fn compare(
    a_dir: &Path,
    a_arm: &str,
    b_dir: Option<&Path>,
    b_arm: &str,
    resamples: usize,
    seed: u64,
) -> Result<Comparison, String> {
    compare_with(
        a_dir,
        a_arm,
        b_dir,
        b_arm,
        &CompareOptions {
            resamples,
            seed,
            allow_interrupted: false,
        },
    )
}

/// [`compare`] with options.
pub fn compare_with(
    a_dir: &Path,
    a_arm: &str,
    b_dir: Option<&Path>,
    b_arm: &str,
    opts: &CompareOptions,
) -> Result<Comparison, String> {
    let sa = read_session(a_dir)?;
    let ra = read_records(a_dir)?;
    let (sb, rb) = match b_dir {
        Some(d) => (read_session(d)?, read_records(d)?),
        None => (sa.clone(), ra.clone()),
    };
    let spec: Option<Spec> = std::fs::read_to_string(a_dir.join("spec.json"))
        .ok()
        .and_then(|t| Spec::from_json(&t).ok());
    let required = spec
        .as_ref()
        .map(|s| s.measurement.isolation_required)
        .unwrap_or(Level::L2);
    compare_loaded(&sa, &ra, &sb, &rb, a_arm, b_arm, required, opts)
}

/// [`compare_with`] on sessions and records already read; `required` is the
/// spec's `isolation_required`.
#[allow(clippy::too_many_arguments)]
pub fn compare_loaded(
    sa: &Session,
    ra: &[Record],
    sb: &Session,
    rb: &[Record],
    a_arm: &str,
    b_arm: &str,
    required: Level,
    opts: &CompareOptions,
) -> Result<Comparison, String> {
    let (resamples, seed) = (opts.resamples, opts.seed);
    let same_session = sa.session_id == sb.session_id;
    if same_session && a_arm == b_arm {
        return Err("compare two different arms".into());
    }
    // A session that did not finish has records the runner never sealed
    // against a complete plan: refuse unless asked, and then say so.
    let mut unfinished: Vec<String> = Vec::new();
    for s in [sa, sb] {
        if s.status != "complete" && !unfinished.iter().any(|u| u.contains(&s.session_id)) {
            unfinished.push(format!("{} is `{}`", s.session_id, s.status));
        }
    }
    if !unfinished.is_empty() && !opts.allow_interrupted {
        return Err(format!(
            "session not complete ({}): refusing to compare; pass --allow-interrupted to compare it as descriptive evidence (the comparison is then recorded as not admissible)",
            unfinished.join("; ")
        ));
    }
    let admissible = unfinished.is_empty();
    let ma = measured(ra, a_arm);
    let mb = measured(rb, b_arm);
    let a = summarize(&sa.session_id, a_arm, &ma)?;
    let b = summarize(&sb.session_id, b_arm, &mb)?;

    // Refusals: different workloads, different units.
    let wa: BTreeSet<&str> = ma.iter().map(|r| r.workload.workload_id.as_str()).collect();
    let wb: BTreeSet<&str> = mb.iter().map(|r| r.workload.workload_id.as_str()).collect();
    if wa != wb {
        return Err(format!(
            "the arms ran different workloads ({} vs {}); a comparison pairs identical workloads",
            wa.len(),
            wb.len()
        ));
    }
    if ma
        .iter()
        .chain(mb.iter())
        .any(|r| r.cost.unit != ma[0].cost.unit)
    {
        return Err("the arms report different units".into());
    }

    let matched = match_pairs(&ma, &mb);
    let workload_of: BTreeMap<&str, &Record> = ma
        .iter()
        .map(|r| (r.workload.workload_id.as_str(), *r))
        .collect();
    let empty = Vec::new();
    let rows: Vec<WorkloadRow> = wa
        .iter()
        .map(|wid| {
            let pairs = matched.by_workload.get(*wid).unwrap_or(&empty);
            let any = workload_of[wid];
            WorkloadRow {
                workload_id: wid.to_string(),
                slug: any.workload.curve.slug.clone(),
                r: any.workload.curve.r,
                floor_s: any.boundaries.floor_s,
                pairs: pairs.len() as u64,
                a_mean_s: mean(&pairs.iter().map(|p| p.a).collect::<Vec<_>>()),
                b_mean_s: mean(&pairs.iter().map(|p| p.b).collect::<Vec<_>>()),
                ratio_b_over_a: pair_ratio(pairs.iter().cloned()),
                censored_pairs: pairs.iter().filter(|p| p.censored()).count() as u64,
            }
        })
        .collect();
    let strata: Vec<Vec<Pair>> = matched
        .by_workload
        .values()
        .filter(|v| !v.is_empty())
        .cloned()
        .collect();
    let rep = ratio_report(&strata, resamples, seed);
    let mut slugs: Vec<String> = rows.iter().map(|r| r.slug.clone()).collect();
    slugs.sort();
    slugs.dedup();
    let curves: Vec<CurveRatio> = slugs
        .iter()
        .map(|slug| {
            let mine: Vec<&WorkloadRow> = rows.iter().filter(|r| &r.slug == slug).collect();
            let strata: Vec<Vec<Pair>> = mine
                .iter()
                .filter_map(|r| matched.by_workload.get(&r.workload_id))
                .filter(|v| !v.is_empty())
                .cloned()
                .collect();
            let c = ratio_report(&strata, resamples, seed);
            CurveRatio {
                slug: slug.clone(),
                log2_r: (mine[0].r as f64).log2(),
                workloads: mine.len() as u64,
                pairs: c.pairs as u64,
                ratio_b_over_a: c.ratio,
                ci95: c.ci95,
                ci_method: c.ci_method,
                indicative: c.indicative,
                censored_pairs: c.censored_pairs as u64,
                bound: c.bound,
                ci95_bootstrap: c.ci95_bootstrap,
            }
        })
        .collect();
    let incomplete = a.verified < a.measured || b.verified < b.measured;
    let band = band_report(&strata, a_arm, b_arm, rep.censored_pairs);
    let ops = OpsResult {
        status: if rep.ratio.is_none() {
            "empty".into()
        } else if incomplete {
            "incomplete".into()
        } else if matched.unpaired > 0 {
            "partial".into()
        } else {
            "ok".into()
        },
        unit: ma[0].cost.unit.clone(),
        ratio_b_over_a: rep.ratio,
        ci95: rep.ci95,
        ci_method: rep.ci_method.clone(),
        pairs: rep.pairs as u64,
        unpaired_runs: matched.unpaired,
        same_seeds: matched.same_seeds,
        same_session,
        pooled_curves: curves.len() as u64,
        bounded: a.lower_bound || b.lower_bound,
        resamples,
        workloads: rep.workloads as u64,
        dof: rep.dof,
        indicative: rep.indicative,
        interval_scope: rep.scope.clone(),
        ci95_bootstrap: rep.ci95_bootstrap,
        ci_bootstrap_method: Some(rep.bootstrap_method.clone()),
        censored_pairs: rep.censored_pairs as u64,
        bound: rep.bound.clone(),
        verified_only_ratio: rep.verified_only_ratio,
        band,
    };

    // Wall time.
    let upper_bound_arms: Vec<String> = [(&a, a_arm), (&b, b_arm)]
        .iter()
        .filter(|(s, _)| s.lower_bound)
        .map(|(_, n)| n.to_string())
        .collect();
    let wall = wall_compare(
        ra,
        a_arm,
        b_arm,
        same_session,
        required,
        resamples,
        seed,
        &upper_bound_arms,
    );
    let quality = measurement_quality(
        ra,
        a_arm,
        b_arm,
        same_session,
        required,
        &wall,
        sa.ticks_per_second,
    );

    let verdict = verdict_line(
        &a,
        &b,
        a_arm,
        b_arm,
        &ops,
        rows.len(),
        &wall,
        &unfinished,
        opts.allow_interrupted,
    );
    let ids = |extra: bool| {
        short_id(
            "ECC1",
            &if extra {
                json!({
                    "schema": COMPARISON_SCHEMA_V2,
                    "a": [a.session_id, a.arm, a.method_id],
                    "b": [b.session_id, b.arm, b.method_id],
                    "resamples": resamples,
                    "seed": seed.to_string(),
                    "allow_interrupted": opts.allow_interrupted,
                })
            } else {
                json!({
                    "a": [a.session_id, a.arm, a.method_id],
                    "b": [b.session_id, b.arm, b.method_id],
                    "resamples": resamples,
                    "seed": seed.to_string(),
                })
            },
        )
    };
    let (comparison_id, _) = ids(true)?;
    let (legacy_id, _) = ids(false)?;
    Ok(Comparison {
        schema: COMPARISON_SCHEMA_V2.into(),
        comparison_id,
        resample_seed: Some(seed),
        a,
        b,
        workloads: rows,
        curves,
        ops,
        wall,
        quality: Some(quality),
        verdict,
        legacy_comparison_id: Some(legacy_id),
        a_session_status: Some(sa.status.clone()),
        b_session_status: Some(sb.status.clone()),
        allow_interrupted: opts.allow_interrupted,
        admissible,
    })
}

fn fmt_ci(c: Option<(f64, f64)>) -> String {
    c.map(|(l, h)| format!(" [{l:.3}, {h:.3}]"))
        .unwrap_or_default()
}

#[allow(clippy::too_many_arguments)]
fn verdict_line(
    a: &ArmSummary,
    b: &ArmSummary,
    a_arm: &str,
    b_arm: &str,
    ops: &OpsResult,
    n_workloads: usize,
    wall: &WallResult,
    unfinished: &[String],
    allow_interrupted: bool,
) -> String {
    let mut v = format!(
        "ops ({}): {} / {} = {}{} ({}{}) over {} pairs on {} workloads, {} curve(s)",
        ops.status,
        b_arm,
        a_arm,
        ops.ratio_b_over_a
            .map(|r| format!("{r:.4}"))
            .unwrap_or_else(|| "unknown".into()),
        fmt_ci(ops.ci95),
        ops.ci_method,
        ops.dof.map(|d| format!(", df {d:.0}")).unwrap_or_default(),
        ops.pairs,
        n_workloads,
        ops.pooled_curves,
    );
    if ops.indicative && ops.ci95.is_some() {
        v.push_str(
            if ops.interval_scope.as_deref() == Some("within_workload") {
                ", INDICATIVE: one workload, the interval does not generalise to other targets"
            } else {
                ", INDICATIVE: fewer than 5 workloads"
            },
        );
    }
    if let Some(dir) = &ops.bound {
        v.push_str(&format!(
            ", BOUND: {} censored pair(s) (timeout or exhausted) entered at their spent cost, so the ratio is a {dir} bound{}",
            ops.censored_pairs,
            if dir == "indeterminate" {
                ", not a bound in either direction"
            } else {
                " on B/A"
            }
        ));
        if let Some(r) = ops.verified_only_ratio {
            v.push_str(&format!("; verified pairs only {r:.4}"));
        }
    }
    if ops.bounded {
        v.push_str(", diagnostic only (unpriced work; ratio does not bound full cost)");
    }
    if let Some(band) = &ops.band {
        let fmt: Vec<String> = band
            .weights
            .iter()
            .zip(&band.ratio_b_over_a)
            .map(|(w, r)| match r {
                Some(r) => format!("w={w}: {r:.4}"),
                None => format!("w={w}: n/a"),
            })
            .collect();
        v.push_str(&format!("; band ({}): {}", fmt.join(", "), band.order));
    }
    v.push_str(match ops.status.as_str() {
        "incomplete" => ", NOT admissible: a measured run did not verify",
        "partial" => ", partial: some runs have no partner",
        _ => "",
    });
    for arm in [a, b] {
        if let Some(t) = &arm.tail {
            if t.heavy_tail {
                v.push_str(&format!(
                    "; heavy tail in {}: top run is {:.0}% of the arm's total S (max/median {}); more runs needed",
                    arm.arm,
                    100.0 * t.top_run_share,
                    t.max_over_median
                        .map(|x| format!("{x:.1}"))
                        .unwrap_or_else(|| "n/a".into()),
                ));
            }
        }
    }
    if !unfinished.is_empty() {
        v.push_str(&format!(
            "; NOT admissible: session not complete ({}){}",
            unfinished.join("; "),
            if allow_interrupted {
                " compared under --allow-interrupted"
            } else {
                ""
            }
        ));
    }
    v.push_str(&format!("; wall: {}", wall.status));
    let ub = if wall.ns_per_gae_upper_bound_arms.is_empty() {
        String::new()
    } else {
        format!(
            "; ns/GAE of {} is an upper bound on its cost per operation (GAE is a lower bound)",
            wall.ns_per_gae_upper_bound_arms.join(", ")
        )
    };
    match (wall.status.as_str(), wall.ratio_b_over_a) {
        ("admitted", Some(r)) => {
            v.push_str(&format!(
                " ns/GAE {b_arm}/{a_arm} {r:.4}{}",
                fmt_ci(wall.ci95)
            ));
            if let Some(c) = wall.corrected_ratio {
                v.push_str(&format!(
                    ", bias-corrected {c:.4}{} (A/A {:.4}, {}), {}",
                    fmt_ci(wall.corrected_ci95),
                    wall.aa_ratio.unwrap_or(f64::NAN),
                    wall.aa_selection.as_deref().unwrap_or("?"),
                    match wall.outside_noise {
                        Some(true) => "outside noise",
                        Some(false) => "within noise",
                        None => "noise test unavailable",
                    }
                ));
            } else if wall.self_aa {
                v.push_str(", an A/A control (nothing to correct)");
            } else {
                v.push_str(", no admitted A/A: not corrected for bias, noise test unavailable");
            }
            if let Some(t) = wall.total_wall_ratio {
                v.push_str(&format!(
                    "; end-to-end wall {t:.4}{}",
                    fmt_ci(wall.total_wall_ci95)
                ));
            }
            v.push_str(&ub);
        }
        ("descriptive", r) => {
            if let Some(r) = r {
                v.push_str(&format!(
                    " (ns/GAE {r:.4}, not a result: {})",
                    wall.reasons.join("; ")
                ));
            } else {
                v.push_str(&format!(" ({})", wall.reasons.join("; ")));
            }
            v.push_str(&ub);
        }
        _ => v.push_str(&format!(" ({})", wall.reasons.join("; "))),
    }
    v
}

// ── Wall time ──────────────────────────────────────────────────────

/// A matched pair's wall figures: `solve_wall` (ns) and counted GAE of each
/// arm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallPair {
    pub a_wall: f64,
    pub b_wall: f64,
    pub a_gae: f64,
    pub b_gae: f64,
}

/// The v2 wall figures over pairs.
#[derive(Clone, Copy, Debug)]
pub struct WallFigures {
    pub ns_per_gae_a: f64,
    pub ns_per_gae_b: f64,
    /// `ns_per_gae_b / ns_per_gae_a` with the linearised standard error
    /// (pairs are the clusters); its interval is read on the log scale.
    pub ratio: Linearised,
    /// `Σ wall_B / Σ wall_A`.
    pub total: Option<Linearised>,
}

/// Per-arm ns per GAE (`Σ wall / Σ GAE`) and its ratio B/A.  The influence
/// of pair `i` on `log ρ` is `bw_i/Σbw − bg_i/Σbg − aw_i/Σaw + ag_i/Σag`;
/// scaled by `ρ` it is the pair's term in the first-order expansion of the
/// ratio.  `None` below two pairs or on a non-positive total.
pub fn wall_figures(pairs: &[WallPair]) -> Option<WallFigures> {
    if pairs.len() < 2 {
        return None;
    }
    let sum = |f: fn(&WallPair) -> f64| pairs.iter().map(f).sum::<f64>();
    let (aw, bw) = (sum(|p| p.a_wall), sum(|p| p.b_wall));
    let (ag, bg) = (sum(|p| p.a_gae), sum(|p| p.b_gae));
    if [aw, bw, ag, bg].iter().any(|x| x.is_nan() || *x <= 0.0) {
        return None;
    }
    let (ns_a, ns_b) = (aw / ag, bw / bg);
    let rho = ns_b / ns_a;
    let u: Vec<f64> = pairs
        .iter()
        .map(|p| rho * (p.b_wall / bw - p.b_gae / bg - p.a_wall / aw + p.a_gae / ag))
        .collect();
    Some(WallFigures {
        ns_per_gae_a: ns_a,
        ns_per_gae_b: ns_b,
        ratio: Linearised {
            estimate: rho,
            se: influence_se(&u)?,
            df: pairs.len() as f64 - 1.0,
            clusters: pairs.len(),
        },
        total: linearised_ratio(
            &pairs
                .iter()
                .map(|p| (p.a_wall, p.b_wall))
                .collect::<Vec<_>>(),
        ),
    })
}

/// The A/A pair for a comparison of `a` against `b`: `(denominator,
/// numerator, why)`.  `arms` are `(name, method id, role)`.  A pair of
/// arms running one method is a candidate; the one that includes arm A is
/// preferred (its bias then transfers to the B/A ratio exactly), then one
/// that includes the reference arm, then the first alphabetical.  `None`
/// when the compared arms are themselves an A/A or no candidate exists.
pub fn choose_aa(
    arms: &[(String, String, String)],
    a: &str,
    b: &str,
) -> Option<(String, String, &'static str)> {
    let mut arms: Vec<&(String, String, String)> = arms.iter().collect();
    arms.sort();
    arms.dedup();
    let method_of = |n: &str| arms.iter().find(|x| x.0 == n).map(|x| x.1.as_str());
    if method_of(a).is_some() && method_of(a) == method_of(b) {
        return None;
    }
    let mut cands: Vec<(&str, &str)> = Vec::new();
    for (i, x) in arms.iter().enumerate() {
        for y in &arms[i + 1..] {
            let is_ab = (x.0 == a && y.0 == b) || (x.0 == b && y.0 == a);
            if x.1 == y.1 && !is_ab {
                cands.push((x.0.as_str(), y.0.as_str()));
            }
        }
    }
    if let Some(&(x, y)) = cands.iter().find(|(x, y)| *x == a || *y == a) {
        let other = if x == a { y } else { x };
        return Some((a.to_string(), other.to_string(), "includes arm A"));
    }
    let reference: Vec<&str> = arms
        .iter()
        .filter(|x| x.2 == "reference")
        .map(|x| x.0.as_str())
        .collect();
    if let Some(&(x, y)) = cands
        .iter()
        .find(|(x, y)| reference.contains(x) || reference.contains(y))
    {
        let den = if reference.contains(&x) { x } else { y };
        let num = if den == x { y } else { x };
        return Some((
            den.to_string(),
            num.to_string(),
            "includes the reference arm",
        ));
    }
    cands
        .first()
        .map(|&(x, y)| (x.to_string(), y.to_string(), "first alphabetical"))
}

struct WallData {
    /// Every verified pair with a wall figure, whatever its level.
    all: Vec<WallPair>,
    /// Pairs where both runs earned `required`.
    admitted: Vec<WallPair>,
    pairs: u64,
    below: u64,
}

fn wall_data(recs: &[Record], a: &str, b: &str, required: Level) -> WallData {
    let mut by: BTreeMap<(String, u32), (Option<&Record>, Option<&Record>)> = BTreeMap::new();
    for r in recs.iter().filter(|r| r.counts()) {
        let key = (r.workload.workload_id.clone(), r.round);
        if r.arm == a {
            by.entry(key).or_default().0 = Some(r);
        } else if r.arm == b {
            by.entry(key).or_default().1 = Some(r);
        }
    }
    let mut d = WallData {
        all: Vec::new(),
        admitted: Vec::new(),
        pairs: 0,
        below: 0,
    };
    for (_, (ra, rb)) in by {
        let (Some(ra), Some(rb)) = (ra, rb) else {
            continue;
        };
        let (Some(ta), Some(tb)) = (ra.time.solve_wall_ns, rb.time.solve_wall_ns) else {
            continue;
        };
        d.pairs += 1;
        let below = ra.isolation.level < required || rb.isolation.level < required;
        if below {
            d.below += 1;
        }
        if let (true, Some(ga), Some(gb)) = (ta > 0 && tb > 0, ra.cost.total_gae, rb.cost.total_gae)
        {
            if ga > 0.0 && gb > 0.0 {
                let p = WallPair {
                    a_wall: ta as f64,
                    b_wall: tb as f64,
                    a_gae: ga,
                    b_gae: gb,
                };
                d.all.push(p);
                if !below {
                    d.admitted.push(p);
                }
            }
        }
    }
    d
}

/// The v1 wall statistic: median of per-pair ratios with a percentile
/// bootstrap, and the first alphabetical same-method pair as its noise
/// floor.  Kept so v1 fields and the legacy criterion keep their values.
struct LegacyWall {
    median_all: Option<f64>,
    median: Option<f64>,
    ci95: Option<(f64, f64)>,
    aa_arms: Option<(String, String)>,
    aa_median: Option<f64>,
    aa_ci95: Option<(f64, f64)>,
    outside_noise: Option<bool>,
}

/// Per-pair wall ratios `B / A` over (workload, round) where both
/// verified and both earned `required`; `(ratios by workload, pairs,
/// pairs below level)`.
fn legacy_wall_pairs(
    recs: &[Record],
    a: &str,
    b: &str,
    required: Level,
) -> (Vec<Vec<f64>>, u64, u64) {
    let mut by: BTreeMap<(String, u32), (Option<&Record>, Option<&Record>)> = BTreeMap::new();
    for r in recs.iter().filter(|r| r.counts()) {
        let key = (r.workload.workload_id.clone(), r.round);
        if r.arm == a {
            by.entry(key).or_default().0 = Some(r);
        } else if r.arm == b {
            by.entry(key).or_default().1 = Some(r);
        }
    }
    let mut strata: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let (mut pairs, mut below) = (0u64, 0u64);
    for ((wid, _), (ra, rb)) in by {
        let (Some(ra), Some(rb)) = (ra, rb) else {
            continue;
        };
        let (Some(ta), Some(tb)) = (ra.time.solve_wall_ns, rb.time.solve_wall_ns) else {
            continue;
        };
        pairs += 1;
        if ra.isolation.level < required || rb.isolation.level < required {
            below += 1;
            continue;
        }
        if ta > 0 {
            strata.entry(wid).or_default().push(tb as f64 / ta as f64);
        }
    }
    (strata.into_values().collect(), pairs, below)
}

fn legacy_wall(
    recs: &[Record],
    a: &str,
    b: &str,
    required: Level,
    resamples: usize,
    seed: u64,
) -> LegacyWall {
    let med = |s: &[Vec<f64>]| median(&s.concat());
    let mut w = LegacyWall {
        median_all: None,
        median: None,
        ci95: None,
        aa_arms: None,
        aa_median: None,
        aa_ci95: None,
        outside_noise: None,
    };
    let (strata, _, _) = legacy_wall_pairs(recs, a, b, required);
    let admitted: usize = strata.iter().map(|s| s.len()).sum();
    let (all, _, _) = legacy_wall_pairs(recs, a, b, Level::L0);
    w.median_all = med(&all);
    if admitted < MIN_WALL_PAIRS {
        return w;
    }
    w.median = med(&strata);
    w.ci95 = bootstrap_ci(&strata, resamples, seed ^ 0xA11, med);
    let mut arms: BTreeMap<String, String> = BTreeMap::new();
    for r in recs {
        arms.entry(r.arm.clone())
            .or_insert_with(|| r.method.method_id.clone());
    }
    let names: Vec<(&String, &String)> = arms.iter().collect();
    'outer: for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            if names[i].1 == names[j].1 {
                let (s, _, _) = legacy_wall_pairs(recs, names[i].0, names[j].0, required);
                let n: usize = s.iter().map(|v| v.len()).sum();
                w.aa_arms = Some((names[i].0.clone(), names[j].0.clone()));
                if n >= MIN_WALL_PAIRS {
                    w.aa_median = med(&s);
                    w.aa_ci95 = bootstrap_ci(&s, resamples, seed ^ 0xAA, med);
                }
                break 'outer;
            }
        }
    }
    if let (Some((lo, hi)), Some((alo, ahi))) = (w.ci95, w.aa_ci95) {
        w.outside_noise = Some((hi < 1.0 || lo > 1.0) && (hi < alo || lo > ahi));
    }
    w
}

#[allow(clippy::too_many_arguments)]
fn wall_compare(
    recs: &[Record],
    a: &str,
    b: &str,
    same_session: bool,
    required: Level,
    resamples: usize,
    seed: u64,
    upper_bound_arms: &[String],
) -> WallResult {
    let mut w = WallResult {
        status: "refused".into(),
        reasons: vec![],
        required_level: required,
        pairs: 0,
        pairs_below_level: 0,
        median_ratio: None,
        ci95: None,
        aa_arms: None,
        aa_pairs: 0,
        aa_median_ratio: None,
        aa_ci95: None,
        outside_noise: None,
        ratio_b_over_a: None,
        ns_per_gae_a: None,
        ns_per_gae_b: None,
        ns_per_gae_upper_bound_arms: upper_bound_arms.to_vec(),
        ci_method: None,
        dof: None,
        total_wall_ratio: None,
        total_wall_ci95: None,
        self_aa: false,
        aa_selection: None,
        aa_ratio: None,
        corrected_ratio: None,
        corrected_ci95: None,
        corrected_dof: None,
        legacy_outside_noise: None,
        legacy_ci95: None,
        legacy_aa_ci95: None,
        legacy_aa_arms: None,
    };
    if !same_session {
        w.reasons
            .push("the arms ran in different sessions, so no pair shares a host window".into());
        return w;
    }
    let data = wall_data(recs, a, b, required);
    w.pairs = data.pairs;
    w.pairs_below_level = data.below;

    // The v1 statistic, for compatibility and the legacy criterion.
    let legacy = legacy_wall(recs, a, b, required, resamples, seed);
    w.median_ratio = legacy.median.or(legacy.median_all);
    w.legacy_ci95 = legacy.ci95;
    w.legacy_aa_arms = legacy.aa_arms.clone();
    w.legacy_aa_ci95 = legacy.aa_ci95;
    w.legacy_outside_noise = legacy.outside_noise;

    // Descriptive per-operation figure over every verified pair.
    if let Some(f) = wall_figures(&data.all) {
        w.ratio_b_over_a = Some(f.ratio.estimate);
        w.ns_per_gae_a = Some(f.ns_per_gae_a);
        w.ns_per_gae_b = Some(f.ns_per_gae_b);
    }
    if data.below > 0 {
        w.reasons.push(format!(
            "{} of {} pairs did not reach {}",
            data.below,
            data.pairs,
            required.name()
        ));
    }
    let admitted = data.admitted.len();
    if admitted < MIN_WALL_PAIRS {
        w.reasons.push(format!(
            "{admitted} admitted pairs; at least {MIN_WALL_PAIRS} are needed"
        ));
        w.status = "descriptive".into();
        return w;
    }
    w.status = "admitted".into();
    let Some(fig) = wall_figures(&data.admitted) else {
        w.reasons
            .push("the per-operation wall figure is undefined on the admitted pairs".into());
        w.status = "descriptive".into();
        return w;
    };
    w.ratio_b_over_a = Some(fig.ratio.estimate);
    w.ns_per_gae_a = Some(fig.ns_per_gae_a);
    w.ns_per_gae_b = Some(fig.ns_per_gae_b);
    w.ci95 = fig.ratio.ci95_log();
    w.ci_method = Some("linearised_t_pairs".into());
    w.dof = Some(fig.ratio.df);
    w.total_wall_ratio = fig.total.map(|t| t.estimate);
    w.total_wall_ci95 = fig.total.and_then(|t| t.ci95_log());

    // The noise floor and bias correction.
    let mut arms: BTreeMap<String, (String, String)> = BTreeMap::new();
    for r in recs {
        arms.entry(r.arm.clone())
            .or_insert_with(|| (r.method.method_id.clone(), r.role.clone()));
    }
    let arms: Vec<(String, String, String)> = arms
        .into_iter()
        .map(|(n, (m, role))| (n, m, role))
        .collect();
    let method_of = |n: &str| arms.iter().find(|x| x.0 == n).map(|x| x.1.clone());
    if method_of(a).is_some() && method_of(a) == method_of(b) {
        w.self_aa = true;
        w.reasons
            .push("the compared arms run one method: this is itself an A/A control".into());
        return w;
    }
    match choose_aa(&arms, a, b) {
        None => w
            .reasons
            .push("no pair of arms runs one method: no A/A noise floor or bias correction".into()),
        Some((den, num, why)) => {
            let aa = wall_data(recs, &den, &num, required);
            w.aa_arms = Some((den.clone(), num.clone()));
            w.aa_selection = Some(why.into());
            w.aa_pairs = aa.admitted.len() as u64;
            if aa.admitted.len() < MIN_WALL_PAIRS {
                w.reasons.push(format!(
                    "the A/A arms ({den}, {num}) have {} admitted pairs; at least {MIN_WALL_PAIRS} are needed for a noise floor",
                    aa.admitted.len()
                ));
            } else if let Some(af) = wall_figures(&aa.admitted) {
                w.aa_median_ratio = legacy.aa_median;
                w.aa_ratio = Some(af.ratio.estimate);
                w.aa_ci95 = af.ratio.ci95_log();
                if let Some(c) = divide_independent(&fig.ratio, &af.ratio) {
                    w.corrected_ratio = Some(c.estimate);
                    w.corrected_ci95 = c.ci95_log();
                    w.corrected_dof = Some(c.df);
                    w.outside_noise = w.corrected_ci95.map(|(lo, hi)| lo > 1.0 || hi < 1.0);
                }
            }
        }
    }
    w
}

fn level_points(level: Level) -> u8 {
    match level {
        Level::L0 => 0,
        Level::L1 => 40,
        Level::L2 => 75,
        Level::L3 => 90,
    }
}

fn measurement_quality(
    recs: &[Record],
    a: &str,
    b: &str,
    same_session: bool,
    required: Level,
    wall: &WallResult,
    hz: u64,
) -> MeasurementQuality {
    let mut q = MeasurementQuality {
        score: None,
        rubric: "average paired isolation points (L0=0, L1=40, L2=75, L3=90); +10 only for an L2-or-higher A/A 95% interval containing 1 within [0.9,1.1]".into(),
        paired_attempts: 0,
        verified_pairs: 0,
        admitted_pairs: 0,
        levels: BTreeMap::new(),
        p95_foreign_busy_percent: None,
        p95_run_queue_percent: None,
        contention_observed_runs: 0,
        notes: Vec::new(),
    };
    if !same_session {
        q.notes
            .push("different sessions: no shared host window for a fidelity score".into());
        return q;
    }
    let mut by: BTreeMap<(String, u32), (Option<&Record>, Option<&Record>)> = BTreeMap::new();
    for r in recs.iter().filter(|r| !r.warmup) {
        let key = (r.workload.workload_id.clone(), r.round);
        if r.arm == a {
            by.entry(key).or_default().0 = Some(r);
        } else if r.arm == b {
            by.entry(key).or_default().1 = Some(r);
        }
    }
    let mut points = 0u64;
    let mut foreign = Vec::new();
    let mut delay = Vec::new();
    for (_, (ra, rb)) in by {
        let (Some(ra), Some(rb)) = (ra, rb) else {
            continue;
        };
        q.paired_attempts += 1;
        if !ra.counts() || !rb.counts() {
            *q.levels.entry("unverified".into()).or_insert(0) += 1;
            continue;
        }
        q.verified_pairs += 1;
        let level = ra.isolation.level.min(rb.isolation.level);
        *q.levels.entry(level.name().into()).or_insert(0) += 1;
        points += level_points(level) as u64;
        if level >= required.max(Level::L2) {
            q.admitted_pairs += 1;
        }
        for r in [ra, rb] {
            let mut foreign_seen = false;
            let mut delay_seen = false;
            let wall_s = r.time.process_wall_ns as f64 / 1e9;
            if hz > 0 && wall_s > 0.0 {
                if let Some(ticks) = r.isolation.foreign_busy_ticks {
                    foreign.push(100.0 * (ticks.max(0) as f64 / hz as f64) / wall_s);
                    foreign_seen = true;
                }
            }
            if let (Some(s), Some(w)) = (r.time.schedstat_solve, r.time.solve_wall_ns) {
                if w > 0 {
                    delay.push(100.0 * s.run_delay_ns as f64 / w as f64);
                    delay_seen = true;
                }
            }
            if foreign_seen && delay_seen {
                q.contention_observed_runs += 1;
            }
        }
    }
    q.p95_foreign_busy_percent = crate::cryptanalysis::ecbench::stats::quantile(&foreign, 0.95);
    q.p95_run_queue_percent = crate::cryptanalysis::ecbench::stats::quantile(&delay, 0.95);
    if let Some(mean_points) = points.checked_div(q.paired_attempts) {
        // The noise floor: the session's A/A interval, or this comparison's
        // own interval when the compared arms are themselves an A/A.
        let aa_interval = wall
            .aa_ci95
            .or(if wall.self_aa && wall.status == "admitted" {
                wall.ci95
            } else {
                None
            });
        let calibrated = required >= Level::L2
            && matches!(aa_interval, Some((lo, hi)) if (0.9..=1.0).contains(&lo) && (1.0..=1.1).contains(&hi));
        q.score = Some((mean_points as u8).saturating_add(if calibrated { 10 } else { 0 }));
        if required < Level::L2 {
            q.notes
                .push("A/A bonus unavailable: the spec permits wall runs below L2".into());
        } else if aa_interval.is_none() {
            q.notes
                .push("no admitted A/A interval; calibration bonus unavailable".into());
        }
        if q.contention_observed_runs < q.verified_pairs * 2 {
            q.notes.push("contention telemetry is incomplete".into());
        }
    } else {
        q.notes.push("no paired attempts".into());
    }
    q
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::stats::{t_975, Resampler};
    use serde_json::Value;

    fn session(id: &str, status: &str) -> Session {
        let mut v: Value = serde_json::from_str(include_str!(
            "../../../research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/session.json"
        ))
        .unwrap();
        v["session_id"] = json!(id);
        v["status"] = json!(status);
        serde_json::from_value(v).unwrap()
    }

    /// A measured record built from a real one, with the fields a
    /// comparison reads replaced.
    #[allow(clippy::too_many_arguments)]
    fn rec(
        arm: &str,
        method: &str,
        role: &str,
        wid: &str,
        round: u32,
        status: &str,
        s: Option<f64>,
        wall_ns: u64,
    ) -> Record {
        let line = include_str!(
            "../../../research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/records.jsonl"
        )
        .lines()
        .next()
        .unwrap();
        let mut v: Value = serde_json::from_str(line).unwrap();
        v["arm"] = json!(arm);
        v["role"] = json!(role);
        v["warmup"] = json!(false);
        v["round"] = json!(round);
        v["method"]["method_id"] = json!(method);
        v["workload"]["workload_id"] = json!(wid);
        v["outcome"]["status"] = json!(status);
        v["cost"]["lower_bound"] = json!(false);
        v["cost"]["unpriced"] = json!([]);
        v["counters"] = json!({});
        v["phases"] = json!([]);
        v["isolation"]["level"] = json!("L2");
        v["time"]["solve_wall_ns"] = json!(if status == "timeout" {
            Value::Null
        } else {
            json!(wall_ns)
        });
        v["time"]["process_wall_ns"] = json!(wall_ns + 1_000_000);
        v["algorithm_seed"] = json!(format!("{round}"));
        let mut r: Record = serde_json::from_value(v).unwrap();
        let sqrt_r = (r.workload.curve.r as f64).sqrt();
        r.cost.s = s;
        r.cost.total_gae = s.map(|s| s * sqrt_r);
        r.boundaries.ratio_to_floor = s.map(|s| s / r.boundaries.floor_s);
        r
    }

    /// A session shaped like a real one: `K` workloads × `rounds` rounds of
    /// reference `A` (rho-like), `B` (cost `b_over_a` times A's per target,
    /// `ns_b_over_a` times A's per operation) and a control `A2` of A's
    /// method with `pos` times A's per-operation cost.
    fn synth(
        k: usize,
        rounds: u32,
        b_over_a: f64,
        ns_b_over_a: f64,
        pos: f64,
        seed: u64,
    ) -> Vec<Record> {
        let mut rng = Resampler::new(seed);
        let unif = |rng: &mut Resampler| (rng.below(1 << 30) as f64 + 0.5) / (1u64 << 30) as f64;
        let mut out = Vec::new();
        for w in 0..k {
            let wid = format!("W{w:02}");
            for round in 1..=rounds {
                let s_a = 0.886 * (-2.0 * (1.0 - unif(&mut rng)).ln()).sqrt() / 1.2533;
                let jit = |rng: &mut Resampler| 1.0 + 0.02 * (unif(rng) - 0.5);
                let ns = 1000.0;
                let wall = |s: f64, per: f64, rng: &mut Resampler| {
                    // wall = S · √r · ns/GAE; √r is folded into the constant.
                    (s * per * jit(rng) * 1e6) as u64 + 1
                };
                let w_a = wall(s_a, ns, &mut rng);
                let s_b = s_a * b_over_a;
                let w_b = wall(s_b, ns * ns_b_over_a, &mut rng);
                let w_c = wall(s_a, ns * pos, &mut rng);
                out.push(rec(
                    "A",
                    "M1",
                    "reference",
                    &wid,
                    round,
                    "verified",
                    Some(s_a),
                    w_a,
                ));
                out.push(rec(
                    "B",
                    "M2",
                    "candidate",
                    &wid,
                    round,
                    "verified",
                    Some(s_b),
                    w_b,
                ));
                out.push(rec(
                    "A2",
                    "M1",
                    "control",
                    &wid,
                    round,
                    "verified",
                    Some(s_a),
                    w_c,
                ));
            }
        }
        out
    }

    fn cmp(
        recs: &[Record],
        a: &str,
        b: &str,
        status: &str,
        allow: bool,
    ) -> Result<Comparison, String> {
        let s = session("S1", status);
        compare_loaded(
            &s,
            recs,
            &s,
            recs,
            a,
            b,
            Level::L2,
            &CompareOptions {
                resamples: 300,
                seed: 7,
                allow_interrupted: allow,
            },
        )
    }

    #[test]
    fn identical_arms_give_a_unit_ratio_and_a_point_interval() {
        let strata = vec![
            vec![Pair::exact(1.0, 1.0), Pair::exact(2.0, 2.0)],
            vec![Pair::exact(3.0, 3.0)],
        ];
        let r = ratio_report(&strata, 500, 1);
        assert_eq!(r.ratio, Some(1.0));
        assert_eq!(r.ci95, Some((1.0, 1.0)));
        assert_eq!(r.ci95_bootstrap, Some((1.0, 1.0)));
        assert_eq!(r.ci_method, "linearised_t");
        assert_eq!(r.bootstrap_method, "cluster");
        assert!(r.indicative);
        assert!(r.bound.is_none());
    }

    #[test]
    fn ratio_of_sums_is_the_totals_ratio() {
        let pairs = [(1.0, 2.0), (8.0, 9.0)];
        assert_eq!(ratio_of_sums(pairs.iter().copied()), Some(11.0 / 9.0));
        assert_eq!(ratio_of_sums(std::iter::empty()), None);
    }

    #[test]
    fn one_workload_is_labelled_within_and_indicative() {
        let strata = vec![vec![
            Pair::exact(1.0, 1.1),
            Pair::exact(2.0, 2.0),
            Pair::exact(1.5, 1.8),
        ]];
        let r = ratio_report(&strata, 200, 1);
        assert_eq!(r.ci_method, "within_t");
        assert_eq!(r.scope.as_deref(), Some("within_workload"));
        assert!(r.indicative);
        assert!(r.ci95.is_some());
        assert_eq!(r.dof, Some(2.0));
    }

    #[test]
    fn five_workloads_are_not_indicative() {
        let strata: Vec<Vec<Pair>> = (0..5)
            .map(|k| vec![Pair::exact(1.0 + k as f64, 2.0 + k as f64 * 1.1)])
            .collect();
        let r = ratio_report(&strata, 100, 1);
        assert!(!r.indicative);
        assert_eq!(r.dof, Some(4.0));
        let (lo, hi) = r.ci95.unwrap();
        assert!(lo < r.ratio.unwrap() && r.ratio.unwrap() < hi);
    }

    #[test]
    fn censored_pairs_make_the_ratio_a_bound() {
        let mut p = Pair::exact(1.0, 5.0);
        p.b_censored = true;
        let strata = vec![vec![Pair::exact(1.0, 1.0), p.clone()]];
        let r = ratio_report(&strata, 100, 1);
        assert_eq!(r.censored_pairs, 1);
        assert_eq!(r.bound.as_deref(), Some("lower"));
        assert_eq!(r.ratio, Some(3.0));
        assert_eq!(r.verified_only_ratio, Some(1.0));
        let mut q = Pair::exact(4.0, 1.0);
        q.a_censored = true;
        assert_eq!(bound_direction(&[q.clone()]).as_deref(), Some("upper"));
        assert_eq!(bound_direction(&[p, q]).as_deref(), Some("indeterminate"));
        assert_eq!(bound_direction(&[Pair::exact(1.0, 1.0)]), None);
    }

    #[test]
    fn band_ranks_only_where_the_order_holds() {
        let mk = |a: [f64; 3], b: [f64; 3]| {
            let mut p = Pair::exact(a[0], b[0]);
            p.band = (0..3).map(|i| (Some(a[i]), Some(b[i]))).collect();
            vec![vec![p]]
        };
        let r = band_report(&mk([1.0, 2.0, 11.0], [2.0, 3.0, 12.0]), "A", "B", 0).unwrap();
        assert_eq!(r.order, "B > A at every weight");
        let r = band_report(&mk([1.0, 2.0, 11.0], [2.0, 2.5, 6.0]), "A", "B", 0).unwrap();
        assert_eq!(r.order, "unresolved inside the band");
        let mut p = Pair::exact(1.0, 2.0);
        p.band = vec![(Some(1.0), Some(2.0)), (None, Some(2.5)), (None, Some(3.0))];
        let r = band_report(&[vec![p]], "A", "B", 0).unwrap();
        assert!(r.order.starts_with("unrankable"));
    }

    #[test]
    fn tail_flags_a_dominant_run() {
        let flat: Vec<(String, f64)> = (0..8).map(|i| (format!("r{i}"), 1.0)).collect();
        let t = tail_diagnostics(&flat).unwrap();
        assert!(!t.heavy_tail);
        assert_eq!(t.max_over_median, Some(1.0));
        let mut heavy = flat.clone();
        heavy.push(("big".into(), 30.0));
        let t = tail_diagnostics(&heavy).unwrap();
        assert!(t.heavy_tail);
        assert_eq!(t.top_run_id, "big");
        assert!((t.top_run_share - 30.0 / 38.0).abs() < 1e-12);
        assert_eq!(t.max_over_median, Some(30.0));
        assert!(tail_diagnostics(&[]).is_none());
    }

    #[test]
    fn aa_selection_prefers_arm_a_then_reference() {
        let arms = |v: &[(&str, &str, &str)]| -> Vec<(String, String, String)> {
            v.iter()
                .map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string()))
                .collect()
        };
        // Alphabetical order would pick (aa1, aa2); arm A is `rho`.
        let v = arms(&[
            ("aa1", "M2", "baseline"),
            ("aa2", "M2", "control"),
            ("bsgs", "M3", "candidate"),
            ("rho", "M1", "reference"),
            ("rho-aa", "M1", "control"),
        ]);
        assert_eq!(
            choose_aa(&v, "rho", "bsgs"),
            Some(("rho".into(), "rho-aa".into(), "includes arm A"))
        );
        // Arm A is `bsgs` (no twin): fall to the reference's pair.
        assert_eq!(
            choose_aa(&v, "bsgs", "aa1"),
            Some(("rho".into(), "rho-aa".into(), "includes the reference arm"))
        );
        // Neither: first alphabetical.
        let w = arms(&[
            ("p", "M1", "candidate"),
            ("q", "M1", "control"),
            ("x", "M2", "candidate"),
            ("y", "M3", "candidate"),
        ]);
        assert_eq!(
            choose_aa(&w, "x", "y"),
            Some(("p".into(), "q".into(), "first alphabetical"))
        );
        // The compared arms are an A/A themselves.
        assert_eq!(choose_aa(&w, "p", "q"), None);
        assert_eq!(
            choose_aa(&arms(&[("a", "M1", "r"), ("b", "M2", "c")]), "a", "b"),
            None
        );
    }

    #[test]
    fn per_operation_wall_ratio_ignores_work_variance() {
        // Equal cost per operation, wildly different work per pair: the
        // median of per-pair wall ratios is far from 1, ns/GAE is 1.
        let mut rng = Resampler::new(11);
        let unif = |rng: &mut Resampler| (rng.below(1 << 30) as f64 + 0.5) / (1u64 << 30) as f64;
        let pairs: Vec<WallPair> = (0..15)
            .map(|_| {
                let ga = 1.0 + 4.0 * unif(&mut rng);
                let gb = 1.0 + 4.0 * unif(&mut rng);
                WallPair {
                    a_wall: ga * 1000.0,
                    b_wall: gb * 1000.0,
                    a_gae: ga,
                    b_gae: gb,
                }
            })
            .collect();
        let f = wall_figures(&pairs).unwrap();
        assert!((f.ratio.estimate - 1.0).abs() < 1e-12);
        assert!(f.ratio.se.abs() < 1e-12);
        let per_pair: Vec<f64> = pairs.iter().map(|p| p.b_wall / p.a_wall).collect();
        let m = median(&per_pair).unwrap();
        let spread = per_pair.iter().cloned().fold(f64::MIN, f64::max)
            / per_pair.iter().cloned().fold(f64::MAX, f64::min);
        assert!(spread > 3.0, "per-pair wall ratios spread {spread}");
        assert!((m - 1.0).abs() > 1e-3);
        // The end-to-end total keeps the work variance.
        assert!((f.total.unwrap().estimate - 1.0).abs() > 1e-3);
    }

    #[test]
    fn wall_interval_covers_a_five_percent_per_op_difference() {
        // 5 workloads × 3 rounds, 3 % timing jitter, B truly 1.05× per op
        // (wall.py of the deep dive): the interval covers 1.05.
        let mut rng = Resampler::new(0x3A11);
        let unif = |rng: &mut Resampler| (rng.below(1 << 30) as f64 + 0.5) / (1u64 << 30) as f64;
        let (reps, mut hit, mut width) = (2000, 0, Vec::new());
        for _ in 0..reps {
            let pairs: Vec<WallPair> = (0..15)
                .map(|_| {
                    let ga = 0.886 * (-2.0 * (1.0 - unif(&mut rng)).ln()).sqrt() / 1.2533;
                    let gb = 0.9 * 0.886 * (-2.0 * (1.0 - unif(&mut rng)).ln()).sqrt() / 1.2533;
                    let j =
                        |rng: &mut Resampler| 1.0 + 0.03 * 3f64.sqrt() * 2.0 * (unif(rng) - 0.5);
                    WallPair {
                        a_wall: ga * j(&mut rng),
                        b_wall: gb * 1.05 * j(&mut rng),
                        a_gae: ga,
                        b_gae: gb,
                    }
                })
                .collect();
            let f = wall_figures(&pairs).unwrap();
            let (lo, hi) = f.ratio.ci95_log().unwrap();
            hit += (lo <= 1.05 && 1.05 <= hi) as usize;
            width.push(hi - lo);
        }
        let cov = hit as f64 / reps as f64;
        assert!((0.92..=0.98).contains(&cov), "wall coverage {cov}");
        // The interval is tight enough to separate 1.05 from 1.
        assert!(median(&width).unwrap() < 0.10, "width {:?}", median(&width));
    }

    #[test]
    fn corrected_interval_removes_the_position_bias() {
        // B/A = 1.05 with a 5 % position bias that the A/A also shows:
        // the corrected figure is 1 and not outside noise; the uncorrected
        // interval excludes 1.
        let pairs = |ns_b: f64| -> Vec<WallPair> {
            (0..12)
                .map(|i| {
                    let g = 1.0 + 0.1 * (i % 3) as f64;
                    let j = 1.0 + 0.002 * ((i * 7) % 5) as f64;
                    WallPair {
                        a_wall: g * 1000.0,
                        b_wall: g * 1000.0 * ns_b * j,
                        a_gae: g,
                        b_gae: g,
                    }
                })
                .collect()
        };
        let ba = wall_figures(&pairs(1.05)).unwrap().ratio;
        let aa = wall_figures(&pairs(1.05)).unwrap().ratio;
        let (lo, hi) = ba.ci95_log().unwrap();
        assert!(lo > 1.0 && hi > 1.0);
        let c = divide_independent(&ba, &aa).unwrap();
        let (clo, chi) = c.ci95_log().unwrap();
        assert!((c.estimate - 1.0).abs() < 1e-12);
        assert!(clo < 1.0 && chi > 1.0);
        // A real 5 % effect on top of a 2 % bias still clears the noise.
        let b2 = wall_figures(&pairs(1.07)).unwrap().ratio;
        let a2 = wall_figures(&pairs(1.02)).unwrap().ratio;
        let c = divide_independent(&b2, &a2).unwrap();
        let (clo, _) = c.ci95_log().unwrap();
        assert!(clo > 1.0, "corrected lower {clo}");
        assert!(t_975(c.df).is_some());
    }

    #[test]
    fn compare_reports_the_new_statistics_end_to_end() {
        // 3 workloads × 4 rounds; B costs twice A in ops and 5 % more per
        // op; the control A2 shares A's method and is 2 % slower per op.
        let recs = synth(3, 4, 2.0, 1.05, 1.02, 5);
        let c = cmp(&recs, "A", "B", "complete", false).unwrap();
        assert_eq!(c.schema, COMPARISON_SCHEMA_V2);
        assert!(c.admissible);
        assert_eq!(c.ops.ci_method, "linearised_t");
        assert_eq!(c.ops.workloads, 3);
        assert_eq!(c.ops.dof, Some(2.0));
        assert!(c.ops.indicative);
        assert!(c.ops.ci95_bootstrap.is_some());
        assert!((c.ops.ratio_b_over_a.unwrap() - 2.0).abs() < 1e-9);
        assert_eq!(c.ops.censored_pairs, 0);
        assert!(c.ops.band.is_some());
        assert_eq!(c.wall.status, "admitted");
        assert_eq!(c.wall.aa_arms, Some(("A".into(), "A2".into())));
        assert_eq!(c.wall.aa_selection.as_deref(), Some("includes arm A"));
        let r = c.wall.ratio_b_over_a.unwrap();
        assert!((r - 1.05).abs() < 0.02, "ns/GAE ratio {r}");
        let aa = c.wall.aa_ratio.unwrap();
        assert!((aa - 1.02).abs() < 0.02, "A/A ratio {aa}");
        assert!(c.wall.corrected_ratio.is_some());
        assert!(c.wall.legacy_ci95.is_some());
        assert!(c.wall.median_ratio.is_some());
        assert!(c.verdict.contains("INDICATIVE"), "{}", c.verdict);
        assert!(c.verdict.contains("ns/GAE"), "{}", c.verdict);
        // The compared arms are themselves an A/A control.
        let s = cmp(&recs, "A", "A2", "complete", false).unwrap();
        assert!(s.wall.self_aa);
        assert!(s.wall.outside_noise.is_none());
        // The quality bonus reads the control's own interval.
        assert!(s.quality.is_some());
        // Round trip.
        let back: Comparison = serde_json::from_str(&serde_json::to_string(&c).unwrap()).unwrap();
        assert_eq!(back.ops.ci95, c.ops.ci95);
    }

    #[test]
    fn timeouts_and_exhausted_runs_are_censored_not_dropped() {
        let mut recs = synth(2, 3, 2.0, 1.0, 1.0, 9);
        // B times out on workload 0 round 2; A is exhausted on workload 1
        // round 1.
        for r in recs.iter_mut() {
            if r.arm == "B" && r.workload.workload_id == "W00" && r.round == 2 {
                r.outcome.status = "timeout".into();
                r.cost.s = None;
                r.cost.total_gae = None;
            }
        }
        let c = cmp(&recs, "A", "B", "complete", false).unwrap();
        assert_eq!(c.ops.status, "incomplete");
        assert_eq!(c.ops.censored_pairs, 1);
        assert_eq!(c.ops.bound.as_deref(), Some("lower"));
        assert_eq!(c.ops.pairs, 6, "the censored pair is kept");
        assert_eq!(c.b.censored, 1);
        assert!(c.verdict.contains("BOUND"), "{}", c.verdict);
        let vo = c.ops.verified_only_ratio.unwrap();
        // The timeout enters at the cost its wall time implies, close to
        // what the run would have cost: the bound stays near the truth
        // instead of the verified-only figure silently ignoring the run.
        assert!((vo - 2.0).abs() < 1e-9);
        assert!((c.ops.ratio_b_over_a.unwrap() - 2.0).abs() < 0.1);

        let mut recs = synth(2, 3, 2.0, 1.0, 1.0, 9);
        for r in recs.iter_mut() {
            if r.arm == "A" && r.workload.workload_id == "W01" && r.round == 1 {
                r.outcome.status = "exhausted".into();
            }
        }
        let c = cmp(&recs, "A", "B", "complete", false).unwrap();
        assert_eq!(c.ops.bound.as_deref(), Some("upper"));
        assert_eq!(c.ops.censored_pairs, 1);
        assert_eq!(c.a.censored, 1);
        // An exhausted run enters at the cost it spent.
        assert!((c.ops.ratio_b_over_a.unwrap() - 2.0).abs() < 1e-9);
        assert!(c.verdict.contains("upper bound"), "{}", c.verdict);
    }

    #[test]
    fn compare_refuses_a_session_that_is_not_complete() {
        let recs = synth(2, 3, 2.0, 1.0, 1.0, 3);
        let e = cmp(&recs, "A", "B", "interrupted", false).unwrap_err();
        assert!(e.contains("not complete"), "{e}");
        assert!(e.contains("--allow-interrupted"), "{e}");
        let c = cmp(&recs, "A", "B", "interrupted", true).unwrap();
        assert!(c.allow_interrupted);
        assert!(!c.admissible);
        assert_eq!(c.a_session_status.as_deref(), Some("interrupted"));
        assert!(
            c.verdict.contains("NOT admissible: session not complete"),
            "{}",
            c.verdict
        );
        assert_ne!(
            c.comparison_id,
            cmp(&recs, "A", "B", "complete", false)
                .unwrap()
                .comparison_id
        );
    }

    #[test]
    fn v1_comparison_files_still_deserialize() {
        let v1: Comparison = serde_json::from_str(include_str!(
            "../../../research/ecbench_rootfind_parity_20261013/sessions/claim-n13-v2/comparisons/rho-strong__ic-c2-frob-counted-m2.json"
        ))
        .unwrap();
        assert_eq!(v1.schema, COMPARISON_SCHEMA);
        assert!(is_comparison_schema(&v1.schema));
        assert!(v1.admissible);
        assert!(v1.ops.ci95_bootstrap.is_none() && v1.ops.bound.is_none());
        assert!(v1.wall.ratio_b_over_a.is_none());
    }
}
