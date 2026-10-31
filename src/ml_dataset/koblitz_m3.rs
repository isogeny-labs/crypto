//! Adapter-side cumulative reservations for the bounded Koblitz m3 pipeline.
//!
//! This module is not yet registered in runtime dispatch. Its constructor must
//! receive limits from a validated original SolverExecutionContext, never from
//! a producer result. Reservations model admitted work/bytes, not RSS or retired
//! instructions. Native work and actual group additions have different units.
use std::time::{Duration, Instant};

const NATIVE_WORK_MAX: u64 = 16_777_216;
const NATIVE_BYTES_MAX: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Stop {
    InvalidLimits,
    Overflow,
    Deadline,
    Phase,
    Nonmonotone,
    Work,
    Bytes,
    Trace,
    Additions,
    Observer,
    NativeFailure,
    Allocation,
    Serialization,
    Events,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Phase {
    Setup,
    Construction,
    Constructed,
    Matrix,
    MatrixComplete,
    Extraction,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Reservation {
    pub work: u64,
    pub bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct NativeLimits {
    pub work: u64,
    pub bytes: u64,
}

/// One ledger per original attempt. No phase refunds cumulative reservations.
#[derive(Debug)]
pub(super) struct Budget {
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
    deadline: Instant,
    construction_deadline: Instant,
    phase: Phase,
    native: Reservation,
    checking_work: u64,
    checking_performed_work: u64,
    phase_native: Reservation,
    phase_limits: NativeLimits,
    adapter_bytes: u64,
    trace_bytes: u64,
    additions: u64,
    stopped: Option<Stop>,
}

impl Budget {
    pub fn new(
        configured_work: u64,
        memory_bytes: u64,
        trace_bytes: u64,
        original_deadline: Instant,
        entry: Instant,
    ) -> Result<Self, Stop> {
        if configured_work == 0 || memory_bytes == 0 || trace_bytes == 0 {
            return Err(Stop::InvalidLimits);
        }
        let work = configured_work.checked_mul(1024).ok_or(Stop::Overflow)?;
        if entry >= original_deadline {
            return Err(Stop::Deadline);
        }
        let construction_deadline = entry
            .checked_add(Duration::from_secs(5))
            .ok_or(Stop::Overflow)?
            .min(original_deadline);
        Ok(Self {
            native_work_limit: work.min(NATIVE_WORK_MAX),
            memory_limit: memory_bytes,
            trace_limit: trace_bytes,
            addition_limit: configured_work,
            deadline: original_deadline,
            construction_deadline,
            phase: Phase::Setup,
            native: Reservation::default(),
            checking_work: 0,
            checking_performed_work: 0,
            phase_native: Reservation::default(),
            phase_limits: NativeLimits { work: 0, bytes: 0 },
            adapter_bytes: 0,
            trace_bytes: 0,
            additions: 0,
            stopped: None,
        })
    }

    fn fail<T>(&mut self, stop: Stop) -> Result<T, Stop> {
        let first = *self.stopped.get_or_insert(stop);
        Err(first)
    }

    fn check(&mut self, now: Instant) -> Result<(), Stop> {
        if let Some(stop) = self.stopped {
            return Err(stop);
        }
        let deadline = if matches!(self.phase, Phase::Setup | Phase::Construction) {
            self.construction_deadline
        } else {
            self.deadline
        };
        if now >= deadline {
            return self.fail(Stop::Deadline);
        }
        if self.phase == Phase::Complete {
            return self.fail(Stop::Phase);
        }
        Ok(())
    }

    /// Reserve adapter-owned allocations before allocating, including callback
    /// storage and serialized payload capacity. Native reservations are separate.
    pub fn reserve_bytes(&mut self, bytes: u64, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let Some(next) = self.adapter_bytes.checked_add(bytes) else {
            return self.fail(Stop::Overflow);
        };
        if next > self.memory_limit - self.native.bytes {
            return self.fail(Stop::Bytes);
        }
        self.adapter_bytes = next;
        Ok(())
    }

    pub fn reserve_trace(&mut self, bytes: u64, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let Some(next) = self.trace_bytes.checked_add(bytes) else {
            return self.fail(Stop::Overflow);
        };
        if next > self.trace_limit {
            return self.fail(Stop::Trace);
        }
        self.trace_bytes = next;
        Ok(())
    }

    /// The returned allowance is a phase ceiling. Call observe() at every native
    /// boundary; callback allocations can reduce remaining shared headroom.
    pub fn begin_native(&mut self, matrix: bool, now: Instant) -> Result<NativeLimits, Stop> {
        self.check(now)?;
        let expected = if matrix {
            Phase::Constructed
        } else {
            Phase::Setup
        };
        if self.phase != expected {
            return self.fail(Stop::Phase);
        }
        let remaining_bytes = self.memory_limit - self.adapter_bytes - self.native.bytes;
        let bytes = if matrix {
            remaining_bytes.min(NATIVE_BYTES_MAX)
        } else {
            remaining_bytes
                .min(self.memory_limit / 4)
                .min(NATIVE_BYTES_MAX)
        };
        let limits = NativeLimits {
            work: self.native_work_limit - self.native.work - self.checking_work,
            bytes,
        };
        self.phase = if matrix {
            Phase::Matrix
        } else {
            Phase::Construction
        };
        self.phase_native = Reservation::default();
        self.phase_limits = limits;
        Ok(limits)
    }

    /// Native APIs report cumulative phase reservations. Convert them to deltas
    /// exactly once and reject regression instead of silently refunding work.
    pub fn observe(&mut self, current: Reservation, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if !matches!(self.phase, Phase::Construction | Phase::Matrix) {
            return self.fail(Stop::Phase);
        }
        if current.work < self.phase_native.work || current.bytes < self.phase_native.bytes {
            return self.fail(Stop::Nonmonotone);
        }
        let dw = current.work - self.phase_native.work;
        let db = current.bytes - self.phase_native.bytes;
        if current.work > self.phase_limits.work
            || dw > self.native_work_limit - self.native.work - self.checking_work
        {
            return self.fail(Stop::Work);
        }
        if current.bytes > self.phase_limits.bytes
            || db > self.memory_limit - self.native.bytes - self.adapter_bytes
        {
            return self.fail(Stop::Bytes);
        }
        self.native.work += dw;
        self.native.bytes += db;
        self.phase_native = current;
        Ok(())
    }

    /// Successful phase completion is supplied only after native terminal status
    /// validation. A stopped native report must instead stop this ledger.
    pub fn finish_native(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        self.phase = match self.phase {
            Phase::Construction => Phase::Constructed,
            Phase::Matrix => Phase::MatrixComplete,
            _ => return self.fail(Stop::Phase),
        };
        Ok(())
    }

    pub fn begin_extraction(&mut self, matrix_required: bool, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let expected = if matrix_required {
            Phase::MatrixComplete
        } else {
            Phase::Constructed
        };
        if self.phase != expected {
            return self.fail(Stop::Phase);
        }
        self.phase = Phase::Extraction;
        Ok(())
    }

    /// Call immediately before each actual group.add, including identity starts.
    pub fn reserve_addition(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if self.phase != Phase::Extraction {
            return self.fail(Stop::Phase);
        }
        if self.additions == self.addition_limit {
            return self.fail(Stop::Additions);
        }
        self.additions += 1;
        Ok(())
    }

    /// The caller must separately establish the exact final cursor. Consuming
    /// exactly the allowance does not itself prevent successful completion.
    pub fn finish_extraction(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if self.phase != Phase::Extraction {
            return self.fail(Stop::Phase);
        }
        self.phase = Phase::Complete;
        Ok(())
    }

    pub fn observer_failed(&mut self) -> Result<(), Stop> {
        self.fail(Stop::Observer)
    }
}

/// Admit persisted originals and the actual capture sink before any arithmetic.
/// This function is staged with the module under test-only registration until
/// the full adapter and its semantic verifier are ready.
pub(super) fn admit_original(
    dir: &std::path::Path,
    config: &super::super::types::CaseConfig,
    context: &super::super::execution::SolverExecutionContext,
    sink: &super::super::capture::CaptureEmitter,
) -> Result<Budget, String> {
    use super::super::{admit_capture_context, execution, read_json};
    // Sample entry before file admission: setup consumes construction time.
    let entry = Instant::now();
    let saved: super::super::types::CaseConfig = read_json(&dir.join("config.json"))?;
    if saved != *config {
        return Err("Koblitz m3 config differs from original attempt".into());
    }
    let original =
        execution::admit(dir)?.ok_or("Koblitz m3 requires original execution context")?;
    if original.launch != context.launch {
        return Err("Koblitz m3 execution context differs from saved original".into());
    }
    let (binding, limits) =
        admit_capture_context(dir)?.ok_or("Koblitz m3 requires original capture context")?;
    if binding != original.launch.binding {
        return Err("Koblitz m3 capture and execution attempts differ".into());
    }
    sink.verify_attempt_limits(&binding, &limits)?;
    let mut budget = Budget::new(
        saved.work_limit,
        original.launch.memory_bytes,
        original.launch.trace_bytes,
        context.deadline.min(original.deadline),
        entry,
    )
    .map_err(|s| format!("Koblitz m3 original resource admission: {s:?}"))?;
    budget
        .check(Instant::now())
        .map_err(|s| format!("Koblitz m3 setup deadline: {s:?}"))?;
    Ok(budget)
}

/// Canonical original mathematical inputs for the initial two-code matrix
/// profile. Setup admission/accounting and persistence must precede runtime use.
pub(super) struct MathInput {
    pub curve: Box<crate::binary_ecc::BinaryCurve>,
    pub target: (u64, u64),
    pub codes: [u64; 2],
    pub signed: [(u64, u64); 4],
}

/// Validate public inputs using the existing independent curve/factor-base
/// replay. Never consult private_witness or solver-produced input artifacts.
/// The caller authenticates case provenance; this checks mathematical content.
pub(super) fn original_math(
    config: &super::super::types::CaseConfig,
    case: &super::super::types::CaseData,
) -> Result<MathInput, String> {
    use super::{curves, domain, Element, Group};
    use num_traits::ToPrimitive;
    use serde_json::json;
    if config.family != "koblitz"
        || config.m != 3
        || !((23..=31).contains(&config.n) || (47..=61).contains(&config.n))
    {
        return Err("outside Koblitz m3 field/arity profile".into());
    }
    let public = &case.public_instance;
    if public["family"] != config.family
        || public["n"] != json!(config.n)
        || public["m"] != json!(config.m)
        || public["twist"] != json!(config.twist)
        || case.factor_base["requested_size"] != json!(config.factor_base_size)
    {
        return Err("Koblitz original public inputs differ from configuration".into());
    }
    let points = case.factor_base["points"]
        .as_array()
        .ok_or("missing original factor base")?;
    if points.len() > 4 || points.is_empty() {
        return Err("Koblitz two-code profile requires at most four original signed points".into());
    }
    let candidates = case.factor_base["construction_candidates"]
        .as_array()
        .ok_or("missing original construction candidates")?;
    if candidates.len() > 255
        || candidates
            .iter()
            .any(|row| row["lifts"].as_array().is_none_or(|lifts| lifts.len() > 2))
    {
        return Err("Koblitz two-code setup candidate replay cap exceeded".into());
    }
    // Full mathematical checks include actual twist, certified order,
    // irreducibility, subgroup checks and factor-base construction replay.
    // Their setup cost is not represented by native S4 reservation counters.
    curves::verify_public(public)?;
    curves::verify_factor_base(public, &case.factor_base)?;
    let group = Group::read(case)?;
    let Group::Binary(curve) = &group else {
        return Err("Koblitz profile requires actual binary curve".into());
    };
    if curve.m != config.n || curve.b.to_biguint().to_u64() != Some(1) {
        return Err("Koblitz S4 profile requires actual b=1 and original field degree".into());
    }
    let word = |p: &Element| -> Result<(u64, u64), String> {
        match p {
            Element::Binary(crate::binary_ecc::BinaryPoint::Affine { x, y }) => Ok((
                x.to_biguint().to_u64().ok_or("x exceeds u64")?,
                y.to_biguint().to_u64().ok_or("y exceeds u64")?,
            )),
            _ => Err("Koblitz original point must be affine".into()),
        }
    };
    let target = word(&group.point(&public["target"])?)?;
    let points = domain(&group, case)?;
    if points.len() != 4 {
        return Err(
            "Koblitz matrix profile requires two distinct x codes, not two signed points".into(),
        );
    }
    let mut signed = [(0, 0); 4];
    for (out, point) in signed.iter_mut().zip(&points) {
        *out = word(point)?;
    }
    signed.sort_unstable();
    if signed.windows(2).any(|p| p[0] == p[1])
        || signed[0].0 != signed[1].0
        || signed[2].0 != signed[3].0
        || signed[0].0 == signed[2].0
        || signed[0].1 ^ signed[0].0 != signed[1].1
        || signed[2].1 ^ signed[2].0 != signed[3].1
    {
        return Err("noncanonical original signed two-code domain".into());
    }
    let codes = [signed[0].0, signed[2].0];
    let Group::Binary(mut curve) = group else {
        unreachable!("validated binary group")
    };
    curve.irreducible.low_terms.sort_unstable();
    Ok(MathInput {
        curve,
        target,
        codes,
        signed,
    })
}

/// Retain completed original equations even if the final adapter boundary fails.
/// Construction never claims a matrix invocation or a complete decomposition.
pub(super) struct Construction {
    pub output: Option<crate::cryptanalysis::koblitz_groebner::bounded_s4::Output>,
    pub native_failure: Option<crate::cryptanalysis::koblitz_groebner::bounded_s4::Failure>,
    pub adapter_stop: Option<Stop>,
}

pub(super) fn construct_original<F>(
    original: &MathInput,
    budget: &mut Budget,
    forward: &mut F,
) -> Construction
where
    F: FnMut(
        crate::cryptanalysis::koblitz_groebner::bounded_s4::Event,
        &mut Budget,
    ) -> Result<(), ()>,
{
    use crate::cryptanalysis::koblitz_groebner::bounded_s4 as native;
    let limits = match budget.begin_native(false, Instant::now()) {
        Ok(limits) => limits,
        Err(stop) => {
            return Construction {
                output: None,
                native_failure: None,
                adapter_stop: Some(stop),
            }
        }
    };
    let deadline = budget.construction_deadline;
    let result = native::build(
        &original.curve.irreducible,
        1,
        &original.codes,
        original.target.0,
        native::Limits {
            work: limits.work,
            bytes: limits.bytes,
        },
        deadline,
        &mut |event| {
            budget
                .observe(
                    Reservation {
                        work: event.reserved.work,
                        bytes: event.reserved.bytes,
                    },
                    Instant::now(),
                )
                .map_err(|_| ())?;
            if forward(event, budget).is_err() {
                let _ = budget.observer_failed();
                return Err(());
            }
            // A callback cannot hide a failed ledger reservation by returning Ok.
            budget.check(Instant::now()).map_err(|_| ())
        },
    );
    match result {
        Ok(output) => {
            let adapter_stop = budget
                .settle_native_terminal(Reservation {
                    work: output.reserved.work,
                    bytes: output.reserved.bytes,
                })
                .and_then(|()| budget.finish_native(Instant::now()))
                .err();
            Construction {
                output: Some(output),
                native_failure: None,
                adapter_stop,
            }
        }
        Err(failure) => {
            let _ = budget.settle_native_terminal(Reservation {
                work: failure.reserved.work,
                bytes: failure.reserved.bytes,
            });
            // Preserve the final native reservation and precise failure while
            // preventing any later phase from treating this return as success.
            let adapter_stop = Some(*budget.stopped.get_or_insert(Stop::NativeFailure));
            Construction {
                output: None,
                native_failure: Some(failure),
                adapter_stop,
            }
        }
    }
}

const INPUT_BYTE_CAP: usize = 64 * 1024;

#[derive(serde::Serialize)]
struct InputView<'a> {
    schema: &'static str,
    binding: &'a super::super::capture::AttemptBinding,
    public_sha256: &'a str,
    factor_base_sha256: &'a str,
    field_degree: u32,
    irreducible_low_terms: &'a [u32],
    curve_a: u64,
    curve_b: u64,
    target: (u64, u64),
    codes: [u64; 2],
    signed_domain: [(u64, u64); 4],
    selector_offsets: [u8; 3],
    n_vars: usize,
    equations: &'a [crate::cryptanalysis::pq_groebner_f2::F2BoolPoly],
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
}

struct InputWriter<'a> {
    bytes: Vec<u8>,
    capacity: usize,
    budget: &'a mut Budget,
}
impl std::io::Write for InputWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.capacity.saturating_sub(self.bytes.len()) {
            let _ = self.budget.fail::<()>(Stop::Trace);
            return Err(std::io::Error::other("Koblitz input serialization cap"));
        }
        self.budget
            .reserve_trace(bytes.len() as u64, Instant::now())
            .map_err(|s| std::io::Error::other(format!("Koblitz input ledger: {s:?}")))?;
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Only successful bounded serialization plus persistence can create this token.
/// Its exclusive budget borrow prevents substituting a fresh phase allowance.
pub(super) struct PersistedInput<'a> {
    output: &'a crate::cryptanalysis::koblitz_groebner::bounded_s4::Output,
    budget: &'a mut Budget,
    bytes: Vec<u8>,
}

#[derive(Debug)]
pub(super) struct PersistenceFailure {
    pub reason: String,
    pub bytes: Vec<u8>,
    pub serialized_complete: bool,
}
impl From<String> for PersistenceFailure {
    fn from(reason: String) -> Self {
        Self {
            reason,
            bytes: Vec::new(),
            serialized_complete: false,
        }
    }
}
impl From<&str> for PersistenceFailure {
    fn from(reason: &str) -> Self {
        reason.to_owned().into()
    }
}

pub(super) fn persist_original<'a, F>(
    original: &MathInput,
    output: &'a crate::cryptanalysis::koblitz_groebner::bounded_s4::Output,
    budget: &'a mut Budget,
    binding: &super::super::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    forward: &mut F,
) -> Result<PersistedInput<'a>, PersistenceFailure>
where
    F: FnMut(&[u8], &mut Budget) -> Result<(), String>,
{
    use num_traits::ToPrimitive;
    budget
        .check(Instant::now())
        .map_err(|s| format!("input admission: {s:?}"))?;
    if budget.phase != Phase::Constructed {
        let _ = budget.fail::<()>(Stop::Phase);
        return Err("original construction has not completed".into());
    }
    let valid = binding
        .validate()
        .and_then(|_| crate::store::index::validate_sha256(public_sha256))
        .and_then(|_| crate::store::index::validate_sha256(factor_base_sha256));
    if let Err(error) = valid {
        let _ = budget.fail::<()>(Stop::Serialization);
        return Err(error.into());
    }
    if output.n_vars != 3
        || output.equations.len() != original.curve.m as usize
        || output
            .equations
            .iter()
            .any(|p| p.n_vars != 3 || p.terms.len() > 8 || p.terms.iter().any(|t| t.mask > 7))
    {
        let _ = budget.fail::<()>(Stop::Serialization);
        return Err("original equations exceed two-code shape".into());
    }
    let view = InputView {
        schema: "crypto.ml-dataset.koblitz-m3-input/v1",
        binding,
        public_sha256,
        factor_base_sha256,
        field_degree: original.curve.m,
        irreducible_low_terms: &original.curve.irreducible.low_terms,
        curve_a: original
            .curve
            .a
            .to_biguint()
            .to_u64()
            .ok_or("curve a exceeds u64")?,
        curve_b: 1,
        target: original.target,
        codes: original.codes,
        signed_domain: original.signed,
        selector_offsets: [0, 1, 2],
        n_vars: 3,
        equations: &output.equations,
        native_work_limit: budget.native_work_limit,
        memory_limit: budget.memory_limit,
        trace_limit: budget.trace_limit,
        addition_limit: budget.addition_limit,
    };
    let capacity = (budget.trace_limit - budget.trace_bytes).min(INPUT_BYTE_CAP as u64) as usize;
    budget
        .reserve_bytes(capacity as u64, Instant::now())
        .map_err(|s| format!("input buffer: {s:?}"))?;
    let mut bytes = Vec::new();
    if bytes.try_reserve_exact(capacity).is_err() {
        let _ = budget.fail::<()>(Stop::Allocation);
        return Err("input buffer allocation failed".into());
    }
    let mut writer = InputWriter {
        bytes,
        capacity,
        budget,
    };
    if let Err(error) = serde_json::to_writer(&mut writer, &view) {
        let _ = writer.budget.fail::<()>(Stop::Serialization);
        return Err(PersistenceFailure {
            reason: error.to_string(),
            bytes: writer.bytes,
            serialized_complete: false,
        });
    }
    let InputWriter { bytes, budget, .. } = writer;
    if let Err(error) = forward(&bytes, budget) {
        let _ = budget.observer_failed();
        return Err(PersistenceFailure {
            reason: error,
            bytes,
            serialized_complete: true,
        });
    }
    if let Err(stop) = budget.check(Instant::now()) {
        return Err(PersistenceFailure {
            reason: format!("input persistence: {stop:?}"),
            bytes,
            serialized_complete: true,
        });
    }
    Ok(PersistedInput {
        output,
        budget,
        bytes,
    })
}

