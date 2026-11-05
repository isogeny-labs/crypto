//! Records: one per execution, failures included.
//!
//! A record carries everything needed to audit its own figure: the
//! method and workload identities, the verified answer, the cost in the
//! unit with every phase, what was not priced, the process's resource
//! use, and the isolation level its own observations earned with the
//! reason for every level it did not.

use std::collections::BTreeMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cryptanalysis::ecbench::canonical::sha256_hex;
#[cfg(test)]
use crate::cryptanalysis::ecbench::host::HostCapsule;
use crate::cryptanalysis::ecbench::isolation::{
    anon_pages_by_node, current_cpu, mempolicy, mems_allowed, own_affinity, Conditions, HwCounters,
    HwCounts, SelfPlacement,
};

use crate::cryptanalysis::ecbench::methods::{
    resolve, FactorBaseFacts, MethodSpec, OnlineWindow, PhaseRecord, ResolvedMethod, SolveReport,
    SolverStats,
};
use crate::cryptanalysis::ecbench::spec::Level;
use crate::cryptanalysis::ecbench::workload::{
    legacy_automorphisms, CurveFacts, CurveSpec, TargetKind, Workload, FLOOR_RULE_V2,
};
use crate::cryptanalysis::ic_boundary::FieldOps;
use crate::cryptanalysis::ic_measurement;

pub use super::measurement::{
    grade, grade_for_version, grade_v2, grade_v3, grading_version_supported, schedstat, Grade,
    GradeInput, OnlineClockDiagnostics, SchedStat, SolveObs, SolveProbe, Timing, GRADING_VERSION,
    MIN_SOLVE_WALL_NS_V3, TOO_SHORT_TO_ADMIT,
};

pub const RECORD_SCHEMA: &str = "ecbench.record/v1";

/// The key under which a session records the version of the inherited-F4
/// word-operation accounting it was made under.
pub const F4_ACCOUNTING_KEY: &str = "inherited_f4_specialise";

/// The versions of the counted-work definitions this binary charges, recorded
/// in every new session.  A method's identity hashes its configuration, not
/// the code, so a change to what a unit counts is otherwise visible only as
/// a different binary hash and, on replay, as bare count differences.
pub fn current_accounting_versions() -> BTreeMap<String, u32> {
    BTreeMap::from([(
        F4_ACCOUNTING_KEY.to_string(),
        crate::cryptanalysis::inherited_f4::INHERITED_F4_ACCOUNTING_VERSION,
    )])
}

/// When inherited-F4 specialisation became eager on every host (`#2043`,
/// 2026-10-17T17:19:59Z), in Unix milliseconds.  Before it a host without BMI2
/// charged the lazy rewrite, which counts differently.
pub const F4_EAGER_EVERYWHERE_UNIX_MS: u128 = 1_792_257_599_000;

/// What to add to a failed replay of an inherited-F4 record, given the
/// accounting versions its session recorded; `None` when the accounting
/// cannot be the reason, so the difference is something else and is left
/// unattributed.
///
/// A session without the field predates it, and is not assigned a version.
/// The one change known to have moved the counts is `#2043`: it is named only
/// when it can apply, that is when the session started before it and its host
/// had no BMI2 (a BMI2 x86-64 host was eager before it as it is now).
pub fn f4_accounting_note(
    recorded: &BTreeMap<String, u32>,
    started_unix_ms: u128,
    recorded_host_has_bmi2: bool,
) -> Option<String> {
    let current = crate::cryptanalysis::inherited_f4::INHERITED_F4_ACCOUNTING_VERSION;
    match recorded.get(F4_ACCOUNTING_KEY) {
        Some(&v) if v == current => None,
        Some(&v) => Some(format!(
            "recorded under inherited-F4 accounting v{v}, this binary charges v{current}: \
             the counts are different definitions, not a disagreement"
        )),
        None if recorded_host_has_bmi2 || started_unix_ms >= F4_EAGER_EVERYWHERE_UNIX_MS => None,
        None => Some(format!(
            "recorded before #2043 (2026-10-17) on a host without BMI2, where inherited-F4 \
             specialisation was charged lazily; this binary charges it eagerly on every host \
             (accounting v{current}), so a count difference here is expected, and later changes \
             recorded only by binary hash can add to it"
        )),
    }
}

pub const CHILD_INPUT_SCHEMA: &str = "ecbench.child_input/v1";
pub const CHILD_OUTPUT_SCHEMA: &str = "ecbench.child_output/v1";

/// The unit every cost is in: group-addition equivalents (additions plus
/// doublings, plus native work at the repository's pinned ratios where
/// one exists), divided by `√r` for `S`.
pub const UNIT: &str = "ecbench.gae";

// ── The child protocol ─────────────────────────────────────────────

/// What the runner sends a measured child on stdin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChildInput {
    /// Reserved explicit diagnostic context; producer execution remains disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<super::diagnostics::Context>,
    /// Explicit v2 transport, admitted only after identity and custody checks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_lifecycle: Option<DiagnosticChild>,
    pub schema: String,
    pub curve: CurveSpec,
    pub target_seed: u64,
    pub target_index: u64,
    /// Planted, planted_shared or public; older inputs are planted.
    #[serde(default, skip_serializing_if = "TargetKind::is_plain_planted")]
    pub target_kind: TargetKind,
    pub expected_workload_id: String,
    pub method: MethodSpec,
    pub expected_method_id: String,
    pub algorithm_seed: u64,
}

/// Explicit provenance and output-root transport for lifecycle diagnostics.
/// This is not scientific admission, and no ambient directory is consulted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticChild {
    pub launch: super::diagnostic_launch::DiagnosticLaunch,
    pub envelope: super::diagnostic_launch::ChildEnvelope,
    pub output_root: std::path::PathBuf,
}

pub const MAX_DIAGNOSTIC_CHILD_BYTES: usize = 4 * super::diagnostic_launch::MAX_LAUNCH_BYTES;

/// Hash the running executable rather than trusting an envelope's declaration.
/// Linux /proc/self/exe pins the exec'd inode even if its pathname is replaced.
fn diagnostic_running_binary_sha256() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    let path = std::path::PathBuf::from("/proc/self/exe");
    #[cfg(not(target_os = "linux"))]
    let path = std::env::current_exe().map_err(|error| error.to_string())?;
    crate::store::hash_file(&path)
        .map(|(digest, _)| digest)
        .map_err(|error| format!("diagnostic child executable identity: {error}"))
}

impl DiagnosticChild {
    /// Validate metadata and output custody without constructing a workload.
    pub fn validate(&self, job: &ChildInput) -> Result<(), String> {
        if job.diagnostics.is_some() {
            return Err("mixed v1/v2 diagnostic contexts".into());
        }
        if job.diagnostic_lifecycle.as_ref() != Some(self) {
            return Err("diagnostic child transport differs from scheduled input".into());
        }
        if serde_json::to_vec(self).map_err(|e| e.to_string())?.len() > MAX_DIAGNOSTIC_CHILD_BYTES {
            return Err("diagnostic child exceeds byte bound".into());
        }
        self.envelope.validate(&self.launch)?;
        let method = resolve(&job.method)?;
        if job.schema != CHILD_INPUT_SCHEMA
            || method.id != self.envelope.arm.method
            || method.method_id != job.expected_method_id
            || super::diagnostic_launch::resolved_method_binding(&method)?
                != self.envelope.prefix.identity.method_config_sha256
            || job.algorithm_seed != self.envelope.prefix.identity.seed
        {
            return Err("diagnostic child scheduled method/seed mismatch".into());
        }
        validate_diagnostic_root(&self.output_root)
    }

    /// After the ordinary workload rebuild and expected-ID check, bind the full
    /// serialized record value exactly as the parent diagnostic reader does.
    /// This is distinct from Workload.workload_sha256's internal identity.
    pub fn validate_workload(&self, workload: &Workload) -> Result<(), String> {
        let value = serde_json::to_value(workload).map_err(|e| e.to_string())?;
        let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
        if sha256_hex(&bytes) != self.envelope.prefix.identity.workload_sha256 {
            return Err("diagnostic child rebuilt workload binding mismatch".into());
        }
        Ok(())
    }

    /// Exclusive lifecycle leaf; prefix records must use a different leaf.
    pub fn lifecycle_directory(&self) -> std::path::PathBuf {
        self.output_root
            .join(format!("{}-lifecycle", self.envelope.prefix.relative_leaf))
    }
}

