//! `ecbench verify`: re-derive a session from its own files.
//!
//! The audit trusts nothing a session says about itself that it can
//! recompute:
//!
//! - every file hashes to what `session.json` says it does, and the
//!   session id recomputes from its preimage;
//! - the spec expands to the same spec id, workloads, execution order
//!   and seeds as `plan.json`;
//! - the host capsule's class id recomputes from its stable facts, and the
//!   session's and every record's environment class and binary hash are
//!   the capsule's;
//! - every record is sealed, in sequence, matches its planned execution,
//!   arm, role, method and workload, and keeps the stderr it hashed;
//! - every record is internally consistent: phases sum to the total (bar
//!   the documented historical exceptions), the automorphisms used do not
//!   exceed those available, the field-operation block is present where
//!   the method counts it, and the determinism it declares is the one the
//!   method registry gives it;
//! - every recovered logarithm is re-checked here, `[k]G = Q`, and the
//!   record's status must agree with that check; a failed run's status
//!   must be consistent with what it records (a timeout has no total, a
//!   kill-signal exit and a wall time at or past the timeout);
//! - every run's isolation level is regraded from its observations under
//!   the grading version the session was recorded with, and an
//!   unsupported version fails the audit;
//! - `status_counts` and `executions_planned` follow from the records and
//!   the plan;
//! - with `--replay N`, N randomly chosen measured deterministic runs are
//!   re-executed (the seed is the auditor's, drawn from the operating
//!   system unless given with `--replay-seed`, and is recorded in the
//!   receipt) and must reproduce their answer, total, phases, counters
//!   and factor base exactly, and their field-operation block when both
//!   the record and the replay carry one (a record written before the
//!   block existed has none, and unknown is not a disagreement).  An
//!   exhausted run must replay to exhausted and an errored one to the
//!   same error.  Operation counts do not depend on the host, so a replay
//!   on any machine is an independent check of the figure.
//!
//! An interrupted session is reported as such, prominently, in the
//! receipt; a torn final line of `records.jsonl` yields a receipt with a
//! problem, not an error with no receipt.
//!
//! The receipt names every file by hash and every check by result; its
//! own SHA-256 is the replay certificate a claim cites.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::cryptanalysis::ecbench::canonical::{sha256_hex, short_id, splitmix64};
use crate::cryptanalysis::ecbench::host::{self, recompute_class, HostCapsule};
use crate::cryptanalysis::ecbench::methods::{MethodSpec, PhaseRecord, ResolvedMethod};
use crate::cryptanalysis::ecbench::record::{
    check_line_seal, child_main, floor_s, from_str_legacy_floats, grade_for_version,
    json_roundtrip, ChildInput, Grade, GradeInput, Record, CHILD_INPUT_SCHEMA,
};
use crate::cryptanalysis::ecbench::runner::{read_session, PlanDoc, Session};
use crate::cryptanalysis::ecbench::spec::{plan, Arm, Execution, Level, Plan, Spec};
use crate::cryptanalysis::ecbench::workload::Workload;

/// Replay every measured deterministic run.
pub const REPLAY_ALL: usize = usize::MAX;

/// What makes a workload the workload: its id and every field the id is
/// a hash of, plus the planted answer.  The registry-derived facts
/// (`registered`, `ec1`, `curve_uid`) and the constructor's text are not
/// in it, so registering a curve later never breaks an honest session.
fn workload_identity(w: &Workload) -> impl PartialEq + std::fmt::Debug + '_ {
    (
        (&w.workload_id, &w.workload_sha256, &w.target, w.planted),
        (
            &w.curve.icv1,
            w.curve.r,
            w.curve.cofactor,
            &w.curve.generator,
        ),
        (&w.target_law, w.target_seed, w.target_index),
    )
}

/// What makes an arm the arm: its name, role and method identity, not the
/// method's descriptive `entry` text.
fn arm_identity(a: &Arm) -> impl PartialEq + std::fmt::Debug + '_ {
    (
        &a.name,
        a.role,
        &a.method.id,
        &a.method.params,
        &a.method.method_id,
        &a.method.method_sha256,
    )
}

/// Equal to within a few ulps of a parsed float (records are parsed with a
/// float reader that is not always correctly rounded).
fn close(a: f64, b: f64) -> bool {
    a == b || (a - b).abs() <= 1e-12 * a.abs().max(b.abs())
}

// ── Determinism, from the method registry ──────────────────────────

/// Whether a method's counts are a function of the workload and the seed.
/// The registry (the method's id and parameters) decides, never the record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Determinism {
    /// Counts reproduce exactly: the record must say so, and it replays.
    Always,
    /// The solver runs under a wall-clock budget
    /// (`solver_budget_seconds > 0`): the run is nondeterministic when a
    /// solver was in play and deterministic when none was, so the record
    /// may say either, and replays only when it says deterministic.
    Budgeted,
    /// Never reproducible (a cold wall watchdog decides the outcome): the
    /// record must say so and is not replayed.
    Never,
}

/// The determinism the registry gives a resolved method.
pub fn method_determinism(m: &ResolvedMethod) -> Determinism {
    if m.id.starts_with("ic.cm_folded_") {
        return Determinism::Never;
    }
    let budget = m
        .params
        .get("solver_budget_seconds")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    if budget > 0 {
        Determinism::Budgeted
    } else {
        Determinism::Always
    }
}

/// Committed sessions whose failed outcomes (`exhausted`, `wrong_answer`,
/// `error`) were recorded by code whose failure paths have since changed,
/// and which were never held to replay before failed outcomes were: their
/// failed runs are checked statically and their verified runs replay as
/// every other session's do.  Session id, and why.
pub const FAILED_OUTCOME_REPLAY_EXEMPT: &[(&str, &str)] = &[
    (
        "ECBS1hedc734e1870d",
        "factor_bases_20261013/koblitz17-algebraic: the two exhausted runs' solver work is pinned now, wall-priced then",
    ),
    (
        "ECBS1h4c7b9db8a389",
        "rootfinding_arity_20261013/prime20-b64-m2-vs-m3: `rootfind-subtract` refused prime curves then and runs on them now",
    ),
    (
        "ECBS1h302934ff99ed",
        "f6_ic_ecbench_ladder_20261005/n7: 48 of the 72 exhausted runs count other phase work under the current code",
    ),
];

/// Whether the audit can re-execute this record's run and expect the same
/// outcome: a measured (not warm-up) run of a method the registry calls
/// deterministic, whose outcome is one a replay can reproduce.  `timeout`,
/// `crashed`, `interrupted` and `spawn_error` are the host's or the
/// runner's doing and are checked statically instead, as are the failed
/// outcomes of the sessions in [`FAILED_OUTCOME_REPLAY_EXEMPT`].
fn replayable(r: &Record) -> bool {
    let failed_outcome = matches!(
        r.outcome.status.as_str(),
        "wrong_answer" | "exhausted" | "error"
    );
    !r.warmup
        && (r.outcome.status == "verified"
            || (failed_outcome
                && !FAILED_OUTCOME_REPLAY_EXEMPT
                    .iter()
                    .any(|(s, _)| *s == r.session_id)))
        && match method_determinism(&r.method) {
            Determinism::Always => true,
            Determinism::Budgeted => r.cost.total_gae.is_some() && r.cost.deterministic,
            Determinism::Never => false,
        }
}

// ── The legacy solver-rounding exception ───────────────────────────

/// Replay detail for a record whose gae carries the pre-fix rounding.
pub const LEGACY_IDENTICAL: &str =
    "identical (legacy gae rounding, reproduced from the record's solver term)";

/// The sealed records the legacy solver-rounding exception exists for, as
/// `(session_id, record_id)`.  Nothing else may use it: a record that is
/// not on this list and does not replay exactly does not reproduce.
///
/// They are the 48 `ic.pipeline` records of
/// `research/ecbench_yield_sweep_20261004/sessions/koblitz` (the
/// `sub6`/`sub8` descent-algebraic arms `da-buch`, `da-crossbred` and
/// `da-sat`), written before 2026-10-05 by a binary that removed the
/// solver's wall-priced term `w` by subtraction (see
/// [`legacy_solver_rounding`]).  The list is exactly the measured records
/// of the session whose replay with the exception disabled differs from
/// the record in the total and the relations phase alone.  The session's
/// other wall-priced records (the `da-f4` arms, and the later rounds whose
/// records carry unpriced counters the replay no longer reports) differ
/// in more than rounding under the current registry, where those solver
/// conversions are pinned, and are reproduced by nothing here.  The list
/// is closed: a record written after the fix replays exactly, and a forged
/// record has a new record id.
pub const LEGACY_ROUNDING_RECORDS: &[(&str, &str)] = &[
    ("ECBS1h0d1ce1d04e54", "ECR1h078db60c4945"),
    ("ECBS1h0d1ce1d04e54", "ECR1h08779723c02a"),
    ("ECBS1h0d1ce1d04e54", "ECR1h0e871fe1a167"),
    ("ECBS1h0d1ce1d04e54", "ECR1h0ee7af855a5a"),
    ("ECBS1h0d1ce1d04e54", "ECR1h19676112de08"),
    ("ECBS1h0d1ce1d04e54", "ECR1h217c90ad1444"),
    ("ECBS1h0d1ce1d04e54", "ECR1h267d21d2f14e"),
    ("ECBS1h0d1ce1d04e54", "ECR1h2a89d07d4e01"),
    ("ECBS1h0d1ce1d04e54", "ECR1h2f127f569526"),
    ("ECBS1h0d1ce1d04e54", "ECR1h324309026b1c"),
    ("ECBS1h0d1ce1d04e54", "ECR1h3291465d01ff"),
    ("ECBS1h0d1ce1d04e54", "ECR1h47e7fcee10a7"),
    ("ECBS1h0d1ce1d04e54", "ECR1h547bce5b48e7"),
    ("ECBS1h0d1ce1d04e54", "ECR1h572b8f988595"),
    ("ECBS1h0d1ce1d04e54", "ECR1h57f49db3b269"),
    ("ECBS1h0d1ce1d04e54", "ECR1h59734b3718c0"),
    ("ECBS1h0d1ce1d04e54", "ECR1h5b34ee29e2c0"),
    ("ECBS1h0d1ce1d04e54", "ECR1h635bae02e0b6"),
    ("ECBS1h0d1ce1d04e54", "ECR1h691adbaa8944"),
    ("ECBS1h0d1ce1d04e54", "ECR1h6b3532220923"),
    ("ECBS1h0d1ce1d04e54", "ECR1h7053ba67e4db"),
    ("ECBS1h0d1ce1d04e54", "ECR1h71c3c5908aa5"),
    ("ECBS1h0d1ce1d04e54", "ECR1h76b898fbf5de"),
    ("ECBS1h0d1ce1d04e54", "ECR1h7900d184021b"),
    ("ECBS1h0d1ce1d04e54", "ECR1h7dc6d5a21f22"),
    ("ECBS1h0d1ce1d04e54", "ECR1h864c5bf09f1b"),
    ("ECBS1h0d1ce1d04e54", "ECR1h874bde2f19d5"),
    ("ECBS1h0d1ce1d04e54", "ECR1h8fc034d4acc5"),
    ("ECBS1h0d1ce1d04e54", "ECR1h9529e3ab831d"),
    ("ECBS1h0d1ce1d04e54", "ECR1h98081d2e34f1"),
    ("ECBS1h0d1ce1d04e54", "ECR1h99674b84fe9e"),
    ("ECBS1h0d1ce1d04e54", "ECR1h9a8b49573f64"),
    ("ECBS1h0d1ce1d04e54", "ECR1h9cf98ecc2318"),
    ("ECBS1h0d1ce1d04e54", "ECR1ha37817cbd0da"),
    ("ECBS1h0d1ce1d04e54", "ECR1ha96c4772a931"),
    ("ECBS1h0d1ce1d04e54", "ECR1hb1b775dae2b6"),
    ("ECBS1h0d1ce1d04e54", "ECR1hb6db88f9ca72"),
    ("ECBS1h0d1ce1d04e54", "ECR1hc18d835d444a"),
    ("ECBS1h0d1ce1d04e54", "ECR1hc1c022d2f789"),
    ("ECBS1h0d1ce1d04e54", "ECR1hc3445ef4f8c2"),
    ("ECBS1h0d1ce1d04e54", "ECR1hc833631bb099"),
    ("ECBS1h0d1ce1d04e54", "ECR1hcbfd99ffacbe"),
    ("ECBS1h0d1ce1d04e54", "ECR1he032f3a8629d"),
    ("ECBS1h0d1ce1d04e54", "ECR1he610e6f9e3f4"),
    ("ECBS1h0d1ce1d04e54", "ECR1hee60cab8d6c6"),
    ("ECBS1h0d1ce1d04e54", "ECR1hf548891654a4"),
    ("ECBS1h0d1ce1d04e54", "ECR1hf6f52cc1c3e5"),
    ("ECBS1h0d1ce1d04e54", "ECR1hfab435cc456f"),
];