pub(super) struct MatrixStep {
    pub original_bytes: Vec<u8>,
    pub report: Option<crate::cryptanalysis::koblitz_groebner::bounded_matrix::Report>,
    pub adapter_stop: Option<Stop>,
}

pub(super) fn matrix_step<F>(
    input: PersistedInput<'_>,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    forward: &mut F,
) -> MatrixStep
where
    F: FnMut(
        &crate::cryptanalysis::koblitz_groebner::bounded_matrix::Event,
        &mut Budget,
    ) -> Result<(), ()>,
{
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    let PersistedInput {
        output,
        budget,
        bytes,
    } = input;
    let limits = match budget.begin_native(true, Instant::now()) {
        Ok(limits) => limits,
        Err(stop) => {
            return MatrixStep {
                original_bytes: bytes,
                report: None,
                adapter_stop: Some(stop),
            }
        }
    };
    let deadline = budget.deadline;
    let native_limits = native::Limits {
        work: limits.work,
        bytes: limits.bytes,
        main_rows: native::MAX_ROWS as u64,
        events: native::MAX_EVENTS,
    };
    let observer = |event: &native::Event| {
        budget
            .observe(
                Reservation {
                    work: event.reserved.work,
                    bytes: event.reserved.bytes,
                },
                Instant::now(),
            )
            .map_err(|_| ())?;
        if forward(event, budget).is_err() {
            let _ = budget.observer_failed();
            return Err(());
        }
        budget.check(Instant::now()).map_err(|_| ())
    };
    let report = match engine {
        native::Engine::F4Macaulay => native::matrix_f4_f2_bounded(
            &output.equations,
            output.n_vars,
            degree,
            native_limits,
            deadline,
            observer,
        ),
        native::Engine::F5KoszulFrobenius => {
            crate::cryptanalysis::matrix_f5_f2::matrix_f5_f2_bounded(
                &output.equations,
                output.n_vars,
                degree,
                native_limits,
                deadline,
                observer,
            )
        }
    };
    let settlement = budget.settle_native_terminal(Reservation {
        work: report.reserved.work,
        bytes: report.reserved.bytes,
    });
    let adapter_stop = if report.stop == native::Stop::Complete && report.native_step_complete {
        settlement
            .and_then(|()| budget.finish_native(Instant::now()))
            .err()
    } else {
        // Preserve an earlier observer/deadline failure, otherwise expose the
        // native failure recorded in the sticky ledger to the pipeline caller.
        Some(*budget.stopped.get_or_insert(Stop::NativeFailure))
    };
    MatrixStep {
        original_bytes: bytes,
        report: Some(report),
        adapter_stop,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct OriginalLimits {
    native_work: u64,
    memory: u64,
    trace: u64,
    additions: u64,
}
impl OriginalLimits {
    /// Historical caller first validates the saved launch against the saved run
    /// and attempt. No current clock is read when binding these resource values.
    pub fn from_original(
        config: &super::super::types::CaseConfig,
        launch: &super::super::execution::LaunchContext,
    ) -> Result<Self, String> {
        if config.work_limit == 0 || launch.memory_bytes == 0 || launch.trace_bytes == 0 {
            return Err("zero original Koblitz resource limit".into());
        }
        Ok(Self {
            native_work: config
                .work_limit
                .checked_mul(1024)
                .ok_or("original work overflow")?
                .min(NATIVE_WORK_MAX),
            memory: launch.memory_bytes,
            trace: launch.trace_bytes,
            additions: config.work_limit,
        })
    }
}
impl Budget {
    pub fn original_limits(&self) -> OriginalLimits {
        OriginalLimits {
            native_work: self.native_work_limit,
            memory: self.memory_limit,
            trace: self.trace_limit,
            additions: self.addition_limit,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedMonomial {
    mask: u8,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedEquation {
    n_vars: usize,
    terms: Vec<SavedMonomial>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedInput {
    schema: String,
    binding: super::super::capture::AttemptBinding,
    public_sha256: String,
    factor_base_sha256: String,
    field_degree: u32,
    irreducible_low_terms: Vec<u32>,
    curve_a: u64,
    curve_b: u64,
    target: (u64, u64),
    codes: [u64; 2],
    signed_domain: [(u64, u64); 4],
    selector_offsets: [u8; 3],
    n_vars: usize,
    equations: Vec<SavedEquation>,
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
}

/// Verify finite-domain ANF independently of the producer's symbolic builder.
/// Expected inputs/digests/limits must come from validated original files.
/// This verifies equations, not native row derivations or a complete solver.
pub(super) fn verify_original_input(
    bytes: &[u8],
    original: &MathInput,
    binding: &super::super::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    limits: OriginalLimits,
) -> Result<(), String> {
    use crate::cryptanalysis::pq_groebner_f2::{cmp_mono, mask_from64, F2BoolMono};
    use crate::cryptanalysis::semaev_decomp::{eval_f3, Gf2};
    use num_traits::ToPrimitive;
    if bytes.len() > INPUT_BYTE_CAP {
        return Err("saved Koblitz input exceeds byte cap".into());
    }
    let saved: SavedInput =
        serde_json::from_slice(bytes).map_err(|e| format!("strict Koblitz input: {e}"))?;
    saved.binding.validate()?;
    if saved.schema != "crypto.ml-dataset.koblitz-m3-input/v1"
        || saved.binding != *binding
        || saved.public_sha256 != public_sha256
        || saved.factor_base_sha256 != factor_base_sha256
        || saved.field_degree != original.curve.m
        || saved.irreducible_low_terms != original.curve.irreducible.low_terms
        || Some(saved.curve_a) != original.curve.a.to_biguint().to_u64()
        || saved.curve_b != 1
        || saved.target != original.target
        || saved.codes != original.codes
        || saved.signed_domain != original.signed
        || saved.selector_offsets != [0, 1, 2]
        || saved.n_vars != 3
        || saved.native_work_limit != limits.native_work
        || saved.memory_limit != limits.memory
        || saved.trace_limit != limits.trace
        || saved.addition_limit != limits.additions
        || saved.equations.len() != original.curve.m as usize
    {
        return Err("saved Koblitz input differs from originals".into());
    }
    let mut coefficients = [0u64; 8];
    let gf = Gf2::new(&original.curve.irreducible);
    for (assignment, value) in coefficients.iter_mut().enumerate() {
        *value = eval_f3(
            original.codes[assignment & 1],
            original.codes[(assignment >> 1) & 1],
            original.codes[(assignment >> 2) & 1],
            original.target.0,
            &gf,
        );
    }
    // Boolean Mobius transform: unique square-free polynomial coefficients.
    for bit in 0..3 {
        for mask in 0..8 {
            if mask & (1 << bit) != 0 {
                coefficients[mask] ^= coefficients[mask ^ (1 << bit)];
            }
        }
    }
    for (coordinate, equation) in saved.equations.iter().enumerate() {
        if equation.n_vars != 3
            || equation.terms.len() > 8
            || equation.terms.iter().any(|t| t.mask > 7)
            || !equation.terms.windows(2).all(|pair| {
                cmp_mono(
                    F2BoolMono::from_mask(mask_from64(pair[0].mask as u64)),
                    F2BoolMono::from_mask(mask_from64(pair[1].mask as u64)),
                )
                .is_gt()
            })
        {
            return Err("noncanonical saved Koblitz equation".into());
        }
        let actual = equation
            .terms
            .iter()
            .fold(0u8, |set, t| set | (1u8 << t.mask));
        let expected = coefficients
            .iter()
            .enumerate()
            .fold(0u8, |set, (mask, value)| {
                set | ((((value >> coordinate) & 1) as u8) << mask)
            });
        if actual != expected {
            return Err("saved Koblitz equation differs from independent scalar ANF".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn budget(work: u64, bytes: u64) -> (Budget, Instant) {
        let now = Instant::now();
        (
            Budget::new(work, bytes, 100, now + Duration::from_secs(20), now).unwrap(),
            now,
        )
    }
    fn constructed(b: &mut Budget, now: Instant, r: Reservation) {
        b.begin_native(false, now).unwrap();
        b.observe(r, now).unwrap();
        b.finish_native(now).unwrap();
    }

    #[test]
    fn native_phases_share_work_and_retained_memory() {
        let (mut b, now) = budget(10, 1000);
        b.reserve_bytes(50, now).unwrap();
        constructed(
            &mut b,
            now,
            Reservation {
                work: 3000,
                bytes: 250,
            },
        );
        assert_eq!(
            b.begin_native(true, now),
            Ok(NativeLimits {
                work: 7240,
                bytes: 700
            })
        );
        b.observe(
            Reservation {
                work: 7240,
                bytes: 700,
            },
            now,
        )
        .unwrap();
        assert_eq!(
            b.native,
            Reservation {
                work: 10240,
                bytes: 950
            }
        );
        assert_eq!(b.reserve_bytes(1, now), Err(Stop::Bytes));
    }

    #[test]
    fn repeated_cumulative_event_is_not_double_charged() {
        let (mut b, now) = budget(10, 1000);
        b.begin_native(false, now).unwrap();
        let r = Reservation {
            work: 100,
            bytes: 100,
        };
        b.observe(r, now).unwrap();
        b.observe(r, now).unwrap();
        assert_eq!(b.native, r);
        assert_eq!(
            b.observe(
                Reservation {
                    work: 99,
                    bytes: 100
                },
                now
            ),
            Err(Stop::Nonmonotone)
        );
        assert_eq!(b.finish_native(now), Err(Stop::Nonmonotone));
    }

    #[test]
    fn callback_memory_can_reduce_native_headroom() {
        let (mut b, now) = budget(10, 1000);
        constructed(
            &mut b,
            now,
            Reservation {
                work: 1,
                bytes: 200,
            },
        );
        b.begin_native(true, now).unwrap();
        b.reserve_bytes(750, now).unwrap();
        b.observe(Reservation { work: 1, bytes: 50 }, now).unwrap();
        assert_eq!(
            b.observe(Reservation { work: 1, bytes: 51 }, now),
            Err(Stop::Bytes)
        );
        assert_eq!(b.native.bytes, 250);
    }

    #[test]
    fn construction_deadline_includes_setup_and_never_restarts() {
        let (mut b, now) = budget(10, 1000);
        b.begin_native(false, now + Duration::from_secs(4)).unwrap();
        assert_eq!(
            b.observe(Reservation::default(), now + Duration::from_secs(5)),
            Err(Stop::Deadline)
        );
        assert_eq!(b.reserve_trace(1, now), Err(Stop::Deadline));
    }

    #[test]
    fn matrix_uses_original_deadline() {
        let (mut b, now) = budget(10, 1000);
        constructed(&mut b, now, Reservation::default());
        b.begin_native(true, now + Duration::from_secs(6)).unwrap();
        assert_eq!(
            b.finish_native(now + Duration::from_secs(20)),
            Err(Stop::Deadline)
        );
    }

    #[test]
    fn exact_addition_limit_can_finish_but_extra_call_fails() {
        for extra in [false, true] {
            let (mut b, now) = budget(2, 1000);
            constructed(&mut b, now, Reservation::default());
            b.begin_extraction(false, now).unwrap();
            b.reserve_addition(now).unwrap();
            b.reserve_addition(now).unwrap();
            if extra {
                assert_eq!(b.reserve_addition(now), Err(Stop::Additions));
                assert_eq!(b.finish_extraction(now), Err(Stop::Additions));
            } else {
                b.finish_extraction(now).unwrap();
            }
        }
    }

    #[test]
    fn matrix_cannot_be_skipped_when_required_or_run_twice() {
        let (mut b, now) = budget(2, 1000);
        constructed(&mut b, now, Reservation::default());
        assert_eq!(b.begin_extraction(true, now), Err(Stop::Phase));
        let (mut b, now) = budget(2, 1000);
        constructed(&mut b, now, Reservation::default());
        b.begin_native(true, now).unwrap();
        b.finish_native(now).unwrap();
        assert_eq!(b.begin_native(true, now), Err(Stop::Phase));
    }

    #[test]
    fn overflow_zero_trace_limit_and_observer_failure_are_explicit() {
        let (mut b, now) = budget(2, 1000);
        assert!(matches!(
            Budget::new(u64::MAX, 1000, 100, now + Duration::from_secs(1), now),
            Err(Stop::Overflow)
        ));
        assert!(matches!(
            Budget::new(1, 1000, 0, now + Duration::from_secs(1), now),
            Err(Stop::InvalidLimits)
        ));
        b.reserve_trace(100, now).unwrap();
        assert_eq!(b.reserve_trace(1, now), Err(Stop::Trace));
        assert_eq!(b.observer_failed(), Err(Stop::Trace));
        let (mut b, now) = budget(2, 1000);
        assert_eq!(b.observer_failed(), Err(Stop::Observer));
        assert_eq!(b.begin_native(false, now), Err(Stop::Observer));
    }

    #[test]
    fn native_one_over_work_or_builder_bytes_is_rejected_before_mutation() {
        for (r, stop) in [
            (
                Reservation {
                    work: 1025,
                    bytes: 0,
                },
                Stop::Work,
            ),
            (
                Reservation {
                    work: 0,
                    bytes: 251,
                },
                Stop::Bytes,
            ),
        ] {
            let (mut b, now) = budget(1, 1000);
            b.begin_native(false, now).unwrap();
            assert_eq!(b.observe(r, now), Err(stop));
            assert_eq!(b.native, Reservation::default());
        }
    }
}

#[cfg(test)]
mod admission_tests {
    use super::*;
    use crate::ml_dataset::{self as dataset, capture, execution, storage, telemetry};
    use serde_json::json;
    use std::path::PathBuf;

    struct Fixture {
        dir: PathBuf,
        config: dataset::types::CaseConfig,
        context: execution::SolverExecutionContext,
        sink: capture::CaptureEmitter,
    }
    impl Fixture {
        fn new() -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir = std::env::temp_dir()
                .join(format!("koblitz-admission-{}-{nonce}", std::process::id()));
            std::fs::create_dir(&dir).unwrap();
            let mut protocol = json!({"schema":"experiment.protocol/v1", "purpose":"Koblitz adapter software admission control"});
            let body = dataset::protocol_canonical_bytes(&protocol).unwrap();
            protocol["freeze"] = json!({"status":"frozen", "protocol_sha256":dataset::digest(&body), "filed_before_affected_data":true, "frozen_at":"2026-10-30T00:00:00Z"});
            let bytes = serde_json::to_vec(&protocol).unwrap();
            let plan = dataset::DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: vec![23],
                arities: vec![3],
                families: vec!["koblitz".into()],
                solvers: vec!["f4".into()],
                curve_seeds: 1,
                generators: 1,
                keys: 1,
                seed: 42,
                factor_base_size: 2,
                degree_bound: 3,
                work_limit: 100,
                twists: true,
                protocol_sha256: Some(dataset::digest(&bytes)),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            let hash = plan.binding_sha256().unwrap();
            let config = plan.cell(0).unwrap();
            let run = json!({"schema":"crypto.ml-dataset.run/v1", "plan_sha256":hash,
                "source_commit":"a".repeat(40), "binary_sha256":"b".repeat(64),
                "seconds_per_case":30,"memory_mib":64,"trace_mib":16,"execution_context_required":true});
            storage::write_json(&dir, "run.json", "run", &run).unwrap();
            storage::write_json(&dir, "plan.json", "plan", &plan).unwrap();
            storage::write_json(&dir, "config.json", "config", &config).unwrap();
            storage::write_json(
                &dir,
                "attempt.json",
                "attempt",
                &json!({"schema":"crypto.ml-dataset.attempt/v1","cell_index":0,"plan_sha256":hash}),
            )
            .unwrap();
            storage::write_bytes(&dir, "protocol.json", "protocol", "json", &bytes).unwrap();
            execution::create(&dir).unwrap();
            telemetry::checked::install_policy(&dir).unwrap();
            let binding = capture::AttemptBinding::from_cell_files(&dir).unwrap();
            let limits = capture::capture_limits(16).unwrap();
            storage::write_json(
                &dir,
                "capture-binding.json",
                "partial-capture-binding",
                &binding,
            )
            .unwrap();
            storage::write_json(
                &dir,
                "capture-limits.json",
                "partial-capture-limits",
                &limits,
            )
            .unwrap();
            let sink = capture::CaptureEmitter::create(&dir.join("live"), binding, limits).unwrap();
            let context = execution::admit(&dir).unwrap().unwrap();
            Self {
                dir,
                config,
                context,
                sink,
            }
        }
        fn admit(&self) -> Result<Budget, String> {
            admit_original(&self.dir, &self.config, &self.context, &self.sink)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn admission_uses_original_limits_and_never_extends_earlier_deadline() {
        let mut f = Fixture::new();
        f.context.deadline = Instant::now() + Duration::from_secs(2);
        let b = f.admit().unwrap();
        assert_eq!(
            b.original_limits(),
            OriginalLimits::from_original(&f.config, &f.context.launch).unwrap()
        );
        assert_eq!(b.native_work_limit, 102400);
        assert_eq!(b.addition_limit, 100);
        assert_eq!(b.memory_limit, 64 << 20);
        assert_eq!(b.trace_limit, 16 << 20);
        assert_eq!(b.deadline, f.context.deadline);
        let before = std::fs::read(f.dir.join("live/committed.json")).unwrap();
        f.context.deadline = Instant::now();
        assert!(f.admit().unwrap_err().contains("Deadline"));
        assert_eq!(
            std::fs::read(f.dir.join("live/committed.json")).unwrap(),
            before
        );
    }

    #[test]
    fn admission_rejects_config_context_capture_and_saved_run_mutations() {
        let mut f = Fixture::new();
        f.config.work_limit += 1;
        assert!(f.admit().unwrap_err().contains("config differs"));
        f.config.work_limit -= 1;
        f.context.launch.memory_bytes += 1;
        assert!(f.admit().unwrap_err().contains("context differs"));
        f.context.launch.memory_bytes -= 1;
        let original = std::fs::read(f.dir.join("capture-limits.json")).unwrap();
        std::fs::write(
            f.dir.join("capture-limits.json"),
            serde_json::to_vec(&capture::capture_limits(8).unwrap()).unwrap(),
        )
        .unwrap();
        assert!(f.admit().unwrap_err().contains("capture limits differ"));
        std::fs::write(f.dir.join("capture-limits.json"), original).unwrap();
        let mut run: serde_json::Value = dataset::read_json(&f.dir.join("run.json")).unwrap();
        run["memory_mib"] = json!(128);
        std::fs::write(f.dir.join("run.json"), serde_json::to_vec(&run).unwrap()).unwrap();
        assert!(f.admit().is_err());
    }

    #[test]
    fn admission_rejects_another_actual_sink_and_missing_context() {
        let f = Fixture::new();
        let other = Fixture::new();
        // Byte-identical fixtures can share their content binding; explicitly
        // alter the attempt identity to test a genuinely different sink.
        let mut binding = other.context.launch.binding.clone();
        binding.attempt_sha256 = "9".repeat(64);
        let sink = capture::CaptureEmitter::create(
            &other.dir.join("different"),
            binding,
            capture::capture_limits(16).unwrap(),
        )
        .unwrap();
        assert!(admit_original(&f.dir, &f.config, &f.context, &sink).is_err());
        std::fs::remove_file(f.dir.join(execution::PATH)).unwrap();
        assert!(f.admit().unwrap_err().contains("execution context missing"));
    }
}

#[cfg(test)]
mod mathematical_tests {
    use super::*;
    use crate::ml_dataset::{curves, types::CaseConfig};
    use serde_json::{json, Value};

    fn config(n: u32, twist: bool, size: usize) -> CaseConfig {
        CaseConfig {
            family: "koblitz".into(),
            n,
            m: 3,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: size,
            degree_bound: 3,
            work_limit: 16384,
            solver: "f4".into(),
        }
    }

    #[test]
    fn original_math_replays_both_actual_twists_at_requested_boundaries() {
        for n in [23, 31, 47, 53, 61] {
            for twist in [false, true] {
                let cfg = config(n, twist, 4);
                let mut case = curves::generate(&cfg).unwrap();
                case.private_witness = Value::Null;
                let original = original_math(&cfg, &case).unwrap();
                assert_eq!(original.curve.m, n);
                assert!(original.codes[0] < original.codes[1]);
                assert_eq!(
                    original.signed.map(|p| p.0),
                    [
                        original.codes[0],
                        original.codes[0],
                        original.codes[1],
                        original.codes[1]
                    ]
                );
                assert_eq!(
                    curves::point_words(&case.public_instance["target"]).unwrap(),
                    Some(original.target)
                );
                assert_eq!(original.curve.irreducible.low_terms[0], 0);
                case.private_witness = json!({"must_not_be_read":"deliberately unusable"});
                assert_eq!(original_math(&cfg, &case).unwrap().signed, original.signed);
            }
        }
    }

    #[test]
    fn two_signed_points_are_not_two_selector_codes() {
        let cfg = config(23, false, 2);
        let case = curves::generate(&cfg).unwrap();
        assert_eq!(case.factor_base["points"].as_array().unwrap().len(), 2);
        assert!(original_math(&cfg, &case)
            .err()
            .unwrap()
            .contains("two distinct x codes"));
    }

    #[test]
    fn original_math_rejects_curve_target_domain_and_configuration_mutations() {
        let cfg = config(23, false, 4);
        let case = curves::generate(&cfg).unwrap();
        original_math(&cfg, &case).unwrap();
        for mutation in 0..8 {
            let mut changed = case.clone();
            match mutation {
                0 => changed.public_instance["curve"]["b"] = json!("2"),
                1 => changed.public_instance["target"] = Value::Null,
                2 => changed.public_instance["twist"] = json!(true),
                3 => changed.factor_base["points"][0] = changed.factor_base["points"][1].clone(),
                4 => changed.public_instance["field"]["irreducible_low_terms"] = json!([0]),
                5 => changed.factor_base["requested_size"] = json!(2),
                6 => {
                    changed.factor_base["construction_candidates"] =
                        json!(vec![json!({"lifts":[]}); 256])
                }
                7 => {
                    changed.factor_base["construction_candidates"][0]["lifts"] =
                        json!([null, null, null])
                }
                _ => unreachable!(),
            }
            assert!(
                original_math(&cfg, &changed).is_err(),
                "accepted mutation {mutation}"
            );
        }
        let mut changed = cfg.clone();
        changed.family = "binary".into();
        assert!(original_math(&changed, &case).is_err());
        changed = cfg;
        changed.m = 4;
        assert!(original_math(&changed, &case).is_err());
    }
}

#[cfg(test)]
mod construction_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_s4 as native;
    use crate::ml_dataset::{curves, types::CaseConfig};

    pub(super) fn fixture() -> MathInput {
        let config = CaseConfig {
            family: "koblitz".into(),
            n: 23,
            m: 3,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 4,
            degree_bound: 3,
            work_limit: 16384,
            solver: "weil".into(),
        };
        original_math(&config, &curves::generate(&config).unwrap()).unwrap()
    }
    pub(super) fn budget() -> Budget {
        let now = Instant::now();
        Budget::new(
            16384,
            512 << 20,
            16 << 20,
            now + Duration::from_secs(20),
            now,
        )
        .unwrap()
    }

    #[test]
    fn native_construction_shares_reservations_and_leaves_matrix_headroom() {
        let input = fixture();
        let mut budget = budget();
        let mut events = 0;
        let result = construct_original(&input, &mut budget, &mut |_, _| {
            events += 1;
            Ok(())
        });
        assert!(
            result.native_failure.is_none(),
            "{:?}",
            result.native_failure
        );
        assert_eq!(result.adapter_stop, None);
        let output = result.output.unwrap();
        assert_eq!(output.n_vars, 3);
        assert_eq!(output.equations.len(), 23);
        assert!(events > 1);
        assert!(output.reserved.work > 0);
        assert_eq!(
            budget.native,
            Reservation {
                work: output.reserved.work,
                bytes: output.reserved.bytes
            }
        );
        let remaining = budget.begin_native(true, Instant::now()).unwrap();
        assert_eq!(remaining.work, NATIVE_WORK_MAX - output.reserved.work);
    }

    #[test]
    fn native_stop_and_callback_failure_prevent_later_matrix_work() {
        let input = fixture();
        let mut baseline = budget();
        let complete = construct_original(&input, &mut baseline, &mut |_, _| Ok(()));
        let used = complete.output.unwrap().reserved.work;
        let mut capped = budget();
        capped.native_work_limit = used - 1;
        let result = construct_original(&input, &mut capped, &mut |_, _| Ok(()));
        assert!(result.output.is_none());
        assert_eq!(result.native_failure.unwrap().stop, native::Stop::WorkLimit);
        assert_eq!(result.adapter_stop, Some(Stop::NativeFailure));
        assert_eq!(
            capped.native.work,
            result.native_failure.unwrap().reserved.work
        );
        assert_eq!(
            capped.begin_native(true, Instant::now()),
            Err(Stop::NativeFailure)
        );
        let mut refused = budget();
        let result = construct_original(&input, &mut refused, &mut |_, _| Err(()));
        assert_eq!(result.native_failure.unwrap().stop, native::Stop::Observer);
        assert_eq!(result.adapter_stop, Some(Stop::Observer));
        assert_eq!(
            refused.begin_native(true, Instant::now()),
            Err(Stop::Observer)
        );
    }

    #[test]
    fn callback_cannot_swallow_ledger_failure_and_expired_setup_does_no_native_work() {
        let input = fixture();
        let mut limited = budget();
        let result = construct_original(&input, &mut limited, &mut |_, b| {
            let _ = b.reserve_trace(u64::MAX, Instant::now());
            Ok(())
        });
        assert_eq!(result.native_failure.unwrap().stop, native::Stop::Observer);
        assert_eq!(result.adapter_stop, Some(Stop::Trace));
        let mut expired = budget();
        expired.construction_deadline = Instant::now();
        let mut called = false;
        let result = construct_original(&input, &mut expired, &mut |_, _| {
            called = true;
            Ok(())
        });
        assert!(!called);
        assert!(result.output.is_none());
        assert!(result.native_failure.is_none());
        assert_eq!(result.adapter_stop, Some(Stop::Deadline));
        assert_eq!(expired.native, Reservation::default());
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::AttemptBinding;
    pub(super) fn binding() -> AttemptBinding {
        AttemptBinding {
            schema: "crypto.ml-dataset.attempt-binding/v1".into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: "4".repeat(64),
            source_commit: "5".repeat(40),
            binary_sha256: "6".repeat(64),
            cell_index: 0,
            attempt_sha256: "7".repeat(64),
        }
    }
    #[test]
    fn both_named_matrix_engines_require_persisted_original_and_keep_native_reports() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            let original = construction_tests::fixture();
            let mut budget = construction_tests::budget();
            let construction = construct_original(&original, &mut budget, &mut |_, _| Ok(()));
            let output = construction.output.unwrap();
            let constructed_work = budget.native.work;
            let mut persisted = false;
            let input = persist_original(
                &original,
                &output,
                &mut budget,
                &binding(),
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |bytes, _| {
                    let value: serde_json::Value = serde_json::from_slice(bytes).unwrap();
                    assert_eq!(value["schema"], "crypto.ml-dataset.koblitz-m3-input/v1");
                    assert_eq!(
                        value["equations"],
                        serde_json::to_value(&output.equations).unwrap()
                    );
                    persisted = true;
                    Ok(())
                },
            )
            .unwrap();
            let mut events = 0;
            let result = matrix_step(input, engine, 3, &mut |_, _| {
                assert!(persisted);
                events += 1;
                Ok(())
            });
            assert_eq!(result.adapter_stop, None);
            let report = result.report.unwrap();
            assert_eq!(report.engine, engine);
            assert_eq!(report.stop, native::Stop::Complete);
            assert!(report.backend_invoked && report.native_step_complete);
            assert!(report.counters.nonempty_rows > 0);
            assert!(report.counters.reduction_word_xors > 0);
            assert!(events > 0);
            assert!(!result.original_bytes.is_empty());
            assert_eq!(budget.native.work, constructed_work + report.reserved.work);
        }
    }
    #[test]
    fn failed_input_flush_retains_complete_bytes_and_prevents_matrix_admission() {
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let error = persist_original(
            &original,
            &output,
            &mut budget,
            &binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Err("selected I/O failure".into()),
        )
        .err()
        .unwrap();
        assert_eq!(error.reason, "selected I/O failure");
        assert!(error.serialized_complete);
        let value: serde_json::Value = serde_json::from_slice(&error.bytes).unwrap();
        assert_eq!(
            value["equations"],
            serde_json::to_value(&output.equations).unwrap()
        );
        assert_eq!(
            budget.begin_native(true, Instant::now()),
            Err(Stop::Observer)
        );
    }
    #[test]
    fn exact_input_trace_cap_passes_and_one_less_never_reaches_sink() {
        let original = construction_tests::fixture();
        let mut baseline = construction_tests::budget();
        let output = construct_original(&original, &mut baseline, &mut |_, _| Ok(()))
            .output
            .unwrap();
        // Fix the serialized trace limit to a stable value; consume all but the
        // exact payload allowance before serialization in the two controls.
        let token = persist_original(
            &original,
            &output,
            &mut baseline,
            &binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let length = token.bytes.len() as u64;
        drop(token);
        for short in [false, true] {
            let mut budget = construction_tests::budget();
            let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
                .output
                .unwrap();
            budget
                .reserve_trace(
                    budget.trace_limit - length + u64::from(short),
                    Instant::now(),
                )
                .unwrap();
            let mut called = false;
            let result = persist_original(
                &original,
                &output,
                &mut budget,
                &binding(),
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |_, _| {
                    called = true;
                    Ok(())
                },
            );
            if short {
                let error = result.err().unwrap();
                assert!(!error.serialized_complete);
                assert!(!called);
                assert_eq!(budget.stopped, Some(Stop::Trace));
            } else {
                assert!(result.is_ok());
                assert!(called);
            }
        }
    }
    #[test]
    fn matrix_callback_failure_retains_input_and_seals_budget() {
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let token = persist_original(
            &original,
            &output,
            &mut budget,
            &binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let result = matrix_step(token, native::Engine::F4Macaulay, 3, &mut |_, _| Err(()));
        assert_eq!(result.report.unwrap().stop, native::Stop::Observer);
        assert_eq!(result.adapter_stop, Some(Stop::Observer));
        assert!(!result.original_bytes.is_empty());
        assert_eq!(
            budget.begin_extraction(true, Instant::now()),
            Err(Stop::Observer)
        );
    }
}

#[cfg(test)]
mod input_verification_tests {
    use super::*;
    use serde_json::{json, Value};
    fn fixture() -> (MathInput, OriginalLimits, Vec<u8>) {
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let limits = budget.original_limits();
        let input = persist_original(
            &original,
            &output,
            &mut budget,
            &matrix_tests::binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let PersistedInput { bytes, .. } = input;
        (original, limits, bytes)
    }
    fn verify(original: &MathInput, limits: OriginalLimits, bytes: &[u8]) -> Result<(), String> {
        verify_original_input(
            bytes,
            original,
            &matrix_tests::binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            limits,
        )
    }
    #[test]
    fn saved_input_matches_independent_scalar_coefficients_without_runtime_clock() {
        let (original, limits, bytes) = fixture();
        verify(&original, limits, &bytes).unwrap();
        // The historical verifier accepts only original values, no live Budget
        // or Instant. A different original allowance still fails binding.
        let mut changed = limits;
        changed.memory += 1;
        assert!(verify(&original, changed, &bytes).is_err());
    }
    #[test]
    fn reserialized_semantic_mutations_and_unknown_nested_fields_fail() {
        let (original, limits, bytes) = fixture();
        let baseline: Value = serde_json::from_slice(&bytes).unwrap();
        for mutation in 0..12 {
            let mut value = baseline.clone();
            match mutation {
                0 => value["codes"][0] = json!(0),
                1 => value["target"][1] = json!(value["target"][1].as_u64().unwrap() ^ 1),
                2 => value["curve_b"] = json!(2),
                3 => value["public_sha256"] = json!("a".repeat(64)),
                4 => value["selector_offsets"] = json!([2, 1, 0]),
                5 => value["native_work_limit"] = json!(1),
                6 => value["binding"]["attempt_sha256"] = json!("b".repeat(64)),
                7 => value["unexpected"] = json!(true),
                8 => value["equations"][0]["unexpected"] = json!(true),
                9 => {
                    let p = value["equations"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|p| !p["terms"].as_array().unwrap().is_empty())
                        .unwrap();
                    p["terms"][0]["unexpected"] = json!(true);
                }
                10 => {
                    let p = value["equations"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|p| !p["terms"].as_array().unwrap().is_empty())
                        .unwrap();
                    p["terms"].as_array_mut().unwrap().remove(0);
                }
                11 => {
                    let p = value["equations"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|p| !p["terms"].as_array().unwrap().is_empty())
                        .unwrap();
                    let term = p["terms"][0].clone();
                    p["terms"].as_array_mut().unwrap().push(term);
                }
                _ => unreachable!(),
            }
            assert!(
                verify(&original, limits, &serde_json::to_vec(&value).unwrap()).is_err(),
                "accepted mutation {mutation}"
            );
        }
        assert!(verify(&original, limits, &vec![b' '; INPUT_BYTE_CAP + 1]).is_err());
    }
    #[test]
    fn independent_input_check_covers_all_requested_boundary_twists() {
        use crate::ml_dataset::{curves, types::CaseConfig};
        for n in [23, 31, 47, 53, 61] {
            for twist in [false, true] {
                let config = CaseConfig {
                    family: "koblitz".into(),
                    n,
                    m: 3,
                    twist,
                    curve_seed: 7,
                    generator_index: 1,
                    key_index: 1,
                    seed: 42,
                    factor_base_size: 4,
                    degree_bound: 3,
                    work_limit: 16384,
                    solver: "weil".into(),
                };
                let original = original_math(&config, &curves::generate(&config).unwrap()).unwrap();
                let mut budget = construction_tests::budget();
                let limits = budget.original_limits();
                let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
                    .output
                    .unwrap();
                let input = persist_original(
                    &original,
                    &output,
                    &mut budget,
                    &matrix_tests::binding(),
                    &"8".repeat(64),
                    &"9".repeat(64),
                    &mut |_, _| Ok(()),
                )
                .unwrap();
                verify(&original, limits, &input.bytes).unwrap();
            }
        }
    }
}
/// A single actual addition in the lexicographic four-point signed domain.
/// `prefix == None` denotes infinity, including an inverse-pair prefix.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SignedPrefix {
    sequence: u16,
    tuple: u8,
    indices: [u8; 3],
    summand: u8,
    prefix: Option<(u64, u64)>,
    matches_target: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PrefixAssessment {
    performed_additions: usize,
    acknowledged_additions: usize,
    completed_tuples: usize,
    candidate_tuples: Vec<u8>,
    all_tuples_acknowledged: bool,
}

/// Replay a bounded historical prefix using only original-domain arithmetic.
/// The final optional event was performed but not acknowledged by the sink.
/// This function establishes neither durable witness custody nor an uncensored
/// solver outcome. Outer binding/role validation and budget accounting remain
/// mandatory before using its assessment in a dataset label.
pub(super) fn verify_signed_prefix(
    original: &MathInput,
    acknowledged: &[SignedPrefix],
    unacknowledged: Option<&SignedPrefix>,
) -> Result<PrefixAssessment, String> {
    use crate::binary_ecc::{curve::point_add, BinaryPoint, F2mElement};
    use num_traits::ToPrimitive;
    let performed = acknowledged
        .len()
        .checked_add(usize::from(unacknowledged.is_some()))
        .ok_or("signed prefix length overflow")?;
    if performed > 192 {
        return Err("signed prefix exceeds 64 ordered triples".into());
    }
    let domain = original.signed.map(|(x, y)| BinaryPoint::Affine {
        x: F2mElement::from_words(&[x], original.curve.m),
        y: F2mElement::from_words(&[y], original.curve.m),
    });
    let mut prefix = BinaryPoint::Infinity;
    let mut candidates = Vec::new();
    candidates
        .try_reserve_exact(64)
        .map_err(|_| "signed prefix allocation")?;
    for (sequence, saved) in acknowledged.iter().chain(unacknowledged).enumerate() {
        let tuple = sequence / 3;
        let summand = sequence % 3;
        let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
        if summand == 0 {
            prefix = BinaryPoint::Infinity;
        }
        prefix = point_add(&original.curve, &prefix, &domain[indices[summand]]);
        let expected = match &prefix {
            BinaryPoint::Infinity => None,
            BinaryPoint::Affine { x, y } => Some((
                x.to_biguint().to_u64().ok_or("reference x exceeds word")?,
                y.to_biguint().to_u64().ok_or("reference y exceeds word")?,
            )),
        };
        let matched = summand == 2 && expected == Some(original.target);
        if saved.sequence as usize != sequence
            || saved.tuple as usize != tuple
            || saved.indices != indices.map(|i| i as u8)
            || saved.summand as usize != summand
            || saved.prefix != expected
            || saved.matches_target != matched
        {
            return Err(format!("signed prefix mismatch at addition {sequence}"));
        }
        if matched {
            candidates.push(tuple as u8);
        }
    }
    Ok(PrefixAssessment {
        performed_additions: performed,
        acknowledged_additions: acknowledged.len(),
        completed_tuples: performed / 3,
        candidate_tuples: candidates,
        all_tuples_acknowledged: acknowledged.len() == 192 && unacknowledged.is_none(),
    })
}

#[cfg(test)]
mod signed_prefix_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve;
    use num_traits::ToPrimitive;

    fn fast_events(original: &MathInput) -> Vec<SignedPrefix> {
        let curve = FastBinaryCurve::new(
            &original.curve.irreducible,
            original.curve.a.to_biguint().to_u64().unwrap(),
        )
        .unwrap();
        let mut events = Vec::new();
        for tuple in 0..64usize {
            let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
            let mut prefix = None;
            for summand in 0..3 {
                prefix = curve.add(prefix, Some(original.signed[indices[summand]]));
                events.push(SignedPrefix {
                    sequence: events.len() as u16,
                    tuple: tuple as u8,
                    indices: indices.map(|i| i as u8),
                    summand: summand as u8,
                    prefix,
                    matches_target: summand == 2 && prefix == Some(original.target),
                });
            }
        }
        events
    }

    #[test]
    fn independent_signed_replay_keeps_every_partial_boundary_and_infinity() {
        let mut original = construction_tests::fixture();
        // A fixed positive control: P + (-P) + Q = Q, without resampling.
        original.target = original.signed[0];
        let events = fast_events(&original);
        assert!(events.iter().any(|e| e.prefix.is_none()));
        let full = verify_signed_prefix(&original, &events, None).unwrap();
        assert!(full.all_tuples_acknowledged);
        assert!(!full.candidate_tuples.is_empty());
        assert_eq!(full.completed_tuples, 64);
        for n in 0..192 {
            let partial = verify_signed_prefix(&original, &events[..n], None).unwrap();
            assert_eq!(partial.performed_additions, n);
            assert!(!partial.all_tuples_acknowledged);
            let refused = verify_signed_prefix(&original, &events[..n], Some(&events[n])).unwrap();
            assert_eq!(refused.performed_additions, n + 1);
            assert_eq!(refused.acknowledged_additions, n);
            assert!(!refused.all_tuples_acknowledged);
        }
    }

    #[test]
    fn independent_signed_replay_rejects_semantic_and_order_mutations() {
        let original = construction_tests::fixture();
        let events = fast_events(&original);
        for mutation in 0..8 {
            let mut changed = events.clone();
            match mutation {
                0 => changed[1].sequence += 1,
                1 => changed[1].tuple += 1,
                2 => changed[1].indices[0] ^= 1,
                3 => changed[1].summand = 0,
                4 => changed[0].prefix = None,
                5 => changed[2].matches_target = !changed[2].matches_target,
                6 => {
                    changed.remove(1);
                }
                7 => changed.push(events[0].clone()),
                _ => unreachable!(),
            }
            assert!(
                verify_signed_prefix(&original, &changed, None).is_err(),
                "mutation {mutation}"
            );
        }
        let mut value = serde_json::to_value(&events[0]).unwrap();
        value["unchecked"] = serde_json::json!(true);
        assert!(serde_json::from_value::<SignedPrefix>(value).is_err());
    }

    #[test]
    fn independent_signed_replay_covers_both_twists_at_field_boundaries() {
        use crate::ml_dataset::{curves, types::CaseConfig};
        for n in [23, 31, 47, 53, 61] {
            for twist in [false, true] {
                let cfg = CaseConfig {
                    family: "koblitz".into(),
                    n,
                    m: 3,
                    twist,
                    curve_seed: 7,
                    generator_index: 1,
                    key_index: 1,
                    seed: 42,
                    factor_base_size: 4,
                    degree_bound: 3,
                    work_limit: 16384,
                    solver: "weil".into(),
                };
                let original = original_math(&cfg, &curves::generate(&cfg).unwrap()).unwrap();
                let events = fast_events(&original);
                let result = verify_signed_prefix(&original, &events, None).unwrap();
                assert!(result.all_tuples_acknowledged);
                assert_eq!(result.performed_additions, 192);
            }
        }
    }
}

const SIGNED_REPLAY_BYTE_CAP: usize = 128 * 1024;

/// A separately bound arithmetic-prefix artifact, not a native execution log.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedSignedReplay {
    schema: String,
    binding: super::super::capture::AttemptBinding,
    public_sha256: String,
    factor_base_sha256: String,
    input_sha256: String,
    addition_limit: u64,
    performed_additions: usize,
    acknowledged_additions: usize,
    acknowledged: Vec<SignedPrefix>,
    unacknowledged: Option<SignedPrefix>,
}

/// Expected digests and limits come from authenticated original artifacts and
/// the admitted launch, never from the saved transcript. A historical check has
/// no live attempt deadline. Decode size and arithmetic replay are bounded.
pub(super) fn verify_signed_replay(
    bytes: &[u8],
    original: &MathInput,
    binding: &super::super::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    input_sha256: &str,
    addition_limit: u64,
) -> Result<PrefixAssessment, String> {
    if bytes.len() > SIGNED_REPLAY_BYTE_CAP {
        return Err("signed replay exceeds byte cap".into());
    }
    let saved: SavedSignedReplay =
        serde_json::from_slice(bytes).map_err(|e| format!("strict signed replay: {e}"))?;
    saved.binding.validate()?;
    let hash_valid = |s: &str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit());
    if saved.schema != "crypto.ml-dataset.koblitz-m3-signed-prefix/v1"
        || saved.binding != *binding
        || !hash_valid(public_sha256)
        || !hash_valid(factor_base_sha256)
        || !hash_valid(input_sha256)
        || saved.public_sha256 != public_sha256
        || saved.factor_base_sha256 != factor_base_sha256
        || saved.input_sha256 != input_sha256
        || saved.addition_limit != addition_limit
        || saved.performed_additions as u64 > addition_limit
        || saved.performed_additions > 192
        || saved.acknowledged_additions != saved.acknowledged.len()
        || saved.performed_additions
            != saved
                .acknowledged
                .len()
                .checked_add(usize::from(saved.unacknowledged.is_some()))
                .ok_or("signed replay counter overflow")?
    {
        return Err("signed replay binding, limit or counter mismatch".into());
    }
    verify_signed_prefix(original, &saved.acknowledged, saved.unacknowledged.as_ref())
}

#[cfg(test)]
mod signed_binding_tests {
    use super::*;
    #[test]
    fn signed_replay_rejects_binding_counter_limit_and_schema_mutations() {
        let original = construction_tests::fixture();
        let binding = matrix_tests::binding();
        let original_value = serde_json::json!({
            "schema": "crypto.ml-dataset.koblitz-m3-signed-prefix/v1",
            "binding": binding,
            "public_sha256": "8".repeat(64),
            "factor_base_sha256": "9".repeat(64),
            "input_sha256": "a".repeat(64),
            "addition_limit": 192,
            "performed_additions": 0,
            "acknowledged_additions": 0,
            "acknowledged": [],
            "unacknowledged": null
        });
        let check = |bytes: &[u8]| {
            verify_signed_replay(
                bytes,
                &original,
                &binding,
                &"8".repeat(64),
                &"9".repeat(64),
                &"a".repeat(64),
                192,
            )
        };
        let empty = check(&serde_json::to_vec(&original_value).unwrap()).unwrap();
        assert!(!empty.all_tuples_acknowledged);
        for mutation in 0..9 {
            let mut v = original_value.clone();
            match mutation {
                0 => v["schema"] = serde_json::json!("wrong"),
                1 => v["binding"]["cell_index"] = serde_json::json!(1),
                2 => v["public_sha256"] = serde_json::json!("b".repeat(64)),
                3 => v["factor_base_sha256"] = serde_json::json!("b".repeat(64)),
                4 => v["input_sha256"] = serde_json::json!("b".repeat(64)),
                5 => v["addition_limit"] = serde_json::json!(193),
                6 => v["performed_additions"] = serde_json::json!(1),
                7 => v["acknowledged_additions"] = serde_json::json!(1),
                8 => v["negative"] = serde_json::json!(true),
                _ => unreachable!(),
            }
            assert!(
                check(&serde_json::to_vec(&v).unwrap()).is_err(),
                "mutation {mutation}"
            );
        }
        assert!(check(&vec![b' '; SIGNED_REPLAY_BYTE_CAP + 1]).is_err());
    }
}
/// Checked mathematical candidate. Durable custody is a separate publication.
pub(super) struct CheckedSignedWitness {
    indices: [u8; 3],
    points: [(u64, u64); 3],
    target: (u64, u64),
}

pub(super) struct SignedEnumeration {
    pub acknowledged: Vec<SignedPrefix>,
    pub unacknowledged: Option<SignedPrefix>,
    pub enumeration_additions: u64,
    pub witness_check_additions: u64,
    pub first_receipt: Option<crate::ml_dataset::capture::WitnessReceipt>,
    pub stop: Option<Stop>,
    pub complete: bool,
}

/// Stage-only producer. The caller must admit field setup and generic-check
/// scratch before entering, and supply trusted capture callbacks which charge
/// serialization, retention and transport work to this same ledger. Runtime
/// activation additionally requires a bound-capture bridge, not arbitrary
/// caller receipts. Counts below are actual addition API calls, including O+P.
fn enumerate_signed(
    original: &MathInput,
    curve: &crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve,
    budget: &mut Budget,
    matrix_required: bool,
    event_sink: &mut impl FnMut(&SignedPrefix, &mut Budget) -> Result<(), Stop>,
    publish: &mut impl FnMut(
        &CheckedSignedWitness,
        &mut Budget,
    ) -> Result<crate::ml_dataset::capture::WitnessReceipt, Stop>,
) -> SignedEnumeration {
    use crate::binary_ecc::{curve::point_add, BinaryPoint, F2mElement};
    use num_traits::ToPrimitive;
    let mut result = SignedEnumeration {
        acknowledged: Vec::new(),
        unacknowledged: None,
        enumeration_additions: 0,
        witness_check_additions: 0,
        first_receipt: None,
        stop: None,
        complete: false,
    };
    let outcome = (|| -> Result<(), Stop> {
        budget.begin_extraction(matrix_required, Instant::now())?;
        // Check prepared-kernel metadata against the original curve. This is
        // not authentication of its private reduction table: runtime entry must
        // use an opaque original-bound setup token rather than a caller kernel.
        let modulus = original
            .curve
            .irreducible
            .low_terms
            .iter()
            .fold(1u64 << original.curve.m, |bits, &term| {
                bits | (1u64 << term)
            });
        if curve.n != original.curve.m
            || curve.gf.n != original.curve.m
            || curve.gf.irr != modulus
            || Some(curve.a) != original.curve.a.to_biguint().to_u64()
        {
            return budget.fail(Stop::InvalidLimits);
        }
        budget.reserve_bytes(
            (192 * std::mem::size_of::<SignedPrefix>()) as u64,
            Instant::now(),
        )?;
        result
            .acknowledged
            .try_reserve_exact(192)
            .map_err(|_| Stop::Allocation)?;
        for tuple in 0..64usize {
            let indices = [tuple / 16, (tuple / 4) % 4, tuple % 4];
            let mut prefix = None;
            for summand in 0..3 {
                budget.reserve_addition(Instant::now())?;
                prefix = curve.add(prefix, Some(original.signed[indices[summand]]));
                result.enumeration_additions += 1;
                let event = SignedPrefix {
                    sequence: (result.enumeration_additions - 1) as u16,
                    tuple: tuple as u8,
                    indices: indices.map(|i| i as u8),
                    summand: summand as u8,
                    prefix,
                    matches_target: summand == 2 && prefix == Some(original.target),
                };
                result.unacknowledged = Some(event.clone());
                budget.check(Instant::now())?;
                if event.matches_target && result.first_receipt.is_none() {
                    let points = indices.map(|i| original.signed[i]);
                    let mut reference = BinaryPoint::Infinity;
                    for (x, y) in points {
                        budget.reserve_bytes(
                            reference_add_requested_bytes(original.curve.m)?,
                            Instant::now(),
                        )?;
                        let point = BinaryPoint::Affine {
                            x: F2mElement::from_words(&[x], original.curve.m),
                            y: F2mElement::from_words(&[y], original.curve.m),
                        };
                        budget.reserve_addition(Instant::now())?;
                        reference = point_add(&original.curve, &reference, &point);
                        result.witness_check_additions += 1;
                        budget.check(Instant::now())?;
                    }
                    let expected = match reference {
                        BinaryPoint::Infinity => None,
                        BinaryPoint::Affine { x, y } => {
                            x.to_biguint().to_u64().zip(y.to_biguint().to_u64())
                        }
                    };
                    if expected != Some(original.target) {
                        return budget.fail(Stop::NativeFailure);
                    }
                    let checked = CheckedSignedWitness {
                        indices: indices.map(|i| i as u8),
                        points,
                        target: original.target,
                    };
                    // Retain a returned durable receipt BEFORE subsequent checks.
                    result.first_receipt = Some(publish(&checked, budget)?);
                    budget.check(Instant::now())?;
                }
                event_sink(&event, budget)?;
                // Sink acknowledgement is a fact even if it consumed the final
                // deadline or swallowed a failed reservation. Preserve it first.
                result.acknowledged.push(event);
                result.unacknowledged = None;
                budget.check(Instant::now())?;
            }
        }
        budget.finish_extraction(Instant::now())?;
        result.complete = true;
        Ok(())
    })();
    if let Err(stop) = outcome {
        result.stop = Some(*budget.stopped.get_or_insert(stop));
    }
    result
}

#[cfg(test)]
mod signed_producer_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve;
    use crate::ml_dataset::capture::{WitnessCommit, WitnessProvenance, WitnessReceipt};
    use num_traits::ToPrimitive;
    fn prepared(original: &MathInput, additions: u64) -> (FastBinaryCurve, Budget) {
        let curve = FastBinaryCurve::new(
            &original.curve.irreducible,
            original.curve.a.to_biguint().to_u64().unwrap(),
        )
        .unwrap();
        let now = Instant::now();
        let mut b = Budget::new(
            additions,
            1 << 20,
            1 << 20,
            now + Duration::from_secs(20),
            now,
        )
        .unwrap();
        b.begin_native(false, now).unwrap();
        b.finish_native(now).unwrap();
        (curve, b)
    }
    fn receipt() -> WitnessReceipt {
        WitnessReceipt {
            commit: WitnessCommit::NewCommit,
            payload_sha256: [1; 32],
            payload_bytes: 128,
            sequence: Some(1),
            provenance: WitnessProvenance::Dedicated,
        }
    }
    #[test]
    fn signed_producer_exact_positive_budget_publishes_only_once() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        for allowance in [194, 195] {
            let (curve, mut b) = prepared(&original, allowance);
            let mut published = 0;
            let r = enumerate_signed(
                &original,
                &curve,
                &mut b,
                false,
                &mut |_, _| Ok(()),
                &mut |_, _| {
                    published += 1;
                    Ok(receipt())
                },
            );
            assert_eq!(published, 1);
            assert_eq!(r.first_receipt, Some(receipt()));
            assert_eq!(r.witness_check_additions, 3);
            assert_eq!(r.enumeration_additions, allowance - 3);
            assert_eq!(b.additions, allowance);
            assert_eq!(r.complete, allowance == 195);
            assert_eq!(
                r.stop,
                if allowance == 195 {
                    None
                } else {
                    Some(Stop::Additions)
                }
            );
        }
    }
    #[test]
    fn signed_producer_retains_receipt_after_publisher_consumes_deadline() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let (curve, mut b) = prepared(&original, 195);
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |_, _| Ok(()),
            &mut |_, b| {
                b.deadline = Instant::now();
                Ok(receipt())
            },
        );
        assert_eq!(r.first_receipt, Some(receipt()));
        assert_eq!(r.stop, Some(Stop::Deadline));
        assert!(r.unacknowledged.as_ref().unwrap().matches_target);
        assert!(!r.complete);
    }
    #[test]
    fn signed_producer_exact_and_one_less_negative_allowance() {
        let original = construction_tests::fixture();
        for allowance in [191, 192] {
            let (curve, mut b) = prepared(&original, allowance);
            let r = enumerate_signed(
                &original,
                &curve,
                &mut b,
                false,
                &mut |_, _| Ok(()),
                &mut |_, _| panic!("fixed negative control unexpectedly matched"),
            );
            assert_eq!(r.enumeration_additions, allowance);
            assert_eq!(r.witness_check_additions, 0);
            assert_eq!(b.additions, allowance);
            assert_eq!(r.complete, allowance == 192);
            assert_eq!(
                r.stop,
                if allowance == 192 {
                    None
                } else {
                    Some(Stop::Additions)
                }
            );
            let replay =
                verify_signed_prefix(&original, &r.acknowledged, r.unacknowledged.as_ref())
                    .unwrap();
            assert_eq!(replay.all_tuples_acknowledged, allowance == 192);
            assert!(replay.candidate_tuples.is_empty());
        }
    }
    #[test]
    fn signed_producer_retains_first_receipt_when_match_event_is_refused() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let (curve, mut b) = prepared(&original, 195);
        let mut published = 0;
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |e, _| {
                if e.matches_target {
                    Err(Stop::Observer)
                } else {
                    Ok(())
                }
            },
            &mut |w, _| {
                assert_eq!(w.target, original.target);
                assert_eq!(w.points, w.indices.map(|i| original.signed[i as usize]));
                published += 1;
                Ok(receipt())
            },
        );
        assert_eq!(published, 1);
        assert_eq!(r.first_receipt, Some(receipt()));
        assert_eq!(r.witness_check_additions, 3);
        assert_eq!(r.stop, Some(Stop::Observer));
        assert!(!r.complete);
        assert!(r.unacknowledged.as_ref().unwrap().matches_target);
        assert_eq!(
            b.additions,
            r.enumeration_additions + r.witness_check_additions
        );
        verify_signed_prefix(&original, &r.acknowledged, r.unacknowledged.as_ref()).unwrap();
    }
    #[test]
    fn signed_producer_keeps_acknowledgement_after_swallowed_sink_budget_failure() {
        let original = construction_tests::fixture();
        let (curve, mut b) = prepared(&original, 192);
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |_, b| {
                assert_eq!(b.reserve_trace(u64::MAX, Instant::now()), Err(Stop::Trace));
                Ok(())
            },
            &mut |_, _| panic!("unexpected witness"),
        );
        assert_eq!(r.acknowledged.len(), 1);
        assert!(r.unacknowledged.is_none());
        assert_eq!(r.stop, Some(Stop::Trace));
        assert!(!r.complete);
    }
    #[test]
    fn signed_producer_rejects_wrong_prepared_kernel_before_any_addition() {
        let original = construction_tests::fixture();
        let (mut curve, mut b) = prepared(&original, 192);
        curve.a ^= 1;
        let r = enumerate_signed(
            &original,
            &curve,
            &mut b,
            false,
            &mut |_, _| panic!("unexpected event"),
            &mut |_, _| panic!("unexpected witness"),
        );
        assert_eq!(r.stop, Some(Stop::InvalidLimits));
        assert_eq!(b.additions, 0);
    }
}
/// Owns a kernel built from validated originals and exclusively borrows the
/// same attempt ledger until enumeration finishes. No caller-selected kernel.
pub(super) struct PreparedSigned<'a> {
    original: &'a MathInput,
    curve: crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve,
    budget: &'a mut Budget,
    matrix_required: bool,
}

/// Conservative requested dynamic storage for Gf2::reduction_table: `pow`
/// contains rows*8 words; `red` contains rows*256 words. Reserve a second
/// red-sized block for a possible Vec-to-box storage move. A folding kernel may
/// use less. No refund is made. This excludes allocator overhead and is not RSS.
fn signed_setup_bytes(n: u32) -> Result<u64, Stop> {
    if !((23..=31).contains(&n) || (47..=61).contains(&n)) {
        return Err(Stop::InvalidLimits);
    }
    let rows = u64::from(n - 1).div_ceil(8);
    Ok(rows * (8 + 2 * 256) * 8
        + std::mem::size_of::<crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve>() as u64)
}

/// Reserve setup before constructing the original-bound native kernel. Table
/// initialization has at most rows*(8+256+255) slots/entries; that bounded setup
/// loop bound is a source-derived bound, not a measured instruction count.
/// Table slot/entry work and requested setup bytes are pre-admitted. Generic
/// reference-check scratch admission remains an activation dependency.
pub(super) fn prepare_signed<'a>(
    original: &'a MathInput,
    budget: &'a mut Budget,
    matrix_required: bool,
) -> Result<PreparedSigned<'a>, Stop> {
    use num_traits::ToPrimitive;
    budget.check(Instant::now())?;
    let expected = if matrix_required {
        Phase::MatrixComplete
    } else {
        Phase::Constructed
    };
    if budget.phase != expected {
        return budget.fail(Stop::Phase);
    }
    let bytes = match signed_setup_bytes(original.curve.m) {
        Ok(bytes) => bytes,
        Err(stop) => return budget.fail(stop),
    };
    let a = match original.curve.a.to_biguint().to_u64() {
        Some(a) => a,
        None => return budget.fail(Stop::InvalidLimits),
    };
    let rows = u64::from(original.curve.m - 1).div_ceil(8);
    // Slot initialization and table-entry bounds; not machine instructions.
    let setup_work = rows * (8 + 2 * 256 + 255);
    budget.reserve_checking(setup_work, bytes, Instant::now())?;
    let curve = crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve::new(
        &original.curve.irreducible,
        a,
    )
    .ok_or(Stop::InvalidLimits)?;
    budget.check(Instant::now())?;
    Ok(PreparedSigned {
        original,
        curve,
        budget,
        matrix_required,
    })
}

