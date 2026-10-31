//! Adapter-side cumulative reservations for the bounded Koblitz m3 pipeline.
//!
//! This module is not yet registered in runtime dispatch. Its constructor must
//! receive limits from a validated original SolverExecutionContext, never from
//! a producer result. Reservations model admitted work/bytes, not RSS or retired
//! instructions. Native work and actual group additions have different units.
use std::time::{Duration, Instant};

const NATIVE_WORK_MAX: u64 = 16_777_216;
const NATIVE_BYTES_MAX: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(super) enum Phase {
    Setup,
    Construction,
    Constructed,
    Matrix,
    MatrixComplete,
    Extraction,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
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
    retention_started: bool,
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
            retention_started: false,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
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
            Some(super::EventSink::Bound(sink)) => sink.verify_digest_limits_at_count(
                &self.binding_digest,
                self.limits,
                *global as u64,
            ),
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
        if path == PARENT_DELIVERY_SLOT.0 {
            let raw = a.role == "unregistered-partial-artifact" && a.encoding == "native";
            if artifacts
                .iter()
                .filter(|other| other.path == a.path)
                .count()
                != 1
                || (!raw
                    && (live.is_some()
                        || a.role != PARENT_DELIVERY_SLOT.1
                        || a.encoding != PARENT_DELIVERY_SLOT.2))
            {
                return Err("invalid parent delivery artifact identity".into());
            }
            continue;
        }

        if RAW_RETENTION_NAMES.iter().any(|(_, flat)| *flat == path) {
            if a.role != "unregistered-partial-artifact"
                || a.encoding != "native"
                || artifacts
                    .iter()
                    .filter(|other| other.path == a.path)
                    .count()
                    != 1
            {
                return Err("invalid flat Koblitz partial artifact identity".into());
            }
            continue;
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum ConstructionBoundaryStage {
    Begin,
    Table,
    Selector,
    Add,
    Square,
    Product,
    Coordinates,
    Complete,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
enum MatrixBoundaryStage {
    Begin,
    CriterionLevel {
        level: u32,
    },
    CriterionCandidate {
        generator: usize,
        multiplier: u8,
        frobenius: bool,
    },
    MainCandidate {
        generator: usize,
        multiplier: u8,
    },
    Pack,
    Reduce,
    Readback,
    Finish,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case", deny_unknown_fields)]
enum NativeBoundary {
    Construction {
        stage: ConstructionBoundaryStage,
        reserved: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Reservation,
    },
    Matrix {
        stage: MatrixBoundaryStage,
        reserved: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Reservation,
        counters: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Counters,
    },
}
impl NativeBoundary {
    fn construction(event: crate::cryptanalysis::koblitz_groebner::bounded_s4::Event) -> Self {
        use crate::cryptanalysis::koblitz_groebner::bounded_s4::Stage as S;
        let stage = match event.stage {
            S::Begin => ConstructionBoundaryStage::Begin,
            S::Table => ConstructionBoundaryStage::Table,
            S::Selector => ConstructionBoundaryStage::Selector,
            S::Add => ConstructionBoundaryStage::Add,
            S::Square => ConstructionBoundaryStage::Square,
            S::Product => ConstructionBoundaryStage::Product,
            S::Coordinates => ConstructionBoundaryStage::Coordinates,
            S::Complete => ConstructionBoundaryStage::Complete,
        };
        Self::Construction {
            stage,
            reserved: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Reservation {
                work: event.reserved.work,
                bytes: event.reserved.bytes,
            },
        }
    }
    fn matrix(event: &crate::cryptanalysis::koblitz_groebner::bounded_matrix::Event) -> Self {
        use crate::cryptanalysis::koblitz_groebner::bounded_matrix::Stage as S;
        let stage = match event.stage {
            S::Begin => MatrixBoundaryStage::Begin,
            S::CriterionLevel { level } => MatrixBoundaryStage::CriterionLevel { level },
            S::CriterionCandidate {
                generator,
                multiplier,
                frobenius,
            } => MatrixBoundaryStage::CriterionCandidate {
                generator,
                multiplier,
                frobenius,
            },
            S::MainCandidate {
                generator,
                multiplier,
            } => MatrixBoundaryStage::MainCandidate {
                generator,
                multiplier,
            },
            S::Pack => MatrixBoundaryStage::Pack,
            S::Reduce => MatrixBoundaryStage::Reduce,
            S::Readback => MatrixBoundaryStage::Readback,
            S::Finish => MatrixBoundaryStage::Finish,
        };
        Self::Matrix {
            stage,
            reserved: event.reserved,
            counters: event.counters,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedBoundaryRecord {
    schema: String,
    binding_sha256: String,
    sequence: u64,
    capture_sequence: u64,
    event: NativeBoundary,
}
/// Strict record admission only; stream ordering, authenticated capture matching
/// and native terminal-result consistency are separate checks.
fn decode_boundary_record(
    line: &[u8],
    binding_sha256: &str,
    sequence: u64,
    equations: usize,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
) -> Result<SavedBoundaryRecord, String> {
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    if line.len() > 2048
        || !line.ends_with(b"\n")
        || !(1..=4096).contains(&sequence)
        || equations > 61
        || !(1..=6).contains(&degree)
    {
        return Err("native boundary decode limits".into());
    }
    crate::store::index::validate_sha256(binding_sha256)?;
    let record: SavedBoundaryRecord =
        serde_json::from_slice(line).map_err(|e| format!("strict native boundary: {e}"))?;
    if record.schema != "crypto.ml-dataset.koblitz-m3-events/v1"
        || record.binding_sha256 != binding_sha256
        || record.sequence != sequence
        || !(1..=4096).contains(&record.capture_sequence)
    {
        return Err("native boundary schema, binding or sequence mismatch".into());
    }
    let reserved = match record.event {
        NativeBoundary::Construction { reserved, .. } => reserved,
        NativeBoundary::Matrix {
            stage,
            reserved,
            counters,
        } => {
            if counters.attempted_events == 0
                || counters.attempted_events > native::MAX_EVENTS
                || counters.acknowledged_events.checked_add(1) != Some(counters.attempted_events)
                || counters.dynamic_layout_bytes
                    > reserved.bytes.saturating_sub(native::METADATA_BYTES)
            {
                return Err("native boundary counter mismatch".into());
            }
            match stage {
                MatrixBoundaryStage::CriterionLevel { level } => {
                    if engine != native::Engine::F5KoszulFrobenius || level >= degree {
                        return Err("native criterion level mismatch".into());
                    }
                }
                MatrixBoundaryStage::CriterionCandidate {
                    generator,
                    multiplier,
                    ..
                } => {
                    if engine != native::Engine::F5KoszulFrobenius
                        || generator >= equations
                        || multiplier > 7
                    {
                        return Err("native criterion candidate mismatch".into());
                    }
                }
                MatrixBoundaryStage::MainCandidate {
                    generator,
                    multiplier,
                } => {
                    if generator >= equations || multiplier > 7 {
                        return Err("native main candidate mismatch".into());
                    }
                }
                _ => {}
            }
            reserved
        }
    };
    if reserved.work > native::MAX_WORK || reserved.bytes > native::MAX_BYTES {
        return Err("native boundary reservation exceeds native caps".into());
    }
    Ok(record)
}

#[cfg(test)]
mod native_boundary_schema_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    fn encode(event: NativeBoundary, sequence: u64) -> Vec<u8> {
        let record = SavedBoundaryRecord {
            schema: "crypto.ml-dataset.koblitz-m3-events/v1".into(),
            binding_sha256: matrix_tests::binding().digest().unwrap(),
            sequence,
            capture_sequence: sequence,
            event,
        };
        let mut bytes = serde_json::to_vec(&record).unwrap();
        bytes.push(b'\n');
        bytes
    }
    #[test]
    fn strict_boundary_records_roundtrip_actual_construction_and_both_matrix_engines() {
        let binding = matrix_tests::binding();
        let digest = binding.digest().unwrap();
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            let original = construction_tests::fixture();
            let mut budget = construction_tests::budget();
            let mut boundaries = Vec::new();
            let output = construct_original(&original, &mut budget, &mut |event, _| {
                boundaries.push(NativeBoundary::construction(event));
                Ok(())
            })
            .output
            .unwrap();
            let construction_count = boundaries.len();
            assert!(construction_count > 0);
            let input = persist_original(
                &original,
                &output,
                &mut budget,
                &binding,
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |_, _| Ok(()),
            )
            .unwrap();
            let step = matrix_step(input, engine, 3, &mut |event, _| {
                boundaries.push(NativeBoundary::matrix(event));
                Ok(())
            });
            assert_eq!(step.adapter_stop, None);
            assert!(boundaries.len() > construction_count);
            let mut stream = Vec::new();
            for (i, event) in boundaries.into_iter().enumerate() {
                let sequence = (i + 1) as u64;
                let bytes = encode(event, sequence);
                stream.extend_from_slice(&bytes);
                let decoded =
                    decode_boundary_record(&bytes, &digest, sequence, 23, engine, 3).unwrap();
                assert_eq!(decoded.event, event);
            }
            let verified = verify_boundary_stream(&stream, &digest, 23, engine, 3).unwrap();
            assert!(verified.construction_complete_captured && verified.matrix_finish_captured);
            let lines = stream.split_inclusive(|b| *b == b'\n').collect::<Vec<_>>();
            assert_eq!(verified.records, lines.len());
            let prefix = lines[..lines.len() - 1].concat();
            assert!(
                !verify_boundary_stream(&prefix, &digest, 23, engine, 3)
                    .unwrap()
                    .matrix_finish_captured
            );
            let mut missing = lines.clone();
            missing.remove(construction_count);
            assert!(verify_boundary_stream(&missing.concat(), &digest, 23, engine, 3).is_err());
            let mut reordered = lines.clone();
            reordered.swap(construction_count, construction_count + 1);
            assert!(verify_boundary_stream(&reordered.concat(), &digest, 23, engine, 3).is_err());
            let mut duplicate = lines.clone();
            duplicate.insert(construction_count, lines[construction_count]);
            assert!(verify_boundary_stream(&duplicate.concat(), &digest, 23, engine, 3).is_err());
        }
    }
    fn candidate() -> NativeBoundary {
        NativeBoundary::Matrix {
            stage: MatrixBoundaryStage::CriterionCandidate {
                generator: 0,
                multiplier: 1,
                frobenius: true,
            },
            reserved: native::Reservation {
                work: 8192,
                bytes: 8192,
            },
            counters: native::Counters {
                attempted_events: 1,
                ..Default::default()
            },
        }
    }
    #[test]
    fn boundary_records_reject_nested_unknown_fields_and_metadata_mutations() {
        let base: serde_json::Value = serde_json::from_slice(&encode(candidate(), 1)).unwrap();
        let digest = matrix_tests::binding().digest().unwrap();
        for mutation in 0..12 {
            let mut v = base.clone();
            match mutation {
                0 => v["extra"] = true.into(),
                1 => v["event"]["extra"] = true.into(),
                2 => v["event"]["stage"]["CriterionCandidate"]["extra"] = true.into(),
                3 => v["event"]["counters"]["extra"] = true.into(),
                4 => v["event"]["reserved"]["extra"] = true.into(),
                5 => v["sequence"] = 2.into(),
                6 => v["capture_sequence"] = 0.into(),
                7 => v["binding_sha256"] = "9".repeat(64).into(),
                8 => v["event"]["stage"]["CriterionCandidate"]["generator"] = 23.into(),
                9 => v["event"]["stage"]["CriterionCandidate"]["multiplier"] = 8.into(),
                10 => v["event"]["counters"]["acknowledged_events"] = 1.into(),
                _ => v["event"]["reserved"]["work"] = u64::MAX.into(),
            }
            let mut bytes = serde_json::to_vec(&v).unwrap();
            bytes.push(b'\n');
            assert!(
                decode_boundary_record(
                    &bytes,
                    &digest,
                    1,
                    23,
                    native::Engine::F5KoszulFrobenius,
                    3
                )
                .is_err(),
                "mutation {mutation}"
            );
        }
    }
    #[test]
    fn boundary_decode_enforces_line_and_engine_limits() {
        let digest = matrix_tests::binding().digest().unwrap();
        let bytes = encode(candidate(), 1);
        assert!(
            decode_boundary_record(&bytes, &digest, 1, 23, native::Engine::F4Macaulay, 3).is_err()
        );
        assert!(decode_boundary_record(
            &bytes[..bytes.len() - 1],
            &digest,
            1,
            23,
            native::Engine::F5KoszulFrobenius,
            3
        )
        .is_err());
        let mut oversized = vec![b' '; 2049];
        oversized.push(b'\n');
        assert!(decode_boundary_record(
            &oversized,
            &digest,
            1,
            23,
            native::Engine::F5KoszulFrobenius,
            3
        )
        .is_err());
    }
}

#[derive(Default)]
struct BoundaryStreamAssessment {
    records: usize,
    construction_captured: usize,
    matrix_captured: usize,
    construction_complete_captured: bool,
    matrix_finish_captured: bool,
    last_capture_sequence: u64,
}
fn native_counter_values(
    c: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Counters,
) -> [u64; 15] {
    [
        c.main_attempts,
        c.nonempty_rows,
        c.selected_rows,
        c.pruned_nonempty_rows,
        c.criterion_attempts,
        c.criterion_rows,
        c.criterion_zero_reductions,
        c.koszul_pruned,
        c.frobenius_pruned,
        c.criterion_word_xors,
        c.reduction_word_xors,
        c.attempted_events,
        c.acknowledged_events,
        c.dynamic_layout_bytes,
        c.allocation_requests,
    ]
}
/// Validate a captured boundary prefix, not native execution or completion.
/// Authentication against the capture chain and result counters remains separate.
fn verify_boundary_stream(
    bytes: &[u8],
    binding_sha256: &str,
    equations: usize,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
) -> Result<BoundaryStreamAssessment, String> {
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    if bytes.len() > 4096 * 2048 || equations > 61 || !(1..=6).contains(&degree) {
        return Err("native boundary stream exceeds cap".into());
    }
    crate::store::index::validate_sha256(binding_sha256)?;
    let mut out = BoundaryStreamAssessment::default();
    let mut reserved = native::Reservation::default();
    let mut counters = [0u64; 15];
    // Matrix phase ranks: 0=not started, 1=begin, 2=criterion,
    // 3=main, 4=pack, 5=reduce, 6=readback, 7=finish.
    let mut matrix_phase = 0u8;
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        let row = decode_boundary_record(
            line,
            binding_sha256,
            (out.records + 1) as u64,
            equations,
            engine,
            degree,
        )?;
        if row.capture_sequence <= out.last_capture_sequence {
            return Err("native capture sequence does not increase".into());
        }
        let current = match row.event {
            NativeBoundary::Construction {
                stage,
                reserved: current,
            } => {
                if matrix_phase != 0
                    || out.construction_complete_captured
                    || ((out.construction_captured == 0)
                        != (stage == ConstructionBoundaryStage::Begin))
                {
                    return Err("construction boundary order mismatch".into());
                }
                out.construction_captured += 1;
                out.construction_complete_captured = stage == ConstructionBoundaryStage::Complete;
                current
            }
            NativeBoundary::Matrix {
                stage,
                reserved: current,
                counters: next,
            } => {
                if !out.construction_complete_captured || out.matrix_finish_captured {
                    return Err("matrix boundary lacks completed construction prefix".into());
                }
                let next_phase = match stage {
                    MatrixBoundaryStage::Begin => 1,
                    MatrixBoundaryStage::CriterionLevel { .. }
                    | MatrixBoundaryStage::CriterionCandidate { .. } => 2,
                    MatrixBoundaryStage::MainCandidate { .. } => 3,
                    MatrixBoundaryStage::Pack => 4,
                    MatrixBoundaryStage::Reduce => 5,
                    MatrixBoundaryStage::Readback => 6,
                    MatrixBoundaryStage::Finish => 7,
                };
                let valid = match next_phase {
                    1 => matrix_phase == 0,
                    2 => matches!(matrix_phase, 1 | 2),
                    3 => matches!(matrix_phase, 1 | 2 | 3),
                    4 => matches!(matrix_phase, 1 | 2 | 3),
                    5..=7 => matrix_phase + 1 == next_phase,
                    _ => false,
                };
                if !valid || next.attempted_events != (out.matrix_captured + 1) as u64 {
                    return Err("matrix boundary phase or native event gap".into());
                }
                if matrix_phase == 0 {
                    reserved = native::Reservation::default();
                }
                let values = native_counter_values(next);
                if values
                    .iter()
                    .zip(counters)
                    .any(|(&next, previous)| next < previous)
                {
                    return Err("native boundary counters regress".into());
                }
                counters = values;
                matrix_phase = next_phase;
                out.matrix_captured += 1;
                out.matrix_finish_captured = next_phase == 7;
                current
            }
        };
        if current.work < reserved.work || current.bytes < reserved.bytes {
            return Err("native boundary reservations regress".into());
        }
        reserved = current;
        out.records += 1;
        out.last_capture_sequence = row.capture_sequence;
    }
    Ok(out)
}

const NATIVE_BOUNDARY_CAP: usize = 2048;
const NATIVE_BOUNDARY_RECORDS: usize = 4096;
#[derive(Clone, Copy)]
struct PendingNativeBoundary {
    sequence: u64,
    capture_sequence: u64,
    event: NativeBoundary,
}
#[derive(serde::Serialize)]
struct NativeBoundaryView<'a> {
    schema: &'static str,
    binding_sha256: &'a str,
    sequence: u64,
    capture_sequence: u64,
    event: &'a NativeBoundary,
}
struct BoundNativeEvents<'a> {
    binding_digest: [u8; 64],
    limits: &'a crate::ml_dataset::capture::CaptureLimits,
    buffer: crate::ml_dataset::capture::TypedEventBuffer,
    canonical: Vec<u8>,
    pending_bytes: Vec<u8>,
    pending: Option<PendingNativeBoundary>,
    acknowledged: u64,
    encoded_capture_bytes: u64,
}
impl<'a> BoundNativeEvents<'a> {
    fn new(
        binding: &crate::ml_dataset::capture::AttemptBinding,
        limits: &'a crate::ml_dataset::capture::CaptureLimits,
        budget: &mut Budget,
    ) -> Result<Self, Stop> {
        use crate::ml_dataset::capture::TypedEventBuffer;
        budget.reserve_checking(64 * 1024, 4096, Instant::now())?;
        if binding.validate().is_err() || limits.validate().is_err() {
            return budget.fail(Stop::InvalidLimits);
        }
        let digest = binding
            .digest()
            .or_else(|_| budget.fail(Stop::InvalidLimits))?;
        let mut binding_digest = [0; 64];
        binding_digest.copy_from_slice(digest.as_bytes());
        let wire = TypedEventBuffer::requested_bytes(NATIVE_BOUNDARY_CAP as u64)
            .or_else(|_| budget.fail(Stop::Overflow))?;
        let canonical_cap = NATIVE_BOUNDARY_RECORDS
            .checked_mul(NATIVE_BOUNDARY_CAP)
            .ok_or(Stop::Overflow)
            .or_else(|s| budget.fail(s))?;
        let bytes = (std::mem::size_of::<Self>() as u64)
            .checked_add(wire)
            .and_then(|v| v.checked_add(canonical_cap as u64))
            .and_then(|v| v.checked_add(NATIVE_BOUNDARY_CAP as u64))
            .ok_or(Stop::Overflow)
            .or_else(|s| budget.fail(s))?;
        budget.reserve_bytes(bytes, Instant::now())?;
        let mut canonical = Vec::new();
        let mut pending_bytes = Vec::new();
        if canonical.try_reserve_exact(canonical_cap).is_err()
            || pending_bytes
                .try_reserve_exact(NATIVE_BOUNDARY_CAP)
                .is_err()
        {
            return budget.fail(Stop::Allocation);
        }
        let buffer = TypedEventBuffer::try_new(NATIVE_BOUNDARY_CAP as u64, wire)
            .or_else(|_| budget.fail(Stop::Allocation))?;
        Ok(Self {
            binding_digest,
            limits,
            buffer,
            canonical,
            pending_bytes,
            pending: None,
            acknowledged: 0,
            encoded_capture_bytes: 0,
        })
    }
    fn emit(
        &mut self,
        event: NativeBoundary,
        budget: &mut Budget,
        global: &mut usize,
    ) -> Result<(), Stop> {
        use std::io::Write;
        if self.pending.is_some() {
            return budget.fail(Stop::Observer);
        }
        if *global > NATIVE_BOUNDARY_RECORDS {
            return budget.fail(Stop::InvalidLimits);
        }
        let snapshot = PendingNativeBoundary {
            sequence: self.acknowledged + 1,
            capture_sequence: *global as u64 + 1,
            event,
        };
        self.pending = Some(snapshot);
        if *global == NATIVE_BOUNDARY_RECORDS || self.acknowledged == NATIVE_BOUNDARY_RECORDS as u64
        {
            return budget.fail(Stop::Events);
        }
        // Typed capture encoding retains its existing 8-per-byte model. The
        // canonical payload is now copied from that encoding (1 per byte),
        // eliminating the separate 8-per-byte canonical serialization pass.
        budget.reserve_checking(1024 + 9 * NATIVE_BOUNDARY_CAP as u64, 4096, Instant::now())?;
        let identity = super::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
            Some(super::EventSink::Bound(sink)) => sink.verify_digest_limits_at_count(
                &self.binding_digest,
                self.limits,
                *global as u64,
            ),
            _ => Err("native Koblitz boundary requires original bound capture".into()),
        });
        if identity.is_err() {
            return budget.fail(Stop::InvalidLimits);
        }
        budget.reserve_trace(NATIVE_BOUNDARY_CAP as u64, Instant::now())?;
        let digest = std::str::from_utf8(&self.binding_digest)
            .or_else(|_| budget.fail(Stop::InvalidLimits))?;
        let view = NativeBoundaryView {
            schema: "crypto.ml-dataset.koblitz-m3-events/v1",
            binding_sha256: digest,
            sequence: snapshot.sequence,
            capture_sequence: snapshot.capture_sequence,
            event: &snapshot.event,
        };
        let captured = super::LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
            Some(super::EventSink::Bound(sink)) => sink.event_typed_precommit(
                crate::ml_dataset::capture::TypedEventKind::KoblitzM3Native,
                &view,
                &mut self.buffer,
                &mut |wire| {
                    use crate::ml_dataset::capture::CaptureError;
                    let payload = match native_capture_payload(wire) {
                        Ok(payload) => payload,
                        Err(stop) => {
                            let _ = budget.fail::<()>(stop);
                            return Err(CaptureError::Integrity(
                                "native payload extraction failed".into(),
                            ));
                        }
                    };
                    let bytes = std::mem::take(&mut self.pending_bytes);
                    let mut writer = InputWriter {
                        bytes,
                        capacity: NATIVE_BOUNDARY_CAP,
                        budget,
                    };
                    // Preserve any byte prefix that fits the original trace
                    // allowance before refusing the remaining unacknowledged copy.
                    let prefix = payload.len().min(
                        writer
                            .budget
                            .trace_limit
                            .saturating_sub(writer.budget.trace_bytes)
                            as usize,
                    );
                    let encoded = writer.write_all(&payload[..prefix]).is_ok()
                        && writer.write_all(&payload[prefix..]).is_ok()
                        && writer.write_all(b"\n").is_ok();
                    self.pending_bytes = writer.bytes;
                    if !encoded {
                        return Err(CaptureError::Integrity(
                            "canonical boundary retention refused".into(),
                        ));
                    }
                    if self.pending_bytes.len()
                        > NATIVE_BOUNDARY_RECORDS * NATIVE_BOUNDARY_CAP - self.canonical.len()
                    {
                        let _ = budget.fail::<()>(Stop::Trace);
                        return Err(CaptureError::Integrity(
                            "canonical boundary capacity exhausted".into(),
                        ));
                    }
                    Ok(())
                },
            ),
            _ => Err(crate::ml_dataset::capture::CaptureError::Integrity(
                "native boundary requires bound capture".into(),
            )),
        });
        if captured.is_err() {
            return budget.fail(Stop::Observer);
        }
        // All storage was pre-admitted. Acknowledgement survives a later clock
        // failure; callers must not reinterpret it as an unacknowledged event.
        self.canonical.extend_from_slice(&self.pending_bytes);
        self.pending_bytes.clear();
        self.pending = None;
        self.acknowledged += 1;
        *global += 1;
        self.encoded_capture_bytes += self.buffer.encoded_len() as u64;
        let _ = budget.check(Instant::now());
        Ok(())
    }
}