/// Whether `(session_id, record_id)` is one of [`LEGACY_ROUNDING_RECORDS`].
pub fn legacy_rounding_allowed(session_id: &str, record_id: &str) -> bool {
    LEGACY_ROUNDING_RECORDS
        .iter()
        .any(|(s, r)| *s == session_id && *r == record_id)
}

/// The largest rounding difference the exception may absorb, in units of
/// the pre-removal total's ulp (`f64::EPSILON * before`): `(x + w) - w`
/// rounds `x` to the binade of `x + w`, an error of at most half an ulp
/// of `x + w`, and the total's four-term sum adds a little more.
pub const LEGACY_ROUNDING_ULPS: f64 = 4.0;

/// ... and no more than this fraction of the replay's exact total: the
/// solver term is the wall price of seconds of solver time, a few orders
/// of magnitude above the counted work and never a million times above it,
/// so a larger difference is a term chosen to hide a real one.
pub const LEGACY_ROUNDING_MAX_RELATIVE: f64 = 1e-6;

/// The gae figures an `ic.pipeline` record written before the
/// solver-rounding fix carries, from the replay's exact phases.
///
/// Those binaries removed a wall-priced solver term `w` by subtraction,
/// so the relations phase was `(x + w) - w` and the total
/// `(fb + prep + (x + w) + la + ver) - w`: `x` rounded to `w`'s binade.
/// The rounding is a function of `w`, which the record carries
/// (`detail.decomposition.solver.gae`), so it is reproduced here from the
/// record rather than from the replay host's clock.  `w` is accepted only
/// when it also reproduces the record's own pre-removal total
/// (`detail.framework_total_gae_before_solver_removal`), so the record
/// cannot pick a `w` that absorbs a real difference; what stays unchecked
/// is below one ulp of `x + w`.
///
/// This reads `w` from the record, so on its own it proves nothing: the
/// audit applies it only to a record on [`LEGACY_ROUNDING_RECORDS`], and
/// only within [`legacy_rounding_bounded`].
pub fn legacy_solver_rounding(
    exact: &[f64],
    detail: &serde_json::Value,
) -> Option<(Vec<f64>, f64)> {
    let solver = detail.pointer("/decomposition/solver")?;
    let w = solver.get("gae")?.as_f64()?;
    if exact.len() != 5
        || w.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
        || solver.get("priced_by")?.as_str()? == "pinned"
    {
        return None;
    }
    let with_w = exact[2] + w;
    let before = exact[0] + exact[1] + with_w + exact[3] + exact[4];
    let recorded = detail
        .get("framework_total_gae_before_solver_removal")?
        .as_f64()?;
    if json_roundtrip(before).to_bits() != recorded.to_bits() {
        return None;
    }
    let mut phases = exact.to_vec();
    phases[2] = with_w - w;
    Some((phases, before - w))
}

/// Whether the legacy figures differ from the exact ones by no more than
/// the rounding they stand for: [`LEGACY_ROUNDING_ULPS`] ulps of the
/// pre-removal total, and [`LEGACY_ROUNDING_MAX_RELATIVE`] of the exact
/// total, in the relations phase and in the total.
pub fn legacy_rounding_bounded(
    exact_phases: &[f64],
    exact_total: f64,
    legacy_phases: &[f64],
    legacy_total: f64,
    detail: &serde_json::Value,
) -> bool {
    let Some(before) = detail
        .get("framework_total_gae_before_solver_removal")
        .and_then(|v| v.as_f64())
    else {
        return false;
    };
    let bound = (LEGACY_ROUNDING_ULPS * f64::EPSILON * before.abs())
        .min(LEGACY_ROUNDING_MAX_RELATIVE * exact_total.abs());
    exact_phases.len() == legacy_phases.len()
        && exact_phases
            .iter()
            .zip(legacy_phases)
            .all(|(a, b)| (a - b).abs() <= bound)
        && (exact_total - legacy_total).abs() <= bound
}

// ── Grading under the recorded version ─────────────────────────────

/// Re-grade one run under the rules version `version` graded it with: the
/// grades any honest recording under that version can carry (more than one
/// only where a version's rules changed without the version being bumped).
///
/// Versions 2 and later are the record module's frozen graders
/// ([`grade_for_version`]).  Version 1 predates three checks that version 2
/// added (the run CPU's unreserved SMT siblings, the busy time on the
/// reserved idle siblings, and the policy-and-pages NUMA read-back); it is
/// reproduced from version 2 by dropping the blockers only version 2
/// raises.  The committed sessions recorded without a version differ on
/// NUMA placement: the first ones read the cpuset's `Mems_allowed`
/// (`memory allowed on Some("0-1"), not bound to node 1`), the later ones
/// the memory policy and where the pages are, and both are `1`; a version-1
/// record must match one of them.  Any other version is refused: an audit
/// does not grade by rules it does not have.
pub fn grades_under_version(version: u32, g: &GradeInput) -> Result<Vec<Grade>, String> {
    if version == 1 {
        return grade_v1_candidates(g)
            .ok_or_else(|| "grading version 1 needs the frozen version-2 grader".to_string());
    }
    grade_for_version(version, g).map(|x| vec![x]).ok_or_else(|| {
        format!(
            "grading version {version} is not supported by this binary (it regrades versions 1 to {})",
            crate::cryptanalysis::ecbench::record::GRADING_VERSION
        )
    })
}

fn level_of(blockers: &[String]) -> Level {
    if blockers.iter().any(|b| b.starts_with("L1: ")) {
        Level::L0
    } else if blockers.iter().any(|b| b.starts_with("L2: ")) {
        Level::L1
    } else if blockers.iter().any(|b| b.starts_with("L3: ")) {
        Level::L2
    } else {
        Level::L3
    }
}

fn grade_v1_candidates(g: &GradeInput) -> Option<Vec<Grade>> {
    let v2 = grade_for_version(2, g)?;
    // The blockers only version 2 raises.
    let v2_only = |b: &str| {
        b.starts_with("L1: the run CPU's SMT siblings")
            || (b.starts_with("L2: ")
                && b.ends_with(" idle siblings and the rest of the reservation"))
    };
    let blockers: Vec<String> = v2.blockers.into_iter().filter(|b| !v2_only(b)).collect();
    let mut out = vec![Grade {
        level: level_of(&blockers),
        blockers: blockers.clone(),
        foreign_busy_ticks: v2.foreign_busy_ticks,
        steal_ticks: v2.steal_ticks,
        idle_sibling_busy_ticks: None,
    }];
    // The first version-1 NUMA rule: the cpuset must allow exactly the node
    // (instead of the policy-and-pages read-back, which it replaced).
    if let (true, Some(plan)) = (g.capsule.stable.numa_nodes > 1, g.plan) {
        if let Some(node) = plan.node {
            let want = node.to_string();
            if g.placement.mems_allowed.as_deref() != Some(&want[..]) {
                let mut a: Vec<String> = blockers
                    .into_iter()
                    .filter(|b| {
                        !(b.starts_with("L1: memory policy read back as")
                            || b.starts_with("L1: no /proc/self/numa_maps")
                            || (b.starts_with("L1: ") && b.contains(" anonymous pages on node ")))
                    })
                    .collect();
                a.insert(
                    0,
                    format!(
                        "L1: memory allowed on {:?}, not bound to node {node}",
                        g.placement.mems_allowed
                    ),
                );
                out.push(Grade {
                    level: level_of(&a),
                    blockers: a,
                    foreign_busy_ticks: v2.foreign_busy_ticks,
                    steal_ticks: v2.steal_ticks,
                    idle_sibling_busy_ticks: None,
                });
            }
        }
    }
    Some(out)
}

// ── The receipt ────────────────────────────────────────────────────

pub const AUDIT_SCHEMA: &str = "ecbench.audit/v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Replay {
    pub seq: u64,
    pub record_id: String,
    pub reproduced: bool,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditReport {
    pub schema: String,
    pub session_id: String,
    /// The session's own status: `complete`, or `interrupted` when that
    /// was accepted (including a session that was never finalised, see
    /// [`never_finalised`]).
    #[serde(default)]
    pub session_status: String,
    /// Why an accepted interruption is one: `never_finalised` when the
    /// session still reads `running` exactly as the runner first wrote it
    /// (the runner was killed before it could mark the session).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption: Option<String>,
    /// Whether `records.jsonl` was checked against the whole-file hash
    /// the runner wrote (`session.records_sha256`) when it finalised the
    /// session.  `false` for a never-finalised session
    /// (`interruption: never_finalised`): the runner never wrote that
    /// hash, so nothing binds the file as a whole.  Its records carry
    /// only their own per-record seals, which are unkeyed hashes, so a
    /// truncated file, or one with valid-looking records appended, is not
    /// detected by any whole-file check.  **An unbound session must not
    /// be cited as complete**: cite it, if at all, as an interrupted,
    /// unbound prefix of its plan.  `None` only in receipts written
    /// before this field existed.
    #[serde(default)]
    pub records_bound: Option<bool>,
    pub ok: bool,
    pub problems: Vec<String>,
    pub checks: BTreeMap<String, bool>,
    pub records: u64,
    pub verified_records: u64,
    /// The plan's execution count: a session with fewer records than this
    /// is a prefix of its plan, whatever else it says.  `0` in receipts
    /// written before this field existed.
    #[serde(default)]
    pub planned_executions: u64,
    /// Where an accepted interruption left the session: the number of
    /// intact records it ends with (the next sequence number that was
    /// never recorded).  Present only for an `interrupted` session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption_point: Option<u64>,
    /// Things a reader must not miss that are not failures: an interrupted
    /// session (never to be cited as complete), a torn final line that was
    /// set aside.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    /// Every run's level was recomputed from its observations under the
    /// grading version the session was recorded with.  `false` when some
    /// run was not (an unsupported version) or did not regrade to its
    /// recorded level; the `regraded` check says which.
    #[serde(default)]
    pub regraded: bool,
    pub replays: Vec<Replay>,
    /// How the replays were chosen: `all`, `sample` (random, from
    /// `replay_seed`), or `explicit` (the sequence numbers asked for,
    /// beside any sample).  Absent in receipts written before this field
    /// existed, whose sample was `candidates[j * len / n]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_selection: Option<String>,
    /// The auditor's randomness for a `sample`: replaying the same session
    /// with the same seed picks the same runs.  The default is drawn from
    /// the operating system, so a session's author cannot know which runs
    /// will be replayed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_seed: Option<u64>,
    /// How many runs were eligible for replay (measured, deterministic by
    /// the registry, with an outcome a replay reproduces).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_candidates: Option<u64>,
    /// SHA-256 of every file the audit read.
    pub files: BTreeMap<String, String>,
    pub auditor_binary_sha256: Option<String>,
    /// The measurement environment class the audit ran under.
    #[serde(default)]
    pub auditor_env_class_id: Option<String>,
    /// Hardware profile class, which omits mutable compiler and OS policy.
    /// A claim rejects an auditor with the session's same hardware class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auditor_hardware_class_id: Option<String>,
    pub audited_unix_ms: u128,
}

impl AuditReport {
    /// The process exit status `ecbench verify` reports: 0 for a clean
    /// audit, 1 for any problem.
    pub fn exit_status(&self) -> u8 {
        u8::from(!self.ok)
    }
}

fn file_sha(dir: &Path, name: &str, files: &mut BTreeMap<String, String>) -> Option<String> {
    let bytes = std::fs::read(dir.join(name)).ok()?;
    let h = sha256_hex(&bytes);
    files.insert(name.to_string(), h.clone());
    Some(h)
}

/// What an audit is asked to do.
#[derive(Clone, Debug, Default)]
pub struct AuditOptions {
    /// Runs to replay, drawn at random from the eligible ones;
    /// [`REPLAY_ALL`] for every one.
    pub replay: usize,
    /// The auditor's randomness for the draw; `None` draws one from the
    /// operating system.
    pub replay_seed: Option<u64>,
    /// Sequence numbers to replay beside the sample.
    pub replay_seqs: Vec<u64>,
    /// Accept an `interrupted` (or never-finalised) session.
    pub allow_interrupted: bool,
}

/// Audit `dir`; replay up to `replay` deterministic measured runs
/// ([`REPLAY_ALL`] for every one).
pub fn audit(dir: &Path, replay: usize) -> Result<AuditReport, String> {
    audit_with(dir, replay, false)
}

/// The interruption [`audit_with`] reports for a session that was never
/// finalised.
pub const NEVER_FINALISED: &str = "never_finalised";