/// Public producer entry consumes the prepared original/budget pair. The inner
/// staged driver is private; callbacks retain the original cumulative ledger.
pub(super) fn enumerate_prepared_signed(
    prepared: PreparedSigned<'_>,
    event_sink: &mut impl FnMut(&SignedPrefix, &mut Budget) -> Result<(), Stop>,
    publish: &mut impl FnMut(
        &CheckedSignedWitness,
        &mut Budget,
    ) -> Result<crate::ml_dataset::capture::WitnessReceipt, Stop>,
) -> SignedEnumeration {
    enumerate_signed(
        prepared.original,
        &prepared.curve,
        prepared.budget,
        prepared.matrix_required,
        event_sink,
        publish,
    )
}

#[cfg(test)]
mod signed_setup_tests {
    use super::*;
    fn constructed_budget(bytes: u64) -> Budget {
        let now = Instant::now();
        let mut b = Budget::new(192, bytes, 1 << 20, now + Duration::from_secs(20), now).unwrap();
        b.begin_native(false, now).unwrap();
        b.finish_native(now).unwrap();
        b
    }
    #[test]
    fn signed_setup_admits_exact_requested_bytes_and_rejects_one_less() {
        let original = construction_tests::fixture();
        let bytes = signed_setup_bytes(original.curve.m).unwrap();
        let mut short = constructed_budget(bytes - 1);
        assert!(matches!(
            prepare_signed(&original, &mut short, false),
            Err(Stop::Bytes)
        ));
        assert_eq!(short.adapter_bytes, 0);
        let mut exact = constructed_budget(bytes);
        let prepared = prepare_signed(&original, &mut exact, false).unwrap();
        assert_eq!(prepared.budget.adapter_bytes, bytes);
        assert_eq!(prepared.curve.n, original.curve.m);
        assert_eq!(prepared.budget.additions, 0);
    }
    #[test]
    fn signed_setup_keeps_prior_reservations_and_requires_requested_matrix() {
        let original = construction_tests::fixture();
        let mut missing = constructed_budget(1 << 20);
        assert!(matches!(
            prepare_signed(&original, &mut missing, true),
            Err(Stop::Phase)
        ));
        let bytes = signed_setup_bytes(original.curve.m).unwrap();
        let mut b = constructed_budget(bytes + 100);
        b.reserve_bytes(101, Instant::now()).unwrap();
        assert!(matches!(
            prepare_signed(&original, &mut b, false),
            Err(Stop::Bytes)
        ));
        assert_eq!(b.adapter_bytes, 101);
    }
    #[test]
    fn signed_setup_token_runs_original_domain_and_same_addition_ledger() {
        let original = construction_tests::fixture();
        let mut b = constructed_budget(1 << 20);
        let prepared = prepare_signed(&original, &mut b, false).unwrap();
        let r = enumerate_prepared_signed(prepared, &mut |_, _| Ok(()), &mut |_, _| {
            panic!("fixed negative control unexpectedly matched")
        });
        assert!(r.complete);
        assert_eq!(r.enumeration_additions, 192);
        assert_eq!(b.additions, 192);
        assert_eq!(
            b.adapter_bytes,
            signed_setup_bytes(original.curve.m).unwrap()
                + (192 * std::mem::size_of::<SignedPrefix>()) as u64
        );
    }
}
/// Cumulative capture checking reservations. Constructed only with the original
/// deadline; unused supply is closed between calls, never reset or refunded.
pub(super) struct CaptureChecking {
    inner: crate::ml_dataset::capture::WitnessBudget,
}
pub(super) struct CaptureCall<T> {
    pub value: Option<T>,
    pub error: Option<crate::ml_dataset::capture::WitnessError>,
    pub stop: Option<Stop>,
}
impl Budget {
    /// Setup/capture checking and native algebra share one admitted work pool.
    /// These are reservation units, not measured machine instructions.
    fn reserve_checking(&mut self, work: u64, bytes: u64, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if work > self.native_work_limit - self.native.work - self.checking_work {
            return self.fail(Stop::Work);
        }
        if bytes > self.memory_limit - self.native.bytes - self.adapter_bytes {
            return self.fail(Stop::Bytes);
        }
        self.checking_work += work;
        self.adapter_bytes += bytes;
        Ok(())
    }
    fn capture_checking(&self) -> CaptureChecking {
        CaptureChecking {
            inner: crate::ml_dataset::capture::WitnessBudget::new(0, 0, self.deadline),
        }
    }
    /// The callback exclusively borrows capture supply, not the parent ledger.
    /// Settle reservations without a clock check on every return. Preserve a
    /// durable receipt even when the following parent check fails.
    fn capture_call<T>(
        &mut self,
        capture: &mut CaptureChecking,
        action: impl FnOnce(
            &mut crate::ml_dataset::capture::WitnessBudget,
        ) -> Result<T, crate::ml_dataset::capture::WitnessError>,
    ) -> CaptureCall<T> {
        use crate::ml_dataset::capture::WitnessError;
        if let Err(stop) = self.check(Instant::now()) {
            return CaptureCall {
                value: None,
                error: None,
                stop: Some(stop),
            };
        }
        let work = self.native_work_limit - self.native.work - self.checking_work;
        let bytes = self.memory_limit - self.native.bytes - self.adapter_bytes;
        let before = (
            capture.inner.reserved_work,
            capture.inner.performed_work,
            capture.inner.reserved_bytes,
        );
        if let Err(error) = capture.inner.set_exclusive_supply(work, bytes) {
            let stop = *self.stopped.get_or_insert(Stop::Overflow);
            return CaptureCall {
                value: None,
                error: Some(error),
                stop: Some(stop),
            };
        }
        let outcome = action(&mut capture.inner);
        capture.inner.release_exclusive_supply();
        let delta = capture
            .inner
            .reserved_work
            .checked_sub(before.0)
            .zip(capture.inner.performed_work.checked_sub(before.1))
            .zip(capture.inner.reserved_bytes.checked_sub(before.2));
        let settled = match delta {
            Some(((reserved, performed), storage))
                if reserved <= work && performed <= reserved && storage <= bytes =>
            {
                self.checking_work += reserved;
                self.checking_performed_work += performed;
                self.adapter_bytes += storage;
                true
            }
            _ => false,
        };
        let (value, error) = match outcome {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };
        if !settled {
            // A broken trusted callback cannot reopen its lent capacity.
            self.checking_work += work;
            self.adapter_bytes += bytes;
            self.stopped.get_or_insert(Stop::Nonmonotone);
        } else if let Some(error) = error {
            self.stopped.get_or_insert(match error {
                WitnessError::Deadline => Stop::Deadline,
                WitnessError::Work => Stop::Work,
                WitnessError::Storage => Stop::Bytes,
                WitnessError::Allocation => Stop::Allocation,
                _ => Stop::Observer,
            });
        }
        let stop = self.check(Instant::now()).err();
        CaptureCall { value, error, stop }
    }
}