#[cfg(test)]
mod native_boundary_emitter_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native_matrix;
    use crate::ml_dataset::capture::{capture_limits, verify_live_committed_prefix};
    fn begin() -> NativeBoundary {
        NativeBoundary::Construction {
            stage: ConstructionBoundaryStage::Begin,
            reserved: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Reservation::default(
            ),
        }
    }
    #[test]
    fn native_callbacks_are_bound_to_the_actual_capture_stream() {
        for engine in [
            native_matrix::Engine::F4Macaulay,
            native_matrix::Engine::F5KoszulFrobenius,
        ] {
            let binding = matrix_tests::binding();
            let limits = capture_limits(16).unwrap();
            let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
            let original = construction_tests::fixture();
            let mut budget = construction_tests::budget();
            let mut events = BoundNativeEvents::new(&binding, &limits, &mut budget).unwrap();
            let mut global = 0;
            let construction = construct_original(&original, &mut budget, &mut |event, b| {
                events
                    .emit(NativeBoundary::construction(event), b, &mut global)
                    .map_err(|_| ())
            });
            assert_eq!(construction.adapter_stop, None);
            let output = construction.output.unwrap();
            let input = persist_original(
                &original,
                &output,
                &mut budget,
                &binding,
                &"8".repeat(64),
                &"9".repeat(64),
                &mut |_, _| Ok(()),
            )
            .unwrap();
            let step = matrix_step(input, engine, 3, &mut |event, b| {
                events
                    .emit(NativeBoundary::matrix(event), b, &mut global)
                    .map_err(|_| ())
            });
            assert_eq!(step.adapter_stop, None);
            let assessment = verify_boundary_stream(
                &events.canonical,
                &binding.digest().unwrap(),
                23,
                engine,
                3,
            )
            .unwrap();
            assert!(assessment.construction_complete_captured && assessment.matrix_finish_captured);
            assert_eq!(assessment.records, global);
            assert!(events.pending.is_none());
            let captured = std::fs::read(sink.dir.join("events.jsonl")).unwrap();
            let rows = captured
                .split(|b| *b == b'\n')
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>();
            let canonical = events
                .canonical
                .split(|b| *b == b'\n')
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>();
            assert_eq!(rows.len(), canonical.len());
            for (i, (wire, saved)) in rows.iter().zip(canonical).enumerate() {
                let row: serde_json::Value = serde_json::from_slice(wire).unwrap();
                let record: serde_json::Value = serde_json::from_slice(saved).unwrap();
                assert_eq!(row["sequence"], (i + 1) as u64);
                assert_eq!(row["event"]["kind"], "koblitz_m3_native");
                assert_eq!(row["event"]["payload"], record);
            }
            assert_eq!(events.encoded_capture_bytes, captured.len() as u64);
            assert!(
                verify_live_committed_prefix(sink.dir.parent().unwrap(), &binding, &limits)
                    .unwrap()
                    .is_some()
            );
        }
    }
    #[test]
    fn incorrect_shared_counter_refuses_publication_and_retains_snapshot() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut budget = construction_tests::budget();
        let mut events = BoundNativeEvents::new(&binding, &limits, &mut budget).unwrap();
        let mut global = 1;
        assert_eq!(
            events.emit(begin(), &mut budget, &mut global),
            Err(Stop::InvalidLimits)
        );
        assert_eq!(events.pending.as_ref().unwrap().capture_sequence, 2);
        assert_eq!(events.acknowledged, 0);
        assert!(events.canonical.is_empty());
        assert_eq!(
            std::fs::metadata(sink.dir.join("events.jsonl"))
                .unwrap()
                .len(),
            0
        );
    }
    #[test]
    fn partial_serialization_and_event_cap_preserve_unacknowledged_evidence() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut budget = construction_tests::budget();
        let mut events = BoundNativeEvents::new(&binding, &limits, &mut budget).unwrap();
        budget.trace_limit = NATIVE_BOUNDARY_CAP as u64 + 20;
        let mut global = 0;
        assert_eq!(
            events.emit(begin(), &mut budget, &mut global),
            Err(Stop::Trace)
        );
        assert!(events.pending.is_some());
        assert!(!events.pending_bytes.is_empty());
        assert!(events.pending_bytes.len() <= 20);
        let retained = events.pending_bytes.clone();
        assert_eq!(
            events.emit(begin(), &mut budget, &mut global),
            Err(Stop::Trace)
        );
        assert_eq!(events.pending_bytes, retained);
        assert_eq!(global, 0);
        assert_eq!(
            std::fs::metadata(sink.dir.join("events.jsonl"))
                .unwrap()
                .len(),
            0
        );
        let mut budget = construction_tests::budget();
        let mut events = BoundNativeEvents::new(&binding, &limits, &mut budget).unwrap();
        global = NATIVE_BOUNDARY_RECORDS;
        assert_eq!(
            events.emit(begin(), &mut budget, &mut global),
            Err(Stop::Events)
        );
        assert_eq!(events.pending.unwrap().capture_sequence, 4097);
        assert!(events.canonical.is_empty());
    }
    #[test]
    fn storage_admission_failure_is_sticky_before_any_capture() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let mut budget = construction_tests::budget();
        budget.memory_limit = 4096;
        assert!(matches!(
            BoundNativeEvents::new(&binding, &limits, &mut budget),
            Err(Stop::Bytes)
        ));
        assert_eq!(budget.stopped, Some(Stop::Bytes));
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedNativeCaptureEvent {
    kind: String,
    payload: SavedBoundaryRecord,
}
/// Canonical bytes are borrowed from the sealed artifact; matching adds no
/// second retained stream. Return observations only after full custody succeeds.
fn verify_native_capture(
    cell: &std::path::Path,
    canonical: &[u8],
    binding: &crate::ml_dataset::capture::AttemptBinding,
    limits: &crate::ml_dataset::capture::CaptureLimits,
    equations: usize,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
) -> Result<BoundaryStreamAssessment, String> {
    let digest = binding.digest()?;
    let assessment = verify_boundary_stream(canonical, &digest, equations, engine, degree)?;
    let mut records = canonical.split_inclusive(|b| *b == b'\n');
    let mut matched = 0;
    let custody = crate::ml_dataset::capture::verify_live_committed_prefix_visit(
        cell,
        binding,
        limits,
        &mut |line| {
            let row: crate::ml_dataset::capture::EventRecord =
                serde_json::from_slice(line).map_err(|e| format!("native capture record: {e}"))?;
            if row.event["kind"] != "koblitz_m3_native" {
                return Ok(());
            }
            let event: SavedNativeCaptureEvent = serde_json::from_value(row.event)
                .map_err(|e| format!("strict native capture event: {e}"))?;
            let bytes = records.next().ok_or("extra native capture event")?;
            let expected =
                decode_boundary_record(bytes, &digest, matched + 1, equations, engine, degree)?;
            if event.kind != "koblitz_m3_native"
                || event.payload.schema != expected.schema
                || event.payload.binding_sha256 != expected.binding_sha256
                || event.payload.sequence != expected.sequence
                || event.payload.capture_sequence != expected.capture_sequence
                || event.payload.capture_sequence != row.sequence
                || event.payload.event != expected.event
            {
                return Err("native canonical record differs from authenticated capture".into());
            }
            matched += 1;
            Ok(())
        },
    )?;
    if custody.is_none() || records.next().is_some() || matched as usize != assessment.records {
        return Err("missing committed native capture records".into());
    }
    Ok(assessment)
}

#[cfg(test)]
mod native_capture_match_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn native_capture_rejects_reencoded_canonical_mutations_and_missing_watermark() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut budget = construction_tests::budget();
        let original = construction_tests::fixture();
        let mut events = BoundNativeEvents::new(&binding, &limits, &mut budget).unwrap();
        let mut global = 0;
        // Non-native capture records consume the same sequence space.
        super::super::live_event(&serde_json::json!({"kind":"adapter_control"})).unwrap();
        global += 1;
        let construction = construct_original(&original, &mut budget, &mut |event, b| {
            events
                .emit(NativeBoundary::construction(event), b, &mut global)
                .map_err(|_| ())
        });
        assert_eq!(construction.adapter_stop, None);
        let check = |bytes: &[u8]| {
            verify_native_capture(
                sink.dir.parent().unwrap(),
                bytes,
                &binding,
                &limits,
                23,
                native::Engine::F4Macaulay,
                3,
            )
        };
        let verified = check(&events.canonical).unwrap();
        assert!(verified.construction_complete_captured);
        assert!(!verified.matrix_finish_captured);
        assert_eq!(verified.last_capture_sequence as usize, global);
        let rows = events
            .canonical
            .split_inclusive(|b| *b == b'\n')
            .collect::<Vec<_>>();
        assert!(check(&rows[..rows.len() - 1].concat()).is_err());
        assert!(check(b"").is_err());
        for mutation in 0..3 {
            let mut changed = Vec::new();
            for (i, line) in rows.iter().enumerate() {
                let mut value: serde_json::Value = serde_json::from_slice(line).unwrap();
                match mutation {
                    // Still an increasing sequence, but no longer the capture sequence.
                    0 => value["capture_sequence"] = serde_json::json!(i + 1),
                    1 if i + 1 == rows.len() => {
                        value["event"]["reserved"]["work"] = serde_json::json!(
                            value["event"]["reserved"]["work"].as_u64().unwrap() + 1
                        )
                    }
                    2 if i == 0 => value["unexpected"] = serde_json::json!(true),
                    _ => {}
                }
                serde_json::to_writer(&mut changed, &value).unwrap();
                changed.push(b'\n');
            }
            assert!(check(&changed).is_err(), "mutation {mutation}");
        }
        let head = sink.dir.join("committed.json");
        let saved = std::fs::read(&head).unwrap();
        std::fs::remove_file(&head).unwrap();
        assert!(check(&events.canonical).is_err());
        std::fs::write(head, saved).unwrap();
        assert!(check(&events.canonical).is_ok());
    }
    #[test]
    fn capture_acknowledgement_survives_native_observer_deadline_failure() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut budget = construction_tests::budget();
        let original = construction_tests::fixture();
        let mut events = BoundNativeEvents::new(&binding, &limits, &mut budget).unwrap();
        let mut global = 0;
        let result = construct_original(&original, &mut budget, &mut |event, b| {
            events
                .emit(NativeBoundary::construction(event), b, &mut global)
                .map_err(|_| ())?;
            // Deterministic deadline fault after a real durable acknowledgement.
            b.deadline = Instant::now();
            b.construction_deadline = b.deadline;
            Ok(())
        });
        assert_eq!(result.adapter_stop, Some(Stop::Deadline));
        assert!(result.output.is_none());
        assert_eq!(events.acknowledged, 1);
        assert_eq!(global, 1);
        assert!(events.pending.is_none());
        let verified = verify_native_capture(
            sink.dir.parent().unwrap(),
            &events.canonical,
            &binding,
            &limits,
            23,
            native::Engine::F4Macaulay,
            3,
        )
        .unwrap();
        assert_eq!(verified.records, 1);
        assert!(!verified.construction_complete_captured);
    }
}

/// Allocation-free snapshot nested in the versioned terminal result.
/// Saved modeled reservations and reported calls, not hardware instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedBudgetLedger {
    original_limits: OriginalLimits,
    phase: Phase,
    first_stop: Option<Stop>,
    native_reserved: Reservation,
    checking_reserved_work: u64,
    checking_reported_work: u64,
    adapter_reserved_bytes: u64,
    charged_trace_bytes: u64,
    group_additions: u64,
}
impl Budget {
    fn saved_ledger(&self) -> SavedBudgetLedger {
        SavedBudgetLedger {
            original_limits: self.original_limits(),
            phase: self.phase,
            first_stop: self.stopped,
            native_reserved: self.native,
            checking_reserved_work: self.checking_work,
            checking_reported_work: self.checking_performed_work,
            adapter_reserved_bytes: self.adapter_bytes,
            charged_trace_bytes: self.trace_bytes,
            group_additions: self.additions,
        }
    }
}
struct BudgetLedgerAssessment {
    reported_work: u64,
    reported_bytes: u64,
    reported_over_limit: bool,
    completion_eligible: bool,
}
fn verify_budget_ledger(
    bytes: &[u8],
    original: OriginalLimits,
) -> Result<BudgetLedgerAssessment, String> {
    if bytes.len() > 2048 {
        return Err("terminal ledger exceeds cap".into());
    }
    let saved: SavedBudgetLedger =
        serde_json::from_slice(bytes).map_err(|e| format!("strict terminal ledger: {e}"))?;
    if saved.original_limits != original
        || original.native_work == 0
        || original.native_work > NATIVE_WORK_MAX
        || original.memory == 0
        || original.trace == 0
        || original.additions == 0
        || saved.checking_reported_work > saved.checking_reserved_work
    {
        return Err("terminal ledger identity or checking work mismatch".into());
    }
    let work = saved
        .native_reserved
        .work
        .checked_add(saved.checking_reserved_work)
        .ok_or("terminal work overflow")?;
    let bytes = saved
        .native_reserved
        .bytes
        .checked_add(saved.adapter_reserved_bytes)
        .ok_or("terminal byte overflow")?;
    let over = work > original.native_work
        || bytes > original.memory
        || saved.charged_trace_bytes > original.trace
        || saved.group_additions > original.additions;
    // A retained late native settlement may exceed remaining shared headroom.
    // Preserve that reported fact, but never accept an uncensored completion.
    if over && saved.first_stop.is_none() {
        return Err("uncensored terminal resource excess".into());
    }
    Ok(BudgetLedgerAssessment {
        reported_work: work,
        reported_bytes: bytes,
        reported_over_limit: over,
        completion_eligible: !over && saved.first_stop.is_none() && saved.phase == Phase::Complete,
    })
}
#[cfg(test)]
mod terminal_ledger_tests {
    use super::*;
    fn check(b: &Budget) -> Result<BudgetLedgerAssessment, String> {
        verify_budget_ledger(
            &serde_json::to_vec(&b.saved_ledger()).unwrap(),
            b.original_limits(),
        )
    }
    #[test]
    fn actual_native_reservations_roundtrip_without_claiming_pipeline_completion() {
        let original = construction_tests::fixture();
        let mut b = construction_tests::budget();
        let result = construct_original(&original, &mut b, &mut |_, _| Ok(()));
        assert!(result.output.is_some() && result.adapter_stop.is_none());
        let a = check(&b).unwrap();
        assert_eq!(a.reported_work, b.native.work + b.checking_work);
        assert_eq!(a.reported_bytes, b.native.bytes + b.adapter_bytes);
        assert!(!a.reported_over_limit && !a.completion_eligible);
    }
    #[test]
    fn late_native_settlement_preserves_excess_and_first_stop() {
        let mut b = construction_tests::budget();
        b.begin_native(false, Instant::now()).unwrap();
        b.reserve_checking(1, 0, Instant::now()).unwrap();
        assert_eq!(b.fail::<()>(Stop::Observer), Err(Stop::Observer));
        let final_native = Reservation {
            work: b.phase_limits.work,
            bytes: 0,
        };
        assert_eq!(b.settle_native_terminal(final_native), Err(Stop::Observer));
        let a = check(&b).unwrap();
        assert!(a.reported_over_limit && !a.completion_eligible);
        assert_eq!(a.reported_work, final_native.work + 1);
        assert_eq!(b.saved_ledger().first_stop, Some(Stop::Observer));
        let mut saved = b.saved_ledger();
        saved.first_stop = None;
        assert!(
            verify_budget_ledger(&serde_json::to_vec(&saved).unwrap(), b.original_limits())
                .is_err()
        );
    }
    #[test]
    fn ledger_rejects_original_limit_changes_unknown_fields_and_integer_overflow() {
        let b = construction_tests::budget();
        let baseline = serde_json::to_value(b.saved_ledger()).unwrap();
        for mutation in 0..5 {
            let mut v = baseline.clone();
            match mutation {
                0 => v["original_limits"]["native_work"] = serde_json::json!(1),
                1 => v["original_limits"]["extra"] = serde_json::json!(true),
                2 => {
                    v["native_reserved"]["work"] = serde_json::json!(u64::MAX);
                    v["checking_reserved_work"] = serde_json::json!(1);
                }
                3 => v["checking_reported_work"] = serde_json::json!(1),
                4 => v["native_reserved"]["extra"] = serde_json::json!(true),
                _ => unreachable!(),
            }
            assert!(
                verify_budget_ledger(&serde_json::to_vec(&v).unwrap(), b.original_limits())
                    .is_err(),
                "mutation {mutation}"
            );
        }
        assert!(verify_budget_ledger(&vec![b' '; 2049], b.original_limits()).is_err());
    }
}

/// Retained native-phase artifacts; no claim of signed-domain or cell completion.
struct NativePhaseRun<'a> {
    construction: Option<Construction>,
    persistence_failure: Option<PersistenceFailure>,
    matrix: Option<MatrixStep>,
    events: Option<BoundNativeEvents<'a>>,
    ledger: SavedBudgetLedger,
}
fn run_native_phases<'a>(
    original: &MathInput,
    binding: &crate::ml_dataset::capture::AttemptBinding,
    capture_limits: &'a crate::ml_dataset::capture::CaptureLimits,
    public_sha256: &str,
    factor_base_sha256: &str,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    budget: &mut Budget,
    global: &mut usize,
    persist: &mut impl FnMut(&[u8], &mut Budget, &mut usize) -> Result<(), String>,
) -> NativePhaseRun<'a> {
    let mut out = NativePhaseRun {
        construction: None,
        persistence_failure: None,
        matrix: None,
        events: None,
        ledger: budget.saved_ledger(),
    };
    let result = (|| -> Result<(), Stop> {
        if budget.phase != Phase::Setup {
            return budget.fail(Stop::Phase);
        }
        budget.reserve_bytes(
            std::mem::size_of::<NativePhaseRun<'_>>() as u64,
            Instant::now(),
        )?;
        out.events = Some(BoundNativeEvents::new(binding, capture_limits, budget)?);
        let events = out.events.as_mut().unwrap();
        out.construction = Some(construct_original(original, budget, &mut |event, b| {
            events
                .emit(NativeBoundary::construction(event), b, global)
                .map_err(|_| ())
        }));
        let construction = out.construction.as_ref().unwrap();
        if let Some(stop) = construction.adapter_stop {
            return Err(stop);
        }
        let Some(output) = construction.output.as_ref() else {
            return budget.fail(Stop::NativeFailure);
        };
        let input = match persist_original(
            original,
            output,
            budget,
            binding,
            public_sha256,
            factor_base_sha256,
            &mut |bytes, b| persist(bytes, b, global),
        ) {
            Ok(input) => input,
            Err(failure) => {
                out.persistence_failure = Some(failure);
                return Err(Stop::Observer);
            }
        };
        out.matrix = Some(matrix_step(input, engine, degree, &mut |event, b| {
            events
                .emit(NativeBoundary::matrix(event), b, global)
                .map_err(|_| ())
        }));
        if let Some(stop) = out.matrix.as_ref().unwrap().adapter_stop {
            return Err(stop);
        }
        Ok(())
    })();
    if let Err(stop) = result {
        let _ = budget.fail::<()>(stop);
    }
    out.ledger = budget.saved_ledger();
    out
}
#[cfg(test)]
mod native_phase_driver_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn original_native_phases_share_capture_and_preserve_exact_persisted_input() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            let binding = matrix_tests::binding();
            let limits = capture_limits(16).unwrap();
            let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
            let original = construction_tests::fixture();
            let mut budget = construction_tests::budget();
            let mut global = 0;
            let mut persisted = Vec::new();
            let run = run_native_phases(
                &original,
                &binding,
                &limits,
                &"8".repeat(64),
                &"9".repeat(64),
                engine,
                3,
                &mut budget,
                &mut global,
                &mut |bytes, _, _| {
                    persisted = bytes.to_vec();
                    Ok(())
                },
            );
            assert!(run.ledger.first_stop.is_none());
            assert_eq!(run.ledger.phase, Phase::MatrixComplete);
            assert!(run.persistence_failure.is_none());
            assert!(run.construction.as_ref().unwrap().output.is_some());
            let matrix = run.matrix.as_ref().unwrap();
            assert_eq!(matrix.original_bytes, persisted);
            assert!(matrix.report.as_ref().unwrap().native_step_complete);
            let events = run.events.as_ref().unwrap();
            let replay = verify_native_capture(
                sink.dir.parent().unwrap(),
                &events.canonical,
                &binding,
                &limits,
                23,
                engine,
                3,
            )
            .unwrap();
            assert!(replay.construction_complete_captured && replay.matrix_finish_captured);
            assert_eq!(replay.records, global);
            let ledger = verify_budget_ledger(
                &serde_json::to_vec(&run.ledger).unwrap(),
                budget.original_limits(),
            )
            .unwrap();
            assert!(!ledger.completion_eligible);
        }
    }
    #[test]
    fn persistence_failure_retains_input_and_prevents_matrix_execution() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let mut global = 0;
        let run = run_native_phases(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Err("selected persistence failure".into()),
        );
        assert_eq!(run.ledger.first_stop, Some(Stop::Observer));
        assert!(run.matrix.is_none());
        let failure = run.persistence_failure.as_ref().unwrap();
        assert!(failure.serialized_complete && !failure.bytes.is_empty());
        assert_eq!(failure.reason, "selected persistence failure");
        assert!(run.construction.as_ref().unwrap().output.is_some());
    }
    #[test]
    fn capture_trace_refusal_retains_pending_boundary_and_skips_downstream_work() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        budget.trace_limit = 1;
        let mut global = 0;
        let mut persistence_called = false;
        let run = run_native_phases(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| {
                persistence_called = true;
                Ok(())
            },
        );
        assert_eq!(run.ledger.first_stop, Some(Stop::Trace));
        assert!(!persistence_called && run.matrix.is_none());
        assert!(run.events.as_ref().unwrap().pending.is_some());
        assert!(run.events.as_ref().unwrap().canonical.is_empty());
        assert_eq!(global, 0);
    }
}

/// In-memory assembly. Canonical result serialization and parent publication
/// remain required before this can produce a completed dataset cell.
struct BoundPipelineRun<'a> {
    native: NativePhaseRun<'a>,
    signed: Option<SignedEnumeration>,
    signed_events: Option<BoundSignedEvents<'a>>,
    publisher: Option<BoundSignedPublisher<'a>>,
    ledger: SavedBudgetLedger,
}
fn run_bound_pipeline<'a>(
    original: &MathInput,
    binding: &'a crate::ml_dataset::capture::AttemptBinding,
    capture_limits: &'a crate::ml_dataset::capture::CaptureLimits,
    public_sha256: &str,
    factor_base_sha256: &str,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    budget: &mut Budget,
    global: &mut usize,
    persist: &mut impl FnMut(&[u8], &mut Budget, &mut usize) -> Result<(), String>,
) -> BoundPipelineRun<'a> {
    let native = run_native_phases(
        original,
        binding,
        capture_limits,
        public_sha256,
        factor_base_sha256,
        engine,
        degree,
        budget,
        global,
        persist,
    );
    let mut out = BoundPipelineRun {
        native,
        signed: None,
        signed_events: None,
        publisher: None,
        ledger: budget.saved_ledger(),
    };
    let result = (|| -> Result<(), Stop> {
        budget.check(Instant::now())?;
        if budget.phase != Phase::MatrixComplete
            || !out
                .native
                .matrix
                .as_ref()
                .and_then(|m| m.report.as_ref())
                .is_some_and(|r| r.native_step_complete)
        {
            return budget.fail(Stop::Phase);
        }
        budget.reserve_bytes(
            std::mem::size_of::<BoundPipelineRun<'_>>() as u64,
            Instant::now(),
        )?;
        out.signed_events = Some(BoundSignedEvents::new(binding, capture_limits, budget)?);
        out.publisher = Some(BoundSignedPublisher::new(binding, capture_limits, budget));
        let events = out.signed_events.as_mut().unwrap();
        let publisher = out.publisher.as_mut().unwrap();
        let prepared = prepare_signed(original, budget, true)?;
        // Both callbacks mutate the caller's actual shared counter immediately.
        // This is a borrow of that counter, not a fresh event allowance.
        let counter = std::cell::RefCell::new(global);
        out.signed = Some(enumerate_prepared_signed(
            prepared,
            &mut |event, b| events.emit(event, b, &mut **counter.borrow_mut()),
            &mut |witness, b| publisher.publish(witness, b, &mut **counter.borrow_mut()),
        ));
        if let Some(stop) = out.signed.as_ref().unwrap().stop {
            return Err(stop);
        }
        Ok(())
    })();
    if let Err(stop) = result {
        let _ = budget.fail::<()>(stop);
    }
    out.ledger = budget.saved_ledger();
    out
}
#[cfg(test)]
mod bound_pipeline_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn actual_native_and_signed_phases_share_one_capture_counter_for_both_engines() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            for positive in [false, true] {
                let binding = matrix_tests::binding();
                let limits = capture_limits(16).unwrap();
                let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
                let mut original = construction_tests::fixture();
                if positive {
                    original.target = original.signed[0];
                }
                let mut budget = construction_tests::budget();
                let mut global = 0;
                let run = run_bound_pipeline(
                    &original,
                    &binding,
                    &limits,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    engine,
                    3,
                    &mut budget,
                    &mut global,
                    &mut |_, _, _| Ok(()),
                );
                assert_eq!(run.ledger.first_stop, None);
                assert_eq!(run.native.ledger.phase, Phase::MatrixComplete);
                assert_eq!(run.ledger.phase, Phase::Complete);
                let signed = run.signed.as_ref().unwrap();
                assert!(signed.complete && signed.unacknowledged.is_none());
                assert_eq!(signed.acknowledged.len(), 192);
                assert_eq!(signed.enumeration_additions, 192);
                assert_eq!(signed.first_receipt.is_some(), positive);
                assert_eq!(signed.witness_check_additions, if positive { 3 } else { 0 });
                assert_eq!(
                    run.ledger.group_additions,
                    signed.enumeration_additions + signed.witness_check_additions
                );
                assert!(
                    verify_signed_prefix(&original, &signed.acknowledged, None)
                        .unwrap()
                        .all_tuples_acknowledged
                );
                let native_events = run.native.events.as_ref().unwrap();
                assert_eq!(
                    global as u64,
                    native_events.acknowledged + 192 + u64::from(positive)
                );
                assert!(
                    verify_native_capture(
                        sink.dir.parent().unwrap(),
                        &native_events.canonical,
                        &binding,
                        &limits,
                        23,
                        engine,
                        3
                    )
                    .unwrap()
                    .matrix_finish_captured
                );
                if positive {
                    let bytes = std::fs::read(sink.dir.join("payloads/pdp-witness.json")).unwrap();
                    verify_signed_witness(&bytes, &original).unwrap();
                    assert_eq!(
                        run.publisher.as_ref().unwrap().receipt,
                        signed.first_receipt
                    );
                }
            }
        }
    }
    #[test]
    fn capped_signed_stage_retains_native_completion_without_resetting_addition_allowance() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        budget.addition_limit = 2;
        let mut global = 0;
        let run = run_bound_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Ok(()),
        );
        assert_eq!(run.ledger.first_stop, Some(Stop::Additions));
        assert_eq!(run.native.ledger.phase, Phase::MatrixComplete);
        assert_eq!(run.ledger.group_additions, 2);
        let signed = run.signed.as_ref().unwrap();
        assert!(!signed.complete && signed.first_receipt.is_none());
        assert_eq!(signed.acknowledged.len(), 2);
        let events = run.native.events.as_ref().unwrap();
        assert!(
            verify_native_capture(
                sink.dir.parent().unwrap(),
                &events.canonical,
                &binding,
                &limits,
                23,
                native::Engine::F4Macaulay,
                3
            )
            .unwrap()
            .matrix_finish_captured
        );
        assert!(
            !verify_budget_ledger(
                &serde_json::to_vec(&run.ledger).unwrap(),
                budget.original_limits()
            )
            .unwrap()
            .completion_eligible
        );
    }
    #[test]
    fn native_persistence_failure_prevents_signed_setup_and_witness_publication() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let mut global = 0;
        let run = run_bound_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Err("selected persistence failure".into()),
        );
        assert_eq!(run.ledger.first_stop, Some(Stop::Observer));
        assert!(run.signed.is_none() && run.signed_events.is_none() && run.publisher.is_none());
        assert!(run.native.persistence_failure.is_some());
        assert!(!sink.dir.join("payloads/pdp-witness.json").exists());
    }
}
#[cfg(test)]
mod signed_shared_counter_tests {
    use super::*;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn signed_continuation_refuses_a_counter_that_differs_from_committed_capture() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut budget = construction_tests::budget();
        let mut events = BoundSignedEvents::new(&binding, &limits, &mut budget).unwrap();
        let event = SignedPrefix {
            sequence: 1,
            tuple: 0,
            indices: [0; 3],
            summand: 1,
            prefix: None,
            matches_target: false,
        };
        let mut global = 1;
        assert_eq!(
            events.emit(&event, &mut budget, &mut global),
            Err(Stop::InvalidLimits)
        );
        assert_eq!(global, 1);
        assert_eq!(events.acknowledged, 0);
        assert_eq!(
            std::fs::metadata(sink.dir.join("events.jsonl"))
                .unwrap()
                .len(),
            0
        );
    }
}