/// Trusted runner creates this private root before spawning children. Refuse
/// traversal, symlink components and shared writable non-sticky ancestors.
/// Metadata checks are not protection against a hostile process with our UID;
/// that owner is already trusted to replace the executable and input pipe.
fn validate_diagnostic_root(root: &std::path::Path) -> Result<(), String> {
    use std::path::Component;
    if !root.is_absolute()
        || root.as_os_str().len() > 4096
        || root
            .components()
            .any(|c| !matches!(c, Component::RootDir | Component::Normal(_)))
    {
        return Err("diagnostic root must be bounded absolute path without traversal".into());
    }
    if std::fs::canonicalize(root).map_err(|e| e.to_string())? != root {
        return Err("diagnostic root is not canonical".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        // SAFETY: geteuid takes no arguments and has no failure mode.
        let uid = unsafe { libc::geteuid() };
        for ancestor in root.ancestors() {
            let meta = std::fs::symlink_metadata(ancestor).map_err(|e| e.to_string())?;
            if !meta.is_dir() || meta.file_type().is_symlink() {
                return Err("diagnostic root has non-directory/symlink ancestor".into());
            }
            let mode = meta.mode();
            if ancestor == root {
                if meta.uid() != uid || mode & 0o7777 != 0o700 {
                    return Err(
                        "diagnostic root must be owner-only0700 and owned by child uid".into(),
                    );
                }
            } else if mode & 0o022 != 0 && mode & 0o1000 == 0 {
                return Err("diagnostic root has unprotected shared writable ancestor".into());
            }
        }
        Ok(())
    }
    #[cfg(not(unix))]
    {
        Err("diagnostic private root requires Unix ownership checks".into())
    }
}

/// Software integration seam: same lifecycle ownership on every arm. Caller
/// owns the enclosing cost interval. Returning this summary is diagnostic only;
/// it neither accepts a report nor covers parent publication or final-write tail.
/// Errors/panics never use Drop to manufacture a finished lifecycle record.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiagnosticCallbackSummary {
    pub lifecycle: super::diagnostic_runtime::LifecycleSummary,
    /// Writer observations only, never merged into algorithm counts or price.
    pub prefix_io: Option<Value>,
}

pub fn with_diagnostic_lifecycle<T>(
    diagnostic: &DiagnosticChild,
    job: &ChildInput,
    callback: impl FnOnce() -> Result<T, String>,
) -> Result<(Result<T, String>, DiagnosticCallbackSummary), String> {
    diagnostic.validate(job)?;
    let scope = super::diagnostic_runtime::DiagnosticScope::install(
        &diagnostic.lifecycle_directory(),
        diagnostic.envelope.prefix.identity.clone(),
    )?;
    let prefix = if diagnostic.envelope.arm.method == "ic.three_leads_genus2_pencil_symbolic_v2" {
        match super::three_leads::ScopedPencilProgress::install(
            &diagnostic
                .output_root
                .join(&diagnostic.envelope.prefix.relative_leaf),
            diagnostic.envelope.prefix.identity.clone(),
        ) {
            Ok(prefix) => Some(prefix),
            Err(error) => {
                let finished = scope.finish(super::diagnostic_runtime::FinishStatus::Error);
                return Err(format!(
                    "diagnostic prefix install: {error}; lifecycle finish: {finished:?}"
                ));
            }
        }
    } else {
        None
    };
    let result = callback();
    let prefix_io = prefix
        .map(|guard| {
            guard
                .finish()
                .and_then(|totals| serde_json::to_value(totals).map_err(|e| e.to_string()))
        })
        .transpose();
    let status = if result.is_ok() && prefix_io.is_ok() {
        super::diagnostic_runtime::FinishStatus::Returned
    } else {
        super::diagnostic_runtime::FinishStatus::Error
    };
    let lifecycle = scope.finish(status);
    // Both explicit finish attempts happen even if prefix publication failed.
    match (prefix_io, lifecycle) {
        (Ok(prefix_io), Ok(lifecycle)) => Ok((
            result,
            DiagnosticCallbackSummary {
                lifecycle,
                prefix_io,
            },
        )),
        (prefix, lifecycle) => Err(format!(
            "diagnostic finish failure: prefix={prefix:?}; lifecycle={lifecycle:?}"
        )),
    }
}

/// What the allocator was told before the solve, and what it is.
///
/// A fresh child builds its workload (curve, target, tables) before the
/// timed solve.  On glibc that leaves the fastbins full of freed chunks,
/// and the first sizeable allocation inside the solve (a hash table, a
/// jump table) pays `malloc_consolidate` for all of them: at small sizes
/// that is milliseconds against a solve of microseconds, and it differs
/// with how much garbage each method's set-up left.  The child therefore
/// trims the allocator after the workload is built and before any clock
/// starts, so every solve begins from the same clean allocator state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocatorFacts {
    /// `glibc 2.39`, `musl`, `libsystem_malloc` or `system`.
    pub name: String,
    /// `malloc_trim(0)` ran after set-up and before the solve clock.
    pub pre_solve_trim: bool,
    /// What `malloc_trim` returned (1: memory was returned to the OS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trim_result: Option<i32>,
    /// `M_MXFAST` set with `mallopt` before the workload was built, if it
    /// was (0 disables fastbins altogether).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mxfast: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Consolidate and return the allocator's free memory, so the solve that
/// follows starts from a clean state.  Called by the child once the
/// workload is built and before any measurement window opens.
pub fn prepare_allocator() -> AllocatorFacts {
    #[allow(unused_mut)]
    let mut facts = AllocatorFacts {
        name: crate::cryptanalysis::ecbench::host::allocator_name(),
        pre_solve_trim: false,
        trim_result: None,
        mxfast: None,
        note: None,
    };
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        // SAFETY: no preconditions; consolidates every arena's fastbins
        // and releases free pages.
        facts.trim_result = Some(unsafe { libc::malloc_trim(0) });
        facts.pre_solve_trim = true;
    }
    #[cfg(not(all(target_os = "linux", target_env = "gnu")))]
    {
        facts.note =
            Some("no malloc_trim on this allocator; the fastbin effect is a glibc one".into());
    }
    facts
}

/// What a measured child prints on stdout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChildOutput {
    pub schema: String,
    pub workload_id: Option<String>,
    pub method_id: Option<String>,
    pub report: Option<SolveReport>,
    /// Common cold method-call interval: dispatch through report construction,
    /// including per-call setup and in-child checks. Excludes workload creation,
    /// child serialization and the runner's independent audit verification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_call_wall_ns: Option<u64>,
    /// Explicit optional lifecycle observations, never mathematical acceptance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_lifecycle: Option<Value>,
    /// From before diagnostic validation/scope allocation through explicit finish.
    /// Excludes child-output serialization and parent resource/publication tails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_call_wall_ns: Option<u64>,
    pub error: Option<String>,
    pub placement: SelfPlacement,
    /// Scheduler statistics across the solve alone.
    pub schedstat_solve: Option<SchedStat>,
    /// Hardware counters across the solve alone.
    #[serde(default)]
    pub hw_solve: Option<HwCounts>,
    /// The child's own view of the solve window: switches, threads, IRQs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve_obs: Option<SolveObs>,
    /// What the allocator was told before the solve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocator: Option<AllocatorFacts>,
}