#[cfg(test)]
mod checking_budget_tests {
    use super::*;
    use crate::ml_dataset::capture::WitnessError;
    fn budget() -> Budget {
        construction_tests::budget()
    }
    #[test]
    fn real_capture_prepare_publish_and_confirm_use_one_parent_supply() {
        use crate::ml_dataset::capture::{capture_limits, CaptureEmitter, WitnessCommit};
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "koblitz-capture-supply-{}-{nonce}",
            std::process::id()
        ));
        let mut emitter =
            CaptureEmitter::create(&dir, matrix_tests::binding(), capture_limits(16).unwrap())
                .unwrap();
        let mut b = budget();
        let mut capture = b.capture_checking();
        let prepared = b.capture_call(&mut capture, |c| emitter.prepare_witness(c));
        assert!(prepared.stop.is_none());
        assert!(prepared.error.is_none());
        let mut workspace = prepared.value.unwrap();
        let prepare_work = b.checking_work;
        let prepare_bytes = b.adapter_bytes;
        assert!(prepare_work > 0);
        assert!(prepare_bytes > 0);
        // This is a transport-accounting control, not a mathematical witness.
        let payload = br#"{"summands":[],"target":null}"#;
        let mut global = 0;
        let published = b.capture_call(&mut capture, |c| {
            emitter.publish_first_witness(payload, &mut workspace, c, &mut global)
        });
        assert!(published.stop.is_none());
        assert!(published.error.is_none());
        let receipt = published.value.unwrap();
        assert_eq!(receipt.commit, WitnessCommit::NewCommit);
        assert_eq!(global, 1);
        assert!(b.checking_work > prepare_work);
        assert_eq!(
            std::fs::read(dir.join("payloads/pdp-witness.json")).unwrap(),
            payload
        );
        let publication_work = b.checking_work;
        let confirmed = b.capture_call(&mut capture, |c| {
            emitter.publish_first_witness(payload, &mut workspace, c, &mut global)
        });
        assert!(confirmed.stop.is_none());
        let confirmation = confirmed.value.unwrap();
        assert_eq!(confirmation.commit, WitnessCommit::AlreadyCommitted);
        assert_eq!(confirmation.payload_sha256, receipt.payload_sha256);
        assert_eq!(global, 1);
        assert!(b.checking_work > publication_work);
        assert_eq!(b.checking_work, capture.inner.reserved_work);
        assert_eq!(b.checking_performed_work, capture.inner.performed_work);
        assert_eq!(b.adapter_bytes, capture.inner.reserved_bytes);
        drop(emitter);
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn checking_reservations_reduce_native_headroom_and_are_atomic() {
        let mut b = budget();
        let limit = b.native_work_limit;
        b.reserve_checking(17, 23, Instant::now()).unwrap();
        let limits = b.begin_native(false, Instant::now()).unwrap();
        assert_eq!(limits.work, limit - 17);
        b.observe(
            Reservation {
                work: limit - 17,
                bytes: 0,
            },
            Instant::now(),
        )
        .unwrap();
        assert_eq!(b.reserve_checking(1, 0, Instant::now()), Err(Stop::Work));
        assert_eq!(b.checking_work, 17);
        assert_eq!(b.adapter_bytes, 23);
    }
    #[test]
    fn capture_settles_failed_call_reservations_without_refund() {
        let mut b = budget();
        let mut capture = b.capture_checking();
        let result: CaptureCall<()> = b.capture_call(&mut capture, |c| {
            // Fault-injected accounting receipt; real emitter tests follow.
            c.reserved_work += 31;
            c.performed_work += 19;
            c.reserved_bytes += 41;
            Err(WitnessError::Io(std::io::ErrorKind::Other))
        });
        assert_eq!(result.stop, Some(Stop::Observer));
        assert!(result.error.is_some());
        assert!(result.value.is_none());
        assert_eq!(b.checking_work, 31);
        assert_eq!(b.checking_performed_work, 19);
        assert_eq!(b.adapter_bytes, 41);
        let second: CaptureCall<()> =
            b.capture_call(&mut capture, |_| panic!("stopped ledger retried"));
        assert_eq!(second.stop, Some(Stop::Observer));
    }
    #[test]
    fn capture_successes_share_cumulative_work_and_storage() {
        let mut b = budget();
        let mut capture = b.capture_checking();
        for expected in [11, 22] {
            let r = b.capture_call(&mut capture, |c| {
                c.reserved_work += 11;
                c.performed_work += 7;
                c.reserved_bytes += 5;
                Ok(42)
            });
            assert_eq!(r.value, Some(42));
            assert!(r.stop.is_none());
            assert_eq!(b.checking_work, expected);
        }
        assert_eq!(b.checking_performed_work, 14);
        assert_eq!(b.adapter_bytes, 10);
    }
    #[test]
    fn invalid_capture_delta_closes_entire_lent_supply() {
        let mut b = budget();
        let mut capture = b.capture_checking();
        let r: CaptureCall<()> = b.capture_call(&mut capture, |c| {
            c.performed_work = 1;
            Ok(())
        });
        assert_eq!(r.stop, Some(Stop::Nonmonotone));
        assert_eq!(b.checking_work, b.native_work_limit);
        assert_eq!(b.adapter_bytes, b.memory_limit);
    }
}
/// Requested-object/word reservation for one general binary point addition at
/// the admitted one-word field sizes. This is neither RSS nor compiler-frame
/// size. See the source audit stored with this increment.
fn reference_add_requested_bytes(n: u32) -> Result<u64, Stop> {
    if !((23..=31).contains(&n) || (47..=61).contains(&n)) {
        return Err(Stop::InvalidLimits);
    }
    // <=22 field-result allocations in Itoh-Tsujii, <=40 including the
    // surrounding point formulas and point construction; reserve64 layouts.
    // Six inversion bits fit capacity8; include both4/8 allocation requests.
    // with_scratch uses20 stack words. Allow final BigUint coordinate views.
    Ok(
        64 * (std::mem::size_of::<crate::binary_ecc::F2mElement>() as u64 + 8)
            + (4 + 8) * std::mem::size_of::<u32>() as u64
            + 20 * 8
            + 4 * (std::mem::size_of::<num_bigint::BigUint>() as u64 + 16),
    )
}