/// One-use terminal encoding allowance, debited before native execution.
/// Retains partial raw bytes on failure; only `complete` permits canonical use.
struct TerminalEncoding {
    bytes: Vec<u8>,
    cap: usize,
    deadline: Instant,
    reserved_work: u64,
    performed_work: u64,
    attempted: bool,
    complete: bool,
    stop: Option<Stop>,
}
impl TerminalEncoding {
    fn admit(cap: usize, budget: &mut Budget) -> Result<Self, Stop> {
        if cap == 0 || cap > 128 * 1024 || budget.phase != Phase::Setup {
            return budget.fail(Stop::InvalidLimits);
        }
        let work = (cap as u64).checked_mul(8).ok_or(Stop::Overflow)?;
        let bytes = (cap as u64)
            .checked_add(std::mem::size_of::<Self>() as u64)
            .ok_or(Stop::Overflow)?;
        budget.reserve_checking(work, bytes, Instant::now())?;
        budget.reserve_trace(cap as u64, Instant::now())?;
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(cap).is_err() {
            return budget.fail(Stop::Allocation);
        }
        Ok(Self {
            bytes: buffer,
            cap,
            deadline: budget.deadline,
            reserved_work: work,
            performed_work: 0,
            attempted: false,
            complete: false,
            stop: None,
        })
    }
    fn encode<T: serde::Serialize>(&mut self, view: &T) -> Result<(), Stop> {
        use std::io::Write;
        if self.attempted {
            return Err(self.stop.unwrap_or(Stop::Phase));
        }
        self.attempted = true;
        if Instant::now() >= self.deadline {
            self.stop = Some(Stop::Deadline);
            return Err(Stop::Deadline);
        }
        if serde_json::to_writer(&mut *self, view).is_err() || self.write_all(b"\n").is_err() {
            let stop = *self.stop.get_or_insert(Stop::Serialization);
            return Err(stop);
        }
        if Instant::now() >= self.deadline {
            self.stop = Some(Stop::Deadline);
            return Err(Stop::Deadline);
        }
        self.complete = true;
        Ok(())
    }
}
impl std::io::Write for TerminalEncoding {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let stop = if Instant::now() >= self.deadline {
            Some(Stop::Deadline)
        } else if bytes.len() > self.cap - self.bytes.len() {
            Some(Stop::Trace)
        } else {
            None
        };
        if let Some(stop) = stop {
            self.stop.get_or_insert(stop);
            return Err(std::io::Error::other("terminal allowance exhausted"));
        }
        let work = (bytes.len() as u64)
            .checked_mul(8)
            .ok_or_else(|| std::io::Error::other("terminal work overflow"))?;
        if work > self.reserved_work - self.performed_work {
            self.stop.get_or_insert(Stop::Work);
            return Err(std::io::Error::other("terminal work exhausted"));
        }
        self.performed_work += work;
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[cfg(test)]
mod terminal_encoding_tests {
    use super::*;
    #[test]
    fn terminal_uses_only_precharged_allowance_after_producer_stop() {
        let mut b = construction_tests::budget();
        let mut terminal = TerminalEncoding::admit(2048, &mut b).unwrap();
        let work = b.checking_work;
        let trace = b.trace_bytes;
        let bytes = b.adapter_bytes;
        assert_eq!(b.fail::<()>(Stop::Observer), Err(Stop::Observer));
        terminal.encode(&b.saved_ledger()).unwrap();
        assert!(terminal.complete && terminal.stop.is_none());
        assert_eq!(b.stopped, Some(Stop::Observer));
        assert_eq!(
            (b.checking_work, b.trace_bytes, b.adapter_bytes),
            (work, trace, bytes)
        );
        assert_eq!(terminal.performed_work, terminal.bytes.len() as u64 * 8);
        assert!(terminal.performed_work <= terminal.reserved_work);
        verify_budget_ledger(&terminal.bytes, b.original_limits()).unwrap();
        let retained = terminal.bytes.clone();
        assert_eq!(terminal.encode(&b.saved_ledger()), Err(Stop::Phase));
        assert_eq!(terminal.bytes, retained);
    }
    #[test]
    fn terminal_cap_retains_partial_bytes_and_does_not_promote_them() {
        let mut b = construction_tests::budget();
        let mut terminal = TerminalEncoding::admit(10, &mut b).unwrap();
        assert_eq!(
            terminal.encode(&serde_json::json!({"value":"long payload"})),
            Err(Stop::Trace)
        );
        assert!(!terminal.complete && !terminal.bytes.is_empty());
        assert!(terminal.bytes.len() <= 10);
        let retained = terminal.bytes.clone();
        assert_eq!(terminal.encode(&0), Err(Stop::Trace));
        assert_eq!(terminal.bytes, retained);
    }
    #[test]
    fn original_deadline_and_admission_limits_are_not_reset_for_terminal_output() {
        let mut b = construction_tests::budget();
        let mut terminal = TerminalEncoding::admit(64, &mut b).unwrap();
        // Fault injection after admission: the original terminal deadline has elapsed.
        terminal.deadline = Instant::now();
        assert_eq!(terminal.encode(&0), Err(Stop::Deadline));
        assert!(terminal.bytes.is_empty() && !terminal.complete);
        let mut b = construction_tests::budget();
        b.trace_limit = 63;
        assert!(matches!(
            TerminalEncoding::admit(64, &mut b),
            Err(Stop::Trace)
        ));
        assert_eq!(b.stopped, Some(Stop::Trace));
    }
}

#[derive(serde::Serialize)]
struct SignedReplayView<'a> {
    schema: &'static str,
    binding: &'a crate::ml_dataset::capture::AttemptBinding,
    public_sha256: &'a str,
    factor_base_sha256: &'a str,
    input_sha256: &'a str,
    addition_limit: u64,
    performed_additions: u64,
    acknowledged_additions: usize,
    acknowledged: &'a [SignedPrefix],
    unacknowledged: &'a Option<SignedPrefix>,
}
#[derive(serde::Serialize)]
struct GroupReplayView<'a> {
    schema: &'static str,
    signed: SignedReplayView<'a>,
    reported_witness_check_additions: u64,
    reported_total_additions: u64,
}
#[derive(serde::Serialize)]
struct ConstructionResultView {
    output_retained: bool,
    reserved: Reservation,
    native_stop: Option<&'static str>,
    adapter_stop: Option<Stop>,
}
fn construction_result_view(c: &Construction) -> ConstructionResultView {
    use crate::cryptanalysis::koblitz_groebner::bounded_s4::Stop as S;
    let reserved = c
        .output
        .as_ref()
        .map(|o| o.reserved)
        .or_else(|| c.native_failure.as_ref().map(|f| f.reserved))
        .unwrap_or_default();
    let native_stop = c.native_failure.as_ref().map(|f| match f.stop {
        S::InvalidInput => "InvalidInput",
        S::WorkLimit => "WorkLimit",
        S::ByteLimit => "ByteLimit",
        S::SupportLimit => "SupportLimit",
        S::PairLimit => "PairLimit",
        S::Allocation => "Allocation",
        S::Overflow => "Overflow",
        S::Deadline => "Deadline",
        S::Observer => "Observer",
    });
    ConstructionResultView {
        output_retained: c.output.is_some(),
        reserved: Reservation {
            work: reserved.work,
            bytes: reserved.bytes,
        },
        native_stop,
        adapter_stop: c.adapter_stop,
    }
}
#[derive(serde::Serialize)]
struct SignedResultView {
    reported_complete: bool,
    first_stop: Option<Stop>,
    enumeration_additions: u64,
    witness_check_additions: u64,
    acknowledged_additions: usize,
    has_unacknowledged_addition: bool,
}
#[derive(serde::Serialize)]
struct ReceiptView {
    commit: &'static str,
    provenance: &'static str,
    payload_sha256_bytes: [u8; 32],
    payload_bytes: u64,
    sequence: Option<u64>,
}
fn receipt_view(r: crate::ml_dataset::capture::WitnessReceipt) -> ReceiptView {
    use crate::ml_dataset::capture::{WitnessCommit as C, WitnessProvenance as P};
    ReceiptView {
        commit: match r.commit {
            C::NewCommit => "NewCommit",
            C::AlreadyCommitted => "AlreadyCommitted",
        },
        provenance: match r.provenance {
            P::Dedicated => "Dedicated",
            P::LegacyAdopted => "LegacyAdopted",
        },
        payload_sha256_bytes: r.payload_sha256,
        payload_bytes: r.payload_bytes,
        sequence: r.sequence,
    }
}
#[derive(serde::Serialize)]
struct PersistenceResultView<'a> {
    reason: &'a str,
    serialized_complete: bool,
    retained_bytes: usize,
}
#[derive(serde::Serialize)]
struct CanonicalResultView<'a> {
    schema: &'static str,
    binding: &'a crate::ml_dataset::capture::AttemptBinding,
    public_sha256: &'a str,
    factor_base_sha256: &'a str,
    input_sha256: Option<&'a str>,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    ledger_before_terminal_encoding: &'a SavedBudgetLedger,
    construction: Option<ConstructionResultView>,
    matrix: Option<&'a crate::cryptanalysis::koblitz_groebner::bounded_matrix::Report>,
    signed: Option<SignedResultView>,
    witness_receipt: Option<ReceiptView>,
    input_persistence_failure: Option<PersistenceResultView<'a>>,
    native_capture_acknowledged: u64,
    signed_capture_acknowledged: u64,
    capture_count_before_artifact_publication: usize,
    pending_native: Option<NativeBoundaryView<'a>>,
    pending_native_encoding: &'a [u8],
    group_replay_encoded: bool,
    group_replay_encoding_stop: Option<Stop>,
    first_fall_degree: Option<u64>,
    last_fall_degree: Option<u64>,
    retired_hardware_instructions: Option<u64>,
}
struct CanonicalPipelineRun<'a> {
    producer_input: Option<Box<ProducerInputReceipt>>,
    artifacts_taken: bool,
    pipeline: Option<BoundPipelineRun<'a>>,
    input_sha256: Option<String>,
    group: Option<TerminalEncoding>,
    result: Option<TerminalEncoding>,
    admission_stop: Option<Stop>,
}
fn run_canonical_pipeline<'a>(
    original: &MathInput,
    binding: &'a crate::ml_dataset::capture::AttemptBinding,
    limits: &'a crate::ml_dataset::capture::CaptureLimits,
    public_sha256: &str,
    factor_base_sha256: &str,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    budget: &mut Budget,
    global: &mut usize,
    persist: &mut impl FnMut(&[u8], &mut Budget, &mut usize) -> Result<(), String>,
) -> CanonicalPipelineRun<'a> {
    let mut out = CanonicalPipelineRun {
        producer_input: None,
        artifacts_taken: false,
        pipeline: None,
        input_sha256: None,
        group: None,
        result: None,
        admission_stop: None,
    };
    let admission = (|| {
        budget.reserve_bytes(
            std::mem::size_of::<CanonicalPipelineRun<'_>>() as u64,
            Instant::now(),
        )?;
        out.group = Some(TerminalEncoding::admit(64 * 1024, budget)?);
        out.result = Some(TerminalEncoding::admit(64 * 1024, budget)?);
        Ok::<(), Stop>(())
    })();
    if let Err(stop) = admission {
        out.admission_stop = Some(stop);
        return out;
    }
    out.pipeline = Some(run_bound_pipeline(
        original,
        binding,
        limits,
        public_sha256,
        factor_base_sha256,
        engine,
        degree,
        budget,
        global,
        &mut |bytes, b, global| {
            b.reserve_checking(1024 + bytes.len() as u64 * 8, 4096, Instant::now())
                .map_err(|s| format!("input digest reservation: {s:?}"))?;
            out.input_sha256 = Some(crate::ml_dataset::digest(bytes));
            persist(bytes, b, global)
        },
    ));
    let run = out.pipeline.as_ref().unwrap();
    let group = out.group.as_mut().unwrap();
    if let (Some(signed), Some(input_sha)) = (&run.signed, out.input_sha256.as_deref()) {
        let view = GroupReplayView {
            schema: "crypto.ml-dataset.koblitz-m3-group-replay/v1",
            signed: SignedReplayView {
                schema: "crypto.ml-dataset.koblitz-m3-signed-prefix/v1",
                binding,
                public_sha256,
                factor_base_sha256,
                input_sha256: input_sha,
                addition_limit: run.ledger.original_limits.additions,
                performed_additions: signed.enumeration_additions,
                acknowledged_additions: signed.acknowledged.len(),
                acknowledged: &signed.acknowledged,
                unacknowledged: &signed.unacknowledged,
            },
            reported_witness_check_additions: signed.witness_check_additions,
            reported_total_additions: run.ledger.group_additions,
        };
        let _ = group.encode(&view);
    }
    let native = run.native.events.as_ref();
    let pending_native = native.and_then(|e| {
        e.pending.as_ref().map(|p| NativeBoundaryView {
            schema: "crypto.ml-dataset.koblitz-m3-events/v1",
            binding_sha256: std::str::from_utf8(&e.binding_digest)
                .expect("validated cached digest"),
            sequence: p.sequence,
            capture_sequence: p.capture_sequence,
            event: &p.event,
        })
    });
    let view = CanonicalResultView {
        schema: "crypto.ml-dataset.koblitz-m3-result/v1",
        binding,
        public_sha256,
        factor_base_sha256,
        input_sha256: out.input_sha256.as_deref(),
        engine,
        degree,
        ledger_before_terminal_encoding: &run.ledger,
        construction: run
            .native
            .construction
            .as_ref()
            .map(construction_result_view),
        matrix: run.native.matrix.as_ref().and_then(|m| m.report.as_ref()),
        signed: run.signed.as_ref().map(|s| SignedResultView {
            reported_complete: s.complete,
            first_stop: s.stop,
            enumeration_additions: s.enumeration_additions,
            witness_check_additions: s.witness_check_additions,
            acknowledged_additions: s.acknowledged.len(),
            has_unacknowledged_addition: s.unacknowledged.is_some(),
        }),
        witness_receipt: run
            .publisher
            .as_ref()
            .and_then(|p| p.receipt)
            .map(receipt_view),
        input_persistence_failure: run.native.persistence_failure.as_ref().map(|f| {
            PersistenceResultView {
                reason: &f.reason,
                serialized_complete: f.serialized_complete,
                retained_bytes: f.bytes.len(),
            }
        }),
        native_capture_acknowledged: native.map_or(0, |e| e.acknowledged),
        signed_capture_acknowledged: run.signed_events.as_ref().map_or(0, |e| e.acknowledged),
        capture_count_before_artifact_publication: *global,
        pending_native,
        pending_native_encoding: native.map_or(&[], |e| e.pending_bytes.as_slice()),
        group_replay_encoded: group.complete,
        group_replay_encoding_stop: group.stop,
        first_fall_degree: None,
        last_fall_degree: None,
        retired_hardware_instructions: None,
    };
    let _ = out.result.as_mut().unwrap().encode(&view);
    out
}
#[cfg(test)]
mod canonical_record_producer_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn canonical_group_bytes_pass_existing_independent_replay_for_both_engines() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            for positive in [false, true] {
                let binding = matrix_tests::binding();
                let limits = capture_limits(16).unwrap();
                let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
                let mut original = construction_tests::fixture();
                if positive {
                    original.target = original.signed[0];
                }
                let mut budget = construction_tests::budget();
                let mut global = 0;
                let run = run_canonical_pipeline(
                    &original,
                    &binding,
                    &limits,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    engine,
                    3,
                    &mut budget,
                    &mut global,
                    &mut |_, _, _| Ok(()),
                );
                assert!(run.admission_stop.is_none());
                let group = run.group.as_ref().unwrap();
                let result = run.result.as_ref().unwrap();
                assert!(
                    group.complete && result.complete,
                    "{:?} {:?}",
                    group.stop,
                    result.stop
                );
                let witness = if positive {
                    Some(std::fs::read(sink.dir.join("payloads/pdp-witness.json")).unwrap())
                } else {
                    None
                };
                let verified = verify_group_replay(
                    &group.bytes,
                    witness.as_deref(),
                    &original,
                    &binding,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    run.input_sha256.as_deref().unwrap(),
                    budget.addition_limit,
                )
                .unwrap();
                assert!(verified.math.prefix.all_tuples_acknowledged);
                assert_eq!(verified.math.witness.is_some(), positive);
                let saved: SavedGroupReplay = serde_json::from_slice(&group.bytes).unwrap();
                verify_signed_capture(
                    sink.dir.parent().unwrap(),
                    &serde_json::to_vec(&saved.signed).unwrap(),
                    &binding,
                    &limits,
                )
                .unwrap();
                let record: serde_json::Value = serde_json::from_slice(&result.bytes).unwrap();
                assert_eq!(record["schema"], "crypto.ml-dataset.koblitz-m3-result/v1");
                assert_eq!(record["signed"]["reported_complete"], true);
                assert_eq!(record["capture_count_before_artifact_publication"], global);
                assert!(
                    record["first_fall_degree"].is_null()
                        && record["last_fall_degree"].is_null()
                        && record["retired_hardware_instructions"].is_null()
                );
                assert_eq!(
                    record["ledger_before_terminal_encoding"]["first_stop"],
                    serde_json::Value::Null
                );
                assert!(
                    group.performed_work <= group.reserved_work
                        && result.performed_work <= result.reserved_work
                );
            }
        }
    }
    #[test]
    fn persisted_diagnostic_retains_failed_input_without_claiming_signed_completion() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let mut global = 0;
        let run = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Err("selected persistence failure".into()),
        );
        assert_eq!(budget.stopped, Some(Stop::Observer));
        assert!(!run.group.as_ref().unwrap().attempted);
        let result = run.result.as_ref().unwrap();
        assert!(result.complete);
        let record: serde_json::Value = serde_json::from_slice(&result.bytes).unwrap();
        assert!(record["signed"].is_null() && record["matrix"].is_null());
        assert_eq!(
            record["ledger_before_terminal_encoding"]["first_stop"],
            "Observer"
        );
        assert_eq!(
            record["input_persistence_failure"]["serialized_complete"],
            true
        );
        assert_eq!(record["group_replay_encoded"], false);
        assert!(!run
            .pipeline
            .as_ref()
            .unwrap()
            .native
            .persistence_failure
            .as_ref()
            .unwrap()
            .bytes
            .is_empty());
    }
    #[test]
    fn terminal_admission_failure_prevents_native_and_signed_work() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        budget.trace_limit = 64 * 1024 - 1;
        let mut global = 0;
        let mut called = false;
        let run = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| {
                called = true;
                Ok(())
            },
        );
        assert_eq!(run.admission_stop, Some(Stop::Trace));
        assert!(run.pipeline.is_none() && !called);
        assert_eq!(global, 0);
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedConstructionResult {
    output_retained: bool,
    reserved: Reservation,
    native_stop: Option<String>,
    adapter_stop: Option<Stop>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedSignedResult {
    reported_complete: bool,
    first_stop: Option<Stop>,
    enumeration_additions: u64,
    witness_check_additions: u64,
    acknowledged_additions: usize,
    has_unacknowledged_addition: bool,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedReceipt {
    commit: String,
    provenance: String,
    payload_sha256_bytes: [u8; 32],
    payload_bytes: u64,
    sequence: Option<u64>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedPersistenceResult {
    reason: String,
    serialized_complete: bool,
    retained_bytes: usize,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedCanonicalResult {
    schema: String,
    binding: crate::ml_dataset::capture::AttemptBinding,
    public_sha256: String,
    factor_base_sha256: String,
    input_sha256: Option<String>,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
    ledger_before_terminal_encoding: SavedBudgetLedger,
    construction: Option<SavedConstructionResult>,
    matrix: Option<SavedMatrixReport>,
    signed: Option<SavedSignedResult>,
    witness_receipt: Option<SavedReceipt>,
    input_persistence_failure: Option<SavedPersistenceResult>,
    native_capture_acknowledged: u64,
    signed_capture_acknowledged: u64,
    capture_count_before_artifact_publication: usize,
    pending_native: Option<SavedBoundaryRecord>,
    pending_native_encoding: Vec<u8>,
    group_replay_encoded: bool,
    group_replay_encoding_stop: Option<Stop>,
    first_fall_degree: Option<u64>,
    last_fall_degree: Option<u64>,
    retired_hardware_instructions: Option<u64>,
}
/// Decode bounded reported metadata only. Independent equations, matrix row
/// checks, group arithmetic and authenticated capture must still be matched.
fn decode_terminal_result(
    bytes: &[u8],
    binding: &crate::ml_dataset::capture::AttemptBinding,
    public_sha256: &str,
    factor_base_sha256: &str,
    input_sha256: Option<&str>,
    limits: OriginalLimits,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
) -> Result<SavedCanonicalResult, String> {
    if bytes.len() > 64 * 1024 {
        return Err("terminal result exceeds cap".into());
    }
    binding.validate()?;
    crate::store::index::validate_sha256(public_sha256)?;
    crate::store::index::validate_sha256(factor_base_sha256)?;
    if let Some(sha) = input_sha256 {
        crate::store::index::validate_sha256(sha)?;
    }
    let r: SavedCanonicalResult =
        serde_json::from_slice(bytes).map_err(|e| format!("strict terminal result: {e}"))?;
    if r.schema != "crypto.ml-dataset.koblitz-m3-result/v1"
        || &r.binding != binding
        || r.public_sha256 != public_sha256
        || r.factor_base_sha256 != factor_base_sha256
        || r.input_sha256.as_deref() != input_sha256
        || r.engine != engine
        || r.degree != degree
        || !(1..=6).contains(&degree)
        || r.first_fall_degree.is_some()
        || r.last_fall_degree.is_some()
        || r.retired_hardware_instructions.is_some()
    {
        return Err("terminal original identity or unsupported claim mismatch".into());
    }
    let ledger = &r.ledger_before_terminal_encoding;
    verify_budget_ledger(
        &serde_json::to_vec(ledger).map_err(|e| e.to_string())?,
        limits,
    )?;
    if r.capture_count_before_artifact_publication > 4096
        || r.native_capture_acknowledged > 4096
        || r.signed_capture_acknowledged > 192
        || r.native_capture_acknowledged + r.signed_capture_acknowledged
            > r.capture_count_before_artifact_publication as u64
        || r.pending_native_encoding.len() > 2048
        || (r.pending_native.is_none() && !r.pending_native_encoding.is_empty())
    {
        return Err("terminal capture count or pending encoding mismatch".into());
    }
    if let Some(p) = &r.pending_native {
        if p.schema != "crypto.ml-dataset.koblitz-m3-events/v1"
            || p.binding_sha256 != binding.digest()?
            || p.sequence != r.native_capture_acknowledged + 1
            || p.capture_sequence != r.capture_count_before_artifact_publication as u64 + 1
            || ledger.first_stop.is_none()
        {
            return Err("terminal pending boundary identity mismatch".into());
        }
    }
    let mut native = Reservation::default();
    if let Some(c) = &r.construction {
        if c.native_stop.as_deref().is_some_and(|s| {
            !matches!(
                s,
                "InvalidInput"
                    | "WorkLimit"
                    | "ByteLimit"
                    | "SupportLimit"
                    | "PairLimit"
                    | "Allocation"
                    | "Overflow"
                    | "Deadline"
                    | "Observer"
            )
        }) || (c.output_retained && c.native_stop.is_some())
            || (c.adapter_stop.is_some() && c.adapter_stop != ledger.first_stop)
        {
            return Err("terminal construction disposition mismatch".into());
        }
        native = c.reserved;
    }
    if let Some(m) = &r.matrix {
        if !r
            .construction
            .as_ref()
            .is_some_and(|c| c.output_retained && c.adapter_stop.is_none())
            || r.input_persistence_failure.is_some()
            || r.input_sha256.is_none()
            || m.engine != engine
            || m.degree != degree
            || m.native_step_complete
                != (m.stop
                    == crate::cryptanalysis::koblitz_groebner::bounded_matrix::Stop::Complete)
        {
            return Err("terminal matrix prerequisites mismatch".into());
        }
        native.work = native
            .work
            .checked_add(m.reserved.work)
            .ok_or("terminal phase work overflow")?;
        native.bytes = native
            .bytes
            .checked_add(m.reserved.bytes)
            .ok_or("terminal phase bytes overflow")?;
    }
    if native != ledger.native_reserved {
        return Err("terminal native reservations differ from retained phases".into());
    }
    if let Some(f) = &r.input_persistence_failure {
        if f.retained_bytes > INPUT_BYTE_CAP
            || r.matrix.is_some()
            || ledger.first_stop.is_none()
            || (f.serialized_complete && f.retained_bytes == 0)
        {
            return Err("terminal persistence failure mismatch".into());
        }
    }
    if matches!(
        ledger.phase,
        Phase::Constructed
            | Phase::Matrix
            | Phase::MatrixComplete
            | Phase::Extraction
            | Phase::Complete
    ) && !r.construction.as_ref().is_some_and(|c| c.output_retained)
    {
        return Err("terminal phase lacks completed construction".into());
    }
    if matches!(
        ledger.phase,
        Phase::MatrixComplete | Phase::Extraction | Phase::Complete
    ) && !r.matrix.as_ref().is_some_and(|m| m.native_step_complete)
    {
        return Err("terminal phase lacks completed matrix".into());
    }
    if matches!(ledger.phase, Phase::Extraction | Phase::Complete) && r.signed.is_none() {
        return Err("terminal extraction phase lacks signed report".into());
    }
    if let Some(s) = &r.signed {
        if !r.matrix.as_ref().is_some_and(|m| m.native_step_complete)
            || s.enumeration_additions > 192
            || s.acknowledged_additions > 192
            || s.witness_check_additions > 3
            || s.acknowledged_additions as u64 != r.signed_capture_acknowledged
            || s.enumeration_additions
                != s.acknowledged_additions as u64 + u64::from(s.has_unacknowledged_addition)
            || s.enumeration_additions + s.witness_check_additions != ledger.group_additions
            || (s.reported_complete
                && (s.first_stop.is_some()
                    || ledger.first_stop.is_some()
                    || ledger.phase != Phase::Complete
                    || s.enumeration_additions != 192
                    || s.acknowledged_additions != 192
                    || s.has_unacknowledged_addition))
            || (!s.reported_complete
                && (s.first_stop.is_none() || s.first_stop != ledger.first_stop))
        {
            return Err("terminal signed disposition or counters mismatch".into());
        }
    } else if ledger.group_additions != 0
        || r.signed_capture_acknowledged != 0
        || r.witness_receipt.is_some()
        || ledger.phase == Phase::Complete
    {
        return Err("terminal signed facts lack signed phase".into());
    }
    if r.group_replay_encoded && (r.signed.is_none() || r.group_replay_encoding_stop.is_some()) {
        return Err("terminal group encoding disposition mismatch".into());
    }
    if let Some(w) = &r.witness_receipt {
        if !matches!(w.commit.as_str(), "NewCommit" | "AlreadyCommitted")
            || !matches!(w.provenance.as_str(), "Dedicated" | "LegacyAdopted")
            || w.payload_bytes == 0
            || w.payload_bytes > SIGNED_WITNESS_BYTE_CAP as u64
            || w.sequence
                .is_some_and(|s| s == 0 || s > r.capture_count_before_artifact_publication as u64)
            || (w.provenance == "Dedicated" && w.sequence.is_none())
            || !r
                .signed
                .as_ref()
                .is_some_and(|s| s.witness_check_additions == 3)
        {
            return Err("terminal witness receipt metadata mismatch".into());
        }
    }
    Ok(r)
}
#[cfg(test)]
mod terminal_result_reader_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::{capture_limits, AttemptBinding};
    fn actual(
        engine: native::Engine,
        mode: u8,
    ) -> (Vec<u8>, AttemptBinding, OriginalLimits, Option<String>) {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut budget = construction_tests::budget();
        if mode == 2 {
            budget.trace_limit = 128 * 1024 + 1;
        }
        let original_limits = budget.original_limits();
        let mut global = 0;
        let run = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            engine,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| {
                if mode == 1 {
                    Err("selected I/O failure".into())
                } else {
                    Ok(())
                }
            },
        );
        assert!(run.result.as_ref().unwrap().complete);
        (
            run.result.as_ref().unwrap().bytes.clone(),
            binding.clone(),
            original_limits,
            run.input_sha256.clone(),
        )
    }
    #[test]
    fn strict_terminal_reader_accepts_actual_complete_and_censored_native_paths() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            for mode in 0..3 {
                let (bytes, binding, limits, input) = actual(engine, mode);
                let parsed = decode_terminal_result(
                    &bytes,
                    &binding,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    input.as_deref(),
                    limits,
                    engine,
                    3,
                )
                .unwrap();
                assert_eq!(parsed.signed.is_some(), mode == 0);
                assert_eq!(parsed.pending_native.is_some(), mode == 2);
                assert_eq!(parsed.input_persistence_failure.is_some(), mode == 1);
            }
        }
    }
    #[test]
    fn strict_terminal_reader_rejects_reencoded_identity_counter_and_claim_mutations() {
        let engine = native::Engine::F4Macaulay;
        let (bytes, binding, limits, input) = actual(engine, 0);
        let base: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for mutation in 0..14 {
            let mut v = base.clone();
            match mutation {
                0 => v["extra"] = true.into(),
                1 => v["ledger_before_terminal_encoding"]["original_limits"]["extra"] = true.into(),
                2 => v["matrix"]["extra"] = true.into(),
                3 => v["public_sha256"] = "0".repeat(64).into(),
                4 => {
                    v["ledger_before_terminal_encoding"]["native_reserved"]["work"] = serde_json::json!(
                        base["ledger_before_terminal_encoding"]["native_reserved"]["work"]
                            .as_u64()
                            .unwrap()
                            + 1
                    )
                }
                5 => v["native_capture_acknowledged"] = 4097.into(),
                6 => v["signed"]["acknowledged_additions"] = 191.into(),
                7 => v["signed"]["reported_complete"] = false.into(),
                8 => v["witness_receipt"]["sequence"] = 4097.into(),
                9 => v["group_replay_encoding_stop"] = "Trace".into(),
                10 => v["first_fall_degree"] = 4.into(),
                11 => v["matrix"] = serde_json::Value::Null,
                12 => v["construction"]["output_retained"] = false.into(),
                13 => v["pending_native_encoding"] = serde_json::json!([0]),
                _ => unreachable!(),
            }
            assert!(
                decode_terminal_result(
                    &serde_json::to_vec(&v).unwrap(),
                    &binding,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    input.as_deref(),
                    limits,
                    engine,
                    3
                )
                .is_err(),
                "mutation {mutation}"
            );
        }
        assert!(decode_terminal_result(
            &vec![b' '; 65537],
            &binding,
            &"8".repeat(64),
            &"9".repeat(64),
            input.as_deref(),
            limits,
            engine,
            3
        )
        .is_err());
    }
}