/// A session whose `session.json` is still in a state the runner writes
/// before the session ends: status `running`, no finish time, no records
/// hash.  The runner writes the final `session.json` (`complete`, or
/// `interrupted` on a signal or an error) only when the session ends, so a
/// session left in this state was killed outright (SIGKILL, host loss) or
/// is still being written by a live runner; the files cannot tell those
/// apart, and a committed session has no live runner.  Since the runner
/// rewrites `session.json` atomically after every record, a killed session
/// may carry progress counters (`records_written`, `last_seq`,
/// `status_counts`); sessions killed before that was done carry zeros.
/// Records are sealed line by line as they are written, so every
/// per-record check applies; only the whole-file hash, which the runner
/// never got to write, does not.
pub fn never_finalised(session: &Session) -> bool {
    session.status == "running"
        && session.finished_unix_ms.is_none()
        && session.records_sha256.is_none()
}

/// [`audit`], optionally accepting an `interrupted` session: it is held
/// to every integrity check (hashes, seals, plan, answers, derived
/// figures), with "every planned execution recorded" replaced by "every
/// record it says it wrote".  A session that was never finalised
/// ([`never_finalised`]) is accepted as interrupted too, held to every
/// per-record check (seals, sequence, plan, answers, derived figures,
/// grading) with no records hash or count to compare against; it is
/// reported as `interrupted` with `interruption: never_finalised` and
/// `records_bound: false` (explicitly unbound: no whole-file hash ties
/// `records.jsonl` together, so truncation or appended records are seen
/// only through the unkeyed per-record seals).  Such a session must not
/// be cited as complete.  A session that claims `complete` gets no such
/// relaxation.
///
/// The replay sample is drawn from the operating system's randomness; use
/// [`audit_opts`] to supply a seed.
pub fn audit_with(
    dir: &Path,
    replay: usize,
    allow_interrupted: bool,
) -> Result<AuditReport, String> {
    audit_opts(
        dir,
        &AuditOptions {
            replay,
            allow_interrupted,
            ..Default::default()
        },
    )
}

/// A seed from the operating system's randomness.
fn os_seed() -> u64 {
    let mut b = [0u8; 8];
    if getrandom::getrandom(&mut b).is_ok() {
        return u64::from_le_bytes(b);
    }
    // No entropy source: the clock and the address space are the best left,
    // and still unknown to the session's author.
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    splitmix64((t as u64) ^ ((std::process::id() as u64) << 32) ^ (&b as *const _ as u64))
}

/// `n` of `len` indices, uniformly at random without replacement from the
/// seed, in increasing order.  The same `(len, n, seed)` always gives the
/// same indices; different seeds give independent draws.
pub fn choose_replays(len: usize, n: usize, seed: u64) -> Vec<usize> {
    let n = n.min(len);
    let mut idx: Vec<usize> = (0..len).collect();
    let key = splitmix64(seed);
    for j in 0..n {
        let r = splitmix64(key ^ splitmix64(j as u64 + 1));
        let k = j + (r % (len - j) as u64) as usize;
        idx.swap(j, k);
    }
    let mut chosen = idx[..n].to_vec();
    chosen.sort_unstable();
    chosen
}

/// The records of `records.jsonl` that parse, in order, with what stopped
/// the read if anything.
struct RecordFile {
    lines: Vec<(String, Record)>,
    /// A read problem: a line that does not parse (and everything after it,
    /// which is not read).
    problem: Option<String>,
    /// Bytes of an unterminated final line that does not parse: the write
    /// the runner was killed in.
    torn_tail: Option<usize>,
}

fn read_record_file(dir: &Path) -> Result<RecordFile, String> {
    let path = dir.join("records.jsonl");
    let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = RecordFile {
        lines: Vec::new(),
        problem: None,
        torn_tail: None,
    };
    let mut segments: Vec<&[u8]> = bytes.split(|b| *b == b'\n').collect();
    if segments.last().is_some_and(|s| s.is_empty()) {
        segments.pop();
    }
    let unterminated = bytes.last().is_some_and(|b| *b != b'\n');
    for (i, seg) in segments.iter().enumerate() {
        let parsed = std::str::from_utf8(seg)
            .map_err(|e| e.to_string())
            .and_then(|t| {
                if t.trim().is_empty() {
                    Err("blank line".to_string())
                } else {
                    from_str_legacy_floats::<Record>(t).map(|r| (t.to_string(), r))
                }
            });
        match parsed {
            Ok(line) => out.lines.push(line),
            Err(_) if unterminated && i + 1 == segments.len() => {
                out.torn_tail = Some(seg.len());
            }
            Err(e) => {
                out.problem = Some(format!("records.jsonl line {}: {e}", i + 1));
                break;
            }
        }
    }
    Ok(out)
}

/// Accumulates the receipt's checks and problems.
#[derive(Default)]
struct Ledger {
    checks: BTreeMap<String, bool>,
    problems: Vec<String>,
}

impl Ledger {
    fn check(&mut self, name: &str, ok: bool, why: impl FnOnce() -> String) {
        self.checks.insert(name.to_string(), ok);
        if !ok {
            self.problems.push(format!("{name}: {}", why()));
        }
    }
}

/// Phases that do not sum to the total on purpose, by method id: the
/// frozen accounting of `rho.cubic` and `rho.quartic` leaves their
/// set-up scalar multiplication out of the total (DEEP_DIVE §1.4; the
/// ids stay byte for byte and a corrected method gets a new id).  Every
/// other method's phases sum to its total.
pub const PHASE_SUM_EXEMPT_METHODS: &[&str] = &["rho.cubic", "rho.quartic"];

/// Relative tolerance of the phases-sum-to-total check.  Phases and total
/// are sums of the same terms in different orders, and the totals of the
/// sessions recorded on 2026-10-04..09 are rounded to single precision
/// (relative error up to 6e-8); a hidden cost is orders of magnitude larger.
const PHASE_SUM_TOLERANCE: f64 = 1e-6;

/// Sessions started at or after this instant (2026-10-10T00:00:00Z) were
/// recorded by a binary that counts field operations; every committed
/// session before 2026-10-07 has no block and every one from 2026-10-12 on
/// has it, so a record from the later era that lacks it was edited.
pub const FIELD_OPS_SINCE_UNIX_MS: u128 = 1_791_590_400_000;

/// The rule behind the field-operation block: a generic method on a
/// prime-field curve counts the field operations behind its group
/// operations (`CountedGroup::counts_field_ops`), so its record carries the
/// block, and deleting it is a change, not an unknown.  A record written
/// before the block existed has none, and unknown is not a disagreement.
fn counts_field_ops(r: &Record, started_unix_ms: u128) -> bool {
    started_unix_ms >= FIELD_OPS_SINCE_UNIX_MS
        && r.workload.curve.family == "prime"
        && (r.method.id.starts_with("rho.")
            || r.method.id.starts_with("bsgs.")
            || r.method.id.starts_with("kangaroo."))
        && r.cost.total_gae.is_some()
}

/// Everything about one record that follows from the record, the arm it
/// ran under and the host capsule, without re-running anything.
fn consistency_problems(
    r: &Record,
    arm: &Arm,
    host: &HostCapsule,
    timeout_seconds: u64,
    started_unix_ms: u128,
) -> Vec<String> {
    let mut out = Vec::new();
    let tag = format!("record seq {}", r.seq);
    // Role, environment and binary.
    let role = serde_json::to_value(arm.role)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();
    if r.role != role {
        out.push(format!(
            "{tag}: role `{}` is not the spec arm's `{role}`",
            r.role
        ));
    }
    if r.env_class_id != host.env_class_id {
        out.push(format!(
            "{tag}: environment class {} is not the host capsule's {}",
            r.env_class_id, host.env_class_id
        ));
    }
    if r.binary_sha256 != host.build.binary_sha256 {
        out.push(format!(
            "{tag}: binary {:?} is not the host capsule's {:?}",
            r.binary_sha256, host.build.binary_sha256
        ));
    }
    // Automorphisms.
    if let Some(used) = r.boundaries.automorphisms_used {
        if used > r.boundaries.automorphisms_available {
            out.push(format!(
                "{tag}: used {used} automorphisms, the curve has {}",
                r.boundaries.automorphisms_available
            ));
        }
    }
    // Phases sum to the total.
    if let Some(total) = r.cost.total_gae {
        if !PHASE_SUM_EXEMPT_METHODS.contains(&r.method.id.as_str()) {
            let sum: f64 = r.phases.iter().map(|p: &PhaseRecord| p.gae).sum();
            let tol = PHASE_SUM_TOLERANCE * sum.abs().max(total.abs());
            let within = (sum - total).abs() <= tol;
            if !within {
                out.push(format!(
                    "{tag}: phases sum to {sum} gae, the total is {total}"
                ));
            }
        }
    }
    // Field operations.
    if counts_field_ops(r, started_unix_ms) && r.field_ops.is_none() {
        out.push(format!(
            "{tag}: `{}` counts field operations on a prime curve and this record has none",
            r.method.id
        ));
    }
    // Determinism is the registry's, not the record's.
    let rule = method_determinism(&r.method);
    if r.cost.total_gae.is_some() {
        match rule {
            Determinism::Always if !r.cost.deterministic => out.push(format!(
                "{tag}: declares itself nondeterministic ({:?}) but `{}` is a deterministic method",
                r.cost.nondeterminism, r.method.id
            )),
            Determinism::Never if r.cost.deterministic => out.push(format!(
                "{tag}: declares itself deterministic but `{}` is not",
                r.method.id
            )),
            _ => {}
        }
    }
    // A run that failed has the shape of its failure.
    let failed = !matches!(
        r.outcome.status.as_str(),
        "verified" | "wrong_answer" | "exhausted"
    );
    if failed
        && (r.cost.total_gae.is_some()
            || r.cost.s.is_some()
            || r.boundaries.ratio_to_floor.is_some())
    {
        out.push(format!(
            "{tag}: status `{}` carries a cost; only a finished solve has one",
            r.outcome.status
        ));
    }
    if !failed && r.cost.total_gae.is_none() {
        out.push(format!(
            "{tag}: status `{}` has no total, so no solve finished",
            r.outcome.status
        ));
    }
    match r.outcome.status.as_str() {
        "timeout" => {
            if r.outcome.exit.as_deref() != Some("signal 9") {
                out.push(format!(
                    "{tag}: a timeout is a SIGKILL by the watchdog, not {:?}",
                    r.outcome.exit
                ));
            }
            if r.time.process_wall_ns < timeout_seconds.saturating_mul(1_000_000_000) {
                out.push(format!(
                    "{tag}: a timeout after {} ns, before the spec's {timeout_seconds} s",
                    r.time.process_wall_ns
                ));
            }
        }
        "crashed" => {
            if matches!(r.outcome.exit.as_deref(), None | Some("exit 0")) {
                out.push(format!(
                    "{tag}: crashed with exit {:?}; a crash is a signal or a nonzero exit",
                    r.outcome.exit
                ));
            }
        }
        _ => {}
    }
    out
}

fn same_blockers(version: u32, a: &[String], b: &[String]) -> bool {
    if version == 1 {
        // Version 1 raised its NUMA blocker at another place in the list.
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        a.sort();
        b.sort();
        a == b
    } else {
        a == b
    }
}

/// Re-execute a run's measured child, as the runner did, and return what
/// it printed.
fn child_output(
    p: &Plan,
    r: &Record,
    ex: &Execution,
) -> Result<crate::cryptanalysis::ecbench::record::ChildOutput, String> {
    let w = &p.workloads[ex.workload];
    let input = ChildInput {
        schema: CHILD_INPUT_SCHEMA.into(),
        curve: w.curve_spec.for_child(p.instance(ex.workload)),
        target_seed: w.target_seed,
        target_index: w.target_index,
        target_kind: w.kind(),
        expected_workload_id: w.workload_id.clone(),
        method: MethodSpec {
            id: r.method.id.clone(),
            params: r.method.params.clone(),
        },
        expected_method_id: r.method.method_id.clone(),
        algorithm_seed: ex.algorithm_seed,
    };
    Ok(child_main(
        &serde_json::to_string(&input).map_err(|e| e.to_string())?,
    ))
}