#[cfg(test)]
mod reference_scratch_tests {
    use super::*;
    #[test]
    fn reference_storage_is_admitted_before_independent_addition() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let now = Instant::now();
        // Admit kernel and enumeration storage exactly, leaving no reference
        // scratch. The first positive match must stop before a reference add.
        let bytes = signed_setup_bytes(original.curve.m).unwrap()
            + (192 * std::mem::size_of::<SignedPrefix>()) as u64;
        let mut b = Budget::new(16384, bytes, 1 << 20, now + Duration::from_secs(20), now).unwrap();
        b.begin_native(false, now).unwrap();
        b.finish_native(now).unwrap();
        let prepared = prepare_signed(&original, &mut b, false).unwrap();
        let result = enumerate_prepared_signed(prepared, &mut |_, _| Ok(()), &mut |_, _| {
            panic!("unadmitted reference check published")
        });
        assert_eq!(result.stop, Some(Stop::Bytes));
        assert_eq!(result.witness_check_additions, 0);
        assert!(result.unacknowledged.unwrap().matches_target);
        assert!(result.first_receipt.is_none());
    }
    #[test]
    fn reference_storage_bounds_cover_only_the_admitted_word_fields() {
        for n in [23, 31, 47, 53, 61] {
            assert!(reference_add_requested_bytes(n).unwrap() > 512);
        }
        for n in [0, 22, 32, 46, 62, 128] {
            assert_eq!(reference_add_requested_bytes(n), Err(Stop::InvalidLimits));
        }
    }
}
const SIGNED_WITNESS_BYTE_CAP: usize = 1024;

/// Decimal strings match the established binary PDP witness schema. All u64
/// formatting fits this stack buffer, with no intermediate String allocation.
#[derive(Clone, Copy)]
struct DecimalWord(u64);
impl serde::Serialize for DecimalWord {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut bytes = [b'0'; 20];
        let mut n = self.0;
        let mut start = bytes.len();
        loop {
            start -= 1;
            bytes[start] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        let text = std::str::from_utf8(&bytes[start..]).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(text)
    }
}
#[derive(Clone, Copy, serde::Serialize)]
struct DecimalPoint {
    x: DecimalWord,
    y: DecimalWord,
}
impl From<(u64, u64)> for DecimalPoint {
    fn from((x, y): (u64, u64)) -> Self {
        Self {
            x: DecimalWord(x),
            y: DecimalWord(y),
        }
    }
}
#[derive(serde::Serialize)]
struct SignedWitnessView {
    summands: [DecimalPoint; 3],
    target: DecimalPoint,
    sum: DecimalPoint,
    replay_group_add_calls: u8,
}
pub(super) struct WitnessEncoding {
    pub bytes: Vec<u8>,
    pub complete: bool,
    pub stop: Option<Stop>,
}

/// Only CheckedSignedWitness values from independent group verification may
/// enter. Original attempt identity is supplied by the bound capture sink.
/// Complete serialization is not durable publication or a completed search.
pub(super) fn encode_signed_witness(
    witness: &CheckedSignedWitness,
    budget: &mut Budget,
) -> WitnessEncoding {
    let mut bytes = Vec::new();
    // The bounded decimal/JSON traversal is pre-admitted under the shared
    // checking-work model; this is not a hardware instruction count.
    if let Err(stop) = budget.reserve_checking(
        8 * SIGNED_WITNESS_BYTE_CAP as u64,
        SIGNED_WITNESS_BYTE_CAP as u64,
        Instant::now(),
    ) {
        return WitnessEncoding {
            bytes,
            complete: false,
            stop: Some(stop),
        };
    }
    if bytes.try_reserve_exact(SIGNED_WITNESS_BYTE_CAP).is_err() {
        let stop = *budget.stopped.get_or_insert(Stop::Allocation);
        return WitnessEncoding {
            bytes,
            complete: false,
            stop: Some(stop),
        };
    }
    let view = SignedWitnessView {
        summands: witness.points.map(DecimalPoint::from),
        target: witness.target.into(),
        sum: witness.target.into(),
        replay_group_add_calls: 3,
    };
    let mut writer = InputWriter {
        bytes,
        capacity: SIGNED_WITNESS_BYTE_CAP,
        budget,
    };
    let complete = serde_json::to_writer(&mut writer, &view).is_ok();
    if !complete {
        writer.budget.stopped.get_or_insert(Stop::Serialization);
    }
    let stop = writer.budget.check(Instant::now()).err();
    WitnessEncoding {
        bytes: writer.bytes,
        complete,
        stop,
    }
}

#[cfg(test)]
mod signed_witness_encoding_tests {
    use super::*;
    #[test]
    fn decimal_words_roundtrip_zero_and_largest_machine_word_as_strings() {
        for n in [0, 1, (1u64 << 61) - 1, u64::MAX] {
            let bytes = serde_json::to_vec(&DecimalWord(n)).unwrap();
            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(value.as_str().unwrap(), n.to_string());
        }
    }
    #[test]
    fn checked_witness_payload_matches_existing_group_encoding() {
        use crate::binary_ecc::{BinaryPoint, F2mElement};
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut b = construction_tests::budget();
        b.begin_native(false, Instant::now()).unwrap();
        b.finish_native(Instant::now()).unwrap();
        let prepared = prepare_signed(&original, &mut b, false).unwrap();
        let mut saved = None;
        let result = enumerate_prepared_signed(prepared, &mut |_, _| Ok(()), &mut |w, b| {
            let encoding = encode_signed_witness(w, b);
            assert!(encoding.complete);
            assert!(encoding.stop.is_none());
            let point = |(x, y)| {
                super::super::Element::Binary(BinaryPoint::Affine {
                    x: F2mElement::from_words(&[x], original.curve.m),
                    y: F2mElement::from_words(&[y], original.curve.m),
                })
                .json()
            };
            let expected = serde_json::json!({"summands":w.points.map(point),"target":point(w.target),
                "sum":point(w.target),"replay_group_add_calls":3});
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&encoding.bytes).unwrap(),
                expected
            );
            saved = Some(encoding);
            // This control stops before publication; encoding alone is not a receipt.
            Err(Stop::Observer)
        });
        assert!(saved.unwrap().complete);
        assert!(result.first_receipt.is_none());
        assert!(!result.complete);
        assert_eq!(result.stop, Some(Stop::Observer));
    }
    #[test]
    fn witness_encoding_retains_partial_bytes_when_trace_allowance_expires() {
        let witness = CheckedSignedWitness {
            indices: [0, 0, 0],
            points: [(1, 2); 3],
            target: (3, 4),
        };
        let mut b = construction_tests::budget();
        b.trace_limit = 16;
        let result = encode_signed_witness(&witness, &mut b);
        assert!(!result.complete);
        assert_eq!(result.stop, Some(Stop::Trace));
        assert!(result.bytes.len() <= 16);
        assert!(!result.bytes.is_empty());
        assert_eq!(b.trace_bytes, result.bytes.len() as u64);
        assert_eq!(b.adapter_bytes, SIGNED_WITNESS_BYTE_CAP as u64);
    }
}
/// One publication attempt for the first independently checked signed witness.
/// Expected binding/limits are borrowed from original admission. No local event
/// counter is created: the caller supplies its adapter-wide acknowledgement count.
pub(super) struct BoundSignedPublisher<'a> {
    binding: &'a crate::ml_dataset::capture::AttemptBinding,
    limits: &'a crate::ml_dataset::capture::CaptureLimits,
    checking: CaptureChecking,
    workspace: Option<crate::ml_dataset::capture::WitnessCaptureWorkspace>,
    encoding: Option<WitnessEncoding>,
    receipt: Option<crate::ml_dataset::capture::WitnessReceipt>,
    capture_error: Option<crate::ml_dataset::capture::WitnessError>,
    attempted: bool,
}
impl<'a> BoundSignedPublisher<'a> {
    fn new(
        binding: &'a crate::ml_dataset::capture::AttemptBinding,
        limits: &'a crate::ml_dataset::capture::CaptureLimits,
        budget: &Budget,
    ) -> Self {
        Self {
            binding,
            limits,
            checking: budget.capture_checking(),
            workspace: None,
            encoding: None,
            receipt: None,
            capture_error: None,
            attempted: false,
        }
    }
    fn publish(
        &mut self,
        witness: &CheckedSignedWitness,
        budget: &mut Budget,
        global_acknowledged: &mut usize,
    ) -> Result<crate::ml_dataset::capture::WitnessReceipt, Stop> {
        if self.attempted {
            return budget.fail(Stop::Phase);
        }
        self.attempted = true;
        // Fixed-size binding validation/hash encoding and bounded identity
        // scratch are admitted before the existing sink admission helper.
        budget.reserve_checking(64 * 1024, 4096, Instant::now())?;
        let identity = super::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
            Some(super::EventSink::Bound(sink)) => {
                sink.verify_attempt_limits(self.binding, self.limits)
            }
            _ => Err("Koblitz witness requires the admitted bound sink".into()),
        });
        if identity.is_err() {
            return budget.fail(Stop::InvalidLimits);
        }
        self.encoding = Some(encode_signed_witness(witness, budget));
        let encoding = self.encoding.as_ref().expect("encoding retained");
        if !encoding.complete || encoding.stop.is_some() {
            return budget.fail(encoding.stop.unwrap_or(Stop::Serialization));
        }
        let prepared = budget.capture_call(&mut self.checking, super::prepare_bound_witness);
        self.capture_error = prepared.error;
        self.workspace = prepared.value;
        if let Some(stop) = prepared.stop {
            return Err(stop);
        }
        let workspace = self.workspace.as_mut().ok_or(Stop::Observer)?;
        let published = budget.capture_call(&mut self.checking, |supply| {
            super::publish_bound_witness(&encoding.bytes, workspace, supply, global_acknowledged)
        });
        self.capture_error = published.error;
        self.receipt = published.value;
        // Return a real acknowledged receipt even if post-call settlement/clock
        // stopped the ledger. enumerate_signed stores it before checking again.
        if let Some(receipt) = self.receipt {
            return Ok(receipt);
        }
        budget.fail(published.stop.unwrap_or(Stop::Observer))
    }
}