struct VerifiedCanonicalComponents {
    producer_ledger: SavedBudgetLedger,
    math: VerifiedCanonicalMath,
    native_boundaries: usize,
    reported_group_complete: bool,
    input_zero_assignments: usize,
}
/// Join canonical arithmetic, reported terminal metadata and authenticated
/// boundary records. Original run admission and supervisor outcome remain
/// external gates; matrix row consistency is not a derivation certificate.
fn verify_canonical_components(
    dir: &std::path::Path,
    artifacts: &[crate::ml_dataset::types::Artifact],
    original: &MathInput,
    binding: &crate::ml_dataset::capture::AttemptBinding,
    capture_limits: &crate::ml_dataset::capture::CaptureLimits,
    limits: OriginalLimits,
    public_sha256: &str,
    factor_base_sha256: &str,
    engine: crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine,
    degree: u32,
) -> Result<VerifiedCanonicalComponents, String> {
    use super::prime_f4_wide::{artifact, read_bounded};
    let math = verify_canonical_math_files(
        dir,
        artifacts,
        original,
        binding,
        capture_limits,
        limits,
        public_sha256,
        factor_base_sha256,
    )?;
    let read = |path: &str, role: &str, encoding: &str, cap: usize| -> Result<Vec<u8>, String> {
        let a = artifact(artifacts, path, role, encoding, true)?
            .ok_or("missing canonical component")?;
        let bytes = read_bounded(dir, path, cap)?;
        if a.bytes != bytes.len() as u64 || a.sha256 != crate::ml_dataset::digest(&bytes) {
            return Err("canonical component changed after manifest verification".into());
        }
        Ok(bytes)
    };
    let input = read(
        "koblitz-m3-input.json",
        "restricted-koblitz-m3-original-equations/v1",
        "json",
        INPUT_BYTE_CAP,
    )?;
    let group = read(
        "koblitz-m3-group-replay.json",
        "restricted-koblitz-m3-group-replay/v1",
        "json",
        SIGNED_REPLAY_BYTE_CAP,
    )?;
    let terminal = read(
        "koblitz-m3-result.json",
        "restricted-koblitz-m3-result/v1",
        "json",
        64 * 1024,
    )?;
    let boundaries = read(
        "koblitz-m3-events.jsonl",
        "restricted-koblitz-m3-boundary-events/v1",
        "jsonl",
        4096 * 2048,
    )?;
    let input_sha = crate::ml_dataset::digest(&input);
    let r = decode_terminal_result(
        &terminal,
        binding,
        public_sha256,
        factor_base_sha256,
        Some(&input_sha),
        limits,
        engine,
        degree,
    )?;
    let g: SavedGroupReplay = serde_json::from_slice(&group).map_err(|e| e.to_string())?;
    let i: SavedInput = serde_json::from_slice(&input).map_err(|e| e.to_string())?;
    let captured = verify_native_capture(
        dir,
        &boundaries,
        binding,
        capture_limits,
        i.equations.len(),
        engine,
        degree,
    )?;
    if !captured.construction_complete_captured
        || !captured.matrix_finish_captured
        || captured.records as u64 != r.native_capture_acknowledged
        || r.pending_native.is_some()
        || !r.group_replay_encoded
        || r.group_replay_encoding_stop.is_some()
    {
        return Err("terminal native completion or retained group differs from capture".into());
    }
    let signed = r
        .signed
        .as_ref()
        .ok_or("canonical group lacks terminal signed report")?;
    if signed.enumeration_additions != g.signed.performed_additions as u64
        || signed.acknowledged_additions != g.signed.acknowledged.len()
        || signed.has_unacknowledged_addition != g.signed.unacknowledged.is_some()
        || signed.witness_check_additions != u64::from(g.reported_witness_check_additions)
        || r.ledger_before_terminal_encoding.group_additions != g.reported_total_additions
        || (signed.reported_complete && !math.group.math.prefix.all_tuples_acknowledged)
    {
        return Err("terminal signed counters differ from replayed group".into());
    }
    let matrix = r
        .matrix
        .as_ref()
        .ok_or("canonical group lacks native matrix report")?;
    if !matrix.native_step_complete
        || matrix.counters.attempted_events != captured.matrix_captured as u64
        || matrix.counters.acknowledged_events != captured.matrix_captured as u64
    {
        return Err("terminal matrix completion counters differ from capture".into());
    }
    let digest = binding.digest()?;
    let mut construction_final = None;
    let mut matrix_final = None;
    for (index, line) in boundaries.split_inclusive(|b| *b == b'\n').enumerate() {
        let row = decode_boundary_record(
            line,
            &digest,
            index as u64 + 1,
            i.equations.len(),
            engine,
            degree,
        )?;
        match row.event {
            NativeBoundary::Construction {
                stage: ConstructionBoundaryStage::Complete,
                reserved,
            } => construction_final = Some(reserved),
            NativeBoundary::Matrix {
                stage: MatrixBoundaryStage::Finish,
                reserved,
                counters,
            } => matrix_final = Some((reserved, counters)),
            _ => {}
        }
    }
    let c = r
        .construction
        .as_ref()
        .ok_or("missing terminal construction")?;
    let c_end = construction_final.ok_or("missing captured construction completion")?;
    let (m_end, mut m_counts) = matrix_final.ok_or("missing captured matrix finish")?;
    m_counts.acknowledged_events = m_counts
        .acknowledged_events
        .checked_add(1)
        .ok_or("native acknowledgement overflow")?;
    if c.reserved.work != c_end.work
        || c.reserved.bytes != c_end.bytes
        || matrix.reserved != m_end
        || native_counter_values(matrix.counters) != native_counter_values(m_counts)
    {
        return Err("terminal native reservations or counters differ from final boundaries".into());
    }
    let raw: serde_json::Value = serde_json::from_slice(&terminal).map_err(|e| e.to_string())?;
    let matrix_bytes = serde_json::to_vec(&raw["matrix"]).map_err(|e| e.to_string())?;
    let row_check = verify_matrix_report(&matrix_bytes, &i.equations, engine, degree, limits)?;
    let mut last_signed = 0;
    crate::ml_dataset::capture::verify_live_committed_prefix_visit(
        dir,
        binding,
        capture_limits,
        &mut |line| {
            let row: crate::ml_dataset::capture::EventRecord =
                serde_json::from_slice(line).map_err(|e| e.to_string())?;
            if row.event["kind"] == "koblitz_m3" {
                last_signed = row.sequence;
            }
            Ok(())
        },
    )?
    .ok_or("missing committed capture")?;
    let final_producer = captured
        .last_capture_sequence
        .max(last_signed)
        .max(math.witness_publication_sequence.unwrap_or(0));
    if final_producer != r.capture_count_before_artifact_publication as u64 {
        return Err("terminal producer capture count differs from authenticated sequence".into());
    }
    match (&r.witness_receipt, math.witness_publication_sequence) {
        (None, None) => {}
        (Some(w), Some(sequence)) => {
            let a = artifact(
                artifacts,
                "pdp-witness.json",
                "independently-verified-pdp",
                "json",
                true,
            )?
            .ok_or("missing witness artifact")?;
            let sha = w
                .payload_sha256_bytes
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>();
            if w.sequence != Some(sequence) || w.payload_bytes != a.bytes || sha != a.sha256 {
                return Err(
                    "terminal receipt differs from authenticated witness publication".into(),
                );
            }
        }
        _ => return Err("terminal witness presence differs from authenticated publication".into()),
    }
    Ok(VerifiedCanonicalComponents {
        producer_ledger: r.ledger_before_terminal_encoding,
        math,
        native_boundaries: captured.records,
        reported_group_complete: signed.reported_complete,
        input_zero_assignments: row_check.input_zero_assignments,
    })
}
#[cfg(test)]
mod canonical_component_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::{capture::capture_limits, types::Artifact};
    fn manifest(root: &std::path::Path, positive: bool) -> Vec<Artifact> {
        let mut names = KOBLITZ_SLOTS.to_vec();
        names.push(PARENT_DELIVERY_SLOT);
        names.extend([
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
        if positive {
            names.push(("pdp-witness.json", "independently-verified-pdp", "json"));
        }
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
    fn canonical_components_join_real_positive_negative_and_capped_runs() {
        for engine in [
            native::Engine::F4Macaulay,
            native::Engine::F5KoszulFrobenius,
        ] {
            for scenario in 0..3 {
                let positive = scenario == 1;
                let binding = matrix_tests::binding();
                let capture = capture_limits(16).unwrap();
                let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
                let root = sink.dir.parent().unwrap();
                let mut original = construction_tests::fixture();
                if positive {
                    original.target = original.signed[0];
                }
                let mut budget = construction_tests::budget();
                if scenario == 2 {
                    budget.addition_limit = 2;
                }
                let limits = budget.original_limits();
                let mut delivery = ParentDeliveryFile::admit(&mut budget).unwrap();
                let mut global = 0;
                let mut run = run_canonical_pipeline(
                    &original,
                    &binding,
                    &capture,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    engine,
                    3,
                    &mut budget,
                    &mut global,
                    &mut |_, _, _| Ok(()),
                );
                assert!(
                    run.group.as_ref().unwrap().complete && run.result.as_ref().unwrap().complete
                );
                let mut parent = ParentRetention::begin(&mut budget).unwrap();
                let transfer = take_parent_artifacts(&mut run, &mut parent);
                assert!(transfer.error.is_none() && transfer.stop.is_none());
                let mut owners = transfer.value.unwrap();
                let before = std::fs::read(sink.dir.join("events.jsonl")).unwrap();
                let storage_result =
                    persist_parent_artifacts(&mut owners, &mut parent, root, &binding, &capture);
                assert!(storage_result.is_ok(), "parent storage: engine={engine:?} scenario={scenario} result={storage_result:?} ledger={:?} items={:?}", parent.budget, owners.items.iter().map(|a|(&a.payload.path,a.payload.bytes.len(),a.capture_acknowledged,a.root_artifact.is_some(),quote_parent_storage(a,&capture))).collect::<Vec<_>>());
                assert!(owners
                    .items
                    .iter()
                    .all(|a| a.capture_acknowledged && a.root_artifact.is_some()));
                let result = &owners
                    .items
                    .iter()
                    .find(|a| a.payload.path == "koblitz-m3-result.json")
                    .unwrap()
                    .payload
                    .bytes;
                if positive {
                    let after = std::fs::read_to_string(sink.dir.join("events.jsonl")).unwrap();
                    let witness_events = |text: &str| {
                        text.lines()
                            .filter(|line| line.contains("payloads/pdp-witness.json"))
                            .count()
                    };
                    assert_eq!(witness_events(std::str::from_utf8(&before).unwrap()), 1);
                    assert_eq!(witness_events(&after), 1);
                }
                delivery.finish(&owners, &parent, &binding, root).unwrap();
                let check = || {
                    verify_canonical_components(
                        root,
                        &manifest(root, positive),
                        &original,
                        &binding,
                        &capture,
                        limits,
                        &"8".repeat(64),
                        &"9".repeat(64),
                        engine,
                        3,
                    )
                };
                let verified = check().unwrap();
                let delivery_check = || {
                    verify_parent_delivery(
                        root,
                        &manifest(root, positive),
                        &binding,
                        &capture,
                        limits,
                        verified.producer_ledger,
                    )
                };
                let delivered = delivery_check().unwrap();
                assert!(delivered.delivery_complete);
                assert_eq!(delivered.root_artifacts, owners.items.len());
                assert_eq!(delivered.capture_acknowledgements, owners.items.len());
                let parent_bytes = std::fs::read(root.join(PARENT_DELIVERY_SLOT.0)).unwrap();
                let parent_json: serde_json::Value = serde_json::from_slice(&parent_bytes).unwrap();
                for mutation in 0..10 {
                    let mut forged = parent_json.clone();
                    match mutation {
                        0 => {
                            forged["after"]["checking_reserved_work"] = serde_json::json!(
                                parent_json["after"]["checking_reserved_work"]
                                    .as_u64()
                                    .unwrap()
                                    + 1
                            )
                        }
                        1 => forged["items"][0]["quote"]["work"] = serde_json::json!(0),
                        2 => forged["items"][0]["capture_acknowledged"] = serde_json::json!(false),
                        3 => {
                            forged["items"][0]["root_artifact"]["sha256"] =
                                serde_json::json!("0".repeat(64))
                        }
                        4 => forged["items"][0]["admitted"] = serde_json::json!(false),
                        5 => forged["delivery_complete"] = serde_json::json!(false),
                        6 => forged["before"]["phase"] = serde_json::json!("Setup"),
                        7 => forged["owner_quote"]["bytes"] = serde_json::json!(0),
                        8 => forged["unknown"] = serde_json::json!(true),
                        9 => {
                            let removed = forged["items"].as_array_mut().unwrap().remove(0);
                            let identity: u64 = ["path", "role", "encoding"]
                                .iter()
                                .map(|k| removed[k].as_str().unwrap().len() as u64)
                                .sum();
                            let owner_work = 512 + 8 * identity;
                            let owner_bytes = 512 + identity;
                            for (section, key, delta) in [
                                ("owner_quote", "work", owner_work),
                                ("owner_quote", "bytes", owner_bytes),
                                (
                                    "after",
                                    "checking_reserved_work",
                                    owner_work + removed["quote"]["work"].as_u64().unwrap(),
                                ),
                                (
                                    "after",
                                    "adapter_reserved_bytes",
                                    owner_bytes + removed["quote"]["bytes"].as_u64().unwrap(),
                                ),
                                (
                                    "after",
                                    "charged_trace_bytes",
                                    removed["quote"]["trace"].as_u64().unwrap(),
                                ),
                            ] {
                                forged[section][key] = serde_json::json!(
                                    forged[section][key].as_u64().unwrap() - delta
                                );
                            }
                            forged["payload_bytes"] = serde_json::json!(
                                forged["payload_bytes"].as_u64().unwrap()
                                    - removed["bytes"].as_u64().unwrap()
                            );
                        }
                        _ => unreachable!(),
                    }
                    std::fs::write(
                        root.join(PARENT_DELIVERY_SLOT.0),
                        serde_json::to_vec(&forged).unwrap(),
                    )
                    .unwrap();
                    assert!(delivery_check().is_err(), "parent mutation {mutation}");
                }
                std::fs::write(root.join(PARENT_DELIVERY_SLOT.0), &parent_bytes).unwrap();
                assert!(delivery_check().unwrap().delivery_complete);

                assert_eq!(verified.math.group.math.witness.is_some(), positive);
                assert_eq!(verified.reported_group_complete, scenario != 2);
                assert!(verified.native_boundaries > 0);
                let outer = serde_json::json!({
                    "status": if scenario == 2 { "partial" } else { "verified" },
                    "solver_completed": scenario != 2,
                    "pdp_witness_verified": positive,
                    "exhaustive_no_decomposition": scenario == 0,
                    "group_domain_complete": scenario != 2,
                    "certificate_durably_persisted": positive
                });
                let facts = verify_delivered_outcome(&outer, &verified, &delivered).unwrap();
                // Retain real canonical files and capture evidence, but replace
                // the delivery record with an honestly incomplete root receipt.
                // The final write was admitted and capture-acknowledged; only
                // the root acknowledgement is missing. Charges remain intact.
                let mut incomplete = parent_json.clone();
                incomplete["items"]
                    .as_array_mut()
                    .unwrap()
                    .last_mut()
                    .unwrap()["root_artifact"] = serde_json::Value::Null;
                incomplete["parent_stop"] = serde_json::json!("Observer");
                incomplete["persistence_error"] =
                    serde_json::json!("final root acknowledgement missing");
                incomplete["delivery_complete"] = serde_json::json!(false);
                std::fs::write(
                    root.join(PARENT_DELIVERY_SLOT.0),
                    serde_json::to_vec(&incomplete).unwrap(),
                )
                .unwrap();
                let incomplete_delivery = delivery_check().unwrap();
                assert!(!incomplete_delivery.delivery_complete);
                assert_eq!(
                    verify_delivered_outcome(&outer, &verified, &incomplete_delivery).is_ok(),
                    scenario == 2
                );
                let mut partial = outer.clone();
                partial["status"] = serde_json::json!("partial");
                partial["solver_completed"] = serde_json::json!(false);
                let retained =
                    verify_delivered_outcome(&partial, &verified, &incomplete_delivery).unwrap();
                assert_eq!(retained.retained_witness_verified, positive);
                assert!(!retained.solver_completed);
                std::fs::write(root.join(PARENT_DELIVERY_SLOT.0), &parent_bytes).unwrap();
                assert!(delivery_check().unwrap().delivery_complete);
                assert!(!facts.supervisor_censored);
                assert_eq!(facts.solver_completed, scenario != 2);
                assert_eq!(facts.retained_witness_verified, positive);
                for (stop, success) in [
                    (Some("timeout"), true),
                    (Some("trace_budget_exhausted"), true),
                    (None, false),
                ] {
                    let censored = crate::ml_dataset::execution::terminal_outcome(
                        outer.clone(),
                        stop,
                        success,
                        Some(1),
                    );
                    let facts = verify_outer_outcome(&censored, &verified).unwrap();
                    assert!(facts.supervisor_censored && !facts.solver_completed);
                    assert_eq!(facts.retained_witness_verified, positive);
                    for key in [
                        "solver_completed",
                        "pdp_witness_verified",
                        "exhaustive_no_decomposition",
                        "certificate_durably_persisted",
                        "group_domain_complete",
                        "native_backend_invoked",
                        "native_statistics_complete",
                        "algebra_completed",
                    ] {
                        let mut forged = censored.clone();
                        forged[key] = serde_json::json!(true);
                        assert!(verify_outer_outcome(&forged, &verified).is_err(), "{key}");
                    }
                    let mut retained = censored.clone();
                    retained["retained_pdp_witness_verified"] = serde_json::json!(true);
                    assert_eq!(verify_outer_outcome(&retained, &verified).is_ok(), positive);
                }
                for key in [
                    "solver_completed",
                    "pdp_witness_verified",
                    "certificate_durably_persisted",
                    "group_domain_complete",
                    "exhaustive_no_decomposition",
                    "retained_pdp_witness_verified",
                ] {
                    for value in [
                        serde_json::Value::Null,
                        serde_json::json!(1),
                        serde_json::json!("true"),
                    ] {
                        let mut forged = outer.clone();
                        forged[key] = value;
                        assert!(verify_outer_outcome(&forged, &verified).is_err(), "{key}");
                    }
                }
                for key in [
                    "first_fall_degree",
                    "last_fall_degree",
                    "retired_hardware_instructions",
                    "native_basis_derivation_verified",
                    "execution_trace_complete_verified",
                    "total_operation_count_verified",
                    "ecdlp_pipeline_completed",
                    "fall_degree_verified",
                    "extraction_complete",
                ] {
                    let mut forged = outer.clone();
                    forged[key] = serde_json::json!(true);
                    assert!(verify_outer_outcome(&forged, &verified).is_err(), "{key}");
                }
                let mut failed = outer.clone();
                failed["live_persistence_errors"] = serde_json::json!(["parent write failed"]);
                assert_eq!(
                    verify_outer_outcome(&failed, &verified).is_ok(),
                    scenario == 2
                );

                if positive {
                    assert!(verified.input_zero_assignments > 0);
                    let base: serde_json::Value = serde_json::from_slice(result).unwrap();
                    for mutation in 0..6 {
                        let mut v = base.clone();
                        match mutation {
                            0 => {
                                v["matrix"]["counters"]["reduction_word_xors"] = serde_json::json!(
                                    base["matrix"]["counters"]["reduction_word_xors"]
                                        .as_u64()
                                        .unwrap()
                                        + 1
                                )
                            }
                            1 => {
                                v["construction"]["reserved"]["work"] = serde_json::json!(
                                    base["construction"]["reserved"]["work"].as_u64().unwrap() + 1
                                );
                                v["ledger_before_terminal_encoding"]["native_reserved"]["work"] = serde_json::json!(
                                    base["ledger_before_terminal_encoding"]["native_reserved"]
                                        ["work"]
                                        .as_u64()
                                        .unwrap()
                                        + 1
                                );
                            }
                            2 => {
                                v["native_capture_acknowledged"] = serde_json::json!(
                                    base["native_capture_acknowledged"].as_u64().unwrap() + 1
                                );
                                v["capture_count_before_artifact_publication"] =
                                    serde_json::json!(global + 1);
                            }
                            3 => {
                                v["capture_count_before_artifact_publication"] =
                                    serde_json::json!(global + 1)
                            }
                            4 => {
                                v["witness_receipt"]["payload_sha256_bytes"][0] = serde_json::json!(
                                    base["witness_receipt"]["payload_sha256_bytes"][0]
                                        .as_u64()
                                        .unwrap()
                                        ^ 1
                                )
                            }
                            5 => {
                                v["matrix"]["rows"] =
                                    serde_json::json!([{"n_vars":3,"terms":[{"mask":0}]}]);
                                v["matrix"]["rank"] = 1.into();
                            }
                            _ => unreachable!(),
                        }
                        std::fs::write(
                            root.join("koblitz-m3-result.json"),
                            serde_json::to_vec(&v).unwrap(),
                        )
                        .unwrap();
                        assert!(check().is_err(), "rehashed mutation {mutation}");
                    }
                    std::fs::write(root.join("koblitz-m3-result.json"), result).unwrap();
                    assert!(check().is_ok());
                }
            }
        }
    }
}

/// A borrowed view of already retained bytes. The visitor itself performs no
/// allocation, serialization, publication, or budget reset. Its caller owns
/// admission of any storage I/O and must preserve an existing witness receipt.
struct RetainedArtifact<'a> {
    path: &'static str,
    role: &'static str,
    encoding: &'static str,
    bytes: &'a [u8],
    publication: Option<crate::ml_dataset::capture::WitnessReceipt>,
}
impl CanonicalPipelineRun<'_> {
    fn visit_retained_artifacts<E>(
        &self,
        mut visit: impl FnMut(RetainedArtifact<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        if self.artifacts_taken {
            return Ok(());
        }
        let mut emit = |slot: (&'static str, &'static str, &'static str), bytes, publication| {
            visit(RetainedArtifact {
                path: slot.0,
                role: slot.1,
                encoding: slot.2,
                bytes,
                publication,
            })
        };
        if let Some(run) = &self.pipeline {
            if let Some(matrix) = &run.native.matrix {
                emit(KOBLITZ_SLOTS[0], matrix.original_bytes.as_slice(), None)?;
            }
            if let Some(failure) = &run.native.persistence_failure {
                if !failure.bytes.is_empty() {
                    emit(
                        if self.producer_input.is_some() {
                            KOBLITZ_SLOTS[0]
                        } else {
                            (
                                "partial/koblitz-m3-input.raw",
                                "unregistered-partial-artifact",
                                "native",
                            )
                        },
                        failure.bytes.as_slice(),
                        None,
                    )?;
                }
            }
            if let Some(events) = &run.native.events {
                // An empty acknowledged prefix is still a valid JSONL stream.
                emit(KOBLITZ_SLOTS[1], events.canonical.as_slice(), None)?;
                if !events.pending_bytes.is_empty() {
                    emit(
                        (
                            "partial/koblitz-m3-native-event.raw",
                            "unregistered-partial-artifact",
                            "native",
                        ),
                        events.pending_bytes.as_slice(),
                        None,
                    )?;
                }
            }
            if let Some(publisher) = &run.publisher {
                if let Some(encoded) = &publisher.encoding {
                    if encoded.complete && publisher.receipt.is_some() {
                        emit(
                            ("pdp-witness.json", "independently-verified-pdp", "json"),
                            encoded.bytes.as_slice(),
                            publisher.receipt,
                        )?;
                    } else if !encoded.bytes.is_empty() {
                        emit(
                            (
                                "partial/koblitz-m3-witness.raw",
                                "unregistered-partial-artifact",
                                "native",
                            ),
                            encoded.bytes.as_slice(),
                            None,
                        )?;
                    }
                }
            }
        }
        for (encoded, canonical, partial) in [
            (
                &self.group,
                KOBLITZ_SLOTS[3],
                "partial/koblitz-m3-group-replay.raw",
            ),
            (
                &self.result,
                KOBLITZ_SLOTS[2],
                "partial/koblitz-m3-result.raw",
            ),
        ] {
            if let Some(encoded) = encoded {
                if encoded.complete {
                    emit(canonical, encoded.bytes.as_slice(), None)?;
                } else if !encoded.bytes.is_empty() {
                    emit(
                        (partial, "unregistered-partial-artifact", "native"),
                        encoded.bytes.as_slice(),
                        None,
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod retained_artifact_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::capture_limits;

    #[test]
    fn borrowed_visit_preserves_bytes_receipts_and_stops_at_parent_failure() {
        for positive in [false, true] {
            let binding = matrix_tests::binding();
            let limits = capture_limits(16).unwrap();
            let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
            let mut original = construction_tests::fixture();
            if positive {
                original.target = original.signed[0];
            }
            let mut budget = construction_tests::budget();
            let mut global = 0;
            let run = run_canonical_pipeline(
                &original,
                &binding,
                &limits,
                &"8".repeat(64),
                &"9".repeat(64),
                native::Engine::F4Macaulay,
                3,
                &mut budget,
                &mut global,
                &mut |_, _, _| Ok(()),
            );
            let mut identities = Vec::new();
            run.visit_retained_artifacts::<()>(|a| {
                if a.path == KOBLITZ_SLOTS[0].0 {
                    assert_eq!(
                        a.bytes.as_ptr(),
                        run.pipeline
                            .as_ref()
                            .unwrap()
                            .native
                            .matrix
                            .as_ref()
                            .unwrap()
                            .original_bytes
                            .as_ptr()
                    );
                }
                if a.path == "pdp-witness.json" {
                    assert!(positive);
                    assert_eq!(
                        a.publication,
                        run.pipeline
                            .as_ref()
                            .unwrap()
                            .publisher
                            .as_ref()
                            .unwrap()
                            .receipt
                    );
                    assert!(a.publication.is_some());
                } else {
                    assert!(a.publication.is_none());
                }
                identities.push((a.path, a.role, a.encoding, a.bytes.len()));
                Ok(())
            })
            .unwrap();
            assert_eq!(identities.len(), if positive { 5 } else { 4 });
            for slot in KOBLITZ_SLOTS {
                assert_eq!(
                    identities
                        .iter()
                        .filter(|a| (a.0, a.1, a.2) == slot)
                        .count(),
                    1
                );
            }
            let mut visited = 0;
            assert_eq!(
                run.visit_retained_artifacts(|_| {
                    visited += 1;
                    Err("parent storage failure")
                }),
                Err("parent storage failure")
            );
            assert_eq!(visited, 1);
            let mut replayed = Vec::new();
            run.visit_retained_artifacts::<()>(|a| {
                replayed.push((a.path, a.role, a.encoding, a.bytes.len()));
                Ok(())
            })
            .unwrap();
            assert_eq!(identities, replayed);
        }
    }

    #[test]
    fn failed_input_partial_terminal_and_unpublished_witness_keep_raw_roles() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut original = construction_tests::fixture();
        original.target = original.signed[0];
        let mut budget = construction_tests::budget();
        let mut global = 0;
        let failed = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Err("selected storage failure".into()),
        );
        let mut paths = Vec::new();
        failed
            .visit_retained_artifacts::<()>(|a| {
                assert_ne!(a.path, KOBLITZ_SLOTS[0].0);
                if a.path.contains("input.raw") {
                    assert_eq!(a.role, "unregistered-partial-artifact");
                    assert_eq!(a.encoding, "native");
                    assert!(!a.bytes.is_empty());
                }
                paths.push(a.path);
                Ok(())
            })
            .unwrap();
        assert!(paths.contains(&"partial/koblitz-m3-input.raw"));
        drop(_sink);
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut budget = construction_tests::budget();
        let mut global = 0;
        let mut run = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Ok(()),
        );
        // Selection controls: retain real bytes while withholding completion or
        // publication evidence. Neither flag alone may promote a raw artifact.
        run.group.as_mut().unwrap().complete = false;
        run.result.as_mut().unwrap().complete = false;
        run.pipeline
            .as_mut()
            .unwrap()
            .publisher
            .as_mut()
            .unwrap()
            .receipt = None;
        let mut raw = 0;
        run.visit_retained_artifacts::<()>(|a| {
            assert!(
                ![KOBLITZ_SLOTS[2].0, KOBLITZ_SLOTS[3].0, "pdp-witness.json"].contains(&a.path)
            );
            if a.path.starts_with("partial/") {
                raw += 1;
                assert_eq!(a.role, "unregistered-partial-artifact");
                assert_eq!(a.encoding, "native");
                assert!(a.publication.is_none());
            }
            Ok(())
        })
        .unwrap();
        assert_eq!(raw, 3);
    }
}

/// Outer labels are checked only after canonical component authentication.
/// Original execution admission remains a separate prerequisite at dispatch.
#[derive(Debug)]
struct VerifiedOuterOutcome {
    supervisor_censored: bool,
    solver_completed: bool,
    retained_witness_verified: bool,
}
/// Completion requires independently checked mathematics and delivery. An
/// omitted outer error field cannot stand in for a positive storage receipt.
fn verify_delivered_outcome(
    outcome: &serde_json::Value,
    verified: &VerifiedCanonicalComponents,
    delivery: &VerifiedParentDelivery,
) -> Result<VerifiedOuterOutcome, String> {
    let facts = verify_outer_outcome(outcome, verified)?;
    if facts.solver_completed && !delivery.delivery_complete {
        return Err("Koblitz completion lacks complete parent delivery".into());
    }
    Ok(facts)
}

fn verify_outer_outcome(
    outcome: &serde_json::Value,
    verified: &VerifiedCanonicalComponents,
) -> Result<VerifiedOuterOutcome, String> {
    if !outcome.is_object() {
        return Err("Koblitz outcome must be an object".into());
    }
    super::prime_f4_wide::verify_unproved_claims(outcome)?;
    for key in [
        "independent_derivation_verified",
        "operation_trajectory_verified",
        "exact_instruction_count_verified",
        "full_execution_trace_verified",
        "fall_degree_verified",
        "linear_extraction_complete",
        "extraction_complete",
    ] {
        if !outcome[key].is_null() && outcome[key] != false {
            return Err(format!("Koblitz replay does not certify {key}"));
        }
    }
    let censored = matches!(
        outcome["status"].as_str(),
        Some("timeout" | "trace_budget_exhausted" | "worker_failed")
    );
    let witness = verified.math.group.math.witness.is_some()
        && verified.math.witness_publication_sequence.is_some();
    let domain = verified.math.group.math.prefix.all_tuples_acknowledged;
    let negative = verified.math.group.math.negative_domain_verified();
    let complete = verified.reported_group_complete && domain && (witness || negative);
    let completed = outcome["solver_completed"]
        .as_bool()
        .ok_or("Koblitz outcome lacks boolean solver_completed")?;
    if !censored && !outcome["pdp_witness_verified"].is_boolean() {
        return Err("Koblitz outcome lacks boolean pdp_witness_verified".into());
    }
    for (key, supported) in [
        ("solver_completed", complete),
        ("pdp_witness_verified", witness),
        ("certificate_durably_persisted", witness),
        ("group_domain_complete", domain),
        ("exhaustive_no_decomposition", negative),
        ("native_backend_invoked", true),
        ("native_statistics_complete", true),
        ("algebra_completed", true),
    ] {
        if let Some(value) = outcome.get(key) {
            let claimed = value
                .as_bool()
                .ok_or_else(|| format!("nonboolean Koblitz claim {key}"))?;
            if claimed && (censored || !supported) {
                return Err(format!("unsupported Koblitz outer claim {key}"));
            }
        }
    }
    if let Some(value) = outcome.get("retained_pdp_witness_verified") {
        let claimed = value
            .as_bool()
            .ok_or("nonboolean retained Koblitz witness claim")?;
        if claimed && !witness {
            return Err("retained Koblitz witness lacks authenticated evidence".into());
        }
    }
    if completed && outcome.get("live_persistence_errors").is_some() {
        return Err("Koblitz completion cannot survive parent persistence failure".into());
    }
    match outcome["status"].as_str() {
        Some("verified")
            if completed
                && (outcome["pdp_witness_verified"] == true
                    || outcome["exhaustive_no_decomposition"] == true) => {}
        Some("partial") if !completed => {}
        Some("timeout" | "trace_budget_exhausted" | "worker_failed") if !completed => {}
        _ => return Err("Koblitz status contradicts checked outcome".into()),
    }
    Ok(VerifiedOuterOutcome {
        supervisor_censored: censored,
        solver_completed: completed,
        retained_witness_verified: witness,
    })
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RetentionQuote {
    work: u64,
    bytes: u64,
    trace: u64,
}
/// Exclusive post-producer retention session. Charges stay in the original
/// ledger. No stop/deadline/phase reset and no fresh native allowance is granted.
struct ParentRetention<'a> {
    budget: &'a mut Budget,
    stop: Option<Stop>,
}
struct RetentionCall<T, E> {
    value: Option<T>,
    error: Option<E>,
    stop: Option<Stop>,
}
impl<'a> ParentRetention<'a> {
    fn begin(budget: &'a mut Budget) -> Result<Self, Stop> {
        if budget.retention_started || (budget.phase != Phase::Complete && budget.stopped.is_none())
        {
            return Err(Stop::Phase);
        }
        budget.retention_started = true;
        Ok(Self { budget, stop: None })
    }
    /// Transactionally reserve all dimensions before the parent action. A late
    /// native settlement can exceed limits; checked sums reject it safely.
    fn admit(&mut self, quote: RetentionQuote, now: Instant) -> Result<(), Stop> {
        if let Some(stop) = self.stop {
            return Err(stop);
        }
        let result = (|| {
            if now >= self.budget.deadline {
                return Err(Stop::Deadline);
            }
            let checking = self
                .budget
                .checking_work
                .checked_add(quote.work)
                .ok_or(Stop::Overflow)?;
            let work = self
                .budget
                .native
                .work
                .checked_add(checking)
                .ok_or(Stop::Overflow)?;
            let adapter = self
                .budget
                .adapter_bytes
                .checked_add(quote.bytes)
                .ok_or(Stop::Overflow)?;
            let bytes = self
                .budget
                .native
                .bytes
                .checked_add(adapter)
                .ok_or(Stop::Overflow)?;
            let trace = self
                .budget
                .trace_bytes
                .checked_add(quote.trace)
                .ok_or(Stop::Overflow)?;
            if work > self.budget.native_work_limit {
                return Err(Stop::Work);
            }
            if bytes > self.budget.memory_limit {
                return Err(Stop::Bytes);
            }
            if trace > self.budget.trace_limit {
                return Err(Stop::Trace);
            }
            self.budget.checking_work = checking;
            self.budget.adapter_bytes = adapter;
            self.budget.trace_bytes = trace;
            Ok(())
        })();
        if let Err(stop) = result {
            self.stop = Some(stop);
        }
        result
    }
    fn execute<T, E>(
        &mut self,
        quote: RetentionQuote,
        action: impl FnOnce() -> Result<T, E>,
    ) -> RetentionCall<T, E> {
        self.execute_with_clock(quote, Instant::now(), action, Instant::now)
    }
    fn execute_with_clock<T, E>(
        &mut self,
        quote: RetentionQuote,
        before: Instant,
        action: impl FnOnce() -> Result<T, E>,
        after: impl FnOnce() -> Instant,
    ) -> RetentionCall<T, E> {
        if let Err(stop) = self.admit(quote, before) {
            return RetentionCall {
                value: None,
                error: None,
                stop: Some(stop),
            };
        }
        // Preserve acknowledgement before observing the return-time clock.
        // A failed or late operation still consumes its complete reservation.
        let (value, error) = match action() {
            Ok(value) => (Some(value), None),
            Err(error) => {
                self.stop = Some(Stop::Observer);
                (None, Some(error))
            }
        };
        if after() >= self.budget.deadline && self.stop.is_none() {
            self.stop = Some(Stop::Deadline);
        }
        RetentionCall {
            value,
            error,
            stop: self.stop,
        }
    }
}
#[cfg(test)]
mod parent_retention_admission_tests {
    use super::*;
    #[test]
    fn stopped_producer_keeps_original_stop_phase_deadline_and_cumulative_charges() {
        let mut b = construction_tests::budget();
        b.stopped = Some(Stop::Observer);
        let phase = b.phase;
        let deadline = b.deadline;
        let before = (b.checking_work, b.adapter_bytes, b.trace_bytes);
        let mut parent = ParentRetention::begin(&mut b).unwrap();
        let r = parent.execute(
            RetentionQuote {
                work: 7,
                bytes: 11,
                trace: 13,
            },
            || Ok::<_, ()>("saved"),
        );
        assert_eq!(r.value, Some("saved"));
        assert!(r.error.is_none() && r.stop.is_none());
        assert_eq!(parent.budget.stopped, Some(Stop::Observer));
        assert_eq!(parent.budget.phase, phase);
        assert_eq!(parent.budget.deadline, deadline);
        assert_eq!(
            (
                parent.budget.checking_work,
                parent.budget.adapter_bytes,
                parent.budget.trace_bytes
            ),
            (before.0 + 7, before.1 + 11, before.2 + 13)
        );
        drop(parent);
        assert!(matches!(ParentRetention::begin(&mut b), Err(Stop::Phase)));
        assert_eq!(b.check(Instant::now()), Err(Stop::Observer));
    }
    #[test]
    fn exact_and_one_over_admission_is_transactional_and_cannot_restart() {
        for dimension in 0..3 {
            let mut b = construction_tests::budget();
            b.phase = Phase::Complete;
            b.native.work = 2;
            b.checking_work = 3;
            b.native_work_limit = 12;
            b.native.bytes = 5;
            b.adapter_bytes = 7;
            b.memory_limit = 23;
            b.trace_bytes = 17;
            b.trace_limit = 30;
            let mut parent = ParentRetention::begin(&mut b).unwrap();
            parent
                .admit(
                    RetentionQuote {
                        work: 7,
                        bytes: 11,
                        trace: 13,
                    },
                    Instant::now(),
                )
                .unwrap();
            let before = (
                parent.budget.checking_work,
                parent.budget.adapter_bytes,
                parent.budget.trace_bytes,
            );
            let mut q = RetentionQuote::default();
            match dimension {
                0 => q.work = 1,
                1 => q.bytes = 1,
                _ => q.trace = 1,
            };
            let expected = [Stop::Work, Stop::Bytes, Stop::Trace][dimension];
            assert_eq!(parent.admit(q, Instant::now()), Err(expected));
            assert_eq!(
                (
                    parent.budget.checking_work,
                    parent.budget.adapter_bytes,
                    parent.budget.trace_bytes
                ),
                before
            );
            assert_eq!(
                parent.admit(RetentionQuote::default(), Instant::now()),
                Err(expected)
            );
            assert_eq!(parent.budget.stopped, None);
            assert_eq!(parent.budget.phase, Phase::Complete);
        }
    }
    #[test]
    fn deadline_failure_and_late_acknowledgement_preserve_exact_evidence() {
        let mut b = construction_tests::budget();
        assert!(matches!(ParentRetention::begin(&mut b), Err(Stop::Phase)));
        b.phase = Phase::Complete;
        let deadline = b.deadline;
        let mut parent = ParentRetention::begin(&mut b).unwrap();
        let r = parent.execute_with_clock(
            RetentionQuote {
                work: 7,
                bytes: 11,
                trace: 13,
            },
            Instant::now(),
            || Ok::<_, ()>(42),
            || deadline,
        );
        assert_eq!(r.value, Some(42));
        assert_eq!(r.stop, Some(Stop::Deadline));
        let before = parent.budget.checking_work;
        let r = parent.execute(
            RetentionQuote {
                work: 1,
                ..Default::default()
            },
            || -> Result<(), ()> { panic!("post-stop I/O") },
        );
        assert_eq!(r.stop, Some(Stop::Deadline));
        assert_eq!(parent.budget.checking_work, before);
        let mut b = construction_tests::budget();
        b.stopped = Some(Stop::Bytes);
        let deadline = b.deadline;
        let mut parent = ParentRetention::begin(&mut b).unwrap();
        let r = parent.execute_with_clock(
            RetentionQuote {
                work: 1,
                ..Default::default()
            },
            deadline,
            || -> Result<(), ()> { panic!("expired I/O") },
            Instant::now,
        );
        assert_eq!(r.stop, Some(Stop::Deadline));
        assert_eq!(parent.budget.checking_work, 0);
        assert_eq!(parent.budget.stopped, Some(Stop::Bytes));
    }
    #[test]
    fn failed_io_overflow_and_late_native_excess_never_refund_or_wrap() {
        let mut b = construction_tests::budget();
        b.phase = Phase::Complete;
        let mut parent = ParentRetention::begin(&mut b).unwrap();
        let r = parent.execute(
            RetentionQuote {
                work: 7,
                bytes: 11,
                trace: 13,
            },
            || Err::<(), _>("selected I/O failure"),
        );
        assert_eq!(r.error, Some("selected I/O failure"));
        assert!(r.value.is_none());
        assert_eq!(r.stop, Some(Stop::Observer));
        assert_eq!(
            (
                parent.budget.checking_work,
                parent.budget.adapter_bytes,
                parent.budget.trace_bytes
            ),
            (7, 11, 13)
        );
        for overflow in [false, true] {
            let mut b = construction_tests::budget();
            b.stopped = Some(Stop::NativeFailure);
            b.native.work = if overflow {
                u64::MAX
            } else {
                b.native_work_limit + 1
            };
            let mut parent = ParentRetention::begin(&mut b).unwrap();
            assert_eq!(
                parent.admit(
                    RetentionQuote {
                        work: 1,
                        ..Default::default()
                    },
                    Instant::now()
                ),
                Err(if overflow { Stop::Overflow } else { Stop::Work })
            );
            assert_eq!(parent.budget.checking_work, 0);
            assert_eq!(parent.budget.stopped, Some(Stop::NativeFailure));
        }
    }
}

const RAW_RETENTION_NAMES: [(&str, &str); 5] = [
    (
        "partial/koblitz-m3-input.raw",
        "partial-koblitz-m3-input.raw",
    ),
    (
        "partial/koblitz-m3-native-event.raw",
        "partial-koblitz-m3-native-event.raw",
    ),
    (
        "partial/koblitz-m3-witness.raw",
        "partial-koblitz-m3-witness.raw",
    ),
    (
        "partial/koblitz-m3-group-replay.raw",
        "partial-koblitz-m3-group-replay.raw",
    ),
    (
        "partial/koblitz-m3-result.raw",
        "partial-koblitz-m3-result.raw",
    ),
];
fn parent_artifact_path(path: &'static str) -> &'static str {
    RAW_RETENTION_NAMES
        .iter()
        .find(|(logical, _)| *logical == path)
        .map_or(path, |(_, flat)| *flat)
}
struct OwnedRetainedArtifact {
    storage_attempted: bool,
    storage_admitted: bool,
    storage_quote: Option<RetentionQuote>,
    logical_path: &'static str,
    capture_acknowledged: bool,
    root_artifact: Option<crate::ml_dataset::types::Artifact>,
    payload: crate::ml_dataset::types::ArtifactPayload,
    publication: Option<crate::ml_dataset::capture::WitnessReceipt>,
}
struct ParentArtifacts {
    producer_input: Option<Box<ProducerInputReceipt>>,
    ledger_before_owners: SavedBudgetLedger,
    owner_quote: RetentionQuote,
    items: Vec<OwnedRetainedArtifact>,
    payload_bytes: u64,
    persistence_error: Option<String>,
    retention_stop: Option<Stop>,
}
#[derive(Debug)]
struct OwnerQuote {
    resources: RetentionQuote,
    count: usize,
    payload_bytes: u64,
}
fn quote_artifact_owners(run: &CanonicalPipelineRun<'_>) -> Result<OwnerQuote, Stop> {
    if run.artifacts_taken {
        return Err(Stop::Phase);
    }
    if std::mem::size_of::<ParentArtifacts>() > 512
        || std::mem::size_of::<OwnedRetainedArtifact>() > 512
    {
        return Err(Stop::Bytes);
    }
    let mut out = OwnerQuote {
        resources: RetentionQuote {
            work: 512,
            bytes: 512,
            trace: 0,
        },
        count: 0,
        payload_bytes: 0,
    };
    run.visit_retained_artifacts(|a| {
        out.count = out.count.checked_add(1).ok_or(Stop::Overflow)?;
        if out.count > 10 {
            return Err(Stop::Events);
        }
        let identity =
            (parent_artifact_path(a.path).len() + a.role.len() + a.encoding.len()) as u64;
        out.resources.bytes = out
            .resources
            .bytes
            .checked_add(512)
            .and_then(|n| n.checked_add(identity))
            .ok_or(Stop::Overflow)?;
        out.resources.work = out
            .resources
            .work
            .checked_add(identity.checked_mul(8).ok_or(Stop::Overflow)?)
            .and_then(|n| n.checked_add(512))
            .ok_or(Stop::Overflow)?;
        out.payload_bytes = out
            .payload_bytes
            .checked_add(a.bytes.len() as u64)
            .ok_or(Stop::Overflow)?;
        Ok(())
    })?;
    Ok(out)
}
fn retained_identity(value: &str) -> Result<String, Stop> {
    let mut out = String::new();
    out.try_reserve_exact(value.len())
        .map_err(|_| Stop::Allocation)?;
    out.push_str(value);
    Ok(out)
}
/// Admit all owner metadata first. Allocate every identity before moving any
/// existing data buffer, so allocation failure leaves the original run intact.
/// This transfers ownership only; live/root storage needs separate admission.
fn take_parent_artifacts(
    run: &mut CanonicalPipelineRun<'_>,
    parent: &mut ParentRetention<'_>,
) -> RetentionCall<ParentArtifacts, Stop> {
    let quote = match quote_artifact_owners(run) {
        Ok(q) => q,
        Err(e) => {
            return RetentionCall {
                value: None,
                error: Some(e),
                stop: Some(e),
            }
        }
    };
    let ledger_before_owners = parent.budget.saved_ledger();
    parent.execute(quote.resources, || {
        let mut items = Vec::new();
        items
            .try_reserve_exact(quote.count)
            .map_err(|_| Stop::Allocation)?;
        run.visit_retained_artifacts(|a| {
            items.push(OwnedRetainedArtifact {
                logical_path: a.path,
                storage_attempted: false,
                storage_admitted: false,
                storage_quote: None,
                capture_acknowledged: false,
                root_artifact: None,
                payload: crate::ml_dataset::types::ArtifactPayload {
                    path: retained_identity(parent_artifact_path(a.path))?,
                    role: retained_identity(a.role)?,
                    encoding: retained_identity(a.encoding)?,
                    bytes: Vec::new(),
                },
                publication: a.publication,
            });
            Ok::<(), Stop>(())
        })?;
        for item in &mut items {
            let bytes = match item.logical_path {
                "koblitz-m3-input.json" => {
                    let native = &mut run.pipeline.as_mut().unwrap().native;
                    if let Some(matrix) = &mut native.matrix {
                        &mut matrix.original_bytes
                    } else {
                        &mut native.persistence_failure.as_mut().unwrap().bytes
                    }
                }
                "koblitz-m3-events.jsonl" => {
                    &mut run
                        .pipeline
                        .as_mut()
                        .unwrap()
                        .native
                        .events
                        .as_mut()
                        .unwrap()
                        .canonical
                }
                "partial/koblitz-m3-input.raw" => {
                    &mut run
                        .pipeline
                        .as_mut()
                        .unwrap()
                        .native
                        .persistence_failure
                        .as_mut()
                        .unwrap()
                        .bytes
                }
                "partial/koblitz-m3-native-event.raw" => {
                    &mut run
                        .pipeline
                        .as_mut()
                        .unwrap()
                        .native
                        .events
                        .as_mut()
                        .unwrap()
                        .pending_bytes
                }
                "pdp-witness.json" | "partial/koblitz-m3-witness.raw" => {
                    &mut run
                        .pipeline
                        .as_mut()
                        .unwrap()
                        .publisher
                        .as_mut()
                        .unwrap()
                        .encoding
                        .as_mut()
                        .unwrap()
                        .bytes
                }
                "koblitz-m3-group-replay.json" | "partial/koblitz-m3-group-replay.raw" => {
                    &mut run.group.as_mut().unwrap().bytes
                }
                "koblitz-m3-result.json" | "partial/koblitz-m3-result.raw" => {
                    &mut run.result.as_mut().unwrap().bytes
                }
                _ => unreachable!("closed retained artifact registry"),
            };
            item.payload.bytes = std::mem::take(bytes);
        }
        let mut producer_input = run.producer_input.take();
        if let Some(r) = &mut producer_input {
            let input = items
                .iter_mut()
                .find(|a| a.logical_path == KOBLITZ_SLOTS[0].0)
                .expect("attached input has owner");
            input.capture_acknowledged = r.capture_sequence.is_some();
            input.root_artifact = r.root_artifact.take();
        }
        run.artifacts_taken = true;
        Ok(ParentArtifacts {
            producer_input,
            ledger_before_owners,
            owner_quote: quote.resources,
            items,
            payload_bytes: quote.payload_bytes,
            persistence_error: None,
            retention_stop: None,
        })
    })
}
#[cfg(test)]
mod parent_artifact_owner_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn actual_buffers_move_once_with_publication_and_raw_identity_preserved() {
        for scenario in 0..3 {
            let binding = matrix_tests::binding();
            let limits = capture_limits(16).unwrap();
            let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
            let mut original = construction_tests::fixture();
            if scenario == 1 {
                original.target = original.signed[0];
            }
            let mut budget = construction_tests::budget();
            let mut global = 0;
            let mut run = run_canonical_pipeline(
                &original,
                &binding,
                &limits,
                &"8".repeat(64),
                &"9".repeat(64),
                native::Engine::F4Macaulay,
                3,
                &mut budget,
                &mut global,
                &mut |_, _, _| {
                    if scenario == 2 {
                        Err("selected input I/O failure".into())
                    } else {
                        Ok(())
                    }
                },
            );
            let mut before = Vec::new();
            run.visit_retained_artifacts::<()>(|a| {
                before.push((a.path, a.bytes.as_ptr(), a.bytes.len(), a.publication));
                Ok(())
            })
            .unwrap();
            let quote = quote_artifact_owners(&run).unwrap();
            let original_stop = budget.stopped;
            let trace = budget.trace_bytes;
            let checking = budget.checking_work;
            let bytes = budget.adapter_bytes;
            let mut parent = ParentRetention::begin(&mut budget).unwrap();
            let result = take_parent_artifacts(&mut run, &mut parent);
            assert!(result.error.is_none() && result.stop.is_none());
            let owned = result.value.unwrap();
            assert_eq!(owned.items.len(), before.len());
            assert_eq!(
                owned.payload_bytes,
                before.iter().map(|a| a.2 as u64).sum::<u64>()
            );
            for (item, (logical, pointer, len, receipt)) in owned.items.iter().zip(before) {
                assert_eq!(item.logical_path, logical);
                assert_eq!(item.payload.bytes.as_ptr(), pointer);
                assert_eq!(item.payload.bytes.len(), len);
                assert_eq!(item.publication, receipt);
                assert!(!item.payload.path.contains('/'));
                if logical.starts_with("partial/") {
                    assert!(item.payload.path.starts_with("partial-koblitz-m3-"));
                    assert_eq!(item.payload.role, "unregistered-partial-artifact");
                }
            }
            assert_eq!(parent.budget.checking_work, checking + quote.resources.work);
            assert_eq!(parent.budget.adapter_bytes, bytes + quote.resources.bytes);
            assert_eq!(parent.budget.trace_bytes, trace);
            assert_eq!(parent.budget.stopped, original_stop);
            assert!(run.artifacts_taken);
            run.visit_retained_artifacts::<()>(|_| panic!("transferred bytes re-exposed"))
                .unwrap();
            assert_eq!(
                take_parent_artifacts(&mut run, &mut parent).error,
                Some(Stop::Phase)
            );
        }
    }
    #[test]
    fn parent_admission_failure_leaves_all_producer_buffers_owned() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let mut global = 0;
        let mut run = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Ok(()),
        );
        let before = run.result.as_ref().unwrap().bytes.as_ptr();
        let len = run.result.as_ref().unwrap().bytes.len();
        budget.memory_limit = budget.native.bytes + budget.adapter_bytes;
        let mut parent = ParentRetention::begin(&mut budget).unwrap();
        let result = take_parent_artifacts(&mut run, &mut parent);
        assert!(result.value.is_none());
        assert_eq!(result.stop, Some(Stop::Bytes));
        assert!(!run.artifacts_taken);
        assert_eq!(run.result.as_ref().unwrap().bytes.as_ptr(), before);
        assert_eq!(run.result.as_ref().unwrap().bytes.len(), len);
    }
    #[test]
    fn flat_partial_registry_rejects_canonical_promotion_and_duplicates() {
        use crate::ml_dataset::types::Artifact;
        for (_, path) in RAW_RETENTION_NAMES {
            let a = Artifact {
                path: path.into(),
                role: "unregistered-partial-artifact".into(),
                encoding: "native".into(),
                bytes: 1,
                sha256: "0".repeat(64),
            };
            verify_slot_identities(std::slice::from_ref(&a)).unwrap();
            let mut forged = a.clone();
            forged.role = KOBLITZ_SLOTS[0].1.into();
            assert!(verify_slot_identities(&[forged]).is_err());
            let mut forged = a.clone();
            forged.encoding = "json".into();
            assert!(verify_slot_identities(&[forged]).is_err());
            assert!(verify_slot_identities(&[a.clone(), a]).is_err());
        }
    }
}

/// Quote storage from actual retained length. Work is a declared hashing and
/// metadata tariff, not machine instructions. Trace covers live/root temporary
/// copies and head metadata; memory includes the store's 1 MiB hash buffers.
fn quote_parent_storage(
    item: &OwnedRetainedArtifact,
    limits: &crate::ml_dataset::capture::CaptureLimits,
) -> Result<RetentionQuote, Stop> {
    quote_parent_storage_length(
        item.payload.bytes.len() as u64,
        item.publication.is_some(),
        limits,
    )
}
fn quote_parent_storage_length(
    len: u64,
    witness: bool,
    limits: &crate::ml_dataset::capture::CaptureLimits,
) -> Result<RetentionQuote, Stop> {
    limits.validate().map_err(|_| Stop::InvalidLimits)?;
    let metadata = if witness {
        0
    } else {
        limits
            .metadata_reserve_bytes
            .checked_mul(2)
            .ok_or(Stop::Overflow)?
    };
    let event = if witness {
        0
    } else {
        limits
            .max_event_bytes
            .checked_mul(2)
            .ok_or(Stop::Overflow)?
    };
    Ok(RetentionQuote {
        work: len
            .checked_mul(32)
            .and_then(|n| n.checked_add(65536))
            .ok_or(Stop::Overflow)?,
        bytes: metadata
            .checked_add(event)
            .and_then(|n| n.checked_add(2 * 1024 * 1024 + 16384))
            .ok_or(Stop::Overflow)?,
        trace: len
            .checked_mul(if witness { 2 } else { 4 })
            .and_then(|n| n.checked_add(metadata))
            .and_then(|n| n.checked_add(4096))
            .ok_or(Stop::Overflow)?,
    })
}
/// Caller supplies the admitted original output directory and binding. This
/// function does not create a new capture sink or a new retention allowance.
fn persist_parent_artifacts(
    owned: &mut ParentArtifacts,
    parent: &mut ParentRetention<'_>,
    root: &std::path::Path,
    binding: &crate::ml_dataset::capture::AttemptBinding,
    limits: &crate::ml_dataset::capture::CaptureLimits,
) -> Result<(), Stop> {
    if let Some(stop) = owned.retention_stop {
        return Err(stop);
    }
    for item in &mut owned.items {
        if owned.producer_input.is_some() && item.logical_path == KOBLITZ_SLOTS[0].0 {
            // Original producer attempt is terminal: never retry it here.
            continue;
        }
        if item.root_artifact.is_some() {
            continue;
        }
        let quote = match quote_parent_storage(item, limits) {
            Ok(q) => q,
            Err(stop) => {
                owned.retention_stop = Some(stop);
                return Err(stop);
            }
        };
        item.storage_attempted = true;
        item.storage_quote = Some(quote);
        let call = parent.execute(quote, || {
            item.storage_admitted = true;
            if item.payload.path != parent_artifact_path(item.logical_path) {
                return Err("parent artifact identity changed".into());
            }
            super::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
                Some(super::EventSink::Bound(sink)) => sink.verify_attempt_limits(binding, limits),
                _ => Err("parent retention requires original bound capture".into()),
            })?;
            if let Some(receipt) = item.publication {
                if item.payload.path != "pdp-witness.json"
                    || item.payload.role != "independently-verified-pdp"
                    || item.payload.encoding != "json"
                    || receipt.payload_bytes != item.payload.bytes.len() as u64
                    || hex::encode(receipt.payload_sha256)
                        != crate::ml_dataset::digest(&item.payload.bytes)
                {
                    return Err("parent witness differs from dedicated receipt".into());
                }
                // The producer already acknowledged this dedicated publication.
                // Never pass it through the generic payload route.
                item.capture_acknowledged = true;
            } else {
                super::live_borrowed_payload(
                    &item.payload.path,
                    &item.payload.role,
                    &item.payload.encoding,
                    &item.payload.bytes,
                )?;
                item.capture_acknowledged = true;
            }
            let artifact = crate::ml_dataset::storage::write_bytes(
                root,
                &item.payload.path,
                &item.payload.role,
                &item.payload.encoding,
                &item.payload.bytes,
            )?;
            item.root_artifact = Some(artifact);
            Ok::<(), String>(())
        });
        if let Some(error) = call.error {
            owned.persistence_error = Some(error);
        }
        if let Some(stop) = call.stop {
            owned.retention_stop = Some(stop);
            return Err(stop);
        }
    }
    Ok(())
}
#[cfg(test)]
mod parent_storage_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn failed_input_raw_payload_crosses_actual_capture_and_root_storage() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let mut delivery = ParentDeliveryFile::admit(&mut budget).unwrap();
        let mut global = 0;
        let mut run = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Err("selected input persistence failure".into()),
        );
        let mut parent = ParentRetention::begin(&mut budget).unwrap();
        let mut owned = take_parent_artifacts(&mut run, &mut parent).value.unwrap();
        persist_parent_artifacts(
            &mut owned,
            &mut parent,
            sink.dir.parent().unwrap(),
            &binding,
            &limits,
        )
        .unwrap();
        assert_eq!(parent.budget.stopped, Some(Stop::Observer));
        assert!(owned.persistence_error.is_none());
        let raw = owned
            .items
            .iter()
            .find(|a| a.payload.path == "partial-koblitz-m3-input.raw")
            .unwrap();
        assert!(raw.capture_acknowledged && raw.root_artifact.is_some());
        assert_eq!(
            std::fs::read(sink.dir.join("payloads/partial-koblitz-m3-input.raw")).unwrap(),
            raw.payload.bytes
        );
        assert_eq!(
            std::fs::read(sink.dir.parent().unwrap().join(&raw.payload.path)).unwrap(),
            raw.payload.bytes
        );
        assert!(!sink.dir.parent().unwrap().join(KOBLITZ_SLOTS[0].0).exists());
        delivery
            .finish(&owned, &parent, &binding, sink.dir.parent().unwrap())
            .unwrap();
        let mut all: Vec<_> = owned
            .items
            .iter()
            .filter_map(|a| a.root_artifact.clone())
            .collect();
        all.push(delivery.artifact.clone().unwrap());
        let verified = verify_parent_delivery(
            sink.dir.parent().unwrap(),
            &all,
            &binding,
            &limits,
            parent.budget.original_limits(),
            owned.ledger_before_owners,
        )
        .unwrap();
        assert!(verified.delivery_complete); // Complete raw delivery, not producer completion.

        let artifacts: Vec<_> = owned
            .items
            .iter()
            .map(|a| a.root_artifact.clone().unwrap())
            .collect();
        verify_slot_identities(&artifacts).unwrap();
        crate::ml_dataset::capture::verify_live_committed_prefix_visit(
            sink.dir.parent().unwrap(),
            &binding,
            &limits,
            &mut |_| Ok(()),
        )
        .unwrap()
        .unwrap();
    }
    #[test]
    fn root_collision_keeps_live_acknowledgement_and_stops_later_items() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let root = sink.dir.parent().unwrap();
        let original = construction_tests::fixture();
        let mut budget = construction_tests::budget();
        let mut delivery = ParentDeliveryFile::admit(&mut budget).unwrap();
        let mut global = 0;
        let mut run = run_canonical_pipeline(
            &original,
            &binding,
            &limits,
            &"8".repeat(64),
            &"9".repeat(64),
            native::Engine::F4Macaulay,
            3,
            &mut budget,
            &mut global,
            &mut |_, _, _| Ok(()),
        );
        let mut parent = ParentRetention::begin(&mut budget).unwrap();
        let mut owned = take_parent_artifacts(&mut run, &mut parent).value.unwrap();
        std::fs::write(root.join(KOBLITZ_SLOTS[0].0), b"existing conflicting bytes").unwrap();
        assert_eq!(
            persist_parent_artifacts(&mut owned, &mut parent, root, &binding, &limits),
            Err(Stop::Observer)
        );
        assert!(owned.items[0].capture_acknowledged && owned.items[0].root_artifact.is_none());
        assert!(owned.persistence_error.is_some());
        assert!(owned
            .items
            .iter()
            .skip(1)
            .all(|a| !a.capture_acknowledged && a.root_artifact.is_none()));
        assert_eq!(
            std::fs::read(sink.dir.join("payloads").join(&owned.items[0].payload.path)).unwrap(),
            owned.items[0].payload.bytes
        );
        assert_eq!(
            std::fs::read(root.join(KOBLITZ_SLOTS[0].0)).unwrap(),
            b"existing conflicting bytes"
        );
        let work = parent.budget.checking_work;
        assert_eq!(
            persist_parent_artifacts(&mut owned, &mut parent, root, &binding, &limits),
            Err(Stop::Observer)
        );
        assert_eq!(parent.budget.checking_work, work);
        delivery.finish(&owned, &parent, &binding, root).unwrap();
        let artifacts = vec![delivery.artifact.clone().unwrap()];
        let verified = verify_parent_delivery(
            root,
            &artifacts,
            &binding,
            &limits,
            parent.budget.original_limits(),
            owned.ledger_before_owners,
        )
        .unwrap();
        assert!(!verified.delivery_complete);
        assert_eq!(verified.capture_acknowledgements, 1);
        assert_eq!(verified.root_artifacts, 0);
    }
    #[test]
    fn wrong_attempt_or_no_headroom_prevents_all_storage() {
        for no_headroom in [false, true] {
            let binding = matrix_tests::binding();
            let limits = capture_limits(16).unwrap();
            let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
            let root = sink.dir.parent().unwrap();
            let original = construction_tests::fixture();
            let mut budget = construction_tests::budget();
            let mut global = 0;
            let mut run = run_canonical_pipeline(
                &original,
                &binding,
                &limits,
                &"8".repeat(64),
                &"9".repeat(64),
                native::Engine::F4Macaulay,
                3,
                &mut budget,
                &mut global,
                &mut |_, _, _| Ok(()),
            );
            let mut parent = ParentRetention::begin(&mut budget).unwrap();
            let mut owned = take_parent_artifacts(&mut run, &mut parent).value.unwrap();
            let before = std::fs::read(sink.dir.join("committed.json")).unwrap();
            let mut supplied = binding.clone();
            if no_headroom {
                parent.budget.trace_limit = parent.budget.trace_bytes;
            } else {
                supplied.cell_index += 1;
            }
            assert_eq!(
                persist_parent_artifacts(&mut owned, &mut parent, root, &supplied, &limits),
                Err(if no_headroom {
                    Stop::Trace
                } else {
                    Stop::Observer
                })
            );
            assert!(owned
                .items
                .iter()
                .all(|a| !a.capture_acknowledged && a.root_artifact.is_none()));
            assert_eq!(
                std::fs::read(sink.dir.join("committed.json")).unwrap(),
                before
            );
            assert!(!root.join(KOBLITZ_SLOTS[0].0).exists());
            assert!(!sink.dir.join("payloads").join(KOBLITZ_SLOTS[0].0).exists());
        }
    }
}