/// The measured child's whole job: rebuild the workload, check its
/// identity, solve, report.  It never sees the planted scalar's use: the
/// runner verifies the answer in its own process.
pub fn child_main(input: &str) -> ChildOutput {
    let mut placement = SelfPlacement {
        affinity: own_affinity(),
        cpu_at_start: current_cpu(),
        cpu_at_end: None,
        mems_allowed: mems_allowed(),
        mempolicy: mempolicy(),
        anon_pages_by_node: None,
    };
    let mut out = ChildOutput {
        schema: CHILD_OUTPUT_SCHEMA.into(),
        workload_id: None,
        method_id: None,
        report: None,
        method_call_wall_ns: None,
        diagnostic_lifecycle: None,
        diagnostic_call_wall_ns: None,
        error: None,
        placement: placement.clone(),
        schedstat_solve: None,
        hw_solve: None,
        solve_obs: None,
        allocator: None,
    };
    let job: ChildInput = match serde_json::from_str(input) {
        Ok(j) => j,
        Err(e) => {
            out.error = Some(format!("child input: {e}"));
            return out;
        }
    };
    if job.diagnostics.is_some() || job.diagnostic_lifecycle.is_some() {
        if input.len() > MAX_DIAGNOSTIC_CHILD_BYTES {
            out.error = Some("diagnostic child input exceeds byte bound".into());
            return out;
        }
        if let Err(error) = super::canonical::reject_duplicate_keys(input) {
            out.error = Some(error);
            return out;
        }
        if job.diagnostics.is_some() && job.diagnostic_lifecycle.is_some() {
            out.error = Some("mixed v1/v2 diagnostic contexts".into());
            return out;
        }
    }
    if let Some(diagnostic) = &job.diagnostic_lifecycle {
        // All checks happen before workload construction or producer installation.
        // This is software admission, not prospective research/budget approval.
        let admission = diagnostic.validate(&job).and_then(|()| {
            let actual = diagnostic_running_binary_sha256()?;
            if actual != diagnostic.envelope.prefix.identity.binary_sha256 {
                return Err("diagnostic child running binary binding mismatch".into());
            }
            Ok(())
        });
        if let Err(error) = admission {
            out.error = Some(error);
            return out;
        }
    }
    // The legacy v1 transport remains reserved and cannot activate a producer.
    if let Some(context) = &job.diagnostics {
        out.error = Some(match context.validate() {
            Ok(()) => "diagnostic producer is not enabled or admitted".into(),
            Err(error) => error,
        });
        return out;
    }
    #[allow(clippy::type_complexity)]
    let result = (|| -> Result<
        (
            SolveReport,
            Option<SchedStat>,
            HwCounts,
            SolveObs,
            AllocatorFacts,
        ),
        String,
    > {
        if job.schema != CHILD_INPUT_SCHEMA {
            return Err(format!("child input schema `{}`", job.schema));
        }
        let (w, inst) = Workload::build(
            &job.curve,
            job.target_seed,
            job.target_index,
            job.target_kind,
        )?;
        if w.workload_id != job.expected_workload_id {
            return Err(format!(
                "rebuilt workload {} is not the planned {}",
                w.workload_id, job.expected_workload_id
            ));
        }
        if let Some(diagnostic) = &job.diagnostic_lifecycle {
            diagnostic.validate_workload(&w)?;
        }
        let m = resolve(&job.method)?;
        if m.method_id != job.expected_method_id {
            return Err(format!(
                "method resolved to {}, planned {}",
                m.method_id, job.expected_method_id
            ));
        }
        // Harness tables a solve may consult by name are loaded outside
        // the measured interval.
        crate::cryptanalysis::curve_id::warm();
        // Everything the workload's set-up freed is consolidated now, so
        // the first sizeable allocation inside the solve does not pay for
        // it (see `AllocatorFacts`).  Not part of any clock.
        let allocator = prepare_allocator();
        // The old method/PMU/phase windows remain inside this callback. The
        // optional lifecycle envelope surrounds it, including common sink setup
        // and explicit finish. Without context, invoke the same callback directly.
        let solve_call = || {
            let counters = HwCounters::open();
            let probe_before = SolveProbe::read();
            let before = schedstat();
            // Profiling is opt-in and the environment lookup is outside both
            // boundaries. Callgrind's first part is workload/setup prework;
            // sum every part after that marker through the solve marker, since
            // ic_measurement may dump additional internal phase parts.
            let profile_solve = std::env::var("ECBENCH_CALLGRIND_SOLVE").as_deref() == Ok("1");
            counters.start();
            if profile_solve {
                ic_measurement::callgrind_dump(b"ecbench_before_solve\0");
            }
            let method_call_start = std::time::Instant::now();
            let report = crate::cryptanalysis::ecbench::methods::solve_measured(
                &m,
                &inst,
                &w.curve,
                &w.target,
                job.algorithm_seed,
            );
            if profile_solve {
                ic_measurement::callgrind_dump(b"ecbench_solve\0");
            }
            let mut report = report?;
            let hw = counters.stop();
            let after = schedstat();
            let probe_after = SolveProbe::read();
            // The factor-base inventory names the base in the record and changes
            // no count; it is rebuilt here, after every window has closed.
            crate::cryptanalysis::ecbench::methods::complete(
                &m,
                &inst,
                &w.curve,
                &w.target,
                &mut report,
            )?;
            // This common cold-method clock remains the outer boundary that the
            // pre-split `methods::solve` call supplied.  In particular it covers
            // post-measurement report completion, while the profiling, PMU and
            // scheduler windows above still exclude that inventory rebuild.  It
            // consequently also includes the small cost of closing those nested
            // instrumentation windows before `complete` runs.
            out.method_call_wall_ns = Some(
                u64::try_from(method_call_start.elapsed().as_nanos())
                    .map_err(|_| "method-call interval overflow")?,
            );
            let delta = match (before, after) {
                (Some(a), Some(b)) => Some(SchedStat {
                    cpu_ns: b.cpu_ns.saturating_sub(a.cpu_ns),
                    run_delay_ns: b.run_delay_ns.saturating_sub(a.run_delay_ns),
                    timeslices: b.timeslices.saturating_sub(a.timeslices),
                }),
                _ => None,
            };
            out.workload_id = Some(w.workload_id);
            out.method_id = Some(m.method_id);
            Ok((
                report,
                delta,
                hw,
                probe_after.since(&probe_before),
                allocator,
            ))
        };
        if let Some(diagnostic) = &job.diagnostic_lifecycle {
            let diagnostic_start = std::time::Instant::now();
            let completed = with_diagnostic_lifecycle(diagnostic, &job, solve_call);
            out.diagnostic_call_wall_ns = Some(
                u64::try_from(diagnostic_start.elapsed().as_nanos())
                    .map_err(|_| "diagnostic-call interval overflow")?,
            );
            let (result, summary) = completed?;
            // Preserve diagnostic writer observations even on a callback error.
            // This field cannot supply a SolveReport or accepted cost.
            out.diagnostic_lifecycle = Some(
                serde_json::to_value(summary).map_err(|error| error.to_string())?,
            );
            result
        } else {
            solve_call()
        }
    })();
    placement.cpu_at_end = current_cpu();
    // Where the solve's memory landed, read after it ran.
    placement.anon_pages_by_node = anon_pages_by_node();
    out.placement = placement;
    match result {
        Ok((r, s, hw, obs, alloc)) => {
            out.report = Some(r);
            out.schedstat_solve = s;
            out.hw_solve = Some(hw);
            out.solve_obs = Some(obs);
            out.allocator = Some(alloc);
        }
        Err(e) => out.error = Some(e),
    }
    out
}