#[cfg(test)]
mod bound_signed_publisher_tests {
    use super::*;
    use crate::ml_dataset::capture::{capture_limits, CaptureEmitter};
    pub(super) struct SinkFixture {
        pub(super) dir: std::path::PathBuf,
        _guard: super::super::LiveSinkGuard,
    }
    impl SinkFixture {
        pub(super) fn new(binding: &crate::ml_dataset::capture::AttemptBinding) -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir = std::env::temp_dir()
                .join(format!(
                    "koblitz-bound-witness-{}-{nonce}",
                    std::process::id()
                ))
                .join("live");
            std::fs::create_dir_all(dir.parent().unwrap()).unwrap();
            let emitter =
                CaptureEmitter::create(&dir, binding.clone(), capture_limits(16).unwrap()).unwrap();
            super::super::LIVE_SINK.with(|slot| {
                assert!(slot.borrow().is_none());
                *slot.borrow_mut() = Some(super::super::EventSink::Bound(emitter));
            });
            Self {
                dir,
                _guard: super::super::LiveSinkGuard,
            }
        }
    }
    impl Drop for SinkFixture {
        fn drop(&mut self) {
            super::super::LIVE_SINK.with(|slot| {
                slot.borrow_mut().take();
            });
            let _ = std::fs::remove_dir_all(self.dir.parent().unwrap());
        }
    }
    #[test]
    fn real_checked_witness_survives_later_event_refusal_and_replays_from_disk() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = SinkFixture::new(&binding);
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut budget = construction_tests::budget();
        let mut publisher = BoundSignedPublisher::new(&binding, &limits, &budget);
        budget.begin_native(false, Instant::now()).unwrap();
        budget.finish_native(Instant::now()).unwrap();
        let prepared = prepare_signed(&original, &mut budget, false).unwrap();
        let mut global = 0;
        let result = enumerate_prepared_signed(
            prepared,
            &mut |event, _| {
                if event.matches_target {
                    Err(Stop::Observer)
                } else {
                    Ok(())
                }
            },
            &mut |w, b| publisher.publish(w, b, &mut global),
        );
        assert_eq!(result.stop, Some(Stop::Observer));
        assert!(!result.complete);
        let receipt = result.first_receipt.unwrap();
        assert_eq!(publisher.receipt, Some(receipt));
        assert_eq!(global, 1);
        assert_eq!(result.witness_check_additions, 3);
        let bytes = std::fs::read(sink.dir.join("payloads/pdp-witness.json")).unwrap();
        assert_eq!(bytes, publisher.encoding.as_ref().unwrap().bytes);
        assert_eq!(bytes.len() as u64, receipt.payload_bytes);
        let digest = crate::ml_dataset::digest(&bytes);
        let expected_hex = receipt
            .payload_sha256
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        assert_eq!(digest, expected_hex);
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let group = super::super::Group::Binary(original.curve.clone());
        let mut sum = group.zero();
        for p in value["summands"].as_array().unwrap() {
            let point = group.point(p).unwrap();
            assert!(group.on_curve(&point));
            assert!(original
                .signed
                .iter()
                .any(|&(x, y)| p["x"] == x.to_string() && p["y"] == y.to_string()));
            sum = group.add(&sum, &point);
        }
        assert_eq!(sum, group.point(&value["target"]).unwrap());
        assert_eq!(sum, group.point(&value["sum"]).unwrap());
        assert!(publisher.capture_error.is_none());
    }
    #[test]
    fn bound_witness_rejects_wrong_attempt_before_payload_or_acknowledgement() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = SinkFixture::new(&binding);
        let mut wrong = binding.clone();
        wrong.cell_index += 1;
        let mut b = construction_tests::budget();
        let mut publisher = BoundSignedPublisher::new(&wrong, &limits, &b);
        let witness = CheckedSignedWitness {
            indices: [0; 3],
            points: [(1, 2); 3],
            target: (3, 4),
        };
        let mut global = 0;
        assert_eq!(
            publisher.publish(&witness, &mut b, &mut global),
            Err(Stop::InvalidLimits)
        );
        assert_eq!(global, 0);
        assert!(publisher.encoding.is_none());
        assert!(publisher.receipt.is_none());
        assert!(!sink.dir.join("payloads/pdp-witness.json").exists());
    }
    #[test]
    fn bound_witness_refuses_unbound_execution() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let mut b = construction_tests::budget();
        let mut publisher = BoundSignedPublisher::new(&binding, &limits, &b);
        let witness = CheckedSignedWitness {
            indices: [0; 3],
            points: [(1, 2); 3],
            target: (3, 4),
        };
        assert_eq!(
            publisher.publish(&witness, &mut b, &mut 0),
            Err(Stop::InvalidLimits)
        );
        assert!(publisher.receipt.is_none());
    }
    #[test]
    fn bound_witness_keeps_encoding_when_workspace_storage_is_refused() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = SinkFixture::new(&binding);
        let mut b = construction_tests::budget();
        b.memory_limit = 4096 + SIGNED_WITNESS_BYTE_CAP as u64;
        let mut publisher = BoundSignedPublisher::new(&binding, &limits, &b);
        // Transport refusal control; no mathematical certificate is claimed.
        let witness = CheckedSignedWitness {
            indices: [0; 3],
            points: [(1, 2); 3],
            target: (3, 4),
        };
        let mut global = 0;
        assert_eq!(
            publisher.publish(&witness, &mut b, &mut global),
            Err(Stop::Bytes)
        );
        assert!(publisher.encoding.as_ref().unwrap().complete);
        assert_eq!(
            publisher.capture_error,
            Some(crate::ml_dataset::capture::WitnessError::Storage)
        );
        assert!(publisher.receipt.is_none());
        assert_eq!(global, 0);
        assert!(!sink.dir.join("payloads/pdp-witness.json").exists());
    }
}
const SIGNED_EVENT_CAP: u64 = 2048;
#[derive(serde::Serialize)]
struct SignedEventView<'a> {
    phase: &'static str,
    progress: &'a SignedPrefix,
}
pub(super) struct BoundSignedEvents<'a> {
    binding_digest: [u8; 64],
    limits: &'a crate::ml_dataset::capture::CaptureLimits,
    buffer: crate::ml_dataset::capture::TypedEventBuffer,
    acknowledged: u64,
    encoded_bytes: u64,
}
impl<'a> BoundSignedEvents<'a> {
    fn new(
        binding: &'a crate::ml_dataset::capture::AttemptBinding,
        limits: &'a crate::ml_dataset::capture::CaptureLimits,
        budget: &mut Budget,
    ) -> Result<Self, Stop> {
        use crate::ml_dataset::capture::TypedEventBuffer;
        budget.reserve_checking(64 * 1024, 4096, Instant::now())?;
        if binding.validate().is_err() || limits.validate().is_err() {
            return budget.fail(Stop::InvalidLimits);
        }
        let digest = binding.digest().map_err(|_| Stop::InvalidLimits)?;
        let mut binding_digest = [0u8; 64];
        binding_digest.copy_from_slice(digest.as_bytes());
        let bytes =
            TypedEventBuffer::requested_bytes(SIGNED_EVENT_CAP).map_err(|_| Stop::InvalidLimits)?;
        budget.reserve_bytes(bytes, Instant::now())?;
        let buffer =
            TypedEventBuffer::try_new(SIGNED_EVENT_CAP, bytes).map_err(|_| Stop::Allocation)?;
        Ok(Self {
            binding_digest,
            limits,
            buffer,
            acknowledged: 0,
            encoded_bytes: 0,
        })
    }
    fn emit(
        &mut self,
        event: &SignedPrefix,
        budget: &mut Budget,
        global: &mut usize,
    ) -> Result<(), Stop> {
        if *global >= 4096 {
            return budget.fail(Stop::Events);
        }
        // Cached identity comparison plus bounded two-pass encoding. Work/trace
        // reservations are explicit model bounds; actual encoded bytes below
        // are recorded only after the emitter acknowledges the event.
        budget.reserve_checking(1024 + 8 * SIGNED_EVENT_CAP, 4096, Instant::now())?;
        budget.reserve_trace(SIGNED_EVENT_CAP, Instant::now())?;
        let identity = super::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
            Some(super::EventSink::Bound(sink)) => {
                sink.verify_digest_limits(&self.binding_digest, self.limits)
            }
            _ => Err("Koblitz typed event requires admitted bound sink".into()),
        });
        if identity.is_err() {
            return budget.fail(Stop::InvalidLimits);
        }
        let view = SignedEventView {
            phase: "signed-domain",
            progress: event,
        };
        if super::live_typed_event(
            crate::ml_dataset::capture::TypedEventKind::KoblitzM3,
            &view,
            &mut self.buffer,
        )
        .is_err()
        {
            return budget.fail(Stop::Observer);
        }
        *global += 1;
        self.acknowledged += 1;
        self.encoded_bytes += self.buffer.encoded_len() as u64;
        // The call acknowledged; a subsequent deadline remains sticky but must
        // not make the producer misclassify this event as unacknowledged.
        let _ = budget.check(Instant::now());
        Ok(())
    }
}

#[cfg(test)]
mod signed_typed_event_tests {
    use super::*;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn real_signed_events_and_first_witness_share_one_acknowledgement_counter() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut budget = construction_tests::budget();
        let mut events = BoundSignedEvents::new(&binding, &limits, &mut budget).unwrap();
        let mut publisher = BoundSignedPublisher::new(&binding, &limits, &budget);
        budget.begin_native(false, Instant::now()).unwrap();
        budget.finish_native(Instant::now()).unwrap();
        let prepared = prepare_signed(&original, &mut budget, false).unwrap();
        let global = std::cell::RefCell::new(0usize);
        let result = enumerate_prepared_signed(
            prepared,
            &mut |event, b| events.emit(event, b, &mut global.borrow_mut()),
            &mut |w, b| publisher.publish(w, b, &mut global.borrow_mut()),
        );
        assert!(result.complete, "{:?}", result.stop);
        assert!(result.first_receipt.is_some());
        assert_eq!(*global.borrow(), 193);
        assert_eq!(events.acknowledged, 192);
        let stream = std::fs::read_to_string(sink.dir.join("events.jsonl")).unwrap();
        let rows = stream
            .lines()
            .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 193);
        let mut encoded = 0usize;
        let mut signed = Vec::new();
        for (i, (row, line)) in rows.iter().zip(stream.lines()).enumerate() {
            assert_eq!(row["sequence"], (i + 1) as u64);
            if row["event"]["kind"] == "koblitz_m3" {
                assert_eq!(row["event"]["payload"]["phase"], "signed-domain");
                signed.push(
                    serde_json::from_value::<SignedPrefix>(
                        row["event"]["payload"]["progress"].clone(),
                    )
                    .unwrap(),
                );
                encoded += line.len() + 1;
            } else {
                assert_eq!(row["event"]["kind"], "capture_payload_committed");
            }
        }
        assert_eq!(signed, result.acknowledged);
        let replay = SavedSignedReplay {
            schema: "crypto.ml-dataset.koblitz-m3-signed-prefix/v1".into(),
            binding: binding.clone(),
            public_sha256: "8".repeat(64),
            factor_base_sha256: "9".repeat(64),
            input_sha256: "a".repeat(64),
            addition_limit: 16384,
            performed_additions: result.enumeration_additions as usize,
            acknowledged_additions: result.acknowledged.len(),
            acknowledged: result.acknowledged.clone(),
            unacknowledged: result.unacknowledged.clone(),
        };
        let replay_bytes = serde_json::to_vec(&replay).unwrap();
        let custody =
            verify_signed_capture(sink.dir.parent().unwrap(), &replay_bytes, &binding, &limits)
                .unwrap();
        assert_eq!(custody.payloads.len(), 1);
        let mut altered: serde_json::Value = serde_json::from_slice(&replay_bytes).unwrap();
        altered["acknowledged"].as_array_mut().unwrap().pop();
        assert!(verify_signed_capture(
            sink.dir.parent().unwrap(),
            &serde_json::to_vec(&altered).unwrap(),
            &binding,
            &limits
        )
        .is_err());
        // Even after all event callbacks match, a changed payload invalidates custody.
        let payload_path = sink.dir.join("payloads/pdp-witness.json");
        let original_payload = std::fs::read(&payload_path).unwrap();
        let mut damaged = original_payload.clone();
        damaged[0] ^= 1;
        std::fs::write(&payload_path, damaged).unwrap();
        assert!(verify_signed_capture(
            sink.dir.parent().unwrap(),
            &replay_bytes,
            &binding,
            &limits
        )
        .is_err());
        std::fs::write(&payload_path, original_payload).unwrap();
        assert_eq!(events.encoded_bytes, encoded as u64);
        assert!(
            verify_signed_prefix(&original, &signed, None)
                .unwrap()
                .all_tuples_acknowledged
        );
    }
    #[test]
    fn signed_event_global_cap_refuses_before_io_or_new_work() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut b = construction_tests::budget();
        let mut events = BoundSignedEvents::new(&binding, &limits, &mut b).unwrap();
        let event = SignedPrefix {
            sequence: 0,
            tuple: 0,
            indices: [0; 3],
            summand: 0,
            prefix: None,
            matches_target: false,
        };
        let mut global = 4096;
        let before = b.checking_work;
        assert_eq!(events.emit(&event, &mut b, &mut global), Err(Stop::Events));
        assert_eq!(b.checking_work, before);
        assert_eq!(global, 4096);
        assert_eq!(events.acknowledged, 0);
        assert!(std::fs::read(sink.dir.join("events.jsonl"))
            .unwrap()
            .is_empty());
    }
    #[test]
    fn signed_events_reject_wrong_bound_sink_without_acknowledgement() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut wrong = binding.clone();
        wrong.cell_index += 1;
        let mut b = construction_tests::budget();
        let mut events = BoundSignedEvents::new(&wrong, &limits, &mut b).unwrap();
        let event = SignedPrefix {
            sequence: 0,
            tuple: 0,
            indices: [0; 3],
            summand: 0,
            prefix: None,
            matches_target: false,
        };
        let mut global = 0;
        assert_eq!(
            events.emit(&event, &mut b, &mut global),
            Err(Stop::InvalidLimits)
        );
        assert_eq!(global, 0);
        assert!(std::fs::read(sink.dir.join("events.jsonl"))
            .unwrap()
            .is_empty());
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedWitnessPoint {
    x: String,
    y: String,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedSignedWitness {
    summands: [SavedWitnessPoint; 3],
    target: SavedWitnessPoint,
    sum: SavedWitnessPoint,
    replay_group_add_calls: u8,
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedSignedWitness {
    indices: [u8; 3],
    tuple: u8,
}
fn saved_witness_point(point: &SavedWitnessPoint) -> Result<(u64, u64), String> {
    let word = |s: &str| -> Result<u64, String> {
        if s.is_empty()
            || s.len() > 20
            || !s.bytes().all(|b| b.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return Err("noncanonical signed-witness coordinate".into());
        }
        s.parse()
            .map_err(|_| "signed-witness coordinate exceeds word".into())
    };
    Ok((word(&point.x)?, word(&point.y)?))
}
/// Independently replay saved witness bytes against validated original inputs.
/// The caller separately authenticates the artifact's path, role, encoding,
/// digest and capture commitment. This proves mathematics, not execution cost.
pub(super) fn verify_signed_witness(
    bytes: &[u8],
    original: &MathInput,
) -> Result<VerifiedSignedWitness, String> {
    use crate::binary_ecc::{curve::point_add, BinaryPoint, F2mElement};
    use num_traits::ToPrimitive;
    if bytes.len() > SIGNED_WITNESS_BYTE_CAP {
        return Err("signed witness exceeds byte cap".into());
    }
    let saved: SavedSignedWitness =
        serde_json::from_slice(bytes).map_err(|e| format!("strict signed witness: {e}"))?;
    if saved.replay_group_add_calls != 3
        || saved_witness_point(&saved.target)? != original.target
        || saved_witness_point(&saved.sum)? != original.target
    {
        return Err("signed witness target, sum or arity mismatch".into());
    }
    let mut indices = [0u8; 3];
    let mut sum = BinaryPoint::Infinity;
    for (i, p) in saved.summands.iter().enumerate() {
        let point = saved_witness_point(p)?;
        indices[i] = original
            .signed
            .iter()
            .position(|&v| v == point)
            .ok_or("signed witness summand outside original domain")? as u8;
        let (x, y) = original.signed[indices[i] as usize];
        let p = BinaryPoint::Affine {
            x: F2mElement::from_words(&[x], original.curve.m),
            y: F2mElement::from_words(&[y], original.curve.m),
        };
        sum = point_add(&original.curve, &sum, &p);
    }
    let actual = match sum {
        BinaryPoint::Infinity => None,
        BinaryPoint::Affine { x, y } => x.to_biguint().to_u64().zip(y.to_biguint().to_u64()),
    };
    if actual != Some(original.target) {
        return Err("signed witness independent sum mismatch".into());
    }
    Ok(VerifiedSignedWitness {
        indices,
        tuple: indices[0] * 16 + indices[1] * 4 + indices[2],
    })
}

/// Mathematical scope only. Durable receipt/stream matching and outer
/// supervisor outcome checks must be supplied by the central artifact verifier.
pub(super) struct VerifiedSignedMath {
    prefix: PrefixAssessment,
    witness: Option<VerifiedSignedWitness>,
}
impl VerifiedSignedMath {
    fn negative_domain_verified(&self) -> bool {
        self.prefix.all_tuples_acknowledged
            && self.prefix.candidate_tuples.is_empty()
            && self.witness.is_none()
    }
}
pub(super) fn verify_signed_math(
    replay_bytes: &[u8],
    witness_bytes: Option<&[u8]>,
    original: &MathInput,
    binding: &crate::ml_dataset::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    input_sha256: &str,
    addition_limit: u64,
) -> Result<VerifiedSignedMath, String> {
    let prefix = verify_signed_replay(
        replay_bytes,
        original,
        binding,
        public_sha256,
        factor_base_sha256,
        input_sha256,
        addition_limit,
    )?;
    let witness = witness_bytes
        .map(|bytes| verify_signed_witness(bytes, original))
        .transpose()?;
    if let Some(witness) = &witness {
        if !prefix.candidate_tuples.contains(&witness.tuple) {
            return Err("saved witness is absent from the replayed signed prefix".into());
        }
    }
    Ok(VerifiedSignedMath { prefix, witness })
}

#[cfg(test)]
mod signed_witness_replay_tests {
    use super::*;
    fn witness() -> (MathInput, Vec<u8>, [u8; 3]) {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut budget = construction_tests::budget();
        budget.begin_native(false, Instant::now()).unwrap();
        budget.finish_native(Instant::now()).unwrap();
        let prepared = prepare_signed(&original, &mut budget, false).unwrap();
        let mut saved = None;
        let result = enumerate_prepared_signed(prepared, &mut |_, _| Ok(()), &mut |w, b| {
            let e = encode_signed_witness(w, b);
            assert!(e.complete && e.stop.is_none());
            saved = Some((e.bytes, w.indices));
            Err(Stop::Observer)
        });
        assert!(result.first_receipt.is_none());
        let (bytes, indices) = saved.unwrap();
        (original, bytes, indices)
    }
    #[test]
    fn saved_signed_witness_is_verified_without_producer_receipt_or_live_deadline() {
        let (original, bytes, indices) = witness();
        let v = verify_signed_witness(&bytes, &original).unwrap();
        assert_eq!(v.indices, indices);
        assert_eq!(v.tuple, indices[0] * 16 + indices[1] * 4 + indices[2]);
    }
    #[test]
    fn signed_witness_replay_rejects_reencoded_semantic_and_schema_mutations() {
        let (original, bytes, _) = witness();
        let base: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for mutation in 0..11 {
            let mut v = base.clone();
            match mutation {
                0 => {
                    v["target"]["x"] = serde_json::json!((base["target"]["x"]
                        .as_str()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                        ^ 1)
                    .to_string())
                }
                1 => {
                    v["sum"]["y"] = serde_json::json!((base["sum"]["y"]
                        .as_str()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                        ^ 1)
                    .to_string())
                }
                2 => v["summands"][0]["x"] = serde_json::json!(u64::MAX.to_string()),
                3 => v["replay_group_add_calls"] = serde_json::json!(2),
                4 => v["summands"][0]["x"] = serde_json::json!(1),
                5 => {
                    v["summands"][0]["x"] = serde_json::json!(format!(
                        "0{}",
                        base["summands"][0]["x"].as_str().unwrap()
                    ))
                }
                6 => v["solver_completed"] = serde_json::json!(true),
                7 => v["summands"][0]["extra"] = serde_json::json!(true),
                8 => {
                    v["summands"].as_array_mut().unwrap().pop();
                }
                9 => v["summands"][0] = serde_json::Value::Null,
                10 => v["summands"][0]["x"] = serde_json::json!("18446744073709551616"),
                _ => unreachable!(),
            }
            assert!(
                verify_signed_witness(&serde_json::to_vec(&v).unwrap(), &original).is_err(),
                "mutation {mutation}"
            );
        }
        assert!(
            verify_signed_witness(&vec![b' '; SIGNED_WITNESS_BYTE_CAP + 1], &original).is_err()
        );
    }
    #[test]
    fn signed_witness_replay_rejects_on_domain_points_with_wrong_group_sum() {
        let (original, bytes, _) = witness();
        let mut v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let p = original.signed[0];
        v["summands"] = serde_json::json!([
            {"x":p.0.to_string(),"y":p.1.to_string()},
            {"x":p.0.to_string(),"y":p.1.to_string()},
            {"x":p.0.to_string(),"y":p.1.to_string()}]);
        assert!(
            verify_signed_witness(&serde_json::to_vec(&v).unwrap(), &original)
                .unwrap_err()
                .contains("sum mismatch")
        );
    }
}

#[cfg(test)]
mod signed_math_pair_tests {
    use super::*;
    fn replay(original: &MathInput, positive: bool) -> (Vec<u8>, Option<Vec<u8>>) {
        let mut b = construction_tests::budget();
        b.begin_native(false, Instant::now()).unwrap();
        b.finish_native(Instant::now()).unwrap();
        let prepared = prepare_signed(original, &mut b, false).unwrap();
        let mut witness = None;
        let result = enumerate_prepared_signed(prepared, &mut |_, _| Ok(()), &mut |w, b| {
            assert!(positive);
            let e = encode_signed_witness(w, b);
            assert!(e.complete && e.stop.is_none());
            witness = Some(e.bytes);
            Err(Stop::Observer)
        });
        assert_eq!(result.complete, !positive);
        let saved = SavedSignedReplay {
            schema: "crypto.ml-dataset.koblitz-m3-signed-prefix/v1".into(),
            binding: matrix_tests::binding(),
            public_sha256: "8".repeat(64),
            factor_base_sha256: "9".repeat(64),
            input_sha256: "a".repeat(64),
            addition_limit: 16384,
            performed_additions: result.enumeration_additions as usize,
            acknowledged_additions: result.acknowledged.len(),
            acknowledged: result.acknowledged,
            unacknowledged: result.unacknowledged,
        };
        (serde_json::to_vec(&saved).unwrap(), witness)
    }
    fn check(
        original: &MathInput,
        prefix: &[u8],
        witness: Option<&[u8]>,
    ) -> Result<VerifiedSignedMath, String> {
        verify_signed_math(
            prefix,
            witness,
            original,
            &matrix_tests::binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &"a".repeat(64),
            16384,
        )
    }
    #[test]
    fn complete_negative_math_and_partial_positive_witness_are_distinct() {
        let original = construction_tests::fixture();
        let (prefix, witness) = replay(&original, false);
        assert!(witness.is_none());
        assert!(check(&original, &prefix, None)
            .unwrap()
            .negative_domain_verified());
        let mut positive = construction_tests::fixture();
        positive.target = positive.signed[0];
        let (prefix, witness) = replay(&positive, true);
        let result = check(&positive, &prefix, witness.as_deref()).unwrap();
        assert!(result.witness.is_some());
        assert!(!result.prefix.all_tuples_acknowledged);
        assert!(!result.negative_domain_verified());
    }
    #[test]
    fn saved_witness_cannot_be_attached_before_its_candidate_prefix() {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let (prefix, witness) = replay(&original, true);
        let mut saved: SavedSignedReplay = serde_json::from_slice(&prefix).unwrap();
        saved.acknowledged.clear();
        saved.unacknowledged = None;
        saved.performed_additions = 0;
        saved.acknowledged_additions = 0;
        let bytes = serde_json::to_vec(&saved).unwrap();
        assert!(check(&original, &bytes, witness.as_deref()).is_err());
        assert!(!check(&original, &bytes, None)
            .unwrap()
            .negative_domain_verified());
    }
}

/// Match a saved arithmetic prefix to each event of a custody-checked stream.
/// Observations are provisional until the outer capture verifier succeeds.
struct SignedCaptureMatch {
    expected: Vec<SignedPrefix>,
    next: usize,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedSignedEvent {
    kind: String,
    payload: SavedSignedEventPayload,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedSignedEventPayload {
    phase: String,
    progress: SignedPrefix,
}
impl SignedCaptureMatch {
    fn new(replay_bytes: &[u8]) -> Result<Self, String> {
        if replay_bytes.len() > SIGNED_REPLAY_BYTE_CAP {
            return Err("signed replay exceeds byte cap".into());
        }
        let saved: SavedSignedReplay = serde_json::from_slice(replay_bytes)
            .map_err(|e| format!("strict signed capture replay: {e}"))?;
        if saved.acknowledged.len() > 192 {
            return Err("signed capture exceeds domain".into());
        }
        Ok(Self {
            expected: saved.acknowledged,
            next: 0,
        })
    }
    fn observe(&mut self, line: &[u8]) -> Result<(), String> {
        let row: crate::ml_dataset::capture::EventRecord =
            serde_json::from_slice(line).map_err(|e| format!("signed capture record: {e}"))?;
        if row.event["kind"] != "koblitz_m3" {
            return Ok(());
        }
        let event: SavedSignedEvent = serde_json::from_value(row.event)
            .map_err(|e| format!("strict signed capture event: {e}"))?;
        if event.kind != "koblitz_m3"
            || event.payload.phase != "signed-domain"
            || self.expected.get(self.next) != Some(&event.payload.progress)
        {
            return Err("signed capture disagrees with acknowledged arithmetic prefix".into());
        }
        self.next += 1;
        Ok(())
    }
    fn finish(self) -> Result<(), String> {
        if self.next != self.expected.len() {
            return Err("signed capture omits acknowledged arithmetic events".into());
        }
        Ok(())
    }
}

/// Authenticate the live committed stream and match its signed events to the
/// saved replay. Arithmetic and original replay binding require verify_signed_math.
/// No observer result escapes when custody validation fails later in the stream.
pub(super) fn verify_signed_capture(
    cell_dir: &std::path::Path,
    replay_bytes: &[u8],
    binding: &crate::ml_dataset::capture::AttemptBinding,
    limits: &crate::ml_dataset::capture::CaptureLimits,
) -> Result<crate::ml_dataset::capture::VerifiedLivePrefix, String> {
    let mut matcher = SignedCaptureMatch::new(replay_bytes)?;
    let custody = crate::ml_dataset::capture::verify_live_committed_prefix_visit(
        cell_dir,
        binding,
        limits,
        &mut |line| matcher.observe(line),
    )?
    .ok_or("missing committed Koblitz capture prefix")?;
    matcher.finish()?;
    Ok(custody)
}

#[cfg(test)]
mod signed_capture_match_tests {
    use super::*;
    fn event(sequence: u16) -> SignedPrefix {
        SignedPrefix {
            sequence,
            tuple: 0,
            indices: [0; 3],
            summand: sequence as u8,
            prefix: None,
            matches_target: false,
        }
    }
    fn line(event: &SignedPrefix) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "schema":"crypto.ml-dataset.live-event/v2", "binding_sha256":"8".repeat(64),
            "sequence":1,"previous_event_sha256":"0".repeat(64),
            "event":{"kind":"koblitz_m3","payload":{"phase":"signed-domain","progress":event}}
        }))
        .unwrap()
    }
    #[test]
    fn acknowledged_stream_match_rejects_omission_duplication_and_reordering() {
        let expected = vec![event(0), event(1)];
        let matcher = || SignedCaptureMatch {
            expected: expected.clone(),
            next: 0,
        };
        assert!(matcher().finish().is_err());
        let mut omitted = matcher();
        omitted.observe(&line(&expected[0])).unwrap();
        assert!(omitted.finish().is_err());
        let mut reversed = matcher();
        assert!(reversed.observe(&line(&expected[1])).is_err());
        let mut duplicate = matcher();
        duplicate.observe(&line(&expected[0])).unwrap();
        assert!(duplicate.observe(&line(&expected[0])).is_err());
        let mut complete = matcher();
        for e in &expected {
            complete.observe(&line(e)).unwrap();
        }
        complete.finish().unwrap();
    }
    #[test]
    fn signed_capture_schema_and_values_are_strict() {
        let expected = event(0);
        for mutation in 0..5 {
            let mut row: serde_json::Value = serde_json::from_slice(&line(&expected)).unwrap();
            match mutation {
                0 => row["event"]["extra"] = true.into(),
                1 => row["event"]["payload"]["extra"] = true.into(),
                2 => row["event"]["payload"]["progress"]["extra"] = true.into(),
                3 => row["event"]["payload"]["phase"] = "other".into(),
                _ => row["event"]["payload"]["progress"]["matches_target"] = true.into(),
            }
            let mut matcher = SignedCaptureMatch {
                expected: vec![expected.clone()],
                next: 0,
            };
            assert!(matcher.observe(&serde_json::to_vec(&row).unwrap()).is_err());
        }
    }
}