/// Extract the final payload of our trusted capture encoder without reparsing or
/// reserializing it. This is only used on event_typed's complete internal buffer,
/// never as a validator for arbitrary external JSON.
fn native_capture_payload(wire: &[u8]) -> Result<&[u8], Stop> {
    const MARKER: &[u8] = b"\"event\":{\"kind\":\"koblitz_m3_native\",\"payload\":";
    let end = wire.strip_suffix(b"}}\n").ok_or(Stop::Serialization)?;
    let start = end
        .windows(MARKER.len())
        .position(|w| w == MARKER)
        .ok_or(Stop::Serialization)?
        + MARKER.len();
    let payload = end.get(start..).ok_or(Stop::Serialization)?;
    if payload.first() != Some(&b'{') || payload.last() != Some(&b'}') {
        return Err(Stop::Serialization);
    }
    Ok(payload)
}
#[cfg(test)]
mod native_payload_reuse_tests {
    use super::*;
    use crate::ml_dataset::capture::capture_limits;
    #[test]
    fn reused_canonical_bytes_equal_original_typed_serialization() {
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let _sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let mut budget = construction_tests::budget();
        let mut events = BoundNativeEvents::new(&binding, &limits, &mut budget).unwrap();
        let mut global = 0;
        let event = NativeBoundary::Construction {
            stage: ConstructionBoundaryStage::Begin,
            reserved: Default::default(),
        };
        let digest = binding.digest().unwrap();
        let view = NativeBoundaryView {
            schema: "crypto.ml-dataset.koblitz-m3-events/v1",
            binding_sha256: &digest,
            sequence: 1,
            capture_sequence: 1,
            event: &event,
        };
        let mut expected = serde_json::to_vec(&view).unwrap();
        expected.push(b'\n');
        events.emit(event, &mut budget, &mut global).unwrap();
        assert_eq!(events.canonical, expected);
        assert_eq!(global, 1);
    }
    #[test]
    fn precommit_refusal_keeps_head_unchanged_and_does_not_poison_sink() {
        use crate::ml_dataset::capture::{CaptureError, TypedEventBuffer, TypedEventKind};
        let binding = matrix_tests::binding();
        let limits = capture_limits(16).unwrap();
        let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
        let before = std::fs::read(sink.dir.join("committed.json")).unwrap();
        let mut buffer =
            TypedEventBuffer::try_new(2048, TypedEventBuffer::requested_bytes(2048).unwrap())
                .unwrap();
        let mut calls = 0;
        super::super::LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut().unwrap() {
            super::super::EventSink::Bound(sink) => {
                let result = sink.event_typed_precommit(
                    TypedEventKind::KoblitzM3Native,
                    &serde_json::json!({"test":1}),
                    &mut buffer,
                    &mut |wire| {
                        calls += 1;
                        assert!(wire.ends_with(b"\n"));
                        Err(CaptureError::Integrity("selected precommit refusal".into()))
                    },
                );
                assert!(result.is_err());
                sink.verify_attempt_limits(&binding, &limits).unwrap();
            }
            _ => panic!("expected bound sink"),
        });
        assert_eq!(calls, 1);
        assert_eq!(
            std::fs::read(sink.dir.join("committed.json")).unwrap(),
            before
        );
        assert!(std::fs::read(sink.dir.join("events.jsonl"))
            .unwrap()
            .is_empty());
    }
}