/// One replay: re-execute the run and compare it with the record.
fn replay_one(p: &Plan, r: &Record, ex: &Execution) -> Result<(bool, String), String> {
    let out = child_output(p, r, ex)?;
    if r.outcome.status == "error" {
        // The recorded error must be the one the replay hits (the runner
        // may append the session's own interruption to it).
        return Ok(match (out.report, out.error) {
            (None, Some(e)) => {
                let recorded = r.outcome.error.as_deref().unwrap_or("");
                if recorded == e || recorded.starts_with(&format!("{e}; session ")) {
                    (true, "identical (same error)".into())
                } else {
                    (false, format!("differs in error: replay says `{e}`"))
                }
            }
            (Some(_), _) => (false, "differs in outcome: the replay solved it".into()),
            (None, None) => (false, "replay failed without an error".into()),
        });
    }
    Ok(match out.report {
        None => (
            false,
            format!("replay failed: {}", out.error.unwrap_or_default()),
        ),
        Some(rep) => {
            let mut diffs = Vec::new();
            if rep.recovered.map(|k| k.to_string()) != r.outcome.recovered {
                diffs.push("recovered");
            }
            if (r.outcome.status == "exhausted") != (rep.recovered.is_none() && rep.exhausted) {
                diffs.push("exhaustion");
            }
            // Integer counts compare exactly; a float compares after
            // the same serialise-parse round trip the record's took.
            let same_gae = |phases: &[f64], total: f64| {
                (
                    Some(json_roundtrip(total).to_bits()) == r.cost.total_gae.map(f64::to_bits),
                    phases
                        .iter()
                        .map(|g| json_roundtrip(*g).to_bits())
                        .eq(r.phases.iter().map(|p| p.gae.to_bits())),
                )
            };
            let exact: Vec<f64> = rep.phases.iter().map(|p| p.gae).collect();
            let (mut total_ok, mut phases_ok) = same_gae(&exact, rep.total_gae);
            let mut legacy = false;
            // The pre-fix solver rounding is for the sealed records it was
            // written into, never for a record that merely claims it.
            if !(total_ok && phases_ok) && legacy_rounding_allowed(&r.session_id, &r.record_id) {
                if let Some((ph, tot)) = legacy_solver_rounding(&exact, &r.detail) {
                    if same_gae(&ph, tot) == (true, true)
                        && legacy_rounding_bounded(&exact, rep.total_gae, &ph, tot, &r.detail)
                    {
                        (total_ok, phases_ok, legacy) = (true, true, true);
                    }
                }
            }
            if !total_ok {
                diffs.push("total_gae");
            }
            let ints = |ps: &[PhaseRecord]| {
                ps.iter()
                    .map(|p| {
                        (
                            p.name.clone(),
                            p.adds,
                            p.doubles,
                            p.scalar_mults,
                            p.native.clone(),
                        )
                    })
                    .collect::<Vec<_>>()
            };
            if ints(&rep.phases) != ints(&r.phases) {
                diffs.push("phase counts");
            }
            if !phases_ok {
                diffs.push("phase gae");
            }
            if rep.counters != r.counters {
                diffs.push("counters");
            }
            if rep.factor_base != r.factor_base {
                diffs.push("factor_base");
            }
            // Field operations compare only when both sides carry
            // them: a committed record written before they were
            // counted has none, and unknown is not a disagreement.
            if let (Some(a), Some(b)) = (rep.field_ops, r.field_ops) {
                if a != b {
                    diffs.push("field_ops");
                }
            }
            if rep.unpriced != r.cost.unpriced
                || rep.deterministic != r.cost.deterministic
                || Some(rep.automorphisms_used) != r.boundaries.automorphisms_used
            {
                diffs.push("unpriced, determinism or automorphisms");
            }
            (
                diffs.is_empty(),
                if diffs.is_empty() && legacy {
                    LEGACY_IDENTICAL.into()
                } else if diffs.is_empty() {
                    "identical".into()
                } else {
                    format!("differs in {}", diffs.join(", "))
                },
            )
        }
    })
}