/// Frozen group-replay artifact envelope. Counts are producer reports whose
/// consistency is checked here; they are not certified execution measurements.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedGroupReplay {
    schema: String,
    signed: SavedSignedReplay,
    reported_witness_check_additions: u8,
    reported_total_additions: u64,
}
pub(super) struct VerifiedGroupReplay {
    math: VerifiedSignedMath,
    reported_witness_check_additions: u8,
    reported_total_additions: u64,
}
pub(super) fn verify_group_replay(
    bytes: &[u8],
    witness_bytes: Option<&[u8]>,
    original: &MathInput,
    binding: &crate::ml_dataset::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    input_sha256: &str,
    addition_limit: u64,
) -> Result<VerifiedGroupReplay, String> {
    if bytes.len() > SIGNED_REPLAY_BYTE_CAP {
        return Err("group replay exceeds byte cap".into());
    }
    let saved: SavedGroupReplay =
        serde_json::from_slice(bytes).map_err(|e| format!("strict group replay: {e}"))?;
    if saved.schema != "crypto.ml-dataset.koblitz-m3-group-replay/v1"
        || saved.reported_witness_check_additions > 3
    {
        return Err("group replay schema or witness-check count mismatch".into());
    }
    let signed_bytes =
        serde_json::to_vec(&saved.signed).map_err(|e| format!("bounded signed replay: {e}"))?;
    let math = verify_signed_math(
        &signed_bytes,
        witness_bytes,
        original,
        binding,
        public_sha256,
        factor_base_sha256,
        input_sha256,
        addition_limit,
    )?;
    let checks = saved.reported_witness_check_additions;
    let total = (math.prefix.performed_additions as u64)
        .checked_add(u64::from(checks))
        .ok_or("group replay counter overflow")?;
    if total != saved.reported_total_additions || total > addition_limit {
        return Err("group replay combined addition allowance mismatch".into());
    }
    let first = math.prefix.candidate_tuples.first();
    if let Some(witness) = &math.witness {
        if first != Some(&witness.tuple) {
            return Err("group replay witness is not the first matching tuple".into());
        }
    }
    if (first.is_none() && checks != 0) || (math.witness.is_some() && checks != 3) {
        return Err("group replay checks lack candidate or witness prerequisites".into());
    }
    // Before three reference calls finish, the first candidate cannot have
    // been acknowledged, and enumeration cannot have advanced beyond it.
    if let Some(first) = first {
        if checks < 3
            && !saved.signed.unacknowledged.as_ref().is_some_and(|e| {
                e.matches_target && e.tuple == *first && math.prefix.candidate_tuples.len() == 1
            })
        {
            return Err("group replay advances past an unfinished first witness check".into());
        }
    }
    Ok(VerifiedGroupReplay {
        math,
        reported_witness_check_additions: checks,
        reported_total_additions: total,
    })
}

#[cfg(test)]
mod group_replay_envelope_tests {
    use super::*;
    fn run(limit: u64) -> (MathInput, SavedGroupReplay, Option<Vec<u8>>) {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut b = construction_tests::budget();
        b.addition_limit = limit;
        b.begin_native(false, Instant::now()).unwrap();
        b.finish_native(Instant::now()).unwrap();
        let prepared = prepare_signed(&original, &mut b, false).unwrap();
        let mut witness = None;
        let result = enumerate_prepared_signed(prepared, &mut |_, _| Ok(()), &mut |w, b| {
            let encoded = encode_signed_witness(w, b);
            assert!(encoded.complete && encoded.stop.is_none());
            witness = Some(encoded.bytes);
            Err(Stop::Observer)
        });
        let saved = SavedGroupReplay {
            schema: "crypto.ml-dataset.koblitz-m3-group-replay/v1".into(),
            signed: SavedSignedReplay {
                schema: "crypto.ml-dataset.koblitz-m3-signed-prefix/v1".into(),
                binding: matrix_tests::binding(),
                public_sha256: "8".repeat(64),
                factor_base_sha256: "9".repeat(64),
                input_sha256: "a".repeat(64),
                addition_limit: limit,
                performed_additions: result.enumeration_additions as usize,
                acknowledged_additions: result.acknowledged.len(),
                acknowledged: result.acknowledged,
                unacknowledged: result.unacknowledged,
            },
            reported_witness_check_additions: result.witness_check_additions as u8,
            reported_total_additions: result.enumeration_additions + result.witness_check_additions,
        };
        (original, saved, witness)
    }
    fn check(
        bytes: &[u8],
        witness: Option<&[u8]>,
        original: &MathInput,
        limit: u64,
    ) -> Result<VerifiedGroupReplay, String> {
        verify_group_replay(
            bytes,
            witness,
            original,
            &matrix_tests::binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &"a".repeat(64),
            limit,
        )
    }
    #[test]
    fn group_replay_preserves_actual_partial_witness_checks_under_shared_allowance() {
        let (_, complete, _) = run(16384);
        let enumeration = complete.signed.performed_additions as u64;
        for checks in 0..=3 {
            let limit = enumeration + checks;
            let (original, saved, witness) = run(limit);
            assert_eq!(
                saved.reported_witness_check_additions,
                u8::try_from(checks).unwrap()
            );
            assert_eq!(saved.reported_total_additions, limit);
            let verified = check(
                &serde_json::to_vec(&saved).unwrap(),
                witness.as_deref(),
                &original,
                limit,
            )
            .unwrap();
            assert_eq!(verified.reported_witness_check_additions, checks as u8);
            assert_eq!(verified.reported_total_additions, limit);
            assert!(!verified.math.negative_domain_verified());
        }
    }
    #[test]
    fn group_replay_rejects_rehashed_counter_limit_and_schema_mutations() {
        let (original, saved, witness) = run(16384);
        let base = serde_json::to_value(saved).unwrap();
        for mutation in 0..7 {
            let mut v = base.clone();
            match mutation {
                0 => v["reported_total_additions"] = 0.into(),
                1 => v["reported_witness_check_additions"] = 4.into(),
                2 => v["reported_witness_check_additions"] = 2.into(),
                3 => v["extra"] = true.into(),
                4 => v["signed"]["extra"] = true.into(),
                5 => v["schema"] = "wrong".into(),
                _ => v["signed"]["addition_limit"] = 16385.into(),
            }
            assert!(check(
                &serde_json::to_vec(&v).unwrap(),
                witness.as_deref(),
                &original,
                16384
            )
            .is_err());
        }
    }
}

const KOBLITZ_SLOTS: [(&str, &str, &str); 4] = [
    (
        "koblitz-m3-input.json",
        "restricted-koblitz-m3-original-equations/v1",
        "json",
    ),
    (
        "koblitz-m3-events.jsonl",
        "restricted-koblitz-m3-boundary-events/v1",
        "jsonl",
    ),
    (
        "koblitz-m3-result.json",
        "restricted-koblitz-m3-result/v1",
        "json",
    ),
    (
        "koblitz-m3-group-replay.json",
        "restricted-koblitz-m3-group-replay/v1",
        "json",
    ),
];
/// Validate reserved identities even on a raw-prefix path. Files and capture
/// chain authentication remain mandatory at the outer custody boundary.
fn verify_slot_identities(artifacts: &[crate::ml_dataset::types::Artifact]) -> Result<(), String> {
    for a in artifacts {
        let live = a.path.strip_prefix("live/payloads/");
        let path = live.unwrap_or(&a.path);
        if !path.starts_with("koblitz-m3-") && !a.role.starts_with("restricted-koblitz-m3-") {
            continue;
        }
        if artifacts
            .iter()
            .filter(|other| other.path == a.path)
            .count()
            != 1
        {
            return Err("duplicate Koblitz artifact path".into());
        }
        let (_, role, encoding) = KOBLITZ_SLOTS
            .iter()
            .find(|slot| slot.0 == path)
            .ok_or("unknown or displaced reserved Koblitz artifact")?;
        let raw = a.role == "unregistered-partial-artifact" && a.encoding == "native";
        if live.is_some() {
            if !raw {
                return Err("Koblitz live copy has canonical manifest identity".into());
            }
        } else if !raw && (a.role != *role || a.encoding != *encoding) {
            return Err("Koblitz canonical artifact identity mismatch".into());
        }
    }
    Ok(())
}
/// All four canonical identities are required by this path regardless of
/// producer completion flags. Censored incomplete prefixes use separate admission.
fn require_canonical_slots(artifacts: &[crate::ml_dataset::types::Artifact]) -> Result<(), String> {
    verify_slot_identities(artifacts)?;
    for (path, role, encoding) in KOBLITZ_SLOTS {
        super::prime_f4_wide::artifact(artifacts, path, role, encoding, true)?;
    }
    Ok(())
}
#[cfg(test)]
mod koblitz_slot_tests {
    use super::*;
    use crate::ml_dataset::types::Artifact;
    fn canonical() -> Vec<Artifact> {
        KOBLITZ_SLOTS
            .iter()
            .map(|(path, role, encoding)| Artifact {
                path: (*path).into(),
                role: (*role).into(),
                encoding: (*encoding).into(),
                sha256: "8".repeat(64),
                bytes: 1,
            })
            .collect()
    }
    #[test]
    fn canonical_registry_rejects_missing_duplicate_alias_and_role_mutations() {
        require_canonical_slots(&canonical()).unwrap();
        for mutation in 0..7 {
            let mut a = canonical();
            match mutation {
                0 => {
                    a.pop();
                }
                1 => a.push(a[0].clone()),
                2 => a[0].path = "alias.json".into(),
                3 => a[0].role = a[1].role.clone(),
                4 => a[0].encoding = "jsonl".into(),
                5 => {
                    a[0].role = "unregistered-partial-artifact".into();
                    a[0].encoding = "native".into();
                }
                _ => a[0].path = "live/payloads/koblitz-m3-input.json".into(),
            }
            assert!(require_canonical_slots(&a).is_err());
        }
    }
    #[test]
    fn raw_live_custody_is_distinct_from_complete_canonical_evidence() {
        let mut a = canonical();
        for item in &mut a {
            item.path = format!("live/payloads/{}", item.path);
            item.role = "unregistered-partial-artifact".into();
            item.encoding = "native".into();
        }
        verify_slot_identities(&a).unwrap();
        assert!(require_canonical_slots(&a).is_err());
        a.extend(canonical());
        require_canonical_slots(&a).unwrap();
        a[0].role = "restricted-koblitz-m3-original-equations/v1".into();
        assert!(verify_slot_identities(&a).is_err());
    }
}