const PARENT_DELIVERY_SLOT: (&str, &str, &str) = (
    "koblitz-m3-parent-delivery.json",
    "restricted-koblitz-m3-parent-delivery/v1",
    "json",
);
const PARENT_DELIVERY_CAP: usize = 16 * 1024;
#[derive(serde::Serialize)]
struct ParentItemView<'a> {
    logical_path: &'a str,
    path: &'a str,
    role: &'a str,
    encoding: &'a str,
    bytes: u64,
    attempted: bool,
    admitted: bool,
    quote: Option<RetentionQuote>,
    capture_acknowledged: bool,
    root_artifact: Option<&'a crate::ml_dataset::types::Artifact>,
    dedicated_publication: bool,
}
struct ParentItems<'a>(&'a [OwnedRetainedArtifact]);
impl serde::Serialize for ParentItems<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_seq(self.0.iter().map(|a| ParentItemView {
            logical_path: a.logical_path,
            path: &a.payload.path,
            role: &a.payload.role,
            encoding: &a.payload.encoding,
            bytes: a.payload.bytes.len() as u64,
            attempted: a.storage_attempted,
            admitted: a.storage_admitted,
            quote: a.storage_quote,
            capture_acknowledged: a.capture_acknowledged,
            root_artifact: a.root_artifact.as_ref(),
            dedicated_publication: a.publication.is_some(),
        }))
    }
}
#[derive(serde::Serialize)]
struct ParentDeliveryView<'a> {
    producer_input: Option<&'a ProducerInputReceipt>,
    schema: &'static str,
    snapshot_boundary: &'static str,
    binding: &'a crate::ml_dataset::capture::AttemptBinding,
    before: SavedBudgetLedger,
    after: SavedBudgetLedger,
    owner_quote: RetentionQuote,
    payload_bytes: u64,
    parent_stop: Option<Stop>,
    persistence_error: Option<&'a str>,
    items: ParentItems<'a>,
    delivery_complete: bool,
}
struct ParentDeliveryFile {
    encoding: TerminalEncoding,
    attempted: bool,
    artifact: Option<crate::ml_dataset::types::Artifact>,
    stop: Option<Stop>,
    error: Option<String>,
}
impl ParentDeliveryFile {
    fn admit(budget: &mut Budget) -> Result<Self, Stop> {
        // Root-only terminal storage is reserved before the producer. It cannot
        // use a fresh allowance to bypass a stopped parent session.
        budget.reserve_checking(
            32 * PARENT_DELIVERY_CAP as u64 + 65536,
            2 * 1024 * 1024 + 16384 + std::mem::size_of::<Self>() as u64,
            Instant::now(),
        )?;
        budget.reserve_trace(2 * PARENT_DELIVERY_CAP as u64 + 4096, Instant::now())?;
        Ok(Self {
            encoding: TerminalEncoding::admit(PARENT_DELIVERY_CAP, budget)?,
            attempted: false,
            artifact: None,
            stop: None,
            error: None,
        })
    }
    fn finish(
        &mut self,
        owned: &ParentArtifacts,
        parent: &ParentRetention<'_>,
        binding: &crate::ml_dataset::capture::AttemptBinding,
        root: &std::path::Path,
    ) -> Result<(), Stop> {
        if self.attempted {
            return Err(self.stop.unwrap_or(Stop::Phase));
        }
        self.attempted = true;
        let parent_stop = owned.retention_stop.or(parent.stop);
        let complete = parent_stop.is_none()
            && owned.persistence_error.is_none()
            && !owned.items.is_empty()
            && owned
                .items
                .iter()
                .all(|a| a.capture_acknowledged && a.root_artifact.is_some());
        let view = ParentDeliveryView {
            producer_input: owned.producer_input.as_deref(),
            schema: "crypto.ml-dataset.koblitz-m3-parent-delivery/v1",
            snapshot_boundary: "before-parent-record-encoding-and-storage",
            binding,
            before: owned.ledger_before_owners,
            after: parent.budget.saved_ledger(),
            owner_quote: owned.owner_quote,
            payload_bytes: owned.payload_bytes,
            parent_stop,
            persistence_error: owned.persistence_error.as_deref(),
            items: ParentItems(&owned.items),
            delivery_complete: complete,
        };
        if let Err(stop) = self.encoding.encode(&view) {
            self.stop = Some(stop);
            return Err(stop);
        }
        if Instant::now() >= self.encoding.deadline {
            self.stop = Some(Stop::Deadline);
            return Err(Stop::Deadline);
        }
        match crate::ml_dataset::storage::write_bytes(
            root,
            PARENT_DELIVERY_SLOT.0,
            PARENT_DELIVERY_SLOT.1,
            PARENT_DELIVERY_SLOT.2,
            &self.encoding.bytes,
        ) {
            Ok(a) => self.artifact = Some(a),
            Err(e) => {
                self.error = Some(e);
                self.stop = Some(Stop::Observer);
                return Err(Stop::Observer);
            }
        }
        if Instant::now() >= self.encoding.deadline {
            self.stop = Some(Stop::Deadline);
            return Err(Stop::Deadline);
        }
        Ok(())
    }
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedParentItem {
    logical_path: String,
    path: String,
    role: String,
    encoding: String,
    bytes: u64,
    attempted: bool,
    admitted: bool,
    quote: Option<RetentionQuote>,
    capture_acknowledged: bool,
    root_artifact: Option<crate::ml_dataset::types::Artifact>,
    dedicated_publication: bool,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedParentDelivery {
    #[serde(default)]
    producer_input: Option<ProducerInputReceipt>,
    schema: String,
    snapshot_boundary: String,
    binding: crate::ml_dataset::capture::AttemptBinding,
    before: SavedBudgetLedger,
    after: SavedBudgetLedger,
    owner_quote: RetentionQuote,
    payload_bytes: u64,
    parent_stop: Option<Stop>,
    persistence_error: Option<String>,
    items: Vec<SavedParentItem>,
    delivery_complete: bool,
}
#[derive(Debug)]
struct VerifiedParentDelivery {
    producer_transition_valid: bool,
    producer_entry_anchor_verified: bool,
    delivery_complete: bool,
    root_artifacts: usize,
    capture_acknowledgements: usize,
}
/// Admission and original mathematical replay supply expected_before. This
/// verifier authenticates delivery/accounting, not producer instruction counts.
fn verify_parent_delivery(
    dir: &std::path::Path,
    artifacts: &[crate::ml_dataset::types::Artifact],
    binding: &crate::ml_dataset::capture::AttemptBinding,
    capture: &crate::ml_dataset::capture::CaptureLimits,
    original: OriginalLimits,
    expected_before: SavedBudgetLedger,
) -> Result<VerifiedParentDelivery, String> {
    use super::prime_f4_wide::{artifact, read_bounded};
    let file = artifact(
        artifacts,
        PARENT_DELIVERY_SLOT.0,
        PARENT_DELIVERY_SLOT.1,
        PARENT_DELIVERY_SLOT.2,
        true,
    )?
    .ok_or("missing parent delivery record")?;
    let bytes = read_bounded(dir, PARENT_DELIVERY_SLOT.0, PARENT_DELIVERY_CAP)?;
    if file.bytes != bytes.len() as u64 || file.sha256 != crate::ml_dataset::digest(&bytes) {
        return Err("parent delivery file hash mismatch".into());
    }
    let r: SavedParentDelivery =
        serde_json::from_slice(&bytes).map_err(|e| format!("strict parent delivery: {e}"))?;
    if r.schema != "crypto.ml-dataset.koblitz-m3-parent-delivery/v1"
        || r.snapshot_boundary != "before-parent-record-encoding-and-storage"
        || &r.binding != binding
        || r.before != expected_before
        || r.items.len() > 10
    {
        return Err("parent delivery original identity mismatch".into());
    }
    for ledger in [r.before, r.after] {
        verify_budget_ledger(
            &serde_json::to_vec(&ledger).map_err(|e| e.to_string())?,
            original,
        )?;
    }
    if r.before.phase != r.after.phase
        || r.before.first_stop != r.after.first_stop
        || r.before.native_reserved != r.after.native_reserved
        || r.before.group_additions != r.after.group_additions
        || r.before.checking_reported_work != r.after.checking_reported_work
    {
        return Err("parent changed producer state".into());
    }
    let prefix = crate::ml_dataset::capture::verify_live_committed_prefix(dir, binding, capture)?
        .ok_or("missing original capture prefix")?;
    // Bind inventory independently of the claimed sums. Otherwise removing a
    // row and subtracting its charges can forge a complete delivery.
    let has_row = |path: &str, role: &str, encoding: &str| {
        r.items
            .iter()
            .any(|a| a.path == path && a.role == role && a.encoding == encoding)
    };
    for a in artifacts {
        let canonical = KOBLITZ_SLOTS.iter().any(|&(path, role, encoding)| {
            a.path == path && a.role == role && a.encoding == encoding
        }) || (a.path == "pdp-witness.json"
            && a.role == "independently-verified-pdp"
            && a.encoding == "json");
        if canonical && !has_row(&a.path, &a.role, &a.encoding) {
            return Err("parent inventory omits registered artifact".into());
        }
    }
    if expected_before.phase == Phase::Complete && expected_before.first_stop.is_none() {
        for &(path, role, encoding) in &KOBLITZ_SLOTS {
            if !has_row(path, role, encoding) {
                return Err("parent inventory omits completed producer artifact".into());
            }
        }
        if prefix
            .payloads
            .iter()
            .any(|(_, p)| p.path == "payloads/pdp-witness.json")
            && !has_row("pdp-witness.json", "independently-verified-pdp", "json")
        {
            return Err("parent inventory omits committed witness".into());
        }
    }
    if let Some(p) = &r.producer_input {
        if p.binding_sha256 != binding.digest()? || p.root_artifact.is_some() {
            return Err("producer input receipt identity mismatch".into());
        }
        verify_producer_input_transition(p, expected_before, capture)?;
        if !has_row(KOBLITZ_SLOTS[0].0, KOBLITZ_SLOTS[0].1, KOBLITZ_SLOTS[0].2) {
            return Err("producer input omitted from parent inventory".into());
        }
    }
    let mut owner = RetentionQuote {
        work: 512,
        bytes: 512,
        trace: 0,
    };
    let mut charged = RetentionQuote::default();
    let mut payload_total = 0u64;
    let mut previous_rank = None;
    let mut uncompleted = false;
    let mut roots = 0usize;
    let mut acknowledgements = 0usize;
    for a in &r.items {
        let (path, role, encoding, rank) = match a.logical_path.as_str() {
            "koblitz-m3-input.json" => (
                KOBLITZ_SLOTS[0].0,
                KOBLITZ_SLOTS[0].1,
                KOBLITZ_SLOTS[0].2,
                0,
            ),
            "koblitz-m3-events.jsonl" => (
                KOBLITZ_SLOTS[1].0,
                KOBLITZ_SLOTS[1].1,
                KOBLITZ_SLOTS[1].2,
                1,
            ),
            "koblitz-m3-result.json" => (
                KOBLITZ_SLOTS[2].0,
                KOBLITZ_SLOTS[2].1,
                KOBLITZ_SLOTS[2].2,
                5,
            ),
            "koblitz-m3-group-replay.json" => (
                KOBLITZ_SLOTS[3].0,
                KOBLITZ_SLOTS[3].1,
                KOBLITZ_SLOTS[3].2,
                4,
            ),
            "pdp-witness.json" => ("pdp-witness.json", "independently-verified-pdp", "json", 3),
            other => {
                let index = RAW_RETENTION_NAMES
                    .iter()
                    .position(|(logical, _)| *logical == other)
                    .ok_or("unknown parent artifact identity")?;
                (
                    RAW_RETENTION_NAMES[index].1,
                    "unregistered-partial-artifact",
                    "native",
                    [0, 2, 3, 4, 5][index],
                )
            }
        };
        if a.path != path
            || a.role != role
            || a.encoding != encoding
            || a.bytes > 8 * 1024 * 1024
            || previous_rank.is_some_and(|p| rank <= p)
            || a.dedicated_publication != (path == "pdp-witness.json")
        {
            return Err("parent artifact identity/order mismatch".into());
        }
        previous_rank = Some(rank);
        let identities = (path.len() + role.len() + encoding.len()) as u64;
        owner.work = owner
            .work
            .checked_add(512 + 8 * identities)
            .ok_or("owner work overflow")?;
        owner.bytes = owner
            .bytes
            .checked_add(512 + identities)
            .ok_or("owner bytes overflow")?;
        payload_total = payload_total
            .checked_add(a.bytes)
            .ok_or("payload sum overflow")?;
        let producer = r
            .producer_input
            .as_ref()
            .filter(|_| path == KOBLITZ_SLOTS[0].0);
        if let Some(p) = producer {
            if a.attempted
                || a.admitted
                || a.quote.is_some()
                || a.bytes != p.payload_bytes
                || a.capture_acknowledged != p.capture_sequence.is_some()
                || a.root_artifact.is_none() && p.exit.first_stop.is_none()
            {
                return Err("producer input rewritten as parent action".into());
            }
        }
        if a.attempted != a.quote.is_some()
            || a.admitted && !a.attempted
            || a.capture_acknowledged && !a.admitted && producer.is_none()
            || a.root_artifact.is_some() && !a.capture_acknowledged
            || uncompleted && a.attempted
        {
            return Err("parent acknowledgement order mismatch".into());
        }
        if a.attempted {
            let quote = quote_parent_storage_length(a.bytes, a.dedicated_publication, capture)
                .map_err(|s| format!("storage quote: {s:?}"))?;
            if a.quote != Some(quote) {
                return Err("parent storage quote mismatch".into());
            }
            if a.admitted {
                charged.work = charged
                    .work
                    .checked_add(quote.work)
                    .ok_or("parent work overflow")?;
                charged.bytes = charged
                    .bytes
                    .checked_add(quote.bytes)
                    .ok_or("parent memory overflow")?;
                charged.trace = charged
                    .trace
                    .checked_add(quote.trace)
                    .ok_or("parent trace overflow")?;
            }
        }
        if a.root_artifact.is_none() && producer.is_none() {
            uncompleted = true;
        }
        let committed: Vec<_> = prefix
            .payloads
            .iter()
            .filter(|(_, p)| p.path == format!("payloads/{path}"))
            .collect();
        if a.capture_acknowledged || a.dedicated_publication {
            if committed.len() != 1 {
                return Err("parent acknowledgement lacks unique capture".into());
            }
            let p = &committed[0].1;
            if p.role != role || p.encoding != encoding || p.bytes != a.bytes {
                return Err("parent capture identity mismatch".into());
            }
        }
        if let Some(p) = producer {
            if let Some(sequence) = p.capture_sequence {
                if committed[0].0 != sequence
                    || p.payload_sha256.map(|h| hex::encode(h)).as_deref()
                        != Some(committed[0].1.sha256.as_str())
                {
                    return Err("producer input capture receipt mismatch".into());
                }
            }
        }
        if a.capture_acknowledged {
            acknowledgements += 1;
        }
        if let Some(root) = &a.root_artifact {
            if root.path != path
                || root.role != role
                || root.encoding != encoding
                || root.bytes != a.bytes
                || root.sha256 != committed[0].1.sha256
            {
                return Err("parent root differs from capture".into());
            }
            let registered = artifact(artifacts, path, role, encoding, true)?
                .ok_or("parent root missing from manifest")?;
            if registered != root {
                return Err("parent root differs from manifest".into());
            }
            crate::ml_dataset::storage::check_file(
                &crate::ml_dataset::storage::contained_path(dir, path, false)?,
                &root.sha256,
                root.bytes,
            )?;
            roots += 1;
        }
    }
    if owner != r.owner_quote || payload_total != r.payload_bytes {
        return Err("parent owner quote mismatch".into());
    }
    let work = r
        .before
        .checking_reserved_work
        .checked_add(owner.work)
        .and_then(|n| n.checked_add(charged.work))
        .ok_or("parent ledger work overflow")?;
    let memory = r
        .before
        .adapter_reserved_bytes
        .checked_add(owner.bytes)
        .and_then(|n| n.checked_add(charged.bytes))
        .ok_or("parent ledger memory overflow")?;
    let trace = r
        .before
        .charged_trace_bytes
        .checked_add(owner.trace)
        .and_then(|n| n.checked_add(charged.trace))
        .ok_or("parent ledger trace overflow")?;
    if work != r.after.checking_reserved_work
        || memory != r.after.adapter_reserved_bytes
        || trace != r.after.charged_trace_bytes
    {
        return Err("parent cumulative ledger mismatch".into());
    }
    if r.persistence_error.is_some() && r.parent_stop != Some(Stop::Observer) {
        return Err("parent error/stop mismatch".into());
    }
    let complete = r.parent_stop.is_none()
        && r.persistence_error.is_none()
        && !r.items.is_empty()
        && roots == r.items.len()
        && acknowledgements == r.items.len();
    if complete != r.delivery_complete {
        return Err("parent delivery completion mismatch".into());
    }
    Ok(VerifiedParentDelivery {
        producer_transition_valid: r.producer_input.is_some(),
        // Later parent boundary does not independently anchor earlier entry.
        producer_entry_anchor_verified: false,
        delivery_complete: complete,
        root_artifacts: roots,
        capture_acknowledgements: acknowledgements,
    })
}

// Draft for insertion in koblitz_m3.rs after the current shared-worker test gate.
// Native source review: F2mElement::from_bit_positions uses inline Words for
// n<=61; it avoids temporary BigUint digit vectors. Worker verification already
// checks irreducibility, curve/order, subgroup and factor-base mathematics.
fn decode_verified_worker_math(
    token: &super::VerifiedWorkerOriginal<'_>,
    budget: &mut Budget,
) -> Result<MathInput, Stop> {
    use crate::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
    use num_bigint::BigUint;
    // Fixed modeled admission: bounded JSON lookups, <=20 digit decimal words,
    // <=61 exponents, <=8 signed points, <=4 field-element bit scans. This is
    // a checking tariff, not a retired-instruction measurement.
    const WORK: u64 = 8192;
    let bytes = (std::mem::size_of::<MathInput>()
        + 2 * std::mem::size_of::<BinaryCurve>()
        + 4 * std::mem::size_of::<BigUint>()
        + 4 * std::mem::size_of::<F2mElement>()
        + 3 * 61 * std::mem::size_of::<u32>()
        + 16 * std::mem::size_of::<(u64, u64)>()
        + 64) as u64;
    budget.reserve_checking(WORK, bytes, Instant::now())?;
    let decoded = (|| {
        let c = token.config;
        let p = token.public;
        let base = token.factor_base;
        if token.binding.is_none()
            || c.family != "koblitz"
            || c.m != 3
            || !((23..=31).contains(&c.n) || (47..=61).contains(&c.n))
            || token.public_bytes > 64 * 1024
            || token.factor_base_bytes > 128 * 1024
            || p["family"] != "koblitz"
            || p["n"].as_u64() != Some(c.n as u64)
            || p["m"].as_u64() != Some(3)
            || p["twist"].as_bool() != Some(c.twist)
            || base["requested_size"].as_u64() != Some(c.factor_base_size as u64)
            || p["field"]["degree"].as_u64() != Some(c.n as u64)
            || p["field"]["characteristic"] != "2"
        {
            return Err(Stop::InvalidLimits);
        }
        fn word(v: &serde_json::Value) -> Result<u64, Stop> {
            let s = v.as_str().ok_or(Stop::InvalidLimits)?;
            if s.is_empty() || s.len() > 20 || (s.len() > 1 && s.as_bytes()[0] == b'0') {
                return Err(Stop::InvalidLimits);
            }
            s.bytes().try_fold(0u64, |n, b| {
                if !b.is_ascii_digit() {
                    return Err(Stop::InvalidLimits);
                }
                n.checked_mul(10)
                    .and_then(|n| n.checked_add((b - b'0') as u64))
                    .ok_or(Stop::InvalidLimits)
            })
        }
        let q = 1u64 << c.n;
        let point = |v: &serde_json::Value| -> Result<(u64, u64), Stop> {
            let x = word(&v["x"])?;
            let y = word(&v["y"])?;
            if x >= q || y >= q {
                return Err(Stop::InvalidLimits);
            }
            Ok((x, y))
        };
        let a = word(&p["curve"]["a"])?;
        if a >= q || word(&p["curve"]["b"])? != 1 {
            return Err(Stop::InvalidLimits);
        }
        let generator = point(&p["generator"])?;
        let target = point(&p["target"])?;
        let order = word(&p["curve"]["subgroup_order"])?;
        let cofactor = word(&p["curve"]["cofactor"])?;
        let rows = base["points"].as_array().ok_or(Stop::InvalidLimits)?;
        if rows.is_empty() || rows.len() > 4 {
            return Err(Stop::InvalidLimits);
        }
        let mut points = [(0u64, 0u64); 8];
        let mut count = 0;
        for row in rows {
            let (x, y) = point(row)?;
            for value in [(x, y), (x, x ^ y)] {
                if !points[..count].contains(&value) {
                    points[count] = value;
                    count += 1;
                }
            }
        }
        if count != 4 {
            return Err(Stop::InvalidLimits);
        }
        points[..count].sort_unstable();
        let signed = [points[0], points[1], points[2], points[3]];
        if signed[0].0 != signed[1].0
            || signed[2].0 != signed[3].0
            || signed[0].0 == signed[2].0
            || signed[0].1 ^ signed[0].0 != signed[1].1
            || signed[2].1 ^ signed[2].0 != signed[3].1
        {
            return Err(Stop::InvalidLimits);
        }
        let terms = p["field"]["irreducible_low_terms"]
            .as_array()
            .ok_or(Stop::InvalidLimits)?;
        if terms.is_empty() || terms.len() > 61 {
            return Err(Stop::InvalidLimits);
        }
        let mut low_terms = Vec::new();
        low_terms
            .try_reserve_exact(terms.len())
            .map_err(|_| Stop::Allocation)?;
        for term in terms {
            let value = term.as_u64().ok_or(Stop::InvalidLimits)?;
            if value >= c.n as u64 || low_terms.contains(&(value as u32)) {
                return Err(Stop::InvalidLimits);
            }
            low_terms.push(value as u32);
        }
        low_terms.sort_unstable();
        let element = |value: u64| {
            let mut positions = [0u32; 61];
            let mut count = 0;
            for bit in 0..c.n {
                if value & (1u64 << bit) != 0 {
                    positions[count] = bit;
                    count += 1;
                }
            }
            F2mElement::from_bit_positions(&positions[..count], c.n)
        };
        Ok(MathInput {
            curve: Box::new(BinaryCurve {
                m: c.n,
                irreducible: IrreduciblePoly {
                    degree: c.n,
                    low_terms,
                },
                a: element(a),
                b: element(1),
                generator: BinaryPoint::Affine {
                    x: element(generator.0),
                    y: element(generator.1),
                },
                order: BigUint::from(order),
                cofactor: BigUint::from(cofactor),
            }),
            target,
            codes: [signed[0].0, signed[2].0],
            signed,
        })
    })();
    match decoded {
        Ok(value) => {
            budget.check(Instant::now())?;
            Ok(value)
        }
        Err(stop) => budget.fail(stop),
    }
}

#[cfg(test)]
mod worker_math_decode_tests {
    use super::super::verified_worker_original_tests::Fixture;
    use super::*;
    fn budget(config: &crate::ml_dataset::types::CaseConfig) -> Budget {
        let now = Instant::now();
        Budget::new(
            config.work_limit,
            512 << 20,
            16 << 20,
            now + Duration::from_secs(20),
            now,
        )
        .unwrap()
    }
    #[test]
    fn worker_math_decode_matches_independent_original_replay_for_sizes_and_twists() {
        for n in [23, 31, 47, 53, 61] {
            for twist in [false, true] {
                let f = Fixture::with_shape(n, twist, 4);
                let mut registry = Vec::new();
                let token = super::super::verify_and_store_worker_originals(
                    &f.config,
                    &f.case,
                    &f.root,
                    Some(&f.binding),
                    &mut registry,
                )
                .unwrap();
                token
                    .matches_bound(&f.config, &f.case, &f.root.join("live"), &f.binding)
                    .unwrap();
                let mut b = budget(&f.config);
                let decoded = decode_verified_worker_math(&token, &mut b).unwrap();
                let replay = original_math(&f.config, &f.case).unwrap();
                assert_eq!(decoded.target, replay.target);
                assert_eq!(decoded.codes, replay.codes);
                assert_eq!(decoded.signed, replay.signed);
                assert_eq!(decoded.curve.m, replay.curve.m);
                assert_eq!(
                    decoded.curve.irreducible.low_terms,
                    replay.curve.irreducible.low_terms
                );
                assert_eq!(decoded.curve.a, replay.curve.a);
                assert_eq!(decoded.curve.b, replay.curve.b);
                assert_eq!(decoded.curve.generator, replay.curve.generator);
                assert_eq!(decoded.curve.order, replay.curve.order);
                assert_eq!(decoded.curve.cofactor, replay.curve.cofactor);
                assert_eq!(b.checking_work, 8192);
                assert_eq!(b.native, Reservation::default());
                assert_eq!(b.trace_bytes, 0);
            }
        }
    }
    #[test]
    fn worker_math_decode_reserves_before_work_and_preserves_original_stops() {
        let f = Fixture::with_shape(23, false, 4);
        let mut registry = Vec::new();
        let token = super::super::verify_and_store_worker_originals(
            &f.config,
            &f.case,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        let mut full = budget(&f.config);
        decode_verified_worker_math(&token, &mut full).unwrap();
        let bytes = full.adapter_bytes;
        let mut exact = budget(&f.config);
        exact.checking_work = exact.native_work_limit - 8192;
        exact.adapter_bytes = exact.memory_limit - bytes;
        assert!(decode_verified_worker_math(&token, &mut exact).is_ok());
        for case in 0..3 {
            let mut b = budget(&f.config);
            let expected = match case {
                0 => {
                    b.checking_work = b.native_work_limit - 8191;
                    Stop::Work
                }
                1 => {
                    b.adapter_bytes = b.memory_limit - bytes + 1;
                    Stop::Bytes
                }
                _ => {
                    b.construction_deadline = Instant::now() - Duration::from_secs(1);
                    Stop::Deadline
                }
            };
            let before = (b.checking_work, b.adapter_bytes);
            assert!(matches!(decode_verified_worker_math(&token,&mut b),Err(s) if s==expected));
            assert_eq!((b.checking_work, b.adapter_bytes), before);
            assert_eq!(b.stopped, Some(expected));
            assert_eq!(b.native, Reservation::default());
            assert_eq!(b.trace_bytes, 0);
        }
        // Verification writes immutable telemetry; this control is a new attempt.
        let f = Fixture::with_shape(23, false, 4);
        let mut registry = Vec::new();
        let unbound = super::super::verify_and_store_worker_originals(
            &f.config,
            &f.case,
            &f.root,
            None,
            &mut registry,
        )
        .unwrap();
        let mut b = budget(&f.config);
        assert!(matches!(
            decode_verified_worker_math(&unbound, &mut b),
            Err(Stop::InvalidLimits)
        ));
        assert_eq!(b.stopped, Some(Stop::InvalidLimits));
    }
}

/// One original-input publication attempt before matrix execution. Receipts are
/// retained before post-call deadline checks; root storage follows capture.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ProducerInputReceipt {
    binding_sha256: String,
    payload_sha256: Option<[u8; 32]>,
    payload_bytes: u64,
    entry: SavedBudgetLedger,
    exit: SavedBudgetLedger,
    quote: Option<RetentionQuote>,
    checking_admitted: bool,
    trace_admitted: bool,
    capture_count_before: u64,
    capture_sequence: Option<u64>,
    root_artifact: Option<crate::ml_dataset::types::Artifact>,
}
/// This verifies modeled transitions, not an independently measured entry balance.
fn verify_producer_input_transition(
    r: &ProducerInputReceipt,
    expected_parent: SavedBudgetLedger,
    capture: &crate::ml_dataset::capture::CaptureLimits,
) -> Result<(), String> {
    let q = r.quote.ok_or("producer input missing storage quote")?;
    if q != quote_parent_storage_length(r.payload_bytes, false, capture)
        .map_err(|s| format!("producer storage quote: {s:?}"))?
        || r.entry.phase != Phase::Constructed
        || r.entry.first_stop.is_some()
        || r.entry.original_limits != expected_parent.original_limits
        || r.exit.original_limits != r.entry.original_limits
        || r.exit.phase != r.entry.phase
        || r.exit.native_reserved != r.entry.native_reserved
        || r.exit.checking_reported_work != r.entry.checking_reported_work
        || r.exit.group_additions != r.entry.group_additions
        || r.trace_admitted && !r.checking_admitted
        || r.capture_sequence.is_some() && !r.trace_admitted
        || r.root_artifact.is_some() && r.capture_sequence.is_none()
        || r.payload_sha256.is_none()
        || (!r.trace_admitted || r.capture_sequence.is_none()) && r.exit.first_stop.is_none()
    {
        return Err("producer input admission order mismatch".into());
    }
    let work = if r.checking_admitted { q.work } else { 0 };
    let bytes = if r.checking_admitted { q.bytes } else { 0 };
    let trace = if r.trace_admitted { q.trace } else { 0 };
    if r.entry.checking_reserved_work.checked_add(work) != Some(r.exit.checking_reserved_work)
        || r.entry.adapter_reserved_bytes.checked_add(bytes) != Some(r.exit.adapter_reserved_bytes)
        || r.entry.charged_trace_bytes.checked_add(trace) != Some(r.exit.charged_trace_bytes)
        || r.exit.checking_reserved_work > expected_parent.checking_reserved_work
        || r.exit.adapter_reserved_bytes > expected_parent.adapter_reserved_bytes
        || r.exit.charged_trace_bytes > expected_parent.charged_trace_bytes
        || r.exit.native_reserved.work > expected_parent.native_reserved.work
        || r.exit.native_reserved.bytes > expected_parent.native_reserved.bytes
        || r.exit.group_additions > expected_parent.group_additions
        || r.exit.first_stop.is_some() && r.exit.first_stop != expected_parent.first_stop
        || r.capture_sequence
            .is_some_and(|seq| r.capture_count_before.checked_add(1) != Some(seq))
    {
        return Err("producer input modeled ledger mismatch".into());
    }
    for ledger in [r.entry, r.exit] {
        verify_budget_ledger(
            &serde_json::to_vec(&ledger).map_err(|e| e.to_string())?,
            expected_parent.original_limits,
        )?;
    }
    Ok(())
}
struct PreMatrixInputStore<'a> {
    root: &'a std::path::Path,
    limits: &'a crate::ml_dataset::capture::CaptureLimits,
    binding_digest: [u8; 64],
    receipt: Option<Box<ProducerInputReceipt>>,
    attempted: bool,
    capture_acknowledged: bool,
    capture_sequence: Option<u64>,
    root_artifact: Option<crate::ml_dataset::types::Artifact>,
    stop: Option<Stop>,
    error: Option<String>,
}
impl<'a> PreMatrixInputStore<'a> {
    fn new(
        root: &'a std::path::Path,
        binding: &crate::ml_dataset::capture::AttemptBinding,
        limits: &'a crate::ml_dataset::capture::CaptureLimits,
        budget: &mut Budget,
    ) -> Result<Self, Stop> {
        // Original setup quote plus a bounded one-shot attachment allowance:
        // 32-byte hex encoding, two 64-byte identity comparisons and fixed
        // state/length checks. This allowance remains charged if never used.
        budget.reserve_checking(
            4096 + 512,
            (std::mem::size_of::<Self>() + std::mem::size_of::<ProducerInputReceipt>()) as u64
                + 4096,
            Instant::now(),
        )?;
        let digest = binding
            .digest()
            .map_err(|_| Stop::InvalidLimits)
            .or_else(|s| budget.fail(s))?;
        let mut binding_digest = [0; 64];
        binding_digest.copy_from_slice(digest.as_bytes());
        Ok(Self {
            root,
            limits,
            binding_digest,
            receipt: Some(Box::new(ProducerInputReceipt {
                binding_sha256: digest,
                payload_sha256: None,
                payload_bytes: 0,
                entry: budget.saved_ledger(),
                exit: budget.saved_ledger(),
                quote: None,
                checking_admitted: false,
                trace_admitted: false,
                capture_count_before: 0,
                capture_sequence: None,
                root_artifact: None,
            })),
            attempted: false,
            capture_acknowledged: false,
            capture_sequence: None,
            root_artifact: None,
            stop: None,
            error: None,
        })
    }
    fn publish(
        &mut self,
        bytes: &[u8],
        budget: &mut Budget,
        global: &mut usize,
    ) -> Result<(), String> {
        self.publish_with_finish_time(bytes, budget, global, Instant::now)
    }
    fn publish_with_finish_time(
        &mut self,
        bytes: &[u8],
        budget: &mut Budget,
        global: &mut usize,
        finish_time: impl FnOnce() -> Instant,
    ) -> Result<(), String> {
        if self.attempted {
            return Err("original input publication already attempted".into());
        }
        self.attempted = true;
        let receipt = self.receipt.as_mut().expect("publisher owns receipt");
        receipt.entry = budget.saved_ledger();
        receipt.payload_bytes = bytes.len() as u64;
        receipt.capture_count_before = *global as u64;
        let result = (|| {
            if budget.phase != Phase::Constructed {
                return budget
                    .fail::<()>(Stop::Phase)
                    .map_err(|s| format!("input phase: {s:?}"));
            }
            if bytes.len() > INPUT_BYTE_CAP {
                return budget
                    .fail::<()>(Stop::Bytes)
                    .map_err(|s| format!("input bytes: {s:?}"));
            }
            let next = global
                .checked_add(1)
                .ok_or("input capture sequence overflow")?;
            let quote = quote_parent_storage_length(bytes.len() as u64, false, self.limits)
                .map_err(|s| format!("input storage quote: {s:?}"))?;
            receipt.quote = Some(quote);
            budget
                .reserve_checking(quote.work, quote.bytes, Instant::now())
                .map_err(|s| format!("input storage admission: {s:?}"))?;
            receipt.checking_admitted = true;
            // Existing 32-per-byte storage tariff covers this digest work.
            use sha2::Digest;
            receipt.payload_sha256 = Some(sha2::Sha256::digest(bytes).into());
            budget
                .reserve_trace(quote.trace, Instant::now())
                .map_err(|s| format!("input storage trace: {s:?}"))?;
            receipt.trace_admitted = true;
            super::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
                Some(super::EventSink::Bound(sink)) => sink.verify_digest_limits_at_count(
                    &self.binding_digest,
                    self.limits,
                    *global as u64,
                ),
                _ => Err("original input requires bound capture".into()),
            })?;
            super::live_borrowed_payload(
                KOBLITZ_SLOTS[0].0,
                KOBLITZ_SLOTS[0].1,
                KOBLITZ_SLOTS[0].2,
                bytes,
            )?;
            // Generic payload storage is idempotent: success need not append
            // an event. Authenticate a new sequence before claiming one.
            super::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
                Some(super::EventSink::Bound(sink)) => sink.verify_digest_limits_at_count(
                    &self.binding_digest,
                    self.limits,
                    next as u64,
                ),
                _ => Err("original input capture disappeared".into()),
            })?;
            self.capture_acknowledged = true;
            self.capture_sequence = Some(next as u64);
            *global = next;
            budget
                .check(Instant::now())
                .map_err(|s| format!("input root deadline: {s:?}"))?;
            self.root_artifact = Some(crate::ml_dataset::storage::write_bytes(
                self.root,
                KOBLITZ_SLOTS[0].0,
                KOBLITZ_SLOTS[0].1,
                KOBLITZ_SLOTS[0].2,
                bytes,
            )?);
            budget
                .check(finish_time())
                .map_err(|s| format!("input stored after deadline: {s:?}"))?;
            Ok::<(), String>(())
        })();
        if let Err(error) = &result {
            if budget.stopped.is_none() {
                let _ = budget.observer_failed();
            }
            self.stop = budget.stopped;
            self.error = Some(error.clone());
        }
        receipt.exit = budget.saved_ledger();
        receipt.capture_sequence = self.capture_sequence;
        result
    }
    /// Attach once to the exact canonical input already hashed by its wrapper.
    /// Fixed comparison/encoding work uses the constructor's prepaid 512 units.
    /// No hashing, write or fresh budget reservation occurs after producer stop.
    /// Error strings use the constructor's existing metadata allocation allowance.
    fn attach_to(&mut self, run: &mut CanonicalPipelineRun<'_>) -> Result<(), String> {
        if !self.attempted || run.artifacts_taken || run.producer_input.is_some() {
            return Err("producer input attachment state mismatch".into());
        }
        let r = self
            .receipt
            .as_ref()
            .ok_or("producer input receipt already moved")?;
        let digest = r
            .payload_sha256
            .ok_or("producer input lacks admitted digest")?;
        let mut encoded = [0u8; 64];
        for (i, byte) in digest.iter().copied().enumerate() {
            encoded[2 * i] = b"0123456789abcdef"[(byte >> 4) as usize];
            encoded[2 * i + 1] = b"0123456789abcdef"[(byte & 15) as usize];
        }
        if run.input_sha256.as_ref().map(|s| s.as_bytes()) != Some(encoded.as_slice()) {
            return Err("producer input differs from canonical buffer".into());
        }
        let native = &run
            .pipeline
            .as_ref()
            .ok_or("missing producer pipeline")?
            .native;
        if native.events.as_ref().map(|e| e.binding_digest) != Some(self.binding_digest) {
            return Err("producer input attempt differs from native attempt".into());
        }
        let retained_len = if let Some(m) = &native.matrix {
            m.original_bytes.len()
        } else if let Some(f) = &native.persistence_failure {
            if !f.serialized_complete {
                return Err("producer input is not fully encoded".into());
            }
            f.bytes.len()
        } else {
            return Err("producer input buffer absent".into());
        };
        if retained_len as u64 != r.payload_bytes {
            return Err("producer input length mismatch".into());
        }
        let mut receipt = self.receipt.take().unwrap();
        receipt.root_artifact = self.root_artifact.take();
        run.producer_input = Some(receipt);
        Ok(())
    }
}