// ── The record ─────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    /// `verified`, `wrong_answer`, `exhausted`, `error`, `timeout` or
    /// `crashed`.  Only `verified` is a result.
    pub status: String,
    pub recovered: Option<String>,
    /// `[recovered]G = Q`, checked by the runner in its own process.
    pub matches_target: Option<bool>,
    pub matches_planted: Option<bool>,
    pub error: Option<String>,
    /// Exit code or signal of the measured process.
    pub exit: Option<String>,
    /// SHA-256 of the child's stderr, kept as `exec/<seq>.stderr` when
    /// nonempty.
    pub stderr_sha256: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cost {
    pub unit: String,
    pub total_gae: Option<f64>,
    /// `total_gae / √r`: the column every row in this repository is read
    /// in, rho at about 1.25 for `A = 1`.
    pub s: Option<f64>,
    /// Some work was counted and not priced; the total is a floor.
    pub lower_bound: bool,
    pub unpriced: Vec<String>,
    pub deterministic: bool,
    pub nondeterminism: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Boundaries {
    /// Automorphisms a generic algorithm may use on this curve.
    pub automorphisms_available: u32,
    pub automorphisms_used: Option<u32>,
    /// `√(π / 2A)` with `A` available: the generic collision floor in `S`
    /// for this curve, which no generic method beats on average.
    pub floor_s: f64,
    pub ratio_to_floor: Option<f64>,
    /// The rule that gave `automorphisms_available` and `floor_s`
    /// ([`FLOOR_RULE_V2`]).  Absent in a record written before the rule was
    /// named: the audit recomputes it under the first rule, `A = 2` on every
    /// prime curve, so a committed record keeps the floor it was written with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_rule: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsolationRecord {
    pub level: Level,
    /// Why the run did not earn each higher level.  Empty at L3.
    pub blockers: Vec<String>,
    pub run_cpu: Option<u32>,
    pub reserved: Vec<u32>,
    pub node: Option<u32>,
    pub placement: SelfPlacement,
    /// Busy time on the run CPU not accounted for by the child, ticks.
    pub foreign_busy_ticks: Option<i64>,
    pub steal_ticks: Option<u64>,
    pub idle_sibling_busy_ticks: Option<u64>,
    pub before: Conditions,
    pub after: Conditions,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub schema: String,
    /// `ECR1h` + 12 hex of this record with `record_id` empty.
    pub record_id: String,
    pub session_id: String,
    /// `<method_id><workload_id>R<n>`: the repository's
    /// `{candidate}W{workload}R{number}` convention.  `n` counts, from 1,
    /// every execution of this (method, workload) in the session in
    /// sequence order — warm-ups and A/A arms included — so a run id is
    /// unique within its session.
    pub run_id: String,
    pub seq: u64,
    pub round: u32,
    pub warmup: bool,
    pub arm: String,
    pub role: String,
    pub method: ResolvedMethod,
    pub workload: Workload,
    pub algorithm_seed: String,
    pub outcome: Outcome,
    pub cost: Cost,
    pub boundaries: Boundaries,
    pub phases: Vec<PhaseRecord>,
    pub counters: BTreeMap<String, u64>,
    pub factor_base: Option<FactorBaseFacts>,
    /// FBM1 measurement identity for this IC run (`docs/ecbench/FACTOR_BASE_PROTOCOL.md` §FBM1).
    /// Absent on pre-FBM1 records and non-IC methods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fbm_id: Option<String>,
    pub detail: Value,
    /// The one-target online window (AGENTS.md "IC measurements").
    #[serde(default)]
    pub online: Option<OnlineWindow>,
    /// The decomposition solver's statistics for an algebraic or SAT
    /// index-calculus run; `null` otherwise.  Informational, outside the
    /// replay comparison.
    #[serde(default)]
    pub solver: Option<SolverStats>,
    /// What was done to the allocator before the solve clock started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocator: Option<AllocatorFacts>,
    /// The field operations behind the group operations
    /// (`SolveReport::field_ops`): modular multiplications, squarings and
    /// inversions over the whole solve, when the method's group counted
    /// them.  Absent means unknown, never zero.  Written only when
    /// counted, so a record written before the block existed is byte for
    /// byte what it was and its seal still checks; the audit compares it
    /// on replay only when both the record and the replay carry it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_ops: Option<FieldOps>,
    pub time: Timing,
    pub isolation: IsolationRecord,
    pub env_class_id: String,
    pub binary_sha256: Option<String>,
    /// The run's work in `bitop/v1` (`ecbench::work`), derived from the
    /// record's own counts.  Written only by runners that price it, so a
    /// record from before the block existed is byte for byte what it was and
    /// its seal still checks; `ecbench work` prices those from their counts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work: Option<crate::cryptanalysis::ecbench::work::WorkBlock>,
}

impl Record {
    /// Seal: compute `record_id` from the exact line the record is
    /// written as, with the id empty, and return that line with the id.
    ///
    /// The seal covers bytes, not a parsed value: `ecbench.record/v1` was
    /// defined with serde_json's former, not-always-correctly-rounded float
    /// parser, so a record parsed and re-serialised can differ in a last digit
    /// from what was written. [`check_line_seal`] therefore checks the raw line
    /// on disk.
    pub fn seal(&mut self) -> String {
        self.record_id = String::new();
        let body = serde_json::to_string(self).expect("a record serialises");
        self.record_id = format!("ECR1h{}", &sha256_hex(body.as_bytes())[..12]);
        serde_json::to_string(self).expect("a record serialises")
    }

    /// A result: verified, measured (not a warm-up).
    pub fn counts(&self) -> bool {
        !self.warmup && self.outcome.status == "verified"
    }
}

/// Whether a raw `records.jsonl` line hashes to the id it carries: the
/// line with its `"record_id":"…"` emptied is what [`Record::seal`]
/// hashed.
pub fn check_line_seal(line: &str, record_id: &str) -> bool {
    let field = format!("\"record_id\":\"{record_id}\"");
    if !line.contains(&field) || !record_id.starts_with("ECR1h") {
        return false;
    }
    let blank = line.replacen(&field, "\"record_id\":\"\"", 1);
    format!("ECR1h{}", &sha256_hex(blank.as_bytes())[..12]) == record_id
}

/// `x` after the serialise-parse round trip a record's floats take, so a
/// recomputed value compares with a parsed one bit for bit.
pub fn json_roundtrip(x: f64) -> f64 {
    legacy_json_float(x).unwrap_or(f64::NAN)
}

/// Reproduce the decimal parser used when `ecbench.record/v1` was defined.
///
/// serde_json formerly accumulated the significand as an integer and then
/// multiplied or divided by a power of ten. Enabling its `float_roundtrip`
/// feature repository-wide makes parsing correctly rounded, but sealed v1
/// records and all bounds derived from them retain the former result. Keep
/// that protocol behavior explicit instead of depending on a crate feature.
pub(crate) fn legacy_json_float(x: f64) -> Option<f64> {
    const POW10: [f64; 309] = [
        1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
        1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29, 1e30, 1e31,
        1e32, 1e33, 1e34, 1e35, 1e36, 1e37, 1e38, 1e39, 1e40, 1e41, 1e42, 1e43, 1e44, 1e45, 1e46,
        1e47, 1e48, 1e49, 1e50, 1e51, 1e52, 1e53, 1e54, 1e55, 1e56, 1e57, 1e58, 1e59, 1e60, 1e61,
        1e62, 1e63, 1e64, 1e65, 1e66, 1e67, 1e68, 1e69, 1e70, 1e71, 1e72, 1e73, 1e74, 1e75, 1e76,
        1e77, 1e78, 1e79, 1e80, 1e81, 1e82, 1e83, 1e84, 1e85, 1e86, 1e87, 1e88, 1e89, 1e90, 1e91,
        1e92, 1e93, 1e94, 1e95, 1e96, 1e97, 1e98, 1e99, 1e100, 1e101, 1e102, 1e103, 1e104, 1e105,
        1e106, 1e107, 1e108, 1e109, 1e110, 1e111, 1e112, 1e113, 1e114, 1e115, 1e116, 1e117, 1e118,
        1e119, 1e120, 1e121, 1e122, 1e123, 1e124, 1e125, 1e126, 1e127, 1e128, 1e129, 1e130, 1e131,
        1e132, 1e133, 1e134, 1e135, 1e136, 1e137, 1e138, 1e139, 1e140, 1e141, 1e142, 1e143, 1e144,
        1e145, 1e146, 1e147, 1e148, 1e149, 1e150, 1e151, 1e152, 1e153, 1e154, 1e155, 1e156, 1e157,
        1e158, 1e159, 1e160, 1e161, 1e162, 1e163, 1e164, 1e165, 1e166, 1e167, 1e168, 1e169, 1e170,
        1e171, 1e172, 1e173, 1e174, 1e175, 1e176, 1e177, 1e178, 1e179, 1e180, 1e181, 1e182, 1e183,
        1e184, 1e185, 1e186, 1e187, 1e188, 1e189, 1e190, 1e191, 1e192, 1e193, 1e194, 1e195, 1e196,
        1e197, 1e198, 1e199, 1e200, 1e201, 1e202, 1e203, 1e204, 1e205, 1e206, 1e207, 1e208, 1e209,
        1e210, 1e211, 1e212, 1e213, 1e214, 1e215, 1e216, 1e217, 1e218, 1e219, 1e220, 1e221, 1e222,
        1e223, 1e224, 1e225, 1e226, 1e227, 1e228, 1e229, 1e230, 1e231, 1e232, 1e233, 1e234, 1e235,
        1e236, 1e237, 1e238, 1e239, 1e240, 1e241, 1e242, 1e243, 1e244, 1e245, 1e246, 1e247, 1e248,
        1e249, 1e250, 1e251, 1e252, 1e253, 1e254, 1e255, 1e256, 1e257, 1e258, 1e259, 1e260, 1e261,
        1e262, 1e263, 1e264, 1e265, 1e266, 1e267, 1e268, 1e269, 1e270, 1e271, 1e272, 1e273, 1e274,
        1e275, 1e276, 1e277, 1e278, 1e279, 1e280, 1e281, 1e282, 1e283, 1e284, 1e285, 1e286, 1e287,
        1e288, 1e289, 1e290, 1e291, 1e292, 1e293, 1e294, 1e295, 1e296, 1e297, 1e298, 1e299, 1e300,
        1e301, 1e302, 1e303, 1e304, 1e305, 1e306, 1e307, 1e308,
    ];

    let encoded = serde_json::to_string(&x).ok()?;
    let (negative, unsigned) = encoded
        .strip_prefix('-')
        .map_or((false, encoded.as_str()), |rest| (true, rest));
    let (mantissa, explicit_exponent) =
        unsigned
            .split_once(['e', 'E'])
            .map_or((unsigned, 0), |(mantissa, exponent)| {
                exponent
                    .parse::<i32>()
                    .map(|value| (mantissa, value))
                    .unwrap_or((mantissa, i32::MAX))
            });
    if explicit_exponent == i32::MAX {
        return None;
    }

    let mut significand = 0u64;
    let mut fractional_digits = 0i32;
    let mut after_decimal = false;
    for byte in mantissa.bytes() {
        match byte {
            b'.' if !after_decimal => after_decimal = true,
            b'0'..=b'9' => {
                significand = significand
                    .checked_mul(10)?
                    .checked_add(u64::from(byte - b'0'))?;
                if after_decimal {
                    fractional_digits = fractional_digits.checked_add(1)?;
                }
            }
            _ => return None,
        }
    }
    let mut exponent = explicit_exponent.checked_sub(fractional_digits)?;
    let mut value = significand as f64;
    // The former parser: one multiplication or division by the tabulated
    // power when the exponent is within the table, and below it repeated
    // division by 1e308 until it is; a positive exponent past the table
    // was an error, as a finite f64 never prints one.
    loop {
        match POW10.get(usize::try_from(exponent.unsigned_abs()).ok()?) {
            Some(&pow) => {
                if exponent >= 0 {
                    value *= pow;
                    if value.is_infinite() {
                        return None;
                    }
                } else {
                    value /= pow;
                }
                break;
            }
            None => {
                if value == 0.0 {
                    break;
                }
                if exponent >= 0 {
                    return None;
                }
                value /= 1e308;
                exponent += 308;
            }
        }
    }
    Some(if negative { -value } else { value })
}

fn normalize_legacy_json_floats(value: &mut Value) -> Result<(), String> {
    match value {
        Value::Array(values) => {
            for value in values {
                normalize_legacy_json_floats(value)?;
            }
        }
        Value::Object(values) => {
            for value in values.values_mut() {
                normalize_legacy_json_floats(value)?;
            }
        }
        Value::Number(number)
            if crate::cryptanalysis::ecbench::canonical::is_nonfinite_lexeme(number) =>
        {
            // `1e400`: neither an integer nor (to serde_json) a float, so
            // it would otherwise pass through to be read as infinity.
            return Err(format!("record number {number} is not a finite f64"));
        }
        Value::Number(number) if number.is_f64() => {
            let current = number
                .as_f64()
                .ok_or_else(|| format!("record float {number} does not fit f64"))?;
            let legacy = legacy_json_float(current).ok_or_else(|| {
                format!("record float {number} is outside the v1 decimal-parser domain")
            })?;
            let number = serde_json::Number::from_f64(legacy)
                .ok_or_else(|| format!("legacy parse of record float {number} is not finite"))?;
            *value = Value::Number(number);
        }
        _ => {}
    }
    Ok(())
}

/// Deserialize a sealed `ecbench.record/v1` JSON value with the decimal
/// semantics under which the schema and its derived bounds were frozen.
pub(crate) fn from_str_legacy_floats<T: DeserializeOwned>(text: &str) -> Result<T, String> {
    // `Value` keeps the last of two equal keys; refuse them like the exact
    // reader does, so a line cannot carry a figure the typed record hides.
    crate::cryptanalysis::ecbench::canonical::reject_duplicate_keys(text)?;
    let mut value: Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    normalize_legacy_json_floats(&mut value)?;
    serde_json::from_value(value).map_err(|error| error.to_string())
}

/// `√(π / 2A)`.
pub fn floor_s(automorphisms: u32) -> f64 {
    (std::f64::consts::PI / (2.0 * automorphisms.max(1) as f64)).sqrt()
}

/// The `A` a record's floor must have been computed from, given the curve
/// facts the audit rebuilds from the spec: the first rule for a record with no
/// `floor_rule`, the facts' own count under [`FLOOR_RULE_V2`], and `None`
/// (unknown, so a failure) for any other rule name.
pub fn expected_automorphisms(floor_rule: Option<&str>, curve: &CurveFacts) -> Option<u32> {
    match floor_rule {
        None => Some(legacy_automorphisms(curve)),
        Some(FLOOR_RULE_V2) => Some(curve.automorphisms_available),
        Some(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct LegacyFloatFixture {
        gae: f64,
        nested: Vec<f64>,
        integer: u64,
    }

    /// The v1 parser accepted every finite decimal: a power of ten past
    /// the fast table was reached by repeated division by 1e308, never an
    /// error.  A record holding `2^-24`, or a subnormal, must stay readable.
    #[test]
    fn legacy_float_parser_covers_the_whole_finite_domain() {
        for x in [
            2f64.powi(-24),
            5.960464477539063e-8,
            1e-300,
            2.2250738585072014e-308,
            5e-324,
            1e300,
            f64::MAX,
            -1e-200,
            0.0,
        ] {
            let y = legacy_json_float(x).unwrap_or_else(|| panic!("{x:e} rejected"));
            assert!(
                ((y - x) / x.abs().max(f64::MIN_POSITIVE)).abs() < 1e-14 || y == x,
                "{x:e} -> {y:e}"
            );
        }
        let text = r#"{"gae":5.960464477539063e-8,"nested":[5e-324,1e300],"integer":1}"#;
        let legacy: LegacyFloatFixture = from_str_legacy_floats(text).unwrap();
        assert!(legacy.gae > 5.9e-8 && legacy.gae < 6e-8);
        assert!(legacy.nested[0] > 0.0 && legacy.nested[1] == 1e300);
    }

    #[test]
    fn legacy_record_deserializer_reproduces_v1_decimal_semantics() {
        let text = r#"{"gae":3934.0137668849675,"nested":[0.37222665730402227],"integer":17}"#;
        let legacy: LegacyFloatFixture = from_str_legacy_floats(text).unwrap();
        let current: LegacyFloatFixture = serde_json::from_str(text).unwrap();

        // Freeze the v1 parser's exact binary results independently of the
        // serde_json feature set used by the current build.
        assert_eq!(legacy.gae.to_bits(), 4_660_869_402_834_764_135);
        assert_eq!(legacy.nested[0].to_bits(), 4_600_377_059_258_432_959);
        assert_eq!(
            legacy.gae.to_bits(),
            legacy_json_float(current.gae).unwrap().to_bits()
        );
        assert_eq!(
            legacy.nested[0].to_bits(),
            legacy_json_float(current.nested[0]).unwrap().to_bits()
        );
        // A newer serde_json parser may already yield the historical value.
        // Compatibility is the archived bit pattern, independent of whether
        // the current parser happens to differ on this decimal fixture.
        assert_eq!(legacy.gae.to_bits(), 3934.013766884967_f64.to_bits());
        // serde_json's v1 integer-significand path parses this sealed decimal
        // one ULP below its correctly rounded value (see the archived audit).
        assert_eq!(legacy.nested[0].to_bits(), 4_600_377_059_258_432_959);
        // Some serde_json feature sets already use the legacy decimal
        // parser. Pin the historical bit patterns instead of assuming the
        // current parser differs from them.
        assert_eq!(legacy.gae.to_bits(), 0x40aebc070c740167);
        assert_eq!(legacy.nested[0].to_bits(), 0x3fd7d28fc1f47dbf);
        // serde_json >= 1.0.15x parses decimals exactly under
        // arbitrary_precision, so the default path now agrees bit for bit
        // with the legacy reproducer instead of losing the tail.
        assert_eq!(legacy.gae.to_bits(), current.gae.to_bits());
        assert_eq!(legacy.nested[0].to_bits(), current.nested[0].to_bits());
        assert_eq!(legacy.integer, 17);
    }

    #[test]
    fn legacy_record_roundtrip_is_not_a_numeric_tolerance() {
        let recorded: f64 = 3934.013766884967;
        let replayed: f64 = 3934.0137668849675;
        assert_ne!(recorded.to_bits(), replayed.to_bits());
        assert_eq!(
            json_roundtrip(replayed).to_bits(),
            json_roundtrip(recorded).to_bits()
        );

        let two_ulp_mutation = f64::from_bits(recorded.to_bits() + 2);
        assert_ne!(
            json_roundtrip(two_ulp_mutation).to_bits(),
            json_roundtrip(recorded).to_bits()
        );
    }
}

#[cfg(test)]
mod grade_tests {
    use super::*;

    /// The frozen version-2 rules reproduce every committed version-2
    /// grade, which is what an audit of those sessions relies on.
    #[test]
    fn frozen_grade_v2_regrades_a_committed_session() {
        use crate::cryptanalysis::ecbench::runner::{read_records, read_session};
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/ecbench_numa_vm_20261002/sessions/v2-auto");
        if !dir.join("session.json").exists() {
            return; // a sparse checkout without the evidence
        }
        let session = read_session(&dir).unwrap();
        assert_eq!(session.grading_version, 2);
        let host: HostCapsule =
            serde_json::from_slice(&std::fs::read(dir.join("host.json")).unwrap()).unwrap();
        let records = read_records(&dir).unwrap();
        assert!(!records.is_empty());
        for r in &records {
            let g = grade_for_version(
                session.grading_version,
                &GradeInput {
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
                },
            )
            .unwrap();
            assert_eq!(g.level, r.isolation.level, "seq {}", r.seq);
            assert_eq!(g.blockers, r.isolation.blockers, "seq {}", r.seq);
        }
    }

    /// A failed replay names the accounting version, and only when it can
    /// matter: not when the session was made under the version charged now.
    #[test]
    fn f4_accounting_note_names_the_version_only_when_it_differs() {
        use crate::cryptanalysis::inherited_f4::INHERITED_F4_ACCOUNTING_VERSION as NOW;
        assert_eq!(
            current_accounting_versions().get(F4_ACCOUNTING_KEY),
            Some(&NOW)
        );
        let (before, after) = (F4_EAGER_EVERYWHERE_UNIX_MS - 1, F4_EAGER_EVERYWHERE_UNIX_MS);
        // Same version: the accounting is not the reason.
        assert_eq!(
            f4_accounting_note(&current_accounting_versions(), before, false),
            None
        );
        // A different recorded version is named, whatever the host.
        let older = BTreeMap::from([(F4_ACCOUNTING_KEY.to_string(), NOW + 1)]);
        let note = f4_accounting_note(&older, after, true).expect("a different version is named");
        assert!(note.contains(&format!("v{}", NOW + 1)) && note.contains(&format!("v{NOW}")));
        // No version recorded: named only when #2043 can apply, that is before
        // it and on a host without BMI2.
        let none = BTreeMap::new();
        let note = f4_accounting_note(&none, before, false).expect("pre-#2043 without BMI2");
        assert!(note.contains("#2043") && note.contains("without BMI2"));
        assert_eq!(
            f4_accounting_note(&none, before, true),
            None,
            "BMI2 hosts were eager"
        );
        assert_eq!(f4_accounting_note(&none, after, false), None, "after #2043");
    }

    /// A session written before the field existed reads with no versions and
    /// is written back without the key, so it is byte for byte what it was.
    #[test]
    fn a_committed_session_without_versions_round_trips_without_the_key() {
        use crate::cryptanalysis::ecbench::runner::read_session;
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("research/ecbench_rootfind_engine_20261012/sessions/claim-n13");
        if !dir.join("session.json").exists() {
            return; // a sparse checkout without the evidence
        }
        let session = read_session(&dir).unwrap();
        assert!(session.accounting_versions.is_empty());
        let text = serde_json::to_string(&session).unwrap();
        assert!(!text.contains("accounting_versions"));
    }
}

#[cfg(test)]
mod allocator_tests {
    use super::*;

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    fn churn(chunks: usize) {
        // Leave a great many freed small chunks behind, as a workload's
        // set-up does: past the per-size tcache they land in fastbins.
        let mut live = Vec::with_capacity(chunks);
        for i in 0..chunks {
            // SAFETY: plain malloc of a small size, freed below.
            live.push(unsafe { libc::malloc(16 + 16 * (i % 7)) });
        }
        for p in live {
            // SAFETY: each pointer came from malloc above and is freed once.
            unsafe { libc::free(p) };
        }
    }

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    fn first_large_allocation() -> std::time::Duration {
        let t = std::time::Instant::now();
        // SAFETY: a 4 KiB malloc, freed at once: a large-bin request, which
        // makes glibc consolidate the fastbins before serving it.
        unsafe {
            let p = libc::malloc(4096);
            std::hint::black_box(p);
            libc::free(p);
        }
        t.elapsed()
    }

    /// The confounder of DEEP_DIVE §1.1: freed set-up garbage makes the
    /// first sizeable allocation of the solve slow.  `prepare_allocator`
    /// pays it before the clock starts.
    #[test]
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    fn prepare_allocator_moves_the_consolidation_out_of_the_solve() {
        churn(600_000);
        let cold = first_large_allocation();
        churn(600_000);
        let facts = prepare_allocator();
        let warm = first_large_allocation();
        assert!(facts.pre_solve_trim, "{facts:?}");
        assert!(facts.name.starts_with("glibc "), "{facts:?}");
        assert!(facts.trim_result.is_some());
        if cold < std::time::Duration::from_micros(500) {
            // The environment turned fastbins off (GLIBC_TUNABLES): there is
            // no confounder to remove here.
            eprintln!("fastbins disabled ({cold:?}); skipping the timing half");
            return;
        }
        assert!(
            warm * 5 < cold,
            "first large allocation after prepare took {warm:?}, without it {cold:?}"
        );
    }

    fn child_input(curve: CurveSpec, method_id: &str, algorithm_seed: u64) -> ChildInput {
        // Identities are checked by the child: take them from a build, and
        // hand it the resolved curve as a session does.
        let (w, inst) = Workload::build(&curve, 7, 0, TargetKind::default()).unwrap();
        let method = MethodSpec {
            id: method_id.into(),
            params: Default::default(),
        };
        let m = resolve(&method).unwrap();
        ChildInput {
            schema: CHILD_INPUT_SCHEMA.into(),
            diagnostics: None,
            diagnostic_lifecycle: None,
            curve: w.curve_spec.for_child(&inst),
            target_seed: 7,
            target_index: 0,
            target_kind: Default::default(),
            expected_workload_id: w.workload_id,
            method,
            expected_method_id: m.method_id,
            algorithm_seed,
        }
    }

    #[test]
    fn the_child_prepares_the_allocator_before_the_solve_and_says_so() {
        let curve: CurveSpec = serde_json::from_str(
            r#"{"kind":"prime_explicit","p":113,"a":19,"b":2,"group_order":112,"r":7,"gx":52,"gy":110}"#,
        )
        .unwrap();
        let input = child_input(curve, "bsgs.textbook", 1);
        let out = child_main(&serde_json::to_string(&input).unwrap());
        assert!(out.error.is_none(), "{:?}", out.error);
        assert!(
            out.method_call_wall_ns.expect("shared cold clock")
                >= out.report.as_ref().unwrap().solve_wall_ns
        );
        let encoded = serde_json::to_value(&out).unwrap();
        let mut legacy = encoded.clone();
        legacy
            .as_object_mut()
            .unwrap()
            .remove("method_call_wall_ns");
        let decoded: ChildOutput = serde_json::from_value(legacy).unwrap();
        assert_eq!(decoded.method_call_wall_ns, None);
        assert_eq!(decoded.report, out.report);
        let rejected = child_main("invalid JSON");
        assert!(rejected.error.is_some());
        assert_eq!(rejected.method_call_wall_ns, None);
        let facts = out.allocator.expect("the child records the allocator");
        assert!(!facts.name.is_empty());
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        assert!(facts.pre_solve_trim);
        let obs = out.solve_obs.expect("the child records its solve window");
        #[cfg(target_os = "linux")]
        {
            assert!(obs.threads_before.unwrap() >= 1);
            assert!(obs.involuntary_switches.is_some());
            assert!(obs.cpu.is_some());
        }
        let _ = obs;
    }

    /// With the clock free of set-up garbage, a solve's wall time follows
    /// its work: a 22-bit curve costs about 6 times a 14-bit curve's
    /// group operations, and its solve must take longer.  (Before the
    /// pre-solve trim the larger solve took *less* time: both were
    /// dominated by how much garbage the set-up left, 6-23 ms against a
    /// solve of 0.1-0.3 ms.)  A fixed cost of about 100 µs per solve
    /// (first-touch page faults, table set-up) keeps the ratio below the
    /// work ratio, so the bound is generous.
    #[test]
    fn solve_wall_scales_with_work_once_the_allocator_is_clean() {
        // The best of several runs of each, over the same seeds, so that
        // noise on a shared machine cannot fake or hide the scaling.
        let best = |bits: u32| {
            let curve: CurveSpec = serde_json::from_str(&format!(
                r#"{{"kind":"prime_search","bits":{bits},"seed":59297}}"#
            ))
            .unwrap();
            let template = child_input(curve, "rho.negation", 0);
            (0..6)
                .map(|s| {
                    let input = ChildInput {
                        algorithm_seed: 100 + s,
                        ..template.clone()
                    };
                    let out = child_main(&serde_json::to_string(&input).unwrap());
                    let r = out.report.expect("solved");
                    (r.solve_wall_ns, r.total_gae)
                })
                .fold((u64::MAX, 0.0f64), |(w, g), (w2, g2)| {
                    (w.min(w2), g + g2 / 6.0)
                })
        };
        let (w_small, g_small) = best(14);
        let (w_big, g_big) = best(22);
        let gae = g_big / g_small;
        let wall = w_big as f64 / w_small as f64;
        assert!(gae > 4.0, "work ratio {gae}");
        assert!(
            wall > 1.3 && wall > gae / 8.0,
            "wall ratio {wall:.2} (small {w_small} ns, big {w_big} ns) does not follow the work ratio {gae:.2}"
        );
    }
}

#[cfg(test)]
mod diagnostic_context_tests {
    use super::*;
    #[test]
    fn explicit_context_rejected_before_workload_build() {
        let input = serde_json::json!({"schema":CHILD_INPUT_SCHEMA,
            "curve":{"kind":"prime_explicit","p":0,"a":0,"b":0,"group_order":0,"r":0,"gx":0,"gy":0},
            "target_seed":0,"target_index":0,"expected_workload_id":"invalid-on-purpose",
            "method":{"id":"invalid-on-purpose","params":{}},"expected_method_id":"invalid-on-purpose","algorithm_seed":0});
        let old: ChildInput = serde_json::from_value(input.clone()).unwrap();
        assert!(old.diagnostics.is_none());
        assert!(serde_json::to_value(&old)
            .unwrap()
            .get("diagnostics")
            .is_none());
        let mut with_context = input;
        with_context["diagnostics"] = serde_json::json!({"schema":"ecbench.diagnostic_context/v1","relative_leaf":"leaf","identity":{"session":"fixture","execution":0,"seed":0,"attempt":0,"workload_sha256":"a".repeat(64),"binary_sha256":"b".repeat(64),"protocol_sha256":"c".repeat(64),"source_sha256":"d".repeat(64),"method_config_sha256":"e".repeat(64)}});
        let result = child_main(&serde_json::to_string(&with_context).unwrap());
        assert_eq!(
            result.error.as_deref(),
            Some("diagnostic producer is not enabled or admitted")
        );
        assert!(result.report.is_none());
        assert!(result.workload_id.is_none());
    }
}

#[cfg(all(test, unix))]
mod diagnostic_child_v2_tests {
    use super::super::diagnostic_launch::{
        resolved_method_binding, ChildEnvelope, DiagnosticLaunch, CHILD_SCHEMA, LAUNCH_SCHEMA,
        PROFILE_VERSION,
    };
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);

    struct Root(std::path::PathBuf);
    impl Root {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "ecbench-child-v2-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
            Self(std::fs::canonicalize(path).unwrap())
        }
    }
    impl Drop for Root {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn fixture_method(candidate: bool) -> MethodSpec {
        let method_name = if candidate {
            "ic.three_leads_genus2_pencil_symbolic_v2"
        } else {
            "rho.three_leads_negation_v1"
        };
        let params = if candidate {
            BTreeMap::from([
                ("small_denominator".into(), "2".into()),
                ("matrix_coefficient_cap".into(), "1000000".into()),
            ])
        } else {
            BTreeMap::from([("trial_cap".into(), "1".into())])
        };
        MethodSpec {
            id: method_name.into(),
            params,
        }
    }
    fn fixture(root: &Root, candidate: bool) -> ChildInput {
        fixture_arm(root, if candidate { "pencil" } else { "rho" })
    }
    fn fixture_arm(root: &Root, arm_name: &str) -> ChildInput {
        assert!(matches!(arm_name, "pencil" | "rho" | "rho_AA"));
        let method = fixture_method(arm_name == "pencil");
        let resolved = resolve(&method).unwrap();
        let full_hash = resolved_method_binding(&resolved).unwrap();
        let profile = serde_json::json!({"version":PROFILE_VERSION,"sha256":"a".repeat(64)});
        let arms: Vec<_> = [
            (
                "pencil",
                "candidate",
                "ic.three_leads_genus2_pencil_symbolic_v2",
            ),
            ("rho", "reference", "rho.three_leads_negation_v1"),
            ("rho_AA", "control", "rho.three_leads_negation_v1"),
        ]
        .iter()
        .map(|(name, role, id)| {
            let arm_method = fixture_method(*name == "pencil");
            let arm_resolved = resolve(&arm_method).unwrap();
            assert_eq!(arm_resolved.id, *id);
            let arm_hash = resolved_method_binding(&arm_resolved).unwrap();
            serde_json::json!({"name":name,"role":role,"method":id,
                    "method_config_sha256":arm_hash,"profile":profile})
        })
        .collect();
        let launch = DiagnosticLaunch::from_bytes(
            &serde_json::to_vec(&serde_json::json!({
                "schema":LAUNCH_SCHEMA,"expected_spec_sha256":"b".repeat(64),
                "binary_sha256":"c".repeat(64),"protocol_sha256":"d".repeat(64),
                "source_receipt":{"scope":"external_manifest_binding_only",
                    "receipt_sha256":"e".repeat(64),"source_manifest_sha256":"f".repeat(64)},
                "scientific_run_id":"software-fixture","attempt":0,"profile":profile,"arms":arms
            }))
            .unwrap(),
        )
        .unwrap();
        let doc = launch.document().unwrap();
        let prefix = super::super::diagnostics::Context {
            schema: "ecbench.diagnostic_context/v1".into(),
            relative_leaf: "execution_0".into(),
            identity: super::super::diagnostics::Identity {
                session: "software-fixture".into(),
                execution: 0,
                workload_sha256: "1".repeat(64),
                binary_sha256: doc.binary_sha256.clone(),
                protocol_sha256: doc.protocol_sha256.clone(),
                seed: 17,
                attempt: 0,
                source_sha256: doc.source_receipt.source_manifest_sha256.clone(),
                method_config_sha256: full_hash,
            },
        };
        let envelope = ChildEnvelope {
            schema: CHILD_SCHEMA.into(),
            launch_sha256: launch.raw_sha256().into(),
            scientific_run_id: doc.scientific_run_id,
            profile: doc.profile,
            source_receipt: doc.source_receipt,
            arm: doc
                .arms
                .iter()
                .find(|arm| arm.name == arm_name)
                .unwrap()
                .clone(),
            prefix,
        };
        // Deliberately invalid curve; no test is permitted to build it.
        serde_json::from_value(serde_json::json!({"schema":CHILD_INPUT_SCHEMA,
            "curve":{"kind":"prime_explicit","p":0,"a":0,"b":0,"group_order":0,"r":0,"gx":0,"gy":0},
            "target_seed":0,"target_index":0,"expected_workload_id":"never-build",
            "method":method,"expected_method_id":resolved.method_id,"algorithm_seed":17,
            "diagnostic_lifecycle":DiagnosticChild{launch,envelope,output_root:root.0.clone()}
        }))
        .unwrap()
    }
    #[test]
    fn child_v2_full_workload_serialization_binding_without_build() {
        let root = Root::new();
        let mut job = fixture(&root, false);
        let mut workload = Workload {
            workload_id: "serialization-fixture".into(),
            workload_sha256: "2".repeat(64),
            curve_spec: job.curve.clone(),
            curve: CurveFacts {
                slug: "fixture".into(),
                icv1: "fixture".into(),
                family: "prime".into(),
                construction: "not-built".into(),
                field_degree: None,
                field_bits: None,
                group_order: 0,
                r: 0,
                cofactor: 0,
                generator: ["0".into(), "0".into()],
                automorphisms_available: 0,
                registered: false,
                ec1: None,
                curve_uid: None,
            },
            target_law: "serialization-only".into(),
            target_seed: 0,
            target_index: 0,
            target: ["0".into(), "0".into()],
            planted: None,
            target_counter: None,
        };
        let bytes = serde_json::to_vec(&serde_json::to_value(&workload).unwrap()).unwrap();
        let diagnostic = job.diagnostic_lifecycle.as_mut().unwrap();
        diagnostic.envelope.prefix.identity.workload_sha256 = sha256_hex(&bytes);
        assert!(diagnostic.validate_workload(&workload).is_ok());
        assert_ne!(
            diagnostic.envelope.prefix.identity.workload_sha256,
            workload.workload_sha256
        );
        workload.target_seed += 1;
        assert!(diagnostic.validate_workload(&workload).is_err());
    }

    #[test]
    fn child_v2_roundtrip_and_running_binary_boundary() {
        let root = Root::new();
        let job = fixture(&root, false);
        job.diagnostic_lifecycle
            .as_ref()
            .unwrap()
            .validate(&job)
            .unwrap();
        let text = serde_json::to_string(&job).unwrap();
        let again: ChildInput = serde_json::from_str(&text).unwrap();
        assert_eq!(again, job);
        let out = child_main(&text);
        assert_eq!(
            out.error.as_deref(),
            Some("diagnostic child running binary binding mismatch")
        );
        assert!(out.report.is_none());
        assert!(out.workload_id.is_none());
        assert!(std::fs::read_dir(&root.0).unwrap().next().is_none());
        let mut old = job;
        old.diagnostic_lifecycle = None;
        assert!(serde_json::to_value(old)
            .unwrap()
            .get("diagnostic_lifecycle")
            .is_none());
    }
    fn bind_fixture_running_binary(job: &mut ChildInput) {
        let binary = diagnostic_running_binary_sha256().unwrap();
        let diagnostic = job.diagnostic_lifecycle.as_mut().unwrap();
        let mut doc = serde_json::to_value(diagnostic.launch.document().unwrap()).unwrap();
        doc["binary_sha256"] = serde_json::json!(binary);
        diagnostic.launch =
            DiagnosticLaunch::from_bytes(&serde_json::to_vec(&doc).unwrap()).unwrap();
        diagnostic.envelope.launch_sha256 = diagnostic.launch.raw_sha256().into();
        diagnostic.envelope.prefix.identity.binary_sha256 = binary;
    }

    #[test]
    fn child_v2_all_arms_validate_binary_then_reject_wrong_workload_before_callback() {
        // Known tiny public curve; this fixture builds a workload but never calls
        // solve_measured, complete, rho, or the pencil collector.
        let curve: CurveSpec = serde_json::from_str(
            r#"{"kind":"prime_explicit","p":113,"a":19,"b":2,"group_order":112,"r":7,"gx":52,"gy":110}"#,
        ).unwrap();
        let (workload, instance) = Workload::build(&curve, 7, 0, TargetKind::default()).unwrap();
        for arm in ["pencil", "rho", "rho_AA"] {
            let root = Root::new();
            let mut job = fixture_arm(&root, arm);
            bind_fixture_running_binary(&mut job);
            job.curve = workload.curve_spec.for_child(&instance);
            job.target_seed = 7;
            job.expected_workload_id = workload.workload_id.clone();
            // Correct normal workload ID; deliberately wrong full serialized
            // workload digest forces the diagnostic-specific boundary.
            let text = serde_json::to_string(&job).unwrap();
            let out = child_main(&text);
            assert_eq!(
                out.error.as_deref(),
                Some("diagnostic child rebuilt workload binding mismatch")
            );
            assert!(out.report.is_none());
            assert!(out.diagnostic_lifecycle.is_none());
            assert!(out.method_call_wall_ns.is_none());
            assert!(out.diagnostic_call_wall_ns.is_none());
            assert!(std::fs::read_dir(&root.0).unwrap().next().is_none());
        }
    }

    #[test]
    fn child_v2_rejects_mixed_seed_method_and_insecure_path() {
        let root = Root::new();
        let job = fixture(&root, false);
        let mut bad = job.clone();
        bad.algorithm_seed += 1;
        assert!(bad
            .diagnostic_lifecycle
            .as_ref()
            .unwrap()
            .validate(&bad)
            .is_err());
        let mut bad = job.clone();
        bad.expected_method_id = "wrong".into();
        assert!(bad
            .diagnostic_lifecycle
            .as_ref()
            .unwrap()
            .validate(&bad)
            .is_err());
        let mut bad = job.clone();
        bad.diagnostics = Some(
            bad.diagnostic_lifecycle
                .as_ref()
                .unwrap()
                .envelope
                .prefix
                .clone(),
        );
        let out = child_main(&serde_json::to_string(&bad).unwrap());
        assert_eq!(
            out.error.as_deref(),
            Some("mixed v1/v2 diagnostic contexts")
        );
        assert!(out.report.is_none());
        let mut bad = job.clone();
        bad.diagnostic_lifecycle.as_mut().unwrap().output_root = "relative".into();
        assert!(bad
            .diagnostic_lifecycle
            .as_ref()
            .unwrap()
            .validate(&bad)
            .is_err());
        std::fs::set_permissions(&root.0, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert!(job
            .diagnostic_lifecycle
            .as_ref()
            .unwrap()
            .validate(&job)
            .is_err());
    }
    #[test]
    fn child_v2_rejects_symlink_output_root() {
        let root = Root::new();
        let job = fixture(&root, false);
        let link = root.0.join("link");
        std::os::unix::fs::symlink(&root.0, &link).unwrap();
        let mut bad = job;
        bad.diagnostic_lifecycle.as_mut().unwrap().output_root = link;
        assert!(bad
            .diagnostic_lifecycle
            .as_ref()
            .unwrap()
            .validate(&bad)
            .is_err());
    }
    #[test]
    fn child_v2_fake_callback_all_arms_and_candidate_only_prefix() {
        for arm_name in ["pencil", "rho", "rho_AA"] {
            let root = Root::new();
            let job = fixture_arm(&root, arm_name);
            let diagnostic = job.diagnostic_lifecycle.as_ref().unwrap();
            assert_eq!(diagnostic.envelope.arm.name, arm_name);
            for arm in diagnostic.launch.document().unwrap().arms {
                let method = resolve(&fixture_method(arm.name == "pencil")).unwrap();
                assert_eq!(
                    arm.method_config_sha256,
                    resolved_method_binding(&method).unwrap()
                );
            }
            let (value, summary) =
                with_diagnostic_lifecycle(diagnostic, &job, || Ok(23u64)).unwrap();
            assert_eq!(value.unwrap(), 23);
            assert!(summary.lifecycle.diagnostic_only);
            assert_eq!(summary.prefix_io.is_some(), arm_name == "pencil");
            assert!(diagnostic
                .lifecycle_directory()
                .join("finished.json")
                .exists());
            // Exclusive writer must never overwrite a prior execution.
            assert!(with_diagnostic_lifecycle(diagnostic, &job, || Ok(24u64)).is_err());
        }
    }
    #[test]
    fn child_v2_fake_callback_error_and_panic_do_not_run_algorithm() {
        let root = Root::new();
        let job = fixture(&root, false);
        let d = job.diagnostic_lifecycle.as_ref().unwrap();
        let (value, summary) =
            with_diagnostic_lifecycle::<()>(d, &job, || Err("fixture-error".into())).unwrap();
        assert_eq!(value.unwrap_err(), "fixture-error");
        assert_eq!(
            summary.lifecycle.requested_finish,
            super::super::diagnostic_runtime::FinishStatus::Error
        );
        let root = Root::new();
        let job = fixture(&root, true);
        let d = job.diagnostic_lifecycle.as_ref().unwrap();
        let panic = std::panic::catch_unwind(|| {
            let _ = with_diagnostic_lifecycle::<()>(d, &job, || panic!("fixture-panic"));
        });
        assert!(panic.is_err());
        assert!(!d.lifecycle_directory().join("finished.json").exists());
        let root = Root::new();
        let job = fixture(&root, true);
        let d = job.diagnostic_lifecycle.as_ref().unwrap();
        assert!(with_diagnostic_lifecycle(d, &job, || Ok(())).is_ok());
    }
}
