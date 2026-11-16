//! External source proposal: pure policy logic, never a hardware collector.
//!
//! A future strict receipt verifier must derive Facts from authenticated inputs,
//! syscall replay, platform admission and task-lifecycle evidence. These public
//! fields are test inputs, not a JSON wire format or permission to trust a worker's
//! claims. No existing dataset can gain eligibility by setting these booleans.
//! Integration must reuse isolation::checked and preserve fixed v2 phase policy.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    RetiredUserInstructions,
    CpuCycles,
    ModeledWorkReservations,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    CallingThreadPhase,
    WorkerBodyCallingThread,
    WorkerExecToExitAllTasks,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskCoverage {
    Unknown,
    BoundaryTaskSnapshots,
    ProcessGroupAbsentAtEnd,
    VerifiedCompleteLineageAndExit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Objective {
    PdpDecision,
    ScalarRecovery,
    FormulaSatisfiability,
    ConstructionOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    VerifiedPdpWitness,
    VerifiedFiniteDomainExhaustion,
    VerifiedIndependentScalarRecovery,
    VerifiedSatisfyingModel,
    VerifiedUnsatisfiabilityCertificate,
    ConstructedKnownAnswer,
    Censored,
    Error,
    Unsupported,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawTuple {
    pub value: u64,
    pub time_enabled_ns: u64,
    pub time_running_ns: u64,
    pub event_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TupleStatus {
    Missing,
    Invalid,
    NotRunning,
    Unmultiplexed,
    MultiplexedEstimate,
    EstimateOverflow,
}

/// Facts are already verified against the exact versioned receipt. The proposal
/// deliberately has no deserializer; a worker cannot submit a self-attestation.
#[derive(Clone, Copy, Debug)]
pub struct Facts {
    pub unit: Unit,
    pub raw: Option<RawTuple>,
    pub expected_event_id: u64,
    pub native_event_request_verified: bool,
    pub selected_lifecycle_replayed: bool,
    pub owner_context_matched: bool,
    pub cleanup_succeeded: bool,
    pub fallback_history_clean: bool,
    pub scope: Scope,
    pub task_coverage: TaskCoverage,
    pub bound_identity_verified: bool,
    pub immutable_receipts_verified: bool,
    pub terminal_receipt_verified: bool,
    pub reset_before_target_exec: bool,
    pub armed_before_target_exec: bool,
    pub actual_target_exec_verified: bool,
    /// Complete verified lifecycle covers every included task's entire lifetime
    /// from target exec to its exit, without an early disable, reset or gap.
    /// Endpoint timestamps or enabled==running alone cannot establish this.
    pub continuous_exec_to_exit_coverage_verified: bool,
    pub read_after_all_tasks_exited: bool,
    pub external_work_absent_or_covered: bool,
    pub measurement_policy_admitted: bool,
    pub platform_event_semantics_admitted: bool,
    pub independent_receipt_replay: bool,
    pub objective: Objective,
    pub outcome: Outcome,
    pub exact_outcome_and_domain_replayed: bool,
    pub worker_exit_success: bool,
    pub original_deadline_respected: bool,
    pub resource_censoring_absent: bool,
    pub interrupted_attempt: bool,
}

pub const WRONG_UNIT: u64 = 1 << 0;
pub const TUPLE_UNUSABLE: u64 = 1 << 1;
pub const LIFECYCLE_UNVERIFIED: u64 = 1 << 2;
pub const MULTIPLEXED_OR_OVERFLOW: u64 = 1 << 3;
pub const WRONG_SCOPE: u64 = 1 << 4;
pub const TASK_COVERAGE_UNVERIFIED: u64 = 1 << 5;
pub const IDENTITY_UNVERIFIED: u64 = 1 << 6;
pub const RECEIPTS_UNVERIFIED: u64 = 1 << 7;
pub const EXEC_BOUNDARY_UNVERIFIED: u64 = 1 << 8;
pub const TERMINAL_BOUNDARY_UNVERIFIED: u64 = 1 << 9;
pub const EXTERNAL_WORK_UNCOVERED: u64 = 1 << 10;
pub const POLICY_UNADMITTED: u64 = 1 << 11;
pub const PLATFORM_UNADMITTED: u64 = 1 << 12;
pub const INDEPENDENT_REPLAY_MISSING: u64 = 1 << 13;
pub const OBJECTIVE_NOT_COMPLETED: u64 = 1 << 14;
pub const OUTCOME_REPLAY_MISSING: u64 = 1 << 15;
pub const TERMINAL_CENSORED_OR_FAILED: u64 = 1 << 16;
pub const CONTINUOUS_INTERVAL_UNVERIFIED: u64 = 1 << 17;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decision {
    pub tuple_status: TupleStatus,
    /// Never use this as a completed-solve label; it preserves a raw observation.
    pub diagnostic_raw: Option<u64>,
    /// Floor(raw * enabled / running); never promoted to an exact label.
    pub diagnostic_scaled_estimate: Option<u64>,
    /// Unmultiplexed within declared whole-attempt user-space scope, conditional on the
    /// validated hardware/lifecycle facts. Failed attempts may retain this cost.
    pub whole_attempt_user_instructions: Option<u64>,
    /// Defined as worker exec-to-exit cost for the declared objective, including
    /// worker construction, replay, serialization and measurement overhead.
    /// Excludes kernel/hypervisor, supervisor, upload and external validation.
    pub instruction_count_to_solve: Option<u64>,
    pub training_cost_eligible: bool,
    pub rejection_bits: u64,
}

fn assess(raw: Option<RawTuple>, expected_id: u64) -> (TupleStatus, Option<u64>) {
    let Some(t) = raw else {
        return (TupleStatus::Missing, None);
    };
    if t.event_id != expected_id || t.time_running_ns > t.time_enabled_ns {
        return (TupleStatus::Invalid, None);
    }
    if t.time_running_ns == 0 {
        return (TupleStatus::NotRunning, None);
    }
    if t.time_running_ns == t.time_enabled_ns {
        return (TupleStatus::Unmultiplexed, None);
    }
    let value = u128::from(t.value) * u128::from(t.time_enabled_ns) / u128::from(t.time_running_ns);
    match u64::try_from(value) {
        Ok(value) => (TupleStatus::MultiplexedEstimate, Some(value)),
        Err(_) => (TupleStatus::EstimateOverflow, None),
    }
}

fn completes(objective: Objective, outcome: Outcome) -> bool {
    matches!(
        (objective, outcome),
        (Objective::PdpDecision, Outcome::VerifiedPdpWitness)
            | (
                Objective::PdpDecision,
                Outcome::VerifiedFiniteDomainExhaustion
            )
            | (
                Objective::ScalarRecovery,
                Outcome::VerifiedIndependentScalarRecovery
            )
            | (
                Objective::FormulaSatisfiability,
                Outcome::VerifiedSatisfyingModel
            )
            | (
                Objective::FormulaSatisfiability,
                Outcome::VerifiedUnsatisfiabilityCertificate
            )
    )
}

/// Fixed work and space: no I/O, allocation, perf call, system clock or loop.
pub fn classify(f: &Facts) -> Decision {
    let (tuple_status, scaled) = assess(f.raw, f.expected_event_id);
    let mut bits = 0;
    let mut require = |ok: bool, reason| {
        if !ok {
            bits |= reason;
        }
    };
    require(f.unit == Unit::RetiredUserInstructions, WRONG_UNIT);
    require(
        !matches!(
            tuple_status,
            TupleStatus::Missing | TupleStatus::Invalid | TupleStatus::NotRunning
        ),
        TUPLE_UNUSABLE,
    );
    require(
        f.native_event_request_verified
            && f.selected_lifecycle_replayed
            && f.owner_context_matched
            && f.cleanup_succeeded
            && f.fallback_history_clean,
        LIFECYCLE_UNVERIFIED,
    );
    require(
        tuple_status == TupleStatus::Unmultiplexed,
        MULTIPLEXED_OR_OVERFLOW,
    );
    require(f.scope == Scope::WorkerExecToExitAllTasks, WRONG_SCOPE);
    require(
        f.task_coverage == TaskCoverage::VerifiedCompleteLineageAndExit,
        TASK_COVERAGE_UNVERIFIED,
    );
    require(f.bound_identity_verified, IDENTITY_UNVERIFIED);
    require(f.immutable_receipts_verified, RECEIPTS_UNVERIFIED);
    require(
        f.reset_before_target_exec && f.armed_before_target_exec && f.actual_target_exec_verified,
        EXEC_BOUNDARY_UNVERIFIED,
    );
    require(
        f.terminal_receipt_verified && f.read_after_all_tasks_exited,
        TERMINAL_BOUNDARY_UNVERIFIED,
    );
    require(
        f.continuous_exec_to_exit_coverage_verified,
        CONTINUOUS_INTERVAL_UNVERIFIED,
    );
    require(f.external_work_absent_or_covered, EXTERNAL_WORK_UNCOVERED);
    require(f.platform_event_semantics_admitted, PLATFORM_UNADMITTED);
    let whole_attempt = if bits == 0 {
        f.raw.map(|t| t.value)
    } else {
        None
    };
    // The remaining gates qualify a training label, separately from recording
    // already-consumed work on failed or censored attempts.
    if !f.measurement_policy_admitted {
        bits |= POLICY_UNADMITTED;
    }
    if !f.independent_receipt_replay {
        bits |= INDEPENDENT_REPLAY_MISSING;
    }
    if !completes(f.objective, f.outcome) {
        bits |= OBJECTIVE_NOT_COMPLETED;
    }
    if !f.exact_outcome_and_domain_replayed {
        bits |= OUTCOME_REPLAY_MISSING;
    }
    if !f.worker_exit_success
        || !f.original_deadline_respected
        || !f.resource_censoring_absent
        || f.interrupted_attempt
    {
        bits |= TERMINAL_CENSORED_OR_FAILED;
    }
    let label = if bits == 0 { whole_attempt } else { None };
    Decision {
        tuple_status,
        diagnostic_raw: if f.unit == Unit::RetiredUserInstructions {
            f.raw.map(|t| t.value)
        } else {
            None
        },
        diagnostic_scaled_estimate: if f.unit == Unit::RetiredUserInstructions {
            scaled
        } else {
            None
        },
        whole_attempt_user_instructions: whole_attempt,
        instruction_count_to_solve: label,
        training_cost_eligible: label.is_some(),
        rejection_bits: bits,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Synthetic policy inputs only; never serialize as measured receipts.
    fn accepted_fixture() -> Facts {
        Facts {
            unit: Unit::RetiredUserInstructions,
            raw: Some(RawTuple {
                value: 43,
                time_enabled_ns: 100,
                time_running_ns: 100,
                event_id: 7,
            }),
            expected_event_id: 7,
            native_event_request_verified: true,
            selected_lifecycle_replayed: true,
            owner_context_matched: true,
            cleanup_succeeded: true,
            fallback_history_clean: true,
            scope: Scope::WorkerExecToExitAllTasks,
            task_coverage: TaskCoverage::VerifiedCompleteLineageAndExit,
            bound_identity_verified: true,
            immutable_receipts_verified: true,
            terminal_receipt_verified: true,
            reset_before_target_exec: true,
            armed_before_target_exec: true,
            actual_target_exec_verified: true,
            continuous_exec_to_exit_coverage_verified: true,
            read_after_all_tasks_exited: true,
            external_work_absent_or_covered: true,
            measurement_policy_admitted: true,
            platform_event_semantics_admitted: true,
            independent_receipt_replay: true,
            objective: Objective::PdpDecision,
            outcome: Outcome::VerifiedPdpWitness,
            exact_outcome_and_domain_replayed: true,
            worker_exit_success: true,
            original_deadline_respected: true,
            resource_censoring_absent: true,
            interrupted_attempt: false,
        }
    }

    #[test]
    fn complete_bound_unmultiplexed_fixture_is_eligible_in_declared_scope() {
        let d = classify(&accepted_fixture());
        assert_eq!(d.instruction_count_to_solve, Some(43));
        assert!(d.training_cost_eligible);
        assert_eq!(d.rejection_bits, 0);
    }

    #[test]
    fn zero_count_is_distinct_from_missing_or_never_scheduled() {
        let mut f = accepted_fixture();
        f.raw.as_mut().unwrap().value = 0;
        assert_eq!(classify(&f).instruction_count_to_solve, Some(0));
        f.raw = None;
        assert_eq!(classify(&f).tuple_status, TupleStatus::Missing);
        let mut f = accepted_fixture();
        f.raw.as_mut().unwrap().time_running_ns = 0;
        assert_eq!(classify(&f).tuple_status, TupleStatus::NotRunning);
        assert!(!classify(&f).training_cost_eligible);
    }

    #[test]
    fn multiplexing_is_estimate_only_and_uses_integer_floor() {
        let mut f = accepted_fixture();
        f.raw.as_mut().unwrap().time_running_ns = 60;
        let d = classify(&f);
        assert_eq!(d.diagnostic_scaled_estimate, Some(71));
        assert_eq!(d.diagnostic_raw, Some(43));
        assert_eq!(d.whole_attempt_user_instructions, None);
        assert_eq!(d.instruction_count_to_solve, None);
    }

    #[test]
    fn estimate_overflow_is_not_saturated_or_exact() {
        let mut f = accepted_fixture();
        f.raw = Some(RawTuple {
            value: u64::MAX,
            time_enabled_ns: u64::MAX,
            time_running_ns: 1,
            event_id: 7,
        });
        let d = classify(&f);
        assert_eq!(d.tuple_status, TupleStatus::EstimateOverflow);
        assert_eq!(d.diagnostic_scaled_estimate, None);
        assert!(!d.training_cost_eligible);
    }

    #[test]
    fn malformed_tuple_and_event_substitution_fail_closed() {
        let mut f = accepted_fixture();
        f.raw.as_mut().unwrap().time_running_ns = 101;
        assert_eq!(classify(&f).tuple_status, TupleStatus::Invalid);
        f.raw.as_mut().unwrap().time_running_ns = 100;
        f.raw.as_mut().unwrap().event_id = 8;
        assert_eq!(classify(&f).tuple_status, TupleStatus::Invalid);
        assert!(!classify(&f).training_cost_eligible);
    }

    #[test]
    fn software_reservations_and_cycles_are_never_instructions() {
        for unit in [Unit::ModeledWorkReservations, Unit::CpuCycles] {
            let mut f = accepted_fixture();
            f.unit = unit;
            let d = classify(&f);
            assert_eq!(d.diagnostic_raw, None);
            assert_eq!(d.whole_attempt_user_instructions, None);
            assert_eq!(d.instruction_count_to_solve, None);
            assert_ne!(d.rejection_bits & WRONG_UNIT, 0);
        }
    }

    #[test]
    fn phase_or_worker_body_records_cannot_be_promoted() {
        for scope in [Scope::CallingThreadPhase, Scope::WorkerBodyCallingThread] {
            let mut f = accepted_fixture();
            f.scope = scope;
            assert_eq!(classify(&f).diagnostic_raw, Some(43));
            assert_eq!(classify(&f).whole_attempt_user_instructions, None);
        }
    }

    #[test]
    fn endpoint_scans_and_process_groups_are_insufficient() {
        for coverage in [
            TaskCoverage::Unknown,
            TaskCoverage::BoundaryTaskSnapshots,
            TaskCoverage::ProcessGroupAbsentAtEnd,
        ] {
            let mut f = accepted_fixture();
            f.task_coverage = coverage;
            let d = classify(&f);
            assert_ne!(d.rejection_bits & TASK_COVERAGE_UNVERIFIED, 0);
            assert!(!d.training_cost_eligible);
        }
    }

    #[test]
    fn every_lifecycle_and_custody_fact_is_required_for_exact_attempt_cost() {
        let changes: &[fn(&mut Facts)] = &[
            |f| f.native_event_request_verified = false,
            |f| f.selected_lifecycle_replayed = false,
            |f| f.owner_context_matched = false,
            |f| f.cleanup_succeeded = false,
            |f| f.fallback_history_clean = false,
            |f| f.bound_identity_verified = false,
            |f| f.immutable_receipts_verified = false,
            |f| f.terminal_receipt_verified = false,
            |f| f.reset_before_target_exec = false,
            |f| f.armed_before_target_exec = false,
            |f| f.actual_target_exec_verified = false,
            |f| f.continuous_exec_to_exit_coverage_verified = false,
            |f| f.read_after_all_tasks_exited = false,
            |f| f.external_work_absent_or_covered = false,
            |f| f.platform_event_semantics_admitted = false,
        ];
        for change in changes {
            let mut f = accepted_fixture();
            change(&mut f);
            let d = classify(&f);
            assert_eq!(d.whole_attempt_user_instructions, None);
            assert_eq!(d.instruction_count_to_solve, None);
            assert_ne!(d.rejection_bits, 0);
        }
    }

    #[test]
    fn label_admission_is_separate_from_preserving_attempt_cost() {
        let changes: &[fn(&mut Facts)] = &[
            |f| f.measurement_policy_admitted = false,
            |f| f.independent_receipt_replay = false,
            |f| f.exact_outcome_and_domain_replayed = false,
            |f| f.worker_exit_success = false,
            |f| f.original_deadline_respected = false,
            |f| f.resource_censoring_absent = false,
            |f| f.interrupted_attempt = true,
        ];
        for change in changes {
            let mut f = accepted_fixture();
            change(&mut f);
            let d = classify(&f);
            assert_eq!(d.whole_attempt_user_instructions, Some(43));
            assert_eq!(d.instruction_count_to_solve, None);
            assert!(!d.training_cost_eligible);
        }
    }

    #[test]
    fn platform_or_interval_failure_preserves_raw_but_no_whole_attempt_count() {
        let mut f = accepted_fixture();
        f.platform_event_semantics_admitted = false;
        let d = classify(&f);
        assert_eq!(d.diagnostic_raw, Some(43));
        assert_eq!(d.whole_attempt_user_instructions, None);
        assert_eq!(d.instruction_count_to_solve, None);
        assert_ne!(d.rejection_bits & PLATFORM_UNADMITTED, 0);
        f.platform_event_semantics_admitted = true;
        f.continuous_exec_to_exit_coverage_verified = false;
        let d = classify(&f);
        // Even valid equal times and read-after-exit do not prove no early stop.
        assert_eq!(d.tuple_status, TupleStatus::Unmultiplexed);
        assert_eq!(d.diagnostic_raw, Some(43));
        assert_eq!(d.whole_attempt_user_instructions, None);
        assert_eq!(d.instruction_count_to_solve, None);
        assert_ne!(d.rejection_bits & CONTINUOUS_INTERVAL_UNVERIFIED, 0);
    }

    #[test]
    fn failed_censored_and_unsupported_attempts_keep_observed_work_only() {
        for outcome in [
            Outcome::Censored,
            Outcome::Error,
            Outcome::Unsupported,
            Outcome::Unknown,
        ] {
            let mut f = accepted_fixture();
            f.outcome = outcome;
            let d = classify(&f);
            assert_eq!(d.whole_attempt_user_instructions, Some(43));
            assert_eq!(d.instruction_count_to_solve, None);
        }
    }

    #[test]
    fn missing_end_read_is_never_zero_or_inferred_from_other_phases() {
        let mut f = accepted_fixture();
        f.raw = None;
        f.terminal_receipt_verified = false;
        assert_eq!(classify(&f).whole_attempt_user_instructions, None);
        assert_eq!(classify(&f).instruction_count_to_solve, None);
    }

    #[test]
    fn objective_and_certificate_type_must_match() {
        let mut f = accepted_fixture();
        f.outcome = Outcome::VerifiedFiniteDomainExhaustion;
        assert!(classify(&f).training_cost_eligible);
        f.objective = Objective::ScalarRecovery;
        assert!(!classify(&f).training_cost_eligible);
        f.outcome = Outcome::ConstructedKnownAnswer;
        assert!(!classify(&f).training_cost_eligible);
        f.outcome = Outcome::VerifiedIndependentScalarRecovery;
        assert!(classify(&f).training_cost_eligible);
        f.objective = Objective::PdpDecision;
        assert!(!classify(&f).training_cost_eligible);
    }

    #[test]
    fn checked_sat_certificates_and_construction_stay_separate() {
        let mut f = accepted_fixture();
        f.objective = Objective::FormulaSatisfiability;
        for outcome in [
            Outcome::VerifiedSatisfyingModel,
            Outcome::VerifiedUnsatisfiabilityCertificate,
        ] {
            f.outcome = outcome;
            assert!(classify(&f).training_cost_eligible);
        }
        f.objective = Objective::ConstructionOnly;
        assert!(!classify(&f).training_cost_eligible);
    }
}