#[cfg(test)]
mod pre_matrix_input_store_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine;
    #[test]
    fn input_is_stored_before_both_matrix_engines_and_failures_preserve_acknowledgements() {
        for engine in [Engine::F4Macaulay, Engine::F5KoszulFrobenius] {
            for mode in 0..7 {
                let binding = matrix_tests::binding();
                let limits = crate::ml_dataset::capture::capture_limits(16).unwrap();
                let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
                let root = sink.dir.parent().unwrap();
                if mode == 1 {
                    std::fs::write(root.join(KOBLITZ_SLOTS[0].0), b"root collision").unwrap();
                }
                if mode == 2 {
                    std::fs::create_dir_all(sink.dir.join("payloads")).unwrap();
                    std::fs::write(
                        sink.dir.join("payloads").join(KOBLITZ_SLOTS[0].0),
                        b"capture collision",
                    )
                    .unwrap();
                }
                let original = construction_tests::fixture();
                let mut budget = construction_tests::budget();
                let mut wrong = binding.clone();
                if mode == 4 {
                    wrong.cell_index += 1;
                }
                let mut storage =
                    PreMatrixInputStore::new(root, &wrong, &limits, &mut budget).unwrap();
                let late = budget.deadline + Duration::from_secs(1);
                let mut global = 0;
                let run = run_native_phases(
                    &original,
                    &binding,
                    &limits,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    engine,
                    3,
                    &mut budget,
                    &mut global,
                    &mut |bytes, b, g| {
                        if mode == 6 {
                            // Fault injection: the exact input is already in
                            // capture. Its existing event is counted truthfully.
                            super::super::live_borrowed_payload(
                                KOBLITZ_SLOTS[0].0,
                                KOBLITZ_SLOTS[0].1,
                                KOBLITZ_SLOTS[0].2,
                                bytes,
                            )
                            .unwrap();
                            *g += 1;
                            let previous = *g;
                            let result = storage.publish(bytes, b, g);
                            assert!(result.is_err());
                            assert_eq!(*g, previous);
                            assert!(storage.capture_sequence.is_none());
                            assert!(!storage.capture_acknowledged);
                            assert!(!root.join(KOBLITZ_SLOTS[0].0).exists());
                            super::super::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
                                Some(super::super::EventSink::Bound(sink)) => {
                                    sink.verify_digest_limits_at_count(
                                        &storage.binding_digest,
                                        &limits,
                                        previous as u64,
                                    )
                                    .unwrap();
                                }
                                _ => panic!("missing bound capture"),
                            });
                            return result;
                        }
                        if mode == 5 {
                            let quote =
                                quote_parent_storage_length(bytes.len() as u64, false, &limits)
                                    .unwrap();
                            b.checking_work = b.native_work_limit - b.native.work - quote.work + 1;
                        }
                        if mode == 3 {
                            storage.publish_with_finish_time(bytes, b, g, || late)
                        } else {
                            storage.publish(bytes, b, g)
                        }
                    },
                );
                assert!(storage.attempted);
                assert_eq!(run.matrix.is_some(), mode == 0, "mode {mode} {engine:?}");
                assert_eq!(storage.root_artifact.is_some(), mode == 0 || mode == 3);
                assert_eq!(storage.capture_acknowledged, matches!(mode, 0 | 1 | 3));
                if mode == 0 {
                    assert!(
                        run.matrix
                            .as_ref()
                            .unwrap()
                            .report
                            .as_ref()
                            .unwrap()
                            .native_step_complete
                    );
                    let bytes = std::fs::read(root.join(KOBLITZ_SLOTS[0].0)).unwrap();
                    assert_eq!(bytes, run.matrix.as_ref().unwrap().original_bytes);
                    let captured = verify_native_capture(
                        root,
                        &run.events.as_ref().unwrap().canonical,
                        &binding,
                        &limits,
                        23,
                        engine,
                        3,
                    )
                    .unwrap();
                    assert!(
                        captured.construction_complete_captured && captured.matrix_finish_captured
                    );
                    assert_eq!(captured.records + 1, global);
                } else {
                    assert!(
                        run.persistence_failure
                            .as_ref()
                            .unwrap()
                            .serialized_complete
                    );
                    assert_eq!(
                        budget.stopped,
                        Some(if mode == 3 {
                            Stop::Deadline
                        } else if mode == 5 {
                            Stop::Work
                        } else {
                            Stop::Observer
                        })
                    );
                }
                if mode == 1 {
                    assert_eq!(
                        std::fs::read(root.join(KOBLITZ_SLOTS[0].0)).unwrap(),
                        b"root collision"
                    );
                }
                let before = global;
                assert!(storage.publish(b"retry", &mut budget, &mut global).is_err());
                assert_eq!(global, before);
            }
        }
    }
}