/// [`audit_with`] with every option: see [`AuditOptions`].
pub fn audit_opts(dir: &Path, opts: &AuditOptions) -> Result<AuditReport, String> {
    let allow_interrupted = opts.allow_interrupted;
    let mut led = Ledger::default();
    let mut files = BTreeMap::new();
    let mut warnings: Vec<String> = Vec::new();
    let session = read_session(dir)?;
    file_sha(dir, "session.json", &mut files);
    file_sha(dir, "spec.json", &mut files);

    // Files against the session's hashes.
    let host_sha = file_sha(dir, "host.json", &mut files);
    led.check(
        "host_hash",
        host_sha.as_deref() == Some(&session.host_sha256[..]),
        || "host.json does not hash to session.host_sha256".into(),
    );
    let plan_sha = file_sha(dir, "plan.json", &mut files);
    led.check(
        "plan_hash",
        plan_sha.as_deref() == Some(&session.plan_sha256[..]),
        || "plan.json does not hash to session.plan_sha256".into(),
    );
    let rec_sha = file_sha(dir, "records.jsonl", &mut files);
    let complete = session.status == "complete";
    let killed = allow_interrupted && never_finalised(&session);
    let interrupted = session.status == "interrupted" || killed;
    let accepted_interruption = allow_interrupted && interrupted;
    led.check(
        "records_hash",
        !(complete || accepted_interruption) || killed || rec_sha == session.records_sha256,
        || "records.jsonl does not hash to session.records_sha256".into(),
    );
    led.check(
        "session_complete",
        complete || accepted_interruption,
        || format!("session status is `{}`", session.status),
    );

    // The session id recomputes from its preimage (all of which the checks
    // above and below tie to files).
    let preimage_id = short_id(
        "ECBS1",
        &json!({
            "spec_sha256": session.spec_sha256,
            "host_sha256": session.host_sha256,
            "plan_sha256": session.plan_sha256,
            "started_unix_ms": session.started_unix_ms.to_string(),
        }),
    )
    .map(|(id, _)| id);
    led.check(
        "session_id",
        preimage_id.as_deref() == Ok(&session.session_id[..]),
        || {
            format!(
                "session.json says {}, its preimage hashes to {}",
                session.session_id,
                preimage_id.clone().unwrap_or_else(|e| e)
            )
        },
    );

    // The host class.
    let host: HostCapsule = serde_json::from_str(
        &std::fs::read_to_string(dir.join("host.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("host.json: {e}"))?;
    let class = recompute_class(&host)?;
    led.check(
        "env_class",
        class == host.env_class_id && class == session.env_class_id,
        || {
            format!(
                "recomputed {class}, recorded {} / {}",
                host.env_class_id, session.env_class_id
            )
        },
    );
    led.check(
        "session_build",
        session.binary_sha256 == host.build.binary_sha256
            && session.git_commit == host.build.git_commit
            && session.git_dirty == host.build.git_dirty,
        || "the session's binary hash or git facts are not the host capsule's".into(),
    );

    // The spec re-expands to the plan.
    let spec_text = std::fs::read_to_string(dir.join("spec.json")).map_err(|e| e.to_string())?;
    let p = plan(Spec::from_json(&spec_text)?)?;
    let doc: PlanDoc = serde_json::from_str(
        &std::fs::read_to_string(dir.join("plan.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("plan.json: {e}"))?;
    led.check(
        "spec_id",
        p.spec_id == session.spec_id
            && p.spec_id == doc.spec_id
            && p.spec_sha256 == session.spec_sha256,
        || format!("spec re-expands to {}", p.spec_id),
    );
    led.check(
        "workloads",
        p.workloads.len() == doc.workloads.len()
            && p.workloads
                .iter()
                .zip(&doc.workloads)
                .all(|(a, b)| workload_identity(a) == workload_identity(b))
            && doc
                .workloads
                .iter()
                .all(|w| w.recompute_id().ok().as_ref() == Some(&w.workload_id)),
        || "the spec rebuilds different workloads, or a workload id does not recompute".into(),
    );
    led.check("executions", p.executions == doc.executions, || {
        "the spec expands to a different execution order or seeds".into()
    });
    led.check(
        "arms",
        p.arms.len() == doc.arms.len()
            && p.arms
                .iter()
                .zip(&doc.arms)
                .all(|(a, b)| arm_identity(a) == arm_identity(b)),
        || "the spec resolves to different methods".into(),
    );
    led.check(
        "executions_planned",
        session.executions_planned == p.executions.len() as u64,
        || {
            format!(
                "session.json plans {}, the spec expands to {}",
                session.executions_planned,
                p.executions.len()
            )
        },
    );

    // Records.
    let file = read_record_file(dir)?;
    let lines = file.lines;
    let records: Vec<Record> = lines.iter().map(|(_, r)| r.clone()).collect();
    let torn_tolerated = allow_interrupted && interrupted;
    if let Some(bytes) = file.torn_tail {
        if torn_tolerated {
            warnings.push(format!(
                "records.jsonl ends in a torn line of {bytes} bytes (the runner was killed while writing it); \
                 it is ignored and the {} intact records before it are audited",
                records.len()
            ));
        } else {
            led.check("records_tail", false, || {
                format!("records.jsonl ends in a torn line of {bytes} bytes")
            });
        }
    }
    if let Some(problem) = &file.problem {
        led.check("records_parse", false, || problem.clone());
    }
    led.check(
        "record_count",
        if complete {
            records.len() as u64 == session.records_written && records.len() == p.executions.len()
        } else if killed {
            // The progress counter was written after the record it counts,
            // so no fewer records than it says may be on disk.
            records.len() <= p.executions.len() && records.len() as u64 >= session.records_written
        } else {
            records.len() as u64 == session.records_written && records.len() <= p.executions.len()
        },
        || {
            format!(
                "{} records, {} written, {} planned",
                records.len(),
                session.records_written,
                p.executions.len()
            )
        },
    );
    let timeout_seconds = p.spec.measurement.timeout_seconds.max(1);
    let mut record_problems = Vec::new();
    let mut verified = 0u64;
    // Records whose line, sequence and plan slot are what they should be:
    // the only ones a replay may address.
    let mut in_plan: BTreeSet<usize> = BTreeSet::new();
    let mut status_counts: BTreeMap<String, u64> = BTreeMap::new();
    for (i, r) in records.iter().enumerate() {
        let tag = format!("record seq {}", r.seq);
        *status_counts.entry(r.outcome.status.clone()).or_insert(0) += 1;
        if r.seq != i as u64 {
            record_problems.push(format!("{tag}: out of sequence at line {}", i + 1));
        }
        if !check_line_seal(&lines[i].0, &r.record_id) {
            record_problems.push(format!("{tag}: content does not hash to {}", r.record_id));
        }
        if r.session_id != session.session_id {
            record_problems.push(format!(
                "{tag}: session {} is not {}",
                r.session_id, session.session_id
            ));
        }
        let Some(ex) = p.executions.get(i) else {
            record_problems.push(format!("{tag}: no planned execution"));
            continue;
        };
        if r.seq == i as u64 {
            in_plan.insert(i);
        }
        let arm = &p.arms[ex.arm];
        let w = &p.workloads[ex.workload];
        if r.round != ex.round
            || r.warmup != ex.warmup
            || r.algorithm_seed != ex.algorithm_seed.to_string()
        {
            record_problems.push(format!(
                "{tag}: round, warm-up or seed differs from the plan"
            ));
        }
        if r.arm != arm.name
            || r.method.method_id != arm.method.method_id
            || r.method.method_sha256 != arm.method.method_sha256
        {
            record_problems.push(format!("{tag}: arm or method differs from the plan"));
        }
        if workload_identity(&r.workload) != workload_identity(w) {
            record_problems.push(format!("{tag}: workload differs from the plan"));
        }
        // Every derived figure must follow from the record's own counts:
        // a hand-edited S, ratio or flag fails here even if resealed.
        let floor = floor_s(r.boundaries.automorphisms_available);
        let sqrt_r = (w.curve.r as f64).sqrt();
        let derived_ok = r.boundaries.automorphisms_available == w.curve.automorphisms_available
            && close(r.boundaries.floor_s, floor)
            && match (r.cost.total_gae, r.cost.s, r.boundaries.ratio_to_floor) {
                (Some(t), Some(sv), Some(q)) => close(sv, t / sqrt_r) && close(q, sv / floor),
                (None, None, None) => true,
                _ => false,
            }
            && r.cost.lower_bound != r.cost.unpriced.is_empty()
            && (r.cost.total_gae.is_none()
                || r.cost.deterministic == r.cost.nondeterminism.is_empty())
            && r.run_id
                .starts_with(&format!("{}{}R", r.method.method_id, w.workload_id));
        if !derived_ok {
            record_problems.push(format!(
                "{tag}: S, floor, ratio, lower-bound flag or run id does not follow from the record"
            ));
        }
        record_problems.extend(consistency_problems(
            r,
            arm,
            &host,
            timeout_seconds,
            session.started_unix_ms,
        ));
        // Re-check the answer here.
        let inst = p.instance(ex.workload);
        let rec: Option<u128> = r.outcome.recovered.as_deref().and_then(|s| s.parse().ok());
        let target_ok = rec.map(|k| inst.mul_generator_hex(k).as_ref() == Some(&w.target));
        let planted_ok = w.planted.and_then(|p| rec.map(|k| k == p));
        if target_ok != r.outcome.matches_target || planted_ok != r.outcome.matches_planted {
            record_problems.push(format!("{tag}: verification does not reproduce"));
        }
        let should_verify = target_ok == Some(true) && planted_ok != Some(false);
        if (r.outcome.status == "verified") != should_verify {
            record_problems.push(format!(
                "{tag}: status `{}` contradicts the check",
                r.outcome.status
            ));
        }
        if r.outcome.status == "verified" {
            verified += 1;
        }
        if let Some(h) = &r.outcome.stderr_sha256 {
            let name = format!("exec/{:06}.stderr", r.seq);
            if file_sha(dir, &name, &mut files).as_deref() != Some(h) {
                record_problems.push(format!("{tag}: {name} missing or altered"));
            }
        }
    }
    // Regrade every run from its own observations, under the rules the
    // session was graded with.  A version this binary does not have is a
    // failure, not a skip: the session.json that names it is not hashed.
    let mut regraded = true;
    let mut regrade_problems = Vec::new();
    for r in &records {
        let input = GradeInput {
            plan: session.cpu_plan.as_ref(),
            preflight: session.preflight.as_ref(),
            eviction: session.eviction.as_ref(),
            runner_on_run_cpu: session.runner_on_run_cpu,
            capsule: &host,
            thresholds: &session.options.thresholds,
            placement: &r.isolation.placement,
            before: &r.isolation.before,
            after: &r.isolation.after,
            timing: &r.time,
            hz: session.ticks_per_second as f64,
        };
        match grades_under_version(session.grading_version, &input) {
            Ok(grades) => {
                let matches = |g: &Grade| {
                    g.level == r.isolation.level
                        && same_blockers(
                            session.grading_version,
                            &g.blockers,
                            &r.isolation.blockers,
                        )
                };
                if !grades.iter().any(matches) {
                    let g = &grades[0];
                    regraded = false;
                    regrade_problems.push(format!(
                        "record seq {}: graded {} with {} blockers, its observations give {} with {}",
                        r.seq,
                        r.isolation.level.name(),
                        r.isolation.blockers.len(),
                        g.level.name(),
                        g.blockers.len()
                    ));
                }
            }
            Err(e) => {
                regraded = false;
                regrade_problems.push(e);
                break;
            }
        }
    }
    let regrade_count = regrade_problems.len();
    led.check("regraded", regraded, || {
        format!(
            "{} isolation levels do not regrade: {}",
            regrade_count,
            regrade_problems.first().cloned().unwrap_or_default()
        )
    });
    record_problems.extend(regrade_problems);
    let mut seen = BTreeSet::new();
    for r in &records {
        if !seen.insert(r.run_id.as_str()) {
            record_problems.push(format!("record seq {}: run id {} repeats", r.seq, r.run_id));
        }
    }
    led.check("records", record_problems.is_empty(), || {
        format!("{} record problems", record_problems.len())
    });
    led.problems.extend(record_problems);

    // The session's own counts follow from its records.
    // A killed runner's counts cover the records it had finished writing
    // when it last rewrote session.json (one more may be on disk).
    let counted: BTreeMap<String, u64> = if killed {
        let mut c = BTreeMap::new();
        for r in records.iter().take(session.records_written as usize) {
            *c.entry(r.outcome.status.clone()).or_insert(0) += 1;
        }
        c
    } else {
        status_counts
    };
    led.check("status_counts", session.status_counts == counted, || {
        format!(
            "session.json counts {:?}, the records have {:?}",
            session.status_counts, counted
        )
    });

    // An interrupted session is an incomplete one, and says so.
    let mut interruption_point = None;
    if interrupted {
        interruption_point = Some(records.len() as u64);
        warnings.insert(
            0,
            format!(
                "INTERRUPTED: session ends after {} of {} planned executions{}; \
                 it is a prefix of its plan and must not be cited as complete or used to stop \
                 early on a favourable figure",
                records.len(),
                p.executions.len(),
                if killed {
                    " (the runner was killed; nothing binds the file as a whole)"
                } else {
                    ""
                }
            ),
        );
    }
    // Where the runner says it got to.  `last_seq` is rewritten after every
    // record, so a finalised session's records end exactly there and a
    // killed one's at most one further (the record written before the
    // runner could rewrite session.json).  `stop.seq` is the execution the
    // session stopped at: the last one recorded, or the next one when the
    // stop came before it ran.  Sessions written before these fields carry
    // neither.
    if let Some(last) = session.last_seq {
        let n = records.len() as u64;
        let ok = if killed {
            n == last + 1 || n == last + 2
        } else {
            n == last + 1
        };
        led.check("last_seq", ok, || {
            format!("session.json says the last record is seq {last}, the records end at {n}")
        });
    }
    if let Some(stop) = &session.stop {
        let n = records.len() as u64;
        led.check(
            "stop",
            !complete && stop.seq.is_none_or(|s| n == s || n == s + 1),
            || {
                format!(
                    "session.json says it stopped ({}) at seq {:?}, the records end at {n}",
                    stop.kind, stop.seq
                )
            },
        );
        if !complete {
            warnings.push(format!(
                "stopped: {} at seq {:?}: {}",
                stop.kind, stop.seq, stop.message
            ));
        }
    }

    // Replays.
    let candidates: Vec<usize> = records
        .iter()
        .enumerate()
        .filter(|(i, r)| in_plan.contains(i) && replayable(r))
        .map(|(i, _)| i)
        .collect();
    let mut replays = Vec::new();
    let mut replay_selection = None;
    let mut replay_seed = None;
    let mut replay_candidates = None;
    let mut picked: BTreeSet<usize> = BTreeSet::new();
    if opts.replay > 0 {
        replay_candidates = Some(candidates.len() as u64);
        if opts.replay == REPLAY_ALL {
            picked.extend(candidates.iter().copied());
            replay_selection = Some("all".to_string());
        } else if !candidates.is_empty() {
            let seed = opts.replay_seed.unwrap_or_else(os_seed);
            replay_seed = Some(seed);
            replay_selection = Some("sample".to_string());
            picked.extend(
                choose_replays(candidates.len(), opts.replay, seed)
                    .into_iter()
                    .map(|j| candidates[j]),
            );
        }
    }
    for &seq in &opts.replay_seqs {
        replay_selection.get_or_insert_with(|| "explicit".to_string());
        match candidates.iter().find(|&&i| records[i].seq == seq) {
            Some(&i) => {
                picked.insert(i);
            }
            None => led.problems.push(format!(
                "replay of seq {seq}: no such record, or it is not replayable (a warm-up, a timeout, a crash, \
                 a nondeterministic method, or out of its plan slot)"
            )),
        }
    }
    for &i in &picked {
        let r = &records[i];
        let ex = &p.executions[i];
        let (ok, detail) = replay_one(&p, r, ex)?;
        if !ok {
            led.problems
                .push(format!("replay of seq {}: {detail}", r.seq));
        }
        replays.push(Replay {
            seq: r.seq,
            record_id: r.record_id.clone(),
            reproduced: ok,
            detail,
        });
    }
    if !replays.is_empty() {
        led.checks
            .insert("replays".into(), replays.iter().all(|r| r.reproduced));
    }

    let auditor_binary_sha256 = std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::read(p).ok())
        .map(|b| sha256_hex(&b));
    let auditor_host = host::capture().ok();
    Ok(AuditReport {
        schema: AUDIT_SCHEMA.into(),
        session_status: if killed {
            "interrupted".into()
        } else {
            session.status.clone()
        },
        interruption: killed.then(|| NEVER_FINALISED.to_string()),
        // The records hash binds the file only when the runner wrote one
        // and it was compared (`records_hash` above).
        records_bound: Some(
            !killed && session.records_sha256.is_some() && rec_sha == session.records_sha256,
        ),
        session_id: session.session_id,
        ok: led.problems.is_empty(),
        problems: led.problems,
        checks: led.checks,
        records: records.len() as u64,
        verified_records: verified,
        planned_executions: p.executions.len() as u64,
        interruption_point,
        warnings,
        regraded,
        replays,
        replay_selection,
        replay_seed,
        replay_candidates,
        files,
        auditor_binary_sha256,
        auditor_env_class_id: auditor_host.as_ref().map(|c| c.env_class_id.clone()),
        auditor_hardware_class_id: auditor_host
            .as_ref()
            .and_then(|c| host::hardware_class_id(c).ok()),
        audited_unix_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0),
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    fn detail(w: f64, before: f64, priced_by: &str) -> serde_json::Value {
        json!({
            "decomposition": {"solver": {"gae": w, "priced_by": priced_by}},
            "framework_total_gae_before_solver_removal": before,
        })
    }

    /// The figures of the committed koblitz session's seq 4
    /// (`sub6-da-buch`): the pre-fix rounding is a function of the
    /// record's own solver term, so it reproduces exactly.
    #[test]
    fn legacy_rounding_is_reproduced_from_the_record_alone() {
        let x = 611.5963;
        let exact = [3.223395, 0.0, x, 2.721705, 15.0];
        let w = 91449423.0;
        let before = exact[0] + exact[1] + (x + w) + exact[3] + exact[4];
        let (ph, tot) = legacy_solver_rounding(&exact, &detail(w, before, "measured")).unwrap();
        assert_eq!(ph[2], (x + w) - w);
        assert_eq!(tot, before - w);
        assert_eq!(ph[..2], exact[..2]);
        assert_eq!(ph[3..], exact[3..]);
        // Another host's clock lands `w` in another binade: the old
        // subtraction then disagrees with itself, the exact figure does not.
        let w2 = 29295605.0;
        assert_ne!((x + w).to_bits(), x.to_bits());
        assert_ne!(((x + w) - w).to_bits(), ((x + w2) - w2).to_bits());
    }

    /// A committed session whose runner was killed
    /// (`research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02`):
    /// `session.json` still reads `running`, as first written, and
    /// `records.jsonl` holds the four sealed records written before the
    /// kill.  Embedded, so every lane that builds the tests has it.
    const KILLED: [(&str, &str); 5] = [
        (
            "session.json",
            include_str!("../../../research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/session.json"),
        ),
        (
            "spec.json",
            include_str!("../../../research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/spec.json"),
        ),
        (
            "plan.json",
            include_str!("../../../research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/plan.json"),
        ),
        (
            "host.json",
            include_str!("../../../research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/host.json"),
        ),
        (
            "records.jsonl",
            include_str!("../../../research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/records.jsonl"),
        ),
    ];

    /// A scratch copy of the killed session, byte for byte, to audit or
    /// mutate.
    fn killed_copy(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ecbench-audit-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for (name, text) in KILLED {
            std::fs::write(dir.join(name), text).unwrap();
        }
        dir
    }

    #[test]
    fn a_killed_session_is_interrupted_only_when_interruptions_are_allowed() {
        let dir = killed_copy("as-committed");
        let session = read_session(&dir).unwrap();
        assert!(never_finalised(&session));

        let strict = audit_with(&dir, 0, false).unwrap();
        assert!(!strict.ok);
        assert_eq!(strict.checks.get("session_complete"), Some(&false));
        assert_eq!(strict.session_status, "running");
        assert_eq!(strict.interruption, None);
        assert_eq!(strict.records_bound, Some(false));

        let lenient = audit_with(&dir, 0, true).unwrap();
        assert!(lenient.ok, "{:?}", lenient.problems);
        assert_eq!(lenient.session_status, "interrupted");
        assert_eq!(lenient.interruption.as_deref(), Some(NEVER_FINALISED));
        // Accepted, but explicitly unbound: no records hash was compared.
        assert_eq!(lenient.records_bound, Some(false));
        let text = serde_json::to_string(&lenient).unwrap();
        assert!(
            text.contains("\"interruption\":\"never_finalised\""),
            "{text}"
        );
        assert!(text.contains("\"records_bound\":false"), "{text}");
        assert_eq!(lenient.records, 4);
        assert_eq!(lenient.checks.get("records"), Some(&true));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The runner now rewrites `session.json` after every record, so a
    /// runner killed outright leaves `running` with progress counters.
    #[test]
    fn a_killed_session_with_progress_counters_is_still_never_finalised() {
        let dir = killed_copy("progress");
        let text = std::fs::read_to_string(dir.join("session.json")).unwrap();
        let mut v: serde_json::Value = serde_json::from_str(&text).unwrap();
        v["records_written"] = json!(4);
        v["last_seq"] = json!(3);
        v["status_counts"] = json!({"verified": 2, "timeout": 2});
        std::fs::write(dir.join("session.json"), v.to_string()).unwrap();
        assert!(never_finalised(&read_session(&dir).unwrap()));
        let strict = audit_with(&dir, 0, false).unwrap();
        assert!(!strict.ok);
        let lenient = audit_with(&dir, 0, true).unwrap();
        assert!(lenient.ok, "{:?}", lenient.problems);
        assert_eq!(lenient.interruption.as_deref(), Some(NEVER_FINALISED));
        assert_eq!(lenient.records_bound, Some(false));
        // A record count above what is on disk is truncation, caught even
        // though no whole-file hash exists.
        v["records_written"] = json!(5);
        std::fs::write(dir.join("session.json"), v.to_string()).unwrap();
        let rep = audit_with(&dir, 0, true).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("record_count"), Some(&false));
        // A finish time still takes it out of the relaxation.
        v["records_written"] = json!(4);
        v["finished_unix_ms"] = json!(1791170600000u64);
        std::fs::write(dir.join("session.json"), v.to_string()).unwrap();
        assert!(!never_finalised(&read_session(&dir).unwrap()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_finalised_interrupted_session_is_records_bound() {
        // The same records, finalised the way the runner marks an
        // interruption: now the whole-file hash binds them.
        let dir = killed_copy("finalised");
        let records = std::fs::read(dir.join("records.jsonl")).unwrap();
        let text = std::fs::read_to_string(dir.join("session.json")).unwrap();
        let mut v: serde_json::Value = serde_json::from_str(&text).unwrap();
        v["status"] = json!("interrupted");
        v["finished_unix_ms"] = json!(1791170600000u64);
        v["records_written"] = json!(4);
        v["records_sha256"] = json!(sha256_hex(&records));
        std::fs::write(dir.join("session.json"), v.to_string()).unwrap();
        let rep = audit_with(&dir, 0, true).unwrap();
        assert_eq!(rep.interruption, None);
        assert_eq!(rep.checks.get("records_hash"), Some(&true));
        assert_eq!(rep.records_bound, Some(true));
        // Appending a byte breaks the binding (and the audit).
        let mut longer = records.clone();
        longer.extend_from_slice(b"\n");
        std::fs::write(dir.join("records.jsonl"), longer).unwrap();
        let rep = audit_with(&dir, 0, true).unwrap();
        assert_eq!(rep.records_bound, Some(false));
        assert_eq!(rep.checks.get("records_hash"), Some(&false));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_killed_session_keeps_every_record_check() {
        // A session the runner did mark (a finish time) is not a killed
        // one, so it gets no relaxation for still reading `running`.
        let dir = killed_copy("finished");
        let text = std::fs::read_to_string(dir.join("session.json")).unwrap();
        let mut v: serde_json::Value = serde_json::from_str(&text).unwrap();
        v["finished_unix_ms"] = json!(1791170600000u64);
        std::fs::write(dir.join("session.json"), v.to_string()).unwrap();
        assert!(!never_finalised(&read_session(&dir).unwrap()));
        let rep = audit_with(&dir, 0, true).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("session_complete"), Some(&false));
        let _ = std::fs::remove_dir_all(&dir);

        // Dropping a sealed record is still caught: the rest are out of
        // sequence against the plan.
        let dir = killed_copy("dropped");
        let text = std::fs::read_to_string(dir.join("records.jsonl")).unwrap();
        let rest: String = text.lines().skip(1).map(|l| format!("{l}\n")).collect();
        std::fs::write(dir.join("records.jsonl"), rest).unwrap();
        let rep = audit_with(&dir, 0, true).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("records"), Some(&false));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn legacy_rounding_needs_the_records_own_total_and_a_wall_price() {
        let exact = [3.223395, 0.0, 611.5963, 2.721705, 15.0];
        let w = 91449423.0;
        let before = exact[0] + exact[1] + (exact[2] + w) + exact[3] + exact[4];
        // A solver term that does not reproduce the recorded pre-removal
        // total is not the one the record was written with.
        assert!(legacy_solver_rounding(&exact, &detail(w * 2.0, before, "measured")).is_none());
        assert!(legacy_solver_rounding(&exact, &detail(w, before + 1.0, "measured")).is_none());
        // Pinned solver work was never subtracted.
        assert!(legacy_solver_rounding(&exact, &detail(w, before, "pinned")).is_none());
        assert!(legacy_solver_rounding(&exact, &json!({})).is_none());
    }

    // ── Tamper tests on a small fresh session ─────────────────────────
    //
    // `research/ecbench_factor_bases_20261013/sessions/prime20`: eighteen
    // sealed records of `rho.negation` and two `ic.pipeline` arms on a
    // 20-bit prime curve, embedded so every lane that builds the tests has
    // it.  Each test mutates a scratch copy and re-seals what an attacker
    // would re-seal (the edited record, the records hash), so only the
    // checks under test can notice.

    pub(crate) const FRESH: [(&str, &str); 5] = [
        (
            "session.json",
            include_str!(
                "../../../research/ecbench_factor_bases_20261013/sessions/prime20/session.json"
            ),
        ),
        (
            "spec.json",
            include_str!(
                "../../../research/ecbench_factor_bases_20261013/sessions/prime20/spec.json"
            ),
        ),
        (
            "plan.json",
            include_str!(
                "../../../research/ecbench_factor_bases_20261013/sessions/prime20/plan.json"
            ),
        ),
        (
            "host.json",
            include_str!(
                "../../../research/ecbench_factor_bases_20261013/sessions/prime20/host.json"
            ),
        ),
        (
            "records.jsonl",
            include_str!(
                "../../../research/ecbench_factor_bases_20261013/sessions/prime20/records.jsonl"
            ),
        ),
    ];

    /// A rho record, an `ic.pipeline` record, in the committed session.
    pub(crate) const RHO_SEQ: usize = 8;
    pub(crate) const IC_SEQ: usize = 7;

    pub(crate) fn fresh(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ecbench-fresh-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for (name, text) in FRESH {
            std::fs::write(dir.join(name), text).unwrap();
        }
        dir
    }

    fn seal(v: &mut serde_json::Value) {
        v["record_id"] = json!("");
        let blank = serde_json::to_string(v).unwrap();
        v["record_id"] = json!(format!("ECR1h{}", &sha256_hex(blank.as_bytes())[..12]));
    }

    /// Edit the records and the session document, then write both back with
    /// every record re-sealed and the records hash recomputed: the edits an
    /// attacker with the files can make consistently.
    pub(crate) fn tamper(
        dir: &Path,
        edit: impl FnOnce(&mut Vec<serde_json::Value>, &mut serde_json::Value),
    ) {
        let text = std::fs::read_to_string(dir.join("records.jsonl")).unwrap();
        let mut records: Vec<serde_json::Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let mut session: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(dir.join("session.json")).unwrap())
                .unwrap();
        edit(&mut records, &mut session);
        let mut body = String::new();
        for r in records.iter_mut() {
            seal(r);
            body.push_str(&serde_json::to_string(r).unwrap());
            body.push('\n');
        }
        std::fs::write(dir.join("records.jsonl"), &body).unwrap();
        session["records_sha256"] = json!(sha256_hex(body.as_bytes()));
        std::fs::write(dir.join("session.json"), session.to_string()).unwrap();
    }

    fn rescale(r: &mut serde_json::Value, factor: f64) {
        // Total, S and the ratio together: only a replay, or a check on
        // the phases, can see it.
        let t = r["cost"]["total_gae"].as_f64().unwrap() * factor;
        let sqrt_r = (r["workload"]["curve"]["r"].as_f64().unwrap()).sqrt();
        let floor = r["boundaries"]["floor_s"].as_f64().unwrap();
        r["cost"]["total_gae"] = json!(t);
        r["cost"]["s"] = json!(t / sqrt_r);
        r["boundaries"]["ratio_to_floor"] = json!(t / sqrt_r / floor);
    }

    fn opts(replay: usize) -> AuditOptions {
        AuditOptions {
            replay,
            ..Default::default()
        }
    }

    fn problems_with(rep: &AuditReport, needle: &str) -> bool {
        rep.problems.iter().any(|p| p.contains(needle))
    }

    #[test]
    fn the_untampered_copy_passes_and_regrades() {
        let dir = fresh("clean");
        // Re-sealed with no edit: the harness itself must not break it.
        tamper(&dir, |_, _| {});
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(rep.ok, "{:?}", rep.problems);
        assert!(rep.regraded);
        assert_eq!(rep.checks.get("regraded"), Some(&true));
        assert_eq!(rep.checks.get("session_id"), Some(&true));
        assert_eq!(rep.planned_executions, 18);
        assert_eq!(rep.exit_status(), 0);
        assert!(rep.warnings.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// t27: a forged `ic.pipeline` record that names its own solver term to
    /// hide a collapse of its total used to replay as "identical (legacy gae
    /// rounding …)".  The exception is for sealed historical records only.
    #[test]
    fn a_forged_record_cannot_claim_the_legacy_rounding_exception() {
        let dir = fresh("t27");
        let text = std::fs::read_to_string(dir.join("spec.json")).unwrap();
        let p = plan(Spec::from_json(&text).unwrap()).unwrap();
        let records = read_record_file(&dir).unwrap().lines;
        let honest = &records[IC_SEQ].1;
        let ex = &p.executions[IC_SEQ];
        let out = child_output(&p, honest, ex).unwrap();
        let exact_total = out.report.as_ref().unwrap().total_gae;
        let exact: Vec<f64> = out.report.unwrap().phases.iter().map(|x| x.gae).collect();
        assert_eq!(exact.len(), 5);
        // A solver term far above the work, so `(x + w) - w` rounds the
        // relations phase to a multiple of w's ulp; pick one that moves it.
        let w = [1e18, 3e18, 5e18, 7e18, 9e18]
            .into_iter()
            .find(|w| (exact[2] + w) - w != exact[2])
            .expect("some w moves the relations phase");
        let before = exact[0] + exact[1] + (exact[2] + w) + exact[3] + exact[4];
        let legacy_phases = vec![exact[0], exact[1], (exact[2] + w) - w, exact[3], exact[4]];
        let legacy_total = before - w;
        tamper(&dir, |recs, _| {
            let r = &mut recs[IC_SEQ];
            r["detail"]["decomposition"]["solver"] = json!({"gae": w, "priced_by": "measured"});
            r["detail"]["framework_total_gae_before_solver_removal"] = json!(before);
            for (ph, g) in r["phases"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .zip(&legacy_phases)
            {
                ph["gae"] = json!(g);
            }
            r["cost"]["total_gae"] = json!(legacy_total);
            let sqrt_r = r["workload"]["curve"]["r"].as_f64().unwrap().sqrt();
            let floor = r["boundaries"]["floor_s"].as_f64().unwrap();
            r["cost"]["s"] = json!(legacy_total / sqrt_r);
            r["boundaries"]["ratio_to_floor"] = json!(legacy_total / sqrt_r / floor);
        });
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                replay_seqs: vec![IC_SEQ as u64],
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.replays.len(), 1);
        assert!(!rep.replays[0].reproduced, "{:?}", rep.replays[0]);
        assert!(
            rep.replays[0].detail.contains("total_gae")
                || rep.replays[0].detail.contains("phase gae"),
            "{}",
            rep.replays[0].detail
        );
        assert!(
            !rep.replays[0].detail.contains("legacy"),
            "{}",
            rep.replays[0].detail
        );
        // The record's own pre-removal total reproduces from its forged
        // solver term, so the old function alone accepted it.
        let forged = &read_record_file(&dir).unwrap().lines[IC_SEQ].1;
        assert!(legacy_solver_rounding(&exact, &forged.detail).is_some());
        // The exception is closed to it twice over: its record id is not on
        // the list, and the difference it stands for is not rounding.
        assert!(!legacy_rounding_allowed(
            &forged.session_id,
            &forged.record_id
        ));
        assert!(!legacy_rounding_bounded(
            &exact,
            exact_total,
            &legacy_phases,
            legacy_total,
            &forged.detail
        ));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn legacy_rounding_is_bounded_by_ulps_of_the_pre_removal_total() {
        let exact = [3.223395, 0.0, 611.5963, 2.721705, 15.0];
        let w = 91449423.0;
        let before = exact[0] + exact[1] + (exact[2] + w) + exact[3] + exact[4];
        let d = detail(w, before, "measured");
        let (ph, tot) = legacy_solver_rounding(&exact, &d).unwrap();
        let exact_total: f64 = exact.iter().sum();
        // The real rounding is within the bound...
        assert!(legacy_rounding_bounded(&exact, exact_total, &ph, tot, &d));
        // ...a figure 0.1 gae off is not rounding, and neither is one a
        // whole phase off.
        let mut off = ph.clone();
        off[2] += 0.1;
        assert!(!legacy_rounding_bounded(&exact, exact_total, &off, tot, &d));
        assert!(!legacy_rounding_bounded(
            &exact,
            exact_total,
            &ph,
            tot + 0.1,
            &d
        ));
        assert!(!legacy_rounding_bounded(
            &exact,
            exact_total,
            &ph,
            tot,
            &json!({})
        ));
        // No record outside the list may use the exception.
        assert!(!legacy_rounding_allowed(
            "ECBS1h000000000000",
            "ECR1h000000000000"
        ));
        for (s, r) in LEGACY_ROUNDING_RECORDS {
            assert!(s.starts_with("ECBS1h") && r.starts_with("ECR1h"));
        }
    }

    /// t06/t07: a record cannot opt out of replay by what it says about
    /// itself.
    #[test]
    fn determinism_comes_from_the_registry_not_the_record() {
        let dir = fresh("t06");
        tamper(&dir, |recs, _| {
            let r = &mut recs[RHO_SEQ];
            rescale(r, 0.5);
            r["cost"]["deterministic"] = json!(false);
            r["cost"]["nondeterminism"] = json!(["host_timer"]);
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert!(
            problems_with(&rep, "declares itself nondeterministic"),
            "{:?}",
            rep.problems
        );
        // And the record is still a replay candidate: its flag decides
        // nothing.
        let records = read_record_file(&dir).unwrap().lines;
        assert!(replayable(&records[RHO_SEQ].1));
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                replay_seqs: vec![RHO_SEQ as u64],
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(rep.replays.len(), 1);
        assert!(!rep.replays[0].reproduced);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn registry_determinism_by_method() {
        let mk = |id: &str, budget: Option<&str>| ResolvedMethod {
            id: id.into(),
            family: String::new(),
            params: budget
                .map(|b| BTreeMap::from([("solver_budget_seconds".to_string(), b.to_string())]))
                .unwrap_or_default(),
            method_id: String::new(),
            method_sha256: String::new(),
            entry: String::new(),
        };
        assert_eq!(
            method_determinism(&mk("rho.negation", None)),
            Determinism::Always
        );
        assert_eq!(
            method_determinism(&mk("ic.pipeline", Some("0"))),
            Determinism::Always
        );
        assert_eq!(
            method_determinism(&mk("ic.pipeline", Some("5"))),
            Determinism::Budgeted
        );
        assert_eq!(
            method_determinism(&mk("ic.cm_folded_native_v1", None)),
            Determinism::Never
        );
    }

    #[test]
    fn a_verified_run_recast_as_a_failure_is_caught() {
        // t07: verified → timeout, answer and cost left in place.
        let dir = fresh("t07");
        tamper(&dir, |recs, _| {
            let o = &mut recs[RHO_SEQ]["outcome"];
            o["status"] = json!("timeout");
            o["recovered"] = json!(null);
            o["matches_target"] = json!(null);
            o["matches_planted"] = json!(null);
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert!(
            problems_with(&rep, "a timeout is a SIGKILL"),
            "{:?}",
            rep.problems
        );
        assert!(problems_with(&rep, "carries a cost"), "{:?}", rep.problems);
        let _ = std::fs::remove_dir_all(&dir);

        // Even with the cost stripped, a "timeout" must look like one: a
        // watchdog kill, after the spec's timeout.
        let dir = fresh("t07b");
        tamper(&dir, |recs, _| {
            let r = &mut recs[RHO_SEQ];
            r["outcome"]["status"] = json!("timeout");
            r["outcome"]["recovered"] = json!(null);
            r["outcome"]["matches_target"] = json!(null);
            r["outcome"]["matches_planted"] = json!(null);
            r["cost"]["total_gae"] = json!(null);
            r["cost"]["s"] = json!(null);
            r["cost"]["deterministic"] = json!(false);
            r["cost"]["nondeterminism"] = json!([]);
            r["boundaries"]["ratio_to_floor"] = json!(null);
            r["boundaries"]["automorphisms_used"] = json!(null);
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert!(
            problems_with(&rep, "a timeout is a SIGKILL"),
            "{:?}",
            rep.problems
        );
        assert!(
            problems_with(&rep, "before the spec's 180 s"),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);

        // verified → exhausted: only a replay sees that the method finds it.
        let dir = fresh("t07c");
        tamper(&dir, |recs, _| {
            let o = &mut recs[RHO_SEQ]["outcome"];
            o["status"] = json!("exhausted");
            o["recovered"] = json!(null);
            o["matches_target"] = json!(null);
            o["matches_planted"] = json!(null);
        });
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                replay_seqs: vec![RHO_SEQ as u64],
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!rep.ok);
        assert!(!rep.replays[0].reproduced);
        assert!(
            rep.replays[0].detail.contains("exhaustion"),
            "{}",
            rep.replays[0].detail
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_genuine_timeout_and_crash_have_no_problems() {
        let dir = fresh("shape");
        let text = std::fs::read_to_string(dir.join("spec.json")).unwrap();
        let p = plan(Spec::from_json(&text).unwrap()).unwrap();
        let host: HostCapsule =
            serde_json::from_str(&std::fs::read_to_string(dir.join("host.json")).unwrap()).unwrap();
        let started = read_session(&dir).unwrap().started_unix_ms;
        let mut r = read_record_file(&dir).unwrap().lines[RHO_SEQ].1.clone();
        let arm = &p.arms[p.executions[RHO_SEQ].arm];
        assert_eq!(
            consistency_problems(&r, arm, &host, 180, started),
            Vec::<String>::new()
        );
        r.outcome.status = "timeout".into();
        r.outcome.exit = Some("signal 9".into());
        r.outcome.recovered = None;
        r.cost.total_gae = None;
        r.cost.s = None;
        r.boundaries.ratio_to_floor = None;
        r.time.process_wall_ns = 180_000_000_001;
        assert_eq!(
            consistency_problems(&r, arm, &host, 180, started),
            Vec::<String>::new()
        );
        r.time.process_wall_ns = 179_000_000_000;
        assert_eq!(consistency_problems(&r, arm, &host, 180, started).len(), 1);
        r.outcome.status = "crashed".into();
        r.outcome.exit = Some("signal 11".into());
        assert_eq!(
            consistency_problems(&r, arm, &host, 180, started),
            Vec::<String>::new()
        );
        r.outcome.exit = Some("exit 0".into());
        assert_eq!(consistency_problems(&r, arm, &host, 180, started).len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_replay_sample_is_random_seeded_and_recorded() {
        // The old sample, candidates[j * len / n], was a fixed function of
        // the record count.
        let fixed = |len: usize, n: usize| -> Vec<usize> { (0..n).map(|j| j * len / n).collect() };
        let a = choose_replays(40, 5, 1);
        let b = choose_replays(40, 5, 1);
        let c = choose_replays(40, 5, 2);
        assert_eq!(a, b, "same seed, same draw");
        assert_ne!(a, c, "another seed, another draw");
        assert_ne!(a, fixed(40, 5));
        assert!(a.windows(2).all(|w| w[0] < w[1]) && a.iter().all(|&i| i < 40));
        // Over many seeds every candidate is drawn, about evenly.
        let mut hits = vec![0u32; 40];
        for seed in 0..4000u64 {
            for i in choose_replays(40, 5, seed) {
                hits[i] += 1;
            }
        }
        // Expected 500 each.
        assert!(hits.iter().all(|&h| (350..650).contains(&h)), "{hits:?}");
        assert_eq!(choose_replays(3, 10, 7), vec![0, 1, 2]);

        // In the receipt.
        let dir = fresh("sample");
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                replay: 2,
                replay_seed: Some(12345),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(rep.ok, "{:?}", rep.problems);
        assert_eq!(rep.replay_seed, Some(12345));
        assert_eq!(rep.replay_selection.as_deref(), Some("sample"));
        // 12 measured runs of a deterministic method are candidates (six
        // warm-ups are not).
        assert_eq!(rep.replay_candidates, Some(12));
        assert_eq!(rep.replays.len(), 2);
        let again = audit_opts(
            &dir,
            &AuditOptions {
                replay: 2,
                replay_seed: Some(12345),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            rep.replays.iter().map(|r| r.seq).collect::<Vec<_>>(),
            again.replays.iter().map(|r| r.seq).collect::<Vec<_>>()
        );
        // Without a seed one is drawn and recorded.
        let drawn = audit_opts(&dir, &opts(1)).unwrap();
        assert!(drawn.replay_seed.is_some());
        // Zero replays: no seed, no selection.
        let none = audit_opts(&dir, &opts(0)).unwrap();
        assert_eq!((none.replay_seed, none.replay_selection), (None, None));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_old_receipt_still_deserializes() {
        let old = r#"{"schema":"ecbench.audit/v1","session_id":"ECBS1h000000000000","session_status":"complete","records_bound":true,
            "ok":true,"problems":[],"checks":{"records":true},"records":1,"verified_records":1,"regraded":true,
            "replays":[{"seq":0,"record_id":"ECR1h000000000000","reproduced":true,"detail":"identical"}],
            "files":{},"auditor_binary_sha256":null,"audited_unix_ms":1}"#;
        let rep: AuditReport = serde_json::from_str(old).unwrap();
        assert_eq!(rep.replay_seed, None);
        assert_eq!(rep.replay_selection, None);
        assert!(rep.warnings.is_empty());
        assert_eq!(rep.planned_executions, 0);
        // New fields appear only when set.
        let text = serde_json::to_string(&rep).unwrap();
        assert!(
            !text.contains("replay_seed") && !text.contains("warnings"),
            "{text}"
        );
    }

    /// t13: raising a level and editing `grading_version` in the unhashed
    /// session.json used to skip the regrade.
    #[test]
    fn the_regrade_runs_under_the_recorded_version_and_can_fail() {
        let raise = |recs: &mut Vec<serde_json::Value>| {
            let i = &mut recs[RHO_SEQ]["isolation"];
            i["level"] = json!("L3");
            i["blockers"] = json!([]);
        };
        // Current version: caught.
        let dir = fresh("t14");
        tamper(&dir, |recs, _| raise(recs));
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok && !rep.regraded);
        assert_eq!(rep.checks.get("regraded"), Some(&false));
        // Version 1 named in session.json: still caught, the record is
        // regraded under version 1.
        let dir1 = fresh("t13");
        tamper(&dir1, |recs, s| {
            raise(recs);
            s["grading_version"] = json!(1);
        });
        let rep = audit_opts(&dir1, &opts(0)).unwrap();
        assert!(!rep.ok && !rep.regraded);
        assert!(problems_with(&rep, "graded L3"), "{:?}", rep.problems);
        // A version this binary does not have is a failed check, not a skip.
        let dir9 = fresh("t13b");
        tamper(&dir9, |_, s| s["grading_version"] = json!(99));
        let rep = audit_opts(&dir9, &opts(0)).unwrap();
        assert!(!rep.ok && !rep.regraded);
        assert_eq!(rep.checks.get("regraded"), Some(&false));
        assert!(
            problems_with(&rep, "grading version 99 is not supported"),
            "{:?}",
            rep.problems
        );
        for d in [dir, dir1, dir9] {
            let _ = std::fs::remove_dir_all(&d);
        }
    }

    #[test]
    fn a_failed_audit_exits_nonzero() {
        let dir = fresh("exit");
        tamper(&dir, |recs, _| rescale(&mut recs[RHO_SEQ], 0.5));
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.exit_status(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// t15: role, environment class and binary hash against the spec arm
    /// and the host capsule.
    #[test]
    fn role_environment_and_binary_follow_the_spec_and_the_capsule() {
        let dir = fresh("t15");
        tamper(&dir, |recs, _| {
            let r = &mut recs[RHO_SEQ];
            r["role"] = json!("control");
            r["env_class_id"] = json!("ECBENV2h000000000000");
            r["binary_sha256"] = json!("0".repeat(64));
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        for needle in [
            "role `control`",
            "environment class ECBENV2h000000000000",
            "binary",
        ] {
            assert!(problems_with(&rep, needle), "{needle}: {:?}", rep.problems);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// t17: deleting `field_ops` from a counting method's record is not an
    /// unknown.
    #[test]
    fn deleting_field_ops_is_a_problem_where_the_method_counts_them() {
        let dir = fresh("t17");
        tamper(&dir, |recs, _| {
            recs[RHO_SEQ].as_object_mut().unwrap().remove("field_ops");
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert!(
            problems_with(&rep, "counts field operations"),
            "{:?}",
            rep.problems
        );
        // Absent where the group does not count them (the ic arms): fine.
        let clean = fresh("t17b");
        let records = read_record_file(&clean).unwrap().lines;
        assert!(records[IC_SEQ].1.field_ops.is_none());
        let started = read_session(&clean).unwrap().started_unix_ms;
        assert!(!counts_field_ops(&records[IC_SEQ].1, started));
        assert!(counts_field_ops(&records[RHO_SEQ].1, started));
        // A session from before the block existed has none to lose.
        assert!(!counts_field_ops(
            &records[RHO_SEQ].1,
            FIELD_OPS_SINCE_UNIX_MS - 1
        ));
        for d in [dir, clean] {
            let _ = std::fs::remove_dir_all(&d);
        }
    }

    /// t21: a phase edited to hide cost, with the total left.
    #[test]
    fn phases_must_sum_to_the_total() {
        let dir = fresh("t21");
        tamper(&dir, |recs, _| {
            let g = recs[RHO_SEQ]["phases"][1]["gae"].as_f64().unwrap();
            recs[RHO_SEQ]["phases"][1]["gae"] = json!(g * 3.0);
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert!(problems_with(&rep, "phases sum to"), "{:?}", rep.problems);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// t22: counters edited and re-sealed are invisible without a replay,
    /// and a replay of that run sees them.
    #[test]
    fn edited_counters_are_found_by_replaying_that_run() {
        let dir = fresh("t22");
        tamper(&dir, |recs, _| {
            let c = recs[RHO_SEQ]["counters"].as_object_mut().unwrap();
            let k = c.keys().next().unwrap().clone();
            let v = c[&k].as_u64().unwrap();
            c.insert(k, json!(v + 1000));
        });
        assert!(audit_opts(&dir, &opts(0)).unwrap().ok);
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                replay_seqs: vec![RHO_SEQ as u64],
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!rep.ok);
        assert!(problems_with(&rep, "counters"), "{:?}", rep.problems);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn automorphisms_used_cannot_exceed_those_available() {
        let dir = fresh("auto");
        tamper(&dir, |recs, _| {
            recs[RHO_SEQ]["boundaries"]["automorphisms_used"] = json!(6);
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert!(
            problems_with(&rep, "used 6 automorphisms, the curve has 2"),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// t23: `status_counts` and `executions_planned` are not free text.
    #[test]
    fn session_counts_follow_from_the_records_and_the_plan() {
        let dir = fresh("t23");
        tamper(&dir, |_, s| {
            s["status_counts"] = json!({"verified": 1000});
            s["executions_planned"] = json!(5);
        });
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("status_counts"), Some(&false));
        assert_eq!(rep.checks.get("executions_planned"), Some(&false));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// t26: rewriting the session id everywhere (session.json and every
    /// record) used to be self-consistent.
    #[test]
    fn the_session_id_recomputes_from_its_preimage() {
        let dir = fresh("t26");
        let old = read_session(&dir).unwrap().session_id;
        tamper(&dir, |recs, s| {
            for r in recs.iter_mut() {
                r["session_id"] = json!("ECBS1h000000000000");
            }
            s["session_id"] = json!("ECBS1h000000000000");
        });
        assert_ne!(old, "ECBS1h000000000000");
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("session_id"), Some(&false));
        assert!(
            problems_with(&rep, "its preimage hashes to"),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_session_id_preimage_includes_the_spec_hash_and_start_time() {
        let dir = fresh("t26b");
        tamper(&dir, |_, s| s["started_unix_ms"] = json!(1));
        assert_eq!(
            audit_opts(&dir, &opts(0)).unwrap().checks.get("session_id"),
            Some(&false)
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Truncation: an interrupted session is a prefix and says so; a
    /// complete one cannot be one.
    #[test]
    fn an_interrupted_prefix_is_reported_prominently() {
        let dir = fresh("trunc");
        tamper(&dir, |recs, s| {
            recs.truncate(10);
            s["status"] = json!("interrupted");
            s["records_written"] = json!(10);
            s["status_counts"] = json!({"verified": 10});
        });
        // Not accepted by default.
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("session_complete"), Some(&false));
        // Accepted with the flag, and loudly.
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                allow_interrupted: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(rep.ok, "{:?}", rep.problems);
        assert_eq!(rep.session_status, "interrupted");
        assert_eq!(rep.interruption_point, Some(10));
        assert_eq!(rep.planned_executions, 18);
        assert!(
            rep.warnings[0].starts_with("INTERRUPTED: session ends after 10 of 18"),
            "{:?}",
            rep.warnings
        );
        let _ = std::fs::remove_dir_all(&dir);

        // A complete session with records missing is not accepted.
        let dir = fresh("trunc2");
        tamper(&dir, |recs, s| {
            recs.truncate(10);
            s["records_written"] = json!(10);
            s["status_counts"] = json!({"verified": 10});
        });
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                allow_interrupted: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("record_count"), Some(&false));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_interruption_the_runner_recorded_must_end_the_records_there() {
        let dir = fresh("dated");
        tamper(&dir, |recs, s| {
            recs.truncate(10);
            s["status"] = json!("interrupted");
            s["records_written"] = json!(10);
            s["status_counts"] = json!({"verified": 10});
            s["last_seq"] = json!(9);
            s["stop"] = json!({"kind": "interrupt", "seq": 10, "signal": 15, "message": "interrupted by signal 15"});
        });
        let allow = AuditOptions {
            allow_interrupted: true,
            ..Default::default()
        };
        let rep = audit_opts(&dir, &allow).unwrap();
        assert!(rep.ok, "{:?}", rep.problems);
        assert_eq!(rep.checks.get("last_seq"), Some(&true));
        assert_eq!(rep.checks.get("stop"), Some(&true));
        assert!(
            rep.warnings
                .iter()
                .any(|w| w.starts_with("stopped: interrupt at seq Some(10)")),
            "{:?}",
            rep.warnings
        );
        // The same records with a later point recorded: some were removed.
        tamper(&dir, |_, s| {
            s["last_seq"] = json!(13);
            s["stop"]["seq"] = json!(14);
        });
        let rep = audit_opts(&dir, &allow).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("last_seq"), Some(&false));
        assert_eq!(rep.checks.get("stop"), Some(&false));
        // A complete session has no stop.
        let dir2 = fresh("dated2");
        tamper(&dir2, |_, s| {
            s["stop"] = json!({"kind": "error", "seq": 17, "message": "x"});
        });
        assert_eq!(
            audit_opts(&dir2, &allow).unwrap().checks.get("stop"),
            Some(&false)
        );
        for d in [dir, dir2] {
            let _ = std::fs::remove_dir_all(&d);
        }
    }

    /// t18/t19: a torn final line gives a receipt, not an error.
    #[test]
    fn a_torn_final_line_yields_a_receipt() {
        // Complete session: a problem in the receipt.
        let dir = fresh("t18");
        let body = std::fs::read(dir.join("records.jsonl")).unwrap();
        std::fs::write(dir.join("records.jsonl"), &body[..body.len() - 500]).unwrap();
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                allow_interrupted: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("records_tail"), Some(&false));
        assert_eq!(rep.records, 17);
        let _ = std::fs::remove_dir_all(&dir);

        // A killed runner (session.json still as first written): the intact
        // prefix is audited and the torn line reported.
        let dir = killed_copy("t19");
        let body = std::fs::read(dir.join("records.jsonl")).unwrap();
        std::fs::write(dir.join("records.jsonl"), &body[..body.len() - 300]).unwrap();
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                allow_interrupted: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(rep.ok, "{:?}", rep.problems);
        assert_eq!(rep.records, 3);
        assert!(
            rep.warnings.iter().any(|w| w.contains("torn line")),
            "{:?}",
            rep.warnings
        );
        assert_eq!(rep.interruption.as_deref(), Some(NEVER_FINALISED));
        // Strict mode: not accepted.
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_bad_line_in_the_middle_is_a_problem_not_an_error() {
        let dir = fresh("garbage");
        let text = std::fs::read_to_string(dir.join("records.jsonl")).unwrap();
        let mut lines: Vec<&str> = text.lines().collect();
        lines[4] = "{\"not\":\"a record\"}";
        std::fs::write(dir.join("records.jsonl"), lines.join("\n") + "\n").unwrap();
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("records_parse"), Some(&false));
        assert_eq!(rep.records, 4);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Duplicate keys and non-finite numbers in a record line are refused
    /// by the reader the audit uses.
    #[test]
    fn a_record_line_with_a_duplicate_key_or_an_infinite_number_is_refused() {
        let line = FRESH[4].1.lines().next().unwrap().to_string();
        assert!(from_str_legacy_floats::<Record>(&line).is_ok());
        let dup = line.replacen('{', r#"{"warmup":false,"#, 1);
        let e = from_str_legacy_floats::<Record>(&dup).unwrap_err();
        assert!(e.contains("duplicate key"), "{e}");
        let nested = line.replacen(r#""counters":{"#, r#""counters":{"steps":1,"steps":2,"#, 1);
        assert_ne!(nested, line);
        assert!(from_str_legacy_floats::<Record>(&nested).is_err());
        let total = serde_json::from_str::<serde_json::Value>(&line).unwrap()["cost"]["total_gae"]
            .to_string();
        let inf = line.replacen(&format!("\"total_gae\":{total}"), "\"total_gae\":1e400", 1);
        assert_ne!(inf, line);
        let e = from_str_legacy_floats::<Record>(&inf).unwrap_err();
        assert!(e.contains("not a finite f64"), "{e}");
        // The audit reports it rather than failing without a receipt.
        let dir = fresh("inf");
        let mut lines: Vec<String> = FRESH[4].1.lines().map(String::from).collect();
        lines[5] = inf;
        std::fs::write(dir.join("records.jsonl"), lines.join("\n") + "\n").unwrap();
        let rep = audit_opts(&dir, &opts(0)).unwrap();
        assert!(!rep.ok);
        assert_eq!(rep.checks.get("records_parse"), Some(&false));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A record beyond the plan used to index past the executions in the
    /// replay and panic.
    #[test]
    fn a_record_beyond_the_plan_is_a_problem_not_a_panic() {
        let dir = fresh("t12");
        tamper(&dir, |recs, s| {
            let mut extra = recs[RHO_SEQ].clone();
            extra["seq"] = json!(recs.len());
            recs.push(extra);
            s["records_written"] = json!(19);
        });
        let rep = audit_opts(&dir, &opts(REPLAY_ALL)).unwrap();
        assert!(!rep.ok);
        assert!(
            problems_with(&rep, "no planned execution"),
            "{:?}",
            rep.problems
        );
        // And asking for it by sequence number is a problem too.
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                replay_seqs: vec![18],
                ..Default::default()
            },
        )
        .unwrap();
        assert!(
            problems_with(&rep, "replay of seq 18"),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
        // A record whose seq points past the plan but sits in an early line.
        let dir = fresh("t12b");
        tamper(&dir, |recs, _| recs[RHO_SEQ]["seq"] = json!(1000));
        let rep = audit_opts(&dir, &opts(REPLAY_ALL)).unwrap();
        assert!(!rep.ok);
        assert!(problems_with(&rep, "out of sequence"), "{:?}", rep.problems);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn replaying_a_clean_session_reproduces_it() {
        let dir = fresh("replay");
        let rep = audit_opts(
            &dir,
            &AuditOptions {
                replay_seqs: vec![RHO_SEQ as u64, IC_SEQ as u64],
                ..Default::default()
            },
        )
        .unwrap();
        assert!(rep.ok, "{:?}", rep.problems);
        assert_eq!(rep.replays.len(), 2);
        assert!(rep
            .replays
            .iter()
            .all(|r| r.reproduced && r.detail == "identical"));
        assert_eq!(rep.replay_selection.as_deref(), Some("explicit"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_legacy_list_names_sealed_records_of_the_committed_session() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/research/ecbench_yield_sweep_20261004/sessions/koblitz/records.jsonl"
        );
        let Ok(text) = std::fs::read_to_string(path) else {
            return; // a checkout without the session
        };
        assert_eq!(LEGACY_ROUNDING_RECORDS.len(), 48);
        let mut seen = BTreeSet::new();
        for (session, record) in LEGACY_ROUNDING_RECORDS {
            assert!(seen.insert((*session, *record)), "{record} listed twice");
            let line = text
                .lines()
                .find(|l| l.contains(&format!("\"record_id\":\"{record}\"")))
                .unwrap_or_else(|| panic!("{record} is not in the session"));
            assert!(check_line_seal(line, record));
            let r: Record = from_str_legacy_floats(line).unwrap();
            assert_eq!(r.session_id, *session);
            assert_eq!(r.method.id, "ic.pipeline");
            assert!(!r.warmup && r.outcome.status == "verified");
            // The exception's own preconditions hold in the record.
            let solver = r.detail.pointer("/decomposition/solver").unwrap();
            assert_ne!(solver["priced_by"], "pinned");
            assert!(solver["gae"].as_f64().unwrap() > 0.0);
        }
    }
}