#[cfg(test)]
mod matrix_failure_propagation_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    #[test]
    fn native_entry_work_failure_is_returned_and_blocks_extraction() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            let original = construction_tests::fixture();
            let mut budget = construction_tests::budget();
            let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
                .output
                .unwrap();
            let input = persist_original(
                &original,
                &output,
                &mut budget,
                &matrix_tests::binding(),
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |_, b| {
                    let remaining = b.native_work_limit - b.native.work - b.checking_work;
                    b.reserve_checking(remaining - 1, 0, Instant::now())
                        .map_err(|s| format!("{s:?}"))
                },
            )
            .unwrap();
            let result = matrix_step(input, engine, 3, &mut |_, _| Ok(()));
            assert_eq!(result.adapter_stop, Some(Stop::NativeFailure));
            let report = result.report.unwrap();
            assert_eq!(report.stop, native::Stop::WorkLimit);
            assert!(!report.native_step_complete);
            assert!(!result.original_bytes.is_empty());
            assert_eq!(budget.stopped, Some(Stop::NativeFailure));
            assert_eq!(
                budget.begin_extraction(true, Instant::now()),
                Err(Stop::NativeFailure)
            );
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedMatrixReport {
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    stop: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Stop,
    backend_invoked: bool,
    native_step_complete: bool,
    criterion_status: crate::cryptanalysis::koblitz_groebner::bounded_matrix::CriterionStatus,
    reserved: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Reservation,
    counters: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Counters,
    columns: usize,
    rank: Option<usize>,
    rows: Vec<SavedEquation>,
}
/// Only finite-assignment consistency and bounded report structure are checked.
/// This is not a derivation certificate, full row-space check or solver verdict.
struct MatrixAssessment {
    input_zero_assignments: usize,
    rows_checked: usize,
    reported_native_step_complete: bool,
}
fn check_saved_boolean_polynomial(poly: &SavedEquation) -> Result<(), String> {
    use crate::cryptanalysis::pq_groebner_f2::{cmp_mono, mask_from64, F2BoolMono};
    if poly.n_vars != 3 || poly.terms.len() > 8 || poly.terms.iter().any(|m| m.mask > 7) {
        return Err("matrix polynomial exceeds three-selector domain".into());
    }
    for pair in poly.terms.windows(2) {
        if !cmp_mono(
            F2BoolMono::from_mask(mask_from64(pair[0].mask as u64)),
            F2BoolMono::from_mask(mask_from64(pair[1].mask as u64)),
        )
        .is_gt()
        {
            return Err("matrix polynomial terms are not canonical".into());
        }
    }
    Ok(())
}
fn boolean_value(poly: &SavedEquation, assignment: u8) -> bool {
    poly.terms.iter().fold(false, |value, mono| {
        value ^ (assignment & mono.mask == mono.mask)
    })
}
fn verify_matrix_report(
    bytes: &[u8],
    original_equations: &[SavedEquation],
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    limits: OriginalLimits,
) -> Result<MatrixAssessment, String> {
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    if bytes.len() > INPUT_BYTE_CAP || original_equations.len() > 61 {
        return Err("matrix replay exceeds fixed decode domain".into());
    }
    let report: SavedMatrixReport =
        serde_json::from_slice(bytes).map_err(|e| format!("strict bounded matrix report: {e}"))?;
    if report.engine != engine
        || report.degree != degree
        || report.columns > 8
        || report.rows.len() > 8
        || report.rank.is_some_and(|rank| rank > report.columns)
        || report.reserved.work > limits.native_work
        || report.reserved.bytes > limits.memory.min(NATIVE_BYTES_MAX)
        || report.counters.acknowledged_events > report.counters.attempted_events
        || report.counters.attempted_events > native::MAX_EVENTS
        || report.native_step_complete != (report.stop == native::Stop::Complete)
        || (report.native_step_complete
            && (!report.backend_invoked || report.rank != Some(report.rows.len())))
        || (engine == native::Engine::F4Macaulay
            && report.criterion_status != native::CriterionStatus::NotRequested)
    {
        return Err("matrix report identity, bounds or completion mismatch".into());
    }
    for poly in original_equations.iter().chain(&report.rows) {
        check_saved_boolean_polynomial(poly)?;
    }
    let mut input_zero_assignments = 0;
    for assignment in 0..8 {
        if original_equations
            .iter()
            .all(|p| !boolean_value(p, assignment))
        {
            input_zero_assignments += 1;
            if report.rows.iter().any(|p| boolean_value(p, assignment)) {
                return Err("matrix row contradicts an original Boolean solution".into());
            }
        }
    }
    Ok(MatrixAssessment {
        input_zero_assignments,
        rows_checked: report.rows.len(),
        reported_native_step_complete: report.native_step_complete,
    })
}

#[cfg(test)]
mod matrix_report_replay_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    fn actual(engine: native::Engine) -> (Vec<u8>, Vec<SavedEquation>, OriginalLimits) {
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut budget = construction_tests::budget();
        let limits = budget.original_limits();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let input = persist_original(
            &original,
            &output,
            &mut budget,
            &matrix_tests::binding(),
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |_, _| Ok(()),
        )
        .unwrap();
        let step = matrix_step(input, engine, 3, &mut |_, _| Ok(()));
        assert_eq!(step.adapter_stop, None);
        let saved: SavedInput = serde_json::from_slice(&step.original_bytes).unwrap();
        (
            serde_json::to_vec(&step.report.unwrap()).unwrap(),
            saved.equations,
            limits,
        )
    }
    #[test]
    fn actual_f4_and_f5_rows_preserve_original_positive_assignments() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            let (bytes, equations, limits) = actual(engine);
            let checked = verify_matrix_report(&bytes, &equations, engine, 3, limits).unwrap();
            assert!(checked.input_zero_assignments > 0);
            assert!(checked.rows_checked > 0);
            assert!(checked.reported_native_step_complete);
        }
    }
    #[test]
    fn matrix_rows_cannot_contradict_a_positive_original_assignment() {
        let engine = native::Engine::F4Macaulay;
        let (bytes, equations, limits) = actual(engine);
        let mut report: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        report["rows"] = serde_json::json!([{"n_vars":3,"terms":[{"mask":0}]}]);
        report["rank"] = 1.into();
        assert!(verify_matrix_report(
            &serde_json::to_vec(&report).unwrap(),
            &equations,
            engine,
            3,
            limits
        )
        .err()
        .unwrap()
        .contains("contradicts an original Boolean solution"));
    }
    #[test]
    fn matrix_replay_rejects_reencoded_schema_domain_and_report_mutations() {
        let engine = native::Engine::F4Macaulay;
        let (bytes, equations, limits) = actual(engine);
        let base: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for mutation in 0..10 {
            let mut v = base.clone();
            match mutation {
                0 => v["extra"] = true.into(),
                1 => v["rows"][0]["extra"] = true.into(),
                2 => v["rows"][0]["terms"][0]["extra"] = true.into(),
                3 => v["rows"][0]["n_vars"] = 4.into(),
                4 => v["rows"][0]["terms"][0]["mask"] = 8.into(),
                5 => v["rank"] = 9.into(),
                6 => v["degree"] = 4.into(),
                7 => v["reserved"]["work"] = u64::MAX.into(),
                8 => {
                    v["counters"]["acknowledged_events"] =
                        (base["counters"]["attempted_events"].as_u64().unwrap() + 1).into()
                }
                _ => v["stop"] = "Deadline".into(),
            }
            assert!(verify_matrix_report(
                &serde_json::to_vec(&v).unwrap(),
                &equations,
                engine,
                3,
                limits
            )
            .is_err());
        }
    }
}

impl Budget {
    /// Reconcile an actual terminal native snapshot without consulting time or
    /// reopening a stopped phase. Valid reservations survive an earlier failure.
    fn settle_native_terminal(&mut self, current: Reservation) -> Result<(), Stop> {
        if !matches!(self.phase, Phase::Construction | Phase::Matrix) {
            return self.fail(Stop::Phase);
        }
        if current.work < self.phase_native.work || current.bytes < self.phase_native.bytes {
            return self.fail(Stop::Nonmonotone);
        }
        if current.work > self.phase_limits.work {
            return self.fail(Stop::Work);
        }
        if current.bytes > self.phase_limits.bytes {
            return self.fail(Stop::Bytes);
        }
        let Some(work) = self
            .native
            .work
            .checked_add(current.work - self.phase_native.work)
        else {
            return self.fail(Stop::Overflow);
        };
        let Some(bytes) = self
            .native
            .bytes
            .checked_add(current.bytes - self.phase_native.bytes)
        else {
            return self.fail(Stop::Overflow);
        };
        // Keep a valid native report even if callback reservations consumed the
        // aggregate headroom. That discrepancy is a failure, never a refund.
        self.native = Reservation { work, bytes };
        self.phase_native = current;
        if work
            .checked_add(self.checking_work)
            .is_none_or(|total| total > self.native_work_limit)
        {
            return self.fail(Stop::Work);
        }
        if bytes
            .checked_add(self.adapter_bytes)
            .is_none_or(|total| total > self.memory_limit)
        {
            return self.fail(Stop::Bytes);
        }
        match self.stopped {
            Some(stop) => Err(stop),
            None => Ok(()),
        }
    }
}
#[cfg(test)]
mod terminal_native_settlement_tests {
    use super::*;
    fn running() -> Budget {
        let mut b = construction_tests::budget();
        b.begin_native(false, Instant::now()).unwrap();
        b
    }
    #[test]
    fn terminal_delta_survives_deadline_and_observer_stop_without_restarting() {
        for stop in [Stop::Deadline, Stop::Observer] {
            let mut b = running();
            b.observe(Reservation { work: 2, bytes: 3 }, Instant::now())
                .unwrap();
            b.stopped = Some(stop);
            b.deadline = Instant::now();
            b.construction_deadline = Instant::now();
            let final_snapshot = Reservation { work: 7, bytes: 11 };
            assert_eq!(b.settle_native_terminal(final_snapshot), Err(stop));
            assert_eq!(b.native, final_snapshot);
            assert_eq!(b.settle_native_terminal(final_snapshot), Err(stop));
            assert_eq!(b.native, final_snapshot);
            assert_eq!(b.finish_native(Instant::now()), Err(stop));
            assert_eq!(b.begin_native(true, Instant::now()), Err(stop));
        }
    }
    #[test]
    fn terminal_snapshot_rejects_regression_permit_overflow_and_integer_overflow() {
        let mut b = running();
        b.observe(Reservation { work: 2, bytes: 3 }, Instant::now())
            .unwrap();
        assert_eq!(
            b.settle_native_terminal(Reservation { work: 1, bytes: 3 }),
            Err(Stop::Nonmonotone)
        );
        assert_eq!(b.native, Reservation { work: 2, bytes: 3 });
        let mut b = running();
        let work = b.phase_limits.work + 1;
        assert_eq!(
            b.settle_native_terminal(Reservation { work, bytes: 0 }),
            Err(Stop::Work)
        );
        assert_eq!(b.native, Reservation::default());
        let mut b = running();
        let bytes = b.phase_limits.bytes + 1;
        assert_eq!(
            b.settle_native_terminal(Reservation { work: 0, bytes }),
            Err(Stop::Bytes)
        );
        assert_eq!(b.native, Reservation::default());
        let mut b = running();
        b.native.work = u64::MAX;
        assert_eq!(
            b.settle_native_terminal(Reservation { work: 1, bytes: 0 }),
            Err(Stop::Overflow)
        );
        assert_eq!(b.native.work, u64::MAX);
    }
    #[test]
    fn terminal_headroom_conflict_retains_report_and_stops_later_work() {
        for memory in [false, true] {
            let mut b = running();
            if memory {
                b.adapter_bytes = b.memory_limit - 1;
            } else {
                b.checking_work = b.native_work_limit - 1;
            }
            let current = Reservation { work: 2, bytes: 2 };
            let expected = if memory { Stop::Bytes } else { Stop::Work };
            assert_eq!(b.settle_native_terminal(current), Err(expected));
            assert_eq!(b.native, current);
            assert_eq!(b.reserve_bytes(1, Instant::now()), Err(expected));
            assert_eq!(b.settle_native_terminal(current), Err(expected));
            assert_eq!(b.native, current);
        }
    }
    #[test]
    fn actual_native_failure_before_begin_callback_is_charged_for_both_engines() {
        use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
        use crate::cryptanalysis::pq_groebner_f2::F2BoolMono;
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            let original = construction_tests::fixture();
            let mut b = construction_tests::budget();
            let mut output = construct_original(&original, &mut b, &mut |_, _| Ok(()))
                .output
                .unwrap();
            // Deliberately noncanonical native input: entry admission precedes
            // this validation, so its reservation has no Begin callback.
            output.equations[0].terms = vec![F2BoolMono::one(), F2BoolMono::one()];
            let before = b.native;
            let input = persist_original(
                &original,
                &output,
                &mut b,
                &matrix_tests::binding(),
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |_, _| Ok(()),
            )
            .unwrap();
            let result = matrix_step(input, engine, 3, &mut |_, _| panic!("unexpected callback"));
            let report = result.report.unwrap();
            assert_eq!(report.stop, native::Stop::InvalidInput);
            assert!(report.reserved.work > 0 && report.reserved.bytes > 0);
            assert_eq!(b.native.work, before.work + report.reserved.work);
            assert_eq!(b.native.bytes, before.bytes + report.reserved.bytes);
            assert_eq!(result.adapter_stop, Some(Stop::NativeFailure));
            assert_eq!(
                b.begin_extraction(true, Instant::now()),
                Err(Stop::NativeFailure)
            );
        }
    }
}

/// Mathematical and witness-publication component of canonical cell replay.
/// The caller supplies a sealed, non-mutating cell and admitted original manifest.
/// Result/event semantics, aggregate resources, original-run admission and outer
/// supervisor claims are separate mandatory gates before any final label.
pub(super) struct VerifiedCanonicalMath {
    group: VerifiedGroupReplay,
    witness_publication_sequence: Option<u64>,
}
pub(super) fn verify_canonical_math_files(
    dir: &std::path::Path,
    artifacts: &[crate::ml_dataset::types::Artifact],
    original: &MathInput,
    binding: &crate::ml_dataset::capture::AttemptBinding,
    capture_limits: &crate::ml_dataset::capture::CaptureLimits,
    limits: OriginalLimits,
    public_sha256: &str,
    factor_base_sha256: &str,
) -> Result<VerifiedCanonicalMath, String> {
    use super::prime_f4_wide::{artifact, read_bounded};
    use crate::ml_dataset::storage;
    require_canonical_slots(artifacts)?;
    for path in ["live/committed.json", "live/events.jsonl"] {
        if artifacts.iter().filter(|a| a.path == path).count() != 1 {
            return Err(format!(
                "Koblitz capture root lacks unique manifest binding: {path}"
            ));
        }
    }
    // The caller has bounded/admitted the original manifest. Verify every
    // referenced file before deriving meaning from any canonical payload.
    storage::verify_manifest(dir, artifacts)?;
    let read = |path: &str, role: &str, encoding: &str, cap: usize| -> Result<Vec<u8>, String> {
        let expected =
            artifact(artifacts, path, role, encoding, true)?.ok_or("missing canonical file")?;
        let bytes = read_bounded(dir, path, cap)?;
        if bytes.len() as u64 != expected.bytes
            || crate::ml_dataset::digest(&bytes) != expected.sha256
        {
            return Err(format!("canonical bytes differ from manifest: {path}"));
        }
        Ok(bytes)
    };
    let input = read(
        "koblitz-m3-input.json",
        "restricted-koblitz-m3-original-equations/v1",
        "json",
        INPUT_BYTE_CAP,
    )?;
    verify_original_input(
        &input,
        original,
        binding,
        public_sha256,
        factor_base_sha256,
        limits,
    )?;
    let input_sha256 = crate::ml_dataset::digest(&input);
    let group_bytes = read(
        "koblitz-m3-group-replay.json",
        "restricted-koblitz-m3-group-replay/v1",
        "json",
        SIGNED_REPLAY_BYTE_CAP,
    )?;
    let witness_artifact = artifact(
        artifacts,
        "pdp-witness.json",
        "independently-verified-pdp",
        "json",
        false,
    )?;
    let witness = witness_artifact
        .map(|_| {
            read(
                "pdp-witness.json",
                "independently-verified-pdp",
                "json",
                SIGNED_WITNESS_BYTE_CAP,
            )
        })
        .transpose()?;
    let group = verify_group_replay(
        &group_bytes,
        witness.as_deref(),
        original,
        binding,
        public_sha256,
        factor_base_sha256,
        &input_sha256,
        limits.additions,
    )?;
    let envelope: SavedGroupReplay = serde_json::from_slice(&group_bytes)
        .map_err(|e| format!("strict canonical group replay: {e}"))?;
    let signed = serde_json::to_vec(&envelope.signed).map_err(|e| e.to_string())?;
    let custody = verify_signed_capture(dir, &signed, binding, capture_limits)?;
    let published = custody
        .payloads
        .iter()
        .filter(|(_, a)| {
            a.path == "payloads/pdp-witness.json" || a.role == "independently-verified-pdp"
        })
        .collect::<Vec<_>>();
    let witness_publication_sequence = match witness_artifact {
        None => {
            if !published.is_empty() {
                return Err("published Koblitz witness lacks canonical identity".into());
            }
            None
        }
        Some(expected) => {
            if published.len() != 1 {
                return Err("canonical Koblitz witness lacks unique publication".into());
            }
            let (sequence, actual) = published[0];
            if actual.path != "payloads/pdp-witness.json"
                || actual.role != expected.role
                || actual.encoding != expected.encoding
                || actual.sha256 != expected.sha256
                || actual.bytes != expected.bytes
            {
                return Err("Koblitz witness differs from authenticated publication".into());
            }
            Some(*sequence)
        }
    };
    Ok(VerifiedCanonicalMath {
        group,
        witness_publication_sequence,
    })
}

#[cfg(test)]
mod canonical_math_file_tests {
    use super::*;
    use crate::ml_dataset::{
        capture::{capture_limits, AttemptBinding},
        types::Artifact,
    };
    fn manifest(root: &std::path::Path, names: &[(&str, &str, &str)]) -> Vec<Artifact> {
        names
            .iter()
            .map(|(path, role, encoding)| {
                let bytes = std::fs::read(root.join(path)).unwrap();
                Artifact {
                    path: (*path).into(),
                    role: (*role).into(),
                    encoding: (*encoding).into(),
                    bytes: bytes.len() as u64,
                    sha256: crate::ml_dataset::digest(&bytes),
                }
            })
            .collect()
    }
    #[test]
    fn canonical_math_joins_actual_input_prefix_and_published_witness() {
        let binding: AttemptBinding = matrix_tests::binding();
        let capture = capture_limits(16).unwrap();
        let fixture = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let root = fixture.dir.parent().unwrap();
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut budget = construction_tests::budget();
        let limits = budget.original_limits();
        let output = construct_original(&original, &mut budget, &mut |_, _| Ok(()))
            .output
            .unwrap();
        let mut input_bytes = Vec::new();
        let input = persist_original(
            &original,
            &output,
            &mut budget,
            &binding,
            &"8".repeat(64),
            &"9".repeat(64),
            &mut |bytes, _| {
                input_bytes = bytes.to_vec();
                Ok(())
            },
        )
        .unwrap();
        drop(input);
        std::fs::write(root.join("koblitz-m3-input.json"), &input_bytes).unwrap();
        let mut events = BoundSignedEvents::new(&binding, &capture, &mut budget).unwrap();
        let mut publisher = BoundSignedPublisher::new(&binding, &capture, &budget);
        let prepared = prepare_signed(&original, &mut budget, false).unwrap();
        let global = std::cell::RefCell::new(0usize);
        let result = enumerate_prepared_signed(
            prepared,
            &mut |e, b| events.emit(e, b, &mut global.borrow_mut()),
            &mut |w, b| publisher.publish(w, b, &mut global.borrow_mut()),
        );
        assert!(result.complete, "{:?}", result.stop);
        let group = SavedGroupReplay {
            schema: "crypto.ml-dataset.koblitz-m3-group-replay/v1".into(),
            signed: SavedSignedReplay {
                schema: "crypto.ml-dataset.koblitz-m3-signed-prefix/v1".into(),
                binding: binding.clone(),
                public_sha256: "8".repeat(64),
                factor_base_sha256: "9".repeat(64),
                input_sha256: crate::ml_dataset::digest(&input_bytes),
                addition_limit: limits.additions,
                performed_additions: result.enumeration_additions as usize,
                acknowledged_additions: result.acknowledged.len(),
                acknowledged: result.acknowledged,
                unacknowledged: result.unacknowledged,
            },
            reported_witness_check_additions: result.witness_check_additions as u8,
            reported_total_additions: result.enumeration_additions + result.witness_check_additions,
        };
        std::fs::write(
            root.join("koblitz-m3-group-replay.json"),
            serde_json::to_vec(&group).unwrap(),
        )
        .unwrap();
        // These two artifact identities are required here; their semantics are
        // deliberately outside this mathematical component's claim.
        std::fs::write(root.join("koblitz-m3-result.json"), b"{}").unwrap();
        std::fs::write(root.join("koblitz-m3-events.jsonl"), b"").unwrap();
        std::fs::copy(
            fixture.dir.join("payloads/pdp-witness.json"),
            root.join("pdp-witness.json"),
        )
        .unwrap();
        let mut names = KOBLITZ_SLOTS.to_vec();
        names.extend([
            ("pdp-witness.json", "independently-verified-pdp", "json"),
            (
                "live/committed.json",
                "unregistered-partial-artifact",
                "native",
            ),
            (
                "live/events.jsonl",
                "unregistered-partial-artifact",
                "native",
            ),
        ]);
        let artifacts = manifest(root, &names);
        let check = |a: &[Artifact]| {
            verify_canonical_math_files(
                root,
                a,
                &original,
                &binding,
                &capture,
                limits,
                &"8".repeat(64),
                &"9".repeat(64),
            )
        };
        let verified = check(&artifacts).unwrap();
        assert!(verified.group.math.witness.is_some());
        assert!(verified.witness_publication_sequence.is_some());
        assert!(verified.group.math.prefix.all_tuples_acknowledged);
        let without = artifacts
            .iter()
            .filter(|a| a.path != "pdp-witness.json")
            .cloned()
            .collect::<Vec<_>>();
        assert!(check(&without)
            .err()
            .unwrap()
            .contains("lacks canonical identity"));
        let without = artifacts
            .iter()
            .filter(|a| a.path != "live/committed.json")
            .cloned()
            .collect::<Vec<_>>();
        assert!(check(&without).err().unwrap().contains("capture root"));
        // Rehash a byte-distinct but mathematically identical canonical witness.
        // File hashing and group arithmetic pass; publication identity must fail.
        let mut witness = std::fs::read(root.join("pdp-witness.json")).unwrap();
        witness.push(b' ');
        std::fs::write(root.join("pdp-witness.json"), witness).unwrap();
        assert!(check(&manifest(root, &names))
            .err()
            .unwrap()
            .contains("differs from authenticated publication"));
    }
}