#[cfg(test)]
mod producer_input_adoption_tests {
    use super::*;
    use crate::cryptanalysis::koblitz_groebner::bounded_matrix::Engine;
    #[test]
    fn original_storage_moves_into_parent_without_republication() {
        // Retain the existing owner tariffs; the receipt allocation was prepaid
        // by its producer and moves through a single Box owner.
        assert!(std::mem::size_of::<ParentArtifacts>() <= 512);
        assert!(std::mem::size_of::<OwnedRetainedArtifact>() <= 512);
        for engine in [Engine::F4Macaulay, Engine::F5KoszulFrobenius] {
            for mode in 0..4 {
                let binding = matrix_tests::binding();
                let capture = crate::ml_dataset::capture::capture_limits(16).unwrap();
                let sink = bound_signed_publisher_tests::SinkFixture::new(&binding);
                let root = sink.dir.parent().unwrap();
                if mode == 1 {
                    std::fs::write(root.join(KOBLITZ_SLOTS[0].0), b"root collision").unwrap();
                }
                let original = construction_tests::fixture();
                let mut budget = construction_tests::budget();
                let original_limits = budget.original_limits();
                let mut delivery = ParentDeliveryFile::admit(&mut budget).unwrap();
                let mut publisher =
                    PreMatrixInputStore::new(root, &binding, &capture, &mut budget).unwrap();
                let late = budget.deadline + Duration::from_secs(1);
                let mut global = 0;
                let mut run = run_canonical_pipeline(
                    &original,
                    &binding,
                    &capture,
                    &"8".repeat(64),
                    &"9".repeat(64),
                    engine,
                    3,
                    &mut budget,
                    &mut global,
                    &mut |bytes, b, g| {
                        if mode == 3 {
                            let quote =
                                quote_parent_storage_length(bytes.len() as u64, false, &capture)
                                    .unwrap();
                            // Fault injection consumes existing trace headroom, without increasing limits.
                            b.trace_bytes = b.trace_limit - quote.trace + 1;
                        }
                        if mode == 2 {
                            publisher.publish_with_finish_time(bytes, b, g, || late)
                        } else {
                            publisher.publish(bytes, b, g)
                        }
                    },
                );
                assert_eq!(
                    run.pipeline.as_ref().unwrap().native.matrix.is_some(),
                    mode == 0
                );
                let actual_input_pointer = if mode == 0 {
                    run.pipeline
                        .as_ref()
                        .unwrap()
                        .native
                        .matrix
                        .as_ref()
                        .unwrap()
                        .original_bytes
                        .as_ptr()
                } else {
                    run.pipeline
                        .as_ref()
                        .unwrap()
                        .native
                        .persistence_failure
                        .as_ref()
                        .unwrap()
                        .bytes
                        .as_ptr()
                };
                publisher.attach_to(&mut run).unwrap();
                assert!(publisher.attach_to(&mut run).is_err());
                let before = budget.saved_ledger();
                let receipt = run.producer_input.as_ref().unwrap();
                verify_producer_input_transition(receipt, before, &capture).unwrap();
                assert!(receipt.checking_admitted);
                assert_eq!(receipt.trace_admitted, mode != 3);
                assert_eq!(receipt.capture_sequence.is_some(), mode != 3);
                if mode == 3 {
                    let q = receipt.quote.unwrap();
                    assert_eq!(
                        receipt.exit.checking_reserved_work,
                        receipt.entry.checking_reserved_work + q.work
                    );
                    assert_eq!(
                        receipt.exit.adapter_reserved_bytes,
                        receipt.entry.adapter_reserved_bytes + q.bytes
                    );
                    assert_eq!(
                        receipt.exit.charged_trace_bytes,
                        receipt.entry.charged_trace_bytes
                    );
                    assert_eq!(receipt.exit.first_stop, Some(Stop::Trace));
                }
                let mut parent = ParentRetention::begin(&mut budget).unwrap();
                let transferred = take_parent_artifacts(&mut run, &mut parent);
                assert!(
                    transferred.error.is_none(),
                    "owner transfer {engine:?} mode {mode}"
                );
                let mut owned = transferred.value.unwrap();
                let input = &owned.items[0];
                assert_eq!(input.logical_path, KOBLITZ_SLOTS[0].0);
                assert_eq!(input.payload.bytes.as_ptr(), actual_input_pointer);
                assert!(
                    !input.storage_attempted
                        && !input.storage_admitted
                        && input.storage_quote.is_none()
                );
                assert_eq!(input.root_artifact.is_some(), mode == 0 || mode == 2);
                assert_eq!(input.capture_acknowledged, mode != 3);
                let prefix_before = crate::ml_dataset::capture::verify_live_committed_prefix(
                    root, &binding, &capture,
                )
                .unwrap()
                .unwrap();
                let original_input_count = prefix_before
                    .payloads
                    .iter()
                    .filter(|(_, a)| a.path == format!("payloads/{}", KOBLITZ_SLOTS[0].0))
                    .count();
                let _ = persist_parent_artifacts(&mut owned, &mut parent, root, &binding, &capture);
                let prefix_after = crate::ml_dataset::capture::verify_live_committed_prefix(
                    root, &binding, &capture,
                )
                .unwrap()
                .unwrap();
                assert_eq!(
                    prefix_after
                        .payloads
                        .iter()
                        .filter(|(_, a)| a.path == format!("payloads/{}", KOBLITZ_SLOTS[0].0))
                        .count(),
                    original_input_count
                );
                assert!(!owned.items[0].storage_attempted);
                if mode == 1 {
                    assert_eq!(
                        std::fs::read(root.join(KOBLITZ_SLOTS[0].0)).unwrap(),
                        b"root collision"
                    );
                }
                delivery.finish(&owned, &parent, &binding, root).unwrap();
                let mut manifest: Vec<_> = owned
                    .items
                    .iter()
                    .filter_map(|a| a.root_artifact.clone())
                    .collect();
                manifest.push(delivery.artifact.clone().unwrap());
                let checked = verify_parent_delivery(
                    root,
                    &manifest,
                    &binding,
                    &capture,
                    original_limits,
                    before,
                )
                .unwrap();
                assert!(checked.producer_transition_valid);
                assert!(!checked.producer_entry_anchor_verified);
                if mode == 1 || mode == 3 {
                    assert!(!checked.delivery_complete);
                }
                if mode == 0 {
                    let path = root.join(PARENT_DELIVERY_SLOT.0);
                    let original_record = std::fs::read(&path).unwrap();
                    let base: serde_json::Value = serde_json::from_slice(&original_record).unwrap();
                    for mutation in 0..7 {
                        let mut forged = base.clone();
                        match mutation {
                            0 => forged["producer_input"]["binding_sha256"] = "0".repeat(64).into(),
                            1 => {
                                let sequence = forged["producer_input"]["capture_sequence"]
                                    .as_u64()
                                    .unwrap();
                                // Keep the claimed sequence transition coherent;
                                // actual committed capture must still reject it.
                                forged["producer_input"]["capture_sequence"] =
                                    (sequence + 1).into();
                                let prior = forged["producer_input"]["capture_count_before"]
                                    .as_u64()
                                    .unwrap();
                                forged["producer_input"]["capture_count_before"] =
                                    (prior + 1).into();
                            }
                            2 => {
                                let byte = forged["producer_input"]["payload_sha256"][0]
                                    .as_u64()
                                    .unwrap();
                                forged["producer_input"]["payload_sha256"][0] =
                                    ((byte + 1) % 256).into();
                            }
                            3 => {
                                let work =
                                    forged["producer_input"]["quote"]["work"].as_u64().unwrap();
                                forged["producer_input"]["quote"]["work"] = (work + 1).into();
                            }
                            4 => {
                                // Forged parent-origin accounting of an already
                                // acknowledged producer storage operation.
                                forged["items"][0]["attempted"] = true.into();
                                forged["items"][0]["admitted"] = true.into();
                                forged["items"][0]["quote"] =
                                    forged["producer_input"]["quote"].clone();
                            }
                            5 => {
                                forged["items"].as_array_mut().unwrap().remove(0);
                            }
                            6 => forged["producer_input"] = serde_json::Value::Null,
                            _ => unreachable!(),
                        }
                        let bytes = serde_json::to_vec(&forged).unwrap();
                        // Re-sign the mutable test manifest entry so this tests
                        // semantic receipt authentication, not outer file hashing.
                        let mut forged_manifest = manifest.clone();
                        let a = forged_manifest
                            .iter_mut()
                            .find(|a| a.path == PARENT_DELIVERY_SLOT.0)
                            .unwrap();
                        a.bytes = bytes.len() as u64;
                        a.sha256 = crate::ml_dataset::digest(&bytes);
                        std::fs::write(&path, &bytes).unwrap();
                        assert!(
                            verify_parent_delivery(
                                root,
                                &forged_manifest,
                                &binding,
                                &capture,
                                original_limits,
                                before
                            )
                            .is_err(),
                            "producer receipt mutation {mutation} {engine:?}"
                        );
                    }
                    std::fs::write(&path, &original_record).unwrap();
                    assert!(verify_parent_delivery(
                        root,
                        &manifest,
                        &binding,
                        &capture,
                        original_limits,
                        before
                    )
                    .is_ok());
                }
                let r = owned.producer_input.as_mut().unwrap();
                let old = r.exit.checking_reserved_work;
                r.exit.checking_reserved_work += 1;
                assert!(verify_producer_input_transition(r, before, &capture).is_err());
                r.exit.checking_reserved_work = old;
                let old_entry = r.entry.checking_reserved_work;
                let old_exit = r.exit.checking_reserved_work;
                // A consistently lowered report can fit beneath the trusted later
                // boundary: it must never acquire independent-anchor status.
                r.entry.checking_reserved_work -= 1;
                r.exit.checking_reserved_work -= 1;
                assert!(verify_producer_input_transition(r, before, &capture).is_ok());
                r.entry.checking_reserved_work = old_entry;
                r.exit.checking_reserved_work = old_exit;
            }
        }
    }
}
